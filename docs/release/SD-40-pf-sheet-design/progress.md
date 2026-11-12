---
canonical: true
bundle_id: SD-40
slug: pf-sheet-design
canonical_branch: tranche-20
target_version: 0.20.0
date: 2026-11-07
status: planning-ready
---

# SD-40 Progress

## Summary

| Field | Value |
|---|---|
| Cards complete | 25 of 54 (→ 25 at E2.27; `awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/ && $5 ~ /^ complete *$/' kanban.md \| awk 'END{print NR}'` → 24 at E2.MC2 (E2.15, E2.12, E2.13, E2.4 and E2.MC2 since E2.11); total `awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/' kanban.md \| awk 'END{print NR}'` → 54: E2.15 discovered by E2.7, + 10 by the E2.G round-2 replan + 3 UI cards + E2.27, `decisions.md §24`) |
| Run id | — (written at launch, `workflow-instruction.md §2.4` item 9) |
| Pinned `tranche-20` SHA | `3ceeb63b6622061d9dde1e0068df4b3d4065d626` (C1 version commit, pushed 2026-11-07; was `d4489b3557a1fa6f59be53ffc16561990c7e6715` at the cut) |

## Run handle

(the lead writes the run id and script path here the moment the run starts; after each gate the
resume `START=` card)

