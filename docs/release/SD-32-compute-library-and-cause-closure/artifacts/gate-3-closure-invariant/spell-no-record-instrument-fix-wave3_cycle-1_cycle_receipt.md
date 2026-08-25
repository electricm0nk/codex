# Cycle 1 — gate-3-closure-invariant / `spell` `no_record`, wave 3 (`decisions.md §20`)

- **Card ID:** card 11 (`epic-2-cause-closure`), rows 11 and 15 left `in-progress` per dispatch
  instruction.
- **Commit SHA:** (this cycle's own commit, see push log)
- **Files touched:**
  - `scripts/shape_ledger.py` — `build_corpus_index`'s aliased-book walk now merges the legacy
    aliased directory AND the book's own correctly-spelled directory, instead of the aliased
    directory only (see "The real defect" below).
  - `scripts/tests/test_shape_ledger.py` — one new RED→GREEN test,
    `test_bestiary_alias_does_not_hide_the_correctly_spelled_directory`.
  - `src/rules_core/cache_gen/spell_lane_dump.rs` — one new `BookSpec` entry (`bestiary_6`) and
    one new import (`bestiary_6`).
  - `data/corpus/bestiary_6/spell/*.json` — 2 new files, written by `gen_cache_spell_lane_dump`.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --unified=0 HEAD -- scripts/shape_ledger.py
  scripts/tests/test_shape_ledger.py src/rules_core/cache_gen/spell_lane_dump.rs | grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → no match)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff scope,
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → no match)
- **Acceptance criterion:** `decisions.md §20` — `no_record == 0` is Gate 3's closure condition.
  This cycle's scope: `spell`'s 167 (brief figure), re-derived below.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`).

## §17a re-derivation before planning

Fresh worktree — oracle slot was empty, bootstrapped via `scripts/fetch-pcgen-oracle.sh --dest
"$PCGEN_REPO_DIR"` (landed at the pinned SHA above, confirmed `scripts/verify.sh --only
preflight-oracle` → PASS).

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger_now.json
python3 -c "import json,collections; d=json.load(open('/tmp/ledger_now.json')); \
  c=collections.Counter(r['kind'] for r in d['rows'] if r['join_status']=='no_record'); \
  print(c.most_common())"
```
→ `monster_ability 191, spell 167, equipment 116, equipment_modifier 33, companion 2` — total 509,
**matches the dispatch brief exactly**. `spell` by book also matched the brief's table exactly
(`bestiary` 109, `advanced_players_guide` 24, `inner_sea_magic` 5, `inner_sea_world_guide` 5,
`adventurers_guide` 4, `inner_sea_gods` 4, `inner_sea_intrigue` 4, `ultimate_magic` 3, `bestiary_4`
2, plus 7 singletons).

## The real defect: `shape_ledger.py`'s alias walk hides the book's own directory

`bestiary`'s 109 `spell` `no_record` units were reported despite `data/corpus/bestiary/spell/`
already holding all 110 of `ce_spells.lst`'s base declarations, committed by a PRIOR cycle
(`epic-2-spell-companion-equipment-no-record_cycle-1`, `a4636b4718`). Verified directly:

```bash
ls data/corpus/bestiary/spell/ | wc -l        # 110
git log -1 --oneline -- data/corpus/bestiary/spell/blur_self_only.json   # a4636b4718, already committed
```

**Root cause** (`scripts/shape_ledger.py::build_corpus_index`): `BOOK_CORPUS_DIR_ALIASES = {"bestiary":
"beastiary"}` exists because `bestiary`'s `monster_ability` corpus output lives under the historical
misspelled `beastiary/` directory (a real, already-documented alias — see the function's own
docstring and `test_bestiary_book_walks_the_beastiary_corpus_directory`). But `bestiary`'s `spell`
(and `equipment`) records live under the CORRECTLY-spelled `bestiary/` directory — a different kind,
a different directory, the same book. The alias is book-wide in the walker (`os.path.join(corpus_root,
BOOK_CORPUS_DIR_ALIASES.get(b, b))`), so when `books={"bestiary", ...}` is passed (always, from
`main()` — `books = {u.get("book") for u in units if u.get("book")}` is never `None` in real use), the
walker visits ONLY `beastiary/`, which has no `spell/` subdirectory at all
(`ls data/corpus/beastiary/` → `ability class companion equipment feat_generic language monster
monster_ability monster_generic race race_generic race_trait race_trait_generic template`, no
`spell`). Every `bestiary` `spell` unit's join therefore found nothing, regardless of whether the
real record existed one directory over — the exact shape the alias's own doc comment already
diagnosed for `monster_ability`, now recurring for `spell` for the opposite reason (a book whose
NEWER kind lives under the CORRECT spelling while an OLDER kind lives under the historical one).

