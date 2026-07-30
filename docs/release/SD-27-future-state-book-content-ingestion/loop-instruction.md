# SD-27 — Loop Instruction

> **Mirror of SD-26's loop-instruction template.** Per-cycle procedure is the canonical six-section shape (per `loop-instruction-template.md §6`).

## 1. Per-cycle procedure (verbatim from `loop-instruction-template.md` §6)

Every cycle follows the same six-section shape, recorded in `artifacts/epic_<n>/<cycle>_receipt.md`:

1. **Cycle header** — `Cycle ID`, `Criterion`, `Owner`, `Status`, `Route class`, `Started at`, `Completed at`.
2. **Inputs** — exact file paths consulted, exact prior cycle outputs.
3. **Outputs** — exact files created/modified, exact lines added, exact commits.
4. **Operations** — RED → GREEN → REFACTOR walkthrough, dual-audit gate result.
5. **Verification** — exact commands run, exact pass/fail counts, exact receipts.
6. **Notes** — judgment calls, deferred items, audit-exclusion requests.

## 2. Pre-launch checklist

The orchestrator must verify before dispatching Epic 2.0.5+:

1. **SD-26 closure PR has landed on develop.** SD-26's `tranche/5-4 → develop` PR is the predecessor's gate. Per `decisions.md §7`.
2. **The bundle label has been resolved.** Cycle 2.0 has run and the operator's choice has been propagated across 20 surfaces (19 `data/stubs/*.json` + the `decisions.md:102` reference).
3. **The dispatcher's tier model has been authorized.** Per cycle's `Route class` field, the operator has confirmed Sonnet (default) or a free/discounted model.
4. **The Shape B v1 license-stripping pre-flight has landed.** Cycle 2.0.5 has run. `src/rules_core/shape_b_v1.rs`, `docs/governance/ogl-pi-blacklist.md`, and the per-book `LICENSE.json` template exist.
5. **The 4 in-scope books' license retro-fit has landed.** Cycles 2.0.6-2.0.9 have run; each in-scope book's `data/corpus/<book>/` has the new `license` field, and per-book `LICENSE.json` files exist.
6. **The all-23-books license-conformance verify has passed.** Cycle 2.0.10 has run with the 5th dual-audit (PI-blacklist grep) and the standard 4-grep dual-audit both clean. Gates E2.1+.
7. **The 2 future-state books (ARG, AG) are operator-approved for SD-27.** Per the operator's 2026-07-25 directive "tune, then go wide," SD-27 covers only these 2 books. The 17 deferred future-state books (Bestiary 2-6, Bonus Bestiary, Horror Adventures, Monster Codex, Mythic Adventures, Occult Adventures, Pathfinder Unchained, the 6 Tier-2 Ultimate books) are NOT in SD-27's scope; they go to SD-28+ after SD-27 closes cleanly. **Beginner Box and Core Essentials were removed from scope per operator directive 2026-07-27** (redundant to other tomes; will not be brought in). Operator-gated.
8. **The workspace-side bundle author has been promoted to `docs/release/SD-27-future-state-book-content-ingestion/`** via the `release-package-promotion` skill. The Workspace citation + repo-local canonical pattern is in force.

## 3. Cycle dispatch

The orchestrator's `Workflow` orchestrator per `workflow-orchestrated-dispatch` skill reads `epic-breakdown.md` and dispatches cycles per the per-epic concurrency + tiering map in `decisions.md §4`.

### 3.1 E1.1 — Identifier Audit

Single cycle. Backend / Sonnet.

Standard audit pattern. Produces `artifacts/epic_1/identifier-audit-cycle_receipt.md`. No renames expected.

### 3.2 E2.0 — Label Resolution

Single cycle. Backend / Sonnet. **Per-cycle blocking decision.**

