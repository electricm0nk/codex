---
title: GE05-E1-F1 Research Handoff — Candidate PCGen Oracle Route Inventory
handoff_id: HANDOFF-CODEX-GE-05-E1-F1-RESEARCH-2026-06-20
handoff_kind: research-brief
work_type: research-only
workflow_route: research
readiness: research-ready
status: active
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/research-handoff.md
source_stc: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/README.md
source_readiness: planning-ready
readiness_closure: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e1-f1-handoff-readiness-closure-2026-06-20.md
selected_slice: GE05-E1-F1 — Candidate oracle route inventory
created_at: 2026-06-20
code_authority: false
---

# Research Handoff: GE05-E1-F1 — Candidate PCGen Oracle Route Inventory

## Objective
Inventory candidate PCGen oracle-output routes for the PF1 Core Rulebook Human Fighter level 1 pilot, classify each route by trust tier and repeatability, identify the lowest-friction next route to attempt, and record blockers without claiming parity or producing fabricated oracle evidence.

## Work Type
`research-only`

This is bounded discovery and documentary research. It may inspect repositories and run non-mutating discovery commands, but it does not authorize implementation code, parity-harness code, PCGen modifications, fixture values, or runtime-output claims.

## Workflow Route
`research`

## Readiness
`research-ready`

Why this handoff is ready:
- the GE-05 source STC is `planning-ready`
- `artifacts/ge05-e1-f1-handoff-readiness-closure-2026-06-20.md` establishes GE05-E1-F1 as the correct first bounded slice
- the selected slice is narrower than the full parity harness
- the required source universe and output receipt path are explicit
- the work can proceed despite Java 25 being absent because the task is route inventory, not runtime-output production

Why this handoff is **not** code-ready:
- `code_authority: false`
- GE-05 remains `planning-ready`, not `implementation-ready`
- this handoff is `research-only`, not `implementation-ready`
- the workflow route is `research`, not `coding`
- no Codex implementation write scope is granted
- no PCGen modification write scope is granted
- Java/runtime facts block GE05-E1-F2 runtime-output production until a later closure

## Source STC
- path: `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/README.md`
- source readiness: `planning-ready`
- source authority: oracle-validation and parity-harness requirements for Codex GE-05

## Readiness Closure
- path: `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e1-f1-handoff-readiness-closure-2026-06-20.md`
- verdict: GE05-E1-F1 is ready only as a non-code discovery/research handoff
- blocked stronger route: GE05-E1-F2 runtime-output handoff is blocked until Java/runtime and a specific PCGen output route are grounded

## Downstream Target
- harness or workflow: `fresh Hermes/God-Emperor session, research-capable agent, or equivalent non-code discovery worker`
- invocation mode: direct session or agent run using this handoff as the primary brief
- target workspace: `/home/ubuntu/workspace`

## Runtime / Repository Context
These are context facts for discovery, not write authority:

| Surface | Path | Posture |
|---|---|---|
| Codex program docs | `/home/ubuntu/workspace/programs/codex` | writable only where this handoff grants exact artifact output |
| GE-05 STC bundle | `/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness` | only the required output receipt may be created by this handoff |
| Codex implementation repo | `/home/ubuntu/workspace/repos/codex` | read-only for this handoff |
| PCGen legacy repo | `/home/ubuntu/workspace/repos/pcgen` | read-only for this handoff |

Grounded state from the readiness closure:
- Codex current branch observed: `ge04-e1-f1-character-input-record-shape`
- future Codex code branches must start from clean current `develop`, but this handoff grants no Codex code branch work
- PCGen branch observed: `master`
- active Java observed: OpenJDK `17.0.19`
- Java 25 observed: not installed in JVM alternatives or `/usr/lib/jvm`
- PCGen repo doctrine states Java 25 is required for 6.09 development
- PCGen Gradle wrapper exists and task discovery succeeded

## Required Reads
Read these before producing the inventory receipt:

1. `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/README.md`
   - GE-05 authority, non-code boundary, required reads, and parity-claim prohibitions
2. `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/technical-requirements.md`
   - normative oracle-runner, fixture, normalization, comparison, report, and known-gap requirements
