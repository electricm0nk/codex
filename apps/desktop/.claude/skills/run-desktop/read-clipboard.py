#!/usr/bin/env python3
"""Print the CLIPBOARD selection of the X display named by $DISPLAY.

Companion to verify-on-screen.sh: after the driver sends ctrl+a ctrl+c to
the webview, this reads back the text that was actually rendered on screen.
No xclip/xsel exists in this container; python3-gi does.

Exits 1 (and prints nothing to stdout) if the clipboard is empty or the
display is unreachable — callers must treat that as a failed extraction,
never as "no expectations to check".
"""
import sys

import gi

gi.require_version("Gtk", "3.0")
from gi.repository import Gdk, Gtk  # noqa: E402

display = Gdk.Display.get_default()
if display is None:
    print("read-clipboard.py: cannot open DISPLAY", file=sys.stderr)
    sys.exit(1)

clipboard = Gtk.Clipboard.get_for_display(display, Gdk.SELECTION_CLIPBOARD)
text = clipboard.wait_for_text()
if not text or not text.strip():
    print("read-clipboard.py: clipboard is empty", file=sys.stderr)
    sys.exit(1)

sys.stdout.write(text)
