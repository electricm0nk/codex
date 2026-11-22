<!-- SD-43 INTERIM NOTES: placeholder for the tranche-43 cut. Replaced by the final notes at E7.3. -->

# Campaign Codex 0.43 — Market, Inventory and Abilities (interim)

These are interim notes. The tranche is not released. The final notes replace this file at E7.3, and the
publish lane refuses a release until they do.

## Summary

Tranche 43 is in progress. Its work is planned in `docs/release/SD-43-market-inventory-and-abilities/`:
the Market and Inventory fixes, the container and gear rules, the sheet layout, and the Abilities tab.
This interim file only points the release tooling at this tranche. No user-facing change ships with it.

## User-Visible Changes

- None yet. The tranche's changes are listed as cards in `docs/release/SD-43-market-inventory-and-abilities/kanban.md`.

## Defects Fixed

- None yet. Defects are reproduced and fixed by the cards in that kanban file.

## Operational Notes

- `docs/release/current-release.json` names tranche 43 and this file. `apps/desktop/package.json` is `0.43.0`.
- `python3 tools/release/resolve_release_notes.py --repo-root .` must exit 0 on this tree.

## Verification Evidence

- `python3 tools/release/resolve_release_notes.py --repo-root .` exits 0 and prints this file's path (run on this tree for card C1).
- `python3 -m pytest tools/release -q` passes (run on this tree for card C1).

## Known Issues

- These notes are interim. Nothing in them is a release claim until E7.3 replaces this file.

## Update Eligibility

- Not eligible. This is an interim pointer for tranche 43, not a published build.
