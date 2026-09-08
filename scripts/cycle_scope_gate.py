#!/usr/bin/env python3
"""The batch floor as a script with a nonzero exit (SD-35 `decisions.md §2`,
AT-35-E1-001), plus the mechanical receipt rows of `workflow-instruction.md §7`.

FLOOR MODE -- run before a cycle touches anything (`workflow-instruction.md §6` step 0):

    python3 scripts/cycle_scope_gate.py --min 500 --bucket B --kind class_feature
      -> scoped=7866 remaining_non_done=23315 floor=500 verdict=PASS                (exit 0)
    python3 scripts/cycle_scope_gate.py --min 500 --bucket A --kind companion
      -> scoped=28 remaining_non_done=23315 floor=500 verdict=FAIL_UNDER_FLOOR     (exit 1)
    python3 scripts/cycle_scope_gate.py --min 500 --bucket A --kind companion --or --bucket X --bucket U --bucket Z
      -> scoped=417 remaining_non_done=417 floor=500 verdict=PASS_WHOLE_REMAINDER  (exit 0)

The scope is read from `docs/work-inventory.json` (`--inventory` overrides).
A unit is DONE exactly when `scripts/completion_atlas.py` puts it in the
`DONE` bucket -- this script re-uses the atlas partition rather than
carrying a second, drifting status list. **Only non-DONE units count as
scoped**; `remaining_non_done` is every non-DONE unit in the inventory.

Scope flags, all repeatable:

    --bucket <A|B|C|D|M|V|U|X|Z|UNCLASSIFIED>   atlas bucket
    --kind <kind>                                unit kind
    --book <book>                                unit book
    --evidence-prefix <text>                     evidence string starts with <text>
    --token <name>                               unit carries the token: an entry of the
                                                 unit's `tokens` list when the inventory
                                                 carries one (AT-35-E2-004 onward), or a
                                                 `wiring_class_signals` entry matched whole
                                                 (`derived:bonus`) or by family (`bonus`)

Within one clause, repeats of the same flag OR together and different flags
AND together. `--or` starts a new clause; the scope is the union of clauses --
that is how mechanisms under the floor are bundled up to it. No flags at all
means the whole remainder.

Verdict: `PASS` when scoped >= floor; `PASS_WHOLE_REMAINDER` when scoped ==
remaining_non_done (the cycle takes everything left, `decisions.md §2`);
otherwise `FAIL_UNDER_FLOOR`, exit 1. Exit 2 is a usage or input error.

RECEIPT MODE -- run after the cycle's last figure-moving commit (`§6` step 5):

    python3 scripts/cycle_scope_gate.py --receipt --since <cycle-start-sha> \\
        --before /tmp/wi-before.json --after docs/work-inventory.json
      -> closed=<n> relabeled=<n> rust_lines_changed=<n> ratio=<lines per unit> builds_recorded=<n> pcgen_live_files=<n>

  closed              ids present in both inventories whose bucket was not DONE
                      before and is DONE after -- the only movement that is closure
                      (`decisions.md §9` L10)
  relabeled           ids whose bucket changed to another non-DONE bucket
  rust_lines_changed  added + deleted lines over `*.rs`, from
                      `git diff --numstat <since> -- '*.rs'` (committed since <since>
                      plus uncommitted; <since> defaults to HEAD)
  ratio               rust_lines_changed / closed, two decimals; `n/a` when closed=0
  builds_recorded     compile sessions in `$CARGO_TARGET_DIR` (`--target-dir` overrides):
                      cargo touches `.fingerprint/*/invoked.timestamp` for every unit it
                      compiles, so the timestamps cluster into sessions (gap > 300 s
                      starts a new one); sessions older than <since>'s commit time are
                      excluded. A `cargo test` that recompiles nothing adds no session.
  pcgen_live_files    `live_files=<n>` parsed from `python3 scripts/pcgen_residue_gate.py
                      --check` (`--residue-gate` overrides); `unavailable` when that
                      script is absent -- never a fabricated zero

Extra detail lines (`closed_by_kind=`, `relabeled_moves=`, `regressed=`,
`added=`, `dropped=`) print above the receipt line; the receipt line is
always the last line on stdout so a receipt can quote it verbatim.
"""

from __future__ import annotations

