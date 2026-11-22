# SD-14 Upstream Dependency Contract

## Purpose
This contract records what the upstream strategic, documentary, and repo surfaces authorize for SD-14 and what they explicitly do not authorize.

## Upstream surfaces and permitted use

| Upstream surface | What SD-14 may rely on | What it does not authorize |
|---|---|---|
| `programs/codex/plans/spec-domains/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision.md` | the strategic objective, scope boundary, minimum persistence truths, and same-domain source-STC obligation | repo implementation authority, concrete storage technology, or any claim that save/load already exists |
| `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md` | bounded character-input/computation truth and the rule that supported character state must remain explainable | broad Pathfinder coverage, durable saved-state implementation, or lifecycle/migration truth |
| `programs/codex/requirements/GE-10-demo-proof-and-onboarding/README.md` | the current truthful desktop/developer-proof posture and anti-counterfeit product language | saved-character continuity, tester-state durability, or public-product maturity claims |
| `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md` | tester-workbench, diagnostics/evidence posture, and GitHub-facing issue-flow boundaries | character persistence authority, migration semantics, or local save-file behavior |
| `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/tester-feedback-evidence-capture-matrix.md` | the fact that saved-state failures may need evidence capture, including save-file attachment vocabulary | proof that saved files already exist in the runtime, or permission to invent their schema |
| `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md` | distribution/update/rollback authority and the rule that operator release truth and tester-facing update truth must remain honest | saved-character migration or continuity success; update transport is not proof of saved-state survival |
| `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/rollback-withdrawal-and-downgrade-policy.md` | adjacent recovery/withdrawal vocabulary and the need to preserve explicit recovery guidance | exact saved-state downgrade semantics or permission to hide persistence incompatibility under release recovery language |
| `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs` | the current bounded authoritative character-input seam that persistence must preserve | a durable save format, revision model, or migration subsystem |
| `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/package_store.rs` | an adjacent deterministic local persistence pattern for versioned artifacts | evidence that character save/load, character revision, or character migration already exists |
| `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts` | adjacent lifecycle-gate vocabulary (`saveAllowed`, `exportAllowed`, `diffMode`) that may inform future UI shape | permission to reuse GE-08 package lifecycle semantics unexamined for character lifecycle |

## Downstream obligations imposed by this contract
Any later SD-14 execution handoff must:
- preserve GE-06 character-domain truth rather than replacing it with UI-local or cache-local folklore
- preserve SD-11 ownership of tester-facing evidence capture and issue-flow UX
- preserve SD-12 ownership of update/rollback transport truth while separately proving saved-state survival
- state explicitly when it is reusing a pattern from GE-08 package persistence and where that pattern stops being valid for character lifecycle
- carry exact compatibility vectors, exact blocked/read-only outcomes, and exact recovery posture

## What this packet still does not prove
This packet does not prove:
- that the current runtime can already save or reopen a character
- that any concrete storage backend is the correct choice
- that every future breadth increase will remain compatible with earlier saves automatically
- that update or rollback workflows already preserve local character state
- that save-file corruption or missing dependencies already produce governed evidence surfaces in the runtime

## Propagation rule
If a later implementation slice discovers a new authoritative saved-state seam, compatibility vector, or recovery surface that changes program-level expectations, patch this contract and the SD-14 README before claiming the new behavior as settled truth.
