# Cycle t9-onboarding-equipment-citation-redirect-instrument-fix — Gate 3 closure invariant / `decisions.md §20`

- **Card ID:** `gate-1-shape-closure` (row 5, kanban.md)
- **Commit SHA:** (see push result, appended after commit)
- **Files touched:** `scripts/shape_ledger.py`, `scripts/shape_coverage_standing_gate.py`,
  `scripts/family_vocabulary_reconcile.py`, `scripts/card15_reconcile.py`,
  `scripts/tests/test_shape_ledger.py`, `docs/release/SD-32-compute-library-and-cause-closure/kanban.md`
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** Gate 3's closure condition is `no_record == 0` (`decisions.md §20`). Scope
  handed to this cycle: `equipment`'s no_record residual, currently 113 of the bundle's 299.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (oracle pin, re-verified via
  `scripts/fetch-pcgen-oracle.sh --check` at cycle start)
- **Status:** complete (instrument correction + full trace of the residual; not a `no_record == 0`
  closure — see "What is NOT done" below)

## §17a re-derivation before planning

Ran `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/shape_ledger_out.json`
against `data/corpus` (the committed corpus, not the pcgen oracle — `shape_ledger.py`'s own
`DEFAULT_CORPUS_ROOT`) before touching anything. Result matched the brief exactly:

```
no_record 299 total: monster_ability 121, equipment 113, spell 57, equipment_modifier 6, companion 2
equipment by book: inner_sea_gods 25, ultimate_magic 19, adventurers_guide 18, inner_sea_intrigue 8,
  bestiary_2 7, inner_sea_combat 7, inner_sea_world_guide 7, bestiary_3 4, ultimate_equipment 4,
  bestiary_4 3, mythic_adventures 3, advanced_class_guide 2, advanced_race_guide 2,
  advanced_players_guide 1, bestiary 1, book_of_the_damned_volume_2 1, inner_sea_races 1
```

The two prior receipts' territory was re-checked first (per the brief's "look for that same shape in
your books first"): `cache_gen::equipment_gap::book_routing()` already carries arms for every one of
my books' short codes (`ISG`, `UM`, `AG`, `ISI`, `ISC`, `ISWG`, `B2`, `B3`, `B4`, `MYTHIC`, `UE`,
`BOTD2`, `ISR`), and `gen_equipment_gap_tables.rs`'s `BOOK_INPUTS` already declares a `BookInput` for
every one of them — the missing-routing-arm / missing-config-row shape those two receipts fixed does
**not** repeat here. A fresh `cargo run --locked --bin gen_equipment_gap_tables` (`git status` clean
diff on `equipment_gap_tables.rs` afterward — it was already current) confirmed `ultimate_magic`
genuinely still generates 0 rows, matching the brief's "gap lever computes zero rows" claim.

## The real cause: a work-inventory citation mismatch, not a generator gap

Traced `ultimate_magic`'s 19 (the brief's largest untraced book) by hand, unit by unit, against both
`docs/work-inventory.json` and `data/corpus/`:

- 11 of the 19 units (e.g. `ultimate_magic:equipment:book_of_harms`) already have a real,
  content-bearing corpus record on disk — `data/corpus/ultimate_magic/equipment/book_of_harms.json`
  cites `source.path: "um_equip_general.lst"`, `source.line: 16`. But the *inventory unit*
  `shape_ledger.py` classifies cites `source_file: "pfs_um_equip_general.lst"`, `source_line: 8` — a
  content-free Pathfinder-Society-legality overlay row (`TYPE:PFSNotLegal`) that restates the same
  item's key with zero new content. `docs/work-inventory.json`'s own equipment enumeration mints
  exactly one unit per `corpus_key`; for these 11, the surviving citation happens to be the overlay
  row, not the base row the real corpus record was actually generated from. `shape_ledger.py`'s
  strict `(book, source_file, source_line)` join is exactly right to report that specific citation as
  absent — the record it names truly was never written — but the record for the SAME key, cited from
  the OTHER file, already exists.
