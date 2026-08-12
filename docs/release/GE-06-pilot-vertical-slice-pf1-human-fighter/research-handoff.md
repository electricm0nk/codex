---
title: GE06-E1-F1 Research Handoff — Grounded Character Selection Ledger
handoff_id: HANDOFF-CODEX-GE-06-E1-F1-RESEARCH-2026-06-20
handoff_kind: research-brief
work_type: research-only
workflow_route: research
readiness: completed-by-final-contract
status: superseded
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/research-handoff.md
source_stc: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md
source_readiness: planning-ready
readiness_closure: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-handoff-readiness-closure-2026-06-20.md
superseded_by: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md
selected_slice: GE06-E1-F1 — Grounded character selection ledger
created_at: 2026-06-20
code_authority: false
---

# Research Handoff: GE06-E1-F1 — Grounded Character Selection Ledger

## Supersession notice
This research handoff is preserved as historical routing evidence. Its requested selection-closure work is complete and superseded by `artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`.

## Objective
Recover the exact deterministic PF1 Core Rulebook Human Fighter level 1 input selection set that GE-06 needs before any integrated implementation can be honest.

The downstream worker must close or explicitly block:

- exact skill allocation
- exact equipment loadout and active-state assumptions
- any additional feat or choice entitlements implied by Human and Fighter source surfaces beyond the charter-seeded `power_attack`
- whether representative anchors such as Chain Shirt and Longsword are sufficient or merely adjacent candidates
- whether the resolved selection set changes the charter boundary, requires a charter patch, or triggers an ADR

The worker must do this using grounded source evidence only. It must not invent computed values, parity evidence, UI truth, or repo implementation scope.

## Work Type
`research-only`

This is bounded discovery and documentary closure work. It may inspect documents and repositories and run non-mutating discovery commands, but it does not authorize implementation code, source-STC edits, PCGen edits, Codex repo edits, final expected output values, or parity claims.

## Workflow Route
`research`

## Readiness
`research-ready`

Why this handoff is ready:
- the GE-06 source STC is `planning-ready`
- the GE-06 next-stage rule explicitly says no `execution-handoff.md` should be created from the source STC and identifies non-code fixture closure as the likely first move
- `artifacts/ge06-e1-f1-handoff-readiness-closure-2026-06-20.md` establishes GE06-E1-F1 as the correct first bounded slice
- the selected slice is narrower than the full integrated pilot
- the required source universe is explicit and grounded to exact documentary and legacy-source surfaces
- the required output receipt path is explicit

Why this handoff is **not** code-ready:
- `code_authority: false`
- GE-06 remains `planning-ready`, not `implementation-ready`
- this handoff is `research-only`, not `implementation-ready`
- the workflow route is `research`, not `coding`
- no write scope is granted to `/home/ubuntu/workspace/repos/codex`
- no write scope is granted to `/home/ubuntu/workspace/repos/pcgen`
- GE-03/GE-04/GE-05 runtime surfaces remain future work and are not authorized by this handoff
- GE-07 has no source STC yet, so any broad UI route remains blocked

## Source STC
- path: `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md`
- source readiness: `planning-ready`
- source authority: integrated PF1 Human Fighter level 1 pilot-proof requirements for Codex GE-06

## Readiness Closure
- path: `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-handoff-readiness-closure-2026-06-20.md`
- verdict: GE06-E1-F1 is ready only as a non-code research handoff
- blocked stronger route: any repo-facing or product-visible GE-06 handoff remains blocked until the exact character input contract is grounded and later slices declare exact upstream runtime dependencies, repo paths, branch/worktree policy, and verification commands

## Downstream Target
- harness or workflow: `fresh Hermes/God-Emperor session, research-capable agent, or equivalent non-code discovery worker`
- invocation mode: direct session or agent run using this handoff as the primary brief
- target workspace: `/home/ubuntu/workspace`

## Runtime / Repository Context
These are context facts for discovery, not write authority:

