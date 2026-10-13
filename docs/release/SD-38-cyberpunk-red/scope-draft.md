---
canonical: true
owner: operator
bundle_id: SD-38
slug: cyberpunk-red
canonical_branch: tranche-18
target_version: 0.18.0
date: 2026-10-13
status: draft — authored under operator-away defaults; operator rulings on licence and scope are on record
---

# SD-38 Scope — Cyberpunk RED

## Operator intent (on record, 2026-10-13, verbatim from the planner's brief)

- **Licence:** *"license - do the whole thing for now. We can back stuff out or reword it later if
  need be. For now, i'm the only person even using the app."*
- **Scope:** *"Scope: full book."*
- **Mode:** travelling for a few weeks; autonomous; report after each epic; implementation lanes as
  Workflow scripts.
- **Automation:** *"none of the automation that we built will apply"* — there is no PCGen data for
  this system; the source is the PDF text only.

Every other scoping choice below is a DEFAULT from the brief or an AUTHOR DEFAULT
(`decisions.md §20`, `§21`), each with a revisit line.

## Bundle intent

Campaign Codex generates **Cyberpunk RED character sheets** from the corebook v1.22, as its third game
system after Pathfinder 1e and Starfinder 1e. A player creates a character by any of the book's three
methods, sees a sheet whose every number comes from the engine, and improves the character after play.
Every table a player or GM uses is browsable; GM tables are print-only. Pathfinder and Starfinder stay
**byte-identical**.

## Source

- `/home/ubuntu/workspace/inbox/cyberpunk-red/cpr-corebook-v122.pdf` (458 pp), its `pdftotext -layout`
  text, and `pages/pNNN.txt` (459 files, CUI F-19; **pNNN = printed page + 1**). Never inside the git tree.
- Seven inventories (copied to `artifacts/cycle_0/inventory/`) → `content-unit-inventory.md`.
- **Not content units:** the setting/fiction chapters *Never Fade Away, View from the Edge, Tales from
  the Street, The Fall of the Towers, Welcome to the Dark Future, Time of the Red, Welcome to Night City,
  Everyday Life, Screamsheets, Black Dog* (`decisions.md §5`).

## Definition of Done (sheet-level, not bookkeeping; verbatim from the brief)

1. A player can create a Cyberpunk RED character by any of the three methods, with Lifepath, skills (86
   pts, ×2, cap, basics), role ability at rank 4 with its sub-choices, starting kit, cyberware pack, cash —
   and every number on the resulting sheet is an engine explanations row. *(G-2; E3)*
2. The sheet shows: STATs, HP/SW/Death Save/Humanity/EMP, MOVE, skills by category with bases, weapons
   with damage/ROF/hands/conceal and range DV mini-table, armor SP per location with penalty, cyberware
   with HL and slots, role ability with rank-derived values, Netrunner deck/programs/NET actions,
   reputation, IP, wound states, criticals checklist, Lifepath text. *(G-3; E2, E4)*
3. Improvement after creation: IP spend on skills/role rank, role switch, cyberware install with Humanity
   loss and therapy, reputation change — all through the saved-character store. *(G-4; E5)*
4. Catalogs/reference: every kind in the inventory browsable; GM tables print-only. *(G-5; E6)*
5. Content: every inventory count is pinned by a test against `data/cyberpunk-red/corpus/MANIFEST.json`;
   every record has `source_page`; 10% sample verified against page images with a receipt. *(G-6; E1)*
6. PF1e and Starfinder byte-identical (seed renders unchanged). *(G-1; E0.6 and every merge check)*
7. Packaged app contains the data (`packaged_resources.rs`), 3 seeds restore, ui-smoke rows green, both
   cargo workspaces + npm green at closure, retro + release notes + architecture docs + graphify + PR
   (operator merges). *(G-7–G-11; E0.3, E7)*

**Not in the Definition of Done** (planned capability deferrals with revisit conditions,
`decisions.md §22`): in-play dice engine (the sheet prints dice) (DEF-2), NET-run simulation (DEF-3),
vehicle-combat simulation (DEF-4), NPC/Mook generator — GM stat blocks are print-only catalog pages
(DEF-5), Netrunning architecture **generator** — tables print-only (DEF-1).

## Epic order

```
C0 → C1 → E0 → E1.0 → E1.1…E1.8 (parallel) → E1.V
     → E2 lanes A ∥ B ∥ C → (E6 in lane C's slot) → E2.10 → E2.MC
     → E3 (E3.1 after lane A + E6.1; E3.2 after E6.MC) ∥ E4 (after E2.MC + E3.2)
     → E5 (after E3.MC + E4.MC) → E7 (serial closure)
```

68 cards, one criterion each (`epic-breakdown.md`, `kanban.md`).

## Cycle dispatch model

One long `Workflow` run (`workflow-instruction.md §2.4`). Opus for engine, UI, verify and merge checks;
Sonnet for transcription lanes, well-specified work and long waits; Haiku for housekeeping. At most 3
cargo lanes. The script reads `kanban.md` and dispatches a card only when its dependencies are
`complete`.

## Verification (end to end)

`acceptance-and-verification.md` G-1…G-11: PF/SF byte-identical; three creation methods through the
real form with every sheet number an engine row; book-internal parity (HP table, 86-point templates,
cyberware pack sums, p121); MANIFEST counts by two implementations; a 10% page-image sample; packaged
resources; three seeds; both cargo workspaces, npm, `verify.sh`; fresh install; the closure PR's
`pr-tests`.
