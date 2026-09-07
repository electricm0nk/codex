# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`domain` mechanism)

This cycle owns **exactly one** of the nine mechanisms `decisions.md §14` decomposed
`AT-34-E3-001` into: `domain_content_absent_from_domain_table_in_core_rulebook`, the
smallest of the nine (1 of 1,006 remaining `core_rulebook` bucket-B units at cycle start).
It does **not** close AT-34-E3-001 itself — eight other mechanisms remain, each its own
cycle (`decisions.md §14`'s table).

- **Commit SHA:** `6eab21d761`
- **Files touched:**
  - `src/rules_core/rules_tables/simple_kind_tables.rs` — `SimpleKindTable::
    resolve_by_coordinate`, a new `by_coordinate` index built from each
    record's own `rename.coordinate` field (present only on PI-masked
    records), plus one new unit test proving the real `domain` table
    resolves `Death (Pharasma)` at `cr_domains.lst:46` by coordinate, never
    by its masked key or (never-stored) real name.
  - `src/bin/v06_work_inventory.rs` — `simple_kind_verdict` gained an
    `Option<&str>` coordinate fallback parameter (6 call sites pass `None`,
    byte-identical to their pre-fix behaviour; only `Kind::Domain`'s call
    site builds and passes `Some("{book}:{source_file}:{source_line}")`
    from the unit's own provenance); two new unit tests (RED confirmed,
    then GREEN) proving a PI-renamed domain unit now leaves bucket B and a
    genuinely-absent coordinate still refuses cleanly.
  - `scripts/completion_atlas.py` — **self-caused regression, fixed same
    cycle**: this cycle's insertions in `v06_work_inventory.rs` shifted all
    ten `BUCKET_DEFINITIONS` `file:line` citations. Each was re-derived by
    `grep -n` against the post-edit file (matched to its pre-edit content
    via `git show HEAD:...`, not guessed) and the literals corrected.
    `--check`'s `citation_failures` went `8 -> 0` (this cycle's own edits
    caused all 8; the other two of the ten citations were unaffected
    because they sit inside the same, already-shifted region and matched
    once the base shift was applied).
  - `docs/work-inventory.json` (regenerated at HEAD, guarded regeneration
    path — plain `cargo run --locked --bin v06_work_inventory`, no
    `--allow-stamp-loss` used or needed; `CORPUS_LITERAL_SWEEP_REPORT` and
    `DERIVED_FIXTURE_CHECK_REPORT` set from this session's own fresh
    `corpus_literal_sweep`/`derived_evaluator_fixture_check` runs so the
    stamp-loss guard had real evidence rather than refusing).
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    (regenerated output of `completion_atlas.py --check`, not hand-edited).
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_domain_cycle_receipt.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`

- **Identifier audit result:** OK_NO_BUNDLE_TAGS on this cycle's own diff
  (`git diff -- src/rules_core/rules_tables/simple_kind_tables.rs
  src/bin/v06_work_inventory.rs | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → no matches). The wider `${BASE_BRANCH}...HEAD` diff over the full Epic-3
  file-touch set DOES contain thousands of `sd32_simple_filename_kind_ingest`/
  `sd32_class_ingest` matches inside `docs/work-inventory.json` — these are
  pre-existing historical `wiring_class_signals` **data values** from
  `AT-34-E1-008` (commit `54e2d24e83`, landed before this cycle started,
  verified by `git log --oneline -- data/corpus/core_rulebook/domain/
  codex_named_unit_domain_core_rulebook_cr_domains_lst_46.json` and `git show
  54e2d24e83 --stat`), not identifiers this cycle introduced, and not code.

