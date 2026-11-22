---
title: GE05-E1-F1 Handoff Readiness Closure
artifact_type: handoff-readiness-closure
stc_id: STC-CODEX-GE-05
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE05-E1-F1 — Candidate oracle route inventory
workflow_route: research
readiness: research-ready
recommended_handoff: ../research-handoff.md or ../agent-handoff.md
handoff_created: true
created_handoff: ../research-handoff.md
review_date: 2026-06-20
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE05-E1-F1 Handoff Readiness Closure

## Verdict
GE05-E1-F1 is **handoff-ready only as a non-code discovery/research handoff**.

The correct next handoff is a bounded `research-handoff.md` or `agent-handoff.md` for **candidate PCGen oracle-route discovery**. It must not be an `execution-handoff.md`, and it must not authorize production parity-harness implementation.

GE05-E1-F2 — first reproducible old-system output route — is **not ready** as a runtime-output handoff yet. The Java/runtime substrate and actual PCGen output route remain unresolved.

## Core problem
The handoff is not blocked by lack of requirements. It is blocked from becoming a runtime-output or code handoff because the next truthful work is route discovery, not parity implementation.

## Selected bounded slice

```text
GE05-E1-F1 — Candidate oracle route inventory
```

Source in `epic-breakdown.md`:

- candidate PCGen CLI, validation, export, scripting, test, and GUI routes are identified or explicitly ruled out for the pilot
- each candidate records command/path evidence, trust tier, repeatability, and limitations
- static source surfaces remain classified as source truth, not runtime parity evidence

## Why this is the first slice
GE-05 cannot honestly build golden fixtures, output normalizers, a comparator, or parity reports until the old-system oracle route is understood.

The first worker must inventory and classify PCGen oracle routes. It may inspect PCGen command-line flags, Gradle tasks, export paths, tests, source entry points, and existing documentation. It may record blockers. It must not pretend to produce oracle evidence unless it actually captures old-system runtime output.

## Grounded facts recovered by tools

| Fact | Result |
|---|---|
| Workspace date from runtime | `2026-06-20` |
| GE-05 source STC | `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/README.md` exists and is `planning-ready` |
| Selected first slice | `GE05-E1-F1 — Candidate oracle route inventory` |
| Target Codex repo | `/home/ubuntu/workspace/repos/codex` |
| Current Codex branch | `ge04-e1-f1-character-input-record-shape` |
| Correct integration base for future Codex code branches | `develop` first, not `main` |
| Current Codex local residue | untracked `AGENTS.md`, `CLAUDE.md`, `Cargo.lock`, and `target/` |
| Codex instruction surface | `/home/ubuntu/workspace/repos/codex/AGENTS.md` exists and requires explicit handoff fields before work |
| Codex Rust substrate | `cargo 1.96.0`, `rustc 1.96.0` available |
| Codex verification smoke | `cargo test --quiet`, `cargo fmt --check`, and `cargo clippy --all-targets -- -D warnings` succeeded |
| PCGen repo | `/home/ubuntu/workspace/repos/pcgen` exists on branch `master` |
| PCGen instruction surface | `/home/ubuntu/workspace/repos/pcgen/AGENTS.md` exists and documents Gradle, CLI flags, entry points, and export examples |
| PCGen Gradle wrapper | `/home/ubuntu/workspace/repos/pcgen/gradlew` exists and is executable |
| PCGen Gradle smoke | `./gradlew tasks --no-daemon --quiet` succeeded and listed runnable tasks |
| Active Java | OpenJDK `17.0.19` |
| Java 25 availability | not installed according to `update-java-alternatives -l` and `/usr/lib/jvm` scan |
| PCGen doctrine runtime requirement | repo `AGENTS.md` states Java 25 is required for 6.09 development |
| Candidate PCGen batch/export clue | `AGENTS.md` documents `--character`, `--exportsheet`, `--outputfile`, `--configfilename`, and batch export via `Main.startupWithoutGUI()` / `PcgenFtlTestCase` |

## Gate table

| Gate | Status | Resolution |
|---|---|---|
| Source STC exists | pass | GE-05 source-STC bundle exists and is planning-ready. |
| Expected output artifacts declared | pass | README declares exact artifact paths and completion rules. |
| Next bounded action selected | pass | GE05-E1-F1 is the correct first slice. |
| Slice narrower than spec domain | pass | Route inventory excludes fixture implementation, normalizer implementation, comparator implementation, parity reporting, and GE-06 integration. |
| Handoff route selected | pass | Route is `research` / discovery, not coding. |
| Target repo/workdir grounded | pass | `/home/ubuntu/workspace/repos/codex` and `/home/ubuntu/workspace/repos/pcgen` both exist. |
| Runtime instruction surfaces grounded | pass | Both Codex and PCGen `AGENTS.md` surfaces exist. |
| Branch/worktree policy explicit | conditional pass | Future code branches must start from clean current `develop`; this discovery handoff should be read-only against repos and should not use the current GE-04 branch for implementation. |
| Allowed write scope explicit | pass for discovery | Future GE05-E1-F1 handoff should write only a route-inventory receipt under this GE-05 STC bundle unless explicitly expanded. |
| Execution substrate grounded | partial pass | Codex/Rust substrate works. PCGen Gradle wrapper works for task discovery. Java 25 is absent, so runtime/build/output attempts may fail or require setup. |
| Verification commands runnable | pass for discovery | Tool-backed smoke checks succeeded for repo state, Codex Rust checks, Java version, Gradle wrapper, and Gradle task discovery. |
| Runtime-output production readiness | fail | GE05-E1-F2 is blocked until Java 25/runtime setup and a specific PCGen old-output route are grounded. |
| Non-goals explicit | pass | No implementation code, no parity claims, no GUI automation default, no broad regression suite, no fixture values invented. |

