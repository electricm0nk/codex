# Cycle AT-33-E5-shape-var — Epic 5 Re-verification / AT-33-E5-002 (VAR-shape sub-population)

- **Commit SHA:** `<pending — recorded by the same-cycle follow-up push, matching `AT-33-E5-remainder-equipment_cycle_receipt.md`'s own convention>`
- **Files touched:**
  - `src/rules_core/equipment_effects/general.rs` (new `compute_var_effect`/`VarBonus` — real, live engine resolver for `BONUS:VAR|<name(s)>|<value>` equipment chains, genuinely unhandled by any resolver before this cycle; 4 new tests, TDD RED confirmed via `cannot find function compute_var_effect`/`cannot find type VarBonus` before the implementation existed)
  - `src/bin/e5_var_shape_ours.rs` (new — repo-local batch "ours" probe, real live calls into `compute_var_effect` via `equipment_id_resolve`, same pattern as `e5_equipment_remainder_skill_ours.rs`)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-shape-var.oracle-results.json` (new — the 108-unit committed deliverable)
  - `docs/release/SD-33-computed-value-verification/progress.md` (this cycle's entry appended)
  - `docs/retro/events/sd33-r3-var.jsonl` (new — 1 correction, 1 incident)

- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS

- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-002 — the 6,589 `literal-verified` units are re-examined
  >
  > **Evidence:** per-unit `(ours, oracle, verdict)` rows committed; agreement and disagreement
  > counts both stated, with the denominator.

## What this lane owns

`AT-33-E5-remainder-equipment_cycle_receipt.md`'s own next-cycle plan named `VAR` (108 units) as one
of "the two largest remaining shapes", needing "a per-sub-shape census first... `VAR` chains
reference heterogeneous variable names with no single export token." This lane is that census, plus
the full 108-unit oracle round-trip.

## The shape, established first (per the brief's own instruction)

Every one of the 108 `VAR`-shape units carries one or more `BONUS:VAR|<name(s)>|<literal-value>`
chains (confirmed this cycle by reading every unit's **whole** corpus record, not a BONUS/PRE-only
filtered view: zero non-literal/formula-valued chains, zero `PRE`-gated chains among the 108 —
re-derive: `python3 -c "..."` over `equipment-remainder-full448-labels.json`'s VAR subset, see
Figures). 83 distinct variable names appear (after splitting 11 comma-joined multi-name chains, e.g.
`CMD_Disarm,CMD_Sunder`).

**What each VAR name resolves to, and whether our engine resolves it the same way:** PCGen exposes
every named variable via its own `VAR.<name>.INTVAL` export token (`plugin.exporttokens.VarToken`,
confirmed against the real pinned PCGen source, `pc.getVariable(name, true)`). Our engine had **zero**
resolver for this family before this cycle — `compute_intelligent_item_effect` (the only prior `VAR`
reader in the repo) matches exactly 5 hardcoded names (`IntItemStatINT/WIS/CHA`, `IntelligentItemEgo`,
`IntItemAlignment`), none of which appear among this population's 83 names. `compute_var_effect`
(this cycle) is a real, general resolver: every `BONUS:VAR|<name(s)>|<literal>` chain becomes one
`VarBonus{name, bonus}` row per named variable, splitting comma-joined names — the item's own literal
contribution, read the same way `compute_general_effect`'s `SKILL` chain already is.

**Whether the variable is character-dependent or item-local — established empirically, not guessed:**
every one of the 83 names is itself `DEFINE:<name>|0`'d inside a specific race/class/feat ability
record (confirmed by grepping the pinned PCGen `data/pathfinder` tree for each name's `DEFINE:` line
— e.g. `LOADSCORE` inside the `STR` stat's own always-granted definition, `WeaponTrainingBase` inside
the Fighter "Weapon Training" class feature, `ShifterAspectMinutes` inside the Shifter "Shifter
Aspect" class feature, `PsiCrystalLVL` inside the "Psicrystal Affinity" feat). A real, live PCGen
probe on a plain Level-1 Human Fighter (`STR16/DEX14/CON14/INT10/WIS10/CHA8`, Core Rulebook only, no
items) confirmed PCGen's variable engine resolves an **undefined** variable name to `0` uniformly
(confirmed with a deliberately made-up name, `NoSuchVarAtAll12345`, which also resolved `0`) — and of
the 83 names, exactly the ones tied to universally-granted internal abilities (`LOADSCORE`, `CMD`,
`CMD_Disarm`/`CMD_Sunder`/`CMD_Trip`/`CMD_Grapple`/`CMD_BullRush`/`CMD_Bullrush`, `CMB_Grapple`,
`CMB_Overrun`, `ArmorCheckPenalty`, every `*ResistanceBonus`/`*Save_ResistanceBonus`/`*Save_LuckBonus`)
resolved **nonzero** on this generic baseline; every class/feat-gated name (`WeaponTrainingBase`,
`KiPoolLVL`, `BloodlineLVL`, every `Shifter*`, `PsiCrystalLVL`, every `*PathLVL`, every
`*DisciplineLVL`, `CavalierChallengeLVL`, `SmiteEvilLVL`, ...) resolved `0`.

## Where PCGen and our engine agree vs. disagree — established, not assumed

Population run: **all 108 of 108** units carry a real `(ours, oracle, verdict)` row.

### `ours`

`e5_var_shape_ours` (real live calls into `compute_var_effect`) resolved **105 of 108** units to at
least one `VarBonus` row. **3 units did not resolve at all** — a real, root-caused, already-precedented
resolver limitation: `equipment_id_resolve`'s three match rules (exact `KEY:` token, exact `name`,
normalized `name`) all miss a record that carries **no raw `KEY:` token** (its LST column-0 name is
its only identity) **and** whose JSON `"key"` field differs from its JSON `"name"` field (an
`OUTPUTNAME:`-driven display override) — e.g. `Backpack (Masterwork)`'s corpus `"key"` is `"Backpack
(Masterwork)"` but its loaded `EquipmentRecord.name` is `"Backpack, Masterwork"` (from `OUTPUTNAME`),
so none of the three match rules fire. This is the **same class** of gap
`AT-33-E5-remainder-equipment_cycle_receipt.md` already named ("11 `ultimate_psionics` items with no
explicit corpus `KEY:` token... a real resolver limitation for positional/parenthetical display keys,
not root-caused further") — confirmed to recur here, not re-litigated. `equipment_id_resolve` is a
shared resolver used by every equipment lane; widening it is out of this cycle's write scope (a
compute-path change with its own blast radius, not a `var`-shape-local fix).

