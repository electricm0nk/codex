# Cycle t9-monster-ability-owner-less-ingest-round3 — Gate 3 (closure invariant) / Card 11 (T9)

- **Card ID:** 11 (`epic-2-cause-closure`, T9 sub-lane
  `t9-monster-ability-owner-less-ingest-round3`)
- **Commit SHA:** `6d7fd2e081` (pushed to `tranche/12`)
- **Files touched:**
  - `scripts/transcribe_monster_tables.py` — `BOOKS` dict gains 5 new
    entries (`ultimate_wilderness`, `ultimate_intrigue`, `ultimate_magic`,
    `bestiary_6`, `bestiary_5`); the mechanism itself is **unchanged**, run
    unmodified for each new book.
  - `src/bin/gen_book_cache.rs` — 5 new `MonsterBookSpec` rows (all
    `races_lsts: &[]` — these books have zero monster rows of their own);
    `bestiary_6` and `bestiary_5` each needed one extra `abilities_lsts`
    entry after this generator's own citation refusal caught it
    (`ce_abilities_race.lst`, `b5_abilities_race_oa.lst`).
  - `src/rules_core/rules_tables/monster_chassis.rs` — 5 new `MonsterBook`
    rows in `MONSTER_BOOKS`.
  - `src/rules_core/rules_tables/{bestiary_5,bestiary_6,ultimate_intrigue,
    ultimate_magic,ultimate_wilderness}/mod.rs` — each gains
    `mod monster_data;`, a `monster_chassis` re-export (deliberately
    excluding `NaturalAttack`/`Speed`/`StatAdjustment` where the module
    already imports the companion chassis' own same-named types), and
    `monsters_static()`/`monster_abilities_static()` wrappers.
  - `src/rules_core/rules_tables/{bestiary_5,bestiary_6,ultimate_intrigue,
    ultimate_magic,ultimate_wilderness}/monster_data.rs` (new, generated via
    `scripts/transcribe_monster_tables.py <book>`).
  - `data/corpus/{bestiary_5,bestiary_6,ultimate_intrigue,ultimate_magic,
    ultimate_wilderness}/monster_ability/*.json` (76 new files, via
    `gen_book_cache <book>`) and each book's `LICENSE.json` (screening-note
    append).
  - `apps/desktop/src-tauri/src/monster_catalog.rs` — 5 new wire codes
    (`BOOK_UW`/`BOOK_UI`/`BOOK_UM`/`BOOK_B6`/`BOOK_B5`, reusing the codes
    this app already serves each book's other families under) wired into
    `book_display_name`/`book_wire_code`; the corpus-wide owner-less-count
    pin moves 881 → 957 (+76).
  - `apps/desktop/src-tauri/src/reach_gate.rs` — 5 new
    `("<book>", "monster_abilities") => Some(chassis_monster_abilities_reach(...))`
    reach-claim arms, 5 new `UNREACHED_RECORD_FINDINGS` entries (76 exact
    keys total), 5 new `OPEN_FINDINGS` entries.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped diff against this
  cycle's own start point `3113458009`, the substantive files above — 0
  hits; the untracked new `monster_data.rs` files separately swept clean).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope — 0 hits).
- **Acceptance criterion:** `decisions.md §20` — drive `monster_ability`
  `no_record` toward zero.
- **Corpus SHA:** `PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`; bootstrapped from empty this cycle via
  `scripts/fetch-pcgen-oracle.sh`).
- **Status:** complete (partial application of the overall `no_record==0`
  goal — see "What remains" below; card 11 stays `in-progress`).
- **Notes:** see full body below. **Correction of record against the prior
  cycle's own "Next-cycle plan"** — see §1.
- **Discovery forwards:** none filed this cycle — the remaining scope (2
  hand-rolled generators to extend, 1 new module to scaffold, and the real
  per-record residual across already-registered books) is named explicitly
  in §5/§6 below with counts.
- **Next-cycle plan:** see §6.

---

