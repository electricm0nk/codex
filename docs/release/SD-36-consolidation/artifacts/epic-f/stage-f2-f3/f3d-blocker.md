# F3d — negative-control flip: BLOCKED (premise refuted by measurement)

Step: flip assertion (b) of the multiclass negative controls from "must stay claim-blocked
in this slice" to `assert_eq!(receipt.status, HeadlessReceiptStatus::Computed, "{diagnostics:?}")`,
list byte-identical, sabotage parity by removing F3b's generic gate arm (spec §5; F3.1–F3.3).

Outcome: not landed. The flip was written, run RED, measured, and reverted. Nothing under
`src/`, `tests/`, `data/` or `site/` changed in this step. Only this evidence was committed.

## Population (measured, not assumed)

| Set | Count | Source |
|---|---|---|
| `sd18_widening --list` tests | 891 | `cargo test --locked -j 8 --test sd18_widening -- --list` (matches F3.2) |
| `sd13_progression --list` tests | 1,136 | same for `sd13_progression` (matches F3.2) |
| `MULTICLASS_NEG_ROWS` rows (sd18 macro) | 64 | `tests/sd18_widening/rows.rs` |
| `multiclass_negative_controls!` rows (sd13 macro) | 59 | the count of `blocked_message:` in `tests/sd13_progression/*.rs` excluding rows.rs |
| hand-written `fn multiclass_*` with "multiclass X must stay claim-blocked in this slice" | 64 | 43 top-level files + 16 sd13_progression + 5 sd18_widening (druid 11–15) |
| **Flip population** | **187** | `f3d-sites.tsv` (one row per test: name, fixture, from, to) |

Not in the population (already different shapes): sd18 Fighter 11–20 (10, already assert no
claim-blocker, the Fighter/Rogue precedent), sd18 Wizard 11–20 + sd13 Wizard 2–10 (19, assert
the Wizard records DO fire and still claim-blocked on posture gates), 12 other `multiclass_*`
tests with no claim-blocked assert.

## RED: 187 of 187 fail, and not for a multiclass reason

`f3d-red.log` (cmd in its header): 28 passed (the non-flipped `multiclass_` filter matches),
**187 failed**, every one `left: Blocked`.

Claim-blocking ids across the 187 failing mixes (`f3d-probe-alone-vs-mix.tsv`, column 4):

| id | mixes |
|---|---|
| `combat.baseline_unsupported` | 187 |
| `skill.selected_modifier.unsupported` | 187 |
| `class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported` | 25 |
| `class_feature.cleric.healing_domain.rebuke_death.unsupported` | 19 |
| `multiclass.monk.class_feature.monk.bounded_progression.bonus_feat.unsupported` | 14 |

No `multiclass.class_chassis.unsupported`, `multiclass.save_shape.*`, `class_chassis.unsupported`
or `defense.total_save.unsupported` fires on any of the 187: the multiclass gate + fold ADMITS
every one of these mixes already.

## Why: the class ALONE is Blocked on the same lines (187 of 187)

Probe (`f3d-probe-test.rs.txt`, a temporary `tests/zz_f3d_probe.rs`, deleted): for each site,
the receipt of the unmodified single-class fixture vs the mix.

- Class alone: **187 of 187 Blocked** (column 3): 129 on exactly `combat.baseline_unsupported`
  + `skill.selected_modifier.unsupported`; 25 + sorcerer bloodline; 19 + cleric healing domain;
  14 + monk bonus feat.
- Mix blocker set == class-alone blocker set (after stripping the `multiclass.<class>.`
  re-scope) for **187 of 187** (14 differ only by sort order after the re-scope).

The two posture gates only compute the exact GE-06 deterministic posture
(Longsword / Chain Shirt / no shield / Power Attack selected-inactive / Weapon Focus as the
fighter bonus feat / Climb, Intimidate, Swim rank 1). These class fixtures are not in that
posture (e.g. `pf1_human_barbarian_level12_sd18_widening_deterministic_input.txt`: Survival 1,
no Weapon Focus, no fighter bonus-feat choice). The census's 185 of 185 mix panel is Computed
because the census builds every mix from the GE-06 fixture
(`class_seeds::FIXTURE_RELATIVE_PATH`) plus canonical seeds — a different input from these
tests' class fixtures.

Adding the Fighter-dip's own choices to the mix (weapon focus feat, power attack
selected-inactive, `choice:fighter_bonus_feat:feat:weapon_focus:weapon:longsword`;
`f3d-probe-mix-plus-fighter-dip-choices.tsv`) still leaves 187 of 187 Blocked:
`skill.selected_modifier.unsupported` 187, `combat.baseline_unsupported` 122 (fixture lacks the
items), plus the 58 class-feature blockers.

A receipt of `Computed` for these mixes would therefore require either rewriting 187 fixtures
into the GE-06 posture (which turns each test into a copy of the census panel, not "this class's
own fixture widened to a mix"), or widening the GE-06 combat/skill gates (an engine change far
outside F3). Asserting `Computed` without one of those is a fabricated success.

## Sabotage parity is also refuted

All 187 mixes are CRB table classes (barbarian, bard, cleric, druid, monk, paladin, ranger,
rogue, sorcerer) + fighter. `multiclass_fold::multiclass_member` admits them through the
`table_class_id(..).and_then(good_saves_for)` branch, which existed before F3b. Sabotage run:
insert `return Err(MULTICLASS_CLASS_UNSUPPORTED)` ahead of the generic (non-table) arm, re-run
the probe: **187 of 187 mix results UNCHANGED** (`f3d-probe-generic-arm-sabotage.tsv` is
byte-identical to `f3d-probe-alone-vs-mix.tsv`; `cmp` exit 0). Restored with
`git checkout -- src/rules_core/pilot_compute/multiclass_fold.rs`. So "remove the generic arm ->
flipped total red" cannot hold for this population under any assertion.

## What would hold (for an operator ruling, not done here)

- **Status parity** (the F3b `class_dispatch` precedent, `..._computes_exactly_when_it_computes_alone`):
  (b) becomes "mix status == class-alone status AND every mix claim-blocker is a class-alone
  blocker (modulo `multiclass.<class>.` re-scope)". Measured: holds for 187 of 187 today. Its
  sabotage is a gate removal (not the generic arm), which would add a `multiclass.*` blocker the
  class alone lacks.
- **Posture rebase**: build each mix from the GE-06 fixture + canonical seeds (the census
  input). Duplicates the census panel (185 of 185, already baselined).

`f3d-attempt.diff` is the reverted mechanical flip (both macros, 64 hand-written sites,
`cbmsg`/`blocked_message` row data dropped), kept for reuse by whichever ruling is taken.
