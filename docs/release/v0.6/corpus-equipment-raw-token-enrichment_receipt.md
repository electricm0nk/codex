# Corpus enrichment receipt — equipment raw_tokens/raw_bonus_chains

- **Finding:** `docs/release/v0.6/book-agnostic-backend-gaps-scoping.md`, Finding 4 (desktop
  runtime corpus reachability).
- **Tool:** `src/bin/enrich_equipment_raw_tokens.rs` (new).
- **Status:** run against the real corpus, verified, committed.

## What this does

Populates the `raw_tokens`/`raw_bonus_chains` fields (added to `EquipmentCacheData` in
`shape_b_v1.rs`, commit `4a37da11`) on every existing on-disk equipment record across all 6
already-ingested books, by re-locating each record's real PCGen LST source line via its own
existing `source.path`/`source.line` citation and copying its real tokens/bonus chains. Book
-agnostic by construction — never inspects which codegen pipeline produced a record, only its
citation, which every pipeline already produces correctly.

## A real bug caught and fixed before this ran for real

The first version of this tool deserialized each record into a typed
`CorpusRecordV1<EquipmentCacheData>` Rust struct and re-serialized the whole thing. This **silently
dropped every field that struct doesn't know about** — real, caught-in-review data loss on the
first run:

- APG/ACG/Bestiary's `weight` field (a different name than CRB's `weight_lbs` — confirmed via
  `apg.rs`'s own codegen, which never adopted the `weight_lbs` name) — 279 records affected.
- PU's `equip_type`/`plus` fields (42 records affected) — PU's own `sd27_pathfinder_unchained_
  cache_shape.rs` test caught this immediately (`equipment_cache_covers_all_4_abp_slot_types_and_3_
  attunement_slot_types` failed with a real panic, not a soft assertion).

**Full run reverted (`git checkout -- data/corpus/`) before any of this was committed.** Rewrote
the tool to operate on raw `serde_json::Value` instead — read the file as generic JSON, insert
exactly 2 new keys onto the existing `data` object, write the whole object back. This preserves
every field the tool doesn't know about, by construction, regardless of how many more
book-specific schema divergences exist that haven't been found yet.

## Verification

- Re-ran clean: **2,918 enriched, 598 no-LST-citation (untouched, real absence — `web_second_
  source`/`same_book_fallback` records with no raw LST line to enrich from), 0 already-enriched,
  0 citation misses** across all 6 books.
- Confirmed no field drops this time: `git diff` shows every removed field name reappears an
  identical number of times as an added field name (key-reordering from `serde_json`'s `Value`
  serialization, not content loss) — spot-checked `equip_type`/`plus` survive with correct values
  on a real PU record.
- Spot-checked real values against known PF1 rules (Padded Armor: AC+1, MaxDex+8, ACP 0, Spell
  Failure 5%, matches the actual published rule) and against the real LST source directly (ARG's
  Dogslicer: `COST:8 WT:1 DAMAGE:1d4 CRITMULT:x2 CRITRANGE:2`, byte-exact).
- `cargo test --workspace --locked --no-fail-fast` (`PCGEN_REPO_DIR` set): **5,379 passed / 2
  failed** — both pre-existing, environment-path-dependent, identical baseline to every prior
  receipt this session. The `sd27_pathfinder_unchained_cache_shape` regression from the first
  (reverted) run is gone.
- Dual-audit gate (`identifier-discipline` + `wired-integration`), run against the prior real
  commit as base — clean.

## Scope note

This is equipment only. Spell and feat content-kinds almost certainly need the same treatment
once their own resolvers/reachability work lands (Shape B v1 was scoped this thin for every
content kind, not equipment-specifically) — not done here.
