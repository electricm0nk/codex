# SD-27 — Technical Design

## 1. Architectural posture

SD-27's architectural posture is **content ingestion + per-book parity baseline**, mirroring SD-26's Epic 3 (JSON cache build) + Epic 2 (oracle-harness comparator). No new engine work. **One schema bump** (Shape B v1, license-aware extension of SD-26's Shape B, landed in cycle 2.0.5). The Shape B v1 schema (`src/rules_core/shape_b_v1.rs`) is the load-bearing authority; SD-27's caches conform to it. SD-27's v0 record shape (Shape B v0 = SD-26's legacy shape) is the precedent for the 4 in-scope books; the v0 → v1 retro-fit is in cycles 2.0.6-2.0.9.

## 2. Shape B schema application (per `src/rules_core/shape_b_v1.rs`, E2.0.5)

Each cycle writes to `data/corpus/<book>/{content_kind}/{content_id}.json` with the Shape B v1 shape:

```json
{
  "population": "in_scope",
  "completeness": "chassis_only" | "chassis_plus_extract" | "full",
  "license": "OGL" | "PI" | "PI-REDACTED",
  "pi_field": "<field_name>" | null,
  "pi_marker": "redacted" | null,
  "data": { /* content-type-specific fields */ },
  "source_lst": {
    "path": "pathfinder/paizo/roleplaying_game/<book>/<file>.lst",
    "sha256": "<hex>",
    "line": <int>
  }
}
```

The Shape B v1 schema is **additive** — every Shape B v0 record is also a v1 record (just missing the `license` field). The 4 in-scope books' v0 records are retro-fitted to v1 in cycles 2.0.6-2.0.9. The 2 in-scope future-state books' cycles (2.1-2.2) emit v1 records directly; the 17 deferred future-state books' cycles will emit v1 records when SD-28+ lands.

**Per-record `license` semantics:**
- `"OGL"` — all fields are OGL-inlinable per the OGL 1.0a. The record can be redistributed with the OGL notice.
- `"PI"` — at least one field is Product Identity; `pi_field` names the field. The value may be inlined if the operator authorizes; default is to redact.
- `"PI-REDACTED"` — at least one field is PI; `pi_field` and `pi_marker` are populated. The PI value is `"[redacted PI]"`.

**Per-book `LICENSE.json`:** at `data/corpus/<book>/LICENSE.json`, declares:
- `book_id`, `book_title`, `population`
- `ogl_version`, `ogl_notice_path`
- `pi_fields`: per-field list (e.g. `["deity", "deity_name", "npc"]`)
- `redaction_policy`: `"redact-to-marker"` (default) or operator override
- `redistribution_posture`: `"OGL-notice-attached"`, `"CC-BY-compatible"`, etc.

The 2 in-scope future-state books' `population` field is `"in_scope"` (was `"future_state"` in SD-26's stub manifests; the resolution flips the field). The 17 deferred future-state books' stubs keep `population: "future_state"` until SD-28+ lands. The `completeness` field is per-content-kind and per-record; SD-26's measured ceiling for the 4 in-scope books is the precedent (CRB equipment description 67.9%, APG equipment 97.9%, ACG equipment 98.1%, B1 equipment 100%).

### 2.1 Content kinds per book

Each book has its own content-kind inventory, derived from the source LST corpus. The 19 future-state books' canonical content kinds are:

- **classes** — class records (CRB-style `_classes.lst`).
- **spells** — spell records (CRB-style `_spells.lst`).
- **equipment** — weapons, armor, gear (CRB-style `_equipment.lst`).
- **feats** — feat records (CRB-style `_feats.lst`).
- **bestiary** — monster stat blocks (for Bestiary 2-6 + bonus_bestiary; per-monster-block shape).
- **racial** — race records (CRB-style `_racialabilities.lst` + `_abilities_race.lst`).
- **archetypes** — archetype records (CRB-style `_archetypes.lst` + `_classarchetypes.lst`).
- **traits** — trait records (CRB-style `_traits.lst`).
- **domains** — domain records (CRB-style `_domains.lst`).

