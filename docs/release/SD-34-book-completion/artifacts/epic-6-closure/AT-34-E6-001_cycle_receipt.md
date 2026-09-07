---
canonical: true
owner: sd34-e6-scan
bundle_id: SD-34
criterion: AT-34-E6-001
date: 2026-08-29
verdict: FAIL
---

# AT-34-E6-001 — final-acceptance scan, attempt 1

**Verdict: FAIL.** The scan stops at obligation 1 (`acceptance-and-verification.md §3`). Per the
criterion's own closing clause and §4, **no retrospective, no sweep, no PR.**

Scan run at `HEAD = 9564144389`, branch `tranche/14`, branch-cut `571307724f`.
`RETRO_ACTOR=sd34-e6-scan`, `CARGO_TARGET_DIR=/tmp/cargo-sd34-e6-scan`.

## Shortfall 1 — five kanban cards are not `complete` (blocking)

`epic-breakdown.md ### AT-34-E6-001`: *"every `kanban.md` card is `complete`. A card at
`in-progress`, `blocked-escalated`, or `complete`-with-a-deferred-half blocks closure."*

Re-derived from the board (not from any dispatch return value, `decisions.md §12` L3):

```bash
grep -n '^| [0-9]' docs/release/SD-34-book-completion/kanban.md \
  | awk -F'|' '{print $2, "|", $3, "|", $5, "|", $6}'
```

| # | Card | Criterion | Status |
|---|---|---|---|
| 13 | `core-bucket-b-zero` | AT-34-E3-001 | **in-progress** |
| 14 | `core-bucket-c-zero` | AT-34-E3-002 | **in-progress** |
| 15 | `core-buckets-m-v-d-u-x-zero` | AT-34-E3-003 | **in-progress** |
| 17 | `core-rulebook-zero-remaining` | AT-34-E3-005 | **in-progress** |
| 20 | `ultimate-campaign-zero-remaining` | AT-34-E4-002 | **partial** |

20 of 27 rows are `complete`; rows 26 and 27 are the closure rows this scan gates, leaving
**20 of 25 substantive rows complete, 5 short**.

Board-hygiene defect, secondary: row 20's status `partial` is **not in `kanban.md`'s own stated
status vocabulary** (`not-started | in-progress | complete | blocked-escalated`). Per the same
file, a cycle that closes part of its population *"leaves its row at `in-progress` and reports
`partial`"* — `partial` is the report, `in-progress` is the status. Row 20 should read
`in-progress`. Either way it is not `complete`.

## Shortfall 2 — the two completion criteria fail their own verifying commands at HEAD

Not the board's account of the work — the commands from `acceptance-and-verification.md §1`,
re-run by this scan (obligation 2):

```bash
python3 scripts/completion_atlas.py --book core_rulebook --check
# book=core_rulebook population=6701 unclassified=0 overlap=0
#   DONE: 1448   A: 0  B: 532  C: 372  D: 382  M: 1048  V: 2793  U: 10  X: 116  Z: 0
# exit=1
```
AT-34-E3-005 requires `DONE=6701 of 6701`, exit 0. Actual: **DONE 1448 of 6701, exit 1** —
5,253 units short.

```bash
python3 scripts/completion_atlas.py --book ultimate_campaign --check
# book=ultimate_campaign population=265 unclassified=0 overlap=0
#   DONE: 130   A: 0  B: 0  C: 0  D: 5  M: 89  V: 18  U: 21  X: 2  Z: 0
# exit=1
```
AT-34-E4-002 requires `DONE=265 of 265`, exit 0. Actual: **DONE 130 of 265, exit 1** — 135 units
short (`M:89 V:18 D:5 U:21 X:2`, summing exactly, as row 20's own note states).

Consequently AT-34-E3-001 (`B` at 0 for `core_rulebook`) reads **B=532**, AT-34-E3-002 (`C` at 0)
reads **C=372**, and AT-34-E3-003 (`M,V,D,U,X` at 0) reads **M=1048 V=2793 D=382 U=10 X=116**.
Five criteria fail by their own §1 commands. This is not a board-labelling error to be corrected
in place — the underlying work is genuinely unbuilt.

## What is NOT the problem

Stated so the next cycle does not spend itself re-diagnosing settled ground:

- **`## Open blockers` is clean.** Obligation 8, real heading bounded at the next `## `,
  `<details>` archive ignored: the section's only entry is the archived, resolved AT-34-E3-001
  filing (`decisions.md §14`), and the live text reads `*(no active blockers)*`. Nothing pauses
  the bundle. The halt here is unbuilt DoD scope, not an escalation.
- **The corpus-wide atlas is healthy and balances at HEAD.**
  `python3 scripts/completion_atlas.py --check` -> `population=49438 buckets=10 unclassified=0
  overlap=0`, `done_evidence_violations=0`, `missing_clearing_mechanisms=0`,
  `citation_failures=0`, exit 0. The §3a deliverable-integrity check on the atlas passes.
- Corpus-wide standing: `DONE 14686 of 49438`, remainder `A:449 B:11831 C:4353 D:3055 M:5114
  V:9558 U:202 X:171 Z:19`.

## Residue

None. Running `completion_atlas.py --check` rewrites `derived_at` in
`artifacts/epic-1-atlas/completion-atlas.json`; this scan restored the file with
`git checkout --` so the tree carries no scan residue. Noted for the next cycle: the committed
`derived_at` is `bc39ff9d56`, five commits behind `HEAD`, so `stale_derived_at` is only False
because a live run refreshes it in place.

## Obligations not reached

Obligations 3–7 and 9–13, and the remaining §3a deliverable-integrity checks (manifest evidence
pointers, forward-plan rate/sample-size rows, the nine tables, `U`/`D` sub-cause censuses, the
gate re-proof, the widest-build-scope run) were **not executed**. The scan halts at the first
blocking shortfall by design; a later attempt runs the full thirteen once the board can be
`complete` end to end.

## Disposition

The bundle continues. Epic 3 (cards 13/14/15/17) and Epic 4 (card 20) have named, populated,
exactly-summing remainders and are dispatchable as further cycles — a sequencing problem, not an
escalation (`../../../../governance/blocker-closure-doctrine.md`). Re-run AT-34-E6-001 when
`--book core_rulebook --check` and `--book ultimate_campaign --check` both exit 0.
