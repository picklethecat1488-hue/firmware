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

# Regex to match ANSI escape sequences (CSI sequences)
ANSI_ESCAPE_RE = re.compile(r"\x1b\[([0-9;]*)m")
ANSI_OTHER_RE = re.compile(r"\x1b(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])")

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
    colors, stripping unhandled control codes, escaping HTML characters, and creating
    clickable links for URLs and file paths.
    """
    if not raw_text:
        return ""

    # Split text by SGR ANSI escape sequences
    parts = ANSI_ESCAPE_RE.split(raw_text)

    out_chunks: List[str] = []
    classes: List[str] = []
    fg_class: str | None = None
    bg_class: str | None = None

    for i in range(0, len(parts), 2):
        chunk = parts[i]
        # Strip any other remaining ANSI non-SGR sequences
        chunk = ANSI_OTHER_RE.sub("", chunk)

        if chunk:
            escaped_chunk = html.escape(chunk)
            formatted_chunk = _format_links(escaped_chunk)

            all_classes = list(classes)
            if fg_class:
                all_classes.append(fg_class)
            if bg_class:
                all_classes.append(bg_class)

            if all_classes:
                class_str = " ".join(all_classes)
                out_chunks.append(f'<span class="{class_str}">{formatted_chunk}</span>')
            else:
                out_chunks.append(formatted_chunk)

        # Check if there is an ANSI code after this chunk
        if i + 1 < len(parts):
            code_str = parts[i + 1]
            if not code_str or code_str == "0":
                # Reset all
                classes.clear()
                fg_class = None
                bg_class = None
            else:
                codes = [int(c) for c in code_str.split(";") if c.isdigit()]
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

    return "".join(out_chunks)
