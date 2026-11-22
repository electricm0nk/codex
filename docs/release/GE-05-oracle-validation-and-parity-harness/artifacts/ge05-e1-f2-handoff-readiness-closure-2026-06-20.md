---
title: GE05-E1-F2 Handoff Readiness Closure
artifact_type: handoff-readiness-closure
stc_id: STC-CODEX-GE-05
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE05-E1-F2 — First reproducible old-system output route
workflow_route: collection
readiness: collection-ready
recommended_handoff: ../collection-handoff.md
handoff_created: true
created_handoff: ../collection-handoff.md
review_date: 2026-06-20
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE05-E1-F2 Handoff Readiness Closure

## Verdict
GE05-E1-F2 is the correct next bounded slice for GE-05 and is now **collection-ready**.

No `execution-handoff.md` should be created from this closure.

The future route is **not coding first**. It is a bounded **collection/runtime-evidence** handoff for one selected old-system output attempt. That handoff is now truthful because the runtime is grounded, retention/write-scope policy is chosen, and provisional pilot-input authority has been explicitly granted.

## Selected bounded slice

```text
GE05-E1-F2 — First reproducible old-system output route
```

Source in `epic-breakdown.md`:

- one selected route can produce or attempt to produce old-system output for the Human Fighter case
- failure output is captured if the route is blocked
- route evidence is sufficient to derive the next fixture or blocker

## Why this is the next slice
GE05-E1-F1 completed the route-inventory phase and selected the lowest-friction next probe.

That route was:

- **PCGen headless batch XML export through the CLI/export path**
- preferably via `./gradlew run --args="..."` until a runnable JAR route is deliberately grounded

GE05-E1-F2 is therefore the next truthful slice because GE-05 cannot advance to fixture schema, capture adapters, comparator logic, or parity reports without at least one attempted old-system output route for the pilot case.

## Grounded facts recovered by tools

| Fact | Result |
|---|---|
| Workspace date from runtime | `2026-06-20` |
| Selected next route from GE05-E1-F1 | headless batch XML export via CLI/export path |
| Route evidence in PCGen repo doctrine | `repos/pcgen/AGENTS.md` documents `--character`, `--exportsheet`, `--outputfile`, `--configfilename`, plus both JAR and Gradle `run --args` examples |
| Route evidence in PCGen test code | `repos/pcgen/code/src/slowtest/pcgen/inttest/PcgenFtlTestCase.java:98-101` invokes `Main.main("--character", ..., "--exportsheet", "code/testsuite/base-xml.ftl", "--outputfile", ..., "--configfilename", TEST_CONFIG_FILE)` |
| Active Java | OpenJDK `25.0.3` |
| Java 25 availability | installed in JVM alternatives and `/usr/lib/jvm/java-25-openjdk-amd64` |
| PCGen repo branch | `master` |
| PCGen working tree | clean during this closure pass |
| Codex repo branch | `ge04-e1-f1-character-input-record-shape` |
| Codex local residue | untracked `AGENTS.md`, `CLAUDE.md`, `Cargo.lock`, `target/` |
| Existing `.pcg` files found in PCGen repo | `data/zen_test/Dex3.pcg`, `characters/SpecialWizard.pcg`, `characters/CodeMonkey.pcg`, `characters/Everything.pcg`, `characters/Sorcerer.pcg` |
| Exact pilot `.pcg` found for `pf1-crb-human-fighter-level1` | **no** |
| Retention policy for PCGen-derived output | still unresolved by GE05-E1-F1 |
| Active GE-05 source posture | `planning-ready` source STC; no active code-authorizing handoff exists |

## Commands run for this closure

```bash
date +%F
java -version
update-java-alternatives -l
find /usr/lib/jvm -maxdepth 1 -mindepth 1 -type d | sort
git -C /home/ubuntu/workspace/repos/pcgen rev-parse --abbrev-ref HEAD
git -C /home/ubuntu/workspace/repos/pcgen status --short
git -C /home/ubuntu/workspace/repos/codex rev-parse --abbrev-ref HEAD
git -C /home/ubuntu/workspace/repos/codex status --short
```

Additional evidence was recovered from:

- `/home/ubuntu/workspace/repos/pcgen/AGENTS.md:216-218`
- `/home/ubuntu/workspace/repos/pcgen/code/src/slowtest/pcgen/inttest/PcgenFtlTestCase.java:98-101`
- GE05-E1-F1 route inventory receipt

## Gate table

