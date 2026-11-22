---
title: GE05-E1-F1 Oracle Route Inventory
artifact_type: research-receipt
stc_id: STC-CODEX-GE-05
source_handoff: ../research-handoff.md
selected_slice: GE05-E1-F1 — Candidate oracle route inventory
status: draft
created_at: 2026-06-20
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE05-E1-F1 Oracle Route Inventory

## Objective
Inventory candidate PCGen oracle-output routes for the `pf1-crb-human-fighter-level1` pilot, classify each route by trust tier and repeatability, identify the lowest-friction next route to attempt, and record blockers without claiming parity, producing fixture values, or authorizing code work.

This receipt executes `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/research-handoff.md` only. It does not create `execution-handoff.md`, does not modify `/home/ubuntu/workspace/repos/codex`, does not modify `/home/ubuntu/workspace/repos/pcgen`, and does not produce old-system runtime oracle output.

## Sources Read

### GE-05 and pilot authority
- `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/research-handoff.md` — required output path, required sections, route classes, write scope, stop conditions, and verification rules.
- `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/README.md` — GE-05 source STC boundary, no-code authority, active research handoff, and required artifact map.
- `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e1-f1-handoff-readiness-closure-2026-06-20.md` — readiness verdict, Java 25 blocker, and GE05-E1-F1 discovery-only posture.
- `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/technical-requirements.md` — oracle discovery, reproducible invocation, PCGen output capture, normalization, comparison, and claim-tier requirements.
- `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/technical-design.md` — conceptual old/new capture pipeline and deferred final PCGen route.
- `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/oracle-strategy-specification-requirements.md` — oracle surface classes and prohibited shortcuts.
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/references/oracle-surface-inventory.md` — static source surfaces already grounded and runtime-output surfaces still not grounded by GE-01.
- `programs/codex/plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md` — first pilot case identity and expected comparison categories.

### Repository conduct surfaces
- `/home/ubuntu/workspace/repos/codex/AGENTS.md` — confirms no implementation should begin without an explicit execution handoff and allowed write scope.
- `/home/ubuntu/workspace/repos/pcgen/AGENTS.md` — Java 25 requirement, Gradle commands, CLI flags, batch export clue, local run examples, and useful PCGen paths.

### PCGen command/startup/export/test surfaces
- `/home/ubuntu/workspace/repos/pcgen/code/src/java/pcgen/system/CommandLineArguments.java` — CLI parser for `--settingsdir`, `--configfilename`, `--campaignmode`, `--exportsheet`, `--outputfile`, `--character`, and `--party`.
- `/home/ubuntu/workspace/repos/pcgen/code/src/java/pcgen/system/Main.java` — startup split between GUI and `startupWithoutGUI()`, batch export selection, bootstrap tasks, and shutdown.
- `/home/ubuntu/workspace/repos/pcgen/code/src/java/pcgen/system/BatchExporter.java` — character/party export implementation and template handling.
- `/home/ubuntu/workspace/repos/pcgen/code/src/test/pcgen/system/CommandLineArgumentsTest.java` — test-backed CLI argument expectations.
- `/home/ubuntu/workspace/repos/pcgen/code/src/slowtest/pcgen/inttest/PcgenFtlTestCase.java` — reusable test route that creates settings, calls `Main.main(...)`, exports XML, and compares expected XML.
- `/home/ubuntu/workspace/repos/pcgen/code/src/slowtest/pcgen/util/TestHelper.java` — data-folder discovery and test settings-file creation used by the FTL route.
- `/home/ubuntu/workspace/repos/pcgen/code/src/slowtest/pcgen/inttest/game_pathfinder/pcGenGUIPfrpgClericTest.java` — Pathfinder FTL integration-test route for `pf_Cleric`.
- `/home/ubuntu/workspace/repos/pcgen/code/src/slowtest/pcgen/inttest/game_pathfinder/pcGenGUIPfrpgPaladinTest.java` — Pathfinder FTL integration-test route for `pf_Paladin`.
- `/home/ubuntu/workspace/repos/pcgen/code/src/slowtest/pcgen/inttest/game_pathfinder/pcGenGUIPfrpgRogueTest.java` — Pathfinder FTL integration-test route for `pf_Rogue`.
- `/home/ubuntu/workspace/repos/pcgen/code/src/slowtest/pcgen/inttest/game_pathfinder/pcGenGUIPfrpgGoldielocksTest.java` — Pathfinder FTL integration-test route for `pf_goldielocks`.
- `/home/ubuntu/workspace/repos/pcgen/build.gradle` — Java 25 toolchain declaration, `application.mainClass`, source sets, `datatest`, `inttest`, per-game `pfinttest`, test JVM args, and build/task routing.
- `/home/ubuntu/workspace/repos/pcgen/outputsheets/` and `/home/ubuntu/workspace/repos/pcgen/code/testsuite/base-xml.ftl` — available output-sheet/export-template surfaces.
- `/home/ubuntu/workspace/repos/pcgen/code/testsuite/PCGfiles/` and `/home/ubuntu/workspace/repos/pcgen/code/testsuite/csheets/` — existing PCG/XML integration fixtures.

## Commands Run

All commands were read-only with respect to source-controlled PCGen/Codex implementation files. No PCGen build, export, or heavyweight test command was run because Java 25 is absent and the handoff prohibits forcing runtime-output attempts through an ungrounded runtime.

```bash
date +%F
java -version
update-java-alternatives -l
python3 - <<'PY'
from pathlib import Path
for p in sorted(Path('/usr/lib/jvm').glob('*')):
    print(p)