import collections
import glob
import json
import os
import re
import subprocess
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
INVENTORY_PATH = os.path.join(REPO_ROOT, "docs", "work-inventory.json")
RESIDUE_GATE_PATH = os.path.join(REPO_ROOT, "scripts", "pcgen_residue_gate.py")

sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import completion_atlas as _atlas  # noqa: E402  (the partition is the atlas's, not a copy)

UNCLASSIFIED = "UNCLASSIFIED"
DEFAULT_FLOOR = 500
BUILD_SESSION_GAP_SECONDS = 300

SCOPE_FLAGS = ("--bucket", "--kind", "--book", "--evidence-prefix", "--token")
_CLAUSE_KEYS = {"--bucket": "bucket", "--kind": "kind", "--book": "book",
                "--evidence-prefix": "evidence_prefix", "--token": "token"}


class ScopeError(ValueError):
    """A scope flag the gate does not understand, or one missing its value."""


# ---------------------------------------------------------------------------
# Partition
# ---------------------------------------------------------------------------

def bucket_of(unit: dict) -> str:
    b = _atlas._bucket_of(unit)
    return UNCLASSIFIED if b is None else b


def is_done(unit: dict) -> bool:
    return bucket_of(unit) == "DONE"


def remaining_non_done(units: list) -> list:
    return [u for u in units if not is_done(u)]


# ---------------------------------------------------------------------------
# Scope clauses
# ---------------------------------------------------------------------------

def _empty_clause() -> dict:
    return {"bucket": set(), "kind": set(), "book": set(), "evidence_prefix": [], "token": set()}


def clause_is_empty(clause: dict) -> bool:
    return not any(clause[k] for k in clause)


def parse_clauses(scope_argv: list) -> list:
    """Turn the scope flags into a list of clauses (union of AND-clauses)."""
    clauses = [_empty_clause()]
    i = 0
    while i < len(scope_argv):
        tok = scope_argv[i]
        if tok == "--or":
            clauses.append(_empty_clause())
            i += 1
            continue
        if tok not in _CLAUSE_KEYS:
            raise ScopeError(f"unknown scope flag {tok!r}; expected one of {', '.join(SCOPE_FLAGS)} or --or")
        if i + 1 >= len(scope_argv) or scope_argv[i + 1].startswith("--"):
            raise ScopeError(f"scope flag {tok} needs a value")
        key = _CLAUSE_KEYS[tok]
        value = scope_argv[i + 1]
        if key == "evidence_prefix":
            clauses[-1][key].append(value)
        else:
            clauses[-1][key].add(value)
        i += 2
    return clauses


def _unit_tokens(unit: dict) -> set:
    names = set()
    for t in unit.get("tokens") or []:
        names.add(str(t))
    for sig in unit.get("wiring_class_signals") or []:
        sig = str(sig)
        names.add(sig)
        if ":" in sig:
            names.add(sig.split(":", 1)[1])
    return names


def unit_matches_clause(unit: dict, clause: dict) -> bool:
    if clause["bucket"] and bucket_of(unit) not in clause["bucket"]:
        return False
    if clause["kind"] and unit.get("kind") not in clause["kind"]:
        return False
    if clause["book"] and unit.get("book") not in clause["book"]:
        return False
    if clause["evidence_prefix"]:
        evidence = unit.get("evidence") or ""
        if not any(evidence.startswith(p) for p in clause["evidence_prefix"]):
            return False
    if clause["token"] and not (_unit_tokens(unit) & clause["token"]):
        return False
    return True


def scoped_units(units: list, clauses: list) -> list:
    """Non-DONE units matching any clause. No clauses (one empty clause) = the whole remainder."""
    out = []
    for u in units:
        if is_done(u):
            continue
        if any(unit_matches_clause(u, c) for c in clauses):
            out.append(u)
    return out


def describe_clauses(clauses: list) -> str:
    parts = []
    for c in clauses:
        if clause_is_empty(c):
            parts.append("(whole remainder)")
            continue
        bits = []
        for key in ("bucket", "kind", "book", "evidence_prefix", "token"):
            vals = sorted(c[key])
            if vals:
                bits.append(f"{key}={'|'.join(vals)}")
        parts.append(" AND ".join(bits))
    return " OR ".join(parts)


# ---------------------------------------------------------------------------
# Verdict
# ---------------------------------------------------------------------------

