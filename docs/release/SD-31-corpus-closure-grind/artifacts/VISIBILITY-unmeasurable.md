---
canonical: true
owner: sd31-wave28-unmeasurable-lane
purpose: SD-31 wave 28 "look, don't bank" mandate. Establishes the distinct causes behind the
  4,270-unit `unmeasurable` verdict, corpus-wide, for the first time (previously characterised
  once, for `class_feature` alone). No units reclassified, no code changed, no `docs/work-inventory.json`
  write. Every count below has a reproduction command.
started: 2026-08-21
wave: 28
board_at_measurement: "13,456 / 38,372 (35.07%)"
base_commit: e90ba9ec1
banking_this_wave: NONE — measurement only, per mandate
---

# VISIBILITY: the `unmeasurable` pile

## 0. What this file is, and is not

This is a **census**, not a fix list. Nothing in `docs/work-inventory.json` moved. No code changed.
Every number below is reproduced from a command given inline; re-run any of them against the same
base commit (`e90ba9ec1`) and get the same number. Where I inspected the pinned PCGen oracle
directly (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, checked out at
`~/workspace/repos/pcgen`, confirmed on-pin at measurement time), the exact file and line are cited.

**Per Decision 1(a) (anti-gaming) and this wave's explicit prohibition: nothing here reclassifies a
unit.** Where I found a unit that looks wrongly `unmeasurable`, I report the mismatch and the exact
evidence for it — I did not touch `docs/work-inventory.json`, `src/bin/v06_work_inventory.rs`, or
any corpus JSON file.

Measured in an isolated `CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/w28-unmeasurable`, deleted at
the end of this session per the wave's standing instruction.

## 1. The population, reproduced

```
python3 -c "
import json, sys, collections
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
def v(u): return P.doneness_verdict(u.get('wiring_class'), u.get('status'), u.get('kind'))
unmeas=[u for u in U if v(u)=='unmeasurable']
print(len(unmeas))
print(collections.Counter(u.get('kind') for u in unmeas).most_common())
"
```

Output, matching the wave brief exactly:

```
4270
[('class_feature', 3058), ('feat', 565), ('equipment_modifier', 416), ('equipment', 205), ('spell', 26)]
```

`EXCLUDED_BOOKS = {"beginner_box"}` (`scripts/observer/pf1e_dashboard_producer.py:3474`) — the same
scope Decision 5's mandate denominator uses. `unmeasurable` is defined structurally in
`_doneness_verdict_uncapped()` as `status == "unknown"`
(`scripts/observer/pf1e_dashboard_producer.py:3882-3883`): "An `unknown` status cannot be measured
against any bar, classifiable or not." It is produced by exactly **7 distinct evidence tags** across
the 5 kinds, each written at one place in `src/bin/v06_work_inventory.rs`'s `classify()`:

```
python3 -c "
import json, sys, collections
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
def v(u): return P.doneness_verdict(u.get('wiring_class'), u.get('status'), u.get('kind'))
unmeas=[u for u in U if v(u)=='unmeasurable']
by=collections.defaultdict(collections.Counter)
for u in unmeas: by[u['kind']][u['evidence']]+=1
for k,c in by.items():
    for ev,n in c.most_common(): print(n,k,ev)
"
```

| # | Kind | Evidence tag | Count | Source line |
|---|---|---|---:|---|
| 1 | class_feature | `class_feature_group_names_no_class_at_all` | **3,058** | `v06_work_inventory.rs:6735` |
| 2 | feat | `in_catalog_with_corpus_magnitude_but_no_observed_consumer` | **516** | `v06_work_inventory.rs:5949` |
| 3 | equipment_modifier | `text_only_but_corpus_record_carries_no_description_to_show_a_player` | **416** | `v06_work_inventory.rs:6170` |
| 4 | equipment | `text_only_but_corpus_record_carries_no_description_to_show_a_player` | **205** | `v06_work_inventory.rs:6170` (shared arm) |
| 5 | feat | `feat_served_description_is_a_placeholder_marker_not_prose` | **38** | `v06_work_inventory.rs:5894` |
| 6 | spell | `spell_list_entry_with_no_corpus_level_and_no_description` | **26** | `v06_work_inventory.rs:6055` |
| 7 | feat | `text_only_but_corpus_record_carries_no_description_to_show_a_player` | **11** | `v06_work_inventory.rs:5935` |