### `oracle` — real, live PCGen exports

**Real per-unit cost, measured before the full run:** the direct-`java` invocation
(`scripts/oracle_harness/charbuild_remainder_run_one.sh`, `AT-33-E5-002`'s own proven lever, reused
unmodified) costs ~19-31s cold per JVM start, dominated by campaign/game-data bootstrap, not by how
many items or variables one character carries (confirmed: a 1-item bin and a 49-item bin both landed
in the same ~19-35s range). **This is the lever that makes 108 units cheap**: one exported character
carrying every VAR-touching item from one book verifies that whole book's units in one JVM start.
Books were kept separate per character after a real, execution-confirmed cross-book failure (see
Notes) — **48 book-homogeneous batch characters** (sizes 1-21) cover all 108 units, each diffed
against a **matching-context baseline character** (same campaigns, zero items) run for that same bin,
avoiding any risk of a splatbook's own optional content shifting a variable's default. Projected vs.
actual: 48 batch + 48 baseline + 14 isolated re-verification (see Notes) = 110 real JVM invocations;
at `-P 10..12` parallel and ~20-30s/invocation this is **well under 10 minutes wall time** — confirmed
directly (all three waves completed inside single `xargs -P` calls this cycle).

## Result

| Verdict | Count | Of |
|---|---:|---|
| `agree` | 44 | 108 var-bonus-shape units |
| `disagree` | 1 | 108 var-bonus-shape units |
| `unverifiable` | 63 | 108 var-bonus-shape units |

**`unverifiable` breakdown (every row has a populated, real, per-unit reason — zero reasonless):**

| Reason | Count | Of |
|---|---:|---|
| `equipment_id_resolve_no_match_keyless_outputname_record` | 3 | 63 unverifiable rows |
| `var_gated_by_unbuilt_class_feature_zero_on_generic_baseline` | 60 | 63 unverifiable rows |