The per-book cycle inventories the source LST corpus and populates whichever content kinds the source supports. A book with no archetype LST does not get an `archetypes/` directory; the absence is honest, not a missed cycle.

### 2.2 Field population strategy

Per `SD-26-decisions.md §11.3`, SD-27 caches are built by serializing the CURRENT state of the upstream `rules_tables` modules, not by re-parsing raw LST from scratch. The 19 future-state books do not have `rules_tables/<book>/` modules yet (only the 4 in-scope books do). SD-27's per-book cycle is the **first** `rules_tables/<book>/` generation for the 19 books (E2.1-2.2 for the 2 in-scope, with the 17 deferred books following in SD-28+), then the JSON cache.

**Generation pipeline:**

1. **Cycle 2.x** — Read source LST corpus at `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/<book>/`. Inventory content kinds.
2. **Cycle 2.x** — Author/derive the `rules_tables/<book>/` Rust module (subset: just enough to populate the JSON cache). This is the "Shape B enumeration" — the Rust module is a thin layer that reads the LST corpus and produces typed records.
3. **Cycle 2.x** — Run the `sd27_gen_book_cache` codegen tool against the `rules_tables/<book>/` module. Output: `data/corpus/<book>/{content_kind}/{content_id}.json` per Shape B.
4. **Cycle 2.x** — Update the `book_stub` registry entry's `content_kind_counts` field with real numbers.
5. **Cycle 2.x** — Update the `data/stubs/<book>.json` manifest to mirror the registry.
6. **Cycle 3.x** — Author the per-book `pf_<book>_human_<class>_level1_golden.pcg` fixture.
7. **Cycle 3.x** — Run the PCGen pipeline against the fixture, sanitize output, write `data/corpus/<book>/_parity/<id>.json`.
8. **Cycle 3.x** — Record the per-cycle parity comparison in `artifacts/epic_3/<id>_parity-cycle_receipt.md`.

This is a templated pipeline. The shape is identical across all 19 books; the per-book content is mechanically extractable.

### 2.3 Why no `rules_tables/<book>/` module exists for the 19 future-state books

SD-26's Epic 3 only generated caches for the 4 in-scope books (CRB, APG, ACG, Bestiary 1). The 19 future-state books were registered as stubs (Epic 4) without a `rules_tables` module because the operator accepted the stubs-only scope per `SD-26-decisions.md §2` (the four-load delivery). SD-27's per-book cycle is the first `rules_tables/<book>/` generation for those 19 books (2 in-scope in E2.1-2.2; 17 deferred to SD-28+).

This is a real piece of work (the `rules_tables/<book>/` Rust module is non-trivial), but it's bounded by the templates established in `rules_tables/{crb,apg,acg,beastiary1}/`. Each module is a thin layer over the LST reader.

## 3. Per-book ingestion pipeline (cycle 2.x abstract)

```
┌─────────────────────────────────────────────────────────────────┐
│  ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/<book>/  │
│  └─ *.lst, *.pcc, _<book>.pcc                                    │
└────────────────────────────┬────────────────────────────────────┘
                             │ sd27_gen_book_cache reads
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│  src/rules_core/rules_tables/<book>/                            │
│  └─ mod.rs, classes.rs, spells.rs, equipment.rs, feats.rs, ...  │
└────────────────────────────┬────────────────────────────────────┘
                             │ sd27_gen_book_cache serializes
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│  data/corpus/<book>/                                            │
│  └─ classes/<class_id>.json                                     │
│  └─ spells/<spell_id>.json                                      │
│  └─ equipment/<item_id>.json                                   │
│  └─ feats/<feat_id>.json                                        │
│  └─ _parity/pf_<book>_human_<class>_level1.json                 │
└─────────────────────────────────────────────────────────────────┘
```

The pipeline is templated. The per-book cycle is the same shape with book-specific content.

## 4. PCGen parity baseline (cycle 3.x abstract)

