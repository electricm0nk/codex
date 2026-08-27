---
title: <PREFIX-NN> — <Bundle Title> — Release Package
status: <planning | running | closed>
bundle_id: <PREFIX-NN>
slug: <bundle-slug>
scope: <path-to-release-packages>/<PREFIX-NN>
artifact_type: release-index
canonical_branch: <branch-name>
board: <board-slug>
target_version: <major>.<minor>.<build>
canonical_source: <path-to-release-packages>/<PREFIX-NN> (this folder)
date: <ISO-8601>
---

# <PREFIX-NN> — <Bundle Title> — Release Package

> ## ⚠️  OPERATING METHOD — REQUIRED FOR THIS BUNDLE  ⚠️
>
> **This bundle is operated via a scripted dispatch tool, invoked from a live session — not an
> unattended timer loop, and not a one-shot task.** An unattended timer loop cannot run without a
> human re-triggering each cycle by hand. For how to actually **create** the dispatch script —
> phase structure, tiering, worked skeleton — see `workflow-instruction.md §2.4`. The full
> per-cycle procedure, orchestration mode, concurrency map, dual-audit gate, event-logging
> discipline, and epic/bundle closure steps live in `workflow-instruction.md`'s body, authored
> from `./workflow-instruction-template.md`. The scope-draft
> ([`./scope-draft.md`](./scope-draft.md)) is the canonical handoff *what*; the
> workflow-instruction is the *how*.

This folder is the canonical surface for `<PREFIX-NN>`. Everything the bundle needs is in this
folder and the in-repo doc tree (sibling release folders, a doctrine/conduct tree, an
architecture-docs tree). Any pre-launch drafting workspace outside this repo is referenced only at
initial-package construction time; once the package lands here, work reads the repo-local copy and
the drafting-workspace copy is no longer consulted.

## 0. Preamble

The bundle's intent, scope, and acceptance-evidence obligations live in
[`scope-draft.md`](./scope-draft.md). The per-cycle launch form, eligibility checks, and self-heal
mechanics live in [`workflow-instruction.md`](./workflow-instruction.md). This README is the index.

## 1. Bundle snapshot

| Field | Value |
|---|---|
| Bundle ID | `<PREFIX-NN>` |
| Slug | `<bundle-slug>` |
| Canonical branch | `<branch-name>` (operator directive `<date>`) |
| Board | `<board-slug>` (operator directive `<date>`) |
| Epics / criteria | `<N epics>` / `<N criteria>` |
| Target version | `<major>.<minor>.<build>` |
| Dispatch mechanism | scripted dispatch tool, invoked from a live session, per `workflow-instruction.md §2` |
| Cadence | N/A — dispatch is a live session, not a timer loop |
| Closure gate | `<branch-name> → <trunk>` merge request; retrospective written + cited; worktree/branch sweep; release-notes generation; architecture-docs refresh (§6) — full sequence in `workflow-instruction.md §11` |

## 2. Files in this folder

| File | Job | Owner |
|---|---|---|
| `scope-draft.md` | Canonical handoff *what* — bundle intent, epics, criteria | operator |
| `workflow-instruction.md` | Per-cycle launch *how* — eligibility, self-heal, post-mortem schema | operator |
| `progress.md` | Live cycle-by-cycle progress + status matrix | dispatch tool (created on first cycle) |
| `decisions.md` | Bundle-specific decision records | operator |
| `technical-requirements.md` | Pre-launch prerequisites + normative requirements | operator |
| `epic-breakdown.md` | Acceptance criteria 1-N grouped across epics | operator |
| `acceptance-and-verification.md` | Closure gates + verification commands + per-criterion artifact map | operator |
| `risks-and-open-questions.md` | Self-healable vs. non-self-healable split; open override flags | operator |
| `technical-design.md` | Architectural surface; component/API shapes; cross-cutting resolution patterns | operator |
| `content-unit-inventory.md` | Per-unit N-tuple, describing whatever the bundle counts (e.g. parts in a catalogue, SKUs, endpoints, records) and its measured baseline | operator |
| `artifacts/` | Per-cycle evidence: fixtures, receipt comments, cycle receipts | dispatch tool (populated per cycle) |
| `artifacts/README.md` | Cycle-artifacts index (per-epic subdirectories + closure-readiness report) | operator-authored at package-construction time |
| `references/` | Doctrine pointers, skill pointers, sibling-bundle pointers | operator |
| `references/README.md` | Doctrine / skill / sibling-bundle reference index | operator |

## 3. In-repo cross-references

Every reference below is repo-relative. No paths into an external drafting workspace — those live
at initial-package-construction time only and are not load-bearing for anyone reading this folder
later.

- **Sibling release folders** — `../<PREFIX-MM>/` for any other in-flight or historical bundle.
- **Repo-local doctrine mirrors** — the conduct/doctrine tree's identifier-discipline and
  spec-lifecycle docs, the no-stub-shipping doctrine, and its stub-exceptions registry.
- **Architecture docs** — an `architecture/` tree (topic-by-topic; the closure epilogue §6
  obligation re-verifies every touched topic).
- **Repo-local conduct surface** — the repo's top-level conduct file (non-negotiable rules) and its
  lightweight activation surface for coding-agent harnesses.

## 4. Relationship to other release folders

- **Sibling bundles** — `../<PREFIX-MM>/` for any other in-flight or historical bundle.
- **Branch posture** — this bundle ships on `<branch-name>`.

## 5. Build version target

