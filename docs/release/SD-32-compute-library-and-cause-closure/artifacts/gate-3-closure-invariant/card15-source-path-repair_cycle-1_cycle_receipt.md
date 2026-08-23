# Cycle card15-source-path-repair — Gate 3 closure invariant / unblock `corpus_literal_sweep` corpus-wide

- **Card ID:** kanban.md card 15 (census-scope-closure); unblocks the six concurrently-running T9
  ingest lanes' guarded regen path.
- **Commit SHA:** (recorded after push)
- **Files touched:**
  - `scripts/ingest_simple_filename_kinds.py` (fix — new `compose_source_path` helper replaces the
    buggy inline `os.path.relpath(file_path, os.path.join(args.pcgen_root, "pathfinder"))`)
  - `scripts/tests/test_ingest_simple_filename_kinds.py` (new — `ComposeSourcePathTests`, 3 tests)
  - `data/corpus/**/{template,power,domain,language,skill}/*.json` (3,124 files — `source.path` and
    `ingested_at` only, regenerated through `scripts/ingest_simple_filename_kinds.py`, never
    hand-edited; verified by sampled `git diff`)
  - `docs/work-inventory.json` (regenerated via `cargo run --bin v06_work_inventory` with
    `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` set, no `--allow-stamp-loss`)
  - `docs/retro/events/t9-onboarding.jsonl` (append — auto-appended `verify.sh` preflight-oracle
    events)
  - `docs/release/SD-32-compute-library-and-cause-closure/progress.md` (append)
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (row 15 entry prepended, no
    status field changed — stays `in-progress` per dispatch instruction)

## The defect

`scripts/ingest_simple_filename_kinds.py` composed `source.path` as:
```python
"path": os.path.relpath(file_path, os.path.join(args.pcgen_root, "pathfinder")),
```
`args.pcgen_root` is already the PCGen data root (`$PCGEN_CORPUS_ROOT`, e.g. `.../pcgen/data`) —
the same value `scripts/ingest_ability.py`'s `corpus_root()` returns, which that script's own
`os.path.relpath(path, root)` (no `"pathfinder"` join) uses correctly. Joining `"pathfinder"` a
second time before taking the relpath strips the leading system segment every other corpus record
carries, producing e.g. `paizo/roleplaying_game/advanced_class_guide/acg_domains.lst` instead of
`pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_domains.lst`.

**Convention established from the 38,234 correctly-shaped records** (not one example): every other
corpus-record-writing script (`ingest_ability.py`) relpaths against the bare data root. Confirmed by
re-deriving the good/bad split myself:
```python
# walk data/corpus/**/*.json, count source.path startswith "pathfinder/"
good = 38234   bad = 3124
```
Matches the orchestrator's count of 3,124 exactly (re-derived, not trusted — §17a); the discovering
lane's figure of 2,585 was stale, not a sign of a second culprit.

**Checked every other script for the same defect.** All scripts that write a corpus record's
`source.path` field: `ingest_ability.py` (correct, no `"pathfinder"` join),
`derive_monster_sla_spell_level_fixtures.py` and `transcribe_monster_tables.py` (read `source.path`,
never write it). `census_independent.py` / `ground_truth_evidence_guard.py` /
`card15_reconcile.py` all also compose a `pathfinder_root = os.path.join(root, "pathfinder")`, but
strictly for their own internal book-relative walk (`rel_path` used for book discovery, never
written into a corpus record's `source` field) — not the same defect, verified by reading each
call site, not by name-matching the pattern.

## Fix

New helper, replacing the inline composition:
```python
def compose_source_path(file_path: str, pcgen_root: str) -> str:
    rel = os.path.relpath(file_path, pcgen_root).replace(os.sep, "/")
    segments = [s for s in rel.split("/") if s]
    is_shaped = len(segments) >= 5 or (len(segments) == 4 and segments[1] == "dreamscarred_press")
    if not is_shaped:
        raise ValueError(...)
    return rel
```
Mirrors `src/bin/corpus_literal_sweep.rs`'s own `book_dir_of` shape check (≥5 segments, or 4 for the
`dreamscarred_press` publisher which has no `<line>` segment) — the same check that caught this
defect downstream — so a malformed path now fails **at the producer**, before a single record ships.