| Surface | Path | Posture |
|---|---|---|
| Codex program docs | `/home/ubuntu/workspace/programs/codex` | writable only at the exact receipt path granted below |
| GE-06 STC bundle | `/home/ubuntu/workspace/programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter` | only the required output receipt may be created by this handoff |
| Codex implementation repo | `/home/ubuntu/workspace/repos/codex` | read-only for this handoff |
| PCGen legacy repo | `/home/ubuntu/workspace/repos/pcgen` | read-only for this handoff |

Grounded state from the readiness closure:
- Codex current branch observed: `ge04-e1-f1-character-input-record-shape`
- local untracked residue observed: `AGENTS.md`, `CLAUDE.md`, `Cargo.lock`, `target/`
- the pilot charter grounds Human / Fighter 1 / ability scores `16/14/14/10/12/8` and the named feat seed `power_attack`
- representative but non-final anchors already grounded include Human source-package surfaces, Fighter class/bonus-feat/class-skill surfaces, Power Attack, Chain Shirt, and Longsword

## Required Reads
Read these before producing the selection-ledger receipt:

1. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md`
   - GE-06 authority, no-code boundary, next-stage rule, and explicit unresolveds
2. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/technical-requirements.md`
   - normative fixture, token-family, canonical-object, proof-path, and non-counterfeit requirements
3. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/epic-breakdown.md`
   - GE06-E1 sequencing and GE06-E1-F1 acceptance criteria
4. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/pilot-charter-alignment.md`
   - charter boundary and no-silent-expansion rules
5. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/pilot-character-fixture-requirements.md`
   - current grounded versus unresolved pilot fixture state
6. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/required-token-family-list-requirements.md`
   - hard-gate token families that may force additional choice closure
7. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/required-canonical-object-list-requirements.md`
   - minimum canonical homes that must not be contradicted by the selection closure
8. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/risks-and-open-questions.md`
   - current RQ-06-001 / 002 / 003 and scope-expansion risk language
9. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-handoff-readiness-closure-2026-06-20.md`
   - grounded readiness facts and route limits
10. `programs/codex/plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md`
    - canonical pilot identity, initial acceptance target, and non-expansion rule
11. `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-corpus-inventory.csv`
    - exact grounded pilot source files for Human and Fighter path discovery
12. `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-token-taxonomy.csv`
    - grounded token families including `STARTFEATS`, `ABILITYPOOL`, `CHOOSE`, `CSKILL`, `STARTSKILLPTS`, and `PRE*`
13. `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv`
    - grounded conversion posture for Fighter, Chain Shirt, Longsword, class skills, saves, and skill-budget semantics
14. `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/unsupported-token-ledger.csv`
    - current unresolved Human choice-pool and Fighter prerequisite/skill-budget caveats
15. `programs/codex/requirements/GE-04-rules-engine-and-explainability-core/artifacts/pilot-golden-computation-fixture-requirements.md`
    - GE-04 requirement that GE-06, not GE-04, finalizes the exact pilot path
16. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc`
    - top-level pilot source-package boundary
17. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/_race.pcc`
    - Human subtree include boundary
18. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/human_races.lst`
    - Human race declaration and `STARTFEATS` grounding
19. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/human_abilities_race.lst`
    - Human `ABILITYPOOL` and bonus-feat / choice entitlement grounding
20. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/human_abilities_globalvar.lst`
    - Human replacement-flag and `PREFACT` gating behavior
21. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst`
    - Fighter class line, progression identity, and `STARTSKILLPTS` surface
22. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst`
    - Fighter proficiency, class-skill, and bonus-feat carrier rows
23. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilitycategories.lst`
    - Fighter bonus-feat category and edit-pool posture
24. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_skills.lst`
    - skill schema, key stats, and representative Fighter class-skill rows
25. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_feats.lst`
    - `Power Attack` and any adjacent feat prerequisite surfaces used in the closure
26. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst`
    - Chain Shirt and Longsword equipment rows and any other explicitly selected equipment rows
27. `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_profs_weapon.lst`
    - Longsword proficiency concept surface

