---
canonical: true
owner: operator
bundle_id: SD-41
slug: gear-and-equipment-sets
canonical_branch: tranche-21
target_version: 0.21.0
date: 2026-11-12
status: planning-ready
---

# SD-41 Risks and Open Questions

## 1. Ranked risks

| # | Risk | Likelihood | Impact | Control | Self-healable? |
|---|---|---|---|---|---|
| R-1 | **An SD-41 card writes an SD-40-owned file before SD-40 merges** (two writers, `character_hub.rs` 21,905 lines) | medium | high | the partition (`decisions.md §4`): Phase 1 never writes under `apps/` (prefix check, G-3); E2.1 depends on `SD-40 merged` (R25); the prefix refuses `tranche-20` as the branch | no — a `blocked-escalated` and a retro incident |
| R-2 | The projection drifts from the engine's reading of `ActiveState` (a copy inside a Not Carried container still weighs) | low | high | one function, one table (`decisions.md §6`); the theorem test over the seeds and the `.pcg` population; E2.2's weight test on a nested container; G-1 | yes |
| R-3 | A move is saved without projecting (the file changes, the sheet does not) | medium | high | every set write goes through one `apply_set_change(root, f)` that projects, recomputes and saves (E2.2 names it); E2.MC reads every command for a save without a projection | yes |
| R-4 | A second weight sum appears in TS or Typst (a twin) | medium | high | NR-2; G-6's grep; the merge checks read the diffs; `containers.rs` is the only new sum | yes |
| R-5 | Copy identity by ordinal shifts when a copy is sold | medium | medium | `remove_equipment_selection` re-ordinals every set (E2.2); a test sells the first of two daggers and checks the second's location survives | yes |
| R-6 | The `Container` tag and the `CONTAINS` token disagree (ammunition holders carry a token, no tag) | certain for 2+ records | low | the tag rules (SD-a); E1.1 pins Pistol as not a container; the receipt lists the token-without-tag set with its count | yes |
| R-7 | Capacity is read from `data/corpus` but the packaged app ships only what `gen-corpus-bundle` copies | medium | high | E1.1 names the read path; E5.1's `packaged_resources` + `tauri-resources-tracked` stages; a capacity that reads `Unstated` in the packaged app for Backpack is a defect E5.1 catches by a packaged-build probe (the receipt names the command) | yes |
| R-8 | ui-smoke cannot drag | certain | low | every row uses Move to; `dropTarget` is a pure tested function; E4.G's eye (`decisions.md §14`) | n/a |
| R-9 | The `run-desktop` screenshot path fails on this box (no display) | medium | medium | SD-l: the skill's own headless path with an isolated `XDG_DATA_HOME`; if it cannot run, E3.MC records the command and the error and the operator's own app is the only eye at E4.G (not a blocker: the smoke rows prove the function) | partly |
| R-10 | A `.pcg` import holds a shape the Default migration mishandles (two copies of one item, a custom slot, an `Absent` shield loadout row) | medium | medium | the population test over every fixture (E1.2); `is_owned_copy`'s loadout rule reused; E2.MC's probe loads every imported root | yes |
| R-11 | An acceptance command passes on the base (cannot fail) | high (SD-39 retro) | medium | R21: the lead runs each on the pre-change tree before dispatch | yes |
| R-12 | FILES YOU OWN misses a second call site (`EquipmentSelection` has 97 producers; `WeaponDamageBreakdown` several) | medium | low | R22 pre-dispatch `git grep`; NR-3 keeps `EquipmentSelection` untouched so the 97 never matter | yes |
| R-13 | SD-40's retrospective lands after this package and names a change SD-41 ignores | medium | low | §12.2 of `workflow-instruction.md`: the lead reads it before E2.1 and records each change | yes — lead action |
| R-14 | Three cargo lanes on the box with SD-40 live → OOM / Proxmox stop | medium | high | R-Q2: the prefix waits on `pgrep` < 3; `free -g` before long runs | yes |
| R-15 | The operator's gate finds defects → the run must stop and restart | expected | low | gate defects are discovered cards; `START=<card>` restart; one wait by design | n/a |
| R-16 | The equipment re-convert (E1.9) rewrites fields it must not (licence, PI, population — the "generated artifacts mutated post-hoc" hazard) or thousands of files SD-40's merge later conflicts with | medium | high | the field-diff Python check (only `cost_gp`, `weight`, `name`, `raw_tokens` may change); no open SD-40 card owns `data/`; E1.MC re-checks; the lane lands before E1.MC so the Phase-2 rebase carries it | yes |
| R-17 | d20pfsrd unreachable from this box (HEAD → 410 on 2026-11-12) and the lookup is now a closure criterion for every Market row (Q7) | high | high | P-9: a browser user agent first; else the lead's `web-fetch` agent per batch of ≤ 150 (E1.11); the count is fixed by E1.8 before any fetch; a row no page prices stays listed with a `tried` reason and buys through Negotiate (§30.1), so the gate can still pass | partly — the lead dispatches the agent |
| R-18 | Buy Custom prices drift from PCGen's eqmod formulas (97 formula kinds) | medium | medium | only flat costs and the named formula kinds are priced; the rest route through Negotiate (Q6) and are counted in the receipt; the test prices a known +1 flaming mithral longsword against the book | yes |
| R-19 | Stacks hide a copy-identity bug (a move of 2 of 3 moves the wrong ordinals) | medium | medium | stacks are a view; moves act on explicit `CopyKey`s in ascending ordinal; E2.2 `move_quantity_splits_a_stack`; E2.1's stack tests | yes |
| R-20 | The Unarmed row changes an existing seed value (a Monk seed's flurry rows) | low | high | the row is appended; G-1's `NEW_ROWS` names exactly the four unarmed ids; any other change is `VALUE_MOVED` | yes |
| R-21 | A character writer is missed by the campaign mirror (171 `SavedCharacterStore::save` sites; six sidecar writers), so the campaign copy goes stale silently | high without a control | medium | the mirror sits inside `SavedCharacterStore::save` (one place for the 171); the source-scanning test `every_character_root_writer_mirrors` fails on any sidecar writer without `after_write` (E2.7); the end-to-end test changes `custom.json` and reads the copy | yes |
| R-22 | A campaign folder on a slow or absent Drive mount makes every save slow or fail | medium | medium | the mirror runs after the local save and returns warnings, never an error; a missing dir is one warning per save shown on the sheet; a membership can be set inactive | yes |
| R-23 | The sheet's membership dialog and the Campaign Manager disagree on `partyCharacterIds` (nonce conflict) | low | low | the dialog writes through the existing `drive_save_campaign` with its nonce (E2.10) and shows its conflict message; the character's `campaigns.json` is the truth the mirror reads (Q9) | yes |

## 2. Open questions → defaults taken

The five questions only the operator can settle are `decisions.md §21` (Q1–Q5), each with the default
one card takes; the run does not wait on them.

| Question | Default | Where |
|---|---|---|
| Package path `docs/release` vs `docs/stc` | `docs/release` | `decisions.md §1` |
| Where sets are stored | a sidecar file; projection into `active_state` | `decisions.md §5, §6` |
| What an unnamed copy is in a set | Carried | `decisions.md §5.3`; Q1 |
| Carried weapons' attack rows | yes, after the in-hand rows | `decisions.md §9`; Q2 |
| A custom container's base | `base_item_id` required for weight and container-ness | `decisions.md §11`; Q3 |
| Weight-reduction containers | "no stated capacity"; contents weigh in full | DEF-1; Q4 |
| Where the gear screen lives | the Gear tab, replacing the Equipment panel | `decisions.md §13`; Q5 |
| Drag library | none (HTML5) | SD-g |
| Buy Custom: unpriced eqmods | sold through Negotiate | `decisions.md §24`; Q6 |
| The d20pfsrd list's reach | **answered: every Market item** (closure criterion) | `decisions.md §30`; Q7 |
| Splitting a bundle | **answered: count single arrows** (pieces in the sidecar) | `decisions.md §30`; Q8 |
| Where a Create item is edited | the Custom tab | `decisions.md §25` |
| Campaign membership truth | the character's `campaigns.json`; the party list kept in step by the dialog | `decisions.md §29`; Q9 |

## 3. Non-self-healable conditions (card → `blocked-escalated`)

Two live lanes on one file; a Phase-1 write under `apps/`; RED→GREEN not preserved; a stub, mock or
"Would …" string in shipping code; a `VALUE_MOVED` seed no named key explains; an SF1e/CPR golden text
delta a card cannot remove; the operator's gate verdict "not usable" (becomes discovered cards, then the
gate re-runs); `CLAIM-EXISTS`.
