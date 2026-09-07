---
canonical: true
owner: sd34-at-34-e6-001
bundle_id: SD-34
criterion: AT-34-E6-001 (gate-lane-C label, wave 25 -- NOT the final-acceptance scan)
date: 2026-09-01
verdict: complete
---

# Wave 25, Gate Lane C -- clippy, the one stage nobody had touched

**Filename note (read first), same self-heal shape as wave 23's gate-lane-a/b/c and wave 24's
gate-lane-a/b receipts.** This dispatch's brief points at
`artifacts/epic-6-closure/AT-34-E6-001_cycle_receipt.md`. That path is already occupied by the
real, committed, historical `AT-34-E6-001` **final-acceptance-scan** attempt 1 (`17f5245f61`,
2026-08-29, verdict FAIL). `AT-34-E6-001` is reused here purely as an Epic-6 tracking label for
one more gate-remediation lane -- not the final-acceptance scan itself (`kanban.md` row 26,
`final-acceptance-scan`, still correctly `not-started`), and not the same lane C as wave 23's
(`AT-34-E6-001_gate-lane-c_cycle_receipt.md`, which closed `denominator-gate`/`figure-provenance`
-- a different pair of stages). Written to a non-colliding, wave-tagged filename instead, per
`workflow-instruction.md §8`'s self-heal posture. `kanban.md` row 26 is **not** touched by this
cycle.

