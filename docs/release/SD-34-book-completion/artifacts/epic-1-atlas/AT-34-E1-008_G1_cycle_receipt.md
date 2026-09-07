# Cycle AT-34-E1-008-G1 — Epic 1 Completion Atlas / AT-34-E1-008 (group G1)

- **Commit SHA:** `54e2d24e83`
- **Files touched:** `data/corpus/advanced_players_guide/**` (964 records), `data/corpus/core_rulebook/**`
  (1031 records), `src/bin/restamp_wiring_class.rs` (new),
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/wiring-class-remediation.json` (new, this
  group's rows), this receipt.
- **Identifier audit result:** matches only inside REMOVED lines of `data/corpus/**` JSON —
  old `wiring_class_signals` values named the writer that produced them (e.g.
  `"display:sd32_class_ingest"`, `"display:sd32_simple_filename_kind_ingest"`), 587 such
  removed-line matches, 0 in ADDED lines. These are corpus DATA field values being replaced by a
  fresh computed signal, not a code identifier introduced by this cycle — reported explicitly
  rather than silently marked `OK_NO_BUNDLE_TAGS`, per `workflow-instruction.md §6` step 2's own
  instruction to run the grep and report the real result.
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion (verbatim, `decisions.md §13`):** "The defects are driven to zero, as
  AT-34-E1-008... AT-34-E1-008's bar is `wiring-class-mismatch = 0`, with the other four trap
  kinds reported at their unchanged counts." This group's bar: `advanced_players_guide` and
  `core_rulebook` both at 0.

## Discovery: no existing canonical generator covers most of this population

`gen_book_cache.rs`/`gen_core_rulebook_cache.rs`/`gen_cache_apg.rs` already compute `wiring_class`
canonically (via `codex::rules_core::cache_gen::WiringClassIndex`, the same module
`src/pcgen_import/corpus_traps.rs::audit_ingested_cache` uses for its fresh recompute) for the
`companion`/`class`/`spell`/`equipment` kinds they generate. But of this group's 1,673 stale
records, only companion (APG 193, CRB 33) and class (APG 12, CRB 17) are covered by those three
binaries — 255 of 1,673. The remaining 1,418 live in `ability`/`domain`/`skill`/`template`/
`race_trait_generic`/`feat_generic`/`trait_generic`, whose only prior writers are one-off Python
scripts (`ingest_ability.py`, `ingest_generic_kind.py`, `ingest_simple_filename_kinds.py`,
`ingest_race_trait_generic.py`) that predate GE-01's real closure determinator and compute their
own much simpler heuristic — `ingest_ability.py`'s is literally `"static" if DEFINE/BONUS token
else "display"`, a two-value vocabulary that cannot ever produce `derived`/`computed`/`ambiguous`.
Re-running those scripts reproduces the same stale stamp; verified directly (see below).

**Verification that Python re-runs cannot close this:** the audit's own findings on, e.g.,
`core_rulebook/ability/1_arcane_caster_level.json` read `stored wiring_class "static" ...
disagrees with computed "computed" (signals ["computed:choice"])` — `"computed"` is not in
`ingest_ability.py`'s output vocabulary at all, so no re-run of that script could ever produce
agreement.

**Fix, following the repo's own established pattern.** `enrich_class_raw_tokens.rs`,
`enrich_equipment_raw_tokens.rs`, `enrich_spell_raw_tokens.rs`, and `enrich_companion_raw_tokens.rs`
already establish "an additive enrichment pass over EXISTING on-disk JSON, never a second
generator" for a DIFFERENT field (`raw_tokens`). `src/bin/restamp_wiring_class.rs` (new, this
cycle) is the same pattern for `wiring_class`/`wiring_class_signals`: it reuses
`WiringClassIndex`/`audit_ingested_cache`'s own book-dir-derivation logic verbatim (so it can
never disagree with what the audit checks), and rewrites ONLY those two top-level keys when they
disagree with a fresh recompute — every other field is parsed as a generic `serde_json::Value`
and re-emitted untouched by construction. 3 unit tests (RED confirmed by first writing the
assertions against the not-yet-written `restamp_one`, GREEN once implemented): a stale
`display`-stamped record with a real `BONUS:` token restamps to `static`; a record already
agreeing is byte-for-byte unrewritten (idempotence); a non-`lst_token` source is skipped.

## Figures + their re-derive commands

- **Baseline (both books, this group's population):**
  `cargo run --locked --bin v06_corpus_trap_report -- --audit --json`, filter
  `trap=="wiring-class-mismatch" and severity=="DEFECT"`, group by book —
  `advanced_players_guide=875`, `core_rulebook=798`, sum **1673 of 7015** (denominator: total
  `wiring-class-mismatch` defects across all 34 affected books at baseline).
- **Per-unit timing (measured before the full run, per §2.5):**
  `time cargo run --locked --bin restamp_wiring_class -- advanced_players_guide` = 3.9s wall
  for 2077 records scanned (964 restamped). Projected for the group's 2 books (~3,500 records
  total): well under 10s — confirmed by the actual `core_rulebook` run taking 2.6s for 1466
  records. No population-scoped risk at this scale.
- **After (both books):** same audit command — `advanced_players_guide=0`, `core_rulebook=0`,
  total corpus-wide `wiring-class-mismatch = 5342` (7015 − 1673).
- **Other four trap kinds, unchanged (out of this criterion's scope, `decisions.md §13`):**
  `mod-record=2117`, `key-differs-from-name=650`, `shared-name-distinct-records=249`,
  `disabled-line=165` — identical before/after.
- **Provenance survival, per record:** Python diff of every record in both books (backup taken
  before regeneration) on `data`/`source`/`license`/`pi_field`/`pi_marker`/`codex_generated_name`/
  `rename`/`population`/`completeness` — **4460 of 4460** `advanced_players_guide` records and
  **6720 of 6720** `core_rulebook` records unchanged on every field except `wiring_class`/
  `wiring_class_signals`; 0 files added, 0 removed.

## Row-count command output

```
$ python3 -c "import json; d=json.load(open('docs/release/SD-34-book-completion/artifacts/epic-1-atlas/wiring-class-remediation.json')); g=[x for x in d['groups'] if x['group']=='G1'][0]; print(len(g['books']), 'books', sum(b['wiring_class_mismatch_after']==0 for b in g['books']), 'at zero')"
2 books 2 at zero
```

## Build scope verified

`cargo test --locked --no-run` exits 0 (root workspace, run at `54e2d24e83`).
`cd apps/desktop/src-tauri && cargo test --locked --no-run` exits 0 (separate cargo workspace,
run at `54e2d24e83`, its own `CARGO_TARGET_DIR`). Run **after** the last commit that could move a
figure (`decisions.md §12` L7) — this cycle's only commit.

## Sweep population

`cargo run --locked --bin corpus_literal_sweep`: **48699 examined before → 48699 examined after**,
delta 0. Correct: this group added or removed 0 records (in-place field restamp only, verified
above), so `decisions.md §12` L8's "population must grow when records are added" does not apply —
nothing was added. 0 findings, CLEAN both before and after.

## Oracle pin

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`) — the
local `~/workspace/repos/pcgen` checkout is pinned at exactly this SHA (confirmed via
`git -C ~/workspace/repos/pcgen log -1 --format=%H`), and is the corpus every figure above was
re-derived against.

## Status: complete

## Movement, four buckets

- **Closure:** 1673 of 1673 group-population `wiring-class-mismatch` defects driven to 0.
- **Reclassification:** 0 — no record's `population`/`completeness` bucket changed.
- **Reachability:** N/A — `wiring_class` describes a corpus record's own shape, not engine
  reachability; `v06_work_inventory` recomputes `wiring_class` independently from raw `.lst`
  source (confirmed in `b32926f2af`'s own commit message), so this restamp moves nothing on the
  product board.
- **Instrument-correction:** 0 — the audit instrument itself did not change; the corpus catches
  up to what it already checks.

## Notes

- Grepped old/new record counts across `tests/`, `src/`, `apps/`, `scripts/` for either book:
  record counts are unchanged (0 added/removed), and no test file under `tests/` reads
  `wiring_class` directly (`grep -rn '\.wiring_class\b\|\["wiring_class"\]' tests/*.rs` returns
  nothing) — no hardcoded assertion needed updating.
- `src/bin/restamp_wiring_class.rs` also restamped a small number of records whose `wiring_class`
  STRING already agreed but whose `wiring_class_signals` differed (e.g. stale
  `sd32_class_ingest`-tagged signals) — the audit's own comparison (`corpus_traps.rs`) only checks
  the class string, so these were not counted in the 1673 baseline, but restamping them brings
  both books to full agreement on both fields, which is strictly more correct and touches nothing
  the audit or any test reads.
- This receipt's identifier-audit finding (587 removed-line matches, 0 added) is reported in full
  above rather than collapsed to `OK_NO_BUNDLE_TAGS`, per this cycle's own reading of
  `workflow-instruction.md §6` step 2.

## Next-cycle plan

G1's two books are done. The remaining 32 books' `wiring-class-mismatch` populations (5342 of
7015) are owned by sibling G-groups in this same wave; `src/bin/restamp_wiring_class.rs` is
reusable as-is for any book (`cargo run --locked --bin restamp_wiring_class -- <book>
[<book> ...]`) — no further per-book code changes should be needed, only the same
run/verify/receipt loop this cycle used. AT-34-E1-007's own `exits 0` bar and AT-34-E6-001's
re-run of both instruments at HEAD are the closing checks once every group lands.
