# Equipment/Spell Content-Completion Log — Epic 6 Criteria 6.2–6.5 (CRB)

> This cycle's granted file-touch set (`loop-instruction.md §2.4`'s Epic 6
> row, corrected per the per-book module shape): CRB only --
> `src/rules_core/rules_tables/crb/equipment_tables.rs`,
> `src/rules_core/rules_tables/crb/equipment_data/`,
> `src/rules_core/rules_tables/crb/spell_list.rs`. APG, ACG, and Bestiary 1
> remediation are separate, disjoint-file-touch follow-on cycles (their own
> `content-completion-log.md` entries), per the audit's own gap list.

## 1. Criterion 6.2 — cost / record coverage (CRB)

**Finding: no remediation needed.** This cycle's re-audit found both of
criterion 6.1's CRB record-coverage gaps were not real gaps:

- **CRB equipment (2977/2977 records):** already 100% entering this cycle
  (unchanged, re-confirmed).
- **CRB spells:** criterion 6.1's "652/675 = 96.6%" figure was a
  measurement error, not a genuine gap. The `675` count came from a crude
  `awk '{print $1}' | sort -u` scan that (a) counted 12 `.COPY=`-suffixed
  rows as distinct spell names instead of merging them into their base
  spell (the same merge rule equipment records already apply), and (b)
  included 10 `TEMPBONUS:`/`TEMPVALUE:` sub-choice records (e.g. `Resist
  Energy (Acid)`, `Desecrate (Standard)`) that carry no `SCHOOL:`/
  `CLASSES:` token at all and are not independent spells, plus one
  `SOURCELONG:` header line miscounted as a spell name. The real,
  level-and-school-bearing spell count in `cr_spells.lst` is **652**,
  exactly matching `SPELL_LIST`'s existing ingest. **CRB spell record
  coverage was already 100%.** See `spell_list.rs`'s corrected doc comment
  and `SpellFieldCoverage::records_expected`'s doc comment.

CRB equipment `cost_gp: None` rows (1715 of 2977) were individually spot-
audited by category: every `None` corresponds to a real corpus absence --
either a `(Base)`-template row with no independent price (its variants
carry the real price via `.COPY=`) or a `cr_equipmods.lst` modifier whose
cost is a formula over the base item's own cost/charges/caster level
(`BASECOST*...`, `%CHARGES`-driven, etc.), not a fixed gp number the
schema can represent. No fabrication; no remediation needed.

## 2. Criterion 6.3 — weight field (CRB)

**Schema change + population, CRB only.** Added `weight_lbs: Option<f64>`
to `EquipmentTableEntry` (`equipment_tables.rs`), populated from each
record's real corpus `WT:` token (`equipment_data/{general,arms_armor,
magic_items,equipmods}.rs`, matched by `KEY:` against the live PCGen
corpus).

| Category | Records | `weight_lbs` populated | Honest ceiling reason for the remainder |
|---|---:|---:|---|
| General | 453 | 379 | 74 `(Base)`/no-independent-weight rows have no `WT:` token in the corpus |
| Arms/Armor | 310 | 137 | 173 `(Base)`-template rows (their `.COPY=` variants carry the real weight) |
| Magic Items | 1556 | 1495 | 61 rows have no `WT:` token in the corpus |
| Equipmods | 658 | 0 | equipment *modifiers* have no independent physical weight at all in the real corpus (0 `WT:` tokens exist in `cr_equipmods.lst`) |
| **Total** | **2977** | **2011 (67.5%)** | |

Every `None` here is a genuine corpus absence, verified per-category
against the live LST files this cycle -- never a fabricated 0 or guessed
value.

## 3. Criterion 6.4 — full description (CRB)

**Schema change + population, CRB only.** Added `description:
Option<&'static str>` to `EquipmentTableEntry`, populated from each
record's real corpus `DESC:` token, PCGen-entity-decoded (`&nl;` ->
newline, `&lbracket;`/`&rbracket;` -> `[`/`]`, `&pipe;` -> `|`).