- **Commit SHA:** see bottom (pushed after this receipt)
- **Files touched:**
  - `src/rules_core/rules_tables/monster_codex/spell_list.rs` (3 soft-hyphen bytes -> regular
    hyphens; the hard compile error, see Discoveries)
  - `src/rules_core/pilot_compute/mod.rs` (2 dead functions + their now-redundant dedicated tests
    deleted; 2 module constants gated `#[cfg(test)]`; 2 `type` aliases added for
    `type_complexity`; 4 `#[allow(clippy::too_many_arguments)]`; assorted doc/mechanical fixes)
  - `src/bin/v06_work_inventory.rs` (1 `type` alias; 2 `field_reassign_with_default` fixes; 2
    `#[allow(clippy::too_many_arguments)]`)
  - `src/bin/fixture_verified_oracle_probe.rs` (1 `type` alias)
  - `src/bin/ingest_race_traits.rs` (empty-line-after-doc-comment fix)
  - `src/rules_core/pilot_compute/class_feature_grant_consumer.rs` (`unnecessary_get_then_check`
    fix; empty-line-after-doc-comment fix)
  - `src/rules_core/cache_gen/class_feature.rs` (`unnecessary_get_then_check` fix)
  - `src/rules_core/cache_gen/spell_mod_access.rs` (1 `type` alias)
  - `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs` (1 local `type` alias)
  - `src/rules_core/rules_tables/ultimate_psionics/aegis_features.rs` (`double_parens` fix)
  - `tests/sd13_barbarian_level{2,3,4,5,6,7,8}_progression.rs` (7 files: dead `claim_blocking`
    helper + its now-unused imports deleted from each)
  - `tests/sd25_monk_level_up_explanation_filter_audit.rs` (`doc_lazy_continuation` fix, blank
    line before `1a.`)
  - `tests/feat_gap_tables.rs` (`tabs_in_doc_comments` fix, `\t` written out instead of a literal
    tab byte)
  - `apps/desktop/src-tauri/src/character_hub.rs` (boxed `corpus_derived`/`snapshot` across all
    three response enums for `large_enum_variant`; 1 narrow `#[allow(dead_code)]`; every affected
    construction/destructure site updated to match)
  - `apps/desktop/src-tauri/src/pf1_adapter.rs` (2 construction sites updated for the same boxing)
  - `apps/desktop/src-tauri/src/characterHub/appendToCharacter.rs` (1 destructure site unboxed)
  - `apps/desktop/src-tauri/src/trait_picker.rs` (5 `clone`-to-`std::slice::from_ref` fixes)
  - `apps/desktop/src-tauri/src/class_catalog_generic.rs` (`cargo clippy --fix` mechanical fix)
  - Also touched by `cargo clippy --fix --tests --allow-dirty` (root pass, mechanical only --
    `identity_op`/`op_ref`/`useless_conversion`/`manual_range_contains`/unused-import fixes, no
    hand edit beyond what `cargo fix` itself wrote): `src/rules_core/cache_gen/
    equipment_copy_citation_repair.rs`, `src/rules_core/cache_gen/equipment_gap.rs`,
    `src/rules_core/equipment_effects/equipmods.rs`, `src/rules_core/rules_tables/{ultimate_
    intrigue,ultimate_wilderness,ultimate_magic,mythic_adventures,advanced_race_guide,
    pathfinder_unchained,bestiary_5,bestiary_6,occult_adventures}/monster_data.rs`,
    `src/rules_core/pilot_compute/generic_class_chassis.rs`,
    `src/rules_core/rules_tables/companion_chassis.rs`,
    `src/rules_core/rules_tables/crb/weapon_tables.rs`, `src/rules_core/corpus_loader.rs`,
    `src/oracle_validation/normalization.rs`, `src/rules_core/trait_effects.rs`,
    `src/bin/declared_pi_shipping_audit.rs`, `src/bin/formula_interpreter.rs`,
    `src/bin/gen_core_rulebook_cache.rs`, `tests/sd24_acg_equipment_field_completion.rs`,
    `tests/sd24_equipment_field_completion.rs`, `tests/sd25_barbarian_level_up_explanation_
    filter_audit.rs`, `tests/sd26_pilot_case_verification.rs`,
    `tests/sd27_advanced_race_guide_parity.rs`, `tests/sd27_pathfinder_unchained_parity.rs`,
    `tests/v06_wizard_pilot_case_verification.rs`
  - `scripts/verify-baselines.env` (both clippy ceilings tightened 50/7 -> 0/0, deliberate, see
    Figures)
  - `docs/release/SD-34-book-completion/artifacts/epic-6-closure/
    AT-34-E6-001_gate-lane-c_wave25_cycle_receipt.md` (new, this file)
  - `docs/release/SD-34-book-completion/progress.md` (prepended cycle entry, same commit)
  - `docs/release/SD-34-book-completion/kanban.md` (untouched -- see filename note)
  - `docs/retro/events/sd34-at-34-e6-001.jsonl` (this cycle's own `correction` event appended --
    a pre-existing shared shard, `RETRO_ACTOR=sd34-at-34-e6-001` per the brief's own environment
    setup, also used by wave-24's lanes A/B; this cycle added exactly one new line)

- **Identifier audit result:** OK_NO_BUNDLE_TAGS -- `git diff --unified=0 -- src/ tests/
  apps/desktop/src-tauri/src/ ':!**/__tests__/**' ':!**/*.test.*' | grep -E '^[+-]' | grep -vE
  '^(\+\+\+|---)' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` returns nothing;
  the raw (unfiltered) command does hit, but every hit is a `diff --git`/`---`/`+++` header line
  restating a pre-existing `tests/sd13_...`/`tests/sd2[4-7]_...` filename, this repo's own
  test-naming convention (matches wave 23 gate-lane-a's own precedent finding) -- no new-content
  line carries a bundle-tag-shaped token.
- **Wired-integration audit result:** OK_NO_TOKENS -- same own-diff scoping, zero
  `STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack` tokens added.
- **Acceptance criterion (verbatim, dispatch brief):** "AT-34-E6-001 -- GATE LANE C -- clippy, the
  one stage nobody has touched. ... root 86 warnings against a ceiling of 50 (36 over), desktop 25
  against a ceiling of 7 (18 over). ... Fix the warnings; do not raise the ceilings. ... Territory:
  clippy warnings wherever they live... You run LAST of the three, so their edits are already in
  -- rebase before you start and re-measure after."

## Figures + their re-derive commands

| Figure | Value | Command | Denominator |
|---|---:|---|---|
| Root, BEFORE this cycle (rebased HEAD `4ae54a8c73`, before any fix) | **exit 101** (1 hard `error`) + 86 `warning:` lines counted | `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001 cargo clippy --locked --tests` (log: `/tmp/clippy-root-run1.log`, this cycle) | of the `lib`+`lib test` units only -- see Discovery 1 |
| Root hard error, isolated | `error: invisible character detected`, `src/rules_core/rules_tables/monster_codex/spell_list.rs:83:134`, `#[deny(clippy::invisible_characters)]` | same log, `grep -n invisible` | 1 of 1 |
| Root, AFTER this cycle | **0 errors, 0 warnings** | `scripts/verify.sh --only clippy` -> `PASS clippy (root:0 desktop:0 warnings, 0 errors)` | of the FULL population (lib + lib-test + all 543 `tests/*.rs` + every `src/bin/*.rs` binary's own test build) -- first time this stage has ever reached that population, see Discovery 1 |
| Desktop, BEFORE this cycle (same rebased HEAD, before any fix) | 0 errors, 12 warnings | `cd apps/desktop/src-tauri && CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001 cargo clippy --locked --tests` (log: `/tmp/clippy-desktop-run1.log`) | of the desktop crate's own `--tests` scope |
| Desktop, AFTER this cycle | **0 errors, 0 warnings** | same `scripts/verify.sh --only clippy` run as above | of the same scope |
| `cargo clippy --fix --tests --allow-dirty` (root), mechanical fixes applied | 60 across 24 files (first pass) | `/tmp/clippy-root-fix1.log`, `grep -c Fixed` counts fix *lines*, not fix *count* per line -- literal per-file counts are in that log | N/A (tool-applied) |
| `cargo clippy --fix --tests --allow-dirty` (desktop), mechanical fixes applied | 3 across 2 files | `/tmp/clippy-desktop-fix1.log` | N/A (tool-applied) |
| Hand-fixed warnings, root (post `--fix`, before hand edits) | 46 (7 `claim_blocking`-unused-import warnings emerged from deleting `claim_blocking` itself, fixed in the same pass) | `/tmp/clippy-root-run2.log`/`run3.log`/`run4.log`, the three iterative re-measurements | of 46 |
| Hand-fixed warnings, desktop (post `--fix`, before hand edits) | 9 | `/tmp/clippy-desktop-run2.log` | of 9 |
| Dead functions deleted, root | 2 (`hunter_animal_focus_bull_bonus`, `resolve_pool_member_sole_magnitude`) + 3 dedicated tests that existed only to cover them | `git diff --stat -- src/rules_core/pilot_compute/mod.rs` (this cycle's own working-tree diff) | of the 2 `function ... is never used` warnings named in the BEFORE breakdown |
| Dead test helper deleted, root | 1 function (`claim_blocking`), copy-pasted into 7 files, called nowhere in any of them | `grep -rn "claim_blocking(" tests/sd13_barbarian_level{2..8}_progression.rs` (this cycle, before deletion) -> 0 hits found even before deleting the definitions, confirming dead | of 7 files |
| `cargo test --locked --no-run`, workspace | exit 0, 589 test binaries built | `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001 cargo test --locked --no-run`, at HEAD (post this cycle's commit, SHA below) | N/A (build check) |
| `cargo test --locked --no-run`, `apps/desktop/src-tauri` (separate workspace) | exit 0, 1 test binary built | same command, run explicitly from that directory, same HEAD | N/A (build check) |
| `BASELINE_CLIPPY_WARNINGS_ROOT`/`DESKTOP` ceilings | 50/7 -> **0/0** | `scripts/verify-baselines.env` tail, this cycle's own deliberate edit | ceiling tightened per the gate's own "loose" note, not a raise |

## Row-count command output (this cycle's own artifact -- the authoritative gate stage)

```
$ CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001 scripts/verify.sh --only clippy --show-actuals
==> clippy — cargo clippy --locked --tests -j 2  (BOTH crates)
    PASS  clippy  (root:0 desktop:0 warnings, 0 errors)
---------------------------------------------------------------
SUMMARY
  passed:  1  clippy

MEASURED (scripts/verify-baselines.env format):
  BASELINE_CLIPPY_WARNINGS_ROOT=0
  BASELINE_CLIPPY_WARNINGS_DESKTOP=0

RESULT: PASS
```

Both crates report `0 warnings, 0 errors`, both ceilings updated to match (0/0), and the stage's
own `RESULT: PASS`. Status set from this count, per `decisions.md §4`.

## Build scope verified

- `cargo test --locked --no-run` (workspace): **exit 0**, 589 test binaries linked.
- `apps/desktop/src-tauri` (separate cargo workspace): **exit 0**, `cargo test --locked --no-run`
  run explicitly from that directory.
- Both runs at HEAD = this cycle's own last figure-moving commit (SHA below); no later commit in
  this cycle touches Rust source after these runs.

## Sweep population

N/A -- this cycle touched no `data/corpus/**` file; `corpus_literal_sweep` not re-run (nothing in
the diff can move its examined-population). `BASELINE_CORPUS_LITERAL_RECORDS` (48708) is
unchanged and was NOT this cycle's to touch -- it was already landed by the wave-23 gate-lane
closing sweep (`scripts/verify-baselines.env`, the "SD-34 gate-remediation closing sweep
(2026-09-01)" block, dated before this cycle) before this cycle's dispatch brief's own stretch
goal was reached; see Discoveries.

## Oracle pin

N/A -- no figure in this cycle rests on the pinned oracle corpus.

## RED -> GREEN evidence (TDD)

This is a warning-remediation/type-fix cycle, not a new-behavior cycle, so RED/GREEN is framed as:
the stage's own failing/passing state, not a new test.

**RED (before this cycle):** `scripts/verify.sh --only clippy` failed for the correct reason --
confirmed via the literal pre-fix `cargo clippy --locked --tests` output: root exited 101 on a
real hard `error` (`clippy::invisible_characters`, deny-by-default) with 86 `warning:` lines
alongside it; desktop exited 0 with 12 `warning:` lines (already over the recorded ceiling of 7).
Every one of the 46 root / 9 desktop warnings remaining after the mechanical `--fix` pass was
individually confirmed to still reproduce, then individually fixed and re-confirmed gone by the
next `cargo clippy` run before moving to the next.

**GREEN (after this cycle):** `scripts/verify.sh --only clippy` -> `PASS clippy (root:0 desktop:0
warnings, 0 errors)`. Re-confirmed twice (`/tmp/verify-clippy-only.log` before the ceiling edit,
`/tmp/verify-clippy-only2.log` after -- both `RESULT: PASS`, only the "ceiling is loose" note
disappearing between the two, as expected).

**No lint category was silenced wholesale.** Every `#[allow]` this cycle added (7 total: 6
`#[allow(clippy::too_many_arguments)]` -- 4 in `src/rules_core/pilot_compute/mod.rs`, 2 in
`src/bin/v06_work_inventory.rs` -- plus 1 `#[allow(dead_code)]` in
`apps/desktop/src-tauri/src/character_hub.rs`) is attached to exactly one function/field, carries
its own comment stating why, and none is a crate-level or module-level blanket allow -- verified:
`grep -rn '^#!\[allow' src/ apps/desktop/src-tauri/src/` returns nothing anywhere in either crate
(not just this cycle's diff), and `git diff --unified=0 -- src/ apps/desktop/src-tauri/src/ |
grep -c '^+[[:space:]]*#\[allow'` -> 7, matching exactly. Two OTHER `#[allow(dead_code)]` attributes exist in
`mod.rs` (lines 2971, 3305) -- both pre-existing, confirmed by `git diff` showing zero `+` lines
adding them; this cycle added neither.

## Discoveries

1. **The clippy stage has silently measured only a fraction of its own population since before
   the fable-review baseline.** `src/rules_core/rules_tables/monster_codex/spell_list.rs` (landed
   2026-08-23, SD-32, predates this bundle entirely) carries three literal U+00AD soft-hyphen
   bytes inside a spell-description string, a `#[deny(clippy::invisible_characters)]`-by-default
   hard error. `cargo clippy --locked --tests` aborts the `lib`/`lib test` compilation on this
   error, which means cargo NEVER attempts to compile any of the 543 `tests/*.rs` integration
   targets or the `src/bin/*.rs` binaries' own test builds (they depend on the lib compiling
   first) -- every prior measurement of this stage, including the fable-review.md "86" and the
   wave-23 gate-sweep's re-confirmed "86", was unknowingly scoped to `lib`+`lib test` only. Fixed
   (3 soft hyphens -> regular hyphens, matching the surrounding text's own existing hyphen style
   exactly, verbatim quote content otherwise byte-for-byte unchanged) as the first action this
   cycle, which is what let `cargo clippy` reach the full population for the first time. Filed as
   a `correction` retro event (`docs/retro/events/sd34-at-34-e6-001.jsonl`, id
   `1788287227716-sd34-at-34-e6-001-751da6`) -- the "86" the dispatch brief quoted was
   real and reproducible, but was never the true population's warning count; there was no way to
   know that without fixing the blocking error first.
2. **Desktop's pre-cycle warning count (12) was already well under the review's stale 25**,
   entirely from wave 24's own already-landed desktop-crate fixes (commit `e36eacb224`,
   "desktop crate fully GREEN (572/0)") -- this cycle's own contribution to desktop is closing the
   remaining 12, not all 25.
3. **`large_enum_variant` needed two rounds of boxing, not one.** Boxing only `corpus_derived`
   (matching the pre-existing precedent comment for `summary`) reduced `CreateCharacterResponse::
   Saved` from ~504 to 248 bytes -- still over clippy's threshold against `Blocked`'s 24 bytes.
   `PilotSnapshotDto` (unboxed) was the remaining bulk; boxing `snapshot` too (across all three
   affected response enums, matching every construction/destructure site compiler-error by
   compiler-error) closed all three `large_enum_variant` warnings to zero. Each fix step was
   confirmed correct by recompiling before moving to the next -- 3 rounds of real `E0308` type
   errors surfaced and fixed this way (8 sites for `corpus_derived`, 8 sites for `snapshot`),
   never guessed at.
4. **Two module constants were confirmed test-only, not deleted.** `HUNTER_ANIMAL_FOCUS_BULL_
   SELECTION_ID` and `EMPOWER_SPELL_METAMAGIC_SELECTION` are each referenced only inside their own
   `#[cfg(test)]` fixtures (confirmed by a crate-wide grep before touching either) -- gated
   `#[cfg(test)]` at their definitions is the real fix (the constant genuinely has no production
   reader), not `#[allow(dead_code)]`, which would have hidden a real scoping fact.
5. **Two functions were confirmed genuinely superseded before deletion**, not assumed dead from
   the lint alone: `hunter_animal_focus_bull_bonus` -- production code already calls the generic
   `hunter_animal_focus_tiered_bonus(option, level)` directly over the shared options table (grep
   confirmed, line ~26931 area), bypassing the named wrapper entirely; `resolve_pool_member_sole_
   magnitude` -- `git log -S` on its own call-site pattern shows the SD-32 "cycle 20" commit that
   introduced `resolve_pool_member_all_magnitudes` is exactly where its last production call site
   was removed, and a crate-wide grep outside `mod.rs` found only doc-comment mentions, zero real
   calls. Both functions' own dedicated tests (which existed only to cover the now-dead function,
   confirmed redundant against a parallel table-driven test that already covers the same cases via
   the real production path) were deleted alongside them, not left to bit-rot.
6. **One `#[allow(dead_code)]` was judged the correct disposition, not a shortcut.**
   `SavedCharacterMutationOpDescriptor.op` (desktop) was introduced in its own original SD-23
   feature commit (`git log -S`, one hit, the introducing commit) -- not a bloat-removal orphan --
   and its own struct doc already describes it as "a dispatch-shape test asserts against," meaning
   a real consumer (a test) was anticipated but never written. Deleting the field would destroy
   real self-documentation (each table row's own explicit correspondence to its
   `SavedCharacterMutationOp` variant); the narrow, commented allow preserves it without silencing
   anything the lint would otherwise catch elsewhere.
7. **`BASELINE_CORPUS_LITERAL_RECORDS` 26500 -> 48708 (the brief's stretch goal) was already
   landed** by the wave-23 gate-remediation closing sweep, confirmed by reading `scripts/
   verify-baselines.env`'s own tail before touching anything -- no action needed from this cycle.
   Reported per `workflow-instruction.md`'s "where the repo and this brief disagree, the repo
   wins" instruction.

## Status: complete

Both root and desktop clippy exit 0 warnings / 0 errors, re-verified via `scripts/verify.sh --only
clippy` (the authoritative gate stage, not just the narrower `cargo clippy` CLI), with the widest
build scope (workspace + separate desktop crate `cargo test --locked --no-run`) both exit 0. Both
ceilings tightened to match the true, reproducible count (0/0), never raised.

## Movement, four buckets

- **Closure:** 0 (gate-remediation lane, not a content-completion cycle; moves no
  `docs/work-inventory.json` bucket).
- **Reclassification:** 0.
- **Reachability:** 0.
- **Instrument-correction:** 2 -- (a) the `clippy` stage now measures its TRUE population for the
  first time in this bundle's history (previously silently truncated by a hard compile error that
  predates SD-34 entirely); (b) `scripts/verify-baselines.env`'s clippy ceilings corrected from a
  stale 50/7 to the real, reproducible 0/0.

## Notes

- **Judgment call:** `#[allow(clippy::too_many_arguments)]` (6 uses) rather than refactoring each
  flagged function's parameters into a struct -- every flagged function's parameters are real,
  independently-varying domain inputs (character input, level, ability modifiers, choice-set
  identity, output sink), and a signature refactor touching a ~80k-line compute-engine file's
  public(crate) call sites is a speculative refactor outside this clippy-remediation cycle's scope
  (`AGENTS.md` rule 3). Each allow is single-function, individually commented, confirmed not
  crate-level.
- **Judgment call:** `#[allow(dead_code)]` (1 use, `character_hub.rs`'s `op` field) rather than
  deletion -- see Discovery 6.
- **Judgment call:** two module constants gated `#[cfg(test)]` rather than either deleted or
  `#[allow]`-silenced -- the correct disposition is neither: they are real, in-scope test fixtures
  with zero production readers, and `#[cfg(test)]` states that fact rather than hiding it.
- **Observed, not fixed (out of this lane's scope):** `scripts/verify.sh --only figure-provenance`
  reports 1 violation, `artifacts/epic-6-closure/AT-34-E6-001_gate-lane-a_wave24_cycle_receipt.md:
  144` (an unsourced `sd27_equipment...` price-classification tuple) -- confirmed pre-existing via
  `git log --oneline -1 -- <that file>` (`de91cfb1e9`, wave 24, not touched by this cycle) and
  confirmed this cycle's own new receipt introduces zero new violations (`denominator-gate`:
  `files_checked=138 violations=0`). A different lane's content, a different stage than this
  dispatch's own clippy scope -- named here rather than silently fixed, per `AGENTS.md` rule 3
  ("do not expand scope... if broader changes appear necessary, stop and explain why").
- Followed `workflow-instruction.md §5`'s concurrent-write protocol for the shared files
  (`progress.md`, `kanban.md`); re-read both immediately before editing; `kanban.md` row 26 left
  untouched (this cycle's own status recorded in `progress.md` only, per the filename note).

## Next-cycle plan

None named for clippy itself -- both stages are green at 0/0 and this lane's scope is exhausted.
Two staleness notes filed for whichever cycle owns them next (neither blocks this criterion):
`BASELINE_ROOT_TEST_BINARIES` (569 recorded, 589 measured by this cycle's own build-scope check,
untouched -- out of scope) and `BASELINE_ROOT_LIB_TESTS`/`BASELINE_ROOT_FULL_TESTS` (unchanged
since the wave-23 gate-sweep already flagged `BASELINE_ROOT_LIB_TESTS` as stale at 2336 vs a
measured 3022, also untouched by this cycle). The final-acceptance scan
(`AT-34-E6-001_cycle_receipt.md`, a separate criterion) should re-verify `scripts/verify.sh --only
clippy` reports `root:0 desktop:0 warnings, 0 errors` as part of its own re-derivation, per
`acceptance-and-verification.md §3` obligation 2.

## Commit SHA (filled in after push)

`9d2e7d9e28` -- the single commit this cycle produced, pushed to `tranche/14` before this receipt
update. Nothing landed on Rust source after this commit, so it is also this cycle's own
build-scope SHA.
