# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`class_feature_option_pool_record_with_magnitude_not_held_by_engine` mechanism, cycle 2)

- **Commit SHA:** `b93fb51a3e` (parent `651ea3e145`)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "**970** Core Rulebook units whose
  table exists but which are not in it. Evidence: the atlas reporting bucket B at zero for
  `core_rulebook`, and the mechanism that placed them named — by mechanism, not per record."
  (970 is `epic-breakdown.md`'s own stale, whole-bucket figure; this cycle owns exactly ONE of
  the nine mechanisms `decisions.md §14` names, and does not itself close the bucket.)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS` (the wider file-touch-set audit surfaces 6
  matches of the word "placeholder" — all from an EARLIER, already-committed cycle's own
  `vacuous-placeholder` sub-cause fix, real PCGen "no selection" CHOOSE-menu rows, not a stub
  this cycle introduced; reviewed, not self-healed, since they are legitimate prose, not code)
- **Status:** partial

Bucket B for `core_rulebook` is nine distinct mechanisms (`decisions.md §14`). This cycle owns
exactly one: `class_feature_option_pool_record_with_magnitude_not_held_by_engine` — continuing
the prior cycle's own work (`AT-34-E3-001_class_feature_option_pool_with_magnitude_cycle_receipt.md`,
333 -> 328).

## Population, re-derived (not quoted)

```
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
u = [x for x in d['units'] if x['book']=='core_rulebook' and x['status']=='engine-does-not-hold'
     and x['evidence']=='class_feature_option_pool_record_with_magnitude_not_held_by_engine']
