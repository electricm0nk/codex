---
canonical: true
owner: operator
bundle_id: SD-40
slug: pf-sheet-design
canonical_branch: tranche-20
target_version: 0.20.0
date: 2026-11-07
status: planning-ready
---

# SD-40 Risks and Open Questions

## 1. Ranked risks

| # | Risk | Likelihood | Impact | Control | Self-healable? |
|---|---|---|---|---|---|
| R-1 | **Fifth print attempt still looks like a list** because an agent cannot judge a picture | medium | high | R7: every look card rasterizes and reads its PNGs against the PCGen reference (E0.1) before claiming; E2.MC/E3.MC re-look; two operator gates (E2.G, E5.2) | no — the gates wait |
| R-2 | A look re-derives a number in Typst (a sum, a modifier) | medium | high | NR-2 greps on `print/` and `resources/print/`; blocks carry strings only (`model.rs`); E2.MC/E3.MC read every `.typ` | yes (move it to the engine row or drop it) |
| R-3 | SF1e / CPR output drifts through the shared frame or dispatcher | medium | medium | `columns == 1` keeps the old loop; G-2 diffs their golden text and page counts against C1's every card; SD-g: never re-baseline | yes (fix the frame) |
| R-4 | PCGen headless render fails (classpath, settings dir, hand-authored `.pcg` refused) | medium | medium | §6 names the launcher the harness already uses; SD-h falls back to a testsuite `.pcg`; `blocked-escalated` only if PCGen will not run at all | partly |
| R-5 | Page 1 does not fit Letter at a readable size | medium | medium | Letter-first design (§10); SD-b shrink-then-move order; 8 pt floor; the operator's eye at E2.G (no page-count ceiling, `decisions.md §22`) | yes |
| R-6 | Golden page counts churn every card, hiding a real regression | high | low | each receipt lists before → after per case with the reason (SD-f); E4.MC reads the whole series; the operator judges density by eye at E2.G and E5.2 (no ceiling, `decisions.md §22`) | n/a (by design) |
| R-7 | Serial main lane + two side lanes: a `ref` or `smoke` lane commit conflicts with a main-lane doc edit | low | low | `ref` writes only under `artifacts/epic_0/E0.1/`; `smoke` only `spec.json`, `docs/testing/ui-smoke-inventory.md` and `artifacts/epic_0/E0.2/`; both plus their `kanban.md` and `progress.md` rows via §5's rebase protocol; SD-k | yes |
| R-8 | `tranche-19` merges to `develop` mid-run; `tranche-20` must catch up | certain | medium | SD-o: the lead rebases between epics, re-runs G-2, logs it; never mid-card | no — lead action |
| R-9 | PNG artefacts bloat the repo | medium | low | SD-p: 60 dpi, ≤ 12 pages and ≤ 2 MB per card directory | yes |
| R-10 | The sections change breaks the app between E4.1 and E4.2 | low | high | E4.1 keeps the booleans and maps them; E4.2 removes them with both DTOs in one card | yes |
| R-11 | A look card's acceptance command passes on the base (cannot fail) | high (SD-39 retro §"did not work") | medium | R21: the lead runs each card's command on the pre-change tree before dispatch and requires a failure; the card quotes RED | yes |
| R-12 | FILES YOU OWN misses a second call site (SD-39: five cards) | medium | low | R22: the lead's pre-dispatch `git grep` over each producer named in the row; fence exits (§19) | yes |
| R-13 | The operator's gate finds defects → the run must stop and restart | expected | low | gate defects are discovered cards; `START=<card>` restart; two waits by design | n/a |
| R-15 | Page 1 no longer fits Letter: one-column skills (35 lines), the AC parts, notes lists and languages all added (`decisions.md §23.3`) | high | medium | Letter-first; 8 pt floor; the new SD-b continues a long list on page 2 (E2.4) without moving a block; E2.MC2's population probe over the 37 `.pcg` imports; the operator's eye at round 2 | yes |
| R-16 | An engine-lane value re-derives a rules number outside the summing function (a twin) or moves an existing value | medium | high | each part / note / total is emitted where the engine already sums or renders it (E2.6, E2.7, E2.14 criteria); the AC invariant `10 + Σ parts = AC` over seeds, probes and the `.pcg` population; G-1 rewritten: everything except the named new keys and rows must stay byte-equal | yes |
| R-17 | The rewritten G-1 hides a moved value under a new key's name | low | high | `NEW_KEYS` / `NEW_ROWS` are listed in the gate itself and grow only with a receipt line naming the key and its card | yes |
| R-18 | The engine side lane and the main lane write one file | low | medium | `engine` writes `src/rules_core/**`, the DTO files and the TS types/models it names, never `print/**` or `resources/print/**`; main-lane readers wait on the engine card in `kanban.md`; §5's rebase protocol for `kanban.md` / `progress.md` | yes |
| R-19 | An operator answer to §23.6 (all five answered 2026-11-11, §23.7) changes again after the cards ran | low | low | each DEFAULT is one card's criterion (E2.8, E3.4, E3.1–E3.3, E2.14); a different answer is a discovered card in that epic (§14); E2.14 is dropped whole on Q5 "no" | yes |
| R-14 | E0.2 and E4.2 both write `spec.json`; or a stale row is "repaired" by deleting or skipping it | medium | medium | serial edge E0.2 → E4.2 in `kanban.md` (R24); E0.2's acceptance runs the 14 ids by name, requires all 14 green and all 14 still in `spec.json` with the row count unchanged (G-9) | yes |

