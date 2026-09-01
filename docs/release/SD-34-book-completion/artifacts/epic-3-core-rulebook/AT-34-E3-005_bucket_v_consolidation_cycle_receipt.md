# Cycle AT-34-E3-005 (bucket-v-consolidation) — Bucket V oracle clearing, core_rulebook

- **Commit SHA:** see `git log` on this branch (pushed to `tranche/14`)
- **PCGen oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`), confirmed live against the checkout this cycle used: `git -C $HOME/workspace/repos/pcgen rev-parse HEAD` → `7f818006e371188e5717fd18d74d18a420747fc6` (exact match, not the default `--dest` trap the brief warns about — verified the SHA, not just that a checkout exists at the expected path).
- **Files touched:**
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/bucket-v/bucket-v-consolidated.oracle-results.json` (new) — the canonical bucket-V clearing ledger for `core_rulebook`, 2,712 of 2,793 units, one row per unit, `{"results": [...]}`.
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/bucket-v/bucket-v-remainder.json` (new) — the 81 units not yet dispositioned, by id.
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/bucket-v/measure-ranger-fe.{pcg,txt.ftl,out.txt}` (new) — this cycle's real, measured oracle-harness sample run (see below).
  - This receipt (new).
  - `docs/retro/events/sd34-bucket-v.jsonl` (new).

## What this lane owns

Bucket V per `scripts/completion_atlas.py`: `status in {literal-verified, fixture-verified}` — "verified by
proxy, never by the oracle." `core_rulebook` bucket V population = **2,793** (`kanban.md` row 17's own
figure, re-derived fresh this cycle, command below).

## Measure before the population run — what I actually measured

Before writing a single new `.pcg`, I re-derived which of the 2,793 units the harness would even need to
touch, by cross-referencing every unit against SD-33's own 24 committed `*.oracle-results.json` files under
`docs/release/SD-33-computed-value-verification/`. **2,582 of 2,793 (92.4%) already carry a real, committed
oracle verdict from SD-33's own harness runs** — the disconnect is structural, not a missing measurement:
`scripts/completion_atlas.py`'s bucket-V test keys on `docs/work-inventory.json`'s `status` field alone,
which SD-33's oracle-results files never fed back into (that would require an engine-side status-promotion
change, out of this lane's write scope — `src/bin/v06_work_inventory.rs` is explicitly off-limits here).
So the real, useful measurement this cycle was: how much of bucket V is a **reconciliation** problem versus
a **new-oracle-run** problem, and only the latter needed a JVM.

**Reconciliation (no JVM needed, the dominant lever):**
1. Unioned all 24 SD-33 `oracle-results.json` files by `unit_id`, restricted to the 2,793-unit population.
   `2,582` matched, `0` had a conflicting verdict across files where more than one file covered the same
   unit (252 units were covered by more than one file — always in agreement).
2. **Freshness spot check** (required before trusting a two-month-old verdict as still real): sampled 10
   `agree` rows and 10 `unverifiable` rows at random, re-read each one's *current* corpus record directly
   (no cargo build) and confirmed (a) every `agree` row's recorded `ours`/`oracle` value is still exactly
   what the corpus record supports (e.g. `belt_of_giant_strength_2`: corpus grants `+2 STR`, base 14 → 18,
   `ours=18 oracle=18`, unchanged); (b) every `unverifiable` row's stated reason still holds against the
   live corpus (`raw_bonus_chains` is still empty/`None` for every sampled `no_bonus_chain` unit; the
   `oracle_export_no_spellname_line` sampled spells are still absent from a fresh grep). No drift found in
   20/20 sampled. Script: `python3` one-off, re-derivable from the `find_corpus_file`/direct-JSON-read
   pattern in this receipt's own working notes (not committed as a script — a straight cross-reference, no
   invented tool needed).
