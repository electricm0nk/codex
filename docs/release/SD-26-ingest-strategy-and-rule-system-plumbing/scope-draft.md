# SD-26 — Ingest Strategy Revision + Rule-System Plumbing

> **⚠️ OPERATING METHOD — REQUIRED FOR THIS BUNDLE ⚠️**
>
> `Workflow` orchestrator at `scripts/workflow-dispatch.sh`. **NOT** `/loop /batch`. Per `/governance/loop-instruction-template.md §2` + skill `workflow-orchestrated-dispatch`.

## 0. Preamble

SD-26 ships the doctrine + tooling that makes every future ingest bundle tractable. Four loads in one bundle:

1. **Oracle-harness comparator** (Epic 2) — the missing piece of `src/oracle_validation/`; consumes SD-25's PCGen runner scaffolding + the SD-26 JSON cache to produce parity-checked output.
2. **JSON cache build, 4 in-scope books** (Epic 3) — durable artifacts for core_rulebook + advanced_players_guide + advanced_class_guide + bestiary 1 at `data/corpus/`.
3. **Book stub manifest, 21 future-state books** (Epic 4) — operator-granted stubs for the remaining 21 PF1 books; entries in `governance/wired-integration-stubs-registry.md` (new `book_stub` kind).
4. **Doctrine-cost reduction** (Epic 5) — audit + cut over-spent per-class gates (per Diagnosis A from earlier conversation: RED-then-GREEN-then-re-audit per class is doctrine-cost, not work-cost). Per the operator's "20-min per class" pushback.

Plus E1 + E6 (canonical governance + closure).

## 1. SD-26 — 6 epics

### 1.1 Epic 1 — Code-Side Identifier Cleanup (governance base; fires FIRST)

Post-SD-25 cleanup, scope is defensive. Same shape as SD-25's E1.

### 1.2 Epic 2 — Oracle-Harness Comparator (the missing piece)

Five criteria + sequential:

- **Criterion 2.1** — `src/oracle_validation/comparator.rs` — implements `compare(canon_pcg: &NormalizedOutput, codex: &SelectedDimensions) -> ComparisonResult`; compares each SelectedDimension against the PCGen-generated value; produces `ComparisonResult { matches, mismatches: Vec<DimensionMismatch>, normalization_refs_used, claim_target: ClaimTier }`.
- **Criterion 2.2** — `src/oracle_validation/normalization.rs` — small rule engine for system-level format differences (PCGen's `0` vs Codex's `0` is a match; PCGen's trailing whitespace doesn't matter; spell-casing differences resolve via the `RuleSetId::Pf1` lookup).
- **Criterion 2.3** — `src/oracle_validation/parity_report.rs` — generates `artifacts/oracle_validation/parity_report_<case-id>.md` per case: Summary, Per-Dimension Comparison, Normalization Rules Used, Discovered Deltas.
- **Criterion 2.4** — `src/oracle_validation/pcgen_runner.rs` — Rust-side wrapper around SD-25's `scripts/pcgen-run-character.sh` + `scripts/pcgen-normalize-output.py`. Each test binary calls the wrapper rather than shelling out directly.
- **Criterion 2.5** — Verification cycle that runs the comparator against the pilot case at `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt` and asserts `current_claim_status` upgrades from `not_yet_grounded` to `oracle_checked`. The fixture file's frontmatter records the delta.

### 1.3 Epic 3 — JSON Cache Build (4 in-scope books, parallel per-book)

Four criteria + parallel (`isolation: 'worktree'`):

- **Criterion 3.1** — `data/corpus/core_rulebook/` — per-class / per-spell / per-equipment JSON files with progressive-completeness flags. Source: `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/*.lst`.
- **Criterion 3.2** — `data/corpus/advanced_players_guide/` — same shape, APG content.
- **Criterion 3.3** — `data/corpus/advanced_class_guide/` — same shape, ACG content.
- **Criterion 3.4** — `data/corpus/beastiary/` — same shape, Bestiary 1 content.

For each book the per-class / per-spell / per-equipment JSON files follow the **Shape B schema** (per the prior conversation): `{book}/{content_kind}/{content_id}.json` with `population: "in_scope" | "future_state" | "rule_system_stub"`, `completeness: "chassis_only" | "chassis_plus_extract" | "full"`. Each file carries a SHA-256 frontmatter tied to the source LST record.

### 1.4 Epic 4 — Book Stub Manifest (21 future-state books)

Two criteria + research epic + parallel fan-out:

- **Criterion 4.1** — Research epic: defines the `book_stub` kind in `governance/wired-integration-stubs-registry.md` (operator-pinned metadata fields: `{book_id, book_name, status: "stubbed", planned_resolution_bundle, registered_by, registered_at}`). Adds a new entry-shape template; validates against an existing stub.
- **Criterion 4.2..4.22** — 21 cycles, one per future-state book: each cycle (a) writes `data/stubs/<book>.json` with `{book_id, book_name, planned_resolution_bundle, content_kind_counts: null};` (b) registers a `book_stub` entry in `governance/wired-integration-stubs-registry.md`. Books: advanced_race_guide, adventurers_guide, beginner_box, bestiary_2..6, bonus_bestiary, core_essentials, horror_adventures, monster_codex, mythic_adventures, occult_adventures, pathfinder_unchained, ultimate_campaign, ultimate_combat, ultimate_equipment, ultimate_intrigue, ultimate_magic, ultimate_wilderness. Concurrency: `parallel: yes` after E4.1; each cycle touches a different file.

### 1.5 Epic 5 — Doctrine-Cost Reduction

One criterion:

- **Criterion 5.1** — Audit + cut per-class gate-cost from ~40 minutes (per the SD-22 Alchemist cycle receipt's `duration: ~40 minutes`) to ~6 minutes. Specifically:
  - Drop per-class doc-comment with source citation (replace with the JSON cache's SHA-256 frontmatter as the durable audit trail).
  - Drop the 135-line cycle-artifact write (replace with the per-cycle receipt's RED → GREEN + dual-audit evidence + duration_seconds).
  - Cut `progress.md` updates to one row per cycle (drop the 4-section fan-out).
- The audit produces `artifacts/epic_5/per-class-cycle-floor-measurement.md` with measured pre-cut vs post-cut floor per class. Operates on the SD-25 PCGen runner + SD-26 E2 comparator to measure, not assume.
- **Operator-pinned rule:** RED + GREEN + dual-audit is *load-bearing*; the cuts above do not affect those three. Cycle floor is what we cut.

### 1.6 Epic 6 — Closure Epilogue (fires LAST)

Five criteria + per-criterion tiering:

- **Criterion 6.1** — Final criterion scan. Subagent: Sonnet.
- **Criterion 6.2** — Architecture closure pipeline (truth-up + graphify + PR + merge). Subagent: Opus (template §2's adversarial-verify).
- **Criterion 6.3** — Release notes. Subagent: Haiku.
- **Criterion 6.4** — Build version increment (`0.5.99`). Subagent: Haiku.
- **Criterion 6.5** — PR + merge. Subagent: Sonnet.

## 2. Bundle at a glance

- **Slug:** `SD-26-ingest-strategy-and-rule-system-plumbing`
- **Branch:** `tranche/5-4` (operator directive 2026-07-21)
- **Board:** `codex-tranche-5` (reused)
- **Epics:** 6 / **Criteria:** 17 declarative + 21 dynamic (38 total; 5 declarative per Epic 2, 4 per Epic 3, 21 dynamic per Epic 4, 1 per Epic 5, 5 per Epic 6, 1 per Epic 1)
- **First concrete build:** `0.5.99`
- **Dispatch:** `Workflow` orchestrator
- **Publish mode:** move-not-copy

## 3. Cross-references

- `/governance/loop-instruction-template.md` — canonical template.
- `governance/no-stub-mvp-doctrine.md` + skill `wired-integration-discipline`.
- `governance/identifier-discipline.md` + skill `identifier-discipline`.
- `governance/wired-integration-stubs-registry.md` — operator-granted stub exceptions (E4's 21 entries land here).
- `~/.hermes/profiles/god-emporer/skills/orchestration/workflow-orchestrated-dispatch/` — orchestrator procedure.
- `../docs/release/SD-25-ui-evaluation-defect-closure/` — Tier-1 launch-gate dependency (closure PR → develop).
- `../docs/release/SD-24-beta-readiness-and-multiclass/` — closed predecessor.
- `src/oracle_validation/{golden_fixture,selected_parity_dimensions}.rs` — Oracle-harness schema (E2 reads + extends).
- `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/` — 26 PF1 book directories.

## 4. Hard-stop conditions

- Working tree diverged from `tranche/5-4` needs manual rebase.
- `## DISCOVERED` queue > 10 entries (operator override required).
- Tier-1 launch gate unsatisfied (SD-25 closure PR not in develop).
- Two live orchestrators on the same `tranche/5-4`.
- RED → GREEN transition not preserved in cycle receipt.
- Cycle finds `success: true` from a fake operation; inline mock in shipping module; "Would …" string in shipping code.
