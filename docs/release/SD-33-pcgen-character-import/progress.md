# SD-33 — Per-cycle Receipts

This file carries the per-cycle receipt for SD-33. Each cycle appends a section with its
cycle-id and operator-readable facts. The supervisor reads this file to verify completion
before the next cycle claim (`loop-instruction.md` Step 6).

Receipts are evidence, not narrative. Keep them short.

## Cycle 0.0 — Package Land (planning-ready)

**Date:** 2026-08-11
**Cycle ID:** `SD33-LAND-1`
**Operator:** Todd Hintzmann (directive 2026-08-11)
**Surface:** this directory

### What landed

The canonical STC chassis for SD-33, authored against facts verified by command at `tranche/9`
HEAD `3570d735` on 2026-08-11.

### Starting-state facts, each verified by command

| Fact | Verification |
|---|---|
| `.pcg` is line-oriented `KEY:value` with pipe subtokens, bracketed groups, repeated keys, and a dotted-ID `EQUIPSET` tree | Read directly from the vendored Fighter fixture |
| Two real `.pcg` fixtures are vendored with pinned sha256 | `tests/ge05_vendored_pcg_fixtures.rs` |
| The existing `import_character` path handles Codex JSON, **not** `.pcg` | `apps/desktop/src/boundary/loadImportCharacter.ts` doc comment; `character_hub.rs:3759` |
| A safe import landing pad exists (fresh id, engine recompute, `Blocked` rather than persist) | `character_hub.rs:3744` onward |
| A real PCGen oracle exists | `src/oracle_validation/pcgen_runner.rs` |
| `src/pcgen_import/` handles `.lst`/`.pcc` data only | module listing: `lst_parser`, `pcc.rs`, `ir_converter.rs`, … |
| Neither SD-29's nor SD-30's partition contains SD-33's write surface | `TR-29-001`, `TR-30-001` |

### Corrections made during authoring

Two candidate framings were checked and discarded before they reached this package:

- **"Close the UI deferral backlog."** SD-29's forward-scope register lists 31 unowned engine/UI
  deferrals, but spot-checking found the UI ones largely closed already — the Add Item picker
  labels its book (`itemPickerFilter.ts:150`), `SpellCatalogScreen` maps wire codes to display
  names, and the orphaned `removeSelection.ts` shim is wired into `CharacterSheet.tsx`. The
  register documents its own decay in §7.3. Founding a bundle on it would have burned cycles
  re-scoping.
- **"The app's inline styling is a defect."** 1030 inline `style={{}}` against 1 `className`
  looks like drift, but `theme.css` is a deliberate two-layer token system consumed via
  `var(--color-*)`, with a community-theme bridge. Only 8 hardcoded hex colors across 258 files.
  Calling that a defect would have been a name-shaped false claim.

Both are recorded here because a successor re-reading the deferral ledger will reach the same
two dead ends otherwise.

### Not verified

- **The PCGen runner has not been executed.** Its scripts and the PCGen checkout are present,
  but no headless run was performed while authoring. `R-4` carries this; Epic 1 should smoke-test
  it before Epics 2–6 build on the assumption.
- Fixture coverage is two level-1 Human CRB characters (`R-2`).

### Open for the operator

The three questions at the foot of `risks-and-open-questions.md` — additional fixtures, whether
acknowledged lossy import should exist in v1, and whether to allowlist the two inert templates.
None blocks Epic 1.
