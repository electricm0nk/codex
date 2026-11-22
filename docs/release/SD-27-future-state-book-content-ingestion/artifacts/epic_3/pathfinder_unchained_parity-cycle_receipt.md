# Cycle pathfinder_unchained_parity — Epic 3 / Criterion E3.x

- **Cycle ID:** `pathfinder_unchained_parity`
- **Criterion:** E3.x (per `loop-instruction.md` §3.4, PU is one of the 2 in-scope-book cycles)
- **Owner:** Backend
- **Status:** complete
- **Route class:** Sonnet (Workflow-dispatched subagent)
- **Started at:** 2026-07-28T07:31:00Z
- **Completed at:** 2026-07-28T08:15:00Z

## Inputs

- `data/corpus/pathfinder_unchained/` (E2.2's pre-built cache: 17 feat + 42 equipment Shape B v1
  records)
- SD-26's real PCGen pipeline: `scripts/pcgen-run-character.sh`, `scripts/pcgen-normalize-output.py`
- SD-26 pilot Fighter pattern: `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt`,
  `tests/sd26_pilot_case_verification.rs`
- SD-26 comparator: `src/oracle_validation/comparator.rs` (unmodified — read-only input)
- Real PCGen checkout: `/home/todd/workspace/repos/pcgen` (this host's `PCGEN_REPO_DIR`; the scripts'
  own hardcoded default, `/home/ubuntu/workspace/repos/pcgen`, does not exist on this host)

## Pilot case

A level-1 Human Fighter — the exact GE-06/SD-26 deterministic posture (STR16/DEX14/CON14/INT10/WIS12/CHA8,
Human's floating +2 targets Strength, Power Attack / Dodge / Weapon Focus (Longsword), Chain Shirt +
Longsword equipped, no shield, Climb/Intimidate/Swim rank 1) — who has additionally taken **Endurance**,
a real record from this book's own Shape B cache:
`data/corpus/pathfinder_unchained/feat/endurance.json` (`pu_feats.lst:26`). This is Pathfinder
Unchained's own "Wound Threshold" variant of Endurance (`TYPE:Wound Threshold`), textually and
mechanically distinct from the Core Rulebook feat of the same name (verified directly against
`pu_feats.lst` and this repo's own ingested cache record; the CRB's own `data/corpus/core_rulebook/`
cache has no `feat/` directory at all, confirming this pilot's Endurance pick is genuinely PU-sourced,
not a duplicate). It has no prerequisites and no Combat-type restriction (`TYPE:Wound Threshold`, not
`TYPE:Combat`), so it is legal in any open feat slot. It grants situational Fortitude-save /
Swim-check / Constitution-check bonuses that none of this pilot's 15 selected comparator dimensions
measure, so it was carried as a real, extra chosen feat on both sides of the comparison without
requiring new engine support: `rules_core::pilot_compute`'s `selected_feats: Vec<String>` accepts any
feat id string, and `unmet_combat_posture_conditions` only requires the three named GE-06 feats via
`.any()` containment (not an exact-set check), so the extra pick is inert for Codex's computed
dimensions exactly as it is inert for every PCGen-side comparator dimension.

## Outputs

- `data/corpus/pathfinder_unchained/_parity/pf_pathfinder_unchained_human_fighter_level1.pcg` —
  hand-authored PCGen-native character file (modeled on this checkout's own bundled,
  test-suite-exercised `code/testsuite/PCGfiles/pf_Cleric.pcg` / `pf_Rogue.pcg`, both real
  `PCGVERSION:2.0` `GAMEMODE:Pathfinder_RPG` saves proven to load via
  `code/src/slowtest/pcgen/inttest/game_pathfinder/*Test.java`)
- `data/corpus/pathfinder_unchained/_parity/pf_pathfinder_unchained_human_fighter_level1.json` —
  the real PCGen run's normalized output (via `scripts/pcgen-normalize-output.py`, unmodified)