def verdict(scoped: int, remaining: int, floor: int) -> tuple:
    if scoped >= floor:
        return "PASS", 0
    if scoped == remaining:
        return "PASS_WHOLE_REMAINDER", 0
    return "FAIL_UNDER_FLOOR", 1


def _counter_line(label: str, counter: "collections.Counter") -> str:
    body = " ".join(f"{k}:{v}" for k, v in sorted(counter.items()))
    return f"{label}={body}"


# ---------------------------------------------------------------------------
# Receipt math
# ---------------------------------------------------------------------------

def _by_id(units: list) -> dict:
    out = {}
    for u in units:
        out[u.get("id")] = u
    return out


def movement(before_units: list, after_units: list) -> dict:
    before = _by_id(before_units)
    after = _by_id(after_units)
    closed_ids = []
    closed_by_kind = collections.Counter()
    relabeled_moves = collections.Counter()
    regressed = 0
    for uid, b_unit in before.items():
        a_unit = after.get(uid)
        if a_unit is None:
            continue
        b_bucket = bucket_of(b_unit)
        a_bucket = bucket_of(a_unit)
        if b_bucket == a_bucket:
            continue
        if a_bucket == "DONE":
            closed_ids.append(uid)
            closed_by_kind[a_unit.get("kind") or "?"] += 1
        elif b_bucket == "DONE":
            regressed += 1
        else:
            relabeled_moves[f"{b_bucket}->{a_bucket}"] += 1
    return {
        "closed": len(closed_ids),
        "closed_ids": closed_ids,
        "closed_by_kind": closed_by_kind,
        "relabeled": sum(relabeled_moves.values()),
        "relabeled_moves": relabeled_moves,
        "regressed": regressed,
        "added": len(set(after) - set(before)),
        "dropped": len(set(before) - set(after)),
    }


def rust_lines_from_numstat(text: str) -> int:
    total = 0
    for line in text.splitlines():
        parts = line.split("\t")
        if len(parts) < 3:
            continue
        added, deleted = parts[0], parts[1]
        if added.isdigit():
            total += int(added)
        if deleted.isdigit():
            total += int(deleted)
    return total


def rust_lines_changed(since: str, repo: str = REPO_ROOT) -> int:
    proc = subprocess.run(
        ["git", "diff", "--numstat", since, "--", "*.rs"],
        cwd=repo, capture_output=True, text=True, check=True,
    )
    return rust_lines_from_numstat(proc.stdout)


def commit_epoch(rev: str, repo: str = REPO_ROOT) -> "float | None":
    try:
        proc = subprocess.run(
            ["git", "show", "-s", "--format=%ct", rev],
            cwd=repo, capture_output=True, text=True, check=True,
        )
        return float(proc.stdout.strip().splitlines()[-1])
    except (subprocess.CalledProcessError, ValueError, IndexError):
        return None


def build_sessions(target_dir: "str | None", since_epoch: "float | None" = None,
                   gap_seconds: float = BUILD_SESSION_GAP_SECONDS) -> int:
    if not target_dir or not os.path.isdir(target_dir):
        return 0
    stamps = []
    for path in glob.glob(os.path.join(target_dir, "**", ".fingerprint", "*", "invoked.timestamp"), recursive=True):
        try:
            stamps.append(os.stat(path).st_mtime)
        except OSError:
            continue
    stamps.sort()
    sessions = 0
    last = None
    for t in stamps:
        if since_epoch is not None and t < since_epoch:
            continue
        if last is None or t - last > gap_seconds:
            sessions += 1
        last = t
    return sessions


_LIVE_FILES_RE = re.compile(r"\blive_files=(\d+)\b")


def pcgen_live_files(gate_path: str = RESIDUE_GATE_PATH, repo: str = REPO_ROOT) -> str:
    if not os.path.isfile(gate_path):
        return "unavailable"
    proc = subprocess.run(
        [sys.executable, gate_path, "--check"],
        cwd=repo, capture_output=True, text=True,
    )
    m = None
    for m in _LIVE_FILES_RE.finditer(proc.stdout + "\n" + proc.stderr):
        pass
    return m.group(1) if m else "unavailable"


def format_ratio(lines: int, closed: int) -> str:
    return f"{lines / closed:.2f}" if closed > 0 else "n/a"


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

_OPTS_WITH_VALUE = ("--min", "--inventory", "--before", "--after", "--since", "--repo",
                    "--target-dir", "--residue-gate")


