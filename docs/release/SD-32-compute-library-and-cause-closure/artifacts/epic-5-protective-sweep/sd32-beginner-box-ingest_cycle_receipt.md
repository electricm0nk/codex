# Cycle receipt — `sd32-beginner-box-ingest`

**Actor:** `sd32-beginner-box-ingest`
**Branch:** `tranche/12`
**Oracle pin:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`)

## Task

Drive `no_record` (`decisions.md §20`) back to zero by ingesting the 14 `beginner_box`
`equipment` units a sibling lane's removal of `pf1e_dashboard_producer.py`'s
`EXCLUDED_BOOKS = {'beginner_box'}` carve-out surfaced.

## Re-derivation (not trusted from the brief)

```
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json
```

Before this cycle (verified live, matches the brief's figures exactly):

```
population : 34416
join_status: matched 11671, no_formula_tokens 22731, no_record 14
```

Coordinates of the 14 (`docs/work-inventory.json` × `/tmp/ledger.json` `rows[].join_status
== "no_record"`, all `kind: equipment`, `book: beginner_box`): `cloak_of_the_dark_tapestry`,
`energy_heart`, `feathered_shield`, `heroic_tabard`, `ioun_stone_mossy_disk`,
`poison_resistant_scale_mail`, `scroll_of_new_life`, `shadow_helm`, `sihedron_medallion`,
`staff_of_curing`, `staff_of_guarding`, `staff_of_scorching`, `staff_of_swampy_dread`,
`sustaining_bowl`. **Confirmed: 14, not a stale figure.** `beginner_box`'s full population is
19 units; the other 5 (`bandages_of_rapid_recovery`, `campfire_bead`, `dawnflower_sash`,
`flying_ointment`, `glowing_glove`) were already `no_formula_tokens` via a coincidental
cross-book name match (identical items reprinted verbatim in `advanced_players_guide` /
`inner_sea_gods`) — real records, just not `beginner_box`'s own.

Oracle presence (the `§27b` hard-impossibility test), verified directly, not assumed:

```
find .../operator-supplied/pcgen/data -iname '*bbox*'
  .../beginner_box/bbox_equip_magic_items.lst
  .../beginner_box/bbox_equip_arms_armor.lst