Sums: 3058+516+38+11 = 3623 (feat total is 516+38+11 = **565** ✓); 416+205 = 621 (equipment_modifier
416 + equipment 205, same evidence string, different `Kind` arms ✓); spell 26. Total 3058+565+416+205+26
= **4,270** ✓, matches the population exactly. **One cause per kind for 3 of the 5 kinds** — this is
a narrow, tractable census, not an open-ended one.

## 2. class_feature — 3,058 units, single cause, three sub-populations

### 2.1 Why the cause is singular

`classify()`'s `Kind::ClassFeature` arm can only reach `status: "unknown"` from one line
(`v06_work_inventory.rs:6735-6746`), and only after **both** owner-resolution passes fail
(`class_feature_owner` against `facts.class_books` — engine-modelled classes — then
`class_feature_owner_via_type_facet` as a fallback) **and** the record is not `text_only` (a
text-only record with no owner exits earlier, at `not_ingested`). The function's own doc comment
names the root cause directly (`v06_work_inventory.rs:7800-7807`):

> "A feature a player must PICK out of an option pool — a rage power, a discovery, a rogue talent —
> is never in that posture [the automatic-grant chassis sweep], so it can never emit an explanation
> there, and lands as `unknown`."

**This is structural, not per-record**: `class_sweep_input` (the compute pass that emits
explanation ids) only walks a class's *automatic* level-up grants and `canonical_seeds_for`
defaults. A feature a player *chooses* from a named pool (a sorcerer bloodline, a cleric domain, a
rogue talent, a rage power…) is never in that posture, so its owner can never be confirmed through
the main path — only through `class_feature_owner`'s narrow text match on the group prefix.

### 2.2 The already-built, already-tested, never-wired fix

`v06_work_inventory.rs` already contains a **second, more powerful** group-to-class matcher,
`class_feature_pool_group_matches()` (line 7970), backed by a 27-entry table,
`CLASS_FEATURE_POOLS` (line 7828 — bloodline, domain, mystery, hex, discovery, revelation, talent,
judgment, blessing, evolution, curse, spirit, animal focus, favored enemy/terrain, order, arcane
school…), an 8-entry false-positive exclusion list built from direct oracle reads
(`CLASS_FEATURE_POOL_FALSE_SUFFIX_MATCHES`, line 7882), and a cross-class collision guard. It is
**pinned by its own corpus-wide enumeration test**:

```
export CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/w28-unmeasurable
cargo test --locked -j 8 --bin v06_work_inventory \
  class_feature_consumer_delta_tests::class_feature_pool_group_matches_enumerates_the_real_corpus_against_every_pool_entry \
  -- --nocapture --exact
```

