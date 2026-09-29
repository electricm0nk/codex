# F3c4: sorcerer bloodlines beyond Arcane (SD-36 Epic F3). BLOCKED, converter change needed

**Status: blocked.** This step made no engine change and no converter change. `data/corpus/**`,
`data/sheet_rules/**` and `site/**` are untouched. The record count is still 49,450, and the census
and baselines are unchanged (`census-f3c3.json` still holds).

The brief asked the Sorcerer module to hand every bloodline it does not model to the CONVERTED
bloodline record through the held-set path. In the current package, that path reaches **none** of a
bloodline's lines. The data that turns those lines on sits on an oracle row outside the
49,450-unit inventory, so a converter change is needed.

The batch invariant says no converter change ("if you believe one is needed, STOP and say why"), so
the step stops here. Measured on `a9d41d21e8`.

## 1. What was measured

The measurement is in `f3c4-bloodline-sweep.md`. The commands and logs sit next to it.

- **Denominator:** **32** converted bloodline records, `<book>:class_feature:sorcerer_bloodline_<x>`,
  from 6 books. Arcane is 1 of them, and 31 are bloodlines the bespoke module does not model.
- **Single-class sorcerer, canonical seed, levels 1..=20:**
  - **1 of 32** is Computed at every level: Arcane.
  - **31 of 32** are Blocked at every level. Each is blocked on
    `class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported` and on nothing else.
- **Lines the records grant:** 287 `granted_by` edges name a record. **287 of 287** are gated on a
  variable.
- **Held-set probe:** held with the sorcerer class at level 20, the record reaches **0 of 287** of
  its lines. Arcane is included in that zero.

## 2. The mechanism (worked on Draconic, CRB p.75)

**The record grants each line behind a variable.** `Sorcerer Bloodline ~ Draconic`
(`cr_abilities_class.lst:2434`) grants its class skill with
`ABILITY:Class Skill|AUTOMATIC|Perception|PREVARGTEQ:Sorcerer_Draconic_BloodlineClassSkill1,1`. It
gates its arcana, bonus spells and each power level the same way.

**The pick row raises those variables, and it is not converted.** The pick row is
`Draconic Bloodline`, `CATEGORY:Sorcerer Bloodline` (`:2435`). It sets
`BONUS:VAR|Sorcerer_Draconic_BloodlineClassSkill1|if(Sorcerer_CF_BloodlineClassSkill==0,1,0)`, the
bloodline level and the progression level. That row is outside the inventory:

- 27 of the 32 bloodlines appear as `Sorcerer Bloodline|<X> Bloodline` rows in
  `_defects/unresolved-references.json` (65 of its 7,119 rows).
- Every one of these variables' `_vars` file records the row under `outside_corpus_rows`.
- It is mechanism E, `forward-scope-register.md` FS-12.

**Nothing else a plain sorcerer holds sets the variables.** Across the 287 edges:

- 166 edge-vars are fed only by Ultimate Magic wildblooded/crossblooded records.
- 112 have no contribution.
- The Draconic ones are also fed by Dragon Disciple's `Draconic Bloodline ~ Standard` (`:2976`),
  which sets `ClassSkill1` to `if(SorcererLVL>=1,0,-1)`, which is 0 for a sorcerer.

The engine reads the variables at their `DEFINE` default, 0, so every line stays off.

## 3. Why an engine-only version would print a wrong number

Dropping the blocker and letting the record stand gives a bloodline Sorcerer that is "Computed"
but is missing:

- its bloodline class skill, from the union;
- its arcana;
- its powers;
- its bonus spells.

The class skill reaches a printed total. The Aquatic bloodline's class skill is Swim
(`apg_abilities_class.lst:3088`, `ABILITY:Class Skill|AUTOMATIC|Swim`). On the census fixture, a
flipped Aquatic sorcerer would print Swim as 1 + 4 − 2 = **3**, where PF1 gives 1 + 4 + 3 − 2 =
**6**. The +3 comes from CRB p.87, and the −2 is the chain shirt's armor check penalty (CRB p.150).
That is the wrong computed number the doctrine forbids.

