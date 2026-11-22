# Core Roster and Support-State Matrix

## Vocabulary authority

The two axes used throughout this matrix — **support state** (the named dimension's current implementation state) and **evidence tier** (the strongest evidence form available for any claim made about that state) — are defined canonically in `programs/codex/doctrine/support-state-vocabulary.md`. This matrix is the row-by-row application of that vocabulary to the SD-13 core roster and level-10 progression surface. Definitions of `supported/partial/lossy/blocked/unverified` and `observed/parsed/converted/computed/oracle-checked/product-visible` live there; this matrix only names each row's state and tier.

When a slice updates a row, the merge must move the row's state and tier consistently with the vocabulary. Counterfeit completion (e.g., `supported` at tier `computed`, or `partial` at tier `observed`) is refused by audit.

## Objective

Define the exact SD-13 roster, the support-state taxonomy, the row model, and the seeded current-truth matrix that later execution slices must update instead of replacing with folklore.
## Exact bounded roster

### Core races
1. Dwarf
2. Elf
3. Gnome
4. Half-Elf
5. Half-Orc
6. Halfling
7. Human

### Core classes
1. Barbarian
2. Bard
3. Cleric
4. Druid
5. Fighter
6. Monk
7. Paladin
8. Ranger
9. Rogue
10. Sorcerer
11. Wizard

## Support-state taxonomy
| State | Meaning | May count as supported breadth? |
|---|---|---|
| `supported` | The named dimension is proven at the required evidence floor and has no known missing semantics inside the bounded claim. | Yes |
| `partial` | Some required semantics are proven, but one or more named required semantics remain incomplete and visible. | No |
| `lossy` | The path works only by discarding or approximating named semantics. | No |
| `blocked` | Known missing semantics, explicit claim-blocking diagnostics, or contradictory behavior prevent the claim. | No |
| `unverified` | No direct evidence yet exists for the named dimension. | No |

## Evidence-tier axis
Use the Codex quality-gate tiers independently from support state:
- `Observed`
- `Parsed`
- `Converted`
- `Computed`
- `Oracle-checked`
- `Product-visible`

## Row model
Each row must carry at least:
- subject type: `race`, `class`, or `interaction`
- subject id
- semantic or progression dimension
- current support state
- highest evidence tier achieved
- grounding artifact or evidence ref
- known blocker or known lossiness note when not `supported`
- next required uplift or owning slice

## Seeded current-truth matrix

### Race rows
| Subject | Dimension | State | Evidence tier | Grounding | Next required uplift |
|---|---|---|---|---|---|
| Human | bounded pilot race semantics actually exercised by GE-06 deterministic proof | `partial` | `Computed` | deterministic fixture plus GE-06 Human bonus-feat / ability-bonus-bearing pilot path | classify remaining Human racial semantics explicitly and decide whether any are out of slice |
| Dwarf | bounded race semantics | `unverified` | `Observed` | named by SD-13 scope only | create race-semantic execution slice and classify the row honestly |
| Elf | bounded race semantics | `unverified` | `Observed` | named by SD-13 scope only | create race-semantic execution slice and classify the row honestly |
| Gnome | bounded race semantics | `unverified` | `Observed` | named by SD-13 scope only | create race-semantic execution slice and classify the row honestly |
| Half-Elf | bounded race semantics | `unverified` | `Observed` | named by SD-13 scope only | create race-semantic execution slice and classify the row honestly |
| Half-Orc | bounded race semantics | `unverified` | `Observed` | named by SD-13 scope only | create race-semantic execution slice and classify the row honestly |
| Halfling | bounded race semantics | `unverified` | `Observed` | named by SD-13 scope only | create race-semantic execution slice and classify the row honestly |

### Class rows
| Subject | Dimension | State | Evidence tier | Grounding | Next required uplift |
|---|---|---|---|---|---|
| Fighter | class progression through level 1 deterministic pilot surface | `partial` | `Computed` | GE-06 deterministic fixture plus bounded save/combat tests | widen beyond level 1 and classify all mandatory level-10 milestones |
| Fighter | class progression through levels 2-10 | `blocked` | `Computed` | GE-06 tests explicitly claim-block `class:fighter:2` | create progression slice that grounds the first real post-level-1 milestone burden |
| Rogue | bounded class progression | `blocked` | `Computed` | GE-06 total-save test explicitly claim-blocks `class:rogue:1` | create class slice and replace current pilot-only chassis gate with honest Rogue evidence |
| Barbarian | bounded class progression | `unverified` | `Observed` | named by SD-13 scope only | create martial-class progression slice and classify real milestone behavior |
| Bard | bounded class progression and spell burden | `unverified` | `Observed` | named by SD-13 scope only | create spellcasting-class slice and classify real burden |
| Cleric | bounded class progression and spell burden | `unverified` | `Observed` | named by SD-13 scope only | create spellcasting-class slice and classify real burden |
| Druid | bounded class progression and spell burden | `unverified` | `Observed` | named by SD-13 scope only | create spellcasting-class slice and classify real burden |
| Monk | bounded class progression | `unverified` | `Observed` | named by SD-13 scope only | create martial-class progression slice and classify real milestone behavior |
| Paladin | bounded class progression and hybrid spell burden | `unverified` | `Observed` | named by SD-13 scope only | classify chassis burden first, then spell burden explicitly |
| Ranger | bounded class progression and hybrid spell burden | `unverified` | `Observed` | named by SD-13 scope only | classify chassis burden first, then spell burden explicitly |
| Sorcerer | bounded class progression and spell burden | `unverified` | `Observed` | named by SD-13 scope only | create spellcasting-class slice and classify real burden |
| Wizard | bounded class progression and spell burden | `unverified` | `Observed` | named by SD-13 scope only | create spellcasting-class slice and classify real burden |

### Interaction rows
| Subject | Dimension | State | Evidence tier | Grounding | Next required uplift |
|---|---|---|---|---|---|
| Human bonus feat / ability-bonus seam | race/class interaction pressure on the deterministic pilot path | `partial` | `Computed` | deterministic GE-06 Human Fighter fixture carries `human_bonus_feat` and `human_ability_bonus` selections | generalize the interaction-row model beyond the current pilot and classify what still remains missing |
| non-Human race with any class progression slice | race/class interaction pressure beyond the pilot | `unverified` | `Observed` | no accepted repo evidence yet | add named interaction rows only where separate race and class rows are insufficient |

## Breadth-claim gate
A future claim such as “core Fighter support” or “core Human + Fighter path is supported” may be made only when:
1. the relevant race row is acceptable for the claim
2. the relevant class row is acceptable for the claim
3. any required interaction row is acceptable for the claim
4. visible debt remains visible where any participating row is not `supported`

## Prohibited interpretations
- a full roster list is not support
- a `Computed` row is not necessarily `supported`
- `Product-visible` without matrix alignment is counterfeit breadth
- `lossy` may be useful operationally, but it is not breadth completion
