---
canonical: true
owner: god-emporer
status: approved (operator review 2026-07-15; operator directives 2026-07-17 expanded scope to APG + ACG; operator clarification 2026-07-18: "ACG, APG are the two advanced guides"; branch + board pinned 2026-07-18 to tranche/5 / codex-tranche-5; bundle marked planning-ready)
date: 2026-07-15
canonical_branch: tranche/5 (operator directive 2026-07-18)
kanban_board: codex-tranche-5 (operator directive 2026-07-18)
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/decisions.md
---

# SD-22 — Technical Requirements (Pre-Loop Prerequisites)

SD-22 cannot begin until every prerequisite in this file is verified. Each prerequisite is independently verifiable; the verification command is the contract.

## 1. Tranche-3 + Tranche-4 baseline green

SD-22's content-source ingest pattern inherits from SD-19's source-book subdirectory convention; SD-19 must be closed (Tranche-3 chassis substrate + corpus-aware seam + canonical Paizo-table store done). SD-20 must be closed (per-character rules-engine). SD-21 must be closed (`tranche/4-1 → develop` promotion PR merged).

**Verification**:
```bash
cd /home/ubuntu/workspace/repos/codex
git log --oneline develop -20 | head -20
# Expect: SD-19 + SD-20 + SD-21 commits present; develop HEAD ahead of any in-flight branches

# Confirm SD-21 closed (the SD-21 closure PR is on develop):
ls docs/SD-21/release-closure-checklist.md 2>&1
# Expect: file exists
```

## 2. SD-19's source-book subdirectory pattern documented

Per SD-19's decision §9, future rule books populate sibling directories under `src/rules_core/rules_tables/<book>/`. SD-22's per-book ingestion follows this pattern. CRB lives in `rules_tables/crb/`; SD-22 ships `apg/`, `acg/`, `beastiary1/`.

**Verification**:
```bash
ls src/rules_core/rules_tables/
# Expect: crb/ exists (populated by SD-19);
#         apg/, acg/, beastiary1/ are SD-22's surfaces (created by Epic 3/4/5 cycles)

# Confirm the SD-19 foundation slice's class tables have a structure SD-22 can mirror:
head -30 src/rules_core/rules_tables/crb/class_tables.rs
# Expect: structured data of the same shape the SD-22 per-book ingestion will use
```

## 3. `codex-tranche-5` kanban board pinned (per operator directive 2026-07-18)

SD-22's launch branch is `tranche/5` and its kanban board is `codex-tranche-5`. The board is reused from the dead-state 2026-07-16 SD-21 launch that was later repurposed to `tranche/4-1` / `codex-tranche-4-1`.

**Verification**:
```bash
hermes kanban boards list | grep codex-tranche-5
# Expect: codex-tranche-5 row in the boards list (it already exists)

# Confirm the board's slug and display name:
hermes kanban boards list | grep -A1 codex-tranche-5
# Expect: slug = codex-tranche-5; display name mirrors SD-21's prior "(SD-22)" naming
```

## 4. `tranche/5` branch pushed to origin (operator action before first launch)

The `tranche/5` branch is fresh (not inherited from SD-21's `tranche/4-1`). Operator creates it once (`git push origin tranche/5` from the operator's side, after `develop` has SD-21's merge but before SD-22's first launch). The loop's `git fetch origin tranche/5` then resolves cleanly.

**Verification**:
```bash
cd /home/ubuntu/workspace/repos/codex
git fetch origin tranche/5
# Expect: no "couldn't find remote ref" warning

# Confirm the branch exists and is fresh:
git log --oneline origin/tranche/5 -5
# Expect: SD-21 closure PR merge commit visible (or close to it); no SD-22 commits yet
```

## 5. Paizo content sources: real PCGen LST data via `src/pcgen_import/` (corrected 2026-07-19)

SD-22's Epic 3 + Epic 4 + Epic 5 cycles depend on structured-data input from the publisher's books (APG classes + ACG classes + Bestiary 1 monsters). An earlier version of this section (operator directive 2026-07-18) called for generating this content per-cycle from the model's own OGL/SRD memory. That framing is **superseded** (`decisions.md §5`, corrected 2026-07-19): a real cloud cycle attempting the Alchemist ingest correctly refused to fabricate class content (two SRD mirrors 403'd; no in-repo source; `AGENTS.md`'s no-fabrication rule and the `rules_tables/crb/class_tables.rs` precedent both rule it out), which surfaced that the "no corpus source exists" premise was wrong.

