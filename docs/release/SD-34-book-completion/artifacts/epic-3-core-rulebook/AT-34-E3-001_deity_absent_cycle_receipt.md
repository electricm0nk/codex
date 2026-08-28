# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`deity_content_absent_from_deity_table_in_core_rulebook` mechanism)

- **Commit SHA:** `9934711054e0eabda678065c4b9b5eb8d006aa0d`
  (parent `5f0a905fb001287bcd5045827cfe0156aa403688`)
- **Files touched:**
  - `src/bin/v06_work_inventory.rs` — `Kind::Deity`'s `classify()` arm gains
    the same PI-safe coordinate fallback `Kind::Domain` already carries
    (`decisions.md §14`): `resolve_by_coordinate("{engine_book}:{source_file}:
    {source_line}")` is tried after the ordinary key/name `resolve` fails,
    never reading or reconstructing the redacted real deity name. Two new
    tests: `a_pi_renamed_deity_record_resolves_by_coordinate_and_leaves_
    bucket_b` (RED→GREEN) and its monotonicity sibling
    `a_deity_record_absent_from_the_table_and_with_no_matching_coordinate_
    stays_bucket_b`.
  - `scripts/completion_atlas.py` — one `BUCKET_DEFINITIONS` `file:line`
    citation (bucket V, `literal-verified`) re-derived: this cycle's own
    line-insertions shifted it `10480 -> 10495`.
  - `scripts/missing_engine_tables.py` — one `ENGINE_SURFACE_CITATIONS`
    citation (`power`) re-derived: `9908 -> 9923`, shifted by this cycle's
    own `Kind::Deity` arm growing by 15 lines. Caught by re-running
    `--check`, not assumed.
  - `docs/work-inventory.json` (regenerated at HEAD, guarded regeneration
    path — `cargo run --locked --release --bin v06_work_inventory`,
    `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` set from
    this session's own fresh `corpus_literal_sweep`/
    `derived_evaluator_fixture_check` runs, no `--allow-stamp-loss` used or
    needed. No corpus records added or regenerated — `data/corpus/**` was
    not touched this cycle, only `classify()`'s logic.)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    and `.../missing-engine-tables.json` (regenerated outputs of their
    scripts' own `--check`, not hand-edited).
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_deity_absent_cycle_receipt.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` on this cycle's own
  working-tree diff (`git diff -- <scoped paths> | grep -nE '\b(sd[0-9]+_|
  SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → no matches). The wider diff against
  `origin/develop`'s merge-base (spanning every prior AT-34-E3-001 cycle on
  this branch, not just this one) surfaces hundreds of pre-existing
  `display:sd32_class_ingest`/`display:sd32_simple_filename_kind_ingest`
  matches inside `docs/work-inventory.json` — historical `wiring_class_
  signals` **data values** written by earlier bundles, the exact same shape
  the `race_trait_absent` and `class_absent` cycles' own receipts already
  documented and self-healed. Not introduced by this cycle.

