# SD-27 — Loop Instruction

> **Mirror of SD-26's loop-instruction template.** Per-cycle procedure is the canonical six-section shape (per `loop-instruction-template.md §6`).

## 1. Per-cycle procedure (verbatim from `loop-instruction-template.md` §6)

Every cycle follows the same six-section shape, recorded in `artifacts/epic_<n>/<cycle>-cycle_receipt.md`
(e.g. `artifacts/epic_2/advanced_race_guide_pre_build-cycle_receipt.md` — note the `-cycle_receipt.md`
suffix, matching `artifacts/README.md`):

1. **Cycle header** — `Cycle ID`, `Criterion`, `Owner`, `Status`, `Route class`, `Started at`, `Completed at`.
2. **Inputs** — exact file paths consulted, exact prior cycle outputs.
3. **Outputs** — exact files created/modified, exact lines added, exact commits.
4. **Operations** — RED → GREEN → REFACTOR walkthrough, dual-audit gate result.
5. **Verification** — exact commands run, exact pass/fail counts, exact receipts.
6. **Notes** — judgment calls, deferred items, audit-exclusion requests.

## 2. Pre-launch checklist

The orchestrator must verify before dispatching Epic 2.0.5+:

1. **SD-26 closure PR has landed on develop.** — **SATISFIED as of 2026-07-27, corrected same day.** An earlier note here said "landed as PR #339, not #338" — that was backwards, caught by an independent multi-agent review. Verified against `origin/develop`'s actual history: commit `62e7b617` ("Merge pull request **#338** from electricm0nk/tranche/5-4") is a confirmed ancestor of `origin/develop`, and the SD-26 package + `src/bin/sd26_gen_core_rulebook_cache.rs` are both present there. **PR #338 is the real closure PR and it did land.** PR #339, merged separately two commits later (`96a031dd`), is an unrelated CG-03 ability-modifier bugfix — commit `803ee60e`'s own message ("tranche/5-4 was merged (PR #338)") says #338, not #339; the earlier note here misread its own citation. **Caveat:** SD-26's own `progress.md` on develop is stale on this point — its terminal `6.5 PR + merge` row still reads "PR opened, awaiting operator merge" and was never updated after the GitHub merge actually happened. That row lagging reality doesn't change that PR #338 merged; it's a known paper-trail gap (SD-26's own closure-readiness-report.md §3 documents several of these). Per `decisions.md §7`.
2. **The bundle label has been resolved.** Cycle 2.0 has run and the operator's choice has been propagated across the 19 in-universe `data/stubs/*.json` files + the SD-26 `decisions.md:102` reference. Note `data/stubs/` holds **21** files — `beginner_box.json` and `core_essentials.json` are descoped but still present, so any glob-and-count assertion must exclude them by name.
3. **The dispatcher's tier model has been authorized.** Per cycle's `Route class` field, the operator has confirmed Sonnet (default) or a free/discounted model.
4. **The Shape B v1 license-stripping pre-flight has landed.** Cycle 2.0.5 has run. `src/rules_core/shape_b_v1.rs`, `docs/governance/ogl-pi-blacklist.md`, and the per-book `LICENSE.json` template exist.
5. **The 4 in-scope books' license retro-fit has landed.** Cycles 2.0.6-2.0.9 have run; each in-scope book's `data/corpus/<book>/` has the new `license` field, and per-book `LICENSE.json` files exist.
6. **The all-23-books license-conformance verify has passed.** Cycle 2.0.10 has run with the 5th dual-audit (PI-blacklist grep) and the standard 4-grep dual-audit both clean. Gates E2.1+.
7. **The 2 future-state books (ARG, PU) are operator-approved for SD-27.** Per the operator's 2026-07-25 directive "tune, then go wide," SD-27 covers only these 2 books. The 17 deferred future-state books (Adventurer's Guide, Bestiary 2-6, Bonus Bestiary, Horror Adventures, Monster Codex, Mythic Adventures, Occult Adventures, the 6 Tier-2 Ultimate books) are NOT in SD-27's scope; they go to SD-28+ after SD-27 closes cleanly. **Beginner Box and Core Essentials were removed from scope per operator directive 2026-07-27** (redundant to other tomes; will not be brought in). Operator-gated.
8. **The bundle is repo-resident at `docs/release/SD-27-future-state-book-content-ingestion/`.** Promotion via the `release-package-promotion` skill has already happened; this is the canonical and only copy (`decisions.md §6`).

