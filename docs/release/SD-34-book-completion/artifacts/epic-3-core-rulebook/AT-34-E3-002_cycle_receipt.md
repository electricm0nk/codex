# Cycle 8 — Epic 3 (Core Rulebook to zero) / AT-34-E3-002 (bucket C, "held and computed, never surfaced")

- **Commit SHA:** `2d9ae34f89` (this cycle's own `pilot_compute::mod.rs` +
  `v06_work_inventory.rs` fix, probes, classify() rung, and 4 new tests —
  RED confirmed then GREEN — committed and pushed as this cycle's own
  checkpoint before the live regen ran) plus this cycle's own follow-up
  commit (`scripts/completion_atlas.py`'s 10 citation line re-pins, this
  cycle's own ~176-line insertion having shifted every one, plus this
  receipt/progress/kanban/retro).
- **Files touched:** `src/rules_core/pilot_compute/mod.rs` (one new function
  `explain_other_classes_favored_class_bonus_choice`, one new call site in
  `compute_pilot_base_chassis`), `src/bin/v06_work_inventory.rs` (one new
  `EngineFacts` field `favored_class_bonus_choice_wired: BTreeSet<String>`,
  one new probe function `probe_favored_class_bonus_choice_wiring`, one new
  `classify()` rung, 4 new tests — 2 positive proof + 2 negative controls,
  RED confirmed then GREEN), `scripts/completion_atlas.py` (10 citation line
  pins re-derived after this cycle's own insertion shifted every one below
  it — a real line-based diff against the pre-cycle file content, `git
  show`-verified exact-content match on all 10, never guessed), this
  receipt, `docs/release/SD-34-book-completion/progress.md`,
  `docs/release/SD-34-book-completion/kanban.md`. **`docs/work-inventory.json` and
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` are
  deliberately NOT committed this cycle** — this dispatch's file-ownership rule assigns their
  regeneration to the wave's single shared regeneration cycle. Every figure below comes from a
  real, local, uncommitted, FULL three-stage regen (`corpus_literal_sweep` →
  `derived_evaluator_fixture_check` → `v06_work_inventory`, `--allow-stamp-loss` never passed) of
  this cycle's own committed source, restored (`git restore`) before this commit.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Run against
  `git diff --unified=0 6f67df49c7...2d9ae34f89 -- src/rules_core/ src/bin/
  scripts/oracle_harness/ docs/work-inventory.json artifacts/epic-3-core-rulebook/
  ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  — `6f67df49c7` (wave 21) is `tranche/14`'s tip at this cycle's own start — zero matches.
