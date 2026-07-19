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

## 5. Paizo content sources generated in-bundle (loop-generated per-cycle from PF1 OGL/SRD; operator directive 2026-07-18)

SD-22's Epic 3 + Epic 4 + Epic 5 cycles depend on structured-data input from the publisher's books (APG classes + ACG classes + Bestiary 1 monsters). Per `decisions.md §5` (operator directive 2026-07-18, superseding the prior operator-supplied posture): **the loop generates these itself, per-cycle, from PF1 OGL/SRD content**. Each ingest cycle's first step generates the cycle's corpus input file (`corpus/apg_alchemist.json`, `corpus/acg_arcanist.json`, `corpus/beastiary1_subset_0.json`, etc.) using the corresponding `corpus-source-inventory.md` row's *Content shape* column as the authoritative generation spec. No operator-supplied corpus is required at launch.

**Verification** (per-cycle; loop-driven):
- The cycle's generated `corpus/<book>_<unit>.json` exists on disk after the cycle's generation step, **and** the cycle's ingest test (the row's `test_fixture_path`) passes against the ingested data.
- Per-cycle representative-sample spot-check tests assert known-good values from the published rules (fabricated-data-risk mitigation per `decisions.md §5`).
- Only unresolvable source ambiguity (SRD conflict, missing SRD coverage for a unit) routes to Open Blockers.

## 6. DM-toolkit canonical Paizo example data generated in Epic 6's first cycle

SD-22's Epic 6 deterministic tests require canonical Paizo encounter-math examples. Per `decisions.md §5`'s same in-bundle generation posture, the canonical encounter-table (Easy / Medium / Hard / Deadly thresholds by party size + average level) is **generated in Epic 6's first cycle** from the published PF1 encounter-building rules, alongside the five deterministic test cases pinned in `corpus-source-inventory.md` §4.1.

**Verification** (informational; not gating):
- Epic 6's first cycle lands the canonical encounter-table data with its deterministic fixtures; the fixtures assert the canonical Paizo examples.
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