## Conditional Reads
Read only if the trigger appears:

- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_profs_armor.lst`
  - only if armor-proficiency identity must be confirmed for a selected loadout
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_race.lst`
  - only if Human trait carrier rows must be reconciled beyond the direct Human subtree files
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr__stats.lst`
  - only if ability-score or modifier-bound input obligations need direct base-stat confirmation
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr__saves.lst`
  - only if save-output categories need source-bound confirmation during boundary analysis
- `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/research-handoff.md`
  - only if the selection closure discovers an oracle-boundary implication that changes what later parity work must compare
- `/home/ubuntu/workspace/repos/codex/AGENTS.md`
  - only if a later session attempts to convert this receipt into code work; do not read it for this research task otherwise

## Source Universe / Inputs
Allowed source universe:

- GE-06 source-STC bundle under `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/`
- the PF1 Human Fighter pilot charter under `programs/codex/plans/pilot-slices/`
- GE-01 pilot-source discovery and conversion artifacts needed to ground Human/Fighter/feat/equipment surfaces
- GE-04 pilot golden-computation fixture requirements as the downstream fixture contract boundary
- the exact read-only PCGen source files listed above under `/home/ubuntu/workspace/repos/pcgen`

Do not use web research unless the local documentary and repository surfaces are insufficient, and if that happens the receipt must explicitly record why local evidence was insufficient.

## Required Output Artifact
Create exactly this artifact:

```text
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-grounded-character-selection-ledger-2026-06-20.md
```

The output artifact must include frontmatter with at least:

```yaml
title: GE06-E1-F1 Grounded Character Selection Ledger
artifact_type: research-receipt
stc_id: STC-CODEX-GE-06
source_handoff: ../research-handoff.md
selected_slice: GE06-E1-F1 — Grounded character selection ledger
status: draft
created_at: 2026-06-20
code_authority: false
```

## Required Output Artifact Contents
The selection-ledger receipt must contain these sections:

1. `# GE06-E1-F1 Grounded Character Selection Ledger`
2. `## Objective`
3. `## Sources Read`
4. `## Commands Run`
5. `## Grounded Inputs Recovered`
6. `## Selection Ledger`
7. `## Entitlement and Gate Reconciliation`
8. `## Charter Boundary Check`
9. `## Remaining Blockers`
10. `## Proposed Upstream Deltas`
11. `## Verification`
12. `## Verdict`

The `Selection Ledger` table must use these columns:

| Column | Meaning |
|---|---|
| `Ledger ID` | Stable local identifier, e.g. `GE06-SL-001`. |
| `Domain` | Race, class, level, ability scores, feat, bonus-feat entitlement, skill allocation, equipment, selector, or export boundary. |
| `Candidate / decision` | Exact selected item, unresolved candidate, or blocker statement. |
| `Exact evidence` | Specific file path, line, artifact, or command evidence. |
| `Evidence class` | `charter-grounded`, `artifact-grounded`, `legacy-source-grounded`, or `blocked`. |
| `Claim ceiling` | Must never exceed `Parsed` for this receipt. Do not claim `Computed`, `Oracle-checked`, or `Product-visible`. |
| `Status` | selected, anchored-but-not-final, blocked, or deferred-with-owner. |
| `Charter impact / blocker` | no-change, charter-patch, ADR-trigger, or the explicit blocker preventing closure. |

## Required Analytical Outcomes
The worker must answer all of the following, even if the answer is “blocked”:

1. Is `power_attack` the only grounded mandatory feat selection, or do Human and Fighter entitlements force one or more additional feat picks?
2. Does the first deterministic loadout close cleanly as `Chain Shirt + Longsword`, or are those only representative anchors rather than a final required loadout?
3. Which exact skill allocations are mandatory for the first deterministic pilot case, if any?
4. Do any Human or Fighter `ABILITYPOOL`, `CHOOSE`, `PRE*`, or class-skill surfaces force explicit additional selector closure in GE-06?
5. Does the resolved selection set stay inside the pilot charter as written, or does it require a charter patch or ADR trigger?