- **Wired-integration audit result:** `OK_NO_TOKENS` on this cycle's own
  working-tree diff (same command, `\b(STUB|MOCK|placeholder|not yet
  implemented|todo|fixme|hack)\b` → no matches). The wider epic-scoped diff
  against `origin/develop` carries pre-existing `placeholder` matches
  entirely inside `ingest_race_traits.rs` (PCGen's own literal `###Block:
  Placeholder objects...` comment, already reviewed and self-healed in the
  `race_trait_absent` cycle's own receipt) — not new, not this cycle's own
  code.

- **Acceptance criterion (verbatim, `epic-breakdown.md` AT-34-E3-001):**
  "**970** Core Rulebook units whose table exists but which are not in it.
  **Evidence:** the atlas reporting bucket B at zero for `core_rulebook`,
  and the mechanism that placed them named — **by mechanism, not per
  record.**" This cycle's own bar (`decisions.md §14`): drive
  `deity_content_absent_from_deity_table_in_core_rulebook` to zero.
  **AT-34-E3-001 as a whole does not close this cycle** — five of the nine
  named mechanisms remain (`domain`, `race_trait_absent_from_race_traits`,
  `class_absent_from_ClassId_ALL_and_book_class_id_enums`, and this cycle's
  own `deity_content_absent...` are now closed); this receipt reports only
  this cycle's own mechanism.

## Re-derived population, not carried forward

Re-derived at this cycle's start SHA (`5f0a905fb0`), **not** quoted from
`decisions.md §14`'s own table (21 — matched exactly, but independently
verified):

```
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
cr = [u for u in d['units'] if u['book']=='core_rulebook' and u['status']=='engine-does-not-hold']
tgt = [u for u in cr if u['evidence']=='deity_content_absent_from_deity_table_in_core_rulebook']
print(len(tgt), 'of', len(cr))
"
```
→ `21 of 1772` (before this cycle's regeneration; `1772` here is a plain
`status`-only count, wider than the atlas's own bucket-B partition — see
Row-count section below for the atlas-shaped `995 -> 974` figure this
receipt actually reports movement against. The mechanism's own numerator,
`21`, is identical either way since its evidence string always contains
`absent_from`).

Root cause (verified against the real corpus record, not assumed): all 21
`cr_deities.lst` deity rows carry `NAMEISPI:YES` and are PI-masked at
ingestion — `data/corpus/core_rulebook/deity/codex_named_unit_deity_
core_rulebook_cr_deities_lst_14.json`'s own `data.key`/`data.name` are
rewritten to `"Codex-Named Unit (deity_core_rulebook_cr_deities_lst_14)"`.
`SimpleKindTable::resolve` in `simple_kind_tables.rs` only matches on the
unit's real, un-masked `key`/`name` (e.g. `"Abadar"`), which the masked
record's own JSON never carries, so every one of the 21 records — which
physically exist — reported `engine-does-not-hold`.

## PI constraint honored (`decisions.md §14`)

The fix matches on the record's own stored coordinate
(`"{book}:{source_file}:{source_line}"`, e.g. `"core_rulebook:cr_deities.
lst:14"`), read straight off `unit.provenance`/`engine_book` — never the
redacted real deity name — and falls back to `SimpleKindTable::
resolve_by_coordinate`, which returns the SAME masked-key record
`resolve` would if it could find it. No code path, test name, receipt, or
commit message in this cycle names, logs, or reconstructs a real deity
name. `scripts/verify.sh --only site-public-status-pi-gate --only
site-dashboard-pi-gate` reported `PASS` (`31 file(s) scanned against 1612
declared-PI name(s), zero leaked` / `21 file(s) ... zero leaked`) at this
cycle's HEAD.

## Figures + their re-derive commands

- **21 of 1,006** — this mechanism's share of `core_rulebook` bucket B, per `decisions.md §14`'s enumeration: `python3 -c "..."` (the deity-evidence filter shown above, run against `docs/work-inventory.json`); independently re-derived, not transcribed.
- **21 units, 21 corpus JSON records** — one-to-one; verified:
  `find data/corpus/core_rulebook/deity -name '*.json' | wc -l` → `21`, and
  each carries a `rename.coordinate` of `core_rulebook:cr_deities.lst:<N>`
  for `N` in the same 21 source lines the 21 bucket-B units carry
  (`{7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,29}`) — checked
  by direct comparison, not assumed to match.
- **995 → 974** — `core_rulebook`'s real atlas-partitioned bucket B before/after this cycle (matching `progress.md`'s figure from the prior `class_absent` cycle): `python3 scripts/completion_atlas.py --book core_rulebook --check` → `B: 974` post-cycle (delta `-21`, exactly this mechanism's population).
- A plain `status == "engine-does-not-hold"` filter on `docs/work-inventory.json` over-counts (1,772 → 1,751, same `-21` delta but the wrong base) because it also catches bucket-D `*_pending_wiring_class_review` units the atlas's own `_B_MARKERS` substring check correctly excludes: `python3 -c "..."` (the plain-`status` filter shown above, re-run to demonstrate the over-count) — caught and corrected before this figure was written down, not carried forward from the wrong filter.
- **49,438** — corpus-wide unit population, unchanged by this cycle (no units added or removed, only reclassified): `python3 -c "import json; print(len(json.load(open('docs/work-inventory.json'))['units']))"` on the regenerated `docs/work-inventory.json` → `49438`.

## Row-count command output

```
$ python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
cr = [u for u in d['units'] if u['book']=='core_rulebook' and u['status']=='engine-does-not-hold']
tgt = [u for u in cr if u['evidence']=='deity_content_absent_from_deity_table_in_core_rulebook']
print('deity mechanism remaining:', len(tgt))
"
deity mechanism remaining: 0
```

Row count is `0` — this cycle's own mechanism is closed. `kanban.md`'s
AT-34-E3-001 row stays `in-progress`. Re-derived at this cycle's END SHA
(post-regeneration) using the atlas's own bucket-B definition
(`completion_atlas.py`'s `_bucket_of`: `status == "engine-does-not-hold"`
AND evidence contains one of `not_held_by_engine`/`absent_from`/
`not_modelled` — NOT a plain `status`-only filter, which over-counts by
including bucket-D `*_pending_wiring_class_review`/`no_explanation_id...`
units too; caught and corrected in this same cycle before it reached this
receipt), `core_rulebook` bucket B is **974**, confirmed independently
against `python3 scripts/completion_atlas.py --book core_rulebook --check`
→ `B: 974` (exact match). The four now-closed `decisions.md §14` mechanisms
(`domain`/`race_trait_absent`/`class_absent`/`deity`) are all `0`; the five
remaining sum to the ENTIRE 974 — no unnamed gap:
`class_feature_option_pool_record_not_held_by_engine` **63**,
`companion_absent_from_core_rulebook_companion_tables` **100**,
`race_trait_race_not_modelled` **132**,
`class_feature_owner_matched_by_name_but_record_not_held_by_engine`
**346** (not the `decisions.md §14` table's `330` — the `class_absent`
cycle's own commit message records a `+16` same-book `class_feature`
reattribution landing on this unowned mechanism; not this cycle's change),
`class_feature_option_pool_record_with_magnitude_not_held_by_engine`
**333**. `63+100+132+346+333 = 974`.

## Build scope verified

- `cargo test --locked --bin v06_work_inventory` (scoped): `376 passed; 0
  failed` at this cycle's HEAD, run **after** the last write that could move
  a figure (the `docs/work-inventory.json` regeneration) — `decisions.md
  §12` L7.
- `cargo test --locked --no-run` (full workspace): clean, exit 0, zero
  `error` lines in the build log. `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001`.
- `apps/desktop/src-tauri` (separate cargo workspace, tested explicitly):
  `cargo test --locked --no-run` in that directory with its own
  `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001-desktop` — clean, exit 0,
  zero `error` lines.
- Run at SHA: `9934711054e0eabda678065c4b9b5eb8d006aa0d`.

## Sweep population

`corpus_literal_sweep`: `48708 records examined of 51482 read, 0 findings,
CLEAN` — before and after this cycle's regeneration are the SAME number
(N/A: this cycle added/regenerated zero corpus records; only `classify()`'s
in-memory logic changed). No delta expected or observed.

`derived_evaluator_fixture_check`: `1839 unit(s) cleared over 2580 fixture
row(s); 0 failed; 0 not ingested` — supplied as
`DERIVED_FIXTURE_CHECK_REPORT` for the guarded regeneration, per precedent.

## Oracle pin

Not applicable — no figure in this receipt comes from the pinned PCGen
oracle corpus; the 21-unit population and its resolution come entirely from
the repo's own committed `data/corpus/` and `docs/work-inventory.json`.

- **Status:** complete

## Movement, four buckets

- **Closure:** 0 — this mechanism moves units OUT of bucket B, not into
  bucket A/DONE via a closed loop.
- **Reclassification:** 21 — all 21 deity units move from bucket B
  (`engine-does-not-hold` / `deity_content_absent_from_deity_table_in_
  core_rulebook`) to bucket D (`text-complete` /
  `deity_content_table_resolve_returned_a_real_record_with_description`).
  This is a correct outcome per this criterion's own instruction: the
  record now has a shelf and the engine holds it via the deity table; all
  21 carry `magnitude_token_count == 0` and a real `DESC:` token
  (verified: `cr_deities.lst:14`'s own raw line carries `DESC:God of
  cities, wealth, merchants, law`), so bucket D (text-complete) is the
  correct landing, not bucket M — whether any deity content *should*
  eventually carry a magnitude is a different question, out of this
  mechanism's scope.
- **Reachability:** unchanged — no unit's visibility/reachability status
  changed, only its table-resolution outcome.
- **Instrument-correction:** 2 — the two `file:line` citation drifts
  (`completion_atlas.py`'s bucket-V citation, `missing_engine_tables.py`'s
  `power` citation) this cycle's own line-insertions caused, both re-derived
  and fixed in this same cycle (`workflow-instruction.md`'s stated hazard,
  confirmed real).

## Notes

- The `simple_kind_verdict`/`resolve_by_coordinate` infrastructure already
  existed from `AT-34-E2-001`/the `domain` mechanism cycle; this cycle adds
  exactly one new call site (`Kind::Deity`), byte-identical in shape to
  `Kind::Domain`'s, per that infrastructure's own doc comment inviting this
  reuse.
- `decisions.md §14`'s own prose says the deity records' `codex_generated_
  name` field is `true` "read off the record's own `data.codex_generated_
  name` field" — verified directly: the field lives at the CORPUS JSON
  TOP LEVEL (`{"codex_generated_name": true, ...}`), not inside `data`
  itself; `docs/work-inventory.json`'s own per-unit `codex_generated_name`
  key (which does not exist in that schema at all) is a distinct, unrelated
  surface. No correction filed — the decision's substantive claim (every
  deity record is PI-masked, matched by coordinate) is verified true; this
  is a documentation-shape nuance, not a wrong fact, so `retro.py
  correction`'s `--verified-by` bar (a genuinely wrong claim) is not met.
- Two other simple-kind-table kinds (`race_trait_race_not_modelled`,
  `class_feature_owner_matched_by_name_but_record_not_held_by_engine`) are
  NOT this mechanism and were not touched.

## Next-cycle plan

Four of `decisions.md §14`'s nine named mechanisms are now closed
(`domain`, `race_trait_absent_from_race_traits`,
`class_absent_from_ClassId_ALL_and_book_class_id_enums`,
`deity_content_absent_from_deity_table_in_core_rulebook`). Remaining named:
`class_feature_option_pool_record_not_held_by_engine` (63),
`companion_absent_from_core_rulebook_companion_tables` (100),
`race_trait_race_not_modelled` (132),
`class_feature_owner_matched_by_name_but_record_not_held_by_engine` (346,
grown from the table's original 330 — see Row-count section),
`class_feature_option_pool_record_with_magnitude_not_held_by_engine` (333)
— re-derive each population fresh at the next cycle's own start SHA before
dispatching, per `decisions.md §12` L2. These five sum to exactly the
atlas's own reported `core_rulebook` bucket B (974) — no unnamed gap
remains; `class_feature_owner_matched_by_name_but_record_not_held_by_engine`
grew from `decisions.md §14`'s table figure (330) to 346 via the
`class_absent` cycle's own recorded `+16` reattribution side effect, so the
next cycle should re-derive its population fresh rather than trusting
either the original table or this receipt.
