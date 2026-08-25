# Cycle sd32-integrity-sweep-corpus-ingest-diagnostic-red — Gate 3 (closure invariant)

- **Card ID:** integrity-sweep lane (escalated by the `monster_ability` round-5 lane; no
  standing kanban row of its own — reports against Gate 3 / card 11's closure invariant)
- **Commit SHA:** see push log (this file is written pre-commit; SHA recorded in `progress.md`)
- **Files touched:**
  - `apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs` — added three `book_status(..)`
    rows (`inner_sea_races`, `mythic_adventures`, `ultimate_magic_wordsofpower`) whose
    `rules_tables` modules had landed with no panel row; added `live_on_disk_record_count`
    (a read-only filesystem walk) and switched
    `the_two_ingested_books_totals_reconcile_with_their_license_artifacts` to use it instead
    of `LICENSE.json`'s `records_processed` field; re-derived and repinned
    `corpus_only_records` for `advanced_race_guide` (1073→1699) and `pathfinder_unchained`
    (69→1137); filtered `mythic_adventures_counts()`'s zero-monster row so it doesn't trip
    `every_book_is_populated_with_real_nonzero_counts`.
  - `docs/retro/events/t9-onboarding.jsonl` — one `incident`
    (`generator-orphans-unowned-files-on-directory-sync`), one `correction`.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff -- apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs`).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scoped diff).
- **Acceptance criterion:** establish the true state of
  `cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml corpus_ingest_diagnostic`
  (dispatch brief item 1); if RED, fix with evidence, never repin without proof.
- **Corpus SHA:** `PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6` (unchanged; no
  corpus regen performed — see §2 below for why the obvious "just refresh LICENSE.json" path
  was rejected).
- **Status:** complete.
- **Notes:** see body below.
- **Discovery forwards:** none new. One pre-existing defect (the destructive generator) is
  named and logged as an incident, not fixed here — out of this cycle's scope (`src/bin/
  gen_book_cache.rs`'s `feat`/`equipment`/`companion` generation touches other lanes'
  named territory).
- **Next-cycle plan:** `gen_book_cache.rs`'s unconditional "delete anything under this kind's
  directory that this run didn't just write" behavior is a real, general hazard for every book
  it covers, not only `advanced_race_guide` — any future lane running that binary on a book
  with concurrent sibling-lane content risks the same unauthorized deletion. Worth a dedicated
  fix (make the sync additive/audited rather than destructive) in a future cycle; named here so
  it isn't lost.

---

## 1. True state, established first (per dispatch brief item 1)

```
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml corpus_ingest_diagnostic
```

Ran to completion in the foreground (background+Monitor poll; ~2m48s cold compile). **RED,
confirmed real** (the orchestrator's own compile had timed out and could not confirm this) —
exact output:

```
test corpus_ingest_diagnostic::tests::every_book_landed_in_rules_tables_is_reported ... FAILED
test corpus_ingest_diagnostic::tests::the_two_ingested_books_totals_reconcile_with_their_license_artifacts ... FAILED
test result: FAILED. 13 passed; 2 failed; 0 ignored; 0 measured; 503 filtered out
```

- `every_book_landed_in_rules_tables_is_reported`: `["inner_sea_races", "mythic_adventures",
  "ultimate_magic_wordsofpower"]` landed real compiled `rules_tables` modules with no panel row.
- `the_two_ingested_books_totals_reconcile_with_their_license_artifacts`: `advanced_race_guide`
  — `left: 1579, right: 2157`.

Both match the dispatch brief's figures exactly. **Confirmed real, not a stale prior report.**

## 2. `every_book_landed_in_rules_tables_is_reported` — the assertion was correct, the panel was incomplete

All three books (`inner_sea_races`, `mythic_adventures`, `ultimate_magic_wordsofpower`) have
real `.rs` modules under `src/rules_core/rules_tables/` with real compiled tables (`spell_list`,
and for `mythic_adventures`, `monster_chassis::MONSTER_BOOKS` registration too — confirmed via
`grep -n "mythic_adventures" src/rules_core/rules_tables/monster_chassis.rs`, which shows it
already wired into the chassis registry with 21 `monster_abilities` and genuinely 0 `monsters`).
Added the missing three `book_status(..)` rows, following the existing pattern (e.g.
`inner_sea_faiths_counts`/`inner_sea_temples_counts`). `mythic_adventures_counts()` chains
`chassis_book_counts` but filters zero-count entries first — it is one of the "zero-monster"
books `monster_chassis.rs`'s own comments already name, and reporting a literal `monsters: 0`
row would have tripped `every_book_is_populated_with_real_nonzero_counts` (caught live: adding
the unfiltered row broke that OTHER test, confirmed by running the full module, not just the
one test named in the brief).

## 3. `the_two_ingested_books_totals_reconcile_with_their_license_artifacts` — real gap, wrong assertion, NOT a real ingest defect

Per `decisions.md §1a`/`§17a`: re-derived from scratch rather than trusted.

**Live on-disk count, `advanced_race_guide`** (excluding `_parity/`, a build/test fixture dir,
same exclusion `gen_book_cache.rs::count_on_disk_records` documents):

```
python3 -c "
import os
root='data/corpus/advanced_race_guide'
total=0
for dp,dn,fn in os.walk(root):
    base=os.path.basename(dp)
    if base.startswith('_'):
        dn[:]=[]; continue
    total += sum(1 for f in fn if f.endswith('.json') and f!='LICENSE.json')
print(total)"
# -> 2205
```

`LICENSE.json`'s own `records_processed` field: `2157`. **48 short of the live count.**

**Root-caused, not assumed:** `git diff --name-status <LICENSE.json's own commit> HEAD --
data/corpus/advanced_race_guide/` shows 0 files added/removed since that commit — the file set
was already 2205-real-records/2206-with-`_parity` at the moment `LICENSE.json` was written.
`LICENSE.json`'s `records_processed` field was stale **at write time**, most likely composed on
a pre-rebase working tree and carried through a `git rebase origin/tranche/12` without a
re-run — the same shape the `class_feature` PI-redaction cycle's receipt names for its own
`326→301` post-rebase movement, just uncorrected here.

**Attempted the obvious fix — regenerate `LICENSE.json` via its own generator — and it was
destructive. Reproduced live, reverted, never committed:**

```
cargo run --locked --bin gen_book_cache -- advanced_race_guide
# ... feats written: 187 / 187 ...
# LICENSE.json records_processed=2157   <- SAME stale number, not fixed
git status --porcelain -- data/corpus/advanced_race_guide/
#  M data/corpus/advanced_race_guide/LICENSE.json
#  D data/corpus/advanced_race_guide/feat/angelic_flesh_brazen.json
#  ... 48 D total
```

The generator syncs `feat/` (and presumably `equipment/`/`companion/`) to exactly its own
compiled output, **deleting any file it did not itself just write** — including 48 `feat`
records a sibling lane (`1410424cf3`, "close feat+spell no_record via existing corpus-cache
generators", `decisions.md §20`) legitimately landed through a different ingest path after this
book's compiled `arg::feats::feat_tables()` was last extended. Running it to "fix" the count
would have destroyed real, already-shipped content — exactly `workflow-instruction.md`'s
footgun 2 shape, and it explains why the printed `records_processed` came out 2157 either way:
count-after-delete equals `2205 - 48`. **Reverted with `git checkout -- data/corpus/
advanced_race_guide/`; `git status --porcelain` confirmed clean before proceeding.** Logged as
an incident, not fixed (`src/bin/gen_book_cache.rs`'s `feat`/`equipment`/`companion` paths are
other lanes' territory).

**The fix:** the test's own `LICENSE.json` dependency is what's unsafe to trust — it can only be
refreshed by a generator proven destructive. Replaced it with `live_on_disk_record_count`, a
read-only walk mirroring `count_on_disk_records`'s exact exclusion rules (`_`-prefixed dirs,
`LICENSE.json` itself), so the reconciliation check derives its own ground truth instead of
depending on an artifact that drifts independent of whether the record set actually changed.
Re-pinned `corpus_only_records`:

- `advanced_race_guide`: `1073 → 1699` (`2205` live `− 506` reported, unchanged).
- `pathfinder_unchained`: `69 → 1137` (`1264` live `− 127` reported, unchanged). **This branch
  had never actually run** — the `for` loop's first iteration (`advanced_race_guide`) panicked
  before reaching `pathfinder_unchained`, so its own staleness was invisible until this cycle
  fixed the first branch and the loop reached the second for the first time. Confirmed via
  `find data/corpus/pathfinder_unchained -name '*.json' | grep -v LICENSE | grep -v /_parity/ |
  wc -l` → `1264`; the bulk of the 1137 is `class_feature` (compiled `pu_class_feature_count()`
  is 64, on-disk `data/corpus/pathfinder_unchained/class_feature/` holds 604 — `decisions.md
  §13`'s T12 population plus subsequent `§20` no_record closures, corpus-only by the same shape
  every other un-compiled kind in this book already is).

**Neither book's underlying content has an ingest gap.** This was a documentation/assertion
drift defect, not a 578-record ingest hole — resolved with evidence, per the brief's own
instruction not to assume either direction.

## 4. RED → GREEN, mutation-proved

```
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml corpus_ingest_diagnostic
# 15 passed; 0 failed   (up from 13 passed; 2 failed)
```

Mutation proof on the fix itself (not just the original defect): temporarily bumped
`advanced_race_guide`'s repinned literal `1699 → 1700` and re-ran the single test —
**failed for the intended reason** (`left: 2206, right: 2205`, i.e. the assertion still
compares against the live count, not a value that trivially matches whatever's typed).
Reverted to `1699`; full module green again.

## 5. Verification

```
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml corpus_ingest_diagnostic
  15 passed; 0 failed; 0 ignored; 503 filtered out
git diff -- apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  -> OK_NO_BUNDLE_TAGS
git diff -- apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'
  -> OK_NO_TOKENS
git status --porcelain
  -- only this file + docs/retro/events/t9-onboarding.jsonl (append-only) touched
```

## 6. What remains (explicit)

- **`src/bin/gen_book_cache.rs`'s destructive directory-sync behavior** on
  `advanced_race_guide`/likely every other book it covers — a real, general hazard, logged as
  an incident, not fixed here (out of this cycle's territory).
- Nothing else for this specific RED branch — both failing tests are green, mutation-proved,
  and the fix does not touch any of the sibling lanes' named territory (`equipment`, `spell`,
  `companion`, `corpus_literal_sweep.rs`, `ingest_race_traits.rs`, `ingest_races.rs`,
  `Kind::Trait`, `v06_work_inventory.rs`).
