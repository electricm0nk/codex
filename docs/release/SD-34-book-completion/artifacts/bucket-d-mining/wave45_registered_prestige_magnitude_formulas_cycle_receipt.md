# Cycle — SD-34 wave 45 — Phrenic Slayer Favored Enemy: 32 of 32 closed (registered prestige class, magnitude-only remainder)

- **Commit SHA:** `c43ffaffa2`
- **Files touched:** `src/rules_core/pilot_compute/mod.rs` (1 new class-id const
  `PHRENIC_SLAYER_CLASS_ID`, 1 new shared `(slug, display name)` const table
  `PHRENIC_SLAYER_FAVORED_ENEMY_MEMBERS` (31 entries), 1 new pure formula function
  `phrenic_slayer_favored_enemy_bonus`, 1 new unconditional-on-chassis grounding function
  `ground_phrenic_slayer_class_features`, 1 new call site inside `compute_pilot_base_chassis`,
  1 new test module `wave45_phrenic_slayer_favored_enemy_tests`, 5 tests), `src/bin/v06_work_inventory.rs`
  (1 new `EngineFacts` field `phrenic_slayer_favored_enemy_wired`, 1 new probe function
  `probe_phrenic_slayer_favored_enemy_wiring` importing the shared member list from
  `pilot_compute` rather than re-declaring it, 1 new wiring call site, 1 new `classify()`
  early-return check, 4 new classify()-level tests + 2 negative controls), `scripts/completion_atlas.py`
  (10 bucket citation-pin re-derivations — this cycle's own insertions shifted every downstream
  line number, same pattern wave 43/44 hit), `scripts/shape_engine_boundary.py` /
  `scripts/missing_engine_tables.py` / `scripts/tests/test_shape_engine_boundary.py` (citation-pin
  re-derivations for the same reason — wave 44 had already flagged these two scripts as
  "nobody watches this, not wired into `verify.sh`"; fixed the citations this cycle's own edits
  require, left the pre-existing, unrelated population-count drift named, not fixed, same as
  wave 44's own choice), `docs/work-inventory.json` (regenerated via the guarded path),
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (a `--check`
  re-run's own artifact), this receipt, `progress.md`, `decisions.md`, `kanban.md`.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --unified=0 HEAD -- src/rules_core/
  pilot_compute/mod.rs src/bin/v06_work_inventory.rs`, no `sd[0-9]+_`/`SD[0-9]+_`/`t_[0-9a-f]{8,}`
  hits outside this wave's own house-style `wave45_*` module name, matching the file's own
  existing `wave4[1-4]_*` convention).
- **Wired-integration audit result:** `OK_NO_TOKENS` (0 hits for `placeholder`/`STUB`/`MOCK`/
  `not yet implemented`/`fixme`/`hack` in this cycle's own diff).
- **Acceptance criterion (verbatim from this wave's dispatch brief):** re-derive sub-mechanism-5's
  current population fresh (not trusted from any prior wave's count), cross-reference every unit
  against the now-74-entry `prestige-class-entry-requirements.json` registry to separate
  registered-class-magnitude-only units from genuinely out-of-scope ones, close a safely
  verifiable batch of the registered set with real pure-formula + reachability tests, verify with
  both `cargo test --locked --lib` and the full `cargo test --locked --no-fail-fast` integration
  suite (run a SECOND time end-to-end if any code changes after the first run), run the guarded
  regen, re-derive the F1/shape_ledger pin if the change is F1-shaped, and report real before/after
  bucket deltas plus what remains named for a future wave.

## Fresh population re-derivation (not trusted from any prior wave's count)

**Method:** `python3 -c "import json; d=json.load(open('docs/work-inventory.json')); print(sum(1
for u in d['units'] if 'class_feature_of_unmodelled_corpus_class' in (u.get('evidence') or '')))"`
against this cycle's own pre-edit HEAD (`4e96826b5e`, wave 44's own wave-end-gate commit).

**Result: 686**, not 634 (wave 37/38's own count) and not 699 (wave 43/44's own count) — both
prior figures are stale; the registry fixture and classifier have both changed materially since
either was written (wave 44's own census-script fix alone recovered 144 fixture entries and moved
16 units off this population). 686 is the real, live count at this cycle's own start.

## Cross-reference against the 74-entry prestige-class registry fixture

**Method:** for each of the 686 units, extract the evidence suffix after
`class_feature_of_unmodelled_corpus_class:` (a slug) and check membership against
`{e['class_id'].split(':',1)[1] for e in json.load(open('tests/fixtures/rules_core/
prestige-class-entry-requirements.json'))['entries']}` (74 entries).

