# Cycle row17-instrument-1 — Gate 1 / Row 17 (`epic-7-shape-categorization-100`) instrument

- **Card ID:** `epic-7-shape-categorization-100` (kanban.md row 17)
- **Commit SHA:** (see push result)
- **Files touched:**
  - `scripts/shape_ledger.py` — `classify_unit` now returns `f0_reached_by`
    (`not_ingested`/`measured_empty`/`fallthrough`) and `pi_redacted_formula`;
    `build_ledger` aggregates `f0_breakdown` + `f0_fallthrough_pi_redacted`;
    CLI prints the breakdown. Imports `scripts/pi_scrub.py::REDACTED_PI_MARKER`
    rather than re-typing it.
  - `scripts/shape_provisional_marker.py` (new) — the §27 provisional-shape-
    default contract: `stamp_provisional_default` (the one sanctioned way to
    set the marker, requires a non-empty reason), `is_provisional_default`,
    `provisional_reason`, `scan_corpus_for_provisional_defaults` (read-only).
  - `scripts/row17_census.py` (new) — the per-kind/per-book census this
    dispatch brief asked for, tying `shape_ledger` + `shape_provisional_marker`
    together; `--check` fails on a marker with a missing reason.
  - `scripts/tests/test_shape_ledger.py` — 6 new tests (RED→GREEN proved for
    `f0_reached_by`/`pi_redacted_formula`/`f0_breakdown` via inline mutation,
    reverted).
  - `scripts/tests/test_shape_provisional_marker.py` (new) — 11 tests.
  - `scripts/tests/test_row17_census.py` (new) — 5 tests, including a
    mutate/RED/revert/GREEN proof against the real on-disk
    `build_corpus_index` join path (not just in-memory fixtures).
  - `docs/release/SD-32-compute-library-and-cause-closure/workflow-instruction.md`
    — new §6a documenting the §27 marker contract and where cycles read it.
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 17
    Notes updated with what landed and the honest size; **status left
    `backlog`** (unchanged — see Notes below).
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-1-shape-closure/row17-census.json`
    (new) — the re-derived census output, committed as evidence.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (checked against both the
  tracked-file diff and the four new files directly — the tracked-file diff
  used `git diff --unified=0 HEAD` since this cycle's own scope, not the
  brief's stale `BASE_BRANCH...HEAD` form, which returns tens of thousands of
  pre-existing tagged lines from unrelated prior cycles and is not a
  per-cycle signal per §6 step 2's own caution).
- **Wired-integration audit result:** the raw grep for
  `\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b` hits nine
  lines, all the literal word "placeholder" used as domain vocabulary
  (row 17's own subject matter — "a placeholder shape assignment", quoting
  `decisions.md §27a`'s own phrase "a placeholder wearing a family label").
  A direct search for `\b(STUB|MOCK)\b` alone across every touched/new file:
  zero hits. **OK_NO_TOKENS** (self-healed by inspection — no STUB/MOCK/
  fixme/hack, and every "placeholder" hit is prose describing the concept
  this row exists to detect, not a stub in shipped code).
- **Acceptance criterion:** dispatch brief, "Your scope — build the
  INSTRUMENT kanban row 17 needs": (1) a machine-countable marker for §27's
  provisional default, impossible to set silently, documented where cycles
  read it; (2) a census over the ledger + corpus reporting derived vs.
  defaulted/fallthrough per kind/book, covering both §27a-named populations;
  (3) row 17's honest size, re-derived, not softened.
- **Corpus SHA:** oracle pin `7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/verify.sh --only preflight-oracle` → PASS this cycle).
- **Status:** complete (this cycle's own instrument-building scope — see
  "What this cycle is NOT" below for what remains genuinely blocked).
- **Notes:**
  - **Re-derived, not inherited.** The dispatch brief's own F0 figures
    (20,113 of 24,914) were stale (per `decisions.md §17a`'s standing
    caution). Live population is **34,631** not-done units; live F0 is
    **23,289**. `no_record` is **132**, matching the brief's own figure.
  - **The instrument defect the brief asked me to check for is real.**
    Before this cycle, `shape_ledger.py`'s `join_status` field already
    distinguished `no_record` from `no_formula_tokens`, but **could not**
    tell a genuinely-measured F0 apart from an F0 reached because every
    present DEFINE/BONUS token on a `matched` record failed to classify.
    That third case existed and was silently folded into `matched`'s
    family rollup with no visible marker — exactly the placeholder-wearing-
    a-family-label shape `decisions.md §27a` names.
  - **Re-deriving it found 84 real units in that bucket**, 83 of which are
    `decisions.md §24b`-ingested Codex-named units whose BONUS/DEFINE token
    VALUE is the literal `[redacted PI]` marker — the record's mechanical
    formula was blanket-redacted alongside its NAME/DESC, not genuinely
    absent. This is a **found instrument-adjacent defect**, reported here
    per this bundle's `§17a` discipline rather than silently fixed (fixing
    the redaction pipeline itself is `equipment`/`ability`/`.FORGET`-lane
    or PI-screen territory this cycle's scope explicitly excludes — "Do no
    ingest work"). The 84th fallthrough unit is a structurally short/
    malformed token, not PI-redaction-shaped; not investigated further
    (out of this cycle's instrument-building scope).
  - **§27's provisional-default marker count is 0 corpus-wide.** No ingest
    cycle has applied the `SpecialQuality` delivery-only default yet
    (confirmed: `FACETS`/`monster_chassis.rs` still deliberately exclude
    it, "each needs its own per-record read" per that file's own comment).
    This is expected, not a defect — the contract exists so the FIRST cycle
    that does apply it is forced through `stamp_provisional_default`.
  - **ROW 17 HONEST SIZE: 84** (fallthrough 84 + provisional-default 0),
    per `python3 scripts/row17_census.py`. Concentrated: `inner_sea_gods`
    43 (`trait` 25 + `ability` 18), `adventurers_guide` 12, `advanced_
    players_guide` 10, plus 11 more units across 6 other books/3 more
    kinds — full breakdown in the committed `row17-census.json`'s
    `per_kind_book` array.
  - **Row 17 stays `backlog`, correctly.** `no_record` (132) is nonzero, and
    `decisions.md §27`/`kanban.md` row 17 both sequence the categorization
    pass strictly after `no_record` reaches zero — closing 132 units is a
    sibling ingest lane's scope (this dispatch's territory notice: "a
    `monster_ability` lane (98 units)" and "a final-32 lane" both live in
    that space), not this instrument-building cycle's. `git status
    --porcelain data/corpus` is empty throughout — no corpus records
    written, read-only census/marker-contract work only.
  - **What this cycle is NOT:** it does not close row 17 (impossible — row
    17 cannot start while `no_record` is nonzero), it does not apply §27's
    default anywhere, and it does not fix the 83-unit PI-redaction-formula
    defect it found (out of scope, named for the operator/next cycle
    instead per Blocker Discipline disposition 2 — this is a finding, not
    a blocker on THIS cycle's own DoD, since the instrument itself is
    complete and correctly reports it).
  - **RED→GREEN proved twice**, at two altitudes: (1) `shape_ledger.py`'s
    own `pi_redacted_formula` detection, mutated to `False` inline, proved
    both new unit tests fail, reverted, proved green (75/75 including
    pre-existing 53). (2) `row17_census.py`'s `test_census_goes_red_on_
    mutation_and_green_on_revert` mutates a real on-disk corpus JSON file
    in a temp fixture (not an in-memory dict) so a genuinely-derived unit's
    formula becomes PI-redacted, confirms `row17_honest_size` moves 1→2 and
    `derived` moves 1→0, reverts, confirms it moves back — proving the
    census is live against the real `build_corpus_index` code path, not
    just the ledger's in-memory classification.
  - 90/90 tests green across the three touched/new test files
    (`test_shape_ledger.py` 59, `test_shape_provisional_marker.py` 11,
    `test_row17_census.py` 5, plus `test_coverage_ledger.py` spot-checked
    unaffected). Full unscoped `cargo test` NOT run (Python-only change;
    no Rust files touched).
  - Disk: `df -h /` at cycle end reported in the final turn report.
- **Discovery forwards:** none filed as `## DISCOVERED` — the 83-unit PI-
  redaction-formula finding is recorded in this receipt and in row 17's own
  kanban notes rather than a separate queue entry, since it is directly
  inside this cycle's own re-derivation, not an unrelated tangent.
- **Next-cycle plan:** once the `monster_ability`/final-32/no_record-closure
  lanes bring `no_record` to 0, row 17 can genuinely start. Its first move
  should re-run `python3 scripts/row17_census.py` (never trust this
  receipt's 84 as still current — re-derive per `§17a`) and work the
  `fallthrough` list kind-by-kind/book-by-book, starting with `inner_sea_
  gods` (43 units, over half the current total). Any cycle applying §27's
  `SpecialQuality` default must call `shape_provisional_marker.
  stamp_provisional_default` at the point of ingest, and row 17's own
  closure requires both `fallthrough` and `provisional_default` reaching 0
  in `scripts/row17_census.py`'s totals — not a budget, not a threshold.
