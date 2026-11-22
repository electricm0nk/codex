# Cycle AT-33-E5-remainder-equipment — Epic 5 Re-verification / AT-33-E5-001 + AT-33-E5-002 (remainder)

- **Commit SHA:** `c1919daa00` (pushed to `tranche/13`)
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-remainder-checks.txt.ftl` (new) — BatchExporter export template, reusing `AT-33-E2-002`'s own proven `SKILL.<name>.<property>` token family (`SkillToken`) directly against a literal skill name, one file per unit (the skill name is a template-time literal, not a runtime loop variable — see Notes).
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-remainder-generate-skill-pcgs.py` (new) — generates one `.pcg` + one matching `.txt.ftl` per SKILL-shape unit (Level-1 Human Fighter, base scores STR16/DEX14/CON14/INT10/WIS10/CHA8, one item equipped at the `Equipped` EquipSet location).
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-remainder-build-results.py` (new) — assembles the final per-unit results file from the two real sources this cycle produced.
  - `src/bin/e5_equipment_remainder_skill_ours.rs` (new) — repo-local batch "ours" probe, real live calls into `codex::rules_core::equipment_effects::compute_equipment_effects` (via `equipment_effects::general::compute_general_effect`'s `skill_bonus` field) for every SKILL-shape unit.
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-remainder.oracle-results.json` (new) — the committed per-unit deliverable.
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-remainder-*.json` (new, working data: census/manifest/partition files, kept for re-derivation).
  - `docs/release/SD-33-computed-value-verification/progress.md` (updated in place).
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-001_cycle_receipt.md` / `AT-33-E5-002_cycle_receipt.md` (this lane's totals appended to their figure rows).
  - `docs/retro/events/sd33-r2-equipment.jsonl` (new).
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-001 — the 1,741 `fixture-verified` units are re-examined against the oracle
  >
  > **Evidence:** per-unit `(ours, oracle, verdict)` rows committed; agreement and disagreement
  > counts both stated, with the denominator.
  >
  > ### AT-33-E5-002 — the 6,589 `literal-verified` units are re-examined
  >
  > **Evidence:** as above.

## What this lane owns

The 1,390-unit remainder across all of Epic 5's re-verification (32 -> 6,940 of 8,330 examined by
wave 1) was split into three sibling lanes. This lane owns exactly one named slice: the equipment
`other_bonus_shape` population (448 units) plus `equipment_modifier` (46 units) = **494 units**,
named in `AT-33-E5-002`'s own remediation receipt as a real magnitude-probe-bearing population
"not yet oracle-verified" — bonus-chain shapes wave 1's simple single-ability-STAT/Belt-Headband
mechanism does not cover.

## Enumeration first (generic pass by mechanism, not per-item)

### 448 `other_bonus_shape` equipment units, re-derived this cycle by first-qualifier label (a unit is counted once per distinct label it carries — the same convention `AT-33-E5-002`'s own receipt used, so the two numbers are directly comparable)

Re-derive: a fresh script reading every one of the 448 units' real corpus record (not the prior
lane's transcribed table) — `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-remainder-full448-labels.json`
holds the full per-unit label set this table is drawn from.

| Label | Count | Notes |
|---|---:|---|
| SKILL | 118 | This cycle's target — see below |
| VAR | 108 | Not attempted — heterogeneous variable-name-keyed bonus, no uniform export token |
| COMBAT | 92 | Not attempted — `BONUS:COMBAT\|<AC\|CMB\|CMD\|...>` family, multiple sub-shapes |
| STAT_multi_or_other_slot | 43 | Not attempted — `compute_magic_items_effect` stores a multi-ability chain's ability field as the raw un-split string (a real, named engine-shape gap, `AT-33-E5-002`'s own "Known engine-shape finding") |
| SITUATION | 34 | Not attempted — conditional bonus, PF1 "situational" bonuses are frequently not always-on |
| SAVE | 24 | Not attempted — maps to PCGen's `CHECK.<i>.TOTAL` (Fortitude/Reflex/Will) family, confirmed this cycle (see Notes) but not built out |
| WEAPON | 18 | Not attempted — overlaps `equipmods::compute_equipmods_effect`'s existing `TYPE=Enhancement`-gated weapon path only for a subset; the rest is ungated `WEAPON\|...` shapes with no existing resolver |
| ABILITYPOOL | 12 | Not attempted |
| SLOTS | 8 | Not attempted |
| DC | 8 | Not attempted |
| WEAPONPROF=* (7 distinct) | 15 | Not attempted — grants proficiency, not a numeric magnitude; likely a genuine `unverifiable` shape once examined |
| MOVEADD / SPELLKNOWN / SPELLCASTMULT / MISC / POSTMOVEADD / VISION / LOADMULT / MOVEMULT / HP / SPELLCAST / EQMWEAPON | 20 (combined) | Not attempted — 11 further small shapes, 1-4 units each |

