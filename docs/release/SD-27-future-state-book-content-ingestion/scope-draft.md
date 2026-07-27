# SD-27 — Scope Draft

> **⚠️ OPERATING METHOD — REQUIRED FOR THIS BUNDLE ⚠️**
>
> **The dispatcher is the in-harness `Workflow` tool, driven from a live session.** **NOT** `/loop /batch`, and **not** a headless `scripts/workflow-dispatch.sh` process — that script does not exist in this repo on any ref, and its `claude code --profile … --task …` invocation does not exist in the live CLI. Per `decisions.md §19`, adopting SD-26's `decisions.md §13`. The deterministic half of dispatch lives in `scripts/sd27-workflow.py`. Per `docs/governance/loop-instruction-template.md §2` + skill `workflow-orchestrated-dispatch`.
>
> **Loader action:** read `forward-scope-register.md` first. This scope-draft tightens the register's class-1 commitments into a concrete bundle shape. The register is the disagreement surface; this scope-draft is the committed shape.

## 0. Preamble

SD-27 ships the resolution of **2 future-state book stubs** (Advanced Race Guide, Pathfinder Unchained) that SD-26 registered in Epic 4. **The 17 remaining future-state books** (Adventurer's Guide, Bestiary 2-6, Bonus Bestiary, Horror Adventures, Monster Codex, Mythic Adventures, Occult Adventures, the 6 Tier-2 Ultimate books) **are deferred to a follow-up SD (SD-28 or later)**, operator-gated on SD-27's first two books closing cleanly. Beginner Box and Core Essentials were removed from scope per operator directive 2026-07-27 (redundant to other tomes; will not be brought in). This is the "tune, then go wide" model: 2 books prove the per-book pre-build + verification + parity cycle pattern; the next bundle (operator-pinned) goes wide.

The bundle is content ingestion for an existing engine — it does not introduce new chassis, new rule mechanics, or new class engines. Every cycle in this bundle runs against the SD-26 Shape B schema (extended to Shape B v1 in cycle 2.0.5) and the SD-26 PCGen parity harness (`src/oracle_validation/`).

**The 4,435 SD-22 in-scope book cache files are the durable validation method.** They exist for CRB, APG, ACG, and Bestiary 1. The pre-build cycles in SD-27 emit shape B v1 records in the same shape; the verification cycles confirm the pre-built caches conform. The 4 in-scope books' v0 records are retro-fit to v1 in cycles 2.0.6-2.0.9.

## 1. SD-27 — 5 epics

### 1.1 Epic 1 — Code-Side Identifier Cleanup (governance base; fires FIRST)

Post-SD-26 cleanup. Same shape as SD-22/23/24/25/26's E1. Defensive scope, runner audits the existing identifier shape, no renames expected.

### 1.2 Epic 2 — Book Pre-Build + Verification (2 future-state books: ARG + PU)

The bundle's payload. The 2 books are pre-built into shape B v1 caches (E2.x = "pre-build" cycles), then verified against the SD-22 validation method (E2.x' = "verification" cycles). The per-tier scheduling:

- **Tier-1 (2 books, serial, operator-gated):** advanced_race_guide (E2.1), pathfinder_unchained (E2.2). Per-book pre-build is a cycle; per-book verification is a cycle; per-book parity is a cycle. The 3 cycles per book (pre-build + verify + parity) are 6 total cycles for E2.
- **Tier-2 (0 books in this bundle):** ultimate_campaign, ultimate_combat, ultimate_equipment, ultimate_intrigue, ultimate_magic, ultimate_wilderness. Deferred to SD-28+. Operator-pinned.

The 2 books in this bundle run sequentially (E2.1 → E2.2). The 2 books' pre-build cycles are file-disjoint (`data/corpus/advanced_race_guide/` vs. `data/corpus/pathfinder_unchained/`) and could run in parallel, but the operator's "tune, then go wide" model means the first book closes (or fails) before the second book dispatches. **Cycle failure on E2.1 (ARG) → operator reviews the per-cycle receipt → operator picks the next action (re-tune, defer, or proceed to E2.2).**

### 1.2.1 Per-book 4-stage concept (operator-confirmed 2026-07-25)

Each book goes through 4 stages in its lifecycle. The bundle's cycle count is 3 cycles per book (Stages 1+2 are combined into one cycle; Stage 3 is one cycle; Stage 4 is one cycle). Same 4 stages per book, different cycle mapping per book kind:

| Stage | In-scope book (CRB, APG, ACG, B1) | Future-state book (ARG, PU) |
|-------|-------------------------------------|------------------------------|
| **Stage 1: License prep** | Embedded in cycle 2.0.6-2.0.9 (Stages 1+2 combined) | Embedded in cycle 2.1 / 2.2 (Stages 1+2 combined) |
| **Stage 2: Pre-build** | Embedded in cycle 2.0.6-2.0.9 (re-emit v0 → v1; SD-22 already did the original LST extract) | Embedded in cycle 2.1 / 2.2 (fresh LST extract, emit v1) |
| **Stage 3: Verify** | Cycle 2.1' / 2.2' (verify v1 records against dual-audit gate) | Cycle 2.1' / 2.2' (verify v1 records against dual-audit gate) |
| **Stage 4: Parity** | Cycle 3.1 / 3.2 (PCGen parity baseline, Epic 3) | Cycle 3.1 / 3.2 (PCGen parity baseline, Epic 3) |

**Why combine Stages 1+2 in one cycle for in-scope books:** The 4 in-scope books already have v0 Shape B records from SD-22. The license retro-fit (Stage 1) and pre-build re-emit (Stage 2) are a single operation: read v0 records, apply the v1 schema, emit v1 records with the `license` field. Splitting them into separate cycles would require the orchestrator to read+write the 4,435 SD-22 cache files twice for no engineering benefit.

**Why combine Stages 1+2 in one cycle for future-state books:** The 2 future-state books have no pre-existing cache. The license prep (Stage 1, apply PI-blacklist) and pre-build (Stage 2, extract from LST and emit Shape B v1 records) are a single operation: read LST corpus, emit v1 records with the `license` field baked in. The pre-build IS the license prep because v1 is the only shape the pre-build knows.

**Why Stages 3 and 4 are separate cycles:** Verify (Stage 3) is a read-only audit of the pre-built cache. Parity (Stage 4) is a PCGen pipeline run that consumes the cache. They're file-disjoint (verify reads `data/corpus/<book>/`; parity reads the cache and writes `data/corpus/<book>/_parity/`). Different per-cycle tier (verify is Sonnet or free-discounted; parity is Sonnet for fixture authoring + free-discounted for the pipeline run).

**Per-book shape (mirrors SD-26 E4 + the OGL/PI license-stripping doctrine):**