- **Wired-integration audit result:** OK_NO_TOKENS. Same diff,
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` — zero matches.
- **Acceptance criterion (verbatim, `epic-breakdown.md` §AT-34-E3-002):** "**370** units the
  engine holds and computes but never surfaces. **Evidence:** per unit, the explanation or
  display path that now carries it. A unit the player still cannot see is not cleared, whatever
  the engine holds." (370 is stale, already retired by waves 15–21; re-derived fresh at this
  cycle's start, `core_rulebook` bucket C was **199**, matching this dispatch's own brief and
  the current committed inventory exactly.)
- **Status:** partial

## Population, re-derived (not quoted)

At this cycle's start, the **committed** `docs/work-inventory.json` (wave-21, `6f67df49c7`)
reads `core_rulebook` bucket C = **199**
(`python3 scripts/completion_atlas.py --book core_rulebook --check`) — matching the dispatch
brief exactly, confirmed live rather than trusted on citation.

**This cycle re-derived cycle 7's own 13-row remainder table fresh, from scratch, by exhaustive
per-key categorization against the live corpus** (`decisions.md §12` L2) — every one of the 199
live bucket-C `core_rulebook` corpus keys assigned to exactly one named sub-cause, zero left
uncategorized, sum verified to equal 199 exactly by script, not by hand:

```
$ python3 <categorization script over docs/work-inventory.json>
48 monk_unarmed_damage_no_formula_in_engine
26 prestige_class_standalone_feature_not_computed
25 bloodline_power_or_bloodline_feat_not_computed
25 base_class_standalone_feature_not_computed
16 class_chassis_internal_tracker
13 rage_power_not_computed
10 prestige_class_chassis_internal_tracker
10 rogue_talent_not_computed
7  druid_nature_bond_domain_selection_not_computed
6  favored_class_bonus_choice__CLOSED_THIS_CYCLE
5  favored_class_bonus_choice_no_seam__npc_classes
5  ranger_favored_x_chassis_or_wild_empathy
2  domain_power_display_record_not_wired
1  versatile_performance_not_computed
sum: 199
uncategorized keys: []
```

**This found and corrected TWO real errors in cycle 7's own remainder table**, logged as a
`correction` retro event (`docs/retro/events/sd34-at-34-e3-002.jsonl`,
`--verified-by` the categorization script itself, run against the live committed inventory):

1. Cycle 7's own 13-row table does not sum to its own stated 199 — its 13 listed values
   (48+35+26+25+16+13+11+10+10+5+7+2+0) sum to **208**, not 199. A real arithmetic error in
   that receipt, not merely stale prose.
2. `base_class_standalone_feature_not_computed` was stated **35**; direct exhaustive corpus
   categorization (every base-class-owned bucket-C record not already claimed by another named
   sub-cause) gives **25** — Barbarian Rage family (4), Cleric Channel Energy family (3),
   Paladin (3), Druid Wild Shape family (3), Fighter (1), Monk (7), Rogue (2), Wizard (2) = 25.
3. `versatile_performance_not_computed` was stated **0** ("closed cycle 5, unchanged"). The
   live corpus still carries `"Bard ~ Versatile Performance"` in bucket C right now —
   cycle 5 closed the 9 individual Perform-type sub-records (a different, `~`-suffixed corpus
   shape cycle 5's own receipt explicitly excluded this bare header from), but this bare
   chooser/header record itself was never separately wired — the same "bare header left behind"
   shape `domain_power_display_record_not_wired`'s own `Nobility Domain` row already
   establishes. Corrected to **1**.
4. `prestige_class_standalone_feature_not_computed` (26) and
   `bloodline_power_or_bloodline_feat_not_computed` (25) both confirmed exactly as cycle 7
   stated, once `"Dragon Disciple ~ Bloodline Feat"` is correctly attributed to the PRESTIGE
   group (its own corpus record's `"class"` field reads `"Dragon Disciple"`, a prestige class,
   confirmed by direct read of `data/corpus/core_rulebook/class_feature/dragon_disciple/
   bloodline_feat.json` — not the Sorcerer bloodline mechanic the name's own "Bloodline" word
   might suggest).

All other sub-causes (`class_chassis_internal_tracker` 16, `prestige_class_chassis_internal_
tracker` 10, `monk_unarmed_damage_no_formula_in_engine` 48, `rage_power_not_computed` 13,
`rogue_talent_not_computed` 10, `ranger_favored_x_chassis_or_wild_empathy` 5,
`druid_nature_bond_domain_selection_not_computed` 7, `domain_power_display_record_not_wired` 2)
are confirmed to hold their exact stated populations.

## Mechanism: the SAME Favored Class Bonus choice rule, generalized from Fighter to five siblings

`favored_class_bonus_choice_not_wired` (cycle 7's population, 11) decomposes exactly into two
shapes:

1. **Six PC classes whose own bounded level-1 chassis recognition seam already exists**
   (`supported_barbarian_level`, `supported_fighter_level`, `supported_monk_level`,
   `supported_paladin_level`, `supported_rogue_level`, `supported_wizard_level`). Fighter's own
   `class_chassis.fighter.favored_class_bonus_choice` explanation shipped SD13-E5
   (`explain_fighter_favored_class_bonus_choice`), but `classify()` had never been taught to
   consult it — the SAME shape every prior cycle in this file has closed (a real, already-shipped
   compute path never wired to the classifier). The PF1 Core Rulebook Favored Class rule (pg. 31:
   "whenever a character gains a level in his favored class, he receives either +1 hit point or
   +1 skill rank") is genuinely class-agnostic, and a Human's favored class is Any — the identical
   reasoning Fighter's own doc comment already established, applying unchanged to Barbarian,
   Monk, Paladin, Rogue, and Wizard. One new function,
   `explain_other_classes_favored_class_bonus_choice`, generalizes the rule to those five rather
   than duplicating Fighter's own function five times — reusing the SAME choice id
   (`FAVORED_CLASS_BONUS_CHOICE_ID`) and the SAME flat, rule-verified +1 magnitude, differing
   only in the class's own bounded-seam gate and the explanation id's class-key segment.
   Fighter's own already-shipped function is untouched.
2. **Five NPC classes with no bounded seam at all** (Adept, Aristocrat, Commoner, Expert,
   Warrior) — confirmed by direct search (`grep -n "^fn supported_adept_level\|
   supported_aristocrat_level\|supported_commoner_level\|supported_expert_level\|
   supported_warrior_level"` returns zero matches). No explanation surface exists for these to
   wire to; a genuine engine gap, named in the remainder, not force-closed.

