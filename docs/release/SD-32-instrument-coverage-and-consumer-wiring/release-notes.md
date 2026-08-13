---
canonical: true
owner: god-emporer
status: template (populated at closure)
date: 2026-08-13
---

# SD-32 Release Notes — Instrument Coverage and Consumer Wiring

_Populated at closure. Structure fixed now so the closure cycle cannot quietly
drop the honesty sections._

## What moved

| kind | units that legitimately reached their existing bar | bar cleared | invocation |
|---|---:|---|---|
| _(populated at closure)_ | | | |

## What was examined and left alone

**This section is mandatory and may not be empty** (`decisions.md §1.2`). A
bundle that examined 10,209 movable units and reports no left-alone set has
either moved everything — which the structural findings say is impossible — or
has not been honest about its shortfall.

| reason class | units | why the bar could not be legitimately reached |
|---|---:|---|
| _(populated at closure)_ | | |

## What stayed structurally blocked

- `spell` — 1,281 held units. No consumer reads a spell magnitude
  (`decisions.md §5`).
- `static`/`derived` — 7,479 held units, unless `decisions.md §2` was answered
  yes. State the answer here either way.

## Shortfall against the scope ceiling

`scope-draft.md §6` set the ceiling at 734 + yield × 1,776 with no measurement
change, and said plainly that "a plausible honest outcome for this bundle is
several hundred units, not several thousand." **State the actual figure against
that ceiling, unrounded.**

## Bar integrity statement

**Required.** State explicitly, with the E8 review's evidence:

- No threshold, classifier definition, bucket definition or check predicate was
  weakened.
- `equipment_key_is_wired()`'s body is unmodified.
- No file under `/home/ubuntu/swarm-observer/` or the producer skill directory
  was touched.
- No `held` figure was reported as, or summed with, `done`.

## Verification

`./scripts/verify.sh` FULL, exit code captured directly, per closing cycle.
