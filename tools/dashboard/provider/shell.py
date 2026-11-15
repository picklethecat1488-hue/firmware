"""Shell logger and formatting utilities for console output and dashboard logging."""

import logging
from pathlib import Path
import sys
import threading
from typing import Any, Optional


class Logger:
    """Logger wrapper for console output."""

    def __init__(self, text="Running...", enabled=True):
        """Initialize logger with default text and enablement state."""
        self.text = text
        self.enabled = enabled
        self.running = False
        self.lock = threading.Lock()

    def print(self, msg, symbol="▶", restart=True):
        """Print formatted console message with indicator symbol."""
        if not self.enabled:
            return
        formatted = f"{symbol} {msg}" if symbol else msg
        print(formatted)

    def log(self, msg: str, symbol: Optional[str] = "📄"):
        """Log message with optional symbol prefix."""
        formatted = f"{symbol} {msg}" if symbol else msg
        print(formatted)

    def done(self):
        """Mark task execution as finished."""
        if self.enabled:
            print(f"Done {self.text}")