| Category | Records | `description` populated | Honest ceiling reason for the remainder |
|---|---:|---:|---|
| General | 453 | 116 | most rows (slot-template items like `Belt`, `Ring`, `Wand`) have no `DESC:` token -- the real prose lives in the printed book, not the LST |
| Arms/Armor | 310 | 147 | same -- weapon/armor `(Base)` template rows generally have no `DESC:` token |
| Magic Items | 1556 | 1556 (100%) | every magic item record carries a `DESC:` token in the real corpus |
| Equipmods | 658 | 2 | equipment modifiers are PCGen bookkeeping records, not player-facing items with their own prose; only 2 carry a `DESC:` token |
| **Total** | **2977** | **1821 (61.2%)** | |

**This is the one criterion where this cycle's honest result falls short
of `decisions.md §5`'s literal text** ("a full description... for all of
these things," with no "where applicable" qualifier, unlike cost/weight).
The real PCGen CRB corpus does not carry prose for every equipment record
-- particularly `cr_equip_general.lst`/`cr_equip_arms_armor.lst` template
rows and `cr_equipmods.lst` modifiers, whose printed-book prose (if any)
is not present in the LST data this repo ingests from. Per the no-stub-
mvp doctrine, this cycle does not fabricate description text to reach
100%. **See `## Open blockers` in `progress.md` for the operator decision
this residual gap requires** (accept the LST-derived ceiling as the
closure bar for `description`, or authorize a second-source ingestion
pass against the printed CRB text, which is out of this bundle's PCGen-
LST-corpus scope).

## 4. Criterion 6.5 — full spell text (CRB)

**Population change, no schema change (CRB).** `SpellListEntry.description`
already existed as a non-optional `&'static str`; this cycle replaced its
value for every present record with the fullest text the real corpus
provides, instead of the pre-cycle truncated-to-first-sentence summary:

- **623 of 652 (95.6%)** now carry the `<Name>.MOD` record's full,
  multi-sentence `DESC:` text (e.g. `Alarm` went from a 51-character
  first-sentence summary to the full ~1000-character mechanic
  description, including the mental-vs-audible-alarm rule and the
  `permanency` interaction).
- **29 of 652 (4.4%)** are narrow spell variants that never got a
  separate `.MOD` record (`Align Weapon (Chaos Only)`, `Elemental Swarm
  (Air Spell Only)`, etc.) -- these carry their own base record's real,
  complete (if short) corpus text, e.g. `Align Weapon (Chaos Only)`:
  "Weapon becomes chaotic." This is the corpus's own complete text for
  that specific named variant, not a truncation.

A trailing PCGen `|PRERULE:1,DisplayFullSpell` display-rule qualifier
(present on 622 of the 623 `.MOD` records' raw `DESC:` token) is stripped
during ingestion -- it is a conditional-display directive, not spell
text, and was verified absent from every equipment `DESC:` token in the
same corpus.

**`full_text_verified` is now 652/652 (100%)** -- every present CRB spell
carries the fullest text the real corpus provides. This is the one
criterion in this cycle's CRB-only scope that reaches a clean, honest
100%.

## 5. Cross-references

- `tests/sd24_equipment_field_completion.rs` -- this cycle's new standing
  regression test (RED -> GREEN evidence in the cycle receipt).
- `tests/sd24_equipment_coverage_audit.rs` -- criterion 6.1's standing
  regression test, updated this cycle to reflect the CRB corrections
  (the record-count re-audit for spells, and the honest field-coverage
  ceiling for weight/description) while leaving the APG/ACG assertions
  (still 0/0, untouched this cycle) unchanged.
- `src/rules_core/rules_tables/crb/equipment_tables.rs`,
  `equipment_data/{general,arms_armor,magic_items,equipmods}.rs`,
  `spell_list.rs` -- the production changes.
- `./crb-field-completion-cycle_cycle_receipt.md` -- this cycle's full
  receipt.
