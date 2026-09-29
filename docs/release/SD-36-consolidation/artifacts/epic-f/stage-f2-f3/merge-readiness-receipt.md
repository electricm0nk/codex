# F2/F3 merge-readiness check (sd36/epic-f2-f3 -> tranche/16)

Checked 2026-09-25 on `sd36/epic-f2-f3` @ `74c45b5d0e` against `tranche/16` @ `cc21cac195`
(the merge base). Nothing was edited in the tree; the one probe (the F3.3 sabotage) was reverted
with `git checkout --`, and `git status --short` was empty at the end.

**Verdict: no blockers. Merge-ready.** Polish items and pre-existing defects are listed at the end.

## 0. Scope note: this batch changed the converter

This batch was framed as "data/sheet_rules untouched, no converter change". The branch does not
hold that: 2,801 files under `data/sheet_rules/**` and 11 converter sources under
`crates/codex-ingest/src/pcgen_import/sheet_rule/` changed (F3b2, F3b2b, F3c3, F3c4b, F3c5 and
others, each with its own `*-receipt.md` and `*-structural-diff.txt` in this directory).
`data/corpus/**` and `site/**` did not change (`git diff --stat tranche/16..HEAD -- data/corpus
site` is empty). The regenerated package is fresh and the frozen figure holds:

| Check | Command | Result |
|---|---|---|
| package freshness | `cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --check` | `records=49450 converted=49450 refused=0 rules=73363 var_tables=6211 verdict=PASS` |
| frozen status | `python3 scripts/site/check_frozen_status.py --check` | frozen at 100% (49,450 units) |
| residue gate | `python3 scripts/pcgen_residue_gate.py --check --closure` | `verdict=PASS` (70,045 shipped files scanned, 0 hits) |

Because the data changed, the before side of every comparison below was a clean `tranche/16`
worktree built into its own `CARGO_TARGET_DIR`. The package path is baked in at compile time, so
that build reads tranche/16's own code and data. Swapping only the data would not have been a
valid "before" here. The scratch worktree was removed afterwards.

## 1. Census re-run

`cargo run --locked -j 8 --bin class_census -- --json <scratch>/census-head.json`

```
ids=137 computed=63 blocked=0
prestige_swept=74 prestige_alone_blocked=74 prestige_mix_computed=68 prestige_mix_unknown=0
mix_panel_swept=185 mix_panel_computed=185 mix_panel_blocked=0
```

- The newest committed census, `census-f3c5.json`, matches byte for byte except for
  `generated_at`. `census-f3c.json` is an older stage (61 computed, 67 prestige mixes, 1
  unknown), and the F3c2 to F3c5 receipts account for every step from it to f3c5.
- `scripts/verify-baselines.env` matches the census: `BASELINE_CENSUS_COMPUTED=63`,
  `BASELINE_CENSUS_MIX_COMPUTED=185`, `BASELINE_CENSUS_PRESTIGE_ALONE_BLOCKED=74` and
  `BASELINE_CENSUS_PRESTIGE_MIX_COMPUTED=68`.
- No Computed row carries a claim-blocking diagnostic. That holds for 63 of 63 non-prestige
  classes, every one Computed at every level, 185 of 185 mix-panel rows and 68 of 68 Computed
  prestige mixes (75 prestige mixes in all). All 74 prestige classes are Blocked when taken
  alone.
- The remainder is named by mechanism. 6 of 74 prestige mixes are Blocked on
  `multiclass.save_shape.unrecognized`: `evangelist`, `exalted`, `mammoth_rider`,
  `pure_legion_enforcer`, `sentinel` and `ulfen_guard`. These are exactly the 6 records behind the
  14 slots in `NAMED_UNRECOGNIZED_SAVES`, and each has a source formula with an operator-precedence
  error (for example `classlevel+1/2`). None of them is folded as poor.

## 2. Hand-worked totals vs the rendered sheet (oracle first)

Fixture: the census's GE-06 deterministic Human (Str 18, Dex 14, Con 14, Int 10, Wis 12, Cha 8),
so Con +2, Dex +2, Wis +1 and Int +0.

