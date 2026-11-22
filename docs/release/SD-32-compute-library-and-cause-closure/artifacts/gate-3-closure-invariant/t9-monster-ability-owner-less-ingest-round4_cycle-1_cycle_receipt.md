# Cycle t9-monster-ability-owner-less-ingest-round4 — Gate 3 (closure invariant) / Card 11 (T9)

- **Card ID:** 11 (`epic-2-cause-closure`, T9 sub-lane
  `t9-monster-ability-owner-less-ingest-round4`)
- **Commit SHA:** see push log (this file is written pre-commit; SHA recorded
  in `progress.md`'s appended receipt)
- **Files touched:**
  - `scripts/transcribe_monster_tables.py` — `BOOKS` dict gains 2 new
    entries (`pathfinder_unchained`, `advanced_race_guide`); the mechanism
    itself is **unchanged**, run unmodified for each.
  - `src/bin/gen_book_cache.rs` — 2 new `MonsterBookSpec` rows (both
    `races_lsts: &[]`); `gen_pathfinder_unchained()` and
    `gen_advanced_race_guide()` each extended with one call to the existing
    `gen_monster_book(monster_book_spec("<book>").expect(...))` after their
    existing writes, reusing the generic mechanism rather than duplicating it.
  - `src/rules_core/rules_tables/monster_chassis.rs` — 2 new `MonsterBook`
    rows in `MONSTER_BOOKS`; `widening_the_facet_vocabulary_does_not_
    reclassify_any_existing_record`'s pin repinned 3613 → 3683 (count) and
    digest `0x5c2ee6087da263c9` → `0x2fa5c4578c0267bb`, re-derived from a
    live test run, not guessed.
  - `src/rules_core/rules_tables/{pathfinder_unchained,advanced_race_guide}/mod.rs`
    — each gains `mod monster_data;`, a `monster_chassis` re-export
    (`MonsterAbilityDelivery`/`MonsterAbilityFacet`/`MonsterAbilityRecord`/
    `MonsterStatBlock`), and `monsters_static()`/`monster_abilities_static()`
    wrappers.
  - `src/rules_core/rules_tables/{pathfinder_unchained,advanced_race_guide}/monster_data.rs`
    (new, generated via `scripts/transcribe_monster_tables.py <book>`).
  - `data/corpus/{pathfinder_unchained,advanced_race_guide}/monster_ability/*.json`
    (69 + 1 = 70 new files, via `gen_book_cache <book>`) and each book's
    `LICENSE.json` (screening-note append).
  - `apps/desktop/src-tauri/src/monster_catalog.rs` — 2 new wire codes
    (`BOOK_PU`/`BOOK_ARG`, reusing the codes `equipment_catalog`/
    `race_catalog`/`companion_catalog` already serve each book's other
    families under) wired into `book_display_name`/`book_wire_code`; the
    corpus-wide owner-less-count pin moves 957 → 1027 (+70).
  - `apps/desktop/src-tauri/src/reach_gate.rs` — 2 new
    `("<book>", "monster_abilities") => Some(chassis_monster_abilities_reach(...))`
    reach-claim arms, 2 new `UNREACHED_RECORD_FINDINGS` entries (70 exact
    keys total: 69 for `pathfinder_unchained`, 1 for `advanced_race_guide`),
    2 new `OPEN_FINDINGS` entries.
  - `apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs` —
    `the_two_ingested_books_totals_reconcile_with_their_license_artifacts`'s
    `corpus_only_records` constants repinned: `advanced_race_guide` 1072 →
    1073 (+1), `pathfinder_unchained` 0 → 69 (+69), both because neither
    book's `rules_tables`-derived count function
    (`advanced_race_guide_counts()`/`pathfinder_unchained_counts()`) tracks
    the `monster`/`monster_ability` family — the same corpus-only shape
    every other family this test already accounts for uses.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped diff against this
  cycle's own start point `1846190eef`, the substantive files above — 0
  hits; the new `monster_data.rs`/`monster_ability/*.json` files separately
  swept clean).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope — 0 hits).
- **Acceptance criterion:** `decisions.md §20` — drive `monster_ability`
  `no_record` toward zero.
- **Corpus SHA:** `PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`; bootstrapped from empty this cycle via
  `scripts/fetch-pcgen-oracle.sh`).
- **Status:** complete (partial application of the overall `no_record==0`
  goal — card 11 stays `in-progress`; see "What remains" below).
