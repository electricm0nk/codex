# Cycle epic2-2.5-pilot_case_verification_followup — Epic 2 Oracle-Harness Comparator / Criterion 2.5 (followup)

- **Card ID:** receipt-only (this followup cycle did not mint a new hermes kanban card; it completes the same criterion 2.5 the original card `t_b7bb32bf` already closed as a real, structural blocker)
- **Commit SHA:** (see this cycle's push to `tranche/5-4`)
- **Files touched (this repo):**
  - `tests/sd26_pilot_case_verification.rs` (rewritten — now points at the completed real pilot `.pcg` instead of the substitute `pf_Paladin.pcg`; asserts the real 7/9-match, 2/9-mismatch outcome)
  - `artifacts/oracle_validation/parity_report_pf1-crb-human-fighter-level1.md` (regenerated — real new pipeline output)
  - `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/artifacts/epic_2/pilot_case_oracle_checked-followup-cycle_receipt.md` (this file)
  - `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/progress.md`
- **Files touched (outside this repo — GE-05 artifact tree, not `src/oracle_validation/`'s file-touch scope, but the completed `.pcg` itself, not code):**
  - `/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg` (completed in place — see "What was completed" below)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (diff-scoped dual-audit gate, `BASE_BRANCH...HEAD`)
- **Wired-integration audit result:** `OK_NO_TOKENS` (diff-scoped dual-audit gate, `BASE_BRANCH...HEAD`)
- **Acceptance criterion:** same as the original 2.5 cycle — "Criterion 2.5 — Verification cycle for the pilot case. Runs the comparator against the pilot case; upgrades `current_claim_status` from `not_yet_grounded` to `oracle_checked`." **Explicitly conditioned on the comparator run genuinely confirming parity; not to be forced.**
- **Status:** complete (real `.pcg` blocker resolved; pipeline re-run for real; upgrade correctly still withheld — real, different, root-caused blocker found instead — see Notes)

## What was already true (found, not fabricated)

A prior GE-05 cycle (`Epic 1 Criterion GE05-E1-F2`, receipt `ge05-e1-f2-runtime-output-attempt-3-2026-06-20.md`) had already produced a real, load-bearing, near-complete `.pcg` file — `pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg` — proven to load in the real PCGen engine (`BUILD SUCCESSFUL in 21s`, exit 0, real XML output whose SHA-256 matches `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt`'s `legacy_raw_output_sha256` field verbatim). It already correctly carried:

- `CAMPAIGN:Core Rulebook`, `GAMEMODE:Pathfinder_RPG`
- `RACE:Human`, `CLASS:Fighter|LEVEL:1`
- Ability scores `STR:16 / DEX:14 / CON:14 / INT:10 / WIS:12 / CHA:8`
- The Human `+2 Strength` ability bonus (`ABILITY:Ability Bonus|...|KEY:+2 Strength`)
- The Power Attack feat

Its own receipt honestly documented what was still missing (its "Provisional Assumptions Still Present" table): no equipment loadout, no skill-rank allocation, no additional feats beyond Power Attack.

## What this cycle completed

Read the exact deterministic input contract at `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt` and completed the `.pcg` to match it exactly. Confirmed every KEY name against the real Core Rulebook LST source at `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/` (never guessed) and the real `.pcg` tag syntax against `~/workspace/repos/pcgen/code/testsuite/PCGfiles/{pf_Cleric.pcg, pf_Paladin.pcg, 35e_L10(fighter-human).pcg}`:

| Addition | Real `.pcg` tag added | Source confirming the exact KEY/syntax |
|---|---|---|
| Dodge (human bonus feat) | `ABILITY:FEAT|TYPE:NORMAL|CATEGORY:FEAT|KEY:Dodge|TYPE:Combat|DESC:...` | `cr_feats.lst:53` (`KEY:` bare `Dodge`, `CATEGORY:FEAT`, `TYPE:Combat`) |
| Weapon Focus (Longsword) (fighter bonus feat) | `ABILITY:FEAT|TYPE:NORMAL|CATEGORY:FEAT|KEY:Weapon Focus|APPLIEDTO:Longsword|TYPE:Combat.WeaponFocus|DESC:...` | `cr_feats.lst:184` (`KEY:Weapon Focus`, `CHOOSE:WEAPONPROFICIENCY`); `APPLIEDTO:<weapon>` subchoice syntax confirmed against `pf_Paladin.pcg:130`'s real `Weapon Focus\|APPLIEDTO:Flail (Heavy)` line |
| Climb, Intimidate, Swim (1 rank each) | `SKILL:Climb\|OUTPUTORDER:1\|CLASSBOUGHT:[CLASS:Fighter\|RANKS:1.0\|COST:1\|CLASSSKILL:Y]` (and same shape for Intimidate/Swim) | `cr_skills.lst` (`Climb`/`Intimidate`/`Swim` skill definitions); `CLASSBOUGHT:`/`CLASSSKILL:Y` syntax and Fighter-class-skill status for exactly these three confirmed against `35e_L10(fighter-human).pcg:74,84,93` (a real bundled Fighter fixture using the identical shape) |
| Chain Shirt (worn/equipped, active) | `EQUIPNAME:Chain Shirt\|OUTPUTORDER:1\|COST:100\|WT:25.0\|QUANTITY:1.0\|CUSTOMIZATION:[BASEITEM:Chain Shirt\|DATA:EQMOD=STEEL]` + `EQUIPSET:Armor\|ID:0.1.1\|VALUE:Chain Shirt\|QUANTITY:1.0\|USETEMPMODS:Y` | `cr_equip_arms_armor.lst:40,53` (`KEY:Chain Shirt`, base `EQMOD:Material ~ Steel`); `EQUIPNAME:`/`EQUIPSET:Armor` slot syntax confirmed against `pf_Cleric.pcg`'s real `Scale Mail` armor entry (identical `CUSTOMIZATION:[BASEITEM:...\|DATA:EQMOD=STEEL]` shape) |
| Longsword (primary weapon, equipped, active) | `EQUIPNAME:Longsword\|OUTPUTORDER:2\|COST:15\|WT:4.0\|QUANTITY:1.0\|CUSTOMIZATION:[BASEITEM:Longsword\|DATA:EQMOD=STEEL]` + `EQUIPSET:Primary Hand\|ID:0.1.2\|VALUE:Longsword\|QUANTITY:1.0\|USETEMPMODS:Y` | `cr_equip_arms_armor.lst:165,223` (`KEY:Longsword`, base `EQMOD:Material ~ Steel`); `Primary Hand` one-handed-weapon slot confirmed against `pf_Cleric.pcg`'s real Masterwork Morningstar entry |
| Shield | (deliberately absent — no `EQUIPSET:Shield` line added) | matches the deterministic input contract's explicit `equipment=item:shield:absent` |

Also renamed `CHARACTERNAME` from `pf1-crb-human-fighter-level1-provisional-ge05-e1-f2` to exactly `pf1-crb-human-fighter-level1` (the pilot `case_id`). This is not a game-mechanic edit; it makes the file's identity field literally state which pilot case it embodies, which matters because `scripts/pcgen-normalize-output.py`'s `character.identity` dimension reads PCGen's `<basics><name>` (mapped 1:1 from `CHARACTERNAME` by `base-xml.ftl:35`) and compares it directly against Codex's own `character.identity` value, which is literally the `case_id` string (`selected_parity_dimensions.rs::from_receipt`). The prior cycle's own receipt independently confirms this is exactly how the dimension is designed to work: its substitute run's `character.identity` mismatch was PCGen's raw character name ("Florian Syrkov") vs. Codex's `case_id` string.

## Validation (real PCGen engine, real output inspected)

Ran `scripts/pcgen-run-character.sh -c <completed .pcg>` twice (once before, once after the `CHARACTERNAME` rename) — both real Gradle/PCGen invocations, ~21s each, `BUILD SUCCESSFUL`, exit 0. The only LST warnings emitted (`Illegal FACT subtoken 'IsOrc'...`, `PRETYPE has been deprecated...`) are pre-existing, unrelated to this character (confirmed by grepping the same warnings out of the file's own prior `ge05-e1-f2-runtime-output-attempt-3-2026-06-20.md` receipt) — not new errors introduced by this cycle's edits.

Inspected the real produced XML directly (not just the exit code) to confirm every added element was genuinely understood by PCGen, not merely tolerated:

- `armor_class`: `total 17`, `listing: ABILITY +2, ARMOR +4, BASE +10, DODGE +1`, `dodge_bonus: 1`, `shield_bonus: 0` — Dodge and the Chain Shirt's armor bonus both applied; shield bonus genuinely 0 (no shield).
- `abilities`: `<name>Dodge</name>` present; `<name>Weapon Focus (Longsword)</name>` present with `<associated>Longsword</associated>`.
- `skills`: `Climb`/`Intimidate`/`Swim` each show `<ranks>1.0</ranks>`; `list_mods: Climb +6, Intimidate +3, Swim +6`.
- `equipment`: `Chain Shirt` and `Longsword` both listed as real equipped items (`<name>Chain Shirt</name>`, `<name>Longsword</name>` under the equipped-item nodes, and `*Longsword` in the weapon-attack block denoting the actively wielded weapon).

## Pipeline re-run (real, end to end)

Updated `tests/sd26_pilot_case_verification.rs`'s `substitute_pcg_fixture()` (renamed `pilot_case_pcg_fixture()`) to point at the completed file's real absolute path outside this repo — the same pattern `PcgenRunOptions::pcgen_repo_dir` already uses for the out-of-repo PCGen checkout; `pcgen_runner.rs::run_pcgen_character` takes any real absolute `.pcg` `Path`, so no in-repo copy/move was needed or made. Left the file at its current GE-05 artifact-tree location per the task brief.

Ran the real pipeline (`rules_core::pilot_compute::build_pilot_headless_receipt` → `SelectedParityDimensions::from_receipt` on the Codex side; `pcgen_runner::run_pcgen_character` → `comparator::compare` → `parity_report::write_parity_report` on the PCGen side, exactly as Criteria 2.1-2.4 built it). **Genuine finding: 7 of 9 dimensions match, 2 genuinely mismatch:**

```text
| Dimension | PCGen | Codex | Match |
|---|---|---|---|
| character.identity | pf1-crb-human-fighter-level1 | pf1-crb-human-fighter-level1 | yes |
| combat.baseline_melee_attack_bonus | 5 | 5 | yes |
| defense.baseline_armor_class | 17 | 17 | yes |
| defense.total_save.fortitude | 4 | 4 | yes |
| defense.total_save.reflex | 2 | 2 | yes |
| defense.total_save.will | 1 | 1 | yes |
| skill.selected_modifier.intimidate | 3 | 3 | yes |
| skill.selected_modifier.climb | 6 | 5 | no |
| skill.selected_modifier.swim | 6 | 5 | no |
```

## Root-caused the two real mismatches — a genuine Codex bug, not a fixture or pipeline defect

Traced both mismatches to one root cause in `src/rules_core/pilot_compute.rs`. Probed the loaded receipt's own `computation.explanations` (temporary local instrumentation, discarded, not committed) to read the exact arithmetic:

```text
skill.selected_modifier.climb: rank 1 + Strength modifier (+3) + class-skill bonus (+3) + Chain Shirt armor-check penalty (-2) = 5
skill.selected_modifier.swim:  rank 1 + Strength modifier (+3) + class-skill bonus (+3) + Chain Shirt armor-check penalty (-2) = 5
```

`ability_modifiers.strength` is `+3` — the modifier for the *raw* STR score 16, not the *effective* (Human `+2` bonus-applied) score 18 (which would give `+4`, matching PCGen's real Climb/Swim total of `1 + 4 + 3 - 2 = 6`). `compute_ability_modifiers` (pilot_compute.rs) computes `AbilityModifiers` directly from `input.chosen.ability_scores` — the chosen *raw* scores — and never applies the chosen `choice:human_ability_bonus -> ability:strength` selection to them. `explain_human_pilot_race_seam`'s `race.human.ability_bonus_target` explanation record *narrates* the selection ("the chosen strength score yields modifier {modifier:+}") but only reads back the already-computed (bonus-less) modifier; it does not add anything.

`combat.baseline_melee_attack_bonus` still coincidentally matches PCGen's total (`5 = 5`), but via non-equivalent arithmetic that masks the same bug: Codex computes `BAB(+1) + STR(+3) + Weapon Focus(+1) = 5`; PCGen's real XML shows `BAB(+1) + STR(+4) + misc(0) = 5` (Weapon Focus's `+1` applies only to the specific-weapon attack roll in PCGen's model, not to the generic `/attack/melee` stat block base-xml.ftl exports). Flagged this explicitly in the test's module doc comment as a real, worth-noting observation — it does not itself fail that dimension's exact-value comparison, but it means that dimension's "match" should not be read as proof the two systems compute melee attack bonus the same way.

`defense.baseline_armor_class` and the three saves are unaffected (Dexterity- and Constitution-driven respectively, not Strength), consistent with this being a Strength-modifier-specific bug rather than a broader ability-modifier defect.

## Decision: did NOT force the `oracle_checked` upgrade

Per this cycle's own explicit instruction and `docs/governance/no-stub-mvp-doctrine.md` ("real failure over fake success, in either direction"): two real dimension mismatches remain, so `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt` is **untouched** by this cycle. The test asserts both the loaded, typed value (`ClaimTier::NotYetGrounded`) and the raw on-disk text (`current_claim_status=not_yet_grounded`) remain exactly as they were.

This is judged the correct outcome, not a shortfall: the original blocker (no real same-character `.pcg`) is genuinely resolved, and running the real pipeline against a genuinely same-character build surfaced a real, previously-invisible Codex bug — exactly the kind of signal an oracle-comparator pipeline exists to produce. Forcing the upgrade here would have hidden a real correctness gap in `rules_core::pilot_compute`.

## Verification transcript

```text
$ cargo test --locked --test sd26_pilot_case_verification
running 2 tests
test golden_fixture_starts_this_cycle_at_not_yet_grounded ... ok
test full_pipeline_runs_end_to_end_and_finds_two_genuine_skill_mismatches ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.60s

$ cargo test --locked --lib
test result: ok. 162 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

$ cargo test --locked --test sd26_comparator
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --locked --test sd26_normalization
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --locked --test sd26_parity_report
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --locked --test sd26_pcgen_runner
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 33.59s

$ cargo test --locked --test sd26_identifier_discipline_audit
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Real generated parity report (this cycle's actual output)

```markdown
# Oracle parity report: pf1-crb-human-fighter-level1

## Summary

- Matches: 7
- Mismatches: 2
- Result: FAIL

## Per-Dimension Comparison

| Dimension | PCGen | Codex | Match | Notes |
|---|---|---|---|---|
| character.identity | pf1-crb-human-fighter-level1 | pf1-crb-human-fighter-level1 | yes | — |
| combat.baseline_melee_attack_bonus | 5 | 5 | yes | — |
| defense.baseline_armor_class | 17 | 17 | yes | — |
| defense.total_save.fortitude | 4 | 4 | yes | — |
| defense.total_save.reflex | 2 | 2 | yes | — |
| defense.total_save.will | 1 | 1 | yes | — |
| skill.selected_modifier.intimidate | 3 | 3 | yes | — |
| skill.selected_modifier.climb | 6 | 5 | no | — |
| skill.selected_modifier.swim | 6 | 5 | no | — |

## Normalization Rules Used

- trailing-whitespace-strip (per `normalization.rs`)
- integer-coercion (per `normalization.rs`)

## Discovered Deltas

- `skill.selected_modifier.climb` — PCGen: 6, Codex: 5 (value mismatch)
- `skill.selected_modifier.swim` — PCGen: 6, Codex: 5 (value mismatch)
```

- **Discovery forwards:**
  - `## DISCOVERED` (forwarded to `progress.md`): `pilot_compute::compute_ability_modifiers` never applies chosen racial ability-score bonuses (Human `+2 Strength`) before deriving `AbilityModifiers`, causing a real `skill.selected_modifier.climb`/`swim` mismatch against the oracle. Blocks `CG-03` until fixed in `src/rules_core/pilot_compute.rs` (out of this cycle's `src/oracle_validation/` file-touch scope). Not self-healable inline.
- **Next-cycle plan:** a `rules_core` fix cycle should correct `compute_ability_modifiers` (or add an explicit racial-bonus-application step before it) to fold `choice:human_ability_bonus` (and, by the same pattern, any other racial ability-score bonus) into the ability score/modifier actually used downstream, then re-run `tests/sd26_pilot_case_verification.rs`. If that re-run shows 9/9 genuine matches, `current_claim_status` can then be genuinely earned to `oracle_checked`. Also worth a follow-up note on `combat.baseline_melee_attack_bonus`'s coincidental (non-equivalent-arithmetic) match once the Strength bug is fixed — confirm it still matches PCGen for the right reason afterward (BAB+4+Weapon Focus should still total the same final number PCGen reports, since the fix only affects the STR term).