## 2. Open questions → defaults taken

The operator answered the first three on 2026-11-07 (`decisions.md §22`). The E2.G round-1 replan
(2026-11-11) raised five (`decisions.md §23.6`); the operator answered them the same day (`§23.7`):

| # | Question | Operator's answer (§23.7) | Card |
|---|---|---|---|
| Q1 | What "known languages" prints (the engine holds none) | "Known + one blank line": the known list from one backend source, then exactly one blank line | E2.8, E2.13 |
| Q2 | Bio text (the app has no backstory field) | "Add a Backstory field now": card E3.6; the Bio page prints details, Backstory, campaign notes, ruled lines | E3.6, E3.4 |
| Q3 | Where daily spells, custom, pets, the armor table print | as proposed: spells / custom / pets in box 2; armor table in box 3 | E3.1–E3.3, E4.1 |
| Q4 | The footer prints the name on every page | keep | — |
| Q5 | Weapon damage as one total (`1d8+4`): the engine refuses one today | "One total": E2.14 adds an engine total row (primary-hand assumption named) | E2.14, E2.10 |

| Question | Default | Where |
|---|---|---|
| Package path `docs/release` vs `docs/stc` | `docs/release` | `decisions.md §1` |
| What "Spellbook" preset prints | **answered "2: A" (2026-11-07):** the spell pages alone; Full prints everything | `decisions.md §9`, §22 |
| Density limits | **answered "3: No hard limits" (2026-11-07):** no page-count ceiling; page counts recorded per receipt; the operator's eye at E2.G and E5.2 | `decisions.md §10`, §22 |
| Fonts and colour | Liberation Sans only; black and white | `decisions.md §11` |
| Grid width | 12 columns | `decisions.md §4` |
| Combat block split | along the UI's box ids | `decisions.md §8` |
| Presets vs checkboxes | **answered at E2.G round 1 (2026-11-11):** six section checkboxes, remembered per character, first time 1 + 2 + 3 | `decisions.md §23.4` |
| E2.4 (parked) | rewritten as page 1's overflow rule on the new page 1 | `decisions.md §23.5` |
| The 14 stale ui-smoke rows (retro change 5) | **answered "1: B" (2026-11-07):** card E0.2 owns them | `forward-scope-register.md` FSR-C1, `decisions.md §22` |

## 3. Non-self-healable conditions (card → `blocked-escalated`)

Two live lanes on one file; RED→GREEN not preserved; a stub, mock or "Would …" string in shipping code;
an SF1e/CPR golden text delta a card cannot remove; PCGen refusing to run headlessly at all (SD-h's
second clause); the operator's gate verdict "not usable" (becomes discovered cards, then the gate re-runs);
`CLAIM-EXISTS`.
