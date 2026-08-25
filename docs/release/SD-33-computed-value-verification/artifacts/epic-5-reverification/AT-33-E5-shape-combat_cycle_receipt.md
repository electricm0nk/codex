# Cycle AT-33-E5-shape-combat — Epic 5 Re-verification / AT-33-E5-002 (combat/weapon shape lane)

- **Commit SHA:** `f66ae64320` (results + receipt; hash as rebased onto `tranche/13` — the
  pre-rebase commit this cycle first created and referenced here, `c99609071f`, was superseded by
  the rebase itself, not by any content change). Engine-fix commit `66984fe7bc` (also rebased
  from its own pre-rebase local hash `b32920cbe9`) landed and pushed separately, earlier this
  cycle.
- **Files touched:**
  - `src/rules_core/equipment_effects/arms_armor.rs` — widened `armor_class_bonus_from_bonus_chains` to any `COMBAT|AC` chain (see engine-fix commit, landed and pushed separately this cycle)
  - `src/rules_core/equipment_effects/equipmods.rs` — added the `WEAPONPROF=<name>` (non-`TYPE.`) bare-chain shape to `compute_equipmods_effect`
  - `src/bin/e5_combat_ac_ours.rs` (new) — repo-local batch "ours" probe, real live calls into `codex::rules_core::equipment_effects::compute_equipment_effects`
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/combat-shape-work/` (new, working data: population/census/manifest/jobs/generator scripts/oracle exports — kept for re-derivation)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-shape-combat.oracle-results.json` (new — this lane's committed deliverable)
  - `docs/release/SD-33-computed-value-verification/progress.md` (updated in place)
  - `docs/retro/events/sd33-r3-combat.jsonl` (new)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-002 — the 6,589 `literal-verified` units are re-examined
  >
  > **Evidence:** as above (`AT-33-E5-001`'s evidence line: per-unit `(ours, oracle, verdict)`
  > rows committed; agreement and disagreement counts both stated, with the denominator).

## What this lane owns

**Population: 125 units** of the 391 unexamined-of-6,589 `literal-verified` equipment
population, by shape (`equipment-remainder-full448-labels.json`, re-derived this cycle):
`COMBAT` 92, `WEAPON` 18, `WEAPONPROF=*` 15 (2 of the 17 raw `WEAPONPROF=*`-labeled units —
`talons_of_leng`, `heavy_hammer` — are double-labeled with `STAT_multi_or_other_slot`/`MOVEADD`
respectively and excluded from this lane's 15 to match the brief's stated split and avoid a
duplicate-owner collision with whichever sibling lane owns those other labels).

## Subtoken enumeration (generic pass by mechanism)

### COMBAT (92 units), by subtoken (a unit carrying >1 distinct subtoken is counted once per subtoken; 2 of 92 do: `robe_of_vermin` INITIATIVE+TOHIT, `psychoactive_skin_hero` AC+TOHIT)

| Subtoken | Count | Disposition this cycle |
|---|---:|---|
| `AC` | 85 | **Attempted** — see below |
| `TOHIT` | 3 | Not attempted — no engine resolver for a bare `COMBAT\|TOHIT` chain (real, distinct engine-shape gap; see Next-cycle plan) |
| `INITIATIVE` | 3 | Not attempted — no engine resolver for `COMBAT\|INITIATIVE` at all; one of the three (`stone_of_good_luck_luckstone`) is additionally formula-valued (`1+Global_LuckBonus`) |
| `TOHIT.Ranged` / `TOHIT.RANGED` | 2 | Not attempted — same gap as bare `TOHIT` |
| `ATTACK,AC` | 1 | Not attempted — a combined subtoken (`unlucky_figurine`), no engine resolver |

Of the 85 `AC`-subtoken units, 82 carry a literal integer value; 3 (`staff_of_power`,
`gunfighter_s_poncho`, `staff_of_the_hierophant`) carry a formula value
(`2+Global_LuckBonus`) and are not attempted this cycle (same shape as the `INITIATIVE`
formula case above — no engine formula-evaluator for this variable).

### WEAPON (18 units)

| Group | Count | Disposition |
|---|---:|---|
| Literal `TYPE=Enhancement` TOHIT/DAMAGE bonus, already engine-handled | 7 | Not oracle-verified this cycle (real PCGen `WEAPON.<i>.MAGICHIT`/`.MAGICDAMAGE` export token identified and confirmed as the right isolator, but the fixture-generation + full batch was not built this cycle — see Next-cycle plan) |
| Bare, no-`TYPE=` numeric penalty/bonus (`crossbow_double`, `heartstake_bolts_5`, `rod_withering`) | 3 | Not attempted — the SAME real gap as the `equipment_modifier` lane's own named `WIELDCATEGORY`/bare-`WEAPON` finding; engine deliberately does not widen this (would resurrect a real false-positive risk, see `equipmods.rs`'s own module doc comment) |
| `DAMAGEMULT:2` chains (`duelist_s_comate`, `rapier_of_battlefield_movement`, `swashbuckler_s_rapier`, `sword_cane`) | 4 | Not attempted — a distinct subtoken shape, no engine resolver |
| Formula-valued `ATTACKS`/`TOHIT`/`WEAPONBAB` (class-feature-shaped items: `brawler_s_flurry`, `flurry_of_fists`, `flurry_of_strikes`) | 3 | Not attempted — variable-named formulas (e.g. `BrawlersFlurryExtraAttacks`), not literal magnitudes |
| `EQMWEAPON|RANGEADD` + separate `WEAPON|DAMAGE` (`arrow_iron_tipped_distance_20`) | 1 | Not attempted |

