# SD-27 — Acceptance and Verification

## 1. Bundle-level acceptance

The bundle is closure-ready when:

1. **Every criterion has a `complete` or `blocked` status in `progress.md`'s status matrix.** Per `loop-instruction-template.md §5`.
2. **Every cycle has a per-cycle receipt at `artifacts/epic_<n>/<cycle>_receipt.md`** with the canonical six-section shape.
3. **Every cycle has passed the dual-audit gate** (identifier-discipline + wired-integration four-check).
4. **The bundle label resolution is propagated across all 20 surfaces** (19 `data/stubs/*.json` + 1 `decisions.md:102`).
5. **The bundle's combined diff passes the `cargo test --workspace --locked` clean check.**
6. **The bundle's combined diff passes the architecture-truth-up script clean check.**
7. **The bundle's combined diff passes the four-check wired-integration audit clean check.**
7a. **The bundle's combined diff passes the 5th dual-audit (PI-blacklist grep) clean check.** Per the OGL/PI license-stripping doctrine, every Shape B record's PI-tagged fields are redacted; the audit grep returns 0 defects.
8. **The bundle's PR is opened from `tranche/7` to `develop`, ready for operator merge.**

## 2. Per-criterion acceptance

### 2.1 E1.1 — Identifier Audit

- **Acceptance:** `artifacts/epic_1/identifier-audit-cycle_receipt.md` exists with the canonical six-section shape.
- **Acceptance:** the cycle's audit raised zero new identifier-discipline violations.
- **Verification:** `bash scripts/identifier-discipline-audit.sh` → exit 0 with zero violations.

### 2.2 E2.0 — Label Resolution

- **Acceptance:** `artifacts/epic_2/label-resolution-cycle_receipt.md` exists with the canonical six-section shape.
- **Acceptance:** `data/stubs/*.json` (19 files) carry the resolved `planned_resolution_bundle` value.
- **Acceptance:** `docs/governance/wired-integration-stubs-registry.md` (19 entries) carry the resolved `Remediation cycle` value.
- **Acceptance:** `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/decisions.md:102` carries the resolved value.
- **Acceptance:** `docs/release/v0.6/risks-and-open-questions.md Q2` records the resolution.
- **Verification:** `grep -n planned_resolution_bundle data/stubs/*.json | sort | uniq -c | wc -l` → 19 (all 19 entries have the same value).
- **Verification:** `grep -n "Remediation cycle" docs/governance/wired-integration-stubs-registry.md | grep -c "<resolved-value>"` → 19.

### 2.2.5 E2.0.5 — Shape B v1 license-stripping pre-flight

- **Acceptance:** `artifacts/epic_2/2.0.5-shape-b-license-stripping-preflight-cycle_receipt.md` exists with the canonical six-section shape.
- **Acceptance:** `src/rules_core/shape_b_v1.rs` exists and compiles.
- **Acceptance:** `docs/governance/ogl-pi-blacklist.md` exists with the initial PI-blacklist and the OGL-inlinable whitelist.
- **Acceptance:** The per-book `LICENSE.json` template exists (one template, parameterized by book_id).
- **Acceptance:** `tests/sd27_license_stripping_shape_v1.rs` passes.
- **Acceptance:** The 4-grep dual-audit gate is clean.
- **Acceptance:** The v1 schema is documented with the per-record `license` / `pi_field` / `pi_marker` field semantics.
- **Verification:** `cargo test --workspace --locked` → all green.
- **Verification:** `tests/sd27_license_stripping_shape_v1.rs` passes.
- **Verification:** Every v0 Shape B record (the legacy shape) is also a valid v1 record (proves additive).

### 2.2.6 E2.0.6 — CRB license retro-fit

- **Acceptance:** `artifacts/epic_2/2.0.6-crb-license-retrofit-cycle_receipt.md` exists.
- **Acceptance:** Every `data/corpus/core_rulebook/{content_kind}/{content_id}.json` record has a `license` field.
- **Acceptance:** Every record with a PI-tagged value has `pi_field` and `pi_marker: "redacted"`, and the PI value is `"[redacted PI]"`.
- **Acceptance:** `data/corpus/core_rulebook/LICENSE.json` exists with the per-book declaration.
- **Acceptance:** `docs/governance/wired-integration-stubs-registry.md` CRB entry updated with `license_status: "PI-stripped"` and `license_status_at: <ISO-8601>`.
- **Acceptance:** The 5th dual-audit (PI-blacklist grep) and the 4-grep dual-audit both clean.
- **Verification:** `cargo test --workspace --locked` → all green.
- **Verification:** `tests/sd27_license_stripping_shape_v1.rs` passes.
- **Verification:** `grep -l '"license": "OGL"' data/corpus/core_rulebook/*/*.json | xargs -I{} sh -c 'pi=$(grep -E "(deity|npc|place|faction|art_url)" {} | grep -v null | wc -l); if [ $pi -gt 0 ]; then echo "DEFECT: {}"; fi' | wc -l` → 0 (no record has OGL license + PI value).

### 2.2.7 E2.0.7 — APG license retro-fit

Same acceptance/verification as 2.2.6 but for `data/corpus/advanced_players_guide/`. Receipt: `artifacts/epic_2/2.0.7-apg-license-retrofit-cycle_receipt.md`.

### 2.2.8 E2.0.8 — ACG license retro-fit

Same acceptance/verification as 2.2.6 but for `data/corpus/advanced_class_guide/`. Receipt: `artifacts/epic_2/2.0.8-acg-license-retrofit-cycle_receipt.md`.

### 2.2.9 E2.0.9 — Bestiary 1 license retro-fit