| Run | Started | Start card | Script | Outcome |
|---|---|---|---|---|
| wf_00f61ec5-013 | 2026-11-07 | C1 | `artifacts/cycle_0/sd40-workflow.js` (session copy in the lead's scratchpad) | running; stops at E2.G |

## Cycle 0 — package authored (2026-11-07, planning session, Fable 5.1)

- Source: the lead's brief carrying the operator's seven rulings (`artifacts/cycle_0/operator-brief-2026-11-07.md`).
- Repo facts checked on `tranche-19` @ `d4489b3557`: 16 block builders, 58 Typst lines in two templates,
  one generic table per block, no grid, 24 text goldens, `Paper::Letter` already present, the dialog's
  three checkboxes, `SheetBoxId` 13 / `SHEET_TABS` 8, the PCGen installed launcher runs headlessly
  (`./pcgen --help`), the oracle on pin, poppler present, 14 version surfaces by `git show 49d598c13a`.
- Author calls without an operator ruling: `README.md §6` rows 1–10; rows 5, 6 and 9
  were the open questions, answered by the operator on 2026-11-07 (below).
- Brief corrections: `decisions.md §21` (Letter already exists; 16 builders not 15; the bare jar does not
  run; the version grep's lockfile false positives).
- **UNATTENDED MODE after launch:** `decisions.md §13`; E2.G and E5.2 are the two planned waits.
- Peer tree `.worktrees/sd39-main` present and untouched (`decisions.md §20`).

### Placeholder gate output (author, 2026-11-07)

Command: `grep -rhoE '<[a-z_-]+>' docs/release/SD-40-pf-sheet-design/*.md | sort | uniq -c | sort -rn`
run before commit; the list is pasted in `artifacts/cycle_0/C0_cycle_receipt.md`. Every token is in the
`workflow-instruction.md §9` allowed list; zero bugs. Deferred values: `(E0.1)` figures in
`content-unit-inventory.md` F-13, F-14; `0.20.<run>`; the run id.

## Operator answers (2026-11-07)

`decisions.md §22`: "1: B" (card E0.2 owns the 14 stale ui-smoke rows; E4.2 waits on it), "2: A"
(Spellbook prints spell pages only; Full prints everything), "3: No hard limits" (no page-count
ceiling; page counts recorded per receipt; the operator's eye at E2.G and E5.2). Cards 23 → 24
(`awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'`).

## 2026-11-11 E2.G round 1 replan (planning lane, Opus 5.5; docs only, no code)

- **Input:** the operator's round-1 list (items 1–6, item 6 answer "commas are fine"), quoted verbatim in
  `decisions.md §23.1`. Round-1 verdict in `artifacts/epic_2/E2.G_cycle_receipt.md` unchanged; round-2
  `usable:` lines blank.
- **Rulings recorded:** page 1 totals only (AC parts the exception, filled from engine rows); one square block
  (HP, initiative, encumbrance, conditions) with engine-gathered conditional notes; one-column skills;
  languages; portrait + Character Name + Campaign, no title, no identity block; six print-section checkboxes
  replace the four presets (`decisions.md §9` superseded), remembered per character, first time 1+2+3.
- **Repo facts found (each with its command in `decisions.md §23` / `epic-breakdown.md`):** the engine
  states no AC part rows (parts exist only as terms of `combat.rs`'s sum); no damage total (deliberate,
  `contract.rs:215-226`); remaining lb and languages exist only in TypeScript (`encumbranceTabModel.ts:75`,
  `CharacterSheet.tsx:418`); conditional HP / initiative / save / AC lines exist in the converted package
  (census F-28, F-29: `python3 -I docs/release/SD-40-pf-sheet-design/artifacts/epic_2/replan-r1/census.py`);
  no carrying-capacity bonus target; no bio/backstory text field.
- **Cards:** 26 → 38. Rewritten: E2.4 (page 1's overflow rule, `decisions.md §23.5`), E2.5 (header), E3.1,
  E3.2, E3.3, E4.1, E4.2, E4.MC, E5.2. New: E2.6, E2.7, E2.8, E2.14 (side lane `engine`), E2.9–E2.13,
  E2.MC2, E3.4, E3.5. E2.G now depends on E2.MC2. `SAME_IDS` check passes.
- **Open operator questions with defaults taken:** `decisions.md §23.6` Q1–Q5.
- **Parked branch:** `sd40/e2.4-parked` (`8f75f60b3e`) kept until E2.4 is `complete`; then the lead deletes it
  locally and on origin (G-8 checks).
- **Operator answers, same day (`decisions.md §23.7`):** Q5 "One total" (E2.14 kept); Q1 "Known + one blank
  line" (E2.8 drops the slot count, E2.13 prints exactly one blank line); Q2 "Add a Backstory field now" (new
  card E3.6, before E3.4; FSR-C7 withdrawn); Q3 as proposed; Q4 keep; smaller defaults accepted. Cards 38 → 39.

## Cycle log

| Cycle | Card | Status | SHA | Receipt |
|---|---|---|---|---|
| C0 | C0 | complete | this commit | artifacts/cycle_0/C0_cycle_receipt.md |
| C1 | C1 | complete | 3ceeb63b66 | artifacts/cycle_1/C1_cycle_receipt.md |
| E0.1 | E0.1 | complete | this commit | artifacts/epic_0/E0.1_cycle_receipt.md |
| E1.1 | E1.1 | complete | ad412a4a88 + receipt commit | artifacts/epic_1/E1.1_cycle_receipt.md |
| E1.2 | E1.2 | complete | (this commit) | artifacts/epic_1/E1.2_cycle_receipt.md |
| E0.2 | E0.2 | complete | 02b9fa99da + receipt commit | artifacts/epic_0/E0.2_cycle_receipt.md |
| E1.MC | E1.MC | complete | a537c27737 + receipt commit | artifacts/epic_1/E1.MC_cycle_receipt.md |
| E2.1 | E2.1 | complete | 9b3edac157 + receipt commit | artifacts/epic_2/E2.1_cycle_receipt.md |
| E2.2 | E2.2 | complete | 972b033a81 + receipt commit | artifacts/epic_2/E2.2_cycle_receipt.md |
| E2.3 | E2.3 | complete | 0d8465c13f + 7d8983278b + receipt commit | artifacts/epic_2/E2.3_cycle_receipt.md |
| E2.MC | E2.MC | complete | f5e4c781b2 + receipt commit | artifacts/epic_2/E2.MC_cycle_receipt.md |
| E2.G | E2.G | partial | (this commit) | artifacts/epic_2/E2.G_cycle_receipt.md |
| E2.5 | E2.5 | complete | f6bc0cd545 + receipt commit | artifacts/epic_2/E2.5_cycle_receipt.md |
| E2.6 | E2.6 | complete | 3cfa76522c + abb7effa16 + 0519db3cd5 + receipt commit | artifacts/epic_2/E2.6_cycle_receipt.md |
| E2.7 | E2.7 | complete | a957b928e5 + d945250ec1 + receipt commit | artifacts/epic_2/E2.7_cycle_receipt.md |
| E2.8 | E2.8 | complete | 41f2385895 + receipt commit | artifacts/epic_2/E2.8_cycle_receipt.md |
| E2.9 | E2.9 | complete | 2553faacfa + receipt commit | artifacts/epic_2/E2.9_cycle_receipt.md |
| E2.14 | E2.14 | complete | 11902abf50 + 7903fc761d + fab4af8a19 + acdbbea41d + receipt commit | artifacts/epic_2/E2.14_cycle_receipt.md |
| E2.10 | E2.10 | complete | ee6661cb82 + receipt commit | artifacts/epic_2/E2.10_cycle_receipt.md |
| E2.11 | E2.11 | complete | 7769943981 + receipt commit | artifacts/epic_2/E2.11_cycle_receipt.md |
| E2.15 | E2.15 | complete | 5af77cd78d + receipt commit | artifacts/epic_2/E2.15_cycle_receipt.md |
| E2.12 | E2.12 | complete | b3fa6a3df6 + receipt commit | artifacts/epic_2/E2.12_cycle_receipt.md |
| E2.13 | E2.13 | complete | b26684e4f1 + receipt commit | artifacts/epic_2/E2.13_cycle_receipt.md |
| E2.4 | E2.4 | complete | 60a5f93776 + receipt commit | artifacts/epic_2/E2.4_cycle_receipt.md |
| E2.MC2 | E2.MC2 | complete | (this commit, docs only) | artifacts/epic_2/E2.MC2_cycle_receipt.md |
| E2.27 | E2.27 | complete | 9c6e14cf24 + receipt commit | artifacts/epic_2/E2.27_cycle_receipt.md |
| E2.16 | E2.16 | complete | 86b1cc2872 + receipt commit | artifacts/epic_2/E2.16_cycle_receipt.md |

## Epic reports

(the merge-check card of each epic appends its block here: cards, refuted claims and fixes, golden
deltas, what was looked at, retro summary, worktrees swept)

### Epic report — E1 (model + grid), by E1.MC, 2026-11-07

- **Cards done:** E1.1, E1.2, E1.MC — 3 of 3 (`awk -F'|' '$2 ~ /^ E1\./ && $5 ~ /^ complete *$/' kanban.md | awk 'END{print NR}'` → 3). Dependencies E0.1, C1 complete.
- **Re-derived on origin/tranche-20 `ce077e4466`:** `cd apps/desktop/src-tauri && cargo test --locked -j 8 print_ -- --test-threads=8` → 65 passed, 0 failed (`awk '/^test .* \.\.\. ok$/{n++} END{print n}'` and `grep -c ' \.\.\. ok$'` → 65, 65); desktop full `--no-fail-fast` → 1244 + 2 passed, 0 `FAILED`; NR-2 `grep -rnE '\b(calc|eval)\(' apps/desktop/src-tauri/resources/print/ | awk 'END{print NR}'` → 0; vocabulary 30 ids; TS vocabulary test exit 0; rasterize 130 PNGs from 16 PDFs, exit 2 on an empty dir.
- **G-1:** C1 `3ceeb63b66` built in its own tree and target dir; `diff base/sha256.txt head/sha256.txt` → empty (3 PF seeds).
- **G-2:** SF1e ×8 + CPR ×8: text and page loop → no output; `sha256sum` base vs head → 16 of 16 byte-identical; `sf1e.rs`/`cpr.rs` diff since C1 → 0 lines; `print_golden.json` diff since C1 → 0 lines.
- **Page counts per golden case (C1 → E1 head, `pdfinfo <pdf> | awk '/Pages/{print $2}'`):** PF (Elowen) a4-none 5 → 5, a4-spellbook 6 → 6, a4-math 12 → 12, a4-all 14 → 14, letter-none 5 → 5, letter-spellbook 6 → 6, letter-math 12 → 12, letter-all 14 → 14. SF1e a4/letter: none 8, spellbook 12, math 12, all 17 (unchanged). CPR a4/letter: none 4, spellbook 5, math 10, all 12 (unchanged). Proof harness Aldric: none 4/4, spellbook 5/5, math 8/9, all 10/11 (A4/Letter).
- **Population probe:** PF seeds 3 of 3 × 2 papers × 4 option sets = 24 renders on the grid, 0 diagnostics (new standing test `print_smoke_every_pf_seed_renders_through_the_grid_with_no_diagnostics`). `.pcg` imports: 65 files (13 repo + 52 oracle) → 37 saved, 26 engine-blocked, 2 converter-refused; 296 renders, all `columns=12`, 0 diagnostics (one-off scratch-tree probe, `artifacts/epic_1/E1.MC/pcg_probe.tsv`).
- **Refuted claims and fixes:** the E1.MC criterion's `.pcg` fixture path (`crates/codex-ingest/tests` holds 0) → corrected in `epic-breakdown.md`; E1.1's untested `check_places` branches → test added, mutation-proved; 2-of-3 seed coverage → 3 of 3. No E1.1/E1.2 code claim refuted.
- **Looked at:** `artifacts/epic_1/E1.MC/*.png` beside `artifacts/epic_0/E0.1/aldric-pcgen-std-1.png`: the grid row prints clean on both papers; what is left (generic table look, blank under Abilities, no write-in space, abilities on the right) is E2's.
- **Retro summary:** `python3 scripts/retro.py summary --since 2026-11-07 --actor sd40-e1.1 --actor sd40-e1.2 --actor sd40-e1.mc` → 4 events, all `correction`; subjects: E1.1 FILES YOU OWN (packaged_resources.rs), decisions §4 `columns` field, the rasterize path in §6 step 8/E1.2, the E1.MC fixture path. No recurrence key fired more than once (none set). The category recurs: 4 of 4 E1 corrections (and E0's 2) are this package's own wording, caught before implementation; no new control (R-W threshold is a key firing more than twice).
- **Open blockers:** none.
- **Worktrees swept:** `.worktrees/sd40-e1mc-base` (E1.MC's scratch base/probe tree) and `/home/ubuntu/cargo-target/sd40-e1mc-base`; `/home/ubuntu/cargo-target/sd40-proof` and `empty-dir` (E1.2's proof dirs). No other `sd40-*` side worktree existed (`git worktree list`). `sd40-main` kept.

### Epic report — E2 (page-1 looks), by E2.MC, 2026-11-08

- **Cards done:** E2.1, E2.2, E2.3, E2.MC — 4 of 4 planned (`awk -F'|' '$2 ~ /^ E2\.(1|2|3|MC) / && $5 ~ /^ complete *$/' kanban.md | awk 'END{print NR}'` → 4). E2.G (operator gate) waits for the operator; **E2.4** (discovered here) waiting. Bundle: 11 of 25 complete (`awk -F'|' '$2 ~ /^ (C|E)[0-9]/ && $5 ~ /^ complete *$/' kanban.md | awk 'END{print NR}'`).
- **Re-derived on origin/tranche-20 `35fede0bd9`:** `cd apps/desktop/src-tauri && cargo test --locked -j 8 print_ -- --test-threads=8` → 69 + 1 passed, 0 failed (`awk '/^test .* \.\.\. ok$/{n++} END{print n}'` and `grep -c ' \.\.\. ok$'` → 70, 70). After E2.MC's test (`f5e4c781b2`): desktop full `--no-fail-fast` → 1250 + 2 passed, 0 `FAILED`; print_pf_blocks 23 / print_golden 24 / print_smoke 9 (`awk` over `desktop_full.log`); looks 10 (`ls …/looks/*.typ | awk 'END{print NR}'`); vocabulary 30 ids; TS vocabulary test exit 0; clippy 0 warnings; `verify.sh --only clippy` PASS; sd24 audit 5 passed.
- **G-1:** C1 `3ceeb63b66` built in its own tree (`.worktrees/sd40-e2mc-base`) and target dir; `diff base/seeds/sha256.txt head/seeds/sha256.txt` → empty (3 PF seeds).
- **G-2:** SF1e ×8 + CPR ×8 vs C1: text and page loop → no output; `sha256sum` → 16 of 16 byte-identical; `sf1e.rs`/`cpr.rs` diff since C1 → 0 lines; SF/CPR lines in `print_golden.json` since C1 → 0.
- **G-4:** `print_pf_blocks_page_one_holds_the_step_one_blocks_on_both_papers` ok (now also: nothing of page 1 runs over onto page 2). E2.G's PDFs and `usable: yes` are the gate's.
- **Page counts per golden case (C1 → E2 head, `pdfinfo <pdf> | awk '/Pages/{print $2}'`):** PF (Elowen) a4-none 5 → 4, a4-spellbook 6 → 5, a4-math 12 → 11, a4-all 14 → 13, letter-none 5 → 5, letter-spellbook 6 → 6, letter-math 12 → 12, letter-all 14 → 14. SF1e a4/letter: none 8, spellbook 12, math 12, all 17 (unchanged). CPR a4/letter: none 4, spellbook 5, math 10, all 12 (unchanged). Proof harness: Aldric A4 4/5/8/10, Letter 4/5/9/11; Elowen A4 4/5/11/13, Letter 5/6/12/14 (none/spellbook/math/all).
- **Population probe:** 3 PF seeds × 2 papers (page-1 test) + 3-weapon variant × 2 + `print_pf_blocks_page_one_stays_whole_with_a_bought_weapon_and_ring_with_no_ranks_and_for_elowen` (Aldric + Market Dagger + worn Ring of Protection +1; Aldric with no ranks; Elowen) × 2 papers: page 1 whole, Typst diagnostics 0. A fourth weapon line fails on Letter (Speed leaves page 1) → E2.4.
- **Refuted claims and fixes:** (1) E2.3's page-1 test passed a block split over pages 1–2 (mutation-proved) and its 3-weapon variant ignored Typst warnings → `assert_page_one_whole` + warnings assert; (2) NR-2's Typst grep could not match `calc.` → `technical-requirements.md` corrected (2 layout-only hits read: `weapons.typ` `calc.min`, `skills.typ` `int(`); (3) E3.1–E3.3 FILES YOU OWN lacked `resources/print/blocks.typ`, the omission that fired at E2.1, E2.2, E2.3 → rows fixed; (4) progress Summary "8 of 24" → 11 of 25; (5) 4+ weapon lines → discovered E2.4. No claim about E2's shipped values was refuted.
- **Looked at:** `artifacts/epic_2/E2.MC/{aldric,elowen}-{letter,a4}-none-1.png` beside `artifacts/epic_0/E0.1/aldric-pcgen-std-1.png`: ten blocks on page 1, clean; differences named in the receipt (AC part cells write-in: no engine rows, SD-n; Skills left vs PCGen right; Identity still the generic table, E3.3; Knowledge labels wrap at 8 pt on Letter, for the operator's eye).
- **Retro summary:** `python3 scripts/retro.py summary --since 2026-11-07 --actor sd40-e2.1 --actor sd40-e2.2 --actor sd40-e2.3 --actor sd40-e2.mc --json` → 15 events (correction 7, note 4, rework 1, verification 3 — the third is this card's `verify.sh --only clippy` run; 0 incidents, 0 deferrals, 0 recurrence keys). Category recurring: FILES YOU OWN not closed under `blocks.typ` (E2.1, E2.2, E2.3: > 2, R-W) → control: the E3 rows now name it, and `print_pf_blocks_every_look_has_its_template` already exits non-zero on a look the dispatcher lacks.
- **Open blockers:** none. Open cards: E2.4 (discovered, `waiting` on E2.MC → ready); E2.5 (portrait box, operator at E2.G round 1, 2026-11-11, `decisions.md §23`). (Both rewritten by the 2026-11-11 replan above.)
- **2026-11-11 E2.G round 1:** operator: "We need to include a spot for the portrait on the top left corner." → E2.5 carded by the lead; run restarts at E2.MC (drains E2.4, E2.5, re-runs E2.MC, re-renders E2.G).
- **2026-11-12 E2.G round 2:** PDFs + PNGs re-rendered from `7bb1b2c874` into `artifacts/epic_2/E2.G/` (2 PDFs, 8 PNGs); receipt holds two empty `usable:` lines; E2.G stays `partial` awaiting the operator.
- **Worktrees swept:** `.worktrees/sd40-e2mc-base` (E2.MC's C1 base tree) and `/home/ubuntu/cargo-target/sd40-e2mc-base`; no other `sd40-*` side worktree existed (`git worktree list`: the main checkout and `sd40-main`). `sd40-main` kept.

### Epic report — E2 round 2 (page 1 redesigned), by E2.MC2, 2026-11-12

- **Cards done:** E2.4, E2.5, E2.6, E2.7, E2.8, E2.9, E2.10, E2.11, E2.12, E2.13, E2.14, E2.15, E2.MC2 — 13 of 13 round-2 cards; all of E2 but the gate: 17 of 18 E2 rows complete (`awk -F'|' '$2 ~ /^ E2\./ && $5 ~ /^ complete *$/' kanban.md | awk 'END{print NR}'` → 17; E2.G `partial`, the operator's gate). Bundle: 24 of 40 complete (`awk -F'|' '$2 ~ /^ (C|E)[0-9]/ && $5 ~ /^ complete *$/' kanban.md | awk 'END{print NR}'`).
- **Re-derived on origin/tranche-20 `9629b14bd3`:** `cd apps/desktop/src-tauri && cargo test --locked -j 8 print_ -- --test-threads=8` → 81 + 1 passed (`awk '/^test .* \.\.\. ok$/{n++} END{print n}'` and `grep -c ' \.\.\. ok$'` → 82, 82); print_pf_blocks / print_golden / print_smoke 34 / 24 / 9; desktop full `--no-fail-fast` → 1269 + 2 passed, 0 `FAILED`; root `cargo test --locked -j 8 --lib` → 3085 passed, 0 `FAILED`; `npm run typecheck && npm test` → 191/191; looks 12; vocabulary 32 ids; TS vocabulary test exit 0; `verify.sh --only pcgen-residue-gate` PASS; `grep -L 'Looked at'` over E2.4/5/9–13 receipts → nothing.
- **G-1 (rewritten):** C1 `3ceeb63b66` in its own tree and target dir; the G-1 Python → SAME_EXCEPT_NAMED for aldric, elowen, tomas; the strip hides no pre-existing row (0 base rows under the two new prefixes; 0 ids lost).
- **G-2:** SF1e ×8 + CPR ×8: text and page loop → no output; 16 of 16 byte-identical; `sf1e.rs`/`cpr.rs` diff since C1 → 0 lines.
- **Page counts per golden case (C1 → E2 round-2 head, `pdfinfo <pdf> | awk '/Pages/{print $2}'`):** PF (Elowen) a4-none 5 → 5, a4-spellbook 6 → 6, a4-math 12 → 12, a4-all 14 → 14, letter-none 5 → 5, letter-spellbook 6 → 6, letter-math 12 → 12, letter-all 14 → 14. SF1e a4/letter: none 8, spellbook 12, math 12, all 17. CPR a4/letter: none 4, spellbook 5, math 10, all 12. Proof harness (none/spellbook/math/all): Aldric A4 and Letter 4/5/9/11; Elowen A4 and Letter 5/6/12/14.
- **Population probe:** 3 seeds × 2 papers + E2.MC's probes + E2.4's 6 stresses × 2 papers (standing tests, ok) + the `.pcg` imports: 65 files → 37 saved (26 blocked, 2 refused) × 2 papers = 74 page 1s, 0 with a diagnostic or a lost line or a block off page 1 (`artifacts/epic_2/E2.MC2/pcg_probe.tsv`; awk and Python agree), 0 needing a continuation (max 2 weapon lines). Closes E2.MC's `.pcg` non-coverage.
- **Refuted claims and fixes:** NR-2's Typst grep printed 2, not 0 (E2.5's and E2.4's layout-only `calc.min` in `looks/name-campaign.typ`, unnamed in their receipts) → `technical-requirements.md` NR-2 names both; retro correction. No claim about E2's shipped values was refuted.
- **Looked at:** `artifacts/epic_2/E2.MC2/*.png` (Aldric and Elowen page 1 on both papers, Aldric Letter page 2) beside `artifacts/epic_0/E0.1/aldric-pcgen-std-1.png` and `decisions.md §23.3`: every §23.3 item present and filled; differences named in the receipt (Melee +8 vs PCGen +7 is the engine's first-weapon Melee, unchanged; 8 pt skills column for the operator's eye; Languages is the engine's `Common`).
- **Retro summary:** `python3 scripts/retro.py summary --since 2026-11-11 --actor sd40-e2.4 … --actor sd40-e2.15 --actor sd40-e2.mc2 --json` (13 actors) → 21 events: correction 11, rework 3, verification 7; 0 incidents, 0 deferrals, 0 recurrence keys. Category: 10 of 11 corrections are this package's own criterion/acceptance/FILES-YOU-OWN wording (the 11th, `pf1_adapter.rs`'s AC row, was fixed in code by E2.7); "the PF seed check is desktop-side, not root `--lib`" fired twice (E2.8, E2.14), under R-W's > 2 threshold. The control that caught them is R21 (the lead runs each acceptance command before dispatch); no new control.
- **Open blockers:** none. Open card: E2.G round 2 (operator gate, `partial`).
- **Worktrees swept:** `.worktrees/sd40-e2mc2-base` (C1 base) and `.worktrees/sd40-e2mc2-probe` (scratch probe tree) with `/home/ubuntu/cargo-target/sd40-e2mc2-base` and `/home/ubuntu/cargo-target/sd40-e2mc2-probe`; no other `sd40-*` side worktree existed (`git worktree list`: main checkout, `appimage-self-install`, `readme`, `sd40-main`). `sd40-main` kept. Branch `sd40/e2.4-parked` (local and origin) still exists: the lead's to delete (`decisions.md §23.5`; E2.4 is complete).

## 2026-11-12 E2.G round 2 verdict and replan (planning lane, Opus 5.5; docs only, no code)

- **Input:** the operator's round-2 words and same-day answers, verbatim in `decisions.md §24.1`; both round-2
  sheets `usable: no` (`artifacts/epic_2/E2.G_cycle_receipt.md`), blank round-3 lines for the seven-language
  Aldric.
- **Rulings recorded (`decisions.md §24.3`):** the two-column grid (Name/Campaign one row; AC row without its
  heading + Speed Land/Swim/Fly beside the portrait; Abilities then Skills on the right); languages without a
  write-in line, seven names fit; every conditional in one fixed-size Conditional Bonuses box; a Resistances
  block; seven sections in the operator's order, each on its own pages under a prominent title.
- **Repo facts (census, `python3 -I docs/release/SD-40-pf-sheet-design/artifacts/epic_2/replan-r2/census.py`,
  output `census.txt`):** `speed.swim`/`speed.fly` rows 0; no resistances field on the load response (Ifrit/Undine
  numbers survive only in the corpus's `raw_bonus_chains`; race_trait energy chains 15); no CMB/CMD conditional
  list (CombatBonus aspects naming CMB/CMD/combat maneuver 22); AC notes printed nowhere; no section titles and
  no progression block; every PF character's languages are the constant `["Common"]`.
- **Correction:** G-4's `grep -c 'usable: yes'` counted the receipt's own instructions (3 with no sign-off on
  `d5cd0a3ea6`); anchored to `^- Aldric .* — usable: yes` (`decisions.md §24.5`).
- **Operator addition (same day):** "let's make adjustments to the ui to include these fields as well" → UI cards
  E2.24 (Swim/Fly beside Land), E2.25 (Conditional Bonuses under Saving Throws), E2.26 (Resistances in the Defense
  tab), placements chosen by the author (`decisions.md §24.3.8`, §24.7); census (f) added.
- **Operator answers to §24.6 (same day):** Q1 "10 lines, 4+ blank" (E2.20); Q2 "Yes, add a field" → new card E2.27
  (stored Languages field; Aldric's seven stored by the print proof, no print seam); Q3 "60 ft. (average)".
  ui-smoke rows 173 → 177 before E2.MC3.
- **Cards:** 40 → 54 (E2.27 added). New: E2.16, E2.18, E2.20, E2.22, E2.23, E2.24–E2.26 (main), E2.17, E2.19, E2.21 (side lane `engine`,
  reopened), E2.MC3, E3.7. Rewritten: E2.G (round 3), E3.5 (section 7), E3.MC, E4.1, E4.2, E4.MC, E5.2, E5.4
  (seven sections). `SAME_IDS` check passes. The three §24.6 questions are answered (`decisions.md §24.6`).

## Lead's dispatch log

(one line per dispatch: card, model, `git status --porcelain` / `git worktree list` readings (R19), the
R21 RED output path, the R22 grep)

## Open blockers

(none)
