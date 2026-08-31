# Cycle 7 — Epic 3 (Core Rulebook to zero) / AT-34-E3-002 (bucket C, "held and computed, never surfaced")

- **Commit SHA:** `f0d724d2c8` (this cycle's own `classify()` fix + probe + 4 tests, a checkpoint
  commit made mid-cycle per the dispatch's clock discipline, pushed to `tranche/14` before this
  cycle's own live regen ran; `tranche/14` tip `c320c61c4f` — wave 20 — at cycle start, no rebase
  needed) plus this cycle's own follow-up commit `5259f4458f` (`scripts/completion_atlas.py`'s 10
  citation re-pins, this cycle's own 186-line insertion having shifted every one) plus this
  cycle's own final receipt/progress/kanban/retro commit (this commit itself — see `git log -1`
  on `tranche/14` immediately after this receipt lands for its own SHA, the same convention a
  commit uses for its own hash).
- **Files touched:** `src/bin/v06_work_inventory.rs` (one new `EngineFacts` field
  `ranger_combat_style_choice_wired: BTreeSet<String>`, one new probe function
  `probe_ranger_combat_style_wiring`, one new `classify()` rung for `group == "Ranger Combat
  Style"`, 4 new tests — 2 positive proof + 2 negative controls, RED confirmed then GREEN),
  `scripts/completion_atlas.py` (10 citation line pins re-derived after this cycle's own 186-line
  insertion shifted every one below it — exact-line-content grep against `git show
  HEAD~1:...`, never guessed), this receipt, `docs/release/SD-34-book-completion/progress.md`,
  `docs/release/SD-34-book-completion/kanban.md`. **`docs/work-inventory.json` and
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` are
  deliberately NOT committed this cycle** — this dispatch's own file-ownership rule assigns their
  regeneration to the wave's single shared regeneration cycle. Every figure below comes from a
  real, local, uncommitted, FULL three-stage regen (`corpus_literal_sweep` →
  `derived_evaluator_fixture_check` → `v06_work_inventory`, `--allow-stamp-loss` never passed) of
  this cycle's own committed source, restored (`git restore`) before this commit.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Run twice: (1) against this cycle's own full
  commit range, `git diff --unified=0 c320c61c4f...HEAD -- src/rules_core/ src/bin/
  scripts/oracle_harness/ docs/work-inventory.json artifacts/epic-3-core-rulebook/
  ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  — `c320c61c4f` is `tranche/14`'s tip at this cycle's own start (wave 20) — zero matches; (2)
  against the full `merge-base(HEAD, origin/develop)...HEAD` range on the same Epic 3
  file-touch set per the dispatch's own audit template — that much wider, whole-bundle-history
  diff surfaces pre-existing matches from earlier, already-audited cycles only (prior cycles' own
  bundle-tagged retro-event fixture strings and test names), none inside this cycle's own hunks.