The 60 `var_gated_...` rows are the population this cycle's brief anticipated: a `VAR` whose value is
genuinely indeterminate without the RIGHT character context (a Shifter for `ShifterAspectMinutes`, a
Fighter with Weapon Training levels for `WeaponTrainingBase`, a Psicrystal-feat character for
`PsiCrystalLVL`, ...) — confirmed **empirically, per unit**, not assumed: each row carries its own
`ours`/`oracle` values so a reader can see exactly which name(s) were gated (e.g.
`pauldrons_of_the_serpent`: 3 of its 4 touched names — `CMD_Bullrush`/`CMD_Grapple`/`CMD_Trip` —
agreed cleanly; only `CMD_Reposition`, a non-core-universal maneuver name, was gated). This is a real
finding, not a parking space: every one of these 60 units WAS fed to the harness and produced a real
oracle read; the read is `0` because the underlying PF1 mechanic genuinely does not exist on a
generic Level-1 Human Fighter, the same fact confirmed directly against the pinned PCGen source's own
`DEFINE:` placement.

**The 1 real disagreement, root-caused:**
`inner_sea_races:equipment:panoply_of_the_fierani_knight` (`ours=6`, `oracle=3`). The item's own
corpus record carries `BONUS:VAR|ArmorCheckPenalty|6` (no `TYPE=`/`PRE` qualifier) **and** an
`ACCHECK:-6` token **and** an attached `EQMOD` naming `Material ~ Mithril ~ Armor / Heavy`. Real PF1
rule: Mithril reduces armor check penalty by 3, and PCGen's own Mithril material EQMOD carries **its
own separate** `BONUS:VAR|ArmorCheckPenalty` reduction chain, living on the **attached modifier**
record, not this item's own base record. `compute_var_effect` (record-level only, this cycle's real
scope) correctly reads the base record's own literal `+6` — that number is not wrong for what it
claims to read — but the TRUE compound value (base `+6` and the auto `ACCHECK`-derived contribution
and the Mithril EQMOD's own `-3`) needs a base-item-plus-attached-modifier composition
`compute_var_effect` does not, and at this cycle's scope should not, attempt. **This is the same
`EQMARMOR` base-item-plus-attached-modifier fixture-construction gap
`AT-33-E5-remainder-equipment_cycle_receipt.md` already named** ("each needs a base-item-plus-
attached-modifier `.pcg` construction, unlike a standalone equipped item") — confirmed to recur here
for `VAR`, not a fresh defect, and not smoothed into `unverifiable` (per `AT-33-E5-003`'s doctrine, a
disagreement is root-caused, not closed by adjusting the expectation). **8 other units in this
population carry the same `ACCHECK:` + material-`EQMOD` shape** (`armor_of_grim_triumph`,
`coat_of_shells`, `gnome_scrap_armor`, `goblin_plate`, `hallowed_chain`, `hallowed_chain_greater`,
`hide_of_grim_triumph`, `mail_of_sly_steps`, `diviner_s_blight`) and all 9 (including this one) were
individually checked this cycle; the other 9 happened to `agree` (their material is Steel/Leather, not
Mithril, or their `ACCHECK` is `0`, so the auto/EQMOD contribution this cycle's harness cannot isolate
happens to be inert for them) — a coincidence of this population's real content, not evidence the
methodology generalizes safely to a Mithril-material item, which is exactly what
`panoply_of_the_fierani_knight` demonstrates.

## A real methodology defect this cycle found and fixed BEFORE it reached the committed results

**First-pass batching produced 5 apparent disagreements that were not real** (logged as a `retro.py
correction` this cycle, `docs/retro/events/sd33-r3-var.jsonl`): `CMD_Disarm`/`CMD_Sunder`/`CMD_Trip`/
`CMD_Grapple`/`CMD_Bullrush`/`CMD_BullRush`/`CMD_Reposition`/`CMB_Grapple`/`CMB_Overrun`/
`CMB_Reposition` are not independent accumulators — PCGen's own core rules define each as `BONUS:VAR|
CMD_X|CMD` (a **live formula** re-evaluated from the character's current `CMD`, confirmed via the
pinned PCGen source, `core_rulebook/cr_abilities_class.lst:2906`). Batching multiple CMD/CMB-family
items on ONE character (this lane's own book-homogeneous batching, e.g. `ultimate_equipment` bin 23:
15 items on one character) meant an item that bonuses bare `CMD` itself (`juggernaut_s_pauldrons`,
`+4`) or `unlucky_figurine` (`-2`) silently shifted **every other CMD-derived unit's own oracle
reading in that same batch** — `acrobat_slippers` (`CMD_Trip+2`) read oracle delta `6` instead of `2`
in the first pass purely because `juggernaut_s_pauldrons`'s own `+4` to base `CMD` was also equipped.
**Caught before this cycle's committed results, not after**: identified all 14 units in this
population whose `var_map` touches `CMD`/`CMB`/`CMD_*`/`CMB_*` and re-ran every one of them in
**isolation** (one item per character, matching `AT-33-E5-001`'s own original precedent) against the
already-measured universal baseline. All 5 false disagreements resolved to real `agree` rows on
re-verification; the final committed file reflects the isolated re-run for all 14 CMD/CMB-family
units, not the contaminated first pass.

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| This lane's population | 108 | of 391 unexamined Epic-5 equipment units | brief-stated (`VAR` label, `equipment-remainder-full448-labels.json`) |
| Population re-derived this cycle | 108 | of 391 | `python3 -c "import json; d=json.load(open('artifacts/epic-5-reverification/equipment-remainder-full448-labels.json')); print(len([u for u,l in d.items() if 'VAR' in l]))"` |
| Units with a non-literal or `PRE`-gated `VAR` chain | 0 | of 108 | swept this cycle over every unit's whole corpus record, see census script in Notes |
| Distinct variable names (post comma-split) | 83 | of 108 units' chains | swept this cycle, see Notes |
| Rows committed | 108 | of 108 (population) | `python3 -c "import json;d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-shape-var.oracle-results.json'));print(len(d['results']))"` -> `108` |
| `agree` | 44 | of 108 examined | `python3 -c "import json,collections;d=json.load(open('.../equipment-shape-var.oracle-results.json'));print(collections.Counter(r['verdict'] for r in d['results']))"` |
| `disagree` | 1 | of 108 examined | same command |
| `unverifiable` | 63 | of 108 examined | same command |
| Reasonless `unverifiable` in this lane's own rows | 0 | of 63 `unverifiable` rows | `python3 -c "import json;d=json.load(open('.../equipment-shape-var.oracle-results.json'));print(len([r for r in d['results'] if r['verdict']=='unverifiable' and not r.get('reason')]))"` -> `0` |
| `ours`-side resolver misses (`equipment_id_resolve` no match) | 3 | of 108 | folded into the `unverifiable` count above, see "What this lane owns" |
| `box_ledger.py --check` against this cycle's file | `uncovered=0 overlap=0 population=49438 oracle_disagreement=1 unverifiable_done=0 stale=False`, exit 1 (correctly — 1 real disagreement, the fail-closed gate doing its job) | population 49,438 | `python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-shape-var.oracle-results.json` |

## Status: complete

108 of this lane's 108-unit population carry a real per-unit `(ours, oracle, verdict)` row (44 agree,
1 real root-caused disagree, 63 unverifiable each with a populated real reason). Row count mechanically
re-derived above (`108`), matching the population exactly.

## Movement, four buckets

- **closure:** 0 — no `docs/work-inventory.json` `status` field changed; oracle verification results
  live in this directory's own JSON, matching `AT-33-E5-001`/`002`/`AT-33-E5-remainder-equipment`'s own
  convention.
- **reclassification:** 0
- **reachability:** 0 — real ceilings found this cycle (the keyless-`OUTPUTNAME` resolver gap, the
  base-item-plus-attached-material-EQMOD compound gap, the class/feat-gating structural fact) but none
  widened.
- **instrument-correction:** 1 — the CMD/CMB-family batch-cross-contamination methodology defect,
  found and fixed within this cycle before it reached the committed results (see retro.py `correction`
  event and the section above).

## Notes

- **Census/full-record read, this cycle:**
  `python3` scripts under `/tmp/sd33-r3-var/work/` (scratch, not committed — the committed deliverable
  is the results JSON and the two source files) built the 108-unit VAR census from
  `equipment-remainder-full448-labels.json` + every unit's whole `data/corpus/.../equipment/*.json`
  record (not a BONUS/PRE-filtered view — confirmed zero `STACK`/`MULT` fields exist in this corpus's
  equipment schema at all; the governing metadata that exists, `TYPE=` stacking qualifiers, was read
  and confirmed irrelevant to isolating one item's own literal magnitude).
- **Real, execution-confirmed cross-book campaign-load hazard, found and fixed this cycle:** an early
  attempt batching items from multiple DIFFERENT Paizo "Inner Sea" campaign-setting books
  (`Inner Sea Gods` + `Inner Sea Races` + `Inner Sea World Guide`) onto ONE character crashed
  `SourceFileLoader` (`IllegalStateException: Cannot ask for resolution: Reference Tiefling ~ Maw or
  Claw (%LIST) has not been resolved`) — real PCGen behavior, not a fixture bug on this cycle's side.
  Fixed by keeping batches book-homogeneous (one book's items per character) rather than the
  kitchen-sink multi-campaign approach first attempted. Logged as a `retro.py incident`
  (`pcgen-precampaign-dependency-gap`), the same class `AT-33-E5-001`'s own receipt already documented
  for `hunter_s_sight`/`advanced_class_guide`.
- **Multi-value units:** a unit whose chains touch more than one variable name (11 of 108, e.g.
  `robe_of_arcane_heritage`: `BloodlineLVL`/`BloodlineProgressionLVL`/`BloodlineFeatProgression`/
  `EldritchHeritageBloodlineLVL`) stores `ours`/`oracle` as a `{name: value}` object rather than a bare
  scalar — `box_ledger.py --check` inspects only the `verdict` field, so this does not affect the
  denominator gate; the convention is named here for the next reader.

## RED→GREEN

`compute_var_effect`/`VarBonus` genuinely did not exist before this cycle (confirmed: `general.rs`
only ever matched `qualifiers[0] == "SKILL"`; the repo's only `VAR` reader,
`compute_intelligent_item_effect`, matches exactly 5 hardcoded names, none in this population).
**Before:** `cargo test --locked --lib rules_core::equipment_effects::general::` failed to compile —
`error[E0425]: cannot find function 'compute_var_effect' in this scope` / `error[E0422]: cannot find
struct 'VarBonus'` (4 new tests written first, real corpus-verbatim fixtures: `muleback_cords`,
`gloves_of_dueling`, `cloak_of_resistance_1`, `climbers_kit`). **After:** `cargo test --locked --lib
rules_core::equipment_effects::general::` -> `test result: ok. 9 passed; 0 failed`. `cargo build
--locked --bin e5_var_shape_ours` exits 0. `cargo build --locked --bins` (full workspace bin sweep)
exits 0, warnings only (pre-existing, unrelated — the same set every prior `AT-33-E5-00x` cycle's
receipt names).

## Test scoping

Ran `cargo test --locked --lib rules_core::equipment_effects::` (62 passed, 0 failed — the whole
module, not just `general`, since `compute_var_effect` lives alongside `compute_general_effect` and a
sibling-module regression is the real risk of touching this file). Ran `cargo build --locked --bins`
(full binary sweep, catches a broken bin target the way `AGENTS.md`'s own Concurrency & Measurement
lesson requires). Ran `scripts/verify.sh --only denominator-gate` (PASS, `files_checked=24
violations=0`). Ran `python3 scripts/box_ledger.py --check --oracle-results
.../equipment-shape-var.oracle-results.json`. **Did not** run the root `cargo test` full sweep or
`apps/desktop/src-tauri` (a separate cargo workspace; no file in it touched this cycle) — matching
every prior `AT-33-E5-00x` cycle's own precedent for a data-pipeline-plus-one-small-resolver change.

## Next-cycle plan

1. **The keyless-`OUTPUTNAME` resolver gap (3 units here, 11 more already named for `SKILL`-shape by
   `AT-33-E5-remainder-equipment_cycle_receipt.md`):** widen `equipment_id_resolve` to also match a
   JSON-corpus record's own `"key"` field directly (not only `record.name`/`KEY:` token) when the
   record was reconstructed by `corpus_loader.rs` from JSON without a raw `KEY:` token. A shared-
   resolver change with its own blast radius across every equipment lane — its own cycle.
2. **The base-item-plus-attached-material-EQMOD compound `ArmorCheckPenalty` gap** (1 confirmed
   disagreement here, `AT-33-E5-remainder-equipment_cycle_receipt.md`'s own `EQMARMOR` family): both
   need the same fixture-construction pattern (base armor + attached `EQMOD`), a materially different
   `.pcg` shape from every `AT-33-E5-00x` cycle's standalone-item pattern so far.
3. **`COMBAT` (92 units, `AT-33-E5-remainder-equipment_cycle_receipt.md`'s own next-largest lever):**
   this cycle's book-homogeneous-batching + isolated-re-verification-for-live-formula-families
   discipline (see "methodology defect" section above) applies directly — `COMBAT|AC`/`CMB`/`CMD`
   chains will hit the exact same CMD/CMB live-formula interaction this cycle found, so batch
   COMBAT-shape units with the same care from the start rather than discovering it mid-cycle again.
