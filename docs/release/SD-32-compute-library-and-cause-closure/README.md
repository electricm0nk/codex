---
canonical: true
owner: god-emporer
status: planning-ready (chassis completed 2026-08-22 from SD-31 session handoff)
date: 2026-08-22
canonical_branch: tranche/12
build_version_target: 0.12.0 (tranche cut bump landed 2026-08-22; published builds stamp 0.12.<build>; decisions.md §1/§9)
predecessor: SD-31-corpus-closure-grind
companion_to: ./scope-draft.md, ./decisions.md
---

# SD-32 — Compute Library and Cause Closure

> ## ⚠️  OPERATING METHOD — REQUIRED FOR THIS BUNDLE  ⚠️
>
> **This bundle is operated via a `Workflow`-tool script, invoked from a live session — NOT `/loop /batch` and NOT a one-shot task.** `/loop /batch` cannot run unattended (`/batch` requires a human to type it per invocation). For how to actually **create** that script for this bundle's own gate structure, see `workflow-instruction.md §2.4`. The full per-cycle procedure, orchestration mode, concurrency map, dual-audit gate, retro-event-logging discipline, and gate/bundle closure steps live in `workflow-instruction.md`'s body, authored from `../../governance/workflow-instruction-template.md`. The scope-draft ([`./scope-draft.md`](./scope-draft.md)) is the canonical handoff *what*; the workflow-instruction is the *how*.

## Note on the directory name and the `SD-32` number

The number `SD-32` has now been held by three different packages over the course of this program:

1. `SD-32-instrument-coverage-and-consumer-wiring/` — deleted 2026-08-15 (`89078307c`), absorbed into SD-30 per operator ruling.
2. `SD-32-engine-capability-builds/` — deleted 2026-08-15 (`docs(sd31): absorb SD-32 into SD-30 and delete the package`), absorbed into SD-31 per operator ruling.
3. `SD-32-compute-library-and-cause-closure/` — **this package**, opened 2026-08-22 from the SD-31 session handoff.

The dead `SD-32-instrument-coverage-and-consumer-wiring/` directory was cleaned up at the same time this package's chassis was filled out (untracked `__pycache__` only; nothing to recover). This is the canonical home for the third package. If a fourth package ever holds the number, follow this same precedent and document the swap here.

## Bundle at a glance

| Field | Value |
|---|---|
| Bundle ID | SD-32 |
| Slug | `compute-library-and-cause-closure` |
| Canonical branch | `tranche/12` (operator ruling 2026-08-22) |
| Kanban board | **local-file** `kanban.md` (no Hermes board; per SD-30 decisions §14a, retired 2026-08-01) |
| Epics / criteria | 5 epics / 4 gates (G0/G1/G2/G3) / 13 kanban cards |
| Target version | `0.12.0` bumped on `tranche/12` at launch (2026-08-22); published builds stamp `0.12.<build>` (`decisions.md §1/§9`) |
| Dispatch mechanism | `Workflow` tool, invoked from a live session, per `workflow-instruction.md §2` |
| Cadence | N/A — dispatch is a live `Workflow` session, not a timer loop |
| PCGen oracle | Repo-local: `artifacts/corpus/operator-supplied/pcgen` (git-ignored slot; `artifacts/corpus/README.md`) — never `~/workspace/repos/pcgen` |
| Closure gate | **Definition of Done — all four gates' AT-32-* criteria met (operator ruling 2026-08-22), never a wave budget.** Then: retrospective written + cited; worktree/branch sweep; `tranche/12 → develop` PR; architecture-docs refresh; release-notes generation — full sequence in `workflow-instruction.md §13` |

## Why this package exists

SD-31 spent thirty waves converting PCGen's rules into this engine feature by feature. The board moved 15.15% → 35.07%. The operator's standing objection, in their own words, is the thing SD-32 exists to answer:

> *"I really feel like it shouldn't be taking this long to convert rewritten Java logic into rust. measure twice, cut once. let's keep measuring."*

They are right, and SD-31's own measurement waves (28, 30, 31) found why. Three findings shape this package, and none of them is "the rules are hard":

**1. We solved the same problem thousands of times.** Across `data/corpus`, 33,830 formula-bearing tokens reduce to 14,752 distinct formulas — and those reduce to **ten semantic families**. Not forty, not one; ten, and that reduction survived independent re-derivation exactly.

