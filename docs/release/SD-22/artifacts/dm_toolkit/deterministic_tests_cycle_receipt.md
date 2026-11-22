# DM-toolkit deterministic tests (criterion 20) cycle receipt — 2026-07-20T03:17:34Z

## Red-phase evidence

Read first (per Step 4): `corpus-source-inventory.md` §4 (DM Toolkit
routing + §4.1's five canonical deterministic test cases),
`epic-breakdown.md` criterion 20's exact wording, `risks-and-open-questions.md`,
`decisions.md`, and both prior cycles' receipts
(`artifacts/dm_toolkit/encounters_cycle_receipt.md`,
`artifacts/dm_toolkit/party_cr_cycle_receipt.md`) — both of which
independently found `corpus-source-inventory.md` §4.1's stated expected
values for case 2 ("Hard") and case 3 ("~3.5") didn't hold up against the
real PF1 Core Rulebook and flagged the reconciliation for this cycle,
rather than force-fitting either module's already-shipped code.

**RED evidence 1 — the acceptance-level test target didn't exist yet.**
`tests/sd22_dm_toolkit_deterministic.rs` is reserved by
`loop-instruction.md`'s file-touch partition for this criterion's cycle;
it had not been created by criteria 18 or 19 (each of those cycles put its
own deterministic tests in an in-file `#[cfg(test)] mod tests` instead, by
explicit design — see both prior receipts). Confirmed by running the test
command against the pre-cycle tree:

```
$ cargo test --locked --test sd22_dm_toolkit_deterministic 2>&1 | head -10
error: no test target named `sd22_dm_toolkit_deterministic` in default-run packages
help: available test targets:
    character_hub_list_saved_characters
    character_input_record
    ...
```

Failed for the intended reason: the file (and therefore the cargo test
target) genuinely did not exist yet.

**RED evidence 2 — the *original* (uncorrected) §4.1 fixture values
genuinely fail against the already-correct, already-shipped code.** Before
committing to the correction, this cycle wrote a throwaway scratch test
(not committed; removed after capture) asserting the ORIGINAL §4.1
case-2/case-3 expected values (`Hard`, `3.5`) against `Encounter::new` and
`party_challenge_rating` as they already ship on `tranche/5`:

```
$ cargo test --locked --test sd22_dm_toolkit_deterministic_prefix_check_scratch 2>&1 | tail -20
running 2 tests
test original_stated_hard_expectation_fails_against_grounded_code ... FAILED
test original_stated_3_5_expectation_fails_against_grounded_code ... FAILED

failures:

---- original_stated_hard_expectation_fails_against_grounded_code stdout ----
thread '...' panicked at ...:13:5:
assertion `left == right` failed: original §4.1 case 2 expectation
  left: Deadly
 right: Hard

---- original_stated_3_5_expectation_fails_against_grounded_code stdout ----
thread '...' panicked at ...:20:5:
assertion `left == right` failed: original §4.1 case 3 expectation
  left: 3.0
 right: 3.5

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
```

Failed for the intended reason: the already-shipped production code
computes `Deadly`/`3.0` (matching both prior cycles' own grounded
derivations); the stale doc values (`Hard`/`3.5`) are what's wrong, not the
code. This is the acceptance-level confirmation that criterion 20's job is
a fixture-doc correction, not a code fix — done independently of the two
prior cycles' own claims, per this cycle's explicit brief to re-verify
rather than trust.

## Independent re-verification of the two discrepancies (this cycle's own work)

Re-fetched `https://legacy.aonprd.com/corerulebook/gamemastering.html`
fresh this cycle (not reusing the prior cycles' cached citations) and read
the tables directly:

- **Table: Encounter Design**: "Easy | APL –1 | Average | APL | Challenging
  | APL +1 | Hard | APL +2 | Epic | APL +3" — matches both prior receipts'
  citations exactly.
- **Table: CR Equivalencies**: "1 Creature | CR | 2 Creatures | CR +2 |
  3 Creatures | CR +3 | 4 Creatures | CR +4 | 6 Creatures | CR +5 |
  8 Creatures | CR +6 | 12 Creatures | CR +7 | 16 Creatures | CR +8" —
  matches.
- **Table: Experience Point Awards**, CR 1-10: 400, 600, 800, 1,200, 1,600,
  2,400, 3,200, 4,800, 6,400, 9,600 — matches.
- **Step 1 — Determine APL**: "You should round this value to the nearest
  whole number (this is one of the few exceptions to the round down
  rule)... If your group contains six or more players, add one to their
  average level. If your group contains three or fewer players, subtract
  one from their average level." — matches. No literal "Determining Party
  Strength" heading exists on the page (confirmed by direct search); "Step
  1 — Determine APL" is the only matching rule, as both prior cycles also
  found.

**Case 2 re-derivation** (4 level-3 PCs vs. 4 CR-3 monsters): APL = 3.
Four creatures of the same CR combine to `CR + 4` per Table: CR
Equivalencies directly: `3 + 4 = 7`. Cross-checked via the independent
XP-summation method: 4 × 800 XP (CR-3's per-creature award) = 3,200 XP,
which is exactly the CR-7 threshold on Table: Experience Point Awards — both
methods agree on EL 7. `EL − APL = 7 − 3 = +4`, beyond even `Epic` (APL+3)
on Table: Encounter Design, i.e. `Difficulty::Deadly` under this bundle's
4-tier collapse. §4.1's original "Hard" is wrong; **Deadly is correct.**

**Case 3 re-derivation** (party CR of 4 level-3 PCs): 4 PCs sits inside the
rulebook's unadjusted "four or five PCs" band — no size adjustment.
Average level = (3+3+3+3)/4 = 3.0 exactly, already a whole number. The
rulebook's APL rule has no step (round-to-nearest, then a flat ±1 party-size
adjustment) that can ever produce a non-integer result for any input.
§4.1's original "~3.5" is wrong; **3.0 is correct.**

Both re-derivations independently confirm the exact conclusions the
criterion-18 and criterion-19 cycles each reached on their own — this
cycle did not merely copy their receipts' claims forward; it re-fetched
the source and re-derived both cases from the raw table text itself before
writing any test assertion or doc correction.

## Green-phase evidence

No bug was found in `src/rules_core/encounters.rs` or
`src/rules_core/party_cr.rs` — both were already grounded correctly by
their own respective cycles. This cycle's GREEN work is (1) the new
acceptance-level test file with the corrected expected values, and (2) the
`corpus-source-inventory.md` §4.1 fixture-table correction. No production
code in either module changed.

```
$ cargo test --locked --test sd22_dm_toolkit_deterministic 2>&1 | tail -15
running 5 tests
test encounters_1_level_1_pc_vs_1_cr_1_monster_returns_valid_difficulty ... ok
test encounters_empty_monsters_returns_easy ... ok
test encounters_4_level_3_pcs_vs_4_cr_3_monsters_is_deadly ... ok
test party_cr_of_4_level_3_pcs_equals_3 ... ok
test encounters_4_level_3_pcs_vs_1_cr_2_monster_is_easy ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```
$ cargo test --locked 2>&1 | grep -c "test result: ok"
415
$ cargo test --locked 2>&1 | grep -E "FAILED|error\[|[1-9][0-9]* failed"
(no output — zero failures across all 415 `test result:` blocks; every
pre-existing suite, including the sibling stream's in-flight ACG
Swashbuckler work present uncommitted in the shared working tree, all six
APG class-chassis suites, both APG shared-table suites, all nine ACG
class-chassis suites landed so far, and criterion 18/19's own `encounters`
and `party_cr` in-file suites, is unaffected)
```

```
$ cargo clippy --locked --tests -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
```

(Clean — no findings.)

## Files touched

- `tests/sd22_dm_toolkit_deterministic.rs` — NEW. Five acceptance-level
  deterministic tests covering both `encounters.rs` (criterion 18) and
  `party_cr.rs` (criterion 19) against the corrected, independently
  re-verified §4.1 fixture values.
- `docs/release/SD-22/corpus-source-inventory.md` — §4.1's fixture table
  corrected: case 2's expected value from "Hard" to "Deadly", case 3's from
  "~3.5" to "3.0" (fixture-slug names updated to match:
  `encounters_4_level_3_pcs_vs_4_cr_3_monsters_is_deadly`,
  `party_cr_of_4_level_3_pcs_equals_3`), with a corrective banner note
  above §4.1 explaining the correction and citing both cycles' evidence.

## Cycle metadata

- cycle_id: 2026-07-20T03:17:34Z
- duration: ~40 minutes (includes independent source re-verification
  against the public PRD mirror, plus the throwaway scratch-test RED
  capture, before touching either doc file)
- bundle_criterion: criterion-20
- corpus_input_path: N/A — this is not a PCGen `.lst` ingest cycle (DM
  Toolkit doesn't parse book content directly). Grounding source:
  `legacy.aonprd.com/corerulebook/gamemastering.html` (Pathfinder RPG Core
  Rulebook, "Gamemastering" chapter), re-verified fresh 2026-07-20 by this
  cycle independently of the two prior cycles' own citations.
- RuleSetId: N/A (DM Toolkit is not book content; no `RuleSetId` variant
  applies to `encounters.rs`/`party_cr.rs`)
- ingest_pipeline_version: N/A (not an ingest cycle)

## kanban

- card: see `docs/release/SD-22/progress.md`'s cycle log for this cycle;
  backfilled once the real card ID / commit SHA are known post-commit
  (same pattern as prior cycles' `hermes kanban` mint + backfill).
- audit_comment: n/a
