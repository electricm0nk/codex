# F3c2: closing what F3b3 and F3c left (SD-36 Epic F3)

This is an engine-only step. Nothing under `data/corpus/**`, `data/sheet_rules/**` or `site/**`
changed, and there was no converter change (`git diff --quiet data site` is clean). The record
count stays 49,450, and `python3 scripts/pcgen_residue_gate.py --check --closure` passes
(`f3c2-verify.log`). The before-figures come from `census-f3c.json`, taken at head `157dc5496a`.

Census command:
`cargo run --locked -j 8 --bin class_census -- --json docs/release/SD-36-consolidation/artifacts/epic-f/census-f3c2.json`
(committed). It printed:

```
ids=137 computed=62 blocked=1
prestige_swept=74 prestige_alone_blocked=74 prestige_mix_computed=67 prestige_mix_unknown=0
mix_panel_swept=185 mix_panel_computed=185 mix_panel_blocked=0
```

**Per-class diff against `census-f3c.json`.** The comparison covers 322 rows: 63 non-prestige
classes, 74 prestige rows and 185 mix-panel rows. Only 2 of the 322 changed:

| row | F3c | F3c2 |
|---|---|---|
| `class:expert` | Blocked at levels 1-20 (`skill.selected_modifier.class_skill_unknown`) | **Computed** at levels 1-20 |
| `class:dragon_disciple` (prestige) | Unknown (no carrier) | **Blocked** in a sorcerer 5 mix (two blockers, named in §2) |

Every other row keeps its status, its blocked levels and its blocker ids.

## 1. The class-skill regression: Expert closed, Psion named (62 of 63)

### Why the two records did not answer

**Expert.** The converted record does carry the answer. It is split across two rules, and the
record has no edge between them:

- The class line hands out the pick: `core_rulebook:class:expert#bonus4` targets
  `Pool(expert_class_skills)` with the value 10 (`cr_classes.lst:549`,
  `BONUS:ABILITYPOOL|Expert Class Skills|10`).
- The pool's one member is the chooser: `core_rulebook:class_feature:expert_class_skills`
  (`cr_abilities_class.lst:2735`, `CHOOSE:SKILL|ALL`, `CSKILL:LIST`). It is converted to
  `offers: Skills(["all"])` plus `FactGrant(ClassSkillChosen(<own id>))`.
- The converter links a pool to its members only for weapon pools (`pool_link.rs`), so the class
  walk never reaches the chooser.
- The skills themselves are the player's choice (CRB p.450, "any 10 skills").

**Fix (one rule, in `class_skill_sheet_rules::add_canonical_class_skill_picks`).** A class whose
class-skill list is a choice gets canonical-default picks seeded through `class_seeds`, the same
way every other choice is seeded:

- The rule: a canonical seed counts when its choice id is a converted rule that offers
  `OptionSet::Skills` under its own id and grants `ClassSkillChosen(<that id>)`. The picked skill
  is then a class skill.
- A pick the chooser does not admit, or a pick that names no converted skill, makes the class
  answer Unknown by name.
- A class with no such seed keeps whatever its record's walk says.
- The seed is `class_seeds::EXPERT_CANONICAL_CLASS_SKILLS`, recorded under
  `EXPERT_CLASS_SKILL_CHOICE_ID`. It is a Path-A default, like the Commoner's weapon: the first
  ten single skills of the CRB skill list in its alphabetical order, skipping the four families
  (Craft, Knowledge, Perform, Profession) whose pick is itself a second choice. The ten are
  Acrobatics, Appraise, Bluff, Climb, Diplomacy, Disable Device, Disguise, Escape Artist, Fly and
  Handle Animal.

**Psion.** The record really has no answer:

- The Psion's base class skills (Autohypnosis, Craft, Knowledge, Profession, Spellcraft) exist
  only on its discipline `SUBCLASS:` lines (`ultimate_psionics/up_classes.lst:221-248`). The
  converter does not carry those lines; the package holds no rule and no choice for them.
- Each discipline's `<Discipline> Class Skills` record is converted (for example
  `ultimate_psionics:class_feature:psychokinesis_class_skills`, which grants Disable Device and
  Intimidate). None of these records has a granting edge, because the `SUBCLASSLEVEL:1` line that
  grants them is not converted either.

Why nothing engine-side closes it:

- Whether Climb, Intimidate or Swim is a class skill depends on the discipline. Psychokinesis
  grants Intimidate and Psychoportation grants Climb and Swim (`up_abilities_class.lst:408`,
  `:410`). A class-keyed oracle row would therefore print a wrong +3 for some psions.
- A seed has no converted choice to go under.

Closing Psion needs a converter change: convert `SUBCLASS:` as a class choice that carries its
`CSKILL:` and `SUBCLASSLEVEL:` grants. That is outside this batch, because `data/sheet_rules/**`
is frozen here. It is recorded as `forward-scope-register.md` FS-16. The refusal
`skill.selected_modifier.class_skill_unknown` keeps firing for Psion only, because the record
genuinely has no answer.

