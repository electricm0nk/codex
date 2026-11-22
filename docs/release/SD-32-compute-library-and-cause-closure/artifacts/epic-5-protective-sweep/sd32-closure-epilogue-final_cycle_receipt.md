# SD-32 closure epilogue — final-acceptance scan (BLOCKED)

RETRO_ACTOR=sd32-closure-epilogue · base commit `bdc5311cfc` on `tranche/12` · 2026-08-24

## Outcome

**Step 1 (final-acceptance scan) is SHORT. Per `workflow-instruction.md §13` and the dispatch
brief: no retrospective section written, no worktree/branch sweep executed, no PR opened, row 13
NOT marked `complete`.** This is the correct outcome for what was found, not a failure of the
cycle — SD-32's first closure attempt opened a PR over exactly this kind of gap and the operator
had to close it.

## What is short

`cd apps/desktop/src-tauri && cargo test --locked --bin codex-desktop` — **541 passed, 7 failed**
(prior cycle's own receipt, `row11-row13-final-closure_cycle-1_cycle_receipt.md`, recorded
**548 passed, 0 failed** at the same command before the `beginner_box` `EXCLUDED_BOOKS` carve-out
was removed at `22:21:04Z` on 2026-08-24 — this is a live regression introduced by that removal,
not a pre-existing red).

Failing tests, all tracing to one root cause — `beginner_box` is a real, now-ingested corpus book
(`data/corpus/beginner_box/`) that was never added to `apps/desktop/src-tauri/src/reach_gate.rs`'s
`CORPUS_BOOK_IDS` table, plus downstream pinned counts that assumed the old (pre-widening)
population:

```
corpus_ingest_diagnostic::tests::the_two_ingested_books_totals_reconcile_with_their_license_artifacts
equipment_catalog::tests::catalog_spans_every_ingested_book_with_their_real_counts
equipment_catalog::tests::description_coverage_is_pinned_per_book
equipment_catalog::tests::filter_equipment_catalog_matches_category_exactly_across_every_book
equipment_catalog::tests::keys_do_not_collide_across_books_and_crbs_own_duplicates_are_pinned
reach_gate::tests::dispatch_gap_race_and_monster_families_all_have_book_level_reach_arms
reach_gate::tests::the_inventory_is_populated_from_all_three_live_sources
```

Two representative panic messages (coordinates only, no PI terms):

```
reach_gate.rs:6708: corpus_inventory found directories this gate cannot name:
  ["data/corpus/beginner_box/ is an ingested book this gate cannot name.
    Add it to CORPUS_BOOK_IDS with the book_id the ingest diagnostic uses."]

advanced_race_guide: this diagnostic reports 506 records from rules_tables plus
1701 known corpus-only records, but a live walk of .../data/corpus/advanced_race_guide
accounts for 2208 real on-disk records. One of the two is stale --
re-derive corpus_only_records fresh (decisions.md §17a), never repin without proof.
  left: 2207
  right: 2208
```

This is exactly the shape of `count-change-needs-sweep-not-just-build`: the `EXCLUDED_BOOKS`
removal was a real, correct closure of the last open deferral (Decision §28 window) and grew the
population 34,397 → 34,416, but the sweep stopped at `data/corpus/` and never reached the pinned
Rust-side book registry and count assertions in `apps/desktop/src-tauri/src/`. Per this skill's
orchestrator/executor boundary, this session (a planning-doc-authoring closure epilogue, not a
dispatched build agent) does not edit `apps/desktop/src-tauri/src/*.rs` directly — that fix needs
its own dispatched cycle scoped to `apps/desktop/src-tauri`.

A second, unresolved measurement: `scripts/verify.sh --only declared-pi-audit` did not complete.
First attempt hit its own 300s `timeout` wrapper and was killed (`Terminated`, no PASS/FAIL
verdict). A second, unwrapped attempt ran the underlying `declared_pi_shipping_audit` binary at
sustained 99.9% CPU for over 6 additional minutes with no output past its `==>` banner line before
this cycle killed it to stay within its own turn. This gate previously reported `PASS (clean)` at
the pre-widening population (`row11-row13-final-closure` receipt). Whether it still passes at
34,416 records is **unverified**, not proven either way — report this as a gap, not a PASS.

