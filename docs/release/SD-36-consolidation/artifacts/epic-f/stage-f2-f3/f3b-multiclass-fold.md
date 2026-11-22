# F3b — multiclass gate + fold for every class with a chassis (SD-36 Epic F3, acceptance F3.4)

Spec: `epic-f-class-completion.md` §5. Code: `src/rules_core/pilot_compute/multiclass_fold.rs` (new),
`class_occult_and_psionic.rs` (`is_supported_multiclass_mix`, `table_class_level_supported`,
`compute_multiclass_base_chassis`), `class_chassis_sheet_rules.rs` (`ClassChassis::save_value_exact`),
`generic_class_chassis.rs` (`record`), `class_shared_core.rs` (fold call),
`feat_pillar_and_pool_aggregation.rs` (weapon-proficiency union).

## Oracle first

`f3b-hand-worked.md` was written before `tests/sd36_multiclass_any_class.rs` first ran. The engine
matched every hand-worked BAB, base save, total save and HP figure on the first GREEN run. The
expectation file was not edited after the engine ran: 0 mismatches over 4 mixes x 8 figures.

## RED -> GREEN

Command: `cargo test --locked -j 8 --test sd36_multiclass_any_class -- --test-threads=8`

| Stage | Result | Log |
|---|---|---|
| RED (gate CRB-11 only, no fold) | 0 of 8 passed. 3 mixes Blocked on `class_chassis.unsupported` + 3 pillar diagnostics; Barbarian 12 / Fighter 1 reached Computed but had no `multiclass.hit_points`; no `multiclass.<class>.*` lines; prestige-only mix and Sentinel mix carried only the generic fallback | `f3b-red.log` |
| Sabotage: fold a non-table class's saves with the base-class closed form instead of its own `Expr` | 2 of 8 fail by number (Fighter 6 / Arcane Archer 3 base saves 8/5/3 vs hand-worked 7/4/3; Wizard 5 / Loremaster 2 2/2/7 vs 2/2/6) | `f3b-sabotage.log` |
| Union RED: a class with no proficiency answer beside Fighter (Fighter 6 / Loremaster 2) | the new union test fails while the old "any Unknown => Unknown" rule is in place | `f3b-union-red.log` |
| GREEN | 9 of 9 | `f3b-verify.log` |

## What the four mixes print (shared census fixture; working in `f3b-hand-worked.md`)

| Mix | Status | BAB | Base F/R/W | Total F/R/W | HP | Class skill points |
|---|---|---:|---|---|---:|---|
| Barbarian 12 / Fighter 1 | Computed | 13 | 10/4/4 | 12/6/5 | 121 | Unknown (PF1: 50) |
| Fighter 6 / Arcane Archer 3 | Computed | 9 | 7/4/3 | 9/6/4 | 76 | Unknown (PF1: 24) |
| Magus 4 / Samurai 2 | Computed | 5 | 7/2/4 | 9/4/5 | 47 | Unknown (PF1: 16) |
| Wizard 5 / Loremaster 2 | **Blocked**, one blocker | 3 | 2/2/6 | 4/4/7 | 44 | Unknown (PF1: 18) |

**Wizard 5 / Loremaster 2 does not reach Computed (acceptance not met for this one mix; named).**
Its only claim-blocking diagnostic is `combat.baseline_weapon_proficiency_unknown`. Wizard grants
no longsword (CRB p.77). Loremaster "gains no proficiency with any weapon or armor" (CRB p.385), but its
converted principal carries no `closure_complete` attestation. So the proficiency reader answers
Unknown, not Known-empty, and the baseline Longsword attack's -4 cannot be decided. 32 of 77 prestige
class records are unattested (`python3` walk of `data/sheet_rules/*/class/*.json` principal
`closure_complete`, 56 of 189 class records attested). The attestation is set by the converter
(`crates/codex-ingest/src/pcgen_import/sheet_rule/attest.rs`) and would mean regenerating
`data/sheet_rules/**`, which this batch does not touch. Every other figure on that sheet computes and
matches the hand-worked value.

**Skill points: Unknown for every class in every mix (named, not 0).** No converted class record
carries skill ranks per level: 135 of 135 chassis records (F3a), 189 of 189 class files. The corpus
has no `STARTSKILLPTS` token either (`grep -rl STARTSKILLPTS data/corpus | wc -l` -> 0). The fold pushes
`class_chassis.skill_points.unknown` once per class (non-blocking) and prints no total. Reaching the
PF1 values needs the ingest to carry `STARTSKILLPTS` through to the converted class record. That is a
`data/corpus/**` + `data/sheet_rules/**` change, outside this batch.

## The fold (one rule; no class named)

- **Gate** (`multiclass_fold::multiclass_member`): a non-prestige member's isolated single-class input
  passes `has_supported_class_chassis`, or a prestige member has a converted chassis row at that level.
  Every save needs a Good/Poor source: the CRB table (`good_saves_for`, the pre-F3b reading) or else
  `ClassChassis::save_shape`. `Degraded` / `Unrecognized` / no record gives the named
  `multiclass.save_shape.{degraded,unrecognized,unknown}` (claim-blocking), never a silent poor save.
  A mix needs one non-prestige member, or it gets `prestige_class.requires_base_class_levels`.
  `table_class_level_supported` keeps the old table-only body for the single-class path only.