- **Criterion 2.0** — Label resolution. One cycle. Resolves the `SD-27` vs. `SD-27+ (unscheduled)` discrepancy across all 20 surfaces (19 `data/stubs/*.json` + `decisions.md:102`). Per-cycle blocking decision; gates 2.0.5+.
- **Criterion 2.0.5** — Shape B v1 license-stripping pre-flight. One cycle. Schema bump to Shape B (add per-record `license: "OGL" | "PI" | "PI-REDACTED"` field, plus per-record `pi_field` and `pi_marker` for downstream filtering), per-book `LICENSE.json` schema, PI-blacklist (which LST fields are Product Identity per the OGL's PI section), and the redaction-to-marker policy. Per-cycle blocking decision; gates 2.0.6+. **Born of operator-pinned OGL-licensing review (2026-07-25)**: the 4 in-scope books were ingested under SD-22 with inlined OGL-licensed content; the 19 future-state books must follow the same Shape B. Both inlinable (OGL) and non-inlinable (PI) content is currently inlined. The 2.0.5 cycle lands the schema bump before any per-book cycle (2.0.6+, 2.1-2.2) consumes it. Retro-fit of the 4 in-scope books to the new shape is rolled into SD-27 as 2.0.6-2.0.9 (4 cycles, one per in-scope book).
- **Criterion 2.0.6 — 2.0.9** — 4 in-scope book **Stages 1+2: License retro-fit + pre-build re-emit**. Four cycles, one per in-scope book (CRB, APG, ACG, Bestiary 1). **Each cycle combines Stage 1 (license retro-fit: read v0 Shape B records, apply the v1 schema, redact PI values) and Stage 2 (pre-build re-emit: re-emit each record with the new `license` field, write per-book `LICENSE.json`, run the dual-audit gate).** For the 4 in-scope books, Stages 1 and 2 are combined because the v0 records are pre-existing (the "pre-build" is a re-emit, not a fresh LST extract — SD-22 already did the original LST extract). For the 2 future-state books (ARG, PU), Stages 1 and 2 are also combined but the cycle does a fresh LST extract, not a re-emit. **Per the operator's 2026-07-25 OGL review**, this is a real liability that must be closed before any of the 19 future-state books fan out — the per-book cycles (2.1-2.2) inherit the new shape, so the in-scope books must conform to the same shape first. Each cycle is file-disjoint (`data/corpus/<book>/` only); they can run in parallel.
- **Criterion 2.0.10** — All-23-books license-conformance verify. One cycle. Dual-audit gate against all 23 books (4 in-scope + 2 pre-built future-state + 17 deferred future-state stubs). For each book: every Shape B record has a `license` field, every PI-tagged record has the `[redacted PI]` marker, the per-book `LICENSE.json` exists and matches the records. Per-cycle terminal state — gates 2.1+ dispatch. **Expected output**: 4 in-scope books conform (post-retro-fit), 2 pre-built future-state books conform (their pre-build is in shape v1), 17 deferred future-state stubs in the untouched state (no records, no `LICENSE.json` yet — their pre-build is in a follow-up bundle).
- **Criterion 2.1** — Advanced Race Guide **Stages 1+2: License prep + pre-build**. One cycle. Per-book pre-build for `data/corpus/advanced_race_guide/`. Reads the LST corpus at `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/advanced_race_guide/`, extracts content kinds (classes, spells, equipment, feats, races, bestiary, etc.), emits shape B v1 records (with `license` field, PI redaction complete), writes per-book `LICENSE.json`, updates the stub manifest with real `content_kind_counts`. Operator-gated: cycle failure → operator picks the next action. Gates 2.1'.
- **Criterion 2.1'** — Advanced Race Guide **Stage 3: Verify**. One cycle. Reads the pre-built cache at `data/corpus/advanced_race_guide/`, runs the dual-audit gate, hands the operator a verification report. **Much shorter than a creation cycle** because the records already exist. Gates 3.1 (Epic 3's parity cycle).
- **Criterion 3.1** (Epic 3) — Advanced Race Guide **Stage 4: Parity baseline**. One cycle. Runs the SD-26 PCGen pipeline against a hand-authored `pf_advanced_race_guide_human_<class>_level1_golden.pcg` fixture. Comparator runs against the pre-built cache.
- **Criterion 2.2** — Pathfinder Unchained Stages 1+2: License prep + pre-build. Same shape as 2.1 but for `data/corpus/pathfinder_unchained/`. Operator-gated. Gates 2.2'.
- **Criterion 2.2'** — Pathfinder Unchained Stage 3: Verify. Same shape as 2.1'.
- **Criterion 3.2** (Epic 3) — Pathfinder Unchained Stage 4: Parity baseline. Same shape as 3.1.
  - Reads the source `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/<book>/` and inventories content kinds.
  - Writes the Shape B JSON cache at `data/corpus/<book>/{classes,spells,equipment,feats,...}/<id>.json` per the SD-26 schema.
  - Updates the `book_stub` registry entry's `content_kind_counts` field with real numbers and `status: "Resolved"` + `resolved_at: <ISO-8601>` + `resolved_by: claude-code` + `bundle_of_record: SD-27` + `cycle_receipt: artifacts/epic_2/<book>_pre_build-cycle_receipt.md`.
  - Updates `data/stubs/<book>.json`'s `content_kind_counts` field to mirror the registry.
  - Passes the dual-audit gate (identifier-discipline + wired-integration four-check).

The 2 cycles are file-disjoint (each writes to a different `data/corpus/<book>/` directory). Worktree isolation is unnecessary; the cycles serialize on the shared `docs/governance/wired-integration-stubs-registry.md` file (one cycle at a time).

### 1.3 Epic 3 — PCGen Parity Baseline (2 fixtures)

Two cycles, one per book (ARG + PU). Each cycle:

- Crafts a `pf_<book>_human_<class>_level1_golden` `.pcg` fixture mirroring the SD-26 pilot Fighter pattern.
- Runs the real PCGen Gradle headless pipeline against the fixture.
- Sanitizes the output and writes `data/corpus/<book>/_parity/<id>.json` as the parity baseline.
- Records the per-cycle parity comparison in `artifacts/epic_3/<id>_parity-cycle_receipt.md`.