- **Wired-integration audit result:** OK_NO_TOKENS. Same two diffs, same
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` — zero matches on
  this cycle's own commit range; the wide-range diff surfaces the same pre-existing
  corpus-vocabulary `placeholder` mentions cycle 6's own receipt already documented and audited,
  none inside this cycle's own hunks.
- **Acceptance criterion (verbatim, `epic-breakdown.md` §AT-34-E3-002):** "**370** units the
  engine holds and computes but never surfaces. **Evidence:** per unit, the explanation or
  display path that now carries it. A unit the player still cannot see is not cleared, whatever
  the engine holds." (370 is stale, already retired by waves 15–19; re-derived fresh at this
  cycle's start, `core_rulebook` bucket C was **201**, matching this dispatch's own brief exactly
  — the committed inventory already carried cycle 6's own folded-in fix via the wave-19 shared
  regen, so no unregenerated-source gap existed at this cycle's start.)
- **Status:** partial

## Population, re-derived (not quoted)

At this cycle's start, the **committed** `docs/work-inventory.json` (last regenerated at
wave-19, `accb12b14d`) reads `core_rulebook` bucket C = **201**
(`python3 scripts/completion_atlas.py --book core_rulebook --check`) — matching cycle 6's own
closing figure exactly, confirmed live rather than trusted on citation.

This cycle re-derived cycle 6's own remainder table fresh, by direct corpus read against
`docs/work-inventory.json` (a Python categorization script over the live `engine-does-not-hold`
+ `no_explanation_id_and_no_diagnostic_names_this_feature` population, never eyeballed), **before
writing any code** (`decisions.md §12` L2). This found and corrected one real count error in
cycle 6's own table: `monk_unarmed_damage_no_formula_in_engine` was stated as **42**; the live
corpus carries **48** (all 8 non-Medium creature-size columns × 6 levels each — Colossal,
Diminutive, Fine, Gargantuan, Huge, Large, Small, Tiny — confirmed by grouping every bucket-C
`"Monk Unarmed Damage LVL *"` key by its own size suffix). The 6 `(Small)` records were missing
from cycle 6's own count. Logged as a `correction` retro event
(`docs/retro/events/sd34-at-34-e3-002.jsonl`), `--verified-by` the categorization command itself.

Cross-checked the other ten of cycle 6's named sub-causes by the same direct-read method: all
still hold their stated populations and reasons (`bloodline_power_or_bloodline_feat_not_computed`
25, `rage_power_not_computed` 13, `rogue_talent_not_computed` 10, `druid_nature_bond_domain_
selection_not_computed` 7, `domain_power_display_record_not_wired` 2, `versatile_performance_
not_computed` 0 — the remaining three, `base_class_standalone_feature_not_computed` /
`prestige_class_standalone_feature_not_computed` / `other_named_group_or_standalone`, are
re-partitioned below into more precise mechanism names after direct corpus reads this cycle
performed on each; see the remainder table's own notes for the mapping).

## Mechanism: the SAME paired display/chassis pattern already established (Favored Enemy /
## Favored Terrain / Domain header), applied to Ranger's Combat Style choice

Cycle 5/6's own next-cycle plan named `base_class_standalone_feature_not_computed` (36) and
`prestige_class_standalone_feature_not_computed` (31) as worth a direct scan before picking.
This cycle performed that scan and found, by reading the raw corpus JSON directly (not assumed
from key shape), that most of both groups decompose into TWO shapes this territory's wiring-only
bar cannot reach without new engine work:

1. **`"<Class> ~ Class"` and bare prestige-class-name records** (`Barbarian ~ Class`, `Arcane
   Archer`, ...) are `"completeness": "chassis_only"` internal PCGen bookkeeping — their own raw
   tokens are `DEFINE:<Class>_CFP_Level|0` and sibling `BONUS:VAR` pool-tracker counters that feed
   OTHER records' `PRE` gates, never a player-facing value of their own (confirmed by reading
   `data/corpus/core_rulebook/class_feature/barbarian/barbarian-2.json` and `.../arcane_archer/
   arcane_archer.json` directly). No explanation surface exists, or should exist, for these — out
   of this territory's bar, not a naming-only fix.
2. A promising-looking lead was checked and DECLINED once verified: `class_chassis.<class>.
   base_attack_bonus` / `base_save.*` explanations are real and extensive (every base class has
   them), but they explain the **`Kind::Class`** record (already `modelled_class_books()`-
   registered and `grounded`, e.g. `core_rulebook:class:barbarian`) — a completely different unit
   from the `Kind::ClassFeature` `"<Class> ~ Class"` bookkeeping record this cycle's candidates
   actually are (`core_rulebook:class_feature:barbarian_class__d7fdbff333ee8aaf`). Crediting the
   latter off the former would have been exactly the "broadened matcher" doctrine forbids —
   confirmed by reading `classify()`'s own `Kind::Class` arm (`facts.class_effect_wired`, a
   wholly separate mechanism from `Kind::ClassFeature`'s owner/suffix matching) before declining.

What DID reach this cycle's own wiring-only bar, found in the same scan: `"Ranger Combat Style ~
Archery"` and `"~ Two-Weapon Combat"` — a real, already-computed, already-tested +0 recognition
explanation (`class_chassis.ranger.combat_style_choice`,
`explain_ranger_level1_chassis_and_class_feature_separation`) that names exactly which style the
character chose, which `classify()` had simply never been taught to consult for these two
records. `group` here (`"Ranger Combat Style"`) can never equal `"ranger"`, so
`class_feature_owner` and its fallbacks can never resolve an owner, and
`class_feature_exact_suffix_grounded`'s `group == owner` guard could never ground this record even
if one resolved — the sibling probe is the only real attribution path, the same shape every prior
Favored Enemy/Terrain/Domain rung in this file already establishes.

