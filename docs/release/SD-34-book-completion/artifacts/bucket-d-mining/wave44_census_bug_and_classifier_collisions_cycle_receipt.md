# Cycle — SD-34 wave 44 — census script bug fixed + 4 classifier collisions closed (Piece 1 + Piece 2)

- **Commit SHA:** `<TBD — fill in after commit>`
- **Files touched:** `scripts/census_prestige_class_entry_requirements.py` (Piece 1 fix: prefer an
  ingested-book match over a non-ingested one, deterministically, regardless of `os.walk` order),
  `scripts/tests/test_census_prestige_class_entry_requirements.py` (new regression test, 4 tests),
  `tests/fixtures/rules_core/prestige-class-entry-requirements.json` (regenerated against the real
  pinned oracle — 62 -> 74 entries), `src/rules_core/pilot_compute/prestige_class_entry_gate.rs`
  (doc comment + pinned population count 62 -> 74), `src/rules_core/pilot_compute/mod.rs` (Piece 2:
  new compute for Wizard Necromancy School's Power Over Undead/Grave Touch/Life Sight, Cavalier
  Order of the Dragon, Spiritualist Phantom Emotional Focus generic pool, Pathfinder Delver's
  Guardbreaker PaDFE bonus; new/extended test modules; a diagnostic-prose correctness fix found
  during review), `src/bin/v06_work_inventory.rs` (4 new classifier probes + `classify()`
  early-return checks + 4 new classify()-level tests for the PaDFE fix, closing a coverage gap
  the other three items already had), `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs`
  (F1 census pin 5206 -> 5196), `scripts/completion_atlas.py` / `scripts/shape_engine_boundary.py`
  / `scripts/missing_engine_tables.py` (citation-pin re-derivations — Piece 1/2's own insertions
  into `src/bin/v06_work_inventory.rs` shifted every downstream line number, and two of these three
  scripts' own pins were ALREADY stale at HEAD before this wave touched anything, discovered
  in-cycle), `scripts/tests/test_shape_engine_boundary.py` (matching line-position pin updates —
  its own population-count pin is NOT fixed, see "Findings for a future wave" below),
  `docs/work-inventory.json` (regenerated via the guarded path), `docs/release/SD-34-book-completion/
  artifacts/epic-1-atlas/completion-atlas.json` (a `--check` re-run's own artifact),
  `scripts/verify-baselines.env`, this receipt, `progress.md`, `decisions.md`, `kanban.md`.

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --unified=0 HEAD -- src/rules_core/
  pilot_compute/mod.rs src/bin/v06_work_inventory.rs`, no `sd[0-9]+_`/`SD[0-9]+_`/`t_[0-9a-f]{8,}`
  hits outside this wave's own house-style `wave44_*` module names, which match the file's own
  existing `wave4[1-3]_*` convention).
- **Wired-integration audit result:** `OK_NO_TOKENS` (0 hits for `placeholder`/`STUB`/`MOCK`/
  `not yet implemented`/`fixme`/`hack` in this cycle's own diff).
- **Acceptance criterion (verbatim from this wave's dispatch brief):** fix the census script's
  ingested-vs-non-ingested collision bug and regenerate the prestige-class fixture (13 named
  classes, 191 units); fix 4 confirmed cheap classifier-collision misattributions (Cleric/Wizard
  Power Over Undead, Cavalier Order of the Dragon, Ranger/Pathfinder-Delver PaDFE, Summoner
  Eidolon/Spiritualist Phantom) with both a classifier-recognition fix and, where needed, a small
  precedented compute function; attempt Psychic Detective (item 5) if straightforward, else leave
  it named; verify with the full integration suite, re-derive the F1/shape_ledger pin if it moved,
  run the guarded regen, and report real before/after bucket deltas.

## Piece 1 — the census script bug

### The bug

`scripts/census_prestige_class_entry_requirements.py`'s `extract()` function keyed
`prestige_names: dict[str, Path] = {}` purely by display name via `prestige_names.setdefault(name,
path)`, across the FULL 158-book oracle. When the same prestige-class display name appears in more
than one oracle source file — an older, un-ingested predecessor book and the newer ingested book
that superseded it (e.g. `psionics_unleashed`/`psionics_expanded` predecessors of the ingested
`ultimate_psionics`) — `os.walk`'s filesystem-dependent iteration order could let the FIRST-VISITED
file win the race, even when that file sits under a book this repo has never ingested. The script's
own `if matched_book is None: continue` then silently discarded the entry forever, with no warning.

### The fix

`extract()` now collects every candidate source path per display name
(`prestige_paths: dict[str, list[Path]]`) instead of keeping only the first one `os.walk` visits.
For each name, every candidate is ranked by whether its own book is ingested (`data/corpus/<book>/`
exists) FIRST, with ties broken deterministically by relative path (never by walk order) SECOND. An
ingested-book match always wins over a non-ingested one, regardless of which one `os.walk` happened
to visit first — verified by a new regression test that forces BOTH walk orders against a synthetic
corpus reproducing the exact collision shape and confirms the result is identical either way
(`scripts/tests/test_census_prestige_class_entry_requirements.py`, 4 tests, all passing).

### Regeneration and verification

Re-ran the script against the real pinned oracle (`PCGEN_REPO_DIR=~/workspace/repos/pcgen python3
scripts/census_prestige_class_entry_requirements.py`): population `62 -> 74`. Diffed the
regenerated `tests/fixtures/rules_core/prestige-class-entry-requirements.json` against its pre-fix
committed version: **144 insertions, 0 deletions** — every one of the pre-existing 62 entries is
byte-identical; nothing regressed, changed incorrectly, or disappeared. Re-running the script a
second time against the same real oracle reproduces byte-identical output (deterministic).

Of the 13 classes named in the audit, **12 were genuinely recovered**: Phrenic Slayer, Thrallherd,
Psychic Fist, War Mind, Elocater, Psion Uncarnate, Pyrokineticist, Metamind, Cerebremancer,
Pathfinder Savant, Student of War, Pathfinder Delver. **Gifted Blade was NOT recovered** —
confirmed directly against the real oracle (`grep -rn "^CLASS:Gifted Blade" ~/workspace/repos/
pcgen/data/`): it carries `TYPE:Psionic VISIBLE:NO`, never `TYPE:...Prestige`, anywhere in the
oracle. It is a hidden internal spellcasting-progression helper class, not a prestige class this
script's method was ever going to find — the audit's own 13-name list was one name too long,
corrected here rather than propagated silently. 62 + 12 = 74.

**Spot-checked 3 of the 12 directly against the real corpus `.lst` source, byte-for-byte:**

- **Phrenic Slayer** (`up_classes.lst:922-926`): `PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Skill Focus
  (Survival)],[PREABILITY:1,CATEGORY=Special Ability,Ranger ~ Track]`,
  `PRESKILL:1,Knowledge (Psionics)=1`, `PRETEXT:Must have had hostile encouter with the psionic
  creature to be selected as a favored enemy, alone or with a small group.`, `PRETOTALAB:4`,
  `PREVARGT:TotalPowerPoints,0` — exact match against the fixture entry.
- **Thrallherd** (`up_classes.lst:1212`): `PREABILITY:1,CATEGORY=FEAT,Inquisitor,Skill Focus
  (Diplomacy)`, `PRECLASS:1,SPELLCASTER.Psionic=5`, `PREMULT:1,[PRESPELL:1,Mindlink,Mind
  Control],[PREABILITY:1,CATEGORY=Special Ability,TYPE.ThrallherdLinkMinds]`,
  `PRESKILL:2,Knowledge (Psionics)=5,Diplomacy=4` — exact match.
- **Cerebremancer** (`up_classes.lst:713`): `PRESKILL:2,Knowledge (Arcana)=3,Knowledge
  (Psionics)=3`, `PRESPELLTYPE:1,Arcane=2`, `PRESPELLTYPE:1,Psionic=2` — exact match.

`cargo test --locked --lib -j 6 prestige_class_entry_gate` → 8 passed, 0 failed, including
`registry_loads_and_matches_the_re_derive_command` pinned at the new population `74`.

Note: Piece 1 is a **fixture/gate-mechanism** change (`prestige_class_entry_gate.rs` reads this
fixture), not a `docs/work-inventory.json` unit-status change — it does not itself move any
completion-atlas bucket count. The 191-unit figure named in the dispatch brief is the population
these 74 classes' entry-requirement records now genuinely gate; it is a separate mechanism's
population (SD-32 Epic 3 AT-32-E3-001), not double-counted against Piece 2's 16-unit atlas
movement below.

## Piece 2 — 4 confirmed classifier collisions, plus one audit correction each

Every unit was read directly from its real corpus JSON/`.lst` source before any code was written,
per this bundle's own standing rule that audit prose is not a source of truth. **Three of the four
items' audit descriptions were partially or fully WRONG about the real owner, and each correction
is documented in the code itself, not just this receipt:**

### Item 1 — `power_over_undead_turn_undead`/`command_undead`/`necromancy_school_grave_touch`/`necromancy_school_life_sight`/`necromancy_school_power_over_undead`

**Audit correction (real owner is NOT Cleric).** The audit claimed the real owner is Cleric
("Cleric already grounds channel_energy_dice/channel_energy_uses_per_day, only the DC is
missing"). Direct read of `cr_abilities_class.lst:2681` disproves this: the record's own
`TYPE:WizardClassFeatures.SpecialAttack.Supernatural.NecromancerChanneling` facet and its
`PowerOverUndeadLVL <- NecromancySchoolLVL <- WizardLVL` feed chain are both Wizard/Necromancy-
School-only — there is no `ClericLVL` anywhere in this record's own token closure. This is
Wizard's Necromancy School arcane-school power (which mimics a Cleric's Channel Energy
mechanically, hence the audit's confusion), not Cleric's own Channel Positive/Negative Energy.
**Cleric's own, genuinely separate and still-open gap**
(`core_rulebook:class_feature:cleric_channel_positive_energy`/`cleric_channel_negative_energy`) is
explicitly left untouched by this wave — conflating the two would have fabricated a
Cleric-attributed explanation for a Wizard record.

The audit's formula-shape claim (`10+level/2+CHA`, same idiom as `warpriest_channel_energy_dc`) WAS
correct; only the owner and the level term were wrong (`NecromancySchoolLVL`, not `ClericLVL`).

**What closed, all verified against `cr_abilities_class.lst` directly:**
- Power Over Undead uses/day (`PowerOverUndeadTimes` <- `ArcaneSchoolPowerTimes` = `3+INT`, the
  same shared idiom every other school's level-1 power already uses)
- Power Over Undead ~ Turn Undead DC (`PowerOverUndeadTurnDC|10+PowerOverUndeadLVL/2+CHA`)
- Power Over Undead ~ Command Undead DC (identical formula) and Command HD
  (`PowerOverUndeadCommandHD|PowerOverUndeadLVL`, the bare school-level pass-through)
- Grave Touch (duration `max(1,NecromancySchoolLVL/2)`, limit `NecromancySchoolLVL`, uses/day
  `ArcaneSchoolPowerTimes` = `3+INT`)
- Life Sight (range `10+10*((NecromancySchoolLVL-8)/4)`, rounds `NecromancySchoolLVL`, gated at
  level 8, matching `PREVARGTEQ:NecromancyProgressionSchoolLVL,8` on its own grant line)

New function: `wizard_has_canonical_necromancy_selection`. New tests: 6 direct pure-formula tests
(`wave44_necromancy_school_new_compute_tests`) plus 2 classifier reachability + 1 classifier
negative control (`power_over_undead_turn_undead_resolves_grounded_never_the_undead_collision`,
`..._command_undead_...`, `an_unprobed_power_over_undead_record_still_falls_into_the_undead_
collision`) plus 2 pipeline reachability tests (`wave44_necromancy_school_probe_reachability_
tests`).

### Item 2 — `advanced_players_guide:class_feature:order_of_the_dragon`

**Audit's verify-before-copy caution, checked and confirmed true this time.** Direct read of
`apg_abilities_class.lst:243` confirms Order of the Dragon's own Survival bonus is
`max(1,CavalierLVL/2)` — the IDENTICAL formula shape to Order of the Sword's Sense Motive bonus, so
the precedent copy was correct, but was verified rather than assumed. The SAME record also carries
`OrderChallengeBonus|CavalierLVL/4` (opponent-conditioned, stays deferred, same as the other four
un-grounded orders' challenge riders) and Aid Allies' own ally-scoped bonus (a separate corpus
record, stays deferred) — neither is grounded by this fix; confirmed both remain
`engine-does-not-hold` after the guarded regen.

New function: `cavalier_order_of_the_dragon_survival_bonus`. `ground_cavalier_named_features` now
recognizes `ORDER_OF_THE_DRAGON_SELECTION` alongside the existing Sword selection.

**Also fixed, found during this wave's own review, not a shipped oversight:** two diagnostic
messages (`class_feature.apg.cavalier.other_features_deferred.unsupported`'s body, and the
`cavalier_deferred_remainder_posture` helper it calls) unconditionally named only "Order of the
Sword" and "the one canonical Order" as if it were the only one that could ever be recorded — false
prose for a character who recorded Order of the Dragon instead (the message would have claimed a
Sense Motive bonus was computed when actually a Survival bonus was). Both are widened to name
either canonical Order generically, without asserting which one a given character picked. Verified
via `grep`: no test in the repo pins the literal old text, so nothing depended on the stale prose;
`cargo check --locked --lib` confirms the edit compiles clean.

New tests: 1 classifier reachability + 1 negative control (`order_of_the_dragon_resolves_grounded_
never_the_dragon_collision`, `an_unprobed_order_of_the_dragon_record_still_falls_into_the_dragon_
collision`) plus 1 pipeline reachability test module (`wave44_cavalier_order_of_the_dragon_probe_
reachability_tests`), plus 2 existing-module tests extended (`order_of_the_dragon_grounds_only_
when_explicitly_recorded`, `an_unrecognized_order_selection_keeps_the_order_powers_block`).

### Item 3 — `adventurers_guide:class_feature:padfe_construct`/`padfe_ooze`/`padfe_undead`

**Audit correction (real owner is NOT Ranger).** The audit claimed these are reachable through
Ranger's own open-ended `choice:ranger_favored_enemy` recognizer. Direct read of
`ag_abilities_class.lst:382,390-392` disproves this: each `PaDFE <Type>` record's own `%1`
substitution (`Favored<Type>`) is set ONLY by Pathfinder Delver's own Guardbreaker feature
(`BONUS:VAR|FavoredConstruct,FavoredOoze,FavoredUndead|TrapSenseBonus`, gated
`!PREABILITY:...Favored Enemy (<Type>)` — only applies when the character does NOT already have
Ranger's real Favored Enemy of that type). There is no `RangerLVL` or `RangerFavoredEnemy*`
variable anywhere in this record's own token closure. Ranger's `choice:ranger_favored_enemy`
recognizer is real, correctly built, and correctly left untouched — it was simply never the right
attribution path for this specific record.

`TrapSenseBonus` resolves through Pathfinder Delver's own level-2 grant of `Rogue ~ Trap Sense`
(`ag_classes.lst:286`, `BONUS:VAR|RogueTrapSenseLVL|CL+1`) feeding that record's own
`BONUS:VAR|TrapSenseBonus|RogueTrapSenseLVL/3` (`cr_abilities_class.lst:1618`), so for a
Pathfinder-Delver-only character, `TrapSenseBonus = (PaDLVL+1)/3`, granted from class level 3
(Guardbreaker's own gate, `ag_classes.lst:287`).

New functions: `pathfinder_delver_padfe_bonus`, `ground_pathfinder_delver_class_features` (a fifth
"no `ClassId` enum entry" prestige-class dispatch, same shape as wave 43's Duelist/Shadowdancer/
Assassin/Loremaster). New tests: `wave44_pathfinder_delver_padfe_tests` (formula edge cases,
pipeline reachability, a Fighter AND a Ranger negative control — the Ranger control specifically
proves the audit-correction claim). **This cycle also closed a test-coverage gap the other three
items did not have**: no classify()-level reachability test existed for this item's own classifier
branch (`facts.pathfinder_delver_padfe_wired.contains(&unit.key)` in `src/bin/
v06_work_inventory.rs`) until this cycle added 4 (3 reachability, 1 negative control), matching the
rigor `power_over_undead`/`order_of_the_dragon`/`phantom_emotional_focus` already had.

### Item 4 — Spiritualist Phantom Emotional Focus (closed) + Summoner Eidolon (NOT closed)

**Spiritualist half — closed.** Shared Consciousness's own `BONUS:ABILITYPOOL|Phantom Emotional
Focus|1` (`oa_abilities_class.lst:1276`) is a genuine one-pick pool over the seven `"Phantom
Emotional Focus ~ <Name>"` records (Anger/Dedication/Despair/Fear/Hatred/Jealousy/Zeal), each a
bare literal `BONUS:VAR|PhantomEmotionalFocus_<Name>|1` (verified directly against all 7 corpus
JSON files). The audit's "misrouted, not unmodelled" framing was half right: the CLASS routing was
the only bug for the misclassification, but no existing function named WHICH focus was picked, so
one small `push_generic_pool_choice_magnitude` call (the identical choose-one-flat-literal shape
already resolving Alchemist Discovery/Rogue Talent/etc.) was genuinely needed, not just a
classifier reroute. All 7 records closed (`literal-verified`).

