---
title: GE05-E1-F2 Runtime Output Attempt 2
artifact_type: runtime-output-receipt
stc_id: STC-CODEX-GE-05
source_handoff: ../collection-handoff.md
selected_slice: GE05-E1-F2 — First reproducible old-system output route
workflow_route: collection
status: completed-with-blocker
created_at: 2026-06-20T22:38:55-04:00
code_authority: false
owner: Todd Hintzmann
scope: program
---

# GE05-E1-F2 Runtime Output Attempt 2

## Objective
Follow up the first failed runtime-output attempt by correcting the provisional `.pcg` campaign identity to the live local Core Rulebook campaign and running one new headless PCGen export attempt.

## Change applied before this attempt
The provisional pilot `.pcg` was changed from:

```text
CAMPAIGN:Paizo - Pathfinder Roleplaying Game Core Rulebook
GAMEMODE:Pathfinder_RPG
```

to:

```text
CAMPAIGN:Core Rulebook
GAMEMODE:Pathfinder
```

The `CAMPAIGN:Core Rulebook` correction was grounded by:

```text
/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc:2 CAMPAIGN:Core Rulebook
/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc:3 KEY:Core Rulebook
/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc:4 GAMEMODE:Pathfinder
```

## Command Run

```bash
mkdir -p /tmp/codex-ge05-e1-f2 && rm -f /tmp/codex-ge05-e1-f2/pf1-crb-human-fighter-level1-attempt2.xml && cd /home/ubuntu/workspace/repos/pcgen && ./gradlew run --args="--settingsdir /tmp/codex-ge05-e1-f2/settings --configfilename config.ini.junit --character /home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg --exportsheet code/testsuite/base-xml.ftl --outputfile /tmp/codex-ge05-e1-f2/pf1-crb-human-fighter-level1-attempt2.xml"
```

Exit code:

```text
1
```

## Result
The attempt advanced past the original campaign-name failure. PCGen loaded sources:

```text
Loading sources [Core Rulebook] using game mode Pathfinder_RPG
Loading game Pathfinder_RPG and sources [Core Rulebook].
```

But the character then failed to load because the `.pcg` line `GAMEMODE:Pathfinder` did not match the runtime game mode selected by PCGen:

```text
Unable to load the character as it uses game mode: "Pathfinder".
PCGen is currently using gamemode "Pathfinder_RPG".
Method ParseGameMode was unable to parse line GAMEMODE:Pathfinder
```

## Output Path
Expected raw XML path:

```text
/tmp/codex-ge05-e1-f2/pf1-crb-human-fighter-level1-attempt2.xml
```

Observed:

```text
NO XML OUTPUT
```

## SHA-256
No SHA-256 was computed because no XML output file was produced.

## Reduced Facts
This attempt established one important route fact:

| Fact | Result |
|---|---|
| `CAMPAIGN:Core Rulebook` is accepted for source loading | yes |
| runtime game mode selected by PCGen after loading Core Rulebook | `Pathfinder_RPG` |
| `.pcg` game mode `Pathfinder` is accepted | no |
| raw XML produced | no |

## Next Corrective Action
Keep:

```text
CAMPAIGN:Core Rulebook
```

but restore the character-file game mode to:

```text
GAMEMODE:Pathfinder_RPG
```

Then run a new bounded export attempt and record it separately.

## Forbidden Write Check
- Codex implementation source files modified: **no**
- tracked PCGen source files modified: **no**
- `execution-handoff.md` created: **no**
- raw XML committed as canonical fixture: **no**
