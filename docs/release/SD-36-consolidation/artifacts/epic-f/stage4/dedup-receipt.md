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

**SUPERSEDED by the stage-4 fix pass, §7.6.** The two tables immediately below are the R2-only
snapshot (before the stage-4 blocker-1/blocker-2 fixes existed on this branch) and are kept here
verbatim for history; they are NOT the shipped join's current answer. §7.6 restates both tables
against the actually-shipped code, with every transition since this snapshot named.

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
  dedicated frontend follow-up. **RESOLVED at the stage-4 fix pass, §7.5**: this was flagged by
  that step's own adversarial check as a live desync (not just a named follow-up), and fixed
  there.
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

## 7. Stage-4 fix pass (adversarial check, 5 blockers)

Spec: same as above. Blockers: `<scratch>/stage4-blockers.json`, from the stage-4 adversarial
check. Fixed at their root, one at a time, RED test first, then re-verified over the real
population and the real sheet render.

### 7.1 Blocker 1 -- scoped-tier and collapsed-tail one-word coincidences (CONFIRMED, fixed)

Root cause, as diagnosed: the collapsed-tail arm credited a partial match with the WHOLE
original tail's coverage merely for consuming the (shorter) collapsed tail. Fixed by capping
collapsed-tail credit at the words actually explained (never promoted), and adding one shared
refusal rule to BOTH the raw-scoped and collapsed-scoped tiers: a candidate that explains
strictly less than its tail AND still has its own unexplained leftover words is refused outright
(dropped before scoring), never scored into a confident `Matched`.

