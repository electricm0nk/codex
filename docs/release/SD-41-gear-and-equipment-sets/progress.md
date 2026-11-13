---
canonical: true
bundle_id: SD-41
slug: gear-and-equipment-sets
canonical_branch: tranche-21
target_version: 0.21.0
date: 2026-11-12
status: planning-ready
---

# SD-41 Progress

## Summary

| Field | Value |
|---|---|
| Cards complete | 1 of 35 (`awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/ && $5 ~ /^ complete *$/' kanban.md \| awk 'END{print NR}'` → 1; total `awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/' kanban.md \| awk 'END{print NR}'` → 35: 23 at C0 + 8 from the seven follow-ups + 3 from the campaign item + E1.11, `decisions.md §22–§30`) |
| Phase | 1 (C1, E1.x may run now beside SD-40; E2.1 waits for `SD-40 merged`, `decisions.md §4`) |
| Run id | — (written at launch, `workflow-instruction.md §2.4` item 9) |
| Pinned `tranche-21` SHA | `f7f85f3d24` at the cut (`origin/tranche-20`'s tip, 2026-11-12); C1's version commit replaces it |

## Run handle

(the lead writes the run id and script path here the moment the run starts; after E1.MC and after E4.G
the resume `START=` card)

| Run | Started | Start card | Script | Outcome |
|---|---|---|---|---|
| — | — | C1 | `artifacts/cycle_0/sd41-workflow.js` (written at launch) | — |

## Cycle 0 — package authored (2026-11-12, planning session, Fable 5.1)

- Source: the lead's brief carrying the operator's request and nine answers
  (`artifacts/cycle_0/operator-brief-2026-11-12.md`).
- Repo facts checked on `tranche-20` @ `f7f85f3d24` by `artifacts/epic_0/census.py` (output
  `artifacts/epic_0/census.txt`): three states and no stored slot; 97 `EquipmentSelection` producers; 18
  slots; `equipment=` lines in `authoritative_character_input.txt`; AC/weight/weapons keyed on
  `ActiveState` at the three lines F-10 names; the market requests `'EquippedActive'`; no drag; no
  container or capacity read; 232 container-tagged sheet rules; 138 `CONTAINS` records (70 / 43 / 25);
  custom items have no weight or base; 174 smoke rows, 31 on gear, no drag op; SD-40 at 32 of 56 with
  the overlap in `census.txt §7`.
- PCGen read for function only: the Equipping and Purchase sub-tab docs under the pinned oracle
  (`decisions.md §17`; described in `technical-design.md §1`).
- Author calls without an operator ruling: `README.md §6` rows 1–9; the five operator questions with
  defaults: `decisions.md §21`.
- Brief corrections: `decisions.md §20` (DTO line numbers; slots live in `body_slots.rs`; two container
  sources; the on-disk format; custom items' missing fields).
- **UNATTENDED MODE after launch:** `decisions.md §12`; E4.G is the one planned wait; the Phase-1 run ends
  at E1.MC by design.
- Peer tree `.worktrees/sd40-main` present and untouched; `git worktree list` on 2026-11-12: the main
  checkout on `develop` @ `2b14020bc7`, `appimage-self-install`, `readme`, `sd40-main` @ `f7f85f3d24`,
  `sd41-main` @ `f7f85f3d24`.

### Placeholder gate output (author, 2026-11-12)

Command: `grep -rhoE '<[a-z_-]+>' docs/release/SD-41-gear-and-equipment-sets/*.md | sort | uniq -c | sort -rn`
run before commit; the list is pasted in `artifacts/cycle_0/C0_cycle_receipt.md`. Every token is in the
`workflow-instruction.md §9` allowed list; zero bugs. Deferred values: `0.21.<run>`; the run id; the
SD-40 merge SHA (R25); F-24's `N of N` (E1.2).

## Cycle 0 amendment — the seven follow-ups (2026-11-12, planning session, docs only)

The operator sent seven more items the same day (verbatim in `artifacts/cycle_0/operator-brief-2026-11-12.md`
and `decisions.md §22–§28`). Census added to `census.py §8` (unarmed, ammunition tags, Market shape, prices
by kind / book / category with the two named items' converter causes, eqmods, bundles, Drop surfaces). New
cards: E1.5, E1.6, E1.7, E1.8, E1.9 (Phase 1; E1.8–E1.9 in the tool-side lane `convert`), E2.5, E2.6, E3.4;
E2.3 rewritten (Create item backend; `base_item_id` dropped — `decisions.md §21` Q3 answered); E2.1, E2.2,
E2.4, E3.1, E3.3, E4.1, E4.MC, E4.G widened. Cards 23 → 31. Open questions Q6–Q8 added with defaults.
Partition re-run: the Market cards touch `CharacterSheet.tsx` (SD-40-owned) → Phase 2; the `convert` lane's
files are owned by no open SD-40 card → Phase 1.

**Eighth item, same day (campaign folders, `decisions.md §29`):** census §9 (171 `SavedCharacterStore::save`
sites, six sidecar writers, the Drive folder lives UI-side, `partyCharacterIds` written by `EditCampaignScreen`,
the Manager's route, 5 campaign smoke rows). New cards E1.10 (root-crate mirror, Phase 1), E2.7 (writer hooks +
scanning test + `set_character_campaigns`), E3.5 (Campaign field dialog + per-campaign buttons); E3.3 → 18 new
rows. Cards 31 → 34. Q9 added. No open SD-40 card owns `src/campaign/**`, `campaign_drive.rs`,
`CharacterHubPage.tsx` or `LandingScreen.tsx` (census §9's last command → nothing).

**Operator answers Q6–Q8, same day (`decisions.md §30`):** Q6 default stands; Q7 "Every Market item" → E1.11
(lane `convert`, batches of ≤ 150 via the lead's `web-fetch` agent) and a closure criterion in G-8/G-10; Q8
"Count single arrows" → pieces per stack in the sidecar's `copies` map, migration on first load (E2.1),
`compute_encumbrance` quantities (E1.7), buy by bundle → pieces, sell / move / discard by pieces (E2.2, E2.5).
Cards 34 → 35. Partition unchanged: E1.11 is root/tool side (Phase 1; its `equipment_catalog.rs` filter is a
named exception the receipt states, or moves to E2.5).

## Cycle log

| Card | Date | SHA | Status | Receipt |
|---|---|---|---|---|
| C0 | 2026-11-12 | this commit | complete | `artifacts/cycle_0/C0_cycle_receipt.md` |

## Epic reports

(written by each merge-check card: E1.MC, E2.MC, E3.MC, E4.MC)

## Lead's dispatch log

(one line per dispatch: card, R21 RED output path, R22 grep, `git reflog -3` + `git worktree list` read;
the R25 line before E2.1: the SD-40 merge SHA and `git log origin/develop -1 --format=%s`)

## Open blockers

(none)
