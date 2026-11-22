# Cycle 1 — gate-3-closure-invariant / `equipment_modifier` `no_record`, wave 5 (`decisions.md §20`)

- **Card ID:** card 11 (`epic-2-cause-closure`), rows 11 and 15 left `in-progress` per dispatch
  instruction.
- **Commit SHA:** (this cycle's own commit, see push log)
- **Files touched:**
  - `src/rules_core/cache_gen/equipment_gap.rs` — one new helper
    (`existing_source_line`), one new `GenerationReport` field
    (`disambiguated_collision`), and a 9-line change to `generate()`'s write path (see below);
    two new tests.
  - `src/bin/gen_cache_equipment_gap.rs` — one new `eprintln!` block reporting the new field.
  - `data/corpus/core_rulebook/equipment/equipmods/{intelligent_item_purpose_slay_all-2,
    intelligent_item_purpose_slay_creature_type-2}.json` — 2 new files, written by
    `gen_cache_equipment_gap`.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git add -N` for the two new corpus files, then
  `git diff --unified=0 HEAD -- src/rules_core/cache_gen/equipment_gap.rs
  src/bin/gen_cache_equipment_gap.rs | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` →
  no match).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff scope,
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → no match).
- **Acceptance criterion:** `decisions.md §20` — `no_record == 0` is Gate 3's closure condition.
  This receipt covers `equipment_modifier`'s brief-named 6, closing 2; see the companion
  `spell-no-record-words-of-power` receipt for the other work item this cycle.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`).

## `§17a` re-derivation before planning

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger_now.json
python3 -c "import json,collections; d=json.load(open('/tmp/ledger_now.json')); \
  c=collections.Counter(r['kind'] for r in d['rows'] if r['join_status']=='no_record'); \
  print(c.most_common())"
