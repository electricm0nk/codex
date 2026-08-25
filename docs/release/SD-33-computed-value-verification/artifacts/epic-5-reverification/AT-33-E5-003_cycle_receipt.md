# Cycle AT-33-E5-003 — Epic 5 Re-verification / AT-33-E5-003

- **Commit SHA:** `77e89e02df` (the landing commit this receipt describes; recorded back into the
  receipt in the same commit's follow-up, matching `AT-33-E5-001`'s `e10dead123` /
  `AT-33-E5-002`'s `4dc7e9d8cf` precedent)
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/README.md` (extended — new `AT-33-E5-003` section appended after `AT-33-E5-002`'s)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003_cycle_receipt.md` (this file)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json` (new — the two prior cycles' 32 records merged, for independent combined verification)
  - `docs/release/SD-33-computed-value-verification/progress.md` (updated — new Cycle entry, `## Disagreement ledger` section)
  - `docs/release/SD-33-computed-value-verification/kanban.md` (updated — row 18)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (see Test scoping)
- **Wired-integration audit result:** OK_NO_TOKENS (see Test scoping)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-003 — every disagreement is a named defect, fixed or escalated
  >
  > A disagreement is **never** closed by adjusting the expectation to match our output. Each is
  > root-caused: either our computation is wrong (fix it) or the oracle comparison is wrong (fix
  > the harness, and re-run everything it already judged).
  >
  > **Evidence:** one entry per disagreement in `progress.md`, each resolved to a commit or an
  > operator escalation. **A filed blocker does not satisfy this criterion**
  > (`../../governance/blocker-closure-doctrine.md`).

## What landed

Independently re-derived the current disagreement population from the two committed oracle-results
files `AT-33-E5-001`/`AT-33-E5-002` produced — not transcribed from either receipt's prose (both
receipts' own next-cycle plans already stated "0 disagree" but this cycle re-computed it directly
from the underlying JSON, per `AGENTS.md`'s "subagent recaps quote stale intermediate figures"
lesson). Merged both files (32 records total, disjoint unit populations — `equipment` kind under
`fixture-verified` status, 11 records; `equipment` kind under `literal-verified` status, 21
records) and re-checked the merge through Epic 1's own fail-closed instrument
(`scripts/box_ledger.py --check --oracle-results`), independently of the harness that produced
them.

**Result: 0 disagreements found in the 32 units `AT-33-E5-001`/`AT-33-E5-002` have examined to
date.** Zero `progress.md` entries are required by this criterion's evidence line, because there is
nothing to enter — a legitimate zero-yield outcome, not a stub, not an unexamined gap, and not an
adjustment of any expectation.

**This is not a claim that the full 8,330-unit population (1,741 `fixture-verified` + 6,589
`literal-verified`) has no disagreement anywhere.** That population is 0.38% examined (32 of 8,330)
this bundle-wide, and `AT-33-E5-001` (row 16) / `AT-33-E5-002` (row 17) are both correctly
`in-progress`, not `complete` — examining the remaining 8,298 units is their scope, not this
criterion's. `AT-33-E5-003`'s own scope, per its Evidence line, is reacting to disagreements that
`AT-33-E5-001`/`AT-33-E5-002` actually produce, and the currently-produced set is empty.