**Fix:** for an aliased book, walk BOTH the aliased directory and the book's own correctly-spelled
directory, merging the index (no key collision risk — `(book, source_file, source_line)` triples are
unique per real corpus record by construction). `beastiary/monster_ability/` and `bestiary/spell/`
never overlap; the merge is a pure union.

### RED → GREEN

New test `test_bestiary_alias_does_not_hide_the_correctly_spelled_directory`
(`scripts/tests/test_shape_ledger.py`): synthetic fixture with a record under `<tmp>/beastiary/
monster_ability/` and a second under `<tmp>/bestiary/spell/`; asserts `build_corpus_index(tmp,
books={"bestiary"})` contains BOTH keys.

```bash
python3 -m unittest scripts.tests.test_shape_ledger.BuildCorpusIndexTest.test_bestiary_alias_does_not_hide_the_correctly_spelled_directory -v
```
RED (before the fix): `AssertionError: ('bestiary', 'ce_spells.lst', 62) not found in
{('bestiary', 'ce_abilities_race.lst', 1280): [...]}` — only the legacy directory's record was
indexed.

GREEN (after the fix, full suite):
```bash
python3 -m unittest scripts.tests.test_shape_ledger -q
```
`Ran 31 tests in 0.412s — OK` (30 pre-existing + 1 new, all pass; the pre-existing
`test_bestiary_book_walks_the_beastiary_corpus_directory` — the ORIGINAL alias test — still passes
unmodified, so the legacy `monster_ability` join is unaffected).

