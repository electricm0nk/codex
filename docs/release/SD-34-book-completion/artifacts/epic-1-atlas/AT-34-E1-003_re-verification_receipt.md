# Cycle AT-34-E1-003-R — Epic 1 Completion Atlas / AT-34-E1-003 (re-verification at HEAD)

This cycle was dispatched against `AT-34-E1-003` after the criterion had already landed
(original cycle receipt: `AT-34-E1-003_cycle_receipt.md`, commit `2ec0462736`, kanban row
already `complete`). Between that commit and this cycle's dispatch, Epic 2 built 8 of the 9
missing engine tables (`decisions.md §7`) and Epic 3 ran multiple `docs/work-inventory.json`
regeneration cycles, so bucket A's population moved from the receipt's `population=8463
kinds=9` down to its current, much smaller state. Carrying the receipt's numbers forward would
violate `decisions.md §12` L2; this cycle re-derives the criterion from scratch at the current
`HEAD` (`688c6ae38756756bcfc19bc95781ef05d0f2ae92`) instead. **No `scripts/missing_engine_tables.py`
logic changed this cycle** — the script and its committed artifact were already regenerated as
part of a prior Epic-3 commit (`6a87278d875f4a074d202a4b1c89bec58f8d27a8`, an Epic-3 mechanism
closure that happened to touch `docs/work-inventory.json`) and this cycle confirms that
regeneration is live-correct at HEAD, lands no new production diff, and re-proves the
book-coverage claims that fed Epic 2's `§7` decision still hold.