**Honest difference from Favored Enemy/Terrain, stated in the probe's own doc comment:** there is
no companion numeric magnitude to cross-check here — the style choice itself grants no flat bonus
(only the SEPARATE, later-gated bonus feat does, already recognized by a different, pre-existing
explanation). A genuinely-observed `choice_observed` — the explanation firing AND its own `detail`
naming the exact `style:*` selection id — is the whole, honest bar, the same "+0 but genuinely
observed and naming this exact record" idiom the domain header closure already established.

**Cross-book collision check performed before shipping** (the hazard the domain header closure's
own book guard exists for): `grep -rn '"key": "Ranger Combat Style` across `data/corpus/`
confirms only `core_rulebook` declares either exact key. A `grep -rl` hit inside
`advanced_class_guide`'s Hooded Champion record is only a `SERVESAS`-style `ABILITY` token
REFERENCING the Ranger's own record (`"Special Ability|AUTOMATIC|Ranger Combat Style ~
Archery"`), not a second declared unit with the same key — read directly, not assumed. No book
guard was needed (the classify() rung is therefore left unguarded, matching the Favored
Enemy/Terrain precedent, which also has none).

## RED → GREEN

RED (confirmed for the intended reason): temporarily changed the new rung's own membership check
from `facts.ranger_combat_style_choice_wired.contains(feature)` to
`facts.ranger_combat_style_choice_wired.contains("RED-CHECK-NEVER-MATCHES")` and re-ran the two
positive proof tests — both failed with `left: "engine-does-not-hold", right: "grounded"` (the
pre-existing fallthrough this cycle closes), confirming the tests fail because the fix is absent,
not for an unrelated reason. Restored the match; all four tests pass.

```
$ cargo test --locked --bin v06_work_inventory ranger_combat_style
running 4 tests
test class_feature_text_complete_rung_tests::a_ranger_combat_style_record_the_probe_never_observed_is_unaffected ... ok
test class_feature_text_complete_rung_tests::a_ranger_combat_style_record_at_a_wired_sibling_style_is_unaffected ... ok
test class_feature_text_complete_rung_tests::a_ranger_combat_style_archery_record_reaches_grounded_off_the_probes_wiring ... FAILED
test class_feature_text_complete_rung_tests::a_ranger_combat_style_two_weapon_combat_record_reaches_grounded_off_the_probes_wiring ... FAILED
  left: "engine-does-not-hold"
 right: "grounded"
```

After restoring the match:

```
$ cargo test --locked --bin v06_work_inventory ranger_combat_style
running 4 tests
test class_feature_text_complete_rung_tests::a_ranger_combat_style_archery_record_reaches_grounded_off_the_probes_wiring ... ok
test class_feature_text_complete_rung_tests::a_ranger_combat_style_record_the_probe_never_observed_is_unaffected ... ok
test class_feature_text_complete_rung_tests::a_ranger_combat_style_record_at_a_wired_sibling_style_is_unaffected ... ok
test class_feature_text_complete_rung_tests::a_ranger_combat_style_two_weapon_combat_record_reaches_grounded_off_the_probes_wiring ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 482 filtered out; finished in 0.00s
```