## 1. `decisions.md §17a` re-derive — the prior receipt's own claim was stale

The prior cycle
(`t9-monster-ability-owner-less-ingest-remaining-books_cycle-1_cycle_receipt.md`,
"Next-cycle plan") stated: *"no further 'apply the mechanism to book N'
cycles remain of this shape."* Re-deriving the brief's own figures fresh, per
`decisions.md §17a`, found this **false**.

```bash
export PCGEN_REPO_DIR=".../artifacts/corpus/operator-supplied/pcgen"
export PCGEN_CORPUS_ROOT="$PCGEN_REPO_DIR/data"
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger_before.json
```
`monster_ability` `no_record`: **267**, matching the brief's own figure
exactly (bundle total `no_record`: 1,114, also exact). Per-book split:

```bash
python3 -c "... Counter(r['book'] for r in ledger['rows'] if r['join_status']=='no_record' and r['kind']=='monster_ability') ..."
```
```
pathfinder_unchained  72   bestiary_3            21   inner_sea_gods         6
bestiary_5            40   inner_sea_bestiary    12   inner_sea_world_guide  3
bestiary               23   bestiary_2            10   advanced_race_guide    1
mythic_adventures      21   horror_adventures      9
bestiary_6             16   bestiary_4             7   occult_adventures      5
```

Cross-checked against `python3 scripts/classify_monster_ability_rows.py`
(no args — every book with remaining units), whose own summary line reads:

> `of which in ZERO-monster books: 171 across 8 books (no monster row in the
> book at all, so nothing can ever own them)`

Those 8 books are exactly `pathfinder_unchained` (72), `bestiary_5` (40),
`mythic_adventures` (21), `bestiary_6` (16), `ultimate_magic` (13),
`ultimate_intrigue` (6), `ultimate_wilderness` (2), `advanced_race_guide`
(1) — **171 units, none of them among the 8 books the prior cycle
registered.** `scripts/transcribe_monster_tables.py`'s own `BOOKS` dict
confirmed none of these 8 were in it. The prior receipt's "no further apply-
the-mechanism-to-book-N cycles remain" claim was true only for the 8 books
it had itself just finished — it did not check whether other zero-monster
books existed unregistered, and 8 did.

**Correction of record, per `decisions.md §17a`.** The mechanism (an
orphan ability row ships `owners: &[]` because no monster row of its own
book can ever claim it) needs no code change; registering a book costs one
`BOOKS` entry, one `MonsterBookSpec`, one `MonsterBook` row and ~15 lines of
`mod.rs` glue — exactly `decisions.md §17`'s "generic pass, not per-object
work" cost model.

## 2. Scope taken this cycle — 5 of the 8, +76 units

Of the 8 unregistered zero-monster books, this cycle registered the 5 that
do **not** require touching a hand-rolled per-book generator function:
`ultimate_wilderness`, `ultimate_intrigue`, `ultimate_magic`, `bestiary_6`,
`bestiary_5`. Deferred, and named exactly why in §6:
`pathfinder_unchained` (72) and `advanced_race_guide` (1) each already have
a dedicated `gen_book_cache.rs` function (`gen_pathfinder_unchained`/
`gen_advanced_race_guide`) for their other families, and `mythic_adventures`
(21) has no `rules_tables/` module directory yet.

Every `.pcc` load-line for the 5 books' own primary abilities file was
verified UNGATED (no `PRECAMPAIGN` token) before registering, except one
named divergence — see §3.

## 3. A genuine gap found and fixed, not worked around (`decisions.md §17`)

`gen_book_cache bestiary_6` and `gen_book_cache bestiary_5` both refused
outright the first run: `bestiary_6` cites `ce_abilities_race.lst` (the
same `core_essentials` cross-book file `bestiary`/`bestiary_2`/`bestiary_3`/
`bestiary_4` already register), and `bestiary_5` cites
`b5_abilities_race_oa.lst`. Widened both `MonsterBookSpec.abilities_lsts`
entries (2 lines).