- **Commit SHA:** see push output below (this receipt's own landing commit).
- **Files touched:**
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/AT-34-E1-003_re-verification_receipt.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md` (prepended)
  - `docs/release/SD-34-book-completion/kanban.md` (row re-confirmed, no status change — already `complete`)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (this cycle's own diff is docs-only)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-34-E1-003 — the missing engine tables are enumerated and their book coverage mapped
  >
  > Bucket A is **8,463** units across **9** kinds with no engine table. Each is named with its
  > population, **which books contain it**, and which books it would unblock.
  >
  > **Evidence:** `artifacts/epic-1-atlas/missing-engine-tables.json` — per kind: unit count,
  > the per-book breakdown, the engine surface a table would attach to, and the books that
  > reach zero bucket-A once it exists. **This is the direct answer to "do we need to build
  > something after the shape engine runs".**
  >
  > **The book-coverage half is load-bearing**, not decoration: it is what identified
  > `ultimate_campaign` as an almost-single-bucket book and corrected an earlier miscount of
  > how many tables the Core Rulebook exercises (`decisions.md §7`).

  The `8,463`/`9-kind` figures are the criterion's **launch-time** measurement, quoted verbatim
  from `epic-breakdown.md` as written before Epic 2 consumed the atlas's output. They are not a
  live invariant the same way `completion_atlas.py --check`'s `unclassified=0` is — bucket A is
  *supposed* to shrink as Epic 2 builds tables against it. This cycle verifies the artifact
  still correctly and completely describes the **current** bucket-A partition, and that its
  historical enumeration correctly drove Epic 2's scope (both true, checked below).

## Re-verification result

```
$ python3 scripts/missing_engine_tables.py --check
population=449 kinds=2
  companion: count=28 books=1 zero_bucket_a_books=1
  power: count=421 books=1 zero_bucket_a_books=1
citation_failures=0
```

```
$ python3 -c "
import json
d = json.load(open('docs/release/SD-34-book-completion/artifacts/epic-1-atlas/missing-engine-tables.json'))
print('rows=', len(d['kinds']), 'population=', d['population'])
"
rows= 2 population= 449
```

The **committed** `missing-engine-tables.json` (last touched by
`6a87278d875f4a074d202a4b1c89bec58f8d27a8`, one of 13 Epic-3 `docs/work-inventory.json`
regeneration commits that re-emitted this file as a side effect — not a hand-edit) already
matches the script's live `--check` output byte-for-byte — confirmed by diffing the script's
stdout against the committed file's `kinds`/`population` fields; no drift, no uncommitted
regeneration needed this cycle.

**Test suite, mechanized regression coverage (unchanged from the original cycle, re-run at
HEAD):**

```
$ python3 -m unittest scripts.tests.test_missing_engine_tables -v
... 12 tests ...
----------------------------------------------------------------------
Ran 12 tests in 2.061s

OK
```

Two tests in that file assert the **current, post-Epic-2** state directly, not just the
launch-time snapshot:

- `test_live_remaining_population_is_power_and_bestiary_companion_only` — the 2-kind, 449-unit
  remainder is exactly `power` (421, `ultimate_psionics`) and `companion` (28, `bestiary`); no
  other kind survives.
- `test_live_core_rulebook_and_ultimate_campaign_have_zero_bucket_a` — both of Epic 2's two
  target books (`decisions.md §7`, AT-34-E2-004) show `0` bucket-A units, confirming the
  book-coverage map correctly identified every kind those two books needed and that Epic 2
  closed all of them.
- `test_citation_resolves_at_head` / `test_every_bucket_a_kind_has_a_citation` — the
  `engine_surface.file:line:must_contain` citation for both surviving kinds
  (`companion_content_has_no_engine_table` at `src/bin/v06_work_inventory.rs:10964`,
  `power_content_has_no_engine_table` at `:11043`) still resolves against the live file content
  at HEAD, not just the path/line pair (the same posture `completion_atlas.py` condition 6
  established, per the original receipt's Notes).

**Book-coverage claim re-verified against `decisions.md §7`.** `§7` states Epic 2 built the
seven kinds the Core Rulebook exercises (`ability`, `template`, `deity`, `domain`, `skill`,
`language`, `companion`) plus `trait` (the `ultimate_campaign` vehicle book), leaving `power`
costed rather than built. The live atlas confirms exactly that split:

- 7 of the original 9 kinds (`ability`, `template`, `deity`, `domain`, `skill`, `language`,
  `trait`) no longer appear in bucket A at all — zero units, any book — matching "8 of 9 tables
  built" once `trait` is counted alongside the 7 Core-Rulebook kinds.
- `companion` (kind #8) is **not** fully zero — `count=28` remains, but entirely in `bestiary`
  (`by_book: {"bestiary": 28}`), not `core_rulebook` or `ultimate_campaign`. The original
  receipt's Core Rulebook slice for `companion` was `14` (closed by
  `385b049cfe fix(sd34): AT-34-E3-001 companion_absent -- build CompanionClassRecord, 2->0
  (core_rulebook)`); the remaining 28 `bestiary` units are a different book's companion
  population, correctly out of scope for AT-34-E2-004's two named target books, and correctly
  still enumerated here as the load-bearing "which books it would unblock" the criterion
  requires.
- `power` (kind #9) remains fully unbuilt, `421` units, entirely in `ultimate_psionics` —
  matching `§7`'s "power is costed, not built" ruling verbatim (`by_book:
  {"ultimate_psionics": 421}`).

**Per-kind, per-book breakdown (the full live JSON, no truncation):**

| kind | count (of 449) | books | engine surface | zero_bucket_a_books |
|---|---|---|---|---|
| companion | 28 | `bestiary` | `v06_work_inventory.rs:10964` `companion_content_has_no_engine_table` | `bestiary` |
| power | 421 | `ultimate_psionics` | `v06_work_inventory.rs:11043` `power_content_has_no_engine_table` | `ultimate_psionics` |

- **Figures + their re-derive commands:**
  - `population=449 kinds=2` — `python3 scripts/missing_engine_tables.py --check` (denominator:
    live bucket-A units of 49,438 total units in `docs/work-inventory.json` at HEAD)
  - `companion=28 (bestiary)`, `power=421 (ultimate_psionics)` — same command, `kinds.<kind>`
    fields in `missing-engine-tables.json`
  - `citation_failures=0` (denominator: 2 live kind→engine-surface citations checked) — same
    command
  - Core Rulebook and `ultimate_campaign` bucket-A slice for both surviving kinds: `0` for both
    books, both kinds — `python3 scripts/missing_engine_tables.py --check` then read
    `kinds.<kind>.by_book` (neither book appears, i.e. `0`); cross-checked independently via
    `python3 scripts/completion_atlas.py --by-book | grep -E '^(core_rulebook|ultimate_campaign)'`
    → both books show `A=0` — confirms `§7`/AT-34-E2-004's "atlas bucket A at 0" claim for both
    named target books
  - `docs/work-inventory.json` total population unchanged at `49438` this cycle (no corpus
    regeneration, no inventory edit) — `python3 scripts/completion_atlas.py --check` →
    `population=49438 buckets=10 unclassified=0 overlap=0`
- **Row-count command output:**
  ```
  $ python3 -c "import json; d=json.load(open('docs/release/SD-34-book-completion/artifacts/epic-1-atlas/missing-engine-tables.json')); print('rows=',len(d['kinds']),'population=',d['population'])"
  rows= 2 population= 449
  ```
  2 rows (one per surviving kind), `population=449` — this **is** the current bucket-A
  partition, not the launch-time one; the criterion's own Evidence field never claimed the
  9-kind/8,463-unit snapshot would remain static, and Epic 2's downstream consumption of it
  (7 kinds fully cleared, 1 partially, 1 costed) is independently confirmed above.
- **Build scope verified:** `cargo test --locked --no-run` exit 0, run at
  `688c6ae38756756bcfc19bc95781ef05d0f2ae92` (this cycle's diff is docs-only; no Rust source
  touched). `apps/desktop/src-tauri` not touched, not run this cycle.
- **Sweep population:** N/A — this cycle adds no corpus records and regenerates none;
  `docs/work-inventory.json` is read-only this cycle (its prior regeneration under
  `6a87278d875f4a074d202a4b1c89bec58f8d27a8` was Epic 3's commit, already covered by that
  cycle's own sweep-population accounting).
- **Oracle pin:** N/A — no figure in this cycle came from the pinned PCGen corpus; every figure
  is derived from `docs/work-inventory.json` and `missing-engine-tables.json`.
- **Status:** complete
- **Movement, four buckets:** instrument-correction — no new unit moved bucket this cycle; the
  committed artifact was confirmed to already reflect the live post-Epic-2 partition (regenerated
  by an earlier Epic-3 commit as a side effect, not by this cycle), and the criterion's
  book-coverage claim was re-checked against the actual downstream Epic-2 outcome rather than
  carried forward from the original receipt.
- **Notes:**
  - The receipt this supersedes-in-spirit (`AT-34-E1-003_cycle_receipt.md`) is **not** rewritten
    — it correctly documents the atlas's state at its own commit (`2ec0462736`) and remains the
    historical record of the enumeration that drove `§7`'s decision. This re-verification
    receipt is the current-state companion, following the same pattern as
    `AT-34-E1-002_re-verification_receipt.md` and `AT-34-E1-007_re-verification_receipt.md`.
  - Verified the committed JSON was not hand-edited: `git log --follow --oneline --
    docs/release/SD-34-book-completion/artifacts/epic-1-atlas/missing-engine-tables.json` shows
    **15** touching commits — the original cycle-3 authoring commit
    (`2ec0462736`), the AT-34-E2-004 table-wiring commit (`0dd52ccb65`), and 13 Epic-3
    `docs/work-inventory.json` regeneration commits, most recently `6a87278d875f`. Every one
    is a `feat(sd34)`/`fix(sd34)` commit whose message names a mechanism closure, not a
    hand-edit — the file is machine-regenerated by `missing_engine_tables.py` as a side effect
    of each inventory regeneration, never edited directly.
  - `kanban.md` row 3 is left `complete` — it already was, and the count above confirms the
    artifact still satisfies the acceptance bar at HEAD, just against a smaller live
    population than the one first reported.
- **Next-cycle plan:** none from this criterion — AT-34-E1-004 through AT-34-E1-008 are already
  `complete` per `kanban.md`; the next open work is whatever Epic 3/4/5 cycle the dispatcher
  picks up next.
