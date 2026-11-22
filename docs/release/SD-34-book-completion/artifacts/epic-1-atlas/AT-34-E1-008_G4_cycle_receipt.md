# Cycle AT-34-E1-008-G4 — Epic 1 Completion Atlas / AT-34-E1-008 (group G4)

- **Commit SHA:** `e519f4ea45`
- **Files touched:** `data/corpus/{bestiary_2,ultimate_wilderness,advanced_race_guide,
  mythic_adventures,advanced_class_guide,adventurers_guide,inner_sea_magic,
  pathfinder_unchained,inner_sea_races,inner_sea_faiths,bestiary_5,ultimate_combat,
  inner_sea_bestiary,book_of_the_damned_volume_2,ultimate_equipment,
  book_of_the_damned_volume_1,inner_sea_world_guide,ultimate_intrigue,bonus_bestiary,
  occult_adventures,inner_sea_intrigue,monster_codex,bestiary_6,inner_sea_combat}/**`
  (3194 records, in-place field restamp only, 0 added/removed),
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/wiring-class-remediation.json`
  (this group's rows appended), this receipt. No code files touched — `src/bin/restamp_wiring_class.rs`
  was landed by G1 and reused as-is, per its own next-cycle plan.
- **Identifier audit result:** not run separately — no code changed this cycle (data-only regeneration
  via an already-landed tool). `OK_NO_TOKENS` for the wired-integration check (no code changed).
- **Wired-integration audit result:** `OK_NO_TOKENS` — no code changed.
- **Acceptance criterion (verbatim, `decisions.md §13`):** "The defects are driven to zero, as
  AT-34-E1-008... AT-34-E1-008's bar is `wiring-class-mismatch = 0`, with the other four trap
  kinds reported at their unchanged counts." This group's bar: all 24 of G4's books at 0.

## Mechanism

`cargo run --locked --bin restamp_wiring_class -- <24 books>`, the tool G1 landed and identified as
reusable as-is (its own next-cycle plan: "no further per-book code changes should be needed, only
the same run/verify/receipt loop"). Confirmed true here — zero code changes, one data-only run
covering all 24 books in a single invocation (17.4s wall).

## Figures + their re-derive commands

- **Baseline (this group's 24 books):**
  `cargo run --locked --bin v06_corpus_trap_report -- --audit --json`, filter
  `trap=="wiring-class-mismatch" and severity=="DEFECT"`, group by book (Python, `re.search`
  on `data/corpus/([^/]+)/` in each finding's `file`) —
  bestiary_2=216, ultimate_wilderness=216, advanced_race_guide=185, mythic_adventures=168,
  advanced_class_guide=167, adventurers_guide=138, inner_sea_magic=124, pathfinder_unchained=86,
  inner_sea_races=85, inner_sea_faiths=83, bestiary_5=66, ultimate_combat=65,
  inner_sea_bestiary=57, book_of_the_damned_volume_2=48, ultimate_equipment=36,
  book_of_the_damned_volume_1=30, inner_sea_world_guide=28, ultimate_intrigue=17,
  bonus_bestiary=13, occult_adventures=13, inner_sea_intrigue=4, monster_codex=4,
  bestiary_6=3, inner_sea_combat=2 — sum **1854 of 1854** (corpus-wide remaining
  `wiring-class-mismatch` DEFECT total at the start of this cycle, confirming G4's 24 books are
  *exactly* the corpus-wide remainder after G1+G2+G3's 5161 — no book outside G4's list carried
  a mismatch at baseline; verified by also checking `set(counts) - set(mygroup) == {}`).
- **Per-unit timing (measured before the full run, per §2.5):**
  `time cargo run --locked --bin restamp_wiring_class -- bestiary_2` (largest single book by
  mismatch count) = 4.4s wall for 1381 records scanned (494 restamped). Projected for the group's
  remaining 23 books at a similar or better per-record rate: well under 60s total. Actual full-group
  run (all 24 books, one invocation) = 17.4s wall — confirms the projection.
- **After (all 24 books):** same audit command — every G4 book = 0. Corpus-wide
  `wiring-class-mismatch = 0` (1854 − 1854), so AT-34-E1-007's own `exits 0` bar for this trap is
  now met corpus-wide (pending the other three groups' books, already at 0 per their receipts).
- **Other four trap kinds, unchanged (out of this criterion's scope, `decisions.md §13`):**
  re-derived from the same post-run audit JSON — `mod-record=2117 DEFECT / 407 TRAP`,
  `key-differs-from-name=650`, `shared-name-distinct-records=249`, `disabled-line=165` — identical
  to `decisions.md §13`'s own figures (`2,117`, `650`, `249`, `165`).
- **Provenance survival, per record:** every one of the 3194 changed files (`git status --porcelain
  -- data/corpus`, all `M`, 0 added/removed) parsed as JSON and diffed old (`git show HEAD:<file>`
  before this cycle's commit) vs new on `data`/`source`/`license`/`pi_field`/`pi_marker`/
  `codex_generated_name`/`rename`/`population`/`completeness` — **3194 of 3194** unchanged on
  every field except `wiring_class`/`wiring_class_signals`; 0 mismatches.
- **Records regenerated per book:** bestiary_2=494, advanced_race_guide=431,
  ultimate_wilderness=417, advanced_class_guide=217, inner_sea_world_guide=202,
  mythic_adventures=198, adventurers_guide=167, bestiary_5=137, pathfinder_unchained=132,
  inner_sea_magic=127, book_of_the_damned_volume_2=107, inner_sea_races=102,
  inner_sea_faiths=85, ultimate_combat=71, inner_sea_bestiary=63, bestiary_6=48,
  occult_adventures=42, ultimate_equipment=41, book_of_the_damned_volume_1=35,
  ultimate_intrigue=24, bonus_bestiary=17, inner_sea_combat=14, monster_codex=13,
  inner_sea_intrigue=10 — sum 3194 (matches the file-status count above). Note this is larger than
  the 1854 mismatch baseline: like G1, the tool also restamps records whose `wiring_class` string
  already agreed but whose `wiring_class_signals` array differed — the audit only compares the
  class string, so these were not in the 1854 baseline, but restamping brings both fields to full
  agreement, strictly more correct, touching nothing the audit or any test reads.

## Row-count command output

```
$ python3 -c "import json; d=json.load(open('docs/release/SD-34-book-completion/artifacts/epic-1-atlas/wiring-class-remediation.json')); g=[x for x in d['groups'] if x['group']=='G4'][0]; print(len(g['books']), 'books', sum(b['wiring_class_mismatch_after']==0 for b in g['books']), 'at zero')"
24 books 24 at zero
```

## Build scope verified

`cargo test --locked --no-run` exits 0 (root workspace, run against the working tree at the
content committed as `e519f4ea45`). `cd apps/desktop/src-tauri && cargo test --locked --no-run`
exits 0 (separate cargo workspace, own `CARGO_TARGET_DIR`). Run after the corpus regeneration —
this cycle's only content change — per `decisions.md §12` L7.

## Sweep population

`cargo run --locked --bin corpus_literal_sweep`: **48699 examined before → 48699 examined after**,
delta 0. Correct: this group added or removed 0 records (in-place field restamp only, verified
above), so `decisions.md §12` L8's "population must grow when records are added" does not apply —
nothing was added. 0 findings, CLEAN both before and after.

## Oracle pin

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`) —
`restamp_wiring_class` recomputes `wiring_class` from each record's own already-ingested citation
(`data.source`), not from a fresh oracle read, so this cycle's figures do not depend on the pinned
checkout being present; the corpus-wide sweep and audit figures above are the record of what was
checked.