3. `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/technical-design.md`
   - conceptual parity-harness pipeline and component boundaries
4. `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/epic-breakdown.md`
   - GE05-E1 sequencing and first handoff-readiness result
5. `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/oracle-strategy-specification-requirements.md`
   - trust-tier and evidence-boundary requirements
6. `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e1-f1-handoff-readiness-closure-2026-06-20.md`
   - grounded readiness facts and blockers
7. `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/references/oracle-surface-inventory.md`
   - existing grounded and candidate PCGen oracle surfaces
8. `programs/codex/plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md`
   - pilot case boundary and initial expected categories
9. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
   - confirms Codex repo conduct surface and that no implementation begins without explicit code handoff
10. `/home/ubuntu/workspace/repos/pcgen/AGENTS.md`
    - PCGen build/tooling doctrine, CLI flags, batch export clue, and Java 25 requirement
11. `/home/ubuntu/workspace/repos/pcgen/code/src/java/pcgen/system/CommandLineArguments.java`
    - command-line flag parser and old-system CLI surface
12. `/home/ubuntu/workspace/repos/pcgen/code/src/java/pcgen/system/Main.java`
    - startup and batch/non-GUI behavior path
13. `/home/ubuntu/workspace/repos/pcgen/code/src/slowtest/pcgen/inttest/PcgenFtlTestCase.java`
    - existing batch/export-style test evidence and route clues

## Conditional Reads
Read only if the trigger appears:

- `/home/ubuntu/workspace/repos/pcgen/code/src/test/pcgen/system/CommandLineArgumentsTest.java`
  - only if CLI flag semantics need test-backed confirmation
- `/home/ubuntu/workspace/repos/pcgen/build.gradle`
  - only if route inventory needs Gradle application/run/task wiring evidence
- `/home/ubuntu/workspace/repos/pcgen/settings.gradle`
  - only if Gradle subproject routing or task ownership is unclear
- `/home/ubuntu/workspace/repos/pcgen/code/gradle/distribution.gradle`
  - only if output sheets, runtime asset packaging, or distribution output paths affect a candidate route
- `/home/ubuntu/workspace/repos/pcgen/outputsheets/`
  - only if export-sheet route feasibility needs available sheet inventory
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc`
  - only if a candidate route must cite the exact Core Rulebook campaign/source input
- `/home/ubuntu/workspace/repos/pcgen/docs/listfilepages/listfileimportanttoknow.html`
  - only if a static documentation route needs token/documentation evidence
- `programs/codex/doctrine/decisions/`
  - only if the research discovers a route that implies intentional divergence or GUI-driving policy acceptance

## Source Universe / Inputs
Allowed source universe:

- GE-05 source STC bundle under `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/`
- GE-01 oracle-surface inventory and pilot-source discovery artifacts
- PF1 Human Fighter pilot charter
- PCGen repository documentation, command-line parser, startup path, Gradle tasks, tests, output sheets, and source files under `/home/ubuntu/workspace/repos/pcgen`
- Codex repo instruction surface under `/home/ubuntu/workspace/repos/codex/AGENTS.md` only to preserve no-code boundary and future branch discipline

Do not use web research unless the local repository surfaces are insufficient and the receipt explicitly records why local evidence was insufficient.

## Required Output Artifact
Create exactly this artifact:

```text
programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e1-f1-oracle-route-inventory-2026-06-20.md
```

The output artifact must include frontmatter with at least:

```yaml
title: GE05-E1-F1 Oracle Route Inventory
artifact_type: research-receipt
stc_id: STC-CODEX-GE-05
source_handoff: ../research-handoff.md
selected_slice: GE05-E1-F1 — Candidate oracle route inventory
status: draft
created_at: 2026-06-20
code_authority: false
```

## Required Output Artifact Contents
The route-inventory receipt must contain these sections:

1. `# GE05-E1-F1 Oracle Route Inventory`
2. `## Objective`
3. `## Sources Read`
4. `## Commands Run`
5. `## Runtime Facts`
6. `## Route Inventory`
7. `## Route Classifications`
8. `## Recommended Next Probe`
9. `## Blockers`
10. `## Proposed Upstream Deltas`
11. `## Verification`
12. `## Verdict`