New `probe_favored_class_bonus_choice_wiring` (mirrors `probe_ranger_combat_style_wiring`'s own
shape exactly) exercises the real `compute_pilot_base_chassis` pipeline for all six classes at
level 1, injecting the `"choice:favored_class_bonus"` selection directly, and checks for the
class-specific explanation id firing with the expected `-> bonus:hp` detail. New `classify()`
rung: `group` for a bare `"<Class>"` key equals the WHOLE key (no `" ~ "` separator), so
`class_feature_owner` and its fallbacks can never resolve an owner via the generic
suffix-matching path — the same shape Ranger Combat Style / Monk Unarmed Damage / the class
chassis trackers above already establish.

**Cross-book collision check performed before shipping**: a corpus-wide scan of the full
49,438-unit committed inventory (`kind == "class_feature"`, `corpus_key` in the six class
names) confirmed each bare key is declared exactly once, only in `core_rulebook` — no other
book anywhere in the corpus declares a bare `class_feature` record with any of these six names.
This rung is deliberately left unguarded, matching the Favored Enemy/Terrain/Ranger Combat
Style precedent above.

## RED → GREEN

RED (confirmed for the intended reason): temporarily changed the new rung's own membership
check from `facts.favored_class_bonus_choice_wired.contains(&unit.key)` to
`facts.favored_class_bonus_choice_wired.contains("RED-CHECK-NEVER-MATCHES")` and re-ran the two
positive proof tests — both failed with `left: "engine-does-not-hold", right: "grounded"` (the
pre-existing fallthrough this cycle closes), confirming the tests fail because the fix is
absent, not for an unrelated reason. Restored the match; all four tests pass.

```
$ cargo test --locked --bin v06_work_inventory favored_class_bonus
running 7 tests
test class_feature_text_complete_rung_tests::a_fighter_favored_class_bonus_record_reaches_grounded_off_the_probes_wiring ... FAILED
test class_feature_text_complete_rung_tests::each_generalized_sibling_favored_class_bonus_record_reaches_grounded_off_the_probes_wiring ... FAILED
  left: "engine-does-not-hold"
 right: "grounded"
```

After restoring the match:

```
$ cargo test --locked --bin v06_work_inventory favored_class_bonus
running 7 tests
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 486 filtered out; finished in 0.00s
```