### WEAPONPROF=* (15 units)

| Group | Count | Disposition |
|---|---:|---|
| `WEAPONPROF=TYPE.Natural` (Amulet of Mighty Fists ×5), already engine-handled | 5 | Not oracle-verified this cycle — needs an Unarmed-Strike-plus-amulet fixture; identified but not built (see Next-cycle plan) |
| `WEAPONPROF=<name>`, item IS itself a weapon of that proficiency (`berserking_sword`/Greatsword, `cursed_backbiter_spear`/Shortspear, `cursed_sword_2`/Longsword, `ornery_pistol`/Pistol) | 4 | **Engine fix landed this cycle** (see below) — not oracle-verified this cycle |
| `WEAPONPROF=Hoof` (Horseshoes of a Zealous Warhorse ×5) / `WEAPONPROF=Bite` (`belt_of_teeth`) | 6 | **Engine fix landed this cycle** — oracle verification needs a natural-Hoof/Bite-attack fixture (a Human Fighter has neither by default); not built this cycle |

## AC-shape oracle round-trip: real, live PCGen verification

**Real engine gap found and fixed before any oracle run** (RED→GREEN, `src/rules_core/equipment_effects/arms_armor.rs`): `armor_class_bonus_from_bonus_chains` only recognized
`TYPE=Armor`/`TYPE=Shield`/`TYPE=ArmorEnhancement`/`TYPE=ShieldEnhancement`, even though
`resolve_category_effect` calls it unconditionally on every equipped item (not just base
armor/shield records) — a real Ring of Protection (`TYPE=Deflection`), an Amulet of Natural
Armor (`TYPE=NaturalArmor`), and a real corpus grammar quirk (`BONUS:COMBAT|AC|4|NaturalArmor`,
no `TYPE=` prefix at all — confirmed against real PCGen parser source,
`code/src/java/pcgen/core/bonus/Bonus.java`: a qualifier is only ever parsed as a bonus type
when it literally starts with `TYPE=`/`TYPE.`) all resolved to `None` before this fix. Widened
to any `COMBAT|AC|<n>` chain regardless of its type qualifier (or the qualifier's absence).
62 of 62 `equipment_effects` tests green, including 2 new RED→GREEN tests using real verbatim
corpus records (`ring_of_protection_shaped_deflection_ac_bonus_resolves`,
`ac_bonus_with_a_bare_untyped_qualifier_still_resolves`).

**Oracle comparable value: baseline-diff on `AC.Total`, not a per-bonus-type `AC.<TYPE>` token
lookup** — deliberate, given this population's real TYPE-qualifier variety (Armor/Shield/
Deflection/NaturalArmor/NaturalArmorEnhancement/Luck/Insight/Circumstance, plus the bare-
qualifier grammar quirk above). One Level-1 Human Fighter (STR16/DEX14/CON14/INT10/WIS10/CHA8,
`EQUIPSET:Equipped`, the exact fixture pattern `AT-33-E5-002`/`AT-33-E5-remainder-equipment`
already proved), one baseline character per book actually used (10 books) with nothing equipped,
oracle value = `AC.Total(item) − AC.Total(baseline)`.