Full `class_feature`-scoped suite: `cargo test --locked --bin v06_work_inventory class_feature`
— **150 passed, 0 failed** (146 pre-existing + this cycle's own 4). Full bin suite: **486
passed, 0 failed** (482 + 4).

## Live regen (local, uncommitted — see file-ownership note above)

**Full three-stage pipeline run, in order, `--allow-stamp-loss` never passed:**

```
$ corpus_literal_sweep --json-out /tmp/sweep-report.json
corpus-literal-sweep: 48708 records examined of 51482 read, 413336 tokens compared (9 synthesized),
51469 digests checked, 0 findings
corpus-literal-sweep: 3138 tokens exempted under decisions.md §24 redaction across 1058
codex_generated_name records
corpus-literal-sweep: CLEAN

$ derived_evaluator_fixture_check --json-out /tmp/fixture-report.json
derived-evaluator-fixture-check: 1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0 not ingested

$ CORPUS_LITERAL_SWEEP_REPORT=/tmp/sweep-report.json DERIVED_FIXTURE_CHECK_REPORT=/tmp/fixture-report.json \
  v06_work_inventory
(writes docs/work-inventory.json; exit 0)
```

Both reports match wave-19's own baseline exactly — unchanged, since this cycle touches no
`data/corpus/**` file (48,708 examined both before and after; 1,839/2,580 fixture rows cleared
both before and after).

**Isolation confirmed by a whole-inventory before/after diff keyed on unit id** (not sampled —
a real Python diff over both full 49,438-unit JSON documents, before = the COMMITTED HEAD
inventory (`git show HEAD:docs/work-inventory.json`), after = this cycle's own local regen
against HEAD's committed source plus this cycle's own edit):

```
before count: 49438 after count: 49438
added: 0 removed: 0
changed: 2
changed by book: {'core_rulebook': 2}
changed by new evidence: {
  'ranger_combat_style_choice_probe_observed_a_real_computed_recognition_for_the_display_record': 2
}
  core_rulebook:class_feature:ranger_combat_style_archery engine-does-not-hold -> grounded
  core_rulebook:class_feature:ranger_combat_style_two_weapon_combat engine-does-not-hold -> grounded
```

Both changes carry this cycle's own new evidence string — zero changes outside this cycle's own
2 targeted ids, and zero changes from any other concurrently-committed lane between this cycle's
checkpoint push and this regen (confirmed: `origin/tranche/14` had not moved past this cycle's
own two checkpoint commits when this regen ran). Every one of the 2 moved
`engine-does-not-hold` (bucket C) → `grounded` (bucket **DONE**) directly, confirmed by direct
post-regen read: both carry `status: "grounded"`, `wiring_class: "computed"` — neither eligible
for the static/derived `V`-reclassification `apply_done_rung_stamps` applies, so neither was
restamped away from DONE.

## Figures + their re-derive commands

| Figure | Value | Command | Denominator |
|---|---:|---|---|
| `core_rulebook` bucket C at cycle start | 201 | `python3 scripts/completion_atlas.py --book core_rulebook --check` against the committed `docs/work-inventory.json` | of 6,701 |
| `core_rulebook` bucket C after this cycle's own fix | **199** | same command, live regen including this cycle's edit | of 6,701 (delta −2) |
| `core_rulebook` bucket DONE after this cycle | **4,615** | same command | of 6,701 (delta +2) |
| `monk_unarmed_damage_no_formula_in_engine` sub-cause population, corrected | 42 → **48** | direct `docs/work-inventory.json` categorization (see Population section) | of 199 (correction, not a bucket move) |
| This cycle's own isolated closures | **2**, both `core_rulebook`, both `Ranger Combat Style ~ *` | whole-inventory diff filtered on this cycle's own evidence string | of 2 (targeted population) |
| Corpus-wide bucket C before/after this cycle's own regen | 4,182 / **4,180** | `python3 scripts/completion_atlas.py --check` | of 49,438 (delta −2) |
| Corpus-wide bucket DONE before/after | 24,724 / **24,726** | same command | of 49,438 (delta +2) |
| `corpus_literal_sweep` (before/after, unchanged) | 48,708 examined, 0 findings | `corpus_literal_sweep --json-out` | of 51,482 read |
| `derived_evaluator_fixture_check` (before/after, unchanged) | 1,839 cleared of 2,580 rows, 0 failed | `derived_evaluator_fixture_check --json-out` | of 2,580 |
| `completion_atlas.py --check` (corpus-wide, post-regen) | `population=49438 unclassified=0 overlap=0` | `python3 scripts/completion_atlas.py --check` | of 49,438 |
| `completion_atlas.py --check` `citation_failures` | 0 (10→0, re-derived this cycle after this cycle's own 186-line insertion shifted 10 pins) | `python3 scripts/completion_atlas.py --check` | of 10 citations |
| `cargo test --locked --bin v06_work_inventory` (full) | `486 passed; 0 failed` | `cargo test --locked --bin v06_work_inventory` | of 486 |
| `cargo test --locked --bin v06_work_inventory class_feature` | `150 passed; 0 failed` | `cargo test --locked --bin v06_work_inventory class_feature` | of 150 |
| `cargo test --locked --no-run` (workspace) | exit 0 | `cargo test --locked --no-run` | — |

## Row-count command output (this cycle's own live artifact, uncommitted per file-ownership rule)

```
$ python3 scripts/completion_atlas.py --book core_rulebook --check
book=core_rulebook population=6701 unclassified=0 overlap=0
  DONE: 4615
  A: 0
  B: 470
  C: 199
  D: 366
  M: 812
  V: 114
  U: 10
  X: 115
  Z: 0
```

Bucket C: **199**, not zero. **Status: partial**, remainder named below (populations sum exactly
to 199). This live command output was produced by the local, uncommitted regen and is NOT
reflected in the currently-committed `docs/work-inventory.json` (restored via `git restore`
before this commit, per the file-ownership rule) — the committed inventory still reads C=201
until the wave's shared regeneration cycle re-runs the pipeline against this cycle's own
committed source.

## Build scope verified

`cargo test --locked --no-run` (workspace) exits **0**, run at commit `f0d724d2c8` — this cycle's
own last commit that can move a figure a test assertion depends on (`decisions.md §12` L7; the
local regen that follows is never committed, so it cannot un-verify this run). Desktop crate
(`apps/desktop/src-tauri`) not tested this cycle: no file under that tree, nor any file it
depends on, was touched by this cycle's own diff (confirmed: `git status --porcelain` before
every commit showed only `src/bin/v06_work_inventory.rs` / `scripts/completion_atlas.py` /
`docs/retro/events/sd34-at-34-e3-002.jsonl` under this cycle's own writes).

## Sweep population

`corpus_literal_sweep`: 48,708 examined, before and after — unchanged, since no
`data/corpus/**` file was added or regenerated this cycle.

## Oracle pin

N/A — no figure in this receipt came from the pinned PCGen oracle corpus.

## Movement, four buckets

- **Closure:** **2** — both `"Ranger Combat Style ~ Archery"` / `"~ Two-Weapon Combat"`, carrying
  `wiring_class: "computed"`, moved `engine-does-not-hold` (bucket C) → `grounded` (bucket
  **DONE**) directly. Nothing remains for these; each is a genuine +0 choice-recognition record
  (choosing a combat style is itself a real, real-cost-free game action) whose own explanation
  the engine already computes — no further magnitude work is owed by these two records
  themselves.
- **Reclassification:** 0 this cycle (no unit moved between two non-DONE buckets).
- **Reachability:** **2** (one new `classify()` rung + one new probe now answer `grounded` for
  these exact corpus keys, reusing one real, already-shipped, already-tested engine explanation —
  no new compute path, no new formula, no engine change).
- **Instrument-correction:** **1** — `monk_unarmed_damage_no_formula_in_engine`'s own stated
  population, corrected 42 → 48 in cycle 6's own remainder table (a count error in a prior
  cycle's own prose, not a bucket-boundary move; logged as a `correction` retro event,
  `--verified-by` a direct categorization command).

**Bucket C's own delta (201 → 199, −2) equals this cycle's own Closure exactly** — the row-count
command's own output above is the ground truth this movement report is checked against, not the
other way around.

## Remainder — 199 of 201, named by mechanism, populations sum exactly

Re-derived fresh at this cycle's own close (`decisions.md §12` L2), by direct categorization of
the live corpus, not by restating cycle 6's differently-shaped table:

| Sub-cause | Population | Status / next step |
|---|---:|---|
| `monk_unarmed_damage_no_formula_in_engine` | **48** (corrected from cycle 6's stated 42 — see Population section) | Genuine engine gap, TWO reasons: the 42 non-Small/Medium band records have no transcribed formula anywhere in the engine (no playable race reaches those 7 sizes at all, per `monk_unarmed_strike_damage_die_for_size`'s own doc comment: `race_resolver::RACE_SIZES` gives the 18 playable races only Medium and Small); the 6 `(Small)` records DO have a real transcribed formula (`small_monk_unarmed_strike_damage_die`), but it is wired ONLY into the Pathfinder Unchained Monk's own compute path (`ground_unchained_monk_unarmed_strike_damage`), deliberately never reused for the Core Rulebook Monk's own Human-only chassis seam — a byte-identical guard test (`tests/sd27_unchained_monk_unarmed_strike_reaches_the_sheet.rs`) protects that boundary. Now the largest remaining named sub-cause. |
| `base_class_standalone_feature_not_computed` | 35 | Unstarted this cycle. Real, distinct base-class mechanics with no shared compute path yet (Rage/Greater Rage/Mighty Rage, Wild Shape, Channel Energy, Divine Bond, Smite Evil, Arcane School, plus several internal chooser/tracker records) — each checked to confirm it is genuinely uncomputed, not merely unwired, by direct corpus read. |
| `prestige_class_standalone_feature_not_computed` | 26 | Unstarted this cycle. Named prestige-class features (Arcane Archer's arrow abilities, Dragon Disciple's draconic features, Eldritch Knight/Loremaster/Mystic Theurge/Pathfinder Chronicler/Shadowdancer features) — no shared compute path exists; each is a genuinely distinct mechanic. |
| `bloodline_power_or_bloodline_feat_not_computed` | 25 | Unchanged from cycle 6 (confirmed by direct read). The residue after cycles 3/4's generic Sorcerer-Bloodline pool-group closure already took the reusable-formula slice; what remains (Elemental Movement/Body, Familiar bonding, bloodline feats, Elemental sub-bloodlines) is each a genuinely distinct mechanic with no shared formula. |
| `class_chassis_internal_tracker` | 16 | **New this cycle, confirmed genuine engine gap, not naming-only** (a NEW finding, not previously named as its own sub-cause): the 16 `"<Class> ~ Class"` records (Adept/Aristocrat/Barbarian/Bard/Cleric/Commoner/Druid/Expert/Fighter/Monk/Paladin/Ranger/Rogue/Sorcerer/Warrior/Wizard) are `completeness: "chassis_only"` internal PCGen `DEFINE`/pool-tracker bookkeeping records (checked directly against the raw corpus: `Barbarian ~ Class`'s own tokens are `DEFINE:Barbarian_CFP_Level\|0` and sibling internal counters, never a player-facing value). No explanation surface exists or should exist for these; out of this territory's wiring-only bar, and arguably not player-facing content at all — a bucket-D/atlas-defect candidate for a future cycle to rule on, not force-closed here. |
| `rage_power_not_computed` | 13 | Unchanged from cycle 6 (re-verified this cycle by direct `pilot_compute` read, not carried forward): `CORE_RULEBOOK_RAGE_POWER_POOL` is a real, already-registered 28-member pool, but only ONE representative power (`Superstition`) has a real magnitude compute — a deliberate, already-shipped "ground one representative option per pool honestly" ruling, the same idiom Battle Mystery/Ward Hex/Life Spirit already follow. Each of the other 27 rage powers (13 in this book's bucket C) is a mechanically distinct effect with no shared formula; closing any one is real per-power engine work, not wiring. |
| `favored_class_bonus_choice_not_wired` | 11 | **New this cycle**: the 11 `FavoredClass`-facet bare-name records (Adept/Aristocrat/Barbarian/Commoner/Expert/Fighter/Monk/Paladin/Rogue/Warrior/Wizard) mark "this class is your favored class." Only ONE class (`explain_fighter_favored_class_bonus_choice`, `class_chassis.fighter.favored_class_bonus_choice`) has a real recognition function today, narrowly gated to a Human Fighter at level 1 — the other 10 classes have no such function at all. Fighter's own 1 unit is a real, closable wiring-only candidate (an already-computed explanation the classifier has never been taught to consult) — named for the next cycle rather than attempted this cycle, to keep this cycle's own diff to one verified mechanism. |
| `prestige_class_chassis_internal_tracker` | 10 | **New this cycle**, same shape as `class_chassis_internal_tracker` above, for the 10 prestige classes' own bare-name chooser records (Arcane Archer, Arcane Trickster, Assassin, Dragon Disciple, Duelist, Eldritch Knight, Loremaster, Mystic Theurge, Pathfinder Chronicler, Shadowdancer) — `completeness: "chassis_only"`, internal `<Class>_CFP_Level` DEFINE/BONUS:VAR tracker tokens feeding sibling `PRE` gates, never a player-facing value of their own. Confirmed by direct corpus read. Out of this territory's wiring-only bar. |
| `rogue_talent_not_computed` | 10 | Unchanged from cycle 6 (re-verified this cycle by direct `pilot_compute` read): the SAME "one representative per pool" idiom as Rage Power — only `Resiliency` has a real magnitude compute, explicitly documented "no talent-effect engine exists in this codebase" at every other numbered slot. Each of the 10 remaining named talents in this bucket is a mechanically distinct effect. |
| `ranger_favored_x_chassis_or_wild_empathy` | 5 (was `ranger_combat_style_or_favored_x_chassis_tracker`: 7; this cycle closed the 2 Combat Style members) | `Basic Favored Enemy`, `Basic Favored Terrain`, `Common Favored Terrain` (internal chooser/pool-definition trackers, same `chassis_only` shape as the class-chassis trackers above — distinct from the ALREADY-CLOSED per-type `"Favored Enemy ~ <type>"` / `"Favored Terrain ~ <type>"` display records cycles 3/AT-34-E3-001 already closed), `Ranger ~ Favored Enemy`, `Ranger ~ Wild Empathy` (each its own distinct un-computed magnitude, not the chooser-tracker shape). Not attempted this cycle. |
| `druid_nature_bond_domain_selection_not_computed` | 7 | Unchanged from cycle 6 (confirmed by direct read). Genuine engine gap: `pilot_compute::mod.rs`'s own Task #64 comment states plainly that Nature Bond's domain option carries NO `DRUID_DOMAIN_CHOICE_ID` seam at all. |
| `domain_power_display_record_not_wired` | 2 | Unchanged from cycle 6 (confirmed by direct read): the last bare header (`"Nobility Domain"`) plus its own zero-token granted-power record. Neither has a live-wired sibling of either reusable shape, and Nobility carries no `domain_power::DOMAIN_POWER_CATALOG` entry. |
| `versatile_performance_not_computed` | 0 | Closed cycle 5; unchanged. |

**Sum check:** 48 + 35 + 26 + 25 + 16 + 13 + 11 + 10 + 10 + 5 + 7 + 2 + 0 = **199**, matching the
row-count command's own remainder exactly (201 − 2 = 199).

## Notes

- **This cycle's fix is deliberately minimal and additive**: one new `classify()` rung, one new
  `EngineFacts` field, one new probe. It reuses ONE already-existing, already-tested explanation
  (`class_chassis.ranger.combat_style_choice`,
  `explain_ranger_level1_chassis_and_class_feature_separation`) — the SAME paired
  display/chassis pattern the Favored Enemy/Favored Terrain and Domain header checks establish:
  **2 real closures from ~60 new lines**.
- **Extensive due diligence before landing on this mechanism, per this dispatch's own instruction
  to confirm prior stated reasons still hold** (this bundle has had one cycle disprove another's).
  Directly re-verified — by reading `pilot_compute/mod.rs` and the raw corpus, not by trusting
  cycle 6's prose — that `monk_unarmed_damage_no_formula_in_engine`, `rage_power_not_computed`,
  and `rogue_talent_not_computed` are genuine engine gaps (confirmed, with one population
  correction), and that the `"<Class> ~ Class"` / bare prestige-class chooser records are internal
  PCGen bookkeeping with no player-facing explanation surface to wire to (a NEW finding this
  cycle, split out into its own two named sub-causes rather than left folded into the larger,
  vaguer `base_class_standalone_feature_not_computed` / `prestige_class_standalone_feature_not_
  computed` cycle 6 inherited). Several promising-looking leads were checked and DECLINED once
  verified — see the Mechanism section above for the `class_chassis.<class>.base_attack_bonus`
  false lead specifically, the most costly one to get wrong (it would have been a genuine
  cross-record-kind misattribution, exactly the "broadened matcher" doctrine forbids).
- **A genuine environment discovery, corrected before it could cause damage:** this cycle's shell
  environment carried a PRE-SET `RETRO_ACTOR=sd31-transcribe` (a prior/sibling lane's own actor
  name, inherited from the process environment rather than set by this cycle) — the dispatch's
  own `export RETRO_ACTOR=...` instruction did not override it because the harness's `export`
  inside one Bash call does not persist to the next. The first `retro.py correction` call
  therefore wrote to `docs/retro/events/sd31-transcribe.jsonl` — a file this dispatch explicitly
  names as another lane's dirty file, never to be touched. Caught immediately via
  `git status --porcelain` before the next git write (the standing discipline this dispatch
  requires); the file was `git restore`d before anything was staged, and the correction was
  re-emitted with an explicit `--actor sd34-at-34-e3-002` flag (bypassing the env-var path
  entirely) to `docs/retro/events/sd34-at-34-e3-002.jsonl` instead. No forbidden file was ever
  staged or committed.
- **Territory respected:** no `CharacterInput` field was added or changed; no trait/ability
  compute path was touched; the EQUIPMENT magnitude sub-causes (owned by a sibling lane) were not
  touched; `pilot_compute::mod.rs` is completely untouched — confirmed by `git status --porcelain`
  before every commit this cycle showing only this territory's own three files.
- **Not attempted this cycle**: every other named sub-cause in the 199-unit remainder table.
  `monk_unarmed_damage_no_formula_in_engine` (48) is now the largest, a genuine engine-formula gap
  (two different reasons), not a naming-only fix like this cycle's.

## Next-cycle plan

1. The Fighter favored-class-bonus-choice unit (1 of the 11 `favored_class_bonus_choice_not_
   wired` records) is the smallest, cleanly-verified, real wiring-only candidate found this
   cycle: extend the classifier to recognize `class_chassis.fighter.favored_class_bonus_choice`
   for the bare `"Fighter"` `FavoredClass`-facet record specifically (the single-token,
   no-`~`-separator shape this territory's other checks explicitly guard against generic
   suffix-matching for, so this needs its own small dedicated rung, not a reused generic one).
2. `class_chassis_internal_tracker` (16) and `prestige_class_chassis_internal_tracker` (10) — 26
   total — are internal PCGen bookkeeping records with no player-facing explanation surface. This
   is worth an explicit operator ruling or an `atlas-defects.md` entry (`decisions.md §2`'s "any
   remaining step the atlas did not predict is a defect in the atlas" rule): are these genuinely
   out of bucket C's scope (never player-facing, so never wireable), or does the doctrine require
   a dedicated "internal, never surfaced by design" bucket disposition? Not decided this cycle.
3. `monk_unarmed_damage_no_formula_in_engine` (48, largest, corrected this cycle) needs real new
   formula work for the 42 non-Small/Medium records (or a deliberate bucket `X` deferral, since no
   playable race reaches those sizes at all) and a real cross-subsystem ADR for the 6 Small
   records (reuse the Unchained Monk formula for the Core Rulebook seam, which the current
   byte-identical guard test explicitly forbids without one) — an operator-scoped question, not a
   wiring-only fix.
4. `base_class_standalone_feature_not_computed` (35) and `prestige_class_standalone_feature_
   not_computed` (26) are both unstarted; each record inside them needs its own per-feature
   verification before any is attempted — this cycle's own due-diligence cost (several hours to
   confirm three large sub-causes and rule out two promising-looking false leads) should be
   budgeted for the next cycle too, not assumed away.
5. Re-derive the remainder partition fresh before picking (`decisions.md §12` L2) — this
   receipt's own table is this cycle's fresh derivation; the NEXT cycle must re-run it fresh
   again rather than trust this one.
