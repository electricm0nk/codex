# UI Smoke Repair Cycle — SD-36 Consolidation

**Date:** 2026-09-17

**HEAD before:** 71b8be5c1f  
**HEAD after:** *(to be filled)*

## Repair Summary

| Dimension | Count |
|-----------|-------|
| Total rows | 67 |
| Green | 18 |
| Red | 29 |
| Blocked | 4 |
| Manual | 16 |

## Fixed Mechanisms

- settings-tab-appearance: DOM probe resolved the appearance tab to green

## Permanently Blocked Mechanisms

*(none)*

## Manual Rows with Reasons

- create-character-fill-and-submit: Ability-score-method selection and the 'Create character' submit were not exercised live under this cycle's read-only scope (CreateCharacterForm.tsx is another agent's territory this cycle)
- load-character-select-and-open: Selecting a specific row requires a row-name target this cycle's read-only pass over LoadCharacterScreen.tsx did not confirm
- load-character-delete-clone: Delete goes through window.confirm() (a native modal xdotool cannot address via the DOM probe) and Clone mutates the on-disk save set
- sheet-open-for-tabs: Reaching the sheet depends on load-character-select-and-open, which is itself manual this cycle
- sheet-action-add-spell: Only reachable when the seeded/loaded character is a caster the 'Add Spell' affordance is offered for
- sheet-menu-clone: Clone mutates the on-disk save set (produces a new saved character each run)
- sheet-menu-export: Actually selecting 'Export' opens a native OS save dialog (see the top-level manual-character-export row)
- dm-toolkit-open-and-kind-tabs: Requires at least one campaign to exist (DmToolkitScreen shows an empty-state message otherwise)
- dm-toolkit-edit-record: Editing an existing record requires selecting a specific list row first, whose accessible name this cycle's read-only pass did not enumerate against seeded data
- dm-toolkit-delete-record: Same dependency as dm-toolkit-edit-record, plus deleteDmRecord's own confirmation UI was not traced this cycle
- dm-toolkit-export: exportConsole() drives runDmConsoleExport, whose completion surface was not confirmed live this cycle
- encounter-builder-add-monster-and-rating: A numeric rating only appears once both a party member AND a monster are added
- campaign-manager-list: The Campaign Manager banner is gated disabled until a local Drive folder is configured
- manual-character-import: handleImport opens a native OS file-open dialog (Tauri's dialog plugin)
- manual-character-export: handleExport / the sheet menu's 'Export' item open a native OS file-save dialog
- manual-portrait-upload: PortraitUpload opens a native OS file-open dialog for the image file

## Follow-ups

Per-record corpus files are read at runtime alongside _settled bundles (race_resolver.rs load_chassis_dir, corpus_loader.rs); a settled-only loader would cut the bundle from ~490 MB to ~11 MB — SD-36 Epic C.

apps/desktop/.claude/skills/run-desktop/SKILL.md example coordinates are stale (1280x900 vs 1920x1200); the ui-smoke probe supersedes coordinate driving.
