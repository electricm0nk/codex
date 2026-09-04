# Cycle — SD-34 wave 39, Lane A — Shape 2's word-choice-synonym remainder: a real alias-table fix, all 20 assigned units closed (16 to DONE, 4 to V)

- **Commit SHA:** `<filled below after commit>`
- **Files touched:** `src/bin/v06_work_inventory.rs` (new `CLASS_FEATURE_ID_KNOWN_SYNONYMS`
  alias table, new `class_feature_known_synonym_grounded` matcher function, wired into the
  existing `grounded`/`grounded_strict` boolean chain and both evidence-string ternaries, 5 new
  tests), `scripts/completion_atlas.py` (10 citation-pin line numbers re-derived — this cycle's
  own +149-line insertion shifted every one uniformly), `docs/work-inventory.json` (guarded
  regen), `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
  (regenerated snapshot — `derived_at` only), this receipt, `progress.md`, `kanban.md`,
  `docs/retro/events/sd34-wave39-lanea.jsonl` (new). **No `data/corpus/**` file touched.**
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --unified=0 HEAD --
  src/bin/v06_work_inventory.rs scripts/completion_atlas.py`, no
  `sd[0-9]+_`/`SD[0-9]+_`/`t_[0-9a-f]{8,}` hits — the literal `sd34-wave39-lanea` strings live
  only in the retro JSONL filename and this receipt's own prose, neither in the scoped diff).
- **Wired-integration audit result:** `19` hits in the full `merge-base(HEAD, origin/develop)`
  bundle-scoped diff (`git diff --unified=0 ea2b3396f2...HEAD -- src/bin/v06_work_inventory.rs
  scripts/completion_atlas.py`) — all 19 are pre-existing `placeholder` occurrences from waves
  landed before this cycle (corpus-data placeholder markers: `%N-placeholder`, redaction
  placeholder, vacuous-placeholder sub-cause), none of the doctrine-flagged tokens
  (`STUB`/`MOCK`/`not yet implemented`/`fixme`/`hack`), and **zero introduced by this cycle's own
  change** — re-confirmed against this cycle's own uncommitted working-tree diff alone
  (`git diff --unified=0 -- src/bin/v06_work_inventory.rs scripts/completion_atlas.py | grep
  -nE ...` → 0 hits, `OK_NO_TOKENS` for this cycle's own contribution specifically).
- **Acceptance criterion (verbatim from this cycle's dispatch brief):** "Close Shape 2's
  remaining 54-unit magnitude-bearing population, starting with the four classes that ALREADY
  have a per-feature compute function but a DIFFERENT word choice than the corpus feature's own
  slug ... Unchained Monk (7), Unchained Barbarian (6), Unchained Rogue (4), Unchained Summoner
  (3) -- 20 units total. For EACH of the four classes, confirm the exact synonym pairs by reading
  `pilot_compute/mod.rs`'s own explanation-id literals directly ... Fix by widening the
  classifier's own matcher ... NOT by renaming the engine's own explanation ids ... A per-feature
  alias/synonym table ... is the most likely correct shape ... Add safety guards the same way
  wave 38 lane C did."

## Worktree base note, and a real premise correction (not a self-heal — a live discrepancy)

This cycle's assigned worktree started at `ea2b3396f2` (PR #377's own merge commit, SD-33's
launch tip) — the same stale base wave 37/38's lanes hit and self-healed from. `git fetch origin
tranche/14` resolved `origin/tranche/14` to `7ea9651b87` ("wip(sd34): wave 33 lane D -- raise the
four stale test-count baselines"). `git merge-base --is-ancestor ea2b3396f2 7ea9651b87` → true;
rebased (`git rebase 7ea9651b87`), zero conflicts, before any commit landed.

**A real, disclosed discrepancy, not a self-heal:** this cycle's own dispatch brief and its
technical brief (`wave38_laneC_shape2_dot_segment_magnitude_id_matcher_cycle_receipt.md`) both
describe a state where Shape 2's magnitude-bearing population had already been reduced 154→54 by
wave 38 lane C's own dot-segment-matcher fix (commit `b80ccbffa4`). **That commit is NOT an
ancestor of `origin/tranche/14`'s real tip** — `git merge-base 7ea9651b87 b80ccbffa4` resolves to
`7ea9651b87` itself (the common ancestor, not a descendant relationship), and `b80ccbffa4` carries
no reachable branch/tag ref in this checkout at all. Waves 34–38's own bucket-D-mining chain was
built and committed in some other isolated worktree that was never pushed/merged to
`origin/tranche/14` — the same shared-checkout divergence class `AGENTS.md` §"Concurrency and
Measurement" warns about, now observed directly rather than inferred. **Retro-logged as a
`correction`** (`docs/retro/events/sd34-wave39-lanea.jsonl`, id
`1788492187534-sd34-wave39-lanea-e2438c`): the real Shape 2 magnitude-bearing population at this
cycle's actual rebased HEAD, before any edit, was **154**, not the brief's stated 54.

**Why this cycle proceeded anyway, rather than escalating:** the four named classes' 20-unit
subset (Unchained Monk 7 / Unchained Barbarian 6 / Unchained Rogue 4 / Unchained Summoner 3) was
independently re-derived fresh against this cycle's own real HEAD and found **identical** to the
brief's own count — because the word-choice-synonym gap this cycle closes is a genuinely
different, orthogonal matcher branch from the dot-segment-crossing gap wave 38 lane C's
(unmerged) fix addresses. Confirmed directly: every one of the 20 target explanation ids is
`class_feature.pu.<owner>.<descriptor>` — exactly **one** feature-bearing dot segment after the
owner, never `<owner>.<feature_slug>.<descriptor>`'s extra segment — so wave 38's fix, merged or
not, would never have touched these 20 units either way. The population re-derivation (below)
is against the real, committed HEAD this cycle actually built on, not the brief's stale figure.

## Fresh per-class re-derivation (all 20, not just the 2 previously spot-checked)

`python3 -c` filter over `docs/work-inventory.json`'s `units`: `evidence ==
"class_feature_no_dedicated_magnitude_id_matched_the_record_slug" and magnitude_token_count > 0`,
restricted to the four assigned classes, at this cycle's own pre-edit HEAD:

```
Counter({'Unchained Monk': 7, 'Unchained Barbarian': 6, 'Unchained Rogue': 4, 'Unchained Summoner': 3})
```

Matches the brief's own count exactly (20 = 7+6+4+3). Read every one of the four classes'
`ground_unchained_<class>_class_features` functions in `pilot_compute/mod.rs` in full (not just
the two examples the brief named) and confirmed each unit's real, already-shipped explanation id
by direct `grep -c` — every one of the 20 exact id strings below returns count `1` against
`pilot_compute/mod.rs`, confirmed **before** writing the table (not assumed from source-reading
alone):

| Owner | Corpus feature (`feature_slug`) | Real engine explanation id (verbatim) | Shape of the mismatch |
|---|---|---|---|
| unchained_monk | AC Bonus (`ac_bonus`) | `...unchained_monk.armor_class_bonus` | outright word substitution |
| unchained_monk | Bonus Feat (`bonus_feat`) | `...unchained_monk.bonus_feats_known` | pluralization + `_known` suffix |
| unchained_monk | Fast Movement (`fast_movement`) | `...unchained_monk.fast_movement_bonus_feet` | slug is a PREFIX of a 2-word-longer descriptor |
| unchained_monk | Ki Pool (`ki_pool`) | `...unchained_monk.ki_points` | outright word substitution |
| unchained_monk | Ki Powers (`ki_powers`) | `...unchained_monk.ki_powers_known` | `_known` suffix (not in the single-word suffix list) |
| unchained_monk | Stunning Fist (`stunning_fist`) | `...unchained_monk.stunning_fist_monk_level` | slug is a PREFIX of a 2-word-longer descriptor |
| unchained_monk | Style Strike (`style_strike`) | `...unchained_monk.style_strikes_known` | pluralization + `_known` suffix |
| unchained_barbarian | Fast Movement (`fast_movement`) | `...unchained_barbarian.fast_movement_bonus_feet` | same shape as Monk's own |
| unchained_barbarian | Greater Rage (`greater_rage`) | `...unchained_barbarian.greater_rage_morale_bonus` | slug is a PREFIX of a 2-word-longer descriptor |
| unchained_barbarian | Mighty Rage (`mighty_rage`) | `...unchained_barbarian.mighty_rage_morale_bonus` | slug is a PREFIX of a 2-word-longer descriptor |
| unchained_barbarian | Rage (`rage`) | `...unchained_barbarian.rage_rounds_per_day` | slug is a PREFIX; class emits 4 real siblings, any one suffices |
| unchained_barbarian | Rage Powers (`rage_powers`) | `...unchained_barbarian.rage_powers_known` | `_known` suffix |
| unchained_barbarian | Uncanny Dodge Tracker (`uncanny_dodge_tracker`) | `...unchained_barbarian.uncanny_dodge_tier` | outright word substitution ("tracker" vs "tier") — corpus-record-confirmed, see below |
| unchained_rogue | Finesse Training (`finesse_training`) | `...unchained_rogue.finesse_training_weapon_choices` | slug is a PREFIX of a 2-word-longer descriptor |
| unchained_rogue | Rogue Talents (`rogue_talents`) | `...unchained_rogue.rogue_talents_known` | `_known` suffix |
| unchained_rogue | Rogue's Edge (`rogues_edge`) | `...unchained_rogue.rogues_edge_skill_unlocks` | slug is a PREFIX of a 2-word-longer descriptor |
| unchained_rogue | Uncanny Dodge Tracker (`uncanny_dodge_tracker`) | `...unchained_rogue.uncanny_dodge_tracker_steps` | `_steps` suffix — engine's own detail string literally names "Uncanny Dodge Tracker" |
| unchained_summoner | Eidolon (`eidolon`) | `...unchained_summoner.eidolon_companion_level` | slug is a PREFIX; class emits 2 real siblings, any one suffices |
| unchained_summoner | Spells (`spells`) | `...unchained_summoner.unchained_summoner_marker` | outright word substitution — corpus-record-confirmed, see below |
| unchained_summoner | Summon Monster (`summon_monster`) | `...unchained_summoner.summon_monster_spell_level` | slug is a PREFIX; class emits 2 real siblings, any one suffices |

**Two entries needed the corpus record read directly, not just the compute function, to confirm
they are genuinely the SAME record under a different word (the brief's own explicit "not assumed
from reasoning alone" bar):**

- **Barbarian's `uncanny_dodge_tracker` → `uncanny_dodge_tier`.** The corpus record
  (`data/corpus/pathfinder_unchained/class_feature/barbarian_unchained_class/
  unchained_barbarian_uncanny_dodge_tracker.json`) carries `raw_bonus_chains` granting
  `UncannyDodgeLVL` (twice, at level 2 and level 5) and `UncannyDodgeFlankingLevel` — exactly the
  two quantities `uncanny_dodge_tier` (the summed 0/1/2 tier) and `uncanny_dodge_flanking_level`
  compute. "Tracker" and "tier" are the same tracked quantity under two different English words,
  confirmed by the corpus record's own token content, not guessed from name similarity.
- **Summoner's `spells` → `unchained_summoner_marker`.** The corpus record
  (`.../summoner_unchained_class/unchained_summoner_spells.json`) carries exactly ONE
  `raw_bonus_chain` token: `DEFINE:UnchainedSummoner|0` — byte-identical to what
  `unchained_summoner_marker`'s own compute function grounds ("The UnchainedSummoner flag is
  set"). This is not a same-concept word swap like the others; it is the identical formula token
  under a feature-level label ("Spells") the record's own DESC explains is really about the
  differing Unchained spell list, which the marker flag is what makes checkable.

## The fix (RED → GREEN, literal alias table, no generic matcher widening)

Added `CLASS_FEATURE_ID_KNOWN_SYNONYMS: &[(&str, &str, &str)]` (`src/bin/v06_work_inventory.rs`)
— 20 literal `(owner, feature_slug, exact_full_explanation_id)` triples, one per row of the table
above, every third field a full id string copied verbatim (confirmed `grep -c` == 1 against
`pilot_compute/mod.rs` for every one, per the population-verification section above). A new
function, `class_feature_known_synonym_grounded`, checks the SAME `group ==
class_name_as_group_text(owner)` guard every sibling matcher uses, looks up the table by exact
`(owner, feature_slug)`, and requires **full string equality** (`id == expected_id`) against
`explanation_ids` — never a substring, suffix, or prefix scan. Wired into the existing
`grounded`/`grounded_strict` boolean chains (both the broad and the roster-excluded-`strict`
variants, matching `suffix_stripped_grounded`'s own `!exact_suffix_grounded` ordering guard) and
into both evidence-string match arms, with an honest new evidence string,
`explanation_id_observed_via_known_class_feature_synonym`, distinguishing this path from the
pre-existing suffix-strip fallback's own evidence string.

**Why an alias table rather than widening `CLASS_FEATURE_ID_MAGNITUDE_SUFFIXES` (e.g. adding
`"known"`):** confirmed by direct analysis that adding `"known"` as a strippable suffix word
would ALSO close 3 of these 20 (Ki Powers, Rage Powers, Rogue Talents — each a case where the
stripped segment already equals `feature_slug` exactly) with no collision risk found corpus-wide
(`grep`'d every `_known`-suffixed id in `pilot_compute/mod.rs`, 12 total, checked each for a
same-owner cross-feature false-match risk — none found). **Not done**, deliberately: the brief's
own explicit steer names "a per-feature alias/synonym table" as "the most likely correct shape,"
and a single uniform mechanism covering all 20 (rather than 3 via a global suffix-list widening
plus 17 via a table) keeps this cycle's fix auditable as one shape, with a strictly narrower
blast radius (a global suffix-list change affects every class-feature classification in the
file; a literal table affects only the 20 named rows) — consistent with wave 38 lane C's own
"narrower over wider, once both are proven correct" precedent for the identical
`CLASS_FEATURE_ID_MAGNITUDE_SUFFIXES` list.

### Safety guards, live-caught BEFORE committing (same discipline as wave 38 lane C)

Ran the actual guarded regeneration as this cycle's own safety-guard dump test — not a temporary
debug test removed afterward (unlike wave 38 lane C's own `.unsupported`/`class_chassis`
collision, no candidate false positive was found needing a coded guard; the diff below IS the
proof): a full before/after `docs/work-inventory.json` unit-set comparison (script:
`python3 -c`, joined on `id`, comparing `(status, evidence)`) shows **exactly 20 units changed**,
**the unit set itself unchanged** (`set(before_ids) == set(after_ids)`), and **every changed unit
is one of the 20 named above** — zero collateral movement, zero false positives, zero units
outside the assigned population touched. The table's own defense-in-depth is structural, not just
tested-and-clean: (1) full-string equality, never substring/suffix, so a near-miss id (e.g.
`armor_class_bonus_level_component`, confirmed via a dedicated negative-control test) cannot
match; (2) every third-field id was confirmed by direct `grep -c == 1` to be a real
`ComputationExplanation`, never a `ComputationDiagnostic` — none end in `.unsupported`/
`.not_modelled`, and none are `.corpus_record.<slug>`-shaped generic roster ids (this cycle's own
population-verification section above lists all 20 verbatim); (3) the same `group == owner`
guard every sibling matcher applies, closing the identical archetype/variant-qualified
cross-credit class the operator's own worked example forbids.

### Tests (`class_feature_known_synonym_grounded_tests`, `v06_work_inventory.rs`)

- `every_known_synonym_table_entry_grounds_via_its_own_exact_id` — iterates all 20 live table
  entries, asserts each grounds against its own real id (RED before the table/function existed;
  GREEN after).
- `an_unlisted_feature_slug_for_a_known_owner_does_not_ground` — a known owner, an id the table
  DOES contain, but a `feature_slug` the table does not name for it: refuses. Proves the lookup
  is a literal table hit, never an inferred/generalized match.
- `a_known_pair_with_the_expected_id_absent_does_not_ground` — the table names what to look FOR;
  an empty `explanation_ids` set still refuses (the table cannot manufacture an id).
- `a_near_miss_id_does_not_ground_via_substring_or_prefix` — `armor_class_bonus_level_component`
  (a REAL, different, adjacent explanation id this cycle read directly) must not satisfy
  `ac_bonus`'s own table entry — proves full-string equality, not substring/prefix.
- `an_archetype_qualified_group_cannot_ground_via_the_synonym_table` — the operator's own worked
  example, restated for this check: `"Ironskin Monk"` (an archetype-qualified group) cannot
  ground off `unchained_monk`'s own table entry.

**5 of 5 new tests pass. 507 of 507 `v06_work_inventory` bin tests pass** (5 new, 0 regressed,
measured both before the regen and again at this cycle's own final HEAD).
`src/rules_core/pilot_compute/mod.rs` itself carries **zero diff** (`git diff --stat -- src/
rules_core/pilot_compute/mod.rs` empty) — every id this table recognizes was already shipped;
this cycle only made 20 of them visible to the classifier.

## Movement — the real, regen-verified delta

`python3 scripts/completion_atlas.py --check`: `population=49438 buckets=10 unclassified=0
overlap=0 citation_failures=0` (10 citation pins re-derived this cycle, see Files touched).

| Bucket | Before | After | Delta |
|---|---:|---:|---:|
| DONE (`grounded` + `text-complete` + ...) | — | — | **+16** (`engine-does-not-hold → grounded`, D → DONE) |
| V (`literal-verified`) | 289 | 293 | **+4** (`engine-does-not-hold → literal-verified`, D → V) |
| D | 2955 | 2935 | **−20** |

Sum check: `+16+4-20 = 0`. Corpus-wide `by_status`: `engine-does-not-hold 19346→19326 (−20)`,
`grounded 4309→4325 (+16)`, `literal-verified 289→293 (+4)` — every other status bucket
unchanged.

**All 20 units changed `(status, evidence)`, every one a genuine improvement, none a regression**
— byte-level per-id comparison of `docs/work-inventory.json` against the true committed
pre-regen baseline (`/tmp/cargo-sd34-wave39-laneA/work-inventory-before.json`, copied before the
regen ran). Exact set (all 20, id / before-status / after-status / after-evidence):

```
pathfinder_unchained:class_feature:unchained_barbarian_fast_movement       engine-does-not-hold -> grounded          explanation_id_observed_via_known_class_feature_synonym
pathfinder_unchained:class_feature:unchained_barbarian_greater_rage        engine-does-not-hold -> grounded          explanation_id_observed_via_known_class_feature_synonym
pathfinder_unchained:class_feature:unchained_barbarian_mighty_rage         engine-does-not-hold -> grounded          explanation_id_observed_via_known_class_feature_synonym
pathfinder_unchained:class_feature:unchained_barbarian_rage                engine-does-not-hold -> literal-verified  explanation_id_observed_via_known_class_feature_synonym
pathfinder_unchained:class_feature:unchained_barbarian_rage_powers         engine-does-not-hold -> grounded          explanation_id_observed_via_known_class_feature_synonym
pathfinder_unchained:class_feature:unchained_barbarian_uncanny_dodge_tracker engine-does-not-hold -> grounded        explanation_id_observed_via_known_class_feature_synonym
pathfinder_unchained:class_feature:unchained_monk_ac_bonus                 engine-does-not-hold -> literal-verified  explanation_id_observed_via_known_class_feature_synonym
pathfinder_unchained:class_feature:unchained_monk_bonus_feat               engine-does-not-hold -> grounded          explanation_id_observed_via_known_class_feature_synonym
pathfinder_unchained:class_feature:unchained_monk_fast_movement            engine-does-not-hold -> grounded          explanation_id_observed_via_known_class_feature_synonym
pathfinder_unchained:class_feature:unchained_monk_ki_pool                  engine-does-not-hold -> literal-verified  explanation_id_observed_via_known_class_feature_synonym
pathfinder_unchained:class_feature:unchained_monk_ki_powers                engine-does-not-hold -> grounded          explanation_id_observed_via_known_class_feature_synonym
pathfinder_unchained:class_feature:unchained_monk_stunning_fist            engine-does-not-hold -> grounded          explanation_id_observed_via_known_class_feature_synonym
pathfinder_unchained:class_feature:unchained_monk_style_strike             engine-does-not-hold -> grounded          explanation_id_observed_via_known_class_feature_synonym
pathfinder_unchained:class_feature:unchained_rogue_finesse_training        engine-does-not-hold -> grounded          explanation_id_observed_via_known_class_feature_synonym
pathfinder_unchained:class_feature:unchained_rogue_rogue_talents           engine-does-not-hold -> grounded          explanation_id_observed_via_known_class_feature_synonym
pathfinder_unchained:class_feature:unchained_rogue_rogues_edge             engine-does-not-hold -> grounded          explanation_id_observed_via_known_class_feature_synonym
pathfinder_unchained:class_feature:unchained_rogue_uncanny_dodge_tracker   engine-does-not-hold -> grounded          explanation_id_observed_via_known_class_feature_synonym
pathfinder_unchained:class_feature:unchained_summoner_eidolon              engine-does-not-hold -> grounded          explanation_id_observed_via_known_class_feature_synonym
pathfinder_unchained:class_feature:unchained_summoner_spells               engine-does-not-hold -> literal-verified  explanation_id_observed_via_known_class_feature_synonym
pathfinder_unchained:class_feature:unchained_summoner_summon_monster       engine-does-not-hold -> grounded          explanation_id_observed_via_known_class_feature_synonym
```

The 4 units that landed on `literal-verified` rather than `grounded` did so via the SAME
pre-existing, separate `apply_bucket_v_oracle_disposition_stamps`-style pass this cycle did not
touch — a byproduct of the classifier now recognizing the id at all, not something this table's
own logic directly produces.

## Shape 2's remaining population, honestly re-stated against this cycle's real HEAD

After this cycle: `159` total Shape-2-evidence units (`179 − 20`), `134` magnitude-bearing
(`154 − 20`) — **not 34**, because wave 38 lane C's own dot-segment fix (which would have
independently reduced the pre-this-cycle baseline from 179/154 to 74/54) was never actually
merged into `origin/tranche/14` (see the premise-correction section above). The 34-unit figure
the dispatch brief implied for "everything left after this cycle" is provisional on that other
fix landing; until it does, the true remainder for the next lane is the FULL set the wave 38
receipt named as unattempted PLUS the dot-segment-crossing population that fix would have
closed. Named precisely, at this cycle's real HEAD:

- **The 9 classes the dispatch brief named as lane B's own scope** (word-choice-synonym check
  needed, not yet traced this cycle): Monk (5), Duelist (4), Shadowdancer (4), Assassin (2),
  Fighter (2), Loremaster (2), Wizard (2), Bard/Cleric/Druid/Paladin/Ranger/Sorcerer/Psychic (1
  each, 7) — same population wave 38 lane C's own receipt named, unchanged, **not touched this
  cycle**.
- **The dot-segment-crossing population wave 38 lane C's own (unmerged) fix would close**: per
  that receipt, ~100 magnitude-bearing units across roughly 20 classes using the
  `<owner>.<feature_slug>.<descriptor>` untabled convention (Antipaladin, Cryptic, Dread,
  Marksman, Psychic Warrior, Soulknife, Aegis, Tactician, Vitalist, Wilder, Kineticist, Medium,
  Mesmerist, Occultist, Psychic, Spiritualist, Magus, Shifter, Vigilante, Psion) — **this
  population is currently UNCLOSED on `origin/tranche/14`'s real tip**, since the commit that
  closes it was never merged. Re-landing that fix (cherry-picking or re-implementing
  `class_feature_exact_suffix_grounded`'s second-to-last-dot-segment widening, with its own two
  safety guards) is real, undone work on this branch, not a completed prior wave.
- Sum check: `20 (this cycle) + 27 (lane B's 9-class remainder) + ~100 (unmerged dot-segment
  population, approximate — re-derive fresh before dispatching) + 7 (Summoner/Fighter/Wizard/
  Psychic units the dispatch brief's 20-unit scope excluded, spot-checked present in the 134
  figure above) ≈ 154`, consistent with the 134 magnitude-bearing total this cycle's own
  re-derivation found (`27+7=34` untouched-by-name plus the unmerged dot-segment population is
  the arithmetic gap; a future lane should re-run the exact filter above rather than trust this
  approximate reconciliation).

## Figures (every number, its command, its denominator)

- `population=49438 buckets=10 unclassified=0 overlap=0 citation_failures=0` —
  `python3 scripts/completion_atlas.py --check`, this cycle's own final HEAD.
- `D: 2955→2935 (−20)`, `V: 289→293 (+4)`, `grounded: 4309→4325 (+16)` — same command,
  before/after this cycle's own guarded regen.
- `179→159` Shape 2 total (`20` closed), `154→134` magnitude-bearing (`20` closed) — Python
  filter over `docs/work-inventory.json`'s units, `evidence ==
  "class_feature_no_dedicated_magnitude_id_matched_the_record_slug"`, before/after.
- `20` of 20 assigned units closed, `0` collateral movements — full unit-set join, before vs.
  after, `id` as key (see Movement section).
- `12` real `_known`-suffixed ids corpus-wide, `0` cross-feature collision risk found — `grep -oE
  '"class_feature\.[a-z0-9_.]*_known"' src/rules_core/pilot_compute/mod.rs | sort -u`, this
  cycle's own HEAD.
- `5` of 5 `class_feature_known_synonym_grounded_tests` pass (5 new), `507` of 507
  `v06_work_inventory` bin tests pass (5 new) — `cargo test --locked --bin v06_work_inventory`,
  this cycle's own final HEAD, run twice (mid-cycle and after the regen).
- `48706 of 51476` corpus records examined, CLEAN, unchanged before/after —
  `corpus_literal_sweep --json-out`, of the full corpus (no `data/corpus/**` record touched this
  cycle — Rust classifier logic only).
- `1839` units cleared over `2580` fixture rows, `0` failed — `derived_evaluator_fixture_check
  --json-out`, of the fixture's own 2,580-row coverage, unchanged before/after.
- `cargo test --locked --no-run` (full workspace) exits `0` — this cycle's own final HEAD.

## Row-count command output

```
$ grep -n "^| 37 |" docs/release/SD-34-book-completion/kanban.md | tail -1
| 37 | `mine-bucket-d` | 3 | wave 32, lane C (no AT-34-E# card yet) | partial | ...
```

Row 37 (`mine-bucket-d`) is the same accumulating row every prior bucket-D mining cycle appends
into — this cycle appends its own sentence. Wave 33's own bucket-D commit (`2eb34a6e91`, "wave 33
-- bucket D's three smallest mechanisms") landed real content on this branch's real history but
left row 37 itself unchanged (only the wave-32 sentence was present before this cycle's own
append) — not this cycle's own gap to fix, noted only for the next lane's own row-count
derivation.

## Build scope verified

- `cargo test --locked --bin v06_work_inventory` → 507/507 pass (this cycle's own final HEAD,
  run twice: once mid-cycle before the regen, once after).
- `cargo test --locked --no-run` (full workspace) → exit 0, run at this cycle's own final HEAD.
- `python3 scripts/tests/test_completion_atlas.py` → 38/38 pass (this cycle touched
  `scripts/completion_atlas.py`'s citation pins).
- Desktop crate (`apps/desktop/src-tauri`) — not run this cycle: `git diff --stat HEAD --
  apps/desktop/` is empty, no file under `apps/desktop/` touched, honestly reported skipped
  (`workflow-instruction.md §6` step 3 scopes this to "if touched").

## Sweep population

`corpus_literal_sweep`: `48706 examined of 51476 read, 0 findings, CLEAN` — no `data/corpus/**`
record added, changed, or removed this cycle (Rust classifier logic only), delta 0, consistent
with 0 records added.

## Oracle pin

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
(`scripts/pcgen-oracle-pin.env`) — no figure in this receipt was derived from the pinned oracle
corpus; every magnitude credited this cycle was already transcribed and unit-tested against
`data/corpus/**` by the pre-existing `ground_unchained_<class>_class_features` functions this
cycle only made VISIBLE to the classifier, not computed anew. Cited for completeness per the
receipt schema.

## Status

**complete** — all 20 assigned units closed (16 to DONE, 4 to a stronger V-bucket rung), zero
collateral movement, zero regressions, full RED→GREEN proof, and a real, disclosed premise
correction (wave 38 lane C's own dot-segment fix was never actually merged to
`origin/tranche/14`, so the "54 remaining" framing this cycle's brief carried forward was
already stale before this cycle began) rather than silently absorbed or hidden.

## Movement, four buckets

- **Closure:** 16 (D → DONE).
- **Reclassification:** 4 (D → V, a stronger bucket than DONE under this bundle's own taxonomy).
- **Reachability:** 0 units newly reached or lost reachability.
- **Instrument-correction:** 10 `completion_atlas.py` citation pins re-derived (this cycle's own
  +149-line insertion) + 1 retro-logged premise correction (the stale "54 remaining" figure).

## Notes (judgment calls)

- **Why the alias table's third field is a full id string rather than an `(owner, feature_slug,
  descriptor_word)` triple relying on the existing `.{owner}.` / trailing-dot-segment scan:**
  several of the 20 entries (`Uncanny Dodge Tracker`, `Spells`) needed corpus-record confirmation,
  not just a descriptor-word guess, and a full literal string is strictly harder to get wrong
  than reassembling it from parts at match time — the entire point of this table is "recognize an
  id that already exists," so storing it pre-assembled removes an entire class of "I reconstructed
  the id wrong" bug.
- **Why `Rage`/`Eidolon`/`Summon Monster` each alias to only ONE of several real sibling
  magnitudes their class computes:** the classifier's own contract only asks "does the engine
  hold ANY record for this feature," not "does this alias enumerate every magnitude" — any one
  suffices, and enumerating every sibling would be scope creep past what closing these 20 units
  requires.
- **The premise correction is the most consequential finding this cycle produced**, not the fix
  itself: the next lane picking up Shape 2's remainder must NOT trust the "34 units left" framing
  a naive `154 total − 20 this cycle − 100 wave-38-already-closed` arithmetic would suggest, since
  the 100-unit wave-38 closure never actually landed. `git merge-base <origin/tranche/14 tip>
  b80ccbffa4` is the one-line check that would catch this before dispatching against a stale
  premise again.

## Next-cycle plan

1. **Confirm whether wave 38 lane C's own dot-segment fix should be RE-LANDED** (re-implemented
   or cherry-picked, if the dangling commit `b80ccbffa4` is still locally reachable in whatever
   worktree produced it) before dispatching the ~100-unit `<owner>.<feature_slug>.<descriptor>`
   population it closes — an operator/orchestrator decision, not this lane's to make unilaterally,
   since re-implementing risks diverging from an already-tested version sitting unmerged
   somewhere.
2. **Lane B's own 9-class remainder** (Monk 5, Duelist 4, Shadowdancer 4, Assassin 2, Fighter 2,
   Loremaster 2, Wizard 2, Bard/Cleric/Druid/Paladin/Ranger/Sorcerer/Psychic 1 each = 27 units) —
   unchanged, not touched this cycle, needs the same "does a per-feature compute function even
   exist" first check the dispatch brief named.
3. **Re-run the exact Shape 2 population filter** (`evidence ==
   "class_feature_no_dedicated_magnitude_id_matched_the_record_slug" and
   magnitude_token_count > 0`) fresh at whatever HEAD the next lane actually starts from, rather
   than trusting this receipt's own 134 figure if any intervening wave lands more bucket-D
   mining work — the premise-correction lesson this cycle just paid for.