- The other 8 of the 19 carry a declared `NAMEISPI:YES` on their base `.COPY=` row (confirmed by
  direct read of `um_equip_general.lst`) — correctly, permanently excluded from `data/corpus` by
  every existing PI screen (`gen_equipment_gap_tables.rs`'s `screen_record`), never a defect.

This fully accounts for `ultimate_magic`'s 19: 11 mismeasured, 8 genuine PI. The same PFS-overlay
citation shape recurred in `bestiary_2` (1) and `bestiary_3` (1, partially — see "not closed" below)
once the fix below was applied and the wider `no_record` set was re-examined.

## Fix: `shape_ledger.py` citation-redirect fallback (TDD, RED→GREEN)

Added `build_corpus_key_index(corpus_root, books)` — indexes every corpus record by its own declared
`(book, kind, data.key)` identity (never `data.name`, matching `equipment_gap.rs`'s own documented
`held`-map name-collision hazard: an earlier version of that generator inserted display names too and
silently suppressed 28 unrelated records that happened to share one). `classify_unit(unit,
corpus_index, key_index=None)` gained an optional `key_index` parameter consulted **only** as a
fallback when the primary `(book, source_file, source_line)` join misses — `key_index` defaults to
`None`, so every existing caller's behavior is byte-for-byte unchanged unless it opts in.

RED first: 6 new tests added to `scripts/tests/test_shape_ledger.py` (citation-redirect matches by
`(book, kind, key)`; still classifies real formula tokens when present; never fires across a
different `kind` for the same key — mirrors `equipment_gap.rs`'s own name-collision test discipline;
primary join wins when both are present; backward-compatible with no `key_index` argument) plus 2 for
the new `build_corpus_key_index` function itself — all failed with `TypeError`/`AttributeError`
before the implementation (module had no `key_index` param, no `build_corpus_key_index`). GREEN after:

```
python3 -m unittest scripts/tests/test_shape_ledger.py scripts/tests/test_shape_coverage_standing_gate.py \
  scripts/tests/test_family_vocabulary_reconcile.py
----------------------------------------------------------------------
Ran 66 tests in 0.784s
OK
```

(38/38 in `test_shape_ledger.py` alone, up from the pre-cycle 31/31 baseline confirmed green first.)

## Consumers updated so the fix is not silently inert for anyone but the CLI

`build_ledger`/`main` plumb `key_index` through. Three OTHER production call sites of
`SL.build_corpus_index`/`SL.build_ledger` exist in this repo (found by grep, not assumed) and were all
updated identically: `scripts/shape_coverage_standing_gate.py` (the actual Gate 3 standing gate —
without this update its own `no_record` figure silently disagreed with `shape_ledger.py`'s CLI: 299 vs
248, caught live by re-running both after the fix), `scripts/family_vocabulary_reconcile.py`, and
`scripts/card15_reconcile.py`. All three now call `SL.build_corpus_key_index` alongside
`SL.build_corpus_index` and pass it through to `SL.build_ledger`.

## `no_record`, before/after (`decisions.md §12c` — population + command named)

Command: `python3 scripts/shape_coverage_standing_gate.py --inventory docs/work-inventory.json --corpus-root data/corpus`
(the live Gate 3 standing gate, re-run before and after, not `shape_ledger.py`'s own CLI in isolation).

| | before | after |
|---|---:|---:|
| bundle-wide `no_record` (all 18 kinds, population 35,328) | 299 | 248 |
| `equipment` `no_record` | 113 | 87 |
| `spell` `no_record` (side effect — the fix is generic, not kind-scoped) | 57 | 32 |
| `monster_ability` / `equipment_modifier` / `companion` `no_record` | 121 / 6 / 2 | 121 / 6 / 2 (unchanged — confirmed no interference with sibling lanes) |

Gate 3 standing gate: `no_record budget: 248/35328 vs. baseline 21521/36028 -- exceeded: False`, exit 0.

## Closure figures — three separate numbers (`decisions.md §16`)

- **Closed (new content ingested):** 0. No corpus JSON was written, added, or modified this cycle.
- **Reclassified (moved shapes):** 0.
- **Instrument correction (mismeasurement fixed, zero content change):** 26 `equipment` units +
  25 `spell` units (side effect) = **51 units**, moving from `no_record` to their true `join_status`
  (`matched` or `no_formula_tokens`, whichever their now-correctly-joined record's own `raw_tokens`
  earn). **This is not closure.** These units were never un-ingested; they were mismeasured by the
  instrument, exactly the shape the brief's own precedent (the `beastiary`/`bestiary` alias fix)
  established and the brief instructs to book separately.

## The residual 87 `equipment` no_record units, traced by book (§17a, none assumed)

For every remaining book, checked each unit's own PCGen source line for a declared `NAMEISPI:YES`,
and (when absent) the item's own name against `pi_screening::PI_BLACKLIST_TERMS` (the same list
`gen_equipment_gap_tables.rs`'s `blacklist_hit` uses) for an undeclared substring hit:

| Book | Residual | Disposition |
|---|---:|---|
| `inner_sea_gods` | 25 | 25/25 Product-Identity-excluded (23 declared, 2 undeclared blacklist hit) |
| `adventurers_guide` | 18 | 18/18 Product-Identity-excluded (declared) |
| `inner_sea_intrigue` | 8 | 8/8 Product-Identity-excluded (declared) |
| `ultimate_magic` | 8 | 8/8 Product-Identity-excluded (2 declared directly, 6 via their base `.COPY=` row's own declared PI — the overlay citation was masking this, confirmed by direct read) |
| `inner_sea_combat` | 7 | 7/7 Product-Identity-excluded (2 declared, 5 undeclared blacklist hit) |
| `inner_sea_world_guide` | 7 | 7/7 Product-Identity-excluded (declared) |
| `bestiary_4` | 3 | 3/3 Product-Identity-excluded (declared) |
| `mythic_adventures` | 3 | 2/3 Product-Identity-excluded (1 declared, 1 undeclared blacklist hit); **1 untraced real gap, named below** |
| `ultimate_equipment` | 2 | 2/2 Product-Identity-excluded (declared) |
| `advanced_class_guide` | 2 | **0/2 PI — both are `.FORGET` directives, not items; named below** |
| `bestiary_2` | 1 | **0/1 PI — name-shorthand mismatch, named below** |
| `bestiary_3` | 1 | **0/1 PI — name-shorthand mismatch, named below** |
| `book_of_the_damned_volume_2` | 1 | 1/1 Product-Identity-excluded (declared) |
| `inner_sea_races` | 1 | 1/1 Product-Identity-excluded (declared) |

82 of 87 are correct, by-design Product-Identity exclusions — no action taken, no term or item name
transcribed into this receipt, the commit, or any test name (`§15`, binding after `§19`'s sign-off).
Per-unit disposition (coordinate + PI/blacklist boolean, no names) is available by re-running the
trace command below; not paginated into this file to avoid transcribing the underlying strings.

Re-derive command (produces coordinates and a PI/blacklist boolean per unit, never the term or the
item's own name):
```
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus --output /tmp/l.json
# then join no_record rows (kind==equipment) against docs/work-inventory.json's source_file/source_line
# and check that line's own PCGen row for NAMEISPI:YES / a PI_BLACKLIST_TERMS substring hit
```

## `equipment`'s 5 non-PI residual units: traced, not fixed this cycle (`decisions.md §16`)

Named explicitly, not silently narrowed out of the population:

1. **`advanced_class_guide` ×2** — `_pfs/pfs_acg_equip.lst:6` and `:7`. Both rows carry PCGen's
   `.FORGET` removal-directive suffix on their key (`gen_equipment_gap_tables.rs`'s own
   `forget_directive_keys_are_recognized_and_ordinary_keys_are_not` test already recognizes this
   shape in the generator). They are not declared items at all — the fix belongs in
   `docs/work-inventory.json`'s own equipment enumeration (`v06_work_inventory.rs`), which needs a
   `.FORGET`-suffix filter so it stops minting a unit for these rows in the first place. Out of this
   cycle's file grant (`v06_work_inventory.rs` is a large, shared, 18k-line file no lane in this wave
   claims but several depend on) and out of the scope handed to this cycle (`equipment`'s 113, "nothing
   else").
2. **`bestiary_2` ×1** (`_pfs/pfs_b2_equip_arms_armor.lst:8`) and **`bestiary_3` ×1**
   (`_pfs/pfs_b3_equip_arms_armor.lst:10`) — the same PFS-overlay shape as `ultimate_magic`'s 11, but
   the overlay row cites a SHORTHAND key (verified: the file's own content, not assumed) for an item
   already ingested under a longer, qualified key. `key_index`'s fallback correctly does **not** fire
   here — the two keys genuinely differ, so treating them as the same record would repeat exactly the
   name-collision hazard `equipment_gap.rs`'s own `held` map was fixed to avoid. A real fix needs a
   name-alias table (shorthand → canonical key), not attempted this cycle.
3. **`mythic_adventures` ×1** (`ma_equip.lst:137`) — no declared `NAMEISPI`, no blacklist hit, no
   hand-authored table entry, absent from `equipment_gap_tables.rs`'s generated output even after a
   fresh regen. This is a genuine, still-untraced gap — root cause not found this cycle.

## PI screening

No corpus write this cycle (instrument-only fix). No PI record transcribed, redacted, or exposed. The
82 correctly-excluded units above are reported by count and coordinate only, per `§15`.

## Fixture discipline (`decisions.md §3`)

RED confirmed before GREEN for every new test (see "Fix" section above — 6+2 new tests all failed for
the intended reason, `TypeError`/`AttributeError` on the not-yet-existing `key_index` param and
`build_corpus_key_index` function, before the implementation landed).

## No corpus deletions or modifications anywhere

`git status --porcelain` before commit shows only the files this receipt lists — no deletions, no
`data/corpus` changes:
```
 M docs/retro/events/sd31-transcribe.jsonl
 M docs/retro/events/t9-onboarding.jsonl
 M scripts/card15_reconcile.py
 M scripts/family_vocabulary_reconcile.py
 M scripts/shape_coverage_standing_gate.py
 M scripts/shape_ledger.py
 M scripts/tests/test_shape_ledger.py
 M docs/release/SD-32-compute-library-and-cause-closure/kanban.md
```
(The two `docs/retro/events/*.jsonl` diffs are `scripts/verify.sh`'s own auto-logged
`preflight-oracle` check events from this cycle's environment setup, not hand-edited.)

## What is NOT done, named explicitly (no silent narrowing)

- `equipment`'s `no_record` is **87, not 0**. Gate 3 is not met for this kind. The 82 correctly-PI-
  excluded units will never reach 0 by ingestion (that is the doctrine working correctly) — Gate 3's
  `no_record == 0` bar, if it is meant to hold literally, needs an operator ruling on whether a
  verified-PI-excluded unit should count against it at all, or move to a distinct disposition bucket.
  Not decided by this cycle; flagged, not adjudicated.
- The 5 non-PI stragglers above (2 `.FORGET`, 2 name-shorthand, 1 untraced) are real, small, and
  concretely scoped for a follow-up cycle.
- `monster_ability`'s 121, `equipment_modifier`'s 6, and `companion`'s 2 were explicitly out of scope
  and untouched (verified unchanged before/after).

## Discoveries

- A generic instrument defect (citation-redirect: the same key cited from two different physical
  files, only one of which the corpus record was built from) recovers real content ledger-wide, not
  just in `equipment` — `spell` moved 57→32 as a side effect with zero code touching `spell`'s own
  ingestion path. Worth a corpus-wide sweep by a future cycle to check `monster_ability`/`companion`
  for the same shape (this cycle confirmed neither kind's `no_record` count changed, meaning either
  they carry no such citation collision or their corpus records lack a `data.key` field the fallback
  can join on — not investigated further, named as an open question).
- Three OTHER scripts call `shape_ledger.build_corpus_index`/`build_ledger` directly
  (`shape_coverage_standing_gate.py`, `family_vocabulary_reconcile.py`, `card15_reconcile.py`) — any
  future change to `shape_ledger.py`'s join logic must grep for all of them, not just update the CLI.

## Next-cycle plan

1. `.FORGET`-suffix filter in `docs/work-inventory.json`'s equipment enumeration (2 units).
2. Name-alias table for PFS-overlay shorthand keys (`bestiary_2`/`bestiary_3`, 2 units; check other
   books for the same shape while there).
3. Trace `mythic_adventures`'s `Nexus Crystal` (`ma_equip.lst:137`) — why `parse_lst`/`screen_record`
   excludes a row with no PI signal at all.
4. Escalate the Gate-3-vs-PI-exclusion question named above for an operator ruling.

## Retro log

Logged via `scripts/verify.sh`'s own auto-append (preflight-oracle checks,
`docs/retro/events/t9-onboarding.jsonl` / `sd31-transcribe.jsonl`) during environment setup this
cycle. No additional `retro.py` events required — this cycle's own findings are captured in full in
this receipt and the kanban entry.

## Disk

`df -h /` reported at the end of this cycle (see final message).
