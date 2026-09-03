# Cycle — SD-34 wave 36, Lane A — Sub-mechanism 1 matcher fix (`psychic_warrior` + `rogue`)

**Commit SHA (fix + guarded regen):** `bdcf5353dc0a58ec8c394530ab1b5bf8929cd9a8` — this receipt
and the `progress.md`/`kanban.md` updates land in a second, docs-only commit immediately after,
per this bundle's own convention of separating code/regen from doc updates.

**Status: complete, with an honest accounting — a matcher fix mostly reclassifies, not
closes.** Dispatched to close wave 35 lane C's own named Sub-mechanism 1 (19 units:
`psychic_warrior` 18 + `rogue` 1) from the `class_feature_of_unmodelled_corpus_class` shape.
Both classes already have a real, working chassis this engine computes; the population was
misattributed by two matcher bugs, both traced to source and named exactly by wave 35 lane C's
own reconnaissance. **Real outcome, measured, not assumed:** of the 19 named units + 1 unit
wave 35 lane C flagged but did not trace further (a `Kind::Class`-level record outside the
931-unit shape, confirmed below), **4 reach real closure** (`bucket D → DONE`), **16 are
honestly reclassified** to a more precise non-`DONE` bucket (`B` or `C`, still `engine-does-not-
hold`, now naming the TRUE remaining gap instead of a false "class not modelled" claim), and
**0 needed an instrument-correction with no bucket movement at all.** No unit was closed by
fabricating a magnitude; the 4 closures ride an existing, already-shipped
`class_feature_pool_catalog` text-complete rung that could never previously reach these records
because the owner resolution failed before it.

## What wave 35 lane C traced, and what this cycle fixed

Both bugs live in `src/bin/v06_work_inventory.rs`'s `classify()` `Kind::ClassFeature` arm and its
`modelled_class_books()` helper (line numbers below are this cycle's own HEAD, re-derived fresh
— wave 35 lane C's own citations had already drifted by the time this cycle started, exactly the
shift hazard `AGENTS.md` item 9 names):

- **Fix (a), `psychic_warrior` (18 units).** `modelled_class_books()`'s untabled-registry loop
  (`untabled_base_class_chassis::untabled_base_class_registry()`, now `:14399-14419`) inserted
  every registry entry's underscore-slugged `bare_name` directly as the `class_books` key
  (`"psychic_warrior"`), never converted to the corpus's own space-joined display form. The
  function's own neighboring CRB-prestige loop, three lines below, already documents the
  correct convention: *"This key is the corpus display name, lowercased AS-IS (never
  underscore-slugged)"*. `class_feature_owner`'s internal comparison already normalizes
  underscores to spaces for the CANDIDATE side (`class_name_as_group_text`), but it returns the
  ORIGINAL, unconverted key string as the resolved owner — so the downstream safety cross-check
  against `facts.corpus_class_names` (built from the corpus's own naturally space-joined
  `unit.name.to_lowercase()`) compared `"psychic_warrior"` (underscore) against
  `"psychic warrior"` (space) and always failed, discarding the correct match and routing the
  whole population to the final `corpus_class_names`-only fallback. Fixed by space-joining
  `bare_name` at the insertion site (`bare_name.replace('_', " ")`), matching the documented
  convention. `psychic_warrior` was the ONLY multi-word entry among the registry's 20 (wave 35
  lane C's own trace, re-confirmed), so no other registry class was affected.
