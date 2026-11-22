# Encounter-difficulty module (`Encounter::new`) cycle receipt — 2026-07-20T01:50:52Z

## Red-phase evidence

Read first (per Step 4 / `epic-breakdown.md`'s red-green TDD mandate):
`corpus-source-inventory.md` §4 (DM Toolkit routing table + §4.1's five
canonical deterministic test cases), `epic-breakdown.md` criterion 18,
`risks-and-open-questions.md`'s self-healable/non-self-healable rows,
`decisions.md`.

RED test location: an in-file `#[cfg(test)] mod tests` inside
`src/rules_core/encounters.rs` itself, not
`tests/sd22_dm_toolkit_deterministic.rs` — that shared file is reserved by
`loop-instruction.md`'s file-touch partition for Epic 6's **criterion 20**
cycle ("DM-toolkit tests cover both modules' deterministic cases"), a
distinct, later cycle from this one (criterion 18, `Encounter::new` landing
only). This mirrors how APG/ACG per-class cycles wrote their own
class-scoped test file even though the cross-book invariant tests were a
separate, later-numbered criterion.

RED was captured by temporarily stubbing `Encounter::new` to always return
a constant `Difficulty::Medium` / `average_party_level: 0` /
`encounter_level: 0` (intentionally wrong, not `todo!()`, so the test run
demonstrates real assertion failures rather than a panic), then running:

```
$ cargo test --locked --lib rules_core::encounters
running 6 tests
test rules_core::encounters::tests::encounter_1_level_1_pc_vs_1_cr_1_monster_returns_valid_difficulty ... ok
test rules_core::encounters::tests::encounter_empty_monsters_returns_easy ... FAILED
test rules_core::encounters::tests::encounter_handles_extreme_cr_without_panicking ... FAILED
test rules_core::encounters::tests::encounter_of_4_level_3_pcs_vs_4_cr_3_monsters_is_deadly_per_grounded_pf1_math ... FAILED
test rules_core::encounters::tests::xp_table_matches_verified_cr_1_through_10 ... ok
test rules_core::encounters::tests::encounter_of_4_level_3_pcs_vs_1_cr_2_monster_is_easy ... FAILED

failures:
---- rules_core::encounters::tests::encounter_empty_monsters_returns_easy stdout ----
thread '...' panicked at src/rules_core/encounters.rs:262:9:
assertion `left == right` failed
  left: Medium
 right: Easy
---- rules_core::encounters::tests::encounter_handles_extreme_cr_without_panicking stdout ----
assertion `left == right` failed
  left: Medium
 right: Deadly
---- rules_core::encounters::tests::encounter_of_4_level_3_pcs_vs_4_cr_3_monsters_is_deadly_per_grounded_pf1_math stdout ----
assertion `left == right` failed
  left: 0
 right: 7
---- rules_core::encounters::tests::encounter_of_4_level_3_pcs_vs_1_cr_2_monster_is_easy stdout ----
assertion `left == right` failed
  left: Medium
 right: Easy

test result: FAILED. 2 passed; 4 failed; 0 ignored; 0 measured; 136 filtered out
```

Failed for the intended reason: the stub's hard-coded output diverges from
the grounded computation on every case except the two whose expected
answer happened to coincide with the stub's constants (the "any valid
difficulty" case, and the verified static XP table which the stub didn't
touch at all). Confirms the tests genuinely exercise `Encounter::new`'s
real math once implemented.

## Green-phase evidence

Restored the real implementation (`xp_for_cr` / `xp_to_cr` /
`group_encounter_level` / `average_party_level` /
`difficulty_for_el_vs_apl` / `Encounter::new`), then:

```
$ cargo test --locked --lib rules_core::encounters
running 6 tests
test rules_core::encounters::tests::encounter_empty_monsters_returns_easy ... ok
test rules_core::encounters::tests::encounter_handles_extreme_cr_without_panicking ... ok
test rules_core::encounters::tests::encounter_of_4_level_3_pcs_vs_1_cr_2_monster_is_easy ... ok
test rules_core::encounters::tests::encounter_1_level_1_pc_vs_1_cr_1_monster_returns_valid_difficulty ... ok
test rules_core::encounters::tests::encounter_of_4_level_3_pcs_vs_4_cr_3_monsters_is_deadly_per_grounded_pf1_math ... ok
test rules_core::encounters::tests::xp_table_matches_verified_cr_1_through_10 ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 136 filtered out
```

Full suite:

```
$ cargo test --locked 2>&1 | grep -E "FAILED|test result|error\["
(every `test result:` line reads `ok. N passed; 0 failed`; zero `FAILED`
or `error[` lines across the entire run — every pre-existing suite,
including all 6 APG class-chassis suites, both APG shared-table suites,
and all 6 ACG class-chassis suites, is unaffected)
```

```
$ cargo clippy --locked --tests -- -D warnings
    Checking codex v0.1.0 (/home/ubuntu/workspace/repos/codex)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 24.94s
```

(One real clippy finding fixed in-cycle: `clippy::new_ret_no_self` on
`Encounter::new`, since the function returns `EncounterResult` rather than
`Self` — this is `epic-breakdown.md` criterion 18's own literal specified
signature (`Encounter::new(party, monsters) -> EncounterResult`), so the
lint is overridden with a documented `#[allow(clippy::new_ret_no_self)]`
rather than renaming away from the acceptance criterion's own API shape.)

## Source grounding (real PF1 rules, not fabricated)

Verified 2026-07-20 against the public PRD mirror
`legacy.aonprd.com/corerulebook/gamemastering.html` (reachable this
session, unlike the earlier-logged `aonprd.com`/`d20pfsrd.com` 403s from a
different sandbox — see `progress.md`'s Epic 3 Alchemist cycle-1 blocker
for that prior session's network posture):

- **Table: Encounter Design** — Easy = APL−1, Average = APL, Challenging =
  APL+1, Hard = APL+2, Epic = APL+3.
- **Table: CR Equivalencies** — 1 creature = CR, 2 = CR+2, 3 = CR+3,
  4 = CR+4, 6 = CR+5, 8 = CR+6, 12 = CR+7, 16 = CR+8.
- **Table: Experience Point Awards**, CR 1–10 verbatim: 400, 600, 800,
  1,200, 1,600, 2,400, 3,200, 4,800, 6,400, 9,600.

`src/rules_core/encounters.rs`'s module doc comment cites all three tables
and derives the mixed-CR-group XP-summation method, the CR-above-10
doubling-every-2-CR extrapolation (a direct continuation of the verified
CR 1-10 progression, flagged as not independently re-verified above CR 10),
and the 5-tier-to-4-tier `Difficulty` collapse this bundle's own
acceptance criteria require.

**A documented discrepancy, not a fabrication-to-force-green:**
`corpus-source-inventory.md` §4.1 case 2 ("4 level-3 PCs vs 4 CR-3
monsters") states an expected result of `Hard`. This module's
grounded-and-cited formula computes `Deadly` (APL 3, group EL 7 — via
4×800 XP = 3,200 XP = the CR-7 threshold — EL−APL = +4, beyond even Epic's
APL+3 on Table: Encounter Design). Rather than bend the formula to match
an unverified fixture-table entry, this cycle followed the sourced
rulebook math and flagged the discrepancy — mirroring this bundle's own
established precedent for planning-doc content later found wrong against a
real source (the Gunslinger/Magus APG-roster correction, the ACG
Alchemist-roster correction, both in `progress.md`'s `## Open blockers` /
cycle log). Left for criterion 20's dedicated deterministic-test cycle (or
an operator/doc-correction pass) to reconcile: either re-verify against a
second independent source, or correct §4.1's case-2 entry. Not treated as
a hard stop for this cycle, since criterion 18 (`Encounter::new` landing)
does not itself require all five §4.1 cases to reconcile — that is
criterion 20's job.

## Files touched

- `src/rules_core/encounters.rs` — NEW. `CharacterSnapshot`, `MonsterRef`,
  `Difficulty`, `EncounterResult`, `Encounter::new`, and the grounded
  XP/CR/EL helper functions, plus the in-file deterministic unit tests.
- `src/rules_core/mod.rs` — added `pub mod encounters;`.

## Cycle metadata

- cycle_id: 2026-07-20T01:50:52Z
- duration: ~45 minutes (includes source-verification research against the
  public PRD mirror before writing any code)
- bundle_criterion: criterion-18
- corpus_input_path: N/A — this is not a PCGen `.lst` ingest cycle (Epic 6
  doesn't parse book content directly; it computes over `CharacterSnapshot`/
  `MonsterRef` values). Grounding source instead:
  `legacy.aonprd.com/corerulebook/gamemastering.html` (Pathfinder RPG Core
  Rulebook, "Gamemastering" chapter — Table: Encounter Design, Table: CR
  Equivalencies, Table: Experience Point Awards), verified 2026-07-20.
- RuleSetId: N/A (DM Toolkit is not book content; no `RuleSetId` variant
  applies to `encounters.rs`)
- ingest_pipeline_version: N/A (not an ingest cycle)

## kanban

- card: see `docs/release/SD-22/progress.md`'s cycle log for this cycle;
  backfilled once the real card ID / commit SHA are known post-commit (same
  pattern as prior cycles' `hermes kanban` mint + backfill).
- audit_comment: n/a