The 2 cycles are file-disjoint. The pipeline is the existing SD-26 PCGen runner. The cycles inherit the current 7-of-9 baseline match rate (due to CG-03) until v0.6 closes that bug; the per-book assertion is "match rate at the time of cycle close."

### 1.4 Epic 4 — Closure Epilogue (fires LAST)

Standard closure:

- **Criterion 4.1** — Final criterion scan. Subagent: Sonnet.
- **Criterion 4.2** — Architecture closure pipeline (truth-up + graphify + PR + merge). Subagent: Opus (template §2's adversarial-verify).
- **Criterion 4.3** — Release notes. Subagent: Haiku.
- **Criterion 4.4** — Build version increment (`0.6.0` → `0.6.1` per the `major.tranche-base.build` scheme). Subagent: Haiku.
- **Criterion 4.5** — PR + merge. Subagent: Sonnet.

## 2. Bundle at a glance

- **Slug:** `SD-27-future-state-book-content-ingestion`
- **Predecessor:** SD-26 (POST-CLOSE-PROMOTE-DEPENDENCY: SD-27 cannot dispatch Epic 2.1+ until SD-26 `tranche/5-4 → develop` PR lands. The tier-1 launch-gate dependency is enforced by E2.1's verification cycle reading SD-26's tier-1 gate.)
- **Branch:** `tranche/7` (operator directive forthcoming; SD-26's promotion to develop completes the merge-base for SD-27).
- **Board:** `codex-tranche-7` (governed convention slug; operator override on file).
- **Epics:** 5 / **Cycle count: 19 cycles total.** Breakdown: E1.1 (1) + E2.0 label-resolution (1) + E2.0.5 schema-bump (1) + E2.0.6-2.0.9 four-in-scope-retro-fit (4, parallel-after-2.0.5) + E2.0.10 all-23-verify (1) + E2.1-2.2 two-pre-builds (2, serial-operator-gated) + E2.1'-2.2' two-verifications (2, serial-after-pre-build) + E3.1-3.2 two-parities (2, serial-after-verify) + E4.1-4.5 closure (5) = 19 cycles. **Pre-build is the new front-loaded epic; verify is much shorter than the original per-book creation cycle.**
- **First concrete build:** `0.6.0` (post-SD-26 promotion; bundle-of-record lands the next concrete build at `0.6.1`).
- **Dispatch:** `Workflow` orchestrator.
- **Concurrency:** E2.0-2.0.10 serial; E2.0.6-2.0.9 parallel-after-2.0.5 (file-disjoint, 4 in-scope books); E2.1-2.2 serial-after-2.0.10 (operator-gated promotion); E3.1-3.2 serial-after-E2.x; E4 serial. **Cross-bundle parallel with v0.6's class work** provided the file-touch partition holds.

## 3. Per-book cycle map

| # | Book | Tier | Bundle | Status | Notes |
|---|------|------|--------|--------|-------|
| 1 | advanced_race_guide | T1 | **SD-27** | pre-build + verify + parity | Per-book pre-build is a cycle (E2.1); verification is a cycle (E2.1'); parity is a cycle (E3.1, Epic 3). 3 cycles for ARG. |
| 2 | pathfinder_unchained | T1 | **SD-27** | pre-build + verify + parity | Same shape as ARG (pre-build E2.2, verify E2.2', parity E3.2). Was row 13 before the 2026-07-27 pairing; promoted into SD-27 to match the operator's `SD-27 (ARG + PU)` dashboard workchannel. |
| 3 | bestiary_2 | T1 | SD-28+ | deferred | |
| 4 | bestiary_3 | T1 | SD-28+ | deferred | |
| 5 | bestiary_4 | T1 | SD-28+ | deferred | |
| 6 | bestiary_5 | T1 | SD-28+ | deferred | |
| 7 | bestiary_6 | T1 | SD-28+ | deferred | |
| 8 | bonus_bestiary | T1 | SD-28+ | deferred | |
| 9 | horror_adventures | T1 | SD-28+ | deferred | |
| 10 | monster_codex | T1 | SD-28+ | deferred | |
| 11 | mythic_adventures | T1 | SD-28+ | deferred | |
| 12 | occult_adventures | T1 | SD-28+ | deferred | |
| 13 | adventurers_guide | T1 | SD-30 | deferred | Was row 2 (in-scope) before the 2026-07-27 pairing; the operator's dashboard routes it to the SD-30 workchannel, not SD-28. |
| 14 | ultimate_campaign | T2 | SD-28+ | deferred | Tier-2; mechanically dense; SD-22's precedent doesn't apply |
| 15 | ultimate_combat | T2 | SD-28+ | deferred | Tier-2 |
| 16 | ultimate_equipment | T2 | SD-28+ | deferred | Tier-2 |
| 17 | ultimate_intrigue | T2 | SD-28+ | deferred | Tier-2 |
| 18 | ultimate_magic | T2 | SD-28+ | deferred | Tier-2 |
| 19 | ultimate_wilderness | T2 | SD-28+ | deferred | Tier-2 |

**17 deferred future-state books** (rows 3-19) are operator-gated on SD-27 closing cleanly. The operator picks the next batch after E4.5 closes; the dashboard already routes them across the SD-28 (Ultimate), SD-29 (Bestiary) and SD-30 (Adventure+) workchannels. Tier-2 (Ultimate-line) books are operator-pinned for a separate bundle because their denser equipment/feat content is a different cycle-shape problem (more records per book, longer per-book cycles).

**Note (2026-07-27):** Beginner Box and Core Essentials were removed from the original 21-book plan. They are redundant to other tomes and will not be brought in. Their stubs at `data/stubs/beginner_box.json` and `data/stubs/core_essentials.json` **are on disk** (as are their registry entries `#0005` and `#0012`) — verified 2026-07-27. They are out-of-scope, not deleted; the closure epilogue may remove them if the operator authorizes a stub-cleanup pass. Any count assertion over `data/stubs/*.json` must therefore expect **21** files and exclude these two by name, not expect 19.

## 4. File-touch partition

SD-27 cycles may write to:

- `data/corpus/<book>/` for the 2 future-state books in this bundle (advanced_race_guide, pathfinder_unchained), **and** `data/corpus/{core_rulebook,advanced_players_guide,advanced_class_guide,beastiary}/` for the 4 in-scope books **during the 2.0.6-2.0.9 license retro-fit cycles only**. Outside those four cycles the in-scope books' corpus is v0.6's lane and must not be touched. (`decisions.md §8` is the authority if these ever disagree.)
- `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/decisions.md` — cycle 2.0 only, to propagate the resolved bundle label at line 102.
- `docs/governance/wired-integration-stubs-registry.md` (serial, one cycle at a time).
- `data/stubs/<book>.json` (serial, one cycle at a time; only ARG + PU stubs are updated to `content_kind_counts: <real>` during SD-27; the 17 deferred books' stubs stay at `null`).
- `src/bin/sd27_gen_book_cache.rs` (new) — the pre-build code-generation tool, parameterized on book id. If Criterion 2.0 generalizes the script, the existing `src/bin/sd26_gen_core_rulebook_cache.rs` is replaced.
- `src/rules_core/shape_b_v1.rs` (new) — the Shape B v1 schema authority, the license-aware extension of SD-26's Shape B. The 4 in-scope retro-fit cycles and the 2 pre-build cycles both consume this.
- `docs/governance/ogl-pi-blacklist.md` (new) — the PI-blacklist that determines which fields are OGL-inlinable vs. PI-redacted.
- `tests/sd27_*` (new test files; mirror of SD-26's `tests/sd26_*`).
- `docs/release/SD-27-future-state-book-content-ingestion/` (the bundle's own docs).

SD-27 cycles **must not** touch:

- `src/rules_core/pilot_compute.rs` — v0.6's lane.
- `src/rules_core/rules_tables/<book>/` for any in-scope book (CRB, APG, ACG, Bestiary 1) — v0.6 may be modifying these.
- `docs/release/v0.6/` — v0.6's lane. **One exception:** cycle 2.0 (label resolution) writes `docs/release/v0.6/risks-and-open-questions.md`'s Open-questions item 2 to record the resolution. That single write is authorized; no other cycle may touch this tree.
- `src/oracle_validation/` — SD-26's lemma from E2.1; cycles consume, do not modify.

A 4-grep dual-audit (`identifier-discipline` + `wired-integration`) is the load-bearing enforcement. Cycles that breach the partition fail the audit and the cycle returns to the operator with a concrete file-list of the breach.

## 5. Why scope is bounded

- 2 books × 3 cycles each (pre-build + verify + parity) = 6 per-book cycles. Templated; each cycle is the same shape with book-specific content.
- 2 fixtures × 1 cycle each = 2 cycles. Templated; the pipeline is identical.
- The bundle's natural bottleneck is the per-book PCGen fixture authoring. SD-26's pilot Fighter fixture is the worked example; the 2 fixtures follow the same `pf_<book>_human_<class>_level1` shape.
- Both books have a real source LST corpus at `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/<book>/`. The dispatcher verifies the directory exists and inventories content kinds, no fabrication.
- No new rules, no new chassis, no new class engines. The bundle is content-only.

## 6. Out-of-scope (carve-out list with citations)

Per `forward-scope-register.md` §"Class excluding":

- Animal companions / familiars — `docs/release/v0.6/risks-and-open-questions.md:107`.
- Parameterized feats (Skill Focus, Teamwork feats) — `docs/release/v0.6/risks-and-open-questions.md:209-230`.
- Temporary HP / favored-class-bonus HP — `docs/release/v0.6/risks-and-open-questions.md:99-100`.
- Multiclass durability-level ordering — `docs/release/v0.6/SWARM_REPORT.md:485-505`.
- Multiclass / class-chassis breadth — v0.6's lane.
- Equipment-attachment schema — v0.6's lane.
- Feat-effects engine beyond 4-feat scope — v0.6's lane.
- Class-skill recognition beyond Fighter/Wizard/Rogue — v0.6's lane.
- Starting wealth for non-CRB classes — v0.6's lane.
- Arcane-school selector UI — v0.6 backlog.
- Unequip / remove-equipment affordance — v0.6 backlog.

## 7. Verification

The bundle's verification stack:

- Per-cycle: RED → GREEN → REFACTOR walkthrough, dual-audit gate (identifier-discipline + wired-integration four-check).
- Per-book: Shape B JSON cache key-set + key-order check against an SD-26 in-scope book (proves the schema is stable).
- Per-book: PCGen parity baseline assert against the SD-26 comparator's normalization library.
- Per-book: `data/stubs/<book>.json`'s `content_kind_counts` matches the registry entry's `content_kind_counts`.
- Per-book: `book_stub` registry entry's `Status` reads "Resolved" with `resolved_at` and `bundle_of_record: SD-27`.
- Per-cycle: the cycle's reporting-JSON item reads `complete` with its `output_path` pointing at the real receipt (`loop-instruction.md §8`).
- Bundle-level: `cargo test --workspace --locked` clean, `tests/sd27_*` catalog complete.
- Bundle-level: `bash scripts/wired-integration-audit.sh` four-check audit clean against the bundle's combined diff.
- Bundle-level: `bash scripts/architecture-truth-up.sh` clean against the bundle's combined diff.
- Bundle-level: `python3 scripts/sd27-workflow.py status` shows 6/6 items `complete`.

## 8. Hard-stop conditions

The bundle stops and reports the blocker instead of guessing when:

- The bundle label discrepancy (`SD-27` vs. `SD-27+ (unscheduled)`) is not resolved by the operator by the end of cycle 2.0.
- The SD-26 closure PR has not landed on develop by the time SD-27 launches Epic 2.1 — the tier-1 launch-gate dependency is a hard stop.
- A per-book cycle fails the dual-audit gate twice in a row — the 19 future-state books share a schema, dual-audit failure here is a schema-level defect, not a per-book issue.
- A per-book cycle's PCGen parity baseline produces a >2-dimension mismatch against the comparator's normalization library — sd-26's 7-of-9 baseline is the worst-case ceiling; >2 mismatches is a corpus-content quality issue, not a per-book cycle issue.
- v0.6's class-skill / equipment-attachment / feat-effects work causes a discrete change to `data/corpus/<book>/` for any of the 4 in-scope books that conflicts with SD-27's per-book ingestion. The bundle's partition is the binding; v0.6's overlap is the trigger.
- The CG-03 (Human ability-modifier bug) baseline shift causes a per-book parity cycle to fail with a "9-of-9 expected" assertion that the operator never signed off on. The 7-of-9 baseline is the cycle's assertion; the bundle's full-completion criterion is match-rate-at-cycle-close, not 9-of-9.