## Recommended handoff type
Use one of:

```text
research-handoff.md
agent-handoff.md
```

Do **not** use:

```text
execution-handoff.md
```

unless a later closure upgrades the selected slice to a coding route with exact branch/worktree, write scope, tests, and runnable implementation verification.

## Minimum objective for the future GE05-E1-F1 handoff

```text
Inventory candidate PCGen oracle-output routes for the PF1 Core Rulebook Human Fighter level 1 pilot, classify each route by trust tier and repeatability, identify the lowest-friction next route to attempt, and record blockers without claiming parity or producing fabricated oracle evidence.
```

## Required reads for the future handoff
- `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/README.md`
- `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/technical-requirements.md`
- `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/technical-design.md`
- `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/epic-breakdown.md`
- `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/oracle-strategy-specification-requirements.md`
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/references/oracle-surface-inventory.md`
- `programs/codex/plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md`
- `/home/ubuntu/workspace/repos/codex/AGENTS.md`
- `/home/ubuntu/workspace/repos/pcgen/AGENTS.md`
- `/home/ubuntu/workspace/repos/pcgen/code/src/java/pcgen/system/CommandLineArguments.java`
- `/home/ubuntu/workspace/repos/pcgen/code/src/java/pcgen/system/Main.java`
- `/home/ubuntu/workspace/repos/pcgen/code/src/slowtest/pcgen/inttest/PcgenFtlTestCase.java`

## Allowed write scope for the future GE05-E1-F1 discovery handoff
The future discovery handoff should write only:

```text
programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e1-f1-oracle-route-inventory-2026-06-20.md
```

If it discovers facts that must propagate to GE-01, GE-03, GE-04, or doctrine decisions, it should record proposed deltas inside that receipt unless the handoff explicitly grants those additional write paths.

## Required output artifact for the future GE05-E1-F1 handoff

```text
programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e1-f1-oracle-route-inventory-2026-06-20.md
```

Minimum contents:

- route inventory table
- command/path/source evidence per route
- route class: CLI, Gradle task, validation, export, script/test, GUI, static source, docs, code path
- trust tier: static source truth, runtime behavior evidence, GUI-derived evidence, unknown/ungrounded
- repeatability notes
- Java/runtime prerequisites
- whether route can support `pf1-crb-human-fighter-level1`
- whether route is suitable for GE05-E1-F2
- blockers and next probe recommendation

## Verification commands proven in this readiness pass

```bash
date +%F
git -C /home/ubuntu/workspace/repos/codex rev-parse --abbrev-ref HEAD
git -C /home/ubuntu/workspace/repos/codex branch --all --no-color
git -C /home/ubuntu/workspace/repos/codex status --short
cargo test --quiet
cargo fmt --check
cargo clippy --all-targets -- -D warnings
java -version
update-java-alternatives -l
cd /home/ubuntu/workspace/repos/pcgen && ./gradlew --version --no-daemon
cd /home/ubuntu/workspace/repos/pcgen && ./gradlew tasks --no-daemon --quiet
```

Observed outcomes:

- Codex Rust verification passed.
- PCGen Gradle wrapper and task discovery passed.
- Active Java is 17, not 25.
- Java 25 is absent from installed JVM alternatives.

## Hard blockers for a stronger handoff
A runtime-output or code-authorizing GE05-E1-F2+ handoff remains blocked until:

1. Java 25 or an accepted PCGen runtime setup is available and grounded.
2. The handoff chooses a specific old-output route to attempt.
3. The handoff states whether it may generate build/runtime artifacts inside `/home/ubuntu/workspace/repos/pcgen` or must remain read-only.
4. The handoff states exact output receipt paths.
5. The handoff states whether any Codex repo code changes are allowed. For GE05-E1-F1, they are not.
6. Any future Codex code branch starts from clean current `develop`, not the current GE-04 feature branch.

## Stop conditions for the future discovery worker
Stop and report rather than improvising if:

- a route requires Java 25 and Java 25 is still unavailable
- a route requires modifying `/home/ubuntu/workspace/repos/pcgen`
- a route requires writing Codex implementation code
- GUI driving becomes the only apparent route
- the worker cannot distinguish static source truth from runtime behavior evidence
- the worker is tempted to fill final expected Human Fighter values manually

## Result
GE05-E1-F1 is **research-ready / discovery-handoff-ready**.

GE-05 remains **not code-ready** and no `execution-handoff.md` should be created from this closure. The route-correct discovery handoff now exists at:

```text
../research-handoff.md
```

That handoff's only required downstream output is the single route-inventory receipt.
