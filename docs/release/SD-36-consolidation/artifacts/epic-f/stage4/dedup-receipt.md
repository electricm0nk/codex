# F1b stage-4 -- the mechanical de-duplication join (R2/R3)

Spec: `epic-f-class-completion.md` §3b.2 (R2 corrected per review finding 2, R3 corrected per
review finding 6). Implementation: `src/rules_core/sheet_line_join.rs` (new module,
`rule_for_explanation` + `JoinResult`), wired into `src/rules_core/sheet_rule.rs`'s `held_set`
(the naive exact-tail walk this replaces used to live at the old `sheet_rule.rs:1960-1976`).
Population instrument: `class_census --duplicates <path>`
(`src/rules_core/class_census.rs::duplicate_scan`, `src/bin/class_census.rs`).

## 0. What changed, and why the literal spec text needed correcting again

`docs/release/SD-36-consolidation/artifacts/epic-f/stage4/desktop-print-paths.md` (copied into
this directory per this step's own instructions) was read first, per instruction. It named the
real join keys available (rule id, corpus `closure_rows` citation, display label) and confirmed
the two worked examples review finding 2 already diagnosed (`brawler_knockout_dc` and
`fighter.weapon_and_armor_proficiency`) are real, not hypothetical.

Implementing the spec's literal one-shot "join the whole remaining segment string by longest
common prefix" and then running it against the REAL population (`class_census --duplicates`,
not a synthetic sample) surfaced three more real shapes the literal spec undersells, each fixed
mechanically (no per-class special case) and each backed by a regression test in
`sheet_line_join.rs` against the REAL committed `data/sheet_rules/`:

1. **Shared, class-agnostic rule text.** PF1 sometimes writes a rule once and several classes
   grant it (`uncanny_dodge`, Rogue's base rule Ninja also borrows) -- the real record carries
   NO class prefix at all. A pure class-scoped join can never reach these. Fix: a second BARE
   tier, tried only when the scoped tier finds nothing, matching the tail alone against any
   `class_feature` slug -- but ONLY when the candidate's words fully consume the WHOLE tail (a
   partial bare match is refused), which is what keeps this tier from ever drifting into an
   unrelated class's rule by a one-word coincidence.
2. **Nested sub-feature names.** Some facet ids name a parent sub-feature ahead of the actual
   attribute (`class_feature.untabled.dread.dread_manifesting.power_points`) -- the tail's own
   first dot-segment is itself a feature name, and the true target
   (`ultimate_psionics:class_feature:dread_power_points`) only becomes reachable once that
   parent segment is dropped. Fix: a sliding window over dot-segment boundaries (dot segments
   are the id author's own structural boundaries; underscore words inside one segment are not),
   tried from the fullest tail down to progressively shorter suffixes, every qualifying match at
   every window position competing on the same (coverage, excess, tier) scale.
3. **Genuinely doubled class names.** Some real, EXISTING records repeat the class word in their
   own slug (`ultimate_intrigue:class_feature:vigilante_vigilante_specialization`,
   `ultimate_combat:class_feature:gunslinger_gunslinger_initiative` -- confirmed by directory
   listing, not typos), while OTHER facet ids merely re-embed the class name redundantly in
   their own tail text without a matching doubled-name record
   (`tactician_manifesting`/`dread_manifesting`, both single, undoubled real slugs). Collapsing
   the doubled word unconditionally (an earlier draft of this fix) hid the first case behind an
   `Ambiguous` tie among the record's own sub-options. Fix: try the RAW (uncollapsed) tail
   first -- an exact doubled-name record wins outright, on excess, over any collapsed
   interpretation -- and offer the COLLAPSED tail only as an additional, never-replacing,
   alternative, credited fully only when it explains the ENTIRE collapsed tail.

None of this is per-class: every rule above is a general property of dot-segment structure,
word-prefix scoping, and full-tail consumption, applied identically regardless of which class or
book is involved -- verified by running it against all 313 builds/2014 facets below, not by
special-casing the specific records the population scan happened to surface.

`JoinResult` stays the 3-arm type review finding 2 specified: `Matched(RuleId)`,
`Ambiguous(Vec<RuleId>)`, `None` -- both `Ambiguous` and `None` mean "no converted rule for this
facet" to `held_set` (the bespoke explanation keeps printing alone; R3's "who wins" question
never arises for it).

## 1. RED-first tests (`cargo test --locked -j 8 --lib sheet_line_join`, 16 tests, all green)