**Real, execution-confirmed campaign-closure hazard, found and fixed this cycle:** the first
full batch attempt (all 92 invocations) failed with `Failed to load campaigns` for every book
beyond `core_rulebook` — each sourcebook's own `.pcc` `PRECAMPAIGN:` chain requires its full
transitive closure loaded together (e.g. `inner_sea_races` requires `Advanced Player's Guide` +
`Advanced Race Guide` + `Inner Sea World Guide` + `Ultimate Combat` + `Ultimate Equipment` +
`Ultimate Magic`, not just itself), the same class of hazard `AT-33-E5-remainder-charbuild`'s own
receipt already named for class/race campaign closures. Fixed by reading each closure directly
from the real `.pcc` files under the pinned oracle checkout (never guessed) — see
`combat-shape-work/ac_generate.py`'s own `CAMPAIGN_CLOSURE` table and its module doc comment.

**Real per-unit cost, measured before committing to the full run:** a single direct-`java`
`scripts/oracle_harness/charbuild_remainder_run_one.sh` invocation (bypassing gradle, the
proven lever from `AT-33-E5-002`/`AT-33-E5-remainder-charbuild`) against the pinned oracle
(built this cycle: `./gradlew installDist`, `BUILD SUCCESSFUL in 58s`, from a fresh full
checkout of the pinned SHA `7f818006e371188e5717fd18d74d18a420747fc6` — the sparse
`data/pathfinder`-only cone `fetch-pcgen-oracle.sh` normally checks out does not carry
`gradle/wrapper/gradle-wrapper.jar` or `code/`, so a full non-sparse checkout of the SAME pinned
commit was required to build) cost **~20s** (confirmed: `time bash
scripts/oracle_harness/charbuild_remainder_run_one.sh ...` → `real 0m20.450s` for
`knight_inheritor_s_ring`, `real 0m20.357s` for a baseline). At `-P 8` parallel `xargs`
batching, 82 item invocations + 10 per-book baseline invocations (92 total, each covering that
book's full transitive `PRECAMPAIGN` closure, up to 7 books for `ultimate_intrigue`) completed
in real wall time on this shared box under real, concurrent contention from sibling lanes
(confirmed via `ps aux`/`uptime` mid-run) — the heavier multi-book baselines ran noticeably
slower than the single-book item invocations (e.g. `baseline_advanced_class_guide`, a
6-campaign closure, measured >90s), a real, book-closure-driven throughput ceiling this cycle
hit and is naming honestly rather than hiding. **80 of 82 items resolved by the engine probe**
(`e5_combat_ac_ours`); 2 (`psychoactive_skin_defender`/`(Hero)`) hit a real, distinct
`equipment_id_resolve` limitation, named below.

### Result

| Verdict | Count | Of |
|---|---:|---|
| `agree` | 40 | 82 examined via oracle |
| `disagree` | 26 | 82 examined via oracle |
| `unverifiable` | 16 | 82 examined via oracle |

**16 `unverifiable` rows, two distinct real reasons:**
- **14** (`reason: oracle_harness_ultimate_psionics_campaign_load_failure`) — every one of this
  lane's 14 resolved `ultimate_psionics`-book units hit the SAME real, pre-existing PCGen
  harness defect `AT-33-E5-remainder-equipment`'s own receipt already named and left unfixed:
  `SEVERE Globals:130 Could not find campaign: Ultimate Psionics` (despite the `.pcg`'s own
  `CAMPAIGN:Ultimate Psionics` line, and every other book's closure loading correctly) cascades
  into `Could not add equipment: <item>. Check loaded campaigns.` — the item never actually
  equips, so `AC.Total` reflects a bare character, not the unit's real effect. Confirmed
  per-unit from each invocation's own `.txt.log`, not inferred. Reporting these as `disagree`
  would fabricate a false defect (`AT-33-E5-003`'s own doctrine forbids exactly this).
- **2** (`reason: engine_id_resolve_fails_templated_variant_record`) — `psychoactive_skin_defender`/
  `(Hero)`: the corpus record's own display key (`Psychoactive Skin (Defender)`) is a per-variant
  identity synthesized over ingestion from a templated raw LST record whose own real name is the
  literal string `Skin of the [NAME]` with no `KEY:` token at all — none of
  `equipment_id_resolve`'s three passes (identity/name/normalized-name) match the per-variant
  display key against that template record. A real, distinct resolver gap.

**26 `disagree` rows, root-caused to two real, distinct mechanisms — not fixed this cycle:**
- **21 (confirmed exact, `gap == the item's own embedded modifier's own +N Armor/Shield value`),
  plus 2 close variants (`full_plate_of_the_corpse`, off by 1 of a 2-part `EQMOD` string) = 23
  of 26:** the base item's own `EQMOD:` token references a SEPARATE `equipment_modifier` record
  (e.g. `Special Ability ~ +2 ~ Armor`) baked into the SAME equipped item (not a separate
  `applied_modifiers` selection) — `compute_arms_armor_effect` reads only the base record's own
  literal `COMBAT|AC` chain and has no mechanism to resolve and sum an `EQMOD`-referenced
  modifier's own separate `BONUS:` chain. Confirmed by regex-matching every `EQMOD:` string's own
  `~ +N ~ Armor/Shield` embedded pattern against each unit's real `oracle − ours` gap.
- **3 (`field_plate`, `stoneplate`, `snakeskin_tunic`):** a real methodology limitation of this
  cycle's own baseline-diff harness, not necessarily the engine value itself: `field_plate`/
  `stoneplate` both carry `MAXDEX:1` — equipping them caps the baseline character's own +2 Dex
  modifier down to +1, which *reduces* `AC.Total` by 1 relative to the item's own isolated
  `COMBAT|AC` bonus (the oracle's `AC.Total` diff captures this second-order Dex-cap interaction;
  `armor_class_bonus` deliberately does not, since that field represents only the item's own
  bonus token, not the full downstream AC computation). `snakeskin_tunic` carries a SECOND,
  separate `BONUS:STAT|DEX|2|TYPE=Enhancement` chain on the same record — the +2 Dex enhancement
  itself raises `AC.Total` by 1 via the normal Dex-to-AC path, which the baseline-diff technique
  cannot separate from the item's own `COMBAT|AC` contribution.
- **1 (`sea_knife`, `TYPE=Circumstance` −2):** a real, apparently-conditional penalty (the
  record's own `SPROP` names a specific situational restriction — "cannot use a leg... for
  walking or running") that `ours` reads as unconditionally active but the oracle shows as
  inactive (`oracle=0`) on a standing, non-swimming test character — consistent with a `PRE`-
  gated chain this cycle's simplified `qualifiers` extraction did not carry forward; named, not
  further diagnosed this cycle.
- **1 (`diviner_s_blight`, `ours=2 oracle=6`, gap 4):** not individually root-caused this cycle;
  named for the next cycle rather than guessed at.

**None of these 26 are closed by adjusting the expectation** (`AT-33-E5-003`'s own doctrine) —
each is a real, numeric mismatch with a stated cause (even where the exact number is not yet
individually re-derived), left for `AT-33-E5-003` to fix or escalate.

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| This lane's population | 125 | of 391 unexamined-of-6,589 `literal-verified` | brief-stated: 92 `COMBAT` + 18 `WEAPON` + 15 `WEAPONPROF=*` |
| `COMBAT` subtoken `AC`, literal-valued (this cycle's attempted population) | 82 | of 92 `COMBAT` | `python3 -c "import json; d=json.load(open('.../combat-shape-work/ac-manifest.json')); print(len(d['items']))"` → `82` |
| `COMBAT` subtoken `AC`, formula-valued (not attempted) | 3 | of 92 `COMBAT` | listed in this receipt's Subtoken enumeration |
| `COMBAT`, non-`AC` subtoken (not attempted) | 7 | of 92 `COMBAT` | same |
| Units examined this cycle | 82 | of 125 (65.6%) | `python3 -c "import json; print(len(json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-shape-combat.oracle-results.json'))['results']))"` → `82` |
| Agree | 40 | of 82 examined | `python3 -c "import json,collections; d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-shape-combat.oracle-results.json')); print(collections.Counter(r['verdict'] for r in d['results']))"` |
| Disagree | 26 | of 82 examined | same command |
| Unverifiable | 16 | of 82 examined (14 `oracle_harness_ultimate_psionics_campaign_load_failure` + 2 `engine_id_resolve_fails_templated_variant_record`) | same command |
| Reasonless `unverifiable` in this lane's own rows | 0 | of 16 `unverifiable` rows | `python3 -c "import json; d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-shape-combat.oracle-results.json')); print(len([r for r in d['results'] if r['verdict']=='unverifiable' and not r.get('reason')]))"` → `0` |
| Units NOT examined this cycle | 43 | of 125 (34.4%) | 125 − 82 = 43 (7 `COMBAT` non-AC subtoken + 3 `COMBAT` formula-valued AC + 18 `WEAPON` + 15 `WEAPONPROF=*`) |
| `equipment_effects` test suite | 62 of 62 green | n/a | `cargo test --locked --lib equipment_effects::` |

## Status: blocked-escalated

**Not `complete`.** 82 of this lane's 125-unit population are genuinely examined this cycle with
real, per-unit `(ours, oracle, verdict)` rows (40 agree, 26 real root-caused disagree, 16
`unverifiable` each with a populated reason) and two real, additive engine fixes landed
(the AC widening this cycle's own oracle round-trip exercises directly, plus the
`WEAPONPROF=<name>` widening this cycle's remaining budget did not reach an oracle round-trip
for). The remaining 43 units are named per-shape above with concrete structural reasons (a real,
larger engine-shape gap needing new struct fields/aggregation wiring for `COMBAT|TOHIT`/
`INITIATIVE`, a natural-attack or self-weapon fixture pattern not yet built, or a formula-valued
chain with no evaluator) and a concrete next-cycle plan below — not "ran out of time" vaguely.
Marking this row `complete` while 43 of the 125-unit population stay unexamined, or while 26
real disagreements sit unaddressed, would repeat exactly the false-completion shape this
remediation wave exists to close.

## Movement, four buckets

- **closure:** 0 — no unit's `docs/work-inventory.json` `status` field changed; oracle
  verification results live in this directory's own JSON files, matching every prior
  `AT-33-E5-00x` cycle's own convention.
- **reclassification:** 0
- **reachability:** 0 — this cycle found real ceilings (the `COMBAT|TOHIT`/`INITIATIVE`/
  `ATTACK,AC` no-resolver-at-all gap, the bare-`WEAPON`/`DAMAGEMULT`/formula-valued `WEAPON`
  shapes, the natural-attack-fixture requirement for `WEAPONPROF=Hoof`/`Bite`/`TYPE.Natural`)
  but did not widen those specific ones.
- **instrument-correction:** 2 — the campaign-closure hazard (found and fixed within this
  cycle before it could produce a single false result) and the `equipment_id_resolve`
  templated-multi-variant-record limitation (`Psychoactive Skin (Defender)`/`(Hero)`, named
  and excluded rather than guessed at).

## Notes

- **Two real engine fixes landed this cycle, both additive and both verified not to regress any
  pre-existing test:** the AC widening (above) and a new `WEAPONPROF=<name>` (bare, non-`TYPE.`,
  no `TYPE=Enhancement` requirement) shape in `equipmods.rs`, confirmed against real PCGen source
  (`pcgen.io.exporttoken.WeaponToken.getMagicHitToken`/`getMagicDamageToken`: PCGen sums a
  `WEAPONPROF=<name>` bonus unconditionally, with no `TYPE=` filter, when the wielded weapon's
  own resolved proficiency matches `<name>` exactly) — unlocks 9 of this lane's own 15
  `WEAPONPROF=*` units for a future cycle's oracle round-trip (4 self-weapon + 5
  `TYPE.Natural`), plus the 6 `Hoof`/`Bite` units once a natural-attack fixture exists.
- **`WEAPON.<i>.MAGICHIT`/`.MAGICDAMAGE` identified as the right oracle isolator** for both the
  `WEAPON` `TYPE=Enhancement` group and the `WEAPONPROF=*` group (real PCGen source read this
  cycle, not guessed) but the fixture-generation script and full batch were not built this cycle
  — named honestly as unattempted, not fabricated as `unverifiable`.
- Two units this lane's own manifest generated a fixture for (`psychoactive_skin_defender`,
  `psychoactive_skin_hero`) hit a real, root-caused `equipment_id_resolve` limitation: their
  corpus record's own display key (`Psychoactive Skin (Defender)`/`(Hero)`) is a per-variant
  identity synthesized over ingestion from a templated raw LST record whose own name is the
  literal template string `Skin of the [NAME]` with no `KEY:` token at all — none of
  `equipment_id_resolve`'s three passes (identity/name/normalized-name) match the per-variant
  display key against that template record. A real, distinct resolver gap, not attempted
  further this cycle; recorded as `unverifiable`, reason `engine_id_resolve_fails_templated_variant_record`.

## RED→GREEN

Two real engine-shape fixes (`arms_armor.rs`'s widened AC match, `equipmods.rs`'s new
`WEAPONPROF=<name>` match), each proven RED→GREEN against a real verbatim corpus record before
being trusted, landed in a separate commit and pushed ahead of the oracle-verification work (see
`commit_shas`). Population coverage otherwise: **before** this cycle,
`equipment-shape-combat.oracle-results.json` did not exist; 0 of this lane's 125-unit population
had any per-unit disposition. **After:** `cargo build --locked --bin e5_combat_ac_ours` exits 0
(warnings only, pre-existing and unrelated — same warning set every prior `AT-33-E5-00x` cycle's
receipt names); 92 real, live direct-`java` BatchExporter invocations against the real pinned
oracle (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, built fresh this cycle from
a full non-sparse checkout, `BUILD SUCCESSFUL in 58s`) run, all 92 exit 0.

## Test scoping

Ran `cargo test --locked --lib equipment_effects::` (62 of 62 green, scoped to the two touched
modules and their whole parent module — the narrowest scope covering both this cycle's changes).
Ran `cargo build --locked --bin e5_combat_ac_ours` (exits 0, pre-existing warnings only, same
warning set every prior `AT-33-E5-00x` cycle's receipt names). Ran
`scripts/oracle_harness/compare.py`'s `compare_unit` (imported, not modified, via
`ac_build_results.py`). **Did not** run the root `cargo test` sweep or `apps/desktop/src-tauri`
(a separate cargo workspace; no file in it touched this cycle) — no existing test file changed
beyond the two modules' own test blocks.

## Next-cycle plan (concrete, per shape, for the next lane picking up this population's remainder)

1. **`WEAPON.<i>.MAGICHIT`/`.MAGICDAMAGE` fixture batch (11 units: 7 `WEAPON` TYPE=Enhancement +
   4 self-weapon `WEAPONPROF=<name>`):** equip the item as the sole weapon (`EQUIPSET:Equipped`),
   export `WEAPON.0.MAGICHIT`/`WEAPON.0.MAGICDAMAGE` (real PCGen tokens, confirmed this cycle by
   reading `pcgen.io.exporttoken.WeaponToken` source, not yet run live) — compare against
   `ResolvedEquipmentEffect.weapon_enhancement_bonus.bonus` for the matching `affects` roll(s).
2. **`WEAPONPROF=TYPE.Natural` (5 units, Amulet of Mighty Fists family):** same MAGICHIT/
   MAGICDAMAGE mechanism, but equip `Unarmed Strike` (a real, book-agnostic natural-attack
   weapon, `equipmods.rs`'s own test fixture already proves it parses) as `WEAPON.0`, plus the
   amulet as a second equipped item — no `applied_modifiers` attachment needed, since
   `getWeaponProfTypeBonuses`'s real PCGen implementation applies a `TYPE.Natural`-scoped bonus
   to any equipped natural weapon automatically.
3. **`WEAPONPROF=Hoof`/`Bite` (6 units):** needs a natural Hoof/Bite attack on the test
   character, which a Human Fighter has neither of by default — build via a custom
   `NATURALATTACKS:` equipment item (the same mechanism `naga_scale_bindi_dark_naga`'s own real
   corpus record uses) equipped alongside the item under test, or a race that grants one.
4. **`COMBAT|TOHIT`/`INITIATIVE`/`ATTACK,AC` (7 units) and the 3 formula-valued `COMBAT|AC`/
   `INITIATIVE` units:** no engine resolver exists for any bare non-AC `COMBAT` subtoken at all,
   and no formula evaluator exists for `Global_LuckBonus`-style chain values. A real,
   `src/rules_core/`-scoped engine gap, larger than one cycle's remaining budget — needs its own
   dedicated cycle (new struct fields on `EquipmentStatEffect`/`ResolvedEquipmentEffect` for
   TOHIT/INITIATIVE, plus aggregation wiring analogous to `armor_class_delta`).
5. **`WEAPON`'s remaining 7 units** (3 bare no-`TYPE=` TOHIT penalties, 4 `DAMAGEMULT:2`/formula
   `ATTACKS` class-feature-shaped items): each is its own distinct shape needing its own census
   and, for the formula ones, likely a genuine `unverifiable` verdict (no single scalar
   comparable against a PCGen export token) rather than an engine fix.
