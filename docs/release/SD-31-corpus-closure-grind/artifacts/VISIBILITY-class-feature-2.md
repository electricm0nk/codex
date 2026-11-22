---
canonical: true
owner: sd31-wave30-class-feature-lane-3
wave: 30
purpose: >
  Wave 30, lane 3. Re-examines the four gaps wave 28's class_feature lane named and did not
  close: G1's 811/817 unclassified option-pool names (Ruling §18 OPEN vs EXCLUSIVE), G3's
  regex-proxy interpreter-readability split, G6's 29 near-miss units, and whether
  `granted_via_archetype:true` is deliberate scope or accidental narrowing. Banks nothing.
  `docs/work-inventory.json` is untouched throughout (confirmed byte-identical at start and
  end, md5 `d64ddfc677fd1683f5b7638889a25c54`).
started: 2026-08-21
board_at_start: "13,458 / 38,372 (35.08%) — frozen, unchanged by this document"
---

# VISIBILITY — `class_feature` deep re-examination (wave 30, lane 3)

## 0. Scope, base, and the one honesty rule this document follows

Base verified: `3627601f1` (tranche/11 tip), `data`/`scripts`/`schemas` all present. Board
re-derived fresh at the start of this pass and matches the wave-30 dispatch brief exactly:

```
python3 -c "
import json, sys, collections
sys.path.insert(0,'scripts/observer')
import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
v=lambda u: P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))
c=collections.Counter(v(u) for u in U)
print(len(U), c.most_common())
"
# -> 38372 [('not-started', 18645), ('done', 13458), ('unmeasurable', 3763),
#           ('in-progress', 1231), ('held', 1230), ('deferred', 45)]
md5sum docs/work-inventory.json   # d64ddfc677fd1683f5b7638889a25c54, unchanged start-to-end
```

**Everything below is measured against this live commit, not transcribed from THE-BOX.** Every
G1/G3/G6 population count in THE-BOX (wave 28) is now stale — wave 29's levers moved real units
through these exact evidence codes between wave 28 and today. Re-deriving instead of trusting the
inherited numbers is itself the first finding (§1, §3).

**Measurement, not banking.** Two throwaway Rust probes were built in an isolated
`CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/w30-classfeature2` (deleted at the end of this
cycle) to ask the REAL production parser/matcher functions directly instead of reimplementing
them in Python. One (`w30_g3_probe`) was a new, temporary `src/bin/*.rs` file, deleted before
this cycle's commit. The other was a temporary `if std::env::var(...)` block added to
`src/bin/v06_work_inventory.rs`'s `main()`, run, and then reverted with `git checkout --`
before this cycle's commit — confirmed by `git status --porcelain` returning empty immediately
after. Neither wrote `docs/work-inventory.json`; both only read `data/corpus/`,
`data/class_feature_grants/`, and the pinned PCGen oracle at
`/home/ubuntu/workspace/repos/pcgen/data` (`git rev-parse HEAD` = `7f818006e371188e5717fd18d74d18a420747fc6`,
confirmed matching `scripts/pcgen-oracle-pin.env`).

**What I could not determine, stated up front so it is not lost in the detail below:** the
OPEN/EXCLUSIVE axis for the majority of G1's real-prose pool population (424 of 444 non-empty
group names, 1,110 of 1,656 non-empty units) is still unclassified after this pass. This document
extends wave 29's 28-entry classification by 20 more names (546 units) and stops there — see §2.

---

## 1. G1 (option-pool records) — population has drifted; re-derived fresh

```
python3 -c "
import json, sys, collections
sys.path.insert(0,'scripts/observer')
import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
cf=[u for u in U if u.get('kind')=='class_feature']
v=lambda u: P.doneness_verdict(u.get('wiring_class'), u.get('status'), u.get('kind'))
ns=[u for u in cf if v(u)=='not-started']
g1=[u for u in ns if u.get('evidence')=='class_feature_option_pool_record_not_held_by_engine']
group=lambda u: u.get('corpus_key','').split(' ~ ')[0].strip()
c=collections.Counter(group(u) for u in g1)
print(len(g1), 'units', len(c), 'distinct group names')
"
# -> 3064 units, 695 distinct group names
```

