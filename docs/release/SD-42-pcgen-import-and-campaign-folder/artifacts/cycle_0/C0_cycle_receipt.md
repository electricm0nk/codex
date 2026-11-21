# Cycle C0 — package / the package committed

- **Card ID:** C0   **Model:** lead session (Fable 5.1, planning)   **RETRO_ACTOR:** sd42-plan
- **Commit SHA:** the commit that adds this file (recorded in `progress.md` Cycle 0)    **Base SHA:** `1046daefc0` (`tranche-22` = `origin/tranche-21` tip on 2026-11-16)
- **Files touched:** `docs/release/SD-42-pcgen-import-and-campaign-folder/**` (16 files + `artifacts/cycle_0/operator-brief-2026-11-16.md`, `artifacts/epic_0/census.{py,json}`, this receipt), `docs/retro/events/sd42-plan.jsonl`
- **Acceptance criterion:** `epic-breakdown.md` C0.
- **Figures:** cards 37 — `awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'` → 37; same IDs — the `kanban.md` check → `SAME_IDS`; rulings traced — every row of `decisions.md §3` has a row in `acceptance-and-verification.md`'s traceability table (23 rulings incl. the two intake items); census — `python3 -I artifacts/epic_0/census.py` → `census.json` (operator file: 89 lines, 38 customised, 21 stacks, 1 zero-quantity, max 84; samples: 7 files, oracle = pin).
- **Placeholder gate:** `grep -rhoE '<[a-z_-]+>' *.md | sort -u` — every token is in `workflow-instruction.md §9`'s list (the output is in `progress.md` Cycle 0).
- **Stale-path audit:** `grep -rn 'programs/codex/requirements\|~/workspace/repos/pcgen\|/home/ubuntu/workspace/repos/pcgen' .` → 0.
- **PII check:** the player's name appears nowhere in the package (`grep -rn "$SD42_PII" .` → 0 with the name set locally); the character's name appears only in this package's docs and the local path.
- **Rulings applied:** every Q in `decisions.md §3`; the defaults in `§21`.
- **Does not cover:** the hypotheses in `technical-design.md §1` (E1.1 proves them); the exact PCGen slot-table length (E1.2 reads the oracle file); whether the PCGen engine runs on this box (P-9, E1.7).
- **Status:** complete
- **Next-cycle plan:** C1 (0.22.0 on the 14 surfaces), then the three Phase-1 lanes.
