# SD-13 Upstream Dependency Contract

## Purpose
This contract records what the upstream strategic, documentary, and repo surfaces authorize for SD-13 and what they explicitly do not authorize.

## Upstream surfaces and permitted use

| Upstream surface | What SD-13 may rely on | What it does not authorize |
|---|---|---|
| `programs/codex/plans/spec-domains/SD-13-core-class-race-roster-and-level-10-progression-matrix.md` | the strategic objective, exact core-race/core-class boundary, single-class-through-level-10 posture, and same-domain source-STC obligation | repo implementation authority, multiclassing, non-core breadth, or proof that broad roster support already exists |
| `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md` | the current accepted character-path proof ceiling and the rule that Human Fighter level 1 is a bounded pilot rather than broad support | evidence for other classes, other races, Fighter level 2+, or level-10 support |
| `programs/codex/requirements/GE-10-demo-proof-and-onboarding/README.md` | the current truthful repo narrative that Codex is still a developer proof harness plus bounded desktop workbench surface | product breadth, general character-builder maturity, or tester-visible roster completeness |
| `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md` | tester-workbench, diagnostics, and support-language ownership boundaries | class/race support taxonomy authority or level-10 progression truth |
| `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/tester-workbench-surface-specification.md` | the fact that later tester surfaces need bounded workflow truth and visible diagnostics | permission to invent support labels independent of the SD-13 matrix |
| `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md` | distribution/update/channel ownership boundaries and the rule that shipping a build is not the same as proving capability depth | any promotion of class/race support truth merely because a build is deliverable |
| `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/distribution-platform-support-and-channel-matrix.md` | downstream platform/channel vocabulary that later breadth claims must not contradict silently | platform delivery as evidence of class/race semantics |
| `/home/ubuntu/workspace/repos/codex/README.md` | the current repo truth that the project is not yet a general character builder or broad Pathfinder product | proof that broad core-roster support already exists in runtime code |
| `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt` | the exact current accepted race/class proof input surface | support for any race other than Human, any class other than Fighter, or any level above 1 |
| `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_total_saves.rs` | direct current negative evidence that Rogue level 1 and Fighter level 2 are claim-blocked in the bounded compute path | blanket blocked status for every other class without direct named evidence |
| `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_combat_baseline.rs` | direct current negative evidence that Fighter level 2 remains unsupported for combat/defense claim surfaces | proof of broader race semantics, broader class families, or level-10 support |

## Downstream obligations imposed by this contract
Any later SD-13 execution handoff must:
- preserve the separate support-state and evidence-tier axes
- preserve the exact roster identity and PF1 Core Rulebook-only scope
- preserve SD-11 ownership of tester-facing support wording and issue-flow UX
- preserve SD-12 ownership of build/update/channel truth
- preserve SD-14 ownership of saved-state continuity
- name explicitly when a race row, class row, or interaction row is the real unit under test
- refuse any “core roster complete” claim unless the matrix rows and burden tables actually justify it

## What this packet still does not prove
This packet does not prove:
- that any non-Human race already works in runtime code
- that any class other than the bounded Fighter level-1 pilot already works in runtime code
- that Fighter level 2 or level 10 works
- that any spellcasting class has an accepted spell burden implementation
- that all race/class combinations deserve equal first-wave execution priority

## Propagation rule
If a later implementation slice discovers a new breadth truth, a new blocked interaction seam, or a new downstream dependency that changes program-level expectations, patch this contract and the SD-13 README before claiming the new posture as settled truth.
