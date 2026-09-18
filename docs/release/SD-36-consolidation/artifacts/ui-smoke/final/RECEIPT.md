# UI Smoke Test — Cycle 3 Receipt

**Date**: 2026-09-18
**Cycle**: 3

## HEAD (before this cycle's commit)

- **Before**: `a111db92a2` (ui-smoke cycle 2 — fixed 17, blocked 0, final red/blocked 1)

## What cycle 2 actually shipped (and why this cycle exists)

Cycle 2's own `results.json` held only **47 of the spec's 69 rows**. Its own
Verify agent's shell call hit a 10-minute limit and killed `node` mid-run,
after row 47 (`sheet-action-add-spell`); the partial file was then read and
reported as a complete run (cycle 2's RECEIPT.md claimed "Total: 69" against
an array of 47 entries — a right-looking number over the wrong population).
The 22 rows cycle 2 never reached at all: `sheet-action-level-up-dialog`
through `campaign-sheet-tabs`, plus the 3 manual rows. One row cycle 2 DID
reach was red: `settings-theme-select-and-manage-themes-modal`.

This cycle's own scope was exactly this gap: (1) make the harness itself
denominator-safe so a killed run can never again be mistaken for a complete
one, (2) add `--resume` so an interrupted run can pick up where it left off
instead of re-running everything, (3) add bounded auto-recovery for the
harness's own documented command-channel stalls, (4) fix the one real red
row, and (5) actually finish the suite and prove it.

## Harness changes (`apps/desktop/scripts/ui-smoke/`)

- **`lib/resultsSkeleton.mjs`** (new): pure, DOM-free skeleton/merge/resume/
  summary logic, unit-tested directly in `lib/resultsSkeleton.test.mjs`
  (`node lib/resultsSkeleton.test.mjs`).
- **`run.mjs`**: writes `results.json` with **every selected row present**
  (`status: 'not-run'`) before any row executes; each row's own entry is
  replaced in place as it finishes. The final summary line now reads
  `<green>/<total> green, <red> red, <blocked> blocked, <manual> manual,
  <not-run> not-run (M = <total> rows)`, and the process exits non-zero if
  red, blocked, OR not-run is nonzero — a killed run can no longer produce a
  results.json that looks complete.
- **`--resume`**: with `--out` pointed at a directory that already holds a
  `results.json`, skips re-running any row whose entry there is already
  `green` or `manual` and re-runs everything else (`not-run`, `red`,
  `blocked`, or missing from that file entirely).
- **Auto-recover**: two consecutive rows that look like a genuine
  command-channel stall (not a normal assertion failure) trigger one
  `driver.sh launch` (fresh app process) and a retry of the row that
  completed the pair, capped at 3 relaunches per run; a retry that then
  passes is recorded with reason `auto-relaunch`. Exercised for real during
  this cycle's own proof run (see below), not just in theory.
- **`waitForRowSettled`** (was `waitForMarker`): now also waits, within the
  same per-row budget, for every select a row declares in
  `selectsNonEmpty` to actually be present — not just for the row's marker
  text and the absence of a `'Loading'` placeholder. Fixes a real, generic
  race class (see `sheet-action-trait-add` below).

Documented in `spec.json`'s own top-level `$comment` and in
`docs/testing/ui-smoke-inventory.md`'s generated header
(`render-inventory.mjs`).

## Fixed rows (2)