> *"1d6 per level. or +2 damage on a dagger, or you get 3 spells at this level and 4 at that level. those types of things repeat a lot. you shouldn't be trying to figure them out from scratch for every item — you need a library of common computes you can draw from."*

**2. Most blockers were our own plumbing, not the rules.** The archetype is the Monk case: a complete chassis table sat unreachable for a month because one line of dispatch was missing, and adding it closed four claim-blocking diagnostics at all 20 levels. SD-31's history holds 98 uses of "root cause", 332 of "silently" and 17 of "blind spot" across 36,930 lines — a taxonomy nobody had mined until wave 31.

**3. The engine can only build eleven classes.** `compute_class_chassis` recognises exactly the eleven CRB base classes. Every prestige class, every Advanced Class Guide class, every archetype computes correctly and reaches nobody, because no character of that class can exist. `class` is 28 done of 185, and it gates `class_feature` — 60% of everything remaining.

## What SD-32 is, in one sentence

**Stop hand-deriving what repeats; build the compute library, close the plumbing causes by class rather than by instance, and unblock the classes that make the rest reachable.**

## What SD-32 inherits from SD-31

Load-bearing, and none of it should be rediscovered:

* **A complete inventory.** `SD-31-corpus-closure-grind/artifacts/THE-BOX.md` — all 24,914 not-done units in 46 groups, uncovered = 0, verified by `scripts/coverage_ledger.py` rather than hand arithmetic. Every group carries a todo entry.
* **A working formula interpreter.** `src/rules_core/pilot_compute/formula_interpreter.rs`, semantics derived per claim from PCGen's own Java source, reproducing 22 of 22 hand-modelled functions across 7,040 comparisons with zero disagreements. Reads 84% of corpus arithmetic; refuses the rest by name rather than guessing.
* **A trustworthy grant-fact parser** and the merged grant data it produces.
* **The anti-gaming apparatus**, which is the reason any of these numbers can be believed: four GAMED verdicts across waves 18–27, every one correct; integration cycles that re-derive rather than trust and have caught a load-bearing defect in every wave since 18.
* **The todo directory** (`SD-31-corpus-closure-grind/todo/`) as the scheduling layer, reconciled every wave.
* **Five operator-pattern footguns** captured in `artifacts/HANDOFF.md` from the SD-31 session — wrong-base worktrees, `find -newermt` lies, omitted `model` on `agent()` calls, `git stash` taking the whole shared checkout, rulings not in force until committed. These live in `workflow-instruction.md §9`, not in this README.

## Epics

| # | Epic | Rests on |
|---|---|---|
| 1 | **The compute library.** Build the top shape families once, prove each once, and reuse. Harvest what already exists — ~166 hand-modelled functions are already proven byte-exact. | Gates 1 + 2 (`decisions.md §6`; kanban card 10) |
| 2 | **Cause closure.** Take each blocker shape from the root-cause taxonomy and close it corpus-wide rather than instance by instance. | Gates 1 + 2 (`decisions.md §6`; kanban card 11) |
| 3 | **Class reachability.** The 77 prestige classes need entry-requirement gating that exists nowhere in the codebase; 18 real base classes have no table; 28 sit in books with no compiled rule set. | Gate 0 (`decisions.md §6`; kanban card 12); figures from SD-31 wave 27 census |
| 4 | **Book onboarding.** Four books have no compiled rule set at all — **422 units** behind a missing `RuleSetId` variant (`epic-breakdown.md` Epic 4; re-derive: `jq` over `docs/work-inventory.json` by book). Cost is calibrated at roughly 1.5–2h per book, dominated by ~7 count-pinning files. | Gate 0 precondition (kanban card 4); `SD-31-corpus-closure-grind/artifacts/THE-BOX.md §3 #3` |
| 5 | **Automation, decided on evidence.** Only candidates whose output can be independently checked. A tool that generates values needs a fixture from bytes it does not read, or it manufactures plausible numbers faster than a human could. | SD-31 wave 31's automation case |

Sequencing lives in `scope-draft.md` §"Sequencing" — the hard ordering is `census closure → shape closure → engines`, not by preference but by construction: an engine's correctness claim *"this shape is now handled"* is false if the shape's population can still grow.

