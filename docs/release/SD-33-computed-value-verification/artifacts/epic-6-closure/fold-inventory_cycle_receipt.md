# Cycle fold-inventory — SD-33 reopen fold / Epic 6 closure

- **Commit SHA:** (recorded after commit — see `commit_shas` in this lane's returned JSON)
- **Files touched:** `docs/work-inventory.json` (regenerated, sole-writer scope), `THE-BOX.md`
  (append-only — group counts corrected + new "Epic 6 note" section), this receipt.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS, with a disclosed shape identical to
  `fold-skinwalker`'s own audit note. `git diff --unified=0 56bbebe3d4...HEAD -- docs/work-inventory.json docs/release/SD-33-computed-value-verification/{THE-BOX.md,kanban.md,progress.md} docs/release/SD-33-computed-value-verification/artifacts/epic-6-closure/fold-inventory_cycle_receipt.md | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  matches only `tests/sd27_alternate_racial_trait_reachability.rs` and
  `tests/sd27_aasimar_globalvar_gate_closes_the_dead_affordance.rs` in `progress.md`'s
  `fold-skinwalker` cycle summary — two **real, pre-existing filenames** this cycle's prose names
  (they exist on disk, confirmed via `ls`), not a fresh bundle-tag identifier this cycle invented.
  `docs/work-inventory.json` is a generated JSON artifact with no free-text prose; `THE-BOX.md`'s
  own prose references bundle names like "SD-33"/"AT-33-E4" without a trailing underscore, which
  the pattern does not match by design.
- **Wired-integration audit result:** OK_NO_TOKENS (same diff, `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → no matches).
- **Acceptance criterion:** Regenerate `docs/work-inventory.json` and re-green every gate after
  the `fold-skinwalker`/`fold-undine` lanes landed their recovered SD-31 work into SD-33
  (post-closure reopen ruling, `workflow-instruction.md`'s "WHY THIS EXISTS" header).

## STEP 1 — verified what actually landed (before touching anything)

Local `tranche/13` was 3 commits behind `origin/tranche/13` at dispatch time (the two fold lanes
pushed directly). `git fetch origin tranche/13 && git merge --ff-only origin/tranche/13` (local
HEAD was a pure ancestor — a genuine fast-forward, confirmed via
`git log --oneline origin/tranche/13..HEAD` → 0 lines before merging) brought in:

- `948976aacb` — feat(sd33): fold-undine
- `6e2f2f076b` — fix(sd33): fold SD31-E6-F4-005's 45 recovered Skinwalker heritage records (65 real)
- `56bbebe3d4` — docs(sd33): record fold-skinwalker's own commit SHA in its receipt

**Skinwalker, re-counted, not trusted:** `find data/corpus/bestiary_5/race_trait/skinwalker -name '*.json' | wc -l` → **75** (10 pre-existing + 65 new). Matches the lane's own report
(`skinwalker_records: 65`) exactly. **No discrepancy.**

**Undine, re-counted:** `python3 -c "import json; d=json.load(open('tests/fixtures/rules_core/derived-evaluator-fixtures.json')); print(len(d['race_trait_formula_entries']))"` → **3**. The
operator ruling's own framing ("103 recovered Undine race-trait fixture entries") is a
**pre-existing correction**, disclosed by the `fold-undine` lane's own receipt before this cycle
ever ran, not a discrepancy this cycle discovered: "103" was a raw string-occurrence count of the
word "Undine" across the whole fixture JSON (~34 mentions × 3 records), not an entries count. The
real, committed population is 3 `race_trait_formula_entries` records (Acid Breath, Nereid
Fascination, Ooze Breath) / 30 sample points / 90 scalar (TL,CON,CHA)→value checks. `fold-undine`
deliberately did **not** add `"undine"` to `race_ids_with_a_magnitude_consumer()` (0 board-credit
units banked, per `OPEN-ISSUES.md` row 365's remediation path (a)) — confirmed this held:
`cargo test --locked --lib race_ids_with_a_magnitude_consumer` still asserts the 18-race set,
unchanged.

Both lanes' `corpus_literal_sweep`/lib/desktop suite claims were **independently re-run**, not
merely trusted — see Gates below; all reproduce exactly (`corpus-literal-sweep: 48699 records
examined ... 0 findings`; lib `2845 passed, 0 failed, 14 ignored`; desktop `548 passed, 0 failed`).

## STEP 2 — `docs/work-inventory.json` regenerated

Regenerated via its own binary only (`cargo run --locked --bin v06_work_inventory`, never by
hand), against the pinned PCGen oracle at the SD-33-required repo-local slot
(`docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data`
— confirmed byte-identical to `~/workspace/repos/pcgen/data` via `diff -rq` before switching, both
pinned at `7f818006e371188e5717fd18d74d18a420747fc6`), with `CORPUS_LITERAL_SWEEP_REPORT`/
`DERIVED_FIXTURE_CHECK_REPORT` pointed at scratch reports mechanically re-derived from the
pre-cycle committed inventory's own already-stamped `literal-verified`/`fixture-verified`
populations (6,589 + 1,741 triples/ids respectively) — same mechanism `AT-33-E4-002` used, never
`--allow-stamp-loss`.

**First attempt refused to write** (`this run would drop 8330 of 8330 verification stamp(s)`) — a
genuine self-heal, not a near-miss suppressed: the scratch JSON reports were written with Python
`json.dump`'s default separators (space after `:`/`,`), and the loader's stamp-preservation code
does a literal substring search for `"clean":true`/`"verified":[` with no space, so it silently
read 0 verified entries from a well-formed-but-differently-spaced file. Logged:
`docs/retro/events/sd33-fold-inventory.jsonl` (rework, cause + avoidable-by). Fixed
(`separators=(',', ':')`) and re-run to completion.

**Figures, before → after, each with its re-derive command:**

| Figure | Before | After | Command |
|---|---:|---:|---|
| Total units | 49,438 | **49,438 (unchanged)** | `jq '.units\|length' docs/work-inventory.json` |
| `not-ingested` | 26,047 | **26,002** | `jq '[.units[]\|select(.status=="not-ingested")]\|length' docs/work-inventory.json` |
| `grounded` | 3,340 | **3,415** | `jq '[.units[]\|select(.status=="grounded")]\|length' docs/work-inventory.json` |
| `ingested-magnitude` | 2,497 | **2,455** | `jq '[.units[]\|select(.status=="ingested-magnitude")]\|length' docs/work-inventory.json` |
| `text-complete` | 8,838 | **8,850** | `jq '[.units[]\|select(.status=="text-complete")]\|length' docs/work-inventory.json` |
| `literal-verified` | 6,589 | **6,589 (unchanged)** | `jq '[.units[]\|select(.status=="literal-verified")]\|length' docs/work-inventory.json` |
| `fixture-verified` | 1,741 | **1,741 (unchanged)** | `jq '[.units[]\|select(.status=="fixture-verified")]\|length' docs/work-inventory.json` |
| Skinwalker `race_trait` units (id contains "skinwalker") | 50 (41 not-ingested, 5 ingested-magnitude, 3 text-complete, 1 grounded) | **50 (unchanged count; 32 not-ingested, 0 ingested-magnitude, 8 text-complete, 10 grounded)** | `python3 -c "import json,collections; u=[x for x in json.load(open('docs/work-inventory.json'))['units'] if x['kind']=='race_trait' and 'skinwalker' in x['id'].lower()]; print(len(u)); print(collections.Counter(x['status'] for x in u))"` |
| Undine `race_trait` units (id contains "undine") | 21 (14 ingested-magnitude, 5 text-complete, 1 not-ingested, 1 grounded) | **21 (unchanged, byte-identical status distribution — 0 movement)** | same query with `'undine'` |

**Why the population itself never moves.** `docs/work-inventory.json`'s population is a fixed
census over the pinned PCGen oracle's own `.lst` rows (`generated_by`/`corpus_root` in the file's
own header), not a walk of `data/corpus/`. Adding 65 new corpus JSON records changes which units
are `ingested`/computed — never how many units the census names. This is why "Skinwalker units"
is 50 both before and after: those 50 rows already existed in the census (mostly as
`not-ingested`), and the fold gave 41 of them (well, a subset of them — see the id-keyed join
below) a real corpus record and, for some, a real engine consumer for the first time.

**89 units moved status, by an `id`-keyed join against the pre-cycle committed file** (git blob
`00ca087775:docs/work-inventory.json`, `AT-33-E4-002`'s own regen — confirmed byte-identical to
the file this cycle started from via `diff`), not an aggregate delta — same method and same
reason `AT-33-E4-002` used (an aggregate delta cannot distinguish this cycle's own movement from
unrelated drift):

```
python3 -c "
import json
b={u['id']:u['status'] for u in json.load(open('<00ca087775 blob>'))['units']}
a={u['id']:u['status'] for u in json.load(open('docs/work-inventory.json'))['units']}
changed=[(i,b[i],a[i]) for i in b if b[i]!=a[i]]
fold=[c for c in changed if 'skinwalker' in c[0].lower() or 'undine' in c[0].lower()]
print(len(changed), len(fold), len(changed)-len(fold))
"
```
→ `89 14 75`.

- **14 fold-attributable** (all 14 carry `skinwalker` in `id`; **0** carry `undine` — exactly as
  `fold-undine`'s own receipt states it deliberately banks 0 board-credit units): transitions
  `ingested-magnitude→grounded` 5 (the two `ALTERNATE_TRAIT_SELECTED_SKILL_BONUSES` wiring-fix
  rows, `Werebear-Kin`/`Wereshark-Kin ~ Animal-Minded`, land here), `not-ingested→grounded` 4,
  `not-ingested→text-complete` 5.
- **75 unrelated drift** — the exact phenomenon `AT-33-E4-002`'s receipt named: the file was not
  regenerated between `00ca087775` (2026-08-25, `AT-33-E4-002`) and this cycle, and
  `git log --oneline 00ca087775..56bbebe3d4 -- src/rules_core/ src/bin/ data/corpus/` shows **25**
  commits in that window (`AT-33-E5`'s disagreement-resolution/equipment-engine fixes,
  `AT-33-E6-001`'s corpus-sweep fix, and this cycle's own 3 fold commits) — a regen was always
  going to surface this, independent of the fold. Transitions:
  `ingested-magnitude→grounded` 37, `not-ingested→grounded` 27, `not-ingested→text-complete` 9,
  `text-complete→grounded` 2. **0 of the 75 carry `skinwalker`/`undine` in `id`.**

## STEP 3 — count sweep

**The population never moved (49,438 both before and after), so the one hard test assertion
`scripts/tests/test_box_ledger.py:395` (`assertIn("uncovered=0 overlap=0 population=49438", ...)`)
needed no fix** — confirmed still true by execution
(`python3 -m unittest scripts.tests.test_box_ledger -v` → **25 passed, 0 failed**, including
`test_check_against_live_committed_files` against the just-regenerated live file).

Grep of OLD (49438/26047/831/6278) and NEW figures across `tests/`, `src/`, `apps/`, `scripts/`
(`grep -rn "49438\|49,438\|26047\|26,047" tests/ src/ apps/ scripts/`) found **zero live
assertions** beyond the one above — every other hit is prose/doc-comments narrating historical
movement (already correctly updated in-place by the `fold-skinwalker` cycle's own count sweep,
e.g. `831 → 910`, `6,278 → 6,260`), or lives under `docs/release/**`/`docs/retro/**` (package
prose, historically accurate as written, not a live assertion). No stale count found to fix.

## STEP 4 — every gate, re-run live, this cycle

| Gate | Command | Result |
|---|---|---|
| box_ledger (no oracle) | `python3 scripts/box_ledger.py --check` | `uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False`, exit 0. 4 WARNING lines (THE-BOX.md's stated group `count`s were stale before this cycle's own THE-BOX.md edit — non-gating, reported not failed) |
| box_ledger (with Epic 5 oracle) | `python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json` | same, `oracle_disagreement=0`, exit 0 |
| denominator-gate | `scripts/verify.sh --only denominator-gate` | PASS, `files_checked=65 violations=0` |
| corpus-sweep (verify.sh) | `scripts/verify.sh --only corpus-sweep` | PASS, `48699 records examined of 51473 read, 413288 tokens compared (9 synthesized), 51460 digests checked, 0 findings` (baseline note only, not a failure: `BASELINE_CORPUS_LITERAL_RECORDS` floor is stale at 26,500 vs the true 48,699 — population only grew, so the floor check passes; left unbumped, out of this cycle's write scope) |
| corpus_literal_sweep (raw binary) | `cargo run --locked --bin corpus_literal_sweep` | `CLEAN`, identical figures to the verify.sh row above |
| no-run | `cargo test --locked --no-run` | exit 0, 599 executables built, 0 compile errors |
| lib suite | `cargo test --locked --lib` | **2845 passed, 0 failed, 14 ignored** (60.77s). Includes `rules_core::equipment_resolver::tests::catalog_rows_span_every_ingested_book_with_their_real_counts` passing — this was `fold-undine`'s own receipt's noted "pre-existing, unrelated" red; it is GREEN on this cycle's HEAD, confirming that failure was transient/already-superseded, not a regression this cycle needed to fix |
| desktop suite | `cd apps/desktop/src-tauri && cargo test --locked` | **548 passed, 0 failed** (93.28s) |

All eight commands green. `CARGO_TARGET_DIR=/tmp/cargo-sd33-sd33-fold-inventory` for the root
workspace, a separate `/tmp/cargo-sd33-sd33-fold-inventory-desktop` for the desktop crate (its own
cargo workspace, per `AGENTS.md`'s "Root sweep misses the desktop crate" rule).

## Oracle unexamined-set — the check this cycle exists to protect

Per the dispatch's own script, run against the just-regenerated file:

```
python3 -c "import json
wi=json.load(open('docs/work-inventory.json'))['units']
pop={u['id'] for u in wi if u.get('status') in ('literal-verified','fixture-verified')}
d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json'))['results']
miss=sorted(pop-{r['unit_id'] for r in d}); print(len(miss))"
```
→ **0**.

This is not a coincidence, it is a structural consequence confirmed above: `literal-verified`
(6,589) and `fixture-verified` (1,741) are byte-identical before and after this fold — none of
the 14 fold-attributable or 75 drift transitions touch either status. Epic 5's own population
(defined exactly as `{id : status in (literal-verified, fixture-verified)}`, 8,330 total) is
neither enlarged nor shrunk by this fold, so `AT-33-E5-003.combined-oracle-results.json`'s
`oracle_disagreement=0` over that population still covers it in full. **No new units were run
through the oracle harness this cycle — none needed to be.**

## Movement, four buckets (this cycle's own 89, by transition pair)

- **Closure 14** — reached `text-complete` ("done") for the first time: `not-ingested→text-complete`
  5 (fold) + 9 (drift).
- **Reachability 73** — reached `grounded` ("engine-grounded, oracle-pending") for the first time,
  a genuinely new compute path, not yet "done": `not-ingested→grounded` 4 (fold) + 27 (drift) = 31;
  `ingested-magnitude→grounded` 5 (fold) + 37 (drift) = 42; 31+42=73.
- **Reclassification 2** — `text-complete→grounded` (drift only): the instrument now sees a real
  magnitude token these 2 records always carried but a prior parsing gap hid (plausibly
  `AT-33-E5-003-corpus-extraction-fix`'s `.MOD`-fold gap fix landing in the same drift window) —
  evidence already present, newly described correctly.
- **Instrument-correction 0** — no group renamed, no unit's evidence changed meaning under an
  unchanged disposition.

14+73+2=89, exact, zero unaccounted.

## Status: complete

## Notes

1. **`docs/work-inventory.json`'s population is a fixed census, not a corpus walk** — this is the
   single fact that makes this cycle's whole shape different from a naive "N new records => N new
   units" expectation, and is why the operator's dispatch's own placeholder `units_after`/
   `not_ingested_after` figures needed live derivation rather than arithmetic from the fold
   lanes' record counts.
2. **`fold-undine`'s "banks 0 board-credit units" choice is now independently confirmed at the
   inventory layer**, not just by the lane's own receipt: 0 of 21 Undine `race_trait` units moved
   status. This was a deliberate, correct choice (avoids `OPEN-ISSUES.md` row 365's gaming
   vector), not a gap this cycle should "fix" by widening `race_ids_with_a_magnitude_consumer`.
3. **THE-BOX.md edits are corrections to existing group `count` fields plus one append-only
   section** — no group was added, removed, or renamed; the append-only constraint
   (`workflow-instruction.md §3`, `THE-BOX.md`'s own "Next-cycle plan" section) is satisfied.
4. **`BASELINE_CORPUS_LITERAL_RECORDS` in `scripts/verify-baselines.env`** is stale (26,500 vs the
   true 48,699, unbumped since well before this fold) — flagged, not fixed: it is a floor, the
   check passes either way, and deliberately bumping it is `scripts/verify-baselines.env`'s own
   documented ceremony (a receipted note, not a silent edit), out of this narrowly-scoped cycle.
5. **kanban.md/progress.md** are updated separately in the same push (three new rows: this cycle's
   two sibling receipts, already committed and unrepresented on the board, plus this cycle) —
   see those files' own diffs for the pointer format.

## Next-cycle plan

- SD-33's final-acceptance scan (`AT-33-E6-001`) and `release-notes.md` were both derived against
  the pre-fold state (commit `075c4543c9`) — per the reopen ruling's own "WHY THIS EXISTS", both
  must be re-derived before PR #377 merges. Neither is this cycle's scope (this cycle's mandate was
  narrowly the inventory regen + gate re-green); flagging for the next cycle in the reopen
  sequence.
- `BASELINE_CORPUS_LITERAL_RECORDS` bump (26,500 → 48,699) is real, low-risk maintenance available
  for a future cycle; not gating.
