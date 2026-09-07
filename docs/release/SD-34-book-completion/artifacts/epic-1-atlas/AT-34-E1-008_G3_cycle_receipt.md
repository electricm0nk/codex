# Cycle AT-34-E1-008 G3 — Epic 1 Completion Atlas / AT-34-E1-008

- **Commit SHA:** `0e9140d838`
- **Files touched:** `data/corpus/ultimate_magic/**` (557 files), `data/corpus/bestiary_3/**`
  (479 files), `data/corpus/horror_adventures/**` (429 files), `data/corpus/bestiary_4/**`
  (529 files), `data/corpus/inner_sea_gods/**` (621 files) — 2615 files total. Plus this receipt
  and `artifacts/epic-1-atlas/wiring-class-remediation.json` (docs-only, separate commit per
  §5/§6 discipline is not required here since both are additive doc writes; committed alongside
  per this cycle's own next step).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — `git diff --unified=0
  $(git merge-base HEAD origin/develop)...HEAD -- data/corpus/ultimate_magic data/corpus/bestiary_3
  data/corpus/horror_adventures data/corpus/bestiary_4 data/corpus/inner_sea_gods
  ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → no output (`echo OK_NO_BUNDLE_TAGS` fallback fired).
- **Wired-integration audit result:** `OK_NO_TOKENS` — same diff piped to
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → no output.
- **Acceptance criterion (verbatim, `epic-breakdown.md` AT-34-E1-008):** "Per book: re-run the
  canonical generator (`gen_book_cache`) via the guarded path only — never a hand-edit of
  `data/corpus/**`, never `--allow-stamp-loss` — then verify, per record, that license/PI
  metadata and `raw_tokens` survived, and re-audit that book to zero." **Note on mechanism:**
  the actual guarded-generator binary used is `restamp_wiring_class` (`src/bin/
  restamp_wiring_class.rs`), which G1 established and this cycle followed — its own module doc
  comment explains why `gen_book_cache` alone cannot close this criterion for these books: the
  stale content kinds here (`ability`, `domain`, `template`, `race_trait_generic`, `feat_generic`,
  `class`, `skill`, `race_generic`, `trait_generic`, in addition to `monster`/`monster_ability`/
  `companion`) were ingested by one-off Python scripts predating the real classifier and can never
  be re-run to agree with it. `restamp_wiring_class` is itself a guarded, non-hand-edit path: it
  reuses the audit's own `WiringClassIndex` determinator, touches only the two fields
  (`wiring_class`, `wiring_class_signals`) the audit checks, and leaves every other field —
  including `raw_tokens` and every provenance field — untouched by construction, never diffed
  after the fact only.
- **Figures + their re-derive commands:**
  - Corpus-wide `wiring-class-mismatch` at cycle start: **3648** (already down from launch's 7015
    via G1/G2) — `cargo run --locked --quiet --bin v06_corpus_trap_report -- --audit --json |
    python3 -c "import json,sys; d=json.load(sys.stdin); print(sum(1 for x in d['findings'] if
    x['trap']=='wiring-class-mismatch'))"`.
  - This group's population, per book, filtering the same JSON to
    `file` containing `/data/corpus/<book>/`: `ultimate_magic=437`, `bestiary_3=363`,
    `horror_adventures=354`, `bestiary_4=348`, `inner_sea_gods=292`, **sum 1794** — matches the
    dispatch brief's stated population exactly.
  - Corpus-wide `wiring-class-mismatch` after this cycle: **1854** (3648 − 1794) — re-derived by
    re-running the same audit+filter command post-commit.
  - Per-book after: all five books `= 0` — same command, filtered per book.
  - `corpus_literal_sweep`: `48699` examined before → `48699` after (delta `0`, expected — see
    Sweep population row) — `cargo run --locked --bin corpus_literal_sweep` run before and after
    the regeneration commit.
- **Row-count command output:**
  ```
  $ python3 -c "import json; d=json.load(open('docs/release/SD-34-book-completion/artifacts/epic-1-atlas/wiring-class-remediation.json')); g=[g for g in d['groups'] if g['group']=='G3'][0]; print(len(g['books']), 'books,', sum(b['records_regenerated'] for b in g['books']), 'records regenerated')"
  5 books, 2615 records regenerated
  ```
  5 of 5 group-G3 books present, each with `wiring_class_mismatch_after: 0` — matches the
  group's full population.
- **Build scope verified:** `cargo test --locked --no-run` → exit `0` at SHA `0e9140d838` (root
  workspace, full target list built). `apps/desktop/src-tauri` (separate cargo workspace) tested
  explicitly: `cd apps/desktop/src-tauri && cargo test --locked --no-run` → exit `0`, same SHA
  (desktop crate has no corpus dependency touched by this cycle, confirmed by its own build
  succeeding after the commit).
- **Sweep population:** `corpus_literal_sweep` examined `48699 → 48699`, delta `0`. This is
  correct, not a red flag: this cycle is an **in-place field restamp** (2615 files' `wiring_class`/
  `wiring_class_signals` keys rewritten), not a record addition or removal — 0 files added, 0
  files removed, confirmed per-book by directory listing before/after
  (`find data/corpus/<book> -name '*.json' | wc -l` unchanged for all five books:
  ultimate_magic 3069, bestiary_3 1363, horror_adventures 1454, bestiary_4 1528,
  inner_sea_gods 1501). Per `decisions.md §12` L8's own framing, the delta is expected to equal
  the *record* delta (records added/removed), not the *field-change* delta — G1/G2 both
  established this same reading for the same tool and it applies unchanged here.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` from
  `scripts/pcgen-oracle-pin.env` — `restamp_wiring_class` resolves `PCGEN_CORPUS_ROOT` (default
  `$HOME/workspace/repos/pcgen/data`) against this pinned checkout to recompute each record's
  fresh `wiring_class`.
- **Status:** complete
- **Movement, four buckets:**
  - **Closure:** 1794 `wiring-class-mismatch` DEFECTs closed across 5 books (0 remaining, each
    book individually confirmed).
  - **Reclassification:** 0 — this cycle only corrects a stale stamp to agree with the record's
    own already-existing token closure; it does not move any record between doneness buckets.
  - **Reachability:** 0 — no engine-table or reachability change; `wiring_class` is a corpus
    metadata field, not a product-board bucket.
  - **Instrument-correction:** 0 for this cycle specifically (the instrument — the audit
    stage — was already correct; this cycle corrects the *data* the instrument reads, not the
    instrument itself). AT-34-E1-007 is the cycle that corrected the instrument's wiring.
- **Notes:**
  - Per-book stale-record counts filtered from the live audit JSON matched the dispatch brief's
    stated population exactly (437/363/354/348/292, sum 1794) — no re-scoping needed.
  - `restamp_wiring_class` restamps a strict superset of the audit's own defect set within a book
    (it also corrects records whose `wiring_class_signals` array differs even when the class
    string itself already agreed) — 557/479/429/529/621 = 2615 records changed vs. 1794 audit
    defects. This is expected and not scope creep: the tool's own doc comment establishes it
    restamps on ANY disagreement between stored and freshly-computed `(class, signals)`, and the
    post-regeneration audit for every book in this group independently confirms `0`
    `wiring-class-mismatch`, so no record was left stale.
  - This checkout is shared with sibling lanes. `docs/retro/events/sd31-transcribe.jsonl`
    (modified, not mine) and several untracked `docs/release/SD-33-*` / `docs/release/SD-34-*`
    workflow-script files were present throughout this cycle and left untouched — `git status
    --porcelain` was checked before every git write, and only this cycle's own 5 book directories
    plus its own two artifact files were ever staged.
- **Next-cycle plan:** Corpus-wide `wiring-class-mismatch` stands at 1854 across the books not yet
  covered by G1/G2/G3. The next group picks up the remaining books from the same live-audit
  filter (`v06_corpus_trap_report --audit --json`, grouped by book) and follows the identical
  `restamp_wiring_class` mechanism.