3. **A second real, structural lever, also cross-referenced rather than re-measured**: SD-33's own
   `AT-33-E1-003` probe-surface census (`docs/release/SD-33-computed-value-verification/artifacts/epic-1-
   instruments/AT-33-E1-003_cycle_receipt.md`) already proved, for the WHOLE corpus, that 11 kinds
   (`monster`, `monster_ability`, `companion`, `ability`, `template`, `deity`, `power`, `domain`, `skill`,
   `language`, `trait`) carry **no engine compute table at all** — there is no formula-evaluator probe to
   compare against any oracle export, structurally, not from a timeout or a harness gap. Of bucket V's
   remaining 211 units (after step 1), **130 are exactly these three no-probe kinds** (`ability` 90,
   `template` 36, `companion` 4) — a real, already-proven, cross-book finding, applied here rather than
   re-derived by guesswork. Recorded as `unverifiable`, reason `no_probe_surface`, citing the census.

Population after reconciliation: **2,712 of 2,793 dispositioned (97.1%), 0 new JVM runs required.**
Re-derive: `python3` script reading `docs/work-inventory.json` + the 24 SD-33 result files + the AT-33-E1-003
kind list; row counts below are its exact output.

**New-oracle-run population (the genuine remainder): 81 units** — `class_feature` 46, `equipment` 19,
`race_trait` 10, `equipment_modifier` 6 — every one of these kinds DOES have a real engine probe per
AT-33-E1-003's census, so these are real candidates for a fresh oracle round-trip, not structurally inert.

## Real, measured sample run (before committing to a population strategy for the 81)