- **Notes:** see full body below.
- **Discovery forwards:** none filed this cycle — the remaining scope
  (`mythic_adventures` module scaffolding, and real per-record work across
  9 already-registered books) is named explicitly in §5/§6 below with
  counts, matching the round-3 receipt's own naming.
- **Next-cycle plan:** see §6.

---

## 0. A near-miss this cycle avoided: corpus collateral damage outside scope

`cargo run --bin gen_book_cache advanced_race_guide` does not write only the
new `monster_ability` family — it re-runs the WHOLE function, including its
pre-existing feat/equipment/spell writers. The first run deleted **48
pre-existing `feat/*.json` files** (Kobold Scale Color choices, dragon-color
picker options, etc.) whose keys the generator's own "clear genuinely stale"
logic decided were absent from the current `feats()` table. This is **not**
something this cycle's monster-ability change caused — the same deletion
would happen on ANY re-run of this generator, a pre-existing drift this
cycle's `git status --porcelain` check caught before commit (`AGENTS.md`
"Never hand-edit `data/corpus/**` … Guarded path only" — but a deletion
outside this cycle's granted `monster_ability` scope is still out of scope,
whatever its cause). Restored via `git checkout HEAD -- data/corpus/advanced_race_guide/feat/`
before committing anything. `git status --porcelain` before commit shows
**zero deletions** anywhere in the tree — checked explicitly
(`git status --porcelain | grep '^ D\|^D '` → empty). Named here rather than
silently worked around: a future cycle touching either hand-rolled generator
function should expect this and re-check for unrelated deletions every time,
not just once.

## 1. Footgun 2 (stale binary) fired, self-healed inline

`cargo run --bin gen_book_cache pathfinder_unchained` panicked
`pathfinder_unchained is not registered in monster_chassis::MONSTER_BOOKS`
on the FIRST rebuild after adding the registration — twice, even after a
full `touch` + rebuild. A `cargo test --lib monster_chassis::` run in the
SAME target dir, same source, succeeded and correctly saw the new book. This
is the exact "a test can pass on a stale binary" shape the brief warns
about, mirrored: here a **binary ran stale** while the **test compiled
fresh**, in the identical `CARGO_TARGET_DIR`. Root-caused to a corrupted
incremental-compilation cache for the `dev` profile specifically (`test`
profile has separate cache state and was unaffected). Fixed by removing the
target dir and rebuilding with `CARGO_INCREMENTAL=0`; confirmed via a
temporary `eprintln!` dump of `MONSTER_BOOKS`' contents inside the running
binary, which showed the two new books present only after the clean
rebuild. This is a DIFFERENT failure mode than the prior cycle's shared-
`CARGO_TARGET_DIR` cross-agent collision (this target dir was private to
this cycle throughout) — a second, independent instance of the same class
of hazard, worth a standing note that a **private but reused**
`CARGO_TARGET_DIR` can also rot.

## 2. Scope taken this cycle — the last 2 of the original 8, +70 units

Of the 8 originally-unregistered zero-monster books the round-3 receipt
named, this cycle registered the 2 the round-3 cycle deferred because each
needed a **hand-rolled generator function extended**, not just a bare
`MonsterBookSpec` row: `pathfinder_unchained` (72 orphan candidates, 69
shipped) and `advanced_race_guide` (1 orphan, 1 shipped). Deferred, and
named exactly why in §6: `mythic_adventures` (21 units) still has no
`rules_tables/` module directory to extend.

Both books' abilities files load UNGATED at the book's own `.pcc` root
(`pu_abilities_race.lst` line 43, `arg_abilities_race.lst` line 57 — neither
carries `PRECAMPAIGN`), and both carry **zero** `NAMEISPI:YES` rows
(`grep -c NAMEISPI:YES pu_abilities_race.lst arg_abilities_race.lst` → 0, 0).

## 3. Real per-record shape found, not forced through (`decisions.md §1a`)

`pathfinder_unchained`'s transcription printed 72 orphan candidates but
shipped 69: `pu_abilities_race.lst:154/156/159` (`Elemental ~ Unchained
Eidolon LVL01/08/20`) is a multi-`DESC:` shape `parse_desc` refuses rather
than mistranscribes — the identical class the round-3 receipt named for
Bestiary 5's residual, now hit a second time in a different book. Not
forced through; left `no_record`, named here and in §6.

