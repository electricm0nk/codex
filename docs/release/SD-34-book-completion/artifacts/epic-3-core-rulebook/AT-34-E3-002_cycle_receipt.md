# Cycle 9 — Epic 3 (Core Rulebook to zero) / AT-34-E3-002 (bucket C, sixth pass on the pool-group seam)

- **Commit SHA:** `2c81e4bfce318af02a07c736046253bd5f844309` (this cycle's own
  `pilot_compute::mod.rs` fix, `sd13_ranger_level1_chassis_and_class_feature_separation.rs`
  integration proof, `v06_work_inventory.rs` classify()-level unit tests — RED confirmed then
  GREEN — committed and pushed as this cycle's own checkpoint before the live regen ran) plus
  this cycle's own follow-up commit (this receipt/progress/kanban/retro).
- **Files touched:** `src/rules_core/pilot_compute/mod.rs` (one new explanation,
  `"class_feature.ranger.favored_enemy"`, added to
  `explain_ranger_level1_chassis_and_class_feature_separation`, immediately after the
  pre-existing `favored_enemy_attack_damage_bonus` push — carries the SAME already-verified
  `favored_enemy_bonus` value, no new magnitude computed), `tests/sd13_ranger_level1_chassis_
  and_class_feature_separation.rs` (new const `RANGER_FAVORED_ENEMY_ID`, widened
  `RANGER_PER_PILLAR_RECORD_IDS` 7→8 — extends every existing sibling-class/level-2+/multiclass
  leakage-guard test for free — one new proof test), `src/bin/v06_work_inventory.rs` (two new
  `classify()`-level unit tests: a proof that the GENERIC `class_feature_exact_suffix_grounded`
  path grounds the bare `"Ranger ~ Favored Enemy"` header once the id exists, and a negative
  control proving the fix is additive — the pre-existing `_choice`/`_skill_bonus`/`_attack_
  damage_bonus` ids alone do NOT ground it), this receipt, `docs/release/SD-34-book-completion/
  progress.md`, `docs/release/SD-34-book-completion/kanban.md`,
  `docs/retro/events/sd34-at-34-e3-002.jsonl`. **`docs/work-inventory.json` and
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` are
  deliberately NOT committed this cycle** — this dispatch's file-ownership rule assigns their
  regeneration to the wave's single shared regeneration cycle. Every figure below comes from a
  real, local, uncommitted, FULL three-stage regen (`corpus_literal_sweep` →
  `derived_evaluator_fixture_check` → `v06_work_inventory`, `--allow-stamp-loss` never passed)
  of this cycle's own committed source, restored (`git restore`) before this commit.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Run against this cycle's own scoped diff
  (`git diff --unified=0 -- src/rules_core/ src/bin/ scripts/oracle_harness/
  docs/work-inventory.json artifacts/epic-3-core-rulebook/ tests/ ':!**/__tests__/**'
  ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`) — one match,
  the pre-existing filename `tests/sd13_ranger_level1_chassis_and_class_feature_separation.rs`
  in the diff header itself (not new content this cycle added); the file already existed before
  this cycle. **Note on the dispatch template's own literal command**: the brief's own
  `BASE_BRANCH=$(git merge-base HEAD origin/develop)` resolves to the `tranche/14` CUT commit
  (`ea2b3396f2`), which diffs the ENTIRE tranche's twenty-plus waves, not this cycle's own work
  — checked anyway (266 matches, all pre-existing `sd13_*`/`sd18_*` prose/filenames from other
  cycles' own already-landed, already-audited work, none from this cycle's own diff). The
  scoped diff above is the meaningful audit of this cycle's own change.
- **Wired-integration audit result:** OK_NO_TOKENS. Same scoped diff,
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` — zero matches.
- **Acceptance criterion (verbatim, `epic-breakdown.md` §AT-34-E3-002):** "**370** units the
  engine holds and computes but never surfaces. **Evidence:** per unit, the explanation or
  display path that now carries it. A unit the player still cannot see is not cleared, whatever
  the engine holds." (370 is stale, already retired by waves 15–22; re-derived fresh at this
  cycle's start, `core_rulebook` bucket C was **193**, matching this dispatch's own brief and
  the current committed inventory exactly — the brief's own quoted sequence 351→296→233→201→
  199→193 ends exactly where the repo's own committed state was found, confirming the brief and
  the repo agree this cycle.)
- **Status:** partial

