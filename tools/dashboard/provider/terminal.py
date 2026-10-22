"""Terminal formatting and coloration engine.

Converts ANSI escape sequences, font weights, colors, URLs, and file paths
into styled HTML matching the VS Code terminal aesthetic.
"""

from __future__ import annotations

import html
import re
from typing import Dict, List

# Standard ANSI SGR color mapping
ANSI_FG_COLORS: Dict[int, str] = {
    30: "term-fg-black",
    31: "term-fg-red",
    32: "term-fg-green",
    33: "term-fg-yellow",
    34: "term-fg-blue",
    35: "term-fg-magenta",
    36: "term-fg-cyan",
    37: "term-fg-white",
    90: "term-fg-bright-black",
    91: "term-fg-bright-red",
    92: "term-fg-bright-green",
    93: "term-fg-bright-yellow",
    94: "term-fg-bright-blue",
    95: "term-fg-bright-magenta",
    96: "term-fg-bright-cyan",
    97: "term-fg-bright-white",
}

ANSI_BG_COLORS: Dict[int, str] = {
    40: "term-bg-black",
    41: "term-bg-red",
    42: "term-bg-green",
    43: "term-bg-yellow",
    44: "term-bg-blue",
    45: "term-bg-magenta",
    46: "term-bg-cyan",
    47: "term-bg-white",
    100: "term-bg-bright-black",
    101: "term-bg-bright-red",
    102: "term-bg-bright-green",
    103: "term-bg-bright-yellow",
    104: "term-bg-bright-blue",
    105: "term-bg-bright-magenta",
    106: "term-bg-bright-cyan",
    107: "term-bg-bright-white",
}

# Regex to match ANSI SGR escapes, OSC 8 hyperlinks, other OSC sequences, and control characters
ANSI_TOKEN_RE = re.compile(
    r"("
    r"\x1b\[([0-9;]*)m"                                      # Group 2: CSI SGR codes
    r"|\x1b\]8;[^\x1b\x07]*;([^\x1b\x07]+)(?:\x1b\\|\x07)"  # Group 3: OSC 8 Hyperlink open (URL)
    r"|\x1b\]8;[^\x1b\x07]*;(?:\x1b\\|\x07)"                # OSC 8 Hyperlink close
    r"|\x1b\][^\x1b\x07]*(?:\x1b\\|\x07)"                    # Other OSC sequences (window title, etc.)
    r"|\x1b(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])"               # Other CSI / Fe escape sequences
    r"|[\x00-\x08\x0b-\x1a\x1c-\x1f]"                       # Other control characters (excluding \t, \n, \r)
    r")"
)

# Regex to detect orphaned/stripped 8;;<URL><TEXT>8;; sequences where \x1b] was dropped
ORPHANED_OSC8_RE = re.compile(r"(^|[^\x1b\]])8;;(https?://[^\s`\"'<>\)]+)(.*?)(?:8;;)")

# Regex to match URLs
URL_RE = re.compile(r"(https?://[^\s<>\"'\)]+)")

# Regex to match file paths
# Matches absolute paths (/Users/..., /home/...) or workspace relative paths (src/..., tools/..., target/...)
FILE_PATH_RE = re.compile(
    r"(?<![a-zA-Z0-9_\-\.\/])("
    r"(?:/(?:Users|home|root|[a-zA-Z0-9_\-\.]+)/[a-zA-Z0-9_\-\.\/]+)"
    r"|(?:(?:src|tools|board|platform|controller|peripheral|tests|docs|attachments|feedback|target)/[a-zA-Z0-9_\-\.\/]+)"
    r")"
)


def _format_links(escaped_text: str) -> str:
    """Detect and convert URLs and file paths into styled clickable links."""
    # First convert web URLs
    def replace_url(match: re.Match) -> str:
        url = match.group(1)
        return f'<a href="{url}" target="_blank" rel="noopener noreferrer" class="term-link">{url}</a>'

    result = URL_RE.sub(replace_url, escaped_text)

    # Next convert file paths (avoiding already created <a> tags)
    def replace_file_path(match: re.Match) -> str:
        path = match.group(1)
        # Avoid matching trailing punctuation
        trailing = ""
        while path and path[-1] in ".,;:?)'\"]":
            trailing = path[-1] + trailing
            path = path[:-1]
        if not path:
            return match.group(1)
        vscode_url = f"vscode://file{path if path.startswith('/') else '/' + path}"
        return f'<a href="{vscode_url}" class="term-link term-file-link" data-path="{path}">{path}</a>{trailing}'

    # Split by existing HTML tags to avoid modifying inside attributes
    tokens = re.split(r"(<[^>]+>)", result)
    for i in range(len(tokens)):
        if not tokens[i].startswith("<"):
            tokens[i] = FILE_PATH_RE.sub(replace_file_path, tokens[i])

    return "".join(tokens)