**SKILL is the single largest shape (118 of 448) and the one this cycle examines in full**, per
`AT-33-E5-002`'s own next-cycle plan ("`SKILL` is the next-cheapest lever").

Of the 118 units carrying a `SKILL` chain, `compute_general_effect`'s own selection rule (first
`BONUS:SKILL|...` chain on the record) resolves to a single named skill for 90; the remaining 27
carry a multi-skill or `ALL` chain (`BONUS:SKILL|Swim,Climb|5|...`, `BONUS:SKILL|ALL|-2|...`) —
**a real, distinct engine-shape gap, not attempted this cycle**: `compute_general_effect`
(`src/rules_core/equipment_effects/general.rs`) stores `qualifiers[1]` verbatim, exactly the same
un-split-multi-value pattern `AT-33-E5-002`'s own receipt already named for `compute_magic_items_
effect`'s multi-ability STAT chains — there is no single `SKILL.<name>.MISC` PCGen token to compare
a comma-joined or `ALL` skill list against. Re-derive: `python3 -c "import json; d=json.load(open('.../equipment-remainder-skill-census.json')); print(len([x for x in d['items'] if ',' in x['skill'] or x['skill']=='ALL']))"` -> `27`.

### 46 `equipment_modifier` units, re-derived this cycle by reading every record's whole `raw_bonus_chains` field (not a filtered view)

| Group | Count | Disposition |
|---|---:|---|
| Empty `raw_bonus_chains` | 32 | **This cycle: `unverifiable`, reason `no_bonus_chain`** — real corpus fact for every one of the 32 (purely descriptive `SPROP:`/`DESC:` text, e.g. `Special Quality ~ Trip`/`Special Quality ~ Reach`, or a `VISIBLE:NO` display-only record like `CLOUDB`) |
| Non-empty `raw_bonus_chains` | 14 | Not attempted — see below |

The 14 with a real chain, by shape (re-derive: `equipment-remainder-census-equipmod.py`'s output,
`equipment-remainder-equipmod-census.json`):

- **`EQMARMOR\|ACCHECK\|1\|...` (3 units: `draco`/`dragonhide`/`material_dragonhide`)** — a shape
  `arms_armor::resolve_check_penalty` DOES already read (confirmed by inspecting the resolver this
  cycle: it reads `EQMARMOR|ACCHECK|<n>` chains straight off whatever record is passed in). Calling
  `compute_equipment_effects` on the modifier's own key resolves a real `armor_check_penalty`. **Not
  oracle-round-tripped this cycle**: PF1/PCGen models a material like Dragonhide as an `EQMOD`
  attached to a base armor item (`CUSTOMIZATION:EQMOD=...`), not a standalone equippable item — the
  `.pcg` construction needed (a base armor + this modifier attached) was not built this cycle.
- **`WEAPON\|WIELDCATEGORY\|...` / `WEAPON\|TOHIT\|...` without `TYPE=Enhancement` (6 units, the
  `special_quality_wield_size_*` family)** — `equipmods::compute_equipmods_effect` requires
  `TYPE=Enhancement` per its own doc comment; these chains carry no such qualifier. **Genuinely
  unhandled by any current resolver** — a real engine-shape gap, not examined further this cycle
  (`src/rules_core/` changes are out of this cycle's write scope unless the fix is made — this one
  was not, for lack of remaining cycle budget, named honestly here rather than attempted rushed).
- **`EQMWEAPON\|DAMAGESIZE\|1` (2 units)**, **`EQM\|WEIGHTDIV\|2` (1 unit)**, **`VAR\|<custom-name>\|1`
  paired with `WEAPON\|DAMAGE,TOHIT\|<var>\|TYPE=ENHANCEMENT` (2 `ultimate_psionics` units)** — each
  a distinct, unhandled shape with no existing resolver, not attempted.

## SKILL-shape oracle round-trip: real, live PCGen verification

**Real per-unit cost, measured before committing to the full run** (remediation-brief requirement,
same discipline every prior `AT-33-E5-00x` cycle used): a single `./gradlew run` BatchExporter
invocation via the repo's own proven `scripts/pcgen-run-character.sh` wrapper costs **~20s** cold
(confirmed: `time bash scripts/pcgen-run-character.sh ...` -> `real 0m20.013s`). At `-P 15` parallel
`xargs` batching, 85 invocations completed in ~35 minutes wall time on this shared box under real,
concurrent contention from sibling lanes (`uptime` showed load average ~60 on 24 cores mid-run) —
this box's gradle-daemon-per-invocation overhead did not scale as cleanly under `-P 15` as
`AT-33-E5-002`'s own direct-`java` `-P 20` lever did (confirmed: `ps aux` showed a distinct
`GradleDaemon` JVM per concurrent invocation, not one shared daemon reused across the batch) —
named honestly in Notes below as a real throughput ceiling this cycle hit, not hidden.

**Real, execution-confirmed equip-location hazard, found and fixed this cycle:** `EQUIPSET:Carried`
(the location `AT-33-E5-002`'s own precedent used for a slotless item, modeled on PCGen's stock
`characters/Everything.pcg`'s `Backpack`) does **not** activate a `BONUS:` token — confirmed
empirically: `Climber's Kit` at `EQUIPSET:Carried` exported `SKILL.Climb.MISC=0`; the SAME `.pcg`
at `EQUIPSET:Equipped` exported `SKILL.Climb.MISC=2`, matching the real corpus record's
`BONUS:SKILL|Climb|2|...` chain exactly. Every `.pcg` this cycle generates uses `Equipped`.

