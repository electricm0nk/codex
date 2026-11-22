# Cycle advanced_race_guide_parity — Epic 3 / E3.1

- **Cycle ID:** `advanced_race_guide_parity`
- **Criterion:** E3.1 (per `loop-instruction.md §3.4`)
- **Owner:** Backend
- **Status:** complete
- **Route class:** Sonnet (Workflow-dispatched subagent)
- **Started at:** 2026-07-28T11:30:00Z
- **Completed at:** 2026-07-28T11:47:00Z

## Inputs

- `data/corpus/advanced_race_guide/` (the E2.1 output; `equipment/arms_armor/dogslicer.json`,
  `feat/general/defiant_luck.json`)
- SD-26 PCGen pipeline: `scripts/pcgen-run-character.sh`, `scripts/pcgen-normalize-output.py`
- SD-26 pilot Fighter pattern: `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt`,
  `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
- SD-26 comparator: `src/oracle_validation/comparator.rs` (consumed unmodified)
- `docs/release/SD-27-future-state-book-content-ingestion/decisions.md §10` (CG-03 baseline, inherited
  not chased)
- The real PCGen checkout at `$PCGEN_REPO_DIR=/home/todd/workspace/repos/pcgen` (its own
  `data/pathfinder/paizo/roleplaying_game/{core_rulebook,core_essentials,advanced_race_guide}/*.lst`,
  `*.pcc` and `system/gameModes/Pathfinder/miscinfo.lst`, read to author a real, loadable `.pcg` — see
  Notes for what this required beyond guessing at the format)

## Pilot case

Level-1 Human Fighter, mirroring SD-26's GE-06 deterministic pilot's ability scores (STR 16→18 human
bonus, DEX 14, CON 14, INT 10, WIS 12, CHA 8), feats (Power Attack, Dodge, Weapon Focus (Longsword)),
skills (Climb/Intimidate/Swim rank 1), and equipment (Chain Shirt + Longsword equipped, no shield) — the
same build SD-26's own CRB pilot uses, so the shared dimensions are a like-for-like baseline. Extended
with two real records from THIS book's own Shape B cache:

- **`data/corpus/advanced_race_guide/equipment/arms_armor/dogslicer.json`** (`arg_equip_arms_armor.lst`
  line 36) — the Dogslicer weapon, equipped as a carried secondary weapon (it requires its own Exotic
  Weapon Proficiency this level-1 pilot does not take, so it is not the primary attack weapon; it is
  carried to exercise the real record honestly, not to claim proficient combat use with it).
- **`data/corpus/advanced_race_guide/feat/general/defiant_luck.json`** (`arg_feats.lst` line 104,
  `PREFACT:1,TEMPLATES,IsHuman=true`) — the Defiant Luck feat, a real ARG feat this Human character
  genuinely qualifies for (confirmed: the real PCGen engine granted it with no prerequisite-failure
  warning in this run's console output). It grants a reroll resource (`DefiantLuckTimes`) none of this
  pilot's selected comparator dimensions measure.

## Outputs

- `data/corpus/advanced_race_guide/_parity/pf_advanced_race_guide_human_fighter_level1.pcg`
  (hand-authored, real PCGen-native character file)
- `data/corpus/advanced_race_guide/_parity/pf_advanced_race_guide_human_fighter_level1.json` (real
  PCGen output, normalized via `scripts/pcgen-normalize-output.py`)
- `tests/sd27_advanced_race_guide_parity.rs` (new; wires the real
  `pcgen_runner -> comparator -> parity_report` pipeline end to end against this cycle's fixtures — same
  shape as `tests/sd26_pilot_case_verification.rs`)
- `tests/sd27_license_stripping_shape_v1.rs` (modified; see Notes — a pre-existing test-scope gap this
  cycle's own real output surfaced and fixed)
- This receipt

## Operations

1. Read `data/corpus/advanced_race_guide/` cache; selected the Dogslicer weapon and Defiant Luck feat as
   this cycle's real exercised records (see Pilot case above).
2. Authored `pf_advanced_race_guide_human_fighter_level1.pcg` against the real PCGen data checkout —
   see Notes for the real format discoveries this required.
3. Ran the real PCGen Gradle pipeline against it:
   `PCGEN_REPO_DIR=/home/todd/workspace/repos/pcgen bash scripts/pcgen-run-character.sh -c <fixture>`
   → `BUILD SUCCESSFUL`, character loaded (`CharacterManager:193 Loaded character
   pf1-arg-human-fighter-level1`), real XML export produced.
4. Normalized via `scripts/pcgen-normalize-output.py` → wrote the required `.json` output.
5. Ran `comparator::compare` for real via the new `tests/sd27_advanced_race_guide_parity.rs`
   (`cargo test --test sd27_advanced_race_guide_parity -- --nocapture`), producing the real
   per-dimension table below.
6. Ran `cargo test --workspace --locked` (see Verification).
7. Ran the dual-audit gate scripts (see Verification — scope caveat noted).
8. Wrote this receipt. Per this task's explicit instruction, did **not** commit or push.

## Real per-dimension comparison (from the real `comparator::compare` run)

| Dimension | PCGen | Codex | Match |
|---|---|---|---|
| character.identity | pf1-arg-human-fighter-level1 | pf1-arg-human-fighter-level1 | yes |
| combat.base_attack_bonus | 1 | 1 | yes |
| defense.baseline_armor_class | 17 | 17 | yes |
| defense.total_save.fortitude | 4 | 4 | yes |
| defense.total_save.reflex | 2 | 2 | yes |
| defense.total_save.will | 1 | 1 | yes |
| skill.selected_modifier.climb | 6 | 6 | yes |
| skill.selected_modifier.intimidate | 3 | 3 | yes |
| skill.selected_modifier.swim | 6 | 6 | yes |
| encumbrance.carrying_capacity.light_max_lbs | 100 | 100 | yes |
| encumbrance.carrying_capacity.medium_max_lbs | 200 | 200 | yes |
| encumbrance.carrying_capacity.heavy_max_lbs | 300 | 300 | yes |
| durability.max_hp | 12 | 12 | yes |
| combat.baseline_melee_attack_bonus | 5 | 6 | **no** |
| encumbrance.total_carried_weight_lbs | 30 | 29 | **no** |

**Result: 13 of 15 compared dimensions match. 2 genuine mismatches**, both real and diagnosed (not
fabricated, not papered over):

1. **`combat.baseline_melee_attack_bonus` (PCGen: 5, Codex: 6) — the inherited CG-03 baseline.** Same
   dimension, same root cause, same value pair SD-26's own CRB pilot documents in
   `tests/sd26_pilot_case_verification.rs`'s module doc comment: Codex's `combat.baseline_melee_attack_bonus`
   is documented as "the Longsword specific attack bonus" (includes Weapon Focus's +1), while PCGen's
   exported `/character/attack/melee/total` does not fold in a weapon-specific `BONUS:WEAPONPROF=...TOHIT`
   bonus at the generic-melee level (confirmed directly in this run: PCGen's own `<feats>` block shows
   `Weapon Focus (Longsword)` correctly granted with `<associated>Longsword</associated>`, yet
   `attack/melee/misc_mod` stays `+0`). Per `decisions.md §10`, this is v0.6's lane (CG-03,
   `pilot_compute.rs:4743-4767`) — inherited, not chased by this cycle.

2. **`encumbrance.total_carried_weight_lbs` (PCGen: 30, Codex: 29) — a real, NEW mismatch this cycle's
   own ARG pilot surfaces**, distinct from CG-03. Root cause, traced to source:
   `src/rules_core/encumbrance.rs::compute_encumbrance` resolves each equipped item's corpus record via
   the generic, book-agnostic `equipment_id_resolve` (which correctly found the real ARG Dogslicer record
   in this test's corpus), but then looks up that item's **weight** via
   `crate::rules_core::rules_tables::crb::equipment_tables::equipment_tables()` — a compiled-in,
   **CRB-only** static table. The Dogslicer (an ARG-only item) is not in that table, so its 1 lb is
   silently dropped into `unresolved_item_ids` instead of being counted. PCGen's real export correctly
   includes it (30 lbs = 25 Chain Shirt + 4 Longsword + 1 Dogslicer); Codex's total stays at 29 (the
   Dogslicer's weight excluded). This is a genuine, structural, book-scoped gap in `encumbrance.rs` that
   this cycle's own partition (`data/corpus/advanced_race_guide/`, `tests/sd27_*`) does **not** authorize
   fixing (`encumbrance.rs` is `src/rules_core/`, out of scope for this per-book parity cycle — the
   `loop-instruction.md §3.4` notes say explicitly "The cycle does NOT modify `src/oracle_validation/`",
   and by the same file-partition logic this cycle does not modify `src/rules_core/encumbrance.rs`
   either). Reported here as new, real, honest evidence for a future cycle to pick up.

## Verification

- `cargo test --test sd27_advanced_race_guide_parity -- --nocapture` → 1 passed; real PCGen Gradle run
  (`BUILD SUCCESSFUL`), real comparator table printed above, both sides genuinely computed (no
  fabricated values).
- `cargo test --workspace --locked` (with `PCGEN_REPO_DIR=/home/todd/workspace/repos/pcgen`) → all green
  **except two pre-existing, environment-path-dependent failures unrelated to this cycle**:
  `tests/sd26_pilot_case_verification.rs::full_pipeline_runs_end_to_end_and_finds_one_genuine_attack_bonus_mismatch`
  and `tests/v06_wizard_pilot_case_verification.rs::full_pipeline_runs_end_to_end_for_the_wizard_pilot_case`,
  both of which hardcode `/home/ubuntu/workspace/programs/codex/requirements/...` fixture paths that do
  not exist on this host (`/home/todd/...`). **Confirmed pre-existing and unrelated** by stashing this
  cycle's own changes and re-running both tests against the unmodified tree: identical failures,
  identical messages. Neither file is in this cycle's file partition (`tests/sd26_*`, `tests/v06_*`), so
  this cycle does not fix them.
- Identifier-discipline / wired-integration dual-audit gate: ran both scripts against `BASE_BRANCH=origin/develop`
  per their documented usage; both reported pre-existing failures. On inspection, **both failures are
  entirely accumulated branch history unrelated to this cycle** — `tranche/7` is 7 commits ahead of
  `origin/develop` and neither script's diff can be scoped to only this cycle's own (uncommitted, per
  this task's explicit "do not commit" instruction) changes via `BASE_BRANCH...HEAD` (a three-dot
  committed-history diff; it does not see the working tree at all). Since this task explicitly forbids
  committing, there is no "cycle's own base" commit to point `BASE_BRANCH` at, exactly the gap
  `loop-instruction.md §6`'s own warning describes for the wrong-base failure mode. **Verified this
  cycle's actual diff is clean by construction**: both scripts' pathspecs scan only `src/**/*.rs` and
  `apps/desktop/**/*.ts*` — this cycle's real outputs (`tests/sd27_*.rs`,
  `data/corpus/advanced_race_guide/_parity/*`) fall entirely outside those pathspecs, so the audits are
  vacuous for this cycle's own changes regardless of base. Additionally ran the two scripts' forbidden
  patterns by hand directly against this cycle's changed/added files
  (`grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` and the bundle-tag
  pattern) — zero matches in either.
- PCGen Gradle pipeline ran end-to-end twice (once via the manual `pcgen-run-character.sh` +
  `pcgen-normalize-output.py` pair to produce the required `.json` output file, once via the Rust test's
  `run_pcgen_character` wrapper) — both `BUILD SUCCESSFUL`, both against the real, unmodified PCGen
  engine and this cycle's real `.pcg` fixture.
- `comparator::compare` produced the real per-dimension match/mismatch table above (13 matches, 2
  mismatches — not fabricated).

## Notes

- **Format discoveries, not guessed at.** The current PCGen checkout's actual data schema differs from
  the pre-existing `code/testsuite/PCGfiles/pf_*.pcg` reference fixtures in two load-bearing ways, both
  confirmed by real failed-then-fixed runs (not assumed):
  - `GAMEMODE:` must be `Pathfinder_RPG` (the internal `GAMEMODEKEY` from
    `system/gameModes/Pathfinder/miscinfo.lst`), not `Pathfinder` (the `.pcc`'s own `GAMEMODE:` field) —
    the mismatch produced a real, specific PCGen error (`Unable to load the character as it uses game
    mode: "Pathfinder"... PCGen is currently using gamemode "Pathfinder_RPG"`) on the first attempt.
  - The base "Human" `RACE:` definition lives in `core_essentials/races/human/human_races.lst`, not
    `core_rulebook/cr_races.lst` (which carries only a `Human.MOD` modifier) — `core_rulebook.pcc`
    transitively `PCC:`-includes `core_essentials/_core_essentials.pcc` and the per-race
    `core_essentials/races/human/_race.pcc`, so `CAMPAIGN:Core Rulebook` alone is sufficient; no separate
    `CAMPAIGN:Core Essentials` line was needed (confirmed: the character loaded and computed correctly
    with only `Core Rulebook` + `Advanced Race Guide`).
  - Human's own `+2 to one ability score` racial bonus requires an explicit second `ABILITY:` line
    consuming the `Ability Bonus` pool (`ABILITY:Ability Bonus|TYPE:NORMAL|CATEGORY:Special Ability|KEY:+2
    Strength|TYPE:AbilityBonus`) — omitting it (first attempt) left the bonus un-applied (STR stayed 16,
    not 18) even though `RACE:Human` alone grants the pool point.
- **The `_parity/` license-stripping test-scope fix is a real, in-partition fix, not scope creep.**
  `tests/sd27_license_stripping_shape_v1.rs` (authored in cycle E2.0.5, before any book had a `_parity/`
  directory) walks all of `data/corpus/` and asserts every `.json` file deserializes as a Shape B v1
  corpus record. This cycle's own required output
  (`data/corpus/advanced_race_guide/_parity/pf_advanced_race_guide_human_fighter_level1.json`) is a
  different schema by design (the oracle-comparison normalized-dimension shape, not `CorpusRecordV1`) —
  running the full test suite with it in place surfaced the gap directly (a real `FAILED` before the
  fix). Fixed by excluding `_parity/` directories from that test's walk, the same way it already excludes
  `LICENSE.json` for the same reason. `tests/sd27_*` is within this cycle's file partition. This also
  benefits the concurrently in-flight `pathfinder_unchained` E3.2 cycle (its own
  `data/corpus/pathfinder_unchained/_parity/*.json` was observed on disk during this cycle, untouched by
  this cycle, and would hit the identical pre-existing gap).
- **CG-03 baseline, inherited not chased, per `decisions.md §10`.** This cycle's own assertion is "match
  rate at the time of cycle close" (13/15), not a required full match. The one CG-03-attributable
  mismatch (`combat.baseline_melee_attack_bonus`) is the same root cause SD-26 already diagnosed and
  deliberately left open (v0.6's lane). The one new mismatch
  (`encumbrance.total_carried_weight_lbs`, the CRB-only `equipment_tables()` gap) is reported as new,
  real, honest evidence — not fixed, since `src/rules_core/encumbrance.rs` is outside this cycle's file
  partition.
- Did not commit or push, per this task's explicit instruction.
- **Partition audit note.** Ran `loop-instruction.md §6`'s own partition-audit pipeline (substituting
  `BOOK=advanced_race_guide`) against this session's full working-tree diff. The only path surviving the
  exclusion pipeline is `data/corpus/pathfinder_unchained/_parity/` — a directory this cycle did **not**
  create or touch; it was already present, untracked, on disk when this cycle started (the concurrently
  in-flight `pathfinder_unchained` E3.2 cycle's own real output). Confirmed via this cycle's own change
  log above: nothing in Outputs touches `pathfinder_unchained`. This cycle's own actual changes are 100%
  inside the partition.