The `Route Inventory` table must use these columns:

| Column | Meaning |
|---|---|
| `Route ID` | Stable local route ID, e.g. `GE05-OR-001`. |
| `Route name` | Human-readable candidate route. |
| `Route class` | CLI, Gradle task, validation, export, script/test, GUI, static source, docs, code path, or unknown. |
| `Evidence` | Exact path, command, test, source line reference, or receipt. |
| `Trust tier` | Static source truth, runtime behavior evidence, GUI-derived evidence, unknown/ungrounded. |
| `Repeatability` | High, medium, low, or blocked, with reason. |
| `Prerequisites` | Java/runtime/build/config/fixture requirements. |
| `Pilot suitability` | Whether it can support `pf1-crb-human-fighter-level1`. |
| `GE05-E1-F2 suitability` | Whether it is suitable for the next runtime-output attempt. |
| `Blockers / next probe` | Exact blocker or recommended next action. |

## Route Classes to Investigate
The worker must evaluate at least these candidate route classes, even if the answer is “blocked” or “not suitable”:

1. PCGen CLI flags / batch export path
2. Gradle `run` or application task path
3. Existing PCGen slow/integration tests, especially `PcgenFtlTestCase`
4. Output sheet/export route
5. Data validation or character integration tasks such as `datatest`, `pfinttest`, `inttest`, or related task families
6. Static PCC/LST source inspection as source truth, not runtime behavior evidence
7. PCGen documentation/listfile docs as semantic documentation evidence
8. PCGen code path inspection for command/startup/export behavior
9. GUI route as last resort only

## Allowed Write Scope
This handoff grants write permission only to:

```text
/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e1-f1-oracle-route-inventory-2026-06-20.md
```

No other file may be modified unless the operator explicitly expands scope.

## Output Placement Rules
- Place the route inventory receipt exactly at the required output path.
- Do not create a GE-05 `execution-handoff.md`.
- Do not create Codex repo implementation files.
- Do not create PCGen repo files.
- Do not place receipt artifacts under `/home/ubuntu/workspace/repos/codex` or `/home/ubuntu/workspace/repos/pcgen`.
- If additional propagation is needed, record proposed deltas inside the route-inventory receipt rather than modifying other STCs directly.

## In Scope
- read-only inspection of PCGen CLI, Gradle, startup, export, validation, test, documentation, and source surfaces
- read-only inspection of Codex/GE-05 documentation surfaces
- command/task discovery that does not modify source files
- Java/runtime prerequisite documentation
- classification of candidate oracle routes by trust tier and repeatability
- recommendation of the next route to attempt for GE05-E1-F2
- writing the single route-inventory receipt artifact

## Out of Scope
- writing implementation code in `/home/ubuntu/workspace/repos/codex`
- modifying `/home/ubuntu/workspace/repos/pcgen`
- installing Java 25 or changing system Java alternatives
- creating or modifying PCGen character files, output sheets, data files, or build scripts
- producing final Human Fighter expected values
- claiming old-system runtime oracle evidence unless the worker actually captures it and records the command/output receipt
- claiming Codex parity or oracle-checked behavior
- building fixture schema, normalizer, comparator, parity report writer, or known-gap ledger implementation
- GUI automation except as a classified last-resort candidate route
- broad Pathfinder or full PCGen regression-suite design

## Route-Specific Constraints
- Treat this as discovery before implementation.
- Prefer headless, CLI, validation, export, script, or test routes over GUI routes.
- Distinguish static source truth from runtime behavior evidence.
- Record exact evidence for each candidate route.
- Record blockers explicitly rather than filling gaps with plausible assumptions.
- If Java 25 absence prevents runtime probing, classify the route as blocked/configuration-dependent rather than forcing it.
- If a command may generate substantial build output, record that risk before running it; do not run build-heavy commands unless they are necessary for discovery and do not modify source-controlled files.

## Acceptance Criteria
The handoff is complete when:

