# Cycle 4 — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`companion_absent_from_core_rulebook_companion_tables` mechanism)

Mechanism owned this cycle: `companion_absent_from_core_rulebook_companion_tables` — one of the
nine bucket-B mechanisms `decisions.md §14` decomposed AT-34-E3-001 into. THREE prior cycles ran
this mechanism (100 → 28, 28 → 28, 28 → 14) and all three receipts are READ, not repeated
(`AT-34-E3-001_companion_absent_cycle_receipt.md`, `_2.md`, `_3.md`). This cycle's mandate,
verbatim: the remaining 14 split 12/2 — (a) close the 12 grant-token-only rows via a per-record,
corpus-wide VERIFIED predicate, never a shape-only reclassification, or state why it cannot be
built safely this cycle; (b) close the 2 monster-class rows via a genuine level-progression record
type verified against `ultimate_magic` (3 rows) and `book_of_the_damned_volume_1` (2 rows) as the
second and third consumers. Report `partial` if only one is taken.

Population re-derived at HEAD (not transcribed from any prior receipt), **before** any change:

```
python3 -c "
import json
d=json.load(open('docs/work-inventory.json'))
u=d['units']
cr=[x for x in u if x['book']=='core_rulebook' and x['status']=='engine-does-not-hold' and x['evidence']=='companion_absent_from_core_rulebook_companion_tables']
print(len(cr))"
-> 14
```

Matches the third cycle's own after-figure exactly.

