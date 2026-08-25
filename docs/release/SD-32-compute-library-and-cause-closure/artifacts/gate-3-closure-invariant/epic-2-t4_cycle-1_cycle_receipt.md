# Cycle 1 — Epic 2 (cause closure) / Card 11 `epic-2-cause-closure`, lane T4

- **Card ID:** `epic-2-cause-closure` (T4 lane — one of six concurrent lanes closing card 11's
  remaining blocker shapes; do not set the card row to `complete` from this lane alone)
- **Commit SHA:** `4911a9b33`
- **Files touched:**
  - `apps/desktop/src/characterHub/classFeaturesModel.ts` (new export
    `unmatchedClassFeatureDescriptions`)
  - `apps/desktop/src/characterHub/classFeaturesModel.test.ts` (4 new tests)
  - `apps/desktop/src/characterHub/CharacterSheet.tsx` (new component
    `ClassFeatureDescriptionReferenceSection`, wired into `ActionsTab`'s render and its
    empty-state check)
  - `apps/desktop/package-lock.json` (incidental — `npm install` synced the lockfile's `version`
    field to `package.json`'s already-bumped `0.12.0`; the lockfile was still `0.11.0`. Unrelated
    to T4, harmless, left in rather than reverted.)
  - `docs/retro/events/epic-2-t4.jsonl` (new — 1 correction; also carries an auto-logged
    `preflight-oracle` verification event from the fresh-worktree oracle bootstrap)
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (card 11 lane note)
  - `docs/release/SD-32-compute-library-and-cause-closure/progress.md` (this cycle's entry)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion:** `acceptance-and-verification.md` AT-32-E2-001 — cause closure by class,
  T4 lane: "Built-but-unreachable render surface." `epic-breakdown.md` Epic 2's T4 row: "up to
  **2,763**... L9's 471 had a true reachable count of zero — a lane filed on-screen evidence that
  was false. Verify with the real driver."
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
  `PCGEN_ORACLE_SHA`) — the underlying `class_feature` cache this population reads
  (`src/rules_core/cache_gen/class_feature.rs`) is itself corpus-derived from this pin; no new
  numeric value is computed by this cycle's own change (see "Fixture discipline" note below).