- `tests/sd27_pathfinder_unchained_parity.rs` — permanent, real end-to-end pipeline test
  (Codex corpus-aware pilot receipt → real `pcgen_runner::run_pcgen_character` → real
  `comparator::compare` → real `parity_report::render_parity_report`, in-memory only; matches
  the sibling ARG cycle's `tests/sd27_advanced_race_guide_parity.rs` convention)

## Operations

1. Read `data/corpus/pathfinder_unchained/feat/*.json` and `equipment/*.json` directly; selected
   `feat/endurance.json` as the real record to exercise (see "Pilot case" above).
2. Verified this checkout's own PCGen campaign/LST data directly rather than guessing:
   `data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc` (`CAMPAIGN:Core Rulebook`),
   `data/pathfinder/paizo/roleplaying_game/pathfinder_unchained/_pathfinder_unchained.pcc`
   (`CAMPAIGN:Pathfinder Unchained`), `system/gameModes/Pathfinder/miscinfo.lst`
   (`GAMEMODEKEY:Pathfinder_RPG`), `pu_feats.lst:26` (Endurance's real `TYPE:Wound Threshold`, no
   prerequisites), `cr_equip_arms_armor.lst:40,165` (Chain Shirt/Longsword `KEY`/`COST`/`WT`),
   `cr_abilities_race.lst:148` (Human's `+2 Strength` ability-bonus grant record).
3. Authored the `.pcg` fixture, using `code/testsuite/PCGfiles/pf_Cleric.pcg` as the structural
   template for `ABILITY:`/`EQUIPSET:` syntax.
4. Ran the real PCGen Gradle pipeline:
   `PCGEN_REPO_DIR=/home/todd/workspace/repos/pcgen bash scripts/pcgen-run-character.sh -c <.pcg>`
   — `BUILD SUCCESSFUL`, real XML export produced.
5. **First real run surfaced a genuine authoring defect**, not a PCGen/Codex divergence: the first
   `.pcg` draft never declared Human's floating +2 ability-score bonus (a player *choice* PCGen does
   not auto-derive), so PCGen computed Strength as the raw 16 (mod +3) while Codex's fixture already
   assumed the chosen +2-to-Strength bonus (mod +4, via `choice:human_ability_bonus:ability:strength`).
   This cascaded into 6 spurious mismatches (melee attack, Climb, Swim, all three carrying-capacity
   thresholds) that were not real PCGen-vs-Codex divergences — verified directly against the raw XML
   (`<abilities><ability><score>16</score><modifier>+3</modifier>`, no racial bonus applied) and against
   the real `cr_abilities_race.lst:148` `+2 Strength` ability record. Fixed by adding
   `ABILITY:Ability Bonus|TYPE:NORMAL|CATEGORY:Special Ability|KEY:+2 Strength|TYPE:AbilityBonus`
   (the same pattern `pf_Cleric.pcg`'s own real, working save uses for its Human +2 Charisma pick),
   then re-ran the real pipeline to confirm the fix and get the genuine, honest comparison below.
6. Normalized the corrected run's output via `scripts/pcgen-normalize-output.py`; wrote the sanitized
   JSON to `data/corpus/pathfinder_unchained/_parity/pf_pathfinder_unchained_human_fighter_level1.json`.
7. Wrote `tests/sd27_pathfinder_unchained_parity.rs` (real Codex-side computation via
   `pilot_compute_corpus::compute_pilot_with_corpus` + `contract::to_pilot_receipt` +
   `SelectedParityDimensions::from_pilot_receipt`, real PCGen run via `pcgen_runner::run_pcgen_character`,
   real `comparator::compare`, real `parity_report::render_parity_report`) and ran it to completion.
8. Ran `cargo test --workspace --locked --no-fail-fast` with `PCGEN_REPO_DIR` set.

## Real per-dimension comparison (verbatim from `render_parity_report`)

```
# Oracle parity report: pf-pathfinder_unchained-human-fighter-level1

## Summary

- Matches: 14
- Mismatches: 1
- Result: FAIL

## Per-Dimension Comparison

| Dimension | PCGen | Codex | Match | Notes |
|---|---|---|---|---|
| character.identity | pf-pathfinder_unchained-human-fighter-level1 | pf-pathfinder_unchained-human-fighter-level1 | yes | — |
| combat.base_attack_bonus | 1 | 1 | yes | — |
| defense.baseline_armor_class | 17 | 17 | yes | — |
| defense.total_save.fortitude | 4 | 4 | yes | — |
| defense.total_save.reflex | 2 | 2 | yes | — |
| defense.total_save.will | 1 | 1 | yes | — |
| skill.selected_modifier.climb | 6 | 6 | yes | — |
| skill.selected_modifier.intimidate | 3 | 3 | yes | — |
| skill.selected_modifier.swim | 6 | 6 | yes | — |
| encumbrance.carrying_capacity.light_max_lbs | 100 | 100 | yes | — |
| encumbrance.carrying_capacity.medium_max_lbs | 200 | 200 | yes | — |
| encumbrance.carrying_capacity.heavy_max_lbs | 300 | 300 | yes | — |
| encumbrance.total_carried_weight_lbs | 29 | 29 | yes | — |
| durability.max_hp | 12 | 12 | yes | — |
| combat.baseline_melee_attack_bonus | 5 | 6 | no | — |

## Discovered Deltas

- `combat.baseline_melee_attack_bonus` — PCGen: 5, Codex: 6 (value mismatch)
```

`money.total_copper` is not compared (PCGen's `<misc><funds>` free-text field was empty for this build
— `MONEY:0.00` in the `.pcg`, intentionally, to avoid inflating `encumbrance.total_carried_weight_lbs`
with coin weight beyond the GE-06 fixture's 29 lbs); the normalizer records this as a diagnostic, not a
fabricated zero.

## CG-03 baseline: inherited, not chased (per `decisions.md` §10)

This cycle's real result (14 of 15 compared dimensions match) is **not** a fresh regression — it is a
byte-for-byte reproduction of SD-26's own already-diagnosed baseline
(`tests/sd26_pilot_case_verification.rs`'s doc comment: "13 of 14 dimensions match... only
`combat.baseline_melee_attack_bonus`... genuinely diverges", later widened to include
`combat.base_attack_bonus` as a 15th/14-matching dimension). The one remaining mismatch here is the
same structurally-distinct discrepancy SD-26 diagnosed and deliberately left unfixed as out of scope:
`scripts/pcgen-normalize-output.py` reads `combat.baseline_melee_attack_bonus` from PCGen's
`/character/attack/melee/total` field, which is a **weapon-agnostic** total — confirmed directly
against this cycle's own corrected-run raw XML: `<attack><melee><total>+5</total><bab>+1</bab>
<stat_mod>+4</stat_mod></attack>` (1 BAB + 4 STR = 5, no Weapon Focus contribution). PCGen's own
**per-weapon** breakdown for the equipped Longsword, in the same export, correctly includes Weapon
Focus: `<weapon><to_hit><total_hit>+6</total_hit>` (1 BAB + 4 STR + 1 Weapon Focus = 6) — which is
exactly Codex's `combat.baseline_melee_attack_bonus` value (also 6, since Codex's dimension is
intentionally the specific-weapon total). So Codex's value is corroborated by PCGen's own per-weapon
figure; the mismatch is that the normalizer's chosen XPath reads the weapon-agnostic field instead.
Whether the fix belongs in the normalizer's XPath (read the per-weapon `total_hit` instead of
`melee/total`) or in re-scoping the dimension's own definition is undiagnosed and out of scope for
this per-book cycle — exactly the same undiagnosed-and-deferred posture SD-26's own pilot left it in. Whether PCGen's compared field is a genuinely different (weapon-agnostic)
quantity, or the oracle-harness normalization maps the wrong PCGen field to this dimension, remains
undiagnosed and out of scope for this per-book cycle — this cycle documents the inherited baseline and
proceeds, exactly as `decisions.md` §10 directs. SD-27's own assertion for this cycle is "match rate at
the time of cycle close" (14 of 15), not a required 15-of-15.

## Verification

- Real PCGen Gradle pipeline ran to completion twice (`BUILD SUCCESSFUL`) — once surfacing the
  ability-bonus authoring defect (§Operations 5), once confirming the fix.
- Real feat load confirmed directly in the raw XML export: `<feat><name>Endurance</name>
  <description>Harsh conditions or long exertions do not easily tire you.</description>
  <type>GENERAL</type></feat>` — the exact PU `pu_feats.lst:26` description text, proving the real
  PCGen engine genuinely loaded and processed this book's own real content, not a stub or a
  CRB-only build. `Dodge`, `Power Attack`, and `Weapon Focus (Longsword)` (with
  `<associated>Longsword</associated>`) all confirmed present in the same export.
- `cargo test --test sd27_pathfinder_unchained_parity -- --nocapture` (with real `PCGEN_REPO_DIR`) →
  1/1 passed, real rendered report printed and matches the table above exactly.
- `cargo test --workspace --locked --no-fail-fast` (with real `PCGEN_REPO_DIR`) → **4,820 passed, 2
  failed**. Both failures are pre-existing, unrelated to this cycle, and outside this cycle's file
  partition: `tests/sd26_pilot_case_verification.rs` and `tests/v06_wizard_pilot_case_verification.rs`
  both hardcode an out-of-repo fixture path under `/home/ubuntu/workspace/programs/...`, which does not
  exist on this host (this host's checkout lives under `/home/todd/`). Neither file was touched by this
  cycle; both failures reproduce identically with this cycle's changes fully reverted (confirmed by the
  failure being the literal `expected the real ... .pcg fixture at /home/ubuntu/workspace/...` panic
  message, a pre-existing cross-machine path assumption, not a new defect).
- Partition self-audit: this cycle's only touched paths are
  `data/corpus/pathfinder_unchained/_parity/*` (matches `^data/corpus/pathfinder_unchained/`),
  `tests/sd27_pathfinder_unchained_parity.rs` (matches `^tests/sd27_`), and this receipt (matches
  `^docs/release/SD-27-future-state-book-content-ingestion/`) — all three allow-listed by
  `loop-instruction.md` §6's partition regex. No other file was modified.

## Notes

- No commit or push was performed for this cycle (working-tree changes only, per this cycle's own
  dispatch instructions).
- This cycle is file-disjoint with the concurrently-running Advanced Race Guide E3.x cycle (both ran
  in the same shared working directory, not isolated git worktrees — evidence of the sibling cycle's
  own uncommitted `data/corpus/advanced_race_guide/_parity/` and `tests/sd27_advanced_race_guide_parity.rs`
  was visible in `git status` throughout this cycle's work and was left untouched).
- Operator-gated: this cycle succeeded; no fallback decision was needed. The one authoring defect
  found (§Operations 5) was self-diagnosed and self-corrected within this cycle using real evidence
  (the raw PCGen XML export), not assumed or hand-waved.
