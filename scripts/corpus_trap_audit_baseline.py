#!/usr/bin/env python3
"""Compare `v06_corpus_trap_report --audit --json` output against the
registered inherited-debt baseline.

Why this exists as a separate, testable comparator rather than an inline
`defects == 0` check in `scripts/verify.sh`
(`docs/release/SD-34-book-completion/decisions.md` §13):

  * §13 rules that the four trap kinds SD-33 registered as out-of-DoD debt
    (`forward-scope-register.md` D1.1) **stay registered, not absorbed** —
    they are reported at their own counts, by name, every run.
  * The same §13 leaves `AT-34-E1-007`'s `exits 0` bar unchanged.

An aggregate `defects == 0` check cannot hold both at once: it is blind to
*which* kind moved, so it stays red forever over registered debt and tells
nobody anything — the permanently-red gate that decays into an unwired one,
which is the exact failure `AT-34-E1-007` exists to end (`AGENTS.md` rule 8,
"a warning is not a control").

So the gate is a **ratchet on named kinds**, which is strictly more
informative than the aggregate it replaces and cannot silently absorb
anything:

  * a DEFECT kind that is not in the register at all  -> FAIL (this is what
    catches `wiring-class-mismatch`, which is deliberately NOT registered:
    `AT-34-E1-008` drove it to 0 and any recurrence must fail immediately);
  * a registered kind ABOVE its pinned count           -> FAIL (regression);
  * a registered kind BELOW its pinned count           -> FAIL (the debt was
    paid down and the register is now stale — re-pin it here and in
    `forward-scope-register.md`, do not let the gate drift silently).

Every kind's count is printed on every run, PASS or FAIL.
"""

from __future__ import annotations

import collections
import json
import sys

# Registered inherited debt: SD-33's `forward-scope-register.md` D1.1
# (`v06_corpus_trap_report` integration-test target, one of the 29
# pre-existing failing suites verified at the `tranche/13` cut and ruled
# outside SD-33's Definition of Done). Re-affirmed as registered — not
# absorbed — by SD-34 `decisions.md` §13.
#
# Pinned 2026-08-27 at `tranche/14`, from the live audit re-run recorded in
# `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/AT-34-E1-007_cycle_receipt.md`.
# Re-derive:
#   cargo run --locked --bin v06_corpus_trap_report -- --audit --json | tail -1 \
#     | python3 -c "import json,sys,collections; \
#         print(collections.Counter(f['trap'] for f in json.load(sys.stdin)['findings'] \
#               if f['severity']=='DEFECT'))"
REGISTERED_DEFECT_BASELINE: dict[str, int] = {
    "mod-record": 2117,
    "key-differs-from-name": 650,
    "shared-name-distinct-records": 249,
    "disabled-line": 165,
}


def load_findings(path: str) -> list[dict]:
    """Take the last `{"findings": ...}` line in a log, the way the stage's
    own parse does — the binary prints progress ahead of its JSON."""
    with open(path, encoding="utf-8", errors="replace") as handle:
        lines = [ln for ln in handle if ln.lstrip().startswith('{"findings"')]
    if not lines:
        raise ValueError("no findings line in log")
    return json.loads(lines[-1])["findings"]


def compare(findings: list[dict]) -> tuple[bool, list[str], str]:
    defects = collections.Counter(
        f.get("trap") for f in findings if f.get("severity") == "DEFECT"
    )
    traps = collections.Counter(
        f.get("trap") for f in findings if f.get("severity") == "TRAP"
    )

    reasons: list[str] = []
    for kind in sorted(set(defects) | set(REGISTERED_DEFECT_BASELINE)):
        seen = defects.get(kind, 0)
        if kind not in REGISTERED_DEFECT_BASELINE:
            reasons.append(f"{kind}={seen} is NOT registered debt (expected 0)")
            continue
        pinned = REGISTERED_DEFECT_BASELINE[kind]
        if seen > pinned:
            reasons.append(f"{kind}={seen} REGRESSED above its pinned {pinned}")
        elif seen < pinned:
            reasons.append(
                f"{kind}={seen} is BELOW its pinned {pinned} — debt was paid down, "
                "re-pin REGISTERED_DEFECT_BASELINE and forward-scope-register.md D1.1"
            )

    parts = [f"{k}={defects.get(k, 0)}" for k in sorted(REGISTERED_DEFECT_BASELINE)]
    parts += [f"{k}={v}" for k, v in sorted(defects.items())
              if k not in REGISTERED_DEFECT_BASELINE]
    # `wiring-class-mismatch` is reported by name on every run even at 0:
    # AT-34-E1-008's evidence bar is that the stage *reports* it as 0.
    if "wiring-class-mismatch" not in defects:
        parts.insert(0, "wiring-class-mismatch=0")
    tally = "defects[" + " ".join(parts) + f"] traps={sum(traps.values())}"
    return (not reasons), reasons, tally


def main(argv: list[str]) -> int:
    if len(argv) != 2:
        print("usage: corpus_trap_audit_baseline.py <audit-json-log>", file=sys.stderr)
        return 1
    try:
        findings = load_findings(argv[1])
    except Exception as exc:  # noqa: BLE001 — surfaced verbatim by the stage
        print(f"PARSE_ERROR={exc}")
        return 1

    ok, reasons, tally = compare(findings)
    print(f"tally={tally}")
    if ok:
        print("verdict=PASS")
        return 0
    for reason in reasons:
        print(f"reason={reason}")
    print("verdict=FAIL")
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv))
