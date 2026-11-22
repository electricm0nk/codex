# Cycle pathfinder_unchained_pre_build — Epic 2 / Criterion 2.2

- **Cycle ID:** `pathfinder_unchained_pre_build`
- **Criterion:** 2.2
- **Owner:** Backend
- **Status:** complete
- **Route class:** Sonnet (Workflow-dispatched subagent, pipelined with 2.2' and running concurrently
  with 2.1/2.1')
- **Started at:** 2026-07-27T21:03:00Z
- **Completed at:** 2026-07-27T21:26:00Z

## Inputs

- Source LST corpus at `$PCGEN_DATA_ROOT/pathfinder_unchained/` (11 `.lst` files)
- `src/rules_core/shape_b_v1.rs`, `docs/governance/ogl-pi-blacklist.md`

## Outputs

- `src/rules_core/rules_tables/pathfinder_unchained/` — new Rust module (feat_tables.rs,
  equipment_tables.rs, mod.rs)
- `gen_pathfinder_unchained()` in the shared `src/bin/sd27_gen_book_cache.rs` binary
- `data/corpus/pathfinder_unchained/{feat,equipment}/<id>.json` — **59 Shape B v1 records** (17 feat +
  42 equipment), each with a real LST citation
- `data/corpus/pathfinder_unchained/LICENSE.json`
- `tests/sd27_pathfinder_unchained_cache_shape.rs` (7 tests)
- `data/stubs/pathfinder_unchained.json`: `content_kind_counts` → `{"feat": 17, "equipment": 42}`

## Scope determination

**In scope:** `pu_equipmods.lst` (42 records), `pu_feats.lst` (17 real distinct feats — 18 raw lines,
1 excluded as a `.MOD` patch onto an existing APG feat, matching the established `.MOD`-exclusion
precedent from `rules_tables::crb::feats`).

**Honest absence, not a missed content-kind:** `pu_spells.lst` is 224 lines, confirmed by direct read
100% comment-only — this book adds no new spells at all. No `spell/` directory was created; a `null`
count would have been dishonest (implies a counting pass ran and found zero), so this content kind is
simply absent from `content_kind_counts`.

**Out of scope, real justification:** `pu_abilities_class.lst` (1,344 real lines — same PCGen
ability/BONUS/DEFINE/PREREQ formula-engine syntax as ARG's equivalent file, no precedent anywhere in
the codebase), `pu_skills.lst` (120 lines — no established "skill" content-kind for any book),
`pu_templates.lst` (17 lines — no established "template" content-kind for any book).

## Operations

1. Read source LST corpus; independently re-counted every file's real record count against raw content
   (not the pre-cycle rough estimate) — found `pu_feats.lst` cites 18 raw lines but only 17 are real,
   distinct new feats.
2. Generated `src/rules_core/rules_tables/pathfinder_unchained/` Rust module — every field hand-verified
   against real LST rows.
3. Ran the codegen tool (real per-record LST citations, values never re-derived from raw LST).
4. Wrote 59 Shape B v1 JSON records + `LICENSE.json`.
5. Wrote `tests/sd27_pathfinder_unchained_cache_shape.rs`.
6. Updated `data/stubs/pathfinder_unchained.json`.
7. Documented the file-touch partition constraint on `src/rules_core/rules_tables/mod.rs` (not
   allow-listed by `loop-instruction.md §6`'s literal regex) — used a `#[path]` include into the shared
   codegen binary instead of registering the module in the library's normal module tree. This means
   `rules_tables::pathfinder_unchained` is reachable only from the `sd27_gen_book_cache` binary crate,
   not `codex::rules_core::rules_tables::pathfinder_unchained` — the same constraint ARG's cycle hit
   and resolved the same way. A future cycle with authority to touch `rules_tables/mod.rs` should wire
   it into the library's public surface properly.
8. Ran dual-audit gate.

## Verification

- **Independently re-verified by the orchestrator:** `git status --porcelain` scope check clean
  (only this book's own paths + the shared binary touched). `sd27_gen_book_cache.rs` read directly:
  `gen_pathfinder_unchained()` (line 253) present and correctly dispatched from `main()`.
- Spot-checked a real feat record (`champion_of_destruction.json`) directly: recomputed the cited
  LST file's sha256 myself, confirmed exact match; confirmed the cited line content matches.
- `cargo test --workspace --locked --test sd27_pathfinder_unchained_cache_shape` → 7/7 passed,
  independently re-run.
- Full workspace suite (shared with ARG's receipt): 4,817 passed / 3 pre-existing failures, zero
  regressions.
- Dual-audit gate: clean. 5th audit: 0 defects across PU's 59 records.

## Notes

- This cycle is file-disjoint with cycle 2.1 (Advanced Race Guide), but both ran in the same shared
  working directory, not isolated git worktrees — same coordination-risk note as ARG's receipt.
- Operator-gated: this cycle succeeded; no fallback decision was needed.