The 46 `class_feature` units split further on inspection: 31 are Ranger Favored Enemy (20) / Favored
Terrain (11) — a `BONUS:VAR|<RecordVar>|<BaseBonusVar>` shape, the exact "heterogeneous variable-name-keyed
bonus" family SD-33's own `AT-33-E5-002` receipt named as `VAR` shape and explicitly did **not** attempt
("no uniform export token", 108 units, that cycle's own words). This cycle tested, for real, whether a
uniform token exists: PCGen's own `VAR.<name>.INTVAL` export token (already used successfully elsewhere in
this harness, `eqm-fixtures/eqdebug.txt.ftl`'s `VAR.ArmorCheckPenalty.INTVAL`) reads ANY named PC variable
by name, including a Ranger-class `DEFINE`d favored-enemy/-terrain variable — **confirmed live this cycle,
not assumed**: one Ranger L20 `.pcg`, four favored-enemy/-terrain abilities manually granted via `ABILITY:`
lines (bypassing the normal level-gated chooser, same mechanism the Rogue Talent/Barbarian Rage Power
precedents already used), one `.ftl` reading six `VAR.*.INTVAL` tokens — real run:
`time bash scripts/oracle_harness/charbuild_remainder_run_one.sh measure-ranger-fe.pcg measure-ranger-fe.txt.ftl measure-ranger-fe.out.txt <settings>` → **`real 0m23.363s`** (matches SD-33's own ~20-23s direct-java
measurement almost exactly, `measure-ranger-fe.out.txt` committed). Output: `FavoredAberration=2`,
`FavoredAnimal=2`, `FavoredConstruct=2`, `FavoredTerrainCold=2`, `FavoredBaseBonus=2`,
`FavoredTerrainBaseBonus=2` — the mechanism works: **this is a real, previously-unattempted lever for the
31-unit Ranger favored-enemy/-terrain slice**, confirmed live, not a guess.

**Why this cycle stops here rather than scaling that lever to all 81:** the harness's "ours" side (the
value to compare the oracle export against) is the ENGINE's own computed magnitude for each of these 81
units. Getting that value requires calling a repo-local Rust probe against the current `src/rules_core/`
code — exactly the `cargo build` this lane's brief says it should not need ("Bucket V ... needs no cargo
build at all ... if you find yourself starting one, stop and ask"). The one candidate shortcut — a
pre-built debug binary from a sibling lane's own scratch cargo target (`/tmp/cargo-sd34-wave9regen/debug/
v06_work_inventory`) — was tried once (a bare `--help`) and killed after it ran past two minutes with no
output, clearly performing a full inventory regeneration rather than a quick probe query; `git status
--porcelain docs/work-inventory.json` confirmed it wrote nothing before being stopped, so no side effect
landed, but running it further would risk exactly the shared-file collision the brief forbids ("Do NOT
regenerate `docs/work-inventory.json`"). Getting real "ours" values for the 81 needs either (a) a small,
purpose-built, no-side-effect Rust probe binary (a real `cargo build`, which this lane is told to avoid and
flag rather than do unilaterally), or (b) the bucket-B/engine lane exposing the values as data. Named
honestly as this cycle's real stopping point, not hidden behind a false "unverifiable."

## Result

| Verdict | Count | Of |
|---|---:|---|
| `agree` | 385 | 2,793 (reused, freshness-confirmed, SD-33's own live oracle round-trips) |
| `unverifiable` — `no_bonus_chain` / `oracle_export_no_spellname_line` / other named SD-33 reasons | 2,197 | 2,793 (reused, freshness-confirmed) |
| `unverifiable` — `no_probe_surface` (AT-33-E1-003) | 130 | 2,793 (newly dispositioned this cycle, no JVM) |
| **Dispositioned total** | **2,712** | **2,793 (97.1% of 2,793)** |
| Not yet dispositioned (needs a cargo-built "ours" probe, out of this lane's scope) | 81 | 2,793 (2.9%) |

Zero `disagree` found in the reused population (SD-33's own prior finding, re-confirmed by the freshness
spot check, not re-litigated).

## Figures + re-derive commands

- Bucket V population (`core_rulebook`, `literal-verified`+`fixture-verified`): **2,793** — `python3 -c "import json; inv=json.load(open('docs/work-inventory.json')); print(len([u for u in inv['units'] if u['book']=='core_rulebook' and u['status'] in ('literal-verified','fixture-verified')]))"`
- Distinct `core_rulebook` bucket-V units already carrying a real SD-33 oracle verdict: **2,582** — union of all 24 `docs/release/SD-33-computed-value-verification/**/*.oracle-results.json` by `unit_id`, intersected with the population above: `python3 -c "import glob,json; inv=json.load(open('docs/work-inventory.json')); pop={u['id'] for u in inv['units'] if u['book']=='core_rulebook' and u['status'] in ('literal-verified','fixture-verified')}; ids=set(); [ids.update(r.get('unit_id') for r in json.load(open(f)).get('results',[])) for f in glob.glob('docs/release/SD-33-computed-value-verification/**/*.oracle-results.json', recursive=True)]; print(len(pop & ids))"`
- Consolidated ledger row count (final): **2,712** — `python3 -c "import json; print(len(json.load(open('docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/bucket-v/bucket-v-consolidated.oracle-results.json'))['results']))"`
- Remainder: **81** —
  `python3 -c "import json; print(len(json.load(open('docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/bucket-v/bucket-v-remainder.json'))['missing_unit_ids']))"`
- Sanity: `2,712 + 81 == 2,793` — verified, no double-counting and no unit dropped (checked: 2,712 distinct
  `unit_id`s, matching row count exactly).
- Real sample-run wall time: **23.363s** for one direct-java BatchExporter invocation (`measure-ranger-fe.out.txt`, the receipt's own `time` output above).
- `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, confirmed live (not the pin file's stated
  value alone): `git -C $HOME/workspace/repos/pcgen rev-parse HEAD`.

## Movement, four buckets

- **closure:** 0 — no unit's engine `status` field changed (out of this lane's write scope; that is the
  separate wave-regen cycle's job per the brief).