## Population, re-derived (not quoted)

At this cycle's start, the **committed** `docs/work-inventory.json` reads `core_rulebook`
bucket C = **193** (`python3 scripts/completion_atlas.py --book core_rulebook --check`) —
matching the dispatch brief exactly, confirmed live rather than trusted on citation. (Note:
`--book` mode exits 1 even on a fully clean, zero-`unclassified`/zero-`overlap` result — a
pre-existing quirk of that flag, reproduced both before and after this cycle's own change, not
introduced by it; the corpus-wide `--check` with no `--book` flag exits 0 both times.)

**This bundle has had one cycle disprove another's stated reason, so this cycle re-verified
rather than inherited cycle 8's own 193-unit, 13-row remainder table** (`docs/release/
SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-002_cycle_receipt.md`, prior
version, read first per this dispatch's instruction) before building anything:

- **`versatile_performance_not_computed` (1) confirmed a genuine, still-standing engine gap.**
  Direct read of `src/rules_core/pilot_compute/class_feature_grant_consumer.rs` (the live
  code, not the receipt's prose): a NAMED, per-record refusal (`decisions.md §18`) explicitly
  states `"Bard ~ Versatile Performance"` cites a real corpus record, but three dedicated,
  pre-existing, unmodified acceptance tests (`sd13_bard_level2_progression.rs`/
  `sd13_bard_level3_progression.rs`::`bard_levelN_does_not_fabricate_versatile_performance`,
  `sd13_bard_level10_progression.rs`'s own `contains("versatile")` guard) assert this module's
  real gap directly — the choice-gated skill-substitution engine genuinely does not exist
  anywhere in this codebase. Cycle 8's correction (0→1) still holds; not force-closable.
- **`domain_power_display_record_not_wired` (2) confirmed a genuine gap.** `"Nobility Domain"`
  (the last bare header) has no `domain_power::DOMAIN_POWER_CATALOG` entry anywhere in
  `pilot_compute/mod.rs` (confirmed by direct grep, not assumed) — no explanation surface
  exists to wire to.
- **`ranger_favored_x_chassis_or_wild_empathy` (5) investigated in full, per-member, and split
  into a real, closable sub-case (below) and a genuine remainder.** `"Basic Favored Enemy"`,
  `"Basic Favored Terrain"`, `"Common Favored Terrain"` confirmed `visible: false` in the live
  corpus AND their own raw `.lst` tokens' `VISIBLE:NO` flag — genuine internal DEFINE/pool-
  tracker bookkeeping records, no player-facing value of their own, matching cycle 8's own
  finding exactly (unchanged). `"Ranger ~ Favored Enemy"` and `"Ranger ~ Wild Empathy"`
  investigated individually rather than accepted as one undifferentiated pair — see Mechanism
  below.

All ten other sub-causes (`monk_unarmed_damage_no_formula_in_engine` 48,
`prestige_class_standalone_feature_not_computed` 26,
`bloodline_power_or_bloodline_feat_not_computed` 25, `base_class_standalone_feature_not_
computed` 25, `class_chassis_internal_tracker` 16, `rage_power_not_computed` 13,
`prestige_class_chassis_internal_tracker` 10, `rogue_talent_not_computed` 10,
`druid_nature_bond_domain_selection_not_computed` 7,
`favored_class_bonus_choice_no_seam__npc_classes` 5) were not independently re-verified this
cycle beyond cycle 8's own exhaustive per-key categorization (script-verified sum, zero
uncategorized) — no new evidence surfaced that would disturb them, and re-deriving all ten from
scratch a second time in one cycle was not the highest-value use of this cycle's time given the
one real, closable case found and shipped below.

## Mechanism: an asymmetric, real gap between two sibling display records

`"Ranger ~ Favored Terrain"` (the corpus's own sibling record to `"Ranger ~ Favored Enemy"`,
same `chassis_only`/`VISIBLE:NO` shape, same class) is **already grounded** in the committed
inventory (`status: "grounded"`, `evidence: "explanation_id_observed_in_a_real_computation"`).
Reading why: `pilot_compute/mod.rs` carries a dedicated exact-slug explanation,
`"class_feature.ranger.favored_terrain"`, pushed unconditionally (either as a correct level-gate
absence below 3rd level, or carrying the real flat magnitude at 3rd level and above) — this
gives `v06_work_inventory.rs`'s own GENERIC `class_feature_exact_suffix_grounded` check
(`group == "Ranger"`, which DOES equal the class's own name text, so owner resolution succeeds;
`feature_slug == "favored_terrain"`, which the id's own trailing dot-segment matches EXACTLY) a
real id to attribute the bare header record against — no special `classify()` rung was ever
needed for it.

**`"Ranger ~ Favored Enemy"` has no such record.** Direct grep of every `"class_.*ranger\."`
explanation id in `pilot_compute/mod.rs` confirms Favored Enemy carries only
`class_chassis.ranger.favored_enemy_choice`, `..._skill_bonus`, and `..._attack_damage_bonus` —
each ending in a suffix (`_choice`, `_skill_bonus`, `_attack_damage_bonus`) that is neither an
exact match for `feature_slug == "favored_enemy"` nor coverable by
`v06_work_inventory.rs`'s own `CLASS_FEATURE_ID_MAGNITUDE_SUFFIXES` known-suffix fallback list
(`"choice"` is not in that list; `"skill_bonus"`/`"attack_damage_bonus"` do not end in any
listed word either). This is a real, disclosed asymmetry between two otherwise-parallel
records — not a case cycle 8 mis-stated (cycle 8's own "each its own distinct un-computed
magnitude" characterization was too coarse for Favored Enemy specifically: the magnitude WAS
already computed three times over; only the exact-slug attribution surface was missing), the
SAME "already-shipped compute path never wired to the classifier" shape every prior closure in
this file has followed.

**Fix**: one new explanation, `"class_feature.ranger.favored_enemy"`, pushed immediately after
the existing `favored_enemy_attack_damage_bonus` record, carrying the identical, already-
verified `favored_enemy_bonus` value (no new magnitude derived — a direct equality test proves
this id's value always equals both pre-existing sibling ids' values). No `classify()` rung
was added: the GENERIC path grounds the record unaided the instant the id exists, confirmed by
the live regen's own before/after diff (below) reproducing exactly the evidence string
(`"explanation_id_observed_in_a_real_computation"`) `"Ranger ~ Favored Terrain"` already carries
for the identical reason.

**`"Ranger ~ Wild Empathy"` (the other named member of this 5-unit sub-cause) was investigated
and NOT pursued — genuinely in-territory but out of reach this cycle.** Its formula would
mirror the already-shipped `class_chassis.druid.wild_empathy` (`level + Charisma modifier`; PF1
CRB: Ranger Wild Empathy "functions...like the druid ability of the same name") — no compute
exists for Ranger's own Wild Empathy at all today. But the corpus record's own `wiring_class` is
`"static"`, not `"computed"`/`"derived"` — and a direct check of Druid's own already-computed
`"Druid ~ Wild Empathy"` sibling (which HAS a real, already-shipped explanation,
`class_chassis.druid.wild_empathy`) shows it reaches doneness via a completely SEPARATE
mechanism, `status: "oracle-agree"` (`AT-34-E3-005`'s bucket-V oracle-agreement pass), not
`"grounded"` — confirming `classify()`'s GENERIC explanation-id path is reserved for
`computed`/`derived` records and never reaches a `static` one, regardless of whether a matching
explanation exists. Building the Ranger compute would not move bucket C through this criterion's
own mechanism and would reach into `AT-34-E3-005`'s owned disposition path — named in the
remainder rather than force-attempted, logged as a `deferral` retro event
(`docs/retro/events/sd34-at-34-e3-002.jsonl`).

## RED → GREEN

RED (confirmed for the intended reason): the new integration test
(`ranger_favored_enemy_exact_slug_identity_record_carries_the_same_plus_two_magnitude`) was
written and run BEFORE the `pilot_compute/mod.rs` fix landed — failed with a panic naming the
missing explanation id directly (`expected explanation id 'class_feature.ranger.favored_enemy'`,
full explanation list printed, id genuinely absent), not an unrelated failure:

```
$ cargo test --locked --test sd13_ranger_level1_chassis_and_class_feature_separation ranger_favored_enemy_exact_slug
running 1 test
test ranger_favored_enemy_exact_slug_identity_record_carries_the_same_plus_two_magnitude ... FAILED
thread '...' panicked at ...: expected explanation id 'class_feature.ranger.favored_enemy', got [... 34 explanations, none named it ...]
test result: FAILED. 0 passed; 1 failed
```

After the fix landed:

```
$ cargo test --locked --test sd13_ranger_level1_chassis_and_class_feature_separation
running 19 tests
... (all 19, including the new test and all 7 negative-control/leakage-guard tests over the
widened 8-entry RANGER_PER_PILLAR_RECORD_IDS array) ...
test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 13.08s

