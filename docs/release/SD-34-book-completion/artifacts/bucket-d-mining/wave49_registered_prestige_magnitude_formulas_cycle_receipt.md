# Cycle — SD-34 wave 49 — 33-class magnitude-only sweep across sub-mechanism-5's registered-prestige remainder: 129 units closed

- **Commit SHA:** `PENDING` (feat commit; this receipt's own SHA fill-in lands in a second,
  docs-only commit immediately after, following this bundle's own established two-commit pattern)
- **Files touched:** `src/rules_core/pilot_compute/mod.rs` (33 new `<CLASS>_CLASS_ID` consts; ~90
  new pure formula functions; 33 new `ground_<class>_class_features` dispatch functions; 33 new
  call sites inside `compute_pilot_base_chassis`; 1 new test module,
  `wave49_registered_prestige_magnitude_formulas_tests`, 39 tests), `src/bin/v06_work_inventory.rs`
  (33 new `probe_<class>_wave49_wiring` functions reusing the existing `probe_wave46_single_owner_
  class_features` helper; 33 new `EngineFacts` fields; 33 new constructor lines; 33 new `classify()`
  early-return checks; 1 new test module, `wave49_registered_prestige_probe_reachability_tests`, 33
  tests — see "A second self-caught gap" below for why this file needed touching at all),
  `docs/work-inventory.json` (regenerated via the guarded path, twice — see below), `docs/release/
  SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (regenerated),
  `scripts/verify-baselines.env`, this receipt, `progress.md`, `decisions.md`, `kanban.md`.
- **Standing-rule departure (operator-directed, this wave only):** batch ALL 33 classes' code and
  tests first, then run the full verification sequence ONE time at the end (lib tests, full
  integration suite, clippy, guarded regen, gate scripts) — not the per-class/per-few-classes
  re-verify cycle waves 43-48 used. One clippy warning, one real correctness bug, and one real
  census-registration gap (all described below) were found across that verification pass and the
  guarded regen that followed it, and fixed, triggering two further full-suite runs (three total)
  rather than the single re-run the wave's own standing instruction anticipated — each fix was a
  genuinely separate, real defect, not repeated caution over the same one.

## Fresh sub-mechanism-5 population re-derivation

Queried `docs/work-inventory.json` directly (pre-edit, at wave 48's own close) for units whose
evidence contains `class_feature_of_unmodelled_corpus_class`: **575 total** (591 − 16, exactly
matching wave 48's own post-cycle figure — no drift since wave 48 closed). Cross-referenced against
the 74-entry `tests/fixtures/rules_core/prestige-class-entry-requirements.json` registry: **487
registered / 88 not registered**, the 88 unchanged from prior waves. Of the 487 registered:
- **116** belong to the excluded cross-class-manifester-level group (Sighted Seeker 21, Thrallherd
  19, Elocater 13, Psion Uncarnate 13, Psicrystal Imprinter 12, Phrenic Slayer 11, Metamind 10,
  Cerebremancer 9, Soul Archer 4, Metaforge 4).
- **2** are Divine Scion's own True Scion remainder.
- **13 (Aldori Swordlord)** and **11 (Magaambyan Arcanist)** are PI-name-blacklisted (wave 48's own
  finding — class NAME itself is Product Identity, ingesting under a redacted
  `codex_named_unit_*` directory).

Leaving **345** as this wave's real working pool across **42 remaining prestige classes** — matches
the dispatch brief's own count exactly, re-derived independently rather than trusted. Every one of
the 42 classes was checked for an ingested directory at `data/corpus/<book>/class_feature/<slug>/`
before any code was written (the wave 48 PI-blacklist signal); all 42 ingest normally under their
own slug.

## Method

For each of the 42 classes: read its real corpus records directly via a small indexing script
(joins `docs/work-inventory.json` units against `data/corpus/**/*.json` by `(source_file,
source_line)`, not by filename guessing), extracted every `DEFINE`/`BONUS`/`SPELLS`/`DR` token,
checked for a precedented compute-function shape already in `pilot_compute/mod.rs` (the flat
level-scaling idiom, the "10 + level factor + ability modifier" DC idiom, the pool-SIZE-only idiom,
the SLA-triple idiom, the cross-file class-table idiom `nature_warden_companion_bond_level`
established), and wrote the compute function + a real-pipeline reachability test. Genuinely
un-attemptable units (no `BONUS`/`DEFINE` token at all, a cross-class prime-stat/parent-entry
dependency matching the already-excluded 10-class shape, dice notation this engine's
`formula_interpreter.rs` does not parse, an untracked equipment/state gate) were left named, not
guessed at.

**A real, useful negative finding: three more Ultimate Psionics prestige classes carry the excluded
AS/MB/Ma cross-class-manifester-level shape, previously undiscovered.** Psychic Fist, Metamorph, and
War Mind — none named in the prior 10-class exclusion list — each carry the identical `Advance
Astral Suit`/`Advance Mind Blade`/`Advance Manifesting` (and their AS+MB+Ma combination) records,
confirmed by direct read, not assumed. Their non-AS/MB/Ma-shaped units (Infused Body, Ki Power,
Chain of Defensive/Personal Superiority, Enduring Body, Alter Metamorphosis, Free Shift, Natural
Shifter, etc.) were still closed normally. This widens the excluded population's own class count
from 10 to 13 (units unchanged, since these three classes' AS/MB/Ma units were already counted
inside the "116" figure above) — named here rather than silently absorbed.

**A real correctness bug self-caught on this wave's own re-verification, before commit (not shipped
as an oversight):** three classes' own draft formulas (Hellknight, Pathfinder Savant, Diabolist)
initially used `total_character_level(input)` for a class-table-fed `XLVL|CL` variable
(`HellknightArmorLVL`, `PaSSkillBonus`/`PaSLVL`, `DiabolistDamnedLVL`/`InfernalCharismaLVL`/
`HeresyLVL`/`HellfireRayLVL`). Re-checking against this bundle's OWN already-shipped precedent for
the identical `<X>LVL|CL` idiom (`PaDLVL|CL` → `ground_pathfinder_delver_class_features`,
`TwilightTalonLVL|CL`, `GoldenLegionnaireLVL|CL`, all already shipped treating bare `CL` on a
class's own table row as **that class's own raw level**, not total character level) showed the
three drafts were wrong: a multiclass character would have gotten an inflated value from every
other class level they held. Fixed to use the raw per-class `level` parameter directly, matching the
established precedent exactly. On this wave's own single-class test fixture the two values are
numerically identical (the fixture has no other class levels), so no test assertion needed to
change — the bug was invisible to every test until traced against the real oracle's own class-table
semantics, exactly the "a wrong computed number looks like a right one" failure mode this bundle's
own doctrine exists to catch.

**A second, more consequential self-caught gap: the census tool itself never knew about any of
these 33 classes.** After writing all 33 `ground_<class>_class_features` functions and their
`pilot_compute/mod.rs` tests (39 passing), a first guarded `docs/work-inventory.json` regen showed
**zero** status changes anywhere — every one of this wave's own new explanation ids fires correctly
against the real pipeline, but `src/bin/v06_work_inventory.rs`'s own `classify()` function has no
knowledge that a corpus class's formulas exist unless a dedicated `probe_<class>_wiring` function,
a matching `EngineFacts` field, and a `classify()` early-return check are ALSO added per class —
exactly the four-part registration pattern waves 43-48 each applied, which this wave's own "batch
the compute code, verify once" plan never accounted for. Fixed by adding all 33 probes (each a thin
wrapper around the existing `probe_wave46_single_owner_class_features` helper, one call per class,
listing that class's own (explanation id, corpus key) pairs) plus a new `wave49_registered_
prestige_probe_reachability_tests` module (33 tests, one per class) proving each probe resolves the
real corpus keys through the real pipeline — not merely that the pure formula functions return the
right numbers. **Worth naming for any future wave using this same batched approach: writing a
working `ground_*` function and its own direct tests is necessary but not sufficient for the census
to see it; a "zero deltas" regen result is the signal to check for exactly this gap, not to assume
the wave genuinely closed nothing.**

One unit was found unclosable through this path during that same investigation: **Student of War's
own Mind Over Metal** (`student_of_war_mind_over_metal_ac_bonus`) is a real, independently-tested
formula gated on the character's Intelligence modifier exceeding their Dexterity modifier, but the
shared census probe fixture's own ability scores (INT 10, DEX 14 → INT mod 0, DEX mod +2) never
satisfy that gate at any level — the probe can never observe it reachable. Rather than build a
bespoke fixture override for one unit under time pressure, it is left named, not force-closed:
this wave's own closed-unit count is **129**, not the 130 originally drafted.

## Class-by-class table

| Class | Book | sm5 units | Closed | Skipped (reason) |
|---|---|---:|---:|---|
| cyphermage | inner_sea_magic / adventurers_guide | 17 | 3 | 14 no `BONUS`/`DEFINE` token (pure prose scroll-metamagic abilities) |
| psychic_fist | ultimate_psionics | 16 | 3 | 13: AS/MB/Ma manifesting-variable family (cross-class prime-stat, matches excluded shape) + Mesmerizing Glow's own save DC (same dependency) |
| asavir | adventurers_guide | 15 | 10 | 5: 3 no-token mount-echo abilities, 1 prose (Inspiring Leader/All Eyes on Me), 1 size-category cross-reference (Trample dice) |
| metamorph | ultimate_psionics | 15 | 3 | 12: AS/MB/Ma family (7) + Learn Level Five/Major Metamorphosis (SPELLKNOWN `%LIST` class param) + Gain Major Metamorphosis/Quick Shift (no token) |
| war_mind | ultimate_psionics | 15 | 3 | 12: AS/MB/Ma/manifesting-variable family + Well of Power (no token) |
| hellknight | adventurers_guide / inner_sea_world_guide | 14 | 7 | 7: Hell's Knight/5 order-weapon proficiencies (no magnitude), Infernal Armor (untracked equipped-armor state) |
| adaptive_warrior | ultimate_psionics | 14 | 5 | 9: AS/MB/Ma family (7) + Block Attack (prose) + Weapon/Armor Proficiencies (no token) |
| sanguine_angel | adventurers_guide | 13 | 2 | 11: Sanguine Angel Discipline's own 6-member `ABILITYPOOL` choice (needs new choice-gating machinery, left for a future wave), Maiden's Shield (score-vs-modifier ambiguity on `PreStatScore_DEX`), 4 no-token feat-grant records |
| body_snatcher | ultimate_psionics | 13 | 3 | 10: AS/MB/Ma family (7) + The Flesh Remembers (`BodySnatcherPrimeStat`) + Mimic Mind/True Mind Switch (no token) |
| golden_legionnaire | adventurers_guide | 12 | 0 | own remainder, unchanged since wave 48 (10 no-token/auto-grant + Combat Feat/Legion Feats bonus-feat pools) |
| steel_falcon | adventurers_guide | 12 | 4 | 8: Heroic Speech (cross-class Bardic Performance dependency), Natural Traps (feat-pool interaction), 6 prose/no-token |
| lantern_bearer | adventurers_guide | 11 | 3 | 8: Lantern Arcana (multi-tier INT-gated spell list, left for a future wave), 7 no-token |
| storm_kindler | adventurers_guide | 11 | 4 | 7 no-token (Eye of the Storm, Fickle Winds, Seasight, Storm Mastery, Thunderstruck, Wave Breaker, Echoing Thunder prose) |
| westcrown_devil | adventurers_guide | 11 | 3 | 8: Classically Trained (shield-check-penalty cross-reference) + 7 PI-redacted-name no-token records |
| pyrokineticist | ultimate_psionics | 11 | 8 | 3: Conflagration/Heat Death (`PyrokineticistPrimeStat`), Fire Soul (shared `NimbusTimes` variable, not independently attributable without double-counting Nimbus's own contribution) |
| aspis_agent | adventurers_guide | 9 | 4 | 5 no-token/prose |
| gray_corsair | adventurers_guide | 9 | 2 | 7 no-token/prose |
| pathfinder_savant | adventurers_guide | 9 | 5 | 4 no-token (Dispelling Master, Esoteric Spells, Silence Master, Spellcasting Master) |
| rivethun_emissary | adventurers_guide | 9 | 5 | 4: Enhanced Familiar (`ShamanHexChoices` cross-record pool size), Animus/Manifest Will (prose), Spirit Conduit (no token) |
| telekinetic_weaponmaster | ultimate_psionics | 9 | 0 | AS/MB/Ma family (7) + Telekinetic Block (`PsionicFocusActive`, an untracked runtime toggle) + Telekinetic Reach (no token) |
| student_of_war | adventurers_guide | 8 | 1 | 7: Mind Over Metal (real formula, but the shared census probe fixture's INT ≤ DEX never satisfies its own gate — see above), Anticipate/Know Your Enemy (`DEFINE`-only, no formula anywhere), 4 no-token stances |
| diabolist | book_of_the_damned_volume_1 | 8 | 6 | 2: Imp Companion (multi-class caster-level max, matches Wizard's Arcane Bond-tier open-ended complexity), Hellish Soul (no token) |
| lion_blade | inner_sea_intrigue | 8 | 3 | 5: Bardic Performance (cross-class dependency), 4 prose |
| bellflower_tiller | adventurers_guide | 7 | 5 | 2: Favored Barn (no token), Scarecrow (prose) |
| hellknight_signifer | adventurers_guide | 7 | 3 | 4: Arcane Armor Expertise/Catechesis (auto feat grants, no magnitude), Diabolic Harbinger (no token), Signifer Armor Training (`ACCHECK` armor-check-penalty cross-reference, left for a future wave) |
| mystic_archer | ultimate_psionics | 7 | 7 | **fully closed** |
| mammoth_rider | adventurers_guide | 6 | 6 | **fully closed** |
| demoniac | book_of_the_damned_volume_2 | 6 | 2 | 4: Energumen (pure ability-score choice, no formula), Obedience (`DEFINE`-only), Weapon Proficiencies (no token) |
| master_chymist | advanced_players_guide | 5 | 5 | **fully closed** (Mutate's own two duration facts, needing cross-class `AlchemistMutagenLVL`/`AlchemistPersistentMutagenLVL`, folded under the same unit's Times fact, which is standalone) |
| twilight_talon | adventurers_guide | 5 | 0 | own remainder, unchanged since wave 48 (pure prose: Many Hats, Eye for Detail, Dead Drop, Resourceful Agent, Unassuming Presence) |
| enchanting_courtesan | inner_sea_intrigue | 5 | 2 | 3 prose (Master Poisoner, Seducer's Leverage, Touch of Ecstasy) |
| dark_tempest | ultimate_psionics | 5 | 5 | **fully closed** (this class does NOT carry the AS/MB/Ma shape, confirmed by direct read before scoping) |
| battle_herald | advanced_players_guide | 4 | 2 | 2: Banner/Voice of Authority (no token) |
| master_spy | advanced_players_guide | 4 | 3 | 1: Death Attack (no token, auto grant) |
| pure_legion_enforcer | inner_sea_combat | 4 | 0 | all 4 prose/no-token |
| evangelist | ultimate_combat | 4 | 1 | 3: Bardic Performance/Sermonic Performance (cross-class Cleric-as-Bard-performance-level substitution), Public Speaker (prose) |
| stalwart_defender | advanced_players_guide | 2 | 0 | own remainder, unchanged since wave 46 (Increased DR needs untracked pool-selection state, Renewed Defense uses dice notation) |
| holy_vindicator | advanced_players_guide | 1 | 0 | own remainder, unchanged since wave 46 (Channel Smite, bonus-feat grant, no magnitude) |
| nature_warden | advanced_players_guide | 1 | 0 | own remainder, unchanged since wave 46 (Woodforging, no token) |
| rage_prophet | advanced_players_guide | 1 | 0 | own remainder, unchanged since wave 46 (Spirit Warrior, no token) |
| ulfen_guard | inner_sea_combat | 1 | 1 | **fully closed** |
| sentinel | ultimate_intrigue | 1 | 0 | this class's one sm5 unit is feat-chain-gated (`CombatStyleLVL`, a Ranger Combat Style tracker), not level-gated on `class:sentinel` at all — a different owner shape than every other unit in this population, left for a future wave rather than guessed at |
| **Total** | | **345** | **129** | **216 named, not attempted** |

## Verification

- `cargo test --locked --lib -j 6`: run 1 (before any fix) → 3182 passed, 0 failed, 14 ignored (up
  from the 3143 baseline by this wave's 39 new `pilot_compute/mod.rs` tests). Run 2, after the
  clippy + CL-semantics fixes → identically 3182 passed, 0 failed, 14 ignored. Run 3, after the
  33 new `v06_work_inventory.rs` probes + their own 33 new tests landed → 3215 passed (+33), 0
  failed, 14 ignored.
- `cargo test --locked --no-fail-fast -j 6`: run 1 (before any fix) → exit 0, all 589 suites green,
  8615 passed, 0 failed, 67 ignored. Run 2 (after the clippy + CL-semantics fixes) → identically
  8615/0/67/589. Run 3 (after the 33 new probes + tests, the definitive final run) →
  8648 passed (+33, the same 33 new `v06_work_inventory.rs` tests), 0 failed, 67 ignored, across 589
  suites, exit 0.
- `cargo clippy --locked --tests -j 6`: run 1 found 1 `clippy::empty_line_after_doc_comments`
  warning (a section-banner comment mistakenly written as a doc comment `///` immediately above
  `cyphermage_analyze_scroll_bonus`, with a blank line separating the two) — fixed by converting the
  banner to a plain `//` comment block. Runs 2 and 3 (including after the probe additions): 0
  warnings.
- F1/`shape_ledger.py` pin: unchanged, 5155 → 5155 (the standing lib-test `f1_population_matches_
  the_current_true_formula_bearing_count_not_the_stale_sd32_census` passed unchanged throughout —
  this wave's own new engine code, both `pilot_compute/mod.rs` and the census probes in
  `v06_work_inventory.rs`, touches no corpus data, so no drift is expected).
- `completion_atlas.py --check`: before `DONE 25474 / D 2382 / V 349`, after
  `DONE 25603 / D 2253 / V 349` (population 49438 unchanged both snapshots).
- `denominator_gate.py --check`: `violations=0`.

Independently re-derived via a direct Python `id`→`status` join over the same before/after
`docs/work-inventory.json` snapshots (not just `completion_atlas.py`'s own bucket counts): exactly
129 units changed status, all from `engine-does-not-hold` to `grounded`, zero collateral movement
(id-set unchanged at 49438 both sides).
