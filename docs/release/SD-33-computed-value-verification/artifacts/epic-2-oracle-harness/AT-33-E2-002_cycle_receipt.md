# Cycle AT-33-E2-002 — Epic 2 Oracle harness / AT-33-E2-002

- **Commit SHA:** `84a5781c11`
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/fixtures/pf1_fighter_l1.pcg` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/computed-values.txt.ftl` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/pf1_fighter_l1.computed.txt` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/build-transcript-05-batchexport-SUCCESS.log` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/AT-33-E2-002_cycle_receipt.md` (this file)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E2-002 — a character round-trips through the oracle
  >
  > One authored `.pcg` input exports through `BatchExporter` via a template that emits computed variables, and the output is machine-readable.
  >
  > **Evidence:** the `.pcg`, the template, the emitted output, and the command — all committed. **Path A is not "established" until a real value comes out.**

## What landed

Hand-authored `fixtures/pf1_fighter_l1.pcg` — a Level 1 Human Fighter using
only `CAMPAIGN:Core Rulebook` (STR 16 / DEX 14 / CON 14 / INT 10 / WIS 10 /
CHA 8). Hand-authored `computed-values.txt.ftl` — a minimal FreeMarker
BatchExporter template emitting PCGen's own **computed** variables (not
literal LST text) as machine-readable `KEY=VALUE` lines: `HP`, `AC.Total`,
`AC.Touch`, `AC.Flatfooted`, `ATTACK.MELEE.BASE` (BAB), `VAR.CMB.INTVAL`,
`VAR.CMD.INTVAL`, and all three saves (via a `<#list pc.checks as check>`
loop, avoiding a hardcoded save-index-to-ability assumption). Every token
name was cross-checked against the stock `outputsheets/base.xml.ftl` at the
same pinned SHA before authoring, so the template uses PCGen's real
export-token vocabulary rather than guessed names.

Ran the real export via `./gradlew run --args=...` (full command in
`README.md`); output committed at `pf1_fighter_l1.computed.txt`.
**A real value came out** of a genuinely-exercised code path, not a stand-in.

**Independent cross-check**, derived from PF1e core rules *before* looking
at the run's output, then compared:

| Field | Hand-derived (RAW) | Real oracle output | Match |
|---|---:|---:|---|
| STR mod | +3 | +3 | yes |
| DEX mod | +2 | +2 | yes |
| CON mod | +2 | +2 | yes |
| HP (d10 max @ L1 + CON) | 12 | 12 | yes |
| AC total (10 + DEX) | 12 | 12 | yes |
| AC touch | 12 | 12 | yes |
| AC flat-footed | 10 | 10 | yes |
| BAB (full, L1) | +1 | +1 | yes |
| Fortitude (L1/2+2 + CON) | +4 | +4 | yes |
| Reflex (L1/3 + DEX) | +2 | +2 | yes |
| Will (L1/3 + WIS) | +0 | +0 | yes |
| CMB (BAB + STR) | 4 | 4 | yes |
| CMD (10 + BAB + STR + DEX) | 16 | 16 | yes |

13 of 13 independently-derived values match the real oracle output exactly.

## First attempt failed for the intended reason

The first export attempt (before the checkout's sparse cone included
`data/homebrew`/`data/_universal`) failed with `Could not find campaign by
filename` / `NoSuchFileException` on
`data/homebrew/conversion_support/conversion_support.pcc` — `core_rulebook`
transitively references that file, and it sat outside the pin's
`data/pathfinder`-only sparse scope. Widening the checkout's sparse cone to
include both top-level `data/` directories fixed it (see
`AT-33-E2-001_cycle_receipt.md`'s "What landed" and this same cycle's
`README.md`). That failing transcript is not separately committed — the
cause is fully described here and in `README.md`, and the corrected run
(`build-transcript-05-batchexport-SUCCESS.log`) is the real evidence.

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| Export command exit code | 0 | of 1 (final, corrected) attempt | `build-transcript-05-batchexport-SUCCESS.log`, last line `BUILD SUCCESSFUL` |
| `SEVERE`-level log lines in the successful run | 0 | of the full transcript | `grep -c SEVERE docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/build-transcript-05-batchexport-SUCCESS.log` |
| Independently-derived values matching the real oracle output | 13 | of 13 fields checked (table above) | manual comparison, table above; re-derive the oracle side with `cat docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/pf1_fighter_l1.computed.txt` |
| Output lines emitted | 26 | of the template's 26 `${pcstring(...)}`/loop-generated lines | `wc -l docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/pf1_fighter_l1.computed.txt` |

## Status: complete

## Movement, four buckets

- **closure:** 0 — this cycle proves a round-trip mechanism; it moves no inventory unit.
- **reclassification:** 0
- **reachability:** 0
- **instrument-correction:** 0

## Notes

- The `.pcg` deliberately uses only the Core Rulebook campaign (not the
  richer multi-campaign samples already present in
  `code/testsuite/PCGfiles/*.pcg`), to keep this cycle's failure surface
  small and timeboxed (`decisions.md §5`) — those samples were read only to
  confirm the `.pcg` tag vocabulary, never copied as content.
- Fixture identifiers (filenames, `unit_id` strings) were renamed once,
  before any commit, to their current non-bundle-prefixed form after this
  cycle's own identifier audit flagged the original names — the transcripts
  and outputs committed here are the clean, post-rename re-run, not
  hand-edited originals.

## Next-cycle plan

`AT-33-E2-003` (same cycle, same commit) builds the comparison harness
against this real committed export; `AT-33-E2-004` records the ruling.