def _split_argv(argv: list) -> tuple:
    """Separate the gate's own options from the scope flags."""
    opts = {"receipt": False}
    scope = []
    i = 0
    while i < len(argv):
        tok = argv[i]
        if tok == "--receipt":
            opts["receipt"] = True
            i += 1
        elif tok in _OPTS_WITH_VALUE:
            if i + 1 >= len(argv):
                raise ScopeError(f"{tok} needs a value")
            opts[tok.lstrip("-").replace("-", "_")] = argv[i + 1]
            i += 2
        elif tok in ("-h", "--help"):
            opts["help"] = True
            i += 1
        else:
            scope.append(tok)
            i += 1
    return opts, scope


def _load(path: str) -> list:
    with open(path, "r", encoding="utf-8") as fh:
        data = json.load(fh)
    units = data.get("units") if isinstance(data, dict) else None
    if not isinstance(units, list):
        raise ScopeError(f"{path}: no `units` list")
    return units


def _floor_mode(opts: dict, scope: list) -> int:
    floor = int(opts["min"])
    inventory = opts.get("inventory", INVENTORY_PATH)
    clauses = parse_clauses(scope)
    units = _load(inventory)
    remaining = remaining_non_done(units)
    scoped = scoped_units(units, clauses)
    by_bucket = collections.Counter(bucket_of(u) for u in scoped)
    by_kind = collections.Counter((u.get("kind") or "?") for u in scoped)
    result, code = verdict(len(scoped), len(remaining), floor)
    print(f"inventory={os.path.relpath(inventory, REPO_ROOT) if inventory.startswith(REPO_ROOT) else inventory}")
    print(f"scope={describe_clauses(clauses)}")
    print(_counter_line("scoped_by_bucket", by_bucket))
    print(_counter_line("scoped_by_kind", by_kind))
    print(f"scoped={len(scoped)} remaining_non_done={len(remaining)} floor={floor} verdict={result}")
    return code


def _receipt_mode(opts: dict) -> int:
    for needed in ("before", "after"):
        if needed not in opts:
            raise ScopeError(f"--receipt needs --before <inventory> and --after <inventory> (missing --{needed})")
    repo = opts.get("repo", REPO_ROOT)
    since = opts.get("since", "HEAD")
    target_dir = opts.get("target_dir", os.environ.get("CARGO_TARGET_DIR"))
    gate = opts.get("residue_gate", RESIDUE_GATE_PATH)

    mv = movement(_load(opts["before"]), _load(opts["after"]))
    lines = rust_lines_changed(since, repo=repo)
    builds = build_sessions(target_dir, since_epoch=commit_epoch(since, repo=repo))
    live = pcgen_live_files(gate, repo=repo)

    print(f"since={since} target_dir={target_dir or '(unset)'} residue_gate={'present' if os.path.isfile(gate) else 'absent'}")
    print(_counter_line("closed_by_kind", mv["closed_by_kind"]))
    print(_counter_line("relabeled_moves", mv["relabeled_moves"]))
    print(f"regressed={mv['regressed']} added={mv['added']} dropped={mv['dropped']}")
    print(
        f"closed={mv['closed']} relabeled={mv['relabeled']} rust_lines_changed={lines} "
        f"ratio={format_ratio(lines, mv['closed'])} builds_recorded={builds} pcgen_live_files={live}"
    )
    return 0


def main(argv: "list[str] | None" = None) -> int:
    argv = list(sys.argv[1:] if argv is None else argv)
    try:
        opts, scope = _split_argv(argv)
        if opts.get("help"):
            print(__doc__)
            return 0
        if opts["receipt"]:
            if scope:
                raise ScopeError(f"--receipt takes no scope flags (got {' '.join(scope)})")
            return _receipt_mode(opts)
        if "min" not in opts:
            raise ScopeError("one mode is required: --min <floor> <scope flags> or --receipt --before <inventory> --after <inventory>")
        return _floor_mode(opts, scope)
    except ScopeError as exc:
        print(f"cycle_scope_gate: {exc}", file=sys.stderr)
        return 2
    except (OSError, json.JSONDecodeError) as exc:
        print(f"cycle_scope_gate: {exc}", file=sys.stderr)
        return 2
    except subprocess.CalledProcessError as exc:
        print(f"cycle_scope_gate: {' '.join(exc.cmd)} failed: {exc.stderr.strip()}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