Rules applied:
- **BAB:** the sum of each class's BAB.
- **Saves:** each class's unrounded value is summed and the total is floored once. Base classes
  use `L/2+2` (good) and `L/3` (poor). Prestige classes use `(L+1)/2` and `(L+1)/3`.
- **HP:** the first-listed class takes its maximum hit die at character level 1. Every other
  level takes `die/2+1`. Con is added at every level.
- **Skill points:** the class term only, `ranks x levels`.

The engine values are the `EXPL|` rows `class_chassis.base_attack_bonus`,
`class_chassis.base_save.*`, `defense.total_save.*`, `multiclass.hit_points` and
`multiclass.skill_points` from
`class_census --sheet-dump <build> --with-sheet-rules`.

| Build (carrier family) | Hand-worked BAB; base F/R/W; total F/R/W; HP; SP | Engine | Match |
|---|---|---|---|
| barbarian 12 + fighter 1 | 13; 10.5->10 / 4.33->4 / 4.33->4; 12/6/5; 14+99+8=121; 48+2=50 | same | yes |
| fighter 6 + duelist 3 (fighter) | 9; 5+1.33->6 / 2+2=4 / 2+1.33->3; 8/6/4; 52+24=76; 12+12=24 | same | yes |
| fighter 6 + arcane archer 3 (fighter) | 9; 7 / 4 / 3; 9/6/4; 76; 24 | same | yes |
| fighter 5 + adaptive warrior 2 (fighter) | 7; 4.5+1.5=6 / 1.67+1.5->3 / 1.67+1->2; 8/5/3; 44+16=60; 10+4=14 | same | yes |
| wizard 5 + loremaster 2 (wizard) | 3; 2 / 2 / 6; 4/4/7; 32+12=44; 10+8=18 | same | yes |
| wizard 6 + eldritch knight 2 (wizard) | 5; 2+1.5->3 / 2+1=3 / 5+1=6; 5/5/7; 38+16=54; 12+4=16 | same | yes |
| cleric 7 + holy vindicator 2 (cleric) | 5+2=7; 5.5+1.5=7 / 2.33+1->3 / 5.5+1.5=7; 9/5/8; 52+16=68; 14+4=18 | same | yes |
| wizard 3 + cleric 3 + mystic theurge 2 (wizard+cleric) | 1+2+1=4; 1+3.5+1->5 / 1+1+1=3 / 3.5+3.5+1.5->8; 7/5/9; 20+21+12=53; 6+6+4=16 | same | yes |
| sorcerer 5 + dragon disciple 3 (sorcerer) | 2+2=4; 1.67+2->3 / 1.67+1.33=3 / 4.5+2->6; 5/5/7; 32+27=59; 10+6=16 | same | yes |
| magus 4 + samurai 2 | 3+2=5; 7 / 1.33+0.67=2 (exact rationals) / 4.67->4; 9/4/5; 31+16=47; 16 | same | yes |
| kineticist 4 + fighter 2 | 3+2=5; 4+3=7 / 4+0.67->4 / 1.33+0.67=2; 9/6/3; 31+16=47; 16+4=20 | same | yes |
| psion 3 + fighter 2 | 1+2=3; 1+3=4 / 1+0.67->1 / 3.5+0.67->4; 6/3/5; 20+16=36; 6+4=10 | same | yes |
| fighter 6 + wizard 4 | 6+2=8; 5+1.33->6 / 2+1.33->3 / 2+4=6; 8/5/7; 52+24=76; 12+8=20 | same | yes |
| monk 4 + rogue 3 | 3+2=5; 4+1=5 / 4+3.5->7 / 4+1=5; 7/9/6; HP **Unknown**; 16+24=40 | same; HP not emitted, named `class_chassis.hit_points.unknown` (not a 0) | yes |
| fighter 5 + sentinel 2 (Unrecognized save) | refuse: sentinel saves are `classlevel+1/2` and `classlevel+1/3` | Blocked, `multiclass.save_shape.unrecognized`; no chassis totals emitted | yes |
| duelist 3 alone (prestige alone) | refuse | Blocked, `prestige_class.requires_base_class_levels`; no BAB/save/HP/SP rows | yes |