- **Status:** complete (T4 lane fully closed)
- **Notes:**

  **Re-derivation first, per the anti-gaming bar.** T4's own MEASURE-TWICE.md row
  (`SD-31-corpus-closure-grind/artifacts/MEASURE-TWICE.md` line 91) names two disjoint
  populations under this shape: **L8** ("up to 2,763", `class_feature_descriptions.rs`'s served
  catalog — the number `epic-breakdown.md`'s T4 row carries forward) and **L9** (471,
  `class_feature_feat_bridge.rs`, already separately measured as 0-of-471 truly reachable). This
  cycle's scope, per the dispatch brief, is the **2,763** figure — L8.

  Re-derived by temporary instrumentation (`eprintln!` inside
  `load_class_feature_descriptions`'s own existing test
  `loads_thousands_of_real_described_class_features_from_the_live_corpus`, reverted immediately
  after measuring — `git diff --stat -- apps/desktop/src-tauri/src/class_feature_descriptions.rs`
  is empty at HEAD):
  ```
  CARGO_TARGET_DIR=<claimed dir> cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml \
    --bin codex-desktop loads_thousands_of_real_described_class_features_from_the_live_corpus -- --nocapture
  ```
  Result: **`SD32_T4_REDERIVE_COUNT=6975`**, not 2,763. **This is a real, material correction, not
  measurement noise** — logged via `scripts/retro.py correction`
  (`docs/retro/events/epic-2-t4.jsonl`, `--claimed-value 2763 --actual-value 6975`). Cause: L8's
  reader (`class_feature_descriptions.rs::load_class_feature_descriptions`) walks
  `data/corpus/*/class_feature/**/*.json` directly with **no `RuleSetId` gate at all** — unlike
  the doneness classifier, it does not check whether a book is registered. Card 4's own SD-32
  cycle (Epic 4, Gate 0) landed `RuleSetId`s for `inner_sea_magic`/`inner_sea_temples`/
  `inner_sea_taverns`/`inner_sea_faiths`, and each of those books' `class_feature` cache records
  were already present under `data/corpus/` (written by `cache_gen::class_feature`, upstream of
  and independent from the `RuleSetId` gate) — so this catalog silently grew by thousands of
  records across the life of this bundle, unnoticed because nothing before this cycle re-ran the
  measurement. **6,975 is the number this cycle closes against**, not 2,763.

  **Root cause and the fix site — traced, not guessed.** `class_feature_descriptions.rs`'s own
  module doc comment names the defect precisely: `ClassFeatureRow.detail` renders the engine's
  computed derivation; `corpusDescription` (the real rulebook `DESC:` text) is a SECOND, additive
  field, but `buildClassFeatureSurface` (`classFeaturesModel.ts`) only ever creates a row by
  iterating `ExplanationDto[]` — `descriptions` is consulted ONLY as enrichment on a row an
  explanation already created (`findCorpusDescription`, called inside the `features` loop only).
  A `class_feature` corpus record with a real, verified description but **no matching engine
  explanation** (not grounded, not even `.unsupported` — `buildClassFeatureSurface` never attaches
  `corpusDescription` inside the `notComputed` loop either) reaches **no code path a player's
  screen renders**, regardless of how many such records exist. This is confirmed, not assumed, by
  the module's own pre-existing regression test,
  `classFeaturesModel.test.ts::verifiesADescriptionWithNoMatchingExplanationProducesNoRowAtAllRegardlessOfHowManyDescriptionsExist`
  (SD-31 wave 29), which this cycle leaves passing unchanged — `buildClassFeatureSurface` itself is
  correctly untouched; the fix is a NEW surface, not a change to that one. The doc comment
  explicitly names the missing piece: unlike `list_class_feature_pool_options` →
  `ClassFeaturePoolReferenceSection` (`CharacterSheet.tsx`), which renders its full catalog
  independent of held explanations, **no equivalent standalone browsable surface existed** for
  `list_class_feature_descriptions`. `class_feature_feat_bridge.rs`'s population (L9, 471 units,
  concatenated into the SAME `classFeatureDescriptions` array `CharacterSheet.tsx` already builds
  before calling `buildClassFeatureSurface`) shares the identical defect shape and is fed through
  the identical join.

  **Closed by class, not by instance — the fix, and why it covers the whole population.**
  1. **`classFeaturesModel.ts`**: new export `unmatchedClassFeatureDescriptions(explanations,
     heldClasses, descriptions)`. Filters `descriptions` to the character's held classes
     (`classSlug`), then excludes any description whose `(classSlug, featureSlug)` matches a
     **grounded** (non-`.unsupported`) explanation id via the SAME `matchesCorpusFeature` join
     `findCorpusDescription` already uses — reusing the adjudicated join, not inventing a second
     one (the same discipline the module's own doc comment already commits to). A description
     matching only an `.unsupported` id is deliberately NOT excluded: `buildClassFeatureSurface`
     never attaches a description to a `notComputed` row either, so that record is still
     unreachable and must still surface here (locked in by
     `verifiesADescriptionMatchingOnlyAnUnsupportedNoticeStillSurfaces`, below).
  2. **`CharacterSheet.tsx`**: new component `ClassFeatureDescriptionReferenceSection`, modelled
     directly on `ClassFeaturePoolReferenceSection`'s own browsable-reference shape (shown per
     held class, independent of the engine's per-character explanation set) — but **data-driven
     off `props.heldClasses`** rather than a hand-maintained per-class array (`POOL_REFERENCE_SECTIONS`),
     so it covers every class corpus-wide with no new entry required per class, present or future.
     Wired into `ActionsTab`'s render (after the pool-reference section) and into its empty-state
     check, so a build whose ONLY class-feature content is an unmatched description no longer
     falsely reports "No class features granted yet."
  3. **Corpus-wide coverage, not a sample.** Because the new component reads the SAME
     `classFeatureDescriptions` array `ActionsTab` already assembles
     (`[...loadClassFeatureDescriptions(), ...loadClassFeatureFeatBridgeDescriptions()]`) and gates
     only on "does the character hold this class," it covers every one of the 6,975 L8 records for
     any class a character can hold — not a hand-picked subset. **L9's 471 records are NOT closed
     by this fix**: their `classSlug` is a synthetic pool-group name (e.g.
     `golden_legionnaire`), not a real class token, so `heldTokens.has(d.classSlug)` correctly
     never matches — confirmed by the prior SD-31 wave-29 finding that only 1 of 471 group slugs is
     even a holdable class token. L9 needs a DIFFERENT reachability mechanism (gated on a held
     feat, not a held class) and is out of this lane's scope (T4's own headline figure, 2,763/6,975,
     was always L8 alone, per `MEASURE-TWICE.md`'s own two-line breakdown). Flagged in `kanban.md`'s
     card 11 lane note so a future cycle does not assume T4 fully closed both populations.

  **RED → GREEN.** Added 4 tests to `classFeaturesModel.test.ts`:
  `verifiesAnUnmatchedDescriptionForAHeldClassIsReturnedByTheReferenceSurface`,
  `verifiesADescriptionAlreadyAttachedToAGroundedRowIsNotDuplicatedInTheReferenceSurface`,
  `verifiesADescriptionMatchingOnlyAnUnsupportedNoticeStillSurfaces`,
  `verifiesADescriptionForAnUnheldClassNeverSurfaces`. Mutation proof: temporarily replaced
  `unmatchedClassFeatureDescriptions`'s body with `return [];`, re-ran
  `node --import tsx apps/desktop/src/characterHub/classFeaturesModel.test.ts` — failed for the
  intended reason (`"a real corpus description for a held class with no explanation must surface:
  expected 1, got 0"`); reverted (`git diff --stat` empty for the intermediate state), re-ran
  clean (exit 0). Full frontend suite: `node apps/desktop/scripts/run-tests.mjs` →
  **97/100 test files passed**, unchanged from before this cycle's own change — the 3 pre-existing
  failures (`release/buildVersionTriple.test.ts`, `releaseChecks/buildLabelFixtureFreshness.test.ts`,
  `releaseChecks/buildVersionTriple.test.ts`) are a stale `Cargo.toml` version (`0.11.0` vs the
  bundle's `0.12.0`), confirmed pre-existing and unrelated by direct read of the failure
  (`Cargo.toml version must match package.json version: expected 0.12.0, got 0.11.0`) — not
  touched by this cycle, not this lane's scope. `npm run typecheck` (`tsc --noEmit`): clean, 0
  errors.

  **Fixture discipline (`decisions.md §3`) — not applicable, and why.** This cycle computes no new
  interpreted value; it makes an already-verified, already-PI-screened, already-leak-checked
  corpus **text** field reachable on screen. `class_feature_descriptions.rs`'s own module doc
  comment records that PI screening and leak-checking are already discharged upstream
  (`cache_gen::class_feature::generate`'s `§52.3`/`§53.5` contracts,
  `render_pcgen_desc`+`leaked_pcgen_syntax`); this is the same posture `race_trait_picker.rs` and
  `monster_catalog::serve_ability_description` already hold for their own corpus-text (not
  corpus-value) render paths, neither of which carries a `derived_evaluator_fixture_check` fixture
  either. Ruling §20's fixture requirement is scoped to *interpreted magnitudes*; this cycle emits
  no magnitude.

  **Ruling §18 — checked, not gated on.** `levers.md` L8's own text flags the 2,763(now 6,975)
  ceiling as "gated further behind Ruling §18's OPEN/EXCLUSIVE pool split for the ~1,656 G1-shaped
  units inside it." Checked against `decisions.md §7`'s carried-forward open rulings (B1/B2/B4/B5)
  and `risks-and-open-questions.md`: **Ruling §18 is not among SD-32's open rulings** and is not
  cited anywhere in this bundle's own acceptance criteria (`AT-32-E2-001` names no such gate for
  T4). It is already in force elsewhere in this codebase (`class_feature_pool_catalog.rs`'s
  `is_archetype_locked()`, per card 11 cycle-1's own T1 note) — this cycle changes nothing about
  which records `class_feature_descriptions.rs` itself serves (that catalog's own filtering is
  untouched, diff-verified empty above), so it inherits whatever Ruling §18 posture that catalog
  already has rather than reopening the question.

- **Discovery forwards:** none requiring a new card — L9 (471 units, feat-bridge population, needs
  a feat-held reachability gate, not a class-held one) is named above and in `kanban.md`'s card 11
  lane note as a residual, scoped item, not a new-scope discovery (it was already named in
  `MEASURE-TWICE.md`'s own T4 row before this cycle).
- **Next-cycle plan:** T4/L8 (6,975 units, re-derived) is closed. If a future cycle wants to close
  L9 too (471 units, disjoint, same MEASURE-TWICE.md T4 row), it needs a feat-held (not class-held)
  reachability gate — a genuinely different mechanism, not an extension of this cycle's fix.