### Effect on the real ledger

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger_after1.json
```
`no_record` (all kinds): `509 → 398` (`-111`), purely from the instrument fix, **zero code/config
change to any generator**. Per kind: `spell 167→59` (`-108`), `equipment 116→113` (`-3`, a side
effect — `bestiary`'s `equipment/` directory was equally hidden by the same alias bug; not otherwise
touched this cycle, named here so it isn't lost), `monster_ability`/`equipment_modifier`/`companion`
unchanged (`bestiary` has no `monster_ability`/`equipment_modifier` corpus directory of its own, only
under the legacy alias, which was already reachable).

`spell` by book after the fix: `advanced_players_guide 24, inner_sea_magic 5, inner_sea_world_guide 5,
adventurers_guide 4, inner_sea_gods 4, inner_sea_intrigue 4, ultimate_magic 3, bestiary_4 2,
bestiary_6 2, bestiary 1, book_of_the_damned_volume_1 1, book_of_the_damned_volume_2 1,
inner_sea_faiths 1, occult_adventures 1, ultimate_combat 1` = 59. (`bestiary`'s 108-unit drop
matches 110 base declarations minus 1 already-PI-dropped `.COPY=` row not covered by this cycle,
named below, minus a handful of extra real citable rows the corpus holds beyond the ledger's own
109-count units — same "extra real data, not an inflated claim" shape the prior cycle's own receipt
documents for this same book.)

## `bestiary_6` (2 → 0): missing `BookSpec` config row, same shape as the prior wave

`bestiary_6:spell:animal_growth_reptiles_only` / `..._animal_shapes_reptiles_only` were `no_record`
despite `src/rules_core/rules_tables/bestiary_6/spell_list.rs` already carrying BOTH compiled records
(SD-31 wave 24, hand-authored, both of the book's two base declarations from `b6_spells.lst`).
`src/bin/ingest_spells.rs`'s `BOOKS` table has no `bestiary_6` entry (this table's compiled table was
never regenerated by this binary — it predates it), but that is not the gap: the corpus-JSON-dump
half, `src/rules_core/cache_gen/spell_lane_dump.rs::book_specs()`, had no `bestiary_6` `BookSpec`
entry either, so no `data/corpus/bestiary_6/spell/*.json` was ever written — the identical
"compiled table exists, corpus dump does not" gap that module's own doc comments already name for
five other books (`adventurers_guide`, `inner_sea_faiths`, etc., wave-19/SD-32 comments in the same
file).

**Config addition — no new logic:** one `BookSpec` entry (`book_id: "bestiary_6"`, `dir:
"pathfinder/paizo/roleplaying_game/bestiary_6"`, `spell_file: "b6_spells.lst"`, `entries:
bestiary_6::spell_list::SPELL_LIST...`), plus adding `bestiary_6` to the module's `use` import list.

### RED → GREEN

`generation_against_the_real_pinned_corpus_resolves_every_citation` is the standing proof this
module's own test suite already runs (asserts `unresolved_citations.is_empty()` against the real
pinned oracle for EVERY `BookSpec`, so an unresolvable citation for the new entry would fail this
test — the same mechanism the prior two spell-widening cycles' own receipts relied on rather than a
per-book mutation test):

```bash
cargo test --locked --lib rules_core::cache_gen::spell_lane_dump
```
9/9 pass, including `generation_against_the_real_pinned_corpus_resolves_every_citation`.

```bash
cargo build --locked --lib                                   # clean, pre-existing warnings only
cargo test --locked --lib rules_core::rules_tables::          # 504/504 pass, 3 pre-existing ignored
cargo test --locked --bin ingest_spells                       # 19/19 pass (unaffected — this
                                                                #   binary was not touched)
```

### Corpus generation — additive-only, verified

```bash
git status --porcelain -- data/corpus | wc -l    # 0, before the run
cargo run --locked --bin gen_cache_spell_lane_dump
# "spell-lane supplemental cache generated: 1279 spell records"
git status --porcelain -- data/corpus
#  ?? data/corpus/bestiary_6/
```
**Zero deletions, zero modifications** — one new untracked directory. `3113458009`'s self-erasure
guard re-confirmed live: this run touched all 21 books now in `book_specs()` and modified none of
the 20 pre-existing ones.

### `spell` `no_record`, before/after (this addition alone)

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger_after2.json
```
```
before (this addition): 59
after:                  57   (delta -2, bestiary_6 fully closed)
```

## `no_record`, before/after — this cycle's full scope

| Kind | Before | After | Delta |
|---|---:|---:|---:|
| `spell` | 167 | 57 | **-110** |
| `equipment` | 116 | 113 | -3 (side effect of the instrument fix, not attempted directly this cycle) |
| `equipment_modifier` | 33 | 33 | 0 (untouched) |
| `monster_ability` | 191 | 191 | 0 (untouched — sibling lane's kind; the alias fix does not move it, verified above) |
| `companion` | 2 | 2 | 0 (untouched) |
| **Bundle total `no_record`** | **509** | **396** | **-113** |

Gate 3 standing check (not touched, verified still green):
```bash
python3 scripts/shape_coverage_standing_gate.py --inventory docs/work-inventory.json
```
`no_record budget: 396/35328 vs. baseline 21521/36028 -- exceeded: False`.
`NO_RECORD_BUDGET_COUNT`/`NO_RECORD_BUDGET_POPULATION` constants **not modified**, per the dispatch
brief's explicit instruction.

## Closure figures — three separate numbers (`decisions.md §16`)

- **Closure** (real ingest, new corpus record, `no_record` → `matched`/`no_formula_tokens`): **2**
  `spell` units (`bestiary_6`'s two, via the new `BookSpec` config row — new corpus JSON written).
- **Reclassification:** none. No unit changed `kind`.
- **Instrument correction (a fourth, distinct bucket — not "closure" and not "reclassification"):**
  **108** `spell` + **3** `equipment` units moved from `no_record` to their true status purely
  because `shape_ledger.py`'s join defect was hiding an ALREADY-REAL, already-committed corpus
  record. No new content was written for these; they were done before this cycle and mis-measured.
  Naming this as its own bucket rather than folding it into "closure" because inflating an instrument
  fix into a content-closure count is exactly the shape `decisions.md §1a`/`§16` exist to refuse —
  the units were never actually `no_record` in truth, only in the tool's own miscount.
- **Reachability (Gate 2):** honest claim of **0** for `bestiary_6`'s 2 new records — not wired into
  `spell_resolver::spell_catalog_rows()` or the desktop spell catalog this cycle, same precedent
  every prior spell-widening cycle in this bundle has set (Gate-1 measurability only).

## PI screening

No new PI-blocked units this cycle — `bestiary_6`'s 2 records carry no `NAMEISPI:YES`/blacklist hit
(verified: both were already compiled by a prior SD-31 cycle with a clean `pi_field: None`/`license:
OGL`; re-confirmed by this cycle's own `cargo run --bin gen_cache_spell_lane_dump` output, `dropped,
NAMEISPI:YES or blacklisted: 0`).

## Fixture discipline (`decisions.md §3`)

`bestiary_6`'s 2 new records carry no `raw_tokens` yet — same reason every other book on this
generator has needed a separate, later `enrich_spell_raw_tokens.rs` pass before `corpus_literal_sweep`
can examine them. Named, not silently skipped.

## What is NOT done, named explicitly (no silent narrowing)

`spell`'s residual is now **57**, traced individually this cycle (not left as an unexamined pile):

- **`advanced_players_guide`'s 24** — traced, NOT widened. Every one of the 24 already has a real
  corpus JSON record under `data/corpus/advanced_players_guide/spell/` (confirmed by `data.key`
  lookup for all 24). The join still fails because the record's own citation diverges from the
  unit's independently-scanned `(source_file, source_line)`, in three distinct sub-shapes:
  - **17 units**: plain `source.kind: "lst_token"` citations at a DIFFERENT line than the unit's own
    scan (e.g. `Beast Shape I (Animals Only)`: unit cites line 1059, record cites line 1044 — both
    real lines in `apg_spells.lst`, not yet reconciled). Needs the same shape of trace
    `repair_spell_citations.rs` already did for the `.MOD`-citation defect, but for THIS mismatch
    class, which that tool's own `TARGET_BOOKS`/logic does not cover.
  - **6 units** (`Fester`, `Heroic Fortune`, `Heroic Fortune (Mass)`, `Malediction`, `Severed Fate`,
    `Unravel Destiny`): `source.kind: "web_second_source"` — deliberately sourced from
    `d20pfsrd.com`/`aonprd.com` (SD-25 Epic 7, `apg.rs::web_sourced_spell`) because the corpus's own
    `.lst` text was garbled for these. `Source::WebSecondSource` carries no `path`/`line` field at
    all, so `shape_ledger.py`'s LST-citation-only join structurally cannot match it. A real fix needs
    either a schema addition (an optional supplementary LST coordinate alongside the web source) or a
    join-key extension — NOT attempted this cycle: `Source`/`CacheRecord` in `apg.rs` back a
    literal-verification pipeline (`corpus_literal_sweep`) whose behavior for a non-`lst_token` kind
    I did not have budget to fully trace before risking a change.
  - **3 units** (`Fester (Mass)`, `Fiery Body`, `Transmute Potion to Poison`):
    `source.kind: "lst_corrected_ingest"` — the record deliberately cites a DIFFERENT line (a `.MOD`-
    stanza line, e.g. 1945) than the unit's own base-declaration line (e.g. 111) because that is
    where the correct `DESC:` text was recovered from a same-line-concatenation parsing defect
    (`apg.rs::corrected_ingest_spell`, SD-25). Confirmed line 111 IS a real base declaration
    (`SCHOOL:`/`CLASSES:` tokens present) — re-pointing the citation there is plausible but risks
    breaking whatever `corpus_literal_sweep`/fixture check currently verifies against line 1945;
    not attempted without deeper verification budget than this cycle had.
  **None of the 24 were widened or overwritten** — the prior cycle's caution about `apg::spell_list`
  having other consumers still applies, and is now sharpened: the residual is a citation-reconciliation
  problem, not a missing-content one.
- **PI-name-blocked spells across 6 already-covered books (~23 units)**: `inner_sea_gods` 4,
  `inner_sea_magic` 5, `inner_sea_world_guide` 5, `inner_sea_intrigue` 4, `inner_sea_faiths` 1,
  `adventurers_guide` 4 — traced individually via `cargo run --bin ingest_spells -- <book>`, which
  reports each as `"PI-dropped (name declared or blacklisted)"` by name (e.g. `inner_sea_gods`:
  `["Abadar's Truthtelling", "Gozreh's Trident", "Rovagug's Fury", "Sympathy (Shelynite)"]` — all
  deity-possessive spell names). These are genuine `decisions.md §24` candidates (the record's own
  NAME is the PI content) but `§24`'s named population (`ability`/`deity`/`class_feature`, ~1,179
  units) does not enumerate `spell`, and the Rust neutral-name port
  (`src/rules_core/codex_neutral_name.rs`) exists and is reusable (already wired into
  `cache_gen::class_feature.rs`'s `name_is_pi` branch, `codex_generated_name`/`rename` fields), but
  porting the same branch into `ingest_spells.rs` (which currently just filters PI-dropped rows out
  entirely, never emits them) is licensing-sensitive production code I judged needed its own careful
  cycle with full test coverage, not a rushed tail addition here — named for the next cycle with the
  exact mechanism to reuse.
- **`occult_adventures`'s 1** (`Repulsion`) and **`ultimate_combat`'s 1** (`Share Language
  (Communal)`): confirmed NOT defects — `cargo run --bin ingest_spells -- <book>` reports both as
  `"Cross-book collisions (kept the existing book's fuller record)"`, i.e. deliberately deduped
  against an already-modeled book's own fuller record via `already_ingested`. The `no_record` unit
  is the SAME real-world spell as an already-`matched` unit elsewhere; whether that is a ledger
  scoping question (should a deliberately-deduped unit count as `no_record` at all) is a
  `v06_work_inventory.rs` census question, out of this cycle's file scope, named for that owner.
- **`ultimate_magic`'s 3** (`um_spells_wordsofpower.lst`): confirmed real, distinct 13-line file (the
  Words of Power casting-variant subsystem) with zero config coverage — same missing-`BookInput`
  shape as `bestiary_6`, but not attempted this cycle (budget); a real, cheap next step.
- **`bestiary`'s 1** (`Veil (self only)`), **`bestiary_4`'s 2** (`Summon Monster IX (Cthulhu)` —
  confirmed genuine `NAMEISPI:YES` drop per `decisions.md §19b`, unchanged; `Summon Swarm (rat swarm
  only)` — a duplicate declaration in a SECOND file, `b4_spells_companion.lst:6`, restating the
  already-ingested `b4_spells_modified.lst:59` record verbatim, the same "different real citation,
  different route" shape the prior cycle's receipt already named), **`book_of_the_damned_volume_1`'s
  1** (`pfs_botd1_spells.lst` — the PFS-legal variant file, deliberately NOT this pipeline's target
  per the existing `BookInput` comment), **`book_of_the_damned_volume_2`'s 1** (`botd2_spells_ndl.lst`
  — the campaign-gated no-duplicates file, deliberately NOT ingested per the existing `BookInput`
  comment) — each traced to its real cause, none a missing-config-row shape, none attempted further
  this cycle.

## Discoveries

- **Discovery forward, corrected instrument:** `scripts/shape_ledger.py`'s `BOOK_CORPUS_DIR_ALIASES`
  mechanism was single-directory-per-book; any FUTURE aliased book that splits its kinds across the
  legacy and correctly-spelled directories (as `bestiary` now does for `spell`/`equipment` vs.
  `monster_ability`) would hit the identical defect. The fix generalizes to any such book, not just
  `bestiary` — no other book is in `BOOK_CORPUS_DIR_ALIASES` today (`grep -c ':' scripts/shape_ledger.py`
  shows it is a one-entry dict), so this is prospective, not a second live incident.
- **Discovery forward:** `advanced_players_guide`'s 6 `web_second_source` spell records structurally
  cannot be found by `shape_ledger.py`'s LST-only join — a real, load-bearing measurement gap
  (these units ARE done in truth, per the `Source::WebSecondSource` doc comment's own SD-25
  provenance, but will show `no_record` forever under the current join). Flagged for whoever owns
  `shape_ledger.py`'s join semantics next: either the join needs to accept a second citation-shape,
  or `WebSecondSource`/`LstCorrectedIngest` records need a supplementary LST coordinate field.

## Next-cycle plan

1. `ultimate_magic`'s Words of Power spell file (3 units) — same missing-`BookSpec`/`BookInput`
   config-row shape as this cycle's `bestiary_6`, cheapest remaining win.
2. Port `codex_neutral_name`'s `name_is_pi` branch (already proven in `cache_gen::class_feature.rs`)
   into `ingest_spells.rs` for the ~23 PI-name-blocked spell units across 6 books — needs its own
   dedicated TDD cycle given the licensing sensitivity `decisions.md §24b` calls out explicitly.
3. `advanced_players_guide`'s 24 — the citation-reconciliation trace above, split by sub-shape;
   the 17 plain-`lst_token` line-mismatches are probably the safest first sub-slice.
4. `bestiary_4`'s `Summon Swarm (rat swarm only)` duplicate-file shape and `occult_adventures`/
   `ultimate_combat`'s cross-book-collision-vs-ledger-scoping question — both belong to whoever owns
   `v06_work_inventory.rs`'s census enumeration, out of this file scope.
5. `equipment`'s 113 (was 116, -3 from this cycle's instrument fix as a side effect) — the prior
   wave's own next-cycle plan (trace `ultimate_equipment`, the largest single book) still stands.

## Disk

```bash
df -h /
```