### RED to GREEN (`tests/sd36_class_skill_from_record.rs`; RED log `f3c2-red.log`)

The test is `expert_prints_its_canonical_class_skill_picks_and_psion_stays_refused_by_name`.

- **RED:** `class_skill_view("expert", 1)` answered `Unknown { "the converted closure of expert
  at level 1 reaches no class-skill grant ..." }`.
- **GREEN:** Expert answers Known with 10 skills and no families. Its receipts at level 1 and
  level 20 carry no `class_skill_unknown`. Psion stays Unknown, and its receipt carries the
  claim-blocking `class_skill_unknown`.

The expected values were worked by hand first, on the shared census fixture: Human, Str 18
(+4), Cha 8 (-1), 1 rank in each skill, chain shirt with ACP -2 (CRB p.150). Climb gets the
class-skill +3 (CRB p.87).

| line | working | asserted |
|---|---|---:|
| Expert 1 and Expert 20, Climb | 1 + 4 + 3 - 2 | 6 |
| Expert 1 and Expert 20, Intimidate | 1 - 1 | 0 |
| Expert 1 and Expert 20, Swim | 1 + 4 - 2 | 3 |

The census-wide scan `every_census_class_prints_the_class_skill_bonus_its_record_grants` still
finds 0 mismatches, and it now includes Expert among the classes whose record answers Known.

**Pins moved** (each logged with `scripts/retro.py correction`,
`docs/retro/events/sd36-f3c2-executor.jsonl`):

- `class_census::tests::census_id_set_matches_the_published_partition`: 61 → **62 of 63**.
- `untabled_…::CLASS_SKILL_REMAINDER`: `[expert, psion]` → `[psion]`.
- `BASELINE_CENSUS_COMPUTED`: 61 → **62**, with a dated reason in `scripts/verify-baselines.env`.

**Desktop.** `pf1_adapter.rs` records no Expert picks. The sheet reads the canonical default
from `class_seeds`, the same contract the proficiency reader already uses (F1c-5 D8). Making the
seeding single-source is F4's work.

## 2. Carriers named by class level, and gate-named options seeded as picks

**How many gates name a class level.** 1 of 74 prestige gates names any class level at all:
`dragon_disciple`, which names `ClassLevel(sorcerer) >= 1` twice. That one class is not a
wizard, cleric or fighter, and no gate names a wizard, cleric or fighter level. No prestige gate
names a `CasterLevel(Class(..))`.

This scan reads every converted `class` record of the 74 prestige slugs in
`census-f3c.json` for a `ClassLevel` term other than the class's own `<=` bookkeeping row. The
script is inline, in the F3c2 session. The same figure is pinned by
`prestige_carrier_distribution_is_measured_and_printed` (`named_names == [dragon_disciple]`).

**Rule (in `class_census.rs`, applied the same way to every gate):**

- **Class-level terms.** `ClassLevel(<class>) >= n` is a translatable numeric axis
  (`NumericAxis::ClassLevel`). A demand that names a class makes that class the carrier
  (`PrestigeCarrier::Named`, or the wizard, cleric or fighter variant when the gate names one of
  those). Its level is at least n, with the usual floor of 5 and the level cap.
- **Class level plus spell kind.** If a named class and a spell-kind demand appear together, the
  gate is refused by name. The named class's casting is not read here, so no single carrier can
  be shown to meet both.
- **Gate-named options.** A `Holds(Rule(<option>))` term translates only when `<option>` is an
  option of a choice that a class named in the same taken branches offers
  (`carrier_pick_for_option`). The chooser comes from that class's own canonical seeds, in either
  of two id spaces:
  - A converted chooser whose member is granted by `Granter::Choice(<chooser>)`.
  - A legacy `choice:<pool>` id, when the option carries the pool as its own TYPE tag and its id
    is `<pool>_<member>`.
- **Seeding.** The resolved pick replaces the carrier's canonical pick for that choice in the mix
  (`input_for_mix_with_picks`).
- **Branch search.** An `AtLeast` clause takes the first n translatable branches, in oracle
  order, whose options settle together. This matches F3c's rule on every gate that has no
  settleable option: the 73 other carriers are unchanged, per the census diff above.

**Dragon Disciple (CRB p.380), `core_rulebook:class:dragon_disciple`.** Branch 3 of the caster
clause is taken: "at least 2 of 2: `ClassLevel(sorcerer) >= 1`, Holds
`sorcerer_bloodline_draconic`". The option is tagged `Sorcerer Bloodline` and has the id
`sorcerer_bloodline_draconic`, so the sorcerer's `choice:sorcerer_bloodline` pick becomes
`bloodline:draconic` in place of the canonical `bloodline:arcane`. The carrier is sorcerer 5 and
the entry gate is `unmet`:

- **Met:** the ClassLevel term, the seeded pick, Knowledge (arcana) 5 ranks.
- **Unmet (printed, never blocking):** the Draconic language and the two race and template
  prohibitions.

Result: the sorcerer 5 / Dragon Disciple 1-10 mix is **Blocked** at every level on two named
lines:

1. `combat.baseline_weapon_proficiency_unknown`. Dragon Disciple's converted closure grants no
   weapon proficiency, and the package carries no closure-complete attestation that it owes none.
   This is a converter attestation, outside this batch.
2. `class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported`. The Sorcerer seam
   grounds the progression of the Arcane bloodline only. Draconic is the bloodline the gate
   names, so the seam refuses rather than printing Arcane's.

**RED to GREEN:** `class_census::tests::dragon_disciple_names_its_sorcerer_carrier_and_seeds_the_draconic_bloodline`.

- **RED** (`f3c2-red.log`): `Err("a clause carries a caster term but no branch of it translates ...")`.
- **GREEN:**
  - carriers `[sorcerer]`;
  - picks `[(sorcerer, choice:sorcerer_bloodline -> bloodline:draconic)]`;
  - reason `took branch(es) [3]`;
  - carrier level 5;
  - the mix input carries exactly one bloodline pick, `bloodline:draconic`.
- **Negative controls:** `eldritch_scion_spells` is offered by no named class, and the wizard
  offers no sorcerer bloodline. Both return `None`.

**Pins moved** (retro correction logged):

- `every_prestige_class_gets_a_carrier_or_is_named_unknown`: ungroundable 1 → **0**.
- `prestige_carrier_distribution_is_measured_and_printed`: unknown 1 → 0, plus a new named
  bucket (named 1, `[dragon_disciple]`).
- `a_prestige_row_referencing_caster_level_…`: dragon_disciple → `Named("sorcerer")`. The
  synthetic spontaneous-caster `AtLeast{3}` gate is still `Err`.
- `f3c_carriers_named_through_at_least_any_and_not_reach_the_engine`: dragon_disciple is Blocked
  on exactly the two ids above.

## 3. Save-formula remainder: 6 of 74, recorded

Six prestige classes stay Blocked on `multiclass.save_shape.unrecognized`:

- evangelist
- exalted
- mammoth_rider
- pure_legion_enforcer
- sentinel
- ulfen_guard

Their oracle `BONUS:SAVE` formulas match no PF1 save form (`f3b3-receipt.md` §3,
`f3c-remainder.md`). Printing the oracle's number would put a save on the sheet that the book
does not state. Repairing a formula from memory would fabricate a row.

The mechanism is an upstream oracle data defect. It can be closed only by a book-cited
override, and it is recorded in two places with the books and pages cited on each class line's
own `SOURCEPAGE:`:

- `forward-scope-register.md` **FS-15**
- `decisions.md` **§14.1**

| class | book, page |
|---|---|
| Evangelist | Inner Sea Gods p.198 |
| Exalted | Inner Sea Gods p.200 |
| Sentinel | Inner Sea Gods p.202 |
| Pure Legion Enforcer | Inner Sea Combat p.32 |
| Ulfen Guard | Inner Sea Combat p.34 |
| Mammoth Rider | Adventurer's Guide p.128 |

## Prestige carrier-mix remainder: 7 of 74, by mechanism (census `prestige_mix_computed=67`)

| mechanism | count of 74 | classes |
|---|---:|---|
| Oracle `BONUS:SAVE` formula defect (FS-15) | 6 | evangelist, exalted, mammoth_rider, pure_legion_enforcer, sentinel, ulfen_guard |
| Named carrier mix Blocked: no weapon-proficiency closure attestation, and a Sorcerer seam that grounds only the Arcane bloodline | 1 | dragon_disciple |
| No nameable carrier | 0 | none (was 1) |

`BASELINE_CENSUS_PRESTIGE_MIX_COMPUTED` stays 67, because the figure did not move. The
re-measurement is dated in `scripts/verify-baselines.env`.

## 4. Docs and verification

- `docs/architecture/status.md` class table was regenerated with
  `python3 scripts/gen_class_status_table.py --json …/census-f3c2.json`, and `--check` returns
  OK. The prestige paragraph was updated.
- `docs/architecture/rules-engine.md`: the class-skill paragraph now covers Expert.
- `python3 scripts/check_class_census_baselines.py` with the env floors returns OK: ids 137,
  computed 62, alone-blocked 74, mix 185, prestige mix 67.

Verification (`f3c2-verify.log`, every command on the final tree):

| command | result |
|---|---|
| `cargo test --locked -j 8 --lib class_census -- --test-threads=8` | 33 of 33 |
| `cargo test --locked -j 8 --lib feat_pillar_and_pool_aggregation -- --test-threads=8` | 9 of 9 |
| `--lib untabled_base_class_features` / `--lib class_skill_sheet_rules` / `--lib multiclass` | 42 / 4 / 37, all pass |
| `--test sd36_class_skill_from_record` / `--test sd36_multiclass_any_class` | 3 of 3 / 11 of 11 |
| `cargo test --locked -j 8 --lib -- --test-threads=8` (widening check) | 2,723 passed, 0 failed, 6 ignored |
| `cargo clippy --locked --tests -j 8 -- -D warnings` (root) | clean |
| `python3 scripts/pcgen_residue_gate.py --check --closure` | PASS (shipped_scanned 69,720, hits 0) |
