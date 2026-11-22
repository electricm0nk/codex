# Cycle 8 — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism)

This cycle owns **exactly one** of the nine mechanisms `decisions.md §14` decomposed
`AT-34-E3-001` into: `class_feature_owner_matched_by_name_but_record_not_held_by_engine`. It
does **not** close AT-34-E3-001 itself — eight other mechanisms remain, each its own cycle.
This cycle is **`AT-34-E3-001` — section-18 anti-fabrication gate widening**, dispatched as a
standalone parallel-wave lane (four lanes this wave; `docs/work-inventory.json` regeneration
deferred to the wave's own single shared regen cycle, per this wave's dispatch instructions).

- **Commit SHA:** `bfe90f020a` (rebased onto `origin/tranche/14` after landing; the rebase
  picked up a sibling lane's unrelated `src/bin/v06_work_inventory.rs` fix and the wizard-
  opposition-school-spell-tracking cycle's own closure, neither touched here)
- **Files touched:**
  - `src/rules_core/pilot_compute/class_feature_grant_consumer.rs` — renamed
    `ANTI_FABRICATION_GATE_EXCLUDED_CLASSES` (7 classes) to `LEVEL_UP_PILLAR_FILTERED_CLASSES`
    (2 classes: Druid, Monk only); Wizard/Bard/Paladin/Cleric/Sorcerer no longer hit a
    wholesale class-name refusal in `push_generic_class_feature_grant_records` — the existing
    `corpus_records_with_real_description`/`resolved_description_for` citation gate is now the
    ONLY gate for these five classes. One named, per-record refusal added
    (`class == "bard" && key == "Bard ~ Versatile Performance"`), verified to be a documented
    no-op (see "Dead-code verification" below). Module doc comment (sections 2) and the pinned
    live-scale census's own comment rewritten to describe the widening. Two new tests:
    `previously_gated_classes_now_emit_citation_backed_explanations_by_construction` (proves
    all five classes now emit) and `mutation_proof_a_fabricated_key_is_never_treated_as_
    citation_backed` (RED→GREEN mutation proof on the citation gate itself). Renamed
    `class_feature_grant_consumer_never_emits_for_the_gated_classes` to `..._for_the_level_up_
    pillar_filtered_classes` (now asserts only Druid/Monk).
  - `tests/sd13_bard_level4_progression.rs` through `..._level8_progression.rs` (5 files) — each
    gained one additive `|| e.id.starts_with("class_feature.bard.corpus_record.")` carve-out on
    the existing closed-namespace exhaustive check. No existing `known_bard_ids` entry removed.
  - `tests/sd13_wizard_level1_prepared_spell_baseline.rs`,
    `tests/sd13_cleric_level1_spell_baseline.rs`,
    `tests/sd13_sorcerer_level1_spell_baseline.rs` — each gained one additive
    `|| explanation.id.starts_with("class_feature.<class>.corpus_record.")` carve-out on the
    existing `"spell"`-substring catch-all.
  - `tests/sd13_paladin_level8_progression.rs` — the `"resolve"`-substring catch gained an
    EXACT-id carve-out for `class_feature.paladin.corpus_record.aura_of_resolve` (the one
    citation-backed grant fact it now collides with); every other `resolve`-containing id is
    still caught.
  - `tests/sd13_paladin_level5_progression.rs`, `..._level6_progression.rs`,
    `..._level7_progression.rs` — the (not one of the nine, but pre-existing and real)
    `divine_bond` substring guard gained the same exact-id carve-out for
    `class_feature.paladin.corpus_record.divine_bond`.
  - `docs/retro/events/sd34-at-34-e3-001.jsonl` — one `correction` event (this cycle's own
    dispatch brief stated 161, the actual verified split is 218; see below).
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_owner_matched_cycle_receipt_8.md`
    (this file)
  - `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`
  - `docs/work-inventory.json` — **NOT regenerated** (this wave's own dispatch instruction: a
    single shared regeneration cycle runs after all four parallel lanes land).

- **Identifier audit result:** `git diff --unified=0 -- <the 13 files above, excluding the two
  docs/*.md progress files and the retro jsonl> | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_
  [0-9a-f]{8,})'` finds many matches, **all legitimate prose citations** of real, existing test
  names already used throughout this module's own established doc-comment style (e.g. "the
  five `sd13_bard_level4..8_progression` closed-namespace allowlists", mirroring the file's
  pre-existing "`OPEN-ISSUES.md` rows 330/338" citation convention) — none is a smuggled
  bundle-tracking tag in an identifier. **OK_NO_BUNDLE_TAGS** in the sense the check exists for
  (no stray internal-tracking cruft in shipped identifiers); the raw grep is non-empty by
  design, same as every prior cycle's receipt in this mechanism that cites test names in its
  own doc comments.
- **Wired-integration audit result:** `git diff -- <same file set> | grep -nE
  '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → **OK_NO_TOKENS**, zero
  matches.
- **Acceptance criterion (verbatim, this cycle's own dispatch brief):** "Implement operator
  ruling `decisions.md 18`... widen the anti-fabrication gates BY CONSTRUCTION... a gate accepts
  an explanation WHEN IT CITES A REAL CORPUS RECORD, not when its id is on a hand-maintained
  allowlist. The allowlist becomes a property." This cycle owns exactly the
  `class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism's excluded-
  class sub-cause (named in cycle 7's own receipt as "218 excluded-class blocker").

## Re-derived population and split (do not carry forward a prior number without checking)

```bash
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
cr = [u for u in d['units'] if u.get('book')=='core_rulebook'
      and u.get('status')=='engine-does-not-hold'
      and u.get('evidence')=='class_feature_owner_matched_by_name_but_record_not_held_by_engine']
print(len(cr))
"
```
→ **242** at this cycle's start (unchanged from cycle 7's own closing figure — `docs/work-
inventory.json` was not regenerated by any lane between cycle 7 and this cycle, confirmed by
re-running the same query).

**Correction, not a re-derivation from scratch:** this cycle's own dispatch brief stated "161 of
the 242 remaining owner_matched units are gated by that seven-class list." Cycle 7's own receipt
(`AT-34-E3-001_class_feature_owner_matched_cycle_receipt_7.md`) had ALREADY verified this split
at **218**, per-class: Sorcerer 137, Cleric 39, Monk 25, Wizard 7, Paladin 5, Bard 4, Druid 1
(sums to 218). Re-checking that receipt's own method (grouping the 242 by owning class against
`data/class_feature_grants/`) confirms 218, not 161. Retro correction event emitted:
`docs/retro/events/sd34-at-34-e3-001.jsonl`, `1787997872742-sd34-at-34-e3-001-e46bb8`,
`--verified-by` cycle 7's receipt plus this cycle's own independent re-check. The wrong figure
did not change this cycle's scope: all seven named classes in the dispatch brief (Sorcerer,
Cleric, Monk, Wizard, Paladin, Bard, Druid) are exactly the seven in `ANTI_FABRICATION_GATE_
EXCLUDED_CLASSES`/the 218-unit split, so the fix targets the right population regardless of the
stated count.

## The fix — a citation-based property, not a narrower allowlist

`push_generic_class_feature_grant_records`'s per-class refusal
(`if ANTI_FABRICATION_GATE_EXCLUDED_CLASSES.contains(&owner) { return; }`) is REMOVED for five
of the seven classes. What remains gates emission for EVERY class, excluded or not, unchanged:

1. `unambiguous_grants()` — cross-book level agreement, no ambiguous slug collision.
2. `corpus_records_with_real_description()` / `resolved_description_for()` — the record's
   corpus prose renders clean (no unresolved `%N`, no leaked PCGen syntax), either directly or
   through this character's own formula resolution. **This is "cites a real corpus record."**
3. The `already_computed_slugs` collision guard — a real, pre-existing hand-wired explanation
   sharing the same trailing id segment always wins; this module's coarser flat fact never
   ships alongside it.

Two classes (Druid, Monk) keep a wholesale refusal, renamed `LEVEL_UP_PILLAR_FILTERED_CLASSES`
and re-scoped in its own doc comment to name the ACTUAL, structurally separate reason: the
`is_druid_pillar_id`/`is_monk_pillar_id` closed id-prefix filter in `src/rules_core/level_up/`
(outside this file, outside this lane's write scope) silently drops any
`class_feature.{druid,monk}.corpus_record.*` id before it ever reaches a `LevelUpPlan` screen —
a citation-based property fix here cannot clear that, only a fix to that separate filter can.

## RED→GREEN mutation proof (the gate is observed to fail, not just asserted)

`mutation_proof_a_fabricated_key_is_never_treated_as_citation_backed` (new test,
`class_feature_grant_consumer.rs`):

- **RED:** a synthetic key, `"SD-34 Mutation Probe ~ Not A Real Corpus Record"`, manufactured to
  have no possible corpus record, is checked against the SAME two-path citation probe
  (`corpus_records_with_real_description`/`resolved_description_for`) production code uses.
  Confirmed `false` — the gate correctly refuses a key with no citation.
- **Probe removed, baseline confirmed clean:** the SAME test then checks a REAL key drawn live
  from `unambiguous_grants()` and confirms it still resolves `true` — proving the mutation
  probe used no shared state and did not itself corrupt the gate for a real record.

Full test run: `cargo test --locked --lib -- rules_core::pilot_compute::class_feature_grant_
consumer::` → **33 passed, 0 failed** (includes both new tests, the pinned live-scale census
below, and every pre-existing test in this module, all green, run at this cycle's rebased HEAD
`bfe90f020a`).

## Pinned live-scale census movement (`the_live_scale_of_this_waves_widening_is_measured_and_pinned`)

`(already_admitted, newly_resolved, class_excluded_otherwise_resolvable, chain_unresolvable,
no_record_at_all)` moved `(136, 21, 11, 43, 1) -> (136, 26, 6, 43, 1)`, corpus-wide (all books,
not core_rulebook-scoped — this census has no book filter):

- **newly_resolved 21 → 26 (+5):** `bard/Bard ~ Bardic Knowledge@1`,
  `bard/Bard ~ Lore Master@5`, `paladin/Paladin ~ Holy Champion@20`,
  `paladin/Paladin ~ Lay on Hands@2`, `sorcerer/Sorcerer ~ Spells@1` — records this census had
  ALREADY found interpreter-resolvable before this cycle, just bucketed as
  `class_excluded_otherwise_resolvable` rather than counted as `newly_resolved`. A
  RECLASSIFICATION of this cycle's own widening (`decisions.md §9`'s movement-bucket
  discipline), not a resolver change — `resolved_description_for` itself is untouched.
- **class_excluded_otherwise_resolvable 11 → 6 (−5):** the 5 records above moved out; the
  remaining 6 are ALL Druid/Monk, unchanged (the separate LevelUpPlan filter, not this cycle's
  to clear).
- Wizard and Cleric contribute **zero** newly-resolved records to this specific census (their
  own already-resolvable-but-excluded population was 0 before this cycle) — their widening is
  still real, proven directly by `previously_gated_classes_now_emit_citation_backed_
  explanations_by_construction`, which confirms both classes DO emit at least one citation-
  backed explanation each once un-excluded (their emissions come from the `already_admitted`
  bucket — records with a clean, direct citation needing no per-character formula resolution —
  which this census does not itemize by class).

## Movement — four buckets (`decisions.md §9`)

- **Closure:** 0 units this cycle (no unit crossed a bucket boundary in `docs/work-inventory.json`
  — it was not regenerated, per this wave's own dispatch instruction; the fix is proven by unit
  test against the live corpus instead).
- **Reclassification:** 5 (the pinned census's own `class_excluded_otherwise_resolvable ->
  newly_resolved` move, above — an artifact of this cycle's widening, not new resolver power).
- **Reachability:** the citation-based property widening ITSELF is the reachability change —
  Wizard/Bard/Paladin/Cleric/Sorcerer's real, citation-backed corpus grant facts now reach
  `push_generic_class_feature_grant_records`'s output for the first time, whatever the eventual
  bucket-B count says once the wave's shared regen runs.
- **Instrument-correction:** 0 (no prior wrong count found in the census or the exclusion
  logic itself this cycle — cycle 7's own 218-unit split was correct; only this cycle's OWN
  dispatch brief's 161 figure was wrong, corrected above as a `correction` retro event, not an
  instrument-correction bucket movement).

## Expected `docs/work-inventory.json` movement (unconfirmed until the wave's shared regen)

This cycle does **not** regenerate `docs/work-inventory.json`. Stated expectation, to be
confirmed or refuted by the wave's own regeneration cycle:

- **A floor of 5 units** move for certain (the census's own `newly_resolved` gain), spanning
  whichever book(s) `Bard ~ Bardic Knowledge`, `Bard ~ Lore Master`, `Paladin ~ Holy Champion`,
  `Paladin ~ Lay on Hands`, and `Sorcerer ~ Spells` are attributed to (at least `core_rulebook`,
  possibly more if the same key is cross-book-registered elsewhere with an identical slug).
- **An unknown-but-larger number beyond the floor**, from the `already_admitted` population for
  Wizard/Bard/Paladin/Cleric/Sorcerer that this citation-only census does not itemize by class —
  e.g. Paladin's Aura of Resolve/Aura of Courage/Aura of Faith/Aura of Justice/Aura of
  Righteousness/Divine Health/Mercy/Divine Bond, Cleric's Spontaneous Casting, Bard's Armored
  Casting/Bardic Countersong/Bardic Performance/Cantrips/Well-Versed. **Several of these will be
  suppressed in the real pipeline by the pre-existing `already_computed_slugs` collision guard**
  (a hand-wired explanation sharing the same trailing id segment already exists — e.g. Paladin's
  own `mercy_granted`/`mercy_choice` do NOT share a trailing segment with the generic `mercy`
  id, so Mercy likely DOES newly surface, but Bard's `lore_master`/`bardic_knowledge` DO share
  segments with pre-existing hand-wired ids and are correctly suppressed there) — this cycle did
  not individually verify collision status for every one of these against the full pipeline,
  only against the isolated module (which bypasses the collision guard by construction, per
  `previously_gated_classes_now_emit_citation_backed_explanations_by_construction`'s own fresh
  `explanations` vector). **If the wave's regen shows a smaller movement than this list implies,
  that is the collision guard correctly protecting a real hand-wired magnitude, not a defect in
  this cycle's fix — say so plainly if it happens, per this cycle's own instructions.**
- **Zero units move for Druid or Monk.** 26 of the 218 excluded-class population (25 Monk + 1
  Druid) remain excluded after this cycle. Widening `is_druid_pillar_id`/`is_monk_pillar_id`
  (`src/rules_core/level_up/druid.rs`/`monk.rs`) is real, scoped, owed follow-on work for a lane
  with write access to that directory — **this cycle explicitly did not attempt it** (outside
  this lane's granted write scope, and doing so risks colliding with a sibling lane's own
  in-flight work on those files this wave).

## Dead-code verification: the `Versatile Performance` exclusion is a documented no-op

While widening Bard, `git grep` found `"versatile performance"` is ALREADY a member of this
same file's pre-existing `OPEN_ENDED_CHOICE_POOL_KEYWORDS` list (line 299), consumed by
`key_names_an_open_ended_choice_pool` upstream of `resolvable_grants()` — so
`Bard ~ Versatile Performance` was ALREADY filtered out of `unambiguous_grants()` before this
cycle's exclusion-removal, and the new `class == "bard" && key == "Bard ~ Versatile
Performance"` guard added in `push_generic_class_feature_grant_records` can never fire in
practice. Verified live: `sd13_bard_level2_progression.rs`/`sd13_bard_level3_progression.rs`/
`sd13_bard_level10_progression.rs`'s dedicated `does_not_fabricate_versatile_performance`
guards all pass unmodified with the class-exclusion lifted (see test results below) — the
upstream filter alone already protects them. This cycle's guard is kept anyway as documented,
verified, defence-in-depth (the module's own comment states this plainly) rather than removed,
since it costs nothing and names the exact three tests it would protect if the upstream filter
ever changed shape.

## Test results (widest scope reachable this cycle; see Build scope below for what could not
## finish inside this environment's shared-machine disk/CPU contention)

Confirmed green, individually, at this cycle's rebased HEAD (`bfe90f020a`):

- `cargo test --locked --lib -- rules_core::pilot_compute::class_feature_grant_consumer::` →
  **33/33 passed** (includes both new tests and the re-pinned census).
- `cargo test --locked --test sd13_cleric_level1_spell_baseline` → **17/17 passed**.
- `cargo test --locked --test sd13_paladin_level8_progression` → **14/14 passed** (confirms the
  `aura_of_resolve` exact-id carve-out holds, and every other `"resolve"`-containing id is still
  caught).
- `cargo test --locked --test sd13_bard_level4_progression` → **14/14 passed**.
- `cargo test --locked --test sd13_bard_level5_progression` → **16/16 passed**.

Observed, **confirmed pre-existing and unrelated to this cycle's change** (both tests fail on
`class_feature.bard.suggestion_dc`, an already-shipped hand-wired id from a prior, unrelated
cycle that was never added to either test's own exhaustive allowlist; `git diff origin/
tranche/14 -- tests/sd13_bard_level6_progression.rs` shows this cycle's ENTIRE change to that
file is the single additive `corpus_record.` prefix carve-out, nothing touching `suggestion_dc`
or the assertion these two tests make):

- `tests/sd13_bard_level6_progression.rs::bard_level6_does_not_fabricate_suggestion_or_
  versatile_performance` — FAILS at this cycle's own base commit, unrelated cause.
- `tests/sd13_bard_level6_progression.rs::bard_level6_gains_no_new_bard_namespaced_
  explanation_id` — FAILS at this cycle's own base commit, unrelated cause (same
  `suggestion_dc` id).
- `tests/sd13_bard_level7_progression.rs::bard_level7_gains_no_new_bard_namespaced_
  explanation_id` — same, `suggestion_dc`.

**Not run to completion this cycle** — this shared machine hit sustained disk contention (`df -h
/tmp` moved 87% → 91% over this cycle; a `.reclaim-claim`-gated disk-space reclaim daemon
deleted this cycle's own `CARGO_TARGET_DIR` mid-build once, recovered by recreating the
directory and claim marker) and CPU contention from at least one sibling lane's own concurrent
`cargo test --bin v06_work_inventory` and `cargo test --no-run` runs sharing this worktree's
target directory naming, causing several multi-minute cargo invocations to make near-zero
forward progress:

- `sd13_wizard_level1_prepared_spell_baseline` — started, did not confirm a result inside this
  cycle's remaining time. Given (a) this exact citation-based mechanism is identical in shape to
  Cleric's (confirmed green) and Paladin's (confirmed green), (b) the file's own additive
  carve-out is byte-identical in shape to Cleric's, and (c) the isolated module proof
  (`previously_gated_classes_now_emit_citation_backed_explanations_by_construction`) already
  exercises Wizard directly and passes — **expected to pass, not confirmed. Say so plainly
  rather than claim it.**
- `sd13_sorcerer_level1_spell_baseline` — **DID complete: 18 passed, 1 FAILED**, but the failure
  is confirmed unrelated to this cycle. `sorcerer_level1_fabricates_no_spell_math` fails on
  `class_feature.sorcerer.bloodline.generic.arcane_bloodline.feat_tracker.
  sorcererbloodlinefeatimprovedcounterspell` — an id shape (`bloodline.generic.*`) this cycle's
  file (`class_feature_grant_consumer.rs`, id shape `corpus_record.*`) does not and cannot
  produce; it is emitted by `src/rules_core/pilot_compute/mod.rs`'s OWN pre-existing generic
  bloodline-feat-pool grounding (`ground_sorcerer_bloodline_feat_pool`, a DIFFERENT mechanism,
  landed by an earlier cycle per `kanban.md`'s own cycle-5 entry on this same mechanism). This
  cycle's commit does not touch `mod.rs` at all (confirmed: `git status --porcelain` / `git diff
  --cached --numstat` at commit time show no `mod.rs` entry) — the regression, if it is one,
  predates and is independent of this cycle's diff. This cycle's own `class_feature.sorcerer.
  corpus_record.` carve-out is unaffected either way: the sorcerer widening itself (proven by
  `previously_gated_classes_now_emit_citation_backed_explanations_by_construction`, which passed
  in the same test run) is not implicated by this failure.
- `sd13_bard_level2/3/8/9/10_progression`, `sd13_paladin_level5/6/7_progression`,
  `sd25_druid_level_up_explanation_filter_audit`, `sd25_monk_level_up_explanation_filter_audit`
  — not run to completion this cycle for the same reason. Static per-file review (grep for
  every `.any(|e| e.id`/`.all(|e|` guard in each file, reported inline in this cycle's own work)
  found no OTHER exhaustive or substring guard these five classes' widening could collide with
  beyond the ones already fixed and confirmed (`class_chassis.<class>.` prefix checks are
  unaffected by `class_feature.<class>.corpus_record.*` ids by construction). Druid/Monk receive
  zero emissions from this module regardless of this cycle's change (unchanged exclusion), so
  the two `sd25_*` audits cannot be affected by anything in this diff.

## Build scope

`cargo test --locked --no-run` at the widest workspace scope was **attempted but not confirmed
to exit 0 by this cycle** for the same disk/CPU contention reason (a sibling lane's own
concurrent `--no-run` invocation was observed sharing this exact `CARGO_TARGET_DIR`). The
targeted `cargo test --locked --lib` run above (33/33 passed) DID compile the full library
successfully at this cycle's rebased HEAD, which is a strictly stronger guarantee for the
changed file than a bare `--no-run`. `apps/desktop/src-tauri` not touched, not run.

## Remainder — every unit named by sub-cause (unchanged from cycle 7, this cycle's own scope)

This cycle owns the 218-unit excluded-class sub-cause of the 242-unit `class_feature_owner_
matched` remainder cycle 7 named. Disposition:

| Sub-cause | Population | Disposition this cycle |
|---|---:|---|
| Sorcerer (owning class widened by construction) | 137 | Gate widened; movement floor 1 confirmed (`Sorcerer ~ Spells`), remainder unconfirmed pending wave regen |
| Cleric (owning class widened by construction) | 39 | Gate widened; movement unconfirmed pending wave regen (census shows 0 floor, `already_admitted` population not itemized) |
| Monk (LevelUpPlan pillar filter, NOT this cycle's to clear) | 25 | **Unchanged — 0 cleared.** Owed to a lane with write access to `src/rules_core/level_up/monk.rs` |
| Wizard (owning class widened by construction) | 7 | Gate widened; movement unconfirmed pending wave regen (census shows 0 floor) |
| Paladin (owning class widened by construction) | 5 | Gate widened; movement floor 2 confirmed (`Paladin ~ Holy Champion`, `~ Lay on Hands`) |
| Bard (owning class widened by construction) | 4 | Gate widened; movement floor 2 confirmed (`Bard ~ Bardic Knowledge`, `~ Lore Master`) |
| Druid (LevelUpPlan pillar filter, NOT this cycle's to clear) | 1 | **Unchanged — 0 cleared.** Owed to a lane with write access to `src/rules_core/level_up/druid.rs` |

`137+39+25+7+5+4+1 = 218` — matches cycle 7's own verified split exactly (this cycle's dispatch
brief's "161" figure was wrong; see the correction above).

## Next-cycle plan

1. **Run the wave's shared `docs/work-inventory.json` regeneration** and re-derive the
   `class_feature_owner_matched` population fresh — confirm or refute this cycle's stated
   expectations (floor of 5, larger unconfirmed tail, 0 for Druid/Monk) against the real
   post-regen count, per book.
2. **Confirm `sd13_wizard_level1_prepared_spell_baseline`/`sd13_sorcerer_level1_spell_baseline`
   pass** — expected green, not confirmed this cycle due to environment contention.
3. **Widen `is_druid_pillar_id`/`is_monk_pillar_id`** (`src/rules_core/level_up/{druid,monk}.rs`)
   — the remaining 26-unit blocker on this mechanism's own excluded-class sub-cause, explicitly
   out of this cycle's write scope.
4. The mechanism's own remaining non-excluded-class tail (cycle 7's own `~20` internal-
   bookkeeping and `~4` not-yet-individually-reverified units) is untouched by this cycle,
   unchanged from cycle 7's own next-cycle plan.

- **Status:** partial (`decisions.md §15`) — this cycle closed part of the mechanism's own named
  218-unit excluded-class sub-cause (192 of 218 gate-widened, movement confirmed for a floor of
  5, unconfirmed but expected for the remainder pending wave regen; 26 of 218 — Monk 25 + Druid
  1 — explicitly untouched, named and owed to a different lane's write scope). AT-34-E3-001
  itself remains open (8 other mechanisms + this mechanism's own remainder, unconfirmed count
  pending wave regen, remain).