**Result: 598 registered / 88 not registered**, summing exactly to 686 (verified by direct count,
not by subtraction alone).

**The 88 not-registered units, named by slug (out of scope for this wave, not attempted):**
`psychic_detective` (18 — Occult Adventures Expanded Arcana choice-pool slot, already named
unclosed by wave 44, confirmed still genuinely more involved than a simple owner-reroute),
`animal` (17 — ACG Shaman's Spirit Animal choice, an unrelated corpus "Animal" bestiary
pseudo-class collision, not a prestige class at all), `eidolon` (16 — Summoner's Eidolon
companion-progression population, the SAME 16-unit "1 First Worlder trigger + 15 Broodmaster
multi-companion progressions" population wave 44 already named and left open, confirmed still
unclosed and unchanged), `phantom` (9 — Ultimate Intrigue's Phantom Thief rogue talents,
colliding with the bestiary's unmodelled "Phantom" pseudo-class, not a prestige class), `plant`
(9 — Ultimate Wilderness's Plant Master archetype focus choices, colliding with an unmodelled
"Plant" pseudo-class), `undead` (8 — a mix of ACG's Undead Savant Subschool and APG's Undead
Scourge/domain-shaped records, colliding with an unmodelled "Undead" pseudo-class; NOT the same
records wave 44 already closed under Wizard's Necromancy School), `dragon` (8 — a mix of APG's
Order of the Dragon's own "Aid Allies" feature — a DIFFERENT feature than the "Survival bonus"
wave 43 already closed — and Ultimate Magic's unrelated Dragon Shaman class, both colliding with
the bestiary's unmodelled "Dragon" pseudo-class), `gifted_blade` (3 — confirmed by wave 44 to
never carry a `TYPE:...Prestige` line anywhere in the oracle, correctly excluded from the
registry). None of these 8 slugs is a registered prestige class; every one is either a genuinely
harder open population already named by a prior wave, or an unrelated corpus collision with a
same-named bestiary/pseudo-class record. Named here, not attempted.

## The registered set (598 units): scope chosen for this cycle

598 units span dozens of registered prestige classes with widely varying per-class population
size (from 45 units for Inner Sea Magic's Divine Scion down to 1 for Ultimate Intrigue's
Sentinel). Rather than attempt a large, heterogeneous slice in one cycle, this wave closes ONE
class's full magnitude-only remainder that (a) is entirely self-contained (needs only the
class's own raw level, no cross-class prime-stat or parent-class resolution), (b) matches this
bundle's own precedented "favored-enemy-style choice" shape (`ground_pathfinder_delver_class_
features`'s PaDFE precedent, wave 44), and (c) was independently, byte-for-byte cross-checked
against the real PCGen oracle (`~/workspace/repos/pcgen/data/pathfinder/dreamscarred_press/
ultimate_psionics/up_abilities_class.lst`), not merely the ingested corpus JSON:

**Ultimate Psionics Phrenic Slayer's Favored Enemy record — 32 of the class's 43 sm5 units.**

- **The base record** (`Phrenic Slayer ~ Favored Enemy`, `up_abilities_class.lst:1326`):
  `DEFINE:SlayerFavoredEnemy|0` / `BONUS:VAR|SlayerFavoredEnemy|2*floor((2+PhrenicSlayerLVL)/3)`.
  `PhrenicSlayerLVL = CL` (`up_classes.lst:932`, the class's own raw level — no prime-stat
  resolution needed, unlike four of this class's OTHER remaining features). Granted from class
  level 1 (`up_classes.lst:935`).
- **31 creature-type sub-records** (`Phrenic Slayer Favored Enemy ~ <Type>`,
  `up_abilities_class.lst:1338-1368`, one file each under `data/corpus/ultimate_psionics/
  class_feature/phrenic_slayer_favored_enemy/`): every one carries no own `DEFINE`/`BONUS` token,
  only a `%1` DESC substitution and an `ASPECT:Ability Benefit|+%1|SlayerFavoredEnemy` referencing
  the SAME shared variable the base record defines — verified directly against every one of the
  31 real `.lst` lines (`grep -n 'KEY:Phrenic Slayer Favored Enemy ~'
  up_abilities_class.lst` → exactly 31 hits, byte-cross-checked against the committed corpus
  JSON's own `data.key`/`data.name` fields, which are identical). This is exactly the
  "favored-enemy-style choice" shape the brief names as precedented
  (`ground_pathfinder_delver_class_features`'s PaDFE bonus, wave 44) — a single shared magnitude,
  gated only by the base record's own grant level, with N creature-type display sub-records that
  never carry an independent formula.

**A real miscount caught and corrected during this cycle's own investigation:** the corpus
directory was first (wrongly) counted as holding 30 creature-type files; a direct `ls | wc -l`
and a second independent cross-check against the real PCGen oracle both confirm **31**, not 30
(the initial test module's own assertion caught this immediately — RED for the right reason,
fixed before any test was allowed to pass). Named here per this bundle's own standing discipline
of reporting a caught error, not just its fix.

**What this class's remaining 11 units are, and why they are NOT in this cycle's scope:** Advance
Astral Suit / Advance Mind Blade / Advance Manifesting (and their 4 two-and-three-way
combinations) key off `ABILITYPOOL|Manifesting Level Advancement` and cross-class variables
(`AegisCL`, `MndBladeLVL`) that depend on which parent psionic class (Aegis, Soulknife, Psion,
etc.) granted the character's entry into Phrenic Slayer — real, separate subsystem modeling this
cycle does not attempt. Brain Nausea, Lucid Buffer, Power Resistance, and Rebound Attack all key
off `PhrenicSlayerPrimeStat` (`BONUS:VAR|PhrenicSlayerPrimeStat|WIS`, a class-specific "prime
manifesting ability" fact that is itself gated on the character's entry-class choice) — a
genuinely different modelling question than the raw-level-only Favored Enemy record this cycle
closes. Left named, not attempted, for a future wave.

## Corpus re-verification (read directly, cross-checked against the real oracle, not trusted from
any summary)

Every one of the 32 target units' corpus JSON records
(`data/corpus/ultimate_psionics/class_feature/phrenic_slayer/favored_enemy.json` and
`data/corpus/ultimate_psionics/class_feature/phrenic_slayer_favored_enemy/*.json`) was read in
full, AND independently cross-checked against the real, non-ingested-corpus oracle source
(`~/workspace/repos/pcgen/data/pathfinder/dreamscarred_press/ultimate_psionics/
up_abilities_class.lst:1326,1338-1368` and `up_classes.lst:931-936`) — a stronger bar than the
committed-corpus-JSON-only verification prior waves used, since the oracle is the independent
upstream source the corpus was ingested from. Both sources agree byte-for-byte on the formula,
the grant level, and every one of the 31 creature-type display strings.

## Before/after bucket movement

`python3 scripts/completion_atlas.py --check` at this cycle's own pre-edit HEAD (`4e96826b5e`):
`population=49438 unclassified=0 overlap=0`, `DONE: 25375`, `D: 2493` (matching wave 44's own
wave-end-gate figures exactly).

| Unit id | corpus_key | status (pre-cycle) |
|---|---|---|
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy` | Phrenic Slayer ~ Favored Enemy | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_aberration` | Phrenic Slayer Favored Enemy ~ Aberration | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_animal` | … ~ Animal | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_construct` | … ~ Construct | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_dragon` | … ~ Dragon | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_fey` | … ~ Fey | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_humanoid_aquatic` | … ~ Humanoid (Aquatic) | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_humanoid_dwarf` | … ~ Humanoid (Dwarf) | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_humanoid_elf` | … ~ Humanoid (Elf) | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_humanoid_giant` | … ~ Humanoid (Giant) | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_humanoid_gnoll` | … ~ Humanoid (Gnoll) | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_humanoid_gnome` | … ~ Humanoid (Gnome) | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_humanoid_goblinoid` | … ~ Humanoid (Goblinoid) | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_humanoid_halfling` | … ~ Humanoid (Halfling) | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_humanoid_human` | … ~ Humanoid (Human) | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_humanoid_orc` | … ~ Humanoid (Orc) | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_humanoid_reptilian` | … ~ Humanoid (Reptilian) | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_magical_beast` | … ~ Magical Beast | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_monstrous_humanoid` | … ~ Monstrous Humanoid | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_ooze` | … ~ Ooze | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_outsider_air` | … ~ Outsider (Air) | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_outsider_chaotic` | … ~ Outsider (Chaotic) | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_outsider_earth` | … ~ Outsider (Earth) | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_outsider_evil` | … ~ Outsider (Evil) | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_outsider_fire` | … ~ Outsider (Fire) | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_outsider_good` | … ~ Outsider (Good) | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_outsider_lawful` | … ~ Outsider (Lawful) | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_outsider_native` | … ~ Outsider (Native) | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_outsider_water` | … ~ Outsider (Water) | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_plant` | … ~ Plant | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_undead` | … ~ Undead | engine-does-not-hold |
| `ultimate_psionics:class_feature:phrenic_slayer_favored_enemy_vermin` | … ~ Vermin | engine-does-not-hold |

All 32 pre-cycle: `status="engine-does-not-hold"`,
`evidence="class_feature_of_unmodelled_corpus_class:phrenic_slayer"`.

**Post-regen:** `python3 scripts/completion_atlas.py --check` on the regenerated
`docs/work-inventory.json`: `population=49438 unclassified=0 overlap=0 citation_failures=0`,
`DONE: 25375→25407 (+32)`, `D: 2493→2461 (−32)`, every other bucket unchanged. Independently
re-derived via a direct Python `id`→`status` join over both the pre- and post-regen inventory
snapshots (not just `--check`'s own summary): pre/post population both 49438, 0 added, 0 removed,
**exactly 32 units changed status, zero collateral movement** — all 32 the exact ids named above,
all landing `status="grounded"`,
`evidence="phrenic_slayer_favored_enemy_probe_observed_a_real_computed_magnitude"`.

## Tests

`cargo check --locked --lib -j 6` → exit 0. `cargo check --locked --bin v06_work_inventory` → exit
0. `cargo test --locked --lib -j 6 wave45_phrenic_slayer_favored_enemy_tests` → 5 passed, 0
failed. `cargo test --locked --bin v06_work_inventory phrenic_slayer` → 4 passed, 0 failed.
`cargo test --locked --lib -j 6` (full lib suite) → **3095 passed, 0 failed, 14 ignored** (up from
the standing 3090 baseline by exactly this cycle's 5 new tests). `cargo test --locked
--no-fail-fast -j 6` (full workspace, re-run end-to-end after this cycle's own git-status
inheritance from a prior interrupted attempt at this same wave — no code changed between this run
and the lib-only run above, so a second full run was not additionally required, but one was run
anyway as the mandatory full-integration check) → **8504 passed, 0 failed, 67 ignored, across 590
suites, exit 0** (grepped and summed directly from the run's own log, not read off a summary
line: `grep -c '^test result:'` → 590, `grep -oE '[0-9]+ (passed|failed|ignored)'` summed → 8504 /
0 / 67; zero `FAILED`/`^error` hits anywhere in the log). Up from the standing 8495 baseline by
+9 — the same +5 lib tests (counted again, root-full runs the lib suite too) plus +4 new
`v06_work_inventory.rs` bin tests, reconciled exactly, no unexplained residual (unlike wave 44's
own +11 shared-checkout mystery). This wave's own guarded regen (below) had already completed,
against this same tree, before this full-suite run started; both `python3
scripts/completion_atlas.py --check` and a fresh `python3 scripts/shape_ledger.py` re-run were
independently re-derived against the already-regenerated `docs/work-inventory.json` after this
full-suite run finished, both matching the figures reported below byte-for-byte — an independent
cross-check of the regen's own correctness, not a re-run of the regen itself.

## Guarded regen

First attempt would have refused (the same guard every prior wave hits) had the two prerequisite
reports not been generated first. Generated both fresh against this cycle's own tree (release
profile):

- `cargo run --locked --release --bin corpus_literal_sweep -- --json-out <path>` →
  `48706 records examined of 51476 read, 413314 tokens compared (9 synthesized), 51463 digests
  checked, 0 findings` / `3138 tokens exempted under decisions.md §24 redaction across 1058
  codex_generated_name records` / **CLEAN** — byte-identical to wave 44's own pre-cycle figures (no
  `data/corpus/**` file touched this cycle, confirmed via `git diff --stat -- data/corpus/` empty).
- `cargo run --locked --release --bin derived_evaluator_fixture_check -- --json-out <path>` →
  `1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0 not ingested` — also byte-identical.

Re-ran `cargo run --locked --bin v06_work_inventory` with both reports set (`CORPUS_LITERAL_SWEEP_
REPORT`/`DERIVED_FIXTURE_CHECK_REPORT`) — completed cleanly (exit 0), no refusal,
`docs/work-inventory.json` regenerated. `git status --porcelain -- docs/work-inventory.json` shows
`M` (modified, not stale).

## F1/shape_ledger pin

`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus`
re-derived against the post-regen `docs/work-inventory.json`: **F1 = 5196, unchanged from wave
44's own pin**. Verified per-id, not assumed: of the 32 units this cycle closed, exactly **0** are
F1-shaped — 31 are `F0` (no `DEFINE`/`BONUS` token at all, the 31 creature-type sub-records, each
carrying only an `ASPECT` reference to the base record's shared variable) and 1 is `F5` (clamped/
capped per-level scaling — the base record's own `2*floor((2+PhrenicSlayerLVL)/3)`).

**A real, honest surprise found while re-deriving this pin, named not fixed:** `shape_ledger.py`'s
own "not-done" population (`coverage_ledger.py::not_done_population`, itself keyed on
`scripts/observer/pf1e_dashboard_producer.py::doneness_verdict`) is a DIFFERENT, older, stricter
doneness instrument than SD-34's own `completion_atlas.py` buckets — its own table maps
`wiring_class in ("display","static","derived")` + `status == "grounded"` to `HELD`, not `DONE`
(only `wiring_class == "computed"` + `grounded` reaches `DONE` there; `literal-verified`/`fixture-
verified` reach `DONE` for `static`/`derived` too). All 32 of this cycle's units carry
`wiring_class` `display` (31) or `derived` (1), so none of them leave that OTHER instrument's
"not-done" population even though every one is genuinely `DONE` in SD-34's own atlas sense —
`shape_ledger.py`'s total population (25751) and every family count (including F1) are therefore
BYTE-IDENTICAL before and after this cycle's regen, which is the honest, correct consequence of
two instruments drawing the "done" line in different places, not a defect in either. Named here so
a future wave does not mistake "shape_ledger's population didn't move" for "this wave changed
nothing" when cross-checking against `completion_atlas.py`'s own (correctly moved) DONE/D counts.

## Findings for a future wave (named, not fixed this cycle)

1. **`scripts/shape_engine_boundary.py`'s own population-count pin remains stale**, now by an even
   larger margin than wave 44 measured (wave 44: 9475 pinned vs ~9012 live, 463-unit drift; this
   cycle: 9475 pinned vs 8996 live). This is the SAME pre-existing, unrelated drift wave 44 named
   and explicitly declined to fix (no established multi-wave update convention for this pin, and
   the test carrying it is not wired into `verify.sh`) — not fixed here either, for the identical
   reason.
2. **Sub-mechanism-5's remaining population after this cycle: 654 units** (686 total minus this
   cycle's 32 closed), split **566 registered** (across dozens of prestige classes, the highest-
   value remaining target — every one already benefits from the SAME `chassis_supported(...) ||
   prestige_class_entry_gate::is_registered(...)` grant-level mechanism firing; only per-feature
   magnitude formulas are missing) and **88 not registered** (named above, each either a genuinely
   harder already-known open population or an unrelated bestiary/pseudo-class name collision).
3. **Phrenic Slayer's own remaining 11 units** (Advance Astral Suit/Mind Blade/Manifesting and
   their combinations, Brain Nausea, Lucid Buffer, Power Resistance, Rebound Attack) need real
   cross-class (`PhrenicSlayerPrimeStat`, parent-class-dependent) modeling this cycle did not
   attempt — named above, left for a future wave.
4. **Summoner Eidolon's 16-unit population** (1 First Worlder trigger + 15 Broodmaster
   multi-companion progressions) and **Psychic Detective's Expanded Arcana choice-pool record**
   remain open, unchanged from wave 44's own naming.

## Build scope verified

- `cargo check --locked --lib -j 6` → exit 0.
- `cargo check --locked --bin v06_work_inventory` → exit 0.
- `cargo test --locked --lib -j 6 wave45_phrenic_slayer_favored_enemy_tests` → 5 passed, 0 failed.
- `cargo test --locked --bin v06_work_inventory phrenic_slayer` → 4 passed, 0 failed.
- `cargo test --locked --lib -j 6` (full lib suite) → 3095 passed, 0 failed, 14 ignored (up from
  the standing 3090 baseline by exactly this cycle's 5 new tests).
- `cargo test --locked --no-fail-fast -j 6` (full workspace) → 8504 passed, 0 failed, 67 ignored,
  590 suites, exit 0 (up from the standing 8495 baseline by exactly +9 = the same 5 lib tests
  counted again + 4 new bin tests).
- `scripts/verify-baselines.env` updated: `BASELINE_ROOT_LIB_TESTS` 3090→3095,
  `BASELINE_ROOT_FULL_TESTS` 8495→8504 (dated entry appended following the file's own convention).
- `python3 scripts/completion_atlas.py --check` (post-regen, re-run independently after the full
  suite): `population=49438 unclassified=0 overlap=0 citation_failures=0`, bucket counts match
  this receipt's own before/after table exactly.
- `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus`
  (re-run independently after the full suite): `F1 = 5196`, matching this receipt's own F1/pin
  section exactly.
- `python3 scripts/denominator_gate.py --check`: `files_checked=180 violations=0`.
- Desktop crate (`apps/desktop/src-tauri`) — not run: `git diff --stat -- apps/desktop/` empty, no
  file under `apps/desktop/` touched this cycle.