## RED → GREEN (mechanical control, `§1a`)

`scripts/tests/test_ingest_simple_filename_kinds.py::ComposeSourcePathTests` (3 new tests):
- `test_compose_source_path_keeps_leading_system_segment` — correct-shape happy path.
- `test_compose_source_path_rejects_a_pcgen_root_pre_joined_with_pathfinder` — reproduces the exact
  original bug (passes `os.path.join(pcgen_root, "pathfinder")` as the base) and asserts `ValueError`.
- `test_compose_source_path_accepts_the_dreamscarred_press_shape` — the 4-segment publisher exception.

**RED proved live:** temporarily set `is_shaped = True` (simulating "no guard exists"), ran
`python3 -m unittest scripts.tests.test_ingest_simple_filename_kinds.ComposeSourcePathTests -v`:
```
test_compose_source_path_rejects_a_pcgen_root_pre_joined_with_pathfinder ... FAIL
AssertionError: ValueError not raised
```
Failed for the intended reason (the guard, not some other error). Reverted; re-ran full suite:
`Ran 13 tests in 0.008s — OK` (10 pre-existing + 3 new).

## Repair — 3,124 records, through the guarded generator only

```
python3 scripts/ingest_simple_filename_kinds.py --inventory docs/work-inventory.json \
  --pcgen-root "$PCGEN_CORPUS_ROOT" --out-root data/corpus \
  --kind template --kind power --kind domain --kind language --kind skill
```
Output: `"written_count": 3124` — exactly the re-derived bad-record count. The script unconditionally
rewrites every unit of the five target kinds (not just previously-bad ones), so this is the same
generator path the original 3,137 ingested records went through; the 13 citation-mismatch rows
(named in `card15-simple-filename-kinds-ingest_cycle-1_cycle_receipt.md`) are still correctly
skipped and not written (3,137 − 13 = 3,124, consistent).

**Never hand-edited:** sampled `git diff` on a repaired record
(`data/corpus/occult_adventures/template/medium.json`) shows only `ingested_at` and `source.path`
changed — no other field moved. `git status --porcelain` shows 3,124 `M` entries under
`data/corpus/**`, 0 `D` (deletions) — no sibling lane's record was touched or lost.

```python
# post-repair sweep of every data/corpus/**/*.json
bad = 0  # was 3,124
```

## Gate reopened — proved corpus-wide

```
cargo run --locked --bin corpus_literal_sweep -- --json-out /tmp/sweep-after.json
```
```
corpus-literal-sweep: 39378 records examined of 41371 read, 338506 tokens compared (9 synthesized), 41358 digests checked, 0 findings
corpus-literal-sweep: CLEAN
```
Exit 0 (was exit 2, the exact defect the six blocked lanes hit).

```
cargo run --locked --bin derived_evaluator_fixture_check -- --json-out /tmp/fixture-after.json
```
```
derived-evaluator-fixture-check: 1836 unit(s) cleared over 2577 fixture row(s); 0 failed; 0 not ingested
```

**Corpus SHA:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
(`scripts/pcgen-oracle-pin.env`), re-fetched fresh in this worktree via
`scripts/fetch-pcgen-oracle.sh` (a fresh worktree's oracle slot is git-ignored and starts empty).

## Regeneration discipline — verification provenance diffed both directions

Regenerated `docs/work-inventory.json` with both report env vars set, **no `--allow-stamp-loss`**:
```
CORPUS_LITERAL_SWEEP_REPORT=/tmp/sweep-after.json DERIVED_FIXTURE_CHECK_REPORT=/tmp/fixture-after.json \
  cargo run --locked --bin v06_work_inventory
```

| status | before | after | delta |
|---|---:|---:|---:|
| `literal-verified` | 6,506 | 6,506 | **0 — preserved exactly** |
| `fixture-verified` | 1,741 | 1,741 | **0 — preserved exactly** |
| `grounded` | 2,724 | 2,724 | 0 |
| `text-complete` | 4,395 | 4,395 | 0 |
| `ingested-magnitude` | 1,515 | 1,515 | 0 |
| `not-ingested` | 28,312 | 28,314 | +2 |
| `unknown` | 4,282 | 4,285 | +3 |
| `deferred-with-reason` | 46 | 46 | 0 |
| `not-started` | 19 | 19 | 0 |
| **TOTAL** | **49,540** | **49,545** | **+5** |

No `literal-verified`/`fixture-verified` stamps lost, either direction. The small `not-ingested`/
`unknown` movement (+5 net units total) is concurrent sibling lanes' already-landed corpus progress
folding into the same regen — not this cycle's own effect, proved by `git diff --stat` on this
cycle's own diff touching 0 `.rs` files (only the Python producer + its test file are Rust-adjacent
input, no Rust source changed).

