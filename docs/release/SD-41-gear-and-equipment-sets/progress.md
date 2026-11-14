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
| Cards complete | 18 of 46 (`awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/ && $5 ~ /^ complete *$/' kanban.md \| awk 'END{print NR}'` → 18 (sd41-e1.mc; 17 at E1.15, which left this cell at 16; 16 at E1.13; 15 at E1.14; 14 at E1.12; 13 at E1.11; 12 at E1.9, E1.10 landed beside it); total `awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/' kanban.md \| awk 'END{print NR}'` → 46 (sd41-e1.11, 2026-11-13: + E2.11 discovered by E1.11; 45 at E1.9: + E1.15 and E2.10 discovered by E1.9; earlier 37 + E1.13 and E2.9 discovered by E1.6; the earlier itemization: 23 at C0 + 8 from the seven follow-ups + 3 from the campaign item + E1.11, `decisions.md §22–§30`, + E2.8 discovered by E1.8, + E1.12, `decisions.md §32`) |
| Phase | 1 (C1, E1.x may run now beside SD-40; E2.1 waits for `SD-40 merged`, `decisions.md §4`) |
| Run id | — (written at launch, `workflow-instruction.md §2.4` item 9) |
| Pinned `tranche-21` SHA | `f7f85f3d24` at the cut (`origin/tranche-20`'s tip, 2026-11-12); C1's version commit replaces it |

## Run handle

(the lead writes the run id and script path here the moment the run starts; after E1.MC and after E4.G
the resume `START=` card)

| Run | Started | Start card | Script | Outcome |
|---|---|---|---|---|
| — | — | C1 | `artifacts/cycle_0/sd41-workflow.js` (written at launch) | — |
| Phase 1 | 2026-11-12 | C1 | `artifacts/cycle_0/sd41-workflow.js` | ended 2026-11-14 at E1.MC (`475a700452` + its docs commit): every Phase-1 card complete. **Resume receipt: `START=E2.1`**, only after SD-40 (PR #406) merges to `develop` and `tranche-21` is rebased onto `develop` (R25: the lead records the SD-40 merge SHA and `git log origin/develop -1 --format=%s` in the dispatch log first); main lane `.worktrees/sd41-main`, `CARGO_TARGET_DIR=/home/ubuntu/cargo-target/sd41-main` (kept for Phase 2); no side worktree is open |

## Cycle 0 — package authored (2026-11-12, planning session, Fable 5.1)

- Source: the lead's brief carrying the operator's request and nine answers
  (`artifacts/cycle_0/operator-brief-2026-11-12.md`).
- Repo facts checked on `tranche-20` @ `f7f85f3d24` by `artifacts/epic_0/census.py` (output
  `artifacts/epic_0/census.txt`): three states and no stored slot; 97 `EquipmentSelection` producers; 18
  slots; `equipment=` lines in `authoritative_character_input.txt`; AC/weight/weapons keyed on
  `ActiveState` at the three lines F-10 names; the market requests `'EquippedActive'`; no drag; no
  container or capacity read; 232 container-tagged sheet rules; `CONTAINS` records 138 (70 / 43 / 25) — **corrected by E1.1 to 210 (88 / 73 / 49), `decisions.md §31` R3**;
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
Cards 34 → 35. Partition unchanged: E1.11 is tool side (Phase 1). **Lead's correction (§30.1):** an unpriced
row stays listed and buys through Negotiate (the operator's "allow the user to specify the price on purchase");
Q7 is the effort bar; the `market: false` filter is dropped (no desktop file in E1.11); `price-unknown.json`
rows carry `tried`; E2.5's test is `market_backend_unpriced_item_buys_through_negotiate`.

**Operator ruling, Backpack 60 lb + override switch (`decisions.md §32`):** discovered card E1.12 (convert
lane, after E1.1 and E1.9; a cited capacity override list, like the price fix; RED test settled capacity ==
60 lb); the per-character "Ignore container capacity limits" toggle added to E2.3 (Overrides field + Custom tab),
E3.1 and E4.1 (warning suppressed, override tagged); E3.3 +1 row. Cards 36 → 37. **Flag:** no PF1e book in the
pinned data prints 60 lb for the Backpack (CRB p.158 `CONTAINS:UNLIM|Any`; UE p.58 `.MOD` unchanged); the
override row's source kind is `operator-ruling`.

**Operator request, print page 1 (`decisions.md §34`, 2026-11-13):** cards E4.2 (title boxes, swap, empty
Conditional Bonuses box), E4.3 (skills table, weapons cleanup, speed 2×2 with Climb "—"), E4.4 (portrait image
else system logo; logo files = ruling request Q10). Phase 2, serial after E4.1. E4.G gains a third `usable: yes`
line (page 1). Census: a portrait store exists (`character_hub.rs:5744/5811/5817`, read by `print/mod.rs`);
no `speed.climb` row (Q11); no logo file in the repo. Cards 39 → 42.

**Operator rulings Q10 + Q11 (`decisions.md §35`, 2026-11-13):** logos in git (E4.4 fetches + cites; no
blocker); climb → discovered card E1.14 (root: `climb_ft` on `DeclaredMoveModes` from the race-trait `MOVE`
data, Vanara 20 ft cited, the constant removed); the `speed.climb` row push is desktop code
(`character_hub.rs:3158`), so it lands in E2.4 (Phase 2); E4.3 reads the row. Cards 42 → 43.

## Cycle log

| Card | Date | SHA | Status | Receipt |
|---|---|---|---|---|
| C0 | 2026-11-12 | this commit | complete | `artifacts/cycle_0/C0_cycle_receipt.md` |
| C1 | 2026-11-12 | this commit | complete | `artifacts/cycle_0/C1_cycle_receipt.md` |
| E1.8 | 2026-11-12 | this commit | complete | `artifacts/epic_1/E1.8_cycle_receipt.md` (census 1,763 of 5,880 Market rows unpriced; discovered E2.8) |
| E1.9 | 2026-11-13 | this commit | complete | `artifacts/epic_1/E1.9_cycle_receipt.md` (converter prices 1,433 records + 1,386 Market table rows; Market unpriced 1,763 → 397, all in `price-unknown.json`; discovered E1.15, E2.10) |
| E1.1 | 2026-11-12 | `370e3346b4` | blocked-escalated | `artifacts/epic_1/E1.1_cycle_receipt.md` |
| E1.1 | 2026-11-12 | `6d7b0227ba` + this commit | complete | `artifacts/epic_1/E1.1_cycle_receipt.md` |
| E1.2 | 2026-11-12 | `d3bdfe8ad6` + `fd85933387` + this commit | complete | `artifacts/epic_1/E1.2_cycle_receipt.md` (9 + 1 tests; 63 of 63 `.pcg` fixtures round-trip) |
| E1.3 | 2026-11-13 | `9841098a84` + this commit | complete | `artifacts/epic_1/E1.3_cycle_receipt.md` (4 tests; root lib 3106 passed) |
| E1.4 | 2026-11-13 | this commit (docs only) | declined (ownedBy E2.4) | `artifacts/epic_1/E1.4_cycle_receipt.md` (no code; the widening breaks a desktop test Phase 1 may not edit) |
| E1.4 | 2026-11-13 | `0a725d14bf` + this commit | complete | `artifacts/epic_1/E1.4_cycle_receipt.md` (4 tests; root lib 3110 passed, 6 ignored; codex-ingest 1862 passed over 179 suites; PF seeds byte-identical) |
| E1.5 | 2026-11-13 | `80b2ce8757` + this commit | complete | `artifacts/epic_1/E1.5_cycle_receipt.md` (4 tests + 3 E1.4 tests updated for the Unarmed row; root lib 3120 tests; PF seeds byte-identical; Aldric `1d3+4`) |
| E1.6 | 2026-11-13 | `3f38f51edf` + this commit | complete | `artifacts/epic_1/E1.6_cycle_receipt.md` (3 card tests + 1 population test + 1 converter test; root lib 3124 tests; codex-ingest 1,863 passed; PF seeds byte-identical; desktop fixture restored byte-identical; discovered E1.13, E2.9) |
| E1.7 | 2026-11-13 | `9050445ff0` + this commit | complete | `artifacts/epic_1/E1.7_cycle_receipt.md` (5 tests; root lib 3129 tests; codex-ingest 1,863 passed; 19 callers pass `&[], &[]`) |
| E1.10 | 2026-11-13 | `de3e5d35d4` + this commit | complete | `artifacts/epic_1/E1.10_cycle_receipt.md` (5 tests; root lib 3134 tests, 3128 passed, 6 ignored; `saved_character` 26 passed) |
| E1.11 | 2026-11-13 | this commit | complete | `artifacts/epic_1/E1.11_cycle_receipt.md` (397 unpriced Market rows in 3 batches: 41 priced from d20pfsrd with URL, 356 in `price-unknown.json` each with `tried`; Market unpriced 397 → 356; settler fence exit for `.COPY=` rows; 13 golden digests re-pinned; SD-27 absent-price pin takes the cited price; discovered E2.11) |
| E1.12 | 2026-11-13 | this commit | complete | `artifacts/epic_1/E1.12_cycle_receipt.md` (cited override list `data/capacity_overrides/container-capacities.json`, 1 row; CRB Backpack `Unlimited` → `Pounds(60.0)`; settled containers lb/unlimited 32/50 → 33/49; root 1 + ingest 2 tests; package grep command corrected, `decisions.md §20`) |
| E1.14 | 2026-11-13 | `7421facb1e` + this commit | partial | `artifacts/epic_1/E1.14_cycle_receipt.md` (`DeclaredMoveModes.climb_ft` settled from `MOVE:…,Climb,<ft>`: 4 of 919 race-trait records; Vanara 20, Human none, Tree Stranger none (3 of the 4 named tests); `VANARA_CLIMB_SPEED_FEET` removed, the seam reads the declared mode; remainder: the Skinwalker form-option test needs a ruling, Open blockers) |
| E1.14 | 2026-11-14 | `7421facb1e` + this commit | complete | `artifacts/epic_1/E1.14_cycle_receipt.md` (ruling (a), `decisions.md §36`: Skinwalker form climb is printed text; acceptance 4 → 3 passed; re-run `climb_speed` → 3 passed, 0 failed, 3135 filtered out) |
| E1.13 | 2026-11-14 | `0434560514` + this commit | complete | `artifacts/epic_1/E1.13_cycle_receipt.md` (4 CRB inherited bundles settle `base_quantity` 20/10/5/10; Darts was already 10 (`lst_token`), 5 → 4 corrected, `decisions.md §20`; only `raw_tokens` moved; longbow + 3 × `Arrows (20)` → 60; root lib 3133 passed; codex-ingest 1,876 passed; fixture sha unchanged) |
| E1.15 | 2026-11-14 | `a1a234fffe` + this commit | complete | `artifacts/epic_1/E1.15_cycle_receipt.md` (`pcgen-residue-gate` FAIL live_hits=5 → PASS live_hits=0; E1.8's classifier moved to `settle_equipment_prices`, which writes `E1.8/corpus-price-causes.json` and fails `--check` when it is stale; census `market_price` 3 passed (package said 2, corrected); `market-prices.json` byte-equal after `json.tool` but the predicate string) |
| E1.MC | 2026-11-14 | `475a700452` + this commit | complete | `artifacts/epic_1/E1.MC_cycle_receipt.md` (partition 0; epic field check `FIELDS_OK`; root lib 3134, ingest 1,877 over 180 suites, price gate 3 passed; clippy + residue gate PASS; container-load twin fixed; 3 acceptance counts corrected) |

## Epic reports

(written by each merge-check card: E1.MC, E2.MC, E3.MC, E4.MC)

### Epic report — E1 (E1.MC, 2026-11-14)

- **Cards:** 15 of 15 E1 cards complete (E1.1–E1.15) + C0, C1; board 18 of 46 complete (awk command in Summary).
- **What E1 delivered (root crate and tool side only):** containers settled (130 containers: 33 lb / 49 unlimited / 48 unstated; Backpack 60 lb by the cited override list); the equipment-sets model, file and projection (63 of 63 `.pcg` fixtures round-trip); container loads; the carried-weapon entry point, the Unarmed row, ammunition users and counts; extra weighed copies, piece quantities and bundle sizes (34 bundle records); the campaign mirror; Market prices (1,763 unpriced → 356, every one listed with `tried`; 41 cited d20pfsrd prices); `climb_ft` on 4 race-trait records; the census classifier moved tool side (residue gate PASS).
- **Merge-check verification on `origin/tranche-21` `04c06e33e7`:** root `--lib` 3133 passed / 0 failed / 6 ignored before, 3134 after the fix; `crates/codex-ingest` 1,877 passed / 0 failed / 43 ignored over 180 suites; desktop `market_price` 3 passed; sd24 audit 5 passed; `verify.sh --only clippy` and `--only pcgen-residue-gate` PASS; dual audit OK over `c817ad502f...HEAD`; partition → 0 (all 34 E1 commits); epic-wide corpus field check `FIELDS_OK`; fixture sha256 unchanged (`74bb85a4…`). Figures and commands: `artifacts/epic_1/E1.MC/verify-summary.txt`.
- **Found and fixed by the merge check:** (1) a twin — `container_loads` weighed every stack as one bundle and never weighed Create items, so 60 arrows + a 5 lb created item read 3.0 lb in a sack and 14.0 lb on the character; now both sums share `encumbrance::copy_bundles` (`475a700452`). (2) three acceptance counts drifted as later cards added tests under the same filter (`container_` 4 → 9, `base_quantity` 1 → 2, `containers_` 4 → 5); corrected, and `artifacts/epic_1/E1.MC/acceptance_filter_counts.py <lib log>` now checks every `--lib … → N passed` claim against one full log (exit 1 on a mismatch or a 0-match) — the R-W control for the repeated key below.
- **Retro (`python3 scripts/retro.py summary --since 2026-11-12T19:51:21-05:00`):** 58 events before this card — 37 corrections, 21 verifications; 0 incidents, 0 deferrals, 0 rework. Repeated subjects: `package` 7, `lead` 6 (8 corrections had already propagated when caught). Most were acceptance wording or counts (R21 RED shapes, filter counts) and census figures; the filter-count control above is the mechanism for the count class. E1.MC adds 3 corrections.
- **Carried to Phase 2 (owned, not open):** E2.1 calls `container_loads(set, held, corpus, created, weighed, quantities)`; E2.4 switches the desktop to the carried entry point + Unarmed rows + `speed.climb`; E2.7 switches savers to `save_with_warnings`; E2.9 regenerates the desktop fixture's ammunition fields; E2.10 / E2.11 Market follow-ups; the R23 grep predicate flags 8 of 15 receipts that state their populations in words (read, not a defect).
- **Sweep:** `sd41-plan` (clean, ancestor of `origin/tranche-21`, idle since 2026-11-13) removed with its local branch; the `convert` lane's tree and target dir were already gone; no other `sd41-*` side tree exists; `sd41-main` and every `sd40-*` tree untouched.
- **Phase-1 run ends here. Resume: `START=E2.1` after SD-40 merges and `tranche-21` is rebased onto `develop` (Run handle row).**

## Lead's dispatch log

(one line per dispatch: card, R21 RED output path, R22 grep, `git reflog -3` + `git worktree list` read;
the R25 line before E2.1: the SD-40 merge SHA and `git log origin/develop -1 --format=%s`)

## Open blockers

- ~~E1.14 Skinwalker climb~~ **closed 2026-11-14: ruling (a), `decisions.md §36` — the bestial-form climb prints as Change Shape text; `climb_speed_skinwalker_alternates` dropped; E1.14 acceptance `3 passed`; E1.14 complete.**

- ~~Q10~~ **closed 2026-11-13: the operator put the logos in git (`decisions.md §35`); E4.4 fetches them from the publishers' sources and cites them in `logos/SOURCES.md`; `LEAD_WEB_FETCH` on a failed fetch.** Original entry: logo files (ruling request, `decisions.md §34` (7), `§21` Q10): E4.4 prints the game
  system's logo in the portrait box when no portrait is stored; the repo is public and system logos are trademarks,
  so the operator supplies the three files (`apps/desktop/src-tauri/resources/print/logos/{pf1e,sf1e,cpr}.png`)
  or rules "no logo". Needed before E4.4's dispatch (Phase 2); E4.4 returns `blocked-escalated` on the files
  alone if they have not arrived. Not a Phase-1 blocker.

- ~~E1.1~~ **closed 2026-11-12 by the lead's rulings R1–R4 and the operator's "Container tag, but not weapons" (`decisions.md §31`); E1.1 reset to `ready`; E1.9 now depends on E1.1; figures corrected; census.py:41 fixed.** Original entry: needs (R1) FILES YOU OWN widened to `crates/codex-ingest/src/pcgen_import/ir_converter.rs` (`equipment_record_to_corpus`) + regenerated `data/corpus/*/_settled/equipment.json`, sequenced before E1.9; (R2) a replacement for SD-a (Pistol and Powder Horn DO carry the `Container` tag; default proposed: container = `Container` tag and no stated damage). Tried: the in-fence paths (the settled record has no tags/tokens; `raw_tokens` in live code is a residue-gate hit). Blocks E1.2–E1.7, E1.10 (serial lane). Receipt: `artifacts/epic_1/E1.1_cycle_receipt.md`.
- ~~E1.4~~ **closed 2026-11-13 by the lead's option (c) (`decisions.md §33`): an additive entry point `resolve_weapon_damage_breakdown_with_carried`; the old function and every existing test unchanged; E2.4 switches the desktop callers and marks carried rows; E4.1 the print. E1.4 reset to `ready`.** Original entry: needs one lead action: (a) grant E1.4 a named `apps/` exception for `apps/desktop/src-tauri/src/character_hub.rs` test `enhancement_rows_a_stowed_plus_one_adds_nothing` (lines 10720-10725: replace `is_empty()` with "one row, `in_hand == false`, enhancement +1") and widen FILES YOU OWN to `tests/worn_vs_carried.rs` (lines 122-123, 155) and `src/rules_core/corpus_loader.rs` (test, 634-636), accepting that carried weapons show unmarked on the Weapons tab and the PF print until E2.4/E4.1; or (b) fold E1.4 into E2.4 (desktop lane) and re-point E1.5's dependency. Tried: the in-fence path (every root-only widening of `resolve_weapon_damage_breakdown` turns that desktop test red; a separate unconsumed function would not be the wired widening the criterion names). Blocks E1.5–E1.7 (serial lane). Receipt: `artifacts/epic_1/E1.4_cycle_receipt.md`.
