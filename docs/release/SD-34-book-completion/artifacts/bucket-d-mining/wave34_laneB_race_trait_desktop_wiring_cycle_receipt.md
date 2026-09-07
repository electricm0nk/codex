# Cycle — SD-34 wave 34, Lane B — wire `adoptedRaceOptions`/`adoptiveParentageOptions` into the desktop UI

**Status: complete.** Closes wave 33 lane B's own next-cycle plan item 1 (`docs/release/
SD-34-book-completion/artifacts/bucket-d-mining/wave33_laneB_race_trait_never_applies_cycle_receipt.md`):
the 27-unit "engine resolves real content, no desktop UI surface reads it" remainder (20
`Adopted Race` selectors + 7 `Adoptive Parentage` options). The Rust resolver chain
(`race_resolver::adopted_race_choose_selectors`/`adoptive_parentage_options`,
`trait_pool::resolve_adopted_race_options`) and the Tauri command surface
(`race_trait_picker.rs`'s `list_alternate_racial_traits`) were already real and tested; this
cycle's own work is the desktop TypeScript boundary, a real picker UI section, and — closing
the loop wave 33 lane B could not, since it was scoped "pure frontend wiring, not new backend
mechanism" — the `v06_work_inventory.rs` classify()-instrument update that moves these 27
units from bucket D to DONE. That update was necessary, not optional: nothing in the
instrument re-derives its "no desktop UI surface reads it" evidence live — it is a hardcoded
per-cycle finding — so leaving it stale after shipping the frontend would have made the atlas
silently wrong, exactly the "field name is not its meaning" hazard this bundle's own standing
memory warns about.

## A note on the worktree this cycle started from

This worktree was **not** actually cut from `tranche/14`'s live tip as the dispatch stated —
`git merge-base HEAD tranche/14` landed on `ea2b3396f2` (the tranche/13→develop merge
`tranche/14` itself was cut from), 409 commits behind the real tip (`fc709b3c98` at cycle
start). `scripts/completion_atlas.py` did not exist at that base at all, and
`race_trait_picker.rs` was missing both wave 24 lane B's desktop-crate fix and
`AT-34-E3-001`'s 7 CRB `Adopted Race` selectors + 2 Human Ethnicity findings — i.e. part of
the exact population this cycle's own brief is about. Caught before writing any code
(`python3 scripts/completion_atlas.py --check` → `No such file or directory` at the stale
base). Fixed by committing this cycle's frontend-only WIP (`4af30598bf`, superseded — see
below) and running `git rebase tranche/14` — clean, zero conflicts (`git log --oneline
ea2b3396f2..tranche/14 -- <this cycle's 5 touched TS files>` returns nothing; only
`race_trait_picker.rs`, untouched this cycle, had two intervening commits, both Rust-side and
non-conflicting with anything this cycle wrote). Retro-logged as an `incident`
(`docs/retro/events/sd34-wave34-laneb.jsonl`, `recurrence_key: wrong-base-worktree` — the
same named class `AGENTS.md` item 8 already tracks at 27 prior occurrences).

- **Files touched this cycle:**
  - `apps/desktop/src/boundary/loadAlternateRacialTraits.ts` — declares
    `AdoptiveParentageGrantDto`/`AdoptiveParentageOptionDto`/`AdoptedRaceTraitGrantDto`/
    `AdoptedRaceOptionDto`, and adds `adoptiveParentageOptions`/`adoptedRaceOptions` to
    `AlternateRacialTraitsResponse` — the exact two fields the Rust DTO already served and
    the TS boundary never declared.
  - `apps/desktop/src/raceCatalog/alternateTraitPickerModel.ts` — three new pure view
    functions (`describeAdoptionOptions`, `describeAdoptiveParentageGrants`,
    `describeAdoptedRaceGrants`), following the file's own established pattern: presentation
    only, no re-derivation of anything the backend decided.
  - `apps/desktop/src/raceCatalog/AlternateTraitPicker.tsx` — a new picker UI section (two
    independent single-select pill pickers, one per option kind, each showing the selected
    option's real rendered description/grants below), wired to the two new response fields.
  - `apps/desktop/src/raceCatalog/alternateTraitPickerModel.test.ts` — new tests for the
    three view functions, real corpus sample data (ARG `Drow`, ISR `Oread`, the `Rougarou`
    empty-pool and malformed-token cases) copied from `race_trait_picker.rs`'s own pinned
    tests.
  - `apps/desktop/src/characterHub/alternateTraitSelection.test.ts` — two literal
    `AlternateRacialTraitsResponse` fixtures updated for the two new required fields
    (compile-forced by the interface change; both irrelevant to that file's own scope, so
    both set to `[]`).
  - `src/bin/v06_work_inventory.rs` — the `Kind::RaceTrait` classify() arm's two branches for
    this shape now return `text-complete` (not `engine-does-not-hold`) with new evidence
    strings naming the desktop picker screen; the two RED/GREEN proof tests
    (`an_adopted_race_selector_with_real_pool_grants_reaches_text_complete`,
    `an_adoptive_parentage_option_with_a_real_description_reaches_text_complete`, renamed
    from their wave-33 `_gets_the_precise_ui_gap_evidence` names) updated to prove the new
    state; regression tests (`Skinwalker`, `Oversized Goblin`) unchanged, still pin the old
    behavior for the populations this cycle does not touch.
  - `scripts/completion_atlas.py` — four `BUCKET_DEFINITIONS` citation lines (A/B/C/V) shifted
    by this cycle's own classify() insertion (+4 net lines) re-derived fresh, each verified
    against the actual current file content, never hand-computed from the diff alone.
  - `docs/work-inventory.json`, `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/
    completion-atlas.json` — regenerated via the guarded path (below), never hand-merged.
  - This receipt, `progress.md`.

- **Commit SHA:** `c889e99943` (code + instrument fix + guarded regen; frontend WIP
  `4af30598bf` was folded into it by the pre-implementation rebase, so it does not appear on
  this branch's own history — `c38cbf767b` is the surviving post-rebase frontend commit,
  parent of `c889e99943`). Receipt/progress land in a second, docs-only commit on top, per
  this bundle's own two-commit precedent (wave 33 lane B, `9ebf638f6f` code / `c675d0ad5f`
  docs).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — `git diff --unified=0 tranche/14 --
  apps/desktop/src/boundary/loadAlternateRacialTraits.ts
  apps/desktop/src/raceCatalog/AlternateTraitPicker.tsx
  apps/desktop/src/raceCatalog/alternateTraitPickerModel.ts
  apps/desktop/src/raceCatalog/alternateTraitPickerModel.test.ts
  apps/desktop/src/characterHub/alternateTraitSelection.test.ts src/bin/v06_work_inventory.rs
  scripts/completion_atlas.py ':!**/__tests__/**' ':!**/*.test.*' | grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` finds nothing.
