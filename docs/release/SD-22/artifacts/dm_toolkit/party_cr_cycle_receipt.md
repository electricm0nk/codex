# Party-challenge-rating module (`party_challenge_rating`) cycle receipt — 2026-07-20T03:00:00Z

## Red-phase evidence

Read first (per Step 4 / `epic-breakdown.md`'s red-green TDD mandate):
`corpus-source-inventory.md` §4 (DM Toolkit routing table + §4.1's five
canonical deterministic test cases, specifically case 3 -- "party CR of 4
level-3 PCs -- CR ~3.5"), `epic-breakdown.md` criterion 19's exact wording,
`risks-and-open-questions.md`'s self-healable/non-self-healable rows,
`decisions.md`, and `artifacts/dm_toolkit/encounters_cycle_receipt.md`
(criterion 18's own documented discrepancy against the same §4.1 fixture
table, followed as precedent by this cycle).

RED test location: an in-file `#[cfg(test)] mod tests` inside
`src/rules_core/party_cr.rs` itself, mirroring criterion 18's own choice --
`tests/sd22_dm_toolkit_deterministic.rs` is reserved by
`loop-instruction.md`'s file-touch partition for Epic 6's **criterion 20**
cycle ("DM-toolkit tests cover both modules' deterministic cases"), a
separate, later cycle from this one (criterion 19, `party_cr.rs` landing
only).

RED was captured by temporarily stubbing `party_challenge_rating` to
always return a constant `99.0` (intentionally wrong, not `todo!()`, so the
test run demonstrates real assertion failures rather than a panic), then
running:

```
$ cargo test --locked --lib rules_core::party_cr
running 6 tests
test rules_core::party_cr::tests::party_cr_applies_no_adjustment_for_five_players ... FAILED
test rules_core::party_cr::tests::party_cr_applies_plus_one_adjustment_for_six_or_more_players ... FAILED
test rules_core::party_cr::tests::party_cr_applies_minus_one_adjustment_for_three_or_fewer_players ... FAILED
test rules_core::party_cr::tests::party_cr_of_1_level_1_pc_applies_minus_one_adjustment ... FAILED
test rules_core::party_cr::tests::party_cr_of_4_level_3_pcs_is_3_per_grounded_pf1_apl_rule ... FAILED
test rules_core::party_cr::tests::party_cr_of_empty_party_is_zero ... FAILED

failures:
---- rules_core::party_cr::tests::party_cr_of_4_level_3_pcs_is_3_per_grounded_pf1_apl_rule stdout ----
assertion `left == right` failed
  left: 99.0
 right: 3.0
(...five more assertion failures of the identical shape, one per test case,
each comparing the 99.0 stub against that case's grounded expected value...)

test result: FAILED. 0 passed; 6 failed; 0 ignored; 0 measured; 142 filtered out; finished in 0.00s
```

Failed for the intended reason: every test's expected value differs from
the stub's hard-coded `99.0` constant, so all six fail on real assertion
mismatches, confirming the tests genuinely exercise
`party_challenge_rating`'s real math once implemented.

## Green-phase evidence

Restored the real implementation (sum levels / party size, round to
nearest whole number, then the rulebook's party-size adjustment: +1 for
six-or-more players, -1 for three-or-fewer), then:

```
$ cargo test --locked --lib rules_core::party_cr
running 6 tests
test rules_core::party_cr::tests::party_cr_applies_no_adjustment_for_five_players ... ok
test rules_core::party_cr::tests::party_cr_applies_minus_one_adjustment_for_three_or_fewer_players ... ok
test rules_core::party_cr::tests::party_cr_of_1_level_1_pc_applies_minus_one_adjustment ... ok
test rules_core::party_cr::tests::party_cr_applies_plus_one_adjustment_for_six_or_more_players ... ok
test rules_core::party_cr::tests::party_cr_of_4_level_3_pcs_is_3_per_grounded_pf1_apl_rule ... ok
test rules_core::party_cr::tests::party_cr_of_empty_party_is_zero ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 142 filtered out; finished in 0.00s
```

Full suite (run concurrently with a sibling stream landing Epic 4's Slayer
class in the same working tree -- this cycle's own `git add` is scoped
strictly to its own files, per `loop-instruction.md`'s parallel-safety
procedure):

```
$ cargo test --locked 2>&1 | grep -c "test result: ok"
413
$ cargo test --locked 2>&1 | grep -E "FAILED|error\[|[1-9][0-9]* failed"
(no output -- zero failures across all 413 `test result:` blocks; every
pre-existing suite, including all six APG class-chassis suites, both APG
shared-table suites, all seven ACG class-chassis suites landed so far, and
criterion 18's own `encounters` suite, is unaffected)
```

```
$ cargo clippy --locked --tests -- -D warnings
    Checking codex v0.1.0 (/home/ubuntu/workspace/repos/codex)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 26.74s
```

(Clean -- no findings, no `#[allow(...)]` needed for this module.)

## Source grounding (real PF1 rules, not fabricated)

Verified 2026-07-20 against the public PRD mirror
`legacy.aonprd.com/corerulebook/gamemastering.html` (`WebFetch`, this
session): the rulebook has no heading literally titled "Determining Party
Strength" or "Party Strength" (confirmed by a direct targeted search of the
page for that heading text) -- the closest, and only, matching rule is
Gamemastering -> "Designing Encounters" -> "Step 1 -- Determine APL", which
`epic-breakdown.md` criterion 19's phrase evidently paraphrases. Quoted
verbatim:

- "Determine the average level of your player characters -- this is their
  Average Party Level (APL for short). You should round this value to the
  nearest whole number."
- "Note that these encounter creation guidelines assume a group of four or
  five PCs. If your group contains six or more players, add one to their
  average level. If your group contains three or fewer players, subtract
  one from their average level."
- Worked example, quoted verbatim and used directly as one of this cycle's
  test cases (`party_cr_applies_plus_one_adjustment_for_six_or_more_players`):
  "if your group consists of six players, two of which are 4th level and
  four of which are 5th level, their APL is 6th (28 total levels, divided
  by six players, rounding up, and adding one to the final result)."

`src/rules_core/party_cr.rs`'s module doc comment cites this rule text and
derives the party-size-adjustment logic directly from it. This module
reuses `encounters.rs`'s existing `CharacterSnapshot` type rather than
redefining an equivalent one (both criterion 18 and criterion 19 operate on
the same input shape per `corpus-source-inventory.md` §4's table).

**A documented discrepancy, not a fabrication-to-force-green:**
`corpus-source-inventory.md` §4.1 case 3 ("party CR of 4 level-3 PCs")
states an expected result of `~3.5`. This module's grounded-and-cited
formula computes `3.0` instead: 4 PCs falls inside the rulebook's
unadjusted "four or five PCs" band (no size adjustment applies); average
level = 12/4 = 3.0 exactly; rounded to the nearest whole number is `3`.
The verified rule has no step that can produce a fractional (`.5`) result
for any input -- APL is defined to always round to a whole number. Rather
than bend the formula to match an unverified fixture-table entry, this
cycle followed the sourced rulebook math and flagged the discrepancy --
mirroring this bundle's own established precedent (the Gunslinger/Magus
APG-roster correction, the ACG Alchemist-roster correction, and
criterion 18's own "Hard" vs. grounded-"Deadly" discrepancy against this
identical §4.1 fixture table, recorded in
`artifacts/dm_toolkit/encounters_cycle_receipt.md`). Left for criterion 20's
dedicated deterministic-test cycle (or an operator/doc-correction pass) to
reconcile: either re-verify against a second independent source, or correct
§4.1's case-3 entry -- the same recommendation criterion 18's cycle made for
case 2, now applying to a second case in the same table.

## Files touched

- `src/rules_core/party_cr.rs` -- NEW. `party_challenge_rating`, the grounded
  APL + party-size-adjustment helper, and the in-file deterministic unit
  tests. Imports `CharacterSnapshot` from `encounters.rs` rather than
  redefining it.
- `src/rules_core/mod.rs` -- added `pub mod party_cr;` (alphabetically
  ordered, between `level_up` and `pilot_compute`).

## Cycle metadata

- cycle_id: 2026-07-20T03:00:00Z
- duration: ~35 minutes (includes source-verification research against the
  public PRD mirror before writing any code)
- bundle_criterion: criterion-19
- corpus_input_path: N/A -- this is not a PCGen `.lst` ingest cycle (Epic 6
  doesn't parse book content directly; it computes over `CharacterSnapshot`
  values). Grounding source instead:
  `legacy.aonprd.com/corerulebook/gamemastering.html` (Pathfinder RPG Core
  Rulebook, "Gamemastering" chapter -- Designing Encounters -> Step 1 --
  Determine APL), verified 2026-07-20.
- RuleSetId: N/A (DM Toolkit is not book content; no `RuleSetId` variant
  applies to `party_cr.rs`)
- ingest_pipeline_version: N/A (not an ingest cycle)

## kanban

- card: see `docs/release/SD-22/progress.md`'s cycle log for this cycle;
  backfilled once the real card ID / commit SHA are known post-commit (same
  pattern as prior cycles' `hermes kanban` mint + backfill).
- audit_comment: n/a
