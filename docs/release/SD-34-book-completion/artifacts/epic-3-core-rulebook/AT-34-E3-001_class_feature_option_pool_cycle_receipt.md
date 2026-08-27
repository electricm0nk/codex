# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`class_feature_option_pool_record_not_held_by_engine` mechanism)

- **Commit SHA:** (filled after commit — see git log; parent `9e380e2ce6`)
- **Files touched:**
  - `src/rules_core/class_feature_pool_catalog.rs` — refactored the shared
    walk-and-render pipeline behind `load_pool_catalog` into
    `load_class_feature_catalog(repo_root, key_filter)`, and added a new
    sibling public entry point, `load_standalone_class_feature_catalog`, plus
    `is_standalone_class_feature` (`!key.contains(" ~ ")`, mutually exclusive
    by construction with the existing `is_registered_pool_group`). Four new
    tests proving real coverage and non-overlap with the pool catalog.
  - `src/bin/v06_work_inventory.rs` — new `EngineFacts::class_feature_standalone_catalog`
    field + `class_feature_standalone_catalog_holds` accessor, populated from
    the new loader; one new rung inside `Kind::ClassFeature`'s "no owner
    resolved, text_only" branch, gated by the SAME three guards the sibling
    pool-catalog rung already uses (`has_real_description`,
    `is_display_wiring_class_for_promotion(wc_class)`,
    `!universal_sheet_modifier`), promoting to `text-complete` with a new
    evidence string, `class_feature_standalone_catalog_serves_a_rendered_description`.
  - `scripts/completion_atlas.py` — all ten `BUCKET_DEFINITIONS` `file:line`
    citations re-derived and corrected (this cycle's own insertions shifted
    every one; the shift was **not** uniform — the four occurring after this
    cycle's own edit site shifted by 19 more lines than the six occurring
    before it — each was independently re-derived by grepping the literal
    target content, not computed by hand-arithmetic on the diff hunks).
  - `scripts/missing_engine_tables.py` — both `ENGINE_SURFACE_CITATIONS`
    entries (`companion`, `power`) re-derived and corrected for the same
    reason (their own `--check` gate was silently green before this cycle —
    `python3 scripts/missing_engine_tables.py --check` was not part of the
    dual-audit or denominator gates I ran, and its own citation drift went
    undetected until I checked it proactively, matching the class named the
    same failure mode this cycle's own task brief warned about).
  - `docs/work-inventory.json` (regenerated at HEAD, guarded regeneration
    path — plain `cargo run --locked --release --bin v06_work_inventory`,
    `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` set from
    this session's own fresh `corpus_literal_sweep`/
    `derived_evaluator_fixture_check` runs, no `--allow-stamp-loss` used or
    needed).
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_cycle_receipt.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`
  - `docs/retro/events/sd34-at-34-e3-001.jsonl` (retro events for this cycle)

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` on the scoped diff —
  `git diff <base>...HEAD -- src/rules_core/ src/bin/ scripts/oracle_harness/
  data/corpus/core_rulebook/** docs/work-inventory.json | grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` surfaces only
  pre-existing `"display:sd32_class_ingest"`/`"display:sd32_simple_filename_
  kind_ingest"` **data values** inside the regenerated
  `docs/work-inventory.json` (historical `wiring_class_signals` values, not
  code — the exact shape the `class_absent`/`deity_absent` cycles' own
  receipts already documented and self-healed as not a violation).

- **Wired-integration audit result:** `OK_NO_TOKENS` in effect. The scoped
  diff carries five `"placeholder"` matches, all inside `src/bin/
  ingest_race_traits.rs` (PCGen's own literal `###Block: Placeholder
  objects...` comment) — confirmed via `git log -1 -- src/bin/
  ingest_race_traits.rs` → `ae25d75d7d`, the **prior sibling cycle**
  (`race_trait_absent_from_race_traits`), not this cycle's own edit. No new
  stub/mock/placeholder token in any file this cycle actually wrote.

- **Acceptance criterion (verbatim, `epic-breakdown.md` AT-34-E3-001):** "**970**
  Core Rulebook units whose table exists but which are not in it. **Evidence:**
  the atlas reporting bucket B at zero for `core_rulebook`, and the mechanism
  that placed them named — **by mechanism, not per record.**" This cycle's
  own bar (task brief / `decisions.md §14`): drive
  `class_feature_option_pool_record_not_held_by_engine` to zero. **AT-34-E3-001
  as a whole does not close this cycle** — four of the nine named mechanisms
  remain fully closed from prior cycles (`domain`, `race_trait_absent`,
  `class_absent`, `deity_absent`); this cycle's own mechanism moves from
  **63 to 57**, not to zero (see "Discoveries" below for why, and the
  four-way sub-decomposition this cycle names for follow-up).

## Re-derived population, not carried forward

Re-derived at this cycle's start SHA (`9e380e2ce6`), matching the task
brief's stated figure exactly (verified, not assumed):

```
$ python3 -c "
import json
with open('docs/work-inventory.json') as f:
    units = json.load(f)['units']
from collections import Counter
c = Counter()
for u in units:
    if u.get('book')=='core_rulebook' and u.get('status')=='engine-does-not-hold':
        c[u.get('evidence')] += 1
print(c['class_feature_option_pool_record_not_held_by_engine'])
"
63
```

## Discoveries — why this mechanism is not a single root cause

Unlike this criterion's four already-closed sibling mechanisms (`domain`=1,
`race_trait_absent`=9, `class_absent`=17, `deity_absent`=21 — each a single,
homogeneous root cause), direct inspection of all 63 units' real corpus rows
(`data/corpus/core_rulebook/class_feature/**/*.json`) found this evidence
string covers **at least six distinct real shapes**, verified per-record:

1. **Genuinely prose-only, mechanically-inert standalone features (6
   units)** — a bare feature name (never `" ~ "`-qualified), a real,
   clean-rendering `DESC:`, and zero PCGen engine-effect tokens
   (`AUTO`/`ABILITY`/`BONUS`/...): `Timeless Body`, `Uncanny Dodge`,
   `Woodland Stride`, `Evasion Output`, `Improved Evasion`, `Blank Weapon
   Block OS`. **Closed this cycle** via the new standalone catalog (below).
2. **Proficiency/mechanical-grant tokens with no tracking system anywhere in
   this engine (28 units)** — `Armor Prof ~ {Heavy,Light,Medium}`, `Weapon
   Prof ~ {Auto,Martial,Simple}`, `Shield Prof`(`~ Tower`), `Weapon
   Proficiencies ~ {Bard,Cleric,Druid,Monk,Rogue}`, `Weapon and Armor
   Proficiency ~ {Bard,Druid,Fighter,Monk,Paladin,Ranger,Rogue}`, `All
   Automatic/Martial Proficiencies`, `Add Spoken Language`, `Armor Training ~
   Heavy Armor`, `Channel {Negative,Positive} Energy`. Verified by direct
   grep: no `struct`/`fn` anywhere in `src/rules_core/` tracks a character's
   weapon/armor-proficiency *possession* as a fact (only `feat_effects.rs`'s
   `weapon_proficiency_grants_from_feats`, which is about explicit FEAT
   possession, a different subsystem, and `src/rules_core/race_resolver.rs`'s
   `ABILITY:FEAT|AUTOMATIC` handling, which is racial-trait-only). Channel
   Energy is the one exception with real, already-computed magnitude
   (`pilot_compute/mod.rs`'s `channel_energy_dice`/`channel_energy_uses_per_day`,
   grounded for Cleric) — but that computation is never attributed back to
   *this* corpus record's key (no probe exists for a non-choice, automatic
   class feature the way `probe_class_feature_effect_wiring` exists for
   choice-driven pools). Building either capability is real, new,
   cross-cutting infrastructure — out of this cycle's narrow, disjoint-file
   scope.
3. **Class-skill lists computed from a wholly separate, hand-kept source
   (10 units)** — `Class Skills ~ {Barbarian,Bard,Cleric,Druid,Fighter,Monk,
   Paladin,Ranger,Rogue}`, `Jack of All Trades ~ Class Skills`. Verified:
   `src/rules_core/skill_allocation.rs`'s `class_skill_set` derives class
   skills from hand-kept `GROUNDED_{FIGHTER,ROGUE,WIZARD}_CLASS_SKILLS`
   constants, **not** from these corpus `CSKILL:` records — even Fighter's
   and Rogue's own records stay correctly unattributed (Decision §2a: a
   shape engine computing a value does not complete the *record* it never
   reads from).
4. **Wizard opposition-school spell-restriction tracking, absent (9
   units)** — `{Abjuration,Conjuration,Divination,Enchantment,Evocation,
   Illusion,Necromancy,Transmutation,Universal} Wizard Spells`. No
   `SPELLKNOWN`-restriction engine exists in `src/rules_core/` for these.
5. **Companion/special-mount summoning not attributed to these specific
   records (3 units)** — `Companion ~ {Animal Companion,Special Mount}`,
   `Special Mount ~ Standard Choices`.
6. **Vacuous placeholder rows with genuinely zero content (3 units)** —
   `Empty Selection ~ Standard {Barbarian,Monk,Rogue}`: `null` description,
   raw_tokens are `KEY`/`CATEGORY`/`TYPE` only (PCGen's "no archetype swap
   selected" filler). Left unclosed rather than invented a new vacuous-verdict
   rung — a real disposition here is `decisions.md §2`'s job (an unpredicted
   verdict shape is a defect in the atlas, not this cycle's to invent).
7. **Domain Power ~ {Leadership, Sun's Blessing} (2 units)** — read against
   the real corpus row: `Leadership` (Nobility domain, 8th-level power)
   grants an automatic feat + a static, untracked "leadership score" bonus
   (no formula, no per-day use — does not fit `domain_power.rs`'s existing
   magnitude/uses-per-day shape at all). `Sun's Blessing` (Sun domain) DOES
   carry a real scaling bonus (`+%1|DomainSunLVL`), but even the FIVE domains
   `domain_power.rs` already computes correctly (Good/War/Strength/
   Destruction/Glory) are **not credited on the atlas at all** — verified
   directly: every one of their own `Domain Power ~ *` units still reports
   `class_feature_option_pool_record_with_magnitude_not_held_by_engine`,
   because `CLASS_FEATURE_POOLS` (the registry `probe_class_feature_effect_
   wiring` walks) has no `"Domain Power"` entry (`class_feature_owner_via_
   pool_catalog("Domain Power", ...)` returns `None`, confirmed by this
   file's own existing test). Adding these two domains' formulas would not
   move either unit — the attribution path itself does not exist yet, and
   building it touches the `with_magnitude` sibling mechanism's entire
   population (333 units, not mine to touch).
8. **Multi-`DESC:` ingest truncation (2 units)** — `Octopus Wild Shape ~
   Poison`, `Martial Weapon Proficiency Output`: real description, but the
   corpus row carries more than one `DESC:` segment, so this catalog's own
   render-and-refuse gate (shared with the pool catalog, proven safe by wave
   23's own finding) correctly refuses rather than serve a truncated
   fragment. A real fix lives in `cache_gen::class_feature::generate`
   (ingest territory, a different file's scope, per this module's own
   established disjoint-file-touch convention).

**28 + 10 + 9 + 3 + 3 + 2 + 2 = 57** — every remaining unit named by
sub-cause, no unnamed gap.

## The fix (6 units)

`Kind::ClassFeature`'s "no owner resolved" branch already checked
`class_feature_pool_catalog_holds`, but that catalog is deliberately gated to
`" ~ "`-qualified keys only (`is_registered_pool_group`) — option-pool
members, never a bare standalone feature name. The six units above are real,
already-shipped CRB features whose description renders clean with **zero**
PCGen engine-effect tokens (`has_no_engine_effect_token`) and exactly one
`DESC:` segment — genuinely nothing left to compute, real prose to show. A
new sibling catalog, `load_standalone_class_feature_catalog`, reuses the
IDENTICAL safety pipeline (render-and-refuse, engine-effect-token,
archetype-lock, multi-`DESC:`, bare-`%N`, unimplemented-marker guards) for
the mutually-exclusive standalone-key partition, so it can never serve a
record the pool catalog already does (or vice-versa) and can never
misclassify a genuinely-mechanical record (`Armor Prof ~ Heavy`'s `AUTO:
ARMORPROF` token is `" ~ "`-qualified, so it never reaches this new catalog
at all; `Channel Negative Energy`'s `null` description fails
`has_real_description` upstream regardless).

## Figures + their re-derive commands

- **63 of 1,006** — this mechanism's share of `core_rulebook` bucket B at
  this cycle's start, per `decisions.md §14`'s enumeration. Command above;
  independently re-derived, matches exactly.
- **63 → 57** — this mechanism's own population, re-derived at this cycle's
  end SHA (same command, on the regenerated `docs/work-inventory.json`) →
  `57`.
- **6 units closed** — `Timeless Body`, `Uncanny Dodge`, `Woodland Stride`,
  `Evasion Output`, `Improved Evasion`, `Blank Weapon Block OS`, each
  confirmed individually:
  ```
  $ python3 -c "
  import json
  d=json.load(open('docs/work-inventory.json'))
  for k in ['Timeless Body','Uncanny Dodge','Woodland Stride','Evasion Output','Improved Evasion','Blank Weapon Block OS']:
      for u in d['units']:
          if u.get('book')=='core_rulebook' and u.get('corpus_key')==k and u.get('kind')=='class_feature':
              print(k,'->',u['status'],u['evidence'])
  "
  Timeless Body -> text-complete class_feature_standalone_catalog_serves_a_rendered_description
  Uncanny Dodge -> text-complete class_feature_standalone_catalog_serves_a_rendered_description
  Woodland Stride -> text-complete class_feature_standalone_catalog_serves_a_rendered_description
  Evasion Output -> text-complete class_feature_standalone_catalog_serves_a_rendered_description
  Improved Evasion -> text-complete class_feature_standalone_catalog_serves_a_rendered_description
  Blank Weapon Block OS -> text-complete class_feature_standalone_catalog_serves_a_rendered_description
  ```
- **974 → 968** — `core_rulebook`'s real atlas-partitioned bucket B
  before/after this cycle: `python3 scripts/completion_atlas.py --book
  core_rulebook --check` → `B: 968` post-cycle (delta `-6`, exactly this
  cycle's closure). Sibling mechanisms confirmed unmoved by this cycle
  (isolation check): `class_feature_owner_matched_by_name_but_record_not_
  held_by_engine` **346** (unchanged), `class_feature_option_pool_record_
  with_magnitude_not_held_by_engine` **333** (unchanged),
  `companion_absent_from_core_rulebook_companion_tables` **100** (unchanged),
  `race_trait_race_not_modelled` **132** (unchanged).
  `57+100+132+346+333 = 968` — matches exactly, no unnamed gap.
- **49,438** — corpus-wide unit population, unchanged by this cycle (no
  units added or removed, only reclassified): `len(d['units'])` on the
  regenerated `docs/work-inventory.json` → `49438`.

## Row-count command output

```
$ python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
cr = [u for u in d['units'] if u['book']=='core_rulebook' and u['status']=='engine-does-not-hold']
tgt = [u for u in cr if u['evidence']=='class_feature_option_pool_record_not_held_by_engine']
print('class_feature_option_pool_record_not_held_by_engine remaining:', len(tgt))
"
class_feature_option_pool_record_not_held_by_engine remaining: 57
```

Row count is `57`, not `0` — **this cycle's own mechanism does not close.**
`kanban.md`'s AT-34-E3-001 row stays `in-progress`; this cycle's own
sub-population figure is recorded there as `63 -> 57`, with the seven named
sub-causes above as the next cycle's dispatch list (matching `decisions.md
§14`'s own precedent for decomposing a criterion that does not fit one
cycle — this is the same move, one level deeper, inside a single named
mechanism that turned out not to be homogeneous).

## Build scope verified

- `cargo test --locked --lib` (workspace lib, includes the new
  `class_feature_pool_catalog` tests): `2866 passed; 0 failed; 14 ignored`.
- `cargo test --locked --bin v06_work_inventory` (scoped): `376 passed; 0
  failed` — unchanged from this cycle's start (no new/removed test in this
  binary), run **after** the last write that could move a figure (the
  `docs/work-inventory.json` regeneration) — `decisions.md §12` L7.
- `cargo test --locked --no-run` (full workspace): clean, exit 0.
  `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001`.
- `apps/desktop/src-tauri` (separate cargo workspace, tested explicitly):
  `cargo test --locked --no-run` in that directory with its own
  `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001-desktop` — clean, exit 0.
- Run at SHA: this cycle's own HEAD (see commit SHA above).

## Sweep population

`corpus_literal_sweep`: `48708 records examined of 51482 read, 0 findings,
CLEAN` — before and after this cycle's regeneration are the SAME number
(N/A: this cycle added/regenerated zero corpus records; only `classify()`'s
in-memory logic changed, plus a new consumer-territory read of
already-committed `data/corpus/` — no new file). No delta expected or
observed.

`derived_evaluator_fixture_check`: `1839 unit(s) cleared over 2580 fixture
row(s); 0 failed; 0 not ingested` — supplied as `DERIVED_FIXTURE_CHECK_REPORT`
for the guarded regeneration, per precedent, unchanged from the inherited
baseline.

## Citation-drift self-heal (task brief's own named hazard)

`completion_atlas.py`'s ten `BUCKET_DEFINITIONS` citations and
`missing_engine_tables.py`'s two `ENGINE_SURFACE_CITATIONS` entries all
drifted from this cycle's own insertions into `v06_work_inventory.rs` (47
lines added across four sites). Caught by running
`python3 scripts/completion_atlas.py --check` (`citation_failures=10`) and
`python3 scripts/missing_engine_tables.py --check` (`citation_failures=2`)
**before** writing this receipt, per the task brief's explicit warning. Each
of the twelve was independently re-derived by grepping the literal target
content (not computed from the diff hunk offsets alone, since the shift is
not uniform across the file). Both gates are clean at this cycle's HEAD:
`citation_failures=0` for both.

## Denominator gate

`python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'`
→ `files_checked=15 violations=0`.

## PI gates (decisions.md §14's own precedent, re-run defensively)

`scripts/verify.sh --only site-public-status-pi-gate --only
site-dashboard-pi-gate` → `PASS` (both). Not directly implicated by this
cycle's change (no deity/PI-adjacent record touched), re-run to confirm no
regression.

## Oracle pin

Not applicable — no figure in this receipt comes from the pinned PCGen
oracle corpus; every figure comes from the repo's own committed
`data/corpus/` and `docs/work-inventory.json`.

- **Status:** blocked-escalated

  **Not an operator-ruling request** — no `## Open blockers` entry is filed,
  and none of the seven named remainders in "Discoveries" is a policy
  question. This is a sequencing report, identical in spirit to
  `decisions.md §14`'s own decomposition of the parent criterion: this one
  mechanism, assigned as if it were a single homogeneous cause (63 units,
  matching the filing cycle's own count), turned out on direct per-record
  inspection to bundle at least seven distinct engineering efforts under one
  evidence string — six of which require new, cross-cutting engine
  capabilities this cycle's narrow, disjoint-file-touch scope must not build
  unreviewed (proficiency-possession tracking, a non-choice class-feature
  attribution probe, wizard opposition-school tracking, a `Domain`-vs-
  `Domain Power` `CLASS_FEATURE_POOLS` registration gap that reaches into the
  333-unit `with_magnitude` sibling's own population, and an ingest-territory
  multi-`DESC:` fix). Reported here, not narrowed, not silently deferred —
  named with populations so a follow-up cycle (or several, one per named
  sub-cause, cheapest-first, exactly as `decisions.md §14` already
  dispatched the top-level nine) can pick this up without re-deriving any of
  this cycle's own investigation.

## Movement, four buckets

- **Closure:** 6 — `Timeless Body`, `Uncanny Dodge`, `Woodland Stride`,
  `Evasion Output`, `Improved Evasion`, `Blank Weapon Block OS` moved from
  bucket B (`engine-does-not-hold`) to `text-complete` (DONE) via a real,
  tested, safety-gated engine addition (the new standalone catalog) — the
  engine genuinely holds and serves these records now, not a relabeling.
- **Reclassification:** 0 — no unit changed bucket without a genuine holds
  change; the sibling mechanisms' counts are independently confirmed
  unmoved (see Figures).
- **Reachability:** 0 — no previously-unreachable unit became reachable this
  cycle (no character-build/reach-gate change).
- **Instrument-correction:** 0 — no count changed because a measurement
  method was wrong; the twelve citation fixes correct **tooling metadata**
  (line-number pointers), not a measurement method, and moved no unit
  count on any board.

## Notes

The task brief's own quoted population (63) matched the re-derived figure
exactly, so no correction was needed there. The real discovery this cycle
makes is that a SINGLE evidence string emitted by ONE `return
engine_does_not_hold(...)` call site does not imply a single root cause —
the four smaller sibling mechanisms this criterion already closed
(domain=1, race_trait_absent=9, class_absent=17, deity_absent=21) each
happened to be homogeneous; this 63-unit one was not, and only direct,
per-record inspection of the real corpus rows (not a bulk grep of the
evidence string alone) surfaced that.

## Next-cycle plan

Dispatch, cheapest-first, matching `decisions.md §14`'s own cadence:

1. **Vacuous placeholders (3 units, `Empty Selection ~ Standard {Barbarian,
   Monk,Rogue}`)** — needs `decisions.md §2`'s own ruling on what verdict a
   record with genuinely zero content (no DESC, no non-taxonomy token)
   should carry; likely a new, narrow, well-guarded rung (not a stub — a
   real "nothing to compute, nothing to display, and the corpus itself
   proves it" check), reported to `atlas-defects.md` per §2's own rule
   before being built, since this is an unpredicted verdict shape.
2. **Multi-`DESC:` ingest truncation (2 units)** — an ingest-territory fix
   in `cache_gen::class_feature::generate` (concatenate multiple `DESC:`
   segments instead of keeping only the first), outside this cycle's
   consumer-territory file-touch set.
3. **Class-skill / companion-mount attribution (13 units)** — requires
   either widening `skill_allocation.rs`'s hand-kept class-skill lists to
   read from these corpus records directly (a real, larger, cross-cutting
   change to a shared module many other kinds' correctness depends on) or a
   new, narrower per-record attribution check; needs its own scoped
   investigation before implementation.
4. **Wizard opposition-school + proficiency/grant tracking (37 units)** —
   the largest remaining share; genuinely new engine subsystems (spell-school
   restriction tracking, character-level proficiency-possession tracking, a
   non-choice-based class-feature effect-attribution probe). Recommend
   splitting further by sub-shape once scoped, rather than one oversized
   cycle.
5. **Domain Power `CLASS_FEATURE_POOLS` registration gap** — a real,
   separately-verified defect (the five ALREADY-computed domains are not
   credited on the atlas at all because `"Domain Power"` has no
   `CLASS_FEATURE_POOLS` entry) that reaches into the 333-unit
   `with_magnitude` sibling mechanism's own population, not this one's — flag
   for whichever cycle owns `class_feature_option_pool_record_with_
   magnitude_not_held_by_engine` rather than fixed here.