- **Wired-integration audit result:** `OK_NO_TOKENS` — same diff range, `grep -nE
  '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` finds nothing.
- **Acceptance criterion:** *(no `AT-34-E#` card names this specific shape — a bucket-D
  mining next-cycle item, wave 33 lane B's own receipt, "Next-cycle plan" item 1: "wire
  `adoptedRaceOptions`/`adoptiveParentageOptions` into the desktop TypeScript boundary and a
  real picker UI section.")*
- **Build scope verified:** `apps/desktop` (`node_modules/.bin/tsc --noEmit` clean; the
  model/selection `.test.ts` files pass) and `--bin v06_work_inventory` (`cargo test`/`cargo
  clippy`, both clean). Full `cargo test --locked --no-run` at the widest workspace scope
  deferred to wave-end per this wave's own dispatch instruction, run once by a different
  agent after all three lanes land.

## RED → GREEN

- **Frontend (TypeScript), RED confirmed** by running `node_modules/.bin/tsc --noEmit` the
  moment `AlternateRacialTraitsResponse` gained the two required fields, before any fixture
  was updated: `TS2739` at 3 sites (`characterHub/alternateTraitSelection.test.ts` ×2,
  `raceCatalog/alternateTraitPickerModel.test.ts` ×1), each `... is missing the following
  properties from type 'AlternateRacialTraitsResponse': adoptiveParentageOptions,
  adoptedRaceOptions` — the intended reason, a real shape change, not a typo.
- **Frontend, GREEN:** `node_modules/.bin/tsc --noEmit` → clean. `node_modules/.bin/tsx
  src/raceCatalog/alternateTraitPickerModel.test.ts` → `alternateTraitPickerModel: all
  assertions passed`. `node_modules/.bin/tsx src/characterHub/alternateTraitSelection.test.ts`
  → `alternateTraitSelection: all assertions passed`. Full `npm test` (100 test files):
  97/100 pass; the 3 failures (`src/release/buildVersionTriple.test.ts`,
  `src/releaseChecks/buildLabelFixtureFreshness.test.ts`,
  `src/releaseChecks/buildVersionTriple.test.ts`) are pre-existing, unrelated version-triple
  pinning drift (`Cargo.toml` vs `package.json` vs release-notes, `0.11.0` vs `0.13.0`) —
  confirmed by `git diff --stat HEAD` naming none of those three files, nor any file they
  read, among this cycle's changes.
- **Backend (Rust), RED confirmed:** `cargo test --locked --bin v06_work_inventory
  race_trait_grounding_tests:: -j 6` against the classify() fix alone (before renaming the
  two tests) failed both existing `_gets_the_precise_ui_gap_evidence` tests:
  `assertion left != right failed ... left: "text-complete" right: "text-complete"` — the
  exact intended contradiction (the fixed code now returns what the old test asserts must
  never happen).
- **Backend, GREEN:** same scoped run, tests renamed/updated → `test result: ok. 38 passed;
  0 failed`. Full binary suite: `cargo test --locked --bin v06_work_inventory -j 6` →
  `test result: ok. 513 passed; 0 failed`. `cargo clippy --locked --bin v06_work_inventory -j
  6` → clean, 0 warnings.

## Guarded regeneration

```
cargo run --locked --bin corpus_literal_sweep -- --json-out <scratch>/corpus_literal_sweep_report.json --quiet
-> corpus-literal-sweep: CLEAN
cargo run --locked --bin derived_evaluator_fixture_check -- --json-out <scratch>/derived_evaluator_fixture_check_report.json --quiet
-> (no findings)
CORPUS_LITERAL_SWEEP_REPORT=<scratch>/corpus_literal_sweep_report.json \
DERIVED_FIXTURE_CHECK_REPORT=<scratch>/derived_evaluator_fixture_check_report.json \
cargo run --locked --bin v06_work_inventory -j 6
-> docs/work-inventory.json regenerated, 49438 units, generated_at 2026-09-02T20:39:49Z
```

`python3 -c "import json; print(len(json.load(open('docs/work-inventory.json'))['units']))"`
→ **49438** — matches the pre-cycle population exactly (no corpus record added or removed
this cycle, only two `classify()` branches and two test names changed).

**Diff verified unit-by-unit, not trusted from the regen alone:** a fresh Python diff of the
committed pre-cycle `docs/work-inventory.json` against the post-regen file confirms the id
set is unchanged (49438 = 49438) and exactly **27** units differ, every one of them only in
`status`/`evidence` — no other field on any of the 49,438 units moved.

## Movement (four buckets, this cycle)

- **Closure (bucket D → DONE): 27** — 20
  `race_trait_adopted_race_selector_grants_rendered_on_the_desktop_picker_screen` + 7
  `race_trait_adoptive_parentage_option_rendered_on_the_desktop_picker_screen`. `D: 2924 →
  2897` (−27), `DONE: 24994 → 25021` (+27), confirmed by `completion_atlas.py --check`
  before/after this cycle and independently by the unit-level diff above.
- **Reclassification:** 0.
- **Reachability:** 0 — these 27 were already engine-resolved as of wave 33 lane B; this
  cycle moves the SHIPPED-SCREEN fact (`AGENTS.md`'s "twin the player reads"), not the
  resolver fact.
- **Instrument-correction:** the `v06_work_inventory.rs` classify() branch for this shape
  updated to reflect the new true state (the desktop UI now reads both fields) — the same
  kind of correction wave 33 lane B itself made in the opposite direction (a false "done" to
  an honest "not yet"; this cycle, a stale "not yet" to a now-true "done"). Plus four
  `completion_atlas.py` `BUCKET_DEFINITIONS` citation-line corrections, forced by the
  classify() edit's own +4 net line shift.

## Figures (every number, its command, its denominator)

- `docs/work-inventory.json` total population: **49438** —
  `python3 -c "import json; print(len(json.load(open('docs/work-inventory.json'))['units']))"`,
  denominator: whole corpus, unchanged from pre-cycle.
- `completion_atlas.py --check`: `population=49438 buckets=10 unclassified=0 overlap=0`;
  `DONE=25021 A=449 B=11769 C=4173 D=2897 M=4449 V=289 U=202 X=170 Z=19`;
  `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False
  citation_failures=0` — `python3 scripts/completion_atlas.py --check`, denominator: whole
  corpus. Run once immediately after the regen (surfaced the 4 stale citation lines, fixed),
  run again after the fix (clean).
- `denominator_gate.py --check`: `files_checked=156 violations=0` —
  `python3 scripts/denominator_gate.py --check`.
- `denominator_gate.py --check-provenance`: `files_checked=86 figures_examined=128
  violations=0` — `python3 scripts/denominator_gate.py --check-provenance`.
- `grep -rln 'adoptedRaceOptions\|adoptiveParentageOptions' apps/desktop/src` → **5 files**
  (`boundary/loadAlternateRacialTraits.ts`, `raceCatalog/alternateTraitPickerModel.ts`,
  `raceCatalog/alternateTraitPickerModel.test.ts`, `raceCatalog/AlternateTraitPicker.tsx`,
  `characterHub/alternateTraitSelection.test.ts`) — where wave 33 lane B's own identical grep
  found **0**.

## Next-cycle plan

Unchanged from wave 33 lane B's own plan, items 2–4 — this cycle closed item 1 only and did
not touch any of the following five:

1. **20** (Skinwalker `Change Shape`) — scope a TYPE-pool option picker; largest remaining
   sub-population of the original 53, real magnitude, needs both a resolver-side pool
   mechanism and a UI surface.
2. **2+1** (Human Ethnicity, Oversized Goblin) — each needs an operator ruling on whether a
   dedicated, mechanically-inert picker UI is in scope, or whether these stay a named,
   accepted gap.
3. **2+1** (`inner_sea_races`, `Rougarou`) — `Human ~ Tribalistic Languages` needs a
   `TEMPLATE:`-reading mechanism (new); `Suli ~ Trusted Mediator` and `Rougarou`'s selector
   have no project-side remedy at all (upstream data gaps) — named permanently blocked
   pending an upstream PCGen fix, not a to-do.
