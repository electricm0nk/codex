# Cycle t12-class-feature-pool-population, cycle 19 — Gate 3 (closure invariant) / row 18 (`epic-8-pool-shaped-class-features`)

- **Card ID:** row 18 (`epic-8-pool-shaped-class-features`), governed by `decisions.md §27b`
  (*"EVERYTHING"*). Dispatched scope: fix the desc-formula resolver's own missing header-merge
  step (cycle 18's own named gap, mirroring cycle 8's fix for the OTHER resolver), then prove
  every remaining refusal is either a `§27b` hard-impossibility data gap or a real bug, named by
  shape and count. Oracle Mystery stays withdrawn.
- **Base:** worktree started on a STALE lineage (`1bb523773d`, PR #374's merge into tranche/11,
  footgun 4, same shape hitting most lanes on this card). Fixed: `git reset --hard "$PIN"`
  (`PIN=a0d577c35707eb8d2126e79c381b5744a4571326`, cycle 18's own commit) — `origin/tranche/12`'s
  tip was still exactly `$PIN` at reset time, so no rebase was needed and no sibling lane had
  landed on row 18's files since. `BASE_OK` re-verified after.
- **Oracle:** not needed this cycle beyond re-confirming corpus facts directly (no new PCGen Java
  citation required — the fix is a pure header-merge wiring change, the SAME lookup mechanism
  cycle 8's own oracle-independent fix already used).

## 1. Fixed: the desc-formula resolver's own missing header-merge step

Cycle 18 named this precisely: `Mountain Domain ~ Foothold`'s real corpus `%1` argument is the
bare identifier `DomainMountainTimes`, resolvable via cycle 18's own domain-kind header merge
through the bonus_vars resolver, but `resolved_description_for_formula_only_desc_argument`
(`class_feature_grant_consumer.rs`) evaluates each `%N` argument against ONLY the ability-modifier
and class-level seed, with no header-chain merge step of its own.

**Fix, mirroring cycle 8's own header-merge fix for the OTHER resolver (`§17`, one merge
implementation, not two):** factored `resolve_pool_member_sole_magnitude`'s own three-step header
merge (per-group header via `pool_header_record_by_normalized_suffix`, the
`"<RegisteredName> Tracker"` header, the bare `"<class> ~ <RegisteredName>"` base header, and the
owning class's own record-level `BONUS:VAR` chain via `class_record_bonus_vars`) out of that
function's own body into a new shared function, `pool_group_header_vars_merged`. Both call sites
(`resolve_pool_member_sole_magnitude` and the new one below) now call this ONE function; the
extraction reproduces `resolve_pool_member_sole_magnitude`'s own behaviour exactly (proven by the
full unchanged `pool_group_closure_census_across_all_six_pools` bonus_vars-only baselines below).

`resolved_description_for_formula_only_desc_argument` gained a new `header_vars: &BTreeMap<String,
String>` parameter: the resolved values of `resolve_pcgen_var_chain(header_vars, ...)` are folded
into the function's own `seed_vars` via `.entry().or_insert()` — never overwriting the
ability-modifier/class-level seeds already bound, matching every other merge in this module. An
empty `header_vars` map (this function's own doc: "both existing call sites before this cycle")
makes the change a true no-op, so the two real call sites
(`push_generic_pool_group_selection_description_magnitude`, the production chassis path for
Warpriest/Cavalier, and `group_has_a_resolvable_member_via_description_formula`, the census
helper) now compute `pool_group_header_vars_merged(class, &group, Some(registered_name))` once per
group and pass it down.

**Why the FIRST attempt (a narrower merge, only the per-group header + class-record chain) missed
the real fix:** `Mountain Domain ~ Foothold`'s `DomainMountainTimes` chains to `DomainPowerTimes`,
which is bound only on Cleric Domain's own bare `"Domains"` base-header record — reached, in
`resolve_pool_member_sole_magnitude`, only through the THIRD merge step
(`pool_header_record_by_normalized_suffix(owning_class, registered_name, None)`, i.e.
`pool_header_record_by_normalized_suffix("Cleric", "Domain", None)`), never the first (per-group)
step. A narrower merge (verified live, then discarded) left the combined census figure completely
unchanged (49/73 stayed 45/73) until all three merge steps were reused via the shared function —
recorded here so a future cycle does not repeat the narrower attempt.

## 2. Measured effect (`§17a`, re-derived, not assumed)

```bash
cargo test --locked --lib -- \
  rules_core::pilot_compute::generic_pool_group_selection_wiring_tests::pool_group_closure_census_across_all_six_pools_both_resolvers \
  --nocapture
```
```
Sorcerer Bloodline: bonus_vars=31/53, combined(bonus_vars OR desc_formula)=32/53
Bloodrager Bloodline: bonus_vars=5/12, combined(bonus_vars OR desc_formula)=6/12
Cleric Domain: bonus_vars=44/73, combined(bonus_vars OR desc_formula)=49/73
Shaman Spirit: bonus_vars=11/14, combined(bonus_vars OR desc_formula)=12/14
Warpriest Blessing: bonus_vars=0/37, combined(bonus_vars OR desc_formula)=8/37
Cavalier Order: bonus_vars=1/9, combined(bonus_vars OR desc_formula)=2/9

test ... ok
```

Bonus_vars-only baselines UNCHANGED on every pool (this cycle touches only the desc-formula
resolver). Combined moved Cleric Domain 45/73 → 49/73 (+4) and Sorcerer Bloodline 31/53 → 32/53
(+1). Diagnostic run (removed before commit, same methodology cycle 18 used) identified the newly
closed groups by name: **Cave, Desert, Mountain, Nobility Domain** (Cleric; Clandestine Domain was
already counted before this cycle) and **Imperious Bloodline** (Sorcerer). Every other pool's
combined figure unchanged, re-verified.

## 3. `§27b`/`§17a` re-classification: every still-unresolved group, re-derived

Re-ran cycle 18's own three-bucket classification (diagnostic written, run, then removed before
commit — same methodology) over every group still unresolved after this cycle's fix:

```
not_numeric=46 refused_bonus_var=37 refused_desc_formula=6   (was 46/39/9 per cycle 18)
```

`not_numeric` (`§7`/`§16` reclassification) is unchanged — this cycle's fix does not touch group
classification, only resolution capability. The 5-group movement (109 − 104 = 5, matching §2's own
delta) came from a mix of buckets: 2 groups that were tagged `refused_bonus_var` (because they
ALSO have a bonus_var-shaped member that still individually refuses) newly closed via a
DIFFERENT, desc-formula-shaped member now resolving; the other 3 came from the desc-formula
bucket directly. Group-level classification counts a group as resolved the moment ANY member
resolves via EITHER resolver, per `group_has_a_resolvable_member_via_either_resolver`'s own
existing contract — unchanged this cycle.

## 4. `§27b` exhaustive verification, per the brief's own instruction

### 4a. Cleric's remaining headerless Domains — verified exhaustively, genuine data gaps

Corpus-wide scan (both real header shapes: `data/corpus/*/domain/<name>.json` with a real
`BONUS:VAR` token, AND a bare `class_feature` record keyed exactly `"<Name> Domain"` with a real
`BONUS:VAR` token) for all 12 remaining headerless-refusal Domains:

```
Anger, Conversion, Fervor, Illumination, Imprisonment, Justice, Order, Persistence, Scalykind,
Truth, Vengeance, Zeal -> no real BONUS:VAR-bearing header anywhere in the corpus
```

`Scalykind` has a domain-kind FILE (`data/corpus/{inner_sea_world_guide,bestiary_6,
ultimate_wilderness}/domain/scalykind.json`), but all three carry ONLY `DEFINE:<name>|0`
zero-baselines and zero `BONUS:VAR` tokens — confirmed by direct inspection, not a header cycle 18
missed, just a placeholder file with no real magnitude data. Every one of these 12 is a genuine
`§27b` hard-impossibility data gap: the source data does not exist anywhere in the ingested
corpus. (`Mountain` and `Jungle`, also on cycle 18's original 14-domain list, are excluded here —
both gained a real header via cycle 18's own domain-kind fix and now refuse for the separate
multi-terminal reason named in §4c below, not a missing header.)

### 4b. Sorcerer/Bloodrager cross-bloodline refusals — spot-verified, reproduce cycle 17's proven shape

Traced one representative per class through to a corpus-wide grep, matching cycle 17's own
standard (read the real binding, name the two candidate records, confirm no third exists):

- **`Fey Bloodrager Bloodline ~ Confusing Critical`**: single terminal, `Bloodrager_Fey_
  BloodlineLVL`. Corpus-wide grep for every real `BONUS:VAR|Bloodrager_Fey_BloodlineLVL|...`
  binding found exactly TWO records, neither held by a plain Bloodrager who merely picked the Fey
  bloodline: `data/corpus/advanced_class_guide/ability/fey_bloodline.json` (an ABILITY record,
  binds a flat constant `1`, the same "Raging Blood Feat Bloodline" shape cycle 17 named for every
  other bloodline) and `.../eldritch_scion_fey_bloodline/eldritch_scion_fey_bloodline.json`
  (`class: "Sorcerer"`, the cross-class archetype cycle 17 also named). No third record exists.
- **`Karmic Bloodline ~ Fate's Retribution`**: single terminal, `Sorcerer_Destined_
  BloodlinePower1LVL` — a DIFFERENT bloodline's own header (Karmic → Destined), the exact
  cross-bloodline reference shape cycle 18 sampled for Aerial → Stormborn.
- **`Envenomed Bloodline ~ Envenom`**: `Sorcerer_Serpentine_BloodlinePower3LVL` — again a
  different bloodline's own header (Envenomed → Serpentine).

Every sample reproduces the SAME shape cycle 17 proved oracle-correct (`PlayerCharacter.
getVariable`, `PlayerCharacter.java:2090`, class-blind but only ever nonzero for a source the
character actually holds) and cycle 18 sampled once (Aerial → Stormborn). This is a genuine
cross-reference/archetype data gap, not a resolver defect — the guard correctly refuses rather
than import a binding that would misrepresent every character of that bloodline as if they also
held an unrelated feat or archetype.

### 4c. New real shape found and named, NOT forced this cycle: genuine multi-terminal refusals

Investigating the remaining refusals surfaced a shape distinct from both 4a and 4b: several
records — `Forbidden Rites Domain`'s all 39 real member records (a genuine Cleric "Separatist"
archetype mechanism letting a Cleric replace their second domain's powers; its own header record,
`Separatist ~ Forbidden Rites`, DOES exist and DOES bind the `SeparistDomainLVL` identifier its
formulas need — this is NOT a missing-header case), Starsoul Bloodline's `Aurora Borealis`/`Feat
Tracker`/`Minute Meteors`/`Breaching the Gulf`/`Voidwalker`, and Celestial/Fey Bloodrager
Bloodline's `Ascension`/`Celestial Resistances`/`Feat Tracker`/`Wings of Heaven` — all carry TWO OR
MORE independent terminal targets on one record (e.g. `Forbidden Rites Domain ~ Air Domain` has
THREE: `DomainAirDC`, `DomainAirTimes`, `DomainAirAbilityTriggerLVL`, none referencing another).
`resolve_pool_member_sole_magnitude`'s own multi-terminal guard correctly refuses these — the
SAME "genuinely novel, multiple independent magnitudes" shape already documented and accepted for
Mountain's own `Thin Air` (`ThinAirRounds`/`ThinAirRange`).

**Why this is NOT a `§27b` hard impossibility and stays open, named:** the source data all exists
and is individually resolvable per-terminal; the blocker is that this module's resolver contract
only ever reports ONE magnitude per record. `§27b` point 5 is explicit: *"needs a new mechanism"*
is NOT an admissible reason for non-closure. This is real, named, remaining work — a
multi-terminal-aware resolver capability — not forced this cycle (a genuinely novel resolver
shape, larger and riskier than this cycle's remaining scope warranted, matching cycle 18's own
"named here for a future cycle, not forced this one" discipline for comparable finds).

## 5. Correction filed

`scripts/retro.py correction`, id `1787590888388-t9-onboarding-f39f7e`: cycle 18's own receipt
named the `real_groups_owned_by` naming-shape false positive (a bare per-bloodline/spirit HEADER
record counted as if it were its own selectable group) ONLY for Shaman Spirit. Investigating the
remaining bonus_var refusals this cycle found the SAME false positive ALSO affects `Sorcerer
Bloodline` and `Bloodrager Bloodline` — 3 fake groups total, not 1 (verified: the corpus key
`"Sorcerer Bloodline ~ Aberrant"` is the Aberrant bloodline's own header record, not a genuine 54th
Sorcerer bloodline named "Sorcerer Bloodline"). This inflates both cycle 18's own 39-group and this
cycle's re-derived 37-group `refused_bonus_var` counts by 2 (real remaining population is smaller
by 2 than either receipt states). Not fixed this cycle — a pre-existing census-instrument gap,
named for a future cycle, matching cycle 18's own scope discipline for the Shaman instance.

## 6. Tests, RED→GREEN, mutation-proved (`§1a`), then full re-run

New test: `mountain_domain_foothold_desc_formula_needs_the_header_merge_to_resolve` — proves
`resolved_description_for_formula_only_desc_argument` returns `None` for `Mountain Domain ~
Foothold` with an empty header map, and `Some(("...", 3))` with the real merged header
(`DomainPowerTimes = 3+WIS`, WIS modifier 0 under `AbilityModifiers::default()`).

**Mutation-proved RED, then reverted:** temporarily gated the new header-merge branch behind
`if false && ...` (marked `// MUTATION-PROOF-TEMP`) — both the new test AND the pinned
`pool_group_closure_census_across_all_six_pools_both_resolvers` assertion FAILED as expected
(reverting to the pre-fix `31/53`/`45/73` figures) — then restored from a saved pre-mutation copy
of the file (never `git stash`), re-verified GREEN.