## What DID verify clean (re-derived live, commands shown)

```
$ scripts/verify.sh --only shape-coverage-standing-gate
PASS  shape-coverage-standing-gate  (population=34416 unclassified=0 no_record=0
                                      corpus_sha=7f818006e371188e5717fd18d74d18a420747fc6)

$ python3 scripts/shape_ledger.py --inventory docs/work-inventory.json
population (not-done units considered): 34416
unclassified: 0
no_record: 0  (0.0%)

$ python3 scripts/row17_census.py --check
row 17 census -- population 34416
  ROW 17 HONEST SIZE                  0
  not_ingested (no_record)            0

$ python3 scripts/retro.py summary --since 2026-08-22 | grep -i DEFERRALS
DEFERRALS  29 total, 0 open, 29 resolved

$ scripts/verify.sh --only pi-sweep
PASS  pi-sweep  (10 hits over src/rules_core/rules_tables, 10 baseline rows)

$ scripts/verify.sh --only site-public-status-pi-gate
PASS  site-public-status-pi-gate  (31 file(s) scanned against 1612 declared-PI name(s), zero leaked)

$ scripts/verify.sh --only site-dashboard-pi-gate
PASS  site-dashboard-pi-gate  (21 file(s) scanned against 1612 declared-PI name(s), zero leaked)
```

## Movement since base commit `bdc5311cfc`

- **closure**: 0 — no step of `§13` closed this cycle; the scan stopped at Step 1 as instructed.
- **reclassification**: 0.
- **reachability**: 0 — population, `no_record`, `unclassified`, deferrals all unchanged and
  re-confirmed at the figures already established (34,416 / 0 / 0 / 29-0-29).
- **instrument-correction**: 1 confirmed, 1 unresolved.
  - Confirmed: the desktop cargo suite regressed 548/0 → 541/7 between the prior cycle's receipt
    and this one, root-caused to `CORPUS_BOOK_IDS` and downstream book-count pins not being swept
    when `beginner_box` was ingested. The instrument (the test suite) is correctly catching a real
    gap in the shipped registry, not itself broken.
  - Unresolved: `declared-pi-audit`'s runtime behavior at the new population size is itself now in
    question (previously fast enough to run inline; now exceeds 300s and was still running after
    6+ more minutes unwrapped). Needs its own timing investigation — flag, don't guess a cause.

## What did NOT run (correctly, per the BLOCKING gate)

Step 2 (retrospective closure section), Step 3 (worktree/branch sweep), Step 4 (release-notes
re-derivation), Step 5 (architecture-docs + graphify), Step 6 (version bump + PR), Step 7 (mark
row 13 `complete`). None of these are safe to run over an open acceptance-scan failure per
`docs/governance/blocker-closure-doctrine.md` and this skill's own closure-gate rule.

## What this cycle changed

Nothing in tracked source. This cycle is read-only verification plus this receipt. `git status
--porcelain` before writing this receipt showed only a pre-existing, not-mine modification to
`docs/retro/events/sd31-transcribe.jsonl` and untracked `data/corpus/**` files that are not this
cycle's — both left untouched.

## Recommended next step (not executed by this cycle — boundary)

Dispatch a build-scoped cycle against `apps/desktop/src-tauri/src/{reach_gate.rs,
equipment_catalog.rs, corpus_ingest_diagnostic.rs}` to: (1) add `beginner_box` to
`CORPUS_BOOK_IDS` with its ingest-diagnostic book_id, (2) re-derive every pinned per-book /
corpus-wide count the added book shifts (starting with `advanced_race_guide`'s 2207→2208 delta
named above — that delta implicates counts beyond `beginner_box` itself and needs its own live
re-walk, not a single-line patch), (3) re-run the full `cargo test --locked --bin codex-desktop`
to green, (4) separately re-run `declared-pi-audit` to a completed verdict (extend the timeout or
profile the binary) before this epilogue can retry Step 1.