## 3. Cycle dispatch

The in-harness **`Workflow` tool** — not a headless script (`decisions.md §19`) — reads `epic-breakdown.md` per the `workflow-orchestrated-dispatch` skill and dispatches cycles per the per-epic concurrency + tiering map in `decisions.md §4`.

Cycle *state* (which stage is claimable, by whom, and what it produced) is not tracked in prose: it lives in the reporting manifest and is driven by `scripts/sd27-workflow.py`. Run `python3 scripts/sd27-workflow.py preflight` before dispatching E2.1, and `… next` to see what is actually claimable. See §8.

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
5. Update `docs/release/v0.6/risks-and-open-questions.md` §"Open questions" item 2 (line 102) to record the resolution.
   This is the single authorized write into `docs/release/v0.6/` — see the §6 audit exception.
6. Run dual-audit gate
7. Commit + push + receipt
```

The choice is operator-pinned. The lead does not pick.

### 3.2.5 E2.0.5 — Shape B v1 license-stripping pre-flight

Per-cycle blocking decision. Backend / Sonnet. Gates E2.0.6+.

```
Inputs:
- SD-26's Shape B schema (the legacy shape) at
  docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/decisions.md §7
- The OGL 1.0a's "Product Identity" section (Paizo's published list)
- This bundle's forward-scope-register.md §1.3 (the new Class 1 commitment)
- 4 in-scope books' existing data/corpus/{core_rulebook,advanced_players_guide,advanced_class_guide,beastiary}/
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
- data/corpus/<book>/LICENSE.json template (for the 23 books):
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
- artifacts/epic_2/2.0.6-crb-license-retrofit-cycle_receipt.md

**Corrected 2026-07-27 (found while executing this cycle for real):** the registry-entry-update output
above described a `book_stub` entry for CRB that does not exist — `docs/governance/wired-integration-stubs-registry.md`
only registers the 19 future-state books' stubs (`#0003`-`#0023`); the 4 in-scope books (CRB/APG/ACG/
Bestiary 1) were ingested directly by SD-22/SD-26 and were never stub-registered. There is nothing to
update in the registry for cycles 2.0.6-2.0.9. This step is a documented no-op, not a skipped step.

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

Same shape as E2.0.6 but for `data/corpus/beastiary/`. Parallel-safe with E2.0.6/2.0.7/2.0.8.

**The corpus directory is `beastiary`** — no trailing digit. Only the rules-engine tree uses `src/rules_core/rules_tables/beastiary1/`. Verified on disk 2026-07-27.

Receipt: `artifacts/epic_2/2.0.9-beastiary-license-retrofit-cycle_receipt.md`

### 3.2.10 E2.0.10 — All-23-books license-conformance verify

Per-cycle terminal state. Backend / Sonnet. Gates E2.1+.