Full `class_feature`-scoped suite: `cargo test --locked --bin v06_work_inventory class_feature`
— **154 passed, 0 failed** (150 pre-existing + this cycle's own 4). Full bin suite: **493
passed, 0 failed** (489 + 4).

## Live regen (local, uncommitted — see file-ownership note above)

**Full three-stage pipeline run, in order, `--allow-stamp-loss` never passed:**

```
$ corpus_literal_sweep --json-out /tmp/sweep-report.json
corpus-literal-sweep: 48708 records examined of 51482 read, 413336 tokens compared (9 synthesized),
51469 digests checked, 0 findings
corpus-literal-sweep: CLEAN

$ derived_evaluator_fixture_check --json-out /tmp/fixture-report.json
derived-evaluator-fixture-check: 1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0 not ingested

$ CORPUS_LITERAL_SWEEP_REPORT=/tmp/sweep-report.json DERIVED_FIXTURE_CHECK_REPORT=/tmp/fixture-report.json \
  v06_work_inventory
(writes docs/work-inventory.json; exit 0)
```

Both reports match wave-21's own baseline exactly — unchanged, since this cycle touches no
`data/corpus/**` file (48,708 examined both before and after; 1,839/2,580 fixture rows cleared
both before and after).

**Isolation confirmed by a whole-inventory before/after diff keyed on unit id** (not sampled —
a real Python diff over both full 49,438-unit JSON documents, before = the COMMITTED HEAD
inventory (`git show 6f67df49c7:docs/work-inventory.json`), after = this cycle's own local regen
against HEAD's committed source plus this cycle's own edit):

```
before count: 49438 after count: 49438
added: 0 removed: 0
changed: 21
changed by book: {'core_rulebook': 21}
changed by new evidence: {
  'favored_class_bonus_choice_probe_observed_a_real_computed_recognition_for_the_bookkeeping_record': 6,
  'engine_diagnostic:vacuous_placeholder_row_no_corpus_content_to_render': 3,
  'engine_diagnostic:grant_token_only_dispatch_row_routes_to_already_shipped_content': 12,
}
  core_rulebook:class_feature:paladin engine-does-not-hold -> grounded (computed)
  core_rulebook:class_feature:barbarian engine-does-not-hold -> grounded (computed)
  core_rulebook:class_feature:monk engine-does-not-hold -> grounded (computed)
  core_rulebook:class_feature:fighter engine-does-not-hold -> grounded (computed)
  core_rulebook:class_feature:wizard engine-does-not-hold -> grounded (computed)
  core_rulebook:class_feature:rogue engine-does-not-hold -> grounded (computed)
```

**Only 6 of the 21 changed units are this cycle's own** — the other 15 (`companion`/
`class_feature` `deferred-with-reason` → `deferred-with-reason`, evidence-string-only, no bucket
move) are `38e10d066b` (a wave-20 fix landed on `tranche/14` between waves, before this cycle's
own start, whose own commit message states plainly the committed inventory would stay red "until
the next wave's regeneration bakes these in" — this cycle's own regen is simply the first to run
since that fix landed, and correctly folds it in without this cycle claiming credit for it).
Confirmed by evidence-string filter: this cycle's own 6 all and only carry
`favored_class_bonus_choice_probe_observed_a_real_computed_recognition_for_the_bookkeeping_record`;
zero changes from any other concurrently-committed lane between this cycle's checkpoint push and
this regen (`origin/tranche/14` had not moved past `2d9ae34f89` when this regen ran). Every one
of the 6 moved `engine-does-not-hold` (bucket C) → `grounded` (bucket **DONE**) directly,
confirmed by direct post-regen read: all six carry `status: "grounded"`, `wiring_class:
"computed"` — none eligible for the static/derived `V`-reclassification `apply_done_rung_stamps`
applies, so none was restamped away from DONE.

**Bucket-level before/after, independently re-derived** (not read from the id-diff above —
`completion_atlas.py`'s own bucket partition, before = `git show 6f67df49c7:docs/work-inventory.json`, after = this cycle's own live regen):

```
corpus-wide BEFORE: DONE=24731 A=449 B=11769 C=4180 D=2955 M=4674 V=289 U=202 X=170 Z=19
corpus-wide AFTER:  DONE=24737 A=449 B=11769 C=4174 D=2955 M=4674 V=289 U=202 X=170 Z=19
core_rulebook BEFORE: DONE=4616 B=470 C=199 D=366 M=811 V=114 U=10 X=115
core_rulebook AFTER:  DONE=4622 B=470 C=193 D=366 M=811 V=114 U=10 X=115
```

Every bucket except DONE and C is byte-identical before/after, at both scopes — the 15
evidence-string-only `X`→`X` changes move no bucket count, confirming they carry no closure of
their own; the ONLY buckets that moved are DONE (+6) and C (−6), both scopes, matching this
cycle's own 6 closures exactly.

## Figures + their re-derive commands

