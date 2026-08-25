---
canonical: true
owner: god-emporer
status: FINAL — populated at closure 2026-08-24, all figures re-derived against the final corpus
  state (`kanban.md` row 13 `closure-epilogue`)
date: 2026-08-22
---

# SD-32 Release Notes — Compute Library and Cause Closure

**Closure re-derivation, 2026-08-24 (`closure-epilogue`, kanban row 13):** every figure below has
been re-derived live against the final corpus state and replaces the 2026-08-22 snapshot and the
partial 2026-08-24 correction that preceded it. **Population is 34,416, not the earlier 34,397** —
the 19-unit delta is `beginner_box`, which a code-level carve-out (`EXCLUDED_BOOKS =
{'beginner_box'}`) had kept outside the counted population; removing the carve-out moved
`no_record` `0 → 14` for one instant and then those 14 were ingested, returning it to `0`. Any
figure below whose denominator predates this cycle is marked historical and superseded. See
`docs/retro/sd32-compute-library-and-cause-closure-retrospective.md`'s closure section for the
full account of what changed between the 2026-08-22 snapshot and this final state.

**Populated at closure.** Every figure below carries the command that produces it (and the
corpus SHA when it came from the oracle), per this program's standing convention.

## Origin

Created 2026-08-22 from the SD-31 session handoff (`artifacts/HANDOFF.md`), after the SD-31 wave
31 measurement returned its three findings (ten semantic families; plumbing beats rules
complexity; class reachability is the gate). The chassis was filled out the same day:
README + scope-draft + epic-breakdown + decisions + workflow-instruction + acceptance-and-verification +
risks + forward-scope + technical-requirements + technical-design + release-notes + progress +
kanban + content-unit-inventory + artifacts/ + references/, all from the four source documents the
SD-31 session produced.

The `SD-32` number is the third package to hold it (see `README.md` "Note on the directory name
and the `SD-32` number"). The dead `SD-32-instrument-coverage-and-consumer-wiring/` folder was
cleaned up at the same time the chassis was filled out — no content to recover.

## What closed

**All four gates (Definition of Done, `decisions.md §2`).** Gates G0-G3 each closed on their own
written AT-32-* criteria (`acceptance-and-verification.md`); full per-gate evidence in
`progress.md`'s Cycles 1-9 and `artifacts/gate-{0,1,2,3}-*`.

**Gate 1 — per-family unit counts, re-derived at closure**
(`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json`, sums to the final
34,416-unit population, `unclassified` = 0, `no_record` = 0). `artifacts/gate-1-shape-closure/
ledger.json` is the 2026-08-22 snapshot (24,914-unit population) and is retained only as
historical scaffolding — the table below is the live re-derivation:

| Family | Units | Family | Units |
|---|---:|---|---:|
| F0 (no formula content) | 22,759 | F6 | 391 |
| F1 | 6,308 | F7 | 12 |
| F2 | 2,337 | F8 (residual) | 196 |
| F3 | 671 | F9 | 62 |
| F4 | 1,086 | F10 | 5 |
| F5 | 589 | | |

Sum: 22,759+6,308+2,337+671+1,086+589+391+12+196+62+5 = **34,416**, matching
`shape-coverage-standing-gate`'s `population=34416` exactly.

**Correction (card `family-vocabulary-reconciliation`, `decisions.md §12a`):** `bonus_stack_reader.
rs`'s binding layer targets **F4** ("named-counter/pool variable"), not F10 (a 3-unit
level-threshold step-count family unrelated to the binding layer) — the engine cycles' own "F10"
labelling of the binding-layer figures was a defect, fixed by the canonical vocabulary at
`artifacts/gate-1-shape-closure/family-vocabulary.md`. F4's per-unit ledger count (570) is a
different, narrower denominator than the two figures the engine cycles cite for the same binding
layer: 1,156 distinct `BONUS:VAR` custom-identifier targets (SD-31 wave 31, 77.2%/893 resolved)
and 4,736 distinct target variables found by the corpus-wide binding run (card 8, 3,519
resolved/1,217 refused) — the ledger counts *units carrying* an F4 formula, the engine cycles
count *distinct target-variable identifiers* the binding layer resolves; a fourth, independently
re-derived count (422 distinct F4-shaped bare-identifier strings, 390 resolved, 92.4%) is at
`family-vocabulary.md` §3. None supersedes the others; all four are named here rather than picking
one (`decisions.md §12c`).