## 4. Re-derived `no_record` — real closure, not a relabel (`decisions.md §16`)

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger_after_r4.json
```
```
no_record (bundle total): 573 -> 503  (-70)
monster_ability:           191 ->  121  (-70)
```
Per book: `pathfinder_unchained` 72→3 (69 shipped, 3 multi-DESC refusals),
`advanced_race_guide` 1→0. 69+1 = 70, matching the ledger delta exactly.

**No unit was reclassified out of `monster_ability` into another kind** —
this is a genuine ingestion closure. Every other kind's `no_record` figure
is byte-identical to the pre-cycle ledger (`equipment` 170, `spell` 167,
`equipment_modifier` 43, `companion` 2 — all untouched, confirmed by direct
re-run above). This cycle touched no other kind.

## 5. Corpus regeneration discipline

No literal-sweep/derived-fixture env vars were needed — this cycle only
ADDED new corpus files under 2 previously-empty `monster_ability`
directories via the existing generator; the one place it touched an
EXISTING populated directory (`advanced_race_guide/feat/`, via the bundled
feat/equipment/spell re-run inside `gen_advanced_race_guide()`) is reverted
in full, per §0 above. `git status --porcelain` before commit: 12 entries,
all either modified files this receipt names or new (`??`) files under the
2 new `monster_ability/` directories and 2 new `monster_data.rs` files —
**zero deletions**, checked explicitly.

## 6. What remains (explicit, three separate figures per `decisions.md §16`)

**Closure this cycle: 70 units, real ingestion, 0 reclassified.**

`monster_ability` `no_record` remaining: **121**, in three shapes:

1. **`mythic_adventures` — 21 units, needs a new module scaffolded.** No
   `rules_tables/mythic_adventures/` directory exists yet; the round-3
   receipt named this and it is unchanged. Needs feat/equipment/etc. tables
   scaffolded before the same registration steps this cycle applied can
   land, not only `monster_data.rs` — real per-book setup work, not a bare
   registry row.
2. **Real per-record/per-facet engineering — 92 units across 9
   already-registered books, unchanged from round 3**
   (`bestiary` 23, `bestiary_3` 21, `inner_sea_bestiary` 12, `bestiary_2`
   10, `horror_adventures` 9, `bestiary_4` 7, `inner_sea_gods` 6,
   `inner_sea_world_guide` 3, `bestiary_5` 1), plus `pathfinder_unchained`'s
   own 3 new multi-`DESC:` refusals above (95 total): multi-`DESC:` parse
   refusals, `TYPE:`-facet-vocabulary gaps, and PI-declared rows correctly
   excluded per `decisions.md §15`. Re-confirmed present by this cycle's own
   ledger re-run — identical population the round-3 receipt named for the
   first 9 of these 10 books.
3. **`occult_adventures` (5 units) — correctly out of scope, not a gap.**
   Unchanged from round 3: its monster row loads under a NEGATED
   `PRECAMPAIGN` gate this repo's campaign set fails.

`monster` kind (28 units, sibling lane's scope) — untouched.

## 7. Tests

```
cargo build --locked --lib                                                clean, 8 warnings (6 pre-existing + 2 new-module unused-import, matching the sibling round-3 modules' identical shape)
cargo test --locked --lib monster_chassis::                               8 passed
cargo test --locked --lib rules_core::rules_tables::pathfinder_unchained::  92 passed, 3 ignored
cargo test --locked --lib rules_core::rules_tables::advanced_race_guide::   7 passed
cargo build --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins   clean, 41 pre-existing warnings
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins monster_catalog::
  26 passed, 0 failed (after the 2 new wire codes and the 957 -> 1027 pin)
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins reach_gate::
  23 passed, 8 failed (all 8 confirmed pre-existing: failure detail for all
  8 names only unrelated families across many OTHER books — classes/
  companions/spells/feats gaps, one PU `class_features` count drift — never
  `monster_abilities` for `pathfinder_unchained`/`advanced_race_guide`;
  exact same 23/8 split the round-3 receipt recorded as its own baseline)
```
RED→GREEN: `monster_chassis::tests::widening_the_facet_vocabulary_does_not_
reclassify_any_existing_record` failed with the real count/digest mismatch
(`left: 3613, right: <live count>` then, after repinning the count, a real
digest mismatch printing the ACTUAL new digest) on first run after
registering the 2 books but before repinning; repinning both the count
(3683) and the digest (re-derived from the failure's own `left:` value,
never guessed) turned it green.

## 8. Pinned base

`PIN=1846190eef5fa2b9b021d3afda6d8493baab44b3` — this worktree's HEAD did
not match on first check (`WRONG BASE`); `git reset --hard "$PIN"` corrected
it before any other work, per §6 step 1.