def ansi_to_html(raw_text: str) -> str:
    """Convert raw text containing ANSI escape sequences into styled HTML.

    Handles font weights (bold, dim), styles (italic, underline), foreground/background
    colors, OSC 8 hyperlinks, stripping unhandled control codes, escaping HTML characters,
    and creating clickable links for URLs and file paths.
    """
    if not raw_text:
        return ""

    # Normalize any orphaned / unprefixed 8;;url...8;; sequences
    raw_text = ORPHANED_OSC8_RE.sub(
        lambda m: m.group(1) + "\x1b]8;;" + m.group(2) + "\x1b\\" + m.group(3) + "\x1b]8;;\x1b\\",
        raw_text,
    )

    out_chunks: List[str] = []
    classes: List[str] = []
    fg_class: str | None = None
    bg_class: str | None = None
    current_url: str | None = None
    last_idx = 0

    for m in ANSI_TOKEN_RE.finditer(raw_text):
        chunk = raw_text[last_idx:m.start()]
        last_idx = m.end()

        if chunk:
            escaped_chunk = html.escape(chunk)
            formatted_chunk = escaped_chunk if current_url else _format_links(escaped_chunk)

            all_classes = list(classes)
            if fg_class:
                all_classes.append(fg_class)
            if bg_class:
                all_classes.append(bg_class)

            if all_classes:
                class_str = " ".join(all_classes)
                content = f'<span class="{class_str}">{formatted_chunk}</span>'
            else:
                content = formatted_chunk

            if current_url:
                content = f'<a href="{html.escape(current_url)}" target="_blank" rel="noopener noreferrer" class="term-link">{content}</a>'
            out_chunks.append(content)

        full_token = m.group(1)
        sgr_code = m.group(2)
        osc8_url = m.group(3)

        if sgr_code is not None:
            if not sgr_code or sgr_code == "0":
                classes.clear()
                fg_class = None
                bg_class = None
            else:
                codes = [int(c) for c in sgr_code.split(";") if c.isdigit()]
                for code in codes:
                    if code == 0:
                        classes.clear()
                        fg_class = None
                        bg_class = None
                    elif code == 1:
                        if "term-bold" not in classes:
                            classes.append("term-bold")
                    elif code == 2:
                        if "term-dim" not in classes:
                            classes.append("term-dim")
                    elif code == 3:
                        if "term-italic" not in classes:
                            classes.append("term-italic")
                    elif code == 4:
                        if "term-underline" not in classes:
                            classes.append("term-underline")
                    elif code in (21, 22):
                        classes = [c for c in classes if c not in ("term-bold", "term-dim")]
                    elif code == 23:
                        classes = [c for c in classes if c != "term-italic"]
                    elif code == 24:
                        classes = [c for c in classes if c != "term-underline"]
                    elif code in ANSI_FG_COLORS:
                        fg_class = ANSI_FG_COLORS[code]
                    elif code == 39:
                        fg_class = None
                    elif code in ANSI_BG_COLORS:
                        bg_class = ANSI_BG_COLORS[code]
                    elif code == 49:
                        bg_class = None
        elif osc8_url is not None:
            current_url = osc8_url
        elif full_token.startswith("\x1b]8;"):
            current_url = None

    chunk = raw_text[last_idx:]
    if chunk:
        escaped_chunk = html.escape(chunk)
        formatted_chunk = escaped_chunk if current_url else _format_links(escaped_chunk)

        all_classes = list(classes)
        if fg_class:
            all_classes.append(fg_class)
        if bg_class:
            all_classes.append(bg_class)

        if all_classes:
            class_str = " ".join(all_classes)
            content = f'<span class="{class_str}">{formatted_chunk}</span>'
        else:
            content = formatted_chunk

        if current_url:
            content = f'<a href="{html.escape(current_url)}" target="_blank" rel="noopener noreferrer" class="term-link">{content}</a>'
        out_chunks.append(content)

    return "".join(out_chunks)