## Shape ledger — `no_record` before/after, honest comparison

"Before" measured against the **true HEAD-committed corpus** (`git archive HEAD -- data/corpus` into
`/tmp/corpus-before`, not the already-repaired working tree, so this is not circular):
```
python3 scripts/shape_ledger.py --inventory /tmp/work-inventory-before.json \
  --corpus-root /tmp/corpus-before/data/corpus
# no_record: 8,434
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json
# no_record: 8,439
```
**8,434 → 8,439 (+5).** The path defect did **not** cause failed joins:
`scripts/shape_ledger.py`'s join key is `(book, source_basename, source_line)`, never `source.path`
— confirmed by the "before" run against the broken-path corpus already returning the same population
shape (`race_trait` 1,883, `template` 1,062, `deity` 459, …) as the "before" run against a
freshly-repaired corpus tree measured earlier in this cycle (identical counts both ways). So this fix
legitimately reduces nothing on its own; the +5 delta is the same concurrent-sibling-progress fold-in
the status-distribution table above shows (`race_trait` −24, `class_feature` +29, net +5) —
reported as measured, not assumed the fix would help, per the dispatch brief's own instruction to
check rather than guess the direction.

## Identifier / wired-integration audit (this cycle's own diff, scoped to touched files only)

```
git diff --unified=0 HEAD -- scripts/ingest_simple_filename_kinds.py scripts/tests/test_ingest_simple_filename_kinds.py \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
git diff --unified=0 HEAD -- scripts/ingest_simple_filename_kinds.py scripts/tests/test_ingest_simple_filename_kinds.py \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
```
Both `OK_*`.

## Tests

- `python3 -m unittest scripts.tests.test_ingest_simple_filename_kinds -v` → **13/13 OK**
  (10 pre-existing + 3 new).
- `cargo run --locked --bin corpus_literal_sweep` → CLEAN, exit 0.
- `cargo run --locked --bin derived_evaluator_fixture_check` → 0 failed, exit 0.
- `python3 scripts/shape_ledger.py` → runs clean both before and after.

**Full unscoped `cargo test` NOT run** (dispatch instruction: may never finish on this box). This
cycle's Rust-adjacent surface is unchanged (0 `.rs` files in the diff); `v06_work_inventory` was run
as a regen, not as its own test suite, but its own committed test suite is untouched by this cycle's
diff.

- **Status:** complete
- **Notes:** the `source.path` defect never affected `shape_ledger.py`'s join — a real finding,
  measured rather than assumed, reported honestly per the dispatch brief's own instruction ("that
  would be a real finding, so measure it").
- **Discovery forwards:** none — this cycle closes exactly the escalated defect from
  `artifacts/gate-0-census-closure/15-duplicate-identity-review_cycle_receipt.md` "Next-cycle plan"
  items 5 and 6.
- **Next-cycle plan:** the prior cycle's rescued 4 `duplicate_identity` units (proven correct in code,
  not yet in the checked-in `docs/work-inventory.json` as of the base this cycle started from) should
  land automatically on the next guarded regen of a lane that picks up `duplicate_identity` work,
  now that the `source.path` blocker is cleared. `deity` (459 units) remains escalated per
  `decisions.md §15`, unaffected by this cycle.

## Disk

`df -h /`: reported at end of turn.
