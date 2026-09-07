# Cycle AT-34-E1-008-G2 — Epic 1 Completion Atlas / AT-34-E1-008 (group G2)

- **Commit SHA:** `8df70c2ee4`
- **Files touched:** `data/corpus/beastiary/**` (1464 records), `data/corpus/ultimate_psionics/**`
  (860 records), `data/corpus/ultimate_campaign/**` (170 records),
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/wiring-class-remediation.json`
  (this group's rows appended), `docs/release/SD-34-book-completion/progress.md` (Cycle 8 entry),
  `docs/release/SD-34-book-completion/kanban.md` (row 8 → `in-progress`), this receipt.
  `src/bin/restamp_wiring_class.rs` is reused unchanged from G1's commit `54e2d24e83` — no code
  change this cycle.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to
  `data/corpus/beastiary data/corpus/ultimate_psionics data/corpus/ultimate_campaign
  src/bin/restamp_wiring_class.rs`, base = `git merge-base HEAD origin/develop` = `ea2b3396f2`).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope/base).
- **Acceptance criterion (verbatim, `decisions.md §13`):** "The defects are driven to zero, as
  AT-34-E1-008... AT-34-E1-008's bar is `wiring-class-mismatch = 0`, with the other four trap
  kinds reported at their unchanged counts." This group's bar: `beastiary`,
  `ultimate_psionics`, `ultimate_campaign` all at 0.

## Mechanism

Followed G1's established precedent exactly (`src/bin/restamp_wiring_class.rs`), not
`gen_book_cache` — G1's receipt documents why: of the stale kinds in these books
(`ability`/`class`/`companion`/`race_trait_generic`/`feat_generic`/`trait_generic`/`race`/
`monster`/`monster_ability`/`template`/`language`), only `companion`/`class` are covered by any
canonical generator (`gen_book_cache`'s monster-book path), and re-running the one-off Python
ingest scripts that wrote the rest can never agree with the audit (they predate the
`derived`/`computed`/`ambiguous` vocabulary entirely). `restamp_wiring_class` is itself the
"guarded generator path" `decisions.md §13`/`N5` require: additive, reuses the audit's own
`WiringClassIndex` determinator so it can never disagree with what is checked, rewrites only
`wiring_class`/`wiring_class_signals`, and every other field is parsed generically and
re-emitted untouched by construction (verified below, not assumed).

`ultimate_campaign` is not a monster/companion book at all (no `gen_book_cache` spec entry) —
its `ability`/`feat`/`trait_generic` kinds were never coverable by that path in the first place,
reinforcing that `restamp_wiring_class` (not `gen_book_cache`) is the real mechanism for this
criterion across all remaining books, exactly as G1 found.

## Figures + their re-derive commands

- **Baseline (this group's population):**
  `cargo run --locked --bin v06_corpus_trap_report -- --audit --json`, filter
  `trap=="wiring-class-mismatch"`, group by the `/data/corpus/<book>/` segment of `file` —
  `beastiary=783`, `ultimate_psionics=759`, `ultimate_campaign=152`, sum **1694 of 5342**
  (denominator: total `wiring-class-mismatch` defects across the 32 books remaining after G1's
  advanced_players_guide+core_rulebook closure — `7015 - 1673 = 5342`). Matches the dispatch
  brief's stated ~1694 exactly.
- **Per-unit timing (measured before the full run, per §2.5):**
  `time cargo run --locked --bin restamp_wiring_class -- ultimate_campaign` (smallest book,
  n=1 first) = 0.42s wall for 419 records scanned (170 restamped). Projected for the group's
  6,557 on-disk records (2608 + 3530 + 419) at ~1000 records/sec: well under 10s. Actual full
  run (`beastiary ultimate_psionics` together) took 18.2s wall (dominated by `sys` time —
  parallel filesystem contention from sibling groups' concurrent builds/regens on the shared
  checkout, not a per-record cost problem). No population-scoped risk at this scale either way.
- **After (this group):** same audit command — `beastiary=0`, `ultimate_psionics=0`,
  `ultimate_campaign=0`, total corpus-wide `wiring-class-mismatch = 3648` (5342 − 1694, i.e.
  7015 − 1673 − 1694 across G1+G2).
- **Other four trap kinds, unchanged (out of this criterion's scope, `decisions.md §13`):**
  `mod-record=2117`, `key-differs-from-name=650`, `shared-name-distinct-records=249`,
  `disabled-line=165` — identical before/after (this tool only ever touches
  `wiring_class`/`wiring_class_signals`, never these trap kinds' underlying fields).
- **Provenance survival, per record:** `git diff` of every one of the 2,494 files this cycle
  changed, against `HEAD` (the pre-restamp commit), checked field-by-field on
  `data`/`source`/`license`/`pi_field`/`pi_marker`/`codex_generated_name`/`rename`/`population`/
  `completeness` — **2494 of 2494** records unchanged on every field except `wiring_class`/
  `wiring_class_signals`; 0 files added, 0 removed (`git diff --diff-filter=A|D --name-only`
  both empty for all three book subtrees). Additionally confirmed via `git diff --stat` +
  `git diff | grep -E '^[+-]"' | grep -v wiring_class` returning empty across the full diff —
  no other JSON key appears in any added/removed line of any of the 2,494 files.

## Row-count command output

```
$ python3 -c "import json; d=json.load(open('docs/release/SD-34-book-completion/artifacts/epic-1-atlas/wiring-class-remediation.json')); g=[x for x in d['groups'] if x['group']=='G2'][0]; print(len(g['books']), 'books', sum(b['wiring_class_mismatch_after']==0 for b in g['books']), 'at zero')"
3 books 3 at zero
```

## Build scope verified

`cargo test --locked --no-run` exits 0 (root workspace, run at `8df70c2ee4`, `CARGO_TARGET_DIR=
/tmp/cargo-sd34-remediation`, shared warm cache, re-claimed at cycle start).
`cd apps/desktop/src-tauri && cargo test --locked --no-run` exits 0 (separate cargo workspace,
run at `8df70c2ee4`, own `CARGO_TARGET_DIR=/tmp/cargo-sd34-remediation-desktop`). Run **after**
the last commit that could move a figure (`decisions.md §12` L7) — this cycle's only commit.

## Sweep population

`cargo run --locked --bin corpus_literal_sweep`: **48699 examined before → 48699 examined
after**, delta 0. Correct: this group added or removed 0 records (in-place field restamp only,
verified above), so `decisions.md §12` L8's "population must grow when records are added" does
not apply — nothing was added. 0 findings, CLEAN both before and after.

## Oracle pin

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`) —
confirmed the local `~/workspace/repos/pcgen` checkout is pinned at exactly this SHA
(`git -C ~/workspace/repos/pcgen log -1 --format=%H`); it is the corpus the audit and the
`WiringClassIndex` recompute against.