- **Fix (b), `rogue` (1 unit).** The FINAL owner-resolution fallback (`classify()`, now
  `:12252-12283`) re-runs `class_feature_owner` against `facts.corpus_class_names` alone when
  every earlier resolution path misses, and reports whatever it finds as
  `class_feature_of_unmodelled_corpus_class:<name>` — WITHOUT re-checking whether that name is
  itself a `facts.class_books` member, the check every earlier branch in the same chain already
  performs. For `pathfinder_unchained:class_feature:unchained_rogue_finesse_training_choice`
  (`corpus_key: "Unchained Rogue ~ Finesse Training Choice"`), the earlier `modelled_owner`
  correctly resolves PU's own `"unchained_rogue"` registry entry (15 chars, wins the
  longest-match tie-break over `"rogue"`), but the safety cross-check against
  `corpus_class_names` discards it because the corpus never declares a standalone `"Unchained
  Rogue"` `Kind::Class` record (confirmed this cycle: `data/corpus/pathfinder_unchained/class/
  rogue_unchained_class.json` exists on disk with `data.name: "Unchained Rogue"`, but
  `pathfinder_unchained` contributes **zero** `Kind::Class` units to `docs/work-inventory.json`
  — the raw corpus file is never enumerated into a `Kind::Class` work-inventory unit at all, so
  it can never populate `corpus_class_names`). Every other fallback misses too (this specific
  record's own `type_facet`, `"Unchained Rogue Finesse Damage Choice.SpecialQuality.
  Extraordinary"`, carries no `"<Class> Class Feature"` marker). The final fallback then finds
  `"rogue"` (a real, unrelated, already-modelled CRB base class) via its own `ends_with("
  rogue")` match and reports it as unmodelled. Fixed by adding the same `facts.class_books`
  membership check (normalized via `class_name_as_group_text` so it is immune to the mixed
  space/underscore key shapes different registration loops produce) immediately before the
  branch returns; when the class IS already modelled, the branch no longer fires and execution
  falls through to the remaining, more specific fallback checks in the same `else` block.

## RED → GREEN

Both fixes were neutralized in place (fix (a): reverted the `.replace('_', " ")` call; fix (b):
removed the membership-check guard so the branch always returns) and the new tests re-run to
confirm each fails for the stated reason, then restored:

- `modelled_class_books_space_joins_a_multi_word_untabled_registry_name` — RED:
  `psychic warrior` not a `class_books` key (`["psychic", "psychic_warrior"]` present instead).
- `psychic_warrior_class_feature_is_not_misreported_as_an_unmodelled_corpus_class` — RED:
  `verdict.evidence == "class_feature_of_unmodelled_corpus_class:psychic_warrior"`.
- `rogue_final_fallback_checks_class_books_membership_before_declaring_unmodelled` — RED:
  `verdict.evidence == "class_feature_of_unmodelled_corpus_class:rogue"`.
- `a_genuinely_unmodelled_corpus_class_still_reads_unmodelled_after_the_membership_check`
  (negative control) — stays GREEN throughout; proves the membership check does not swallow
  the genuine Sub-mechanism 5 (832-unit, 60-class) population this same branch also serves.

GREEN, both fixes restored: `cargo test --locked --bin v06_work_inventory -j 6 --
class_feature_type_facet_owner_fallback_tests::` → **13 passed; 0 failed** (9 pre-existing + 4
new). Wider regression sweep, `cargo test --locked --bin v06_work_inventory -j 6 --
class_feature` → **167 passed; 0 failed; 356 filtered out**. `cargo clippy --locked --bin
v06_work_inventory -j 6 -- -D warnings` → clean, 0 warnings.

## Real movement, measured against a guarded regeneration — not assumed

Guarded path run in full: `corpus_literal_sweep` → `CLEAN`, `records_examined: 48706` (unchanged
before/after — no corpus record added or removed this cycle). `derived_evaluator_fixture_check`
→ clean (no findings). `docs/work-inventory.json` regenerated:
`python3 -c "import json; print(len(json.load(open('docs/work-inventory.json'))['units']))"` →
**49438**, unchanged. Whole-inventory id-diff: **0 added, 0 removed, 20 changed** — every changed
id traced by hand against the fix, none unexplained:

| Unit | Before (bucket) | After (bucket) | Movement |
|---|---|---|---|
| `psychic_warrior_martial_power` | D (`class_feature_of_unmodelled_corpus_class:psychic_warrior`) | **DONE** (`text-complete`, `class_feature_pool_catalog_serves_a_rendered_description`) | **Closure** |
| `psychic_warrior_psionic_proficiency` | D | **DONE** (same evidence) | **Closure** |
| `psychic_warrior_secondary_path` | D | **DONE** (same evidence) | **Closure** |
| `psychic_warrior_twisting_path` | D | **DONE** (same evidence) | **Closure** |
| `archetype_psychic_warrior` | D | B (`class_feature_owner_matched_by_name_but_record_not_held_by_engine`) | Reclassification |
| `psychic_warrior_manifesting` | D | B (same evidence) | Reclassification |
| `unchained_rogue_finesse_training_choice` | D | B (`class_feature_option_pool_record_with_magnitude_not_held_by_engine`) | Reclassification |
| 12 remaining `psychic_warrior_*` class-feature units (7 archetypes: `martial_kineticist`, `meditant`, `pathmaster`, `protector`, `scaled_rider`, `thunderjarl`, `traceur`; plus `eternal_warrior`, `manifesting_variables`, `power_points`, `pathweaving`, `warrior_s_path`) | D | C (`no_explanation_id_and_no_diagnostic_names_this_feature`) | Reclassification |
| `ultimate_psionics:class:psychic_warrior` (**outside the named 19-unit population** — see below) | B (`class_absent_from_ClassId_ALL_and_book_class_id_enums`) | D (`class_modelled_but_no_observed_delta_on_the_rendered_snapshot`) | Reclassification (a correction of a previously FALSE "absent from enums" claim, not a regression — it now states the true, more specific reason) |

Re-derive command: a small script diffing `docs/work-inventory.json` before/after by id against
`scripts/completion_atlas.py`'s own `_bucket_of` logic (bucket markers `_A_MARKER`/
`_B_MARKERS`/`_C_MARKERS`, `scripts/completion_atlas.py:65-71`), saved this cycle at
the agent's own scratchpad directory (`movement.py`), output captured verbatim above.