**Engines built vs. required — 2 of 2, both run corpus-wide (AT-32-G2-004 met for both):**
`formula_interpreter.rs` (F1-F9, `cargo test --locked --test formula_interpreter_family_fixture_check`
for the 9-family fixture proof; `cargo run --locked --bin formula_interpreter -- --corpus-wide`
for the corpus-wide run — 4,696 of 4,798 F1-F9 units recognised, 97.9%, 102 refused within the
engine's own disclosed proof-width gaps) and the generalised `bonus_stack_reader.rs` (F4's
producer-bound subset, `cargo run --locked --bin bonus_stack_reader -- --corpus-wide` — 3,519 of
4,736 distinct target variables resolved, 1,217 refused).

**Classes reached — mechanism built, population corrected, one half deferred (card 12,
AT-32-E3-001).** New `prestige_class_entry_gate.rs` wires entry-requirement gating at the
`compute_class_chassis` call site (`cargo test --locked prestige_class_entry_gate`, 8 unit + 3
wiring tests). Population corrected 77 → **62** at closure (`scripts/census_prestige_class_entry_requirements.py`
— only classes anchored in an ingested `data/corpus/<book>` with at least one real `PRE`-family
entry-requirement token are covered; 131 `TYPE:...Prestige CLASS:` names exist oracle-wide, most
never ingested into this repo). The 18-untabled-base-class half and the 28 in-books-without-ruleset
figure are **explicitly deferred**, not silently dropped (`kanban.md` #12).

**Books onboarded — 4 of 4 (AT-32-G0-003, AT-32-E4-001 met).** `inner_sea_faiths` (2 spell
entries), `inner_sea_magic` (34), `inner_sea_taverns` (9, feat gap-row lane, no `*_spells.lst`),
`inner_sea_temples` (21) each land their first compiled `RuleSetId` — re-derived arithmetic
3+335+20+64=422 matches `epic-breakdown.md`'s own Epic 4 figure. Verified reaching a player via
`reach_gate.rs` against live `build_spell_catalog()`/`build_feat_catalog()` responses
(`scripts/verify.sh --only reach` → PASS 30/30).

**Epic 5 protective sweep — 29 generators checked (`ls src/bin/{gen_,ingest_,enrich_}*.rs | wc -l`),
7 found vulnerable to the self-erasure shape, 7 fixed** (the 2 SD-31 D9 binaries'
4 vulnerable functions plus 5 more newly found in the "17 never checked" bucket — `gen_cache_acg`,
`gen_cache_apg`, `gen_cache_beastiary` [SD-31's own "safe" verdict corrected], `gen_cache_spell_lane_dump`,
`gen_cache_ultimate_equipment` — protecting up to ~3,100 enriched records). Live RED→GREEN for the
binaries, unit-test RED→GREEN for the library modules.

**Cause closure (Epic 2, card 11) — T1 closed corpus-wide, T5/T3 cited, the rest deferred with
named owners.** See "Deferred findings" below; full reasoning in
`artifacts/gate-3-closure-invariant/epic-2-cause-closure_cycle-1_cycle_receipt.md`.

## Closure figures (2026-08-24, `closure-epilogue`)

- **Population:** 34,416 (`scripts/verify.sh --only shape-coverage-standing-gate` →
  `population=34416 unclassified=0 no_record=0`).
- **Deferrals:** 29 total, 0 open, 29 resolved (`python3 scripts/retro.py summary --since
  2026-08-22 | grep -i DEFERRALS`). `retro.py`'s `deferrals.open` previously read
  `deferrals[-limit:]` — fixed to an unresolved-deferral count, invariant under `--limit`; 19 of
  29 deferrals had never actually been checked under the old code.
- **`declared-pi-audit`:** CLEAN over the full shipped corpus (`cargo run --locked --bin
  declared_pi_shipping_audit`, run in a clean `git worktree` checkout of `tranche/12` to exclude
  concurrent-lane working-tree contamination — see the retrospective's closure section). Previously
  could not reach a verdict (99.9% CPU, 6+ min, no output) because it re-read each cited `.lst`
  file once per citing record (36.7 GB of redundant reads for 72 MB of unique bytes); memoized.
- **`apps/desktop/src-tauri` test suite:** 548 passed, 0 failed (`cargo test --locked --bin
  codex-desktop`, run in a clean checkout — `beginner_box` had been ingested but never added to
  `reach_gate.rs`'s `CORPUS_BOOK_IDS`, and `equipment_catalog.rs`'s pins were stale; both fixed).
- **PI vocabulary:** stands at the `§19`-approved 60 terms, no expansion (`decisions.md §28`,
  operator ruling 2026-08-24).
- **Kanban:** 21 of 22 cards `complete`; row 13 (`closure-epilogue`) is this cycle's own card.
- **Worktree/branch sweep:** 18 `git worktree`s found (17 besides the primary checkout, all with
  tip commits already ancestors of `tranche/12` HEAD — merged by content, `git merge-base
  --is-ancestor <sha> HEAD`), all removed. 148 local `worktree-wf_*` branches found; 144 merged by
  content and deleted, 4 unmerged (SD-31-lane work, out of this bundle's scope) left untouched.
  `sd31/racetrait4-SD31-E6-F4-005` (the SD-31 rescue branch that must not be merged on trust) left
  alone, outside SD-32's lane.

## Deferred findings, each with a named owner — SUPERSEDED, see 2026-08-24 correction above

**Every item this section originally listed as deferred to a successor bundle has since closed
inside SD-32 itself**, once the operator ruled (`decisions.md §10`) that filing under Open
blockers is a request for a ruling, not a disposition. Retained for the historical record of what
was measured, not as a live forward-scope list:

- ~~Epic 2's remaining blocker shapes — T2a/T2b/T9/T4/T12/T7/T8.~~ **Closed.** Mostly via the
  generic verbatim-ingest mechanism `decisions.md §20`/`§17` authorized; T8 under a scoped write
  grant (`decisions.md §11`). `kanban.md` row 11 `complete`, re-verified live this cycle.
- ~~Epic 3's 18 untabled base classes and the 28 in-books-without-ruleset population.~~ **Closed.**
  `kanban.md` row 12 `complete`.
- ~~The class_feature kind-unenumerable finding (27,847 units).~~ **Closed.** Became `kanban.md`
  rows 14/15/17 (`decisions.md §12`, `§27a`, `§27b`), all `complete`; `python3
  scripts/row17_census.py --check` → `ROW 17 HONEST SIZE 0`.

## Known issues

- Two pre-launch planning-prose figures were wrong and are corrected, not silently reconciled:
  the "158-book" oracle claim was **186** books (`python3 scripts/census_independent.py`), and the
  "38,372 units" denominator was **38,391** (`jq '.total_units' docs/work-inventory.json` at Gate
  0's cycle time — this number moves as the inventory regenerates; it is not re-frozen here).
- `docs/work-inventory.json` was **not** regenerated during this bundle's Gate 0/Epic 4 cycles —
  the generator's fail-closed guard (protecting 8,246 verification stamps) correctly refused to
  run without its two upstream report inputs; the four onboarded books' reach is proven via
  `reach_gate.rs`/`verify.sh --only reach` instead. A future cycle running the sweep+fixture-check
  pipeline will pick up the regeneration.
- `release-pipeline.md`'s `check-release-manifest.yml` `paths:` filter (`sd11`/`sd15`/`sd16`/`sd17`
  globs) has gone fully stale — none of the four match a real directory any more. Found and
  documented during this closure's architecture-docs refresh; the workflow YAML itself is
  unchanged (out of `docs/architecture/`'s own write scope).

## Bundle metadata

- **Branch:** `tranche/12`, cut from `tranche/11`'s tip. SD-31's content reached `develop` via PR
  #374 (merged 2026-08-22T19:53Z), verified by content.
- **Build version:** `0.12.0` (`grep -h '"version"' apps/desktop/package.json
  apps/desktop/src-tauri/tauri.conf.json`).
- **`tranche/12 → develop` PR:** PR #375 was opened 2026-08-22 against the rejected closure and is
  **CLOSED** (`decisions.md §10`). The real PR opens once `closure-epilogue` (`kanban.md` row 13)
  itself reaches `complete` — blocked this cycle on the worktree/branch sweep step, see
  `docs/retro/sd32-compute-library-and-cause-closure-retrospective.md`.
- **Merge SHA:** [Populated once the real PR merges — operator approves this merge per standing
  scope]