```

Both files present. Neither `§27b` admissible reason (source absent, licensing forbids
shipping) applies.

## Finding: an inherited carve-out this task's own premise had to overturn

`src/bin/v06_work_inventory.rs`'s `out_of_scope` set carried `beginner_box` under an
operator directive dated 2026-07-27 ("redundant to other tomes, will not be brought in").
`decisions.md §27b` (2026-08-23, `sd32-beginner-box-ingest` scope): *"EVERYTHING... no
'unregistered book' exemption... the only admissible reason for a unit not to close is a
hard impossibility."* This directly overturns the 2026-07-27 disposition — recorded and
removed (see Remediation below), not silently worked around.

## Remediation — the guarded generator path

`beginner_box` had never been onboarded into the equipment gap lane
(`gen_equipment_gap_tables.rs` → `equipment_gap_tables.rs` → `cache_gen::equipment_gap` →
`data/corpus/<book>/equipment/*.json`), the established path 26 other already-compiled books
already use for equipment with no hand-authored per-book table. No new `RuleSetId`; the gap
lane needs none (confirmed by reading `book_routing`'s actual code, not its precedent
comments — no `RuleSetId`/`COMPILED_RULE_SETS` dependency at all).

**Files changed** (book-registration + the generator's own config, no hand-written corpus
JSON):

- `src/bin/gen_equipment_gap_tables.rs` — `EQUIPMENT_BOOK_BB` const + `BookInput` entry
  (2 `.lst` files: `bbox_equip_magic_items.lst`, `bbox_equip_arms_armor.lst`).
- `src/rules_core/cache_gen/equipment_gap.rs` — `book_routing("BB")` arm; both hardcoded
  test coverage lists (`book_routing_covers_every_non_ue_gap_book`,
  `find_citation_full_population_regression`) extended.
- `src/bin/v06_work_inventory.rs` — `equipment_book_slug_for("BB")` arm (else the resolver's
  own panic guard hard-crashes every caller the moment `equipment_catalog_rows()` carries
  `"BB"` rows); `beginner_box` removed from the stale `out_of_scope` set.
- `src/rules_core/rules_tables/equipment_gap_tables.rs` — **regenerated**
  (`cargo run --locked --bin gen_equipment_gap_tables`), 1954 → 1973 rows
  (+19, all `beginner_box`).
- `data/corpus/beginner_box/equipment/*.json` — **regenerated**
  (`cargo run --locked --bin gen_cache_equipment_gap`), 19 new files. 1 of the 19
  (`bbox_equip_magic_items.lst:16`) carries a declared/blacklisted name and shipped under a
  Codex-generated neutral identity per `decisions.md §24` (not excluded).
- `tests/equipment_gap_tables.rs` — `EXPECTED_PER_BOOK` `("BB", 19)` entry; total assertion
  1954 → 1973.

Never hand-edited `data/corpus/**` — both writes are the guarded generator's own no-clobber
`write_json` path.

## Acceptance criterion

```
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json
  population         : 34416   (unchanged)
  no_record           : 0      (was 14)
  no_formula_tokens   : 22745  (was 22731, +14 — the 14 formerly-no_record units, now
                                 real own-book corpus records with zero DEFINE/BONUS tokens)
  matched             : 11671  (unchanged)
```

All 19 `beginner_box` `equipment` rows now resolve to their own-book corpus record (spot
check: `rows[].join_status` for `book == "beginner_box"` — 19/19 `no_formula_tokens`, 0
`no_record`; the 5 previously cross-book-matched rows now match their own book too).

**`no_record: 0`. Acceptance criterion met.**

## Count sweep (`decisions.md`'s own "a count change compiles clean and still leaves other
files' assertions red" lesson)

```
grep -rn '1954\b' tests src apps --include=*.rs   # only tests/equipment_gap_tables.rs (fixed above)
grep -rn '1973\b' tests src apps --include=*.rs   # equipment_gap_tables.rs header + the fixed test
grep -rln '34397\|34,397\|22731\|22,731'          # no hits outside docs/release/**
grep -rn 'beginner_box' tests src apps --include=*.rs --include=*.py --include=*.ts --include=*.tsx
  # only the 4 files listed above under Remediation
```

No other file carried a stale pinned count.

## Verification run (scoped, no unscoped sweep)

```
cargo test --locked --test equipment_gap_tables                          # 7/7 pass
cargo test --locked --lib equipment_gap::                                # 29 pass, 1 ignored (oracle-gated, unrelated)
cargo test --locked --bin v06_work_inventory equipment_book_slug_for     # 1/1 pass
cargo test --locked --test v06_work_inventory                            # 16/16 pass, 1 ignored (2-min double-run, unrelated)
```

`git status --porcelain` checked immediately after both generator runs — only the intended
files changed; no shared output tree was touched.

## Dual-audit gate (diff vs pre-cycle origin/tranche/12 HEAD, my paths only)

```
OK_NO_BUNDLE_TAGS
OK_NO_TOKENS
```

## Movement buckets

- **Closure:** 14 `no_record` beginner_box equipment units closed (real corpus records,
  guarded generator, PI-screened). `no_record` reaches its `§20` closure condition: zero.
- **Reclassification:** 0 (no unit changed family/status outside the join fix itself).
- **Reachability:** unaffected — `§16` keeps reachability its own number; ingestion and
  measurement are separate from whether a modelled campaign set reaches these units.
- **Instrument-correction:** the `out_of_scope` carve-out in `v06_work_inventory.rs` was a
  stale disposition, not an instrument bug — corrected per `§27b`, recorded above rather
  than silently dropped.

## Out-of-territory, reported not fixed

`v06_work_inventory.rs`'s `beginner_box` book now classifies `scope: "unregistered"`
(no compiled `RuleSetId` — no `monster`/`race_trait`/`class_feature`/`spell` tables exist for
it). That is a real, honest label, not an exemption: its `equipment` units are ingested and
measured like every other book's. Whether `beginner_box` warrants a full `RuleSetId`
(feat/spell/class_feature/race_trait tables) for `§27a`'s "all the shapes, every book, 100%"
pass is out of this cycle's `equipment`-only territory — flagged for whichever lane owns
that pass, not silently left as a new carve-out.