Tracing the ACTUAL live mechanism (not assumed from the finding's own diagnosis) found a second,
independent source of the same defect the finding's evidence did not name: the BARE tier. Its
"whole tail is a matched PREFIX of the candidate" rule let a candidate with its own extra
trailing words win (`magus_arcana_pool` -> `magus_arcana_pool_strike`, `strike` never asked for)
purely because it happened to have the highest raw `coverage` of any tier. Confirmed by
instrumenting the live join and re-running the `magus_arcana_pool` case: BEFORE this
sub-fix, the bare-tier candidate (coverage=3, excess=1) beat the properly-refused collapsed-tier
candidate outright on the primary `coverage` key -- the collapsed-arm fix alone was
insufficient. Fixed by requiring the bare tier's own match to be EXACT (`excess == 0`): the
whole tail consumed AND no leftover candidate words, not merely a prefix relationship.

RED tests added (`sheet_line_join.rs`, all against the REAL committed `data/sheet_rules/`):
- `real_package_magus_arcana_pool_never_joins_the_unrelated_pool_strike_arcana`
- `real_package_skald_raging_climber_and_swimmer_never_join_raging_song_on_one_word`
  (also covers `raging_leaper`, the finding's own third worked example)
- `a_partial_match_with_its_own_leftover_words_is_refused_not_tied` (synthetic regression guard)
- `two_candidates_that_both_fully_explain_the_tail_and_only_then_diverge_are_ambiguous`
  (renamed/re-scoped from the pre-fix `two_candidates_with_the_same_leftover_...` test, which
  used a tail the fix now correctly refuses to `None` -- the genuine-tie invariant this test
  pins is still real and still reachable, just at a tail length where BOTH candidates fully
  explain it)

All three of the finding's confirmed-wrong joins are now closed -- **correction (stage-4
blocker-2, §7.6): the magus row below was originally stated as `None` here; it is actually
`Matched(magus_magus_arcana)`, a DIFFERENT, better match the fix's own refusal rule newly makes
reachable (the previous, wrong candidate `magus_arcana_pool_strike` is gone either way; the row
retargets rather than empties). Reproduced fresh against the committed tree (row
`magus:20` in `duplicates-after-fix.json`, `HELD|`/`LINE|` at `<scratch>/sheets/post_r2_fixed/
magus_L20.txt` lines 121/160):**

| Facet | Wrong (pre-fix) | Corrected (post-fix) |
|---|---|---|
| `class_feature.untabled.magus.magus_arcana.pool` (magus:20) | `Matched(magus_arcana_pool_strike)` | `Matched(magus_magus_arcana)` |
| `class_feature.acg.skald.raging_climber` (skald:20) | `Matched(skald_raging_song)` | `None` |
| `class_feature.acg.skald.raging_swimmer` (skald:20) | `Matched(skald_raging_song)` | `None` |

### 7.2 Blocker 2 -- bare-tier principal guard (CONFIRMED, fixed)

Root cause confirmed exactly as diagnosed: the bare tier's own comment claiming a bare
class-slug-only candidate "could never fully consume a real (non-empty) tail" is false whenever
a later dot-segment equals the class slug (`class_feature.<class>.<class>`, or any nested
sliding-window position landing there) -- the sliding window makes this reachable, and the bare
tier had no guard against it. Fixed: `if slug == class_slug { continue }` added to the bare-tier
loop, matching the scoped tier's own guard.

RED test: `bare_tier_never_returns_the_class_principal_rule_either` -- both the direct
`class_feature.fighter.fighter` shape and a nested `class_feature.fighter.x.fighter` window,
against a package containing ONLY the class principal rule (so a pre-fix run would provably
return `Matched(fighter)`, not `None`, at the bare tier alone).

### 7.3 Blocker 3 -- population scan denominator (CONFIRMED, fixed)

Root cause confirmed exactly as diagnosed: `prestige_applies_gate`/`determine_carriers` failures
were silently `continue`d in `duplicate_scan`, dropping 7 of 74 prestige classes from
`builds_scanned`'s denominator with no count and no name. Fixed: `duplicate_scan`'s prestige
loop now calls the ALREADY-NAMED `carrier_assignment` (the same named-`Err` function
`census --json`'s `carrier_assignment_summary` already uses elsewhere), collects every skip's
own reason string into a new `DuplicateScanReport::prestige_skipped: Vec<String>` field, and the
`population` string is corrected to state the true denominator: **"every non-prestige census
entry @ own max_level + N of 74 prestige carriers (M named, no carrier determinable) @ own
max_level + the full multiclass mix panel"**. `class_census --duplicates`'s one-line summary now
also prints `prestige_skipped=[...]`.

Re-run: `prestige_skipped` names exactly the 7 classes the finding's own evidence named --
`dark_tempest`, `dragon_disciple`, `elocater`, `evangelist`, `psion_uncarnate`,
`pure_legion_enforcer`, `thrallherd` -- each with `determine_carriers`'s own reason ("gate
references a caster level or spell-kind term only inside a Not/AtLeast clause..."). Population
string now reads `"... + 67 of 74 prestige carriers (7 named, no carrier determinable) @ own
max_level + ..."`. `builds_scanned` itself is UNCHANGED at 313 (the skip logic was always
skip-and-continue; only the naming was missing), confirming this was a reporting defect, not a
population-size defect.

### 7.4 Blocker 4 -- the 70-build gate's blind spot to the join itself (CONFIRMED, fixed)

Root cause confirmed exactly as diagnosed: stage-3's re-run rendered BOTH halves with R2 already
in the tree, so every newly-joined facet was identical on both sides and the classifier's
delta-only methodology could not see it -- `class_census --duplicates`'s own before/after
columns (which DO isolate the join, since `pre_r2_naive_join` and `rule_for_explanation` are run
against the identical package for every row) were already the right instrument for the join
itself; §2/§7.1-7.3 above are that evidence. What §4 of the original receipt could not do is
show the join's effect in the actual RENDERED SHEET TEXT a player reads -- so this step produced
that render, genuinely isolating the join as the only variable:

1. `src/rules_core/sheet_rule.rs`'s ONE join call-site was TEMPORARILY swapped (never committed)
   to call a byte-accurate copy of `pre_r2_naive_join` instead of `rule_for_explanation`; a
   `class_census` binary was built from that tree and copied out
   (`<scratch>/class_census_pre_r2_baseline`); the source edit was then reverted
   (`git checkout -- src/rules_core/sheet_rule.rs`, confirmed clean) BEFORE building the real,
   fixed `class_census` binary (`<scratch>/class_census_post_r2_fixed`) from this step's actual
   committed code.
2. Both binaries rendered the SAME 70-build manifest against the SAME package (the post-repair
   `<scratch>/dump-after`, swapped in via the documented `rsync` + `git checkout --`/`git clean
   -fd` procedure, confirmed `git status --short data/sheet_rules` empty before and after both
   swaps): `<scratch>/sheets/pre_r2_baseline/*.txt` (naive walk) and
   `<scratch>/sheets/post_r2_fixed/*.txt` (this step's corrected join). The join is now the ONLY
   variable between the two render sets.
3. `classify_sheet_diff.py --before pre_r2_baseline --after post_r2_fixed` (§4's own tool,
   pointed at these two new directories):

```
gate_pass=False totals={'added_correct': 0, 'duplicate': 0, 'changed_value': 0, 'removed': 0, 'unclassified': 4} removed_unexplained=0
```

(`<scratch>/blast-radius-classify-stage4fix.json`, `<scratch>/blast-radius-receipt-stage4fix.md`.)

This is the genuine pre-R2-vs-post-R2-fixed delta the finding asked for -- and this time it
really does isolate the join: **66 of 70 builds are byte-identical** (verified by direct `diff`
over every pair, not just the classifier's own summary); the other 4
(`oracle_L1`/`L7`/`L14`/`L20`) each gain exactly one new HELD/LINE pair,
`advanced_players_guide:class_feature:oracle_clouded_vision` ("Clouded Vision"), and nothing
else changes anywhere (0 duplicate, 0 changed-value, 0 removed, across all 70 builds).

The classifier reports these 4 as `unclassified`, not `added_correct` -- its own ADDED-CORRECT
check requires a `granted_by` edge on the record to auto-verify the character legitimately
holds it, and `oracle_clouded_vision`'s converted record carries none (a separate, pre-existing
gap in that record's own conversion, unrelated to the join). Hand-traced instead: the facet is
`class_feature.apg.oracle.clouded_vision_curse.vision_range_cap` -- its own tail literally names
the Clouded Vision curse, matching only `oracle_clouded_vision` and no other record, at every
level tested (Oracle 1/7/14/20 all carry this curse facet). Confirmed correct, not blindly
accepted.

Cross-checked against the population scan: the population scan's own 313-build set includes
only `oracle:20` (its census `max_level`), not `oracle:1`/`7`/`14` (those three exist only in
this 70-build manifest) -- so `duplicates-after-fix.json` names this exact facet as
`none -> matched` for `oracle:20` alone, and this render independently confirms the SAME fix
reaches the SAME rule correctly at three MORE levels the population scan never swept. Zero
regressions (no `some -> none`/`some -> ambiguous`/`some -> DIFFERENT matched` anywhere in
either instrument) is the headline result the finding asked this render to actually be able to
show.

**Hand-audit of newly-matched facets.** A fixed-seed (`20260921`) random sample of 40 of the
597 `before=None -> after=Matched` facets from the re-run population scan
(`<scratch>/duplicates-after-fix.json`) was drawn and eyeballed for target-slug plausibility
against the facet's own tail words (`<scratch>/adv_join_fixed.py`'s own printed sample) --
every sampled pair is a legible, on-topic stem match (e.g.
`eidolon.bite_damage_die -> summoner_eidolon`,
`pu.unchained_barbarian.rage_powers_known -> unchained_barbarian_rage_powers`,
`vigilante_specialization.pool -> vigilante_vigilante_specialization`), none reproduces the
one-word-coincidence shape §7.1 fixed.

**Independent cross-validation.** `<scratch>/adv_join_fixed.py`, a from-scratch Python
reimplementation of the CORRECTED algorithm (every rule mirrored from the Rust source, not
copy-pasted), run over the same 2014-facet population: **0 mismatches** against the Rust join's
own `after` column for every one of the 1071 distinct `(class, explanation_id)` facets in the
population. This is a second, independent implementation agreeing byte-for-byte with the fixed
Rust code, not the same code re-run.

### 7.5 Blocker 5 -- desktop frontend join desync (CONFIRMED, fixed)

Root cause confirmed exactly as diagnosed: `classFeaturesModel.ts`'s `noticeHasSheetRule` was
still the pre-R2 exact-slug walk (`candidates = [\`${classToken}_${feature}\`, feature]`),
untouched by the Rust-side R2 fix, so a facet the Rust join now matches to a real STEM rule kept
BOTH its printed rule line and its "not computed" notice.

Fixed by porting `rule_for_explanation` to TypeScript
(`classFeaturesModel.ts::ruleForExplanation`) -- the same two tiers, the same sliding window,
the same collapsed-tail credit cap, the same partial-match-with-leftover refusal, the same
explicit principal-rule guard on both tiers -- and wiring `noticeHasSheetRule` to call it
instead of the stale candidate list. Per spec 3b.2's own instruction ("through a `rule_id` the
DTO already carries as the line id -- no new wire field"), candidates are the character's own
currently-rendered `class_feature` sheet lines (`SheetLineDto.id`), not a new wire field; this
can only make the join MORE conservative than the Rust original (every refusal rule is a
standalone property of one candidate's own shape, never a comparison against other candidates),
and the winning candidate for any facet the Rust join actually holds is, by construction, always
present among these lines (`HeldSeed::from_character` already ran this same join server-side for
every `class_feature.*` facet and held/printed whatever it matched).

RED tests (`classFeaturesModel.test.ts`):
- `verifiesANoticeIsDroppedByTheSameStemMatchTheRustJoinUses` -- spec 3b.2's own worked example
  (`brawler_knockout_dc` -> `brawler_knockout`): pre-fix, the notice would have survived
  alongside the printed line (the exact desync the finding named); post-fix, the notice is
  dropped.
- `verifiesAOneWordCoincidenceNeverDropsTheNotice` -- the `raging_climber`/`skald_raging_song`
  shape §7.1 fixed, mirrored on the frontend: a one-word coincidence must never suppress a real
  notice either.

`src/rules_core/sheet_line_join.rs`'s own module doc comment previously (and incorrectly)
claimed the frontend already called this function directly ("no second copy anywhere") -- false
as of the finding's own evidence. Corrected to name the TypeScript port explicitly and why a
wire boundary makes a literal shared call impossible.

## 7.6 Stage-4 blocker-2 close-out: the 11 dropped matches, the duplicate ruling

Spec: same as above. Blockers: `<scratch>/stage4-blockers-2.json` (2 findings, both against
THIS receipt's own text, not the join's behavior). Both fixed at their root; the second finding
also surfaced a real, unnamed side effect (the 11 dropped matches) that needed closing.

### 7.6.1 The 11 dropped matches, named with mechanism

Root-caused by diffing `<scratch>/duplicates-after.json` (the population scan from BEFORE the
blocker-1 fix, i.e. R2 with the one-word-coincidence bug still live) against
`<scratch>/duplicates-after-fix.json` (AFTER blocker-1, before this fix pass) by
`(class, explanation_id)` key -- reproduced fresh here, not quoted from the earlier finding:
**11 rows go `matched -> none`**, 1 retargets (magus, §7.1's own correction above), 1
`ambiguous -> matched` (an unrelated, separately-closed improvement). Every one of the 11 has
`before: None` in the ORIGINAL pre-R2 naive walk too (`duplicates-after.json`'s own `before`
field), so none of the 11 is a shipped regression -- all were unreachable before R2 existed at
all, and blocker-1's refusal rule cost the population 11 of the NEW gains R2 itself opened up,
not 3.

**Mechanism, per row.** Word-for-word comparison of each facet's tail against its target's own
slug (`class_slug` stripped) shows exactly two shapes:

| # | Facet (class.tail) | Dropped target | Shape |
|---|---|---|---|
| 1 | `druid.resist_natures_lure` | `druid_resist_nature_s_lure` | TOKENISATION: the corpus's own possessive-apostrophe convention (`nature's` -> `nature_s`, a bare `"s"` word-token) vs. the facet's own merged `natures` |
| 2 | `brawler.bonus_feat_count` | `brawler_bonus_feats` | TOKENISATION: plural/singular (`feat` <-> `feats`); `_count` is a facet-only qualifier over an otherwise fully-consumed stem |
| 3 | `swashbuckler.bonus_feat_count` | `swashbuckler_bonus_feats` | Same as #2 |
| 4 | `unchained_monk.bonus_feats_known` | `unchained_monk_bonus_feat` | TOKENISATION: plural/singular (`feats` <-> `feat`); `_known` is a facet-only qualifier |
| 5 | `unchained_monk.style_strikes_known` | `unchained_monk_style_strike` | TOKENISATION: plural/singular (`strikes` <-> `strike`); `_known` is a facet-only qualifier |
| 6 | `asavir.efreeti_blessing_mount.fire_resistance` | `asavir_efreeti_s_blessing_mount` (a MORE specific sibling than the facet's own literal `asavir_efreeti_s_blessing`, see below) | TOKENISATION: possessive-apostrophe (`efreeti's` -> `efreeti_s`), through the sliding window |
| 7 | `cavalier.bonus_combat_feat_count` | `cavalier_bonus_feat` | GENUINE DIVERGENCE: the facet names an extra descriptive word ("combat") the rule's own slug never uses at all -- not a plural/apostrophe variant, a different word placed mid-tail |
| 8 | `bloodrager.uncanny_dodge_flanking_level` | `bloodrager_uncanny_dodge_tracker` | GENUINE DIVERGENCE: the facet's trailing words ("flanking_level") and the rule's own trailing word ("tracker") name different concepts past the shared "uncanny_dodge" stem |
| 9 | `unchained_monk.flurry_attack_count` | `unchained_monk_flurry_of_blows` | GENUINE DIVERGENCE: "attack" and "of_blows" are unrelated words, not a tokenisation of the same word |
| 10 | `skald.raging_climber` | (refuses; the finding's own case) | one-word coincidence ("raging"), correctly still refused |
| 11 | `skald.raging_swimmer` | (refuses; the finding's own case) | Same as #10 |

**The fix (one mechanical rule, two normalisation classes, never a per-facet list):**

1. **Possessive-apostrophe merge** (`sheet_line_join.rs::words`): a lone single-letter `"s"`
   word-token, split off by the corpus's own apostrophe-to-underscore slugifier, merges into the
   token immediately before it. Evidenced by real, committed slugs
   (`druid_resist_nature_s_lure`, `asavir_efreeti_s_blessing`/`_mount`) -- a structural fact
   about the corpus's own encoding, applied identically everywhere `words()` is called.
2. **Plural/singular equivalence** (`sheet_line_join.rs::words_eq`): two words match when equal,
   OR when one is the other plus a trailing `"s"` (`feat`/`feats`, `strike`/`strikes`).
   Independent of #1 (a possessive splits into a bare `"s"` token merged away by #1 before this
   function runs; a plural is a real word already ending in `s`).
3. **Exact-beats-normalised tiebreak** (`sheet_line_join.rs::longest_common_prefix`/
   `Candidate::exact`/`consider`): when #1/#2 create a genuine tie on (coverage, excess, tier)
   between two candidates, the one reached WITHOUT any normalisation wins. Needed because #2,
   run over the REAL population, manufactured two new ties between an already-correct EXACT
   match and a real, distinct GENERIC pool-container record that only ties because of the
   plural equivalence -- a real `some -> ambiguous` regression (below) the tiebreak closes.

**Population re-run** (`class_census --duplicates`, TRACKED package, no swap needed: the target
RULES for all 9 recoverable facets above already exist in the currently committed
`data/sheet_rules/` -- this fix needs no corpus regeneration at all, only the join's own word
comparison; verified `git status --short` empty before and after this whole step):

| Metric | Before this fix (`duplicates-after-fix.json`) | After this fix (`<scratch>/duplicates-tokfix2.json`) |
|---|---|---|
| Matched | 1111 | 1137 |
| Ambiguous | 4 | 4 |
| None | 899 | 873 |

Transition diff (`(class, explanation_id)` key, 1071 distinct facets both sides): **`none ->
matched`: 26.** Zero of every other transition kind (`matched -> none`, `matched -> ambiguous`,
`ambiguous -> none`, `matched -> different matched`, `ambiguous -> matched`) -- a clean,
strictly-additive fix, verified over the WHOLE population, not just the 11 named above. The 26
include the 6 recoverable of the 11 (druid, brawler, swashbuckler, 2x unchained_monk, asavir)
plus 20 MORE possessive-apostrophe facets elsewhere in the population this same mechanical rule
also reaches for the first time (`hunter's_bond`, `maker's_call`, `warrior's_path`, `djinni's_
blessing`(+mount), `janni's_blessing`(+mount), `marid's_blessing_mount`, `shaitan's_blessing`,
`shifter's_fury`, `weather's_fury`, `steed's_reach`, `swashbuckler's_grace`/`_edge`,
`warpriest.blessing_dc`/`_uses_per_day` -> `warpriest_blessings`) -- named here as evidence the
rule is genuinely mechanical (it recovers every possessive-apostrophe/plural stem the population
happens to carry, not just the 6 the blocker's own finding enumerated), not tuned to the 6.

**Which of the 11 now match, which stay `None`, and why** (re-run against the real package,
pinned by `real_package_tokenisation_variants_recover_six_of_the_eleven_dropped_matches` and
`real_package_genuine_content_divergence_among_the_eleven_stays_refused`,
`sheet_line_join.rs`):

| # | Facet | Result | Why |
|---|---|---|---|
| 1 | `druid.resist_natures_lure` | `Matched(druid_resist_nature_s_lure)` | recovered, rule #1 |
| 2 | `brawler.bonus_feat_count` | `Matched(brawler_bonus_feats)` | recovered, rule #2 |
| 3 | `swashbuckler.bonus_feat_count` | `Matched(swashbuckler_bonus_feats)` | recovered, rule #2 |
| 4 | `unchained_monk.bonus_feats_known` | `Matched(unchained_monk_bonus_feat)` | recovered, rule #2 |
| 5 | `unchained_monk.style_strikes_known` | `Matched(unchained_monk_style_strike)` | recovered, rule #2 |
| 6 | `asavir.efreeti_blessing_mount.fire_resistance` | `Matched(asavir_efreeti_s_blessing_mount)` | recovered, rule #1 -- retargets to the MORE SPECIFIC of two real sibling records sharing the merged stem (also explains the tail's own "mount" word); see the content nuance this raises below |
| 7 | `cavalier.bonus_combat_feat_count` | `None` | stays refused -- genuine divergence (#7 above), correctly not a tokenisation variant |
| 8 | `bloodrager.uncanny_dodge_flanking_level` | `None` | stays refused -- genuine divergence (#8 above) |
| 9 | `unchained_monk.flurry_attack_count` | `None` | stays refused -- genuine divergence (#9 above) |
| 10 | `skald.raging_climber` | `None` | stays refused -- one-word coincidence, unaffected by this fix (test `real_package_skald_raging_climber_and_swimmer_never_join_raging_song_on_one_word` still green) |
| 11 | `skald.raging_swimmer` | `None` | stays refused -- same as #10 |

**Content nuance on #6 (`asavir_efreeti_s_blessing_mount`):** this sibling's own prose is "Your
hoof attacks deal an additional 1d6 points of fire damage" -- it does NOT itself carry a
fire-resistance clause (that lives on the shorter `asavir_efreeti_s_blessing`, "gaining fire
resistance 5. It also deals..."), while the facet's own tail literally asks about
`fire_resistance`. The join's own scoring is correct and defensible (`_mount` explains MORE of
the tail -- the word "mount" too -- with zero leftover words either way, exactly the "fullest
full-consumption candidate wins" rule every other recovered facet above also relies on); this is
a converted-PACKAGE content question (which sibling record actually carries which clause of the
ability), not a join defect -- the same class of finding as §3's `weapon_and_armor_proficiency`
naming inconsistency. Named here for whoever owns the asavir facet-id/record-split shape;
outside this step's file ownership (data/sheet_rules is not touched by this step).

**Exact-beats-normalised tiebreak, evidenced:** the plural rule alone (#2), before the tiebreak
(#3) existed, turned 2 already-correct `Matched` rows into `Ambiguous` --
`swashbuckler.deed.evasive_grant` (`Matched(swashbuckler_evasive)` -> tied against
`swashbuckler_deeds`, the real umbrella "Swashbucklers spend panache points to accomplish deeds"
record, once "deed" matches its plural "deeds") and `warpriest.focus_weapon.
bonus_feat_granted` (`Matched(warpriest_focus_weapon)` -> tied against `warpriest_bonus_feats`,
the real umbrella bonus-feat-progression record, once "feat" matches "feats"). Root-caused,
fixed with the tiebreak, and pinned by
`real_package_an_exact_match_beats_a_normalised_tie_against_a_generic_pool_container` -- both
rows verified back to their pre-normalisation `Matched` answer, 0 regressions in the final
population re-run table above.

**TS mirror.** `apps/desktop/src/characterHub/classFeaturesModel.ts`'s `wordsOf`/`ruleForExplanation`
ported both normalisation rules and the `exact` tiebreak byte-for-byte from the Rust source
(`wordsOf`'s apostrophe merge, a new `wordsEq` helper, `lcpOf` returning `{n, exact}`,
`JoinCandidate.exact` as the same lowest-priority tiebreak key in `consider`). RED tests added
(`classFeaturesModel.test.ts`): `verifiesTokenisationVariantsDropTheNoticeSameAsTheRustJoin`
(the possessive-apostrophe and plural/singular recoveries, mirroring
`druid_resist_nature_s_lure`/`brawler_bonus_feats`) and
`verifiesAnExactMatchBeatsANormalisedTieAndStillDropsTheNotice` (the `swashbuckler_deeds`/
`swashbuckler_evasive` tiebreak case). `npm run typecheck` clean; `npm test` 125/125 test files
green.

### 7.6.2 The 8 "duplicate" lines, judged

Rendered fresh with `--sheet-dump <build> --with-sheet-rules`, post-repair package (temporary
`rsync` swap + `git checkout -- data/sheet_rules && git clean -fd data/sheet_rules` restore,
`git status --short` empty before and after) -- the tracker/duplicate shape does not manifest at
all in the TRACKED package alone (confirmed: `barbarian:1/7/14/20` against tracked, no swap,
hold ONLY `barbarian_uncanny_dodge`/`barbarian_improved_uncanny_dodge`, never the `_tracker`
sibling -- the tracker is held only once the post-repair package's `granted_by` edges exist,
an unrelated ingest-side fix landed on a different branch/step, swapped in here only to
reproduce the finding, never committed).

`barbarian:5` (as this step specified) reproduces the Barbarian pair (the tracker becomes held
at any barbarian level once the swap is active, not only the 70-build manifest's own
1/7/14/20 sample -- confirmed by also checking those four directly, same result). `paladin:11`
(as this step specified) reproduces NOTHING: Aura of Righteousness is a 17th-level paladin
feature and correctly prints no line at any of the paladin build's records at level 11 (`EXPL|
... correctly absent at level 11 by PF1 Core Rulebook level gate` -- the level gate is exactly
right; this is not a bug). The actual paladin duplicate is at `paladin:20` (17th level Aura of
Righteousness IS active there), which the original stage-3/stage-4 receipts already used and
this step re-confirms.

**Exact rendered lines, quoted verbatim** (`SheetLine.label` + `SheetLine.prose`, the same two
fields a player reads on the actual sheet -- `printed` is empty on every one of these four
records, all `SheetValue::Text`/`Words`):

| Record | Label | Prose (verbatim, empty means no body text at all) |
|---|---|---|
| `core_rulebook:class_feature:barbarian_uncanny_dodge` (held/printed at every barbarian level via the R2 join) | "Uncanny Dodge" | *(empty -- no `prose` field in the source record)* |
| `core_rulebook:class_feature:barbarian_uncanny_dodge_tracker` (held via the unrelated `granted_by` chassis edge, post-repair only) | "Barbarian ~ Uncanny Dodge Tracker" | *(empty -- no `prose` field in the source record)* |
| `core_rulebook:class_feature:rogue_uncanny_dodge` | "Uncanny Dodge" | *(empty)* |
| `core_rulebook:class_feature:rogue_uncanny_dodge_tracker` | "Rogue ~ Uncanny Dodge Tracker" | *(empty)* |
| `class_chassis.paladin.aura_of_righteousness` (the bespoke `pilot_compute` EXPLANATION, a Class Features section row, NOT a `render_sheet` LINE) | "Aura of Righteousness" (its `ClassFeatureRow` label, from `classFeaturesModel.ts`) | `Paladin Aura of Righteousness granted at paladin level 20 (PF1 Core Rulebook, 17th-level paladin class feature): "At 17th level, a paladin gains DR 5/evil and immunity to compulsion spells and spell-like abilities." This stays a bounded grant-only identity record (value 0, non-fabricated) because two of its three clauses remain ungrounded: compulsion immunity needs a spell-effect-type engine, and the ally +4 morale bonus against fear and compulsion applies to OTHER creatures within 10 feet, which this codebase models nowhere. Its DR clause IS grounded, separately, as class_chassis.paladin.damage_reduction.` |
| `core_rulebook:class_feature:paladin_aura_of_righteousness` (the converted RULE, a `render_sheet` LINE) | "Aura of Righteousness" | `You gain DR 5/Evil and immunity to compulsion spells and spell-like abilities. Each ally within 10 feet or you gains a +4 morale bonus on saving throws against fear compulsion . This ability functions only while you are conscious, not if you are unconscious or dead.` + `DR: 5/Evil` |

**Ruling, per the paper-sheet doctrine's own test ("if the two lines carry the same visible
text, a player sees a duplicate ... if the texts genuinely differ, keep both"):**

- **Barbarian/Rogue Uncanny Dodge Tracker (7 rows: barbarian 1/7/14/20, rogue 7/14/20): KEEP
  BOTH, texts genuinely differ.** The two records' LABELS are different strings ("Uncanny Dodge"
  vs "Barbarian ~ Uncanny Dodge Tracker" / "Rogue ~ Uncanny Dodge Tracker") and neither carries
  any body prose at all -- there is no shared visible SENTENCE for a player to read twice, only
  two differently-labelled, content-free rows. This matches the existing R3 ruling this receipt
  already carried ("two distinct oracle records, both print -- that is what the book says"),
  now re-confirmed against the EXACT rendered text rather than the stage-3 classifier's own
  fuzzy text-similarity proxy (which flags these at score 0.852/0.826 because it compares the
  tracker's label against the BESPOKE EXPL text for the base ability, not against the base
  RULE's own line -- a known, already-named proxy limitation, §4). **Separately named, NOT a
  duplication finding:** the `_tracker` records' own content is genuinely empty (no `prose`
  field at all in the source JSON) -- a real converted-package content gap (candidate fix:
  `print: false` at ingest, since these records exist only to feed the shared "Uncanny Dodge
  Flanking Level"/"Uncanny Dodge LVL" `_vars/*.json` contribution tables, not to be read
  directly). The fix belongs in the ingest converter (`crates/codex-ingest`) or
  `src/rules_core/sheet_rule.rs`'s `held_set`/`render_sheet` -- **outside this step's granted
  file ownership** (`sheet_line_join.rs`/`class_census.rs`/`classFeaturesModel.ts`/this receipt
  only), and touching either risks colliding with other in-flight work on this branch. Named for
  whoever owns the ingest converter or `sheet_rule.rs` next, not silently fixed and not silently
  dropped.
- **Paladin Aura of Righteousness (1 row, paladin:20): KEEP BOTH, texts genuinely differ.** The
  bespoke EXPL text is a paraphrase-with-caveats (what the ability does, WHY two of its three
  clauses are not computed) from a completely different render path
  (`pilot_compute`/`ExplanationDto`, shown in the Class Features section) than the converted
  RULE's own full official rules text (`SheetRule`/`render_sheet`, shown in Rules and features).
  These are not the same sentence by any reasonable reading -- the EXPL is honest about its own
  gaps, the LINE is the verbatim book text -- so this is the SAME shape R3 already ruled on for
  the trackers, independently re-confirmed here by direct quote rather than the classifier's
  0.84 fuzzy score.

**Net: `duplicate=8` at the 70-build gate is the correct, named, non-blocking outcome, not an
unresolved defect** -- every one of the 8 is judged above by exact quoted text, not left as an
unexamined "suspected" flag.

### 7.6.3 The 70-build gate, re-run

`render_before_sheets.sh` (rebuilt, TRACKED package, this step's binary) /
`render_after_sheets.sh`-equivalent (rebuilt, swap + restore, this step's binary) re-rendered
all 70 builds; `classify_sheet_diff.py` (updated, see below) re-run over the fresh pair:

```
gate_pass=False totals={'added_correct': 202, 'duplicate': 8, 'changed_value': 0, 'changed_value_accepted': 6, 'removed': 0, 'unclassified': 0}
removed_unexplained=0
```

(`<scratch>/blast-radius-classify-tokfix.json`, `<scratch>/blast-radius-receipt-tokfix.md`.)

- **`duplicate=8`: the named ruling above (§7.6.2), not a blocker.**
- **`changed_value=0`, `changed_value_accepted=6`: matches the orchestrator ruling's citation,
  with one honest correction.** `classify_sheet_diff.py` was updated (`RAGE_BONUS_ACCEPTED_
  TRANSITIONS`, a mechanical id-AND-value rule: only `core_rulebook:class_feature:standard_
  rage#bonus1/2/3`, and only when the value actually changed to exactly the documented
  Greater/Mighty Rage progression, is excluded from the gate-blocking `changed_value` count --
  never a bare build list) to carry the class the orchestrator ruling names, with its CRB
  citation, in a new `changed_value_accepted` bucket the gate formula excludes. Re-run against
  THIS render pair, only **6** of the ruling's named 9 rows actually differ: `barbarian:14` and
  `barbarian:20` each show all 3 sibling lines change (`+2/+4/+4` -> `+3/+6/+6` /
  `+4/+8/+8`, exactly as the ruling states). `mix_barbarian12_fighter1`'s own three lines are
  **already `+3/+6/+6` on BOTH sides of this render pair** (barbarian level 12 alone already
  crosses the Greater Rage threshold; verified directly, `grep standard_rage#bonus
  sheets/{before,after}/mix_barbarian12_fighter1.txt` are byte-identical) -- not a regression,
  not a missing row, simply already-settled going into this comparison, so `classify_build`
  correctly reports no diff for it at all. `changed_value=0` (the gate-blocking count) is
  unaffected either way.
- `removed=0`, `unclassified=0`: clean, matching the expected criteria exactly.

**`gate_pass=False` is accurate but not a blocker**, same reasoning as §4/§7's own prior
findings: `duplicate=8` is judged and named (§7.6.2), `changed_value_accepted=6` is a cited,
mechanical correctness-fix classification, not a defect, and both are the SAME already-understood
rows, not a new unexplained population.

## 8. Commands run, this fix pass

```
cargo test --lib -j 8 rules_core::sheet_line_join::     # 20/20 green (16 original + 4 new; 1 renamed/re-scoped)
cargo test --locked -j 8 --lib                          # 2663/2663 green, 7 ignored
cargo clippy --locked -j 8 --lib --bins -- -D warnings  # clean
python3 scripts/pcgen_residue_gate.py --check --closure   # verdict=PASS
cd apps/desktop && npm run typecheck                      # clean
cd apps/desktop && npm test                                # 125/125 test files green
class_census --duplicates <scratch>/duplicates-after-fix.json          # tracked package
class_census --duplicates <scratch>/duplicates-postrepair-fix.json     # post-repair package (swap+restore, identical figures)
python3 <scratch>/adv_join_fixed.py                                     # 0 mismatches vs Rust, over 1071 distinct facets
<scratch>/render_variant.sh <scratch>/class_census_pre_r2_baseline  <scratch>/sheets/pre_r2_baseline   # temp source swap, reverted before commit
<scratch>/render_variant.sh <scratch>/class_census_post_r2_fixed <scratch>/sheets/post_r2_fixed         # this step's real, committed code
classify_sheet_diff.py --before pre_r2_baseline --after post_r2_fixed --dump dump-after ...
```

## 9. Commands run, stage-4 blocker-2 fix pass (this step)

```
cargo test --locked -j 8 --lib rules_core::sheet_line_join::   # 23/23 green (20 prior + 3 new)
cargo test --locked -j 8 --lib                                  # 2666/2666 green, 7 ignored
cargo clippy --locked -j 8 --lib --bins -- -D warnings           # clean
python3 scripts/pcgen_residue_gate.py --check --closure            # verdict=PASS (checked after every swap+restore)
cd apps/desktop && npm run typecheck                               # clean
cd apps/desktop && npm test                                        # 125/125 test files green (incl. classFeaturesModel.test.ts)
class_census --duplicates <scratch>/duplicates-tokfix.json          # tracked package, before the exact-tiebreak fix
class_census --duplicates <scratch>/duplicates-tokfix2.json         # tracked package, after the exact-tiebreak fix (final)
render_before.sh    -> <scratch>/sheets/before   # rebuilt, tracked package, no swap, this step's binary
rsync -a <scratch>/dump-after/ data/sheet_rules/    # temporary swap
render_after_par.sh -> <scratch>/sheets/after    # rebuilt, post-repair package, this step's binary
git checkout -- data/sheet_rules && git clean -fd data/sheet_rules   # restore, confirmed empty
classify_sheet_diff.py --before sheets/before --after sheets/after --dump dump-after \
  --out-json <scratch>/blast-radius-classify-tokfix.json --out-receipt <scratch>/blast-radius-receipt-tokfix.md \
  --census-before <scratch>/census-before.json --census-after <scratch>/census-after2.json
```

Worktree clean (`git status --short`) before this step's commit; `data/sheet_rules` untouched by
this step's own commit (every swap restored via `git checkout -- data/sheet_rules && git clean
-fd data/sheet_rules`, confirmed empty each time -- checked before and after both the population
scan's own swap and the 70-build gate's re-render swap in this step).