For `<PREFIX-NN>`, the release version triple is `<major>.<minor>.<build>`:

- **`major`** — `0` until first publish to the trunk/production line; increments by `1` per
  publish to it.
- **`minor`** — base digit of the active branch line (choose the convention that fits your
  repo — e.g. one digit per release train).
- **`build`** — monotonic counter across all builds across all branches (never resets).

`<PREFIX-NN>`'s first concrete value is `<major>.<minor>.<current_build_at_launch>`.

## 6. Architecture-docs and closure obligation (bundle closure epilogue, artifact sub-steps)

This section is the *artifact-level* half of the bundle closure epilogue — architecture docs, the
merge request, and merge-conflict resolution. It fires **once**, as the bundle's own final epic,
never per-epic. The *procedural* half — writing and citing the bundle's retrospective, and the
full worktree/branch sweep — is defined in `workflow-instruction.md §11`; both halves are one
pipeline, split across these two files only because this file owns the chassis/artifact shape and
that one owns the per-cycle procedure. Do not duplicate either half's content into the other file;
cross-reference instead.

The living architecture documentation (repo-relative) is part of this bundle's closure gate. The
pipeline is sequential — every sub-step fires regardless of diff content:

1. **All acceptance criteria done, and every epic/board card at `complete`?** If not, self-heal
   and dispatch more cycles.

   **This is a hard gate, and a filed blocker does not satisfy it** (see the blocker-closure
   doctrine referenced from `references/README.md`). An `## Open blockers` entry is a request for
   an operator ruling — not a disposition and never a closure path; filing one **pauses the
   bundle**. A blocker standing between the bundle and 100% of its definition of done gets
   **cleared** (decompose it and run the cycles — a large blocker is a sequencing problem, not an
   exemption) or **escalated to the operator** with the specific ruling, write scope, or
   precondition named — never deferred, never handed to a successor bundle on the cycle's own
   authority. Never write a closure criterion as "complete *or* filed under `## Open blockers`":
   that phrasing is the defect this doctrine removes.

   If anything is short, **the closure epilogue stops here** — no retrospective, no sweep, **no
   merge request**. Report what is short with the command that shows it, and exit. That is a
   correct outcome for a closure cycle, not a failure.

   **Between step 1 and step 2, `workflow-instruction.md §11` steps 2–3 fire**: write and cite the
   bundle's retrospective, then run the full worktree/branch sweep. Both must be done before step
   2 below opens the merge request — a retrospective or a stray worktree found *after* the merge
   request is open is a correction cycle, not a clean closure.
2. **Architecture docs updated?** If not, run this bundle's architecture truth-up tooling (whatever
   your repo provides) with the bundle ID and a receipts path. The tooling edits touched docs in
   place, removes obsolete statements, refreshes "last verified" headers, runs the maintenance
   contract's verification commands, and appends a receipt to `receipts.md`. Empty diffs still
   write a receipt — the receipt IS the audit evidence that the gate fired.
3. **Merge request open?** If not, open it.
4. **Merge conflicts resolved?** If any, fix them. On conflicts, record a `merge_conflict:*`
   receipt and stop — self-heal or operator resolves manually, then the dispatch loop re-runs
   until clean.
5. **Stop — closure is complete.**

The full rules and procedure live in the architecture tree's own maintenance-contract doc. The
receipt block in `<this-folder>/receipts.md` is the durable audit trail; without it, the bundle
did not run through the closure pipeline in a verifiable way.

## 7. Initial-package construction (operator-only, before launch)

Before this folder exists, the operator constructs the release package from initial requirements
in a drafting workspace outside the repo. A promotion step takes the finished package and copies
it into this folder deterministically. The drafting-workspace copy is not consulted again after
the promotion.

Required canonical files (the promotion step refuses to copy if any are missing):

- `README.md`, `scope-draft.md`, `workflow-instruction.md`, `progress.md`
- `epic-breakdown.md`, `decisions.md`, `risks-and-open-questions.md`,
  `acceptance-and-verification.md`
- `content-unit-inventory.md`, `artifacts/`, `artifacts/README.md`, `references/`,
  `references/README.md`

Every placeholder in the copied package must be resolved to a real value, or explicitly documented
with a named resolution point (e.g. "resolved at the first cycle of Epic 3"), before the bundle is
marked launch-ready. An unresolved `<...>`-style placeholder with no named resolution point is a
launch blocker, not a cosmetic gap.

## 8. Cross-reference

- `./workflow-instruction-template.md` — the per-cycle dispatch procedure
  `workflow-instruction.md` is authored from. Distinct scope: this template covers the
  release-folder's file index and bundle-snapshot table; that one covers the per-cycle dispatch
  procedure, including its epic wrap-up and bundle closure epilogue (the retro-write and
  worktree-sweep half of §6 above). Both must agree on the dispatch mechanism — if one changes,
  check the other.
- A blocker-closure doctrine doc — a blocker on the definition of done is cleared or escalated,
  never deferred; `## Open blockers` is a request for an operator ruling, not a closure path.
  Gates §6 step 1 above.
- A deferral-revisit doctrine doc — the sibling rule for a *planned capability deferral*. The test
  that separates the two: was this scope in the definition of done at launch?
- A retrospectives folder — retrospectives written at bundle closure (§6 /
  `workflow-instruction.md §11`).
- `../skills/release-package-authoring/SKILL.md` — the coding-agent-native rendering of this
  template plus `workflow-instruction-template.md`, for a session authoring or auditing a bundle
  directly in this repo.
