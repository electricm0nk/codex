---
title: SD-20 — Technical Requirements (Pre-Loop Prerequisites)
status: approved (operator review 2026-07-16; changes noted: §2 broadened to any class/any level, Q2 revised to class-selection trigger mechanic, Q3 revised to print-ready data; SD-20 launches on tranche/4 branch)
date: 2026-07-15
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-20-rules-engine-completeness/decisions.md
---

# SD-20 — Technical Requirements

SD-20 cannot begin until every prerequisite in this file is verified. Each prerequisite below is independently verifiable; the verification command is the contract.

## 1. Tranche-3 closure gates must all be green

SD-20 builds on the rules-engine substrate tranche-3 closes. Every tranche-3 closure gate must be green before SD-20 begins.

**Verification**:
```bash
# SD-18 chassis grounding (per-character chassis at level 1-20):
cd /home/ubuntu/workspace/repos/codex
cargo test --locked 2>&1 | tail -20
# Expect: 0 failed, all existing tests pass (no SD-18 regressions)

# SD-19 corpus-aware seam (spell-book reachability + bounded equipment baseline):
cargo test --locked --test sd19_seam_shapes_correctness 2>&1 | tail -10
# Expect: green
```

## 2. Canonical Paizo-table store must be populated with CRB cells

SD-19's foundation slice populates `src/rules_core/rules_tables/crb/` with structured data files (class tables, spell list, equipment tables). SD-20's epics (spellbook engine, feat prereqs, skill ranks, equipment effects, damage total, Level Up grants) all read from this directory.

**Verification**:
```bash
cd /home/ubuntu/workspace/repos/codex
ls src/rules_core/rules_tables/crb/ 2>&1 | head -20
# Expect: at minimum {class_tables,spell_list,equipment_tables}.rs (or .toml/.json — the canonical format pinned in SD-19 §9)

# Spell list has at least one entry per school:
cargo test --locked --test sd19_table_store_foundation 2>&1 | tail -10
# Expect: green
```

## 3. SD-18 §3.4/§3.5 dated blocker entries are not in scope

SD-20 is not SD-19's reachability work; SD-19 closes that. SD-20 starts from "spells reachable from corpus, equipment reachable from corpus with bounded baseline stats," and produces "spell effects, feat effects, skill totals, damage rolls, equipment stats, Level Up grants."

**Verification** (operator-driven; one-shot):
- SD-19's closure is recorded in `~/workspace/SD-18-core-rules-breadth-progress.md` (shared progress doc, SD-19 appends under its own section).
- The SD-19 closure gate from `acceptance-and-verification.md` is recorded as MET.

## 4. Engine-side boundary contract landing target must be agreed

SD-20 owns the boundary contract between engine and GUI. Before epic 1 (boundary contract + wire-fixture parity tests) lands, the contract shape must be sketched at minimum (the closure gate for epic 1 enumerates what goes in the contract; see `epic-breakdown.md`). This is a "have the conversation before the slice" prerequisite, not a "do a capability slice first" prerequisite.

**Verification** (operator-driven; one-shot):
- `docs/SD-20/boundary-contract.md` draft exists at the SD-20 bundle's `artifacts/` boundary-contract section, OR will be created as part of epic 1's slice work.

## Cross-reference

- `acceptance-and-verification.md` — closure gates including tabletop-readiness.
- `decisions.md` — the 9-item decision record (Tabletop-readiness posture, per-character scope, SD-21 promotion, etc.).
- `epic-breakdown.md` — 15 acceptance criteria grouped into 8 epics.
- `risks-and-open-questions.md` — self-healable vs. non-self-healable split + open override flags (boundary contract shape, parity JSON format, GUI outside bundle scope).
- `technical-design.md` — per-epic seam signatures, boundary contract shape, wire-fixture parity test format, per-epic authority surface.
- `./scope-draft.md` — canonical handoff.
- `./loop-instruction.md` — loop body.
- `~/workspace/programs/codex/requirements/SD-18-core-rules-breadth/` — chassis grounding.
- `../SD-19/` — corpus-aware seam + canonical Paizo-table store.
- `../SD-21/` (sibling bundle, parallelizable).
- `~/workspace/programs/codex/requirements/SD-17-pcgen-corpus-include-graph-resolution/` — corpus-side parsing work SD-20's spellbook and equipment epics consume.
- `~/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/` — matrix vocabulary SD-20 inherits for epic row updates.