Same acceptance/verification as 2.2.6 but for `data/corpus/beastiary1/`. Receipt: `artifacts/epic_2/2.0.9-beastiary1-license-retrofit-cycle_receipt.md`.

### 2.2.10 E2.0.10 — All-23-books license-conformance verify

- **Acceptance:** `artifacts/epic_2/2.0.10-all-23-books-license-conformance-verify-cycle_receipt.md` exists with the per-book terminal-state table.
- **Acceptance:** All 23 books' Shape B records have a `license` field.
- **Acceptance:** All 23 books' per-book `LICENSE.json` exists and matches the records.
- **Acceptance:** The 5th dual-audit (PI-blacklist grep) is clean for all 23 books.
- **Acceptance:** The 4-grep dual-audit is clean for the bundle's combined diff.
- **Acceptance:** The PI-blacklist is exhaustive (any field not in the blacklist is either OGL-inlinable or has a per-book override).
- **Verification:** per-book sweep with terminal-state table; one row per book; no defects.
- **Verification:** `cargo test --workspace --locked` → all green.
- **Verification:** the 5th audit grep returns 0 defects across all 23 books.

### 2.3 E2.x — Per-book cache cycle (2 cycles in scope; 19 deferred to SD-28+)

- **Acceptance:** `artifacts/epic_2/<book>_cache-cycle_receipt.md` exists with the canonical six-section shape.
- **Acceptance:** `data/corpus/<book>/{content_kind}/{content_id}.json` files exist per Shape B schema.
- **Acceptance:** `data/stubs/<book>.json` carries real `content_kind_counts` (not `null`).
- **Acceptance:** `docs/governance/wired-integration-stubs-registry.md` entry for `<book>` reads `Status: "Resolved"`.
- **Acceptance:** `tests/sd27_<book>_cache_shape.rs` passes Shape B key-set + key-order conformance.
- **Verification:** `cargo test --workspace --locked` → all green (including `tests/sd27_<book>_cache_shape.rs`).
- **Verification:** `bash scripts/identifier-discipline-audit.sh` → exit 0.
- **Verification:** `bash scripts/wired-integration-audit.sh` → exit 0.

### 2.4 E3.x — Per-book parity baseline cycle (2 cycles in scope; 19 deferred to SD-28+)

- **Acceptance:** `artifacts/epic_3/<book>_parity-cycle_receipt.md` exists with the canonical six-section shape.
- **Acceptance:** `data/corpus/<book>/_parity/pf_<book>_human_<class>_level1.pcg` exists.
- **Acceptance:** `data/corpus/<book>/_parity/pf_<book>_human_<class>_level1.json` exists (normalized PCGen output).
- **Acceptance:** the comparator's per-dimension match/mismatch table is recorded in the receipt.
- **Acceptance:** the inherited CG-03 baseline (7-of-9 ceiling) is documented in the receipt.
- **Verification:** PCGen Gradle pipeline runs end-to-end (BUILDSUCCESSFUL).
- **Verification:** `cargo test --workspace --locked` → all green.

### 2.5 E4.1 — Final Criterion Scan

- **Acceptance:** `artifacts/epic_4/final-criterion-scan-cycle_receipt.md` exists with the canonical six-section shape.
- **Acceptance:** the per-criterion terminal-state table cross-checks against 3 independent sources (cycle receipts, kanban board, status matrix).
- **Verification:** manual review of the per-criterion table.

### 2.6 E4.2 — Architecture Closure

- **Acceptance:** `artifacts/epic_4/architecture-closure-cycle_receipt.md` exists with the canonical six-section shape.
- **Acceptance:** `architecture-truth-up` script clean against the bundle's combined diff.
- **Acceptance:** graphify-update succeeded (green exit).
- **Verification:** `bash scripts/architecture-truth-up.sh` → exit 0.
- **Verification:** `graphify cluster-only` → exit 0.

### 2.7 E4.3 — Release Notes

- **Acceptance:** `release-notes.md` is populated with the canonical 7-section shape.
- **Acceptance:** the per-book resolution table is complete.
- **Verification:** manual review.

### 2.8 E4.4 — Version Bump

- **Acceptance:** build version is `0.6.1` (post-SD-27-promotion).
- **Acceptance:** all four version-anchor files are updated (`apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json`, `apps/desktop/src-tauri/Cargo.toml`, plus the `0.5.` anchor in `buildVersionTriple.test.ts:44-47`).
- **Verification:** `cargo test --workspace --locked` → all green.

### 2.9 E4.5 — PR + Merge

- **Acceptance:** PR is opened from `tranche/7` to `develop`.
- **Acceptance:** PR title follows the convention `<bundle-slug> — <bundle-name>`.
- **Acceptance:** PR body references `artifacts/epic_4/closure-readiness-report.md`.
- **Acceptance:** PR is not auto-merged; the operator merges per standing convention.
- **Verification:** `gh pr view --json title,state,body` → expected strings present.

## 3. Cross-reference

- `./scope-draft.md` — the committed scope.
- `./decisions.md` — decision record.
- `./technical-design.md` — architectural surface.
- `./technical-requirements.md` — pre-loop prerequisites + normative requirements.
- `./epic-breakdown.md` — per-cycle stories.
- `./loop-instruction.md` — per-cycle procedure.
- `./progress.md` — live cycle log.
- `./release-notes.md` — bundle summary at closure.
- `./artifacts/` — per-cycle receipt structure.
- `documents/governance/repo-root-AGENTS.md` — repo-local governance rules.
- `loop-instruction-template.md` (governance) — the canonical template.