| Figure | Value | Command | Denominator |
|---|---:|---|---|
| `core_rulebook` bucket C at cycle start | 199 | `python3 scripts/completion_atlas.py --book core_rulebook --check` against the committed `docs/work-inventory.json` | of 6,701 |
| `core_rulebook` bucket C after this cycle's own fix | **193** | same command, live regen including this cycle's edit | of 6,701 (delta −6) |
| `core_rulebook` bucket DONE after this cycle | **4,622** | same command | of 6,701 (delta +6) |
| `base_class_standalone_feature_not_computed` sub-cause population, corrected | 35 → **25** | direct `docs/work-inventory.json` exhaustive categorization (see Population section) | of 199/193 (correction, not a bucket move) |
| `versatile_performance_not_computed` sub-cause population, corrected | 0 → **1** | same | of 199/193 (correction, not a bucket move) |
| This cycle's own isolated closures | **6**, all `core_rulebook`, all `favored_class_bonus_choice` | whole-inventory diff filtered on this cycle's own evidence string | of 6 (targeted population) |
| Corpus-wide bucket C before/after this cycle's own regen | 4,180 / **4,174** | `python3 scripts/completion_atlas.py --check` | of 49,438 (delta −6) |
| Corpus-wide bucket DONE before/after | 24,731 / **24,737** | same command | of 49,438 (delta +6) |
| `corpus_literal_sweep` (before/after, unchanged) | 48,708 examined, 0 findings | `corpus_literal_sweep --json-out` | of 51,482 read |
| `derived_evaluator_fixture_check` (before/after, unchanged) | 1,839 cleared of 2,580 rows, 0 failed | `derived_evaluator_fixture_check --json-out` | of 2,580 |
| `completion_atlas.py --check` (corpus-wide, post-regen) | `population=49438 unclassified=0 overlap=0 citation_failures=0`, exit 0 | `python3 scripts/completion_atlas.py --check` | of 49,438 |
| `completion_atlas.py --check` `citation_failures` | 0 (10→0, re-derived this cycle after this cycle's own ~176-line insertion shifted 10 pins, mapped via a real line-based diff, `git show`-verified content match on all 10, not guessed) | `python3 scripts/completion_atlas.py --check` | of 10 citations |
| `cargo test --locked --bin v06_work_inventory` (full) | `493 passed; 0 failed` | `cargo test --locked --bin v06_work_inventory` | of 493 |
| `cargo test --locked --bin v06_work_inventory class_feature` | `154 passed; 0 failed` | `cargo test --locked --bin v06_work_inventory class_feature` | of 154 |
| `cargo test --locked --no-run` (workspace) | exit 0 | `cargo test --locked --no-run` | — |
| `cargo test --locked --lib` (workspace library) | `2989 passed; 5 failed` — all 5 pre-existing, unrelated to this cycle's diff (see Notes) | `cargo test --locked --lib` | of 2994 |

## Row-count command output (this cycle's own live artifact, uncommitted per file-ownership rule)

```
$ python3 scripts/completion_atlas.py --book core_rulebook --check
book=core_rulebook population=6701 unclassified=0 overlap=0
  DONE: 4622
  A: 0
  B: 470
  C: 193
  D: 366
  M: 811
  V: 114
  U: 10
  X: 115
  Z: 0
```

Bucket C: **193**, not zero. **Status: partial**, remainder named below (populations sum
exactly to 193). This live command output was produced by the local, uncommitted regen and is
NOT reflected in the currently-committed `docs/work-inventory.json` (restored via `git restore`
before this commit, per the file-ownership rule) — the committed inventory still reads C=199
until the wave's shared regeneration cycle re-runs the pipeline against this cycle's own
committed source.

## Build scope verified

`cargo test --locked --no-run` (workspace) exits **0**, run at commit `2d9ae34f89` — this
cycle's own last commit that can move a figure a test assertion depends on
(`decisions.md §12` L7; the local regen that follows is never committed, so it cannot un-verify
this run). Desktop crate (`apps/desktop/src-tauri`) not tested this cycle: no file under that
tree, nor any file it depends on, was touched by this cycle's own diff (confirmed:
`git status --porcelain` before every commit this cycle showed only
`src/bin/v06_work_inventory.rs` / `src/rules_core/pilot_compute/mod.rs` /
`scripts/completion_atlas.py` / `docs/retro/events/sd34-at-34-e3-002.jsonl` under this cycle's
own writes).

## Sweep population

`corpus_literal_sweep`: 48,708 examined, before and after — unchanged, since no
`data/corpus/**` file was added or regenerated this cycle.

## Oracle pin

N/A — no figure in this receipt came from the pinned PCGen oracle corpus.

## Movement, four buckets

- **Closure:** **6** — `"Fighter"`, `"Barbarian"`, `"Monk"`, `"Paladin"`, `"Rogue"`, `"Wizard"`
  (all `core_rulebook`, all `class_feature`), carrying `wiring_class: "computed"` (verified by
  direct post-regen read), moved `engine-does-not-hold` (bucket C) → `grounded` (bucket
  **DONE**) directly. Each is a genuine +1 choice-recognition record (rule-verified,
  non-fabricated magnitude) whose own explanation the engine now genuinely computes for every
  one of the six — no further wiring work is owed by these six records themselves.