## Status: complete

## Movement, four buckets

- **Closure:** 1694 of 1694 group-population `wiring-class-mismatch` defects driven to 0.
- **Reclassification:** 0 — no record's `population`/`completeness` bucket changed.
- **Reachability:** N/A — `wiring_class` describes a corpus record's own shape, not engine
  reachability; `v06_work_inventory` recomputes `wiring_class` independently from raw `.lst`
  source (per `b32926f2af`'s own commit message, and confirmed unaffected here — this restamp
  moves nothing on the product board).
- **Instrument-correction:** 0 — the audit instrument itself did not change; the corpus catches
  up to what it already checks.

## Notes

- Grepped old/new record counts across `tests/`, `src/`, `apps/`, `scripts/` for all three
  books: record counts are unchanged (0 added/removed), and no test file under `tests/` reads
  `wiring_class`/`wiring_class_signals` as a value assertion (only shape/schema tests reference
  the field names themselves, unaffected by a value restamp) — no hardcoded assertion needed
  updating.
- `restamp_wiring_class`'s `find_record_files` walks one directory level below the book dir
  (`book_dir/<kind>/*.json`). `beastiary` has some kinds one level deeper still
  (`race_trait/<subrace>/*.json`, e.g. `race_trait/tiefling/`) that this one-level walk does not
  reach directly — verified this does not leave any stale record behind: the post-run audit
  (which uses its own, book-relative recursive citation lookup, unrelated to this tool's file
  walk) reports `beastiary=0` mismatches with **no exceptions**, so every record the audit
  itself examines already agrees. The un-walked nested files were confirmed to be either
  already-agreeing before this run or non-`lst_token`-sourced (out of this tool's scope by
  design, same as G1's `no-lst-citation (untouched)` bucket).
- This is a **parallel-group** criterion: G1 (`advanced_players_guide`, `core_rulebook`) landed
  first at `54e2d24e83`; this is G2. `kanban.md` row 8 is set to `in-progress`, not `complete`,
  because the corpus-wide bar (`wiring-class-mismatch=0` across all originally-affected 34
  books) is not yet met — 3648 of 7015 remain, owned by sibling groups not covered by this
  dispatch.

## Next-cycle plan

G2's three books are done. `src/bin/restamp_wiring_class.rs` remains reusable as-is
(`cargo run --locked --bin restamp_wiring_class -- <book> [<book> ...]`) for any remaining book
— no further code changes anticipated. Sibling groups (however the remaining ~30 books are
split) run the same loop this cycle and G1 used. AT-34-E1-007's own `exits 0` bar and
AT-34-E6-001's re-run of both instruments at HEAD are the closing checks once every group lands;
kanban row 8 flips to `complete` only then.