## Files in this folder

| File | Job | Owner |
|---|---|---|
| `README.md` | Bundle index, why-it-exists, files in folder | operator |
| `scope-draft.md` | Canonical handoff *what* — definition of done, four gates, sequencing | operator |
| `decisions.md` | Bundle-specific ADRs (build version, gates, anti-gaming, ordering) | operator |
| `workflow-instruction.md` | Per-cycle launch *how* — eligibility, self-heal, post-mortem schema | operator |
| `progress.md` | Live cycle-by-cycle progress + status matrix | loop |
| `epic-breakdown.md` | Five epics with measured ceilings and per-class cycles | operator |
| `acceptance-and-verification.md` | AT-32-* criteria + the four gates as closure gates | operator |
| `risks-and-open-questions.md` | Self-healable vs. non-self-healable split; B1/B2/B4/B5 rulings | operator |
| `forward-scope-register.md` | Deferred systems (Traveller/Cyberpunk/WoD/Solarus Arcanum) | operator |
| `release-notes.md` | Closure-time release notes template | operator |
| `kanban.md` | Local-file kanban in claim-priority order | loop |
| `content-unit-inventory.md` | Per-content-unit N-tuple (per kind/per book/per cycle) | operator |
| `technical-requirements.md` | Pre-loop prerequisites + normative requirements | operator |
| `technical-design.md` | Architectural surface; engine/API shapes; cross-book resolution patterns | operator |
| `artifacts/` | Per-cycle evidence: parity fixtures, receipt comments, cycle receipts | loop |
| `artifacts/HANDOFF.md` | SD-31 → SD-32 session handoff (read first) | operator (captured 2026-08-22) |
| `artifacts/UNMERGED-BRANCHES.md` | 10 branches at tranche/11 → tranche/12 boundary | operator (captured 2026-08-22) |
| `artifacts/README.md` | Cycle-artifacts index | operator |
| `artifacts/corpus/` | The repo-local PCGen oracle slot (`operator-supplied/pcgen`, git-ignored); the only oracle location this bundle references | operator (populated by `scripts/fetch-pcgen-oracle.sh --dest`) |
| `references/` | Doctrine pointers, skill pointers, sibling bundle pointers | operator |
| `references/README.md` | Doctrine / skill / sibling-bundle reference index | operator |

## Standing constraints carried forward

These are not negotiable and were each bought with a failure:

* **Anti-gaming (Decision 1a).** A gate that cannot fail is worse than no gate.
* **Proof width.** A correctness proof is only as wide as the cases it covers. Wave 21's parser passed its own mutation proof, reproduced 64 records exactly, and still fabricated 73.4% of its output. Every lane states which shapes its proof does **not** cover.
* **Fixtures.** Every interpreted value clears `derived_evaluator_fixture_check`, whose expected value is transcribed from bytes the evaluator never reads. An interpreted value with no fixture is not done. This is the condition operator ruling §20 rests on.
* **Reclassification is not a gain**, and is reported as its own number.
* **§7's prose bar**: shown to a player, proven on screen with the real driver. A record that merely loads is not done.
* **§18**: option pools show only valid choices; exclusive pools may not use the browsable pattern.
* **Race attribution frozen**; the Supersession Register proposed, not applied.

## Status

**Planning-ready.** Opened 2026-08-22 during SD-31 wave 31 so the reasoning is captured while it is fresh. The four source documents (README, scope-draft, epic-breakdown, HANDOFF) were filled out the same day. The chassis is complete: 14 root `.md` files (the 13 template-canonical files plus `technical-requirements.md`/`technical-design.md`/`forward-scope-register.md`/`kanban.md`/`release-notes.md` per house convention — see the table above), full `artifacts/` and `references/` trees, dead `SD-32-instrument-coverage-and-consumer-wiring/` folder cleaned up. Launch-readiness remediation ran 2026-08-22: SD-31's content is on `develop` (PR #374, verified by content), `workflow-instruction.md §1` was run for real with outputs pasted, the PCGen oracle moved into `artifacts/corpus/`, and the version bump to `0.12.0` landed. **Launch-ready on `tranche/12`.**
