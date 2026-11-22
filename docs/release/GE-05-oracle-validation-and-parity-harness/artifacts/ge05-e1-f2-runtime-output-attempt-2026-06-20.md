---
title: GE05-E1-F2 Runtime Output Attempt
artifact_type: runtime-output-receipt
stc_id: STC-CODEX-GE-05
source_handoff: ../collection-handoff.md
selected_slice: GE05-E1-F2 — First reproducible old-system output route
workflow_route: collection
status: completed-with-blocker
created_at: 2026-06-21T02:34:25Z
code_authority: false
owner: Todd Hintzmann
scope: program
---

# GE05-E1-F2 Runtime Output Attempt

## Objective
Attempt exactly one headless PCGen batch XML export for the PF1 Core Rulebook Human Fighter level 1 pilot using the selected CLI/export route, then capture either produced old-system output or exact failure evidence.

This receipt does **not** authorize Codex implementation work, PCGen source edits, parity claims, or a downstream Claude Code handoff.

## Verdict
The GE05-E1-F2 collection route executed far enough to prove that PCGen starts in headless batch-export mode under Java 25, but the single authorized export attempt failed before XML generation.

The blocker is now specific:

```text
The provisional .pcg used older/incorrect campaign identity text.
Current local PCGen Core Rulebook data identifies the campaign as:
- CAMPAIGN:Core Rulebook
- KEY:Core Rulebook
- GAMEMODE:Pathfinder

The attempted .pcg used:
- CAMPAIGN:Paizo - Pathfinder Roleplaying Game Core Rulebook
- GAMEMODE:Pathfinder_RPG
```

PCGen reported:

```text
Could not find campaign: Paizo - Pathfinder Roleplaying Game Core Rulebook
Character's campaign entry was empty.
Loading sources [] using game mode Pathfinder_RPG
Failed to load sources
pcgen.persistence.PersistenceLayerException: You must select at least one campaign to load.
```

No raw XML was produced.

## Commands Run

### Java version
```bash
java -version 2>&1
javac -version 2>&1
```

Observed:

```text
openjdk version "25.0.3" 2026-04-21
OpenJDK Runtime Environment (build 25.0.3+9-2-24.04.2-Ubuntu)
OpenJDK 64-Bit Server VM (build 25.0.3+9-2-24.04.2-Ubuntu, mixed mode, sharing)
javac 25.0.3
```

### Repo status before export
```bash
git -C /home/ubuntu/workspace/repos/pcgen status --short
git -C /home/ubuntu/workspace/repos/pcgen rev-parse --abbrev-ref HEAD
```

Observed:

```text
PCGen branch: master
PCGen status: clean
```

### Authoritative `.pcg` search
```text
search_files(path="/home/ubuntu/workspace/repos/pcgen", pattern="*.pcg", target="files")
```

Found only unrelated sample/test character files:

```text
/home/ubuntu/workspace/repos/pcgen/data/zen_test/Dex3.pcg
/home/ubuntu/workspace/repos/pcgen/characters/SpecialWizard.pcg
/home/ubuntu/workspace/repos/pcgen/characters/CodeMonkey.pcg
/home/ubuntu/workspace/repos/pcgen/characters/Everything.pcg
/home/ubuntu/workspace/repos/pcgen/characters/Sorcerer.pcg
```

Additional test-suite examples existed under `code/testsuite/PCGfiles/`, but none was an authoritative `pf1-crb-human-fighter-level1` pilot file.

### Temporary runtime config
Created:

```text
/tmp/codex-ge05-e1-f2/settings/config.ini.junit
```

Contents:

```text
settingsPath=/tmp/codex-ge05-e1-f2/settings
pccFilesPath=data
customPath=testsuite\\customdata
```

This file is local/generated evidence under `/tmp`, not a PCGen tracked source edit.

### Provisional pilot `.pcg`
Created:

```text
/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg
```

The file was created because no authoritative pilot `.pcg` existed.

### Single export attempt
Exactly one export attempt was run:

```bash
mkdir -p /tmp/codex-ge05-e1-f2 && cd /home/ubuntu/workspace/repos/pcgen && ./gradlew run --args="--settingsdir /tmp/codex-ge05-e1-f2/settings --configfilename config.ini.junit --character /home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg --exportsheet code/testsuite/base-xml.ftl --outputfile /tmp/codex-ge05-e1-f2/pf1-crb-human-fighter-level1.xml"
```

Exit code:

```text
1
```

Key output:

```text
Starting PCGen v6.09.08.RC1
Started export of /home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg using code/testsuite/base-xml.ftl to /tmp/codex-ge05-e1-f2/pf1-crb-human-fighter-level1.xml
SEVERE main Globals:130 Could not find campaign: Paizo - Pathfinder Roleplaying Game Core Rulebook
SEVERE main PCGVer2Parser:295 Character's campaign entry was empty.
INFO main BatchExporter:122 Loading sources [] using game mode Pathfinder_RPG
SEVERE main SourceFileLoader:236 Failed to load sources
pcgen.persistence.PersistenceLayerException: You must select at least one campaign to load.
Execution failed for task ':run'
Process 'command '/usr/lib/jvm/java-25-openjdk-amd64/bin/java'' finished with non-zero exit value 1
```

## Output Path
Expected raw XML path:

```text
/tmp/codex-ge05-e1-f2/pf1-crb-human-fighter-level1.xml
```

Observed:

```text
NO XML OUTPUT
```

## SHA-256
No SHA-256 was computed because no XML output file was produced.

## Reduced Extracted Facts
No XML structural facts are available because no raw XML was produced.

Runtime facts established by the failed attempt:

| Fact | Result |
|---|---|
| Java 25 usable by Gradle route | yes |
| PCGen Gradle `run` route reaches batch exporter | yes |
| `code/testsuite/base-xml.ftl` accepted as selected export sheet path | route reached this stage |
| provisional `.pcg` file opened by PCGen | yes, export log names it |
| campaign source loading succeeded | no |
| raw XML produced | no |
| blocker class | campaign identity / game-mode naming mismatch in provisional `.pcg` |

## Grounded Inputs Used
Grounded from the GE-05 handoff and GE-06 selection ledger:

- case id: `pf1-crb-human-fighter-level1`
- race: `Human`
- class: `Fighter 1`
- ability scores: `STR 16 / DEX 14 / CON 14 / INT 10 / WIS 12 / CHA 8`
- named feat seed: `Power Attack`

## Provisional Assumptions Made
These assumptions were used only for the provisional runtime input. They are **not** canonical GE-06 pilot truth.

| Assumption | Why it was made | Status |
|---|---|---|
| `ABILITY:Ability Bonus` selected `+2 Strength` | PCGen Pathfinder human examples represent Human ability bonus as an ability row; final score vector already includes STR 16, but GE-06 has not canonically decomposed the racial bonus target. | provisional |
| no final equipment loadout | GE-06 explicitly says Chain Shirt + Longsword are grounded anchors, not final loadout closure. | provisional omission |
| no skill-rank allocation | GE-06 explicitly says exact skill allocation is unresolved. | provisional omission |
| no additional feat-slot closure beyond Power Attack | GE-06 explicitly says Human/Fighter feat-slot debt remains unmapped. | provisional omission |
| campaign name `Paizo - Pathfinder Roleplaying Game Core Rulebook` and `GAMEMODE:Pathfinder_RPG` | Borrowed from older Pathfinder test PCG examples such as `pf_Paladin.pcg`; the live Core Rulebook `.pcc` later proved current local identity differs. | failed provisional assumption |

## Source Evidence for Newly Identified Blocker
The local Core Rulebook `.pcc` currently says:

```text
/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc:2 CAMPAIGN:Core Rulebook
/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc:3 KEY:Core Rulebook
/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc:4 GAMEMODE:Pathfinder
```

This contradicts the older test PCG naming style used in the provisional file.

## Files Written
Within authorized scope:

- `/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg`
- `/tmp/codex-ge05-e1-f2/settings/config.ini.junit`
- `/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e1-f2-runtime-output-attempt-2026-06-20.md`

No raw XML was written.

Gradle also used normal build/cache surfaces during `./gradlew run`. PCGen `git status --short` remained clean after the attempt.

## Forbidden Write Check
- Codex implementation source files modified: **no**
- tracked PCGen source files modified: **no**
- `execution-handoff.md` created: **no**
- GE-06 source artifacts rewritten: **no**
- raw XML committed as canonical fixture: **no**

Note: `/home/ubuntu/workspace/repos/codex` had pre-existing untracked files unrelated to this collection run (`AGENTS.md`, `CLAUDE.md`, `Cargo.lock`, `target/`). This GE05-E1-F2 collection run did not write Codex repo implementation files.

## Next Required Action
Do not run another export under this same receipt.

The next bounded collection attempt should update or regenerate the provisional `.pcg` to use the live campaign identity:

```text
CAMPAIGN:Core Rulebook
GAMEMODE:Pathfinder
```

Then it may run a new single export attempt under a follow-on receipt, preserving the same retention/write-scope policy.

## Completion Statement
GE05-E1-F2 did not produce oracle XML, but it did produce route evidence:

1. Java 25 and Gradle can start PCGen headlessly.
2. The batch exporter route is real and reachable.
3. The current blocker is not Java, Gradle, or the export sheet; it is the provisional pilot `.pcg` campaign/game-mode identity.
4. The next GE-05 move is a narrower corrective collection attempt, not a code handoff.