```
Inputs:
- src/rules_core/shape_b_v1.rs (the v1 schema)
- docs/governance/ogl-pi-blacklist.md (the PI-blacklist)
- The 4 in-scope books' data/corpus/<book>/ records and LICENSE.json
  (the ONLY books with a corpus at this point — see the scope note below)
- All 21 book_stub registry entries (#0003-#0023)

Scope note (corrected 2026-07-27):
  This cycle runs BEFORE E2.1, so only the 4 in-scope books have a corpus at all.
  The 19 future-state books are still stubs; ARG and PU are not built until E2.1/E2.2.
  "All-23-books conformance" therefore means "every book is in a known, conformant
  state", which is two different assertions:
    - 4 in-scope books  -> corpus exists and is v1-conformant (license field on every
      record, PI redaction complete, per-book LICENSE.json present and consistent)
    - 19 future-state   -> NO corpus, and that absence is honest: the stub manifest
      exists with content_kind_counts: null, and the registry entry is unresolved
  Asserting "all 23 books' corpus records conform" literally is impossible here and
  must not be attempted — 19 of the 23 have no records to check.

Outputs:
- artifacts/epic_2/2.0.10-all-23-books-license-conformance-verify-cycle_receipt.md
  (with per-book terminal-state table: license fields populated, PI redaction complete,
  per-book LICENSE.json present and consistent, dual-audit gates pass)
- docs/governance/ogl-pi-blacklist.md — versioned at this closure; discovered-PI-fields
  added to the blacklist with provenance

Operations:
1. For each of the 4 in-scope books (the only ones with a corpus):
   - Every Shape B record has a `license` field
   - Every PI-tagged record has `pi_marker: "redacted"`
   - Per-book LICENSE.json exists and matches the records
2. For each of the 19 future-state books:
   - data/corpus/<book>/ does NOT exist (the gap is honest, not a defect)
   - data/stubs/<book>.json exists with content_kind_counts: null
   - the registry entry is present and unresolved
3. Run the 5th dual-audit (PI-blacklist grep) across the 4 in-scope corpora
4. Run the standard 4-grep dual-audit
5. Document any defects as "Open blockers" (hard-stop if any defect)
6. Commit + push + receipt

Verification:
- The 5th audit passes for all 4 in-scope corpora
- The 4-grep dual-audit passes for the bundle's combined diff
- Per-book terminal-state table is complete: 23 rows, 4 "corpus conformant",
  19 "stub, honest gap"

Notes:
- A defect in any book blocks E2.1+ (the next cycle fan-out)
- The 23-book sweep is a 1-cycle task; the data is file-disjoint, but the audit
  itself is one cycle's report, not 23
- E2.1'/E2.2' re-run the corpus half of this check against their own freshly built
  book — this cycle cannot have covered them, since they did not exist yet
- The PI-blacklist grep accumulates any field-not-in-initial-blacklist found
  to be PI in any book
```

### 3.3 E2.x — Per-book pre-build + verify cycle (4 cycles: ARG + PU, pre-build + verify each)