Engine-side alternatives were considered and each was rejected:

- **Assume the variables are 1.** That encodes the pick row's semantics from memory.
- **Key the class skill by bloodline.** That is per-bloodline code.
- **Mark the gate Situational.** Then no class skill is granted, and a Swim line is again printed
  without the +3 or refused.

None of these is the ONE mechanical rule the doctrine requires.

## 4. Dragon Disciple

Dragon Disciple's carrier mix is sorcerer 5 with `bloodline:draconic`, prestige levels 1..=10
(`census-f3c3.json`). It carries **two** claim-blocking diagnostics at 10 of 10 levels:

1. `class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported` (§2 above).
2. `combat.baseline_weapon_proficiency_unknown`. This is Dragon Disciple's own closure defect,
   `Internal|Bite` (`ce_abilities_race.lst:249`), an oracle row outside the inventory (mechanism N,
   `f3c3-receipt.md` §4).

So even a converter fix for the bloodline pick rows leaves the mix Blocked on (2). The brief's
expected "prestige mixes 67 → 68 of 74" is not reachable from this step alone.

The prestige mix figure stays **67 of 74**. The named remainder is 7:

- the 6 FS-15 oracle save-formula defects;
- Dragon Disciple, on the two mechanisms above.

## 5. What would close it

This is a converter batch, recorded as `forward-scope-register.md` FS-17.

- **Converter.** Carry each bloodline's `CATEGORY:Sorcerer Bloodline` pick row. These are 27 rows
  in CRB/APG/UM/ACG plus the 5 `support/*_um.lst` rows. Carry them as the members of the
  `sorcerer_bloodline` pool that `sorcerer_standard_bloodline_selection` already offers, so their
  `BONUS:VAR` contributions and their `ABILITY:...|AUTOMATIC|Sorcerer Bloodline ~ <X>` grants join
  the package.
- **Engine.** In the same batch, link the character's `choice:sorcerer_bloodline -> bloodline:<x>`
  to that pool member, the way `class_seeds` already links Psion's discipline in F3c3. The
  class-skill reader and the print path then answer through the held set with no per-bloodline
  code. The bespoke Arcane module keeps winning where it computes.
- **Acceptance.** Re-run `scripts/f3c4_bloodline_sweep.rs`. It should show lines held beyond the
  record, and the Aquatic Swim +3 should be asserted by hand.

## 6. Also observed (pre-existing, not changed)

`push_generic_pool_group_selection_magnitude` (SD-32, `prestige_class_features_campaign.rs:6096`)
emits `class_feature.sorcerer.bloodline.generic.*` member magnitudes without a level gate:

- A Draconic sorcerer 5 gets `...breath_weapon.sorcererdraconicbreathweapondc = 14`, although
  Breath Weapon is a 9th-level power.
- The same sorcerer gets `...power_of_wyrms.blindsenserange = 60`, although Power of Wyrms is a
  20th-level power.
- It gets `...dragon_resistances...naturalarmorbonus = 2`, while the bespoke, level-correct value
  is 1.
- The Arcane sorcerer 5 that is already Computed gets `...arcane_apotheosis...power3 = -1`.

None of these feeds a sheet total. The bespoke `draconic_bloodline.dragon_resistances.*` records
are the values that reach AC. Whoever closes FS-17 must decide this pass's fate before a
non-Arcane bloodline flips to Computed. This step did not change it: it is pinned by
`pool_groups.rs` tests and shared by every class that uses the pass.

## 7. Verify

- There is no code change, so there is no cargo gate to re-run.
- The temporary sweep test `tests/zz_probe_f3c4.rs` was deleted. It is kept as
  `artifacts/epic-f/scripts/f3c4_bloodline_sweep.rs`.
- `python3 scripts/pcgen_residue_gate.py --check --closure`: **PASS** on the final tree
  (`identifier_hits=0`, `shipped_data_hits=0` of 69,762 scanned, `live_hits=0`).
- The worktree is clean after the commit (unfiltered `git status`).
