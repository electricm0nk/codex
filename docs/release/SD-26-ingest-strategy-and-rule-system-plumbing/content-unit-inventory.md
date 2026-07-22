# SD-26 — Content Unit Inventory

> **Per-content-unit N-tuple.** For SD-26, content units are the per-book / per-content-kind artifacts that flow into `data/corpus/` and `data/stubs/`.

## 1. JSON cache (Epic 3)

> **Generation strategy + real per-book ceilings corrected per `decisions.md §11`** (SD-25's actual corpus-intake execution, not the pre-execution estimates this file originally carried). E3's cycles generate from the completed Rust `rules_tables` modules, not a fresh LST re-parse — see `technical-design.md §3.3`.

### 1.1 Per-book routing

| Book | Path on disk | Source LST path | Real completion ceiling reached (SD-25, per `decisions.md §11.4`) |
|---|---|---|---|
| core_rulebook | `data/corpus/core_rulebook/{class,spell,equipment}/*.json` | `pathfinder/paizo/roleplaying_game/core_rulebook/*.lst` | equipment `description` 2021/2977 (67.9%) — genuine corpus ceiling, not "look harder" |
| advanced_players_guide | `data/corpus/advanced_players_guide/{class,spell,equipment}/*.json` | `pathfinder/paizo/roleplaying_game/advanced_players_guide/*.lst` | equipment `description` 331/338 (97.9%); spell `full_text` 284/297 (95.6%) |
| advanced_class_guide | `data/corpus/advanced_class_guide/{class,spell,equipment}/*.json` | `pathfinder/paizo/roleplaying_game/advanced_class_guide/*.lst` | not touched by this SD-25 pass — verify real ceiling independently before E3.3 |
| beastiary (Bestiary 1) | `data/corpus/beastiary/{monster,template}/*.json` | `pathfinder/paizo/roleplaying_game/bestiary/*.lst` | equipment 4/4 (100%) — **real record count is 4, not the ~7 originally estimated**; no spell-list concept exists for this book at all (confirmed, not a gap) |

### 1.2 Required fields per JSON file (Shape B schema, per `decisions.md §7`, corrected per `decisions.md §11.1`/`§11.2`)

| Field | Type | Required |
|---|---|---|
| `population` | `"in_scope" \| "future_state" \| "rule_system_stub"` | yes |
| `completeness` | `"chassis_only" \| "chassis_plus_extract" \| "full"` | yes |
| `ingested_at` | string (ISO-8601, stamped at JSON-write time) | yes — **new field; do not derive from `git log`, see `decisions.md §11.1`** |
| `data` | content-type-specific | yes |
| `source.kind` | `"lst_token" \| "lst_inherited_copy" \| "lst_corrected_ingest" \| "web_second_source" \| "same_book_fallback"` | yes — **discriminated union, replaces the old single-shape `source_lst`; see `decisions.md §11.2` for per-kind required sub-fields** |

`source_lst.{path,sha256,line,record_key}` (the original single shape) survives only as the required sub-fields when `source.kind == "lst_token"`. The other four kinds have their own required sub-fields — full spec in `decisions.md §11.2`. This is not a cosmetic rename: a real, substantial fraction of the fields SD-25 already populated (e.g. 100% of APG's populated equipment descriptions) have no `lst_token`-shaped provenance at all.

## 2. Book stub manifest (Epic 4)

### 2.1 Per-book routing

21 cycles, one per future-state book. Each cycle writes:

| Output | Path | Format |
|---|---|---|
| Per-book stub manifest | `data/stubs/<book_id>.json` | `{book_id, book_name, planned_resolution_bundle, content_kind_counts: null, registered_at: <ISO-8601>}` |
| Stubs Registry entry | `docs/governance/wired-integration-stubs-registry.md` | `book_stub` kind (added in E4.1) |

### 2.2 Books in scope

advanced_race_guide, adventurers_guide, beginner_box, bestiary_2, bestiary_3, bestiary_4, bestiary_5, bestiary_6, bonus_bestiary, core_essentials, horror_adventures, monster_codex, mythic_adventures, occult_adventures, pathfinder_unchained, ultimate_campaign, ultimate_combat, ultimate_equipment, ultimate_intrigue, ultimate_magic, ultimate_wilderness.

**Pre-existing count discrepancy, resolved 2026-07-22 publishing pass:** this list has exactly **21** entries — `core_rulebook`, `advanced_players_guide`, `advanced_class_guide`, `bestiary` (4 books) are in-scope; these 21 are the future-state books; 4 + 21 = 25 real PF1 books total, not 26. The pre-publish bundle carried "22 future-state books" / "22 entries" / "Criterion 4.2..4.23" (22 cycles) across multiple files with no 22nd book name listed anywhere; the publish-pass correction direction (consistent with `technical-requirements.md §3.3` which already said "21 future-state books") is to take the 21 names as canonical and correct every "22"/"4.2..4.23" reference to "21"/"4.2..4.22" across the bundle. All references now consistent (see `risks-and-open-questions.md §4 Q5` for the resolution record).

## 3. Oracle-harness comparator (Epic 2)

### 3.1 Per-content-unit

| Component | Path | Source canonical |
|---|---|---|
| `comparator.rs` | `src/oracle_validation/comparator.rs` | Reads from `selected_parity_dimensions.rs` |
| `normalization.rs` | `src/oracle_validation/normalization.rs` | Rules from `pcgen-run-character.sh` outputs |
| `parity_report.rs` | `src/oracle_validation/parity_report.rs` | Per-case report |
| `pcgen_runner.rs` | `src/oracle_validation/pcgen_runner.rs` | Wraps `scripts/pcgen-run-character.sh` |
| Pilot case verification | `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt` | Upgrade `current_claim_status` |

## 4. Cross-reference

- `./scope-draft.md §1` — Epic decomposition
- `./epic-breakdown.md §3` — per-cycle stories (per-book routing)
- `./decisions.md §7` — JSON schema (Shape B)
- `./decisions.md §8` — Stubs Registry `book_stub` kind