**`b5_abilities_race_oa.lst` needed a divergence recorded, per
`decisions.md §22`.** `_bestiary_5.pcc:66` loads it under
`PRECAMPAIGN:1,Occult Adventures` — this repo has not registered
`occult_adventures` as an included book, so PCGen's own chargen would not
load this file for a `bestiary_5`-only campaign. Registered anyway,
deliberately: `docs/work-inventory.json` independently attributes these 3
rows to `book: "bestiary_5"` (the census walker reads a book's own `.pcc`
`ABILITY:` lines regardless of `PRECAMPAIGN`, which governs optional
chargen inclusion, not which book physically owns the file) — Gate 0's
census already scoped them as this book's content, and this cycle ingests
what the census already counted rather than re-litigating it. Recorded in
the generator's own inline comment, here, and named by coordinate (not
content) — no PI concern, the file carries zero `NAMEISPI:YES` rows.

## 4. Real per-record shape found, not forced through (`decisions.md §1a`)

`bestiary_5`'s transcription printed 40 orphan candidates but shipped 39:
`b5_abilities_race.lst:96` (`Traits Output ~ Sahkil`) is a multi-`DESC:`
shape `parse_desc` refuses rather than mistranscribes — the identical class
the prior cycle's own receipt named for its 86-unit residual. Not forced
through; left `no_record`, named here and in §5.

