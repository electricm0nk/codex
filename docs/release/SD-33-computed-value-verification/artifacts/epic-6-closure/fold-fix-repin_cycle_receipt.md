# Cycle fold-fix-repin — epic-6-closure / AT-33-E6-001 (row 19), one-file re-pin

- **Commit SHA:** recorded on landing (see `progress.md` entry `fold-fix-repin`).
- **Files touched:** `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs`,
  `src/bin/v06_work_inventory.rs`, this receipt, `progress.md`, `kanban.md` (row 19),
  `docs/retro/events/sd33-fold-fix-repin.jsonl`.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (own diff against HEAD, both touched files)
- **Wired-integration audit result:** OK_NO_TOKENS (own diff against HEAD; the wider
  `merge-base origin/develop` scoped diff surfaces one PRE-EXISTING `placeholder` match in
  `v06_work_inventory.rs` from an earlier commit this cycle did not touch — confirmed by
  `git diff HEAD -- <file>` showing only the one-line `831 -> 910` change)
- **Acceptance criterion (verbatim):** attempt 11's own SHORTFALL 1 — `cargo test --locked
  --lib` must be green; `f1_population_matches_the_current_true_formula_bearing_count_not_the_stale_sd32_census`
  must assert the live F1 population, not the pre-regen figure.

## Root cause (inherited from attempt 11, independently re-derived here, not taken on trust)

`6e2f2f076b` (fold-skinwalker) correctly re-pinned F1's population 6,278 -> 6,260 against the
`docs/work-inventory.json` committed at that moment. The very next commit, `cef0ca1b39`
(fold-inventory), then **regenerated that same file** (89 of 49,438 units moved status) and did
**not** re-run `cargo test --locked --lib` afterwards — its own receipt's "lib 2845 passed, 0
failed" is a true measurement of the tree *before* its own inventory write, not of the tree it
landed. This is an **ordering bug**, not a further content defect: the fix is not "pick a new
number", it is "run the suite after the last write that can move it."

## STEP 1 — F1 re-derived live, not taken from the receipt

```
$ python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus
  family rollup: F1 6257
```

Cross-checked against the pre-regen inventory the same way attempt 11 did:

```
$ git show 56bbebe3d4:docs/work-inventory.json > <scratch>/work-inventory-56bbebe3d4.json
$ python3 scripts/shape_ledger.py --inventory <scratch>/work-inventory-56bbebe3d4.json --corpus-root data/corpus
  family rollup: F1 6260
```

**Live F1 population: 6,257.** Attempt 11's figure confirmed independently, not copied.

The three named movers were re-confirmed by id-keyed set diff between the two `shape_ledger.py`
`--output` JSON dumps (`rows` filtered to `family == "F1"`), both directions:

```
$ python3 -c "... b_ids - a_ids, a_ids - b_ids over the two --output JSON dumps ..."
before 6260 after 6257
left F1 ( 3 ):
  bestiary_5:race_trait:skinwalker_speed
  ultimate_psionics:equipment_modifier:plusn_svs
  ultimate_psionics:equipment_modifier:special_quality_severis_enhancement_bonus
