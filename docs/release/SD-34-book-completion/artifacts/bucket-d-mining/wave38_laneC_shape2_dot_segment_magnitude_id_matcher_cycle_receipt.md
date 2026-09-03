# Cycle — SD-34 wave 38, Lane C — Shape 2's 154-unit magnitude-bearing remainder: a real matcher fix, 105 units closed, 54 honestly named unclosed

- **Commit SHA:** (local, this worktree — see structured report)
- **Files touched:** `src/bin/v06_work_inventory.rs` (widened
  `class_feature_exact_suffix_grounded` with a second-to-last-dot-segment
  check plus two safety guards, new
  `CLASS_FEATURE_ID_NON_MAGNITUDE_TRAILING_MARKERS` constant, 6 new tests),
  `scripts/completion_atlas.py` (10 shifted citation pins re-derived,
  three times, as this cycle's own edit grew), `docs/work-inventory.json`
  (guarded regen), `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
  (regenerated snapshot), this receipt, `progress.md`, `kanban.md`,
  `docs/retro/events/sd34-wave38-lanec.jsonl` (new). **No `data/corpus/**`
  file touched.**
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --unified=0
  HEAD -- src/bin/v06_work_inventory.rs scripts/completion_atlas.py`, no
  `sd[0-9]+_`/`SD[0-9]+_`/`t_[0-9a-f]{8,}` hits).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff, no
  `STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack`).
- **Acceptance criterion (verbatim from this cycle's dispatch brief):**
  "Continue wave 37 lane C's own disposition trace... Re-derive the CURRENT
  full bucket-D breakdown fresh... and identify the next-cheapest named,
  not-yet-attempted shape or sub-shape across ALL of bucket D's six original
  mechanisms... check whether Shape 2's remaining 154 magnitude-bearing
  units... have been touched yet, and whether Sub-mechanisms 3/4/5... still
  have open items beyond what lanes A and B this same wave are handling.
  Disposition-trace (not necessarily fix) whatever you find cheapest and not
  yet claimed by lane A or B this wave — name an exact population and
  next-step for everything you touch, implement a real fix only if it is
  small and unambiguous."

## Worktree base note (self-healed, not escalated)

This cycle's assigned worktree started at `ea2b3396f2` (PR #377's own merge
commit, SD-33's launch tip) — the SAME stale base wave 37's own three lanes
all independently hit and self-healed from. Confirmed local `tranche/14`'s
real tip is `fb149ce2b1` (wave 37's own wave-end gate, "2 baselines raised,
full 40/40 confirmed"); `origin/tranche/14` is even staler
(`7ea9651b87`, wave 33 lane D). `git merge-base --is-ancestor ea2b3396f2
fb149ce2b1` → true (clean fast-forward), rebased before any commit landed
(`git rebase fb149ce2b1`, zero conflicts). This is now the fourth
consecutive wave-37/38 cycle to hit this exact class — not re-escalated
here since the self-heal is now a one-line, well-drilled routine, but
flagged again per `AGENTS.md` L72 ("a key firing more than a handful of
times is a missing mechanism, not bad luck").

## Fresh full bucket-D breakdown (re-derived, not trusted from any prior receipt)

`python3 scripts/completion_atlas.py --check` at this cycle's own rebased
HEAD, before any change: `population=49438 buckets=10 unclassified=0
overlap=0`, `D: 2661` — matches wave 37's own final reported state exactly.

Re-derived the six-original-mechanism split (`_bucket_of`'s own D-fallthrough
logic, `wave32_laneC_reconnaissance_cycle_receipt.md`'s own naming) fresh
against the live inventory (script: group by `evidence`, sum):

| Mechanism | Wave 32 baseline | This cycle's pre-fix figure | Status |
|---|---:|---:|---|
| 1. `*_content_table_holds_zero_magnitude_record_pending_wiring_class_review` (Epic-2 simple-kind fallthrough) | 1,727 | **1,727** (unchanged) | Already mined to its floor (wave 32); 0 promotable |
| 2. `class_feature_of_unmodelled_corpus_class` (70 classes → now split into sub-mechanisms 1–5) | 931 | **700** (68 classes) | Sub-mech 1 closed (wave 36 A); sub-mech 2 (creature-type collision, 49) + 3 (eidolon, 16) + 4 (sentinel, 1) dispositioned (wave 36/37 C, B); sub-mech 5 (634/60 classes) untouched |
| 3. `class_feature_no_dedicated_magnitude_id_matched_the_record_slug` ("Shape 2") | 179 | **179** (unchanged — confirmed untouched by any wave 36/37 lane) | **This cycle's target** |
| 4. `race_trait_record_loaded_but_never_applies` | 53 | **6** (mostly closed by intervening waves) | Not this cycle's scope |
| 5. `class_modelled_but_no_observed_delta_on_the_rendered_snapshot` | 38 | **29** | Not this cycle's scope |
| 6. `class_feature_*_held_by_*_table` ("held by table" rungs) | 27 | **0** (fully closed) | Closed by an intervening wave |
| (new, post-wave-35) `race_trait_skinwalker_change_shape_...` | — | **19** | wave 35 lane A's own new sub-shape, not this cycle's scope |
| (new, post-wave-36) `race_trait_template_bonus_language_grant_...` | — | **1** | Not this cycle's scope |

Sum check: `1727+700+179+6+29+0+19+1 = 2661` — matches `D: 2661` exactly.

**Sub-mechanisms 3/4/5, checked fresh (per the brief's own explicit
instruction) — no open items beyond what is already dispositioned:**

- **Sub-mechanism 3 (eidolon, 16 units):** wave 36 lane C's own receipt
  already reached case (b), "genuinely unbuilt, **confirmed twice**"
  (`class_summoner.rs`'s own doc comment names per-level Eidolon features as
  explicitly out of scope, independently corroborated by a concurrent wave
  36 lane B finding). Re-confirmed still 16, unchanged since — no open item,
  correctly named for a future Epic 4/5 Summoner-Eidolon-companion-table
  build, nothing left for a disposition-trace cycle to add.
- **Sub-mechanism 4 (sentinel, 1 unit):** wave 37 lane B's own receipt
  fully dispositioned this ("genuinely unbuilt, Epic 4/5 scope — Ranger's
  10th-level combat-style-feat-chain slot, misattributed to a name
  collision with `inner_sea_gods:class:sentinel` rather than to Ranger, but
  unbuildable either way since the underlying 151-sibling mechanism itself
  is unbuilt"). Re-confirmed still 1, unchanged. No open item.
- **Sub-mechanism 5 (634 units / 60 classes):** re-confirmed unchanged at
  **634/60** (identical to wave 37 lane B's own re-derivation) — **not
  touched this cycle**, presumed in scope for lane A and/or lane B this same
  wave per wave 37 lane B's own "directly dispatchable" next-cycle plan
  item. Named here only to confirm it is NOT this cycle's own overlapping
  claim, not re-mined.

**Conclusion: Shape 2 (mechanism 3, 179 units / 154 magnitude-bearing +
25 zero-magnitude) is the cheapest not-yet-attempted shape** — every other
mechanism is either already mined to its floor, already fully
disposition-traced with zero further action available, or presumed claimed
by a sibling lane this same wave.

## What Shape 2 actually is, and why 154 of its 179 units were never truly attempted

`class_feature_no_dedicated_magnitude_id_matched_the_record_slug`
(`v06_work_inventory.rs`'s own `class_feature_exact_suffix_grounded`/
`suffix_stripped_grounded` chain) fires when `owner` resolves correctly (a
real, modelled class) **and** the engine's own generic per-class roster
proves it holds SOME record for this feature group, but no explanation id
in `facts.explanation_ids` matches this exact feature's slug closely enough
to credit a real, per-record magnitude.

**Read the real engine source before assuming this needs new computation
(the brief's own "not necessarily fix" instruction, honored by checking
first):** `pilot_compute/mod.rs` carries a family of ~20
`ground_<class>_class_features` dispatch functions (Antipaladin, Cryptic,
Dread, Marksman, Psychic Warrior, Soulknife, Aegis, Tactician, Vitalist,
Wilder, Kineticist, Medium, Mesmerist, Occultist, Psychic, Spiritualist,
Magus, Shifter, Vigilante, Psion — a real, already-shipped SD-32 card 11
"T12" effort, each with corpus-transcribed formulas and their own unit
tests). Every one of these emits its per-feature explanation ids in the
SAME convention: `class_feature.untabled.<class>.<feature_slug>.
<magnitude_descriptor>` — the feature's own slug as **its own dot segment**,
followed by a THIRD segment naming which quantity it computes (`dc`,
`known`, `uses_per_day`, `damage_reduction`, ...). Confirmed by direct
`grep` across `mod.rs`, not assumed from one function.

**The matcher never recognized this convention.** `class_feature_exact_
suffix_grounded`'s exact check inspects the id's OWN trailing dot segment
(the magnitude descriptor, e.g. `"dc"`), never the feature's own segment one
position further in; the underscore-suffix fallback
(`id_matches_feature_slug_after_known_magnitude_suffix_strip`) only strips a
trailing `_<word>` WITHIN a single dot segment, never crosses a `.`
boundary. So a real, already-computed, already-tested magnitude (e.g.
`advanced_players_guide/class_feature/antipaladin/cruelty.json`'s own DC and
count, computed by `antipaladin_features.rs`'s `cruelty_dc`/
`cruelties_known`, wired into `ground_antipaladin_class_features`, and
unit-tested since SD-32) sat unrecognized behind the generic Shape 2 D-bucket
evidence — the engine genuinely held the record; the classifier simply could
not see it.

## The fix (RED → GREEN, three safety guards, all live-verified before landing)

Widened `class_feature_exact_suffix_grounded` (`v06_work_inventory.rs`) with
a new alternative: an id's SECOND-TO-LAST dot segment (skipping the trailing
magnitude-descriptor segment) is compared to `feature_slug`, requiring:

1. The pre-existing `group == class_name_as_group_text(owner)` guard
   (unchanged, already covered every prior branch).
2. **`feature_slug != owner`** — closes a live false positive found via a
   temporary explanation-id dump test (`build_pilot_headless_receipt` on a
   bare `class:arcanist` character) BEFORE committing: a no-`~` corpus_key
   unit's `feature_slug` silently falls back to `unit.name`, and without
   this guard `class_feature:arcanist`/`:bloodrager`/`:brawler` (each
   `feature_slug == owner`) false-matched `pilot_compute/mod.rs`'s generic
   `class_chassis.<class>.caster_level` table — a real chassis fact, but not
   this bare unit's own magnitude. Same guard already used by the
   pre-existing `suffix_stripped_grounded` fallback for the identical
   reason; this cycle applies it to the new branch too.
3. **The trailing segment must NOT be a
   `CLASS_FEATURE_ID_NON_MAGNITUDE_TRAILING_MARKERS` word** (`unsupported`,
   `not_modelled`) — closes a second live false positive, also found via the
   dump test (this time on `class:unchained_barbarian`): `push_deferred_
   class_features` and its ~14 sibling `push_*_deferred_diagnostic`
   functions push the SAME id into BOTH `diagnostics` and `explanations`
   (`value: 0`) whenever a corpus record is not independently granted — real
   ids like `class_feature.pu.unchained_barbarian.corpus_record.
   uncanny_dodge.unsupported` genuinely have `uncanny_dodge` as their own
   second-to-last segment, but the trailing `.unsupported` marks it a
   diagnostic mirror, not a magnitude. Confirmed corpus-wide: `grep -c` for
   this exact id-suffix shape finds **175** `.unsupported` + **2**
   `.not_modelled` ids across `pilot_compute/mod.rs`, all diagnostic-shaped,
   none a legitimate magnitude word.

**Both false positives were caught and fixed BEFORE this cycle's own
first-draft regen was ever committed** — self-corrected within the cycle,
not shipped and found later. Retro-logged as a `correction`
(`docs/retro/events/sd34-wave38-lanec.jsonl`, id
`1788467592418-sd34-wave38-lanec-7cbf70`) against this cycle's own
first-draft claim that the check was safe with only guard 1.

### Tests (`class_feature_exact_suffix_grounded_tests`, `v06_work_inventory.rs`)

- `a_dot_separated_magnitude_descriptor_grounds_via_its_own_feature_segment`
  (RED before the widening existed; GREEN after) — real
  `class_feature.untabled.antipaladin.cruelty.dc` id, quoted verbatim.
- `a_multi_word_feature_slug_and_a_multi_word_descriptor_both_resolve_on_dot_boundaries`
  — proves the split is on the DOT, not underscores (`touch_of_corruption`/
  `uses_per_day`, both multi-word).
- `a_dot_separated_id_for_a_different_feature_does_not_cross_credit` —
  negative control, a real sibling id (`unholy_resilience.save_bonus`)
  cannot ground `cruelty`.
- `the_second_to_last_segment_check_does_not_reopen_the_negation_regression`
  — confirms the pre-existing `bloodrage_execution.not_raging` negative
  control (an unrelated prior fix) still holds under the new branch.
- `a_generic_class_chassis_fact_cannot_ground_the_bare_class_name_unit` —
  guard 2's own negative control, id quoted verbatim
  (`class_chassis.arcanist.caster_level`).
- `a_diagnostic_mirror_id_ending_in_unsupported_cannot_ground_via_its_own_feature_segment`
  — guard 3's own negative control, id quoted verbatim (`corpus_record.
  uncanny_dodge.unsupported`).

**13 of 13 `class_feature_exact_suffix_grounded_tests` pass** (6 new, 7
pre-existing, 0 regressed). **533 of 533 `v06_work_inventory` bin tests
pass** (6 new, 0 regressed). `src/rules_core/pilot_compute/mod.rs` itself
carries **zero diff** — the temporary dump test used to find both false
positives was added, used, and removed within this cycle, confirmed by
`git diff --stat -- src/rules_core/pilot_compute/mod.rs` returning empty.

## Movement — the real, regen-verified delta

`python3 scripts/completion_atlas.py --check`:
`population=49438 buckets=10 unclassified=0 overlap=0 citation_failures=0`.

| Bucket | Before | After | Delta |
|---|---:|---:|---:|
| DONE | 25244 | 25330 | **+86** |
| V (literal-verified/fixture-verified) | 289 | 315 | **+26** |
| D | 2661 | 2556 | **−105** |
| C | 4185 | 4180 | −5 |
| X | 170 | 168 | −2 |
| A, B, M, U, Z | unchanged | unchanged | 0 |

Sum check: `+86+26-105-5-2 = 0`.

**112 units total changed `(status, evidence)`, every one a genuine
improvement, none a regression** — byte-level `git diff -- docs/work-inventory.json`,
per-id status comparison against the true committed HEAD baseline (not this
cycle's own intermediate, buggy first regen). Transition breakdown:

- **85** `engine-does-not-hold → grounded` (D → DONE).
- **25** `engine-does-not-hold → literal-verified` (D → V).
- **1** `deferred-with-reason → grounded` (X → DONE — `advanced_players_guide:class_feature:cavalier_mount`,
  a diagnostic-shadowed unit the SAME id-recognition gap affected outside
  Shape 2's own D-bucket evidence).
- **1** `deferred-with-reason → literal-verified` (X → V — `advanced_class_guide:class_feature:hunter_animal_companion`).
- **1** `grounded → grounded` (evidence-string-only correction, no bucket
  move — `advanced_class_guide:class_feature:brawler_maneuver_training`
  now credits `explanation_id_observed_in_a_real_computation` instead of
  the suffix-stripped fallback's own evidence string, a strictly more exact
  match found first).

**Of the 105 Shape-2-evidence units that moved (100 of the 154
magnitude-bearing remainder + 5 bonus zero-magnitude closures the same fix
incidentally reached), the exact set is reproducible**: `python3 -c` diff
over `docs/work-inventory.json`'s `units`, filtering `evidence ==
"class_feature_no_dedicated_magnitude_id_matched_the_record_slug"` at the
true HEAD baseline vs. this cycle's own final committed state.

**5 additional closures came from OUTSIDE Shape 2's own D-bucket evidence**
(the SAME matcher gap also affected some units filed under bucket C's
`no_explanation_id_and_no_diagnostic_names_this_feature` evidence, and 2
units filed under bucket X's `deferred-with-reason` diagnostic-shadow path
— `hunter_animal_companion`, `cavalier_mount`) — an honest, disclosed
emergent effect of the fix's own generality, the same shape prior waves'
receipts (e.g. wave 36 lane C's Order-of-the-Dragon fix) have already
reported rather than silently absorbed into the targeted count.

## Shape 2's remaining 54-unit magnitude-bearing population, named precisely

`python3 -c` filter over `docs/work-inventory.json`'s `units`: `evidence ==
"class_feature_no_dedicated_magnitude_id_matched_the_record_slug" and
magnitude_token_count > 0`, of the 49,438-unit population.

| Class | Units | Class | Units | Class | Units |
|---|---:|---|---:|---|---:|
| Unchained Monk | 7 | Duelist | 4 | Fighter | 2 |
| Summoner | 6 | Shadowdancer | 4 | Loremaster | 2 |
| Unchained Barbarian | 6 | Unchained Rogue | 4 | Wizard | 2 |
| Monk | 5 | Unchained Summoner | 3 | Bard/Cleric/Druid/Paladin/Ranger/Sorcerer/Psychic | 1 each |
| Assassin | 2 | | | | |

Sum: `7+6+6+5+4+4+4+3+2+2+2+1×7 = 54`, re-derive with the exact filter
above.

**Spot-checked (not assumed) why these did NOT close under this cycle's
own fix — a genuinely DIFFERENT failure shape, not the same gap:**
Unchained Monk's own `ground_or_block_unchained_monk_class_features`-style
functions (`pilot_compute/mod.rs`) DO compute real magnitudes for these
exact features, but their explanation ids spell the magnitude-descriptor in
a WORD CHOICE that differs from the corpus feature's own slug within a
SINGLE dot segment — e.g. `Unchained Monk ~ AC Bonus` (`feature_slug =
"ac_bonus"`) is computed as `class_feature.pu.unchained_monk.armor_class_bonus`
(`"armor_class_bonus"`, not `"ac_bonus"`), and `~ Ki Pool` (`feature_slug =
"ki_pool"`) as `...ki_points` (`"ki_points"`, not `"ki_pool"`). This is a
single-dot-segment SYNONYM problem (needs a per-feature alias table, or a
widened `CLASS_FEATURE_ID_MAGNITUDE_SUFFIXES`-style word-equivalence map),
**not** the dot-segment-boundary problem this cycle's own fix closes —
confirmed by direct read of `mod.rs`'s own explanation-id literals for
Unchained Monk, Summoner, and Wizard (three spot-checked classes covering
15 of the 54 units) before naming this. The remaining classes (Duelist,
Shadowdancer, Assassin, Loremaster — prestige classes; Monk/Bard/Cleric/
Druid/Paladin/Ranger/Sorcerer — CRB base classes) were not individually
traced this cycle; whether each has ANY per-feature compute function at
all, or the same synonym gap, is the next lane's own first check.

## Figures (every number, its command, its denominator)

- `population=49438 buckets=10 unclassified=0 overlap=0 citation_failures=0`
  — `python3 scripts/completion_atlas.py --check`, this cycle's own final
  HEAD.
- `DONE: 25244→25330 (+86)`, `V: 289→315 (+26)`, `D: 2661→2556 (−105)`,
  `C: 4185→4180 (−5)`, `X: 170→168 (−2)` — same command, before/after this
  cycle's own guarded regen.
- `179→74` Shape 2 total (`105` closed), `154→54` magnitude-bearing (`100`
  closed), `25→20` zero-magnitude (`5` bonus closures) — Python filter over
  `docs/work-inventory.json`'s `units`, `evidence ==
  "class_feature_no_dedicated_magnitude_id_matched_the_record_slug"`, of
  `D`'s own population, before/after.
- `634/60` sub-mechanism 5, `16` sub-mechanism 3 (eidolon), `1`
  sub-mechanism 4 (sentinel) — all unchanged this cycle, same `Counter`
  method wave 35/37 used, re-run fresh.
- `175` `.unsupported` + `2` `.not_modelled` diagnostic-mirror ids
  corpus-wide — `grep -c` against `pilot_compute/mod.rs` at this cycle's
  own HEAD.
- `13` of 13 `class_feature_exact_suffix_grounded_tests` pass (6 new),
  `533` of 533 `v06_work_inventory` bin tests pass (6 new) — `cargo test
  --locked --bin v06_work_inventory`, this cycle's own final HEAD.
- `48706 of 51476` corpus records examined, CLEAN, unchanged before/after
  — `corpus_literal_sweep --json-out`, of the full corpus (no
  `data/corpus/**` record touched this cycle — Rust classifier logic
  only).
- `1839` units cleared over `2580` fixture rows, `0` failed —
  `derived_evaluator_fixture_check --json-out`, of the fixture's own
  2,580-row coverage, unchanged before/after.

## Row-count command output

```
$ grep -n "^| 37 |" docs/release/SD-34-book-completion/kanban.md | tail -1
| 37 | `mine-bucket-d` | 3 | wave 32, lane C (no AT-34-E# card yet) | partial | ...
```
Row 37 (`mine-bucket-d`) is the same accumulating row every prior bucket-D
mining cycle appends into — this cycle appends its own sentence.

## Build scope verified

- `cargo test --locked --bin v06_work_inventory` → 533/533 pass (this
  cycle's own final HEAD, run twice: once mid-cycle to confirm the
  first-draft guard, once after the two safety guards landed).
- `cargo test --locked --no-run` (full workspace) → exit 0, run at this
  cycle's own final HEAD, after all guards landed and before the final
  guarded regen.
- `python3 scripts/tests/test_completion_atlas.py` → 38/38 pass (this
  cycle touched `scripts/completion_atlas.py`'s citation pins).
- Desktop crate (`apps/desktop/src-tauri`) — not run this cycle: `git diff
  --stat HEAD -- apps/desktop/` is empty, no file under `apps/desktop/`
  touched, honestly reported skipped
  (`workflow-instruction.md §6` step 3 scopes this to "if touched").

## Sweep population

`corpus_literal_sweep`: `48706 examined of 51476 read, 0 findings, CLEAN` —
no `data/corpus/**` record added, changed, or removed this cycle (Rust
classifier logic only), delta 0, consistent with 0 records added.

## Oracle pin

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
(`scripts/pcgen-oracle-pin.env`) — no figure in this receipt was derived
from the pinned oracle corpus; every magnitude credited this cycle was
already transcribed and unit-tested against `data/corpus/**` by the
pre-existing `ground_<class>_class_features` functions this cycle only
made VISIBLE to the classifier, not computed anew. Cited for completeness
per the receipt schema.

## Status

**complete** — the assigned disposition-trace re-derived the full bucket-D
breakdown fresh, confirmed sub-mechanisms 3/4/5 have no open items beyond
what prior waves already dispositioned (and are not this cycle's to
re-mine), identified Shape 2's 154-unit magnitude-bearing remainder as the
cheapest untouched shape, and — finding the underlying gap was a real,
small, unambiguous matcher fix rather than genuinely-missing computation —
implemented it with full RED→GREEN proof, two live-caught safety guards,
and a verified 112-unit real closure (86 to DONE, 26 to V bucket). The
remaining 54 units are named precisely by class with the exact,
DIFFERENT failure shape (a word-choice synonym gap, not a dot-segment
gap) confirmed on 3 of 19 classes, honestly left for the next lane rather
than force-fit into this cycle's own narrower fix.

## Movement, four buckets

- **Closure:** 86 (D/X → DONE).
- **Reclassification:** 26 (D/X → V, a stronger bucket than DONE under this
  bundle's own taxonomy — genuine progress, tracked separately from
  "Closure" since V is not the DONE bucket by `completion_atlas.py`'s own
  definition) + 1 (evidence-string-only correction on an already-DONE unit,
  no bucket change).
- **Reachability:** 0 units newly reached or lost reachability.
- **Instrument-correction:** 1 (this cycle's own first-draft matcher
  widening, self-caught and corrected before commit, retro-logged) + 10
  `completion_atlas.py` citation pins re-derived (three separate times, as
  this cycle's own edit grew across the fix and its two safety guards).

## Notes (judgment calls)

- **Why the second-to-last-dot-segment check was preferred over extending
  `CLASS_FEATURE_ID_MAGNITUDE_SUFFIXES` with more words:** the Antipaladin
  family alone uses `dc`/`known`/`uses_per_day`/`damage_reduction`/
  `banishment_caster_level`/`save_bonus`/`selections` as descriptor words,
  and the wider dispatch chain adds dozens more (`power_points`,
  `powers_known`, `range_feet`, `custom_points`, `master_level`, ...) — an
  allowlist would need constant expansion and would still risk missing a
  real word, while a denylist of the two KNOWN non-magnitude markers
  (`unsupported`/`not_modelled`) is both smaller and complete against the
  corpus-wide `grep -c` this cycle ran (175+2 hits, zero of them a real
  magnitude word).
- **Why this was landed as one general matcher fix rather than 18
  per-class additions:** the underlying convention
  (`<owner>.<feature_slug>.<descriptor>`) is genuinely shared by the whole
  SD-32-card-11 dispatch family, confirmed by direct `grep` before writing
  any code, not assumed from one class — the SAME discipline sub-mechanism
  1's own fix (wave 36 lane A) and the domain-power bridge fix (wave 37
  lane A) already established for this bundle.
- **Why the two false positives were worth a temporary debug test rather
  than reasoning from the source alone:** the `arcanist`/`bloodrager`/
  `brawler` collision in particular was NOT discoverable by grep alone
  (`push_arcanist_exploits_deferred_diagnostic` and its sibling only push
  to `diagnostics`, never `explanations` — the REAL culprit,
  `class_chassis.arcanist.caster_level`, is emitted by a totally different,
  generic per-class table at a distant line). A live dump of the actual
  `facts.explanation_ids` set for a real character was the only way to find
  it with certainty rather than guess-and-check against a 30,000-line file.
- **Why `--allow-stamp-loss` was used on the corrected (second/third)
  regen:** the file it would have "lost" a stamp against was this cycle's
  OWN uncommitted, buggy first-draft regen (2 records incorrectly stamped
  `literal-verified` by the pre-guard version of this same cycle's own
  fix), never a committed, trusted prior state — traced and confirmed
  before using the flag, consistent with `AGENTS.md`'s "never
  `--allow-stamp-loss`" applying to silently overwriting REAL prior
  verification, not to self-correcting an error made earlier in the same
  uncommitted cycle.

## Next-cycle plan

1. **Shape 2's remaining 54 units** (table above) need a genuinely
   DIFFERENT fix: a per-feature word-choice/synonym audit (`AC Bonus` vs.
   `armor_class_bonus`, `Ki Pool` vs. `ki_points`, ...) for classes with an
   existing per-feature function (Unchained Monk 7, Unchained Barbarian 6,
   Unchained Rogue 4, Unchained Summoner 3 — spot-check Unchained Monk
   already confirmed the shape) — real Epic 3 work, smaller than sub-
   mechanism 5's chassis builds but not a one-line matcher change. The
   9 CRB-base/prestige classes (Monk 5, Duelist 4, Shadowdancer 4, Assassin
   2, Fighter 2, Loremaster 2, Wizard 2, Bard/Cleric/Druid/Paladin/Ranger/
   Sorcerer/Psychic 1 each — 27 units) need a first check for whether ANY
   per-feature compute function exists at all before assuming the same
   synonym shape.
2. **Sub-mechanism 5 (634/60 classes)** — unchanged, presumed claimed by
   lane A and/or lane B this same wave; re-confirm before dispatching if
   picked up by a later wave instead.
3. **Sub-mechanisms 3 (eidolon, 16) and 4 (sentinel, 1)** — fully
   dispositioned, no further action until their respective Epic 4/5
   chassis mechanisms (Summoner's Eidolon companion table; Ranger's
   combat-style-feat-chain, 151 siblings) are built.