Named per this step's own RED-FIRST list, plus the additional real cases the population scan
surfaced:

- `the_three_known_good_pairs_still_join` -- `fighter_bravery`, `fighter_armor_training`,
  `brawler_ac_bonus` (the naive spec's own accidental successes; a regression guard).
- `a_facet_id_never_joins_to_the_class_principal_rule` -- `brawler_knockout_dc` joins
  `brawler_knockout`, never `brawler`; the `fighter.weapon_and_armor_proficiency` reversed-order
  case is asserted `None`/`Ambiguous`, never `Matched(fighter)` (both the synthetic-package and
  real-package variants).
- `real_package_barbarian_and_rogue_uncanny_dodge_join_the_base_rule_not_the_tracker` --
  Barbarian/Rogue Uncanny Dodge join their BASE rule, not the `_tracker` sibling (an exact,
  fully-consumed stem beats a longer candidate that merely starts the same way).
- `an_exact_stem_wins_over_a_longer_sibling_that_merely_starts_the_same_way` -- the synthetic
  version of the same invariant.
- `two_candidates_with_the_same_leftover_past_the_same_prefix_are_ambiguous` -- a genuine tie is
  surfaced, not silently resolved.
- `the_join_never_crosses_classes` -- a same-tailed rule under a DIFFERENT class's own prefix is
  never a candidate.
- `class_slug_alone_is_never_a_valid_match_length`,
  `an_explanation_id_that_never_names_this_class_is_refused_not_guessed`,
  `corpus_record_ids_still_join_through_the_family_marker` -- the remaining named edge cases.
- `real_package_ninja_joins_the_bare_shared_uncanny_dodge_rules` -- the BARE tier (shape 1
  above).
- `real_package_a_nested_sub_feature_name_does_not_bury_the_real_attribute_rule` -- the sliding
  window (shape 2).
- `real_package_a_doubled_class_name_in_the_tail_is_collapsed_not_left_to_win`,
  `real_package_a_genuine_doubled_class_name_rule_wins_over_its_own_sub_options`,
  `real_package_a_doubled_class_name_facet_id_with_no_collapsed_alternative_still_joins` --
  the raw-vs-collapsed handling (shape 3), both directions.
- `real_package_a_full_bare_match_outranks_a_shorter_scoped_partial_match` -- Oracle's
  `life_mystery_healing_hands` (a full, class-agnostic bare match) correctly outranks the
  shorter, class-scoped `oracle_life_mystery` (the mystery-selector rule) partial match.

Also green: `cargo test --locked -j 8 --lib sheet_rule` (58 tests, including two new
`held_set`/`render_sheet`-level regression tests --
`a_joinable_facet_holds_and_prints_its_rule_exactly_once` and
`an_unjoinable_facet_never_falls_back_to_the_class_principal_and_never_disturbs_other_held_rules`),
and the full `cargo test --locked -j 8 --lib` (2659 tests, 0 failed, 7 ignored, over the
CURRENTLY TRACKED, pre-link-repair `data/sheet_rules/` -- confirming this commit changes no
shipped behavior yet, see §4).

## 2. Population scan (`class_census --duplicates`)

**Denominator, exactly, stated by the report itself
(`<scratch>/duplicates-after.json:population`):** every non-prestige census entry at its own
`max_level`, every prestige class's first-named carrier build at its own `max_level`, and every
row of the committed 185-row multiclass mix panel -- **313 builds, 2014 `class_feature.*` facets
scanned** (the count of `(class, explanation_id)` pairs `HeldSeed::from_character` extracts from
those 313 builds' own chassis explanations). Run against the POST-repair package
(`<scratch>/dump-after`, temporarily `rsync`ed over `data/sheet_rules/` and restored with
`git checkout -- data/sheet_rules && git clean -fd data/sheet_rules`; `git status --short` was
empty before the swap and empty again after both swaps in this step, and the residue gate
(`python3 scripts/pcgen_residue_gate.py --check --closure`) reported `verdict=PASS` after the
final restore).

Command: `class_census --duplicates <scratch>/duplicates-after.json`. Full per-facet rows saved
to `<scratch>/duplicates-after.json`; a BEFORE-only projection (the pre-R2 naive join's own
answer for every one of the same 2014 facets) saved to `<scratch>/duplicates-before.json`.

| Metric | BEFORE (pre-R2 naive walk) | AFTER (corrected join) |
|---|---|---|
| Matched | 514 | 1121 |
| Ambiguous | n/a (algorithm had no ambiguity concept) | 671 |
| None | 1500 | 222 |
| **Joined to the class's own PRINCIPAL rule** | **0** | **0 (structurally refused)** |

**`before_principal_mismatches = 0`, both before and after.** Running the ACTUAL pre-R2 code
(`class_census.rs::pre_r2_naive_join`, a byte-accurate measurement-only reproduction of the old
`sheet_rule.rs` walk, verified against `git show HEAD~N:src/rules_core/sheet_rule.rs` before this
step's own edit) over the real population never once produced the class-principal-collapse
review finding 2's prose describes ("drops the last segment and matches `brawler`") -- the OLD
code drops LEADING segments as its index increases, not the last one, so it could only ever land
on a bare `{class}` candidate when the WHOLE tail was empty, which `HeldSeed::from_character`
already excludes via `.unsupported` filtering upstream. This is stated here, with the evidence,
rather than left as an unverified inherited claim: the harm review finding 2 named (a mis-join to
the wrong rule) is real and measured below (**`changed_by_r2 = 607`** facets, i.e. 30% of the
population get a DIFFERENT answer under the corrected join than the old one), but the SPECIFIC
"collapses onto the principal" mechanism as literally described never fires in the shipped code
-- the corrected join's own explicit, redundant refusal of the principal (both by `min_len` and
by an explicit id check) is kept regardless, since a caller's own segment walk landing there by
accident is exactly the failure mode the test `a_facet_id_never_joins_to_the_class_principal_rule`
pins against, independent of whether THIS specific historical mechanism was ever the live path.

**Transition breakdown** (every row's pre-R2 answer vs the corrected join's answer, `<scratch>/analyze_dup_scan.py`):

| Transition | Count | What it means |
|---|---|---|
| `none -> matched` | 607 | The fix reaches a real target the old walk missed entirely -- the corrected join's whole point. |
| `none -> ambiguous` | 671 | Newly reachable, but the join genuinely cannot pick one candidate -- surfaced, not guessed (see §3). |
| `some -> same matched` | 514 | Unchanged: every facet the old walk got right, the new one still gets right. |
| `none -> none` | 222 | Still correctly unjoinable (see §3 -- mostly domain powers, a facet shape outside this join's scope). |
| `some -> none` | 0 | **Zero regressions**: nothing the old walk matched is now silently dropped. |
| `some -> ambiguous` | 0 | **Zero regressions**: nothing the old walk matched is now silently made ambiguous. |
| `some -> DIFFERENT matched` | 0 | **Zero regressions**: every facet the old walk matched to something, the new join either agrees or (in the transitions above/below) was already accounted for. |

Two earlier drafts of this join (single-pass scoped-only, then scoped+bare with unconditional
collapsing) DID regress several already-correct old answers -- `ninja` Uncanny Dodge, the
`dread`/`cryptic`/`marksman`/`psion`/`psychic_warrior`/`vitalist`/`wilder`/`tactician` psionics
`_manifesting`/`_power_points` pairs, Oracle's `life_mystery.healing_hands`, and (after adding the
sliding window) `vigilante_specialization`, `shifter_aspect`, `psychic_discipline`,
`magus_arcana`, `gunslinger_gunslinger_initiative`, `mesmerist_mesmerist_tricks`,
`shifter_shifter_claws` -- each was root-caused against the real corpus (not guessed) and fixed
by the RAW-before-collapsed ordering in §0.3; the final run above shows all of them closed
(0 regressions in every category). This is why the population scan ran three times over this
step, not once -- `<scratch>/duplicates-after.log`/`after2.log`/`after3.log` are each run's raw
output, in order.

## 3. Unjoinable pairs, named (not dropped)

**Ambiguous (671 rows, 46 distinct `(class, explanation_id)` facets):**

| Facet | Rows | Real cause |
|---|---|---|
| `class_feature.<class>.weapon_and_armor_proficiency` (fighter, rogue, bard, ranger, and 10 more CRB classes) | 312 of 671 | The real converted records for this concept are named in REVERSED word order (`weapon_and_armor_proficiency_<class>`, confirmed `desktop-print-paths.md` §2d) across 14 different classes; the bare tier correctly finds ALL 14 as equally-good full-tail matches (none carries the requesting class's own prefix, so nothing breaks the tie) -- a real, class-crossing naming inconsistency in the CONVERTED PACKAGE ITSELF, not a join defect. Also confirmed harmless today: `render_sheet` filters on `r.print`, and every one of these 14 records is `print: false` (§2d), so this ambiguity never reaches a printed line regardless. |
| `class_feature.sorcerer.bloodline.generic.arcane_bloodline.*` (9 distinct facets, 20 rows each) | 180 of 671 | Sorcerer's generic bloodline-tracker facet ids are templated per-bloodline placeholders (`sorcererarcanebloodlinefeat*`, `sorcererarcanemetamagicadepttimes`, ...) that legitimately tie across several real per-bloodline-feat-tracker records sharing the same word stem. Named here; a real follow-up for whoever owns the sorcerer bloodline facet-id shape specifically (outside this step's file ownership). |
| everything else (35 facets) | 179 of 671 | Spot-checked (`<scratch>/analyze_dup_scan.py`'s per-facet listing): the same two shapes above, recurring across sibling classes (bloodrager/psychic/vigilante/shifter template families) and levels. |

**None (222 rows, 73 distinct facets):** the largest single group,
`class_feature.domain.<domain>_<ability>_*` (25 rows for `cleric`), never names a `class_feature`
scope segment equal to the requesting class slug at all (`domain` is not `cleric`) -- refused by
this join's own "an id that never names this class is never guessed at" rule
(`an_explanation_id_that_never_names_this_class_is_refused_not_guessed`), correctly: domain
powers are a DIFFERENT facet shape this join is not scoped to interpret, not a class-feature
join failure. The rest (`sorcerer` bloodline sub-attribute facets, `druid` animal-companion
`_vacuous` placeholders, and ~60 one-off facets across 25 other classes) are named in
`<scratch>/duplicates-after.json`'s full row list for the orchestrator; none of them collapses
onto a class principal (§2's `before_principal_mismatches`/`after` both 0).

## 4. Stage-3 gate re-run (spec 3b.3, the 70-build sample)

Re-ran `render_before_sheets.sh` (new, this step -- the baseline half `render_after_sheets.sh`
already had; same 70-build manifest, current TRACKED `data/sheet_rules/`, no swap) and
`render_after_sheets.sh` (swap + restore, as before) with the R2-fixed `class_census` binary,
then `classify_sheet_diff.py` over both:

```
gate_pass=False totals={'added_correct': 239, 'duplicate': 8, 'changed_value': 9, 'removed': 0, 'unclassified': 0}
removed_unexplained=0
```

(`<scratch>/blast-radius-classify-r2.json`, `<scratch>/blast-radius-receipt-r2.md`.)

- **`duplicate=8`: IDENTICAL to the original stage-3 run** (same builds, same ids: Barbarian/Rogue
  Uncanny Dodge `_tracker` sibling x7, Paladin Aura of Righteousness x1). Confirmed by direct
  trace, NOT fixed by R2 and NOT supposed to be: the Uncanny Dodge trackers are held via the F1
  converter's own `granted_by` edges (an unrelated grant mechanism, already landed in prior
  commits on this branch), never through `HeldSeed`'s `class_features` facet walk this join
  replaces -- and R3 (3b.2, corrected per review finding 6) explicitly rules this shape
  "two distinct oracle records, both print (that is what the book says)". The stage-3
  classifier's own text-similarity proxy still flags them because it has no `rule_for_explanation`
  call of its own (by design, unchanged from the original run) -- these are reported, not hidden.
- **`changed_value=9`: NEW since the original run (was 0), fully root-caused, and VERIFIED
  CORRECT against the PF1 rules, not a defect.** All 9 are `core_rulebook:class_feature:
  standard_rage#bonus1/2/3` ("Rage" Will/Str/Con bonuses) at `barbarian:14`, `barbarian:20`, and
  `mix_barbarian12_fighter1`. Before (pre-repair package): `+2/+4/+4` at EVERY level tested
  (14 and 20 alike) -- BASE Rage only, regardless of level. After (post-repair package, same R2
  join): `+3/+6/+6` at level 14 and `+4/+8/+8` at level 20. Cross-checked against the actual PF1
  Core Rulebook Rage progression: Rage is `+4 Str/+4 Con/+2 Will` at levels 1-10, Greater Rage
  (granted at barbarian level 11) is `+6/+6/+3`, Mighty Rage (level 20) is `+8/+8/+4`. **The
  AFTER values are the textbook-correct Greater/Mighty Rage numbers; the BEFORE values are
  wrong -- stuck at base Rage past level 11 in the CURRENTLY SHIPPED, pre-repair package.** `EXPL`
  is confirmed byte-identical before/after (no bespoke `pilot_compute` module computes a
  competing Rage-bonus number at all -- these are LINE-only, converted-path values), and
  `barbarian:1`/`barbarian:7` show no change (both below Greater Rage's level-11 threshold,
  consistent with the fix only firing where it should). This is F1's converter fix (already
  landed, "every converted rule reaches the live package") delivering a previously-inaccessible
  Greater/Mighty Rage sibling record that ONLY the corrected join can now actually reach and
  fold into the shared Rage variable -- a real, previously-invisible understated-bonus bug the
  OLD naive join's failure to join in EITHER before or after was masking (nothing differed, so
  stage-3's original delta-only methodology reported `changed_value=0` while the true value was
  silently wrong on both sides). **This commit does not ship the fix**: `data/sheet_rules/` is
  not regenerated in this stage (per this step's own INVARIANTS), so the currently tracked
  package still lacks the Greater/Mighty Rage sibling record the fix would reach -- confirmed by
  the full `cargo test --locked --lib` run (2659 green) over the TRACKED package showing no
  behavior change. This is a verified, positive, FORWARD-looking finding for whoever regenerates
  `data/sheet_rules/` next (F1's own later population-commit step), not a blocker here.
- `removed=0`, `removed_unexplained=0`, `unclassified=0`: clean.

**`gate_pass=False` is accurate but not a blocker for this step**: both non-zero categories are
individually investigated above (not "fixed and hidden" -- doctrine forbids that), neither is
caused by a defect in the join itself, and both are the SAME 8+9=17 real, already-understood
rows every time (not a new, unexplained population). The gate's own blanket
`duplicate=0, changed-value=0` acceptance number was written before `rule_for_explanation`
existed and before F1's converter fix could be reached at all; §12 review-log style: this is
reported to the orchestrator as a named, non-blocking finding, not silently resolved.

## 5. Known follow-ups (named, not silently dropped)

- **Frontend TS mirror (`apps/desktop/src/characterHub/classFeaturesModel.ts`'s
  `noticeHasSheetRule`)**: `desktop-print-paths.md` item 1c confirms this function is its OWN
  hand-written copy of the naive `{class}_{tail}`/`{tail}` walk, now stale relative to R2. Spec
  3b.2 says the frontend should use "the `rule_id` the DTO already carries" rather than
  re-deriving the join client-side. This is genuinely frontend TypeScript work, outside this
  step's file ownership (desktop backend only, and only conditionally on a desktop-only
  collision -- confirmed NOT needed: the desktop Rust backend calls the SAME
  `HeldSeed::from_character`/`held_set` this step already fixed, `character_hub.rs:778,783`, so
  no separate Rust change was needed there). Named for whoever picks up F4 (Desktop) or a
  dedicated frontend follow-up.
- **The reversed-word-order `weapon_and_armor_proficiency_<class>` naming (14 records, §3)** is a
  converter/ingest-side naming inconsistency, not a join defect -- harmless today only because
  every one of the 14 records is `print: false`. Named for whoever owns the converter's class
  weapon/armor proficiency naming.
- **Sorcerer's generic bloodline-tracker facet-id shape and the domain-power facet-id shape
  (§3)** are both facet-id conventions this join is correctly scoped NOT to interpret (bloodline
  templating; no class segment at all for domain powers). Named for whoever owns those specific
  facet-id shapes.

## 6. Commands run, in order

```
cargo test --locked -j 8 --lib sheet_line_join        # 16/16 green
cargo test --locked -j 8 --lib sheet_rule              # 58/58 green (2 new held_set/render_sheet regression tests)
cargo test --locked -j 8 --lib                         # 2659/2659 green, 7 ignored (tracked package, no behavior change)
cargo clippy --locked -j 8 --lib --bins -- -D warnings # clean
python3 scripts/pcgen_residue_gate.py --check --closure  # verdict=PASS
class_census --duplicates <scratch>/duplicates-after.json  # x3 (see §2), post-repair package (swap+restore)
class_census --json <scratch>/census-before.json           # tracked package
class_census --json <scratch>/census-after2.json           # post-repair package (swap+restore)
render_before_sheets.sh / render_after_sheets.sh           # 70-build manifest, both halves
classify_sheet_diff.py --before ... --after ... --census-before ... --census-after ...
```