```bash
cargo test --locked --lib -- rules_core::pilot_compute::
```
```
test result: ok. 975 passed; 0 failed; 0 ignored; 0 measured; 1843 filtered out
```
(up from cycle 18's 974 — net +1, the new `mountain_domain_foothold_...` regression test)

```bash
cargo test --locked --lib -- rules_core::derived_evaluator_fixture_check::
```
```
test result: ok. 121 passed; 0 failed; 0 ignored; 0 measured; 2697 filtered out
```
(unchanged)

```bash
cargo test --locked --lib -- hunter oracle_dispatch_widening_safety_tests cavalier
```
```
test result: ok. 110 passed; 0 failed; 0 ignored; 0 measured; 2708 filtered out
```
(unchanged; `oracle_dispatch_widening_safety_tests::a_mystery_pick_alone_grounds_no_tier_one_
revelation` untouched, still green — Oracle Mystery stays withdrawn.)

## 7. Not attempted / not applicable this cycle

- The multi-terminal resolver capability (§4c) — genuinely novel, larger scope, not forced.
- 6 remaining desc-formula refusals (Sanguine Bloodline; Crime, Sedition, Torture, Valor, Void
  Domain) — not investigated this cycle, time-boxed, named for a future cycle.
- The 3-fake-group `real_groups_owned_by` naming-shape census instrument gap (§5) — not fixed,
  pre-existing, named for a future cycle.
- Wiring `push_generic_pool_group_selection_description_magnitude` into the real Sorcerer/Cleric
  chassis dispatch (currently only Warpriest/Cavalier call it) — the census counts corpus-record
  resolvability, not runtime reachability, the SAME `§16` distinction this card's own receipts
  have drawn since cycle 5; wiring Cleric's own DOMAIN_POWER_CATALOG-covered domains would need a
  careful exclusion-list audit to avoid double-grounding an already-hand-modelled domain power
  under a second id — a real, separate, larger change, not attempted this cycle.
- Rows 11/15 left as found (`in-progress`/`complete`), untouched. `apps/desktop`'s row 19/20 lanes
  not touched. `data/corpus/**` untouched throughout (`git status --porcelain -- data/corpus` — 0
  changes).

## 8. Audits

- **Identifier audit** (`grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'`, scoped to
  `git diff --unified=0` of `src/rules_core/pilot_compute/mod.rs`,
  `src/rules_core/pilot_compute/class_feature_grant_consumer.rs`, and the kanban diff, SD-32
  self-references excluded): `OK_NO_BUNDLE_TAGS` — 0 hits.
- **Wired-integration audit** (`grep -inE "STUB|MOCK|placeholder|not.?yet.?implemented|todo|
  fixme|hack"`, scoped to the code diff only): `OK_NO_TOKENS` — 0 hits. (The same scan over the
  kanban diff surfaces two PRE-EXISTING phrases — "Bloodrager-specific hack", "No stubs" — both
  confirmed present in `git show HEAD:kanban.md` before this cycle; not new.)
- **PI audit**: `pi_scrub.normalized_term_hits(...)` (imported via `scripts/pi_scrub.py`, never
  copied), scoped to `git diff --unified=0` of the full cycle diff (code + kanban together) →
  `[]` (0 hits). `data/corpus/**` untouched throughout.

## 9. Kanban

`docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 18 only (cycle field 18 →
19, Notes prepended). Verified structurally after editing (backtick-aware parser): 22 distinct
`^| N |` rows (including header), 0 duplicate ids, row 18 parses to 9 backtick-aware raw
pipe-split fields (7 real columns) before and after. Rows 11 (`in-progress`) / 15 (`complete`)
confirmed untouched (`git diff --stat` shows exactly 1 line changed in the kanban file). **Status
stays `in-progress`**: real remaining work exists (the multi-terminal resolver capability, 6
unstudied desc-formula refusals, the 3-fake-group census instrument gap) — the honest disposition
per `§17a`, not a comfortable close and not a lazy hold-open. The header-merge fix and the 5 new
real closures it produced are real, mutation-proved progress; the remaining refusals are either
`§27b`-verified hard-impossibility data gaps (Cleric's 12 headerless domains, Sorcerer/Bloodrager's
spot-verified cross-bloodline gaps) or real, named, un-forced compute-shape work (the multi-
terminal resolver gap, the 6 unstudied desc-formula refusals).

## 10. `df -h /`

```bash
df -h /
```
```
Filesystem      Size  Used Avail Use% Mounted on
/dev/sda1       968G  517G  452G  54% /
```
