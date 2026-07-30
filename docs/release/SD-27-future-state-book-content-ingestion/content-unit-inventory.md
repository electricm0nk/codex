# SD-27 — Content Unit Inventory

> **Per-content-unit N-tuple.** For SD-27, content units are the per-book / per-content-kind artifacts that flow into `data/corpus/<book>/` (Shape B v1 JSON cache) and `data/stubs/<book>.json` (stub manifest).

## 1. JSON cache (Epic 2 — pre-build + verify)

### 1.1 Per-book routing (per-book cycle E2.x, file-disjoint)

| Book | Path on disk | Source LST path | In-scope for SD-27? |
|---|---|---|---|
| advanced_race_guide | `data/corpus/advanced_race_guide/{class,spell,equipment,feat,race,archetype,bestiary,...}/*.json` | `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/advanced_race_guide/*.lst` | **YES** — E2.1 (pre-build) + E2.1' (verify) |
| adventurers_guide | `data/corpus/adventurers_guide/{class,spell,equipment,feat,race,archetype,bestiary,...}/*.json` | `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/adventurers_guide/*.lst` | **YES** — E2.2 (pre-build) + E2.2' (verify) |
| beginner_box | (not created) | `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/beginner_box/*.lst` | **NO** — removed from scope per operator directive 2026-07-27 |
| bestiary_2..6 | (not created in SD-27) | per-book LST corpus | NO — deferred to SD-28+ |
| bonus_bestiary | (not created) | per-book LST corpus | NO — deferred to SD-28+ |
| core_essentials | (not created) | `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/*.lst` | **NO** — removed from scope per operator directive 2026-07-27 |
| horror_adventures, monster_codex, mythic_adventures, occult_adventures, pathfinder_unchained | (not created in SD-27) | per-book LST corpus | NO — deferred to SD-28+ |
| the 6 Tier-2 Ultimate books (ultimate_campaign, ultimate_combat, ultimate_equipment, ultimate_intrigue, ultimate_magic, ultimate_wilderness) | (not created in SD-27) | per-book LST corpus | NO — deferred to SD-28+ |

**Total books in SD-27's JSON cache payload: 2** (advanced_race_guide, adventurers_guide).
**Total future-state books removed or deferred: 19** (2 removed from scope, 17 deferred).

### 1.2 Required fields per JSON file (Shape B v1 schema, per `decisions.md §7`, extended per cycle 2.0.5)