| Gate | Status | Resolution |
|---|---|---|
| Source STC exists | pass | GE-05 source-STC bundle exists and remains planning-honest. |
| Next bounded slice selected | pass | GE05-E1-F2 is explicitly defined in `epic-breakdown.md`. |
| Candidate route selected | pass | GE05-E1-F1 selected the CLI/export XML route as the next probe. |
| Slice narrower than spec domain | pass | This slice is one old-system output attempt, not fixture schema, comparator logic, parity reporting, or GE-06 integration. |
| Route class explicit | pass | This is a runtime-output collection/evidence slice, not a code-implementation slice. |
| Runtime command family grounded | pass | PCGen doctrine and tests both ground the CLI/export path. |
| Java/runtime substrate grounded enough to run the selected route | pass | Java 25 is present, active, and matches current PCGen doctrine for 6.09 development. |
| Exact pilot character input grounded | fail | No exact `pf1-crb-human-fighter-level1` `.pcg` file is present or authorized yet. |
| Old-system output retention rule grounded | fail | GE05-E1-F1 explicitly left retention unresolved. |
| Exact output receipt path and raw-output posture declared | fail | The future handoff has not yet fixed whether output is stored directly, reduced, hashed/reference-only, or generated on demand. |
| Allowed write scope explicit | fail | The future handoff has not yet stated whether generated artifacts under `repos/pcgen/build/` are allowed, nor the exact GE-05 artifact output path for the runtime attempt receipt. |
| Codex code authority needed | pass | none; this slice does not require Codex implementation authority yet. |
| Non-goals explicit | pass | No comparator code, no GE-03/GE-04 modifications, no PCGen source edits, no GUI default path, no parity claims. |

## What this closure proves
This closure advances GE-05 in three ways:

1. It upgrades the phase label from vague “research later” to a specific **runtime-output collection** phase.
2. It confirms that the selected route and Java runtime are no longer the blockers, and that the retention/write-scope policy is now explicit.
3. It makes clear that the next truthful artifact is still **not** a coding handoff. It is a narrower non-code collection handoff for one output attempt.

## Remaining blockers

| Blocker | Effect | Exact resolution needed |
|---|---|---|
| Exact pilot `.pcg` absent | No already-authoritative pilot file exists. | Use the now-authorized provisional pilot `.pcg` path in the derived collection handoff and record every non-grounded choice as provisional. |
| Output retention policy grounded | pass | Retain receipt + reduced facts + command metadata + SHA256 of raw XML; keep raw XML local/generated rather than canonical committed evidence. |
| Future write scope explicit enough for collection handoff | pass | Allow one GE-05 runtime-output receipt, one provisional pilot `.pcg`, transient generated artifacts under `repos/pcgen/build/`, and local/generated raw XML only. |

## Future handoff type
The correct next artifact is now:

```text
collection-handoff.md
```

Preferred route label:

- `work_type: data-collection`
- `workflow_route: collection`

This keeps the human/operator boundary explicit: Hermes handles the old-system evidence collection pass; Claude Code remains reserved for later code-authorizing handoffs only.

## Minimum objective for the future GE05-E1-F2 handoff

```text
Attempt exactly one headless PCGen batch XML export for the PF1 Core Rulebook Human Fighter level 1 pilot using the selected CLI/export route, capture either the produced old-system output or the exact failure evidence, and write a single GE-05 runtime-output receipt without modifying Codex source code or PCGen tracked source files.
```

## Minimum required facts before creating that handoff
The derived GE05-E1-F2 handoff must explicitly state all of the following:

1. Exact pilot character input path, with authority to create one provisional pilot `.pcg` if absent.
2. Exact receipt path for the runtime-output attempt.
3. Exact posture for raw output retention.
4. Whether transient generated artifacts under `repos/pcgen/build/` are allowed.
5. That no Codex repo code changes are authorized.
6. That no PCGen tracked source-file edits are authorized.

## Candidate command shape for the future handoff
These are grounded candidate shapes only. They were **not executed** in this closure pass.

Gradle wrapper route:

```bash
cd /home/ubuntu/workspace/repos/pcgen
./gradlew run --args="--character <authorized-pilot.pcg> --exportsheet code/testsuite/base-xml.ftl --outputfile <authorized-output.xml> --configfilename config.ini.junit"
```

Direct JAR route after an authorized build produces the runtime:

```bash
java -jar build/libs/pcgen-<version>.jar --character <authorized-pilot.pcg> --exportsheet code/testsuite/base-xml.ftl --outputfile <authorized-output.xml> --configfilename config.ini.junit
```

## Stop conditions for the future GE05-E1-F2 worker
Stop and report rather than improvising if:

1. The only way forward is to invent or guess the pilot `.pcg`.
2. The run would require editing tracked files in `/home/ubuntu/workspace/repos/pcgen`.
3. The run would require writing Codex implementation code.
4. GUI automation becomes the only route.

## Result
GE05-E1-F2 is the correct next bounded slice.

The selected oracle route is grounded.
The Java runtime is now grounded.
The pilot input is not yet canonical, but provisional authority is now explicit.

Therefore:

- no `execution-handoff.md` should be created now
- no Claude Code lane should be opened from GE-05 yet
- the correct next surface is the bounded collection handoff at `../collection-handoff.md`

## Closure status
GE05-E1-F2 is selected, better defined than before, and **collection-ready**. Code authority remains absent by design.