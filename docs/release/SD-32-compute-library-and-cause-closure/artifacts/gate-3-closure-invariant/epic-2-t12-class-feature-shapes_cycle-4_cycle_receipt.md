# Cycle epic-2-t12-class-feature-shapes-cycle4 — Gate 3 (closure invariant) / Card 11 (row 15's T12 remainder)

- **Card ID:** 11 (`epic-2-cause-closure`), row 15's named T12 sub-scope: the last 53 magnitude-bearing
  class-feature records across 10 classes (per the dispatch brief), plus `psion`'s genuinely-third
  convention.
- **Commit SHA:** see `git log -1` at push time (rebased onto `origin/tranche/12` before pushing, §5)
- **Files touched:**
  - `src/rules_core/rules_tables/occult_adventures/{kineticist,medium,mesmerist,occultist,psychic,
    spiritualist}_features.rs` — **new**: 6 modules, 6+9+14+7+4+3 = 43 real per-feature compute
    functions across the six `occult_adventures` classes sharing `oa_abilities_class.lst`
  - `src/rules_core/rules_tables/ultimate_magic/magus_features.rs` — **new**: 6 real per-feature compute
    functions for Magus
  - `src/rules_core/rules_tables/ultimate_wilderness/shifter_features.rs` — **new**: 6 real per-feature
    compute functions for Shifter
  - `src/rules_core/rules_tables/ultimate_intrigue/vigilante_features.rs` — **new**: 9 real per-feature
    compute functions for Vigilante
  - `src/rules_core/rules_tables/{occult_adventures,ultimate_magic,ultimate_wilderness,
    ultimate_intrigue}/mod.rs` — registered the 9 new modules
  - `src/rules_core/pilot_compute/mod.rs` — 9 new `ground_<class>_class_features` functions, dispatch
    wiring in `compute_class_chassis`'s `untabled_base_class_chassis::resolve` arm, 9 new end-to-end
    wiring tests (level-20 magnitude tests) plus 9 new cases added to the shared
    `each_new_class_lacks_its_highest_gated_magnitude_one_level_early` table (now 18 total)
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 11 cycle entry prepended
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped diff, this cycle's touched files only — the
  only matches are this cycle's own `SD-32 card 11 (T12), cycle 4` module doc-comment citations, the
  same convention cycles 1-3 used in their own files)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope — `grep -inE
  "TODO|FIXME|unimplemented!|not.?implemented|placeholder|stub|would have|success: true"` over every new
  and modified file: 0 hits; every new function is a real formula transcribed from the corpus's own
  already-ingested tokens and reaches `compute_pilot_base_chassis` through the real dispatch path, proven
  by the wiring tests)
- **Acceptance criterion:** re-derive the 53-record/9-class remainder honestly, close all nine remaining
  classes end-to-end following the established template, and report `psion`'s status precisely.
