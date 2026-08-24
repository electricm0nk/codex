---
canonical: true
owner: god-emporer
status: planning-ready (chassis completed 2026-08-22)
date: 2026-08-22
---

# SD-32 References — Index

Doctrine pointers, skill pointers, and sibling-bundle pointers this package's cycles read.
Every entry resolves to a real file (relative to this folder) or to a repo-relative path; no
`~/workspace/...` or `programs/codex/requirements/...` paths — those live at
initial-package-construction time only.

## Doctrine (repo-local)

| Path | What it carries | When a SD-32 cycle reads it |
|---|---|---|
| `../../../governance/no-stub-mvp-doctrine.md` | The doctrine that no stubs / mocks / "would" strings / empty onClick handlers ship in production code | Every cycle (dual-audit gate, `workflow-instruction.md §6`) |
| `../../../governance/wired-integration-stubs-registry.md` | Operator-granted stub exceptions | Any cycle that needs to defer a wired-integration audit violation |
| `../../../governance/workflow-instruction-template.md` | The dispatch procedure this bundle's `workflow-instruction.md` is authored from | First cycle of every bundle, for the dispatch mechanism |
| `../../../governance/ogl-pi-blacklist.md` | The Product Identity blacklist terms | Not consumed by SD-32 directly; consumed by `pi_screening::PI_BLACKLIST_TERMS` (SD-30 Epic 3 surface, read-only) |
| `../../../governance/book-ingestion-playbook.md` | Per-book cycle procedure | Not consumed by SD-32 (SD-32 is not a book-ingestion bundle); useful for Epic 4's per-book cycles if the procedure is needed |
| `../../../governance/deferral-revisit-doctrine.md` | When a deferred item comes back | Any cycle that proposes a `## Open blockers` entry or a `forward-scope-register.md` item |
| `../../../governance/license-matrix.md` | Licensing per source | Epic 4 book-onboarding cycles that need to verify a book's licence before ingest |
| `../../../governance/third-party-tier-licensing-survey.md` | Third-party publisher tiering (Dreamscarred Press, etc.) | Epic 4 if any third-party books are onboarded (per the operator-pinned 2026-08-01 amendment) |
| `../../../governance/pi-sweep-baseline.tsv` | The PI-sweep baseline data | Not consumed by SD-32 directly; the linked reader (`pi_screening::*`) reads it |
| `../../../doctrine-external/identifier-discipline.md` | The forbidden source-identifier patterns | Every cycle (dual-audit gate, `workflow-instruction.md §6`) |
| `../../../doctrine-external/spec-domain-lifecycle.md` | How spec-domain routing works | Not directly relevant to SD-32 (this is a research/CODE bundle, not a spec-domain routing decision); useful when defining how SD-32's outputs route into a successor spec-domain bundle |

## Skills (profile-local)

These are skills the bundle's cycles may load at runtime via `skill_view(name)`. They are not
mandatory at planning time — cycles load them when they need them.

| Skill | When a SD-32 cycle loads it |
|---|---|
| `wired-integration-discipline` | Any cycle that touches production code and needs the four-check audit recipe (`OK_NO_TOKENS` / `OK_NO_NOOP_HANDLERS` / `OK_NO_MOCK_LEAKS` / `OK_NO_WOULD_STRINGS`) |
| `identifier-discipline` | Any cycle that touches production code and needs the source-identifier audit recipe |
| `graphify-update` | The closure epilogue cycle (architecture-docs refresh; `workflow-instruction.md §13` step 4) |
| `architecture-truth-up` | The closure epilogue cycle (architecture-docs refresh) |
| `merge-conflict-resolution` | The closure epilogue cycle (PR open + merge-conflict resolution) |
| `kanban-worker` | Any cycle that needs the local-file kanban pattern (this bundle uses the pattern) |
| `kanban-handoff-projection-audit` | Any cycle that needs to audit which handoff files belong on which cards |

## Retrospectives

| Path | What it carries | When a SD-32 cycle reads it |
|---|---|---|
| `../../../retro/sd31-retrospective.md` | The SD-31 retrospective, grounded in the 1,940-event log. Our own written artifacts (briefs, dispatch prompts, README/kanban bodies) are the most frequently wrong thing in the program — not code, not people. Two infrastructure failures (disk-full 120x + disk-pressure 16x; wrong-base-worktree 27x) went unfixed for 30 waves. | First cycle of every gate, and any cycle authoring a dispatch prompt or planning-doc figure — this is the source for why the dual-audit gate and re-derive-don't-transcribe rules exist. |
| `../../../retro/sd32-compute-library-and-cause-closure-retrospective.md` | This bundle's own retrospective, grounded in `scripts/retro.py summary --since 2026-08-22 --json`, updated at final closure (2026-08-24). All four gates closed clean on their own written criteria; the original "card 11 filed under Open blockers" closure was overturned by the operator the same day (`decisions.md §10`) and card 11 closed for real 34 cycles later by doing the work; every Epic 1-13 kanban card reached `complete`; this closure cycle's own PI-sweep catch (a stale generated artifact, `feat_gap_tables.rs`) and its unresolved worktree/branch-sweep escalation are the two live findings carried forward. | Written and cited by the closure epilogue cycle (`workflow-instruction.md §13` step 2); any successor bundle's chassis-authoring cycle should read it before writing its own `workflow-instruction.md §2.4`. |