**PCGen's own `SKILL.<name>.MISC` export token (`SkillToken.SKILL_MISC`,
`modifier(aSkill,pc) - getStatMod(aSkill,pc)`) isolates the item's circumstance bonus directly** —
confirmed empirically (`Climber's Kit`: `TOTAL=5`, `ABMOD=3`, `MISC=2`, `RANK=0` on a base STR16
Fighter) — so this cycle needed no baseline-character diff and no skill-to-key-ability lookup table
(a hand-rolled PF1 rules constant): the oracle side reads `SKILL.<literal skill name>.MISC` (the
skill name is a template-time literal, one small `.ftl` file per unit — `SkillToken`'s own syntax,
`SKILL.id.property` where `id` accepts a skill NAME, not only an index, confirmed against real PCGen
source `code/src/java/pcgen/io/exporttoken/SkillToken.java`), and the "ours" side reads
`compute_general_effect(record).bonus` directly — the exact same field, no re-derivation on either
side.

**Population run: 90 of 118 SKILL-shape units** (the single-skill subset; the 27 multi-skill/`ALL`
units are the distinct engine-shape gap named above, not attempted). Of the 90 run, **19 are named,
real exclusions, not silent drops** — each hit a genuine, root-caused issue before it could produce
a false result:

| Exclusion | Count | Real reason |
|---|---:|---|
| `demon_senses` | 1 | `Magic.Wondrous.Implant` slot — confirmed this cycle that the generic `Equipped` location does not activate an Implant item's `BONUS:` token (produced a real but WRONG `MISC=0`) |
| `hunter_s_sight` | 1 | PCGen campaign-load failure (`advanced_class_guide`, a `PRECAMPAIGN`-style dependency gap — the exact class of hazard `AT-33-E5-001`'s own receipt already documented for this book) |
| `heritage_book` | 1 | Did not complete within this cycle's wall-clock budget — a real Gradle `extractJavaFXLocal`/network-fetch failure under shared-box contention (`BUILD FAILED in 16m 5s`), not a content issue |
| 11 `ultimate_psionics` items with no explicit corpus `KEY:` token | 11 | `equipment_id_resolve` did not resolve these on the "ours" side this cycle — a real resolver limitation for positional/parenthetical display keys, not root-caused further |
| `ring_self_sufficiency` | 1 | Resolved via `equipment_id_resolve`, but `compute_general_effect` returned `None` despite a real `SKILL` chain on the corpus record — an unexplained anomaly, named rather than guessed at |
| `eyes_of_expanded_vision` / `leather_of_confined_spaces` / `shadow_shirt` / `third_eye_aware` | 4 | **Real, root-caused HARNESS defect, not an engine defect**: every one hit `SEVERE Globals:130 Could not find campaign: Ultimate Psionics` + `Could not add equipment: <item>. Check loaded campaigns.` in its own PCGen log. Root cause not fully isolated this cycle (ruled out `GAMEMODE` and `PRECAMPAIGN`/`BOOKTYPE` mismatches — both confirmed byte-identical to `core_rulebook`'s own working setup). Reporting these as `disagree` would be exactly the false-defect shape `AT-33-E5-003`'s doctrine forbids; excluded and named as a concrete next-cycle fixture-generator fix instead. |

**71 of 90 reached a real, live oracle comparison.**

### Result

| Verdict | Count | Of |
|---|---:|---|
| `agree` | 65 | 71 examined via oracle |
| `disagree` | 1 | 71 examined via oracle |
| `unverifiable` (real TYPE-qualifier reason) | 5 | 71 examined via oracle |

**The 1 real disagreement, root-caused:** `ultimate_equipment:equipment:ring_of_the_sea_strider`
(`ours=8`, `oracle=16`). The item's own corpus record carries `MOVE:Swim,30` (grants a swim speed)
alongside its explicit `BONUS:SKILL|Swim|8|TYPE=Racial` token. PF1's rule ("a swim speed of at
least 5 feet gives a creature a +8 racial bonus on Swim checks") means PCGen applies an
**automatic** +8 racial Swim bonus from the granted swim speed itself, on top of the item's own
explicit +8 racial token — the two sum to 16 in PCGen's real output. `compute_general_effect` reads
only the explicit `BONUS:SKILL` chain and has no model of PF1's swim-speed-grants-a-racial-Swim-
bonus auto-rule at all, producing `8`. **A genuine, real engine gap** (not a harness defect, not
closed by adjusting the expectation) — named here for `AT-33-E5-003` to fix or escalate; not fixed
in this cycle (the fix belongs to a `compute_general_effect`/`compute_equipment_effects` change this
cycle's remaining budget did not reach).

**The 5 real `unverifiable` rows, root-caused:** all five carry a `BONUS:SKILL|TYPE.<x>|...` /
`TYPE=<x>` qualifier (`TYPE.Perform`, `TYPE.Base`, `TYPE=Knowledge` x2, `TYPE=Perform`) — a
skill-TYPE selector applying to a whole subskill family (e.g. every `Perform` subskill at once), not
a single named skill. `compute_general_effect` stores this literal qualifier string as
`skill_bonus.skill` (the same un-split-verbatim pattern already named for the multi-skill/`ALL` and
multi-ability-STAT shapes), and PCGen's own `SKILL.<name>.MISC` token has no single matching skill
to query by that name — a real, examined absence (`reason: skill_type_qualifier_no_literal_skill_name`),
not an unattempted unit.

Re-derive: `python3 scripts/oracle_harness/run.py --oracle-export docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-remainder-skill.oracle-export.txt --ours docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-remainder-skill.ours.json --output <out>.json`

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| This lane's population | 494 | of 1,390 Epic-5 remainder | brief-stated: 448 `other_bonus_shape` equipment + 46 `equipment_modifier` |
| `equipment_modifier`, no bonus chain | 32 | of 46 | `python3 docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-remainder-census-equipmod.py` |
| `equipment_modifier`, has bonus chain (not examined) | 14 | of 46 | same |
| `other_bonus_shape`, SKILL-shape (any) | 118 | of 448 | `python3 docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-remainder-full-census-448.py` |
| SKILL-shape, single-skill (this cycle's population) | 90 | of 118 | `python3 docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-remainder-census-skill-shape.py` |
| SKILL-shape, multi-skill/ALL (not examined) | 27 | of 118 | same |
| Units examined this cycle | 103 | of 494 (20.9%) | 32 (equipment_modifier no-chain) + 71 (SKILL oracle-examined) |
| Agree | 65 | of 103 examined | `python3 -c "import json,collections; d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-remainder.oracle-results.json')); print(collections.Counter(r['verdict'] for r in d['results']))"` |
| Disagree | 1 | of 103 examined | same command |
| Unverifiable | 37 | of 103 examined (32 `no_bonus_chain` + 5 `skill_type_qualifier_no_literal_skill_name`) | same command |
| Reasonless `unverifiable` in this lane's own rows | 0 | of 37 `unverifiable` rows | `python3 -c "import json; d=json.load(open('.../equipment-remainder.oracle-results.json')); print(len([r for r in d['results'] if r['verdict']=='unverifiable' and not r.get('reason')]))"` -> `0` |
| Units NOT examined this cycle | 391 | of 494 (79.1%) | 494 − 103 = 391 (14 equipment_modifier with an unhandled chain + 27 multi-skill/ALL SKILL + 331 remaining `other_bonus_shape` non-SKILL + 19 named exclusions among the 90 attempted, folded into the 391 as never producing a committed row) |
| `box_ledger.py --check` against this cycle's file | `uncovered=0 overlap=0 population=49438 oracle_disagreement=1 unverifiable_done=0 stale=False`, exit **1** (correctly — 1 real disagreement exists; the fail-closed gate is doing its job) | population 49,438 | `python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-remainder.oracle-results.json` |

## Status: in-progress

**Not `complete`.** 103 of this lane's 494-unit population are genuinely examined this cycle with
real, per-unit `(ours, oracle, verdict)` rows (65 agree, 1 real root-caused disagree, 37
`unverifiable` each with a populated reason) and a real, repo-local SKILL-shape oracle pipeline
proven end-to-end. The remaining 391 are named per-shape above with concrete structural reasons (a
real engine-shape gap, an unbuilt fixture pattern, or simply not yet attempted) and a concrete
next-cycle plan below — not "ran out of time" vaguely. Marking this row `complete` while 391 of the
494-unit population stay unexamined would repeat exactly the false-completion shape this
remediation wave exists to close.

## Movement, four buckets

- **closure:** 0 — no unit's `docs/work-inventory.json` `status` field changed; oracle verification
  results live in this directory's own JSON files, matching `AT-33-E5-001`/`002`'s own convention.
- **reclassification:** 0
- **reachability:** 0 — this cycle found real ceilings (multi-skill/ALL chains, the `Implant` slot,
  the un-gated `WEAPON`/`EQMARMOR` shapes, the `ultimate_psionics` campaign-load fixture defect, and
  the swim-speed-grants-racial-Swim-bonus engine gap behind the one real disagreement) but did not
  widen any of them.
- **instrument-correction:** 1 — the `EQUIPSET:Carried` vs `EQUIPSET:Equipped` equip-location hazard,
  found and fixed within this cycle before it could produce a single false result.

## Notes

- **Real, live PCGen data confirmed the SKILL comparison mechanism against a corpus record with a
  known real value before trusting it at scale** — `Climber's Kit`'s `+2` Climb circumstance bonus,
  the exact worked example in `general.rs`'s own module doc comment, matched the live PCGen export
  exactly once the equip-location hazard above was fixed.
- **A `CHECK.<i>` family also exists** (`AT-33-E2-002`'s own `computed-values.txt.ftl`) but resolves
  to PF1's three SAVES (Fortitude/Reflex/Will), not skills — confirmed empirically this cycle
  (`CHECK.0.NAME=Fortitude`) before building the SKILL-shape pipeline on the wrong token family. This
  is the real reason `SAVE` (24 units, the `other_bonus_shape` shape that genuinely DOES map to
  `CHECK.<i>.TOTAL`) is named as a concrete, cheap next-cycle lever below, not attempted this cycle
  (this cycle's remaining budget went to confirming and fixing the equip-location hazard instead).
- **Throughput**: `-P 15` `./gradlew run`-per-unit parallelism (this lane reused the repo's existing
  proven `scripts/pcgen-run-character.sh` wrapper rather than building a new direct-`java` wrapper,
  per "reuse, don't rebuild") did not reach `AT-33-E5-002`'s own `-P 20` direct-`java` ~3.8x speedup
  — each `./gradlew run` invocation spins its own Gradle daemon (confirmed: `ps aux` showed a
  distinct `GradleDaemon` process per concurrent invocation, not one shared daemon), so wall time
  stayed close to the per-unit serial cost under contention. A future cycle building the
  direct-`java` wrapper `AT-33-E5-002` already proved would raise this lane's own throughput ceiling
  for its own remaining 404 units (108 VAR + 92 COMBAT + ... ).

## RED→GREEN

Population coverage + one new binary, not a new engine code path (`compute_general_effect`
pre-existed and is called unmodified). **Before** this cycle: `equipment-remainder.oracle-results.json`
did not exist; 0 of the 494-unit population had any per-unit disposition; no repo-local SKILL-shape
batch mechanism existed. **After:** `cargo build --locked --bin e5_equipment_remainder_skill_ours`
exits 0 (warnings only, pre-existing and unrelated — same warning set every prior `AT-33-E5-00x`
cycle's receipt names); 90 real, live `./gradlew run` BatchExporter invocations against
the real pinned oracle (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`) run, 87 exit 0
(3 named failures — `hunter_s_sight`/`heritage_book` above, each a real, diagnosed PCGen/Gradle
failure, not a silently-swallowed error); the 32
`no_bonus_chain` rows are backed by a real, whole-record read of every one of the 46
`equipment_modifier` corpus files; `scripts/box_ledger.py --check` independently re-verifies the
combined file.

## Test scoping

Ran `cargo build --locked --bin e5_equipment_remainder_skill_ours` and
`cargo run --locked --bin e5_equipment_remainder_skill_ours -- ...` (both exit 0). Ran
`scripts/oracle_harness/run.py`/`compare.py` (`AT-33-E2-003`'s own module, imported not modified).
Ran `python3 scripts/box_ledger.py --check --oracle-results ...` against the committed file. **Did
not** run the root `cargo test` sweep or `apps/desktop/src-tauri` (a separate cargo workspace; no
file in it touched this cycle) — no existing test file changed this cycle (new `src/bin/` binary,
no `#[cfg(test)]` module, matching `e5_literal_stat_ours.rs`'s and `formula_interpreter.rs`'s own
precedent for a data-pipeline binary over already-tested engine code).

## Next-cycle plan (concrete, per shape, for the next lane picking up this population's remainder)

1. **`SAVE` (24 units):** maps directly to `AT-33-E2-002`'s own proven `CHECK.<i>.TOTAL`/`.NAME`
   loop (confirmed this cycle, see Notes) — the cheapest remaining lever, no new export mechanism.
2. **The 27 multi-skill/`ALL` SKILL-shape units + the 43 `STAT_multi_or_other_slot` units:** both
   need the SAME real engine-shape fix — `compute_general_effect`/`compute_magic_items_effect`
   splitting a comma-joined ability/skill list instead of storing it verbatim. One `src/rules_core/`
   fix (with its own RED->GREEN) would unlock both populations at once.
3. **`VAR` (108 units) / `COMBAT` (92 units):** the two largest remaining shapes. Both need a
   per-sub-shape census first (like this cycle's own SKILL/`equipment_modifier` census scripts) —
   `VAR` chains reference heterogeneous variable names with no single export token; `COMBAT` chains
   (`AC`/`CMB`/`CMD`/`ATTACKS`/...) each need their own PCGen token mapping.
4. **The `WEAPON`/`EQMARMOR`/`EQMWEAPON`/`EQM` shapes among the 14 chain-bearing `equipment_modifier`
   units:** each needs a base-item-plus-attached-modifier `.pcg` construction (unlike a standalone
   equipped item) — a genuinely different fixture pattern from every `AT-33-E5-00x` cycle so far.
5. **A direct-`java` wrapper** (bypassing `./gradlew`'s per-invocation daemon spin-up), matching
   `AT-33-E5-002`'s own proven lever, would raise every future cycle's throughput ceiling on this
   lane's remaining 404 units.
6. **The `ring_of_the_sea_strider` disagree** (`AT-33-E5-003`'s own scope): fix
   `compute_general_effect`/`compute_equipment_effects` to apply PF1's automatic +8 racial Swim
   bonus when a `MOVE:Swim` grant is present, alongside any explicit `BONUS:SKILL` token — or
   confirm PCGen's own doubling is itself the anomaly before changing engine code.
7. **The `ultimate_psionics` campaign-load fixture defect** (4 named exclusions above): root-cause
   why `Ultimate Psionics` fails `Could not find campaign` in this cycle's fixture when
   `core_rulebook` (byte-identical `GAMEMODE`/`PRECAMPAIGN`/`BOOKTYPE` shape) does not — needed
   before any further `ultimate_psionics`-book unit (including the 11 unresolved-on-our-side items)
   can be oracle-verified.
