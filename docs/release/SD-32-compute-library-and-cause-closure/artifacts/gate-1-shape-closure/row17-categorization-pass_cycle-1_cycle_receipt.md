# Row 17 categorization pass — cycle 1 receipt

`kanban.md` row 17 (`epic-7-shape-categorization-100`), `decisions.md §27`/§27a/§27b.

## Re-derivation (`§17a`) before any work

```
python3 scripts/row17_census.py --output artifacts/gate-1-shape-closure/row17-census.json
```

At `no_record == 0` (independently re-verified: `join_status.no_record` in
`docs/work-inventory.json`'s coverage ledger is 0), the census reported:

```
row 17 census (decisions.md §27a/§27b, kanban.md row 17) — population 34397
  F0 by fallthrough                   0
  §27 provisional default            23   (corpus-wide total incl. done units: 24)
  ROW 17 HONEST SIZE                 23
  not_ingested (no_record)            0
```

The brief's own stated figure (23) matched the re-derivation. The 24th
corpus-wide hit (`occult_adventures:class_feature:Psychic ~ Phrenic Pool`)
was outside the not-done population count at re-derivation time but is
discussed below (it re-enters the count once the 23 close — see "Residual
population").

## Instrument validation (`§1a`)

Mutated a genuinely-resolved record (`aurumvorax_rake.json`) to carry the
`§27` marker again with a synthetic reason, confirmed
`ROW 17 HONEST SIZE` moved 1 → 2, then reverted via
`scripts/close_row17_provisional_defaults.py` (idempotent — recomputes the
correct facet and clears the marker again). Confirms the census gate can
still fail closed, not just report zero by construction.

## The 23-unit disposition

Each of the 23 was individually re-derived against the corpus and/or the
PCGen oracle, never re-labelled by assertion. Full per-record evidence is
recorded as code comments on `_MONSTER_ABILITY_FACET_OVERRIDES` in
`scripts/transcribe_monster_tables.py` (the single source of truth this
receipt summarizes, not duplicates):

**4 reclassified — the `§27` `SpecialQuality` default was wrong:**

| Record | Old facet | New facet | Evidence |
|---|---|---|---|
| `Aurumvorax ~ Rake` | SpecialQuality | **SpecialAttack** | universal rule "Rake" is `SpecialAttack` unanimously corpus-wide (`beastiary/monster_ability/rake.json` base rule + `gynosphinx_rake.json` + `bandersnatch_rake.json`) |
| `Bunyip ~ Blood Rage` | SpecialQuality | **SpecialAttack** | universal rule "Blood Rage" base record (`bestiary_2/monster_ability/blood_rage.json`) + `inner_sea_bestiary`'s `volnagur_blood_rage.json` both genuinely `SpecialAttack` |
| `Yrthak ~ Sonic Lance` | SpecialQuality | **SpecialAttack** | same creature's sibling `Yrthak ~ Explosion` (`b2_abilities_race.lst:1416`) genuinely declares `SpecialAttack.Extraordinary` and its own DESC names the identical sonic-lance mechanic |
| `Howler ~ Abyssal Strike` | SpecialQuality | **SpecialAttack** | identical shape to genuinely-declared `nascent_demon_lord_aligned_strike.json` (`SpecialAttack`); same creature's siblings Howl/Pain also both `SpecialAttack` |

**19 confirmed — the `§27` default was already the genuinely-correct
answer; marker removed, no longer provisional:**

`Adlet ~ Spell-Like Abilities`, `Lorthact ~ Spell-Like Abilities`,
`Mothman ~ Agent of Fate` (255:22 corpus-wide majority for unqualified
"Spell-Like Abilities" → `SpecialQuality`), `Denizen of Leng ~ Planar Fast
Healing` (§27's own cited example; Fast Healing = SQ), `Xocothian ~ Speed
Burst` (movement ability, not an attack), `Carnivorous Blob ~ Split`
(universal rule "Split" is SQ 4-of-5 corpus-wide), `Lamia Matriarch ~
Spells`, `Royal Naga ~ Spells`, `Water Naga ~ Spells`, `Lunar Naga ~ Spells`
(racial spellcasting grant, SQ by exclusion among the 7 modeled facets),
`Asurendra ~ None` (structural sibling pattern), `Unfettered Eidolon ~
Con/Str/Wis/Dex/Cha/Int` (×6, flat stat bonus, SQ by exclusion),
`Petrified Maiden ~ Weapon Selection` (granted proficiency, same shape),
`Morlock ~ Sneak Attack` (invisible internal numeric feed — round 6's own
"genuinely novel shape", no other `Internal`-trait record anywhere in the
corpus to compare against; SQ by exclusion among the 7).

## Mechanism

1. `scripts/transcribe_monster_tables.py`: added
   `_MONSTER_ABILITY_FACET_OVERRIDES` (KEY: → real facet, evidence in
   comments) and wired it into `parse_type_or_provisional_default` — a row
   named there ships with `reason=None` (never provisional) exactly like a
   row whose own `TYPE:` segments resolved cleanly. TDD: 5 existing tests
   in `ProvisionalFacetDefaultRound8` (using these exact KEY values) went
   RED for the right reason, updated to assert the new genuinely-derived
   behavior, plus one new control test proving the override is scoped by
   `KEY:`, not by shape.
2. Re-ran `python3 scripts/transcribe_monster_tables.py <book>` for the 5
   affected books (`bestiary`, `bestiary_2`, `bestiary_3`,
   `inner_sea_bestiary`, `inner_sea_gods`) — regenerates
   `monster_data.rs` (the compiled source of truth) with the corrected
   facets.
3. `cargo run --bin gen_book_cache -- <book>` is **additive-only** (never
   overwrites an existing on-disk record — confirmed live: "N already on
   disk, left untouched"), so it cannot correct the 23 already-shipped
   JSON records. Added `scripts/shape_provisional_marker.py::
   clear_provisional_default` (TDD'd, 4 new tests) as the sanctioned
   paired counterpart to `stamp_provisional_default`, and
   `scripts/close_row17_provisional_defaults.py` (TDD'd, 6 new tests) as
   the finishing step: imports `_MONSTER_ABILITY_FACET_OVERRIDES` from
   `transcribe_monster_tables.py` (single source of truth, no duplicated
   resolution table per `§17`/§26) and applies it to the shipped corpus
   JSON directly. `--dry-run` confirmed exactly 23 records matched before
   the real run.

## Verification

- `python3 -m unittest scripts.tests.test_transcribe_monster_tables scripts.tests.test_shape_provisional_marker scripts.tests.test_close_row17_provisional_defaults scripts.tests.test_row17_census scripts.tests.test_shape_ledger` — 128/129 pass; the one failure
  (`InternalBundleAbilityHopIsResolved.test_an_ability_no_bundle_names_stays_an_orphan_and_is_not_shipped`)
  reproduces identically against the unmodified `HEAD` copy of
  `transcribe_monster_tables.py` (confirmed by temporarily restoring it and
  re-running just that test class) — pre-existing, unrelated to this cycle.
- `cargo test --lib rules_tables::bestiary:: rules_tables::bestiary_2::
  rules_tables::bestiary_3:: rules_tables::inner_sea_bestiary::
  rules_tables::inner_sea_gods::` — 40/40 pass (`CARGO_TARGET_DIR`/
  `CARGO_INCREMENTAL=0` per environment block).
- `python3 scripts/row17_census.py --check` re-run after the fix: exit 0,
  `provisional_default_missing_reason` 0.

## Residual population — NOT closed, escalated

After the 23 close, the census reports:

```
§27 provisional default             1   (corpus-wide total incl. done units: 1)
ROW 17 HONEST SIZE                  1
```

The remaining unit: `occult_adventures:class_feature:Psychic ~ Phrenic
Pool`. This marker is NOT a `§27` `TYPE:`-facet-default at all — it is a
**per-character ability-score-choice default**: `ground_psychic_class_features`
(`src/rules_core/pilot_compute/mod.rs`) hard-codes Charisma for the
Phrenic Pool formula, but the real PF1e rule is that the governing ability
score is set by whichever of the 9 Psychic Discipline options the
character chose (CHA for 4 disciplines, WIS for 5) — a genuine per-character
input the compute chassis does not currently track at all. Confirmed by
grep: no "chosen discipline"/"chosen bloodline"/"chosen domain"/"chosen
mystery"-shaped selection input exists anywhere in `pilot_compute::mod.rs`
for ANY class with this shape (Sorcerer bloodline, Oracle mystery, Cleric
domain included) — this is not a one-line fix reusing an existing pattern,
it is a new character-input mechanism.

`decisions.md §27b` item 5 is explicit that "needs a new mechanism" is not
an admissible reason to leave a unit open, and item 4 requires resolving
every scope question by widening rather than carving out. I did not
find a way to safely build a general per-character class-feature-choice
input, thread it through the Psychic class specifically, and prove it by
class (not by instance) within this one cycle without risking the
existing 212+ passing psychic-adjacent tests on a rushed, undertested
mechanism — this is a blocker bigger than one cycle
(`AGENTS.md`'s Blocker Discipline: "decompose it and run the cycles"),
not an exemption.

**Escalating by name, per Blocker Discipline:** the next cycle needs write
scope to `src/rules_core/pilot_compute/mod.rs` (and whatever character-input
schema backs it) to add a `chosen_psychic_discipline`-shaped input (9
disciplines, CHA for 4 / WIS for 5, cited in the marker's own reason text)
threaded into `ground_psychic_class_features`, proven by a test per
discipline, before `scripts/shape_provisional_marker.py::
clear_provisional_default` can honestly retire this marker. Until then
`ROW 17 HONEST SIZE` is **1, not 0**, and row 17 stays `in-progress`, not
`complete` — per this bundle's own `§10`: SD-32 does not close with any
Epic card short of `complete`.

## Kanban row 17

Moved `backlog` → `in-progress`. Not `complete`: `ROW 17 HONEST SIZE` is 1,
not the required 0. 23 of 24 corpus-wide provisional-default units closed
this cycle; the residual unit is a distinct mechanism-shaped blocker,
escalated above.
