# Import lane — resume receipt (E1.MC, 2026-11-17)

- **Lane state:** ended. E1.1–E1.10 and E1.MC complete (`awk -F'|' '$2 ~ /^ E1\./ && $5 ~ /complete/' kanban.md | awk 'END{print NR}'` → 11 of 11).
- **Tree:** `.worktrees/sd42-import`, local branch `sd42/import` and `CARGO_TARGET_DIR=/home/ubuntu/cargo-target/sd42-import` removed by E1.MC after its push.
- **What Phase 2 inherits from the converter (the wire, `crates/codex-ingest/src/pcg_import/wire.rs`):** `equipmentSets` (one stack per place, pieces in `CopyFacts.quantity`, overrides `damageOverride` / `weightOverrideLbs` / `costOverrideCopper` / `properties`, `eqmods` on the selection row), Create items in `custom.equipment` (stats carry any `CUSTOMIZATION` field as text), `bio.notes` (10 kinds), `gameSystem`.
- **Owned next:** E4.1 (write the wire into `equipment_sets.json` / `custom.json` / `bio.json`), E4.2 (the engine reads the three overrides), E4.13 (Pathfinder totals parity at the desktop), E4.14 (the 12 engine/data ledger rows).
- **Restart:** nothing to resume in this lane; a converter defect found in Phase 2 is a discovered card in lane `main` (`decisions.md §19`).
