# Cycle sd32-retro-deferral-resolution — retro.py deferral-tracking fix

- **Card ID:** `sd32-retro-deferral-resolution`
- **Territory:** `scripts/retro.py`, `docs/retro/schema.json`, `scripts/tests/test_retro.py` (new tests only)
- **Files touched:** `scripts/retro.py`, `docs/retro/schema.json`, `scripts/tests/test_retro.py`
- **Status:** complete

## The defect, reproduced

`scripts/retro.py summary`'s `deferrals.open` was `deferrals[-limit:]` — the last
`--limit` deferral events in emission order, defaulting to 10 — and had never measured
resolution at all. Reproduced against the live SD-32-window log before this fix (checked
out the pre-fix `scripts/retro.py` from `HEAD` at
`1bb523773d32705d1b7387fd4c494861523f55ba` and ran it in place, then restored the edited
file — no working-tree files were left modified by the check):

```
python3 scripts/retro.py summary --since 2026-08-22 --json --limit 3   -> deferrals.open has 3 items
python3 scripts/retro.py summary --since 2026-08-22 --json --limit 10  -> deferrals.open has 10 items
python3 scripts/retro.py summary --since 2026-08-22 --json --limit 29  -> deferrals.open has 29 items
```

`deferrals.total` in that same window is **29** (`by_type.deferral` in the same command).
A closure lane reading the default `--limit 10` output as the bundle's whole open-deferral
list would have checked 10 of 29 and left 19 unexamined — matching
`docs/retro/sd32-compute-library-and-cause-closure-retrospective.md`'s deferrals
paragraph (lines 287–298), which independently re-verified the 10 it saw and reported
"Zero of the ten remained open" without ever seeing the other 19.

## Fix

1. **New `resolution` event type** (`docs/retro/schema.json`), required fields
   `resolves` (the id of the event being closed out) and `how` (what closed it, held
   to the same evidentiary bar as `correction.verified_by`). A data-only addition: the
   CLI builds its own argparse from the schema, so `scripts/retro.py resolution
   --resolves <id> --how "<what closed it>"` works with no code special-case for the
   type itself.
2. **`build_summary` computes `resolved_ids` generically** — every event's `resolves`
   field, not gated to `type == "resolution"` — so a future second way of closing
   something out (if the vocabulary ever grows one) is picked up automatically rather
   than needing a second code path.
3. **`deferrals.open` is now `len(open_deferrals)`** — an int count of deferrals whose
   id is not in `resolved_ids` — genuinely independent of `--limit`.
   `deferrals.resolved` is the complementary count. `deferrals.open_items` carries the
   full unresolved list (uncapped, deliberately — capping it would just move the same
   bug one field over). `deferrals.recent` keeps the old `--limit`-capped tail-slice
   behavior under an honest name, for whoever wants "what got deferred lately" rather
   than "what's still open."
4. `render_summary`'s text mode updated to print `N total, N open, N resolved` and
   iterate `open_items`.

### Why a new event type rather than a field on `deferral` itself

The log is append-only (`storage.format` in the schema: "Lines are never edited or
deleted"). A deferral can't gain a `resolved: true` field after the fact without
mutating its own line, and pointing a *new* `deferral` event at an old one would force
every resolution to also carry `deferral`'s required `what`/`reason` fields, which is
the wrong shape for "this one's done." A dedicated `resolution` type with a generic
`resolves` pointer keeps deferral emission single-purpose and reuses the same
`corrects`-style append-a-new-event-that-points-back pattern the schema already uses
for `correction`.

## Verification

- `python3 -m unittest scripts.tests.test_retro.TestDeferralResolution -v` → 5/5 pass,
  confirmed RED first (all three counting tests failed for the stated reason —
  `KeyError: 'open_items'`, `AssertionError: [...] != 1`, and the direct regression
  test `test_open_count_does_not_vary_with_limit` failing with the list-vs-int
  mismatch — before the fix landed).
- `python3 -m unittest scripts.tests.test_retro -v` → 39/39 pass (full existing suite,
  no regressions).
- `python3 scripts/retro.py summary --since 2026-08-22` → exits 0, text renders
  `DEFERRALS  29 total, 29 open, 0 resolved` (no `resolution` events exist yet in the
  live log, so every real deferral in the window correctly still reads open).
- `python3 scripts/retro.py summary --since 2026-08-22 --json --limit 3|10|29` → `open`
  is `29` at every limit — the direct fix for the reproduced defect.
- `python3 scripts/retro.py validate` → **88 problems across 2360 events**, same count
  with the pre-fix `scripts/retro.py` (checked directly, both runs against the
  unmodified event shards). These are pre-existing malformed lines in shards this cycle
  did not touch (`sd31-w11-integrate.jsonl`, `w27-prestige-class.jsonl`,
  `wave25b-class-feature-grant-interp.jsonl`) — out of this card's territory (event
  shards are not among `scripts/retro.py`, `docs/retro/schema.json`, or new test
  files), reported here rather than silently absorbed into "validate passes."
- Dual-audit gate on the diff (`scripts/retro.py`, `docs/retro/schema.json`,
  `scripts/tests/test_retro.py` only, base
  `1bb523773d32705d1b7387fd4c494861523f55ba`): `OK_NO_BUNDLE_TAGS`, `OK_NO_TOKENS`.

## Before/after figures (SD-32 window, `--since 2026-08-22`)

| | before (per diagnosed proof) | after |
|---|---|---|
| `deferrals.total` | 29 | 29 |
| `deferrals.open` @ `--limit 10` (default) | 10 | 29 |
| `deferrals.open` @ `--limit 3` | 3 | 29 |
| `deferrals.open` @ `--limit 29` | 29 | 29 |
| `deferrals.resolved` | (field did not exist) | 0 |

No `resolution` events have been emitted against the live log by this cycle — emitting
one for each of the 29 deferrals the SD-32 closure lane already re-verified by hand is
downstream work for whichever lane owns those deferrals' content, not this card's
territory (`scripts/retro.py`, `docs/retro/schema.json` only). This receipt fixes the
instrument; it does not re-run the 29 individual verifications.