**Why only 4 close, not 19.** Fixing the matcher restores the CORRECT owner for these records,
but correct ownership only reaches `grounded`/`text-complete` when a real, already-shipped
consumer (an `explanation_id` match, or — for the 4 that closed — the `class_feature_pool_
catalog` text-complete rung `class_feature_pool_catalog_serves_a_rendered_description`) can
observe it. The 4 closures are option-pool-catalog-served, zero-magnitude, real-description
records that only needed a correct owner to unblock an already-existing rung. The other 15 need
a per-feature magnitude id this engine has never computed for `psychic_warrior`'s named path
features/manifesting/power-points mechanics, or (for `rogue`) real PU-specific magnitude wiring
for `Unchained Rogue`'s own finesse-training mechanic — genuine remaining Epic 3/4 scope, now
correctly labeled instead of hidden behind a false "class not modelled" claim.

## The flagged concern beyond the 931-unit shape — checked, one real hit found

The dispatch brief named this explicitly: "the brief flags this bug may also affect classes
outside this 931-unit shape (e.g. `Kind::Class`-level records) -- check that too and report what
you find, even if out of this cycle's own scope to fix." Checked directly:

- **`Kind::Class` arm (`classify()`, `:11660-11689`).** This arm does a DIRECT
  `facts.class_books.contains_key(&unit.name.to_lowercase())` check with no
  `class_name_as_group_text` normalization at all — so fix (a) (space-joining `bare_name`)
  directly fixes this arm too for `psychic_warrior`'s own `Kind::Class` unit, confirmed live in
  the diff table above (`ultimate_psionics:class:psychic_warrior`, previously wrongly
  `class_absent_from_ClassId_ALL_and_book_class_id_enums` despite the chassis genuinely
  existing, now correctly `class_modelled_but_no_observed_delta_on_the_rendered_snapshot`). This
  is the ONE real unit this cycle found outside the named 19-unit population — 20 total changed,
  not 19.