- **Commit SHA:** `faae8dd4972096c17439fd18c7b93196e35bd275`
- **Files touched:**
  - `src/rules_core/rules_tables/companion_chassis.rs` — added
    `GRANT_TOKEN_ONLY_DISPATCH_ROWS: &[(&str, &str)]` (12 named keys + reason strings) and
    `grant_token_only_dispatch_reason(key)` lookup, mirroring
    `class_feature_pool_catalog::vacuous_placeholder_reason`'s established named-list pattern
    (never a live shape scan); replaced `companion_absent_14_sub_causes_are_named_and_sum_exactly`
    with `companion_absent_2_sub_causes_are_named_and_sum_exactly` (re-derives the now-2-unit
    population against live `docs/work-inventory.json`); added
    `grant_token_only_rows_dispatch_to_already_held_content` (proves, per record, against the live
    corpus AND the live work-inventory, that each of the 12 keys is genuinely zero-content and
    every one of its `ABILITY:` targets is a `core_rulebook` companion row this engine ALREADY
    holds — `grounded`/`text-complete`/`literal-verified`, not merely "a corpus file exists").
  - `src/bin/v06_work_inventory.rs` — `Kind::Companion` arm: consults
    `companion_chassis::grant_token_only_dispatch_reason` immediately before its final
    `companion_absent_from_<book>_companion_tables` fallback, returning `deferred-with-reason`
    (bucket B → X) for a match; no other production logic changed.
  - `scripts/completion_atlas.py` — this cycle's own insertion shifted two of the ten hardcoded
    `BUCKET_DEFINITIONS` citations (bucket A `has_no_engine_table` 10583→10601, bucket V
    `literal-verified` 11234→11252); re-derived both with `grep -n` against the post-edit file and
    fixed the literals in this same cycle, per the workflow instruction's own warning. The other
    four citations (B 10281, C 10506, D 8916, M 8751) sit before this cycle's insertion point and
    were re-checked unchanged.
  - `docs/work-inventory.json` — regenerated (`v06_work_inventory` release binary, with
    `CORPUS_LITERAL_SWEEP_REPORT` and `DERIVED_FIXTURE_CHECK_REPORT` set) — the ONLY units it moved
    are the 12 grant-token-only `core_rulebook` companion rows (verified below).
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_companion_absent_cycle_receipt_4.md` (this file)
  - `docs/retro/events/sd34-at-34-e3-001.jsonl` — one `resolution` event closing the 12-row portion
    of cycle 3's grant-token-only deferral, one `deferral` event for the 2-row class-row remainder.
  - `docs/release/SD-34-book-completion/progress.md`, `kanban.md` (prepend/update, §5)

## What was built, and why the 2 class rows stayed open

**The 12 grant-token-only rows — CLOSED, via a verified per-record predicate.** Cycle 3's own
atlas defect 3 warned that its shape query (`ABILITY` present, no `TYPE`/`DESC`/`BONUS`) matches
461 of 51,482 corpus-wide, and that reclassifying by shape alone risks the exact 188-record
near-miss defect 1's own investigation already recorded. Before building anything, this cycle
re-checked that exact concern: applying defect 3's shape query corpus-wide and then testing
"every `ABILITY:` target resolves in-book to a record carrying real content" against all 461
matches finds only **171 safe**, **104 whose target exists but carries no content**, and **280
whose target key cannot even be found in-book** — confirming a shape-only rule would silently
misclassify 290 of 461 records and is genuinely unsafe corpus-wide.

What this cycle built instead is narrower and stronger: a **closed, named 12-key list**
(`GRANT_TOKEN_ONLY_DISPATCH_ROWS`), each entry verified individually against BOTH the live corpus
(zero-content shape) AND the live `docs/work-inventory.json` (every `ABILITY:` target's own engine
status is already `grounded`, `text-complete`, or `literal-verified` — not merely "a corpus record
exists with some content", which is what the corpus-wide re-check above shows is not sufficient).
All 12 `core_rulebook` rows pass this check with zero counter-examples — every one of their
`ABILITY:` grant tokens fans out only to `Animal Companion ~ *` ability rows this engine already
ships. The other 449 of 461 corpus-wide matches of the bare shape are untouched; only the 12 rows
this mechanism owns, individually verified, are reclassified.

**The 2 monster-class rows — NOT closed, per this cycle's own mandate to report honestly.**
Re-checked directly against the live corpus (`cr_classes_companion.lst:6`, `:15`): `CLASS:` rows
with `VISIBLE:NO`, an `HD:`/`MAXLEVEL:` progression, `BONUS:VAR|Class…` tokens — no `SIZE:`,
`MOVE:`, `NATURALATTACKS:`. `companion_chassis.rs`'s own module doc comment already states this
chassis models three record shapes (creature, ability, class) and explicitly does NOT model the
third: "Hit-dice progressions, neither creature nor ability; this chassis does not model them and
no registered book carries one." Confirmed this cycle: `ultimate_magic` (3 `_classes_companion.lst`
rows: `Vermin Companion`, `1`, `Black Blade`) and `book_of_the_damned_volume_1` (2 rows: `1`,
`Imp Companion`) both carry the identical shape and are already registered `COMPANION_BOOKS`
entries with no class-row support either — real second and third consumers exist, exactly as the
mandate named. Building a level-progression record type, wiring it into `classify()`, and
verifying it against all 3 consumers inside this same cycle — on top of the 12-row predicate work
above — was judged to risk exactly the narrow, single-consumer widening Shape 8's own cycle 3
already warned against; left honestly `engine-does-not-hold`, named here with its 3 real consumers
for the next cycle, rather than rushed.

## Row-count command output (before -> after, this mechanism)

```
BEFORE: 14   (companion_absent_from_core_rulebook_companion_tables, core_rulebook, engine-does-not-hold)
AFTER:  2    (12 grant-token-only rows closed to deferred-with-reason; 2 class-definition rows remain)
```

Re-derive command (same as above, run against HEAD after this cycle's regen):
`python3 -c "import json; d=json.load(open('docs/work-inventory.json')); u=d['units']; print(len([x for x in u if x['book']=='core_rulebook' and x['status']=='engine-does-not-hold' and x['evidence']=='companion_absent_from_core_rulebook_companion_tables']))"` -> `2`

## Figures + re-derive commands

| Figure | Value | Command |
|---|---:|---|
| Mechanism population (before) | 14 of 14 | see above |
| Mechanism population (after) | 2 of 14 | see above |
| Units closed this cycle | 12 | 14 - 2 |
| Destination status of the 12 closed units | `deferred-with-reason` (bucket X), evidence `grant_token_only_dispatch_row_routes_to_already_shipped_content` | `python3 -c "import json;d=json.load(open('docs/work-inventory.json'));print(len([x for x in d['units'] if x['book']=='core_rulebook' and x['status']=='deferred-with-reason' and x['evidence']=='grant_token_only_dispatch_row_routes_to_already_shipped_content']))"` -> `12` |
| Corpus-wide re-check of defect 3's own bare shape query (safe / unsafe / target-missing) | 171 / 104 / 280 of 461 | `companion_chassis::GRANT_TOKEN_ONLY_DISPATCH_ROWS`'s own doc comment carries the re-runnable script; re-run manually this cycle over `data/corpus/*/*/**/*.json` |
| `core_rulebook` companion units by status (all statuses) | 184 total: 156 held pre-cycle-3 + 14 via cycle 3 + 12 now `deferred-with-reason` + 2 remaining B | `python3 -c "import json;d=json.load(open('docs/work-inventory.json'));import collections;print(collections.Counter(x['status'] for x in d['units'] if x['book']=='core_rulebook' and x['kind']=='companion'))"` |
| `completion_atlas.py` full check | population=49438 buckets=10 unclassified=0 overlap=0, citation_failures=0 | `python3 scripts/completion_atlas.py --check` |
| `completion_atlas.py` bucket B (corpus-wide, all books/mechanisms; other AT-34-E3-001 cycles also moved units this session) | 12,123 (down from 12,188 after cycle 3) | `python3 scripts/completion_atlas.py --check` |
| `completion_atlas.py` bucket X (deferred-with-reason) | 61 (includes this cycle's 12 plus the pre-existing 49) | same command |
| Total corpus population (unchanged by this cycle) | 49,438 | `python3 -c "import json;print(json.load(open('docs/work-inventory.json'))['totals']['units'])"` |
| `corpus_literal_sweep` examined population | 48,708 of 51,482, CLEAN, 0 findings (unchanged — this cycle adds 0 `data/corpus` records) | `/tmp/cargo-sd34-at-34-e3-001/release/corpus_literal_sweep --json-out /tmp/sd34-e3-001-reports/sweep.json` |
| `derived_evaluator_fixture_check` | 1,839 units cleared over 2,580 fixture rows, 0 failed | `/tmp/cargo-sd34-at-34-e3-001/release/derived_evaluator_fixture_check --json-out /tmp/sd34-e3-001-reports/derived.json` |
| `denominator_gate.py` against this package | violations=0 (15 files checked) | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` |

## Verification

- TDD: RED confirmed by re-running `cargo test --lib rules_core::rules_tables::companion_chassis`
  immediately after the `companion_chassis.rs`/`v06_work_inventory.rs` edits and BEFORE
  regenerating `docs/work-inventory.json` — `companion_absent_2_sub_causes_are_named_and_sum_exactly`
  failed for the intended reason (`left: 14, right: 2` — the JSON inventory still reflected the
  pre-fix world). `grant_token_only_rows_dispatch_to_already_held_content` passed immediately
  (it reads the live corpus and the live, not-yet-regenerated inventory — the 12 targets were
  already engine-held before this cycle, since they are pre-existing `Animal Companion ~ *` ability
  rows, not new content). GREEN after the inventory regen: all 18 `companion_chassis` tests pass.
- Full `cargo test --lib` (workspace lib target): **2,884 passed, 0 failed, 14 ignored.**
- `cargo test --bin v06_work_inventory`: **395 passed, 0 failed.**
- **Build scope verified:** `cargo test --locked --no-run` (workspace, all bin/test targets)
  exits 0. `apps/desktop/src-tauri` (separate cargo workspace) also run explicitly this cycle:
  `cargo test --locked --no-run` exits 0 (`CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001-desktop`).
  Both run AFTER the last commit in this cycle that can move a figure this receipt depends on
  (`decisions.md §12` L7).
- `python3 scripts/completion_atlas.py --check`: `citation_failures=0` — this cycle's own two
  shifted citations (bucket A, bucket V) re-derived by `grep -n` against the post-edit file and
  fixed in this same cycle, per the criterion's own line-number-drift warning.
- `docs/work-inventory.json` regen touched ONLY the 12 grant-token-only `core_rulebook` companion
  units — verified by diffing every `corpus_key` the regen changed
  (`git diff docs/work-inventory.json | grep '^[+-]' | grep -o '"corpus_key": "[^"]*"' | sort -u`)
  against the 12-key `GRANT_TOKEN_ONLY_DISPATCH_ROWS` list; they match exactly.
- Identifier audit (this cycle's own working-tree diff, `companion_chassis.rs` +
  `v06_work_inventory.rs` + `completion_atlas.py`): `OK_NO_BUNDLE_TAGS`.
- Wired-integration audit (same own-diff scope): `OK_NO_TOKENS`. The epic's CUMULATIVE
  `${BASE_BRANCH}...HEAD` diff over the same file-touch set still shows `placeholder` matches, all
  from EARLIER, already-merged AT-34-E3-001 sub-cycles' vacuous-placeholder-class-feature work
  (real corpus content and a real, already-merged named list, not an implementation stub, and not
  this cycle's diff — same precedent cycle 3's own receipt already recorded).

## Movement, four buckets

- **Closure:** 0 units reached a DONE-tier or verified-tier status this cycle (the 12 closed units
  land in bucket X, `deferred-with-reason` — a genuine placement, but not DONE or V).
- **Reclassification:** 12 units (bucket B → X). Each is a real, individually-verified
  reclassification — every one's `ABILITY:` targets are proven, per record, to already be
  engine-held content — never a shape-only relabel. This is the honest "shelf, not half-fix"
  outcome `decisions.md §2`'s bucket-X meaning describes: these rows carry no content of their own
  to compute or display, and their real job (dispatching to content that already ships) is now the
  engine's own recorded, tested reason for their status.
- **Reachability:** 0 new `reach_gate` findings — these 12 rows carry no `DESC:`/`BONUS:` token, so
  there is nothing new for any render path to reach; the content they dispatch to was already
  reachable before this cycle.
- **Instrument-correction:** 0. This is real, individually-verified work (a 12-key predicate proven
  against the live corpus and live engine state), never a count-only fix.

- **Status:** partial

## Remainder (2 units), named by sub-cause — none of it is a shrug

| Sub-cause | Units | Why it is not closed this cycle |
|---|---:|---|
| `cr_classes_companion.lst` PCGen monster-class definitions (`Companion`, `Shadow Companion`) | 2 | A level-progression record type genuinely new to this chassis, confirmed again against the live corpus rows. Two real second/third consumers confirmed this cycle (`ultimate_magic`: `Vermin Companion`, `1`, `Black Blade`; `book_of_the_damned_volume_1`: `1`, `Imp Companion` — 5 more rows, 7 total corpus-wide across the 3 named books), but building and verifying the type against all 3 in the same cycle as the 12-row predicate above was judged out of this cycle's safe scope. |

**2 = 2.** Every remaining unit is named by sub-cause with a population;
`companion_absent_2_sub_causes_are_named_and_sum_exactly` (committed, re-runnable) proves this
against live `docs/work-inventory.json`.

## Next-cycle plan

This mechanism has 2 units left, all of one sub-cause, with its 2 additional real consumers now
named and confirmed (not merely asserted): `ultimate_magic` (3 rows: `Vermin Companion`, `1`,
`Black Blade`) and `book_of_the_damned_volume_1` (2 rows: `1`, `Imp Companion`), both already
registered `COMPANION_BOOKS` entries. A future cycle should build a genuine level-progression
record type (`CLASS:` shape: `VISIBLE:NO`, `HD:`/`MAXLEVEL:` progression, `BONUS:VAR|Class…`
tokens — distinct from both `CompanionRecord` and `CompanionAbilityRecord`), wire it into
`companion_chassis`, and verify it against all 3 consumers (7 rows total) in the same cycle, per
this cycle's own mandate. AT-34-E3-001 itself remains open — the other eight mechanisms are owned
by other cycles.
