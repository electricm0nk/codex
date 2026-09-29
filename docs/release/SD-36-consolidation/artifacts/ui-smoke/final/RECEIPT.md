# ui-smoke final proof — RECEIPT

Date: 2026-09-18
Run: cycle 3 — clean single pass, then one --resume pass for a stalled row.

HEAD before run: 3d4cebd470e29a8344c0c8097e40239e08310372
HEAD after run (commit below): see commit SHA in the commit message.

Denominator: 69 rows.

## Totals

- Green: 66
- Red: 0
- Blocked: 0
- Manual: 3
- Not-run: 0

## History

- cycle 1: 18/29/4/16 of 67 (early partial run)
- cycle 2: 46 green of only 47 executed (shell-timeout truncation, not a real result)
- cycle 3, first full pass: 65/69 green, 0 red, 1 blocked, 3 manual, 0 not-run
- cycle 3, resume pass: retried the 1 blocked row (`campaign-manager-list`); it passed
- cycle 3, final: 66/69 green, 0 red, 0 blocked, 3 manual, 0 not-run

Commits 5b08fb88a6 and 3d4cebd470 recorded an erroneous receipt (2 green / 67 not-run,
with 65 PNGs deleted) after a Haiku-tier agent could not keep the long run alive
("probe communication timeout"). This receipt supersedes both.

## Manual rows (native OS dialogs — cannot be driven by the DOM command channel)

- `manual-character-import` — `handleImport` opens a native OS file-open dialog
  (Tauri's dialog plugin); xdotool has no DOM to drive inside it.
- `manual-character-export` — `handleExport` / the sheet menu's "Export" item open
  a native OS file-save dialog.
- `manual-portrait-upload` — `PortraitUpload` opens a native OS file-open dialog
  for the image file.

## Red/blocked rows

- `campaign-manager-list` — first pass: BLOCKED, "could not reach the landing
  screen within 6 reset attempts." Second pass (--resume): PASS. No code or spec
  change was needed; the row was a transient stall, not a defect.

## Auto-relaunch (verbatim from the run log)

```
run.mjs: 2 consecutive rows stalled on the command channel -- relaunching the app (auto-relaunch 1/3) and retrying 'campaign-create'.
PASS  campaign-create  -- auto-relaunch
```

## Follow-ups

1. A settled-only loader would cut the bundle from ~490 MB to ~11 MB (SD-36 Epic C).
2. The `run-desktop` SKILL.md coordinate-driving examples are stale — the DOM
   probe/command channel supersedes coordinate driving.
3. Haiku-tier agents twice failed to keep a 20-40 minute ui-smoke run alive.
   Drive long suites from a Sonnet agent, or a nohup + poll loop, only.