| Field | Type | Required |
|---|---|---|
| `population` | `"in_scope" \| "future_state" \| "rule_system_stub"` | yes — **in-scope future-state books flip from `future_state` (SD-26 stub) to `in_scope` (SD-27 pre-build)** |
| `completeness` | `"chassis_only" \| "chassis_plus_extract" \| "full"` | yes — per-content-kind and per-record |
| `ingested_at` | string (ISO-8601, stamped at JSON-write time) | yes — same convention as SD-26 |
| `license` | `"OGL" \| "PI" \| "PI-REDACTED"` | yes — **new field for v1 (cycle 2.0.5 schema bump)**; every Shape B v1 record carries exactly one of these |
| `pi_field` | `<field_name> \| null` | yes — names the field whose value is Product Identity (when `license` is `PI` or `PI-REDACTED`) |
| `pi_marker` | `"redacted" \| null` | yes — for PI-tagged records, must be `"redacted"` (per the 2026-07-25 OGL review's redaction-to-marker policy) |
| `data` | content-type-specific | yes |
| `source_lst` | `{ path, sha256, line }` | yes — provenance chain back to the source LST record |

The `license` + `pi_field` + `pi_marker` triad is the schema bump cycle 2.0.5 lands. v0 records (Shape B v0 = SD-26's legacy shape) survive in the 4 in-scope books but are retro-fitted to v1 in cycles 2.0.6-2.0.9.

### 1.3 Per-book content-kind inventory (per `technical-design.md §2.1`)

Each book's per-content-kind inventory is derived from the source LST corpus, not assumed from a canonical list. The 19 future-state books' canonical content kinds are:

- `classes` — class records (CRB-style `_classes.lst`)
- `spells` — spell records (CRB-style `_spells.lst`)
- `equipment` — weapons, armor, gear (CRB-style `_equipment.lst`)
- `feats` — feat records (CRB-style `_feats.lst`)
- `bestiary` — monster stat blocks (for Bestiary 2-6 + bonus_bestiary; per-monster-block shape)
- `racial` — race records (CRB-style `_racialabilities.lst` + `_abilities_race.lst`)
- `archetypes` — archetype records (CRB-style `_archetypes.lst` + `_classarchetypes.lst`)
- `traits` — trait records (CRB-style `_traits.lst`)
- `domains` — domain records (CRB-style `_domains.lst`)

Per-book cycle inventories the source LST and populates whichever content kinds the source supports. A book with no archetype LST does not get an `archetypes/` directory; the absence is honest, not a missed cycle.

### 1.4 In-scope book content ceilings (per SD-26 `decisions.md §11.4` precedent)

For SD-27's per-book cycles, the per-field completion ceiling follows SD-26's precedent. The 4 in-scope books (CRB, APG, ACG, Bestiary 1) have known ceilings; the 2 in-scope future-state books (ARG, AG) have unknown ceilings until the E3.x parity cycle re-verifies against the corpus directly.

| Book (per SD-26's measured ceiling) | Real completion ceiling (per SD-26 E4) |
|---|---|
| core_rulebook | equipment `description` 67.9% — genuine corpus ceiling, not "look harder" |
| advanced_players_guide | equipment `description` 97.9%; spell `full_text` 95.6% |
| advanced_class_guide | not touched by SD-25's pass — verify real ceiling independently |
| beastiary (Bestiary 1) | equipment 4/4 (100%) — real record count is 4 |

For ARG + AG: the per-cycle assertion is "match rate at the time of cycle close," not a fixed percentage. Inherited CG-03 baseline (7-of-9 ceiling) is documented in each parity-cycle receipt.

## 2. Book stub manifest (Epic 4 — closure epilogue updates)

### 2.1 Per-book stub manifest shape

| Output | Path | Format |
|---|---|---|
| Per-book stub manifest | `data/stubs/<book_id>.json` | `{book_id, book_name, planned_resolution_bundle, content_kind_counts, registered_at, resolved_at, resolved_by, bundle_of_record, cycle_receipt}` |
| Stubs Registry entry | `docs/governance/wired-integration-stubs-registry.md` | `book_stub` kind (existing since SD-26 E4.1) |

For the 2 in-scope future-state books (ARG, AG), the cycle updates the stub manifest's `content_kind_counts` from `null` to a real number map and flips the registry entry's `Status` from "Registered stub" to "Resolved" with `resolved_at: <ISO-8601>`, `resolved_by: claude-code`, `bundle_of_record: SD-27`, `cycle_receipt: artifacts/epic_2/<book>_cache-cycle_receipt.md`.

For the 17 deferred future-state books, the stub manifests stay at `content_kind_counts: null` until SD-28+ lands.

For the 2 removed-from-scope books (Beginner Box, Core Essentials), their registry slots (#0005 and #0012) and stub manifests, if they exist on disk, are out-of-scope and may be deleted by the closure epilogue with operator authorization.

### 2.2 Books in SD-27's stub-manifest payload (19 stubs in scope)

advanced_race_guide, adventurers_guide, bestiary_2, bestiary_3, bestiary_4, bestiary_5, bestiary_6, bonus_bestiary, horror_adventures, monster_codex, mythic_adventures, occult_adventures, pathfinder_unchained, ultimate_campaign, ultimate_combat, ultimate_equipment, ultimate_intrigue, ultimate_magic, ultimate_wilderness.

**2 in scope for SD-27's closure update (E4.x):** advanced_race_guide, adventurers_guide.
**17 deferred to SD-28+:** the remaining 17.
**2 removed from scope:** beginner_box, core_essentials (per operator directive 2026-07-27).

Registry entries map 1:1 with `data/stubs/<book>.json` files:
- #0003 advanced_race_guide, #0004 adventurers_guide, #0005 beginner_box (out-of-scope; may be deleted), #0006 bestiary_2, #0007 bestiary_3, #0008 bestiary_4, #0009 bestiary_5, #0010 bestiary_6, #0011 bonus_bestiary, #0012 core_essentials (out-of-scope; may be deleted), #0013 horror_adventures, #0014 monster_codex, #0015 mythic_adventures, #0016 occult_adventures, #0017 pathfinder_unchained, #0018 ultimate_campaign, #0019 ultimate_combat, #0020 ultimate_equipment, #0021 ultimate_intrigue, #0022 ultimate_magic, #0023 ultimate_wilderness.

After the 2 removals (registry slots #0005 and #0012), the surviving 19 registry entries are #0003-#0004 + #0006-#0021 (with #0005 and #0012 deleted; #0013-#0021 shift down by 1, OR the slots are gap-leaved for audit-trail integrity). The closure epilogue resolves the deletion-vs-gap decision per operator authorization.

## 3. Oracle-harness comparator (Epic 3 — parity baseline)

### 3.1 Per-content-unit

| Component | Path | Source canonical |
|---|---|---|
| `comparator.rs` | `src/oracle_validation/comparator.rs` | Reads from `selected_parity_dimensions.rs` |
| `normalization.rs` | `src/oracle_validation/normalization.rs` | Rules from `pcgen-run-character.sh` outputs |
| `parity_report.rs` | `src/oracle_validation/parity_report.rs` | Per-case report |
| `pcgen_runner.rs` | `src/oracle_validation/pcgen_runner.rs` | Wraps `scripts/pcgen-run-character.sh` |
| Per-book pilot case | `tests/fixtures/oracle_validation/pf_<book>_human_<class>_level1_golden.pcg` (hand-authored, mirrors SD-26 pilot Fighter pattern) | Per-book parity baseline write target |

Per-book fixture authoring (E3.x) for the 2 in-scope future-state books:
- E3.1: `pf_advanced_race_guide_human_<class>_level1_golden.pcg`
- E3.2: `pf_adventurers_guide_human_<class>_level1_golden.pcg`

### 3.2 Comparator output (per-book parity baseline)

| Output | Path | Format |
|---|---|---|
| Per-book PCGen XML output | `‹tmp›/pf_<book>_<class>.xml` | PCGen Gradle headless run output |
| Per-book normalized baseline | `data/corpus/<book>/_parity/pf_<book>_human_<class>_level1.json` | Normalized PCGen output, comparable to Codex's receipt |
| Per-book parity-cycle receipt | `artifacts/epic_3/<book>_parity-cycle_receipt.md` | Per-dimension match/mismatch table; CG-03 inherited baseline documented |

The 7-of-9 baseline (CG-03 inherited) is the worst-case ceiling; the per-book assertion is "match rate at the time of cycle close," not "9-of-9 fully oracle-checked." Documented in `forward-scope-register.md §"Class 0.3"`.

## 4. Cross-reference

- `./scope-draft.md` §3 — per-book cycle map (19 books, 2 in-scope for SD-27).
- `./scope-draft.md` §4 — file-touch partition; the dual-audit gate is the load-bearing enforcement.
- `./decisions.md` §7 — partition doctrine (no `src/rules_core/pilot_compute.rs`, no `src/oracle_validation/`, no `docs/release/v0.6/`).
- `./decisions.md` §9 — Tier-1 / Tier-2 partition for the 19 future-state books.
- `./decisions.md` §11 — per-cycle tier model (Sonnet default; free/discounted operator-authorized for per-book bodies).
- `./technical-design.md` §2 — Shape B v1 schema application.
- `./technical-design.md` §3 — per-book ingestion pipeline (cycle 2.x abstract).
- `./technical-design.md` §4 — PCGen parity baseline (cycle 3.x abstract).
- `./acceptance-and-verification.md` §2.2-2.4 — per-criterion acceptance + verification commands.
- `./loop-instruction.md` §3 — cycle dispatch (E1.1, E2.0, E2.0.5, E2.x per-book, E3.x per-book, E2.0.10 all-23-verify, E4.x closure).