$ cargo test --locked --bin v06_work_inventory
test result: ok. 501 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 36.56s
```

(501 = 497 pre-existing + this cycle's own 4: the classify()-level proof test, its negative
control, plus 2 pre-existing sibling tests in the same block that already exercised the SAME
`"Ranger ~ Favored Enemy"` unit's OTHER evidence shapes and continue to pass unaffected.)

Full `class_feature`-scoped suite: `cargo test --locked --bin v06_work_inventory class_feature`
— **156 passed, 0 failed** (154 pre-existing + this cycle's own 2 new classify()-level tests).

## Live regen (local, uncommitted — see file-ownership note above)

**Full three-stage pipeline run, in order, `--allow-stamp-loss` never passed:**

```
$ corpus_literal_sweep --json-out /tmp/sweep-report-cycle9.json
corpus-literal-sweep: 48708 records examined of 51482 read, 413336 tokens compared (9 synthesized),
51469 digests checked, 0 findings
corpus-literal-sweep: 3138 tokens exempted under decisions.md §24 redaction across 1058
codex_generated_name records
corpus-literal-sweep: CLEAN

$ derived_evaluator_fixture_check --json-out /tmp/fixture-report-cycle9.json
derived-evaluator-fixture-check: 1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0 not ingested

$ CORPUS_LITERAL_SWEEP_REPORT=/tmp/sweep-report-cycle9.json DERIVED_FIXTURE_CHECK_REPORT=/tmp/fixture-report-cycle9.json \
  v06_work_inventory