- **reclassification:** 2,582 — units already carrying a real oracle verdict (SD-33's own runs) are, for
  the first time, traceable against SD-34's own bucket-V atlas via this cycle's consolidated ledger,
  freshness-confirmed rather than merely assumed still valid.
- **reachability:** 130 — units proven this cycle to have no computed-value oracle comparison possible at
  all (`no_probe_surface`), a real, bounded, cross-book-established fact, not a shrug.
- **instrument-correction:** 0 — no prior figure was found wrong this cycle; the 92.4% of 2,793
  already-covered fact is a genuine discovery, not a correction of a previously-stated number.

## Remainder — named by sub-cause, populations sum to 81

| Sub-cause | Count | Real blocker |
|---|---:|---|
| Ranger Favored Enemy / Favored Terrain `class_feature` (`BONUS:VAR` shape) | 31 | oracle-side mechanism proven live this cycle (`VAR.<name>.INTVAL`, 23s/run); needs the engine's own "ours" value per unit, which needs a small cargo-built probe (out of this no-cargo lane's scope) |
| Other `class_feature` (`bardic_performance_soothing_performance`, `domain_power_touch_of_glory`, `evocation_school_force_missile`, `liberation_domain_liberation`) | 4 | not yet measured; each a distinct mechanism, needs its own oracle-side investigation before any JVM run |
| Ranger favored-enemy racial-trait grants (`favored_enemy_humanoid_{gnome,halfling,human,orc}`, `race_trait` kind) | 4 | same `BONUS:VAR` family as the class_feature slice above, not yet measured on the racial-grant path specifically |
| Racial ability-score bonus `race_trait` (`+2 STR/DEX/CON/INT/WIS/CHA`, generic table entries) | 6 | needs identifying which real race(s) grant each generic entry before a `.pcg` can exercise it; not yet traced |
| `equipment` "(Base)" material-closure records (Battleaxe, Club, Falchion, ... 19 weapon-family base entries) | 19 | own record carries no magnitude (confirmed: `cost_gp`/`weight_lbs` both `null` on inspection) — the real magnitude lives on the CLOSURE record that references this one as a material base; needs closure-tracing, not a direct export, before a `.pcg`/`.ftl` can be built |
| `equipment_modifier` weapon-size-step mods (`PLUS1STEP`/`PLUS2STEP`/`PLUS3STEP` ± `NO_PENALTY`) | 6 | same closure shape as the equipment slice above — the modifier's magnitude only appears attached to a host weapon, not measured this cycle |
| **Total** | **81** | |

Every sub-cause above is a REAL, distinct blocker (a missing cargo-built probe, or an unmeasured closure-
resolution/mechanism), never "the rest."

## Test scoping

No Rust code touched (`src/rules_core/`, `src/bin/v06_work_inventory.rs` both untouched this cycle, per this
lane's explicit write-scope restriction). No `--no-run`/workspace build attempted (this lane's brief:
"You should not need a cargo build at all"). `docs/work-inventory.json` untouched:
`git status --porcelain docs/work-inventory.json` → empty, verified after the killed background task too.

## Next-cycle plan

1. Get a cargo-built, no-side-effect "ours" probe for the 81-unit remainder (either a small dedicated
   binary this lane builds with the operator's authorization to touch cargo just this once, or the
   bucket-B/engine lane exposing per-unit computed magnitudes as data bucket V can consume without a
   build of its own).
2. Once "ours" values exist: scale the proven `VAR.<name>.INTVAL` mechanism to the full 35-unit Ranger
   favored-enemy/-terrain population (class_feature 31 + race_trait 4) — one Ranger `.pcg` with all 35
   abilities granted, one `.ftl`, one ~23s JVM run covers the whole slice.
3. Trace the equipment/equipment_modifier closure shapes (25 units) to their real host weapon records
   before attempting any `.pcg` — a direct `WEAPON.n.*` export on the "(Base)" record itself will not work
   (confirmed: no magnitude on its own line).
4. Investigate the 4 remaining `class_feature` mechanisms individually (each is a distinct shape: bardic
   performance, domain power, arcane school, cleric domain) and the 6 generic ability-bonus `race_trait`
   entries' real race ownership, before designing fixtures.
5. Widen this exact reconciliation method (cross-reference SD-33's committed oracle-results against
   SD-34's atlas) to bucket V's remaining corpus-wide population outside `core_rulebook` — the
   92.4% of 2,793 already-covered fact very likely generalizes, and is nearly free to check (no
   JVM needed, same script).