- **Reclassification:** 0 this cycle (no unit moved between two non-DONE buckets).
- **Reachability:** **6** (one new `classify()` rung + one new probe + one new generalized
  pilot_compute function now answer `grounded` for these exact six corpus keys, reusing the
  SAME rule/choice-id/magnitude Fighter's own SD13-E5 explanation already established — no
  fabricated formula, no new engine mechanic).
- **Instrument-correction:** **2** — `base_class_standalone_feature_not_computed`'s own stated
  population, corrected 35 → 25 in cycle 7's own remainder table (cycle 7's own 13-row sum did
  not equal its claimed 199 total — a real arithmetic error, not merely stale prose); and
  `versatile_performance_not_computed`, corrected 0 → 1 (`"Bard ~ Versatile Performance"`'s own
  bare header record was never actually closed, contrary to cycle 7's claim it was closed cycle
  5 and unchanged). Both logged as `correction` retro events,
  `--verified-by` the categorization script itself.

**Bucket C's own delta (199 → 193, −6) equals this cycle's own Closure exactly** — the
row-count command's own output above is the ground truth this movement report is checked
against, not the other way around.

## Remainder — 193 of 199, named by mechanism, populations sum exactly

Re-derived fresh at this cycle's own close (`decisions.md §12` L2), by exhaustive direct
per-key categorization of the live corpus (script, not eyeballed):