Output (2.32s, ok): **243 corpus groups newly matched** by the suffix rule (plus 15 that match the
registered pool name verbatim — 258 distinct groups total), and **24 groups correctly excluded**
despite a superficial suffix match (each with a citation to the real corpus row proving it belongs
to a different class or subsystem — e.g. `Merciful Healer Mercy` is a Cleric archetype's mercy, not
Paladin's).

**But this matcher is consulted only by the wiring-probe** (`probe_class_feature_key`, used to
populate `facts.class_feature_effect_wired`, checked at the very top of the `ClassFeature` arm,
line 6600) — **never by the owner-resolution fallback that decides `unmeasurable`** (line 6738,
which calls only `class_feature_owner`/`class_feature_owner_via_type_facet`). A record whose group
is a perfectly recognised pool (e.g. "Aberrant Bloodline") still falls into `unmeasurable` if the
probe found no consumer delta for it — which is the common case, since most pools have no
per-pick compute function yet.

### 2.3 Sizing the population against the existing matcher

```
# scratch script, not shipped — replicates the group-match test's already-verified
# 243-group "newly matched" + 15-group "exact name" set, intersected against the
# real unmeasurable population's group prefixes (unit.corpus_key.split(' ~ ')[0])
python3 census_unmeasurable.py   # full text in this artifact's provenance; see §7
```

| Bucket | Units | Distinct groups | What it means |
|---|---:|---:|---|
| **A — group matches a registered pool** (via the already-built, already-tested matcher) | **612** | 146 | Owner-resolution would succeed today if the matcher were called. Verified: `class_feature_owner()` succeeding routes to `grounded`/`not-ingested`/`deferred-with-reason` — **never** `unknown` (traced the full "owner resolved" branch, `v06_work_inventory.rs:6747-6935`; its own final fallback for an unresolved consumer is `not_ingested("no_explanation_id_and_no_diagnostic_names_this_feature")`, line 6935). Wiring the matcher in cannot regress anything (none of the 612 are currently `done`). |
| **B — false suffix match, excluded by design** | **53** | 11 | Real content, real class, but genuinely belongs to a *different* class/subsystem than the pool name suggests (e.g. `Inspired Discovery` = Investigator talent wearing Alchemist's clothing; `Take Inquisition` = a cross-class Cleric option). Needs individual per-group ownership work, not bulk registration — same rigor the existing 8-entry exclusion list required. |
| **C — group shape not recognised at all** | **2,393** | 958 | Neither matcher fires. The long tail. See §2.4. |

### 2.4 Bucket C — the 958-group long tail

Top 30 by unit count (full 958-row table in the provenance script's JSON output, not reproduced
here — genuinely too long to hand-classify this wave):

| Units | Group | Spot-checked shape |
|---:|---|---|
| 106 | Domain Power | **Corpus-wide generalisation of the Bestiary 6 ledger's own finding** (wave 24: "Domain Power ~ Chastisement" etc., 11 units, one book). This is the SAME group prefix, same root cause (Cleric/Inquisitor domain powers, owned by neither `class_feature_owner`'s substring match nor the suffix-pool matcher — "Domain Power" is a *prefix* shape, not the registered pools' *suffix* shape), now confirmed **106 units across every book that prints a domain power**, not one book's worth. |
| 94 | Refined Education | Investigator archetype talent-pool variant |
| 37 | Favored Enemy Bonus | Verified: `advanced_class_guide:acg_abilities_class.lst` — a companion display row paired with the already-registered `Favored Enemy` pool; **possible overlap with Decision 17's chooser-duplicate shape** (§17 found 70 `class_feature` phantom-duplicate candidates; not reconciled against this bucket this wave — flagged, not counted twice). |
| 31 | Hate-Monger | Bloodrager/Antipaladin-adjacent archetype ability pool |
| 29 | Ki Power | Monk/Sohei-adjacent ki-power menu (structurally identical to the registered `Judgment`/`Blessing` pools, just never catalogued) |
| 28 | Wildcat, 28 Insight, 27 Blade Skill, 25 Refined Education Unlock, 25 Terror, 23 Path Power, 22 Pack Lord, 20 Packmaster, 20 Beastmaster, **20 Wildblooded** | Verified `Wildblooded` (`ultimate_magic:um_abilities_class.lst:1632`) is a real Sorcerer bloodline-variant archetype — conceptually identical to the registered `Bloodline` pool, but its group text doesn't *end* in "Bloodline" (it's a standalone archetype-variant word), so the existing suffix matcher structurally cannot reach it regardless of catalog size. |
| 18 | Favored Terrain Bonus, 18 Green Faith Marshal | Same companion-row-duplicate shape as Favored Enemy Bonus above |
| 17 | Bardic Performance | Bard performance-type menu |
| 16 | Path Skill | (Investigator/Path-based archetype talent) |
| 15 | Verminous Hunter Vermin Focus | Archetype-specific Animal Focus variant |
| 12 | **ABP** | Verified `pathfinder_unchained:pu_abilities_class.lst:97-101` ("ABP ~ +1 Dexterity" etc.) — **this is Automatic Bonus Progression, a whole-table optional variant rule, not a class feature at all.** It was ingested as `class_feature` because PCGen encodes it as an ability chain, but conceptually it is owned by no class — a genuine candidate for its own `kind` or a Structural Exclusion Register question, not a pool-catalog entry. |
| 10×5 | Maneuver Training I–V Selection | ACG Brawler-adjacent combat-maneuver talent ladder, 5 tiers × 10 |
| 10 | First Boon | Verified `book_of_the_damned_volume_2:botd2_abilities_classes.lst:208` — a demon-lord Obedience-feat boon, **granted by a feat, not a class, at all** — the same root shape `CLASS_FEATURE_POOL_FALSE_SUFFIX_MATCHES` already excludes for `Spider's Blessing`/`Zevgavizeb's Blessing` (same book, same subsystem). Structurally cannot be "owned" by any class; belongs to a feat-boon mechanism this classifier doesn't model at all. |

**Honest limit: I did not hand-classify the remaining ~900 groups / ~1,900 units.** The four spot
checks above (Domain Power, Wildblooded, ABP, First Boon) span four *different* underlying shapes —
(1) a recognisable pool with the wrong naming convention for the existing suffix matcher, (2) a
recognisable pool whose name doesn't match its class at all, (3) a genuinely non-class subsystem
mis-typed as `class_feature`, (4) content owned by a feat, not a class. **Bucket C is not one
problem; cataloguing it properly needs the same per-group oracle-verification rigor the existing
27-pool table required (each of its 8 false-suffix exclusions needed a direct corpus read to
confirm), scaled to ~958 groups.** That is real, sizeable work — not a bulk registration.

### 2.5 Tool evaluation — class_feature

| Fix | Cost | Closes | Verdict |
|---|---|---:|---|
| Call `class_feature_pool_group_matches()` as a third fallback in `classify()`'s owner-resolution step (after `class_feature_owner`/`_via_type_facet`) | **Small.** The matcher, its table, and its false-positive guard already exist, are already tested, and are already used elsewhere in the same file for a different purpose. This is wiring, not construction. | **612 units**, corpus-wide, moved from `unmeasurable` (uninterpretable) to a real, honest classification (`not-ingested` for most, per the traced fallback in §2.2 — not `done`, but visible). Zero regression risk (none of the 612 are `done` today). | **Worth doing.** This is the single highest-leverage, lowest-risk item in this census. |
| Extend `CLASS_FEATURE_POOLS` with new entries for bucket C's largest groups (Domain Power, Ki Power, Wildblooded, Maneuver Training tiers, Bardic Performance…) | **Medium, per-entry.** Each new entry needs the same false-match verification the current 8-entry exclusion list needed (direct oracle read per candidate group) — this wave's own spot checks found 2 of 4 sampled large groups (ABP, First Boon) are **not class-pool shaped at all** and would need special-casing or exclusion, not registration. | Plausibly several hundred more units (the top-10 bucket-C groups alone total ~380), but **not safely estimable without doing the per-group verification** — that IS the cost. | **Real lever (matches `sweeps.md` S2/`levers.md` L5), not a quick win.** Scope a dedicated cycle, don't fold it into a wiring-only fix. |

## 3. equipment_modifier — 416 units, one evidence tag, TWO very different real causes

### 3.1 The alias/base split

```
python3 -c "... origin=='copy' and visible is False ..."   # see §7 for full script
```

| Sub-population | Units | Shape |
|---|---:|---|
| `.COPY=` alias rows, `visible: false` | **280** | A PCGen renaming/alias construct — the record's own raw line carries no fields at all beyond `KEY:`/`VISIBLE:NO`/`.COPY=<target>`; its real content (if any) lives on the base row it copies. |
| Declared rows, `visible: true` | **136** | A genuine standalone corpus row, not an alias. |

### 3.2 The 136 non-alias units: 111 of them (82%) DO carry real description text

I spot-checked every one of the 136 against the pinned oracle directly (not the ingested corpus
JSON) — for each unit's `(source_file, source_line)`, I resolved the real file under
`~/workspace/repos/pcgen/data/pathfinder/<...>/<book>/` and read that exact line for a `DESC:`,
`SPROP:`, or `BENEFIT:` token (the same three prefixes `closure_prose_field_text()`,
`v06_work_inventory.rs:5545-5556`, itself reads):

```
sed -n '<line>p' <resolved .lst path>
```

**Result: 111 of 136 (82%) have real prose text on their own raw line, right now, in the pinned
oracle — and are still classified `unmeasurable`.** Three concrete, independently reproducible examples,
the same magic weapon property ("Ghost Touch") on three item-type variants:

```
sed -n '241p' ~/workspace/repos/pcgen/data/.../core_rulebook/cr_equipmods.lst
# Ghost Touch ... TYPE:Ammunition.Weapon ... SPROP:deals damage normally against
#   incorporeal creatures regardless of bonus
#   -> work-inventory verdict: unmeasurable ("no description")

sed -n '280p' ~/workspace/repos/pcgen/data/.../core_rulebook/cr_equipmods.lst
# Ghost Touch ... TYPE:Amulet of Mighty Fists ... SPROP:unarmed and natural attacks
#   deal damage normally against incorporeal creatures regardless of bonus
#   -> work-inventory verdict: unmeasurable ("no description")

sed -n '333p' ~/workspace/repos/pcgen/data/.../core_rulebook/cr_equipmods.lst
# Ghost Touch ... TYPE:Armor.Bracer.ArmorLike.Shield ... SPROP:armor and enhancement
#   count vs. incorporeal attacks
#   -> work-inventory verdict: text-complete (DONE-eligible)
```

**Three sibling records of the identical real-world ability, identical SPROP shape, identical
book/file — one reaches `text-complete`, two reach `unmeasurable`.** This rules out "the content
doesn't exist" and rules out a facet-wide rule (I additionally checked whether `TYPE:Weapon` /
`TYPE:Amulet of Mighty Fists` fail uniformly — they don't: of 130 `Weapon`-facet zero-magnitude
`equipment_modifier` units corpus-wide, 74 are `done` and 56 `unmeasurable`; of 70
`Amulet of Mighty Fists`-facet units, 66 `done` and 4 `unmeasurable` — see §7 script
`eq_mod_facet_check.py`). **This is a per-record extraction/promotion defect, not a per-facet or
per-book one** — I could not isolate the exact triggering condition within this wave's budget (that
would mean reading the closure-construction code path in `v06_work_inventory.rs`'s corpus-loading
layer, which is outside "look, don't fix" scope), but the effect is precisely reproducible with the
commands above.