```
1. Pull the operator's choice (SD-27 or SD-27+ (unscheduled))
2. For each of 19 `data/stubs/*.json` files: update `planned_resolution_bundle`
3. For each of 19 `wired-integration-stubs-registry.md` entries:
   - Update `Remediation cycle` field
4. Update `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/decisions.md:102`
   to reflect the operator's choice
5. Update `docs/release/v0.6/risks-and-open-questions.md Q2` to record the resolution
6. Run dual-audit gate
7. Commit + push + receipt
```

The choice is operator-pinned. The lead does not pick.

### 3.2.5 E2.0.5 — Shape B v1 license-stripping pre-flight

Per-cycle blocking decision. Backend / Sonnet. Gates E2.0.6+.

```
Inputs:
- SD-26's Shape B schema (the legacy shape) at
  programs/codex/requirements/SD-26-ingest-strategy-and-rule-system-plumbing/decisions.md §7
- The OGL 1.0a's "Product Identity" section (Paizo's published list)
- This bundle's forward-scope-register.md §1.3 (the new Class 1 commitment)
- 4 in-scope books' existing data/corpus/{core_rulebook,advanced_players_guide,advanced_class_guide,beastiary1}/
  records (read-only; the cycle does not modify them)
- 19 future-state books' data/stubs/<book>.json manifests (read-only)

Outputs:
- src/rules_core/shape_b_v1.rs (new Shape B schema authority):
  - Per-record `license: "OGL" | "PI" | "PI-REDACTED"` field
  - Per-record `pi_field: <field_name>` and `pi_marker: "redacted"` fields
  - Validation: every record has a license field
- docs/governance/ogl-pi-blacklist.md (new):
  - The initial PI-blacklist (deities, NPCs, places, factions, art, fiction, etc.)
  - The OGL-inlinable whitelist (class features, spell mechanics, equipment stats, feat mechanics, etc.)
  - Per-book overrides (Paizo's PI list varies by book)
- data/corpus/<book>/LICENSE.json template (for the 25 books):
  - Per-book license declaration
  - Redaction policy
  - Redistribution posture (CC-BY-compatible, OGL-notice-attached, etc.)
- tests/sd27_license_stripping_shape_v1.rs (new dual-audit gate)
- artifacts/epic_2/2.0.5-shape-b-license-stripping-preflight-cycle_receipt.md

Operations:
1. Read SD-26's Shape B schema
2. Author the v1 schema (additive — every v0 record is also a v1 record)
3. Author the PI-blacklist from the OGL's PI section
4. Author the per-book LICENSE.json template
5. Author the dual-audit test
6. Run the dual-audit gate against the schema (this cycle's test, no record-modification)
7. Commit + push + receipt

Verification:
- cargo test --workspace --locked → all green
- tests/sd27_license_stripping_shape_v1.rs passes
- src/rules_core/shape_b_v1.rs compiles and the v1 schema is documented
- The PI-blacklist is exhaustive (any field not in the blacklist is either OGL-inlinable or has a per-book override)

Notes:
- This cycle does NOT modify the 4 in-scope books' records
- This cycle does NOT modify the 19 future-state books' stubs
- The v1 schema is additive: every v0 record is also a v1 record (just missing the license field)
- The 4-grep dual-audit applies
- The PI-blacklist grep is a 5th audit (added in 2.0.6+)
```

### 3.2.6 E2.0.6 — CRB license retro-fit

Per-cycle blocking decision. Backend / Sonnet. Gates E2.0.10.

```
Inputs:
- This bundle's src/rules_core/shape_b_v1.rs (the v1 schema)
- This bundle's docs/governance/ogl-pi-blacklist.md (the PI-blacklist)
- This bundle's data/corpus/<book>/LICENSE.json template
- data/corpus/core_rulebook/ (CRB's existing Shape B records, read-modify-write)
- The 4 in-scope books are NOT a v0.6 lane; this retro-fit is the SD-27 cycle's own work

Outputs:
- data/corpus/core_rulebook/ — every Shape B record re-emitted with:
  - `license: "OGL" | "PI" | "PI-REDACTED"` field
  - PI-tagged values redacted to `"[redacted PI]"` markers
  - `pi_field` and `pi_marker` populated for PI records
- data/corpus/core_rulebook/LICENSE.json (the per-book license declaration)
- docs/governance/wired-integration-stubs-registry.md — CRB's `book_stub` entry
  updated with `license_status: "PI-stripped"` + `license_status_at: <ISO-8601>`
- artifacts/epic_2/2.0.6-crb-license-retrofit-cycle_receipt.md

Operations:
1. Read data/corpus/core_rulebook/ records
2. For each record: classify each field per the PI-blacklist
3. Re-emit the record with the new license field
4. Redact PI values to `"[redacted PI]"` markers (preserves schema)
5. Write data/corpus/core_rulebook/LICENSE.json
6. Update the wired-integration stubs registry entry
7. Run the 5th dual-audit (PI-blacklist grep)
8. Run the standard 4-grep dual-audit
9. Commit + push + receipt

Verification:
- cargo test --workspace --locked → all green
- tests/sd27_license_stripping_shape_v1.rs passes
- 5th audit: every PI-matching field has license: "PI" | "PI-REDACTED" and pi_marker: "redacted"
- 4-grep dual-audit clean
- Per-book LICENSE.json exists and matches the records

Notes:
- This cycle is file-disjoint with cycles 2.0.7 (APG), 2.0.8 (ACG), 2.0.9 (Bestiary 1) — they can run in parallel
- This cycle does NOT modify v0.6's lane (no pilot_compute.rs, no docs/release/v0.6/)
- The cycle does NOT modify src/rules_core/rules_tables/core_rulebook/
```

### 3.2.7 E2.0.7 — APG license retro-fit

Same shape as E2.0.6 but for `data/corpus/advanced_players_guide/`. Parallel-safe with E2.0.6/2.0.8/2.0.9.

Receipt: `artifacts/epic_2/2.0.7-apg-license-retrofit-cycle_receipt.md`

### 3.2.8 E2.0.8 — ACG license retro-fit

Same shape as E2.0.6 but for `data/corpus/advanced_class_guide/`. Parallel-safe with E2.0.6/2.0.7/2.0.9.

Receipt: `artifacts/epic_2/2.0.8-acg-license-retrofit-cycle_receipt.md`

### 3.2.9 E2.0.9 — Bestiary 1 license retro-fit

Same shape as E2.0.6 but for `data/corpus/beastiary1/`. Parallel-safe with E2.0.6/2.0.7/2.0.8.

Receipt: `artifacts/epic_2/2.0.9-beastiary1-license-retrofit-cycle_receipt.md`

### 3.2.10 E2.0.10 — All-25-books license-conformance verify

Per-cycle terminal state. Backend / Sonnet. Gates E2.1+.

```
Inputs:
- src/rules_core/shape_b_v1.rs (the v1 schema)
- docs/governance/ogl-pi-blacklist.md (the PI-blacklist)
- All 23 books' data/corpus/<book>/ records and LICENSE.json
- All 23 books' wired-integration stubs registry entries

Outputs:
- artifacts/epic_2/2.0.10-all-23-books-license-conformance-verify-cycle_receipt.md
  (with per-book terminal-state table: license fields populated, PI redaction complete,
  per-book LICENSE.json present and consistent, dual-audit gates pass)
- docs/governance/ogl-pi-blacklist.md — versioned at the 23-book closure;
  discovered-PI-fields added to the blacklist with provenance

Operations:
1. For each of 23 books:
   - Every Shape B record has a `license` field
   - Every PI-tagged record has `pi_marker: "redacted"`
   - Per-book LICENSE.json exists and matches the records
2. Run the 5th dual-audit across all 23 books
3. Run the standard 4-grep dual-audit
4. Document any defects as "Open blockers" (hard-stop if any defect)
5. Commit + push + receipt

Verification:
- The 5th audit passes for all 23 books
- The 4-grep dual-audit passes for the bundle's combined diff
- Per-book terminal-state table is complete

Notes:
- A defect in any book blocks E2.1+ (the next cycle fan-out)
- The 23-book sweep is a 1-cycle task; the data is file-disjoint, but the audit
  itself is one cycle's report, not 23
- The PI-blacklist grep accumulates any field-not-in-initial-blacklist found
  to be PI in any book
```

### 3.3 E2.x — Per-book pre-build + verify cycle (2 cycles, ARG + AG)

Per-book pair (pre-build + verify). Backend / Sonnet (or free/discounted per cycle 2.0's tier authorization). **This bundle covers 2 books** (Advanced Race Guide, Adventurer's Guide). The other 19 future-state books are deferred to SD-28+.

#### 3.3.1 E2.1 — Advanced Race Guide pre-build (1 cycle)

```
Inputs:
- Source LST corpus at ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/advanced_race_guide/
- **Shape B v1 schema authority** at this bundle's `src/rules_core/shape_b_v1.rs` (NOT the legacy Shape B at SD-26; v1 is the license-aware shape from E2.0.5)
- Per-book `data/corpus/<book>/LICENSE.json` template (the per-book license declaration)
- `docs/governance/ogl-pi-blacklist.md` (the PI-blacklist from E2.0.5)
- The 4 in-scope books' `data/corpus/{crb,apg,acg,beastiary1}/` as the validation reference (post-2.0.6-2.0.9 retro-fit, all v1-conformant)
- This bundle's technical-design.md §2.3 for the rules_tables/<book>/ generation pipeline
- The 2.0.10 license-conformance verify cycle's "all 23 books conform" report

Outputs:
- src/rules_core/rules_tables/advanced_race_guide/ Rust module (subset: thin layer over LST reader)
- data/corpus/advanced_race_guide/{classes,spells,equipment,feats,bestiary,races,...}/<id>.json per Shape B v1 (with `license` field, PI redaction complete)
- data/corpus/advanced_race_guide/LICENSE.json (per-book license declaration)
- tests/sd27_advanced_race_guide_cache_shape.rs (Shape B key-set + key-order conformance)
- data/stubs/advanced_race_guide.json (content_kind_counts → real number map; planned_resolution_bundle updated to "SD-27")
- docs/governance/wired-integration-stubs-registry.md entry #0003 updated to `Status: "Resolved"`
- artifacts/epic_2/advanced_race_guide_pre_build-cycle_receipt.md

Operations:
1. Read source LST corpus
2. Inventory content kinds (classes, spells, equipment, feats, races, bestiary, etc.)
3. Generate src/rules_core/rules_tables/advanced_race_guide/ Rust module
4. Run sd27_gen_book_cache codegen tool
5. Write data/corpus/advanced_race_guide/{content_kind}/{content_id}.json files (Shape B v1, with license field, PI redaction complete)
6. Write data/corpus/advanced_race_guide/LICENSE.json
7. Write tests/sd27_advanced_race_guide_cache_shape.rs
8. Update registry entry (serial-on-shared-file)
9. Update data/stubs/advanced_race_guide.json (serial-on-shared-file)
10. Run dual-audit gate
    - identifier-discipline: bash scripts/identifier-discipline-audit.sh
    - wired-integration four-check: grep + document
11. Commit + push + receipt

Verification:
- cargo test --workspace --locked → all green
- tests/sd27_advanced_race_guide_cache_shape.rs passes
- Registry entry Status: "Resolved"
- data/stubs/advanced_race_guide.json content_kind_counts matches registry
- All Shape B v1 records have a `license` field, all PI-tagged records have `pi_marker: "redacted"`

Notes:
- This cycle is file-disjoint with cycles 2.0.6-2.0.9 (4 in-scope books) and 2.2 (Adventurer's Guide)
- Operator-gated: cycle failure → operator reviews the per-cycle receipt → operator picks the next action (re-tune, defer, or proceed to E2.2)
```

#### 3.3.2 E2.1' — Advanced Race Guide verification (1 cycle)

```
Inputs:
- The pre-built cache at data/corpus/advanced_race_guide/ (output of E2.1)
- Shape B v1 schema authority
- The 2.0.10 license-conformance verify cycle's report

Outputs:
- artifacts/epic_2/advanced_race_guide_verify-cycle_receipt.md
  (with: dual-audit gate result, license-field coverage, PI-redaction coverage, content_kind_counts, per-record schema-conformance report)

Operations:
1. Read the pre-built cache
2. Run the dual-audit gate against the cache
3. Verify license-field coverage (every record has a license)
4. Verify PI-redaction coverage (every PI-tagged record has pi_marker: "redacted")
5. Verify per-book LICENSE.json exists and matches the records
6. Verify content_kind_counts consistency across data/stubs/ and data/corpus/
7. Hand the operator a verification report
8. Commit + push + receipt

Verification:
- cargo test --workspace --locked → all green
- Dual-audit gate clean
- All 25 books (4 in-scope + 2 pre-built + 19 deferred) confirmed by the previous 2.0.10 verify; this cycle is the per-book confirmation of the pre-build's output
- Operator-gated: a defect here halts SD-27's progression to E2.2
```

#### 3.3.3 E2.2 — Adventurer's Guide pre-build (1 cycle)

Same shape as E2.1 but for `data/corpus/adventurers_guide/`. Serial-after-E2.1' (operator-gated promotion).

Receipt: `artifacts/epic_2/adventurers_guide_pre_build-cycle_receipt.md`

#### 3.3.4 E2.2' — Adventurer's Guide verification (1 cycle)

Same shape as E2.1'. Serial-after-E2.2.

Receipt: `artifacts/epic_2/adventurers_guide_verify-cycle_receipt.md`


- The cycle does NOT touch src/rules_core/pilot_compute.rs (v0.6's lane)
- The cycle does NOT modify src/rules_core/rules_tables/<book>/ for the 4 in-scope books
- The cycle does NOT modify data/corpus/<book>/ for the 4 in-scope books
- The cycle does NOT modify docs/release/v0.6/
- The cycle does NOT modify src/oracle_validation/
```

### 3.4 E3.x — Per-book parity baseline cycle (19 cycles)

Per-book cycle. Backend / Sonnet (or free/discounted).

```
Inputs:
- This bundle's data/corpus/<book>/ (the E2.x output)
- SD-26 PCGen pipeline at scripts/pcgen-run-character.sh + scripts/pcgen-normalize-output.py
- SD-26 pilot Fighter pattern at programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/
- SD-26 comparator at src/oracle_validation/comparator.rs

Outputs:
- data/corpus/<book>/_parity/pf_<book>_human_<class>_level1.pcg (hand-authored)
- data/corpus/<book>/_parity/pf_<book>_human_<class>_level1.json (PCGen output, normalized)
- artifacts/epic_3/<id>_parity-cycle_receipt.md

Operations:
1. Read data/corpus/<book>/ cache
2. Author pf_<book>_human_<class>_level1_golden.pcg fixture
3. Run PCGen Gradle pipeline against the fixture
4. Sanitize output, write data/corpus/<book>/_parity/<id>.json
5. Run comparator::compare against the per-book receipt
6. Record per-cycle parity comparison in artifacts/epic_3/<id>_parity-cycle_receipt.md
7. Document the inherited CG-03 baseline (7-of-9 ceiling) in the receipt
8. Commit + push + receipt

Verification:
- cargo test --workspace --locked → all green
- PCGen Gradle pipeline runs end-to-end (BUILDSUCCESSFUL)
- comparator::compare produces a per-dimension match/mismatch table
- The cycle's assertion is "match rate at the time of cycle close"

Notes:
- Per-book cycle is file-disjoint with other books' cycles
- The cycle consu mes SD-26's PCGen pipeline without modification
- The cycle does NOT modify src/oracle_validation/
- The cycle does NOT modify any other data/corpus/<book>/ directory
- Receipt records the CG-03 baseline shift explicitly
```

### 3.5 E4.x — Closure Epic

5 cycles per `epic-breakdown.md §4`. Standard closure shape.

#### 4.1 — Final Criterion Scan

Backend / Sonnet. Per-criterion terminal-state table cross-checked against 3 independent sources (cycle receipts, kanban board, status matrix).

#### 4.2 — Architecture Closure

Backend / Opus. Truth-up + graphify + PR + merge.

#### 4.3 — Release Notes

Backend / Haiku. Standard 7-section release-notes template.

#### 4.4 — Version Bump

Backend / Haiku. 0.6.0 → 0.6.1 per `major.tranche-base.build` scheme.

#### 4.5 — PR + Merge

Backend / Sonnet. Per standing convention, the operator merges the PR.

## 4. Per-cycle failure mode

If a cycle fails the dual-audit gate or any other verification, the cycle returns to the operator with the failed-step output. The orchestrator does not auto-retry. The operator decides whether to fix forward or revert.

## 5. Per-cycle tier model

Per cycle's `Route class` field, the tier is one of:

- **Sonnet** (default). Per cycle's `.claude/settings.json` model assignment.
- **Free/model-free** (operator-authorized per `decisions.md §11`). For per-book cycles (E2.1-2.2, E3.1-3.2) only.

The orchestrator's `--tier <model>` flag sets the per-cycle tier. The flag is recorded in the cycle's `Route class` field.

## 6. Per-cycle partition enforcement

Every cycle must verify its file touches against the partition (§8 of `scope-draft.md`):

```bash
# Audit command — run before commit
git diff --name-only <branch-base>..HEAD | grep -vE '^data/corpus/(<book>/)?' | grep -vE '^docs/governance/wired-integration-stubs-registry\.md$' | grep -vE '^data/stubs/<book>\.json$' | grep -vE '^src/rules_core/rules_tables/<book>/' | grep -vE '^src/bin/sd27_gen_book_cache\.rs$' | grep -vE '^tests/sd27_'
# If any path matches, the cycle is in violation
```

A cycle that violates the partition is auto-reverted and the operator is notified.

## 7. Cross-reference

- `./scope-draft.md` — the committed scope.
- `./decisions.md` — decision record.
- `./technical-design.md` — architectural surface.
- `./technical-requirements.md` — pre-loop prerequisites + normative requirements.
- `./epic-breakdown.md` — per-cycle stories.
- `./progress.md` — live cycle log.
- `./artifacts/` — per-cycle receipt structure.
- `loop-instruction-template.md` (governance) — the canonical template.
- `workflow-orchestrated-dispatch` skill — dispatch shape.
- `wired-integration-discipline.md` (governance) — the four-check audit.
- `identifier-discipline.md` (governance) — the identifier-discipline audit.
- `release-package-promotion.md` (governance) — workspace-to-repo promotion.
- `dual-canonical-doctrine.md` (governance) — workspace-citation + repo-local canonical pattern.