- **Corpus SHA:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`)
  — every formula in this cycle was re-derived directly against this pinned oracle checkout (bootstrapped
  fresh into this worktree's repo-local slot,
  `docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/`, per
  this bundle's operator directive — never `~/workspace/repos/pcgen`), no `data/corpus/**` write.
- **Status:** complete (this cycle's scope only). **All 108 of 108 magnitude-bearing
  `untabled_base_class_feature_roster` records are now closed across all 19 classes.** `psion`'s
  genuinely-third convention (absent from that roster fixture entirely — `class_id=psion` has 0 entries)
  remains sized, not closed, from an earlier cycle; untouched this cycle, named explicitly below.
- **Discovery forwards:** none new beyond the four documented judgment calls in §3 below.
- **Next-cycle plan:** scope and close `psion`'s own-named class features, which use a genuinely
  different (third) grant convention — `ABILITY:Psion Class Feature|AUTOMATIC|Psion Manifesting`, no
  `"Psion ~ "` group-separator prefix at all, chaining through further per-discipline `ABILITY:`
  indirection (e.g. `Clairsentience Discipline` itself granting `Psion Class Feature|AUTOMATIC|
  Clairsentience ~ ...` rows). Registered book: `ultimate_psionics` (`up_classes.lst`), confirmed by
  cycle 1's own investigation, not the superseded `psionics_unleashed` an earlier brief mis-cited.

---

## 1. Re-deriving the remainder (§17a — re-derive, don't inherit)

```bash
python3 -c "
import json, collections
d = json.load(open('tests/fixtures/rules_core/untabled-base-class-feature-roster.json'))
mag = [e for e in d['entries'] if not e['text_only']]
print('total:', len(mag))
print(collections.Counter(e['class_id'] for e in mag))
"
```
```
total: 108
Counter({'mesmerist': 10, 'aegis': 7, 'antipaladin': 7, 'medium': 7, 'vigilante': 7, 'cryptic': 6,
'dread': 6, 'kineticist': 6, 'occultist': 6, 'tactician': 6, 'vitalist': 6, 'magus': 5, 'marksman': 5,
'shifter': 5, 'wilder': 5, 'psychic': 4, 'soulknife': 4, 'psychic_warrior': 3, 'spiritualist': 3})
```
Confirmed **108 across 19 classes**, unchanged from cycle 3 — no sibling lane touched this fixture since
the base commit. 55 already closed (Antipaladin 7, Cryptic 6, Dread 6, Marksman 5, Psychic Warrior 3,
Soulknife 4, Aegis 7, Tactician 6, Vitalist 6, Wilder 5). This cycle's target: the remaining 53 across
Kineticist (6), Medium (7), Mesmerist (10), Occultist (6), Psychic (4), Spiritualist (3), Magus (5),
Shifter (5), Vigilante (7) = 53, exactly matching the dispatch brief's figure.

`psion` confirmed to have **zero** entries in this roster fixture at `class_id=psion` — it was never
"53 of 108"; it is a wholly separate, unsized-in-this-fixture population, consistent with cycle 1's
finding that its own-named features use a structurally different grant convention this roster's census
script does not walk.

## 2. Oracle bootstrap (a fresh worktree's slot is empty)

```bash
scripts/fetch-pcgen-oracle.sh --dest "$PWD/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen"
```
```
pcgen-oracle: OK 7f818006e371188e5717fd18d74d18a420747fc6 .../artifacts/corpus/operator-supplied/pcgen
```
All formulas below were re-derived directly against `oa_abilities_class.lst` (six classes),
`um_abilities_class.lst`/`um_classes.lst` (Magus), `uw_abilities_class.lst`/`uw_classes.lst` (Shifter),
and `ui_abilities_class.lst`/`ui_classes.lst` (Vigilante) in this repo-local checkout, cross-referenced
against each record's already-ingested `data/corpus/<book>/class_feature/<class>/*.json` for its
`raw_tokens` and `wiring_class`/`wiring_class_signals` fields.

## 3. Every one of the 53 records fits the four established shapes, plus three DESC-prose-only
magnitudes and one genuinely novel size-keyed shape

- **Flat/constant:** `kinetic_blast.range_feet`=30, `seamless_guise.bonus`=20,
  `vigilante_specialization.pool`=1, `shared_consciousness.focus_pool`=1, `psychic_discipline.pool`=1.
- **`level`-scaled linear/floor-division:** the large majority — e.g. `spirit.bonus`=1+level/4,
  `magus_arcana.pool`=level/3, `social_talent.count`=(level+1)/2, `phrenic_amplifications.count`=
  1+((level-1)/4).
- **`level + ability_modifier`:** e.g. `wild_talents.dc`=10+level/2+CON, `mesmerist_trick.dc`=
  10+level/2+CHA, `vigilante_talent.dc`=10+level/2+CHA, `frightening/stunning_appearance.dc`=
  10+level/2+CHA — the same `10+level/2+ability` shape cycles 2-3 already closed repeatedly.
- **`ability_modifier`-only (no level term):** `towering_ego.bonus`=CHA,
  `armor_proficiency.level`=level (pure pass-through, no ability at all — same shape as Vitalist's
  Health Sense).
- **Three magnitudes whose real formula lives only in the record's own `%N`-substituted DESC prose, no
  machine `BONUS:` token exists for them at all** — `Kineticist ~ Burn`'s "3 + her Constitution
  modifier" burn cap, `Spiritualist ~ Calm Spirit`'s "once per day... an additional time every 4 levels
  (max 4 at 19th)" uses/day, `Vigilante ~ Frightening/Stunning Appearance`'s `%1`/`%2`-substituted DC/HD
  formulas. Grounded exactly as the prose states, documented as such, not fabricated (`§1a`).
- **One genuinely novel shape, sized and closed, not excluded (`§27b`): `Shifter Claws`' base damage
  value is keyed on the character's resolved creature size** (`PRESIZEEQ:S/M/L` rows in the oracle),
  combined with three level thresholds. Resolved via `race_size_for_race_token`, the same
  size-resolution mechanism `combat_size_modifiers` already uses for AC/CMB/CMD — no new mechanism
  needed, just a new caller.

No record in this cycle resisted all four established shapes plus the DESC-prose and size-keyed
extensions above; nothing was filed as a blocker or exclusion.

## 4. Four documented judgment calls

1. **Favored-class-bonus (FCB) terms dropped, four Mesmerist/Occultist formulas.** This engine has no
   FCB input anywhere in `CharacterInput`. Every affected formula grounds the FCB-less base value (the
   correct value for a character who has not invested a favored-class bonus into that feature),
   documented in each function's own doc comment.
2. **`Phrenic Pool`'s ability term is discipline-dependent.** The record's own `BONUS:VAR|PhrenicPool|
   (PsychicLVL/2)+PhrenicPoolAbility` combines with 9 separate Psychic Discipline records, each setting
   `PhrenicPoolAbility` to CHA (4 disciplines) or WIS (5 disciplines) via its own `BONUS:VAR` token. This
   engine does not yet track which discipline a Psychic has chosen; Charisma is supplied as the
   documented default.
3. **Defensive Instinct grounds the unencumbered case.** The oracle's own `ENCUMBERANCE`-conditional row
   grants the identical value when unencumbered; the encumbered branch is not modelled, matching this
   engine's existing unencumbered-by-default combat baseline.
4. **Elemental Focus and Magus Arcana each carry two independent `BONUS:VAR` tokens on different
   variables.** The level-scaled one is grounded as the record's real magnitude; the trivial pass-
   through/flat-pool-of-1 companion token is skipped as duplicate of the feature grant itself — the same
   discipline cycle 3 documented for Aegis's Reconfigure.

None of these four is a stub: each grounds a real, correctly-derived value for the common/default case,
with the simplification named and reasoned rather than silently made.

## 5. Nine classes closed end-to-end, real compute + real wiring

Built real per-feature compute functions for all magnitude-bearing records of Kineticist (6), Medium
(7), Mesmerist (10), Occultist (6), Psychic (4), Spiritualist (3), Magus (5), Shifter (5), and Vigilante
(7) — 53 records, 64 compute functions (several records carry more than one independently formula-
bearing token, same pattern cycle 3 established for Aegis/Vitalist). Every formula transcribed from the
corpus's own already-ingested tokens (`data/corpus/{occult_adventures,ultimate_magic,
ultimate_wilderness,ultimate_intrigue}/class_feature/<class>/*.json`), cross-checked against the oracle's
own `.lst` files.

Wired into `compute_class_chassis` via nine new `else if` arms on `class_level.class_id`, immediately
following the existing nine-class `ultimate_psionics` arms. `ground_shifter_class_features` takes
`input: &CharacterInput` (an established pattern, e.g. `ground_or_block_alchemist_mutagen`) to resolve
the character's size via `race_size_for_race_token(&input.chosen.race_id)`.

## 6. RED→GREEN, proven live at both altitudes (§1a)

Mutated `kineticist_features::wild_talents_dc` to add `+99`:
```
test ...kineticist_features::tests::wild_talents_dc_combines_level_and_constitution ... FAILED
  left: Some(122)
 right: Some(23)
test ...kineticist_level_20_reaches_every_real_magnitude_with_a_real_constitution_score ... FAILED
  left: 122
 right: 23
```
Both the unit-level compute test and the end-to-end wiring test failed for the intended reason.
Reverted, re-ran the full targeted scope:
```bash
cargo test --locked --lib -- kineticist medium_features mesmerist_features occultist_features \
  psychic_features spiritualist_features magus_features shifter_features vigilante_features \
  kineticist_level_20 medium_level_20 mesmerist_level_20 occultist_level_20 psychic_level_20 \
  spiritualist_level_20 magus_level_20 shifter_level_20 vigilante_level_20 each_new_class_lacks \
  a_class_with_no_roster_data aegis tactician_features vitalist_features wilder_features \
  antipaladin cryptic_ dread_ marksman_ psychic_warrior_ soulknife_
```
`146/146 green`, no regressions in any pre-existing Antipaladin/Cryptic/Dread/Marksman/Psychic
Warrior/Soulknife/Aegis/Tactician/Vitalist/Wilder test.

## 7. Sweep (§3)

`grep -rn "55 of 108\|55/108\|53 of 108\|53 magnitude" tests/ src/ scripts/ apps/` — no count-pinning
hits outside this cycle's own new doc comments and the kanban entry; this cycle changed which records
are *closed*, not the roster's own fixed total (108/19), so no fixture count-pinning file needed
updating.

## 8. Scope discipline

Did not attempt: `psion`'s genuinely-third convention (named precisely in §"Next-cycle plan" above, not
filed out of scope); row 15's own 27,847 kind-unenumerable-object scope (untouched, different sub-lane).
No class was filed as out of scope — every one of the nine targeted classes closed whole, no fragments.

Row 11 left `in-progress` per dispatch instruction — `psion` remains.

`df -h /`: reported in the dispatch's final report.
