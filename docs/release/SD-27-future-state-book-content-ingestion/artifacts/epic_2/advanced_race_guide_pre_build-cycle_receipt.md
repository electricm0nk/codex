# Cycle advanced_race_guide_pre_build — Epic 2 / Criterion 2.1

- **Cycle ID:** `advanced_race_guide_pre_build`
- **Criterion:** 2.1
- **Owner:** Backend
- **Status:** complete
- **Route class:** Sonnet (Workflow-dispatched subagent, pipelined with 2.1' and running concurrently
  with 2.2/2.2')
- **Started at:** 2026-07-27T21:03:00Z
- **Completed at:** 2026-07-27T21:31:00Z

## Inputs

- Source LST corpus at `$PCGEN_DATA_ROOT/advanced_race_guide/` (23 `.lst` files)
- `src/rules_core/shape_b_v1.rs` (the v1 schema authority)
- `docs/governance/ogl-pi-blacklist.md` (the PI-blacklist)
- The 4 in-scope books' `data/corpus/{...}/` as architectural reference (post-2.0.6-2.0.9, v1-conformant)
- `technical-design.md §2` for the `rules_tables/<book>/` generation pipeline

## Outputs

- `src/rules_core/rules_tables/advanced_race_guide/` — new Rust module (spell_list.rs,
  equipment_tables.rs + equipment_data/*.rs, feats.rs + feat_data/*.rs, json_cache.rs)
- `gen_advanced_race_guide()` in the shared `src/bin/sd27_gen_book_cache.rs` binary
- `data/corpus/advanced_race_guide/{spell,equipment,feat}/<id>.json` — **479 Shape B v1 records** (92
  spell + 200 equipment + 187 feat), each with a real LST citation (path/sha256/line/record_key)
- `data/corpus/advanced_race_guide/LICENSE.json`
- `tests/sd27_advanced_race_guide_cache_shape.rs` (8 tests)
- `data/stubs/advanced_race_guide.json`: `content_kind_counts` → `{"spell": 92, "equipment": 200, "feat": 187}`

## Scope determination (real, load-bearing — not corner-cutting)

**In scope**, following the established simple-record pattern (`class_tables`/`spell_list`/
`equipment_tables.rs`): `arg_spells.lst`, `arg_equip_arms_armor.lst`, `arg_equip_general.lst`,
`arg_equip_magic_items.lst`, `arg_equipmods.lst`, `arg_feats.lst`.

**Out of scope, with real justification, independently re-verified twice (by the pre-build agent and
again by the verify agent) against the live corpus, not taken on faith:**

| File | Real record count | Why out of scope |
|---|---|---|
| `arg_abilities_class.lst` | 792 | PCGen low-level ability/BONUS/DEFINE/PREREQ formula-engine syntax. No book in this repo, including CRB, has ever cached this content shape. |
| `arg_abilities_race.lst` | 1,359 | Racial-trait formula content, same reasoning. |
| `arg_abilities_builder.lst` | 422 | Race-builder point-buy formula content. |
| `arg_races.lst` | 39 | Race chassis definitions — no `race/` content-kind directory has ever existed under any book's `data/corpus/`. |
| `arg_races_companion.lst` | 8 | Companion-race variants, same reasoning. |
| `arg_templates.lst` | 47 | Template/racial-template content, same reasoning. |

Building a cache for this content would mean inventing a formula-parsing architecture with no
precedent to validate against anywhere in the codebase — genuinely out of this cycle's bounded scope.

## Operations

1. Read source LST corpus.
2. Inventoried content kinds; independently re-counted every file's real record count against raw LST
   (not the pre-cycle rough estimates) — found and documented 3 real discrepancies: a byte-for-byte
   duplicate equipment record ("Bonebreaker Gauntlets"), a redundant `.COPY=` row, and a "# Old KEYs"
   `VISIBLE:NO` alias block in `arg_equipmods.lst` (14 rows correctly excluded); and that 52 of
   `arg_feats.lst`'s 239 raw lines are `CATEGORY:Special Ability` sub-choices granted by a parent feat,
   not independently selectable feats (real count: 187).
3. Generated `src/rules_core/rules_tables/advanced_race_guide/` Rust module.
4. Ran the codegen tool against the rules_tables module (real per-record LST citations computed at
   generation time, values never re-derived from raw LST — matching `sd26_gen_core_rulebook_cache.rs`'s
   discipline).
5. Wrote 479 Shape B v1 JSON records + `LICENSE.json`.
6. Wrote `tests/sd27_advanced_race_guide_cache_shape.rs`.
7. Updated `data/stubs/advanced_race_guide.json` (this cycle's own file; serial-on-shared-file
   discipline did not apply here since no other cycle touches this specific stub).
8. **Real, mid-cycle partition self-correction.** Discovered — because both per-book cycles ran in the
   same shared working directory, not isolated git worktrees — that an initial design (registering the
   book in `src/rules_core/rules_tables/mod.rs`, a standalone per-book binary, a `RuleSetId` variant)
   violated the literal partition allow-list (`loop-instruction.md §6`), which allow-lists exactly one
   shared binary (`src/bin/sd27_gen_book_cache.rs`), not a per-book file, and does not allow-list
   `rules_tables/mod.rs` at all. Reverted the `mod.rs` edit, deleted the standalone binary, and folded
   generation into the shared binary's `gen_advanced_race_guide()` function alongside the sibling PU
   cycle's own `gen_pathfinder_unchained()` (verified neither clobbers the other — independently
   re-confirmed by the orchestrator post-hoc, see Verification).
9. Ran dual-audit gate.

## Verification

- **Independently re-verified by the orchestrator** (not just the implementing subagent's own report):
  - `git status --porcelain` scope check: only `data/corpus/advanced_race_guide/`,
    `data/stubs/advanced_race_guide.json`, `src/rules_core/rules_tables/advanced_race_guide/`,
    `src/bin/sd27_gen_book_cache.rs` (shared), `tests/sd27_advanced_race_guide_cache_shape.rs` touched.
    `src/rules_core/rules_tables/mod.rs` confirmed genuinely unmodified (`git diff --stat` empty).
  - `src/bin/sd27_gen_book_cache.rs` read directly: confirmed both `gen_pathfinder_unchained()` (line
    253) and `gen_advanced_race_guide()` (line 421) present, `main()`'s `match` dispatches to both — no
    clobbering.
  - Population field: `python3` sweep of all 479 on-disk records confirmed 479/479 `"population":
    "in_scope"` (a mid-run agent-to-agent report during concurrent execution had claimed a stale
    `"future_state"` reading; confirmed this was a timing artifact from a snapshot taken while this
    cycle's own generation was still in progress, not a real defect in the final state).
  - Spot-checked 2 real records directly (`resilient_reservoir.json` spell, a formula-priced
    `equipmods` record): recomputed `sha256sum` of the cited real LST file myself and confirmed exact
    match; read the cited line number directly and confirmed it matches the record's `data` fields;
    confirmed the formula-priced record correctly has `cost_gp: null` rather than a fabricated number.
  - `cargo build --workspace --locked` → clean (dead-code warnings only, expected since this module is
    reached only via the codegen binary's `#[path]` include, not the public library surface — matches
    the partition constraint, not an oversight).
  - `cargo test --workspace --locked --test sd27_advanced_race_guide_cache_shape` → 8/8 passed,
    re-run independently.
  - `cargo test --workspace --locked --no-fail-fast` → **4,817 passed / 3 failed** (exactly the 3
    pre-existing environment-dependent failures, unchanged; 4,817 = 4,802 baseline + 8 (ARG) + 7 (PU) new
    tests, exact arithmetic match).
  - `tests/sd27_license_stripping_shape_v1.rs` (the shared cross-cutting gate) → 4/4 passed, now
    covering 4,973 total records across all 6 books.
  - Dual-audit gate against `HEAD`: `OK_NO_BUNDLE_TAGS`, `AUDIT PASSED. All four checks clean.`
  - 5th audit (PI-blacklist): 0 defects across ARG's 479 records.

## Notes

- This cycle is file-disjoint with cycle 2.2 (Pathfinder Unchained), but both ran in the same shared
  working directory (no `isolation: 'worktree'`) — a real coordination risk, not just a theoretical
  one. It resolved correctly this time via mid-task discovery and self-correction, but this should not
  be relied on for future concurrent per-book cycles; recommend `isolation: 'worktree'` for any future
  batch of 3+ books run this way.
- Operator-gated: this cycle succeeded; no fallback decision was needed.
- Two minor cosmetic issues found during verification and fixed directly by the orchestrator: a stale
  doc comment in the test file referencing the deleted standalone binary, and a PI-blacklist doc
  comment copy-pasted from the PU module that didn't mention ARG despite the constant now being shared
  by both books' generators.
