---
title: GE05-E1-F2 Runtime Output Attempt 3
artifact_type: runtime-output-receipt
stc_id: STC-CODEX-GE-05
source_handoff: ../collection-handoff.md
selected_slice: GE05-E1-F2 — First reproducible old-system output route
workflow_route: collection
status: completed-with-output
created_at: 2026-06-20T22:39:55-04:00
code_authority: false
owner: Todd Hintzmann
scope: program
raw_xml_retention: local-generated-only
---

# GE05-E1-F2 Runtime Output Attempt 3

## Objective
Run one bounded headless PCGen XML export after correcting the provisional pilot `.pcg` to the route identity established by attempts 1 and 2.

This receipt records reduced facts and output metadata only. The raw XML remains local/generated evidence under `/tmp` and is not committed as canonical fixture content.

## Change applied before this attempt
The provisional `.pcg` retained the accepted campaign source identity:

```text
CAMPAIGN:Core Rulebook
```

and restored the PCGen runtime game mode required by the character loader:

```text
GAMEMODE:Pathfinder_RPG
```

## Command Run

```bash
mkdir -p /tmp/codex-ge05-e1-f2 && rm -f /tmp/codex-ge05-e1-f2/pf1-crb-human-fighter-level1-attempt3.xml && cd /home/ubuntu/workspace/repos/pcgen && ./gradlew run --args="--settingsdir /tmp/codex-ge05-e1-f2/settings --configfilename config.ini.junit --character /home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg --exportsheet code/testsuite/base-xml.ftl --outputfile /tmp/codex-ge05-e1-f2/pf1-crb-human-fighter-level1-attempt3.xml"
```

Exit code:

```text
0
```

Key output:

```text
Starting PCGen v6.09.08.RC1
Started export of /home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg using code/testsuite/base-xml.ftl to /tmp/codex-ge05-e1-f2/pf1-crb-human-fighter-level1-attempt3.xml
Loading sources [Core Rulebook] using game mode Pathfinder_RPG
Loading game Pathfinder_RPG and sources [Core Rulebook].
Loaded character pf1-crb-human-fighter-level1-provisional-ge05-e1-f2 - /home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg
BUILD SUCCESSFUL in 21s
```

Observed warnings/errors during source load:

```text
LSTERROR ... Illegal FACT subtoken 'IsOrc' 'True' for pcgen.core.PCTemplate IsOrc
Evaluation called on invalid variable: 'CHANNEL*STATSCORE', assuming default for Number
Evaluation called on invalid variable: 'Score', assuming default for Number
Evaluation called on invalid variable: 'Mod', assuming default for Number
```

These warnings did not prevent the export from completing.

## Output Metadata
Raw XML path:

```text
/tmp/codex-ge05-e1-f2/pf1-crb-human-fighter-level1-attempt3.xml
```

Size:

```text
38915 bytes
```

SHA-256:

```text
3c0e38e7837dbcd6c5003ba100eb35e3f0992366d086228c4706d6f165d281e1
```

Retention posture:

```text
raw XML local/generated only; not committed as canonical fixture content
```

## Reduced Structural Summary
| Field | Value |
|---|---|
| root tag | `character` |
| element count | `1023` |
| distinct tag count | `188` |
| top-level sections | `basics`, `abilities`, `hit_points`, `armor_class`, `initiative`, `skills`, `saving_throws`, `attack`, `weapons`, `protection`, `class_features`, `equipment`, `weight_allowance`, `special_abilities`, `feats`, `ability_objects`, `weapon_proficiencies`, `languages`, `misc`, `companions`, `spells` |

## Reduced Character Facts
These facts are extracted from the generated XML and are therefore old-system runtime output for the provisional input, not canonical GE-06 truth.

| Fact | Value |
|---|---|
| character name | `pf1-crb-human-fighter-level1-provisional-ge05-e1-f2` |
| race | `Human` |
| class | `Fighter` |
| level | `1` |
| class shortform | `Ftr1` |
| alignment | `TN` / `Neutral` |
| size | `Medium` |
| type | `Humanoid` |
| BAB | `+1` |
| hit points | `12` |
| hit dice | `(1d10)+2` |
| AC | `12` |
| flat-footed AC | `10` |
| touch AC | `12` |
| initiative | `+2` |
| Fortitude | `+4` |
| Reflex | `+2` |
| Will | `+1` |
| melee attack total | `+5` |
| ranged attack total | `+3` |
| grapple total | `+1` |
| languages | `Common` |
| equipment total weight | `0 lbs.` |
| equipment total value | `0 gp` |
| skill points total / used / unused | `3 / 0 / 3` |

## Reduced Ability Facts
The XML output applied the provisional `+2 Strength` human ability bonus assumption.

| Ability | Base | Output score | Output modifier |
|---|---:|---:|---:|
| STR | `16` | `18` | `+4` |
| DEX | `14` | `14` | `+2` |
| CON | `14` | `14` | `+2` |
| INT | `10` | `10` | `+0` |
| WIS | `12` | `12` | `+1` |
| CHA | `8` | `8` | `-1` |

## Reduced Feat Facts
Visible feat output includes:

- `Power Attack`
- hidden/virtual Power Attack variants:
  - `Power Attack (Light)`
  - `Power Attack (Off-Hand)`
  - `Power Attack (One-Handed)`
  - `Power Attack (Two-Handed)`
- internal `CMB Output`

These are runtime-output facts for the provisional file only. They do not close GE-06's unmapped feat-slot debt.

## Provisional Assumptions Still Present
These remain non-canonical:

| Assumption | Status |
|---|---|
| Human ability bonus set to `+2 Strength` | provisional; output confirms PCGen then produces STR 18 |
| no final equipment loadout | provisional omission |
| no skill-rank allocation | provisional omission; output shows `3` unused skill points |
| no additional feat-slot closure beyond `Power Attack` | provisional omission |

## Source/Route Facts Established
| Fact | Result |
|---|---|
| Java 25 Gradle run path | works |
| PCGen headless batch exporter | works |
| source campaign identity for CLI/source load | `CAMPAIGN:Core Rulebook` |
| runtime game mode accepted by character loader | `Pathfinder_RPG` |
| export sheet | `code/testsuite/base-xml.ftl` |
| provisional character can load | yes |
| XML output produced | yes |

## Files Written
Within authorized scope:

- `/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg`
- `/tmp/codex-ge05-e1-f2/pf1-crb-human-fighter-level1-attempt3.xml`
- `/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e1-f2-runtime-output-attempt-3-2026-06-20.md`

## Forbidden Write Check
- Codex implementation source files modified: **no**
- tracked PCGen source files modified: **no**
- `execution-handoff.md` created: **no**
- raw XML committed as canonical fixture: **no**
- GE-06 source artifacts rewritten: **no**

PCGen `git status --short` after the run was clean.

The Codex implementation repo still had pre-existing untracked files unrelated to this GE05-E1-F2 collection run:

```text
?? AGENTS.md
?? CLAUDE.md
?? Cargo.lock
?? target/
```

## Completion Statement
GE05-E1-F2 now has a successful old-system runtime-output receipt.

This does **not** create code authority. The next program move is to decide whether to:

1. derive a further Hermes-route GE-05 collection/reduction handoff to normalize the reduced facts and output-shape contract, or
2. derive a first code-authorizing GE-05 execution handoff for a bounded comparator/fixture-reader slice that consumes this receipt, while treating the provisional `.pcg` assumptions as non-canonical.