- **A related, currently-dormant instance of the SAME bug shape, found and NOT fixed (out of
  this cycle's scope, flagged for the next wave).** `modelled_class_books()`'s `PuClassId::ALL`
  loop (`:14372-14374`) inserts `id.name().to_string()` directly, and `PuClassId::name()`
  (`src/rules_core/rules_tables/pathfinder_unchained/class_chassis.rs:119-122`) returns the SAME
  underscore-slugged form (`"unchained_barbarian"`, `"unchained_monk"`, `"unchained_rogue"`,
  `"unchained_summoner"` — all 4 PU classes are multi-word). This is structurally the identical
  shape fix (a) just corrected for the untabled registry. It currently produces **no live
  defect** for two independently-confirmed reasons: (1) `pathfinder_unchained` contributes
  **zero** `Kind::Class` units to `docs/work-inventory.json` (confirmed above), so the
  `Kind::Class` arm's direct `contains_key` check never runs against these 4 keys at all; (2)
  every one of `unchained_barbarian`/`unchained_monk`/`unchained_summoner`'s own class-feature
  records that this cycle checked resolves its owner via the `type_facet`-based fallback (which
  compares the underscored `class_books` key against an underscored candidate extracted from
  `type_facet`, both sides consistently underscored, so it matches despite never being
  space-joined) — bypassing the corpus_wide cross-check that is where the underscore/space
  mismatch actually bites. Confirmed by grep: `unchained_rogue` is the ONLY
  `pathfinder_unchained` unit in the whole corpus currently carrying
  `class_feature_of_unmodelled_corpus_class` evidence (the shape this bug produces) — none of
  the other 3 PU classes' own features are affected today. **Not fixed this cycle** — the
  brief's scope was the two named, live-verified bugs; this is a real but currently-inert
  structural twin, named here rather than silently carried forward.

## Files touched this cycle

- `src/bin/v06_work_inventory.rs` — `modelled_class_books()`'s untabled-registry loop
  space-joins `bare_name` (fix a); the final `corpus_class_names`-only owner-resolution fallback
  in `classify()`'s `Kind::ClassFeature` arm gains a `facts.class_books` membership check,
  normalized via the existing `class_name_as_group_text` helper (fix b). 5 new tests in
  `class_feature_type_facet_owner_fallback_tests` (2 direct fix-proof integration tests, 1 direct
  `modelled_class_books()` unit test, 1 negative control for fix (b), plus the pre-existing
  suite's own coverage unchanged).
- `scripts/completion_atlas.py` — 4 citation lines (`A`, `B`, `C`, `V` buckets) re-derived after
  this cycle's own insertions shifted them (`+21` lines from fix (b), landing above all four;
  fix (a) lands below them and shifts nothing above). Each re-derived by fresh `grep -n` for the
  same unique literal, line content read back and confirmed still the real construction site,
  matching the exact re-derivation method every prior wave's own citation-shift entries used.
- `docs/work-inventory.json`, `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/
  completion-atlas.json` — regenerated via the guarded path (above).
- `docs/retro/events/sd34-wave36-lanea.jsonl` — 1 `incident` (`wrong-base-worktree`, this
  cycle's own worktree cut 428 commits behind local `tranche/14` — see below).
- This receipt, `progress.md`.

## Identifier / wired-integration audits

`git diff --unified=0 HEAD -- src/bin/v06_work_inventory.rs | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|
Sd[0-9]+|t_[0-9a-f]{8,})\b'` → `OK_NO_BUNDLE_TAGS`.
`git diff --unified=0 HEAD -- src/bin/v06_work_inventory.rs | grep -inE '\b(STUB|MOCK|
placeholder|not yet implemented|todo|fixme|hack)\b'` → `OK_NO_TOKENS`.

## Build scope verified

`cargo build --locked --bin v06_work_inventory -j 6` → clean. `cargo test --locked --no-run -j 6`
(full workspace) → **exit 0**, every test binary built (`v06_work_inventory`, every `sd2*`/`sd3*`/
`v06_*` integration test crate, etc.). `apps/desktop/src-tauri` not touched this cycle — not run,
per `decisions.md §10`'s own "tested explicitly if touched" scoping.

## Sweep population

`corpus_literal_sweep`: `records_examined: 48706` before and after (unchanged — no corpus record
added, removed, or regenerated this cycle; only `src/bin/v06_work_inventory.rs` and
`scripts/completion_atlas.py` changed, plus the two derived JSON artifacts). `CLEAN` both times.

## Oracle pin

Not consulted this cycle — no figure here came from the pinned PCGen corpus checkout; every
quoted number is derived from this repo's own `docs/work-inventory.json`, `data/corpus/`, and
`scripts/completion_atlas.py`.

## Movement (four buckets, this cycle)

- **Closure (bucket → DONE):** **4** — `psychic_warrior_martial_power`, `_psionic_proficiency`,
  `_secondary_path`, `_twisting_path`, all D → DONE via `class_feature_pool_catalog_serves_a_
  rendered_description`, verified by the guarded regen's own before/after diff.
- **Reclassification (bucket → different non-DONE bucket):** **16** — 2 units D → B
  (`archetype_psychic_warrior`, `psychic_warrior_manifesting`), 1 unit D → B (`rogue`'s own
  record), 12 units D → C (the remaining `psychic_warrior_*` class-feature records), 1 unit B →
  D (`ultimate_psionics:class:psychic_warrior`, a correction of a previously false "absent from
  enums" evidence string — see the flagged-concern section above for why this reads as a
  bucket-letter "regression" but is actually the honest fix).
- **Reachability:** 0 units newly reached or lost reachability (no `TraitRole`/consumer-wiring
  concept applies to this shape).
- **Instrument-correction:** 0 with no bucket movement — every one of the 20 changed units moved
  buckets (either to DONE or to a more precise non-DONE bucket); none stayed in the identical
  bucket with only a cosmetic evidence-string change this cycle.

## Figures (every number, its command, its denominator)

- Population this cycle owns: **19** named (`psychic_warrior` 18 + `rogue` 1), wave 35 lane C's
  own `class_feature_of_unmodelled_corpus_class` Sub-mechanism 1 total, re-confirmed unchanged
  at cycle start: `grep -c 'class_feature_of_unmodelled_corpus_class:psychic_warrior\|
  class_feature_of_unmodelled_corpus_class:rogue' docs/work-inventory.json`-equivalent JSON
  query → 18 + 1 = 19, denominator: the 931-unit Sub-mechanism 1 shape wave 35 lane C named.
- Real units changed this cycle: **20** (19 named + 1 outside the named shape, the
  `Kind::Class`-level `psychic_warrior` record — see the flagged-concern section).
- Real closures (bucket → DONE): **4**, denominator: the 19-unit named population.
- `docs/work-inventory.json` total population: **49438** —
  `python3 -c "import json; print(len(json.load(open('docs/work-inventory.json'))['units']))"`,
  denominator: whole corpus, unchanged from pre-cycle.
- `completion_atlas.py --check`: `population=49438 buckets=10 unclassified=0 overlap=0
  citation_failures=0`, denominator: whole corpus.
- Bucket counts, before → after (`python3 scripts/completion_atlas.py --check`, temporarily
  swapping in the pre-cycle inventory snapshot to derive "before", then restoring the real
  post-cycle file — restore confirmed by `generated_at` matching the regenerated file's own
  timestamp): `DONE` 25027→25031 (+4), `A` 449→449 (0), `B` 11769→11771 (+2 net: +3 D→B minus
  the 1 B→D correction), `C` 4173→4185 (+12), `D` 2891→2873 (−18), all other buckets unchanged.

## Retro events this cycle

- `incident` (`docs/retro/events/sd34-wave36-lanea.jsonl`,
  `1788422176241-sd34-wave36-lanea-01187c`, `recurrence_key: wrong-base-worktree`): this
  worktree's own branch was cut at `ea2b3396f2` (428 commits behind the real, local-only
  `tranche/14` tip `4379c9be05` — `origin/tranche/14` was itself stale). The identical shape
  `wave34_laneB`'s and `wave35_laneB`'s own receipts each name — `AGENTS.md` item 8's tracked
  class, recurring again. Resolved via `git rebase tranche/14` (the local, ahead-of-origin ref),
  a clean fast-forward with 0 commits to replay.

## Next-cycle plan

1. **15 units remain non-`DONE`** after this fix, now correctly bucketed (12 in C — "held and
   computed, never surfaced" — 2 in B for `psychic_warrior`'s own remaining records, 1 in B for
   `rogue`'s record). Real per-feature magnitude/explanation-id wiring, Epic 3/4 scope: the 12
   `C`-bucket `psychic_warrior_*` records (archetypes ×7, `eternal_warrior`,
   `manifesting_variables`, `power_points`, `pathweaving`, `warrior_s_path`) each need a
   dedicated explanation id this engine has never computed for that specific path/archetype/
   manifesting mechanic; the 2 `B`-bucket `psychic_warrior` records
   (`archetype_psychic_warrior`, `manifesting`) and the 1 `B`-bucket `rogue` record need a real
   consumer table, not merely a corrected owner.
2. **The dormant `PuClassId` underscore-key twin** (flagged above, not fixed): apply the
   identical space-join fix to the `PuClassId::ALL` loop in `modelled_class_books()`
   (`:14372-14374`) as a defensive correction even though no LIVE unit is affected today — a
   future PU `Kind::Class` ingestion or a `type_facet`-marker-less PU class-feature record for
   `unchained_barbarian`/`_monk`/`_summoner` would silently hit the identical bug this cycle
   just fixed for `psychic_warrior`. Cheap (same one-line shape as fix (a)), zero measured
   population today, named rather than carried forward silently per `AGENTS.md` item 8.
3. **Sub-mechanisms 2-5** (80 + 832 units) remain exactly as wave 35 lane C's own reconnaissance
   named them — untouched this cycle, out of this cycle's scope.
