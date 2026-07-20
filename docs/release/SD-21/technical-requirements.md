---
title: SD-21 — Technical Requirements (Pre-Loop Prerequisites)
status: approved (operator review 2026-07-16; operator directives 2026-07-17: branch flip tranche/5 → tranche/4-1, board flip codex-tranche-5 → codex-tranche-4-1, APG+ACG+advanced guides moved to SD-22, Identifier Cleanup renumbered as Epic 1, 7-epic / 30-criteria final shape; Q1–Q5 PINNED, override flags A–D defaulted; bundle marked approved with operator directives 2026-07-16/17)
date: 2026-07-15
canonical_branch: tranche/4-1
kanban_board: codex-tranche-4-1
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-21-campaign-manager-and-persistence/decisions.md
---

# SD-21 — Technical Requirements

SD-21 cannot begin until every prerequisite in this file is verified. Each prerequisite is independently verifiable; the verification command is the contract.

## 1. Tranche-3 closure gates must all be green

SD-21 reads from SD-19's table store for spell names, class names, equipment names, race names — everything the campaign manager cares about. Tranche-3's three bundles (SD-18 chassis, SD-19 corpus-aware seam + canonical Paizo-table store, SD-19 §3.4/§3.5 acceptance criteria) must all be closed.

**Verification**:
```bash
cd /home/ubuntu/workspace/repos/codex
cargo test --locked 2>&1 | tail -20
# Expect: 0 failed, all existing tests pass

# Confirm SD-19 closed (grounded table store):
ls src/rules_core/rules_tables/crb/ 2>&1 | head -10
# Expect: at minimum the foundation slice's structured-data files
```

## 2. SD-19's source-book subdirectory pattern documented

Per SD-19's decision §9, future rule books populate sibling directories under `src/rules_core/rules_tables/<book>/`. SD-22 owns the content-source ingest lane (APG goes in `apg/`, ACG goes in `acg/`, advanced guides + Bestiary 1 follow the same sibling-directory pattern). SD-21's Epic 2 (campaign manager) reads from SD-19's `rules_tables/crb/` only.

**Verification**:
```bash
ls src/rules_core/rules_tables/
# Expect: crb/ exists (populated by SD-19's foundation slice);
#         apg/, acg/, ultimate_combat/, ultimate_magic/, bestiary1/ are SD-22's surfaces
#         (out of scope for SD-21; the loop's Epic 2 never creates or modifies them)

# Confirm the SD-19 foundation slice's class tables have a structure SD-22's
# APG/ACG ingestion cycles can mirror:
head -30 src/rules_core/rules_tables/crb/class_tables.rs
# Expect: structured data of the same shape the SD-22 APG/ACG ingestion will use
```

## 3. The Tauri `character_hub` persistence boundary exists

SD-21's Drive adapter sits above the existing Tauri Rust `character_hub` persistence boundary. Without that boundary, SD-21's Drive adapter has nothing to wrap.

**Verification**:
```bash
ls apps/desktop/src-tauri/src/character_hub.rs
# Expect: file exists

grep -l "save_character\|load_character" apps/desktop/src-tauri/src/character_hub.rs
# Expect: at least one persistence function exists to be wrapped
```

## 4. Drive OAuth client ID and secret (operator-supplied)

The Drive adapter uses Google OAuth to obtain a per-user token. The redirect URI and client secret come from the operator's Google Cloud Console project for codex.

**Verification** (operator-driven; one-shot):
- Operator provides: `GOOGLE_OAUTH_CLIENT_ID`, `GOOGLE_OAUTH_CLIENT_SECRET`, `GOOGLE_OAUTH_REDIRECT_URI` (or a placeholder URL the operator fills in).
- These are stored in `~/.hermes/profiles/god-emporer/.env` (per the credentials pattern from the existing `classic-token credential location` rule in persistent memory).
- Without these, SD-21's Epic 2 capability slice cannot ship and the loop exits with a named blocker.

## 5. (Optional) PR #316 merged before Epic 2's loop cycles

SD-21's Epic 2's loop cycles consume the `CampaignSnapshot` and markdown file-format contracts from the artifacts docs. The GUI work on `ui-work2` is the operator's parallel track (per their established vibe-coded workflow). Epic 2's loop does not depend on PR #316's merge for its engine-side work; the engine-side runs without the GUI.

**Verification** (informational; not gating):
- The `ui-work2` branch may or may not be merged by the time SD-21's Epic 2 cycles run. The engine-side is independent of the GUI-side's merge status.

## Cross-reference

- `acceptance-and-verification.md` — 13 closure gates including campaign-manager integration.
- `decisions.md` — the 21-item decision record.
- `epic-breakdown.md` — 30 acceptance criteria grouped into 7 epics.
- `risks-and-open-questions.md` — self-healable vs. non-self-healable split + open override flags.
- `technical-design.md` — campaign-shape boundary contract shape, Drive adapter boundary contract, markdown interop format.
- `./scope-draft.md` — canonical handoff.
- `./loop-instruction.md` — loop body.
- `../SD-18/` — chassis grounding.
- `../SD-19/` — corpus-aware seam + canonical Paizo-table store.
- `../SD-20/` — sibling bundle, parallel.
- `../SD-22/` — sibling bundle; owns APG + ACG + advanced guides + Bestiary 1 content-source ingest (per operator directive 2026-07-17; SD-21's Epic 2 reads `rules_tables/crb/` only).