entered F1 ( 0 ):
```

**three_movers_confirmed: true**, by id, both directions (0 entered, exactly the 3 named left).

## STEP 2 — re-pinned, doc comment updated with it

`src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs:616-617`:
`f1.population, 6257` (was `6260`). The doc comment above the test gained a new paragraph
(`**6,260 -> 6,257 by `cef0ca1b39` ... an ORDERING bug**`) naming the cause, the exact
re-derive command, the three mover ids, and the attribution split (one of the three,
`skinwalker_speed`, is itself fold-attributable per the corrected 50/39 split — see Step 5). The
assertion message's own text was updated in the same edit (6,257 as current, 6,260 named as the
invalidated pre-regen figure with its own cause, the rest of the provenance chain — 6,278 /
6,308 / 6,032 — left untouched, since those are still-accurate historical anchors).

**doc_comment_updated: true.**

## STEP 3 — the ordering rule, made mechanical

Order actually run this cycle, in sequence:

1. `docs/work-inventory.json` — **not written this cycle** (confirmed untouched: `git status
   --porcelain` after every step below shows no change to it; `jq '.units|length'` still
   `49438`).
2. `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root
   data/corpus` — re-derive (Step 1 above).
3. Re-pin `formula_interpreter_corpus_wide.rs:616-617` + doc comment (Step 2 above).
4. Fix the stale prose (Step 4 below).
5. Emit the retro correction (Step 5 below).
6. `cargo test --locked --lib` **run last**, after every source write above — not before. Result:
   `2845 passed; 0 failed; 14 ignored`.

The lesson this cycle exists to make mechanical: **the lib suite runs after the last commit
that can move a pinned figure, never before it.** `cef0ca1b39` broke this by writing
`docs/work-inventory.json` (a figure-moving write) and then reporting a suite result measured
before that write. This cycle's own order avoids the same mistake by writing no
figure-moving file at all — `docs/work-inventory.json` stays untouched, and the suite runs
after the two source edits it needs to validate.

**run_order_stated:** `work-inventory (untouched this cycle, confirmed) -> shape_ledger.py
re-derive -> re-pin (source + doc comment) -> stale-prose fix -> retro correction -> cargo
test --locked --lib (LAST)`.

## STEP 4 — stale prose, re-derived independently, not copied from attempt 11

**`src/bin/v06_work_inventory.rs:4308`** — comment read "every one of the 831
currently-ingested `race_trait` records". Re-derived two independent ways:

```
$ find data/corpus -type d -name "race_trait" | while read d; do find "$d" -name "*.json" | wc -l; done | awk '{s+=$1} END {print s}'
910
```

and the already-passing, already-corpus-walking test
`no_ingested_race_trait_key_contains_a_colon_so_the_storage_namespace_is_lossless`
(`tests/sd27_alternate_racial_trait_reachability.rs`), which asserts `checked == 910` and
passed in this cycle's own `cargo test --locked --lib` / full suite run. **Live value: 910.**
Fixed: `831` -> `910` in the comment.

**`src/rules_core/race_resolver.rs:3105`** — checked, **NOT fixed**, and this is a correction
to attempt 11's own claim, not a confirmation of it. The full sentence is: "SD-33 Epic 6's 45
folded Skinwalker heritage records ...; every one resolves with no inert flag, same as the
other 370." This "370" sits inside
`no_alternate_the_picker_offers_fires_a_flag_that_suppresses_and_grants_nothing`'s own
`checked == 415` assertion message, as the running-total-before-the-final-addend in an additive
derivation chain (`153 + 8 + 1 + 76 + ... + 45 = 415`; algebraically the "other" contribution
before the +45 term **must** be `415 - 45 = 370`, independent of how the earlier terms compose).
Re-derived: `415 - 45 = 370`. The two live assertions this sentence supports
(`assert_eq!(count(TraitRole::Alternate), 415)` at `:2693` and
`assert_eq!(selectable_alternate_trait_keys().len(), 415)` at `:3379`) are already correctly
pinned at `415` and passed in this cycle's own suite run — **not** touched, per the dispatch's
own note that they were "correctly re-pinned". Rewriting "370" to "415" here would make the
sentence **wrong** (415 already includes the 45 being described; "same as the other 415" is
self-referential). **Verdict: not stale. Left unchanged.** This is a false positive in the
prior scan's own RISK 2 finding — recorded, not silently corrected, per `AGENTS.md`'s "re-check
the finding that looks *good*" lesson (here inverted: the finding that looked *bad* on a bare
`grep '370'` match doesn't survive re-derivation).

**stale_prose_fixed:** one entry (`v06_work_inventory.rs:4308`, `831` -> `910`). The
`race_resolver.rs:3105` candidate was checked and found not stale; not counted as a fix.

**figure_sweep_result:** `for n in 831 910 370 415 6278 6260 6257 48634 48699; do grep -rn
"\b$n\b" src/ tests/ apps/ scripts/; done` re-run after all edits landed — every remaining
`831`/`370`/`6278`/`6260` occurrence is either a legitimate `<old> -> <new>` provenance comment,
an unrelated `source_line:`/table value, or (for `370`) the one verified-not-stale running-total
reference above. `48634` and `48699` have zero hardcoded occurrences anywhere in
`src/`/`tests/`/`apps/`/`scripts/` (both are live-computed sweep output, never pinned). No
further stale figure found.

## STEP 5 — attribution correction recorded

`fold-inventory_cycle_receipt.md`'s fold-attribution split of the 89 moved units (14
fold-attributable / 75 drift, via a `'skinwalker' in id` substring test) is not this cycle's to
fix, but attempt 11 already derived the correct split — 50 fold-attributable / 39 drift, since
36 `were*_kin_*`-named `bestiary_5` ids the substring test missed all map to a
`data/corpus/bestiary_5/race_trait/skinwalker/*.json` file the fold created. Logged as a retro
`correction` event this cycle (not merely cited):

```
$ python3 scripts/retro.py correction --subject "fold-inventory_cycle_receipt.md" \
    --claimed "... 14 fold-attributable / 75 drift" \
    --actual "... 50 fold-attributable / 39 drift ..." \
    --verified-by "id-keyed join ... cross-checked against the fold's own new corpus filenames ..." \
    --blast-radius "one of the 3 units that left F1's gate (skinwalker_speed) sits inside the 36 missed" \
    --artifact ".../fold-inventory_cycle_receipt.md" --caught-before "release" \
    --task "SD-33 fold-fix-repin" --actor "sd33-fold-fix-repin"
retro: correction 1787784083169-sd33-fold-fix-repin-9fdd7a -> docs/retro/events/sd33-fold-fix-repin.jsonl
```

**attribution_correction_logged: true.** Epic 5's population is unaffected (independently
re-derived by attempt 11, not re-touched here — no oracle re-run owed).

## RED -> GREEN

RED (attempt 11, reproduced at the top of this cycle before any edit):
```
$ cargo test --locked --lib
test rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::f1_population_matches_the_current_true_formula_bearing_count_not_the_stale_sd32_census ... FAILED
assertion `left == right` failed  (formula_interpreter_corpus_wide.rs:616)
  left: 6257
 right: 6260
test result: FAILED. 2844 passed; 1 failed; 14 ignored
```

GREEN (this cycle, run LAST, after every source edit above, `docs/work-inventory.json`
untouched):
```
$ cargo test --locked --lib
test rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::f1_population_matches_the_current_true_formula_bearing_count_not_the_stale_sd32_census ... ok
test result: ok. 2845 passed; 0 failed; 14 ignored; 0 measured; 0 filtered out; finished in 50.87s
```

## Finish line, run in the stated order

```
$ python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus
  family rollup: F1 6257
$ cargo test --locked --lib
  2845 passed; 0 failed; 14 ignored
$ cargo test --locked --no-run ; echo NO_RUN_EXIT=$?
  NO_RUN_EXIT=0 ; grep -c "Executable tests/" -> 543 ; ls tests/*.rs | wc -l -> 543
$ (cd apps/desktop/src-tauri && cargo test --locked)     # own CARGO_TARGET_DIR
  test result: ok. 548 passed; 0 failed; finished in 141.69s ; DESKTOP_EXIT=0
$ cargo run --locked --bin corpus_literal_sweep
  corpus-literal-sweep: 48699 records examined of 51473 read, 413288 tokens compared
  (9 synthesized), 51460 digests checked, 0 findings
  corpus-literal-sweep: CLEAN ; SWEEP_EXIT=0
$ python3 scripts/denominator_gate.py --check
  files_checked=67  violations=0
```

`docs/work-inventory.json` confirmed untouched throughout: `git status --porcelain` never lists
it; `jq '.units|length' docs/work-inventory.json` -> `49438`, unchanged.

- **Figures + their re-derive commands:**
  - F1 population 6,257 -- `python3 scripts/shape_ledger.py --inventory
    docs/work-inventory.json --corpus-root data/corpus`
  - race_trait ingested total 910 -- `find data/corpus -type d -name race_trait | while read d;
    do find "$d" -name "*.json" | wc -l; done | awk '{s+=$1} END {print s}'` (cross-checked by
    the passing `checked == 910` test)
  - lib suite 2,845 passed / 0 failed -- `cargo test --locked --lib`
  - no-run 543 of 543 -- `cargo test --locked --no-run 2>&1 | grep -c "Executable tests/"`
  - desktop 548 of 548 -- `(cd apps/desktop/src-tauri && cargo test --locked)`
  - corpus sweep 48,699 examined / 0 findings -- `cargo run --locked --bin
    corpus_literal_sweep`
  - denominator gate 0 violations of 67 -- `python3 scripts/denominator_gate.py --check`
- **Status:** complete
- **Movement, four buckets:** closure **1** (row 19's blocking shortfall closed — the lib suite
  is green and stays green after `docs/work-inventory.json`'s already-landed state) /
  reclassification 0 / reachability 0 (a one-file re-pin plus a doc-comment/prose fix; no
  wiring) / instrument-correction **1** (fold-attribution split logged to retro, Step 5)
- **Notes:** The `race_resolver.rs:3105` candidate the dispatch named as stale was checked and
  found NOT stale on re-derivation (`415 - 45 = 370` is exactly what the sentence claims); left
  unchanged rather than "fixed" into a wrong statement. Two long-lived untracked worktrees
  (`worktree-wf_51e862fa-310-1`, `worktree-wf_51e862fa-310-2`) and a set of untracked
  `*.workflow.js` / `docs/release/SD-34-book-completion/` files were present in the shared
  checkout before this cycle started and were not created or touched by it (`git status
  --porcelain` at cycle start, reproduced above the STEP 1 section of this receipt's working
  notes) — left alone per the concurrent-write protocol; not this cycle's scope.
- **Next-cycle plan:** dispatch `AT-33-E6-001` attempt 12 (final-acceptance re-scan) against
  this commit. If green, `release-notes.md` and PR #377 are next; neither was touched by this
  cycle, per the dispatch's own instruction.
