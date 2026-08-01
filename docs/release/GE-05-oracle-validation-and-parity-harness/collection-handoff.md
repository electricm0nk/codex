---
title: GE-05 Collection Handoff — GE05-E1-F2 First Reproducible Old-System Output Route
stc_id: STC-CODEX-GE-05
artifact_type: agent-handoff
status: active
scope: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness
source_stc: ./README.md
target_agent: god-emporer
work_type: data-collection
workflow_route: collection
readiness: collection-ready
selected_slice: GE05-E1-F2 — First reproducible old-system output route
code_authority: false
---

# Agent Handoff: GE05-E1-F2 First Reproducible Old-System Output Route

## Objective
Attempt exactly one headless PCGen batch XML export for the PF1 Core Rulebook Human Fighter level 1 pilot using the selected CLI/export route, capture either the produced old-system output or the exact failure evidence, and write a single GE-05 runtime-output receipt without modifying Codex source code or PCGen tracked source files.

## Work Type
`data-collection`

## Workflow Route
`collection`

## Readiness
`collection-ready`

## Source STC
- path: `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/README.md`
- source readiness: `planning-ready`
- authoritative for: GE-05 scope, oracle/parity boundaries, evidence rules, and non-code routing posture

## Downstream Target
- harness or workflow: `fresh god-emporer / Hermes session`
- invocation mode: `direct session`

## Required Reads
- `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/README.md`
- `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/technical-requirements.md`
- `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/acceptance-and-verification.md`
- `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/research-handoff.md`
- `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e1-f1-oracle-route-inventory-2026-06-20.md`
- `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e1-f2-handoff-readiness-closure-2026-06-20.md`
- `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/oracle-strategy-specification-requirements.md`
- `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/known-gap-policy.md`
- `programs/codex/plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md`
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-grounded-character-selection-ledger-2026-06-20.md`
- `/home/ubuntu/workspace/repos/pcgen/AGENTS.md`
- `/home/ubuntu/workspace/repos/pcgen/code/src/slowtest/pcgen/inttest/PcgenFtlTestCase.java`

## Grounded pilot inputs already available
Treat these as grounded inputs, not guesses:

- case id: `pf1-crb-human-fighter-level1`
- race: `human`
- class: `fighter 1`
- ability scores: `STR 16 / DEX 14 / CON 14 / INT 10 / WIS 12 / CHA 8`
- named feat seed: `power_attack`

## Unresolved pilot inputs that remain provisional if touched
Do **not** silently treat these as canonical:

- full feat-slot closure beyond `power_attack`
- explicit Human `+2` ability-target decomposition
- exact skill-rank allocation
- final equipped loadout
- `Chain Shirt + Longsword` are only grounded anchors, not final required loadout

## Source Universe / Inputs
- `/home/ubuntu/workspace/repos/pcgen`
- `/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/`
- `/home/ubuntu/workspace/programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/`

## Required Output Artifacts
Create or update exactly these governed outputs:

- `/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e1-f2-runtime-output-attempt-2026-06-20.md`
- `/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg` — only if no already-authoritative pilot `.pcg` is found

Local/generated, non-canonical evidence path:

- `/tmp/codex-ge05-e1-f2/pf1-crb-human-fighter-level1.xml`

## Required collection steps
1. Verify Java 25 and note exact versions in the receipt.
2. Verify the selected CLI/export route remains available.
3. Search for an already-authoritative pilot `.pcg` for `pf1-crb-human-fighter-level1`.
4. If none exists, create exactly one **provisional** pilot `.pcg` at the required path above using only the grounded inputs listed in this handoff.
5. If PCGen requires additional choices beyond those grounded inputs, do **not** silently close them as canonical. Either:
   - record the exact failure evidence, or
   - if a minimal extra choice is strictly required to make the runtime attempt possible, make it only as a **provisional assumption** and document it explicitly in the receipt.
6. Attempt exactly one headless XML export via the grounded CLI/export path.
7. Compute a SHA-256 hash of the produced raw XML if output is produced.
8. Write the runtime-output receipt with reduced extracted facts, command metadata, and either produced-output evidence or exact failure evidence.

## Candidate command shape
Gradle route preferred:

```bash
cd /home/ubuntu/workspace/repos/pcgen
./gradlew run --args="--character /home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg --exportsheet code/testsuite/base-xml.ftl --outputfile /tmp/codex-ge05-e1-f2/pf1-crb-human-fighter-level1.xml --configfilename config.ini.junit"
```

If a different already-grounded route is required, the receipt must justify it with source evidence.

## Receipt requirements
The runtime-output receipt MUST include:

- exact commands run
- Java version
- current PCGen branch and working-tree status
- whether an authoritative pilot `.pcg` was found or a provisional one was created
- every provisional assumption made, if any
- exact output path
- whether raw XML was produced
- SHA-256 of raw XML if produced
- reduced extracted facts or structural summary of the XML
- exact failure output if no XML was produced
- explicit statement that no Codex source code was modified
- explicit statement that no tracked PCGen source files were modified
- exact paths of every file written

## Retention policy
Use the selected GE-05 retention rule:

- commit the runtime-output receipt
- include reduced facts / structural summary in the receipt
- include exact command metadata in the receipt
- include SHA-256 of raw XML in the receipt if raw XML is produced
- keep raw XML as local/generated evidence only
- do **not** commit raw XML as canonical GE-05 fixture content in this pass

## Allowed write scope
Allowed writes are limited to:

- `/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e1-f2-runtime-output-attempt-2026-06-20.md`
- `/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg` if needed
- `/tmp/codex-ge05-e1-f2/` local/generated evidence
- transient generated artifacts under `/home/ubuntu/workspace/repos/pcgen/build/`
- incidental Gradle cache/tool artifacts outside the repo if the toolchain creates them automatically

## Forbidden writes
- no Codex repo source changes
- no tracked PCGen source-file edits
- no `execution-handoff.md`
- no GE-06 source artifact rewrites in this run
- no silent promotion of provisional choices into canonical pilot truth

## Non-Goals
- do not implement comparator logic, canonical models, or normalization code
- do not broaden to more than one runtime-output attempt
- do not claim parity
- do not resolve all GE-06 pilot input debt unless the exact grounded source already exists
- do not pivot into GUI automation

## Acceptance Criteria
- one runtime-output receipt exists at the exact required path
- if no authoritative pilot `.pcg` existed, one provisional pilot `.pcg` exists at the exact required path
- the receipt explicitly distinguishes grounded inputs from provisional assumptions
- the receipt either contains raw-output evidence metadata plus SHA-256 and reduced facts, or exact failure evidence
- no disallowed files were modified

## Verification
- read back the created receipt
- verify the receipt cites the exact files and commands used
- verify any provisional `.pcg` path matches the authorized location
- verify no Codex source files were changed
- verify no tracked PCGen source files were changed

## Stop conditions
Stop and report rather than improvising if:

1. The only way forward is broad manual character authoring with multiple ungrounded choices.
2. The export route requires GUI interaction.
3. The run requires tracked edits inside `/home/ubuntu/workspace/repos/pcgen`.
4. The run requires Codex implementation code.
5. The worker cannot distinguish grounded inputs from provisional assumptions in the receipt.

## Notes for the downstream agent
- This handoff is route-correct for Hermes, not Claude Code.
- The objective is evidence collection, not truth inflation.
- A failed attempt with exact evidence is a successful GE05-E1-F2 outcome if it narrows the next blocker honestly.