## This bundle's own artifacts (read first)

| Path | What it carries | When a SD-32 cycle reads it |
|---|---|---|
| `../artifacts/HANDOFF.md` | The SD-31 → SD-32 session handoff: five operator-pattern footguns, two theses refuted, the anti-gaming apparatus, what is immediately actionable. Captured 2026-08-22; not edited — where a figure has since been re-derived, the citing doc carries the correction (`decisions.md §8`). | **Before anything else** in every dispatched cycle. |
| `../artifacts/UNMERGED-BRANCHES.md` | Ten branches at the `tranche/11 → tranche/12` boundary (nine local-only) and their disposition order. | Card 2 (`boundary-branch-review`) and the §13 closure sweep. |
| `../artifacts/corpus/README.md` | The repo-local PCGen oracle slot (`operator-supplied/pcgen`, git-ignored) — the only oracle location this bundle references. | Every cycle that touches the corpus (`workflow-instruction.md §2.1` env block). |

## Sibling bundles

| Path | Bundle | Relationship to SD-32 |
|---|---|---|
| `../SD-31-corpus-closure-grind/` | SD-31 — Corpus Closure: the Grind and the Capability Builds | **Predecessor.** Content merged to develop via PR #374 (2026-08-22). Carries the inventory + interpreter + anti-gaming apparatus SD-32 inherits; its `loop-instruction.md` is the worked example `workflow-instruction.md` follows; its `todo/blocked.md` is the source of the B1/B2/B4/B5 numbering. This bundle's own `../artifacts/HANDOFF.md` (row above) is the captured session context. |
| `../SD-30-class-feature-archetype-bundle/` | SD-30 — Class Feature Archetype Bundle | **Indirect predecessor.** Owns `pi_screening::*`, the `doneness_verdict()` table, and the build-version numbering amendment SD-32 inherits via `decisions.md §1`. SD-30 absorbed the previous `SD-32-instrument-coverage-and-consumer-wiring/` package on 2026-08-15; the precedent for SD-32's swap is in SD-30's own `decisions.md §50`. |
| `../SD-29-corpus-wide-catch-up-lanes/` | SD-29 — Corpus-Wide Catch-Up Lanes | **Pre-predecessor.** Carries `corpus-shape-37-books.md` and the per-kind `done`-floor table that SD-32's Gate 0 census walk reads. |
| `../SD-28-ultimate-book-content-ingestion/` | SD-28 — Ultimate Book Content Ingestion | **Pre-predecessor.** Carries `forward-scope-register.md`'s C2.x precedent for third-party publisher tiering (Dreamscarred Press, Ultimate Psionics). |
| `../SD-33-pcgen-character-import/` | SD-33 — PCGen Character Import | **Parallel (out-of-scope adjacent).** Renamed from the original SD-31 by operator ruling 2026-08-14. No file-level touch points with SD-32. |
| `../SD-27-future-state-book-content-ingestion/` | SD-27 — Future-State Book Content Ingestion | **Pre-predecessor.** Source of the "two advanced guides" book list pattern (`APG` + `ACG`). Not consumed by SD-32 directly. |
| `../template/template.md` | Bundle template | The release-folder's file index and bundle-snapshot table this package's `README.md` was authored from. |

## Root-level cross-references

| Path | What it is |
|---|---|
| `../../../AGENTS.md` | The repo-root conduct surface — non-negotiable rules, hard stops, retrospective-log discipline. Every cycle reads this once per bundle. |
| `../../../CLAUDE.md` | Lightweight activation surface. Read for the project-specific pointers (file structure, tooling). |
| `../../../architecture/` | Architecture docs; refreshed at closure per the closure pipeline (`workflow-instruction.md §13` step 4). |
| `../../corpus-work-channels.md` | The 37-book workchannel map SD-32's Gate 0 census diff reads (`docs/release/corpus-work-channels.md`). |

## What is NOT in `references/`

- Operator-ruling text — those live in `SD-31-corpus-closure-grind/artifacts/OPERATOR-RULINGS-*.md`
  and are read by reference, not duplicated here.
- The anti-gaming apparatus itself — that lives in the cycle procedure (`workflow-instruction.md §6`'s
  dual-audit gate) and the recipe (`SD-31-.../state-goals-and-lessons.md`). Duplication would
  drift; a reference is sufficient.
- The form-interpreter PMMG build — referenced from `SD-30-.../state-goals-and-lessons.md §1.3`,
  not from here.
