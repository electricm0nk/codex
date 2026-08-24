---
title: SD-33 — Computed-Value Verification — Release Package
status: planning
bundle_id: SD-33
slug: computed-value-verification
scope: docs/release/SD-33-computed-value-verification
artifact_type: release-index
canonical_branch: tranche/13
kanban_board: retired — local-file dispatch via ./kanban.md
target_version: 0.13.<build_at_launch>
canonical_source: docs/release/SD-33-computed-value-verification (this folder)
date: 2026-08-24
---

# SD-33 — Computed-Value Verification — Release Package

> ## ⚠️  OPERATING METHOD — REQUIRED FOR THIS BUNDLE  ⚠️
>
> **This bundle is operated via a `Workflow`-tool script, invoked from a live session — NOT `/loop /batch` and NOT a one-shot task.** For how to **create** that script — phase structure, tiering, worked skeleton — see `workflow-instruction.md §2.4`. The full per-cycle procedure, orchestration mode, concurrency map, dual-audit gate, retro-event-logging discipline, and epic/bundle closure steps live in `workflow-instruction.md`'s body, authored from `../../governance/workflow-instruction-template.md`. The scope-draft ([`./scope-draft.md`](./scope-draft.md)) is the canonical handoff *what*; the workflow-instruction is the *how*.

This folder is the canonical surface for SD-33. Everything the bundle needs is in this folder and the in-repo doc tree (sibling release folders, `../../doctrine-external/`, `../../architecture/`).

## 0. Preamble

The bundle's intent, scope, and acceptance-evidence obligations live in [`scope-draft.md`](./scope-draft.md). The per-cycle launch form, eligibility checks, and self-heal mechanics live in [`workflow-instruction.md`](./workflow-instruction.md). This README is the index.

## 1. Bundle snapshot

| Field | Value |
|---|---|
| Bundle ID | SD-33 |
| Slug | `computed-value-verification` |
| Canonical branch | `tranche/13` (**not yet cut** — see §9 launch gates) |
| Kanban board | retired — local-file `./kanban.md` per `SD-30-.../decisions.md` Decision 14a |
| Epics / criteria | 6 epics / 21 criteria |
| Target version | `0.13.<build_at_launch>` (**deferred placeholder** — resolved at first cycle per `decisions.md §3`) |
| Dispatch mechanism | `Workflow` tool, invoked from a live session, per `workflow-instruction.md §2` |
| Cadence | N/A — dispatch is a live `Workflow` session, not a timer loop |
| Closure gate | `tranche/13 → develop` PR; retrospective written + cited; worktree/branch sweep; release-notes generation; architecture-docs refresh (§6) — full sequence in `workflow-instruction.md §11` |

## 2. The one-sentence thesis

**Every gate this program has built so far checks presence. SD-33's question is correctness, and a wrong computed number looks exactly like a right one.**

SD-32 closed `no_record` to zero, drove `unclassified` to zero, and swept Product Identity clean corpus-wide. Each of those asks *does the thing exist?* — a cheap, binary question that **ingestion cannot silently lie about**. A record is present or it is not.

SD-33 asks *is the number right?* Nothing in a computed value betrays a wrong one. Answering it requires an **oracle** — something that already knows the answer — and this bundle's first job is to establish whether we can have one.

## 3. Files in this folder

| File | Job | Owner |
|---|---|---|
| `scope-draft.md` | Canonical handoff *what* — bundle intent, epics, criteria | operator |
| `workflow-instruction.md` | Per-cycle launch *how* — eligibility, self-heal, receipt schema | operator |
| `kanban.md` | Local-file card board (Hermes board retired 2026-08-01) | loop |
| `progress.md` | Live cycle-by-cycle progress + status matrix | loop |
| `decisions.md` | Bundle-specific ADRs | operator |
| `technical-requirements.md` | Pre-loop prerequisites + normative requirements | operator |
| `epic-breakdown.md` | Acceptance criteria grouped across epics | operator |
| `acceptance-and-verification.md` | Closure gates + verification commands + per-criterion artifact map | operator |
| `risks-and-open-questions.md` | Self-healable vs. non-self-healable split; open override flags | operator |
| `technical-design.md` | Architectural surface; oracle harness shape; THE-BOX schema | operator |
| `content-unit-inventory.md` | Per-content-unit N-tuple (module / fixture / cycle artifact / command) | operator |
| `forward-scope-register.md` | Successor work depending on this package's output | operator |
| `receipts.md` | Closure-pipeline receipts (architecture-truth-up, graphify) | loop |
| `release-notes.md` | Generated at closure | loop |
| `artifacts/` | Per-cycle evidence: fixtures, receipts, THE-BOX snapshots | loop |
| `artifacts/README.md` | Cycle-artifacts index | operator |
| `references/` | Doctrine pointers, skill pointers, sibling bundle pointers | operator |
| `references/README.md` | Doctrine / skill / sibling-bundle reference index | operator |

## 4. The population, re-derived

Every figure below was re-derived at `1d6ae1e72b` on 2026-08-24 against `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`. **Each carries the command that produces it** — per `workflow-instruction.md §12`, a figure without its command is not re-derived.