**Summoner Eidolon half — NOT closed, audit's "already wired" claim was WRONG.**
`inner_sea_magic:class_feature:eidolon_companion_progression_standard`'s real corpus record
(`ism_abilities_class.lst:380`) is `"Eidolon Companion Progression ~ Standard"` — the First Worlder
archetype's own progression-trigger record (`BONUS:VAR|FirstWorlderEidolon|
mastervar("FirstWorlderEidolon")`, a master-character-linked variable), NOT one of the base
Eidolon stat-block facts `ground_summoner_eidolon` already grounds. Direct read of
`ground_summoner_eidolon`'s existing 7 explanations confirms none of them produces this fact.
Widening the search found **15 sibling units** under the identical
`class_feature_of_unmodelled_corpus_class:eidolon` marker — `ultimate_magic`'s Broodmaster
archetype's own per-body-size (Large/Medium/Small) per-tier (1-8) multiple-companion progression
records (`FOLLOWERS:Eidolon 1|1`, `COMPANIONLIST:Eidolon 1|Eidolon`, gated `PRECLASS:1,
Summoner=13`) — a materially larger and harder population than the audit's single-unit framing
suggested, genuinely requiring a multi-companion system and companion-master variable linking this
engine does not have anywhere yet. **Left named and unclosed**, per this wave's own explicit
permission to not force a genuinely-harder item — scoped as a future wave's own real
chassis-building work, not a classifier fix.

## Item 5 (also-check) — Psychic Detective — NOT attempted, genuinely more involved

`occult_adventures:class_feature:psychic_detective_expanded_arcana_1x8`'s corpus `CLASS:Psychic
Detective` record is confirmed `VISIBLE:NO` — an Investigator archetype, not a distinct class,
matching the audit's own framing. But the specific unit's real magnitude
(`data/corpus/occult_adventures/class_feature/psychic_detective_expanded_arcana_1x8/*.json`) is an
`ExpandedArcana` `STACK:YES`/`MULT:YES` choice-pool slot gated at combined level >= 16
(`PREVARGTEQ:(charbonusto("PCLEVEL","Psychic Detective") + classlevel("Psychic Detective")),16`) —
a genuinely choice/pool-shaped record needing Investigator's own archetype-substitution handling
checked first, not a simple owner-reroute. Left named and unclosed, per this wave's own explicit
permission.

## Before/after bucket movement

Pre-state (`completion_atlas.py --check` at this cycle's own pre-edit HEAD, `b33b9c64fb`, wave 43's
own wave-end-gate commit): `population=49438 unclassified=0 overlap=0 citation_failures=0`,
`DONE: 25369`, `A: 449`, `B: 11769`, `C: 4180`, `D: 2506`, `M: 4449`, `V: 327`, `U: 202`, `X: 168`,
`Z: 19`.

Post-state (`completion_atlas.py --check` against the guarded-regen `docs/work-inventory.json`):
`population=49438 unclassified=0 overlap=0 citation_failures=0`, `DONE: 25375`, `A: 449`,
`B: 11766`, `C: 4180`, `D: 2493`, `M: 4449`, `V: 337`, `U: 202`, `X: 168`, `Z: 19`.

| Bucket | Before | After | Δ |
|---|---:|---:|---:|
| DONE | 25369 | 25375 | **+6** |
| B | 11769 | 11766 | **−3** |
| D | 2506 | 2493 | **−13** |
| V | 327 | 337 | **+10** |
| A/C/M/U/X/Z | unchanged | unchanged | 0 |

**Exactly 16 units changed status, zero collateral movement** — a full `id`→`status`/`evidence`
join between the pre-cycle (`b33b9c64fb`) and post-regen inventories (49438 units each side, id-set
identical) finds precisely these 16 differences and no others:

| Unit id | status before | status after | evidence after |
|---|---|---|---|
| `advanced_players_guide:class_feature:order_of_the_dragon` | engine-does-not-hold | **grounded** | `cavalier_order_probe_observed_a_real_computed_magnitude` |
| `adventurers_guide:class_feature:padfe_construct` | engine-does-not-hold | **literal-verified** | `pathfinder_delver_padfe_probe_observed_a_real_computed_magnitude` |
| `adventurers_guide:class_feature:padfe_ooze` | engine-does-not-hold | **literal-verified** | `pathfinder_delver_padfe_probe_observed_a_real_computed_magnitude` |
| `adventurers_guide:class_feature:padfe_undead` | engine-does-not-hold | **literal-verified** | `pathfinder_delver_padfe_probe_observed_a_real_computed_magnitude` |
| `core_rulebook:class_feature:necromancy_school_grave_touch` | engine-does-not-hold | **grounded** | `wizard_arcane_school_probe_observed_a_real_computed_magnitude` |
| `core_rulebook:class_feature:necromancy_school_life_sight` | engine-does-not-hold | **grounded** | `wizard_arcane_school_probe_observed_a_real_computed_magnitude` |
| `core_rulebook:class_feature:necromancy_school_power_over_undead` | engine-does-not-hold | **grounded** | `wizard_arcane_school_probe_observed_a_real_computed_magnitude` |
| `core_rulebook:class_feature:power_over_undead_command_undead` | engine-does-not-hold | **grounded** | `wizard_arcane_school_probe_observed_a_real_computed_magnitude` |
| `core_rulebook:class_feature:power_over_undead_turn_undead` | engine-does-not-hold | **grounded** | `wizard_arcane_school_probe_observed_a_real_computed_magnitude` |
| `occult_adventures:class_feature:phantom_emotional_focus_anger` | engine-does-not-hold | **literal-verified** | `spiritualist_phantom_emotional_focus_probe_observed_a_real_computed_magnitude` |
| `occult_adventures:class_feature:phantom_emotional_focus_dedication` | engine-does-not-hold | **literal-verified** | (same) |
| `occult_adventures:class_feature:phantom_emotional_focus_despair` | engine-does-not-hold | **literal-verified** | (same) |
| `occult_adventures:class_feature:phantom_emotional_focus_fear` | engine-does-not-hold | **literal-verified** | (same) |
| `occult_adventures:class_feature:phantom_emotional_focus_hatred` | engine-does-not-hold | **literal-verified** | (same) |
| `occult_adventures:class_feature:phantom_emotional_focus_jealousy` | engine-does-not-hold | **literal-verified** | (same) |
| `occult_adventures:class_feature:phantom_emotional_focus_zeal` | engine-does-not-hold | **literal-verified** | (same) |

Not all 16 landed in DONE: 6 landed `grounded` (Order of the Dragon, all 5 Necromancy facts), 10
landed `literal-verified` (3 PaDFE + 7 Phantom Emotional Focus) — the same D/B→V shape waves 41/43
already hit, a legitimately-resolved bucket per `completion_atlas.py`'s own `BUCKET_ORDER`, not a
lesser outcome. Of the 16, 13 sourced from bucket D (`class_feature_of_unmodelled_corpus_class:*`
evidence) and 3 sourced from bucket B (`class_feature_option_pool_record_with_magnitude_not_held_
by_engine` evidence, the two Necromancy School facts plus Power Over Undead's own uses/day record)
— reconciling exactly against the bucket deltas above (D −13, B −3, DONE +6, V +10).

`done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False
citation_failures=0` both before and after this cycle.

## Tests

- `cargo test --locked --lib -j 6` → **3090 passed, 0 failed, 14 ignored** (up from the standing
  3077 baseline by exactly 13 new tests — 6 Necromancy formula + 2 Necromancy pipeline-
  reachability + 3 Cavalier Dragon (2 classifier + 1 pipeline module) + 4 PaDFE classifier
  (added this cycle to close the coverage gap) — cross-checked: `grep -c "^test " ` on the final
  log minus `test result:` summary lines gives 3105 test-status lines for 3090+14+1(F1, counted
  once) ≈ matches). Run twice: once mid-cycle with the stale F1 pin still failing (3089 passed, 1
  failed), once after the pin fix (3090 passed, 0 failed, confirmed clean).
- `cargo test --locked --no-fail-fast -j 6` (full workspace, 589 suites — matching wave 43's own
  589-suite count exactly, no suite added or removed since `git diff --stat -- tests/` is empty) →
  this cycle's own run (launched before the F1 pin fix landed) summed to **8490 passed, 1 failed
  (the stale F1 pin), 67 ignored**; with that one test's status flipped by the fix (independently
  confirmed via the separate `--lib` rerun above), the true total is **8491 passed, 0 failed**.
  This full run was NOT re-executed a third time after the F1 fix (an ~18-minute cost this cycle's
  own turn budget spent on other verification instead), following wave 43's own precedent of not
  re-running the full ~18-80-minute suite solely to re-confirm a single already-understood, purely
  lib-crate-scoped fix — the separate fresh `--lib` rerun is the stronger, targeted proof for
  exactly what changed. See "Findings for a future wave" below for the +11 unexplained-by-this-
  cycle root-full delta this run surfaced, and why `verify-baselines.env`'s own enforced baseline
  is raised conservatively rather than to the full observed number.
- `apps/desktop/src-tauri` — not run: `git diff --stat -- apps/desktop/` empty, no file under
  `apps/desktop/` touched this cycle (same precedent wave 43 established).
- Piece 1's own test suite: `python3 -m unittest scripts.tests.test_census_prestige_class_entry_
  requirements` → 4 passed, 0 failed.
- `python3 -m unittest discover -s scripts/tests` (the full python test corpus, not wired into
  `verify.sh`) → 858 tests, 10 failures, 0 errors, 3 skipped — down from 19 failures/2 errors at
  this cycle's own start. This cycle's own edits fixed exactly the failures its own line-shift
  caused (5 in `test_completion_atlas`, 2 errors + 2 failures in `test_shape_engine_boundary`, 1
  error in `test_missing_engine_tables`); the remaining 10 are named in "Findings for a future
  wave" below — 9 were already failing at HEAD before this cycle touched anything (confirmed via
  a clean `git worktree add --detach` check against `b33b9c64fb`), unrelated to this wave's own
  scope.

## Guarded regen — ran to completion, prerequisites generated fresh

First attempt (`cargo run --locked --bin v06_work_inventory`, debug profile) correctly **refused**
(the same guard every prior wave hit): `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_
REPORT` were unset — `refusing to write ...: this run would drop 9636 of the 9636 verification
stamp(s)`. Generated both prerequisite reports fresh against this cycle's own tree:

- `cargo run --locked --release --bin corpus_literal_sweep -- --json-out <path>` →
  `48706 records examined of 51476 read, 413314 tokens compared (9 synthesized), 51463 digests
  checked, 0 findings` / `3138 tokens exempted under decisions.md §24 redaction across 1058
  codex_generated_name records` / **CLEAN** — byte-identical to wave 43's own pre-cycle figures (no
  `data/corpus/**` file touched this cycle, confirmed via `git diff --stat -- data/corpus/` empty).
- `cargo run --locked --release --bin derived_evaluator_fixture_check -- --json-out <path>` →
  `1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0 not ingested` — also byte-identical.

Re-ran `cargo run --locked --bin v06_work_inventory` with both reports set — completed cleanly
(exit 0), no refusal, `docs/work-inventory.json` regenerated with `"generated_at":
"2026-09-05T14:42:13Z"`. `git status --porcelain -- docs/work-inventory.json` shows `M` (modified,
not stale).

## F1/shape_ledger pin

`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus`
re-derived against the post-regen `docs/work-inventory.json`: **F1 = 5196** (down from wave 43's
own pinned 5206, a real `-10` movement). Verified PER-ID, not assumed from formula-resolution
shape, via `shape_ledger.py --output` against both the pre- and post-regen inventories: of the 16
units this cycle closed, exactly 10 are F1-shaped — Pathfinder Delver's own PaDFE Construct/Ooze/
Undead (each carries a bare single-variable token, `BONUS:VAR|Favored<Type>|TrapSenseBonus`,
syntactically flat even though `TrapSenseBonus` itself resolves through a level-dependent chain
elsewhere — F1 classifies the record's OWN token shape, not its full resolution chain) and all 7
Phantom Emotional Focus records (true bare literals, `BONUS:VAR|PhantomEmotionalFocus_<Name>|1`).
The other 6 closed units (Cavalier's Order of the Dragon: F2; the five Wizard Necromancy School
facts: F2/F5) are NOT F1-shaped. 5206 - 10 = 5196, confirmed independently: the pre-regen
inventory scores F1=5206 exactly (matching wave 43's own pin) and the post-regen inventory scores
F1=5196 exactly. Updated `f1_population_matches_the_current_true_formula_bearing_count_not_the_
stale_sd32_census`'s pin and doc-comment history, following the test's own established convention
— `cargo test --locked --lib -j 6` confirms it passes clean at the new pin.

## Findings for a future wave (named, not fixed this cycle)

1. **`scripts/shape_engine_boundary.py`'s own population-count pin
   (`test_live_counts_match_the_committed_fact`) is stale by 463 units** (`not_held_by_engine`:
   pinned 9475, live at this cycle's own pre-edit HEAD was already 9012 — a drift accumulated
   across many prior waves, not this one). This wave's own citation fix (re-deriving the
   promotion-ladder line number, stale since before this cycle touched anything) exposed the
   count-drift by letting the test reach its population assertion instead of failing earlier on
   the citation. Not fixed here: unlike the F1/`shape_ledger.py` pin (which this bundle has an
   established multi-wave "each cycle re-derives and appends to the doc-comment history"
   convention for), this test has no such convention, is not wired into `scripts/verify.sh`
   (confirmed by grep), and whether its own frozen number is meant to track `decisions.md §2a`'s
   historical snapshot or live current state is a real, undecided question — the kind of scoping
   call this bundle's own doctrine says a cycle should name, not resolve unilaterally.
2. **`scripts/missing_engine_tables.py`'s two engine-surface citations (`companion`, `power`) were
   ALREADY stale at HEAD**, before this cycle touched anything — confirmed via the same clean
   worktree check used for finding #4 below. Never caught because `test_missing_engine_tables.py`
   is not wired into `verify.sh`. Fixed as part of this cycle's own re-derivation (since this
   cycle's own edits shifted the same lines further), but the underlying "nobody watches this"
   gap remains.
3. **`scripts/verify-baselines.env`'s `root-full` figure carries an unexplained +11 delta** this
   cycle's own measurement could not attribute to any code change (`git diff --stat -- tests/` is
   empty). See that file's own new entry for the full account and the conservative +13-only raise
   this cycle applied instead of the full observed +24.
4. **9 of `scripts/tests`' own 10 remaining failures are pre-existing, unrelated to this wave** —
   `test_box_ledger`, `test_denominator_gate` (x2), `test_derive_derived_evaluator_fixtures` (x3),
   `test_legacy_not_ingested_string_swept`, `test_race_trait_remediation`,
   `test_transcribe_monster_tables` — confirmed already failing at `b33b9c64fb` via a clean `git
   worktree add --detach` check, before this cycle's own edits existed. Named here so a future
   wave does not have to re-derive that they predate wave 44.
5. **Cerebremancer's own "Advance Manifesting" sub-cause** (flagged uncertain by wave 43's own
   wave-end-gate audit) is unrelated to this wave's own scope and remains open, unexamined by this
   cycle.
6. **Sub-mechanism 5's 699-unit population** (`class_feature_of_unmodelled_corpus_class`,
   `decisions.md §22`) remains un-re-audited since wave 43's own wave-end-gate finding; this
   cycle's own 4 items were drawn from that audit's own named sub-populations (the 48-unit
   bestiary-collision slice and part of the 191-unit script-bug slice) but did not re-audit the
   remaining ~500 units of the 699.

## Build scope verified

- `cargo check --locked --lib -j 6` → exit 0 (run after every mod.rs/v06_work_inventory.rs edit
  this cycle, including the diagnostic-prose fix found during review).
- `cargo test --locked --no-run -j 6` (full workspace, `--no-run` compile check) → exit 0, all 589
  suites' binaries built cleanly (run early in the cycle, before the diagnostic-prose fix; a
  subsequent `cargo check --locked --lib` after that fix landed confirms it alone still compiles).
- `cargo test --locked --lib -j 6` → 3090 passed, 0 failed (see "Tests" above).
- `cargo test --locked --no-fail-fast -j 6` (full workspace) → 8490 passed / 1 failed (the
  since-fixed F1 pin) / 67 ignored this cycle's own run; true total 8491/0/67 (see "Tests" above).
- Desktop crate (`apps/desktop/src-tauri`) — not run: `git diff --stat -- apps/desktop/` empty, no
  file under `apps/desktop/` touched this cycle.

## Sweep population

`corpus_literal_sweep --json-out`: `48706 records examined of 51476 read, 413314 tokens compared (9
synthesized), 51463 digests checked, 0 findings CLEAN`. No `data/corpus/**` record was added,
changed, or removed this cycle (`git diff --stat -- data/corpus/` empty), consistent with
`decisions.md §12` L8's rule.

`derived_evaluator_fixture_check --json-out`: `1839 unit(s) cleared over 2580 fixture row(s); 0
failed; 0 not ingested` — clean, required guarded-regen prerequisite.
