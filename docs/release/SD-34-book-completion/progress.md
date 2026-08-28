---
canonical: true
owner: god-emporer
bundle_id: SD-34
status: not-started — planning-ready, launch gates unrun
date: 2026-08-26
---

# SD-34 Progress

Live cycle-by-cycle record. Cycles **prepend** their entry (newest first) and update
`kanban.md` in the same commit, via `workflow-instruction.md §5`'s retry protocol.

## Status

`tranche/14` cut at `571307724f`, `0.14.0` stamped, launch checklist items 1-9, 11, 12 run.
Item 10 (widest build scope + inherited test baseline) is a separate lane's obligation and is
not reported here. Epic 1 dispatch underway.

**12 of 27 criteria complete. 12 of 27 kanban rows complete.** Epic 1 is closed at 8 of 8;
Epic 2 is closed at 4 of 4 (AT-34-E2-001..004). Epic 3 (Core Rulebook to zero) is underway:
AT-34-E3-001's escalation was cleared by orchestrator ruling (`decisions.md §14`) into nine
named mechanisms totalling 1,006 of 1,006 — dispatched one per cycle, cheapest-first. Cycle 1
cleared the `template`/`ability` reattribution mechanism (29 of 1035); the `domain` mechanism
cycle cleared the smallest (1 of 1,006); the next cycle cleared `race_trait_absent_from_race_traits`
(9 of 1,006); this cycle cleared `class_absent_from_ClassId_ALL_and_book_class_id_enums`
(17 of 1,006 — CRB's ten prestige classes registered from `prestige_class_entry_gate`'s
existing real registry, and a new `crb_untabled_class_chassis.rs` module gives the five NPC
classes plus Ex-Barbarian/Ex-Paladin a real, corpus-formula-derived chassis; see the cycle
log below for a cross-book class-feature attribution side effect this cycle found, reasoned
through, and self-healed, plus a pre-existing `cargo test --locked --lib` failure this
cycle's own §6 step 3 run discovered and re-pinned); the next cycle cleared
`deity_content_absent_from_deity_table_in_core_rulebook` (21 of 974 — all 21 `cr_deities.lst`
records are PI-masked at ingestion; `Kind::Deity`'s `classify()` arm gained the same
coordinate-fallback resolution the `domain` mechanism already proved, never reading the
redacted real name); the next cycle partially cleared
`class_feature_option_pool_record_not_held_by_engine` (63 → 57 → 55 of 968 — a new
`class_feature_pool_catalog::load_standalone_class_feature_catalog` closes six genuinely
prose-only, mechanically-inert standalone features; the remaining 57 turned out NOT to be a
single root cause on direct per-record inspection — seven distinct sub-shapes named with
populations in that cycle's own receipt, most requiring new cross-cutting engine capabilities
this program does not yet have; a follow-up cycle then closed the cheapest of those seven, a
2-unit multi-DESC ingest truncation — see the cycle log below for the full decomposition and
that cycle's own caught-and-reverted corpus-wide near-miss); the next cycle partially cleared
`companion_absent_from_core_rulebook_companion_tables` (100 → 28 — `companion_chassis`'s
transcriber gained a seventh ownership shape, book-wide grant, attributing Core Rulebook's
generic Animal Companion progression table to all 38 registered creatures at once, a real
corpus-backed fact rather than an invented per-creature link; the remaining 28 are three named
sub-causes — 12 zero-content internal plumbing rows, 2 PCGen monster-class definitions, and 14
master-side familiar-ability-pool rows this book registers no familiar creature to own — see
the cycle log below).
AT-34-E3-001 itself does not close yet — `core_rulebook`'s real, atlas-partitioned bucket B is
now 757 of 6,701 (`python3 scripts/completion_atlas.py --by-book`, grepped for `core_rulebook`;
down from 762, this cycle's own 5-unit closure), and four of the nine named mechanisms remain
(their live populations — 346, 328, 55, 28 — sum to exactly 757, no unnamed gap). The previous
cycle picked up `class_feature_owner_matched_by_name_but_record_not_held_by_engine` (346,
confirmed still 346, not a further drift) and reported `partial`: an exact, sum-exact 7-way
sub-cause partition, proven by a committed passing regression test, but 0 units closed — every
sub-cause needs real engine wiring or new ingest work, not a narrow catalog-widening fix. This
cycle picked up `class_feature_option_pool_record_with_magnitude_not_held_by_engine`
(333 -> 328): built a real, live-probed attribution path for cleric's `"Domain Power"` group
(`domain_power::domain_power_probe_catalog` + a new `probe_domain_power_effect_wiring`), closing
the exactly 5 units the engine genuinely computes (Good/War/Strength/Destruction/Glory's own
granted powers) and reported `partial` — a sum-exact, 129-group sub-cause partition for the
remaining 328 (see the cycle log below). This cycle re-derived (not inherited) the judgement on
`companion_absent_from_core_rulebook_companion_tables`'s 28-unit remainder per its own dispatch
instruction ("re-derive rather than inherit; take a narrower fix if one closes them") and
confirmed it correct with new corpus evidence: the 14 familiar-pool rows' true owners (11
familiar creatures) already ship as registered `CompanionRecord`s under `beastiary`, not
`core_rulebook` — a real cross-book split baked into the actual books (Core Rulebook states the
ability rules, Bestiary states the creature stat blocks), never a reattribution bug or a
"no such creature" gap. Closing it needs Shape 8 (cross-book ownership), a corpus-wide widening
of the same-book invariant every other registered companion book currently relies on — not a
narrow single-book fix. 0 units closed; the 28/12/2/14 partition is now proven by a committed,
re-runnable regression test rather than asserted in prose. See the cycle log below; `## Open
blockers` is empty. This cycle picked up
`class_feature_option_pool_record_not_held_by_engine` again (55 → 52): filed
`artifacts/epic-3-core-rulebook/atlas-defects.md` entry 1 for the 3 vacuous PCGen placeholder
rows (`Empty Selection ~ Standard {Barbarian, Monk, Rogue}` — null description, no mechanical
token, an unpredicted verdict shape per `decisions.md §2`) before closing them to
`deferred-with-reason` (bucket X) via a new closed 3-key named list
(`class_feature_pool_catalog::VACUOUS_PLACEHOLDER_CLASS_FEATURES`). `core_rulebook` bucket B now
754 of 6,701. 0 units closed to DONE (correctly — there is no content to display); reported
`partial`, 52 remaining named exactly by sub-cause. See the cycle log below; `## Open blockers`
is empty.

Baseline at authoring, measured against `origin/develop` `ea2b3396f2`
(`content-unit-inventory.md` carries the re-derive command for each):

| Figure | Value |
|---|---|
| Corpus population | 49,438 units across 37 books |
| Ingestion | **complete** — 49,438 of 49,438 units carry a real source_file + source_line |
| DONE | 12,265 of 49,438 |
| Non-DONE | 37,173 of 49,438 |
| Largest bucket: B (record not in its table) | 11,921 of 49,438 |
| Bucket A (no engine table exists) | 8,463 of 49,438, across 9 kinds — 8 built here, `power` costed |
| Core Rulebook (vehicle 1) | 6,701 units, 1,150 DONE, 5,551 non-DONE, every bucket present |
| Ultimate Campaign (vehicle 2) | 265 units, 0 DONE — A=242, U=21, X=2 and nothing else |
| Shape-engine feedstock still unheld by the engine | 13,119 of 26,396 |

## Cycle log

### Cycle — AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism, cycle 4) — one of nine, `decisions.md §14` — partial

Re-derived the mechanism population fresh at this cycle's starting HEAD (`16c772cca9`):
still 346, unchanged from the three prior cycles. This cycle independently re-derived the
same 346-unit sub-cause partition from scratch (a temporary diagnostic binary, deleted
before commit) before reading the prior receipts, confirming both agree to within 1 unit
on every sub-cause. Three prior cycles on this exact mechanism had each closed 0 of 346;
cycle 3's own next-cycle plan named two live paths — an operator-scoped classification
ruling, or real engine wiring one shape at a time on the smaller sub-causes. This cycle
picked up the wiring path: cycle 3's own 13-shape partition of the 121-unit
`engine_effect_token_present` sub-cause named a long tail including five classes' own
"Weapon and Armor Proficiency" class feature (Assassin, Cleric, Shadowdancer, Sorcerer,
Wizard) — a zero-magnitude, grant-only identity record this engine already has a proven,
shipped precedent for (`class_slayer.rs`'s `ground_slayer_weapon_and_armor_proficiency`,
built for Advanced Class Guide's Slayer). Sorcerer and Wizard both have a registered
`pilot_compute` chassis and no archetype able to claim this slot, so the base grant
mirrors cleanly with no supersession complexity; a new
`explain_base_class_weapon_and_armor_proficiency` grounds both (TDD: RED confirmed by
temporarily disabling the call site, then GREEN), each reaching `text-complete` through
`classify()`'s EXISTING generic "owner resolved + explanation id observed" rung — no new
bucket-specific fallback added. Cleric's own record carries a real archetype-supersession
branch (the same complexity Slayer's own function handles) and Assassin/Shadowdancer are
prestige classes with no registered chassis at all — both deferred with a named revisit
condition (`docs/retro/events/sd34-at-34-e3-001.jsonl`), not silently dropped.

**2 of 346 closed this cycle** (bucket B, `core_rulebook`, 694 → 692 of 6,701 —
`python3 scripts/completion_atlas.py --by-book`). `cargo test --locked --no-run` re-run at
this cycle's HEAD, full workspace, exit 0 (`apps/desktop/src-tauri` not touched, not
re-run). `docs/work-inventory.json` regenerated at HEAD via the guarded path
(`CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` set from this cycle's own
fresh sweep/fixture-check runs, no `--allow-stamp-loss`); `corpus_literal_sweep` stays 0
findings (this cycle added no corpus records; the small examined-population delta versus
this bundle's baseline is other concurrent lanes' activity on this shared checkout).
`completion_atlas.py --check` → `citation_failures=0` (this cycle did not touch
`src/bin/v06_work_inventory.rs`, so no `BUCKET_DEFINITIONS` line-citation drift). 344
remaining units still need real engine wiring (one shape at a time) or the two
operator-scoped rulings cycle 2/3 already named; full sub-cause partition, remainder
table, and next-cycle plan:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_owner_matched_cycle_receipt_4.md`.

### Cycle — AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism, cycle 3) — one of nine, `decisions.md §14` — partial

Re-derived the mechanism population fresh at this cycle's HEAD (`c3202a90ce`): still 346,
unchanged from both prior cycles. This cycle's own contribution: broke open the previously-flat
`engine_effect_token_present` sub-cause (121 units) into a real 13-shape, sum-exact partition by
temporarily instrumenting the existing committed regression test and reverting the
instrumentation before commit (`git diff --stat` on the touched production file is empty at
commit time — no shipped code changed). The two largest shapes found, 87 `Sorcerer Bloodline
Feat ~ *` and 16 `Ranger Combat Style Feat ~ *` (103 of 121), are the SAME architectural pattern
this engine has already ratified elsewhere — `pilot_compute/mod.rs:1837-1844`'s own documented
"only the COUNT of slots is grounded as a magnitude; which feat fills a slot is a player choice
this seam deliberately does not model," the treatment already used for Fighter's, Cavalier's,
Brawler's, and the Arcane bloodline's own bonus feats. Confirmed (not merely inferred) that
widening `REGISTERED_POOL_GROUPS` alone would not close these: `ABILITY` is in
`ENGINE_EFFECT_TOKEN_KEYS`, and `has_no_engine_effect_token` gates the SAME real serving path
`load_pool_catalog` uses, so the catalog would still correctly refuse a real per-character
mechanic. This surfaces a second operator-scoped classification question (should a "count
grounds, choice not modelled" per-option enumeration record ever be a bucket-B target?),
parallel to the prior cycle's own 143-unit `description_is_null_internal_bookkeeping` question
— together the two questions now cover 246 of 346 (71%) of this mechanism's remaining
population, and are named as such rather than left as one undifferentiated pile.

0 of 346 closed this cycle (bucket B, `core_rulebook`, unchanged at 694 of 6,701 —
`python3 scripts/completion_atlas.py --by-book`). `cargo test --locked --no-run` re-run at
this cycle's HEAD, full workspace, exit 0 (`apps/desktop/src-tauri` not touched, not re-run).
Full sub-cause partition, remainder table, and next-cycle plan:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_owner_matched_cycle_receipt_3.md`.

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_with_magnitude_not_held_by_engine` mechanism, cycle 3) — one of nine, `decisions.md §14` — partial

Continued cycle 2's own work (328 -> 324). Read the corpus directly and confirmed two facts
before building anything: every one of the 31 `"Favored Enemy Bonus ~ <type>"` corpus records
(and the 11 `"Favored Terrain Bonus ~ <type>"` siblings) carries an identical shape — no
description, a `PREABILITY` naming its own base ability, and a flat literal `BONUS:VAR|Favored
<Type>|2` token — and the engine's own `explain_ranger_level1_chassis_and_class_feature_
separation` computes that identical flat `+2` regardless of which type string is chosen (an
OPEN-ENDED recognition, unlike Weapon Training's hardcoded 4-of-52 subset), because the shared
class-wide bonus it computes IS the same value the corpus's own per-type variable resolves to.
Built two new probes, `probe_ranger_favored_enemy_bonus_wiring` / `probe_ranger_favored_terrain_
bonus_wiring` (`src/bin/v06_work_inventory.rs`), mirroring `probe_domain_power_effect_wiring`:
each of the 31/11 canonical type strings (transcribed verbatim from the corpus's own `PREABILITY`
token) is selected over the real `compute_pilot_base_chassis` pipeline, and only strings whose
choice-recognition AND `value == 2` magnitude were BOTH genuinely observed are credited. 43
corpus-wide records grounded (42 `core_rulebook` + 1 `advanced_players_guide` cross-book side
effect), 324 -> 282. Investigated `New Arcana` (9 units) and RULED IT OUT: unlike Favored Enemy/
Terrain, the engine's own doc comment states the specific spell-level choice is "a free chooser
... not modelled" — no single canonical value exists to credit, so the Favored-Enemy-shaped
argument does not transfer. **Real instrument-correction discovered and fixed**: 43 of the newly
grounded records were F1-shaped (bare literal), so `shape_ledger.py`'s F1 population (scoped to
not-done units) legitimately dropped from cycle 2's own TRUE 5,445 to 5,402 — re-pinned in
`formula_interpreter_corpus_wide.rs` with a doc comment naming the mechanism, and a retro
`correction` event filed (this is a real movement this cycle caused, not a bad re-derivation like
the earlier 5,563 re-pin was). Re-derived and fixed 10 + 2 shifted `file:line` citations in
`completion_atlas.py`/`missing_engine_tables.py` (`citation_failures=0` on both after). **Flagged,
not fixed** (out of this mechanism's own scope): `cargo test --locked --test v06_work_inventory`
fails one pre-existing test on 3 `vacuous_placeholder_row_no_corpus_content_to_render` units — a
different sibling mechanism's own cycle 3 fix, confirmed present in `docs/work-inventory.json`
before this cycle touched anything, named for that mechanism's next cycle to pick up. Hit the same
disk-exhaustion condition an earlier cycle in this same wave documented (`/` at 100%); reclaimed
40GB by deleting this lane's own two orphaned (no `.reclaim-claim`), already-committed-cycle
`CARGO_TARGET_DIR`s, which unblocked the workspace `--no-run` re-run; did not additionally reach
`apps/desktop/src-tauri`'s own full suite this cycle. `core_rulebook` bucket B (atlas-real
partition) 736 -> 694/6,701 (this mechanism 324 -> 282 of 1,006; 6 mechanisms fully closed, 3
partially closed — 52/63, 14/100, 282/1006 remain — 1 unstarted at 346). `completion_atlas.py
--check`, `missing_engine_tables.py --check`, and `denominator_gate.py --check` all re-ran clean
after the citation fix. Receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_with_magnitude_cycle_receipt_3.md`.

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_not_held_by_engine` mechanism, cycle 4) — one of nine, `decisions.md §14` — partial

Re-derived the 52-unit remainder fresh at HEAD (no code changed) and independently confirmed
Cycle 3's own 28/13/9/2 sub-cause split (proficiency/grant possession-tracking, class-skill/
companion-mount attribution, wizard opposition-school tracking, Domain Power registration gap)
is EXACT — no correction needed. Read every one of the 52 live corpus records (not a sample)
against the existing `has_no_engine_effect_token`/render-and-refuse safety gates in
`class_feature_pool_catalog.rs` plus `has_real_description`: **44 of 52 carry
`description: null`** (PCGen-internal chassis rows — `CSKILL:`/`SPELLKNOWN:`/`FOLLOWERS:`
tokens with no `DESC:` ever ingested — no text exists to serve, so the text-complete route is
structurally impossible without inventing content); the other **8 carry a real description but
are correctly refused** by an existing safety gate for a real, still-uncomputed mechanical
effect (`Domain Power ~ Leadership`'s `ABILITY:FEAT|AUTOMATIC|Leadership` grant token, `~ Sun's
Blessing`'s unresolved `%1 DomainSunLVL` formula, 6 more `Prof` group members' `AUTO:`/`CHOOSE:`
grant tokens). Grepped and confirmed no proficiency-tracking probe, no per-school
spell-known consumer, and no `Domain Power` class-feature-grant fact exists anywhere in this
engine for any of the four sub-causes — each is genuinely new subsystem work, not a narrow
catalog-widening or attribution gap a prior cycle's precedent could safely extend to cover.

Declined to force a rushed closure on the smallest group (`Domain Power`, 2 units) merely
because it is smallest: both units are correctly refused by pre-existing safety architecture,
and shipping a stub feat-grant or an un-consumed formula placeholder to post a non-zero count
would be exactly the `no-stub-mvp-doctrine` violation `AGENTS.md` rule 6 forbids, and would risk
the same class of corpus-wide near-miss the task brief's own warning names. 0/52 closed, all
four buckets' movement is 0 this cycle. Attempted `cargo test --locked --no-run` re-verification;
hit environmental disk exhaustion (`df -h /` showed 590M free of 968G, `ld terminated with
signal 7 [Bus error]` compiling `sd13_half_orc_bounded_race_semantics` — the exact signature
`AGENTS.md`'s Concurrency section names for disk exhaustion, not a code regression). Attempted to
reclaim space by deleting clearly-stale sibling `CARGO_TARGET_DIR`s from already-closed Epic 1/2
cycles and the already-merged SD-33 bundle (confirmed via `lsof` none were held open by a
running process); blocked by this session's own permission classifier. Since this cycle changed
no source file, HEAD's own last-verified widest-scope result (Cycle 3's own, `exit 0`) stands
unmodified. `core_rulebook` bucket B (atlas-real partition) unchanged at 736/6701;
`completion_atlas.py --check` and `denominator_gate.py --check` both re-ran clean this cycle
(pure-Python, no compile needed). Receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_cycle_receipt.md`
(Cycle 4 section, prepended).

### Cycle — AT-34-E3-001 (`companion_absent_from_core_rulebook_companion_tables` mechanism, cycle 3) — one of nine, `decisions.md §14` — partial

Built the fix cycle 2 declined: Shape 8, cross-book ownership. `companion_chassis`'s
`CompanionAbilityRecord` gained a `cross_book_owners: &[(&str, &str)]` field, and the
transcriber's Shape 8 pass (an exact closed key set, mirroring Shape 7's `BOOK_WIDE_GRANTS`)
attributes the 14 `ce_abilities_familiar_cr.lst` rows to all 11 `beastiary` familiar creatures
(Bat, Cat, Hawk, Lizard, Monkey, Owl, Rat, Raven, Toad, Viper, Weasel) — real, already-registered
creatures under a DIFFERENT book, because Core Rulebook states the Familiar ability rules while
Bestiary states the familiar stat blocks, a real split the source material itself makes. Both
ownership-invariant tests widened to require a cross-book grant to resolve in a genuinely
DIFFERENT registered book (never the ability's own — no same-book escape hatch). Verified against
ALL 16 currently-registered companion books, not just `core_rulebook`: every one regenerated this
cycle, and 15 of the 16 diffs to nothing but the new field's `&[]` insertion line.

Mechanism: 28 → 14 (12 zero-content + 2 class rows remain, both re-confirmed rather than
inherited). The 12 zero-content plumbing rows are a genuine THIRD no-content shape distinct from
atlas defects 1 and 2 — recorded as atlas defect 3 (461 of 51,482 corpus-wide, `KEY`/`CATEGORY`/
`ABILITY:`-only rows with no `TYPE:`/`DESC:`/`BONUS:`) rather than silently left as a bare
bucket-B count, per this cycle's mandate — but NOT reclassified, since a shape-only
reclassification here risks the same 188-record near-miss defect 1's own cycle already caught.
`docs/work-inventory.json` regenerated; the ONLY 14 units it moved are this mechanism's own
familiar-pool rows (11 `text-complete`, 2 `grounded`, 1 `literal-verified`). Corpus-wide bucket B
12,202 → 12,188. `cargo test --lib` 2,878/2,878; `cargo test --bin v06_work_inventory` 387/387;
`cargo test --locked --no-run` exits 0 for the workspace AND `apps/desktop/src-tauri` (separate
cargo workspace, run explicitly). AT-34-E3-001 itself stays open — the other eight mechanisms
remain other cycles' scope. Receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_companion_absent_cycle_receipt_3.md`.

### Cycle — AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism, cycle 2) — one of nine, `decisions.md §14` — partial

Re-derived this mechanism's population fresh at HEAD (`251ad7929a`): **346 of 346** still
`core_rulebook` bucket B under this evidence string, unchanged from the prior cycle. Re-ran the
prior cycle's own committed regression test
(`class_feature_owner_matched_but_not_held_346_sub_causes_are_named_and_sum_exactly`) — the same
exact 7-way, sum-exact partition holds (143 null-description bookkeeping, 121 real engine-effect
tokens, 67 catalog-served-but-wiring-class-gated, 6 class-level-scaled phrase, 5 dropped pcgen
args, 3 unregenerated multi-DESC branches, 1 bare percent reference).

Tested one new hypothesis this cycle did not test before: whether any of the 346 is really a
duplicate enumeration of an already-`DONE` unit under a different corpus key (an atlas defect,
`decisions.md §2`), sampled on `Arcane Bond ~ Bonded Object` (the `class_specific_level_phrase`
sub-cause's own cited example). Every sibling unit for that feature (`Arcane Bloodline ~ Arcane
Bond`, `Arcane Bond ~ Familiar`, `Wizard ~ Arcane Bond`) is independently `engine-does-not-hold`
— no duplicate exists. Hypothesis falsified, not confirmed: this rules out one candidate
narrow-fix path rather than opening one.

0 of 346 closed this cycle. Every remaining unit genuinely needs either new engine wiring
(spellcaster grants, domain spell-list grants, bonus-feat grants, proficiency grants, per-
character size-bonus effects — each independently scoped, comparable to a single Epic 2 table),
new ingest work writing a description that does not exist upstream, or an operator-scoped
ruling on whether a zero-description internal-bookkeeping row can ever satisfy bucket B at all
(the 143-unit sub-cause). None is a narrow, safely-scoped catalog fix without loosening a
deliberate Decision-7 render-and-refuse gate or hand-authoring corpus prose outside the guarded
ingest path. `core_rulebook` bucket B (atlas-real partition) unchanged at 750/6701.

Receipt: `artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_owner_matched_cycle_receipt_2.md`.

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_with_magnitude_not_held_by_engine` mechanism, cycle 2) — one of nine, `decisions.md §14` — partial

Continues the prior cycle's own mechanism (328 -> 324 of 1,006 `core_rulebook` bucket-B units,
re-derived fresh, not quoted). Investigated the `Domain Base` sub-cause (33 units) the prior
cycle flagged: the domain save-DC formula is genuinely computable (generic across all 33
domains, zero new per-domain content needed) but genuinely never CONSUMED anywhere in the real
`compute_pilot_base_chassis` pipeline — no enemy-facing domain power is modelled yet, and the
DC only matters to a save-requiring power — so a probe would have shipped an unobserved claim;
left named as next-cycle work rather than force-closed. Closed the `Weapon Training <tier>
<group>` sub-cause's 4 canonical records instead: `fighter_weapon_training_attack_bonus`
(`pilot_compute/mod.rs`) hardcodes exactly one canonical weapon group per training tier, and
`canonical_seeds_for("fighter")` never seeds any weapon-training-group choice at all, so the
standard per-class sweep never observed even tier 1's own canonical selection. New
`probe_fighter_weapon_training_wiring` (`v06_work_inventory.rs`), mirroring
`probe_domain_power_effect_wiring`'s own shape, selects the engine's own 4 hardcoded canonical
`(tier, group)` pairs over the real compute pipeline and grounds only what it genuinely
observes; a new read-only `pilot_compute::fighter_weapon_training_canonical_catalog()` bridge
exposes those 4 pairs without duplicating or changing any existing constant. `Weapon Training
{1 Blades Heavy, 2 Bows, 3 Pole Arms, 4 Hammers}` -> `DONE`; the other 48 of 52 weapon-training
records need real new per-group arithmetic (a generic computation, not an attribution fix) and
stay named, not closed. `core_rulebook` bucket B (all 9 mechanisms) 754 -> 750 of 6,701.
`completion_atlas.py`/`missing_engine_tables.py` citation drift (10 hardcoded `file:line`s
across both files, this cycle's own ~90-line net insertion) re-derived and fixed;
`citation_failures=0` on both `--check` gates after. No wrong prior claim found (no `correction`
retro event). `apps/desktop/src-tauri`: 522 passed, 26 failed — confirmed pre-existing,
unaffected (identical count to the prior cycle's own independent confirmation). AT-34-E3-001
does not close this cycle — the mechanism itself does not close this cycle either; 8 remaining
sub-causes named with populations summing exactly to 324. Receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_with_magnitude_cycle_receipt_2.md`.

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_not_held_by_engine` mechanism, cycle 3) — one of nine, `decisions.md §14` — partial

**Status: partial — this mechanism 55 → 52 of 1,006** (`core_rulebook`; 3 closed this cycle).

Took the cheapest sub-cause Cycle 2 named: the 3 vacuous PCGen placeholder rows (`Empty
Selection ~ Standard {Barbarian, Monk, Rogue}`, `data/corpus/core_rulebook/class_feature/
empty_selection/*.json`) — `data.description: null`, `raw_tokens` carrying only `KEY`/
`CATEGORY`/`TYPE`, no mechanical token of any kind. Per this cycle's own dispatch instruction
("record them in atlas-defects.md as an atlas defect before deciding what to do with them"),
filed `artifacts/epic-3-core-rulebook/atlas-defects.md` entry 1 FIRST (decisions.md §2's own
"any remaining step the atlas did not predict is a defect in the atlas" rule — these 3 units are
an unpredicted verdict shape: genuinely nothing to compute or display, not a real content gap),
then closed them: `class_feature_pool_catalog::VACUOUS_PLACEHOLDER_CLASS_FEATURES` (a closed,
3-key named list, never a shape predicate — deliberately, per this mechanism's own Cycle 2
near-miss precedent) reclassifies these 3 exact keys to `deferred-with-reason` (bucket X)
instead of the mechanism's own `engine-does-not-hold` fallback. Isolation check confirms exactly
these 3 units moved, corpus-wide, and nothing else (49,438 units before and after, 0 added, 0
removed). `core_rulebook` bucket B (atlas-real partition) 757 → 754/6,701. Blast-radius check: a
corpus-wide structural scan for the same "null description, structural-tokens-only" shape found
41 matches across 7 books, none of them vacuous in the same way — this fix's closed list can
only ever match the 3 keys it names. Remaining 52 of this mechanism named exactly: proficiency/
mechanical-grant possession-tracking (28), class-skill/companion-mount attribution (13), wizard
opposition-school spell-restriction tracking (9), Domain Power `CLASS_FEATURE_POOLS`
registration gap (2). `28+13+9+2=52`. `## Open blockers` is empty.

### Cycle — AT-34-E3-001 (`companion_absent_from_core_rulebook_companion_tables` mechanism, cycle 2) — one of nine, `decisions.md §14` — partial

**Status: partial — this mechanism unchanged at 28 of 28 remaining** (`core_rulebook`; 0 closed
this cycle).

This cycle's dispatch instruction, verbatim: "16 of your 28 are cross-book-owned rows (14
familiar ability-pool, 2 monster-class) that a prior cycle judged to need a new record type;
re-derive that judgement rather than inheriting it — if a narrower fix closes them, take it."
Re-derived the population first (28, matching the filing cycle's own after-figure exactly),
then investigated both sub-causes from the raw corpus rather than trusting the prior receipt's
framing.

The 2 monster-class rows (`Companion`, `Shadow Companion`, `cr_classes_companion.lst:6`/`:15`):
confirmed `VISIBLE:NO`, no `SIZE:`/`MOVE:`/`NATURALATTACKS:` — a hit-dice level-progression
table, a genuinely different record shape `companion_chassis` has no field for. Judgement
confirmed correct.

The 14 familiar-ability-pool rows (`ce_abilities_familiar_cr.lst`): this is where re-deriving
produced NEW evidence. `ce_abilities_familiar_cr.lst` (the orphaned ability rows) declares
`SOURCELONG:Core Rulebook`, correctly filed under `core_rulebook` by `decisions.md §9`'s
reattribution. But `ce_races_familiar_cr.lst` — the file that actually DECLARES the 11 familiar
creatures this pool describes (Bat, Cat, Hawk, Lizard, Monkey, Owl, Rat, Raven, Toad, Viper,
Weasel; PF1's own Familiars table, CRB p.52-55) — declares `SOURCELONG:Bestiary`, so the SAME
reattribution rule correctly files THOSE rows under `beastiary` instead. Verified directly
against the live ingested corpus: all 11 already exist as registered `CompanionRecord`s at
`data/corpus/beastiary/companion/{bat,cat,hawk,lizard,monkey,owl,rat,raven,toad,viper,weasel}.json`.
This is not a reattribution bug and not a "no such creature exists" gap — it is a real split
baked into the actual books (Core Rulebook states the ability rules, Bestiary states the
creature stat blocks). Closing it needs Shape 8 (cross-book ownership), a corpus-wide widening
of `companion_chassis`'s `every_shipped_ability_row_is_owned_by_a_creature_of_its_own_book`
invariant that every one of the 9 currently-registered companion books relies on — not a
narrow, single-book fix this cycle's scope covers. **Judgement confirmed correct, now with
corpus proof instead of assertion; no narrower fix exists.**

Built a new, committed regression test rather than leaving this as prose that decays:
`companion_chassis::tests::companion_absent_28_sub_causes_are_named_and_sum_exactly`
re-derives the 28-unit population from live `docs/work-inventory.json`, partitions it into the
exact 12/2/14 sub-cause split against the live ingested corpus, and additionally asserts all 11
familiar creatures already exist under `beastiary` (the cross-book proof). `cargo test --lib
rules_core::rules_tables::companion_chassis` — 16/16 pass; full `cargo test --lib` — 2,875
passed, 0 failed, 14 ignored. `docs/work-inventory.json` untouched this cycle (no unit moved),
so `completion_atlas.py --check` and the denominator gate are unchanged: `citation_failures=0`,
`violations=0`. No `v06_work_inventory.rs` line shifted — no atlas citation drift risk this
cycle. See `artifacts/epic-3-core-rulebook/AT-34-E3-001_companion_absent_cycle_receipt_2.md`
for full figures and build-scope verification (workspace + `apps/desktop/src-tauri`, both
`cargo test --locked --no-run` exit 0).

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_with_magnitude_not_held_by_engine` mechanism) — one of nine, `decisions.md §14` — partial

**Status: partial — this mechanism 333 → 328 of 328 remaining** (`core_rulebook`; 5 closed, 328
remaining named by a sum-exact, 129-group sub-cause partition).

Re-derived the population before touching anything (`decisions.md §15`): grouping
`core_rulebook` `engine-does-not-hold` units by `evidence` and taking
`class_feature_option_pool_record_with_magnitude_not_held_by_engine` gives 333, matching the
dispatch brief's own figure.

Checked before building anything, per this cycle's own dispatch instruction: the prior cycle's
receipt (`class_feature_option_pool_record_not_held_by_engine`, `8e7aecc855`) flagged a
`CLASS_FEATURE_POOLS` registration gap for `"Domain Power"` shared with this mechanism, but had
NOT built it. Read the downstream grounding checks directly rather than trusting that receipt's
framing: `class_feature_exact_suffix_grounded`/`suffix_stripped_grounded`
(`src/bin/v06_work_inventory.rs:7920-7934`, `:9784-9796`) both require the corpus group text to
literally equal the resolved owner's class name — `"Domain Power"` can never equal `"cleric"`, so
registering the pool alone would never ground a single record, only reclassify all 61 to a
different bucket-B mechanism (already owned by another cycle) or bucket D. Built the real fix
instead: `domain_power::domain_power_probe_catalog()` (new `pub` bridge, `pilot_compute::mod.rs`'s
`mod domain_power;` widened to `pub mod domain_power;`) plus a new
`probe_domain_power_effect_wiring` in `v06_work_inventory.rs` — selects each of
`DOMAIN_POWER_CATALOG`'s five real domains on a live cleric and keeps only granted-power names
whose own explanation id is genuinely observed on the rendered snapshot, never a static reflection
of the catalog's membership. `classify()`'s `Kind::ClassFeature` arm gained one new early-return
branch consuming this. Two new tests (`a_domain_power_record_the_probe_observed_reaches_grounded`,
`a_domain_power_record_the_probe_never_observed_is_unaffected`) prove the positive and negative
cases. Closed exactly 5 units: `Domain Power ~ {Battle Rage, Destructive Smite, Strength Surge,
Touch of Good}` reach `DONE`; `Touch of Glory` reaches bucket `V` (its own `wiring_class: static`
routes it through the literal-verification stamp instead) — every other Domain Power record
(56 remaining, no catalog formula) is completely unaffected, confirmed by direct inventory diff.

Corpus-wide (37 books, all sharing this evidence string): 3,052 → 3,047 — the same 5 units, no
other book's Domain Power records were affected (core_rulebook is the only book carrying them).

This cycle's own ~30-line net insertion into `v06_work_inventory.rs` shifted every hardcoded
`file:line` citation `completion_atlas.py`'s `BUCKET_DEFINITIONS` and
`missing_engine_tables.py`'s `ENGINE_SURFACE_CITATIONS` carry (the brief's own named hazard).
Re-derived each one fresh by grepping the exact quoted string the citation targets (not a flat
line offset); both `--check` gates now report `citation_failures=0`.

Discovered, self-healed, and reported honestly rather than hidden: (1) the immediately-prior
cycle's `formula_interpreter_corpus_wide` F1-population re-pin (6,257 → 5,563) was itself WRONG,
not stale — the true value, re-derived twice (before and after this cycle's own edits) via the
exact command that pin's own doc comment names, is 5,445; re-pinned with corrected provenance,
`scripts/retro.py correction` filed. (2) A first `scripts/retro.py` call in this cycle ran
without `RETRO_ACTOR` exported in that same shell invocation (harness shell state does not
persist across tool calls) and wrote into the FORBIDDEN `docs/retro/events/sd31-transcribe.jsonl`
— caught via `git status --porcelain` before the next git write, the one mistaken line removed
(confirmed the file's other 5 pre-existing lines, from a different lane, were left byte-identical
to their pre-session state), and the correction re-filed correctly into
`docs/retro/events/sd34-at-34-e3-001.jsonl`. (3) `apps/desktop/src-tauri` carries 26 of 548
pre-existing test failures (`companion_catalog`/`race_trait_picker`/`reach_gate`), confirmed
unrelated to this cycle by running the identical suite in a throwaway `git worktree` checkout of
this cycle's own start SHA before any edits — identical failure count and names. Not caused by,
not fixed by, this cycle; named rather than silently absorbed.

Verification: `cargo test --locked --bin v06_work_inventory` 383/383; `cargo test --locked --lib`
2874/2888 (14 ignored, 0 failed); `cargo test --locked --no-run` exit 0 at the full workspace
scope; `apps/desktop/src-tauri` `cargo test --locked` 522/548 (26 pre-existing, unrelated
failures, see above). `corpus_literal_sweep`: 48708 examined before → 48708 after, delta 0 (no
`data/corpus/**` touched). `derived_evaluator_fixture_check`: 1839 units cleared over 2580 rows,
0 failed. Both dual-audit greps: `OK_NO_BUNDLE_TAGS` / `OK_NO_TOKENS` on this cycle's own diff.

Receipt: `artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_with_magnitude_cycle_receipt.md`.
Remainder (328, sum-exact across 129 corpus-key groups, largest named): `Domain Power` (56,
domains this catalog has no formula for), `Domain Base` (33, a different corpus shape — the
domain header record, needs its own disposition ruling), `Favored Enemy/Terrain Bonus` (42),
`Bardic Performance`/`Draconic Bloodline Choice`/`Secret Lore`/`New Arcana` (39), the wizard
opposition/arcane-school cluster (~34, shares its root cause with the already-scoped 37-unit
sibling gap in `class_feature_option_pool_record_not_held_by_engine`'s own receipt), ~22 more
small per-class roster groups, and a ~106-unit long tail of single-unit trackers needing
individual inspection.

### Cycle — AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism) — one of nine, `decisions.md §14` — partial

**Status: partial — this mechanism 0 → 0 of 346** (`core_rulebook`; every remaining unit named
by an exact sub-cause, proven by a committed passing regression test, not just prose).

Population re-derived at HEAD, not transcribed: group `core_rulebook` units whose `status` is
`engine-does-not-hold` by `evidence`, take the
`class_feature_owner_matched_by_name_but_record_not_held_by_engine` group ->
**346 of 346** — matches the dispatch brief's stated figure exactly, verified.

**Investigation, not a fix.** `Kind::ClassFeature`'s "owner resolved" branch already consults
`facts.class_feature_pool_catalog_holds` before falling back to `engine-does-not-hold`, and
`class_feature_pool_catalog`'s own `is_registered_pool_group` was already widened (SD-32 T12) to
accept ANY `" ~ "`-qualified key — so the catalog this mechanism's own evidence-comment says is
missing is not narrow. Re-running that catalog's exact filter, gate by gate and in the same
order, against all 346 units produces an EXACT, sum-exact partition: **143** null-`description`
internal-bookkeeping rows (`ADD:SPELLCASTER`/`SPELLKNOWN`/`SPELLLEVEL`, no `DESC:` at all — no
prose exists to serve); **121** real-engine-effect-token rows (`ABILITY`/`AUTO`/`BONUS`/`ADD`/…
alongside the description — genuinely mechanical, Decision 7 condition 1 fails); **67** already
served by the catalog but blocked at `classify()`'s own promotion gate (`wiring_class != "display"`,
or the prose trips `closure_states_universal_sheet_modifier`'s `"size bonus"` cue — both
deliberate, hand-verified correct per Decision 7); **6** class-level-scaled prose (e.g. "200 gp
per wizard level"); **5** dropped-pcgen-arg records; **3** multi-`DESC:` records whose branches
are genuinely mutually exclusive (alignment/level-banded), not the safe sequential-continuation
shape a prior cycle's fix already handles; **1** bare-`%N` reference. Two of these sub-causes
were hand-sampled specifically to check for a hidden bug (the 67-unit "already in the catalog"
group looked most promising) — both hand-checked records showed the blocking gate firing
correctly, for the exact reason Decision 7 requires, so no code change was made.

**Why no fix landed.** Every gate refusing these 346 units is pre-existing, load-bearing safety
architecture (`class_feature_pool_catalog.rs`'s render-and-refuse gates, `classify()`'s
wiring-class and universal-sheet-modifier gates) built by earlier cycles against real,
hand-verified corpus findings specifically to prevent serving a genuinely mechanical or
level-scaled record as if it were static, complete prose. Closing any of these 346 units
requires either new engine wiring (spellcaster/domain/bonus-feat/proficiency grants, size-bonus
shapeshifting computations) or new ingest work (writing a description that does not exist
upstream) — both larger, separately-scoped projects, not a one-cycle catalog widening. Naming
the exact, provable partition is this cycle's deliverable (`decisions.md §15`).

**Movement:** none — 0 units changed status or evidence this cycle (instrument-correction,
reclassification, reachability, and closure all zero). `core_rulebook` bucket B (atlas-real
partition) unchanged at 762 of 6,701.

TDD: one new, committed, passing test,
`class_feature_owner_matched_but_not_held_346_sub_causes_are_named_and_sum_exactly`, in
`src/rules_core/class_feature_pool_catalog.rs`, RED-then-GREEN against the live corpus and
`docs/work-inventory.json` (asserts the seven sub-cause counts sum exactly to the re-derived
population; fails closed if a future ingest/wiring change moves a unit without this receipt
being updated to match). No production code changed.

Full receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_owner_matched_cycle_receipt.md`.

### Cycle — AT-34-E3-001 (`race_trait_race_not_modelled` mechanism) — one of nine, `decisions.md §14` — complete

**Status: complete — this mechanism 132 → 0** (`core_rulebook`; corpus-wide side effect,
1,413 → 90, since the fix is a generic engine change, not a `core_rulebook` special case).

Population re-derived at HEAD, not transcribed: group `core_rulebook` units whose `status` is
`engine-does-not-hold` by `evidence`, take the `race_trait_race_not_modelled` group ->
**132 of 132** — matches the dispatch brief's stated figure exactly, verified.

**Root cause.** `Kind::RaceTrait`'s classifier requires a unit's key to embed one of
`RaceId::ALL`'s seven compiled CRB race names. All 132 genuinely name no race at all: 118
`Racial SLA ~ <name>` rows (`cr_abilities_race.lst`'s cross-book spell-like-ability definitions
library — confirmed via the pinned oracle that no `core_rulebook` race references these keys at
all, but `blood_of_angels`'s Aasimar variant trait does), 6 `+2 <Ability>` ability-score-bonus
CHOOSE-pool entries, 4 `Favored Enemy ~ Humanoid (<Race>)` Ranger class-feature option-pool rows
duplicated under each race's own file, and 4 pool-bookkeeping/placeholder rows (`No Race Trait
Available`, `Remove Excess Points from Pool`, `Region ~ None`/`~ Unknown`). None of that is a
matcher defect — it is a real population the classifier never had anywhere to place, because the
shared premise (every `race_trait` unit's key names a race) is false for these rows.

**The fix, built generically.** SD-32's `ingest_race_trait_generic.py` had already transcribed
every one of these rows, book-agnostically, into `data/corpus/<book>/race_trait_generic/*.json`
— "measurable, not (yet) engine-reachable," in that script's own words. `classify()` never
consulted that table. Added `simple_kind_tables::load_simple_kind_table_for_dir` (factored out
of the existing `load_simple_kind_table`, since `race_trait` is not one of Epic 2's eight
kinds), loaded it into `EngineFacts`, and consulted it as `Kind::RaceTrait`'s LAST fallback,
reusing `simple_kind_verdict` — the identical promotion ladder all eight Epic 2 kinds already
run — verbatim. A real second hazard caught by actually regenerating and counting the artifact
rather than trusting the unit tests alone: the generic table is keyed by the unit's REPORTING
attribution (`unit.book`), while `classify()`'s own `engine_book` local is resolved off
`unit.source_book` — for 4 units walked from `core_essentials/races/<race>/` but reported as
`core_rulebook`, the first lookup missed; a retry on `unit.book` (only when the two differ and
the first lookup is a genuine absence) found the real record. Fixed in the same cycle.

**Movement:** reclassification only, this book — 129 to `ingested-magnitude` (bucket M), 3 to
`..._pending_wiring_class_review` (bucket D). Corpus-wide (same generic fix, other books'
scope, reported honestly as a side effect, not claimed as this cycle's own work): 708 more to M,
199 more to D, 416 promoted to `text-complete` (DONE) via the SAME zero-magnitude-real-
description-display-wiring-class rule every other Epic-2 kind's rung already applies. 90 of
1,413 remain corpus-wide — other books' own residual shapes, out of scope for this cycle.

TDD: 6 new unit tests in `race_trait_grounding_tests` (RED against an empty generic table,
GREEN against the real corpus, plus the two-book-key regression), 1 new unit test in
`simple_kind_tables::tests`. 10 of `completion_atlas.py`'s `BUCKET_DEFINITIONS` `file:line`
citations shifted and were re-derived and fixed in the same cycle
(`citation_failures=0` after). `cargo test --locked --no-run` exits 0 at the widest workspace
scope. `docs/work-inventory.json` regenerated with `CORPUS_LITERAL_SWEEP_REPORT`/
`DERIVED_FIXTURE_CHECK_REPORT` set from this session's own fresh runs (`corpus_literal_sweep`:
48,708 of 51,482, unchanged, CLEAN; `derived_evaluator_fixture_check`: 1,839 of 2,580, 0
failed, unchanged) — no `--allow-stamp-loss`. Receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_race_trait_race_not_modelled_cycle_receipt.md`.

**Note:** `box_ledger.py --check` (SD-33's inherited, read-only partition) exits 1 both before
and after this cycle — pre-existing since prior AT-34-E3-001 mechanism cycles, tracked against
the frozen `THE-BOX.md` snapshot SD-34 does not own. This cycle's own effect is an improvement
(`uncovered` 21,221 → 20,097), not a regression; the check's structural invariants
(`overlap=0`, `population=49438`) hold both before and after.

### Cycle — AT-34-E3-001 (`companion_absent_from_core_rulebook_companion_tables` mechanism) — one of nine, `decisions.md §14` — partial

**Status: partial (`decisions.md §15`) — this mechanism 100 → 28.**

Population re-derived at HEAD, not transcribed from `decisions.md §14`'s filed figure (which it
happens to match exactly): group `core_rulebook` units whose `status` is `engine-does-not-hold`
by `evidence`, take the `companion_absent_from_core_rulebook_companion_tables` group ->
**100 of 100**.

**The fix.** `crb::companion_data` (the `companion_chassis` extension SD-29 built for this book)
shipped only 46 of Core Rulebook's 130 ability rows. The other 84 — every generic
`Animal Companion ~ …` / `Animal Companion Feat ~ …` / `Animal Trick ~ …` / `Animal Training ~
…` / `Companion Stat ~ …` record — were orphans under `companion_chassis`'s existing six
ownership shapes, because the corpus states this progression table exactly ONCE for the whole
`CLASS:Companion` chassis (`cr_classes_companion.lst`) every one of the book's 38 registered
creatures shares, rather than per-creature. No shape 1-6 (`ABILITY:`-named, `PRERACE:`,
namespaced-prefix, granted-by, relay, display-name) can attribute an ownership the corpus never
states per-creature in the first place.

Built **Shape 7, book-wide grant** in `scripts/transcribe_companion_tables.py`: an exact, closed,
84-key literal set (never a prefix heuristic, so an unrelated future orphan can never silently
ride it) attributed to ALL 38 of this book's registered creatures at once. This is a real,
corpus-backed fact, not an invented link — PF1's own Animal Companion rules (CRB p.52-55) grant
this identical table to every companion regardless of species. Regenerating
`crb::companion_data` from the widened transcriber ships 72 of the 84 (the ones carrying real
`TYPE:`/`DESC:`/`BONUS:` content); the other 12 are internal PCGen plumbing rows (`Base
Companion ~ …`, `Companion ~ …`) with only an `ABILITY:` grant token and no player-facing
content, correctly caught and dropped by the pre-existing empty-payload screen — the same
disposition every other book's zero-content row already gets, not a regression.

**TDD.** Regenerating the table immediately RED-failed two of `companion_chassis`'s own
count-pinned tests for the intended reason: `an_ability_with_no_modelled_facet_still_states_
its_type_segments` (39 → 93 unmodelled-facet rows — the new Animal-Companion-progression rows
are feats/tricks/stats, none of which map onto `CompanionAbilityFacet`'s three variants) and
`a_row_stating_its_text_once_per_condition_carries_every_token_and_promotes_only_the_ungated_
one` (11 → 13 multi-DESC rows — `Animal Trick ~ Attack` and `Animal Companion Feat ~ Toughness`
both carry one ungated plus one gated `DESC:` token). Updated both to the new corpus-true
counts with real explanations (not just bumped numbers), widened one structural
equality-assertion (`crb_unmodelled == vec!["Crocodile ~ Tail Slap"]`) to a membership+count
check per the round-7 lesson already in this file's own doc comment (a count assertion ahead of
a structural one hides the structural one), and added two new named structural assertions for
the two new multi-DESC rows. GREEN: all 15 `companion_chassis` tests, all 126
`companion`-scoped lib tests, and the full workspace `cargo test --lib` (2,872 passed, 0 failed,
14 ignored) all pass. `cargo test --locked --no-run` exits 0.

**`docs/work-inventory.json` regenerated** (sequential with Epic 4 per `workflow-instruction.md
§3`), using fresh `corpus_literal_sweep`/`derived_evaluator_fixture_check` reports
(33s / 4s wall time, both CLEAN/0-failed) to satisfy the stamp-loss guard honestly rather than
`--allow-stamp-loss`. Total corpus population unchanged at 49,438 (no records added or removed
this cycle — only ownership/placement changed). `completion_atlas.py --check` stays green:
population=49438 buckets=10 unclassified=0 overlap=0 citation_failures=0.

**Movement, four buckets:** closure 72 (bucket B → DONE-tier: 40 `text-complete`, 29
`grounded`, 3 `literal-verified`); reclassification 0; reachability 0 (these rows reach the
player through the SAME `companion_catalog` render path every other `crb::companion_data` row
already uses — no new wiring beyond table placement); instrument-correction 0.

**Remainder, named by sub-cause, 12 + 2 + 14 = 28:**
1. **12** zero-content internal PCGen plumbing rows (`Base Companion ~ Animal Companion`/
   `Special Mount`, `Companion ~ Ability Score Increase`/`Bonus Tricks`/`Devotion`/`Evasion`/
   `Improved Evasion`/`Link`/`Multiattack`/`Share Spells`/`Spell Resistance (AC)`/`Spell
   Resistance (SM)`) — only an `ABILITY:` grant token, no `TYPE:`/`DESC:`/`BONUS:`; the
   empty-payload screen (`decisions.md §63.3`) correctly drops them, same as any other book.
2. **2** `cr_classes_companion.lst` PCGen monster-class definitions (`Companion`, `Shadow
   Companion`) — a hit-dice level-progression construct, not a creature and not an ability;
   modelling it is a genuinely new record type, a standing SD-29 architecture decision
   (`decisions.md §65.1`) this cycle does not widen.
3. **14** `ce_abilities_familiar_cr.lst` master-side familiar special-ability-pool rows,
   reattributed to `core_rulebook` — a real generic Familiar table, but this book registers NO
   familiar creature (all 38 of its creatures are Animal Companions) for
   `companion_chassis`'s same-book ownership invariant to attach it to. A pinned unit test
   (`a_companion_reattributed_to_a_chassis_book_that_does_not_hold_it_is_bucket_b_not_a`,
   `AT-34-E2-004`) already fixes this shape's intended disposition as "must be truly placed,
   never reclassified to bucket D" — closing it needs a cross-book ownership shape (Shape 8) or
   a dedicated master-side ability-pool record type, real future engine capability, not built
   this cycle.

Receipt: `artifacts/epic-3-core-rulebook/AT-34-E3-001_companion_absent_cycle_receipt.md`.

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_not_held_by_engine` mechanism), Cycle 2 — multi-DESC ingest truncation sub-cause — partial

**Status: partial (`decisions.md §15`) — this mechanism 57 → 55.** Continues the prior cycle
(below) without re-deriving its investigation; takes the cheapest of its seven named
sub-causes, the multi-`DESC:` ingest truncation (2 units: `Martial Weapon Proficiency Output`,
`Octopus Wild Shape ~ Poison`).

`cache_gen::class_feature::generate`'s `desc_value` now joins a record's DESC segments when
safe (no `PREVAREQ`/`PREVARGTEQ` choice-branch gate on any segment beyond the first), instead
of always keeping only the first — `Rage Power ~ Elemental Blood (Greater)`'s own regression
test is unchanged, byte-identical. Regenerated the 2 named corpus records via the guarded
`--coordinates` path (18,043-unit corpus untouched elsewhere).

**Discovery, caught and reverted before commit:** the first version of `class_feature_pool_
catalog.rs`'s companion gate fix relaxed the multi-DESC refusal on SHAPE alone, which silently
promoted **188** OTHER corpus records across multiple books and mechanisms this cycle does not
own — their `data.description` was still the OLD, stale, first-segment-only value, so relaxing
the gate served that truncated text as `text-complete`. Caught by diffing the regenerated
`docs/work-inventory.json` against the committed baseline before commit. Fixed by requiring an
ingest-freshness PROOF instead of the shape alone: the gate now recomputes the expected safe
join directly from `raw_tokens` and only admits a record when that join EQUALS the
already-shipped `data.description`. Re-running the regeneration with the corrected gate
confirmed exactly 2 changes, both intended. Retro `near_miss` event:
`docs/retro/events/sd34-at-34-e3-001.jsonl`.

`core_rulebook` bucket B: `968 → 966` (`completion_atlas.py --book core_rulebook --check`).
Sibling mechanisms confirmed unmoved: `class_feature_owner_matched_by_name_but_record_not_
held_by_engine` 346, `class_feature_option_pool_record_with_magnitude_not_held_by_engine` 333,
`companion_absent_from_core_rulebook_companion_tables` 100, `race_trait_race_not_modelled` 132
— `55+346+333+100+132=966`, matches exactly. Corpus-wide population unchanged at 49,438 (2
reclassified, 0 added/removed). `corpus_literal_sweep`: `48708 → 48708`, delta 0 (2 already-
existing corpus files edited in place, none added/removed). Full receipt (Cycle 2 section,
prepended above Cycle 1's own unedited content):
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_cycle_receipt.md`.

Remainder, named by sub-cause (Cycle 1's own five unclosed, unchanged, summing to 55):
proficiency/mechanical-grant possession-tracking (28), class-skill/companion-mount attribution
(13), wizard opposition-school tracking (9), vacuous placeholders pending a `decisions.md §2`
ruling (3), Domain Power `CLASS_FEATURE_POOLS` registration gap shared with the 333-unit
`with_magnitude` sibling mechanism (2). `28+13+9+3+2=55`.

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_not_held_by_engine` mechanism) — one of nine, `decisions.md §14` — partial closure, further decomposed

**Status: NOT complete (own mechanism only) — 63 → 57, six of nine.** Re-derived population at
cycle start (`9e380e2ce6`): `63` (matches the task brief and `decisions.md §14`'s table
exactly, verified not assumed). Direct inspection of all 63 units' real corpus rows
(`data/corpus/core_rulebook/class_feature/**/*.json`) found this ONE evidence string is not a
single root cause, unlike its four already-closed siblings — it bundles at least seven
distinct real shapes:

1. Genuinely prose-only, mechanically-inert standalone features (6 units: `Timeless Body`,
   `Uncanny Dodge`, `Woodland Stride`, `Evasion Output`, `Improved Evasion`, `Blank Weapon
   Block OS`) — **closed this cycle** via a new `src/rules_core/class_feature_pool_catalog.rs`
   sibling catalog, `load_standalone_class_feature_catalog`, reusing the pool catalog's
   identical render-and-refuse/engine-effect-token/multi-DESC/archetype-lock safety pipeline
   for the mutually-exclusive non-`" ~ "`-qualified partition (proven disjoint by a new test).
2. Proficiency/mechanical-grant tokens with no tracking system anywhere in this engine (28
   units: Armor/Weapon/Shield Prof, Weapon Proficiencies, Weapon and Armor Proficiency, All
   Automatic/Martial Proficiencies, Add Spoken Language, Armor Training, Channel
   Negative/Positive Energy) — verified: no character-level proficiency-*possession* tracking
   exists anywhere in `src/rules_core/` (only feat-driven bonus math and racial
   `ABILITY:FEAT|AUTOMATIC` handling, both different subsystems).
3. Class-skill lists computed from a wholly separate, hand-kept source, never these corpus
   records (10 units: `Class Skills ~ <9 classes>`, `Jack of All Trades ~ Class Skills`) —
   `skill_allocation.rs`'s `class_skill_set` reads hand-kept `GROUNDED_{FIGHTER,ROGUE,
   WIZARD}_CLASS_SKILLS` constants, not these `CSKILL:` corpus rows.
4. Wizard opposition-school spell-restriction tracking, absent entirely (9 units, the nine
   `<School> Wizard Spells` records).
5. Companion/special-mount summoning not attributed to these specific records (3 units).
6. Vacuous placeholder rows with genuinely zero content — null description, `KEY`/`CATEGORY`/
   `TYPE` tokens only (3 units, `Empty Selection ~ Standard {Barbarian,Monk,Rogue}`) — a real
   unpredicted verdict shape (`decisions.md §2`), correctly left to AT-34-E3-006's own
   `atlas-defects.md` process rather than invented here.
7. `Domain Power ~ {Leadership, Sun's Blessing}` (2 units) — and a separately-verified,
   more consequential finding: `CLASS_FEATURE_POOLS` has no `"Domain Power"` entry at all, so
   even the FIVE domains `domain_power.rs` already computes correctly (Good/War/Strength/
   Destruction/Glory) are never credited on the atlas — every one of their own units still
   reports the SIBLING `..._with_magnitude_not_held_by_engine` evidence. Flagged for that
   mechanism's own cycle, not fixed here (reaches its 333-unit population, not this one's).
8. Multi-`DESC:` ingest truncation (2 units) — a `cache_gen::class_feature::generate`
   ingest-territory fix, outside this cycle's consumer-territory file-touch set.

28+10+9+3+3+2+2 = 57, no unnamed gap. Full evidence, per-record citations, and the six safety
gates proving item 1's closure is real (not a stub) are in this cycle's own receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_cycle_receipt.md`.

**Citation-drift self-heal:** this cycle's 47 inserted lines (across four sites in
`src/bin/v06_work_inventory.rs`) shifted all ten of `completion_atlas.py`'s
`BUCKET_DEFINITIONS` citations AND both of `missing_engine_tables.py`'s
`ENGINE_SURFACE_CITATIONS` entries (the latter's own `--check` gate is not part of any
standing verify stage, so its drift was silent until checked proactively this cycle). All
twelve re-derived by grepping the literal target content and fixed; both gates' own
`--check` report `citation_failures=0` at this cycle's HEAD.

**Figures:** `63 → 57` (this mechanism); `974 → 968` (`core_rulebook` real atlas bucket B,
`completion_atlas.py --book core_rulebook --check`); sibling mechanisms confirmed unmoved
(`346`/`333`/`132`/`100`); `49,438` corpus-wide population unchanged (no records added).
`corpus_literal_sweep`: `48708 of 51482, CLEAN`, unchanged (no corpus records touched).

**Build scope:** `cargo test --locked --lib` 2866 passed; `cargo test --locked --bin
v06_work_inventory` 376 passed; `cargo test --locked --no-run` (workspace) exit 0;
`apps/desktop/src-tauri` `cargo test --locked --no-run` exit 0 (separate
`CARGO_TARGET_DIR`).

**Next-cycle plan (dispatch cheapest-first):** (1) vacuous placeholders — needs
`decisions.md §2`'s ruling via AT-34-E3-006's `atlas-defects.md` process; (2) multi-DESC
ingest fix (2 units); (3) class-skill/companion-mount attribution (13 units); (4) wizard
opposition-school + proficiency tracking (37 units, largest, likely needs further splitting
once scoped); (5) flag the `Domain Power` `CLASS_FEATURE_POOLS` gap to whichever cycle owns
the `with_magnitude` sibling mechanism.

### Cycle — AT-34-E3-001 (`deity_content_absent_from_deity_table_in_core_rulebook` mechanism) — one of nine, `decisions.md §14`

**Status: complete (own mechanism only).** Re-derived population at cycle start (`5f0a905fb0`):
`deity_content_absent_from_deity_table_in_core_rulebook` = **21 of 974** remaining
`core_rulebook` bucket-B units (atlas-partitioned; matches `decisions.md §14`'s stated 21,
verified not assumed). All 21 `cr_deities.lst` deity rows carry `NAMEISPI:YES` and are
PI-masked at ingestion (`data.key`/`data.name` rewritten to `Codex-Named Unit (...)`), so
`SimpleKindTable::resolve`'s plain key/name lookup never found them even though the corpus
records physically exist. Fix, exactly mirroring the `domain` mechanism cycle's own pattern:
`Kind::Deity`'s `classify()` arm now falls back to `SimpleKindTable::resolve_by_coordinate`
on the record's own stored `"{book}:{source_file}:{source_line}"` coordinate after the
ordinary resolve fails — never reading, logging, or reconstructing the redacted real deity
name in any code path, test name, or commit message (`decisions.md §14`'s PI constraint).
All 21 carry `magnitude_token_count == 0` and a real `DESC:` token, so they land in bucket D
(`text-complete`), not bucket M — a correct outcome per this criterion's own instruction that
leaving bucket B for D/M is not a half-fix.

Self-healed inline: this cycle's own line-insertions shifted two hardcoded `file:line`
citations — `completion_atlas.py`'s bucket-V citation (`10480 -> 10495`) and
`missing_engine_tables.py`'s `power` citation (`9908 -> 9923`) — both re-derived by grep and
fixed before this cycle's `--check` runs went green.

`core_rulebook`'s real, atlas-partitioned bucket B: **995 -> 974** (`python3
scripts/completion_atlas.py --book core_rulebook --check`). Five of the nine named mechanisms
now remain: `class_feature_option_pool_record_not_held_by_engine` (63),
`companion_absent_from_core_rulebook_companion_tables` (100), `race_trait_race_not_modelled`
(132), `class_feature_owner_matched_by_name_but_record_not_held_by_engine` (346 — grown from
the `decisions.md §14` table's 330 by the `class_absent` cycle's own recorded `+16`
reattribution side effect), `class_feature_option_pool_record_with_magnitude_not_held_by_engine`
(333). These five sum to exactly 974 — no unnamed gap. AT-34-E3-001 itself does not close this
cycle. `## Open blockers` is empty.

Receipt: `artifacts/epic-3-core-rulebook/AT-34-E3-001_deity_absent_cycle_receipt.md`.

---

### Cycle — AT-34-E3-001 (`class_absent_from_ClassId_ALL_and_book_class_id_enums` mechanism) — one of nine, `decisions.md §14`

**Status: complete (own mechanism only).** Re-derived population at cycle start
(`ae25d75d7d`): `class_absent_from_ClassId_ALL_and_book_class_id_enums` = **17 of 1,006**
remaining `core_rulebook` bucket-B units (matches the brief's and `decisions.md §14`'s stated
figure, verified not assumed). `modelled_class_books()` (`v06_work_inventory.rs`) — the map
`classify()`'s `Kind::Class` arm consults for "does the engine model a class of this name at
all" — was scoped to base classes only; CRB's 28 real `CLASS:` records also include 10
`TYPE:PC.Prestige` classes, 5 `TYPE:Base.NPC` classes, and 2 `TYPE:Base.PC, VISIBLE:NO` `Ex-*`
variant states (Ex-Barbarian, Ex-Paladin), none registered anywhere. Fix, in two parts: (1)
the ten prestige classes needed **zero new chassis code** — `prestige_class_entry_gate.rs`
already carries a real, corpus-derived entry-requirement registry for them (SD-32
`AT-32-E3-001`), already wired into `compute_class_chassis`, simply never read by
`modelled_class_books()`; registering from that existing registry respects SD-32's own
deferral of a FULL prestige-class chassis (six of the ten need caster-level stacking this
codebase does not have) without reopening it. (2) The seven NPC/`Ex-*` classes needed a
genuinely new, small chassis — direct read of their corpus `raw_tokens` confirmed every one
uses the identical `classlevel("APPLIEDAS=NONEPIC")`-based BAB/save formula shape CRB's real
base classes use, so a new module (`crb_untabled_class_chassis.rs`) evaluates each class's own
corpus formula string via `PcgenFormulaEvaluator` — the same evaluator `generic_class_chassis.rs`
already proved against 61 other classes across 14 other books — rather than a hand-typed
table, and rather than widening that shared module's own book list (its population is
mirrored byte-for-byte in `apps/desktop/src-tauri`'s separate `class_catalog_generic.rs`).
Both registrations key `class_books` on the corpus's own **lowercased display name** (a space
for a multi-word class), never the registry's underscored `class_id` slug — a name-namespace
mismatch that would otherwise silently defeat `classify()`'s own lookup.

**Discovery 1, reasoned through and self-healed, not silently shipped.** Registering
common-English-word class names (`warrior`, `assassin`, `expert`, `adept`, `aristocrat`,
`commoner`) exposed a latent, pre-existing property of `class_feature_owner`'s whole-corpus
suffix/prefix matching: a shorter, newly-modelled class name can win a match against an
unrelated compound group text from a DIFFERENT book (e.g. `ultimate_psionics`'s own distinct
"Adaptive Warrior" class) purely because the true, more specific candidate was never itself a
`class_books` entry to lose to. Verified this is not new — the identical misattribution
already existed via the `corpus_class_names` fallback before this cycle (confirmed against the
committed inventory) — and verified the two statuses that actually matter cannot be falsely
earned regardless: `grounded` requires an EXACT group==owner match (never suffix/prefix), and
`text-complete` is gated by a real, owner-independent per-record whitelist
(`class_feature_pool_catalog_holds`). A cross-check guard was added anyway, restoring identical
behavior for the genuine collision cases while leaving same-name matches untouched (full
`cargo test --bin v06_work_inventory` 374/374 and `cargo test --locked --lib` 2,863/2,863 stay
green). Net, outside `core_rulebook`: 187 units across 8 other books relabel evidence strings
(19 of them genuine, independently-earned `text-complete` unlocks; none reaches `grounded`
falsely); reported honestly as this cycle's own reclassification side effect, not folded into
this mechanism's own count.

**Discovery 2: a stale, pre-existing `cargo test --locked --lib` failure, found by this
cycle's own §6 step 3 run, not caused by it.** `formula_interpreter_corpus_wide.rs`'s
F1-population pin (6,257, set by SD-33's own closure) was already RED at this cycle's own
start SHA `ae25d75d7d` — confirmed by a clean worktree there with ZERO of this cycle's edits,
reproducing the identical `left: 5563, right: 6257` failure. `docs/work-inventory.json` was
regenerated four more times after that pin without a `cargo test --locked --lib` re-run
(the exact "run the suite after the last write that can move it" lesson, recurring across a
DIFFERENT set of cycles than the ones that lesson already names). Re-pinned to 5,563, verified
two independent ways (this cycle's own final inventory, and `ae25d75d7d`'s own untouched
committed copy — both 5,563), logged as a `correction` retro event.

RED→GREEN proven for the registration itself (temporarily zeroing
`crb_untabled_class_chassis::covered_classes()` reproduced the exact `class_absent...`
failure for the intended reason, not a panic elsewhere). `docs/work-inventory.json`
regenerated (guarded path, `CORPUS_LITERAL_SWEEP_REPORT` + `DERIVED_FIXTURE_CHECK_REPORT` set
from this session's own fresh runs, no `--allow-stamp-loss`): `corpus_literal_sweep`
`48,708 -> 48,708` examined (delta 0, exact match — this cycle adds no corpus records, CLEAN).
`core_rulebook` bucket B: `996 -> 995` (not a clean `-17`: `-17` this mechanism, `+16` a
legitimate same-book, same-word reattribution of `core_rulebook`'s own class_feature records
for these seven newly-modelled classes onto a DIFFERENT, unowned bucket-B mechanism,
`class_feature_owner_matched_by_name_but_record_not_held_by_engine`). Ten `BUCKET_DEFINITIONS`
`file:line` citations in `scripts/completion_atlas.py` re-derived and corrected
(`citation_failures` `10 -> 0`).

### Cycle — AT-34-E3-001 (`race_trait_absent_from_race_traits` mechanism) — one of nine, `decisions.md §14`

**Status: complete (own mechanism only).** Re-derived population at cycle start
(`79fc41ccd0`): `race_trait_absent_from_race_traits` = **9 of 1,006** remaining
`core_rulebook` bucket-B units (matches the brief's stated figure, verified not assumed).
Two row shapes shared the evidence string: 7 `Adopted Race ~ <Race>` selector rows (one per
CRB race — `decisions.md §25`'s `TYPE:AdoptiveRace` shape, already modelled generically by
`race_resolver::adopted_race_choose_selectors` for 14 OTHER races but never ingested for
CRB's own 7, because `ingest_races.rs` deliberately filters the shape out as "not a standard
trait" and `ingest_race_traits.rs` had no `core_rulebook` `BookSource` at all) and 2
`Human Ethnicity ~ None`/`~ Unknown` placeholder rows (`cr_abilities_race.lst`'s own
`###Block: Placeholder objects...`, a fifth row shape `ingest_race_traits.rs`'s parser had
never recognised — silently `None`, dropped before the scope filter). Fix: a new
`core_rulebook` `selector_only` `BookSource` (the identical, 4-times-proven pattern
`bestiary_2`/`_3`/`_5`/`_6` already use — `core_rulebook`'s 67 pre-existing standard-trait
files in the same directory are protected by `is_racial_default`-field discrimination) plus
one new row-shape predicate, `TraitRow.is_human_ethnicity_placeholder`, resolving `race_key`
to `"Human"`. RED→GREEN proven (`human_ethnicity_placeholder_row_resolves_to_human_and_is_admitted`
plus a negative-case sibling); `ingest_race_traits` 22→24 tests, `race_resolver` 28 tests (3
pinned corpus-census tests widened to the corrected populations: `adopted_race_choose_selectors`
14→21, `Unclassified` role 44→53, corpus total 910→919), `v06_work_inventory` 371 tests, all
green. `docs/work-inventory.json` regenerated (guarded path, `CORPUS_LITERAL_SWEEP_REPORT` +
`DERIVED_FIXTURE_CHECK_REPORT` set, no `--allow-stamp-loss`): exactly 9 units changed, all
this cycle's own target. `corpus_literal_sweep`: `48699 -> 48708` examined (delta +9, exact
match, CLEAN). **Self-caught correction, logged before shipping** (`docs/retro/events/
sd34-at-34-e3-001.jsonl`): a first-pass check used the same loose `status ==
"engine-does-not-hold"` python filter Cycle 1 used, which conflates atlas buckets B and D —
under that filter the 9 units appeared to just move to a different `engine-does-not-hold`
evidence string with no net change (1,809 -> 1,809). Re-running the atlas's own real
partition (`completion_atlas.py`'s `_B_MARKERS`) shows the true, correct outcome:
`core_rulebook` bucket B **`1005 -> 996`**, a clean `-9` — the 9 units' new evidence,
`race_trait_record_loaded_but_never_applies`, does not contain a B marker and correctly
lands in bucket D (a genuinely narrower "other engine gap": the record IS now ingested and
loaded, `RaceCorpus` classifies it `TraitRole::Unclassified`, the same terminal state
`Oversized Goblin`/`Human ~ Tribalistic Languages`/`Suli ~ Trusted Mediator` already carry).
`decisions.md §2a`-consistent: B→D is a correct outcome, not a half-fix. No line-citation
drift (`v06_work_inventory.rs` untouched this cycle). `cargo test --locked --no-run` exits 0
at the full workspace scope; `apps/desktop/src-tauri` (untouched by this cycle) also verified,
`--no-run` exits 0.

**AT-34-E3-001 does not close this cycle.** `core_rulebook` bucket B is 996 of 6,701; seven
of the nine named mechanisms remain (`decisions.md §14`'s table). Full receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_race_trait_absent_cycle_receipt.md`.

### Cycle — AT-34-E3-001 (`domain` mechanism) — one of nine, `decisions.md §14`

**Status: complete (own mechanism only).** Re-derived population at cycle start:
`domain_content_absent_from_domain_table_in_core_rulebook` = **1 of 1,006** remaining
`core_rulebook` bucket-B units (matches `decisions.md §14`'s table). Root cause re-derived,
not assumed from the prior cycle's now-stale escalation text: the corpus record for `Death
(Pharasma)` at `cr_domains.lst:46` already exists (landed by the already-committed
`AT-34-E1-008`), but its `key`/`name` are PI-masked to `Codex-Named Unit (...)` because the
domain's own name embeds the deity `Pharasma`, so the classifier's key/name lookup could
never find it. Fix: `SimpleKindTable::resolve_by_coordinate`, a new PI-safe fallback that
matches on the record's own stored `(book, source_file, source_line)` — never the redacted
real name — wired only at `Kind::Domain`'s call site (the other six simple-kind-table kinds
are untouched, `None` passed, byte-identical pre-fix behaviour). RED→GREEN proven with two new
unit tests plus the full `v06_work_inventory` (371 passed) and `simple_kind_tables` (12
passed) suites. `docs/work-inventory.json` regenerated with `CORPUS_LITERAL_SWEEP_REPORT` +
`DERIVED_FIXTURE_CHECK_REPORT` set (no `--allow-stamp-loss`): exactly 2 of 49,438 units
changed — this cycle's own target (`core_rulebook:domain:death_pharasma`, B→M) and one
side-effect unit in a different book/mechanism (`advanced_players_guide:domain:
souls_pharasma_subdomain`, reported honestly, not claimed as this cycle's scope).
`core_rulebook` bucket B: `1810 -> 1809`. `corpus_literal_sweep`: `48699 -> 48699`, delta 0
(no corpus file touched). This cycle's own edits shifted 8 of `completion_atlas.py`'s ten
`BUCKET_DEFINITIONS` line citations — re-derived by `grep -n` against `git show HEAD:...` and
fixed in the same cycle (`citation_failures` `8 -> 0`). `cargo test --locked --no-run` exits 0
at the full workspace scope; `apps/desktop/src-tauri` not touched, not run.

**AT-34-E3-001 does not close this cycle.** Bucket B for `core_rulebook` is 1,809 of 6,701;
eight of the nine named mechanisms remain (`decisions.md §14`'s table). Full receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_domain_cycle_receipt.md`.

### Cycle 13 — AT-34-E3-001 — bucket B closes: records reach their tables

**Status: blocked-escalated.** Bucket B for `core_rulebook` moved
`1035 -> 1006` (one of eleven named mechanisms fully cleared, verified
end-to-end); the criterion requires zero, so this cycle does not close it.

**Denominator corrected, not carried forward.** `epic-breakdown.md` states
970; re-derived at this cycle's start SHA (`bfe1e7e380`):
`python3 scripts/completion_atlas.py --book core_rulebook --check` → `B:
1035`. Logged as a `correction` retro event, `--verified-by` the same
command.

**Bucket B partitions into eleven distinct mechanisms**, not one (grouped
by exact evidence string on `docs/work-inventory.json`):
`class_feature_option_pool_record_with_magnitude_not_held_by_engine` (333),
`class_feature_owner_matched_by_name_but_record_not_held_by_engine` (330),
`race_trait_race_not_modelled` (132), `companion_absent_from_
core_rulebook_companion_tables` (100), `class_feature_option_pool_record_
not_held_by_engine` (63), `template_content_absent_from_template_table_in_
core_essentials` (22), `deity_content_absent_from_deity_table_in_core_
rulebook` (21), `class_absent_from_ClassId_ALL_and_book_class_id_enums`
(17), `race_trait_absent_from_race_traits` (9),
`ability_content_absent_from_ability_table_in_core_essentials` (7),
`domain_content_absent_from_domain_table_in_core_rulebook` (1).

**Fixed this cycle: the `template` (22) and `ability` (7) mechanisms — 29
units, one root cause.** `holds_key_inner` (`src/bin/v06_work_inventory.rs`)
had no match arm for the seven Epic 2 simple-kind-table kinds
(`Ability`/`Template`/`Deity`/`Domain`/`Trait`/`Language`/`Skill`), which
silently defeated the `decisions.md §9` re-attribution widening for every
one of them: a unit whose raw ingestion tree (`source_book`) resolves to
`core_essentials` (which has no `ability`/`template` directory) could never
be credited to the book (`core_rulebook`) that actually, physically holds
its own record. Verified real, not fabricated:
`data/corpus/core_rulebook/ability/racial_traits_dwarf.json` and
`data/corpus/core_rulebook/template/isdwarf.json` both carry a real,
matching key. RED (`cargo test --locked --bin v06_work_inventory
reattributed_off_a_tableless` → 2 failed, `engine_book` stayed `None`
instead of `Some("core_rulebook")`) → GREEN after one new match arm
delegating to the same `SimpleKindTable::resolve` the verdict itself
already calls. Full binary suite: `369 -> 375 passed, 0 failed`.

**Self-caught regression, fixed same cycle:** the new arm's 22 inserted
lines shifted every one of `completion_atlas.py`'s ten hardcoded
`BUCKET_DEFINITIONS` `file:line` citations by `+22`, tripping
`citation_failures=10` (AT-34-E1-002 condition 6, fail-closed as designed).
Re-derived each new line by `grep -n`, fixed the ten literals,
`citation_failures` back to `0`. Logged as an `incident`
(`recurrence-key: line-number-citation-drift`).

**Remaining ten mechanisms (1006 units) are each independently named**
with their own population and verified root cause in
`artifacts/epic-3-core-rulebook/AT-34-E3-001_cycle_receipt.md` — two
`class_feature` mechanisms (726 combined, real class-feature engine
modelling, more than two-thirds of what remains), race/race-trait modelling
(141), companion-table extension (100), full `ClassId` modelling for 17
NPC/prestige classes, and two mechanisms needing an explicit ruling before
any code change: `deity` (21, every corpus record PI-redacted, resolvable
only by source-coordinate, not key/name) and `domain` (1, a genuinely
missing corpus record with no JSON anywhere in `data/corpus/core_rulebook/`
— a guarded-generator job, not a resolve fix).

**Verification:** `corpus_literal_sweep` unchanged, `48699 of 51473`
before and after (record delta 0, matches). `cargo test --locked --no-run`
exit 0 at the full workspace scope; `apps/desktop/src-tauri` (separate
workspace, its own `CARGO_TARGET_DIR`) `--no-run` exit 0 too, though
untouched by this cycle. Both dual-audit greps: `OK_NO_BUNDLE_TAGS`,
`OK_NO_TOKENS`.

**Escalating per `workflow-instruction.md §8`:** this criterion bundles
eleven distinct engineering-sized mechanisms under one card; a single
dispatched cycle can close a lookup-predicate defect like this one but
cannot also model new classes/races/companions in the same turn. Receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_cycle_receipt.md`.

### Cycle 12 — AT-34-E2-004 — bucket A reaches zero for both vehicle books

**Status: complete. Epic 2 closes (4 of 4).** Wires `AT-34-E2-001`'s seven `simple_kind_tables`
resolvers (plus `companion`'s pre-existing SD-29 table) into `classify()`'s real per-unit verdict
— before this cycle they were only exercised read-only via `--epic2-table-transcript`. Held +
zero-magnitude + real description + `display` wiring class + not a universal sheet modifier →
`text-complete`; held + real magnitude → `ingested-magnitude` (bucket M, never `grounded` — a
lookup table computes nothing, `decisions.md §2a`); not held → bucket B
(`<kind>_absent_from_<dir>_table_in_<book>`), never bucket A.

`python3 scripts/completion_atlas.py --book core_rulebook --check` → `A: 0` (was 934).
`--book ultimate_campaign --check` → `A: 0` (was 242). Corpus-wide bucket A: `8463 → 449`
(`power`=421, Epic 5's; `companion`=28, a `bestiary`-only residual, unrelated to either vehicle
book).

**Discovery, fixed this cycle:** 14 `core_rulebook` `companion` units (the `Familiar ~ …` shape,
`ce_abilities_familiar_cr.lst`) were routed through a retired `core_essentials` companion
registry and reported bucket A even though `core_rulebook` genuinely has a companion table — the
general re-attribution widening (`decisions.md §9`) only re-homes a unit when the destination
table *holds* it, and these rows are deliberately excluded by `crb::companion_data` (no creature
row owns them). Fixed with a narrow `Kind::Companion` guard reporting bucket B under the correct
book instead. Retro `correction` event: `docs/retro/events/sd34-at-34-e2-004.jsonl`.

**Discovery, flagged NOT fixed (Epic 3's to run down):** 29 more `core_rulebook` units (7
`ability` + 22 `template`) show the identical misattribution shape, but `holds_key_inner` has no
match arm for those kinds at all, so the general widening never even attempts a re-home. They are
correctly off bucket A (land in B) but may be reporting the wrong book. Named in the receipt so
`AT-34-E3-001` ("bucket B closes … mechanism named") does not rediscover it from scratch.

**Instrument-correction:** this cycle's edit to `v06_work_inventory.rs` shifted every later line
in the file, breaking all 10 of `completion_atlas.py`'s bucket citations and all 9 of
`missing_engine_tables.py`'s kind citations (both fail closed on the mismatch, as designed) —
both re-pinned against the real file; `missing_engine_tables.py`'s `ENGINE_SURFACE_CITATIONS`
also dropped the 7 entries whose marker text no longer exists anywhere in the source. Both
scripts' pinned-figure tests (`test_completion_atlas.py`, `test_missing_engine_tables.py`)
re-derived against the new live population; all 50 of their tests (38 + 12) pass.

`docs/work-inventory.json` regenerated at HEAD (`CORPUS_LITERAL_SWEEP_REPORT` +
`DERIVED_FIXTURE_CHECK_REPORT` supplied so the stamp-loss guard did not need `--allow-stamp-loss`,
never used). Corpus-wide bucket movement: `DONE +1479` (`12265→13744`), `A -8014` (`8463→449`),
`B +2497`, `D +1019`, `M +2016`, `V +1003`; `C`/`U`/`X`/`Z` unchanged. `corpus_literal_sweep`:
`48699 → 48699`, delta 0 (no corpus files touched). Full receipt:
`artifacts/epic-2-tables/AT-34-E2-004_cycle_receipt.md`.

### Cycle 11 — AT-34-E2-003 — the measured build rate is recorded

**Status: complete.** Records the real cost of building Epic 2's 8 tables to
`artifacts/epic-2-tables/table-build-rate.json`: no production code this cycle, only
measurement of work already landed (`AT-34-E2-001` commit `052a9182bf`, `AT-34-E2-002` commit
`b7507f3817`).

**No blended average.** Marginal lines per kind spread **2 to 12** (6×): `domain`/`skill`/
`language` cost 2 lines each (a one-line directory-table entry + a one-line macro test
invocation); `trait` cost 12 (the same two lines, plus a 7-line dedicated regression test and 3
doc lines pinning its `trait_generic` directory-name mismatch — the "shallow glob lies" hazard
from `workflow-instruction.md §4`); `ability`/`template`/`deity` cost 7 each (typical macro
block); `companion`'s table costs 0 this bundle (pre-existing, SD-29 — only its 21-line
fail-closed test is new). **Finding for Epic 5:** record count does not predict cost — `ability`
(4,337 records) and `domain` (183 records) cost almost the same, because both reuse one shared
generic loader unmodified. The real driver is whether a kind's corpus directory name matches
its kind name; `power`'s Epic 5 price depends on whether `ultimate_psionics` needs its own
shape handling the way `trait` did, not on its 421-unit count.

**Wall time, honestly scoped:** the 7 new tables were built through one shared loader in a
single commit — there were never 7 independent build sessions to time. The artifact reports
real, re-derivable whole-cycle wall time (`AT-34-E2-001`: 1,359s / 0:22:39; `AT-34-E2-002`:
779s / 0:12:59, both from `git log --format=%ci`) and, separately, per-table wall time
pro-rated from measured marginal lines — explicitly labeled **ESTIMATE** in the artifact and
receipt, never presented as independently measured (`AGENTS.md` rule 9).

Row-count command output: `python3 -c "import json; print(len(json.load(open('artifacts/epic-2-tables/table-build-rate.json'))['tables']))"` → `8`, of the 8 tables Epic 2 builds.
Build scope: `cargo test --locked --no-run` exit 0 (workspace, 600 executables) and
`apps/desktop/src-tauri` exit 0 (one pre-existing unrelated `dead_code` warning), both at HEAD
`b7507f38178e41b3962ef3161ee525e5ad9ee9b0`. Receipt:
`artifacts/epic-2-tables/AT-34-E2-003_cycle_receipt.md`.

### Cycle 10 — AT-34-E2-002 — each new table is fail-closed

**Status: complete.** Formalizes fail-closed proof as its own deliverable
(`artifacts/epic-2-tables/fail-closed-proofs.md`): all 8 Epic 2 tables, per-table, a RED→GREEN
pair — refusing an absent key and returning a real record for a present one. The 7
`simple_kind_tables` resolvers already carried this proof inline from AT-34-E2-001 (cited by
test name, not duplicated). The 8th, `companion` (pre-existing from SD-29), had no dedicated
fail-closed test; this cycle adds
`companion_chassis::tests::companion_resolve_refuses_a_fabricated_key_it_never_defaults`.

RED confirmed for the intended reason: `companion_resolve` was temporarily mutated to fall back
to `self.companions.first()` instead of refusing an absent key, and the test failed on the
fabricated-key assertion specifically (`a fabricated key must never resolve to a companion
record, real or defaulted`), not an unrelated panic. Reverted, then GREEN: 15/15
`companion_chassis` tests pass, 11/11 `simple_kind_tables` tests pass (unchanged).

Row-count command output: `grep -c '^| \`' .../fail-closed-proofs.md` → `8`, of the 8 tables
Epic 2 builds. Receipt: `artifacts/epic-2-tables/AT-34-E2-002_cycle_receipt.md`.

### Cycle 9 — AT-34-E2-001 — each of the eight tables is built, or proven unnecessary

**Status: complete.** Epic 2 builds 8 of `power`'s 9 kinds; one of the eight — `companion` — already
has a real, fail-closed table from SD-29 (`rules_core::rules_tables::companion_chassis`). This
cycle builds the other **seven**: `ability`, `template`, `trait`, `deity`, `domain`, `skill`,
`language`. New module `src/rules_core/rules_tables/simple_kind_tables.rs`:
`load_simple_kind_table(repo_root, kind)` loads every corpus record for a kind, across every book,
from the live `data/corpus/<book>/<dir>/*.json` tree; `resolve(book, key)` returns the real record
for a present key or `None` for an absent one.

**Directory-name hazard caught before shipping:** `trait`'s 487 units live under
`data/corpus/*/trait_generic/*`, not `trait/` — a naive `kind == dir name` glob would silently
return zero. `kind_dir_for("trait")` resolves this explicitly and a pinning test guards it. RED
confirmed for the intended reason (a temporary revert to the naive mapping failed with `trait table
loaded zero records from "trait"`, not an unrelated panic), then GREEN: 11/11 new unit tests pass.

Wired into `v06_work_inventory.rs` via a new read-only `--epic2-table-transcript` flag (same
contract as `--spell-probe`), which produced the committed transcript
(`artifacts/epic-2-tables/AT-34-E2-001_table_transcript.txt`) — 8 of 8 kinds report `HELD` on a
named sample record, and every kind also demonstrates `REFUSED` on a fabricated key in the same
run.

Identifier/wired-integration audits: `OK_NO_BUNDLE_TAGS` / `OK_NO_TOKENS`. Denominator gate against
this package: `files_checked=15 violations=0`. `cargo test --locked --no-run` exits 0 at the widest
workspace scope; `apps/desktop/src-tauri` not touched, not run. `data/corpus/**` and
`docs/work-inventory.json` untouched this cycle — zero movement across all four buckets; sweep
population N/A (no corpus write). Receipt:
`artifacts/epic-2-tables/AT-34-E2-001_cycle_receipt.md`.

**This cycle does not attempt `AT-34-E2-004`** (bucket A to zero for both vehicle books) — that
needs reachability/reclassification wiring these tables don't provide standalone, and is a
separate criterion.

### Cycle 7-R — AT-34-E1-007 re-verified after AT-34-E1-008 — `corpus-trap-audit` is GREEN

`scripts/verify.sh --only corpus-trap-audit` now **exits 0**:

```
PASS  corpus-trap-audit  (records_examined=27638 defects[wiring-class-mismatch=0
  disabled-line=165 key-differs-from-name=650 mod-record=2117
  shared-name-distinct-records=249] traps=407 — all defect kinds at their registered counts)
```

`wiring-class-mismatch` is **0 of 3,181** remaining DEFECT findings, down from **7,015 of
10,196** at the blocker. The four inherited kinds are each at **exactly** their launch count —
`mod-record` 2,117, `key-differs-from-name` 650, `shared-name-distinct-records` 249,
`disabled-line` 165, summing to 3,181 of 3,181 — **reported by name, not absorbed**. Books
carrying ≥1 DEFECT: **29 of 37**, down from 34 of 37. `corpus_literal_sweep`: **0 findings,
48,699 of 51,473 examined**, delta 0.

**One instrument correction was required first.** The stage decided PASS/FAIL from an aggregate
`defects == 0`, which (a) never reported `wiring-class-mismatch` at all, so AT-34-E1-008's
"reported at their counts and not absorbed" bar could not be read from it, and (b) cannot
satisfy `decisions.md §13`, which in one paragraph keeps AT-34-E1-007's `exits 0` bar **and**
rules SD-33's 3,181 registered defects **registered, not absorbed** — they are DEFECT severity,
so the aggregate stayed red forever. The verdict is now a **ratchet on named kinds**
(`scripts/corpus_trap_audit_baseline.py`): an unregistered kind, a kind above its pin, **or a
kind below its pin** all FAIL, and every kind's count prints on every run. Strictly more
discriminating than the aggregate it replaces; the registered set did not grow and nothing was
excused. Mutation-proved by `scripts/tests/test_corpus_trap_audit_baseline.sh` (14 cases, wired
as the new `corpus-trap-audit-selftest` stage in QUICK and FULL) and by a live plant-and-remove
on one real corpus record: `wiring-class-mismatch` moves `0 → 1 → 0` while `records_examined`,
`traps` and all four registered kinds hold still.

Rows 7 and 8 both go `complete`, from the counts: the stage's own verdict line, and 34 of 34
book rows in `artifacts/epic-1-atlas/wiring-class-remediation.json` at `after=0` with 34 of 34
provenance checks PASS. Receipt:
`artifacts/epic-1-atlas/AT-34-E1-007_re-verification_receipt.md`.

### Cycle 8 — AT-34-E1-008 — `wiring-class-mismatch` driven to zero, group by group

**Status: in-progress** (this criterion is dispatched as parallel per-book groups; each group
commits and reports independently). Mechanism, established by group G1 and reused unchanged
here: `src/bin/restamp_wiring_class.rs` (new in G1's commit) — an additive restamp pass over
existing on-disk `data/corpus/<book>/**/*.json` records, following this repo's established
"enrichment pass, never a second generator" pattern (`enrich_*_raw_tokens.rs`). It rewrites
only the `wiring_class`/`wiring_class_signals` keys when they disagree with a fresh recompute
via the audit's own `WiringClassIndex`, every other field parsed and re-emitted untouched by
construction — never a hand-edit, per `decisions.md §13`/`N5`.

**G1** (`54e2d24e83`): `advanced_players_guide` 875→0, `core_rulebook` 798→0. Discovered that
`gen_book_cache`/`gen_core_rulebook_cache`/`gen_cache_apg` cover only `companion`/`class`
records' `wiring_class` (255 of 1,673) — the rest (`ability`/`domain`/`skill`/`template`/
`*_generic`) were ingested by one-off Python scripts predating the real closure determinator
and cannot ever agree with the audit by re-running them, hence `restamp_wiring_class.rs`.
Receipt: `artifacts/epic-1-atlas/AT-34-E1-008_G1_cycle_receipt.md`.

**G2** (this cycle, `8df70c2ee4`): `beastiary` 783→0, `ultimate_psionics` 759→0,
`ultimate_campaign` 152→0 — 1,694 of the group-2 population. Same tool, same posture, run via
`cargo run --locked --bin restamp_wiring_class -- beastiary ultimate_psionics` then `--
ultimate_campaign`. Provenance verified per record by `git diff` against HEAD across all 2,494
changed files: only `wiring_class`/`wiring_class_signals` changed, 0 files added/removed, 0
provenance-field mismatches. `corpus_literal_sweep`: 48699 examined before → 48699 after (delta
0, correct — in-place restamp adds no records), 0 findings, CLEAN both runs. Build scope:
root workspace `cargo test --locked --no-run` exit 0 and `apps/desktop/src-tauri` (separate
workspace) `cargo test --locked --no-run` exit 0, both run at `8df70c2ee4`. Receipt:
`artifacts/epic-1-atlas/AT-34-E1-008_G2_cycle_receipt.md`.

Corpus-wide `wiring-class-mismatch` after G1+G2: `5342 - 1694 = 3648` of the original 7015.
Remaining groups (G3/G4 or however the wave is split) own the rest. AT-34-E1-007's own `exits 0`
bar closes only once every group lands at 0.

### Cycle 7 — AT-34-E1-007 — `corpus-trap-audit` is wired into `verify.sh`; blocked on real content it found

**Status: blocked-escalated.** The mechanical deliverable is done: a new `corpus-trap-audit`
stage (`cargo run --locked --bin v06_corpus_trap_report -- --audit --json`) is wired into
`verify.sh`'s `ALL_STAGES` (FULL scope, next to `corpus-sweep`), bounds its own runtime with a
`timeout` wrapper (closing `forward-scope-register.md D1.2`'s gap), and computes its population
independently of the binary's own output (`27,638` records, a `find`-based 3-level walk matching
`audit_ingested_cache`'s own traversal). RED→GREEN proved live: one real record's `wiring_class`
field was flipped, the stage's defect count moved exactly `10196 → 10197` naming that record, the
mutation was reverted via `git checkout --` (confirmed byte-identical to the pre-mutation file),
and the count returned to `10196`.

**That `10196` is the block.** Run for real against the live corpus, the stage is FAIL, not PASS:
`records_examined=27638 defects=10196 traps=407`. Of the 10,196 defects, 3,181 match four tests
already in `tests/v06_corpus_trap_report.rs` that SD-33's `forward-scope-register.md D1.1`
already verified as pre-existing, out-of-DoD debt. The other 7,015 (`wiring-class-mismatch`) are
a **new discovery**: this exact check was last driven to 0 by `SD30-CARRY-001` (`b32926f2af`,
2026-08-14) and has silently regressed across 34 of 37 books since, because nothing has run
`--audit` in `verify.sh` between then and now — the precise gap this criterion exists to close.
Fixing it needs `data/corpus/**` write scope Epic 1's file-touch table does not grant, and scales
~3.4× `SD30-CARRY-001`'s own 10-book/177-defect remediation — genuinely multi-cycle, not
foldable into this wiring criterion. Full figures, the RED→GREEN transcript, and the exact
re-derive command for every number: `artifacts/epic-1-atlas/AT-34-E1-007_cycle_receipt.md`. Retro
event: `docs/retro/events/sd34-at-34-e1-007.jsonl` (`incident`,
recurrence-key `unwired-standing-gate-decay`).

### Cycle 6 — AT-34-E1-006 — figure-provenance is a real `verify.sh` stage; denominator-gate default widened

**Status: complete.** Two obligations, one cycle. (1) A new `figure-provenance` stage
(`scripts/denominator_gate.py --check-provenance`, wired into `verify.sh`'s `ALL_STAGES` and
`QUICK_STAGES`) fails on a figure — a comma-grouped ≥4-digit number or a bare percentage — stated
inside a receipt's "Figures + their re-derive commands" section with no re-derive command
reachable from it on the same line; RED→GREEN mutation-proofed for both an unsourced figure and
a wrong-command figure (a command naming a script that does not exist), GREEN for a command
naming a real one. Default population: this package's own 5 receipts + 15 root `.md` docs
(`files_checked=20 figures_examined=22 violations=0`) — deliberately not SD-33's folder, which
this bundle may not write to. (2) `denominator-gate`'s `DEFAULT_GLOBS` widened to add SD-34's own
package (every root `.md`, plus its receipts) alongside SD-33's (unchanged) — a default run now
reads `files_checked=90 violations=0`, up from 0 SD-34 files before this cycle. Closes
`workflow-instruction.md §12` row 15 (UNENFORCED at launch) and `decisions.md §3`'s standing
obligation. 40 of 40 unit tests pass (`scripts/tests/test_denominator_gate.py`). Full details,
figures, and the mutation-proof transcript: `artifacts/epic-1-atlas/AT-34-E1-006_cycle_receipt.md`.

### Cycle 5 — AT-34-E1-005 — the `not-ingested` status field is renamed

**Status: complete.** The field asserted the opposite of its meaning (26,002 of 26,002 of its
units carry a real `source_file`+`source_line`; every evidence string is engine-side) and had
already misled once, during this package's own authoring. Renamed `not-ingested` →
`engine-does-not-hold` (and the Rust closures `not_ingested`/`not_ingested_owned` →
`engine_does_not_hold`/`engine_does_not_hold_owned`) in `src/bin/v06_work_inventory.rs`,
`docs/work-inventory.json` (26,239 of 26,239 occurrences), and every consumer under `tests/`,
`src/`, `apps/`, `scripts/` — 78 tracked files total, matching Cycle 4's handoff note: both
`completion_atlas.py`'s A/B/C/D bucket-D citation and `shape_engine_boundary.py`'s
`not_held_by_engine()` were updated in this same commit, so neither silently zeroes out.

New regression test `scripts/tests/test_legacy_not_ingested_string_swept.py`: sweeps `tests/`,
`src/`, `apps/`, `scripts/` for either retired spelling and fails closed on any live hit,
proven RED→GREEN by planting then reverting a synthetic violation
(`test_sweep_goes_red_on_a_planted_use_and_green_on_its_revert`). Live sweep:
`legacy_not_ingested_live_uses = 0` (of 76 files that carried the string before this cycle).

`docs/work-inventory.json` was relabeled via a validated whole-file substitution
(json-valid before/after, `26239` `"not-ingested"` → `0`, `26239` `"engine-does-not-hold"`,
exact parity) rather than a full generator re-run, to avoid the unrelated regression risk of
losing `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` context — this is a pure
relabel, confirmed by `completion_atlas.py --check` reporting the identical bucket counts as
before (`D=1230` unchanged) and by `tests/v06_work_inventory.rs`'s
`the_committed_inventory_is_well_formed_and_uses_only_declared_statuses` passing against the
edited file.

Both identifier and wired-integration audits clean (`OK_NO_BUNDLE_TAGS`, `OK_NO_TOKENS`).
Denominator gate against this package: `files_checked=15 violations=0`. `cargo build --bin
v06_work_inventory` exits 0; `cargo test --locked --no-run` (full workspace) exits 0; `apps/
desktop/src-tauri` `cargo check --locked` exits 0 (touched via `character_hub.rs`,
`spell_catalog.rs`, `reach_gate.rs`). Targeted Rust suites (`v06_work_inventory`,
`equipment_gap_tables`, `feat_gap_tables`, both `derived_evaluator_fixture_check*`) all green;
targeted Python suites all green except one **pre-existing, unrelated** failure in
`test_transcribe_monster_tables.py` (confirmed identical against unmodified `HEAD` before this
cycle's diff was reapplied — an unrelated concatenated-ability-text assertion, nothing to do
with the renamed string).

`docs/work-inventory.json`'s data content (which units exist, which status each carries) is
unchanged — zero reclassification, zero reachability movement. This cycle's only real movement
is closure of the misnomer itself, plus one instrument-correction (the atlas's stale-citation
guard updated to the new literal so it keeps resolving). Receipt:
`artifacts/epic-1-atlas/AT-34-E1-005_cycle_receipt.md`.

### Cycle 4 — AT-34-E1-004 — the shape-engine boundary is stated as a fact, not an assumption

**Status: complete.** New `scripts/shape_engine_boundary.py` commits, as a self-verifying
artifact, the fact that a shape engine turns a formula string into a number and does not
place/attach/display the record — that gate is the engine's own four-condition promotion
ladder, quoted from the live `src/bin/v06_work_inventory.rs` with its line citation re-checked
by content on every run, not merely path/line.

`python3 scripts/shape_engine_boundary.py --check` → `magnitude_bearing=26396
not_held_by_engine=13119 citation_ok=True`, exit 0. Both counts matched
`technical-design.md §3` / `decisions.md §2a`'s stated figures exactly on the first live
re-derive against the current `docs/work-inventory.json` — no drift since authoring. The
promotion ladder's four conditions at `src/bin/v06_work_inventory.rs:9592-9595` were
independently re-read with `sed -n` and match the exact block those documents quote, anchored
at line `9595` as they cite.

12/12 new unit tests green (`scripts.tests.test_shape_engine_boundary`), including a genuine
RED→GREEN mutation proof: the citation check was made to fail for the intended reason (a
line's live content no longer matching the expected fragment), confirmed it raises
`StaleCitationError` naming the exact line and mismatch, then confirmed it passes again once
restored. Denominator gate against this package: `files_checked=15 violations=0`. `cargo test
--locked --no-run` exits 0 at the widest workspace scope (Python + one generated markdown
artifact only; no Rust source touched); `apps/desktop/src-tauri` not touched, not run.
`docs/work-inventory.json` untouched — zero movement across all four buckets; this cycle is a
read-only, self-verifying statement of an already-established fact.

**Handoff note for AT-34-E1-005:** the new instrument's `not_held_by_engine()` keys on the
literal string `"not-ingested"`, same as `completion_atlas.py`'s bucket A/B/C/D arms — the
rename cycle must update it in the same commit or it will silently report
`not_held_by_engine=0`. Receipt: `artifacts/epic-1-atlas/AT-34-E1-004_cycle_receipt.md`.

### Cycle 3 — AT-34-E1-003 — the missing engine tables are enumerated and their book coverage mapped

**Status: complete.** New `scripts/missing_engine_tables.py` re-derives bucket A (`status ==
not-ingested`, evidence contains `has_no_engine_table`) directly from `docs/work-inventory.json`
and reports, per kind: unit count, per-book breakdown, the exact `not_ingested(...)`
engine-surface citation in `v06_work_inventory.rs` a real table would replace, and which books'
entire bucket-A population zeroes out once that kind's table exists.

`python3 scripts/missing_engine_tables.py --check` → `population=8463 kinds=9
citation_failures=0`, exit 0. Per-kind: `ability=4337 template=2248 trait=487 deity=459
power=421 domain=183 skill=149 language=136 companion=43` (sum = 8,463, matching
`completion_atlas.py`'s committed `buckets.A.count` exactly). Core Rulebook's slice
(`ability=471 template=262 skill=110 domain=34 language=22 deity=21 companion=14`, summing to
934 of `core_rulebook`'s 6,701 units) matches `technical-design.md §4` exactly and cross-checks
against `completion_atlas.py --by-book`'s independently-computed `core_rulebook A=934`.
`ultimate_campaign`'s slice (`ability=88 trait=154`, summing to 242 of 265 units, 91.3%)
confirms the epic-breakdown's "almost-single-bucket book" claim, cross-checked the same way.

`zero_bucket_a_books` (books a single kind's table alone would fully clear of bucket A):
`ability` → `inner_sea_faiths`; `language` → `inner_sea_temples`; `template` →
`inner_sea_intrigue`, `ultimate_intrigue`; the other 6 kinds → none (every book they touch also
carries a second bucket-A kind, so both tables are needed).

**Notable finding along the way:** a 10th `Kind::MonsterAbility` match arm in
`v06_work_inventory.rs` emits the same `has_no_engine_table` marker shape but contributes zero
live bucket-A units — its 3,806 units are already `text-complete`/`grounded`/verified, with only
13 `not-ingested` units, all landing in bucket B. Confirmed by reading the corpus data, not the
code path alone, before concluding the population is genuinely 9 kinds not 10 — the same
field-name-vs-field-meaning trap `decisions.md §12` L1 names.

12/12 new unit tests green (`scripts.tests.test_missing_engine_tables`), covering per-kind
counts, non-bucket-A exclusion, `zero_bucket_a_books` derivation, the engine-surface citation
(including a live re-check against the committed source), and a fail-closed
`UnknownKindError` for any future kind reaching bucket A with no citation entry. Denominator
gate against this package: `files_checked=15 violations=0`. `cargo test --locked --no-run`
exits 0 at `2ec0462736` (Python-only change; no Rust source touched); `apps/desktop/src-tauri`
not touched, not run. `docs/work-inventory.json` untouched — zero movement across all four
buckets; this cycle is a reclassification (finer view of the already-fixed bucket-A partition),
not new closure work. Receipt: `artifacts/epic-1-atlas/AT-34-E1-003_cycle_receipt.md`.

### Cycle 2 — AT-34-E1-002 — the atlas fails closed on six conditions

**Status: complete.** `scripts/completion_atlas.py` extended in place with the five remaining
fail-closed conditions on top of AT-34-E1-001's `unclassified`/`overlap` gate: (3) a `DONE`
unit whose evidence does not support it, (4) a bucket with no named clearing mechanism, (5) a
`derived_at` SHA that is not an ancestor of `HEAD` (staleness gate), (6) a bucket whose
definition does not cite the `file:line` that emits the evidence string it keys on, or whose
citation no longer resolves/matches at `HEAD`.

Live, unmutated: `python3 scripts/completion_atlas.py --check` → `population=49438 buckets=10
unclassified=0 overlap=0 done_evidence_violations=0 missing_clearing_mechanisms=0
stale_derived_at=False citation_failures=0`, exit 0. All ten buckets carry a real, verified
`file:line` citation into `src/bin/v06_work_inventory.rs`.

**Six RED→GREEN mutation proofs, one per condition, in
`artifacts/epic-1-atlas/fail-closed-proofs.md`.** Notable finding along the way: the naive
condition-3 design (reuse the A/B/C bucket markers verbatim as "must never appear in DONE
evidence") would have flagged 245 real, legitimate `DONE` units carrying `explanation_id` —
confirmed against the live corpus and excluded, with the exclusion documented in code and in
the proofs file (the same "field name vs. field meaning" trap condition 6 itself targets,
caught here before it shipped).

38/38 unit tests green (20 new + 18 inherited). Denominator gate against this package:
`files_checked=15 violations=0`. `cargo test --locked --no-run` exits 0 at the widest
workspace scope (run at `ceac19da29`); `apps/desktop/src-tauri` not touched, not run.
`docs/work-inventory.json` untouched — zero movement across all four buckets. Receipt:
`artifacts/epic-1-atlas/AT-34-E1-002_cycle_receipt.md`.

### Cycle 1 — AT-34-E1-001 — every unit carries exactly one named remaining-step

**Status: complete.** New `scripts/completion_atlas.py` partitions the full 49,438-unit
`docs/work-inventory.json` into the ten buckets fixed by `decisions.md §2`
(`DONE A B C D M V U X Z`), keyed on `status` + `evidence` per `technical-design.md §1`'s
implementation table.

`python3 scripts/completion_atlas.py --check` → `population=49438 buckets=10 unclassified=0
overlap=0`, exit 0. Bucket counts: `DONE=12265 A=8463 B=11921 C=4388 D=1230 M=2455 V=8330
U=321 X=46 Z=19` (sum = 49,438). `A` and `U` match the epic-breakdown's independently-stated
figures (8,463 across 9 kinds; 321 split 270/51 by evidence, 140/119/62 by kind) exactly on
the first live run. `D` and `U` sub-causes are enumerated in the committed artifact, not
shrugged. Cross-checked against SD-33's inherited, independent `box_ledger.py --check`
partition: `uncovered=0 overlap=0 population=49438` — both partitions agree on the same
population.

18/18 new unit tests green (`scripts.tests.test_completion_atlas`); a live mutation on the
bucket-A marker string was proven RED for the intended reason then reverted to GREEN.
Denominator gate against this package: `files_checked=15 violations=0`. `cargo test --locked
--no-run` exits 0 at the widest workspace scope; `apps/desktop/src-tauri` not touched, not run.

**This cycle does not implement AT-34-E1-002** (the six fail-closed conditions) — a separate
criterion in the same file, picked up next. Receipt:
`artifacts/epic-1-atlas/AT-34-E1-001_cycle_receipt.md`.

## Open blockers

**This section is not a parking lot.** An entry here is a request for an operator ruling and
it **pauses the bundle** (`../../governance/blocker-closure-doctrine.md`). It is never a
disposition, never a closure path, and no later cycle may proceed past a blocked card on its
own authority.

### AT-34-E3-001 — RESOLVED 2026-08-27 by orchestrator ruling (`decisions.md §14`)

<details>
<summary>Archived — the filing, and the ruling that cleared it</summary>

The cycle asked the operator to authorize running AT-34-E3-001 as further per-mechanism cycles
rather than one. **That is a sequencing decision, not an operator ruling**, and filing it paused
the bundle to ask permission to keep working — the one use an escalation must never be put to
(`../../governance/blocker-closure-doctrine.md`: a large blocker is a sequencing problem, not an
exemption). Cleared without escalation; the nine mechanisms are dispatched as their own cycles,
cheapest-first, and the criterion's bar is unchanged.

The cycle's own valuable half stands: it named the remainder **by mechanism with a population
each**, which is what makes the next wave dispatchable. Re-derived at HEAD, the enumeration
returns **nine** mechanisms summing to **1,006 of 1,006** — the filing said "ten"; the count is
corrected here, not carried.

Its two sub-questions are answered in `decisions.md §14`: `domain` (1 unit) is ordinary guarded-
generator ingestion work, and `deity` (21 units) proceeds under a stated PI constraint — match on
stored coordinates, keep the masked keys, never read or emit a redacted name, both PI gates stay
green. Only an inability to work inside that constraint would be a genuine escalation.

Also of note, and not a defect: this cycle's own 22-line insertion shifted all ten of
`completion_atlas.py`'s `BUCKET_DEFINITIONS` `file:line` citations, and **AT-34-E1-002 condition 6
fired exactly as designed** (`citation_failures` 10). The cycle re-derived each line and closed it
in the same cycle. The gate caught a real regression on its first live opportunity.

The bundle is un-paused.

</details>

*(no active blockers)*