## 5. Re-derived `no_record` — real closure, not a relabel (`decisions.md §16`)

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger_after.json
```
```
no_record (bundle total): 1,114 -> 1,038  (-76)
monster_ability:            267 ->   191  (-76)
```
Per book: `ultimate_wilderness` 2→0, `ultimate_intrigue` 6→0,
`ultimate_magic` 13→0, `bestiary_6` 16→0 (**four fully closed**),
`bestiary_5` 40→1 (the parse-refused `Traits Output ~ Sahkil` above).
2+6+13+16+(40-1) = 76, matching the ledger delta exactly.

**Re-derived again after rebasing onto `origin/tranche/12`**, which had
landed a sibling `spell` no_record closure in the interim (`3f8ddca7fd`:
`spell` 285→167, bundle total post-rebase-base 864): bundle total
864 → 788 (companion 217, `monster_ability` 191, equipment 170, spell 167,
equipment_modifier 43) — `monster_ability`'s own 267→191 delta is
unchanged by the rebase, confirming this cycle's -76 is independent of the
sibling lane's work.

**No unit was reclassified out of `monster_ability` into another kind** —
this is a genuine ingestion closure. Every other kind's `no_record` figure
is byte-identical to the pre-cycle ledger (this cycle touched no other
kind). The standing shape-coverage gate stays PASS and its budget constants
are untouched, per this cycle's brief:

```bash
python3 scripts/shape_coverage_standing_gate.py --inventory docs/work-inventory.json
```
```
no_record budget: 788/35328 vs. baseline 21521/36028 -- exceeded: False  (post-rebase re-derive)
piles reconcile: True
```

## 6. What remains (explicit, three separate figures per `decisions.md §16`)

**Closure this cycle: 76 units, real ingestion, 0 reclassified.**

`monster_ability` `no_record` remaining: **191**, in three shapes:

1. **Deferred zero-monster books — 94 units, same mechanism, more
   surgery.** `pathfinder_unchained` (72) and `advanced_race_guide` (1)
   each need their own hand-rolled `gen_pathfinder_unchained()`/
   `gen_advanced_race_guide()` extended to also call `gen_monster_book`
   for a normally-registered `MonsterBookSpec`, rather than a bare new
   spec row (their CLI dispatch already special-cases both book names to
   those functions — `src/bin/gen_book_cache.rs::main`). `mythic_adventures`
   (21) needs a new `rules_tables/mythic_adventures/` module directory
   scaffolded from scratch (feat/equipment/etc. tables, not only
   `monster_data.rs`) before the same registration steps apply. None of
   these three needs a new mechanism — only more per-book wiring than this
   cycle's remaining scope covered.
2. **Real per-record/per-facet engineering — ~92 units across the 5
   already-registered books this cycle didn't touch**
   (`bestiary` 23, `bestiary_3` 21, `inner_sea_bestiary` 12, `bestiary_2`
   10, `horror_adventures` 9, `bestiary_4` 7, `inner_sea_gods` 6,
   `inner_sea_world_guide` 3), plus `bestiary_5`'s own 1 above: multi-
   `DESC:` parse refusals, `TYPE:`-facet-vocabulary gaps, and PI-declared
   rows correctly excluded per `decisions.md §15`. Unchanged by this cycle
   — identical population the prior receipt named, re-confirmed present.
3. **`occult_adventures` (5 units) — correctly out of scope, not a gap.**
   Its monster row loads under a NEGATED `PRECAMPAIGN` gate this repo's
   campaign set fails (`!PRECAMPAIGN:1,INCLUDES=Bestiary 3`, and this repo
   DOES include Bestiary 3) — the same disqualification the original 13-
   book lane already recorded for this exact book.

`monster` kind (28 units, sibling lane's scope) — untouched.

## 7. Tests

```
cargo build --locked --lib                                            clean, 6 pre-existing warnings
cargo test --locked --lib rules_core::rules_tables::ultimate_wilderness::  12 passed
cargo test --locked --lib rules_core::rules_tables::ultimate_intrigue::    11 passed
cargo test --locked --lib rules_core::rules_tables::ultimate_magic::       18 passed
cargo test --locked --lib rules_core::rules_tables::bestiary_6::           7 passed
cargo test --locked --lib rules_core::rules_tables::bestiary_5::           4 passed
cargo test --locked --lib monster_chassis::                               8 passed
cargo run --locked --release --bin pi_sweep_rules_tables
  13 hits, 10 baseline, 3 UNBASELINED -- all 3 in feat_gap_tables.rs (`Aldori`),
  a file this cycle never touched; confirmed pre-existing (git diff against
  this cycle's own start point shows zero changes to that file or to
  ogl-pi-blacklist.md).
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins monster_catalog::
  26 passed, 0 failed (after the 5 new wire codes and the 881 -> 957 pin)
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins reach_gate::
  23 passed, 8 failed (all 8 confirmed pre-existing: grep of the failure
  detail for the 5 new books' names finds them ONLY in unrelated pre-
  existing family gaps -- classes/companions/feat_generic/template -- never
  in "monster_abilities"; same 8 test names, same pass/fail split, as the
  prior cycle's own confirmed-pre-existing baseline)
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins corpus_ingest_diagnostic::
  15 passed, 0 failed
```
RED→GREEN: the three `monster_catalog` tests
(`a_grounding_note_never_names_another_books_corpus`,
`bonus_bestiary_ability_keys_carry_the_namespace`,
`every_served_key_resolves_back_to_its_record`) failed with the real panic
messages (`no display name for chassis book "ultimate_wilderness"`, pin
mismatch `881` vs `957`) on first run after registering the 5 books but
before touching `monster_catalog.rs`; adding the wire codes and repinning
turned all 26 green.

`scripts/tests/test_shape_coverage_standing_gate.py` could not be run —
no `pytest` in this environment (`ModuleNotFoundError`) — a pre-existing
environment gap, not a regression; this cycle touches no code the gate's
own module depends on, and `shape_coverage_standing_gate.py`'s CLI itself
(§5 above) ran clean directly.

## 8. Corpus regeneration discipline

No literal-sweep/derived-fixture env vars were needed — this cycle only
ADDED new corpus files under 5 previously-empty `monster_ability`
directories via the existing generator (never regenerated an existing
populated directory with `--allow-stamp-loss` or similar).
`git status --porcelain` before commit: 25 entries, all either modified
files this receipt names or new (`??`) files under the 5 new
`monster_ability/` directories and 5 new `monster_data.rs` files — **zero
deletions**, checked explicitly (`git status --porcelain | grep '^ D\|^D '`
→ empty).