Spot checks: `arcane_archer:5`, `mystic_theurge:3` and `sentinel:2`, each taken alone, are all
Blocked with no chassis totals.

Class-skill union, one build at a time: the selected Climb, Intimidate and Swim modifiers carry
the +3 class-skill bonus exactly where a class in the mix lists the skill. For example, Eldritch
Knight has Climb and Swim but not Intimidate, and Wizard/Loremaster, Mystic Theurge and
Sorcerer/Dragon Disciple are not granted it. Longsword attack totals follow the class
proficiency records:
- Eldritch Knight, Loremaster, Dragon Disciple and Mystic Theurge carry the -4 penalty, because
  their records say they "gain no proficiency with any weapon or armor" and the carriers hold none.
- Holy Vindicator, Duelist and Arcane Archer builds are proficient.

## 3. Render delta, 16 builds: tranche/16 vs this branch

Commands:
- Render: `class_census --sheet-dump <build> --with-sheet-rules`, once per build, with each
  side's own binary.
- Classify: `scripts/f1c_render_classify.py`, `f1c_render_pairs.py` and
  `f1c_render_attribute.py` (tranche/16 package vs branch package).
- Population: the 16 builds in §2. Dump failures: 0 before, 0 after. Duplicate lines: 1 before,
  1 after, unchanged.

Status flips (Blocked -> Computed): 11 of 16. These are all the prestige mixes except sentinel,
plus the generic mixes magus/samurai, kineticist/fighter and psion/fighter. Every flip has
matching hand-worked totals in §2. Three of 16 were already Computed on tranche/16
(barbarian/fighter, fighter/wizard, monk/rogue). Their BAB and saves are unchanged. They gain only
`multiclass.hit_points`, `multiclass.skill_points` and the re-scoped `multiclass.<class>.*`
class lines. sentinel and duelist alone stay Blocked, and their `EXPL|` rows are identical.

`LINE|` delta: 52 rows over 20 ids. Every row is `added`, and 0 were changed, removed or lost.

| Mechanism (f1c_render_attribute bucket) | Ids | Rows | Commit |
|---|---:|---:|---|
| new grant edge: ability-category pick rows convert as grants (`racial_traits_human` 16, APG `cmb` 15, `arcane_school_tracker` 4, `bloodline_tracker` 1, `arcane_bloodline_feat_tracker` 1, `monk_bonus_feat_default` 1) | 6 | 38 | F3c3 07f02c6387 |
| new grant edge / new record: PCGen SUBCLASS -> class choice (`psion_egoist`, psychometabolism lines) | 4 | 4 | F3b2 a9d41d21e8 |
| new record / same record, now held: sorcerer bloodline printed from the converted record (`pool_option:sorcerer_bloodline_arcane_bloodline`, `sorcerer_bloodline_arcane(#bonus0)`, 5 arcane-bloodline feature lines) | 8 | 8 | F3c4/F3c4b 5bdf879c24 |
| same record, now held because the mix folds samurai's lines: `exotic_weapon_proficiency`, `exotic_weapon_proficiency_firearms` | 2 | 2 | F3b fold (same lines samurai alone prints on tranche/16) |

`EXPL|` delta: 331 distinct ids and 531 build-rows, grouped by mechanism below:
- **New rows:** only in the 14 builds that are now Computed. They are the fold's totals, the
  combat/defense/skill pillars on the 11 flipped builds, and `multiclass.<class>.*`
  re-scoped class lines.
- **9 changed:** all `combat.weapon_attack_bonus.longsword`. The BAB term moved from +0 on the
  Blocked tranche/16 build to the real multiclass sum. Hand-checked in §2.