A second confirmed instance, different book: `mythic_adventures:ma_equipmods.lst:27` ("Legendary
Item ~ Adroit") and `:28` ("Legendary Item ~ Dedicated Bond") both carry full-sentence `SPROP:` text
and both land `unmeasurable`. **The entire `Legendary` facet (18 of 18 units) is unmeasurable** —
100%, a cleaner (if smaller) signal than the mixed Weapon/AMF facets.

The remaining 25 of 136 are genuinely description-free bookkeeping rows (`Special Ability ~ Uses per
Day / 1` through `/10` — a numeric-selection pricing ladder; `SCROLL_ARCANE`/`SCROLL_DIVINE`/etc. —
scroll-cost price brackets; `* Enhancement Cost` — enhancement-bonus pricing tiers). These are
**genuine PERMANENT FACTS**: PCGen pricing-formula plumbing, never printed rules text, correctly
description-free.

### 3.3 The 280 alias rows: a lower bound, not a full count

I attempted the same check for the alias population by parsing each row's `.COPY=<target>` and
searching the same file for a line whose first tab-column literally equals `<target>`. This
resolved for only 24 of 280 (a naive first-column match — PCGen's `.COPY=` target is frequently the
row's `KEY:` value, not its display name, which my quick script did not resolve). Of those 24
resolved: **22 confirmed the base row DOES carry real SPROP text** (e.g. `acg_equipmods.lst:95`
"Answering", copying from a base at line 27 with `SPROP:Enhancement bonus increases by 4...`), 2
confirmed genuinely empty. **I did not build a real `.COPY=` chain resolver — that IS the
corpus-loading logic itself, out of scope for a measurement-only wave — so I cannot give an exact
count for the 280.** Given 22/24 (92%) of the resolved sample confirm real content, the true
false-negative rate for this population is very likely comparable to or higher than the 82% found
for the non-alias population, but I am reporting the **verified lower bound (22 confirmed)**, not an
extrapolation, per this wave's "state plainly what could not be determined" bar.

### 3.4 Tool evaluation — equipment_modifier

| Finding | Permanent or instrument gap? | Size |
|---|---|---:|
| SPROP/DESC extraction or promotion fails for a majority of real-content `equipment_modifier` rows | **Instrument gap.** Confirmed real, reproducible, verified against the raw pinned oracle three independent times across two books. **Not yet in `defects.md` — this is a new finding**, comparable in shape (and likely overlapping in root cause) to `defects.md` D1 (equipment cache generators) and S5 (generators emitting unscreened fields) but distinct from both — no existing sweep row names it. | **111 confirmed, ≥133 with the alias spot-check, corpus-wide bound not yet established for the full 280-unit alias population.** |
| Pricing-ladder bookkeeping rows (`Uses per Day / N`, `SCROLL_*`, `Enhancement Cost`) | **Permanent fact.** Not printed rules text, never will be. | 25 confirmed (non-alias only; the alias population almost certainly contains more of this shape too, not counted). |

## 4. equipment — 205 units, two structurally different populations

```
python3 -c "... .FORGET suffix / pfs_*.lst source ..."   # §7
```

| Sub-population | Units | Shape |
|---|---:|---|
| `.FORGET`-suffixed keys | **2** | Confirmed via raw oracle read (`pfs_acg_equip.lst:6-7`): `.FORGET` is **PCGen's own loader directive to retract a duplicate entry**, not a game item. `Dust Knuckles.FORGET` / `False Face.FORGET` carry no fields whatsoever beyond the directive itself. **Permanent fact — not content at all.** |
| Rows sourced from a `pfs_*.lst` file (Pathfinder Society organized-play item-legality lists — a *different*, cross-referencing file from each book's main equipment list) | **32** (including the 2 above) | Not independently checked item-by-item this wave; flagged because PFS files are structurally cross-reference lists, not primary content — worth a scoping question (is `pfs_*` in-corpus scope at all, the same question Decision 9/§16 already answered for `core_essentials`?) rather than a per-item fix. |
| Neither `.FORGET` nor `pfs_*` | **173** | See §4.1. |

### 4.1 The 173: real mainline content, zero DESC by design — and the render surface can't show what IS there either

All 173 confirmed via direct oracle read (`sed -n '<line>p'`) to carry **zero** `DESC:`/`SPROP:`/
`BENEFIT:` anywhere on their own line — unlike equipment_modifier's false negatives, this is a real,
confirmed absence, not an extraction bug. But the raw rows are not empty:

```
sed -n '221p' ~/workspace/repos/pcgen/data/.../core_rulebook/cr_equip_arms_armor.lst
# Battleaxe (Base).COPY=Battleaxe  KEY:Battleaxe  ... COST:10  WT:6  CRITMULT:x3
#   CRITRANGE:1  DAMAGE:1d8  WIELD:OneHanded  SIZE:M  VISIBLE:YES
```

**Battleaxe — one of the most basic weapons in the entire game — is genuinely a stat-table row, not
a prose one.** This is PCGen convention across mundane weapons/armor: cost, weight, damage,
critical multiplier/range, and wield are all structured fields; PF1e's own printed weapon tables
show these as a spreadsheet row, never a paragraph. So `has_real_description`'s "no DESC" finding is
literally correct — **but Decision 7 condition 3 asks the wrong question for this content type.**
The apparent alternative — checking whether the *stats themselves* reach the player — also fails:
`apps/desktop/src-tauri/src/equipment_catalog.rs`'s `EquipmentCatalogEntryDto`
(`equipment_catalog.rs:112-134`) carries exactly `key`, `category`, `name`, `cost_gp`, `book`,
`description` — **no weight, damage, critical, or wield field at all.** A weapon's real mechanical
content (damage die, crit range/multiplier, weight) has no field to travel through on this DTO
today, description or otherwise.

**This is a genuine instrument gap, of a different shape than equipment_modifier's**: not an
extraction bug on existing data, but a render-surface that was never built to carry a weapon's stat
line at all. It is scoped to mundane, cost/weight/damage-only items — 173 in the `unmeasurable`
population specifically, but the same DTO gap plausibly affects every `equipment` unit of this shape
regardless of its current verdict (I did not check the `done` equipment population for the same
missing-field pattern this wave — out of scope, flagged as a follow-on question).

### 4.2 Tool evaluation — equipment

| Finding | Permanent or instrument gap? | Size |
|---|---|---:|
| `.FORGET` rows ingested as equipment | Permanent fact — should be a Structural Exclusion Register candidate or an ingest-time filter, operator's call per the Core-Essentials-residuals precedent (§16). | 2 confirmed, more likely exist outside the `unmeasurable` slice (not checked). |
| `pfs_*.lst` sourcing | Needs a scoping ruling, not a per-item fix. | 32 |
| Weapon/armor stat fields (cost/weight/damage/crit) have no DTO field to render through | Instrument gap — real engineering (new DTO fields + a new render path), not a quick add. | 173 confirmed in this population; likely wider (unchecked). |

## 5. feat — 565 units, three causes

| Cause | Units | Read |
|---|---:|---|
| `in_catalog_with_corpus_magnitude_but_no_observed_consumer` | **516** | The feat is real, in the engine's catalog, carries a real magnitude token — but the engine's posture-sweep probe never observed a computed delta. Per the code's own comment: "the effect may need a posture, an opponent or a combat action this engine does not model." **Instrument gap (posture-coverage), not a description problem** — distinct in shape from every other cause in this census. Not spot-checked against the raw oracle this wave (the claim here is about SIMULATION coverage, not corpus content, so an oracle read would not settle it the way it did for equipment/equipment_modifier). |
| `feat_served_description_is_a_placeholder_marker_not_prose` | **38** | Confirmed by design, not a bug: the corpus row's real text is a PI-redaction stand-in or upstream PCGen's own "[NOT IMPLEMENTED]" editorial admission. Correctly refused rather than served as fabricated prose. **Permanent-ish** — tied to the oracle's own text, not this program's tooling; revisit only if the oracle's text changes. |
| `text_only_but_corpus_record_carries_no_description_to_show_a_player` | **11** | Spot-checked all 11 against the raw oracle: **10 confirmed genuinely empty, 1 mismatch** (`ultimate_magic:um_feats.lst:70`, "Greater Wild Empathy" — carries real text on its raw line but still landed unmeasurable). Small population; the 1-in-11 mismatch rate here is much lower than equipment_modifier's 82%, suggesting this is NOT the same systemic bug — likely an isolated case. |

## 6. spell — 26 units, one cause, clean

```
sed -n '<line>p' <resolved .lst>   # all 26, see §7
```

All 26 confirmed genuinely empty on direct oracle read (0 mismatches). `evidence:
spell_list_entry_with_no_corpus_level_and_no_description` — the spell resolves in the engine's
spell list for this book but carries neither a resolved caster level nor real description text.
**Permanent fact for this specific book's printing of the spell** (most are likely reprints/variant
listings where the level table lives in a different book's row — not verified further this wave).

## 7. Reproducibility — scripts

Every count above came from small, throwaway Python scripts reading `docs/work-inventory.json`
(read-only) and the pinned oracle checkout (read-only), plus one `cargo test` invocation against the
already-committed test suite (no code edited). None were committed to the repo (scratch only, per
"measure, don't bank"). The load-bearing ones, reconstructable verbatim:

**Population + per-cause census** (§1): the two `python3 -c "..."` blocks above, runnable as-is.

**class_feature bucket A/B/C split** (§2.3): intersect each unmeasurable unit's `corpus_key.split(" ~ ")[0]`
against the `cargo test ... class_feature_pool_group_matches_enumerates_the_real_corpus_against_every_pool_entry
-- --nocapture` output's two printed sets (`newly recognised` + the 15 pool names that appear verbatim as a
corpus group), and separately its `excluded by CLASS_FEATURE_POOL_FALSE_SUFFIX_MATCHES` set.

**equipment_modifier / equipment / spell / feat description spot-checks** (§3, §4, §5, §6): for each
unit, `find ~/workspace/repos/pcgen/data/pathfinder -iname '<source_file>'` (preferring a path
containing `/<book>/`), then `sed -n '<source_line>p' <path>` and check for `SPROP:`/`DESC:`/
`BENEFIT:` substrings — the identical three prefixes `closure_prose_field_text()`
(`v06_work_inventory.rs:5551`) reads.

## 8. The opposite direction — does the formula interpreter (wave 25b) already reach any of these?

**No. Verified by tracing the code path, not by assumption.**

`apply_done_rung_stamps()` (`v06_work_inventory.rs:7429-7508`) is the **only** place
`derived_evaluator_fixture_check`'s output (`DERIVED_FIXTURE_CHECK_REPORT`) is consulted anywhere in
this binary. Its match arm is explicit and narrow:

```rust
wiring_class::WiringClass::Derived => {
    if matches!(item.verdict.status, "ingested-magnitude" | "grounded" | "text-complete")
        && derived_fixture_verified.contains(&item.id)
    { item.verdict.status = "fixture-verified"; }
}
```

It only **upgrades** a unit whose status is *already* `ingested-magnitude`/`grounded`/`text-complete`
— it can never touch a unit whose status is `unknown`. None of the 7 evidence tags in §1 route
through fixture/formula logic at all — they are all either owner-resolution failures (class_feature),
posture-probe misses (feat), or description-availability checks (equipment/equipment_modifier/spell).
**The interpreter has zero consumers wired to any of the causes in this census.** It cannot help a
unit reach `not-ingested`, `grounded`, or any other status by itself — a unit must first reach one of
the three upgradeable statuses through completely separate machinery before the interpreter's fixture
check is even consulted. This closes the operator's explicit "check the opposite direction" ask with
a definitive no, not a guess — worth recording precisely so a future wave doesn't re-open the
question without new evidence.

(Separately, even where it did apply: class_feature's `derived`-wiring_class subset within the
`unmeasurable` population is 1,080 units, but they are blocked on owner-resolution — §2 — which is
strictly upstream of any compute mechanism. Fixing the interpreter's reach doesn't help until
owner-resolution is fixed first.)

## 9. Cross-reference against `todo/*.md`

None of `sweeps.md`, `defects.md`, `blocked.md`, or `levers.md` name any of the causes in this
census by their evidence tag or exact mechanism. The closest existing entries:

- **`levers.md` L5** (option-pool catalog scaling, "2 of 27 pools registered") describes the
  *consequence* of the same gap §2 diagnoses (only 2 pools reach `text-complete` via
  `REGISTERED_POOL_GROUPS`), but does not name the **owner-resolution** fallback as a separate,
  cheaper, already-half-built fix — §2.2's finding (wire `class_feature_pool_group_matches` into
  `classify()`'s owner step) is new and smaller-scoped than L5's framing.
- **`sweeps.md` S2** ("as with monk, make a note to check that with all the objects") is the closest
  spirit-match to this whole wave's charter, but was never actually run against `unmeasurable`
  before now.
- **No existing row names** the equipment_modifier SPROP-extraction defect (§3), the equipment
  DTO stat-field gap (§4), or the ABP/First-Boon mis-typing found in class_feature bucket C (§2.4).
  **These are three new findings**, not re-discoveries — worth stating plainly per this wave's own
  bar ("re-discovering something already recorded is itself a finding worth reporting" — the
  converse, confirming something is genuinely new, is equally worth stating so the next reader
  doesn't waste a cycle re-deriving it).

I did not add rows to `todo/*.md` myself — reconciliation into those files is the integration
cycle's job per `todo/README.md`'s own division of labor, and this wave's charter is to write this
visibility artifact, not to edit the scheduling layer.

## 10. Summary — permanent fact vs. instrument gap, by unit

| Cause | Units | Permanent fact | Instrument gap | Notes |
|---|---:|---:|---:|---|
| class_feature: bucket A (matches a registered pool) | 612 | 0 | **612** | Fix identified, small, already built |
| class_feature: bucket B (false suffix match) | 53 | 0 | **53** | Real content, wrong owner map, needs per-group work |
| class_feature: bucket C (unrecognised group) | 2,393 | ~12 (ABP-shaped) | **~2,381** | Long tail, only top-30 spot-checked |
| feat: no observed consumer | 516 | 0 | **516** | Posture-coverage gap |
| feat: placeholder marker | 38 | **38** | 0 | Oracle-text-tied, not our tooling |
| feat: no description (11) | 11 | 10 | **1** | 1 confirmed mismatch, isolated |
| equipment_modifier: alias rows | 280 | ≤2 (spot-checked) | **≥22 confirmed, likely most of 280** | Resolver incomplete, lower bound only |
| equipment_modifier: declared rows | 136 | 25 | **111** | 82% false-negative rate, confirmed 3× against raw oracle |
| equipment: FORGET | 2 | **2** | 0 | Not content at all |
| equipment: pfs_*.lst | 30 (net of the 2 above) | unresolved | unresolved | Scoping question, not a per-item read |
| equipment: 173 (mundane stat-only) | 173 | 0 | **173** | Different shape — DTO field gap, not extraction bug |
| spell | 26 | **26** | 0 | Clean, confirmed |
| **Total** | **4,270** | **≈103** | **≈4,000+** (majority unresolved-precision in the alias/pfs/bucket-C tails) | |

**The headline: the overwhelming majority of the 4,270-unit `unmeasurable` pile is real content
blocked by instrument gaps, not absent content.** Only a small, mostly-already-enumerated slice
(spell's 26, feat's 38+10, equipment's 2 FORGET rows, equipment_modifier's ~25 pricing-ladder rows,
class_feature's ~12 ABP-shaped units — **≈103 units, 2.4% of the pile**) are genuine permanent facts.
Everything else is either a small, cheap, already-half-built fix (class_feature bucket A, 612 units)
or real, sizeable, honestly-priced engineering work whose exact size this wave could not fully
determine (class_feature bucket C's ~900 uncatalogued groups; the equipment_modifier alias
population's true false-negative rate; equipment's DTO gap corpus-wide reach beyond this population).

## 11. What I could not determine

- **Bucket C's remaining ~900 groups / ~1,900 units** (class_feature): only the top 30 by count were
  spot-checked. The four shapes found among those 30 (uncatalogued real pool, mis-shaped real pool,
  non-class subsystem, feat-owned content) do not obviously generalise to the rest — each needs its
  own oracle read.
- **The equipment_modifier alias population's exact false-negative count** (280 units): my
  `.COPY=` target resolution only matched 24 of 280; I have a verified lower bound (22) but not a
  full count. Building a real `.COPY=` chain resolver is corpus-loading work, out of this wave's
  "measure, don't touch the pipeline" scope.
- **equipment's `pfs_*.lst` sourcing** (32 units): flagged as a scoping question, not resolved —
  I do not know whether PFS legality lists should be in-corpus scope at all.
- **The exact root cause of equipment_modifier's per-record SPROP-extraction failure**: confirmed
  reproducible and real (three independent verified instances across two books), but I did not trace
  it into the corpus-loading/closure-construction code that would explain *why* "Ghost Touch ~
  Armor" succeeds while "Ghost Touch ~ Weapon" and "~ Amulet of Mighty Fists" fail — that requires
  reading and reasoning about code this wave's charter puts out of scope (fixing, or the diagnosis
  depth that borders on it).
- **Whether the equipment DTO stat-field gap (§4.1) reaches beyond the `unmeasurable` population**
  into equipment already marked `done` — not checked this wave.