- **Wired-integration audit result:** OK_NO_TOKENS (same own-diff scope,
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`
  → no matches).

- **Acceptance criterion (verbatim, `epic-breakdown.md` AT-34-E3-001):**
  "**970** Core Rulebook units whose table exists but which are not in it.
  **Evidence:** the atlas reporting bucket B at zero for `core_rulebook`,
  and the mechanism that placed them named — by mechanism, not per
  record." This cycle's own bar (`decisions.md §14`, the orchestrator's
  ruling that decomposed the criterion): the `domain` mechanism's own
  population reaches zero, verified end-to-end with a RED→GREEN proof —
  not the whole criterion.

- **Root cause (re-derived, not assumed from the prior cycle's stale
  claim):** the prior cycle's escalation (`AT-34-E3-001_cycle_receipt.md`)
  stated "no corpus JSON anywhere under `data/corpus/core_rulebook/`" for
  `Death (Pharasma)` at `cr_domains.lst:46`. That was true when written but
  is now stale: `AT-34-E1-008` (a different, already-landed cycle) restamped
  wiring classes across `core_rulebook` and, as a side effect, the record
  now physically exists at
  `data/corpus/core_rulebook/domain/codex_named_unit_domain_core_rulebook_cr_domains_lst_46.json`
  — but its `key`/`name` are PI-masked to `Codex-Named Unit (...)` because
  the domain's own name embeds the deity `Pharasma` (Product Identity).
  `SimpleKindTable::resolve` looks up by `(book, key)`/`(book, name)`, and
  the work-inventory unit's own `corpus_key` is the real, un-masked LST name
  (`"Death (Pharasma)"`), so the lookup could never hit even though the
  record existed. The fix is therefore **not** ingestion (the directive's
  original framing, now superseded by this re-derivation) but a **PI-safe
  coordinate resolve path** — exactly the shape `decisions.md §14` already
  prescribes for the sibling `deity` mechanism: match on the record's own
  stored `(book, source_file, source_line)`, never the redacted real name.

- **Figures + their re-derive commands:**
  - Population at cycle start, re-derived (not quoted from the directive):
    `python3 -c "import json; inv=json.load(open('docs/work-inventory.json')); b=[u for u in inv['units'] if u['book']=='core_rulebook' and u['status']=='engine-does-not-hold' and u['evidence']=='domain_content_absent_from_domain_table_in_core_rulebook']; print(len(b))"`
    → **1** (matches `decisions.md §14`'s table exactly).
  - Population after this cycle's fix + regeneration, same command → **0**.
  - Bucket B total for `core_rulebook`, before → after: `python3 -c "import json; inv=json.load(open('docs/work-inventory.json')); print(len([u for u in inv['units'] if u['book']=='core_rulebook' and u['status']=='engine-does-not-hold']))"` → **1810 → 1809** (denominator: 6,701 `core_rulebook` units total).
  - Whole-corpus unit id-level diff (before vs after, all 49,438 units): `diff <(git show HEAD:docs/work-inventory.json | python3 -c "import json,sys; print('\n'.join(sorted(u['id'] for u in json.load(sys.stdin)['units'])))") <(python3 -c "import json; print('\n'.join(sorted(u['id'] for u in json.load(open('docs/work-inventory.json'))['units'])))")` — **exactly 2 units changed**, both domain records whose name
    embeds `Pharasma` — `core_rulebook:domain:death_pharasma` (this
    cycle's own target) and `advanced_players_guide:domain:
    souls_pharasma_subdomain` (a **different book's** own
    `domain_content_absent_from_domain_table_in_advanced_players_guide`
    mechanism, not part of this cycle's 1,006-unit population (re-derived via `python3 scripts/completion_atlas.py --book core_rulebook --check`) and not this
    cycle's to claim — reported here for honesty per `workflow-instruction.md
    §6` step 9, since the same generic coordinate-resolve code path closed
    it as a side effect. It is a correct promotion of a real held record,
    same rung as the target unit; the owning Epic 3 book (`core_rulebook`)
    still requires its own cycles to close its other 8 mechanisms, and this
    receipt makes no claim about `advanced_players_guide`'s own bucket B).
  - Atlas re-derive: `python3 scripts/completion_atlas.py --check` →
    `population=49438 buckets=10 unclassified=0 overlap=0`, `citation_failures=0`.
  - `docs/work-inventory.json` unit-population sanity: `population=49438`
    across both the before and after files (id sets identical, verified by
    set-equality in the same script above) — no unit created or destroyed,
    only 2 reclassified.

- **Row-count command output (literal, this cycle's own artifact — the
  `domain` mechanism's population in the regenerated
  `docs/work-inventory.json`):**
  ```
  $ python3 -c "
  import json
  inv = json.load(open('docs/work-inventory.json'))
  b = [u for u in inv['units'] if u['book']=='core_rulebook'
       and u['status']=='engine-does-not-hold'
       and u['evidence']=='domain_content_absent_from_domain_table_in_core_rulebook']
  print(len(b))
  "
  0
  ```

- **Build scope verified:** `cargo test --locked --no-run` at the full
  workspace scope, run at commit SHA (see above) — exit 0 (see this
  receipt's commit for the exact log). `apps/desktop/src-tauri` is a
  separate cargo workspace; not touched this cycle, not run.
  Targeted suites run and green before that: `cargo test --lib
  rules_core::rules_tables::simple_kind_tables` (12 passed), `cargo test
  --bin v06_work_inventory` (371 passed, 0 failed) — full binary suite, not
  a filtered subset.

- **Sweep population:** `corpus_literal_sweep`: before `48699 records
  examined of 51473 read` (SD-33/prior-cycle baseline) → after `48699
  records examined of 51473 read` (this cycle's own re-run, exit `CLEAN`,
  `0 findings`). **Delta: 0, matching a record delta of 0** — this cycle
  did not add, remove, or regenerate any `data/corpus/**` file (confirmed:
  `git status --porcelain -- data/corpus/` is empty throughout); the
  corpus record it resolves already existed, landed by a different,
  already-committed cycle (`AT-34-E1-008`).

- **Oracle pin:** Not applicable — no figure in this receipt is sourced
  from the pinned PCGen oracle checkout; this is corpus-attribution
  classifier logic, not an oracle-compared magnitude.

- **Status:** complete

- **Movement, four buckets:**
  - **Closure:** 1 unit (`core_rulebook:domain:death_pharasma`) moves from
    bucket B (`engine-does-not-hold`,
    `domain_content_absent_from_domain_table_in_core_rulebook`) to bucket M
    (`ingested-magnitude`, `domain_content_table_holds_record_magnitude_
    not_yet_computed`) — a real PI-safe lookup-path defect fixed, not a
    relabeling of the same status. Per `decisions.md §2a`: the domain
    table is a lookup, not a compute path, so a held record carrying a
    real magnitude token (this one carries 1) lands `ingested-magnitude`,
    not `grounded`/`text-complete` — the record now has a shelf and the
    engine holds it; whether it computes is a different bucket's (M's own)
    concern, for a later cycle/Epic 5's compute-path work.
  - **Reclassification:** 0 — the 1 unit genuinely changes engine-visible
    status (was invisible to `resolve`, now resolves), it is not merely
    relabeled at the same status.
  - **Reachability:** 0 — `ingested-magnitude` is still not player-visible;
    no unit newly reachable by a player this cycle.
  - **Instrument-correction:** 1 — the prior cycle's escalation stated "no
    corpus JSON anywhere" for this record; re-derived at HEAD, the record
    physically exists (landed by `AT-34-E1-008`, a different, already-
    committed cycle) but was unreachable by the classifier's key/name
    lookup because of PI-masking. Logged as a `correction` retro event
    (see `scripts/retro.py` invocation below).
  - **Side-effect movement, reported honestly, not claimed as this
    cycle's own scope:** 1 unit in `advanced_players_guide`'s own bucket B
    (a different mechanism, different book) also resolved by the same
    generic code path. Not counted toward this cycle's 1-unit population
    and not toward `core_rulebook`'s bucket-B closure.

- **Notes:** This is the smallest of the nine mechanisms `decisions.md
  §14` named (1 of 1,006). The other eight remain: `race_trait_absent_
  from_race_traits` (9), `class_absent_from_ClassId_ALL_and_book_class_id_
  enums` (17), `deity_content_absent_from_deity_table_in_core_rulebook`
  (21, same PI-safe-coordinate shape this cycle proved out — a future
  cycle can very likely reuse `resolve_by_coordinate` directly, wiring
  `Some(coordinate)` at `Kind::Deity`'s own call site the same way this
  cycle wired `Kind::Domain`'s), `class_feature_option_pool_record_not_
  held_by_engine` (63), `companion_absent_from_core_rulebook_companion_
  tables` (100), `race_trait_race_not_modelled` (132), `class_feature_
  owner_matched_by_name_but_record_not_held_by_engine` (330), `class_
  feature_option_pool_record_with_magnitude_not_held_by_engine` (333).
  **AT-34-E3-001 does not close this cycle** — bucket B for `core_rulebook`
  is 1,809 of 6,701, not zero. This receipt closes only its one named
  mechanism, per the direction that owns it.

- **Next-cycle plan:** `deity` (21 units) next — smallest of the remaining
  eight and the same PI-safe-coordinate-match shape this cycle already
  proved end-to-end, so it should carry a similar cost. `decisions.md §14`
  already states its constraint: match on stored coordinates, keep the
  masked key, never read/log/emit the redacted real name;
  `scripts/verify.sh --only site-public-status-pi-gate` and
  `--only site-dashboard-pi-gate` must stay green.