- **12 removed:** all in sorcerer 5 + dragon disciple 3. They are
  `class_feature.sorcerer.bloodline.generic.arcane_bloodline.*` generic-pool rows that tranche/16
  emitted without honouring level gates (for example `new_arcana` = 1 and `arcanebondlvl` = 10 at
  sorcerer 5, which are wrong). The pool pass now honours level gates (F3c4b). The
  record-backed `class_feature.sorcerer.arcane_bloodline.*` rows are unchanged.

No unexplained delta remains.

## 4. Tests

| Criterion | Command | Result |
|---|---|---|
| F3.2 `--list` parity | `cargo test --locked -j 8 --test <t> -- --list --format terse` on both trees | `sd18_widening` 891 = 891 IDENTICAL; `sd13_progression` 1,136 = 1,136 IDENTICAL |
| F3.3 sabotage (reproduced) | guard at `multiclass_fold.rs:431` became `if true \|\| ...`; the 45-binary command from `f3d-sabotage-log.md`, filter `multiclass_` | **passed 201, failed 14**; the 14 are exactly the log's 14 Monk tests; EXIT=101 |
| F3.3 restored | `git checkout -- src/rules_core/pilot_compute/multiclass_fold.rs`, same command | **passed 215, failed 0**, EXIT=0 |
| vacuity | `tests/common/mod.rs::assert_multiclass_status_parity` | guards `mix.class_levels.len() >= 2` and `alone < mix`, and asserts both status and the set with the re-scope stripped. The flipped negative controls go red under the sabotage (above), so they are not vacuous |

## 5. Clippy

`cargo clippy --locked --tests -j 8 -- -D warnings`:
- root: exit 0, 0 errors, 0 warnings.
- `crates/codex-ingest`: exit 0, 0 errors, 0 warnings.
- desktop (`--manifest-path apps/desktop/src-tauri/Cargo.toml`): exit 0, 0 errors, 0 warnings.

## 6. Polish and pre-existing defects (not blocking)

1. **Monk flurry in a mix.** The fold copies each class's lines verbatim from its isolated run,
   as §5 of the spec intends. So `multiclass.monk.class_chassis.monk.flurry_of_blows_attack_bonus`
   = 2 on monk 4 + rogue 3 is the monk-alone value (monk level - 2). PF1 flurry in a
   multiclass uses the monk level plus the BAB from other classes, which gives 4 here. This is an
   explanation value, not a printed total (the `Flurry of Blows` LINE prints no number). It is the
   one class line seen whose value depends on the other classes. Register it forward.
2. **Save explanation text.** The detail text of `class_chassis.base_save.*` lists the per-class
   *base attack bonus* terms ("across (class:barbarian 12: base attack bonus 12; ...)") instead
   of the per-class save terms. The values are correct, and only the words are wrong.
3. **Duplicate slugs across books.** `multiclass_fold::all_book_records` resolves a slug found in
   two books by alphabetical book order (`or_insert`). This is not the supersession ruling
   (newest printing wins). Today the 3 duplicates (`cyphermage`, `hellknight`,
   `red_mantis_assassin`) have identical chassis rows, so nothing is wrong yet.
4. **Monk hit-die message (pre-existing data).** The CRB monk's record prints `Hit die d10`
   (corpus `HD: 10`, from `cr_classes.lst:147`), but the PF1 monk's hit die is d8. It is harmless
   today because monk has no chassis record, so its HP is Unknown. The Unknown message says "no
   converted class record states this class's hit points", which is inaccurate. If a monk chassis
   lands, this becomes a wrong HP.
5. **Firearms proficiency on samurai (pre-existing, tranche/16).**
   `ultimate_combat:class_feature:exotic_weapon_proficiency_firearms` is granted by *any* held
   `core_rulebook:feat:exotic_weapon_proficiency`. Samurai alone on tranche/16 already prints
   `Exotic Weapon Proficiency ~ Firearms`, and the magus/samurai mix now carries the same line.
   This is a fabricated proficiency line and is not registered.
6. **Task framing.** The "no converter change" invariant in this batch's brief did not match the
   branch (§0). The converter work carries receipts and passes freshness. The operator should
   accept the converter change explicitly at merge.