- **BAB**: sum of each member's own BAB (isolated chassis, or the prestige row).
- **Saves**: exact `Rat` sums of each member's own save value, floored once. That value is
  `level/2+2` / `level/3` for a table class, or the class's own converted `Expr` (a prestige class
  folds `(level+1)/2`, not `level/2+2`). The pre-F3b fold summed `f64`s.
- **HP** (`multiclass.hit_points`): `ClassChassis::hit_points` per member. The maximized die applies
  only to the first-listed class's first level. Con comes from the base (pre-rage) modifiers. A
  member with no record or no hit die gives `class_chassis.hit_points.unknown` and no total.
- **Class lines**: each member's isolated run, `class_feature.*` / `class_spell.*` /
  `class_chassis.<class>.*`, re-scoped `multiclass.<class>.<id>`. A line is skipped if the mix already
  printed that id, and so is a class's `.level_1_hit_points`. Its claim-blocking class lines carry
  over the same way, so a class that cannot compute alone does not compute in a mix. Pillar
  diagnostics (combat / defense / skills / chassis gate) do not carry over, because the mix computes
  those itself.
- **Prestige entry requirements**: `multiclass.prestige_entry_gate.{met,unmet}`, non-blocking.
- **Weapon proficiency union**: one class that grants the weapon now decides it. A class with no
  answer leaves the verdict Unknown only when no class grants it. Before this step, Fighter 6 /
  Aldori Swordlord 3 was Unknown on the longsword.
- **Class skills**: already a union over `class_levels` (`selected_skill_*_is_class_skill`). Unchanged.

## Census (`cargo run --locked -j 8 --bin class_census -- --json`; artifact `../census-f3b.json`)

| Figure | F2b | F3b | Denominator |
|---|---:|---:|---|
| ids | 137 | 137 | — |
| non-prestige Computed at every level | 63 | 63 | of 63 (0 status changes, per-id diff) |
| prestige alone Blocked | 74 | 74 | of 74 |
| prestige carrier mix Computed | **0** | **56** | of 74 (56 Blocked -> Computed) |
| prestige carrier mix Unknown | 11 | 11 | of 74: no nameable carrier (entry gate states its caster-level/spell term only inside AtLeast/Not) |
| prestige carrier mix Blocked | 63 | 7 | of 74 |
| mix panel Computed | 185 | 185 | of 185 |

The 7 still Blocked, by mechanism:
- 4 on `multiclass.save_shape.unrecognized`: Exalted, Mammoth Rider, Sentinel, Ulfen Guard. These
  are F3a's named source formulas such as `classlevel+1/2`. The other two of F3a's six Unrecognized
  records, Evangelist and Pure Legion Enforcer, are in the 11 with no carrier.
- 3 on `combat.baseline_weapon_proficiency_unknown`: Cyphermage (Wizard carrier), Magaambyan
  Arcanist (Wizard), Mystic Theurge (Wizard, Cleric). The carrier grants no longsword and the
  prestige record is unattested, the same mechanism as Loremaster above.

`BASELINE_CENSUS_PRESTIGE_MIX_COMPUTED` is not added here. The brief leaves it to F3c.

## Negative controls

- `sd18_widening` 891 of 891 and `sd13_progression` 1,136 of 1,136 pass unchanged. Their mixes still
  block, because each class blocks alone on those fixtures and its blockers carry over. The test
  lists are unchanged; no test was added or renamed there.
- `class_dispatch` lib tests `an_{apg,acg}_class_multiclassed_with_fighter_stays_blocked` asserted a
  mix of those families could never compute ("deliberately not registered"). That premise is gone.
  They are renamed `..._computes_exactly_when_it_computes_alone` and assert the equivalence: mix
  status == the class's own status on the same input, and every mix blocker is that class's
  re-scoped line. ACG, measured: 5 of 10 compute at 4th + Fighter 1 on that fixture (Brawler, Hunter,
  Skald, Slayer, Swashbuckler), 5 block on their own spell/feature lines. APG: 0 of 6.

## Desktop consequence (the loophole this closes)

`pf1_adapter::tests::monk_multiclass_dip_reaches_computed_from_apply_level_up_alone` went red. Before
F3b, a Fighter who dipped Monk with no Monk bonus-feat choice reached Computed. That only happened
because the mix never ran Monk's own seam, so Monk's "no bonus feat chosen" blocker was silently
dropped. Since F3b the mix carries it. `apply_level_up`'s dip branch now mirrors the creation-time
Monk seed, as it already did for Wizard and Arcanist. The test is unchanged and green again. Dips into
other chooser classes the dip branch does not seed (Cavalier, Inquisitor, Oracle, ...) now block on
that class's own line, which is honest. Seeding them is a desktop follow-up and is not done here.

## Verify (`f3b-verify.log`)

- `cargo test --locked -j 8 --test sd36_multiclass_any_class -- --test-threads=8`: 9 of 9
- `cargo test --locked -j 8 --test sd21_multiclass_fighter_wizard_chassis_computes --test sd24_multiclass_integration -- --test-threads=8`: 7 + 5 (F3.4)
- `cargo test --locked -j 8 --lib multiclass -- --test-threads=8`: 33 of 33 (full `--lib`: 2713 passed, 6 ignored)
- full root `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8`: 288 test binaries, 0 FAILED
- desktop `cargo test --locked -j 8 --no-fail-fast --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8`: 615 of 615
- `cargo clippy --locked --tests -j 8 -- -D warnings`, root and desktop manifests: clean
- `python3 scripts/pcgen_residue_gate.py --check --closure`: PASS. `data/**` and `site/**` untouched.