**The reopening condition is mechanical, not a promise to remember** (`decisions.md §4` — "a lesson
without a mechanism is a quote"): `AT-33-E1-002`'s condition 3 (`oracle_disagreement`) already
makes `scripts/box_ledger.py --check --oracle-results <file>` exit non-zero and name the offending
`unit_id` the moment any future `AT-33-E5-001`/`AT-33-E5-002` cycle lands an oracle-results file
containing even one `"verdict": "disagree"` record. Proven this cycle by mutation (see RED→GREEN
below), not assumed from `AT-33-E1-002`'s own receipt.

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| Units examined by `AT-33-E5-001`+`AT-33-E5-002` to date | 32 | of 8,330 (1,741 `fixture-verified` + 6,589 `literal-verified`) = 0.38% | `python3 -c "import json;a=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment.oracle-results.json'));b=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-literal.oracle-results.json'));print(len(a['results'])+len(b['results']))"` |
| Disagreements among the 32 examined | 0 | of 32 examined | `python3 -c "import json,collections;a=json.load(open('.../equipment.oracle-results.json'));b=json.load(open('.../equipment-literal.oracle-results.json'));print(collections.Counter(r['verdict'] for r in a['results']+b['results']))"` → `Counter({'agree': 32})` |
| `box_ledger.py --check` against the combined 32-record file | `uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False`, exit 0 | population 49,438 (whole inventory, unchanged by this cycle) | `python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json` |
| `progress.md` disagreement-ledger entries required this cycle | 0 | of 0 disagreements found | criterion's own evidence line — one entry per disagreement; 0 disagreements ⇒ 0 entries |
| Mutation proof: injected 1-record `disagree` file | `oracle_disagreement=1`, names `ultimate_equipment:equipment:belt_of_mighty_hurling_greater`, exit 1 | 1 of 1 injected record | see RED→GREEN below; temp file, never committed |

## Status: complete

Every disagreement `AT-33-E5-001`/`AT-33-E5-002` have produced as of this cycle (0 of 32 examined
units) is accounted for: the evidence line's obligation ("one entry per disagreement ... resolved
to a commit or an operator escalation") is satisfied because the set of disagreements requiring an
entry is empty, verified independently rather than assumed. No `## Open blockers` entry is filed —
nothing is blocked; there is genuinely nothing outstanding for this criterion to act on today.

**This status is not permanent-by-default.** It is coupled to a real, already-wired mechanical
check (`box_ledger.py` condition 3), not to a promise that nobody looks again: the moment a future
`AT-33-E5-001`/`AT-33-E5-002` cycle's oracle-results file contains a `disagree` record,
`box_ledger.py --check` against that file exits non-zero by name, which is the trigger for this
criterion's next cycle to add the corresponding `progress.md` entry and root-cause it. This is the
same "checked, not remembered" standard `decisions.md §4`/`workflow-instruction.md §12` row 5
applies to deferral revisit conditions, applied here to a disagreement-count revisit condition.

## Movement, four buckets

- **closure:** 0 — no inventory unit's `status` field changed; this criterion does not move units,
  it resolves disagreements, and none exist to resolve.
- **reclassification:** 0
- **reachability:** 0
- **instrument-correction:** 0 — `box_ledger.py`'s oracle-disagreement gate (`AT-33-E1-002`) and
  `scripts/oracle_harness/run.py` (`AT-33-E2-003`) were used exactly as built and re-verified by
  mutation this cycle; no defect found in either instrument.

## RED→GREEN

This criterion has no disagreement to fix in application code this cycle (0 found), so RED→GREEN
takes the form of a mutation proof against the **detection mechanism** that would trigger this
criterion's real work in a future cycle — the same technique `AT-33-E1-002`'s own five mutation
proofs used to prove `box_ledger.py`'s gates are not merely present but actually fire.

**RED:** a scratch copy of the committed 32-record combined file with one record's `verdict`
mutated to `disagree` (`ours` also changed to a wrong value, `999`, for internal consistency) —
`python3 scripts/box_ledger.py --check --oracle-results /tmp/mutated.json` →
`oracle_disagreement=1`, `ORACLE_DISAGREEMENT: ultimate_equipment:equipment:belt_of_mighty_hurling_greater`,
exit `1`.

**GREEN:** the real, unmutated, committed `AT-33-E5-003.combined-oracle-results.json` —
`python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json` →
`oracle_disagreement=0`, exit `0`. The mutated scratch file was written to `/tmp` only and was never
committed.

## Notes

- **Why "0 disagreements, complete" and not "in-progress" like rows 16/17:** rows 16/17
  (`AT-33-E5-001`/`AT-33-E5-002`) are `in-progress` because their evidence bar is a **fixed,
  known population** (1,741 and 6,589 units respectively) that is mostly unexamined — marking
  those `complete` on a small slice would be the exact false-100% shape `decisions.md §2` forbids.
  This criterion's evidence bar is different in kind: it is defined over **disagreements actually
  produced** by rows 16/17, which is a derived, currently-empty set, not a fixed population with a
  known-larger denominator sitting unexamined. Closing on an empty, mechanically-verified set is
  not the same claim as closing on a small slice of a known-large population.
- **What this cycle does NOT claim:** it does not claim the 8,298 not-yet-examined units of the
  8,330-unit population contain no disagreement — that is unknown and unknowable until
  `AT-33-E5-001`/`AT-33-E5-002` examine them. It claims only that of what has been examined and
  compared against the real oracle so far, nothing disagrees, verified independently this cycle
  rather than trusted from either prior receipt's prose.
- **Considered, rejected:** manufacturing a synthetic "disagreement" against real production code
  (e.g., temporarily reverting a real fix) purely to exercise the fix/escalate machinery end-to-end
  on a genuine defect. Rejected — the scope note is explicit that "a disagreement is never closed
  by adjusting the expectation to match our output," and fabricating a defect to then "resolve" it
  is the same shape of dishonesty in reverse: manufacturing evidence rather than reporting the real
  state. The mutation proof above tests the **detection** mechanism (which is legitimate mutation
  testing, the same pattern `AT-33-E1-002` used) without fabricating a false claim about production
  correctness.
- **Cross-check against `THE-BOX.md`'s own default invocation:** `python3 scripts/box_ledger.py
  --check` (no `--oracle-results` override) resolves its default path to
  `artifacts/epic-2-oracle-harness/oracle-results.json`, which is Epic 2's own demo/fixture
  artifact, not this cycle's real data — confirmed this cycle does not rely on that default; every
  command above passes `--oracle-results` explicitly, pointed at the real committed merge.

## Test scoping

Ran `python3 scripts/box_ledger.py --check --oracle-results <combined file>` (three invocations:
the real committed merge, twice — before and after the mutation-proof pass — plus once against the
mutated scratch copy) and independent `python3 -c` verdict tallies against both source
oracle-results files, all against real committed/derived data. Did not re-run
`scripts/tests/test_box_ledger.py` or `scripts/tests/test_oracle_harness.py` (neither file changed
this cycle — confirmed unmodified via `git status --porcelain` before this cycle's first write).
Did not run the Rust workspace's `cargo test`/`cargo build` (no `src/` file touched this cycle). Did
not run `apps/desktop/src-tauri` (separate cargo workspace, no file in it touched).

Ran `workflow-instruction.md §6` step 2/4 audits against `BASE_BRANCH=$(git merge-base HEAD
origin/develop)` on the final diff, scoped to this criterion's touched paths
(`artifacts/epic-5-reverification/`, `progress.md`, `kanban.md`) under
`docs/release/SD-33-computed-value-verification/`:

```
$ BASE_BRANCH=$(git merge-base HEAD origin/develop)
$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification docs/release/SD-33-computed-value-verification/progress.md docs/release/SD-33-computed-value-verification/kanban.md ':!**/__tests__/**' ':!**/*.test.*' \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
OK_NO_BUNDLE_TAGS
$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification docs/release/SD-33-computed-value-verification/progress.md docs/release/SD-33-computed-value-verification/kanban.md ':!**/__tests__/**' ':!**/*.test.*' \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
OK_NO_TOKENS
```

## Next-cycle plan

Not this criterion's own scope to advance further — its work is fully reactive to
`AT-33-E5-001`/`AT-33-E5-002`'s findings. When a future cycle of either of those criteria lands an
oracle-results file with any `disagree` record (`scripts/box_ledger.py --check` on that file will
exit non-zero and name the unit, per the mutation proof above), the next `AT-33-E5-003` cycle:
(1) root-causes the disagreement — read the engine computation path and the oracle export/template
alongside each other; (2) determines whether the defect is in `codex`'s computation (fix it, in
`src/`, outside this cycle's own write scope — escalate the fix to whichever criterion owns that
file, or take the fix directly if the write scope allows) or in the oracle harness/comparison
(fix `scripts/oracle_harness/`, per `AT-33-E2-003`'s owning criterion, and re-run every prior
oracle-results file through the corrected harness, per this criterion's own Evidence line); (3)
adds one `progress.md` entry per disagreement, resolved to a commit SHA or an explicit operator
escalation — never a filed `## Open blockers` entry alone.