```
→ `monster_ability 121, equipment 113, spell 57, equipment_modifier 6, companion 2` — matches the
dispatch brief exactly. `equipment_modifier`'s 6, by unit:
`advanced_players_guide:crrsve_brst_m`, `advanced_players_guide:crrsve_brst_r`,
`adventurers_guide:special_ability_agile_maiden_armor`, `core_rulebook:intelligent_item_purpose_slay_all`,
`core_rulebook:intelligent_item_purpose_slay_creature_type`, `ultimate_combat:reach`.

## The real defect: `write_json`'s slug-collision guard has no way to tell TWO real records apart from ONE record re-run

Traced from the prior cycle's own receipt (`t9-onboarding-equipment-copy-citation-repair_cycle-1_
cycle_receipt.md`), which explicitly named this shape as out of its own scope: *"`core_rulebook`'s 2
`Intelligent Item Purpose` units (a named, DIFFERENT `write_json` slug-collision defect ... needs its
own fix)."*

`equipment_gap_tables::equipment_gap_rows()` carries the `.COPY=`-named short-form rows only
(`"Intelligent Item Purpose (Slay All)"`/`"(Slay Creature Type)"`, at real citations
`cr_equipmods.lst:895`/`:890`) — the BASE declared rows (`"Intelligent Item ~ Purpose / Slay All"`/
`"... / Slay Creature Type"`, at `:446`/`:441`) are shipped by a wholly different, earlier pipeline
and are not in this table at all. Both real names slugify to the identical filename
(`slugify` lowercases and collapses non-alphanumerics, and `~`/`/` vs `(`/`)` both collapse to `_`).
`write_json`'s own doc comment already names this exact incident (`core_rulebook`'s gap row
"colliding with an already-shipped, richer record at a DIFFERENT real citation line, 446 vs. 895")
but the guard it documents only ever has one disposition: **skip, unconditionally, whenever the path
already exists** — correct for an idempotent rerun of the SAME row (the overwhelmingly common case:
1,874 of this generator's 1,876 candidate rows hit this path, verified below), wrong for the 2 rows
where a genuinely DIFFERENT real citation is hiding behind an occupied slug.

**Fix:** before accepting the slug `write_json` would use, read the already-shipped file's own
`source.line` (`existing_source_line`, new helper — reads the on-disk JSON generically via
`serde_json::Value`, never fabricates a value, `None` on any read/parse failure). If it differs from
THIS row's own resolved citation line, call `slugify` a second time (the SAME `used`-set mechanism
that already disambiguates two collisions WITHIN one run) to get a `-2` sibling filename, and write
under that instead. If the line matches, behavior is unchanged — same slug, `write_json` still sees
the file and still skips, exactly as before.

### Blast-radius check before trusting the fix (`§17a`)

This changes a shared generator that also handles `equipment` (not just `equipment_modifier`), so
the fix was validated against the REAL run before being called safe, not assumed additive from the
diff alone:

```bash
git status --porcelain -- data/corpus | wc -l    # 3 (this cycle's OTHER work item's spell files), before
cargo run --locked --bin gen_cache_equipment_gap
```
```
Equipment gap cache generated: 0 equipment, 2 equipment_modifier records; ingested_at=...
NOTE: 2 record(s) written under a disambiguated slug -- a DIFFERENT real citation line than the
file already occupying that slug: ["core_rulebook:Intelligent Item Purpose (Slay Creature Type)
(line 890, was slug of the line-441 record)", "core_rulebook:Intelligent Item Purpose (Slay All)
(line 895, was slug of the line-446 record)"]
```
Exactly the 2 targeted units, nothing else — `report.skipped_pre_existing` still carries all 1,874
of the OTHER rows unchanged (verified: the printed `skipped_pre_existing` count did not move).
`equipment_written: 0` — this fix's population is `equipment_modifier`-only for the real corpus as
it stands today (`equipment`'s own 113 `no_record` residual is untouched, unrelated citation shapes
per the prior cycle's own receipt).

```bash
git status --porcelain -- data/corpus
#  A data/corpus/core_rulebook/equipment/equipmods/intelligent_item_purpose_slay_all-2.json
#  A data/corpus/core_rulebook/equipment/equipmods/intelligent_item_purpose_slay_creature_type-2.json
```
**Zero deletions, zero modifications** to any pre-existing file, including
`intelligent_item_purpose_slay_all.json`/`intelligent_item_purpose_slay_creature_type.json`
themselves (confirmed: `git diff --stat` on both is empty) — the base declarations' own richer
records are untouched, exactly the guarantee `write_json`'s doc comment requires.

### RED → GREEN

Two new tests in `equipment_gap.rs`:

- `existing_source_line_reads_a_real_record_and_is_none_otherwise` — proves the new helper reads a
  real `source.line`, and returns `None` (never fabricates) for an absent file and a malformed one.
- `a_different_citation_line_at_an_occupied_slug_is_disambiguated_not_dropped` — reproduces the exact
  named incident with a fixture file at `line: 446`, proves a `line: 895` row disambiguates to
  `-2`, and proves a `line: 446` rerun of the SAME row does NOT disambiguate (idempotency preserved).

```bash
cargo test --locked --lib rules_core::cache_gen::equipment_gap
```
```
running 19 tests
test rules_core::cache_gen::equipment_gap::tests::existing_source_line_reads_a_real_record_and_is_none_otherwise ... ok
test rules_core::cache_gen::equipment_gap::tests::a_different_citation_line_at_an_occupied_slug_is_disambiguated_not_dropped ... ok
...
test result: ok. 18 passed; 0 failed; 1 ignored; 0 measured; 2463 filtered out
```

**Integration-level RED → GREEN (the real binary, not just the unit test), by hand mutation:**
changed `generate()`'s own `if existing_line != line` to `if false && existing_line != line`
(mutating the production condition itself, not the test), deleted the 2 new corpus files, and reran
the real generator:
```bash
rm data/corpus/core_rulebook/equipment/equipmods/intelligent_item_purpose_slay_{all,creature_type}-2.json
cargo run --locked --bin gen_cache_equipment_gap
# RED: "Equipment gap cache generated: 0 equipment, 0 equipment_modifier records" -- no
#      disambiguation, exactly the pre-fix silent-drop behavior.
```
Reverted the mutation, reran:
```bash
cargo run --locked --bin gen_cache_equipment_gap
# GREEN: "Equipment gap cache generated: 0 equipment, 2 equipment_modifier records"
#        NOTE: 2 record(s) written under a disambiguated slug: [...]
```
Both new files reappeared byte-identical to the first run (same `source.line`/`data.key`/
`ingested_at` timestamp field aside).

```bash
cargo test --locked --lib rules_core::cache_gen::    # 146/146 pass, 11 pre-existing ignored (no regression)
```

## `no_record`, before/after

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger_after.json
```
| Kind | Before | After | Delta |
|---|---:|---:|---:|
| `equipment_modifier` | 6 | 4 | **-2** |
| `equipment` | 113 | 113 | 0 (this cycle's own report confirms `equipment_written: 0`) |
| `spell` / `monster_ability` / `companion` | 57→54 (companion lane) / 121 / 2 | see companion
  receipt / unchanged / unchanged | (spell's delta is the OTHER work item this cycle, own receipt) |

## Closure figures — three separate numbers (`decisions.md §16`)

- **Closure** (real ingest, new corpus record, `no_record` → `matched`/`no_formula_tokens`): **2**
  `equipment_modifier` units (`core_rulebook`'s two `Intelligent Item Purpose` `.COPY=`-named rows).
- **Reclassification:** none.
- **Instrument correction:** none this receipt's scope.
- **Reachability (Gate 2):** honest claim of **0** — not wired into
  `equipment_resolver::equipment_catalog_rows()` or any consumer this cycle (Gate-1 measurability
  only, same precedent every prior equipment-widening cycle in this bundle has set).

## PI screening

Zero drops. Both new records passed the SAME `declared_pi_at`/`classify_field("name", ...)` checks
`generate()`'s loop already runs unconditionally for every row (this fix sits entirely downstream of
PI screening — it only changes what happens AFTER a record has already cleared that gate and is
about to be written).

## Fixture discipline (`decisions.md §3`)

`existing_source_line` never fabricates: `None` on any read/parse failure (proved by the new test's
`not_json.json` case), and the disambiguation decision only ever WIDENS what gets written (a `Some`
differing line), never narrows what gets skipped — an unreadable existing file still blocks the
write via `write_json`'s own unconditional `path.exists()` check, unchanged.

## What is NOT done, named explicitly (no silent narrowing)

`equipment_modifier`'s residual is now **4**: `advanced_players_guide`'s 2 (`crrsve_brst_m`/`_r`,
already traced by the ledger's own book breakdown but not re-investigated this cycle),
`adventurers_guide`'s 1 (`special_ability_agile_maiden_armor`), `ultimate_combat`'s 1 (`reach`) —
none attempted this cycle; not the same slug-collision shape (none showed up in this run's
`disambiguated_collision` list, so their cause is different and untraced).

## Discoveries

- **Discovery forward:** `write_json`'s slug-collision guard is shared by BOTH `equipment` and
  `equipment_modifier` rows in `equipment_gap.rs`, and the doc comment's own incident predates this
  fix — any FUTURE `equipment` (not just `equipment_modifier`) row that collides with a
  different-line already-shipped record now gets the same disambiguation treatment automatically
  (verified this run: `equipment_written: 0`, so no such collision exists in the CURRENT `equipment`
  residual, but the fix is not `equipment_modifier`-specific).

## Disk

```bash
df -h /
```
(see cycle report below)

## Post-rebase addendum (`§17a`)

`git rebase origin/tranche/12` (§5 protocol) landed several concurrent sibling commits, none of
which touch `equipment_modifier` (the `shape_ledger.py` citation-redirect instrument fix,
`978d2152270c3ab0623c3be0c8ad39ed6cce57cc`, scopes only `equipment`/`spell`; the
`monster_ability`-round-5 commit is unrelated). `equipment_modifier`'s `6 -> 4` stands unchanged,
confirmed by a fresh `shape_ledger.py` run post-rebase (`equipment_modifier: 4`). `cargo test
--locked --lib rules_core::cache_gen::equipment_gap` re-run post-rebase: 18/18 (1 pre-existing
ignored), unchanged. See `progress.md`'s own "Post-rebase re-derivation" section for the full
bundle-wide figure.