Per-book pair (pre-build + verify). Backend / Sonnet (or free/discounted per cycle 2.0's tier authorization). **This bundle covers 2 books** (Advanced Race Guide, Pathfinder Unchained) — matching the operator's `SD-27 (ARG + PU)` dashboard workchannel. The other 17 future-state books are deferred to SD-28+; Adventurer's Guide among them, routed to SD-30.

#### 3.3.1 E2.1 — Advanced Race Guide pre-build (1 cycle)

```
Inputs:
- Source LST corpus at $PCGEN_DATA_ROOT/advanced_race_guide/ (23 .lst files; default root
  ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ — outside this repo)
- **Shape B v1 schema authority** at this bundle's `src/rules_core/shape_b_v1.rs` (NOT the legacy Shape B at SD-26; v1 is the license-aware shape from E2.0.5)
- Per-book `data/corpus/<book>/LICENSE.json` template (the per-book license declaration)
- `docs/governance/ogl-pi-blacklist.md` (the PI-blacklist from E2.0.5)
- The 4 in-scope books' `data/corpus/{core_rulebook,advanced_players_guide,advanced_class_guide,beastiary}/` as the validation reference (post-2.0.6-2.0.9 retro-fit, all v1-conformant)
- This bundle's technical-design.md §2.2 for the rules_tables/<book>/ generation pipeline
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
    - identifier-discipline: bash scripts/identifier-discipline-audit.sh  -> OK_NO_BUNDLE_TAGS
    - wired-integration four-check: bash scripts/wired-integration-audit.sh -> AUDIT PASSED
      (both are diff-scoped: set BASE_BRANCH to the cycle's own base, not the long-lived
       integration branch, or the audit reports the whole branch's history)
11. Commit + push + receipt

Verification:
- cargo test --workspace --locked → all green
- tests/sd27_advanced_race_guide_cache_shape.rs passes
- Registry entry Status: "Resolved"
- data/stubs/advanced_race_guide.json content_kind_counts matches registry
- All Shape B v1 records have a `license` field, all PI-tagged records have `pi_marker: "redacted"`

Notes:
- This cycle is file-disjoint with cycles 2.0.6-2.0.9 (4 in-scope books) and 2.2 (Pathfinder Unchained)
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
- This cycle is the FIRST conformance check of this book's corpus. 2.0.10 could not have
  covered it: 2.0.10 gates E2.1, which is what builds this corpus. 2.0.10 confirmed the 4
  in-scope corpora + the 19 honest stub gaps; this cycle confirms the newly built book.
- Operator-gated: a defect here halts SD-27's progression to E2.2

Notes (apply to every per-book cycle in §3.3):
- The cycle does NOT touch src/rules_core/pilot_compute.rs (v0.6's lane)
- The cycle does NOT modify src/rules_core/rules_tables/<book>/ for the 4 in-scope books
- The cycle does NOT modify data/corpus/<book>/ for the 4 in-scope books
  (the 2.0.6-2.0.9 retro-fit cycles are the sole authorized writers there — decisions.md §8)
- The cycle does NOT modify docs/release/v0.6/
- The cycle does NOT modify src/oracle_validation/
```

#### 3.3.3 E2.2 — Pathfinder Unchained pre-build (1 cycle)

Same shape as E2.1 but for `data/corpus/pathfinder_unchained/`. File-disjoint with E2.1, so it may run
in parallel; the operator may still gate it behind E2.1' under the "tune, then go wide" model.

Receipt: `artifacts/epic_2/pathfinder_unchained_pre_build-cycle_receipt.md`

#### 3.3.4 E2.2' — Pathfinder Unchained verification (1 cycle)

Same shape as E2.1'. Serial-after-E2.2.

Receipt: `artifacts/epic_2/pathfinder_unchained_verify-cycle_receipt.md`

### 3.4 E3.x — Per-book parity baseline cycle (2 cycles)

Per-book cycle, one per in-scope book (ARG, PU). Backend / Sonnet (or free/discounted).

```
Inputs:
- This bundle's data/corpus/<book>/ (the E2.x output)
- SD-26 PCGen pipeline at scripts/pcgen-run-character.sh + scripts/pcgen-normalize-output.py
- SD-26 pilot Fighter pattern at tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt
  (the `programs/codex/requirements/GE-05-.../artifacts/` path is workspace-only; no `programs/` tree exists in this repo)
- SD-26 comparator at src/oracle_validation/comparator.rs

Outputs:
- data/corpus/<book>/_parity/pf_<book>_human_<class>_level1.pcg (hand-authored)
- data/corpus/<book>/_parity/pf_<book>_human_<class>_level1.json (PCGen output, normalized)
- artifacts/epic_3/<book>_parity-cycle_receipt.md

Operations:
1. Read data/corpus/<book>/ cache
2. Author pf_<book>_human_<class>_level1_golden.pcg fixture
3. Run PCGen Gradle pipeline against the fixture
4. Sanitize output, write data/corpus/<book>/_parity/pf_<book>_human_<class>_level1.json
5. Run comparator::compare against the per-book receipt
6. Record per-cycle parity comparison in artifacts/epic_3/<book>_parity-cycle_receipt.md
7. Document the inherited CG-03 baseline (7-of-9 ceiling) in the receipt
8. Commit + push + receipt

Verification:
- cargo test --workspace --locked → all green
- PCGen Gradle pipeline runs end-to-end (BUILDSUCCESSFUL)
- comparator::compare produces a per-dimension match/mismatch table
- The cycle's assertion is "match rate at the time of cycle close"

Notes:
- Per-book cycle is file-disjoint with other books' cycles
- The cycle consumes SD-26's PCGen pipeline without modification
- The cycle does NOT modify src/oracle_validation/
- The cycle does NOT modify any other data/corpus/<book>/ directory
- Receipt records the CG-03 baseline shift explicitly
```

### 3.5 E4.x — Closure Epic

5 cycles per `epic-breakdown.md §4`. Standard closure shape.

#### 3.5.1 E4.1 — Final Criterion Scan

Backend / Sonnet. Per-criterion terminal-state table cross-checked against 3 independent sources (cycle receipts, kanban board, status matrix).

#### 3.5.2 E4.2 — Architecture Closure

Backend / Opus. `bash scripts/architecture-truth-up.sh` + `bash scripts/graphify-update.sh` + PR. Both gates refuse a dirty working tree — commit first.

#### 3.5.3 E4.3 — Release Notes

Backend / Haiku. Populates the 7-section `release-notes.md` template, which ships unpopulated.

#### 3.5.4 E4.4 — Version Bump

Backend / Haiku. 0.6.0 → 0.6.1 per `major.tranche-base.build` scheme.

#### 3.5.5 E4.5 — PR + Merge

Backend / Sonnet. Per standing convention, the operator merges the PR.

## 4. Per-cycle failure mode

If a cycle fails the dual-audit gate or any other verification, the cycle returns to the operator with the failed-step output. The orchestrator does not auto-retry. The operator decides whether to fix forward or revert.

## 5. Per-cycle tier model

Per cycle's `Route class` field, the tier is one of:

- **Sonnet** (default).
- **Free/model-free** (operator-authorized per `decisions.md §11`). For per-book cycles (E2.1-2.2, E3.1-3.2) only.

The tier is chosen per `Workflow`-tool agent invocation (its `model` option) and recorded verbatim in
the cycle's `Route class` receipt field. There is no `--tier` flag and no `.claude/settings.json` in
this repo — both belonged to the headless-dispatcher framing that `decisions.md §19` retired.

## 6. Per-cycle partition enforcement

Every cycle must verify its file touches against the partition (`scope-draft.md §4`, with `decisions.md §8` as the authority where they differ — §8 of scope-draft is *Hard-stop conditions*, not the partition):

Substitute the cycle's own book id for `<book>` before running this — the patterns are literal,
not templated at runtime.

```bash
# Audit command — run before commit. Every allowed path is subtracted; whatever
# survives the pipeline is a partition breach.
BOOK=advanced_race_guide   # or pathfinder_unchained; the cycle's own book
git diff --name-only <branch-base>..HEAD \
  | grep -vE "^data/corpus/${BOOK}/" \
  | grep -vE '^docs/governance/wired-integration-stubs-registry\.md$' \
  | grep -vE '^docs/governance/ogl-pi-blacklist\.md$' \
  | grep -vE "^data/stubs/${BOOK}\\.json$" \
  | grep -vE "^src/rules_core/rules_tables/${BOOK}/" \
  | grep -vE '^src/rules_core/shape_b_v1\.rs$' \
  | grep -vE '^src/bin/sd27_gen_book_cache\.rs$' \
  | grep -vE '^tests/sd27_' \
  | grep -vE '^docs/release/SD-27-future-state-book-content-ingestion/' \
  | grep -vE '^scripts/sd27-workflow\.py$'
# If any path survives, the cycle is in violation
```

**Cycle 2.0 exception.** The label-resolution cycle is the one cycle whose prescribed outputs
(§3.2 steps 4 and 5) fall outside the list above. When and only when running cycle 2.0, also subtract:

```bash
  | grep -vE '^docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/decisions\.md$' \
  | grep -vE '^docs/release/v0\.6/risks-and-open-questions\.md$'
```

Without this, cycle 2.0 breaches its own gate and auto-reverts — the two writes it is *required* to
make are both otherwise forbidden (`docs/release/v0.6/` is on the must-not-touch list). No other
cycle may subtract these.

> **Do not relax the first pattern back to `^data/corpus/(<book>/)?`.** The optional group makes it
> match bare `^data/corpus/`, which subtracts *every* book's corpus — so a cycle writing another
> book's records, including the 4 in-scope books that are v0.6's lane, passes the audit silently.
> Verified 2026-07-27: with the optional form, a `core_rulebook` record such as
> `data/corpus/core_rulebook/class/<id>.json` survives as "allowed" from a
> `pathfinder_unchained` cycle. The partition's single most important
> rule is unenforced in that form.

A cycle that violates the partition is auto-reverted and the operator is notified.

## 7. Cross-reference

- `./scope-draft.md` — the committed scope (§3 per-book cycle map; §4 file-touch partition).
- `./decisions.md` — decision record (§8 partition authority; §19 dispatch; §20 reporting).
- `./technical-design.md` — architectural surface.
- `./technical-requirements.md` — pre-loop prerequisites + normative requirements.
- `./epic-breakdown.md` — per-cycle stories; what the dispatcher reads.
- `./progress.md` — live cycle log.
- `./artifacts/` — per-cycle receipt structure.
- `docs/governance/loop-instruction-template.md` — the canonical template.
- `docs/governance/no-stub-mvp-doctrine.md` — the wired-integration parent doctrine.
- `docs/governance/wired-integration-stubs-registry.md` — the `book_stub` entries this bundle resolves.
- `scripts/sd27-workflow.py` — dispatch-state driver + reporting writer (§8).
- `scripts/identifier-discipline-audit.sh`, `scripts/wired-integration-audit.sh` — the dual-audit gate.
- `scripts/architecture-truth-up.sh`, `scripts/graphify-update.sh` — E4.2 closure gates.

**Machine-local skills** (real, but outside this repo — they resolve under `$HERMES_HOME/profiles/god-emporer/skills/`, not `~/.hermes/`, and not under `docs/governance/`): `workflow-orchestrated-dispatch` (dispatch shape), `wired-integration-discipline` (four-check audit), `identifier-discipline` (identifier audit), `release-package-promotion` (workspace-to-repo promotion), `dual-canonical-doctrine` (workspace-citation + repo-local canonical pattern), `kanban-claude-code-execution-receipt` (receipt schema). The runnable halves of the two audit skills are vendored into `scripts/` above, so no cycle depends on the skills tree being present.

## 8. Reporting surface (mandatory)

Every cycle reports into the operator's dashboard. A cycle that ran but did not report is **incomplete**
(`decisions.md §20`).

### 8.1 What and where

| | |
|---|---|
| **File** | `$PF1E_JSON_PATH` (default `/home/todd/hermes-home/swarm-observer/PF1e-dashboard.json`) |
| **Manifest** | `manifests.sd27_book_pre_build` (`workchannel: "SD-27"`, `managed_by: "orchestrator"`) |
| **Items** | 6 — 2 books × 3 cycles (`pre_build`, `verify`, `parity`) |
| **Item id** | `sd27.<book>.<stage>` — e.g. `sd27.advanced_race_guide.pre_build` |
| **Writer** | `scripts/sd27-workflow.py`, which delegates to the orchestrator helper |

### 8.2 The only sanctioned writer

All mutation goes through `pf1e_dashboard_producer_orchestrator_helper.py`'s public API
(`add_manifest_item`, `claim_item`, `complete_item`, `list_pending_items`). It performs an atomic
tempfile + `fsync` + `os.replace` write under an `flock`, and `chmod`s the result to `0644` so nginx can
serve it.

**Never write the JSON directly.** A hand-edit bypasses that discipline *and* is silently discarded by
the producer's next tick. Do not call `write_json()` while holding `acquire_lock()` — on this kernel the
two are different file descriptions and the inner lock blocks forever; mutate in memory under the lock,
release, then write. `scripts/sd27-workflow.py` already honors all of this.

### 8.3 What survives a producer tick

`pf1e_dashboard_producer.py` regenerates the dashboard from the v0.6 markdown roughly every 5 minutes.
It **preserves** each manifest's `items` and `stats`, so cycle writes survive. It does **not** preserve a
manifest's `scope`, `workchannel`, or `managed_by` — those are reseeded from the producer's own
`_seed_manifests()` and cannot be changed from this bundle.

### 8.4 Per-cycle protocol

```bash
# Once, before E2.1 — hard-fails on any missing prerequisite
python3 scripts/sd27-workflow.py preflight

# Once — writes the 6 items (idempotent; refuses to clobber in-flight work)
python3 scripts/sd27-workflow.py seed

# Per cycle
python3 scripts/sd27-workflow.py next
python3 scripts/sd27-workflow.py claim sd27.<book>.<stage> --agent backend
#   ... run the cycle body, dual-audit gate, commit, write the receipt ...
python3 scripts/sd27-workflow.py complete sd27.<book>.<stage> --receipt <repo-relative path>

# On failure — the orchestrator does NOT auto-retry (§4); returns to the operator
python3 scripts/sd27-workflow.py block sd27.<book>.<stage> --reason "<what failed>"
```

`claim` enforces the `depends_on` chain, so a stage cannot start before its predecessor completes, and
`complete` refuses a receipt path that does not exist on disk — a green item always points at real proof.

### 8.5 Failure posture

The helper fails **silently** when the dashboard is unreachable: `read_json()` returns `None` and
`list_pending_items()` turns that into `[]`, which is indistinguishable from "no work to do." A
misconfigured run would therefore look like a clean no-op while writing nothing.

`scripts/sd27-workflow.py` hard-fails (exit 2) on an unreachable or corrupt dashboard rather than
proceeding. **Never work around that check** — an unreported cycle is worse than a halted one, because
the halt is visible.
