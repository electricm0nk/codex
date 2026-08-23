# Cycle 1 — Epic 2 (cause closure) / Card 11 `epic-2-cause-closure`, lane T4-L9

- **Card ID:** `epic-2-cause-closure` (T4-L9 lane — one of card 11's five sub-populations
  `decisions.md §13` ruled closed by doing the work; do not set the card row to `complete` from
  this lane alone — four sibling shapes, T2b/T9/T12/T2a-residual, are still open)
- **Commit SHA:** `e8762d846`
- **Files touched:**
  - `apps/desktop/src-tauri/src/class_feature_descriptions.rs` (new DTO field `granted_feat:
    Option<String>`, set `None` at this module's own construction site)
  - `apps/desktop/src-tauri/src/class_feature_feat_bridge.rs` (populates `granted_feat:
    Some(feat_target)`; 1 new test, `every_bridged_record_corpus_wide_carries_its_granted_feat`)
  - `apps/desktop/src/boundary/loadClassFeatureDescriptions.ts` (DTO type gains `grantedFeat:
    string | null`)
  - `apps/desktop/src/characterHub/classFeaturesModel.ts` (`unmatchedClassFeatureDescriptions`
    gains a 4th, optional `selectedFeats` param and a feat-held reachability arm)
  - `apps/desktop/src/characterHub/classFeaturesModel.test.ts` (4 new tests; `descriptionDto` test
    helper gains a `grantedFeat` param)
  - `apps/desktop/src/characterHub/CharacterSheet.tsx` (`ActionsTab` and
    `ClassFeatureDescriptionReferenceSection` thread `selectedFeats` through to the model call;
    `<ActionsTab>`'s own call site now passes `props.detail?.selectedFeats ?? []`)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion:** `acceptance-and-verification.md` AT-32-E2-001 — cause closure by
  class, T4 lane, L9 sub-population: "the rule is *class-closure*, not *instance-closure*."
  `decisions.md §13` row T4-L9: "Needs a **feat-held** reachability gate; today's gate is
  class-held." Consequence 3: "T4-L9 blocks card 11 exactly as a whole card would... a card at
  `complete` with a named, uncleared sub-population is the half-deferral defect card 12 was
  reopened for, reproduced inside card 11."
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
  `PCGEN_ORACLE_SHA`) — the underlying `class_feature` cache this population reads is corpus-derived
  from this pin; this cycle computes no new interpreted magnitude (see "Fixture discipline" below).
- **Status:** complete (T4-L9 sub-population fully closed, corpus-wide)
- **Notes:**

  **Re-derivation first, per the anti-gaming bar.** Re-ran T4-L8's own lane's pinned test before
  touching anything:
  ```
  CARGO_TARGET_DIR=<claimed dir> cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml \
    --bin codex-desktop class_feature_feat_bridge_serves_the_full_corpus_wide_population -- --nocapture
  ```
  Result: **471**, matching `decisions.md §13`'s cited figure and `class_feature_feat_bridge.rs`'s
  own pinned corpus-wide test exactly. **No correction logged** — nothing to correct.

  **Root cause, traced not guessed (T4's own L8 receipt already found it; this cycle acts on the
  finding it deferred).** `class_feature_feat_bridge.rs`'s 471 records exist to bridge a
  `class_feature` record whose sole content is a grant of an already-modelled `feat` — but the
  DTO's `class_slug` is a **synthetic pool-group name** (`slug()` of the corpus `data.class`
  value, e.g. `"golden_legionnaire"` for the Adventurer's Guide prestige-class-shaped group), never
  a real class token the character sheet's `heldClasses` can ever contain. T4-L8's own fix,
  `unmatchedClassFeatureDescriptions`, gates every candidate on `heldTokens.has(d.classSlug)` — a
  check this population can never pass, by construction, confirmed corpus-wide by
  `class_feature_feat_bridge.rs`'s own prior finding that only 1 of 471 group slugs is even a
  holdable class token. **The record reaches no code path a player's screen renders, for any of the
  471** — the identical unreachable-render-surface shape T4-L8 fixed, but a different held-cause.

  **Closed by class, not by instance — the fix, and why it covers the whole population.**
  1. **`ClassFeatureDescriptionDto::granted_feat: Option<String>`** (new field,
     `class_feature_descriptions.rs`). `None` for every L8 record (that module's own construction
     site). `class_feature_feat_bridge.rs` sets `Some(feat_target)` — the EXACT string
     `feat_description_by_exact_name` already matched on to find the feat's own text (not the class
     feature's own `name`, which can differ from the feat name it grants) — carried straight from
     that module's own `sole_feat_grant_target` result, no re-derivation, no re-guessing.
  2. **`unmatchedClassFeatureDescriptions` (`classFeaturesModel.ts`)**: gains a 4th param,
     `selectedFeats: readonly string[] = []`, and `isReachableByHeldCause` becomes a two-arm
     predicate: `d.grantedFeat !== null ? heldFeatIdentities.has(normalizeFeatIdentity(d.grantedFeat))
     : heldTokens.has(d.classSlug)`. **This is a predicate over field presence, not a hand-listed
     set of the 471 keys** — the T8 lane's own precedent (`epic-2-cause-closure_cycle-2_t8_cycle_
     receipt.md`: "this cycle's second revision generalises [a 12-id allowlist] to a predicate")
     followed deliberately: any future `class_feature_feat_bridge.rs` record needs no new entry
     here, it is covered the moment the Rust side sets `granted_feat`.
  3. **The fold is reused, not reinvented.** `normalizeFeatIdentity` (`featsTabModel.ts`) is the
     SAME fold `feat_identity.rs::holds` mirrors on the Rust side — that Rust module's own doc
     comment names the pairing explicitly ("The Feats tab has always folded for display... Effect
     resolution did not [until] this module... Display and effect resolution now fold through this
     single function"). This cycle adds a THIRD consumer of the identical fold rather than writing
     a new comparison, so a future drift between `selectedFeats`' two real shapes (`"Swift Aid"` vs
     `"feat:swift_aid"`) cannot silently break this gate the way it once broke effect resolution.
  4. **Corpus-wide coverage, not a sample.** New Rust test
     `every_bridged_record_corpus_wide_carries_its_granted_feat` proves ALL 471 records this cycle's
     scope covers carry a non-empty `granted_feat` — not the one sampled `Golden Legionnaire ~
     Swift Aid` record `loads_real_bridged_descriptions_from_the_live_corpus` already checks (that
     test gained an assertion on `granted_feat` too, for the same record).
  5. **`CharacterSheet.tsx`** threads `props.detail?.selectedFeats ?? []` through `ActionsTab` to
     both call sites of `unmatchedClassFeatureDescriptions` (the inline `unmatchedDescriptions` and
     `ClassFeatureDescriptionReferenceSection`'s own prop) — the same `selectedFeats` shape
     `FeatsTab` already reads at its own call site a few lines above, no new IPC surface.

  **RED → GREEN, proven twice, both reverted clean.**
  - TS: temporarily reduced `isReachableByHeldCause` to `heldTokens.has(d.classSlug)` (the old,
    class-held-only check) and re-ran `node --import tsx
    src/characterHub/classFeaturesModel.test.ts` — failed for the intended reason: `"a bridge
    record must surface on the feat-held arm even though no held class matches its synthetic
    classSlug: expected 1, got 0"`. Reverted; `diff` against the pre-mutation file empty; re-ran
    clean (exit 0).
  - Rust: temporarily forced `granted_feat: None` in `class_feature_feat_bridge.rs` and re-ran
    `every_bridged_record_corpus_wide_carries_its_granted_feat` — failed for the intended reason:
    `"471 of 471 bridged records carry no granted_feat"`. Reverted; `diff` against the
    pre-mutation file empty; re-ran clean.

  **Suites, both workspaces, plus the audits and the reach gate.**
  - `cargo test --locked --lib` (root workspace): **2388/2388** passing, 13 ignored (unchanged from
    the consolidation cycle's own cited baseline — this lane touches only the `apps/desktop`
    workspace).
  - `cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml` (desktop crate, a
    **separate** cargo workspace — per this bundle's own standing lesson, a root-only sweep would
    have missed this lane's own changes entirely): **517/517** passing (516 at T4-L8's own cycle's
    end, +1 for this cycle's new test).
  - `scripts/verify.sh --only reach`: **PASS (31)** — unaffected; this fix lives entirely in
    `class_feature_descriptions.rs`/`class_feature_feat_bridge.rs`/the frontend model, not
    `reach_gate.rs` itself.
  - Frontend suite (`node apps/desktop/scripts/run-tests.mjs`): **97/100** test files passed —
    unchanged from T4-L8's own cycle; the 3 failures
    (`release/buildVersionTriple.test.ts`, `releaseChecks/buildLabelFixtureFreshness.test.ts`,
    `releaseChecks/buildVersionTriple.test.ts`) are the same pre-existing, unrelated
    `Cargo.toml`-vs-`package.json` version-drift failures T4-L8's receipt already confirmed and
    left untouched.
  - `npm run typecheck` (`tsc --noEmit`): clean, 0 errors.
  - Dual-audit gate, re-run on the final diff (`BASE_BRANCH=$(git merge-base HEAD origin/develop)`,
    scoped to this cycle's own 6 files): `OK_NO_BUNDLE_TAGS`, `OK_NO_TOKENS`.

  **Pinned-count sweep (`workflow-instruction.md §6` step 7).** `grep -rn '\b471\b'` across
  `src/`, `apps/`, `scripts/`, `tests/` found no OTHER file asserting the 471 figure that this
  cycle's change could have made stale — the only pre-existing hardcoded `471` occurrences are
  unrelated `.lst` source-line citations (`monk_features.rs`, `monster_data.rs`, etc.) and
  `v06_work_inventory.rs`'s own unrelated synthetic test fixture line number. This cycle does not
  change the 471 count itself (it makes the existing 471 reachable, not more or fewer of them), so
  no pinned-count file needed updating.

  **Fixture discipline (`decisions.md §3`) — not applicable, and why.** Same posture T4-L8's own
  receipt already established for this exact render-path family: this cycle computes no new
  interpreted magnitude. `granted_feat` is a corpus-derived identity string, already PI-screened
  and leak-checked upstream (`cache_gen::class_feature`'s `§52.3`/`§53.5` contracts, discharged
  before this DTO is ever built; `class_feature_feat_bridge.rs`'s own module doc comment records
  this trust boundary), carried through unchanged from `sole_feat_grant_target`'s own already-
  verified result — no evaluator interprets it, so `derived_evaluator_fixture_check` does not
  apply, the same reasoning T4-L8's own "Fixture discipline" note gives for its own text passthrough.

  **Scope discipline.** Left the reference-list gutter label as-is
  (`ClassFeatureDescriptionReferenceSection`'s `heldLabels.get(description.classSlug) ??
  description.classSlug` fallback, which now renders the raw synthetic slug, e.g.
  `"golden_legionnaire"`, for these records) — a cosmetic label-humanisation improvement, not a
  reachability requirement; the record's real name and description are already legible text on
  screen, which is the acceptance bar this criterion sets. Named here rather than silently
  polished, so a future cycle can pick it up deliberately if wanted.

- **Discovery forwards:** none requiring a new card.
- **Next-cycle plan:** T4 (both L8 and L9) is now fully closed. Card 11 still carries four open
  sub-populations needing an operator ruling or further work per `decisions.md §13`: T2b (2,472),
  T9 (2,712), T12 (2,453), and T2a's residual (~2,775). A consolidation cycle owns re-checking the
  row's overall status once all five are addressed — this lane does not touch `kanban.md` row 11's
  `Status` column beyond its own lane note.