PY
cd /home/ubuntu/workspace/repos/pcgen && ./gradlew --version --no-daemon
cd /home/ubuntu/workspace/repos/pcgen && ./gradlew tasks --no-daemon --quiet
cd /home/ubuntu/workspace/repos/pcgen && ./gradlew tasks --all --no-daemon --quiet
git -C /home/ubuntu/workspace/repos/codex status --short
git -C /home/ubuntu/workspace/repos/pcgen status --short
```

Observed command results:

- `date +%F` returned `2026-06-20`.
- `java -version` returned OpenJDK `17.0.19`.
- `update-java-alternatives -l` listed only `java-1.17.0-openjdk-amd64`.
- `/usr/lib/jvm` contained Java 17 entries only: `.java-1.17.0-openjdk-amd64.jinfo`, `java-1.17.0-openjdk-amd64`, and `java-17-openjdk-amd64`.
- `./gradlew --version --no-daemon` succeeded and reported Gradle `9.5.1`, launcher JVM `17.0.19`, and daemon JVM `/usr/lib/jvm/java-17-openjdk-amd64`.
- `./gradlew tasks --no-daemon --quiet` and `./gradlew tasks --all --no-daemon --quiet` confirmed task discovery works under the current runtime.
- Relevant task names discovered include `run`, `build`, `test`, `itestClasses`, `slowtestClasses`, `testClasses`, `buildDist`, `qbuild`, `testCoverage`, `allTasks`, `datatest`, `inttest`, `pfinttest`, `sfinttest`, `rsrdinttest`, `srdinttest`, and `msrdinttest`.
- `git -C /home/ubuntu/workspace/repos/codex status --short` showed pre-existing untracked residue: `AGENTS.md`, `CLAUDE.md`, `Cargo.lock`, and `target/`.
- `git -C /home/ubuntu/workspace/repos/pcgen status --short` returned no changes.

## Runtime Facts

| Fact | Result | Evidence |
|---|---|---|
| Workspace date | `2026-06-20` | `date +%F` command output. |
| Active Java | OpenJDK `17.0.19` | `java -version` command output. |
| Installed Java alternatives | Java 17 only | `update-java-alternatives -l` and `/usr/lib/jvm` scan. |
| PCGen documented Java requirement | Java 25 / Temurin | `/home/ubuntu/workspace/repos/pcgen/AGENTS.md` lines 10-13 and 42-45; `/home/ubuntu/workspace/repos/pcgen/build.gradle` lines 63-90 declare `javaVersion = 25` and toolchain `JavaLanguageVersion.of(javaVersion)`. |
| Gradle wrapper discovery | Works for version/task discovery under Java 17 | `./gradlew --version --no-daemon`, `./gradlew tasks --no-daemon --quiet`, and `./gradlew tasks --all --no-daemon --quiet`. |
| PCGen source repo status | Clean | `git -C /home/ubuntu/workspace/repos/pcgen status --short` returned no entries. |
| Codex repo status | Pre-existing untracked residue | `git -C /home/ubuntu/workspace/repos/codex status --short`; this receipt did not modify the repo. |
| Existing exact pilot PCG fixture | Not found in discovered PCGen tests | Existing Pathfinder fixtures are `pf_Cleric`, `pf_Paladin`, `pf_Rogue`, and `pf_goldielocks`; none is the exact `pf1-crb-human-fighter-level1` case. |
| Runtime oracle output captured in this pass | No | Intentionally not run because Java 25 is absent and the handoff is route inventory only. |

## Route Inventory

| Route ID | Route name | Route class | Evidence | Trust tier | Repeatability | Prerequisites | Pilot suitability | GE05-E1-F2 suitability | Blockers / next probe |
|---|---|---|---|---|---|---|---|---|---|
| GE05-OR-001 | Direct PCGen CLI batch export with `--character`, `--exportsheet`, `--outputfile`, `--configfilename` | CLI / export | `/home/ubuntu/workspace/repos/pcgen/AGENTS.md` lines 75-88 and 210-218; `CommandLineArguments.java` lines 62-80 and 112-129; `Main.java` lines 134-141 and 339-363; `BatchExporter.java` lines 104-145 and 267-286. | Code-path evidence; becomes runtime behavior evidence only after successful command/output capture. | High after runtime is fixed and a pilot PCG exists; blocked today. | Java 25 or accepted PCGen runtime setup; built runnable PCGen classpath/JAR or Gradle `run`; export sheet; config file/settings dir; exact pilot `.pcg`. | Strong candidate for `pf1-crb-human-fighter-level1` once a deterministic pilot `.pcg` is authorized and created or located. | Best candidate route for the next runtime-output attempt, but not ready today. | Do not run under Java 17. Next probe: provision/ground Java 25, build or run via Gradle, create/locate exact pilot `.pcg`, then attempt a single batch XML export to an explicit receipt path. |
| GE05-OR-002 | Gradle `run --args="..."` wrapper around CLI batch export | Gradle task / CLI / export | `/home/ubuntu/workspace/repos/pcgen/AGENTS.md` lines 60, 72-73, and 217-218; `build.gradle` lines 94-96 declare `application.mainClass = pcgen.system.Main`; command discovery confirmed `run - Runs this project as a JVM application`. | Code-path/task evidence; becomes runtime behavior evidence only after successful `gradlew run` export. | Medium-high after runtime is fixed; Gradle may create build artifacts but should not modify source files. | Java 25 toolchain; Gradle dependencies; same CLI prerequisites as GE05-OR-001. | Suitable if exact pilot `.pcg` exists and `run --args` can pass the export flags. | Suitable as the practical invocation wrapper for GE05-OR-001. | Java 25 absent. Next probe should decide whether generated build output under PCGen is allowed before running. |
| GE05-OR-003 | Existing `PcgenFtlTestCase` batch export integration-test route | Script/test / export | `PcgenFtlTestCase.java` lines 71-101 creates settings, calls `Main.main("--character", ..., "--exportsheet", "code/testsuite/base-xml.ftl", "--outputfile", ..., "--configfilename", "config.ini.junit")`, then compares XML; `TestHelper.java` lines 398-441 finds data and writes the test config; Pathfinder tests call `runTest(...)`. | Runtime behavior evidence when the test is actually run; in this pass, code-path/test evidence only. | High for existing fixtures after runtime is fixed; blocked today. | Java 25; Gradle slowtest/pfinttest runtime; existing or authorized pilot PCG/XML fixture. | Not exact today because no `pf1-crb-human-fighter-level1` fixture was found; useful pattern for authorizing a future exact pilot test. | Strong future E1-F2 route if the next handoff authorizes a one-case test fixture or selected existing fixture proxy. | Java 25 absent; exact pilot fixture absent; this handoff does not authorize PCGen test/fixture creation. |
| GE05-OR-004 | Existing Pathfinder integration tests: `pfinttest` / `inttest` | Gradle task / script/test | `build.gradle` lines 744-770 define `inttest` and per-game `pfinttest`; Pathfinder test files call `runTest("pf_Cleric", "Pathfinder_RPG")`, `runTest("pf_Paladin", "Pathfinder_RPG")`, `runTest("pf_Rogue", "Pathfinder_RPG")`, and `runTest("pf_goldielocks", "Pathfinder_RPG")`; `AGENTS.md` lines 52-55 list `pfinttest` and `inttest`. | Runtime behavior evidence only if the tests are run and output is captured; currently task/test evidence. | Medium; tests are repeatable but broader than one pilot and may be heavier than needed. | Java 25; Gradle test runtime; existing test fixtures. | Partial. Existing Pathfinder cases include Human Cleric and Human Paladin, but not the pilot Human Fighter level 1 case. | Useful as validation/regression route, not the clean first E1-F2 probe unless narrowed by test filter or exact fixture. | Do not run broad tests under Java 17. Future probe should prefer a single filtered test or direct CLI export before broad `pfinttest`. |
| GE05-OR-005 | PCGen output sheet / export template route | Export / static template | `/home/ubuntu/workspace/repos/pcgen/code/testsuite/base-xml.ftl` exists; `/home/ubuntu/workspace/repos/pcgen/outputsheets/base.xml.ftl` exists; output-sheet search found 27 `.ftl` files including Pathfinder/fantasy HTML/XML templates; `BatchExporter.java` lines 267-286 writes non-PDF output; lines 525-535 locate default XML template. | Static source/template truth; output becomes runtime evidence only when applied by an export command/test. | High once paired with GE05-OR-001/002/003. | Export route, character, template, and runtime. | Suitable for the required “one exportable character summary/stat block” boundary if the output dimensions are selected explicitly. | Suitable as the output format component, not a standalone runtime route. | Select the minimal XML/export template for future E1-F2 and record retention/legal policy before storing derived output. |
| GE05-OR-006 | PCGen data validation route: `datatest` | Validation / Gradle task | `/home/ubuntu/workspace/repos/pcgen/AGENTS.md` line 52; `build.gradle` lines 735-742 define `datatest` over `DataTest` and `DataLoadTest`. | Runtime validation evidence for data loading, not character-output parity evidence. | Medium after Java 25; task is known but broader than pilot. | Java 25; Gradle test runtime; PCGen data tree. | Supports source/campaign loading confidence but does not produce the Human Fighter character output. | Not sufficient for GE05-E1-F2 by itself. | Use later as supporting evidence only. Do not substitute data-test pass for oracle output. |
| GE05-OR-007 | Static PCC/LST source inspection for Core Rulebook / Human / Fighter surfaces | Static source | GE-01 oracle-surface inventory lines 8-24; Core Rulebook campaign path `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc`; pilot charter lines 56-73. | Static source truth. | High; read-only and repeatable. | None beyond local checkout. | Supports identifying campaign/source package and pilot-relevant data files. | Not suitable for E1-F2 as runtime output. | Continue using for input grounding only. Must not be treated as oracle-checked runtime behavior. |
| GE05-OR-008 | PCGen listfile documentation route | Docs | GE-01 oracle-surface inventory lines 20-21 cites listfile docs including `/home/ubuntu/workspace/repos/pcgen/docs/listfilepages/listfileimportanttoknow.html`; GE-05 oracle strategy classifies docs as semantic documentation evidence. | Documentation evidence; not runtime behavior evidence. | High for cited docs; semantic trust is medium until exact sections are cited. | Exact doc sections for specific tokens. | Useful for interpreting tokens and source semantics around the pilot. | Not suitable for E1-F2 by itself. | Use only to explain/triage differences. Do not upgrade docs to runtime parity evidence. |
| GE05-OR-009 | PCGen code-path inspection for command/startup/export behavior | Code path | `CommandLineArguments.java`, `Main.java`, `BatchExporter.java`, `PcgenFtlTestCase.java`, `TestHelper.java`, and `build.gradle` line references above. | Code-path evidence; not runtime behavior evidence. | High as explanatory evidence. | Local checkout. | Strongly supports choosing the lowest-friction route. | Not sufficient alone for E1-F2; must be followed by actual runtime output after Java 25 is available. | Use as basis for the next handoff’s exact command and preconditions. |
| GE05-OR-010 | GUI startup and manual export route | GUI | `Main.java` lines 180-197 show GUI startup; `/home/ubuntu/workspace/repos/pcgen/AGENTS.md` lines 210-213 list `./gradlew run` to launch GUI. | GUI-derived evidence only if manually captured; currently code/docs evidence. | Low in this environment; least preferred and harder to automate/audit. | Java 25, GUI-capable environment, explicit decision/risk acceptance. | Last resort only. Could theoretically export a character, but not preferable. | Not suitable as first E1-F2 probe unless all headless routes fail and a decision record authorizes GUI-derived evidence. | Do not pursue now. Headless CLI/test routes exist and are better candidates. |
| GE05-OR-011 | Build/distribution route: `qbuild`, `buildDist`, runnable JAR | Gradle task / build route | `AGENTS.md` lines 58-61; `build.gradle` lines 513-530 copy build outputs; task discovery found `qbuild`, `buildDist`, `assembleDist`, `distZip`, and `installDist`. | Build-route evidence; not oracle output. | Medium after Java 25; may create substantial generated artifacts. | Java 25; permission to generate PCGen build artifacts; dependency resolution. | Supports producing a runnable artifact for GE05-OR-001, but does not itself define the pilot output. | Supporting route only. | Future handoff must authorize build output generation before using this route. |

## Route Classifications

| Route class required by handoff | Evaluated route IDs | Classification result |
|---|---|---|
| PCGen CLI flags / batch export path | GE05-OR-001 | Best overall candidate, blocked by Java 25 and missing exact pilot `.pcg`. |
| Gradle `run` or application task path | GE05-OR-002 | Good wrapper around the CLI route; likely practical once Java 25 is grounded. |
| Existing PCGen slow/integration tests, especially `PcgenFtlTestCase` | GE05-OR-003, GE05-OR-004 | Strong test-backed pattern; existing Pathfinder fixtures are not the exact Human Fighter pilot. |
| Output sheet/export route | GE05-OR-005 | Viable output-format component; not a standalone oracle route. |
| Data validation or character integration tasks | GE05-OR-004, GE05-OR-006 | Useful supporting runtime evidence after Java 25; not sufficient as first pilot output by itself. |
| Static PCC/LST source inspection | GE05-OR-007 | Source truth only; explicitly not runtime parity evidence. |
| PCGen documentation/listfile docs | GE05-OR-008 | Semantic documentation evidence only; cite exact sections if used later. |
| PCGen code path inspection | GE05-OR-009 | Sufficient to select the next route, not sufficient to claim runtime behavior. |
| GUI route as last resort | GE05-OR-010 | Not recommended; only if headless/export/test routes fail and a decision record accepts the risk. |

## Recommended Next Probe

The decisive next probe for GE05-E1-F2 is a **single headless batch XML export using the CLI/export route, preferably via Gradle `run --args` until a runnable JAR route is explicitly grounded**.

Do not begin that probe until the next handoff grounds or authorizes all of the following:

1. Java 25 or an accepted PCGen runtime/toolchain setup.
2. Permission to generate build/runtime artifacts inside `/home/ubuntu/workspace/repos/pcgen`, if Gradle build/run is used.
3. An exact pilot character input path for `pf1-crb-human-fighter-level1` or authority to create one.
4. Exact output receipt path for the old-system output artifact and command log.
5. Whether captured PCGen-derived output may be stored directly, reduced, hashed/referenced, or generated on demand.

Candidate future command shape, not executed in this pass:

```bash
cd /home/ubuntu/workspace/repos/pcgen
./gradlew run --args="--character <authorized-pilot.pcg> --exportsheet code/testsuite/base-xml.ftl --outputfile <authorized-output.xml> --configfilename config.ini.junit"
```

The direct JAR shape documented by PCGen is also viable after a build creates the JAR:

```bash
java -jar build/libs/pcgen-<version>.jar --character <authorized-pilot.pcg> --exportsheet code/testsuite/base-xml.ftl --outputfile <authorized-output.xml> --configfilename config.ini.junit
```

These commands are **not** parity claims. They are only candidates for producing old-system evidence in the next route-specific runtime handoff.

## Blockers

| Blocker | Effect | Resolution needed |
|---|---|---|
| Java 25 absent | Blocks truthful runtime-output attempts for current PCGen doctrine/toolchain. | Install/provision/ground Java 25 or explicitly accept an alternative runtime route. |
| Active Java is 17 | Gradle task discovery works, but runtime/build output must not be assumed valid for PCGen 6.09 development. | Re-run runtime probes under Java 25 and record `java -version` in the output receipt. |
| Exact `pf1-crb-human-fighter-level1` PCG fixture not found | Existing Pathfinder tests are not the pilot case. | Locate an existing exact fixture or authorize creation of a minimal deterministic pilot `.pcg` in a future handoff. |
| PCGen build/run artifact generation not authorized by this research handoff | Prevents running build-heavy or output-producing commands in this pass. | Future handoff must state whether generated build/runtime artifacts under `/home/ubuntu/workspace/repos/pcgen` are allowed. |
| PCGen-derived output retention policy unresolved | Prevents deciding whether to store full XML, reduced facts, hashes, or command-only references. | GE-05/decision surface must choose retention rule before durable fixture storage. |
| No Codex implementation authority | Prevents building comparator, normalizer, or fixture schema in this pass. | A later execution handoff must define branch, write scope, tests, and verification. |

## Proposed Upstream Deltas

No upstream documents were modified because this handoff grants only the single receipt path. These are proposed deltas for a later authorized documentation update:

1. Add a GE-01 or GE-05 note that PCGen’s batch export path is now **grounded as a code/test/discovery route**, not yet as runtime behavior evidence.
2. Record `GE05-OR-001` as the preferred next old-system output probe once Java 25 and an exact pilot `.pcg` are available.
3. Record that existing Pathfinder FTL tests cover `pf_Cleric`, `pf_Paladin`, `pf_Rogue`, and `pf_goldielocks`, but not the exact Human Fighter level 1 pilot.
4. Before GE05-E1-F2, add a retention decision for PCGen-derived XML/output artifacts.
5. Preserve the rule that GUI-derived evidence is last-resort only and requires explicit acceptance.

## Verification

Planned structural checks for this receipt:

```bash
test -f /home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e1-f1-oracle-route-inventory-2026-06-20.md
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
test ! -f /home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/execution-handoff.md
git -C /home/ubuntu/workspace/repos/codex status --short
git -C /home/ubuntu/workspace/repos/pcgen status --short
```

Actual verification result from this run:

```text
receipt_bytes 25293
receipt_lines 227
[codex status]
?? AGENTS.md
?? CLAUDE.md
?? Cargo.lock
?? target/
[pcgen status]
VERIFICATION_PASSED
receipt=/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e1-f1-oracle-route-inventory-2026-06-20.md
GE-05 execution-handoff.md is absent
```

The Codex status entries are the pre-existing untracked residue already recorded by the readiness closure; the PCGen source checkout remained clean.

## Verdict

GE05-E1-F1 is complete for route inventory.

The lowest-friction next route is **PCGen headless batch XML export through the CLI/export path**, with Gradle `run --args` as the likely invocation wrapper until a runnable JAR is deliberately produced. The route is credible because the CLI parser, startup branch, exporter, integration-test base, and Pathfinder test examples all converge on the same mechanism.

GE05-E1-F2 is **not ready today** because Java 25 is absent, the exact pilot `.pcg` is not grounded, and this research handoff does not authorize build/output generation or PCGen fixture creation.

No parity claim, no oracle-checked claim, no final Human Fighter values, no Codex code changes, no PCGen modifications, and no GE-05 `execution-handoff.md` were produced.