- `artifacts/ge05-e1-f1-oracle-route-inventory-2026-06-20.md` exists
- the receipt has frontmatter and `stc_id: STC-CODEX-GE-05`
- every required route class has an entry or an explicit blocked/not-suitable finding
- every route entry cites exact local evidence: path, command, test, source file, or documented observation
- the receipt distinguishes static source truth, documentation/code-path evidence, runtime behavior evidence, GUI-derived evidence, and unknown/ungrounded routes
- Java/runtime prerequisites are recorded accurately
- the receipt names the best next GE05-E1-F2 probe or states why no runtime-output probe is ready
- no code-authorizing handoff is created
- no repo implementation files are modified
- no final expected Human Fighter values or parity claims are invented

## Verification
Run or perform these checks before declaring completion:

1. Confirm the receipt exists:

```bash
test -f /home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e1-f1-oracle-route-inventory-2026-06-20.md
```

2. Confirm the receipt contains required markers:

```bash
python3 - <<'PY'
from pathlib import Path
p = Path('/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e1-f1-oracle-route-inventory-2026-06-20.md')
text = p.read_text()
for needle in [
    'stc_id: STC-CODEX-GE-05',
    'artifact_type: research-receipt',
    '## Route Inventory',
    '## Recommended Next Probe',
    '## Blockers',
    '## Verdict',
]:
    assert needle in text, needle
PY
```

3. Confirm GE-05 still has no code-authorizing execution handoff:

```bash
test ! -f /home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/execution-handoff.md
```

4. Confirm no implementation repo source files were modified by this handoff:

```bash
git -C /home/ubuntu/workspace/repos/codex status --short
git -C /home/ubuntu/workspace/repos/pcgen status --short
```

Known pre-existing Codex residue from the readiness closure may still appear:

```text
?? AGENTS.md
?? CLAUDE.md
?? Cargo.lock
?? target/
```

Do not treat that residue as permission to modify it.

## Suggested Discovery Commands
These commands are permitted if the worker judges them necessary and records the outputs in the receipt:

```bash
java -version
update-java-alternatives -l
cd /home/ubuntu/workspace/repos/pcgen && ./gradlew --version --no-daemon
cd /home/ubuntu/workspace/repos/pcgen && ./gradlew tasks --no-daemon --quiet
```

Additional source inspection should use read/search tools or equivalent safe read-only commands. Prefer narrow searches over broad log dumps.

Do not run heavyweight PCGen build/test/export commands unless the worker first records why the command is needed and why it is safe under the read-only/no-source-modification boundary.

## Allowed Assumptions
- The PF1 Core Rulebook Human Fighter level 1 pilot remains the first oracle-route target.
- PCGen is the legacy oracle substrate, not the Codex architecture.
- Static PCGen source files can support source-truth findings but cannot prove runtime parity by themselves.
- Java 25 absence may block runtime-output attempts but does not block route inventory.
- The route inventory may recommend a setup step or future runtime probe without performing it.

## Blockers / Forbidden Assumptions
Stop and report if:

- a route requires Java 25 and Java 25 is still unavailable
- a candidate route would require modifying `/home/ubuntu/workspace/repos/pcgen`
- a candidate route would require writing Codex implementation code
- the worker cannot determine whether a command is read-only or source-modifying
- GUI driving becomes the only apparent route and no decision record authorizes it
- the worker cannot distinguish static source truth from runtime behavior evidence

Forbidden assumptions:
- do not assume Java 17 is acceptable for PCGen runtime output merely because Gradle task discovery worked
- do not assume PCGen source file inspection proves runtime oracle behavior
- do not assume a CLI/export route works until command evidence exists
- do not invent final expected Human Fighter values
- do not claim parity, `Oracle-checked`, or product-visible correctness
- do not create `execution-handoff.md`

## Anti-Waste Instruction for the Downstream Agent
Do not produce a generic plan. Produce the required route-inventory receipt.

The purpose of this run is not to solve parity. The purpose is to identify the exact route by which parity evidence can later be attempted, or to prove why no such route is ready yet.

If the route is blocked by Java/runtime setup, say so directly and recommend the smallest next setup/probe step. Do not work around the blocker by hand-writing expected output values.

## Completion Report Expected From Downstream Agent
When finished, report:

- receipt path written
- route classes evaluated
- best next route to attempt
- Java/runtime blocker status
- commands run
- files read
- verification result
- whether GE05-E1-F2 is now ready or remains blocked
