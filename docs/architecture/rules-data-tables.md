# Rules Data Tables

> Scope: the hand-transcribed, per-book Paizo table store rules-core queries for class chassis, race traits, feats, spells, equipment, and monster stat blocks.
> Last verified: 2026-07-21 against deeff110a104
> Maintenance: updated at SD closure — see [README.md](./README.md) §Maintenance contract

## Purpose

`src/rules_core/rules_tables/` (`src/rules_core/rules_tables/mod.rs`:
"Canonical Paizo-table store") is a book-partitioned store of
hand-transcribed rule data: class chassis tables, race trait tables,
feats, spell lists, equipment, and monster stat blocks. Each book gets
its own sibling directory and its own `RuleSetId` variant. This is a
separate surface from [corpus-ingest.md](./corpus-ingest.md)'s
`SourcePackageContent` projection pipeline: table data here is
authored by transcribing values out of the real PCGen `.lst` corpus by
hand (or, in one documented case, generated programmatically from it),
not produced by running the ingest pipeline's parsers at build time.

## Per-book directory pattern

Four book directories exist under `src/rules_core/rules_tables/`,
declared in `src/rules_core/rules_tables/mod.rs`:

```
pub mod acg;
pub mod apg;
pub mod beastiary1;
pub mod crb;
```

