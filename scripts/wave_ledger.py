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

# Silence longer than this means dead, not merely quiet. Shared with the autonomous
# nudge's rule 3 so the two instruments cannot disagree -- see annotate().
IDLE_DEAD_SECONDS = 40 * 60

# Run id -> the wave number it dispatched. A run only lands here once its identity is
# established from the repo (the commits it produced) or from this session's own launch.
# Anything absent is reported by run id rather than guessed at.
KNOWN_WAVES = {
    "wf_ae542ab0-a19": ("runaway", "accidental six-epic run: the deleted bucketB entry point "
                                   "let args fall through to all six epics"),
    "wf_d510aba2-144": ("11", "three lanes: UC / V-ledger / M"),
    "wf_5ba78e03-272": ("12", "same three lanes, relaunched after the crash restart"),
    "wf_e2fc3f32-68a": ("13", "UC / C / M in parallel -- killed by the host reset at 10:38"),
    "wf_2dcca902-e6d": ("14", "UC parallel with (C then M serialized), checkpoint rule; "
                              "stopped at operator request for a clean restart, lane 1 saved "
                              "to salvage/wave14-lane1"),
    "wf_894155bf-d58": ("15", "wave 14 relaunched clean for live ledger visibility; lane 1 "
                              "resumes from salvage/wave14-lane1, which supersedes the wave-13 one"),
    "wf_d6622487-007": ("16", "same three lanes; adds the no-relabel rule after wave 15 reported "
                              "a C->V move as a closure"),
    "wf_850b57b3-2ed": ("17", "bucket-C premise retired; killed by the host at 22:12 when the "
                              "guest ran out of memory, but both lanes had pushed first -- "
                              "nothing lost, and .cargo/config.toml jobs=6 landed because of it"),
    "wf_47422ae1-5ea": ("18", "first wave under .cargo/config.toml jobs=6 -- watch free memory, "
                              "it is the run that proves whether the cap stops the host kills"),
    "wf_195c6a9e-931": ("19", "first wave with the page-cache guard on root cron; C 233, M 944"),
    "wf_75aaf9fb-a7d": ("20", "UC brief retired after 7 cycles of staleness; fresh-base rule "
                              "added after a lane opened at the tranche cut point"),
    "wf_e22a7b7d-419": ("21", "pool-group seam nearly mined out; regen must bake in 38e10d066b "
                              "and turn the committed-inventory test green"),
    "wf_56c5bcae-8f5": ("22", "UC lane's first task is the trait-picker no-stub gap it found "
                              "(4 of 7 tables chained); first wave after the 104 GB worktree sweep"),
    "wf_4a1d662c-fd1": ("23", "GATE wave -- all three lanes retargeted from bucket mining to "
                              "the 14 red verify.sh stages the fable review found"),
    "wf_31807799-a21": ("24", "gate 14 red -> 5 after wave 23; A takes root-full + the 124-row "
                              "NAMEISPI policy rows (masked, not a leak), B desktop/reach/site, C clippy"),
    "wf_6807ac3c-039": ("25", "last 4 data/corpus mechanisms of root-full; C re-measures the "
                              "whole gate itself -- the 14-red figure is several waves stale"),
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
        # A lane blocked on a long subprocess writes NOTHING to its transcript while it
        # waits. On 2026-09-01 wave 25's lane sat 10 minutes silent inside a
        # `scripts/verify.sh` run and this function -- then using a 3-minute threshold --
        # reported "0 running", which under the autonomous nudge's rule 4 ("no wave
        # running: dispatch the next immediately") would have put a SECOND writer on
        # tranche/14. That is the one thing the dispatch protocol forbids outright.
        #
        # 40 minutes matches the nudge's own "silent over 40 minutes: treat as dead" rule.
        # Two instruments disagreeing about what "running" means is how the near-miss
        # happened, so they now share one number. A full verify.sh sweep takes ~2h and can
        # be quiet for a long stretch of it.
        w["running"] = idle < IDLE_DEAD_SECONDS
        w["idle_s"] = int(idle)
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
        if w["running"] and w.get("idle_s", 0) > 180:
            state += f" (quiet {w['idle_s'] // 60}m)"
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
