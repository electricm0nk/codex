# Unsupported, Partial, Lossy, and Unverified Semantics Ledger

## Objective
Define the visible debt surface for SD-13 so unsupported or incomplete breadth truth cannot disappear into chat history, private knowledge, or optimistic UI labels.

## Minimum row fields
Every ledger row must carry at least:
- subject type (`race`, `class`, `interaction`, `cross-cutting`)
- subject id
- semantic family or progression dimension
- current state (`partial`, `lossy`, `blocked`, `unverified`)
- evidence tier achieved
- grounding artifact or evidence ref
- why the current state is not `supported`
- who must see the debt (`operator`, `tester`, or `both`)
- next required uplift or owning future slice

## Visibility rules
- `blocked` debt that affects claim honesty MUST be visible to operators.
- `partial` or `lossy` debt that is surfaced in a tester-visible feature MUST also be visible to testers.
- `unverified` debt MAY stay operator-facing until the product or workflow exposes the relevant surface.
- no row may be deleted merely because a later slice wants cleaner wording; it must be upgraded, superseded, or explicitly retired.

## Seeded current-state rows
| Subject type | Subject id | Semantic family | State | Evidence tier | Grounding | Why not supported | Who must see it | Next required uplift |
|---|---|---|---|---|---|---|---|---|
| race | Human | full bounded race semantics beyond the deterministic pilot seam | `partial` | `Computed` | GE-06 deterministic Human Fighter fixture | current evidence proves only the bounded Human seam exercised by the pilot, not the full Human race burden | operator | SD13-E2 race-semantics slice |
| race | Dwarf | bounded race semantics | `unverified` | `Observed` | SD-13 packet roster only | no direct runtime evidence yet | operator | SD13-E2 race-semantics slice |
| race | Elf | bounded race semantics | `unverified` | `Observed` | SD-13 packet roster only | no direct runtime evidence yet | operator | SD13-E2 race-semantics slice |
| race | Gnome | bounded race semantics | `unverified` | `Observed` | SD-13 packet roster only | no direct runtime evidence yet | operator | SD13-E2 race-semantics slice |
| race | Half-Elf | bounded race semantics | `unverified` | `Observed` | SD-13 packet roster only | no direct runtime evidence yet | operator | SD13-E2 race-semantics slice |
| race | Half-Orc | bounded race semantics | `unverified` | `Observed` | SD-13 packet roster only | no direct runtime evidence yet | operator | SD13-E2 race-semantics slice |
| race | Halfling | bounded race semantics | `unverified` | `Observed` | SD-13 packet roster only | no direct runtime evidence yet | operator | SD13-E2 race-semantics slice |
| class | Fighter | level-10 progression | `blocked` | `Computed` | GE-06 tests explicitly block Fighter level 2 | level-10 cannot be claimed while level 2 is still blocked | operator | SD13-E3 martial progression slice |
| class | Rogue | bounded class progression | `blocked` | `Computed` | `tests/ge06_pilot_total_saves.rs` claim-blocks Rogue level 1 under the current bounded compute path | current pilot chassis cannot be swapped to Rogue honestly | operator | SD13-E3 martial progression slice |
| class | Barbarian | bounded class progression | `unverified` | `Observed` | SD-13 packet roster only | no direct runtime evidence yet | operator | SD13-E3 martial progression slice |
| class | Bard | spellcasting and class progression burden | `unverified` | `Observed` | SD-13 packet roster only | no direct spell or class progression evidence yet | operator | SD13-E4 spellcasting slice |
| class | Cleric | spellcasting and class progression burden | `unverified` | `Observed` | SD-13 packet roster only | no direct spell or class progression evidence yet | operator | SD13-E4 spellcasting slice |
| class | Druid | spellcasting and class progression burden | `unverified` | `Observed` | SD-13 packet roster only | no direct spell or class progression evidence yet | operator | SD13-E4 spellcasting slice |
| class | Monk | bounded class progression | `unverified` | `Observed` | SD-13 packet roster only | no direct runtime evidence yet | operator | SD13-E3 martial progression slice |
| class | Paladin | hybrid chassis plus spell burden | `unverified` | `Observed` | SD-13 packet roster only | no direct chassis or spell evidence yet | operator | SD13-E3 then SD13-E4 |
| class | Ranger | hybrid chassis plus spell burden | `unverified` | `Observed` | SD-13 packet roster only | no direct chassis or spell evidence yet | operator | SD13-E3 then SD13-E4 |
| class | Sorcerer | spellcasting and class progression burden | `unverified` | `Observed` | SD-13 packet roster only | no direct spell or class progression evidence yet | operator | SD13-E4 spellcasting slice |
| class | Wizard | spellcasting and class progression burden | `unverified` | `Observed` | SD-13 packet roster only | no direct spell or class progression evidence yet | operator | SD13-E4 spellcasting slice |
| interaction | Human bonus feat / ability bonus with class progression | bounded interaction seam | `partial` | `Computed` | deterministic GE-06 Human Fighter pilot | only the pilot seam is grounded, not the general interaction-row model | operator | SD13-E2 / SD13-E3 coupling |
| cross-cutting | multiclassing | class progression breadth | `blocked` | `Observed` | explicit SD-13 out-of-scope rule | outside the tranche by doctrine | operator and tester when later surfaces mention it | later authority only |

## Mutation rules
- upgrade a row only when a named evidence artifact or verification command justifies the change
- if a later slice narrows support or discovers new lossiness, patch the row immediately rather than leaving the older optimistic state in place
- if a tester-visible surface consumes this ledger, it must not suppress the state word or the blocker reason
