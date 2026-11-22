# SD31-E6-F5-002: book × kind cache-gen map

Card `epic-6-ingest-lanes` (the cache-gen dump for every kind except
`class_feature`). This is the "build the full map first, commit it" deliverable
the card mandates — derived from the live checkout, not narrated, at HEAD
`89846f5c9` (this cycle's start) + this cycle's own commits.

## Methodology and commands

1. `ls src/rules_core/rules_tables/` → **38** entries (re-derives the dispatch
   preamble's "38 shipped book modules" figure exactly).
2. `ls src/rules_core/cache_gen/` (before this cycle) → **4** modules:
   `acg`, `apg`, `beastiary1`, `ultimate_equipment`. This cycle adds a 5th:
   `equipment_gap` (see below — NOT a per-book module; a per-KIND join across
   8 books' residue).
3. For each of the 29 `rules_tables/<book>/` subdirectories (the 9 remaining
   entries are standalone `.rs` files spanning multiple books —
   `feats_all.rs`, `feat_gap_tables.rs`, `equipment_gap_tables.rs`,
   `monster_chassis.rs`, `companion_chassis.rs`, `class_spell_levels.rs`,
   `archetype_swap.rs`, `equipment_gap_tables.rs` covered separately, `mod.rs`),
   a keyword scan (script below) over every `.rs` file under it for
   kind-suggestive symbols (`class_table(`, `EquipmentTableEntry`,
   `SpellListEntry`, `RaceTrait`, `MonsterAbility`, `CompanionTableEntry`,
   `ClassFeature`, `FeatTableEntry`, ...). **This is a heuristic, not a proof**
   — a doc-comment cross-reference to a sibling book can false-positive; it
   answers "does this book's module plausibly carry this kind's records
   in-memory," which is what a future dispatch needs to decide where to look
   next, not a doneness claim.
4. `data/corpus/<book>/<kind>/` existence — direct `os.listdir` walk, this
   cycle's own tip (after this cycle's new directories, marked below).
5. `equipment_gap_tables.rs`/`feat_gap_tables.rs` are **NOT per-book
   directories** — they are pre-generated, already-oracle-verified join
   tables (`gen_equipment_gap_tables`/`gen_feat_gap_tables`, run once against
   the real PCGen oracle, checked in as plain Rust `pub static` arrays) that
   carry ALREADY-COMPILED-book residue: records belonging to a book whose own
   hand-authored table doesn't hold them. Counted directly per static array /
   per `book:` field.

Script: `/tmp/.../scratchpad/build_map.py` (ad-hoc `find`/regex over the
checkout, not committed — the OUTPUT below is the deliverable).

## Finding 1: the two "closed-form" gap tables are the biggest untapped lever

| Table | Rows (real, oracle-cited, uningested before this cycle) | Books covered |
|---|---|---|
| `equipment_gap_tables::equipment_gap_rows()` | **704** (excl. `UE`'s 65, owned by a sibling lane) | CRB 335, APG 37, ACG 50, ARG 15, UC 20, UI 7, UPSI 113, UW 127 |
| `feat_gap_tables::feat_gap_rows_for()` | **83** | CRB 16, ARG 48, UC 2, UI 3, UM 12, UPSI 1, UW 1 |

Neither table had ever been dumped to `data/corpus/` before this cycle — same
shape as `OPEN-ISSUES.md` row 11/12's finding for `ultimate_equipment`, but
these are DIFFERENT, already-shipped Rust data, not requiring any new corpus
parsing. **This cycle built `cache_gen::equipment_gap` and dumped 701/704 of
the equipment table** (127 equipment + 574 equipment_modifier, real citations,
real PI screening on name AND description — see `progress.md`'s
`SD31-E6-F5-002` receipt for the full trace). `feat_gap_tables` (83 rows) is
the same shape, same lever, NOT yet dumped by this cycle — the largest
concretely-scoped remaining item this map surfaces for the next cycle
(`cache_gen::feat_gap`, mirroring `equipment_gap`'s citation-resolution code
almost exactly: same `find_by_key_field`/first-column/`.COPY=` three-strategy
search, same PI-both-fields correction, same 7-book routing minus `UE`).

## Finding 2: per-book kind coverage (heuristic scan × real corpus-dir check)

`Y` = kind hint found in that book's `rules_tables` module. `dir` = a
`data/corpus/<book>/<kind>/` directory exists on disk (`+` = created by
this cycle). `class_feature` is excluded (sibling lane's kind).

| book | class | feat | equipment | equip_mod | companion | monster_ability | race_trait |
|---|---|---|---|---|---|---|---|
| core_rulebook | Y/dir | Y/dir | Y/dir | Y/dir (pre-existing `equipmods/`, this cycle added 4 records) | Y/dir | — | Y/dir |
| advanced_players_guide | Y/dir | Y/dir | Y/dir | —/dir(+) | Y/dir | — | Y/dir |
| advanced_class_guide | Y/dir | Y/dir | Y/dir | —/dir(+) | Y/— | — | — |
| advanced_race_guide | Y/dir | Y/dir | Y/dir | —/— | Y/dir | — | Y/dir |
| pathfinder_unchained | Y/dir | Y/dir | Y/dir | —/— | Y/— | — | — |
| ultimate_equipment (sibling) | — | Y/— | Y/dir | Y/dir | Y/— | — | — |
| ultimate_combat | Y/— | Y/— | Y/dir(+) | Y/dir(+) | Y/— | — | — |
| ultimate_intrigue | — | Y/— | Y/— | Y/— | Y/— | — | — |
| ultimate_psionics | — | Y/— | Y/dir(+) | Y/dir(+) | Y/— | Y/dir | — |
| ultimate_wilderness | — | Y/— | Y/dir(+, new) | —/— | Y/dir | — | — |
| ultimate_magic | — | Y/— | Y/— | Y/— | Y/dir | — | — |
| beastiary1 (Bestiary) | Y/— | — | Y/— | — | Y/dir | Y/dir | Y/dir |
| bestiary_2..6 | — | partial | — | — | Y/dir | Y/partial | Y/partial |
| core_essentials | — | — | — | — | Y/dir | — | Y/dir |
| inner_sea_* / monster_codex / bonus_bestiary / book_of_the_damned_* / horror_adventures | — | partial | — | — | partial | Y/partial | partial |

Reading this table: a `Y` with no `dir` is exactly this card's remaining
target shape — a book whose module plausibly carries the kind in-memory, with
no on-disk cache yet. The clearest, most bounded next targets after
`feat_gap_tables` (Finding 1) are `advanced_class_guide`'s and
`pathfinder_unchained`'s `companion` kind (module hints present, no corpus
dir) and `ultimate_magic`/`ultimate_intrigue`'s `equipment_modifier` residue —
none scoped or built this cycle; named here for the next dispatch, not
claimed as done.

## Finding 3: this cycle's actual delivery against the map

- `cache_gen::equipment_gap` (new module, `src/rules_core/cache_gen/
  equipment_gap.rs`) + `src/bin/gen_cache_equipment_gap.rs`: dumped
  **701/704** of `equipment_gap_tables`'s non-`UE` rows to
  `data/corpus/<book>/equipment/*.json` across **7** books (core_rulebook,
  advanced_players_guide, advanced_class_guide, advanced_race_guide,
  ultimate_combat, ultimate_intrigue, ultimate_wilderness) **+ 65 more from
  ultimate_psionics** (real records, correctly written, but see
  `OPEN-ISSUES.md` row 46 for why they cannot be swept today). 3 rows
  genuinely unresolved (`core_rulebook`'s "Rock (Small)"/"Rock (Medium)"/
  "Poison (Violet Venom)" — real PCGen keys that trace to `core_essentials`,
  a DIFFERENT book directory than the gap table's own `"CRB"` tag names; not
  guessed at, left unwritten).
- `feat_gap_tables` (83 rows): mapped, NOT dumped this cycle (see Finding 1
  — the concretely-scoped next lever).
- The Mitre-of-the-Hierophant guard/record (`OPEN-ISSUES.md` row 40): see
  `progress.md`'s receipt for this cycle's disposition.
