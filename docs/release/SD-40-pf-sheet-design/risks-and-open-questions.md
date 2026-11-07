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
| R-5 | Page 1 does not fit Letter at a readable size | medium | medium | Letter-first design (§10); SD-b shrink-then-move order; 8 pt floor; `pages_max` | yes |
| R-6 | Golden page counts churn every card, hiding a real regression | high | low | each receipt lists before → after per case with the reason; `pages_max` ceilings never move up (SD-b); E4.MC reads the whole series | n/a (by design) |
| R-7 | Serial main lane + one side lane: a `ref` lane commit to `artifacts/` conflicts with a main-lane doc edit | low | low | `ref` writes only under `artifacts/epic_0/`, `kanban.md` and `progress.md` rows via §5's rebase protocol; SD-k | yes |
| R-8 | `tranche-19` merges to `develop` mid-run; `tranche-20` must catch up | certain | medium | SD-o: the lead rebases between epics, re-runs G-2, logs it; never mid-card | no — lead action |
| R-9 | PNG artefacts bloat the repo | medium | low | SD-p: 60 dpi, ≤ 12 pages and ≤ 2 MB per card directory | yes |
| R-10 | The preset change breaks the app between E4.1 and E4.2 | low | high | E4.1 keeps the booleans and maps them; E4.2 removes them with both DTOs in one card | yes |
| R-11 | A look card's acceptance command passes on the base (cannot fail) | high (SD-39 retro §"did not work") | medium | R21: the lead runs each card's command on the pre-change tree before dispatch and requires a failure; the card quotes RED | yes |
| R-12 | FILES YOU OWN misses a second call site (SD-39: five cards) | medium | low | R22: the lead's pre-dispatch `git grep` over each producer named in the row; fence exits (§19) | yes |
| R-13 | The operator's gate finds defects → the run must stop and restart | expected | low | gate defects are discovered cards; `START=<card>` restart; two waits by design | n/a |

## 2. Open questions → defaults taken

| Question | Default | Where |
|---|---|---|
| Package path `docs/release` vs `docs/stc` | `docs/release` | `decisions.md §1` |
| What "Spellbook" preset prints | the spellbook pages alone | `decisions.md §9` |
| Density numbers before E0.1 measures | Aldric Basic ≤ 2, Elowen Basic ≤ 3, … | `decisions.md §10` |
| Fonts and colour | Liberation Sans only; black and white | `decisions.md §11` |
| Grid width | 12 columns | `decisions.md §4` |
| Combat block split | along the UI's box ids | `decisions.md §8` |
| The 14 stale ui-smoke rows (retro change 5) | **operator question**, `README.md §6` | `forward-scope-register.md` FSR-C1 |

## 3. Non-self-healable conditions (card → `blocked-escalated`)

Two live lanes on one file; RED→GREEN not preserved; a stub, mock or "Would …" string in shipping code;
an SF1e/CPR golden text delta a card cannot remove; PCGen refusing to run headlessly at all (SD-h's
second clause); the operator's gate verdict "not usable" (becomes discovered cards, then the gate re-runs);
`CLAIM-EXISTS`.