print(len(u))
"
```
Before this cycle: **328** of 1,006 `core_rulebook` bucket-B units (re-derived at this cycle's
own start SHA `651ea3e145`, matching the dispatch brief's own 328 figure exactly — no drift to
correct). Corpus-wide (37 books) before: **3,047** across 21 books (same command, no `book`
filter).

## Mechanism-specific direction followed

The dispatch named "the Domain Power `CLASS_FEATURE_POOLS` registration gap" and warned that
registering the entry alone would not close mechanism 2's Domain Power units without additional
formula work. That specific registration lever was already investigated and exhausted by the
PRIOR cycle in this wave (its own receipt's "Discovery" section: registering `"Domain Power"`
in `CLASS_FEATURE_POOLS` would only reclassify records between bucket-B mechanisms, never ground
one, because `class_feature_exact_suffix_grounded`'s `group == owner` guard can never pass for a
group text that never equals a class name). This cycle re-confirmed that conclusion still holds
(no code changed that guard) and did **not** re-attempt it. Instead, per the same underlying
requirement — "verify what registration actually moves... before assuming it is the lever" — this
cycle investigated TWO alternative sub-causes of the 328-unit remainder the prior cycle named,
before committing to one:

1. **`Domain Base` (33 units) — investigated, NOT closed.** Each `"Domain Base ~ <domain>"`
   record (`data/corpus/core_rulebook/class_feature/domain_base/*.json`) carries `description:
   null` and a `Domain<X>DC|10+(Domain<X>LVL/2)+CHA` formula that is genuinely computable
   (identical shape across all 33 domains, and `domain_power.rs`'s own `parse_pcgen_expr`/
   `eval_expr`/`domain_power_env` already handle it generically with zero new per-domain
   content). But tracing where this DC value is actually CONSUMED found that no domain power
   this engine's real `compute_pilot_base_chassis` pipeline ever grants uses it: the DC only
   matters for a save-requiring (enemy-facing) domain power, and `domain_power.rs`'s own module
   doc names Evil/Darkness/Madness (the enemy-facing shape) as DELIBERATELY excluded from
   `DOMAIN_POWER_CATALOG` for a different, real reason (self-application-safety, not a formula
   gap). So no explanation id anywhere in the real pipeline would ever carry this DC — a probe
   that computed it in isolation (the same generic formula interpretation, run standalone rather
   than through the real character-compute pipeline) would be exactly the "plausible-looking but
   not actually observed by the engine" shape `probe_domain_power_effect_wiring`'s own doc
   comment warns against, and would misrepresent an isolated formula evaluation as "the engine
   computes this." Closing this sub-cause for real needs new production wiring (an actual
   save-DC consumer for at least one enemy-facing domain power), not a probe — named here as
   next-cycle work, not attempted this cycle to avoid shipping an unobserved claim.
2. **`Weapon Training <tier> <group>` (52 units) — investigated and CLOSED, 4 of 52.** Traced
   generically (see "The real fix" below).

## The real fix: a dedicated, verified probe (Weapon Training sub-cause)

Read `fighter_weapon_training_attack_bonus` (`src/rules_core/pilot_compute/mod.rs`) directly
before building anything: it hardcodes exactly ONE canonical weapon group per training tier
(tier 1 -> Heavy Blades, tier 2 -> Bows, tier 3 -> Polearms, tier 4 -> Hammers — the deterministic
Longsword fixture's own group, plus three canonical explanation-only companions), and returns 0
/ emits no explanation for any other group. Also confirmed live: `canonical_seeds_for("fighter")`
never seeds ANY `choice:fighter_weapon_training_group*` selection at all (`grep` returns nothing),
so the standard per-class sweep that fills `EngineFacts::explanation_ids` never observes even
tier 1's own canonical selection — this is the exact same "canonical sweep never selects the
non-default choice" gap `probe_domain_power_effect_wiring` closed for Domain Power.

Built `probe_fighter_weapon_training_wiring` (`src/bin/v06_work_inventory.rs`), the same shape as
`probe_domain_power_effect_wiring`: explicitly selects each of the engine's own 4 hardcoded
canonical `(tier, group, choice id, selection)` tuples — exposed read-only from
`pilot_compute::mod` via a new `pub fn fighter_weapon_training_canonical_catalog()` bridge (no
behavior change to the weapon-training computation itself; every tuple reuses the SAME constant
the real computation already reads, never a re-typed copy) — sweeps `SWEEP_LEVELS` through the
real `compute_pilot_base_chassis` pipeline, and keeps only the `(tier, group)` pairs whose own
explanation id was genuinely observed. `classify()`'s `Kind::ClassFeature` arm gained one new
early check, mirroring the Domain Power check exactly: parses `"Weapon Training <tier> <group>"`
corpus keys and grounds only when `facts.fighter_weapon_training_wired` contains that exact
`(tier, group)` pair. Every other one of the corpus's 52 weapon-training records — including 48
whose group/tier combination the engine simply never computes — falls through completely
unaffected.

## RED -> GREEN

RED (confirmed for the intended reason): before this cycle's `classify()` edit, `"Weapon Training
1 Blades Heavy"` fell through the same owner-resolution path every other weapon-training record
does (the corpus key carries no class name and no `" ~ "` separator at all, so
`class_feature_owner` and both its fallbacks can never resolve an owner), landing at
`engine-does-not-hold` / `class_feature_option_pool_record_with_magnitude_not_held_by_engine` —
the live corpus data itself is the RED proof (`docs/work-inventory.json` at this cycle's start
SHA lists exactly this verdict for all 4 of the engine's own canonical tier/group pairs, not just
the 48 the engine genuinely never computes).

GREEN: two new tests added to `class_feature_text_complete_rung_tests`:
- `a_fighter_weapon_training_record_the_probe_observed_reaches_grounded` — `facts.
  fighter_weapon_training_wired` seeded with `(1, "Blades Heavy")`; asserts `status ==
  "grounded"`, `evidence == "fighter_weapon_training_probe_observed_a_real_computed_magnitude"`.
- `a_fighter_weapon_training_record_the_probe_never_observed_is_unaffected` — negative control,
  `"Weapon Training 1 Axes"` (a real record no canonical selection ever covers), empty probe
  result; asserts the pre-existing `engine-does-not-hold` /
  `class_feature_option_pool_record_with_magnitude_not_held_by_engine` verdict is unchanged.

```
cargo test --locked --bin v06_work_inventory fighter_weapon_training
running 2 tests
test class_feature_text_complete_rung_tests::a_fighter_weapon_training_record_the_probe_observed_reaches_grounded ... ok
test class_feature_text_complete_rung_tests::a_fighter_weapon_training_record_the_probe_never_observed_is_unaffected ... ok
test result: ok. 2 passed; 0 failed; ...
```

Live regen confirms exactly the predicted 4 records moved:

```
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
for k in ['Weapon Training 1 Blades Heavy','Weapon Training 2 Bows','Weapon Training 3 Pole Arms','Weapon Training 4 Hammers']:
    u=[x for x in d['units'] if x['book']=='core_rulebook' and x.get('corpus_key')==k]
    for x in u: print(x['corpus_key'], x['status'], x['evidence'])
"
Weapon Training 1 Blades Heavy grounded fighter_weapon_training_probe_observed_a_real_computed_magnitude
Weapon Training 2 Bows grounded fighter_weapon_training_probe_observed_a_real_computed_magnitude
Weapon Training 3 Pole Arms grounded fighter_weapon_training_probe_observed_a_real_computed_magnitude
Weapon Training 4 Hammers grounded fighter_weapon_training_probe_observed_a_real_computed_magnitude
```

## Files touched

- `src/rules_core/pilot_compute/mod.rs` — new `pub fn fighter_weapon_training_canonical_catalog()`.
  No change to any existing function's behavior or any existing constant's value.
- `src/bin/v06_work_inventory.rs` — new import of that function; new `EngineFacts::
  fighter_weapon_training_wired: BTreeSet<(u8, String)>` field, populated in `gather_engine_facts`
  via new `probe_fighter_weapon_training_wiring`; one new early-return branch in `classify()`'s
  `Kind::ClassFeature` arm; two new tests.
- `scripts/completion_atlas.py`, `scripts/missing_engine_tables.py` — this cycle's own ~90-line
  net insertion into `v06_work_inventory.rs` and `mod.rs` shifted every hardcoded `file:line`
  citation both files carry (the brief's own named hazard). Re-derived each one fresh by grepping
  the exact quoted string each citation targets (never a flat offset), confirmed by re-running
  both `--check` gates: `citation_failures=0` on both.
- `docs/work-inventory.json` — regenerated at HEAD, guarded path (`CORPUS_LITERAL_SWEEP_REPORT`/
  `DERIVED_FIXTURE_CHECK_REPORT` set from this cycle's own fresh sweep/fixture-check runs, no
  `--allow-stamp-loss` used or needed).
- `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`,
  `.../missing-engine-tables.json` — regenerated by their own `--check` runs at HEAD.
- `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_with_magnitude_cycle_receipt_2.md`
  (this file).
- `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`.

## Discoveries

No wrong prior claim found this cycle (the prior cycle's own 328 figure and its "registration
alone does not ground" finding both re-verified true, unchanged) — no `correction` retro event
filed. The `Domain Base` investigation above is recorded as a real, verified discovery (a
sub-cause this cycle's own dispatch invited investigation of, found NOT closable by a probe
without shipping an unobserved claim) so a later cycle does not repeat the investigation from
scratch.

`apps/desktop/src-tauri`: 26 of 548 tests FAIL (`companion_catalog`, `race_trait_picker`,
`reach_gate` modules) — confirmed pre-existing and unaffected by this cycle: 522 passed, 26
failed, identical to the prior cycle's own independently-confirmed count at a throwaway
pre-cycle checkout. Outside SD-33's documented 29-suite/46-failure baseline (a distinct,
already-existing, larger regression this cycle did not introduce and was not asked to fix).

## Figures + their re-derive commands

| Figure | Value | Command | Denominator |
|---|---|---|---|
| Mechanism population before | 328 | `python3 -c "..."` (above) against `docs/work-inventory.json` at start SHA `651ea3e145` | of 1,006 `core_rulebook` bucket-B units |
| Mechanism population after | 324 | same command against the regenerated `docs/work-inventory.json` | of 1,006 `core_rulebook` bucket-B units (pre-cycle denominator) |
| Corpus-wide mechanism population before | 3,047 | same command, no `book` filter | of 49,438 units |
| Corpus-wide mechanism population after | 3,043 | same command, no `book` filter | of 49,438 units (delta -4, exactly this cycle's 4 CRB closures — no other book's weapon-training records were affected) |
| `core_rulebook` bucket B (whole book, all 9 mechanisms) before | 754 | `python3 scripts/completion_atlas.py --book core_rulebook --check` | of 6,701 `core_rulebook` units |
| `core_rulebook` bucket B (whole book, all 9 mechanisms) after | 750 | same | of 6,701 `core_rulebook` units |
| Units closed | 4 | `Weapon Training {1 Blades Heavy, 2 Bows, 3 Pole Arms, 4 Hammers}`, verified individually against `docs/work-inventory.json` | of 328 |
| `completion_atlas.py --check` (corpus-wide) | `unclassified=0 overlap=0 citation_failures=0` | `python3 scripts/completion_atlas.py --check` | of 49,438 |
| `missing_engine_tables.py --check` | `citation_failures=0` | `python3 scripts/missing_engine_tables.py --check` | of 449 |
| `corpus_literal_sweep` | `48708 examined of 51482 read, 0 findings` (before and after — no `data/corpus/**` file touched this cycle) | `"$CARGO_TARGET_DIR/release/corpus_literal_sweep" --json-out ...` | of 51,482 |
| `derived_evaluator_fixture_check` | `1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0 not ingested` | `"$CARGO_TARGET_DIR/release/derived_evaluator_fixture_check" --json-out ...` | of 2,580 fixture rows |
| `cargo test --locked --bin v06_work_inventory` | `387 passed; 0 failed` | `cargo test --locked --bin v06_work_inventory` | of 387 |
| `cargo test --locked --lib` | `2877 passed; 0 failed; 14 ignored` | `cargo test --locked --lib` | of 2891 |
| `cargo test --locked --no-run` (workspace) | exit 0 | `cargo test --locked --no-run` | — |
| `apps/desktop/src-tauri` `cargo test --locked` | `522 passed; 26 failed` (pre-existing, unaffected) | `cd apps/desktop/src-tauri && cargo test --locked` | of 548 |
| `box_ledger.py --check` | 6 stale-count WARNINGs (inherited SD-33 drift, unowned by SD-34, unaffected by this cycle), `uncovered=20085 oracle_disagreement=0 unverifiable_done=0 stale=False` | `python3 scripts/box_ledger.py --check` | of 49,438 |
| `denominator_gate.py --check` | `files_checked=15 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` | of 15 files |

## Row-count command output (this cycle's own artifact)

```
$ python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
u = [x for x in d['units'] if x['book']=='core_rulebook' and x['status']=='engine-does-not-hold'
     and x['evidence']=='class_feature_option_pool_record_with_magnitude_not_held_by_engine']
print(len(u))
"
324
```
Not zero. **Status: partial**, remainder named below.

## Build scope verified

`cargo test --locked --no-run` exit 0, workspace-wide, run at SHA (this cycle's own commit, see
`commit_sha` in the structured return). `cargo test --locked --lib`: 2877 passed, 0 failed, 14
ignored. `apps/desktop/src-tauri` (separate cargo workspace): `cargo test --locked`, 522 passed,
26 failed — confirmed pre-existing (see Discoveries), unaffected by this cycle.

## Sweep population

`corpus_literal_sweep`: 48708 examined before -> 48708 examined after, delta 0 (no
`data/corpus/**` file touched or regenerated this cycle — only `src/rules_core/` and `src/bin/`
Rust source).

## Movement, four buckets

- **Closure:** 4 — `Weapon Training {1 Blades Heavy, 2 Bows, 3 Pole Arms, 4 Hammers}` ->
  `DONE` (`grounded`, `wiring_class: computed`). All four carry a REAL, live-computed magnitude
  (the Fighter's own attack/damage-roll bonus for that weapon group) a player's sheet already
  renders, proven by a live probe against the real compute pipeline, not asserted.
- **Reclassification:** 0 — no unit changed bucket without a genuine holds change.
- **Reachability:** 0 — no previously-unreachable unit became reachable (the 4 closed records
  were already reachable Fighter class features; this cycle changed only the atlas's own
  attribution of an already-computed value, not what a player can select).
- **Instrument-correction:** 0 — no wrong prior figure found and corrected this cycle (see
  Discoveries).

## Remainder — 324 units, named by sub-cause (`decisions.md §15`)

Grouped by corpus-key group prefix, re-derived fresh against the post-cycle inventory (populations sum exactly to 324):

| Sub-cause | Units | Notes |
|---|---|---|
| `Domain Power` (the 56 domains `DOMAIN_POWER_CATALOG` carries no formula for) | 56 | Unchanged from the prior cycle's own receipt — dice notation, multi-`DESC`/level-gated variants, enemy-facing effects, no header chain, or simply not yet reached. Real new-formula work per domain. |
| `Domain Base` | 33 | Investigated THIS cycle (see "Mechanism-specific direction followed" above): the DC formula is genuinely computable but genuinely never CONSUMED by the real pipeline (no enemy-facing domain power is modelled to use it). Closing this needs a real new save-DC consumer, not a probe — named as next-cycle work, not a disposition ruling (unlike the prior cycle's framing, this is not "is magnitude_token_count even correct" — the magnitude IS real, PF1's own domain save DC; it is simply not wired to any computation yet). |
| `Favored Enemy Bonus` / `Favored Terrain Bonus` | 31 + 11 = 42 | Ranger tracking: no `favored_enemy`/`favored_terrain`-keyed per-record magnitude consumer exists in `src/rules_core/` today (confirmed by grep, unchanged from prior cycle). New engine subsystem. |
| Wizard opposition/arcane school cluster (`{Abjuration,Conjuration,...} {Opposition }School`, `Arcane School Tracker`, `Universal School`) | 18 | Wizard school-restriction tracking absent entirely (unchanged from prior cycle's framing; re-derived count differs slightly from the prior receipt's ~34 estimate because that figure bundled `Domain Base`-adjacent groups the finer re-derivation here separates out). |
| `Bardic Performance` | 10 | Bard performance-type roster; no per-performance-type magnitude consumer. |
| `Draconic Bloodline Choice` | 10 | Sorcerer bloodline sub-choice roster; no per-type magnitude consumer. |
| `Secret Lore` | 10 | Oracle Lore mystery's sub-roster; no per-record consumer. |
| `New Arcana` | 9 | Wizard/sorcerer bonus-spell-slot roster; no per-record consumer. |
| `Weapon Training` (the 48 non-canonical tier/group combinations) | 48 | Structurally unclosable by this mechanism's own attribution approach: `fighter_weapon_training_attack_bonus` only ever computes the engine's 4 hardcoded canonical pairs; the other 12 groups (Axes, Blades Light, Close, Crossbows, Double, Flails, Natural, Spears at 4 tiers each, minus the 1 canonical tier each of Bows/Hammers/Pole Arms/Blades Heavy already closed) have NO computation at all in this codebase — closing any of them needs new production arithmetic (a generic per-group weapon-training bonus, replacing the current single-canonical-group special case), not an attribution fix. |
| Small (2-3 unit) per-class roster groups (`Ki Stat Choice`, `Physical Enhancement`, `Damage Reduction`, `Divine Bond`, `Favored Class Bonus`, `Hunter's Bond`, `Nature's Bond`, `Precise Strike`, `Wildshape 2`, `Wildshape 3`) | 20 | Each its own narrow new-consumer gap, unchanged from prior cycle. |
| Long tail (single-unit groups: `Aura of {Chaos,Evil,Good,Law}`, `Bloodline {Feat,}Tracker`, `CMB`/`CMB Output`, `{...} Qualify`, `Death Attack`, `Domains`, `Equipment`, `Default`, ...) | 68 | One-off tracker/bookkeeping rows, each needing individual per-record inspection before disposition. |

70 distinct group families total, summing exactly to 324 (verified by the same re-derivation
script that produced this table, `sum(groups.values()) == 324`).

## Next-cycle plan

Cheapest-first for whichever cycle next owns this mechanism:
1. **`Domain Base`'s real DC consumer (33 units)** — the highest-value single fix this cycle
   identified but did not build: wiring ONE real save-DC-consuming domain power (e.g. Evil's
   Touch of Evil, already parseable under this catalog's own grammar per `domain_power.rs`'s own
   doc comment) would both close that power's own `Domain Power` record AND establish the DC
   consumer every one of the 33 `Domain Base` records needs, likely closing several `Domain Base`
   records at once via the SAME generic mechanism this cycle used for Weapon Training.
2. **Wizard school cluster (18)** — shares its root cause with the already-scoped sibling gap
   named in `class_feature_option_pool_record_not_held_by_engine`'s own receipt.
3. **`Favored Enemy/Terrain Bonus` (42)** and **`Bardic Performance`/`Draconic Bloodline Choice`/
   `Secret Lore`/`New Arcana` (39)** — each its own scoped "new per-record consumer" investigation.
4. **`Weapon Training`'s remaining 48** — needs a real generic per-group bonus computation
   (replacing the current single-canonical-group special case), not an attribution fix; likely
   the single largest remaining closable chunk once built, since it would close all 48 at once
   the same way this cycle's fix closed 4.

- **Status:** partial