## Status: complete

## Movement, four buckets

- **Closure:** 1854 of 1854 group-population `wiring-class-mismatch` DEFECTs driven to 0.
- **Reclassification:** 0 — no record's `population`/`completeness` bucket changed (verified in
  the provenance diff above).
- **Reachability:** N/A — `wiring_class` describes a corpus record's own shape, not engine
  reachability; `v06_work_inventory` recomputes `wiring_class` independently from raw `.lst`
  source, so this restamp moves nothing on the product board.
- **Instrument-correction:** 0 — the audit instrument itself did not change; the corpus catches up
  to what it already checks.

## Notes

- Grepped old/new record counts across `tests/`, `src/`, `apps/`, `scripts/`: 0 records
  added/removed for any of the 24 books (confirmed via `git status --porcelain -- data/corpus`
  showing only `M` lines), and `grep -rn '\.wiring_class\b\|\["wiring_class"\]' tests/*.rs` returns
  nothing — no hardcoded test assertion reads `wiring_class` directly, so nothing needed updating.
- All 24 books were regenerated in a single `restamp_wiring_class` invocation (space-separated
  book list) rather than 24 separate calls — the tool accepts multiple books per run and this
  measured faster (17.4s total) than the per-book baseline would project (24 × ~4-5s serially,
  largely startup-cost-dominated).
- Idempotence re-verified live: a second run of the same command against the now-regenerated tree
  reported `0 restamped` for all 24 books (all "already agreed"), confirming the change is stable
  and the audit's own agreement is what was achieved, not an artifact of a single pass.
- This is the fourth and, per the dispatch brief's book list, final G-group for AT-34-E1-008 —
  corpus-wide `wiring-class-mismatch` is now 0 across all 37 books (G1: 2, G2: 3, G3: 5, G4: 24 =
  34 books with prior mismatches + 3 books that never had any, matching `decisions.md §13`'s
  "34 of 37 books" figure). AT-34-E1-007's own `verify.sh --only corpus-trap-audit` should now
  report `wiring-class-mismatch=0` corpus-wide; not re-run here as it is AT-34-E1-007's own closing
  check, owned by AT-34-E6-001's re-derivation at HEAD once every group's commit lands.

## Next-cycle plan

G4's 24 books are done. Combined with G1 (2 books), G2 (3 books), G3 (5 books), all books
carrying a `wiring-class-mismatch` at AT-34-E1-007's original baseline are now at 0. The remaining
step is AT-34-E6-001's re-run of `v06_corpus_trap_report --audit` and `verify.sh --only
corpus-trap-audit` at HEAD, once all four groups' commits are on `tranche/14`, to confirm the
corpus-wide `exits 0` bar AT-34-E1-007 itself is gated on.