The real source is **PCGen's published `.lst` data**, ingested by the existing, already-tested engine at `src/pcgen_import/` — the same pipeline SD-19 used to populate the CRB. No new parser code and no per-cycle content generation from memory; a cycle parses the real record and transcribes it, citing the source file as provenance (mirroring `rules_tables/crb/class_tables.rs`'s own doc-comment convention).

**Verification**:
```bash
# Local sibling repo (already present on this machine):
ls /home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst
ls /home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_classes.lst
ls /home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst
# Expect: all three exist and are non-empty (confirmed 2026-07-19).

# In a cloud/remote sandbox that only clones `codex`, clone the public upstream too
# (confirmed as the local sibling repo's own `origin` remote):
#   https://github.com/PCGen/pcgen
# and use the identical data/pathfinder/paizo/roleplaying_game/<book>/ path inside it.
```
- `corpus-source-inventory.md`'s `rust_module_path`/`test_fixture_path`/`cycle_artifact_path`/`RuleSetId` routing columns remain valid; its "Content shape" prose columns do not (see that file's corrective banner) and must be re-derived from the real `.lst` record per cycle.
- Only a genuinely unreachable LST tree (neither the local sibling repo nor a cloned public mirror resolves) routes a cycle to Open Blockers.

## 6. DM-toolkit canonical Paizo example data

SD-22's Epic 6 deterministic tests require canonical Paizo encounter-math examples (Easy / Medium / Hard / Deadly thresholds by party size + average level). These are published PF1 core-rules math (a formula/table, not book-specific narrative content), so they carry a materially lower fabrication-risk than named class features — but per the same corrected posture, Epic 6's first cycle should still verify its thresholds against a checkable source (the CRB's own encounter-building rules, already partially reflected in `rules_tables/crb/`) rather than asserting values from memory alone.

**Verification** (informational; not gating):
- Epic 6's first cycle lands the canonical encounter-table data with its deterministic fixtures, with the source of each threshold cited in the module's doc comment.
- Suspicious or ambiguous threshold values route to the Epic 9 judgment log, not to silent acceptance.

## Cross-reference

- `acceptance-and-verification.md` — 16 closure gates including SD-22's gates 1-9 (Tranche baseline, APG/ACG/Bestiary 1 ingest, cross-book resolution, DM Toolkit, MD interop, promotion PR).
- `decisions.md` — the 5-item decision record (§1 scope, §2 tranche/5 + codex-tranche-5, §3 deferred shape decisions, §4 Epic 9 — Closure Readiness, §5 corpus generation in-bundle + `/batch` deferred).
- `epic-breakdown.md` — 30 acceptance criteria grouped into 8 epics.
- `risks-and-open-questions.md` — self-healable vs. non-self-healable split + open override flags (Flag A through Flag D; Open Q1 through Open Q5).
- `technical-design.md` — content-source ingest patterns + DM-toolkit architecture.
- `~/workspace/SD-22-content-source-ingest-and-dm-toolkit-scope-draft.md` — canonical handoff; carries the prominent-early `/loop /goal` OPERATING METHOD callout (`/batch` deferred per `decisions.md §5`).
- `~/workspace/SD-22-content-source-ingest-and-dm-toolkit-loop-instruction.md` — loop body.
- `~/workspace/programs/codex/requirements/SD-18-core-rules-breadth/` — chassis grounding.
- `~/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/` — corpus-aware seam + canonical Paizo-table store (CRB); SD-22's per-book ingestion pattern inherits from this.
- `~/workspace/programs/codex/requirements/SD-20-rules-engine-completeness/` — sibling bundle, parallel; per-character rules-engine.
- `~/workspace/programs/codex/requirements/SD-21-campaign-manager-and-persistence/` — sibling bundle; SD-21's Epic 2 (Campaign Manager + Drive) consumes the party-CR math that SD-22 will provide.