- **`crb/`** (Core Rulebook) — the fully-populated book:
  `class_tables.rs` (per-class BAB/save chassis, `ClassId` enum,
  `good_saves_for` — the good-Fortitude/Reflex/Will classification SD-24
  Epic 5's multiclass dispatch reads directly, see [rules-engine.md](./rules-engine.md)),
  `race_tables.rs` (per-race trait dimensions, `RaceId` enum),
  `feats.rs` + `feat_data/{general,combat,item_creation,metamagic}.rs`
  (185-feat catalog split by `FeatCategory`), `spell_list.rs`, and
  `equipment_tables.rs` + `equipment_data/{arms_armor,general,magic_items,equipmods}.rs`.
- **`apg/`** (Advanced Player's Guide) — `mod.rs` plus one file per
  class (`class_alchemist.rs`, `class_cavalier.rs`, `class_inquisitor.rs`,
  `class_oracle.rs`, `class_summoner.rs`, `class_witch.rs` — all six
  real APG classes; `mod.rs`'s doc comment notes Gunslinger and Magus
  are deliberately excluded because they are not real APG corpus
  content), plus a shared `spell_list.rs`, `equipment_tables.rs`, and a
  single `equipment_data.rs` (SD-24 Epic 6; one flat file rather than
  CRB/ACG's `equipment_data/` directory split).
- **`acg/`** (Advanced Class Guide) — the same class-file shape: `mod.rs`
  plus ten per-class files (`class_arcanist.rs`, `class_bloodrager.rs`,
  `class_brawler.rs`, `class_hunter.rs`, `class_investigator.rs`,
  `class_shaman.rs`, `class_skald.rs`, `class_slayer.rs`,
  `class_swashbuckler.rs`, `class_warpriest.rs` — the full, corrected
  10-class roster per `mod.rs`'s roster-correction note), plus
  `spell_list.rs`, `equipment_tables.rs`, and — mirroring CRB's own split
  (SD-24 Epic 6) — `equipment_data/{arms_armor,general,magic_items,equipmods}.rs`,
  the last of which ingests `acg_equipmods.lst` into a new
  `EquipmentCategory::Equipmods` variant that criterion 6.1's original
  scope had not counted at all.
- **`beastiary1/`** (Bestiary 1) — `mod.rs` plus eight
  `monster_subset_01.rs` .. `monster_subset_08.rs` files, five (or six,
  for subset 06) monsters each, 41 monsters total as of this
  verification. Subsets are appended in CR-band order as ingest cycles
  land; `mod.rs`'s doc comment documents each subset's exact roster and
  any correction against an earlier planning-doc sample list.

Every book directory follows the same two-tier shape: a `mod.rs`
defining the book's `RuleSetId`-scoped resolver function(s) plus a
shared row/record struct, and per-unit files (one per class, or one per
monster subset) supplying the literal data.

## `RuleSetId` and per-book resolution

`RuleSetId` (`src/rules_core/rules_tables/mod.rs`) currently has four
populated variants plus a placeholder comment for future books:

```rust
pub enum RuleSetId {
    Crb,
    Apg,
    Acg,
    Bestiary1,
    // future: Um, ...
}
```

Each book's resolver function takes a book-scoped ID enum plus a
`RuleSetId` and returns `None` immediately if the `RuleSetId` does not
match that book — e.g. `apg::class_chassis_resolve(class_id: ApgClassId,
level: u8, rule_set: RuleSetId) -> Option<ClassTableRow>` (`src/rules_core/rules_tables/apg/mod.rs`)
starts with `if rule_set != RuleSetId::Apg { return None; }` before
dispatching on `class_id`. `acg::class_chassis_resolve` and
`beastiary1::monster_resolve` follow the identical guard-then-dispatch
shape. This means an `ApgClassId::Witch` query against `RuleSetId::Crb`
is a defined, tested `None` — not a panic or a silent wrong answer.

The cross-book invariant is asserted directly in the per-class/monster
acceptance tests. `tests/sd22_apg_class_witch_resolves.rs` resolves
`ApgClassId::Witch` at level 1 under `RuleSetId::Apg` and asserts
`Some` (`class_chassis_resolve(ApgClassId::Witch, 1, RuleSetId::Apg).expect(...)`,
line 29), then resolves the same `ApgClassId::Witch` at level 1 under
`RuleSetId::Crb` and asserts it is `None` (line 62: `assert_eq!(class_chassis_resolve(ApgClassId::Witch, 1, RuleSetId::Crb), None, "APG-only class chassis must not resolve under RuleSetId::Crb");`).
Every per-class and per-monster test file in the `tests/sd22_*_resolves.rs`
family follows this same "resolves under its own book, `None` under
every other book" pattern.

## The hand-transcription convention

Table data is transcribed from the real PCGen `.lst` corpus by hand,
with a source citation in the transcribing function's doc comment that
names the exact corpus file, and usually the exact line number and
token(s), the value came from. Two representative examples, quoted
verbatim:

`src/rules_core/rules_tables/apg/class_witch.rs` (module-level doc
comment):

> Source: PCGen `apg_classes.lst`, `CLASS:Witch` record (line 172 of
> the SD-22 Epic 3 corpus checkout), parsed via
> `pcgen_import::lst_parser::spellcasting_class` ... The real record's
> chassis-bearing tokens:
> - `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")/2` — half BAB, poor (the first poor-BAB class in this roster).
> - `BONUS:SAVE|BASE.Will|classlevel("APPLIEDAS=NONEPIC")/2+2` — good Will save.

`src/rules_core/rules_tables/beastiary1/monster_subset_01.rs`, on the
per-monster function itself:

> Source: `b1_races.lst:200`, `CR:1`. Real row tokens: `SIZE:M`,
> `MOVE:Walk,30`, `NATURALATTACKS:Claw,...,*2,1d6`,
> `NATURALATTACKS:Bite,...,*1,1d6`, `RACETYPE:Undead`, `CR:1`,
> `SOURCEPAGE:p.146`.

Both examples cite the exact corpus file and either a line number or
the literal token text, so a reviewer can re-open the corpus and verify
the transcription independently. `beastiary1/mod.rs`'s module doc
comment additionally documents every roster correction made against an
earlier (wrong) planning-document sample list — e.g. subset 01's real
roster (Ghoul, Gnoll, Goblin Dog, Lizardfolk, Wolf) replaces an
illustrative list that named creatures with no real standalone CR-1
stat block in the corpus. Scope is bounded the same way across every
per-unit file: only fields literally present as tokens on the source
row are transcribed (e.g. `MonsterStatBlock` excludes AC/HP/saves,
which PCGen computes at runtime rather than publishing as row tokens;
`class_witch.rs` transcribes only the BAB/save chassis, not named
per-level features).

One book deviates from pure hand-transcription:
`src/rules_core/rules_tables/crb/feats.rs`'s doc comment states its
185-feat catalog is "generated programmatically from the live corpus"
rather than hand-transcribed line-by-line, specifically to avoid
transcription error at that scale; its category derivation rule (the
`TYPE:` facet) and excluded-record list are documented in the same doc
comment.

## Equipment/spell content completeness (SD-24 Epic 6)

`EquipmentTableEntry` (defined once per book, in each book's own
`equipment_tables.rs`) carries the same core shape across CRB/APG/ACG —
`key`, `category`, `name`, `cost_gp: Option<f64>`, a per-record weight field
(`weight_lbs` for CRB and ACG, `weight` for APG — the field name itself was
not reconciled across books this cycle), and `description: Option<&'static str>`
— but the fields' *population* ceiling differs by book because it is bounded
by what the real PCGen corpus actually carries, not by transcription effort:

| Book | Equipment records | Weight populated | Description populated | Description source |
|---|---|---|---|---|
| CRB | 2977/2977 (100%) | 2011/2977 (67.5%, honest "where applicable" ceiling) | 1821/2977 (61.2% — real corpus ceiling, not fabricated; see [status.md](./status.md) Open blockers) | corpus `DESC:` token |
| APG | 338/338 (100%) | 319/338 (94.4%, 19 real corpus gaps) | 0/338 (0% — APG's corpus has zero `DESC:` tokens on any equipment row; see [status.md](./status.md) Open blockers) | n/a |
| ACG | 269/269 (100%; 221 `acg_equip.lst` + 48 `acg_equipmods.lst`) | 135/269 (50.2%; Equipmods genuinely 0/48) | 264/269 (98.1%) | corpus `SPROP:` token (ACG also has zero `DESC:` tokens, but its `SPROP:` — "Special Property" — token is a near-universal convention here; a trailing `\|<conditional-tag>` qualifier is stripped) |

Spell records carry an equivalent `description`/full-text field. CRB reaches
652/652 (100%, sourced from the fullest available corpus text — a matching
`.MOD` record's text where one exists, 623/652, else the base record's own
text); ACG reaches 144/144 (100%, its base record already carries full text
natively — the reverse of CRB's `.MOD`-record convention); APG reaches
261/297 (87.9%) — 41/297 lack a `SCHOOL:`/`CLASSES:` token at all, a real
corpus gap that caps it below 100%.

Record-count coverage (the count of rows ingested at all, independent of
which fields are populated) is 100% for equipment and spells across CRB,
APG, and ACG; Bestiary 1's own equipment corpus (7 records) has no ingested
module yet. See [status.md](./status.md) for the full stub/gap ledger and
the specific `## Open blockers`-equivalent ceilings.

## Adding a new book

Following the existing four books' pattern, adding book `<xyz>` means:

1. Create `src/rules_core/rules_tables/<xyz>/mod.rs` as a new sibling
   directory, declared via `pub mod <xyz>;` in
   `src/rules_core/rules_tables/mod.rs`.
2. Add a `RuleSetId::<Xyz>` variant in
   `src/rules_core/rules_tables/mod.rs`.
3. In `<xyz>/mod.rs`, define the book-local row/record struct(s) (e.g.
   a `ClassTableRow` or `MonsterStatBlock` shape — books do not share
   these structs across directories even when the shape is identical;
   `acg/mod.rs`'s `ClassTableRow` doc comment notes it deliberately
   mirrors `apg::ClassTableRow` rather than importing it), the
   book-scoped ID enum (`<Xyz>ClassId` or equivalent), and the
   `RuleSetId`-guarded resolver function(s) following the
   guard-then-dispatch shape shown above.
4. Add one per-unit file per class/monster/table family (e.g.
   `class_<name>.rs`), each with a module or function-level doc comment
   citing the exact corpus file (and line number or literal tokens) the
   transcription came from, plus an explicit note of any fields
   deliberately excluded as out of scope.
5. Add a `tests/sd<NN>_<xyz>_<unit>_resolves.rs` acceptance test per
   unit, asserting `Some` under the new `RuleSetId` and `None` under
   every existing `RuleSetId`, following
   `tests/sd22_apg_class_witch_resolves.rs`'s pattern. Where the
   transcription's source claim is checkable against the real corpus,
   gate an additional real-corpus test on `PCGEN_CORPUS_ROOT` using the
   graceful-skip pattern described in
   [corpus-ingest.md](./corpus-ingest.md).

See [corpus-ingest.md](./corpus-ingest.md) for how corpus text is
parsed upstream of this hand-transcription step, and
[rules-engine.md](./rules-engine.md) for how `rules_tables` resolvers
are consumed by rules-core compute.