| # | Figure | Value | Re-derive |
|---|---|---:|---|
| A | inventory units | 49,438 | `jq '.units \| length' docs/work-inventory.json` |
| B | ledger population (not-done, non-excluded) | 34,397 | `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json` |
| C | outside the ledger entirely (A − B) | 15,041 | 15,022 `verdict==done` + 19 `EXCLUDED_BOOKS` |
| E | formula-bearing units in F1..F9 | 11,652 | ledger `matched` rows grouped by `family` |
| F | units the corpus-wide engine run actually ran | 4,798 | `artifacts/gate-2-engines/formula_interpreter.corpus-wide.json` `.total_population` (SD-32) |
| G | **never run through an engine (E − F)** | **6,854** | E minus F, per family |
| H | `status: unknown` → verdict `unmeasurable` | 4,224 | `jq '[.units[]\|select(.status=="unknown")]\|length' docs/work-inventory.json` |
| I | blessed by fixture or literal, never by oracle | 8,330 | 1,741 `fixture-verified` + 6,589 `literal-verified` |
| K | formula-bearing units that carry a magnitude | 625 | 529 `grounded` + 96 `ingested-magnitude`, joined to ledger `matched` |

**The denominator that matters:** SD-32's Gate 2 reports **97.9% recognised**. That is true of the **4,798 units it ran** — **41% of the 11,652 that exist**. Stating the first without the second is the defect `decisions.md §2` exists to prevent.

## 5. In-repo cross-references

Every reference is repo-relative.

- **Sibling release folders** — `../SD-32-compute-library-and-cause-closure/` (direct predecessor), `../SD-31-corpus-closure-grind/`.
- **Repo-local doctrine mirrors** — `../../doctrine-external/identifier-discipline.md`, `../../governance/no-stub-mvp-doctrine.md`, `../../governance/blocker-closure-doctrine.md`, `../../governance/deferral-revisit-doctrine.md`.
- **Architecture docs** — `../../architecture/`.
- **Repo-local conduct surface** — `../../AGENTS.md`, `../../CLAUDE.md`.
- **Retrospective of record** — `../../retro/sd32-compute-library-and-cause-closure-retrospective.md`, cited from `references/README.md`.

## 6. Relationship to other release folders

- **SD-32** — direct predecessor. SD-33 consumes SD-32's closed corpus (`no_record == 0`), its shape families F0–F10, and its engines. **SD-33 does not inherit SD-32's open items**: SD-32's own instrument debt closes inside SD-32, per `../../governance/blocker-closure-doctrine.md`. See §9.
- **Tranche branch posture** — this bundle ships on `tranche/13`, cut from `develop` after SD-32's PR merges.

## 7. Build version target

For SD-33 the release version triple is `0.13.<build_at_launch>`:

- **`major`** — `0` until first publish to `main`.
- **`tranche-base`** — `13`, the base digit of `tranche/13`. Bumps because SD-33 cuts a **new** tranche branch.
- **`build`** — monotonic counter across all builds across all branches.

`develop` is at `0.12.0` (`apps/desktop/package.json` + `apps/desktop/src-tauri/tauri.conf.json`, verified 2026-08-24). **`<build_at_launch>` is a deferred placeholder**, resolved in SD-33's first cycle once `tranche/13` exists — per `decisions.md §3` and `workflow-instruction.md §9`.

## 8. Architecture-docs, graphify, and PR closure obligation

Identical to `../template/template.md §6`, unchanged. It fires **once**, as the bundle's own final epic (Epic 6), never per-epic. The procedural half — retrospective and worktree sweep — is `workflow-instruction.md §11`.

**A filed blocker does not satisfy step 1** (`../../governance/blocker-closure-doctrine.md`). No closure criterion in this package is written as "complete *or* filed under `## Open blockers`".

## 9. Launch gates (must be true before Epic 1's first cycle)

These are **open** at authoring time and are tracked in `technical-requirements.md §1`:

1. **SD-32's closure PR is merged to `develop`.** As of 2026-08-24 `origin/develop`'s HEAD is `1bb523773d` (PR #374, `tranche/11`); `tranche/12` is unmerged. SD-33 is a Tier-1 dependent — it reads SD-32's corpus and engines.
2. **SD-32's own instrument debt is closed inside SD-32** — the `retro.py` `deferrals.open` defect, the 7 unverified deferrals, and the `EXCLUDED_BOOKS = ['beginner_box']` carve-out. All three are named in `../../retro/sd32-compute-library-and-cause-closure-retrospective.md`. **These are not SD-33 epics**; importing them would be exactly the laundering `blocker-closure-doctrine.md` forbids.
3. **`tranche/13` cut from `develop` and pushed.**

## 10. Cross-reference

- `../../governance/workflow-instruction-template.md` — the per-cycle dispatch procedure `workflow-instruction.md` is authored from.
- `../../governance/blocker-closure-doctrine.md` — gates §8 step 1.
- `../../governance/deferral-revisit-doctrine.md` — the sibling rule for a planned capability deferral.
- `../../retro/sd32-compute-library-and-cause-closure-retrospective.md` — the predecessor retrospective this bundle's `decisions.md §1–§4` are derived from.
- `.claude/skills/stc-authoring/SKILL.md` — the skill this package was authored with.