## Allowed Write Scope
This handoff grants write permission only to:

```text
/home/ubuntu/workspace/programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-grounded-character-selection-ledger-2026-06-20.md
```

No other file may be modified unless the operator explicitly expands scope.

## In Scope
- source-grounded closure of GE-06 fixture-selection debt
- explicit blocker promotion when closure cannot be grounded honestly
- reconciliation of charter facts against Human/Fighter feat/choice/skill/equipment surfaces
- determining whether representative equipment anchors are sufficient for the first deterministic pilot path
- recording any charter-patch or ADR-trigger implications discovered during the closure

## Out of Scope
- any code in `/home/ubuntu/workspace/repos/codex`
- any modification to `/home/ubuntu/workspace/repos/pcgen`
- editing GE-06 control documents or artifact requirements files
- creating an `execution-handoff.md`
- computing final combat/save/AC/skill values
- claiming PCGen parity or new-system correctness
- defining the GE-06 integration branch/worktree
- designing the GE-07 UI surface
- broadening the pilot beyond the current charter without explicit escalation

## Route-Specific Constraints
- Treat the charter's named `power_attack` feat as a grounded seed, not proof that all feat debt is closed.
- Treat Chain Shirt and Longsword as grounded anchors, not automatically final selections, unless the evidence closes them explicitly.
- If multiple plausible skill allocations or loadouts remain and no authority surface chooses between them, record a blocker instead of inventing a pick.
- Any choice that expands the pilot must be classified as `no-change`, `charter-patch`, or `ADR-trigger`; silent expansion is forbidden.
- This receipt may classify evidence only up to `Parsed`; it must not speak in the language of computed outputs, oracle success, or product truth.
- Do not patch source STC files from this handoff. If documentary reconciliation is needed after the receipt exists, that is a separate downstream review handoff.

## Acceptance Criteria
The handoff is complete when:
- the exact skill/equipment/additional-feat/choice surfaces are either closed with grounded evidence or promoted into named blockers
- the receipt cites exact files and lines for every selection-critical surface it uses
- every ledger row declares an evidence class and a claim ceiling capped at `Parsed`
- the receipt explicitly states whether the resulting selection set is `no-change`, `charter-patch`, or `ADR-trigger` against the pilot charter
- the receipt exists at `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-grounded-character-selection-ledger-2026-06-20.md`
- no file outside the granted write scope is modified
- no computed/parity/UI-truth claims are made

## Verification
Run or perform these checks before declaring completion:

1. Confirm the required GE-06, GE-01, GE-04, charter, and PCGen source files exist.
2. Confirm the receipt exists at the exact required path.
3. Confirm the receipt includes all required sections.
4. Confirm every `Selection Ledger` row has `Exact evidence`, `Evidence class`, `Claim ceiling`, and `Charter impact / blocker` populated.
5. Confirm no row claims above `Parsed`.
6. Confirm no other file was modified.

## Allowed Assumptions
- the PF1 Core Rulebook Human Fighter level 1 pilot remains the first proof target
- the initial ability-score vector and named `power_attack` seed remain grounded charter inputs
- GE-06 owns integrated pilot-path closure, while GE-03 / GE-04 / GE-05 continue to own importer, computation, and parity responsibilities respectively
- representative equipment anchors already grounded by GE-01 are legitimate candidates to inspect first, not final answers by default

## Blockers / Forbidden Assumptions
Stop and report if:
- a required source file is missing or empty
- the closure would require computed behavior rather than source-grounded selection evidence
- the work would need to modify STC files instead of only writing the receipt
- a missing authority surface forces the worker to invent a feat, skill, equipment, or selector decision
- a proposed “closure” depends on UI convenience, screenshots, or parity claims rather than grounded source selection evidence

## Final rule
The lesser models would have turned GE-06 into a vague integration sprint. Do not do that. This handoff exists to answer one narrower question with evidence: **what exact pilot character input contract are we actually allowed to claim?**