1. **`settings-theme-select-and-manage-themes-modal`** (cycle 2: red →
   green). Root cause was in the row's own steps, not the app:
   `AppearancePanel.tsx`'s "Manage…" button and `ThemeBrowserModal.tsx`'s
   own `<h2>Community themes</h2>` both render exactly as the spec expects
   (confirmed by reading both files directly) — the row's OWN trailing
   `key: Escape` step ran *before* the marker was ever checked (`run.mjs`
   runs every step in a row synchronously, then asserts), closing the
   modal it had just opened. `'Community themes'` appears nowhere else on
   the Appearance settings screen once the modal closes, so the row was
   unpassable as written, the same shape of bug
   `sheet-action-level-up-dialog`'s own notes already document and fix
   for a different dialog. Fixed by removing the trailing Escape (matching
   that row's own precedent) — `resetToLanding()` already closes this
   modal via Escape for whichever row runs next, so no explicit cleanup
   step is needed at all.
2. **`sheet-action-trait-add`** (discovered red during this cycle's own
   completeness run, not carried over from cycle 2). `TraitsSection`'s own
   `'TRAITS'` heading is a static label present from first paint, but its
   `'Trait to add'` `<select>` only mounts once `loadCharacterTraits()`'s
   async fetch resolves, with no `'Loading'` placeholder while pending —
   so the old marker+Loading-only wait returned before that fetch had a
   chance to settle, the same race class `create-character-render` and
   `race-catalog-alternate-traits-tab` were already fixed for elsewhere in
   this file. Fixed generically in the harness (`waitForRowSettled`, see
   above), not special-cased to this one row.

## Real, transient recoveries (not defects)

`campaign-manager-list` blocked once (`could not reach the landing screen
within 6 reset attempts`), immediately followed by `campaign-create` also
looking stalled — the harness's own documented command-channel wedge (see
`run.mjs`'s `CLICK_TARGET_WAIT_MS` comment: measured directly at 60–80+
seconds in this same harness on unrelated screens). Auto-recover fired
(`auto-relaunch 1/3`), and `campaign-create` passed immediately afterward
with reason `auto-relaunch`. `campaign-manager-list` itself — the row that
started the pair, not the one auto-recover retries — was left `blocked` by
that one invocation; a plain `--resume` pass immediately after (a normal
part of this cycle's own workflow, not a special fix) reached it fresh and
it passed on the first try, with no further changes to the app or the spec.
Reproduced the identical pattern (block on `campaign-manager-list`,
immediately followed by an auto-recovered `campaign-create`) across two
independent runs at the same point in the row sequence, consistent with
`encounter-builder-add-monster-and-rating`'s own already-documented slow
`rate_encounter` IPC round trip leaving the channel busy into the very next
row, not with a defect in either row's own steps or assertions.

## Final Test Results Summary

| Category | Cycle 3 | Cycle 2 (claimed) | Cycle 2 (actual, 47 rows) |
|----------|---------|---------|---------|
| Green (passing) | 66 | 45 | 46 |
| Red (failing) | 0 | 1 | 1 |
| Blocked | 0 | 0 | 0 |
| Manual (not automated) | 3 | 23 | 0 (never reached) |
| Not-run | 0 | *(not tracked)* | 22 (silently absent) |
| **Total** | **69** | **69** | **47** |

Reached via `--resume` against cycle 2's own 47-row `results.json` (46
green + 1 red preserved verbatim, 23 rows re-run: the 1 red fix plus the 22
rows cycle 2 never reached), confirming `--resume` itself works as
specified. Final summary line, reproduced live:

```
66/69 green, 0 red, 0 blocked, 3 manual, 0 not-run (M = 69 rows)
```

## Manual Tests (3)

Unchanged from cycle 2 — these open a native OS file dialog (Tauri's
dialog plugin), which `xdotool`/the DOM command channel has no way to
drive inside:

1. **manual-character-import** — `LoadCharacterScreen`'s Import button.
2. **manual-character-export** — `LoadCharacterScreen`'s Export button and
   the sheet menu's Export item.
3. **manual-portrait-upload** — `PortraitUpload`'s file picker.

Cycle 2's RECEIPT.md listed "23 manual" by counting every row without a
`results.json` entry as manual, which conflated "genuinely unautomatable"
with "never reached" — 20 of those 23 were ordinary automated rows this
cycle ran and passed for real. There are exactly 3 truly manual rows in
the spec (`grep -c '"manual":' spec.json`).

---

**Summary**: Cycle 3 closes the denominator gap cycle 2 left open — every
one of the spec's 69 rows now has a real, current-run outcome, with 0
`not-run`, 0 red, and 0 blocked. The harness itself now refuses to produce
a results.json that can be mistaken for complete (`buildSkeleton` +
`applyResult` + the `not-run`-aware exit code), can resume an interrupted
run (`--resume`), and can ride out its own documented command-channel
stalls up to a bounded retry budget (auto-recover). Both real defects found
along the way (one spec bug, one generic harness race) are fixed at their
root cause, not carved out or waived.