**Corrected: 695 names / 3,064 units — not THE-BOX's 817 names / 3,347 units.** The drop is real,
not noise: wave 29's lever #2 (`class_feature_pool_group_matches()` wired into `classify()`'s
owner-resolution fallback) moved every unit matching a `CLASS_FEATURE_POOLS`-registered name OUT
of G1's evidence code entirely, because G1 means "no table/picker/catalog holds it" — once the
matcher recognizes a name, that stops being true for it.

**Consequence: the overlap wave 28 measured (6 names / 161 units) is now ZERO.**

```
python3 -c "... poolset = the 28 CLASS_FEATURE_POOLS names, lowercased ..."
# -> overlap names 0, overlap units 0, remaining names 695, remaining units 3064
```

Confirmed independently by re-running the registry-count command against `src/bin/v06_work_inventory.rs` (`awk` between the `const CLASS_FEATURE_POOLS` and `];` markers, piped to `grep -c` for entry-opening lines): 28 (matches wave 29's own re-derivation, not the dispatch brief's stated
"27-entry" or THE-BOX's "817/6/161"). **Every one of today's 695 names is unclassified by any
prior wave's registration work** — the population isn't overlapping-and-shrinking, it's a clean,
disjoint remainder.

---

## 2. G1's OPEN vs EXCLUSIVE axis (Ruling §18) — real progress, real remainder stated exactly

### 2.1 The population is not one shape — split by whether there is real prose to classify at all

Ruling §18's OPEN/EXCLUSIVE question is about whether a reference-catalog PATTERN is honest for a
pool's *prose*. That question presupposes prose exists. It does not, for a third of this
population:

```
python3 -c "
# index data.description for every class_feature corpus record; join to G1 units by corpus_key
# empty = description field present but '' after strip; no_record = corpus_key not in index at all
"
# -> total 3064
#    no corpus record at all:      1017  (33.2%)
#    ingested, empty description:   391  (12.8%)
#    ingested, real description:   1656  (54.0%)
#    SUM 3064
```

This is a genuinely different, decidable split from anything wave 28 filed, and it reframes the
"817/695 unclassified pool names" problem into three separate next-steps, not one:

- **1,017 (33.2%) have no corpus record at all** — the raw `.lst` closure scan found the row;
  nothing was ever ingested for it. Ruling §18 cannot apply to a record that does not exist yet;
  this sub-population needs ingestion, not classification.
- **391 (12.8%) are ingested with an EMPTY description.** Spot-checked (`Master of Many Styles ~
  Aldori Style`, `Skill unlock ~ Acrobatics`, `Combat Trick ~ Agile Maneuvers`, `Blade Skill ~
  Absorbing Blade` — all four confirmed empty by direct read): these read as bare
  reference/pointer records naming a feat or sub-choice whose real prose lives on a DIFFERENT
  corpus record (a feat, most often) — the same shape `todo/levers.md` L9 already investigated for
  the class_feature→feat bridge. Ruling §18's OPEN/EXCLUSIVE test does not apply to these directly
  either; they need the reference-bridge question answered first (does the bridge reach this
  group's real content, and is THAT content's own selection mechanic open or exclusive).
- **1,656 (54.0%) are ingested with real prose.** This is the actual population Ruling §18's axis
  is about, and is where this section's classification work below is spent.

### 2.2 A defect this split surfaced: one group is not a pool at all

`Weapon and Armor Proficiency` (13 units, all `core_rulebook`) looked like a pool name in every
census so far. It is not:

```
core_rulebook | Weapon and Armor Proficiency ~ Adept     | data.class = "Weapon and Armor Proficiency"
core_rulebook | Weapon and Armor Proficiency ~ Aristocrat | data.class = "Weapon and Armor Proficiency"
core_rulebook | Weapon and Armor Proficiency ~ Bard       | data.class = "Weapon and Armor Proficiency"
... (13 rows, one per class: Adept, Aristocrat, Bard, Druid, Eldritch Knight, Expert, Fighter,
     Loremaster, Monk, Paladin, Ranger, Rogue, Warrior)
```

PCGen's own `KEY:` for each class's flat, universal, non-chosen proficiency grant is
`"Weapon and Armor Proficiency ~ <ClassName>"` — the FEATURE name in the group position and the
owning CLASS in the suffix position, the exact reverse of every other class_feature key's
`"<Class> ~ <Feature>"` shape this program's tooling assumes everywhere else. G1's own
group-prefix heuristic has no way to see the real owner here, so it correctly reports "no table
holds this by name" while being wrong about WHY — these are 13 real, single-owner, unconditional
class features (not a choice at all, so Ruling §18's OPEN/EXCLUSIVE axis does not apply to them
either), mis-swept into the "unowned option pool" bucket by a reversed-key-shape blind spot.
Filed as `defects.md` D9.

### 2.3 Classified this wave: 20 more group names (546 of the 1,656 real-prose units), by the same method wave 29 used

Same method as `todo/levers.md` L5's own 28-entry pass, and the SAME caveat: **self-derived from
standard, widely-published PF1e class-mechanic text, cross-checked against each group's own
sampled DESC prose (not memory alone), NOT per-group re-verified against the pinned oracle.**
PROPOSED, not registered. Confidence tier stated per row; low-confidence rows should not be acted
on without a real oracle check.

| Pool | Likely owner | Axis | Confidence | Why |
|---|---|---|---|---|
| Wild Talent | kineticist | **OPEN** | High | Kineticist gains an additional wild talent at nearly every level, keeps all previously known — confirmed against sampled DESC text (elemental/utility talents, no "instead of" language) |
| Domain Power | cleric/inquisitor | **EXCLUSIVE** | High (operator-ruled) | Ruling §18's own worked example |
| Ki Power | (Monk-adjacent archetype) | **OPEN** | Medium-High | Same repeatable-growing-list shape as base Monk's own Ki Powers (already a G6 near-miss unit under the same name, §4) |
| Demonic Obedience | (Mythic Adventures) | **EXCLUSIVE** | Medium | Each member names ONE demon lord's daily ritual+boon; a character follows one patron's obedience, not several simultaneously — closer to a once-chosen, fixed track than a repeatable pick |
| Implement School Focus Power | occultist | **OPEN** | High | Occultist gains additional focus powers per bound implement school as they level, keeps all |
| Hunter's Tricks | hunter | **OPEN** | High | Growing repeatable list, same shape as Ranger's Combat Style feats |
| Inspired Discovery | investigator | **OPEN** | Medium-High | Investigator analogue of Alchemist's Discovery (already-ruled OPEN) |
| Social Talent | vigilante | **OPEN** | Medium-High | Vigilante Talents/Social Talents are both growing, repeatable lists gained across levels |
| Inspiring Command | (Battle Herald prestige class) | **OPEN** | Medium | Growing command list, same shape as Bard's Versatile Performance (already-ruled OPEN) |
| Cruelty | antipaladin | **OPEN** | High | Direct mechanical mirror of Paladin's Mercy (already-ruled OPEN, `todo/levers.md` L5) |
| Phrenic Amplification | psychic | **OPEN** | Medium | Growing repeatable list tied to the psychic's phrenic pool |
| Runeforger | (Runeforger archetype) | **OPEN** | Low-Medium | Reads as a Discovery-shaped growing rune list; not independently confirmed |
| Bodily Mutation | (mutation-themed archetype) | **OPEN** | Low-Medium | Reads as a Discovery-shaped growing mutation list; not independently confirmed |
| Masterful Trick | (Cult Leader-adjacent) | **OPEN** | Low | Growing-list shape by analogy only; source class not confidently identified |
| Environment Weapon | ranger/hunter (terrain-linked) | **OPEN** | Low | Reads as a Favored-Terrain-linked (already-ruled OPEN) sub-list; not independently confirmed |
| Feral Child | (heritage/origin trait) | **EXCLUSIVE** | Low-Medium | Sample DESC reads as a one-time origin choice ("may choose one specific type of animal as the type that raised her"), not a repeatable pick |
| Wave Warden Feat | (Wave Warden archetype) | **OPEN** | Low | Reads as a bonus-feat-list pool, same shape as Fighter's Bonus Feats; not independently confirmed |
| Composite Blast | kineticist | **UNCLEAR** | Low | Kinetic blast types are feat-gated and typically fixed once taken, closer to EXCLUSIVE-shaped, but not confidently distinguished from a repeatable unlock here |
| Infiltrator | (unidentified) | **NOT CLASSIFIED** | — | Sample DESC (`amphibious, darkvision 60 ft., Iron Will, natural armor +2`) reads as a creature-type-keyed transformation bundle (Shifter/Skinwalker-Aspect-shaped), not confidently matched to a known mechanic |
| Weapon and Armor Proficiency | (13 different classes) | **N/A — not a pool** | High | See §2.2; excluded from this axis entirely, filed as a defect instead |

**Net this wave: 15 new OPEN, 2 new EXCLUSIVE, 1 UNCLEAR, 1 explicitly not-a-pool, 1 not
classified at all (Infiltrator) — 546 of the 1,656 real-prose units (33.0%) now have a proposed
axis, up from 0 before this pass** (recall §1: the 6-name/161-unit overlap wave 28 measured no
longer applies to G1 at all).

### 2.4 A secondary, noisy signal tried and reported honestly: does this group ever appear as an archetype auto-grant target?

```
python3 -c "... cross-reference each G1 group name against every 'key' group-prefix in
    data/class_feature_grants/**/*.json ..."
# -> 220 of 695 group names (1,002 of 3,064 units) ALSO appear as an auto-grant target elsewhere
#    475 of 695 group names (2,062 of 3,064 units) never do
```

**This is NOT a reliable OPEN/EXCLUSIVE discriminator on its own — disclosed here so nobody
reaches for it as one.** `Wild Talent` (confidently OPEN, above) shows up in this "overlap" set
too: Kineticist's own `.MOD` row auto-grants exactly one specific member (`Wild Talent ~ Extreme
Range`, a real, known PF1e rule — every Kineticist gets it automatically at 9th level) alongside
the normal, separately-chosen OPEN pool. A pool can be BOTH a real open, repeatable-pick catalog
AND separately auto-grant one specific member unconditionally; overlap with
`data/class_feature_grants` proves neither shape by itself. Useful only as a prompt to check the
SPECIFIC overlapping member individually, never as a group-level classifier.

### 2.5 What remains unclassified, stated exactly

- **1,017 units, no corpus record** — needs ingestion, not OPEN/EXCLUSIVE classification (§2.1).
- **391 units, empty description across an unknown number of names** — needs the reference-bridge
  question answered first (§2.1), not this axis directly.
- **424 of 444 real-prose group names (1,110 of 1,656 real-prose units, 67.0%) still have NO axis
  assigned at all after this wave.** Every one of them needs the same per-name mechanic read this
  section applied to the top 20 — real work, not a tool problem (the triage tool `THE-BOX.md` §4
  proposes only routes a group to a bucket by TYPE-facet text; it cannot answer OPEN vs EXCLUSIVE,
  which is a question about the owning class's own selection mechanic, not the record's own text
  shape).

---

## 3. G3 (magnitude-bearing, no consumer) — the real parser, not a regex proxy

### 3.1 Population re-derived fresh — also drifted

```
python3 -c "... g3 = ns filtered to evidence == 'no_explanation_id_and_no_diagnostic_names_this_feature' ..."
# -> 3320 units (not THE-BOX's 2,583)
# by wiring_class: computed 2151, derived 743, static 419, ambiguous 7
```

### 3.2 The real interpreter, asked directly — `formula_interpreter::recognises_shape()`

The wave-28 regex proxy scanned `BONUS`/`SA`/`ABILITY`/`DESC` raw-token TEXT for four known-refused
SUBSTRING shapes. This pass instead extracts each unit's real corpus record's `BONUS`/`DEFINE`
formula field (via the interpreter module's OWN `extract_formula_field` helper — the same
positional extraction the module's tests use) and calls `recognises_shape()`, the module's real
recursive-descent parser, exactly the function its own doc comment names as "the metric the
corpus-wide shape-coverage scan uses."

A throwaway probe (`src/bin/w30_g3_probe.rs`, built in the isolated `CARGO_TARGET_DIR`, deleted
after use) does this for all 3,320 units:

```
G3 total units in list: 3320
no corpus record found: 167
no BONUS/DEFINE formula-shaped token at all: 864
has formula token(s), ALL refused by real parser: 3
has formula token(s), AT LEAST ONE readable by real parser: 2286
SUM check: 3320 == 3320
```

By `wiring_class`:

| wiring_class | readable | no_formula_token | no_corpus_record | all_refused |
|---|---:|---:|---:|---:|
| computed | 1,435 | 602 | 113 | 1 |
| derived | 453 | 256 | 32 | 2 |
| static | 391 | 6 | 22 | 0 |
| ambiguous | 7 | 0 | 0 | 0 |

**This is a two-orders-of-magnitude correction to the wave-28 filed number.** The regex proxy
claimed 554 refused-shape (14.8%); the real parser refuses **3** (0.09%) of the same-shaped
population. The proxy claimed 882 readable (23.6%); the real parser reads **2,286** (68.9%). The
"1,690 unclassified" bucket the proxy could not characterize resolves cleanly here into 864
genuinely-no-formula-token records plus 167 with no corpus record at all — nothing is left
unclassified by this measurement. `defects.md` D10 files the corrected figures; the proxy's own
554/882/1,690 split should not be used for any interpreter-scoping decision going forward.

The 3 genuine refusals (real, reproducible, sample formulas quoted verbatim from the pinned
oracle, none PI-flagged):

```
Voice of the Wild ~ Wild Knowledge :: max(floor(BardLVL/2))
Aquatic Druid ~ Deep Diver :: var("DruidLVL")/2
Monk ~ Standard Class :: classlevel("Monk","APPLIEDAS=NONEPIC")*3/4  (and 5 sibling calls)
```

`max()` with a single argument, `var("X")` as a function call, and `classlevel()`'s two-argument
form (a class name plus an `APPLIEDAS=` qualifier) are the three real gaps — none of the four D3
shapes (`PREVARGTEQ`-embedded conditionals, boolean-to-int coercion, `&&`, `skillinfo()`) appear
in this population at all, contradicting the proxy's own scan basis.

### 3.3 A second pass tried: DESC's own `%N`-substitution pipe-tail — real gain, but flagged, not headline

`extract_formula_field` (by its own doc comment) only reads `BONUS`'s third field or `DEFINE`'s
second field — not `DESC`'s own `%1`/`%2`-substitution formula segments (`DESC:<text with
%1>|<formula for %1>|...`), the single most common magnitude-carrying shape in this corpus
(`Animist ~ Dominate Spirit`: `DESC:...%1 times per day.|floor((ShamanLVL-6)/4)`, zero BONUS/DEFINE
tokens at all). Extending the probe (not the shipped `extract_formula_field`) to also try every
DESC pipe-tail segment:

```
no BONUS/DEFINE/DESC-tail formula-shaped token at all: 633
has formula token(s), ALL refused by real parser: 13
has formula token(s), AT LEAST ONE readable by real parser: 2507
```

**This number is reported, not adopted as the headline, because the extension itself has a
real false-positive shape.** `Shaman Spirit Hex ~ Thunder Foot` carries a SECOND `DESC:` token
whose pipe-tail is `PRECLASS:1,Shaman=7` — a PRE-token gate condition PCGen embeds as a
conditional-DESC-line marker, not a `%N`-substitution formula at all. The naive
"everything after the first `|` is a formula" probe extension mis-reads this as a refused formula
(`recognises_shape("PRECLASS:1,Shaman=7")` correctly fails, but for the wrong reason — it was
never a formula to begin with). This inflates BOTH the readable count (some are real DESC formulas)
and the refused count (some are DESC-embedded PRE-gates, not formulas) by an unknown, unmeasured
amount. **Use §3.2's BONUS/DEFINE-only figures as the defensible headline**; this DESC-tail
number is a real, useful lower bound on how much §3.2 under-counts readability, not a corrected
replacement for it. A convention-aware DESC-tail extractor (distinguishing a `%N` formula segment
from a bare PRE-token gate segment) is real, small, follow-up work — filed as `levers.md` L11.

---

## 4. G6 (29 near-miss units) — built, and confirmed: NOT the Ninja/Samurai shape

### 4.1 Population confirmed unchanged (wave 29 did not touch this evidence code)

```
grep -c "class_feature_no_dedicated_magnitude_id_matched_the_record_slug" docs/work-inventory.json
# -> 29
```

Book split reproduced exactly as THE-BOX's own correction states: `pathfinder_unchained` 24 /
`core_rulebook` 3 / `advanced_players_guide` 2.

### 4.2 A live engine build, run against the pinned oracle, answers the open question directly

A temporary debug block in `src/bin/v06_work_inventory.rs`'s `main()` (reverted with
`git checkout --` after use — `git status --porcelain` confirmed empty afterward) dumped the real
`facts.explanation_ids` for the 7 owning classes and, for each of the 29 units, peeled trailing
`_<word>` segments one at a time from every non-roster (`.corpus_record.`-excluded) id under that
owner until the remainder's trailing dot-segment equalled the unit's own `feature_slug` —
reproducing `id_matches_feature_slug_after_known_magnitude_suffix_strip`'s own real algorithm
exactly (not a re-implementation guess), extended to report HOW MANY words away a candidate is
when the single-strip production function fails.

**Result: none of the 29 are one-missing-downstream-table-row (the Ninja/Samurai shape). Four
different, real mechanisms are actually in play:**

| Shape | Count | Units | What it means |
|---|---:|---|---|
| **One allowlist word away** | **4** | Unchained Barbarian ~ Rage Powers (`rage_powers_known`), Unchained Monk ~ Ki Powers (`ki_powers_known`), Unchained Rogue ~ Rogue Talents (`rogue_talents_known`), Unchained Rogue ~ Uncanny Dodge Tracker (`uncanny_dodge_tracker_steps`) | A real, non-roster, per-feature magnitude id exists and is EXACTLY one trailing word (`"known"` or `"steps"`) away from matching — genuinely the cheapest fix in this population: add those two words to `CLASS_FEATURE_ID_MAGNITUDE_SUFFIXES`. **This is the S9-shaped subset** — small, cheap, real. |
| **One dot-namespace miss** | **1** | Summoner ~ Eidolon | 10 real, non-roster, per-feature magnitude ids exist (`class_feature.apg.summoner.eidolon.base_attack_bonus`, `.base_land_speed`, `.bite_damage_die`, `.evolution_points_spent/unspent`, `.evolution_pool`, `.improved_natural_armor`, `.max_natural_attacks`, `.natural_armor_bonus`, `.quadruped_ability_bonuses`, `.total_natural_armor`) — genuinely computed Eidolon stats — but the matcher only ever strips a trailing underscore-suffix from the LAST dot-segment; it cannot see `eidolon` as a MIDDLE dot-segment (`...summoner.eidolon.base_attack_bonus`). A namespace-aware matcher extension (checking whether any middle segment equals `feature_slug`, not only the trailing one) would unlock this specific unit and is worth building once, since a dot-namespaced convention could recur for other multi-attribute features. |
| **Two words away** | **10** | Unchained Barbarian ~ Fast Movement/Greater Rage/Mighty Rage/Rage; Unchained Monk ~ Fast Movement/Stunning Fist; Unchained Rogue ~ Finesse Training/Rogues Edge; Unchained Summoner ~ Eidolon/Summon Monster | A real magnitude id exists but needs TWO trailing words peeled (e.g. `fast_movement_bonus_feet` needs both `"feet"` and `"bonus"` stripped to reach `fast_movement`). The single-strip production function is deliberately narrow (its own doc comment cites `OPEN-ISSUES.md` row 78 declining the wider "relax exact-suffix to a scoped-but-looser check" as a separate, higher-blast-radius decision, not this cycle's to make) — these 10 are exactly the population that wider fix would reach, and are NOT a one-row S9 fix. |
| **No candidate at all** | **14** | Summoner ~ Greater Aspect; Fighter ~ Bonus Feats/Weapon Training; Ranger ~ Combat Style Feat; Unchained Barbarian ~ Uncanny Dodge Tracker; Unchained Monk ~ AC Bonus/Bonus Feat/Flurry of Blows/Ki Pool/Style Strike; Unchained Rogue ~ Evasion/Skills; Unchained Summoner ~ Skills/Spells | No non-roster explanation id anywhere under the owning class contains this feature's slug, at any peel depth. A genuine, unmodelled engine gap — real per-feature computation is needed, not a naming-convention fix. |
| **Sum** | **29** | | ✓ |

**Answer to the operator's framing ("if even a handful match the Ninja/Samurai shape, that is a
very cheap close"): zero of the 29 do, but 4 are an equally cheap sibling shape** (a two-word
allowlist gap rather than a missing table row), and a 5th (Summoner ~ Eidolon) is a
single-mechanism matcher extension away from unlocking 10 already-computed values. Filed as
`levers.md` L12 (the 4+1 cheap subset) and `sweeps.md` S12 (whether the same allowlist/namespace
gaps recur outside these 29 — not checked corpus-wide this wave).

---

## 5. `granted_via_archetype` — deliberate, tested, and correct; one real, currently-incidental gap found

### 5.1 The field is not "every row true" — read wrong by the dispatch brief's framing

```
grep -rho '"granted_via_archetype": *[a-z]*' data/class_feature_grants/ | sort | uniq -c
#   362 false
#  3121 true
```

### 5.2 The generating script, read directly: computation is correct and deliberate

`resolve_token()` (`src/rules_core/cache_gen/class_feature_grants.rs:404-528`) sets
`granted_via_archetype = matches!(gate_kind, GateKind::Preclass) && row_is_archetype`, where
`row_is_archetype` reads the EMBEDDING row's own `CATEGORY:` field directly from the corpus text —
never defaulted, never inherited. Verified live against the pinned oracle, not just the module's
own doc comment: `ultimate_wilderness/uw_abilities_class.lst:963`'s "Desert Raider" archetype row
(`CATEGORY:Archetype`) embeds `ABILITY:...|Desert Raider ~ Light Step|PRECLASS:1,Rogue=3` — the
shipped fact (`Desert Raider ~ Light Step`, class `Rogue`, level `3`, `granted_via_archetype:
true`) matches exactly, level included. 9 further random samples (`ultimate_intrigue`,
`advanced_players_guide`, `horror_adventures`, `advanced_class_guide`, `occult_adventures`,
`adventurers_guide`) all independently confirmed the same shape.

### 5.3 The consumer, read directly: `true` is correctly treated as a refusal, `false` is correctly treated as safe-to-auto-grant

`class_feature_grant_consumer.rs:379`: `if row["granted_via_archetype"].as_bool().unwrap_or(true)
{ continue; }` — refuses the record from `resolvable_grants()` (which STRICT-auto-grants a
base-class feature to every member of that class). This is the documented, deliberate fix for a
named `SD-31 wave-23` fabrication defect (`"Rogue ~ Careful Disarm"`, archetype-only, would
otherwise auto-grant to every Rogue) — not accidental narrowing.

### 5.4 The one real gap: multi-hop archetype chains, currently protected only by an incidental cross-book conflict

`granted_via_archetype` only inspects the row DIRECTLY embedding the `ABILITY:` token — a genuine,
disclosed, single-hop limitation (the field's own doc comment states this explicitly). A
multi-hop chain (an archetype's `CATEGORY:Archetype` row grants a `CATEGORY:Special Ability` row,
which itself embeds a further `ABILITY:` token naming the BASE class, not the archetype) is
invisible to it. Found live: `ultimate_combat/uc_abilities_class.lst:584`'s "Sermonic Performance"
row (Evangelist archetype, `CATEGORY:Special Ability`) embeds `ABILITY:...|Cleric ~ Channel
Energy|PRECLASS:1,Cleric=3` — `granted_via_archetype: false` is shipped for this fact, even though
it originates from an archetype-only feature-replacement chain.

**Checked whether this is currently exploitable — it is not, but only by coincidence.** The
STRICT/base-class-name-matching population (233 units where the key's own group text equals the
class name — the only shape `class_feature_grant_consumer.rs`'s name filter would even let through)
contains exactly 4 `preclass`-gated + `granted_via_archetype:false` facts. 3 of the 4
(`Cleric ~ Channel Energy` level 3, `Druid ~ Wild Shape` level 6, `Paladin ~ Smite Evil` level 2)
disagree with the base class's OWN `.MOD`-row-sourced fact for the same `(class, key)` pair (levels
1/4/1 respectively) — `resolvable_grants()`'s cross-book-conflict rule drops the WHOLE pair when
levels disagree, so these 3 never reach a credited grant regardless. The 4th
(`Gunslinger ~ Gun Training`, level 1) has no conflicting fact and stands alone — verified against
the oracle directly: it is genuinely NOT archetype-gated at all (an "Optional Rules Firearms"
house-rule toggle row, `CATEGORY:Internal`, not an archetype swap), so its `false` is correct on
independent grounds. **Zero live fabrication risk today, proven by checking all 4 candidates
individually against the oracle — not assumed.** But the protection for the 3 conflict-dropped
cases is an accident of the current corpus's own level numbers, not a structural guarantee: a
future book whose multi-hop archetype chain happens to restate the SAME level as the true base
class would ship an undetected `false` and silently auto-grant an archetype-only feature to the
whole base class. Filed as `defects.md` D11.

---

## 6. Todo entries filed

See `todo/sweeps.md` S12, `todo/defects.md` D9/D10/D11, `todo/levers.md` L11/L12. Every group
above maps to one of these five entries; none is left without one.

## 7. What I could not determine, stated plainly

- **424 of 444 real-prose G1 group names (1,110 of 1,656 real-prose units) have no OPEN/EXCLUSIVE
  axis yet.** This wave extended wave 29's 28-name pass by 20 more names; the remainder needs the
  same per-name mechanic read, at scale, or a genuinely new corpus-grounded discriminator this
  pass did not find (§2.4's auto-grant-overlap signal is real but proven insufficient on its own).
- **`Infiltrator` (32 real-prose units) and `Composite Blast` (13 units)** were read and left
  unclassified rather than guessed — see §2.3's table for the specific uncertainty in each.
- **The DESC-tail formula-extraction extension (§3.3)** is a real, useful lower bound but not a
  trustworthy headline number until a convention-aware extractor (distinguishing a `%N`-formula
  pipe-tail from a PRE-token-gate pipe-tail) exists.
- **Whether the same allowlist-gap/dot-namespace-miss shapes G6 found recur outside these 29
  units** was not checked corpus-wide — filed as `sweeps.md` S12, not answered here.
- **1,017 no-corpus-record + 391 empty-description G1 units (§2.1)** were characterized by shape
  but not resolved — they need ingestion and reference-bridge work respectively, neither of which
  this measurement-only wave performed.

---

## 8. Reproduction commands (every number above, in one place)

```
# Base/board
git log --oneline -1                          # 3627601f1
md5sum docs/work-inventory.json               # d64ddfc677fd1683f5b7638889a25c54 (start == end)

# §1 G1 population
python3 -c "import json,sys,collections; sys.path.insert(0,'scripts/observer'); \
import pf1e_dashboard_producer as P; d=json.load(open('docs/work-inventory.json')); \
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]; \
cf=[u for u in U if u.get('kind')=='class_feature']; \
v=lambda u: P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind')); \
ns=[u for u in cf if v(u)=='not-started']; \
g1=[u for u in ns if u.get('evidence')=='class_feature_option_pool_record_not_held_by_engine']; \
group=lambda u: u.get('corpus_key','').split(' ~ ')[0].strip(); \
c=collections.Counter(group(u) for u in g1); print(len(g1), len(c))"
# -> 3064 695

# §1 pool registry count
awk '/const CLASS_FEATURE_POOLS/,/^\];/' src/bin/v06_work_inventory.rs | grep -c '^\s*("'
# -> 28

# §3 G3 population
python3 -c "... evidence == 'no_explanation_id_and_no_diagnostic_names_this_feature' ..."
# -> 3320

# §3 real-parser probe (built + run in isolated CARGO_TARGET_DIR, deleted after)
# src/bin/w30_g3_probe.rs -- reads data/corpus/**/class_feature/**/*.json raw_tokens,
# calls codex::rules_core::pilot_compute::formula_interpreter::{extract_formula_field,recognises_shape}

# §4 G6 population
grep -c "class_feature_no_dedicated_magnitude_id_matched_the_record_slug" docs/work-inventory.json
# -> 29

# §4 G6 analysis: temporary debug block in src/bin/v06_work_inventory.rs main(), env-gated
# (W30_G6_ANALYZE=<owner|feature_slug|corpus_key file>), run against
# PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data, reverted via `git checkout --` after.

# §5 granted_via_archetype
grep -rho '"granted_via_archetype": *[a-z]*' data/class_feature_grants/ | sort | uniq -c
# -> 362 false, 3121 true

# Post-cycle cleanup verification
git status --porcelain   # empty except this new artifact file
```
