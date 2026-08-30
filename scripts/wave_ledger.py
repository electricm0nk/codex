#!/usr/bin/env python3
"""Show every SD-34 dispatch wave and how long it ran.

Waves are Workflow-tool runs. Each one leaves a transcript directory on disk at
`~/.claude/projects/*/*/subagents/workflows/wf_*/`, and that directory is the source
of truth here -- not a hand-maintained log, which would go stale and would not
survive the host resets this box has been taking (a wave killed mid-run never gets
to write its own "finished" record, but its files are still on disk).

Timing comes from two file kinds inside that directory:

  * `agent-<id>.meta.json` is written when a lane is spawned, so the earliest one
    is the wave's start.
  * `agent-<id>.jsonl` is appended to as a lane works, so the latest mtime is the
    wave's last sign of life.

A wave whose last activity is recent and whose task is still alive is RUNNING and
its duration is counted to now. Everything else is finished-or-killed, and this
script cannot tell those two apart from the filesystem alone -- a wave killed by a
host reset looks exactly like one that finished. `KILLED?` marks any wave whose
last activity lands within a minute of a known boot boundary.

Usage:
    python3 scripts/wave_ledger.py            # the table
    python3 scripts/wave_ledger.py --json     # machine-readable
    python3 scripts/wave_ledger.py --since 2026-08-27
"""

import argparse
import datetime as dt
import glob
import json
import os
import subprocess
import sys

PROJECTS = os.path.expanduser("~/.claude/projects")

# Run id -> the wave number it dispatched. A run only lands here once its identity is
# established from the repo (the commits it produced) or from this session's own launch.
# Anything absent is reported by run id rather than guessed at.
KNOWN_WAVES = {
    "wf_ae542ab0-a19": ("runaway", "accidental six-epic run: the deleted bucketB entry point "
                                   "let args fall through to all six epics"),
    "wf_d510aba2-144": ("11", "three lanes: UC / V-ledger / M"),
    "wf_5ba78e03-272": ("12", "same three lanes, relaunched after the crash restart"),
    "wf_e2fc3f32-68a": ("13", "UC / C / M in parallel -- killed by the host reset at 10:38"),
    "wf_2dcca902-e6d": ("14", "UC parallel with (C then M serialized), checkpoint rule"),
}


def boot_times():
    """When each boot ENDED -- the moment the host killed the box.

    `journalctl --list-boots` prints, per boot, an IDX, a boot id, then a FIRST ENTRY
    and a LAST ENTRY, each as `Day YYYY-MM-DD HH:MM:SS TZ` (4 fields). A wave dies at
    a boot's LAST entry, never its first, so only the last is a kill boundary. The
    current boot's last entry is just "now" and is skipped.
    """
    try:
        out = subprocess.run(["journalctl", "--list-boots", "--no-pager"],
                             capture_output=True, text=True, timeout=10).stdout
    except Exception:
        return []
    stamps = []
    for line in out.splitlines():
        parts = line.split()
        if len(parts) < 10 or not parts[0].lstrip("-").isdigit():
            continue
        if parts[0] == "0":          # the boot we are in has not ended
            continue
        try:
            stamps.append(dt.datetime.strptime(" ".join(parts[7:9]), "%Y-%m-%d %H:%M:%S"))
        except ValueError:
            pass
    return stamps


def collect(since):
    waves = []
    for d in glob.glob(os.path.join(PROJECTS, "*", "*", "subagents", "workflows", "wf_*")):
        metas = glob.glob(os.path.join(d, "agent-*.meta.json"))
        logs = glob.glob(os.path.join(d, "agent-*.jsonl"))
        if not logs:
            continue
        start = min(os.path.getmtime(f) for f in (metas or logs))
        end = max(os.path.getmtime(f) for f in logs)
        if dt.datetime.fromtimestamp(start) < since:
            continue
        waves.append({
            "run": os.path.basename(d),
            "start": start,
            "last_activity": end,
            "lanes": len(logs),
            "dir": d,
        })
    waves.sort(key=lambda w: w["start"])
    return waves


def annotate(waves, boots):
    now = dt.datetime.now().timestamp()
    for w in waves:
        wave, note = KNOWN_WAVES.get(w["run"], ("?", ""))
        w["wave"] = wave
        w["note"] = note
        idle = now - w["last_activity"]
        # Still writing in the last 3 minutes: treat as live.
        w["running"] = idle < 180
        w["duration_s"] = int((now if w["running"] else w["last_activity"]) - w["start"])
        w["killed"] = False
        if not w["running"]:
            last = dt.datetime.fromtimestamp(w["last_activity"])
            w["killed"] = any(abs((last - b).total_seconds()) < 180 for b in boots)
    return waves


def hms(seconds):
    return str(dt.timedelta(seconds=int(seconds)))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--since", default="2026-08-27",
                    help="ignore waves that started before this date (default: the SD-34 cut)")
    ap.add_argument("--json", action="store_true")
    args = ap.parse_args()

    since = dt.datetime.strptime(args.since, "%Y-%m-%d")
    waves = annotate(collect(since), boot_times())

    if args.json:
        json.dump(waves, sys.stdout, indent=2)
        print()
        return

    if not waves:
        print(f"no waves on disk since {args.since}")
        return

    fmt = "{:<12} {:<17} {:<15} {:<15} {:>9} {:>6}  {}"
    print(fmt.format("WAVE", "RUN", "STARTED", "LAST ACTIVITY", "RAN FOR", "LANES", "STATE"))
    print("-" * 100)
    for w in waves:
        f = lambda t: dt.datetime.fromtimestamp(t).strftime("%m-%d %H:%M:%S")
        state = "RUNNING" if w["running"] else ("KILLED?" if w["killed"] else "done")
        print(fmt.format("wave " + w["wave"], w["run"], f(w["start"]), f(w["last_activity"]),
                         hms(w["duration_s"]), w["lanes"], state))
        if w["note"]:
            print(" " * 13 + w["note"])

    live = [w for w in waves if w["running"]]
    print()
    print("{} waves since {}; {} running now. Total lane-time: {}.".format(
        len(waves), args.since, len(live), hms(sum(w["duration_s"] for w in waves))))
    print('"KILLED?" = last activity coincides with a boot boundary. The filesystem cannot')
    print("tell a finished wave from one the host killed; only the boot time can.")


if __name__ == "__main__":
    main()