| Sub-cause | Population | Status / next step |
|---|---:|---|
| `monk_unarmed_damage_no_formula_in_engine` | **48** | Unchanged, re-confirmed. Genuine engine gap, TWO reasons (cycle 7's own finding, re-verified): the 42 non-Small/Medium band records have no transcribed formula anywhere in the engine (no playable race reaches those 7 sizes at all); the 6 `(Small)` records DO have a real transcribed formula, but it is wired ONLY into the Pathfinder Unchained Monk's own compute path, deliberately never reused for the Core Rulebook Monk's own chassis seam — a byte-identical guard test protects that boundary. Still the largest remaining named sub-cause. |
| `prestige_class_standalone_feature_not_computed` | **26** | Unchanged, re-confirmed (cycle 7's own stated figure, now independently verified by exhaustive per-key read rather than trusted). Named prestige-class features (Arcane Archer's arrow abilities, Dragon Disciple's draconic features + bloodline feat grant, Eldritch Knight/Loremaster/Mystic Theurge/Pathfinder Chronicler/Shadowdancer features) — no shared compute path exists; each is a genuinely distinct mechanic. Not attempted this cycle. |
| `bloodline_power_or_bloodline_feat_not_computed` | **25** | Unchanged, re-confirmed (cycle 7's own stated figure, independently verified). The residue after cycles 3/4's generic Sorcerer-Bloodline pool-group closure already took the reusable-formula slice; what remains (Elemental Movement/Body, Familiar bonding, sub-bloodline feats, Elemental sub-bloodlines) is each a genuinely distinct mechanic. Not attempted this cycle. |
| `base_class_standalone_feature_not_computed` | **25** (corrected from cycle 7's stated 35 — see Population section) | Real, distinct base-class mechanics with no shared compute path yet: Barbarian (Greater/Mighty Rage, Rage, Rage Powers pool tracker — 4), Cleric (Extra Channel, Channel Negative/Positive Energy — 3), Paladin (Extra Channel, Divine Bond, Smite Evil — 3), Druid (Wild Shape + its Progression/Times trackers — 3), Fighter (Level Advanced Feat Tracker — 1), Monk (AC Tracker, Bonus Feat Default/Improved Critical, Ki Pool, Monk Bonus Feat, Standard Class, Unarmed Damage tracker — 7), Rogue (Archetype Support, Rogue Talents tracker — 2), Wizard (Remove Scribe Scroll, Arcane School — 2). Not attempted this cycle. |
| `class_chassis_internal_tracker` | **16** | Unchanged, re-confirmed. The 16 `"<Class> ~ Class"` records are `completeness: "chassis_only"` internal PCGen `DEFINE`/pool-tracker bookkeeping records, never a player-facing value of their own. No explanation surface exists or should exist for these; out of this territory's wiring-only bar. Still an open definitional question for `atlas-defects.md` (cycle 7's own next-cycle plan item 2, not decided this cycle either). |
| `rage_power_not_computed` | **13** | Unchanged, re-confirmed. `CORE_RULEBOOK_RAGE_POWER_POOL` is a real, already-registered pool, but only ONE representative power (`Superstition`) has a real magnitude compute — a deliberate, already-shipped "ground one representative option per pool honestly" ruling. Each of the 13 remaining rage powers is a mechanically distinct effect with no shared formula. |
| `prestige_class_chassis_internal_tracker` | **10** | Unchanged, re-confirmed. Same shape as `class_chassis_internal_tracker` above, for the 10 prestige classes' own bare-name chooser records — `completeness: "chassis_only"`, never a player-facing value of their own. Out of this territory's wiring-only bar. |
| `rogue_talent_not_computed` | **10** | Unchanged, re-confirmed. The SAME "one representative per pool" idiom as Rage Power — only `Resiliency` has a real magnitude compute. Each of the 10 remaining named talents is a mechanically distinct effect. |
| `druid_nature_bond_domain_selection_not_computed` | **7** | Unchanged, re-confirmed. Genuine engine gap: `pilot_compute::mod.rs`'s own Task #64 comment states plainly that Nature Bond's domain option carries NO `DRUID_DOMAIN_CHOICE_ID` seam at all. |
| `favored_class_bonus_choice_no_seam__npc_classes` | **5** | Adept, Aristocrat, Commoner, Expert, Warrior — genuine engine gap, confirmed by direct search: none has a `supported_<class>_level` bounded chassis seam at all (`grep` for the five functions returns zero matches), so no explanation surface exists to wire to. Named for a future cycle that builds those seams (real new engine work, not wiring). |
| `ranger_favored_x_chassis_or_wild_empathy` | **5** | Unchanged, re-confirmed. `Basic Favored Enemy`, `Basic Favored Terrain`, `Common Favored Terrain` (internal chooser/pool-definition trackers), `Ranger ~ Favored Enemy`, `Ranger ~ Wild Empathy` (each its own distinct un-computed magnitude). Not attempted this cycle. |
| `domain_power_display_record_not_wired` | **2** | Unchanged, re-confirmed: the last bare header (`"Nobility Domain"`) plus its own zero-token granted-power record. Neither has a live-wired sibling of either reusable shape, and Nobility carries no `domain_power::DOMAIN_POWER_CATALOG` entry. |
| `versatile_performance_not_computed` | **1** (corrected from cycle 7's stated 0 — see Population section) | `"Bard ~ Versatile Performance"`, the bare chooser/header record cycle 5's own closure of the 9 individual Perform-type sub-records deliberately excluded (a different, `~`-suffixed corpus shape) and never separately wired since — the same "bare header left behind" shape `domain_power_display_record_not_wired`'s own `Nobility Domain` row already establishes. Named for a future cycle. |

**Sum check:** 48 + 26 + 25 + 25 + 16 + 13 + 10 + 10 + 7 + 5 + 5 + 2 + 1 = **193**, matching
the row-count command's own remainder exactly (199 − 6 = 193).

## Notes

- **This cycle's fix is deliberately minimal and additive**: one new `pilot_compute::mod.rs`
  function generalizing an already-shipped rule to five classes, one new `classify()` rung, one
  new `EngineFacts` field, one new probe. Fighter's own already-shipped function is completely
  untouched — zero regression risk to its own already-tested path. **Build generically**: one
  mechanism, six real closures (the same shape the dispatch brief's own "123 units across 6
  books from one change" example calls out).
- **Extensive due diligence before landing on this mechanism, per this dispatch's own
  instruction to confirm prior stated reasons still hold** (this bundle has had one cycle
  disprove another's). Re-derived cycle 7's ENTIRE remainder table from scratch by exhaustive
  per-key categorization (not sampling, not trusting prose) — found and corrected two real
  errors (see Population section): a genuine arithmetic mistake in cycle 7's own stated sum, and
  a genuinely stale "already closed" claim for `versatile_performance_not_computed`. Both logged
  as `correction` retro events.
- **`cargo test --locked --lib` (workspace library, run for the first time this cycle at the
  widest scope beyond the bin-scoped suites cycles 1–7 ran) surfaces 5 pre-existing failures,
  none caused by this cycle's diff**: `rules_core::class_feature_pool_catalog::tests::
  class_feature_owner_matched_non_excluded_remainder_is_24_and_named_by_subcause` (reads
  `docs/work-inventory.json` directly off disk — a file this cycle never wrote to; its own doc
  comment already documents this exact count as having drifted stale once before, from
  regenerations by OTHER lanes' cycles, e.g. Sorcerer 137/Cleric 38/Monk 25/Wizard 5/Paladin 5;
  last touched by `AT-34-E3-001` cycle 10, `935cef27b5`, well before this cycle and unrelated to
  it); three `rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::*` failures, all
  crashing inside a Python subprocess (`scripts/shape_ledger.py` → `coverage_ledger.py` →
  `pf1e_dashboard_producer.py`) with `ValueError: doneness: unmapped 'derived' + 'oracle-agree'`
  — a Python doneness-mapping gap unrelated to any Rust code this cycle touched; and
  `rules_core::rules_tables::companion_chassis::tests::
  grant_token_only_rows_dispatch_to_already_held_content` (a corpus/oracle-status check on an
  unrelated `Base Companion ~ Animal Companion` record). **Confirmed unrelated to this cycle's
  diff by `git status --porcelain`**: this cycle's own writes touch exactly two files
  (`src/bin/v06_work_inventory.rs`, `src/rules_core/pilot_compute/mod.rs`), neither of which
  `class_feature_pool_catalog.rs`, `formula_interpreter_corpus_wide.rs`, nor
  `companion_chassis.rs` import or call into for these specific assertions — all five read
  static committed corpus/inventory data or shell out to Python scripts this cycle never
  touched. Not fixed this cycle — out of this territory's wiring-only scope (a dashboard-producer
  Python bug and a stale hardcoded count in another lane's own test, not an explanation-id/
  diagnostic-naming gap).
- **Territory respected:** no `CharacterInput` field was added or changed; no trait/ability
  compute path was touched; the EQUIPMENT magnitude sub-causes (owned by a sibling lane) were
  not touched; `git status --porcelain` before every commit this cycle showed only this
  territory's own two source files plus the retro event log.
- **Not attempted this cycle**: every other named sub-cause in the 193-unit remainder table.

## Next-cycle plan

1. `class_chassis_internal_tracker` (16) and `prestige_class_chassis_internal_tracker` (10) — 26
   total — are internal PCGen bookkeeping records with no player-facing explanation surface,
   named as an open definitional question for `atlas-defects.md` two cycles running now (cycle
   7's next-cycle plan item 2, unchanged, not decided this cycle either): are these genuinely
   out of bucket C's scope (never player-facing, so never wireable), or does the doctrine
   require a dedicated "internal, never surfaced by design" bucket disposition?
2. `favored_class_bonus_choice_no_seam__npc_classes` (5) needs real new engine work (a
   `supported_<class>_level` bounded chassis seam built for each of the 5 NPC classes) before
   any wiring-only fix could reach it — an operator-scoped question about whether NPC classes
   are in scope for bounded-seam investment at all, not decided this cycle.
3. `versatile_performance_not_computed` (1, corrected this cycle) is the smallest, cleanest
   remaining wiring-only candidate this cycle found but did not attempt: `"Bard ~ Versatile
   Performance"`'s bare header record likely has a real sibling explanation (the same "paired
   display/chassis" shape every closure in this file has followed) worth a direct check before
   the next cycle picks a larger sub-cause.
4. `monk_unarmed_damage_no_formula_in_engine` (48, largest) needs real new formula work for the
   42 non-Small/Medium records (or a deliberate bucket `X` deferral) and a real cross-subsystem
   ADR for the 6 Small records — an operator-scoped question, not a wiring-only fix.
5. `base_class_standalone_feature_not_computed` (25, corrected this cycle) and
   `prestige_class_standalone_feature_not_computed` (26) are both unstarted; each record inside
   them needs its own per-feature verification before any is attempted.
6. Re-derive the remainder partition fresh before picking (`decisions.md §12` L2) — this
   receipt's own table is this cycle's fresh derivation; the NEXT cycle must re-run it fresh
   again rather than trust this one.