```
┌─────────────────────────────────────────────────────────────────┐
│  data/corpus/<book>/_parity/pf_<book>_human_<class>_level1.pcg  │
│  └─ hand-authored, mirrors SD-26 pilot Fighter pattern           │
└────────────────────────────┬────────────────────────────────────┘
                             │ PCGen Gradle headless run
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│  ‹tmp›/pf_<book>_<class>.xml                                    │
│  └─ PCGen's output of the fixture                               │
└────────────────────────────┬────────────────────────────────────┘
                             │ pcgen-normalize-output.py
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│  data/corpus/<book>/_parity/pf_<book>_human_<class>_level1.json  │
│  └─ normalized PCGen output, comparable to Codex's receipt       │
└────────────────────────────┬────────────────────────────────────┘
                             │ comparator (SD-26 E2.1)
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│  artifacts/epic_3/<id>_parity-cycle_receipt.md                   │
│  └─ per-dimension match/mismatch table                           │
└─────────────────────────────────────────────────────────────────┘
```

The parity baseline is the canonical reference for the 19 books' future per-cycle Oracle parity work. The 7-of-9 baseline (CG-03 inherited) is the worst-case ceiling; the per-book assertion is "match rate at cycle close."

## 5. Book-stub registry update (cycle 2.x final step)

The per-book cycle ends with two registry updates:

1. `docs/governance/wired-integration-stubs-registry.md` — the `book_stub` entry's `Status` flips from "Registered stub" to "Resolved" with `resolved_at: <ISO-8601>`, `resolved_by: claude-code`, `bundle_of_record: SD-27`, `cycle_receipt: artifacts/epic_2/<book>_pre_build-cycle_receipt.md`.

2. `data/stubs/<book>.json` — the manifest's `content_kind_counts` field is updated from `null` to a real number map (e.g. `{"classes": 50, "spells": 200, "equipment": 150, "feats": 110}`). The `planned_resolution_bundle` field stays at the value resolved in cycle 2.0.

Both updates are serial-on-the-shared-file cycles. The 19 per-book cycles serialize on `docs/governance/wired-integration-stubs-registry.md` and `data/stubs/*.json`.

## 6. Architectural decisions inherited from SD-26

- **Shape B schema** (per `SD-26-decisions.md §7`).
- **Generation strategy** (per `SD-26-decisions.md §11.3`): serialize CURRENT state of `rules_tables/<book>/`, not re-parse raw LST.
- **Per-book `feats.lst` content kind**: 185 CRB records (50+110+8+17=185) is the precedent for the JSON cache per-book feat count.
- **PCGen comparator pipeline** (per SD-26 E2.1-E2.4): `selectedParityDimensions::from_receipt` + `pcgen-normalize-output.py` + `comparator::compare`.
- **Dual-audit gate** (per `wired-integration-discipline` + `identifier-discipline`): every cycle must pass both.

No new schemas. No new engines. No new doctrine.

## 7. Cross-reference

- `./scope-draft.md` — the committed scope.
- `./decisions.md` — decision record.
- `../SD-26-ingest-strategy-and-rule-system-plumbing/technical-design.md` — predecessor's architectural surface.
- `../SD-26-ingest-strategy-and-rule-system-plumbing/decisions.md` — Shape B schema authority.
- `src/oracle_validation/` — SD-26's PCGen harness; SD-27 consumes.
- `src/rules_core/rules_tables/{crb,apg,acg,beastiary1}/` — the 4 in-scope precedents; SD-27 mirrors.
- `$PCGEN_DATA_ROOT/<book>/` (default `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/`) — the source LST corpus for each of the 19 future-state books. **Outside this repo**; `scripts/sd27-workflow.py preflight` asserts the 2 in-scope books' directories exist and are non-empty.
- `scripts/pcgen-run-character.sh` + `scripts/pcgen-normalize-output.py` — SD-25/26 PCGen pipeline; SD-27 reuses.
- `skill:workflow-orchestrated-dispatch` — dispatch shape.
- `skill:identifier-discipline` — audit + rename cycle.
- `skill:wired-integration-discipline` — four-check audit.
