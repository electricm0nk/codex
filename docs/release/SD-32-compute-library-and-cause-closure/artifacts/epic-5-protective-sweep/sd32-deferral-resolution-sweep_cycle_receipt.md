# Cycle sd32-deferral-resolution-sweep — make the deferral log tell the truth

- **Card ID:** `epic-5-protective-sweep` — retro-log correction pass.
- **Territory:** `docs/retro/events/` (new `resolution` events only, no edits/deletes of existing
  lines), and this receipt. Did NOT touch `scripts/retro.py` (already fixed), `data/corpus/**`, or
  `scripts/site/**`.
- **Files touched:** `docs/retro/events/sd32-deferral-resolution-sweep.jsonl` (27 new `resolution`
  events, this actor's own shard). `docs/retro/events/sd31-transcribe.jsonl` gained one
  auto-emitted `verification` line from this cycle's own `scripts/verify.sh --only
  shape-coverage-standing-gate` run (side effect of `verify.sh`'s own instrumentation, not a hand
  edit — that shard's actor default is `sd31-transcribe`, not overridden here).
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6`, fetched fresh this cycle via
  `scripts/fetch-pcgen-oracle.sh --dest <repo-local artifacts/corpus/operator-supplied/pcgen>`
  (the slot was empty at cycle start).
- **Status:** complete

## Starting state

`scripts/retro.py summary --since 2026-08-22` read `29 total, 29 open, 0 resolved` — correct that
`open` is now `--limit`-invariant (the sibling lane's fix), wrong in the other direction: no
`resolution` events had ever been emitted because the mechanism didn't exist until this bundle's
own fix landed, so every real deferral still read open even where sibling lanes had since closed
the underlying defect.

## Method

For each of the 29 SD-32-window deferrals (`python3 scripts/retro.py query --since 2026-08-22
--type deferral --json --limit 100`), determined resolution status by evidence I could reproduce
myself this cycle — a live command re-run, not a receipt's prose claim on trust:

- `scripts/verify.sh --only shape-coverage-standing-gate` (live) → `PASS (population=34416
  unclassified=0 no_record=14 corpus_sha=7f818006e371188e5717fd18d74d18a420747fc6)`.
- `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output <path>` (live) →
  the **only** `no_record` rows in the entire 34,416-unit population are `(equipment,
  beginner_box)` × 14 — the known-still-open item the brief named. Every other kind
  (`class_feature`, `ability`, `template`, `race_trait`, `spell`, `feat`, `equipment_modifier`,
  `monster_ability`, `companion`, `trait`, `deity`, `power`, `monster`, `domain`, `class`, `skill`,
  `language`, `race`) is at zero. This directly re-derives against the live corpus (`shape_ledger.py`
  builds its own corpus index fresh every run) rather than trusting `docs/work-inventory.json`'s own
  `status` field, which I found to be **stale** (its `not-ingested` count for `deity` — 459 —
  matched the *old* population exactly, but `find data/corpus -path '*/deity/*' -name '*.json' |
  wc -l` live returned 459 *existing* files: the kind is ingested, the inventory's status column
  just hadn't been regenerated since). Named as a finding, not silently absorbed — any future cycle
  reading `docs/work-inventory.json`'s per-unit `status` field for a similar question should
  re-derive against the live corpus first.
- `python3 scripts/census_independent.py --pcgen-root <oracle> --inventory docs/work-inventory.json`
  (live) → `unexplained=0`.
- `python3 scripts/pi_key_rawtokens_audit.py --json-out <path>` (live) → `confirmed_records=0`
  corpus-wide over 27,619 scanned records (60-term SIGNED-OFF blacklist); `candidate_records=27092`
  unratified-vocabulary heuristic hits, top terms (`Base`, `Weapon`, `Racial`, `Sorcerer`,
  `Fighter`, …) are common SRD-open words, not real PI.
- `cargo run --locked --bin corpus_literal_sweep` (live) → `48632 records examined … 0 findings …
  CLEAN`.
- `python3 -m unittest scripts.tests.test_sd32_t9_pi_normalization_and_inheritance -v` (live) →
  21/21 pass, including the `Section26JarnJamFoldCollisionTests` mutation-proof suite.
- `python3 scripts/row17_census.py --check` (live) → `ROW 17 HONEST SIZE 0`.
- `python3` walk of `data/corpus/*/class/*.json` (live) → 168/168 carry `data.raw_tokens`.
- `cd apps/desktop/src-tauri && cargo test --locked --bin codex-desktop` (live, backgrounded and
  awaited in-turn) → **548 passed, 0 failed** — the whole separate cargo workspace.
- `cargo test --locked --lib untabled_base_class_chassis` (live) → 11/11 pass, including
  `registry_loads_all_20_corpus_derived_entries`.
- `cargo test --locked --lib class_feature_grant_consumer` (live) → 31/31 pass, including the
  T7/D12 regression test.
- `python3 scripts/observer/pf1e_dashboard_producer.py --out <path>` (live) →
  `work_inventory.classifier_reclassified_units.count=12`, unit list matching the T8/D13 deferral's
  own named 12 exactly.
- `bash scripts/publish-site-dashboard.sh --check` (live) → first line `site/dashboard/
  PF1e-dashboard.json is current` (a later, unrelated crash in the same script,
  `build_public_status.py`'s own `KIND_LABELS` gap, is lane E's own defect, not this deferral's
  claim).
- `grep -c remove_dir_all` / `remove_stale_owned_files` in `src/bin/gen_book_cache.rs` (live).
- `docs/release/…/decisions.md` read directly for the two items resolved by named operator ruling
  (Decision 24 — deity/class_feature neutral-name ingest; Decision 26 — the `rn`→`m` fold
  exemption) rather than by a gate re-run alone.

Cross-checked every figure against `kanban.md` row 11 (`epic-2-cause-closure`, status `complete`),
row 12 (`epic-3-class-reachability`, `complete`), and row 15 (`census-scope-closure`, `complete`,
"zero-gap confirmed") — consistent with all of the above, never substituted for it.

## Result: 27 of 29 resolved, 2 genuinely still open

```
python3 scripts/retro.py summary --since 2026-08-22
DEFERRALS  29 total, 2 open, 27 resolved
```

`--limit`-invariance re-confirmed: `--json --limit 3` and `--limit 29` both report `open: 2`.

**The 2 that remain open, by id, and what each needs:**

1. **`1787493382983-t9-onboarding-9161f5`** — "Decide whether `ogl-pi-blacklist.md`'s SIGNED-OFF
   60-term vocabulary should expand to cover the 23,090(now 27,092)-record candidate population."
   Live-checked this cycle: the SIGNED-OFF list was **not** expanded (`decisions.md` §19/§19a
   approved a specific 60 terms, not a standing license to add more), and the candidate population
   is dominated by common SRD-open words (verified against `top_terms` live). **Needs:** the named
   operator ruling — review `artifacts/gate-3-closure-invariant/pi-key-rawtokens-corpus-report.md`'s
   candidate sample and rule on which (if any) sampled terms join the blacklist.
2. **`1787601153844-t9-onboarding-537f43`** — row 13's worktree/branch sweep, 128 of 142 worktrees
   confirmed removable via dry-run, execution refused for a worktree-isolated dispatched agent
   (confirmed again this cycle — I am also worktree-isolated and out of `docs/retro/events/`'s own
   scope for this anyway). **Needs:** the orchestrating (non-worktree-isolated) session re-runs the
   merge/clean check immediately before each removal and executes `git worktree remove` +
   `git branch -D` directly.

## Known caveat honored

`beginner_box`'s units are being ingested by a sibling lane concurrently with this cycle
(`shape_ledger.py`'s live `no_record=14`, all `equipment/beginner_box` — up from 0 at an earlier
point in the branch's history per `progress.md`). No resolution was emitted for anything touching
that population; it was not one of the 29 SD-32-window deferrals in the first place (grepped for
`beginner_box` across all 29 — zero hits), so this is a note, not a correction to the resolved
list above.

## One instrument finding, named plainly

`docs/work-inventory.json`'s per-unit `status` field is stale relative to the live corpus (see
"Method" above, the `deity` example). `shape_ledger.py`'s own `no_record` figure is NOT stale —
it re-derives its corpus join fresh against `data/corpus/**` on every run, independent of the
inventory JSON's `status` column (only the inventory's `units` list — the population, not their
status — feeds the join). Any future reader treating `work-inventory.json`'s `status: not-ingested`
as current truth without cross-checking the live corpus will reproduce this exact false-negative.

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (both the new shard and the one auto-appended
  line in `sd31-transcribe.jsonl`).
- **Wired-integration audit result:** `OK_NO_TOKENS`.
- **Discovery forwards:** none new requiring a card — the `docs/work-inventory.json` staleness
  finding is recorded above for the next reader, not a defect in scope to fix here.
- **Next-cycle plan:** the 2 open items above are what SD-32's closure scan should consume; no
  further action inside this deferral-resolution sweep's own territory.