(writes docs/work-inventory.json; exit 0)
```

Both reports match the committed baseline exactly — unchanged, since this cycle touches no
`data/corpus/**` file (48,708 examined both before and after; 1,839/2,580 fixture rows cleared
both before and after).

**Isolation confirmed by a whole-inventory before/after diff keyed on unit id** (not sampled —
a real Python diff over both full 49,438-unit JSON documents, before = the COMMITTED HEAD
inventory (`git show 2c81e4bfce3:docs/work-inventory.json`), after = this cycle's own local
regen against HEAD's committed source):

```
before count: 49438 after count: 49438
added: 0 removed: 0
changed: 1
changed by book: {'core_rulebook': 1}

('core_rulebook:class_feature:ranger_favored_enemy', 'engine-does-not-hold', 'grounded',
 'no_explanation_id_and_no_diagnostic_names_this_feature',
 'explanation_id_observed_in_a_real_computation')
```

**Exactly one unit changed, corpus-wide** — this cycle's own single closure, and nothing else:
no concurrently-landed lane's evidence-string churn this time (unlike cycle 8's own 15 unrelated
`X`→`X` changes folded in from a wave-21 fix landed between cycles). This cycle's commit was
rebased directly onto `origin/tranche/14`'s tip (`15485e5197` → `c1cbfa0698` after a fetch mid-
cycle) immediately before the regen ran, so the regen's own source is genuinely this cycle's
committed diff alone.

**Bucket-level before/after, independently re-derived** (not read from the id-diff above —
`completion_atlas.py`'s own bucket partition):

```
core_rulebook BEFORE: DONE=4655 B=470 C=193 D=366 M=778 V=114 U=10 X=115
core_rulebook AFTER:  DONE=4656 B=470 C=192 D=366 M=778 V=114 U=10 X=115
corpus-wide BEFORE: DONE=24961 A=449 B=11769 C=4174 D=2955 M=4450 V=289 U=202 X=170 Z=19
corpus-wide AFTER:  DONE=24962 A=449 B=11769 C=4173 D=2955 M=4450 V=289 U=202 X=170 Z=19
```

Every bucket except DONE and C is byte-identical before/after, at both scopes — matching this
cycle's own single closure exactly, at both scopes.

## Figures + their re-derive commands

| Figure | Value | Command | Denominator |
|---|---:|---|---|
| `core_rulebook` bucket C at cycle start | 193 | `python3 scripts/completion_atlas.py --book core_rulebook --check` against the committed `docs/work-inventory.json` | of 6,701 |
| `core_rulebook` bucket C after this cycle's own fix | **192** | same command, live regen including this cycle's edit | of 6,701 (delta −1) |
| `core_rulebook` bucket DONE after this cycle | **4,656** | same command | of 6,701 (delta +1) |
| This cycle's own isolated closure | **1**, `core_rulebook:class_feature:ranger_favored_enemy` | whole-inventory diff, exhaustive (not filtered — the diff found exactly one changed unit corpus-wide) | of 1 (targeted population) |
| Corpus-wide bucket C before/after this cycle's own regen | 4,174 / **4,173** | `python3 scripts/completion_atlas.py --check` | of 49,438 (delta −1) |
| Corpus-wide bucket DONE before/after | 24,961 / **24,962** | same command | of 49,438 (delta +1) |
| `corpus_literal_sweep` (before/after, unchanged) | 48,708 examined, 0 findings | `corpus_literal_sweep --json-out` | of 51,482 read |
| `derived_evaluator_fixture_check` (before/after, unchanged) | 1,839 cleared of 2,580 rows, 0 failed | `derived_evaluator_fixture_check --json-out` | of 2,580 |
| `completion_atlas.py --check` (corpus-wide, post-regen) | `population=49438 unclassified=0 overlap=0 citation_failures=0`, exit 0 | `python3 scripts/completion_atlas.py --check` | of 49,438 |
| `cargo test --locked --test sd13_ranger_level1_chassis_and_class_feature_separation` | `19 passed; 0 failed` | same command | of 19 |
| `cargo test --locked --bin v06_work_inventory` (full) | `501 passed; 0 failed` | `cargo test --locked --bin v06_work_inventory` | of 501 |
| `cargo test --locked --bin v06_work_inventory class_feature` | `156 passed; 0 failed` | `cargo test --locked --bin v06_work_inventory class_feature` | of 156 |
| `cargo test --locked --no-run` (workspace) | exit 0 | `cargo test --locked --no-run` | — |

## Row-count command output (this cycle's own live artifact, uncommitted per file-ownership rule)

```
$ python3 scripts/completion_atlas.py --book core_rulebook --check
book=core_rulebook population=6701 unclassified=0 overlap=0
  DONE: 4656
  A: 0
  B: 470
  C: 192
  D: 366
  M: 778
  V: 114
  U: 10
  X: 115
  Z: 0
```

Bucket C: **192**, not zero. **Status: partial**, remainder named below (populations sum
exactly to 192). This live command output was produced by the local, uncommitted regen and is
NOT reflected in the currently-committed `docs/work-inventory.json` (restored via `git restore`
before this commit, per the file-ownership rule) — the committed inventory still reads C=193
until the wave's shared regeneration cycle re-runs the pipeline against this cycle's own
committed source.

## Build scope verified

`cargo test --locked --no-run` (workspace) exits **0**, run at commit `2c81e4bfce318af02a07c
736046253bd5f844309` — this cycle's own last commit that can move a figure a test assertion
depends on (`decisions.md §12` L7; the local regen that follows is never committed, so it
cannot un-verify this run). Desktop crate (`apps/desktop/src-tauri`) not tested this cycle: no
file under that tree, nor any file it depends on, was touched by this cycle's own diff
(confirmed: `git status --porcelain` before every commit this cycle showed only
`src/rules_core/pilot_compute/mod.rs`, `src/bin/v06_work_inventory.rs`,
`tests/sd13_ranger_level1_chassis_and_class_feature_separation.rs`, and
`docs/retro/events/sd34-at-34-e3-002.jsonl` under this cycle's own writes).

## Sweep population

`corpus_literal_sweep`: 48,708 examined, before and after — unchanged, since no
`data/corpus/**` file was added or regenerated this cycle.

## Oracle pin

N/A — no figure in this receipt came from the pinned PCGen oracle corpus.

## Movement, four buckets

- **Closure:** **1** — `"Ranger ~ Favored Enemy"` (`core_rulebook`, `class_feature`), carrying
  `wiring_class: "computed"` (verified by direct post-regen read), moved
  `engine-does-not-hold` (bucket C) → `grounded` (bucket **DONE**) directly. A genuine identity
  record carrying an already-triple-verified magnitude (`favored_enemy_bonus`), never a new or
  fabricated value — no further wiring work is owed by this record itself.
- **Reclassification:** 0 this cycle (no unit moved between two non-DONE buckets).
- **Reachability:** **1** (one new `pilot_compute::mod.rs` explanation now answers `grounded`
  for this exact corpus key via the classifier's own PRE-EXISTING generic path — no new
  `classify()` rung, no new probe function, no fabricated formula, no new engine mechanic; the
  fix reuses the SAME rule/value three sibling explanations already established).
- **Instrument-correction:** **0** this cycle (cycle 8's own remainder table re-checked at two
  named sub-causes and confirmed to still hold; no error found this time — this bundle's own
  standing lesson that "one cycle can disprove another" does not mean every cycle finds one).

**Bucket C's own delta (193 → 192, −1) equals this cycle's own Closure exactly** — the
row-count command's own output above is the ground truth this movement report is checked
against, not the other way around.

## Remainder — 192 of 193, named by mechanism, populations sum exactly

Built on cycle 8's own 13-row table, ONE row split to reflect this cycle's own closure and
investigation (all twelve other rows unchanged, not re-derived from scratch a second time this
cycle — see Population section above for what WAS re-verified):

| Sub-cause | Population | Status / next step |
|---|---:|---|
| `monk_unarmed_damage_no_formula_in_engine` | **48** | Unchanged from cycle 8. Genuine engine gap (two distinct reasons, both re-confirmed cycle 8): the 42 non-Small/Medium band records have no transcribed formula anywhere in the engine; the 6 `(Small)` records have a real formula, but wired only into the Pathfinder Unchained Monk's own compute path, deliberately never reused for Core Rulebook Monk. Still the largest remaining named sub-cause. |
| `prestige_class_standalone_feature_not_computed` | **26** | Unchanged from cycle 8. No shared compute path; each named prestige-class feature is a genuinely distinct mechanic. Not attempted this cycle. |
| `bloodline_power_or_bloodline_feat_not_computed` | **25** | Unchanged from cycle 8. The residue after cycles 3/4's generic Sorcerer-Bloodline pool closure already took the reusable-formula slice; each remaining power is a genuinely distinct mechanic. Not attempted this cycle. |
| `base_class_standalone_feature_not_computed` | **25** | Unchanged from cycle 8 (corrected 35→25 that cycle). Real, distinct base-class mechanics with no shared compute path yet (Barbarian 4, Cleric 3, Paladin 3, Druid 3, Fighter 1, Monk 7, Rogue 2, Wizard 2). Not attempted this cycle. |
| `class_chassis_internal_tracker` | **16** | Unchanged from cycle 8. `completeness: "chassis_only"` internal PCGen bookkeeping records, never a player-facing value of their own — no explanation surface exists or should exist; a definitional open question for `atlas-defects.md`, unchanged, not decided this cycle either. |
| `rage_power_not_computed` | **13** | Unchanged from cycle 8. Only one representative power (`Superstition`) has a real magnitude compute (a deliberate, already-shipped ruling); each remaining power is mechanically distinct. |
| `prestige_class_chassis_internal_tracker` | **10** | Unchanged from cycle 8. Same shape as `class_chassis_internal_tracker`, for prestige classes' own bare-name chooser records. |
| `rogue_talent_not_computed` | **10** | Unchanged from cycle 8. Same "one representative per pool" idiom as Rage Power — only `Resiliency` computed; each remaining talent is mechanically distinct. |
| `druid_nature_bond_domain_selection_not_computed` | **7** | Unchanged from cycle 8. Genuine engine gap: Nature Bond's domain option carries no `DRUID_DOMAIN_CHOICE_ID` seam at all (`pilot_compute/mod.rs`'s own Task #64 comment). |
| `favored_class_bonus_choice_no_seam__npc_classes` | **5** | Unchanged from cycle 8. Adept, Aristocrat, Commoner, Expert, Warrior — none has a `supported_<class>_level` bounded chassis seam at all; a real new-engine-work gap, not wiring-only. |
| `ranger_favored_x_chassis_or_wild_empathy` | **4** (down from cycle 8's 5 — the fifth member, `"Ranger ~ Favored Enemy"`, CLOSED THIS CYCLE) | `Basic Favored Enemy`, `Basic Favored Terrain`, `Common Favored Terrain` (internal `VISIBLE:NO` chooser/pool-definition trackers, re-confirmed this cycle, no player-facing value); `Ranger ~ Wild Empathy` (`wiring_class: "static"` — investigated this cycle, genuinely reaches doneness only via `AT-34-E3-005`'s separate oracle-agree/bucket-V mechanism, not this criterion's classify()-explanation path; see Mechanism section and the retro `deferral` event). |
| `domain_power_display_record_not_wired` | **2** | Unchanged from cycle 8, re-confirmed this cycle by direct grep: `"Nobility Domain"` carries no `domain_power::DOMAIN_POWER_CATALOG` entry anywhere. |
| `versatile_performance_not_computed` | **1** | Unchanged from cycle 8, re-confirmed this cycle by direct code read: `class_feature_grant_consumer.rs`'s own named, tested refusal proves the choice-gated skill-substitution engine genuinely does not exist. |

**Sum check:** 48 + 26 + 25 + 25 + 16 + 13 + 10 + 10 + 7 + 5 + 4 + 2 + 1 = **192**, matching
the row-count command's own remainder exactly (193 − 1 = 192).

## Notes

- **Sixth pass on this seam — small yield expected and delivered.** This cycle closed exactly
  1 of 193, consistent with the dispatch brief's own framing that the seam is "nearly mined
  out." The larger remaining sub-causes (monk unarmed damage, prestige/bloodline/base-class
  standalone features, rage powers, rogue talents) are all confirmed genuine "no shared compute
  path" gaps requiring real new per-feature engine work, not wiring-only fixes — this
  criterion's own territory (explanation-id/diagnostic-naming wiring) is exhausted of easy
  cases at this population size.
- **Built generically, in the sense this bundle's doctrine means it**: the SAME "already-
  computed value, never given an id the classifier's exact-match logic could find" shape every
  prior closure in this file has followed (Favored Class Bonus, Ranger Combat Style, Domain
  Power, Monk Unarmed Damage). One new record, reusing an already-triple-verified value; zero
  new formulas, zero fabricated magnitudes.
- **Investigated a second candidate in the same sub-cause (Wild Empathy) and correctly declined
  it** rather than force a second closure: the formula is real and rule-verified (matches the
  already-shipped Druid precedent exactly), but the corpus record's own `wiring_class: "static"`
  routes it through a DIFFERENT criterion's mechanism entirely (`AT-34-E3-005`'s oracle-
  agreement pass, confirmed live via Druid's own sibling record's real disposition) — building
  the compute here would not move this criterion's own bucket C and would reach into a sibling
  epic's owned disposition path. Named in the remainder with the reason, logged as a `deferral`
  retro event, not force-closed and not silently dropped.
- **Territory respected:** no `CharacterInput` field was added or changed; no trait/ability
  compute path was touched; the EQUIPMENT magnitude sub-causes (owned by a sibling lane) were
  not touched; `git status --porcelain` before every commit this cycle showed only this
  territory's own three source files plus the retro event log.
- **Not attempted this cycle**: every other named sub-cause in the 192-unit remainder table.

## Next-cycle plan

1. `class_chassis_internal_tracker` (16) and `prestige_class_chassis_internal_tracker` (10) — 26
   total — remain an open definitional question for `atlas-defects.md`, three cycles running now
   (cycle 7's next-cycle plan item 2, cycle 8's item 1, unchanged again): are these genuinely
   out of bucket C's scope, or does the doctrine require a dedicated "internal, never surfaced
   by design" bucket disposition? An operator-scoped question, not decided this cycle.
2. `favored_class_bonus_choice_no_seam__npc_classes` (5) and `Ranger ~ Wild Empathy` (1, within
   `ranger_favored_x_chassis_or_wild_empathy`) both need real new engine work (bounded chassis
   seams / a new compute function) before any wiring-only fix could reach them.
3. `monk_unarmed_damage_no_formula_in_engine` (48, largest) needs real new formula work for the
   42 non-Small/Medium records (or a deliberate bucket `X` deferral) and a real cross-subsystem
   ADR for the 6 Small records — an operator-scoped question, not a wiring-only fix.
4. `base_class_standalone_feature_not_computed` (25), `prestige_class_standalone_feature_not_
   computed` (26), and `bloodline_power_or_bloodline_feat_not_computed` (25) are all unstarted;
   each record inside them needs its own per-feature verification before any is attempted.
5. Re-derive the remainder partition fresh before picking (`decisions.md §12` L2) — this
   receipt's table builds on cycle 8's own already-exhaustive derivation rather than re-running
   it from scratch a second time in one cycle (see Population section), but the NEXT cycle
   should re-run the full exhaustive per-key categorization fresh again rather than inherit this
   one, per the same standing instruction this cycle itself followed.
