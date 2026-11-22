#!/usr/bin/env python3
"""E3.MC evidence capture, run beside `node scripts/ui-smoke/run.mjs` (it never touches the app).

Every row of run.mjs deletes the characters it created once it is back on the landing screen, and
the isolated app-data root is removed at the end of the run. This watcher keeps what the
comparison needs before that happens:

  * each character directory that appears under the run's isolated root
    (/tmp/codex-ui-smoke-<agent>-*/**/characters/<id>/), copied file by file while it exists
    (the last copy is the saved state the sheet was loaded from);
  * the UI probe snapshot (/tmp/run-desktop-driver-<agent>.ui-probe.json) whenever its bodyText
    shows a loaded Cyberpunk RED sheet for one of the NAMES (the sheet heading line followed by
    "Cyberpunk RED"), the last such snapshot per name.

    python3 capture.py <agent> <out dir> <name> [<name> ...]     (stops on SIGTERM)
"""
import glob
import json
import os
import shutil
import signal
import sys
import time

agent, out, names = sys.argv[1], sys.argv[2], sys.argv[3:]
label = "".join(c if c.isalnum() or c in "_-" else "_" for c in agent)
probe_path = f"/tmp/run-desktop-driver-{agent}.ui-probe.json"
os.makedirs(os.path.join(out, "characters"), exist_ok=True)
running = True


def stop(*_):
    global running
    running = False


signal.signal(signal.SIGTERM, stop)
seen_ts = None
while running:
    for char_dir in glob.glob(f"/tmp/codex-ui-smoke-{label}-*/**/characters/*/", recursive=True):
        dest = os.path.join(out, "characters", os.path.basename(os.path.normpath(char_dir)))
        try:
            shutil.copytree(char_dir, dest, dirs_exist_ok=True)
        except (OSError, shutil.Error):
            pass  # deleted mid-copy; the previous copy stands
    try:
        with open(probe_path, encoding="utf-8") as handle:
            snapshot = json.load(handle)
    except (OSError, ValueError):
        snapshot = None
    if snapshot and snapshot.get("ts") != seen_ts:
        seen_ts = snapshot.get("ts")
        lines = [line.strip() for line in snapshot.get("bodyText", "").split("\n")]
        for name in names:
            if any(lines[i] == name and lines[i + 1] == "Cyberpunk RED" for i in range(len(lines) - 1)):
                slug = "".join(c if c.isalnum() else "-" for c in name.lower())
                with open(os.path.join(out, f"probe-{slug}.json"), "w", encoding="utf-8") as handle:
                    json.dump(snapshot, handle)
    time.sleep(0.2)
