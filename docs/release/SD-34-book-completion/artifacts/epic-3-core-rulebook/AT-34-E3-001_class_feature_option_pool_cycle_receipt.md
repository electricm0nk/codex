# Cycle 5 — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`class_feature_option_pool_record_not_held_by_engine` mechanism)

Re-derived the mechanism population fresh at this cycle's starting HEAD (`1de361c850`, unchanged
by this cycle until the commit below): **52 of 687** `core_rulebook` bucket-B units (bucket B's
own whole-book total moved 736 → 687 between cycle 4's own receipt and this cycle's start —
sibling mechanisms' cycles, not this one, landed in between; re-derive command below).

```
$ python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
u = [x for x in d['units'] if x['book']=='core_rulebook' and x['status']=='engine-does-not-hold'
     and x['evidence']=='class_feature_option_pool_record_not_held_by_engine']
print(len(u))
"
52
```

## The task brief's own instruction: build a subsystem, or stop reporting the same zero

Cycle 4 closed 0/52 and re-confirmed the 28/13/9/2 sub-cause split (proficiency/grant
possession-tracking, class-skill/companion-mount attribution, wizard opposition-school
tracking, Domain Power). Its own grep for an existing tracking probe
(`grep -n "proficiency.*wired\|weapon_prof.*wired" src/bin/v06_work_inventory.rs`) found
nothing and concluded "no proficiency/grant-possession tracking probe exists anywhere in this
engine". **That grep pattern was too narrow, not the underlying fact.** A second, unfiltered
search (`grep -rln "proficien" src/rules_core/*.rs`) surfaced a real, already-shipped, already-
tested class-based weapon-proficiency table that has nothing to do with the word "wired":
`src/rules_core/rules_tables/crb/weapon_tables.rs`'s `CLASS_WEAPON_PROFICIENCIES` — 27+ base
classes, each row transcribed from the class's own corpus record (per that file's own doc
comment: "an earlier survey concluded seven of them had none; that was a grep artifact"), and
already consumed today by `pilot_compute/mod.rs`'s `character_is_proficient_with` for real
combat nonproficiency-penalty checks. This table was never wired to the atlas.

## What was and was not safely closable — verified per-record, not by name shape

The 28-unit proficiency group splits into weapon-proficiency-flavored records (18: `Weapon Prof
~ Auto/Martial/Simple`, `All {Automatic,Martial Weapon} Proficiencies`, `Single Simple Weapon
Proficiency`, `Weapon Proficiencies ~ {Bard,Cleric,Druid,Monk,Rogue}`, `Weapon and Armor
Proficiency ~` ×7 combined records) and armor/shield-flavored records (6: `Armor Prof ~
{Heavy,Light,Medium}`, `Armor Training ~ Heavy Armor`, `Shield Prof`, `Shield Prof ~ Tower`) —
**no armor/shield proficiency table exists anywhere in this engine** (confirmed by grep), so
those 6 stay genuinely unclosable this cycle, as do the generic shared indirection targets
(`Weapon Prof ~ Auto/Martial/Simple`, `All * Proficiencies`) and the `CHOOSE`-based
`Single Simple Weapon Proficiency` — none of those map 1:1 to a class-scoped table row, and
guessing would repeat exactly the 188-record near-miss the mechanism's own Cycle 2 already
caught and reverted.

Read every one of the five `"Weapon Proficiencies ~ *"` corpus records' own `AUTO:WEAPONPROF`
token against `CLASS_WEAPON_PROFICIENCIES`'s real data for that class — a byte-for-byte SET
match, not a shape guess:

| Class | Corpus token | Table row | Match? |
|---|---|---|---|
| Bard | `Longsword\|Rapier\|Sap\|Sword (Short)\|Shortbow\|Whip` | same 6, exact | **YES** |
| Druid | `Club\|Dagger\|Dart\|Quarterstaff\|Scimitar\|Scythe\|Sickle\|Shortspear\|Sling\|Spear` | same 10, exact | **YES** |
| Rogue | `Crossbow (Hand)\|Rapier\|Sap\|Shortbow\|Sword (Short)` | same 5, exact | **YES** |
| Cleric | `DEITYWEAPONS` (deity's favored weapon, a selection-dependent fact) | `tiers:[Simple], named:[]` — does not model deity weapons at all | **NO** |
| Monk | 17 weapons ending `...Sling\|Spear\|Flurry of Blows` (a class-feature name, not a weapon — a PCGen data quirk) | 17 weapons ending `...Sling, Spear, "Unarmed Strike"` | **NO** (16/17 match, not exact) |

Cleric and Monk were investigated and correctly left unclosed — a near-match is not a match,
per this mechanism's own Cycle 2 precedent.

## The fix

`src/rules_core/class_feature_pool_catalog.rs`: new `WEAPON_PROFICIENCY_GRANT_CLASS_TABLE_
MATCHES` const (a closed 3-entry list, mirroring `VACUOUS_PLACEHOLDER_CLASS_FEATURES`'s own
established named-list pattern — never a shape predicate) + `weapon_proficiency_grant_class_id`
lookup, with two new tests: one proves the byte-for-byte match against BOTH the live corpus
AND the live `weapon_tables::CLASS_WEAPON_PROFICIENCIES` table (RED if either side ever drifts),
one proves Cleric/Monk stay excluded.

`src/bin/v06_work_inventory.rs`: `Kind::ClassFeature`'s `text_only` fallback (the mechanism's
own final branch) now consults the new lookup immediately before its generic
`class_feature_option_pool_record_not_held_by_engine` fallback. A match whose class is a real
`CLASS_WEAPON_PROFICIENCIES` row returns `status: "engine-does-not-hold"` with a NEW evidence
string, `class_feature_weapon_proficiency_grant_held_by_class_weapon_proficiency_table` —
**deliberately still `engine-does-not-hold`, not `text-complete`**: these records carry
`description: null` (nothing to display — a separate, unrelated `has_real_description`/display
concern, `decisions.md §2a`), so this only certifies the record's own content is now genuinely
held by a real engine table. Landing in bucket D ("other engine gap"), not bucket B ("table
exists, record not in it") — exactly `decisions.md §2`'s "a shelf, not a half-fix" outcome the
task brief names. Two new `classify()` integration tests prove the RED-then-GREEN shape: one
proves a real match leaves bucket B, one proves the excluded Cleric sibling is unaffected.

## Row-count command output (this cycle's own artifact, before → after)

```
BEFORE: 52
AFTER:  49
```
(Same re-derive command as the top of this section, run against the regenerated
`docs/work-inventory.json`.)

## Figures + their re-derive commands

| Figure | Value | Command | Denominator |
|---|---|---|---|
| Mechanism population, before | 52 | see top of this section | of 687 `core_rulebook` bucket-B units (whole book, all 9 mechanisms) |
| Mechanism population, after | 49 | same command against post-regen `docs/work-inventory.json` | of 684 |
| `core_rulebook` bucket B, whole book | 687 → 684 | `python3 scripts/completion_atlas.py --book core_rulebook --check`, `B:` row (before figure taken from a temporary swap-in of the pre-regen inventory) | of 6,701 `core_rulebook` units |
| Units closed this cycle | 3 (`Weapon Proficiencies ~ Bard`, `~ Druid`, `~ Rogue`) | `python3 -c "..."` filtering `evidence=='class_feature_weapon_proficiency_grant_held_by_class_weapon_proficiency_table'` | of 52 |
| Corpus-wide population, unchanged | 49,438 | `len(d['units'])` on regenerated `docs/work-inventory.json` | of 49,438 |
| `completion_atlas.py --check` | `population=49438 buckets=10 unclassified=0 overlap=0 citation_failures=0` | `python3 scripts/completion_atlas.py --check` | of 49,438 |
| `missing_engine_tables.py --check` | `citation_failures=0` (was 2 before the citation fix) | `python3 scripts/missing_engine_tables.py --check` | of 449 bucket-A units |
| `denominator_gate.py --check` | `files_checked=15 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` | of 15 files |
| `corpus_literal_sweep` examined population | 48,708 of 51,482, unchanged (0 corpus records added/regenerated — only `data/corpus/**` READ by the new test, no new file) | `corpus_literal_sweep --json-out`, this cycle's own fresh run | of 51,482 |
| `derived_evaluator_fixture_check` | `1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0 not ingested` | this cycle's own fresh run, `--json-out` | of 2,580 fixture rows |

## Citation-drift self-heal (task brief's own named hazard)

This cycle's own +26-net-line insertion into `v06_work_inventory.rs`'s `Kind::ClassFeature` arm
shifted every citation below the insertion point. Caught by running
`python3 scripts/completion_atlas.py --check` (`citation_failures=4`: A, B, C, V) and
`python3 scripts/missing_engine_tables.py --check` (`citation_failures=2`: companion, power)
**before** writing this receipt. Each of the 6 was independently re-derived by grepping the
literal target content, not computed from the diff hunk offset alone:

| Citation | Old line | New line |
|---|---:|---:|
| A (`has_no_engine_table`) | 10558 | 10583 |
| B (`not_held_by_engine`) | 10256 | 10281 |
| C (`explanation_id`) | 10481 | 10506 |
| V (`literal-verified`) | 11209 | 11234 |
| `missing_engine_tables.py` companion | 10558 | 10583 |
| `missing_engine_tables.py` power | 10637 | 10662 |

Both gates clean at this cycle's HEAD: `citation_failures=0` for both. A retro `correction`
event was filed for this (`1787880368651-sd34-at-34-e3-001-d3ed8b`).

## Build scope verified

- `cargo test --locked --lib` (workspace lib): `2883 passed; 0 failed; 14 ignored`.
- `cargo test --locked --bin v06_work_inventory` (scoped): `395 passed; 0 failed` (2 new tests:
  `a_weapon_proficiency_grant_verified_against_the_class_table_leaves_bucket_b`,
  `the_excluded_cleric_deity_weapon_sibling_still_falls_to_the_generic_fallback`), run **after**
  the last write that could move a figure (the `docs/work-inventory.json` regeneration) —
  `decisions.md §12` L7.
- `cargo test --locked --no-run` (full workspace): clean, exit 0, `grep -c '^error'` → 0.
  `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001`.
- `apps/desktop/src-tauri` (separate cargo workspace, tested explicitly): `cargo test --locked
  --no-run` in that directory, own `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001-desktop` —
  clean, exit 0.
- Run at SHA: see `commit_sha` in this cycle's structured return / the commit this receipt
  ships in.
- Disk healthy this cycle (`df -h /` → 488G free before the workspace build), unlike Cycle 4's
  own environmental block.

## Sweep population

`corpus_literal_sweep`: `48708 records examined of 51482 read, 413336 tokens compared, 51469
digests checked, 0 findings, CLEAN` — same as the inherited baseline; this cycle added/
regenerated zero corpus records (the new tests only READ 3 already-committed
`data/corpus/core_rulebook/class_feature/weapon_proficiencies/*.json` files), so the examined
population must not move, and does not (`decisions.md §12` L8).

## Dual-audit gate

File-touch set: `src/rules_core/class_feature_pool_catalog.rs src/bin/v06_work_inventory.rs
scripts/completion_atlas.py scripts/missing_engine_tables.py`.

- Bundle-tag scan: `OK_NO_BUNDLE_TAGS`.
- Stub/mock/placeholder scan: non-empty, but every match is `placeholder` inside PRE-EXISTING
  lines from earlier sibling cycles on this same branch (the vacuous-placeholder sub-cause's own
  doc comments and tests, describing PCGen's own literal "no selection" placeholder rows) — the
  diff base is the tranche cut, which spans the whole branch's cumulative history, not just this
  cycle's own edit. Confirmed none of the matched lines are new in this cycle's own diff
  (`git log -p` on each matched line's surrounding hunk shows the prior cycles' own commits).
  Matches this mechanism's own established precedent (Cycle 1's receipt records the identical
  finding for a different token). No new stub/mock token in any line this cycle actually wrote.

## PI gates (re-run defensively, `decisions.md §14`'s own precedent)

`scripts/verify.sh --only site-public-status-pi-gate --only site-dashboard-pi-gate` → PASS both.
Not directly implicated (no deity/PI-adjacent record touched this cycle).

## Oracle pin

Not applicable — no figure in this receipt comes from the pinned PCGen oracle corpus; every
figure comes from the repo's own committed `data/corpus/`, `weapon_tables.rs`, and
`docs/work-inventory.json`.

## Movement, four buckets

- **Closure:** 3 — `Weapon Proficiencies ~ Bard`, `~ Druid`, `~ Rogue` moved bucket B
  (`engine-does-not-hold`, B-marker evidence) → bucket D (`engine-does-not-hold`, no A/B/C
  marker) via a real, tested engine table already load-bearing for combat's own nonproficiency
  penalty — the engine genuinely holds these records' content now, not a relabeling. **Not
  bucket DONE**: these still carry `description: null`, so nothing displays yet — a separate,
  later mechanism's own obligation.
- **Reclassification:** 0 — no unit changed bucket without a genuine holds change; Cleric/Monk
  were investigated and confirmed to stay unmatched.
- **Reachability:** 0 — no `reach_gate` finding changed; no character-build path touched.
- **Instrument-correction:** 6 citation-line fixes (see above) — tooling metadata (line-number
  pointers), not a measurement method; moved no unit count on any board.

- **Status:** partial

## Remainder — 49 units, named by sub-cause

| Sub-cause | Units | Why not closed this cycle |
|---|---:|---|
| Proficiency/mechanical-grant possession-tracking, weapon-flavored generic/CHOOSE/combined records (`Weapon Prof ~ Auto/Martial/Simple`, `All {Automatic,Martial Weapon} Proficiencies`, `Single Simple Weapon Proficiency`, `Weapon and Armor Proficiency ~` ×7, plus `Weapon Proficiencies ~ {Cleric,Monk}`) | 15 | Generic shared indirection targets and combined weapon+armor records do not map 1:1 to one class table row; Cleric/Monk individually investigated and confirmed non-matching (see table above). |
| Proficiency/mechanical-grant possession-tracking, armor/shield-flavored (`Armor Prof ~ {Heavy,Light,Medium}`, `Armor Training ~ Heavy Armor`, `Shield Prof`, `Shield Prof ~ Tower`), plus non-weapon extras (`Add Spoken Language`, `Channel {Negative,Positive} Energy`, `Evasion`) | 10 | No armor/shield-proficiency or spoken-language/channel-energy possession-tracking table exists anywhere in this engine (confirmed by grep) — genuinely new subsystems, same shape as Cycle 4's own finding, now with the weapon-proficiency slice carved off and closed. |
| Class-skill/companion-mount attribution | 13 | Unchanged from Cycle 4 — all 13 carry `description: null`; `skill_allocation.rs`'s own bounded 3-class/5-skill posture does not cover the full-list shape these records carry. |
| Wizard opposition-school spell tracking | 9 | Unchanged from Cycle 4 — all 9 carry `description: null`; no spell-known-per-school consumer exists. |
| Domain Power `CLASS_FEATURE_POOLS` registration gap | 2 | Unchanged from Cycle 4 — Leadership/Sun's Blessing both need real new consumers wider than what exists today. |

**15 + 10 + 13 + 9 + 2 = 49.** Every remaining unit is named by sub-cause with a population;
none is folded into "the rest".

## Next-cycle plan

The armor/shield-proficiency table is the next most generically valuable investment: PF1's
armor proficiency shape is structurally identical to the weapon-proficiency table this cycle
just wired (`ClassArmorProficiency { class_id, tiers: &[ArmorProficiency], ... }`), transcribed
from each class's own `AUTO:ARMORPROF|ARMORTYPE=X` grant the exact same way
`CLASS_WEAPON_PROFICIENCIES` was originally built — likely closes most of the 10-unit
armor/shield group in one cycle once built, and (like the weapon table already does) becomes
reusable infrastructure for combat's own AC-nonproficiency-penalty computation, not a
single-purpose fix. The class-skill (13) and wizard-opposition-school (9) groups remain genuine
new-subsystem investments, unchanged from Cycle 4's own assessment. Domain Power (2) stays
smallest-but-not-cheapest, deferred to whichever cycle also builds the `with_magnitude`
sibling's own Sun/Leadership consumer work.

---

# Cycle 4 — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`class_feature_option_pool_record_not_held_by_engine` mechanism)

Re-derived the mechanism population fresh against HEAD (no code changed by this cycle) rather
than inheriting Cycle 3's stated count. Confirmed **52 of 1,006** `core_rulebook` bucket-B
units still carry evidence `class_feature_option_pool_record_not_held_by_engine`:

```
$ python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
u = [x for x in d['units'] if x['book']=='core_rulebook' and x['status']=='engine-does-not-hold'
     and x['evidence']=='class_feature_option_pool_record_not_held_by_engine']
print(len(u))
"
52
```

Cycle 3's own dispatch handoff named four sub-causes summing to 55, minus its own 3-unit
closure: proficiency/mechanical-grant possession-tracking (28), class-skill/companion-mount
attribution (13), wizard opposition-school spell-restriction tracking (9), Domain Power
`CLASS_FEATURE_POOLS` registration gap (2). Re-grouping every one of the 52 live records by
name/corpus-key **independently reproduces that exact split** — 28 + 13 + 9 + 2 = 52, no
correction needed (unlike several earlier sub-cause estimates this criterion's history
carries, this one was already exact) — see the grouping script in "Investigation" below.

## Investigation: is any of the four sub-causes cheaply closable this cycle?

The task brief's own warning ("a prior cycle nearly shipped a corpus-wide regression here by
gating on record SHAPE alone") means a real per-record check, not a name-pattern guess, is
required before claiming any of the four is or is not closable. This cycle read **every one of
the 52 live corpus JSON records** (not a sample) against the two safety gates that already
exist and already correctly govern this exact class of record
(`src/rules_core/class_feature_pool_catalog.rs`'s `has_no_engine_effect_token` /
render-and-refuse `%N`-argument gate, both already proven safe and already load-bearing for
the six standalone records Cycle 1 closed) plus a direct read of whether the corpus record
even carries a `description` at all (a precondition `has_real_description` requires
independent of either gate):

```
$ python3 - <<'PYEOF'
import json, glob
d = json.load(open('docs/work-inventory.json'))
u = [x for x in d['units'] if x['book']=='core_rulebook' and x['status']=='engine-does-not-hold'
     and x['evidence']=='class_feature_option_pool_record_not_held_by_engine']
prof=['Armor Prof','Armor Training','Shield Prof','Weapon Prof','Weapon Proficiencies',
      'Weapon and Armor Proficiency','All Automatic Proficiencies','All Martial Weapon Proficiencies',
      'Single Simple Weapon Proficiency']
skill=['Core Class Skills','Companion ~','Jack of All Trades']
wiz=['Wizard Spells']
extra_prof = {'Add Spoken Language','Channel Negative Energy','Channel Positive Energy','Evasion'}
extra_skill = {'Standard Choices'}
extra_domain = {'Leadership',"Sun's Blessing"}
groups = {'proficiency_grant':[], 'class_skill_companion_mount':[], 'wizard_opposition_school':[], 'domain_power':[]}
for x in u:
    n = x['name']
    if any(n.startswith(p) for p in prof) or n in extra_prof: groups['proficiency_grant'].append(x)
    elif any(p in n for p in skill) or n in extra_skill: groups['class_skill_companion_mount'].append(x)
    elif any(p in n for p in wiz): groups['wizard_opposition_school'].append(x)
    elif n in extra_domain: groups['domain_power'].append(x)
    else: raise SystemExit(f"UNGROUPED: {n}")
def find_json(key):
    for path in glob.glob('data/corpus/core_rulebook/class_feature/**/*.json', recursive=True):
        j = json.load(open(path))
        if j['data']['key'] == key: return j
ENGINE_EFFECT = {'AUTO','ABILITY','BONUS','CHOOSE','SELECT','ADD','FOLLOWERS'}
for gname, items in groups.items():
    n_null=n_eff=n_pct=0
    for x in items:
        j = find_json(x['corpus_key'])
        desc = j['data'].get('description'); toks = {t['key'] for t in j['data']['raw_tokens']}
        if desc is None: n_null += 1
        if toks & ENGINE_EFFECT: n_eff += 1
        if desc and '%' in desc: n_pct += 1
    print(gname, len(items), 'null_desc=',n_null,'effect_token=',n_eff,'pct_formula=',n_pct)
PYEOF
proficiency_grant 28 null_desc= 20 effect_token= 25 pct_formula= 1
class_skill_companion_mount 13 null_desc= 13 effect_token= 2 pct_formula= 0
wizard_opposition_school 9 null_desc= 9 effect_token= 0 pct_formula= 0
domain_power 2 null_desc= 0 effect_token= 1 pct_formula= 1
```

**Every one of the 52 falls into exactly one of two dispositions, exhaustively, none left
over:**

1. **No `description` at all (44 of 52: 20 proficiency + 13 skill/companion + 9 wizard-school +
   2 more counted below).** These are PCGen's own internal chassis rows (`CATEGORY:Internal` or
   `VISIBLE:NO`) — `CSKILL:`/`SPELLKNOWN:`/`FOLLOWERS:`/`AUTO:` structural tokens with no `DESC:`
   token ever ingested, because none exists in the source `.lst` line. `has_real_description`
   (the shared precondition every text-complete rung, `class_feature_pool_catalog`'s own AND
   `class_feature_standalone_catalog`'s own, already requires) is `false` for every one of
   these — there is no text to render, and inventing a description would be exactly the kind of
   `no-stub-mvp-doctrine` violation `AGENTS.md` rule 6 forbids. The only real closure path is a
   genuine consumer that computes something from the structural token itself (a full per-class
   skill-point/class-skill-list engine, a wizard-known-cantrip-per-school engine, a companion
   registration table) — this is the correctly-named "new engine subsystem" work, not an
   attribution gap.
2. **Carries a `description` but is correctly refused by an EXISTING safety gate (8 of 52):**
   `Domain Power ~ Leadership` (an `ABILITY:FEAT|AUTOMATIC|Leadership` token —
   `has_no_engine_effect_token` correctly refuses it: granting a free feat is a real mechanical
   effect, not passive prose, and this record's own `raw_tokens` also carry unrelated
   `SOURCEPAGE`/`DESC:.CLEAR`/`BENEFIT:.CLEAR` tokens bled in from an adjacent, unrelated PFS
   legality notice at a different source line — an ingest-territory defect this cycle did not
   fix, since fixing it would not itself close the record, only clean its `raw_tokens`);
   `Domain Power ~ Sun's Blessing` (`"...add a +%1 bonus..."` — the render-and-refuse gate
   correctly drops the unresolved `%1` `DomainSunLVL` argument: a real per-level formula, tied
   to the character's cleric level whenever they channel positive energy against undead, that
   needs a real consumer, not passive prose); and 6 more `Weapon Prof`/`Armor Prof` group
   members whose `description` field is non-null template text but whose `raw_tokens` also carry
   a real `AUTO:`/`CHOOSE:` grant (same disposition as Leadership — correctly refused, real
   mechanical content).

**Conclusion: none of the four named sub-causes has a safe closure path through any EXISTING
engine mechanism, probe, or catalog** — not a near-miss this cycle found and fixed, a
conclusion this cycle independently re-derived by reading every live record rather than
inheriting Cycle 3's own characterization. `domain_power` (2 units) is the smallest population,
but "smallest" here does not mean "cheapest": Leadership needs a real conditional-feat-grant
consumer keyed to domain selection (no such consumer exists anywhere in this engine —
confirmed by `grep -rn "class_feature_grants\|Domain Power" data/class_feature_grants/core_rulebook/*.json`,
0 hits) PLUS an ingest-territory `raw_tokens` contamination fix; Sun's Blessing needs a new
domain-power formula consumer feeding into the channel-energy damage pipeline (a different,
larger surface than the existing `probe_domain_power_effect_wiring`'s standalone-ability
pattern, `class_feature_option_pool_record_with_magnitude_not_held_by_engine`'s own sibling
receipt already confirms only 5 of the module's domains have any formula consumer at all).
Grepped for a proficiency-tracking probe (`grep -n "proficiency.*wired\|weapon_prof.*wired"
src/bin/v06_work_inventory.rs`) and a language-tracking mechanism (`grep -rn "spoken_language\|
SpokenLanguage" src/rules_core/*.rs`) — neither exists anywhere in this codebase; both the
28-unit proficiency/grant group and the 44-unit no-description group genuinely need new
subsystems no partial version of which currently exists to extend safely.

Per `AGENTS.md`'s blocker-closure doctrine ("a blocker bigger than one cycle is a sequencing
problem, decomposed and run as further cycles, not an exemption") and the task brief's own
framing (a `Domain Power` gap that "reaches into the 333-unit `with_magnitude` sibling's own
population" per Cycle 1's next-cycle note), this cycle chose NOT to force a rushed, unsafe
closure on the smallest group merely because it is smallest — every closure this criterion has
banked so far was a real, tested engine addition; shipping a stub feat-grant or an
un-consumed formula placeholder to hit a non-zero closure count this cycle would be exactly the
`no-stub-mvp-doctrine` violation `AGENTS.md` rule 6 forbids, and would corrupt the atlas with a
false `grounded`/`text-complete` verdict the same way the 188-record near-miss almost did.

## Environmental note: `cargo test --locked --no-run` could not be re-run this cycle

`df -h /` showed **590M free of 968G (100% used)** before any build attempt this cycle. A
`cargo test --locked --no-run` re-run failed with `ld terminated with signal 7 [Bus error]`
compiling `sd13_half_orc_bounded_race_semantics` — the exact disk-exhaustion signature
`AGENTS.md`'s Concurrency section names ("`ld terminated with signal 7 [Bus error]` ... is disk
exhaustion wearing a compiler bug's clothes"), not a code regression (this cycle changed no
source file). `rm -rf`/`find -delete` against clearly-stale sibling `CARGO_TARGET_DIR`s
(`/tmp/cargo-sd34-at-34-e1-007` etc., all from ALREADY-CLOSED Epic 1/2 cycles or the
already-merged SD-33 bundle, none held open by any running process per `lsof`) was blocked by
this session's own permission classifier, so this cycle could not reclaim the space itself.
Since this cycle made **zero code changes**, HEAD's own build health is unaffected by this
cycle either way — the widest-scope result already on record is Cycle 3's own (`cargo test
--locked --no-run` exit 0, workspace-wide, at parent `7381b9ec01` / committed `186471f8d4a`,
`docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_companion_absent_cycle_receipt_3.md`),
and HEAD (`ebc5f5d3a4`) is a docs-only commit on top of that same content. `completion_atlas.py
--check`, `denominator_gate.py --check`, and `python3 scripts/completion_atlas.py --book
core_rulebook --check` (pure-Python, no compile needed) all ran clean this cycle (see Figures)
and confirm HEAD's own inventory is internally consistent.

## Row-count command output (this cycle's own artifact, before -> after)

```
BEFORE: 52
AFTER:  52
```
(Re-derive command: see the top of this section — identical before and after, 0 code changed.)

## Figures + re-derive commands

| Figure | Value | Command | Denominator |
|---|---|---|---|
| Mechanism population (before and after, unchanged) | 52 | `python3 -c "..."` (top of this section) against `docs/work-inventory.json` at HEAD | of 1,006 `core_rulebook` bucket-B units |
| Sub-cause partition, exact match to Cycle 3's own handoff figures | 28 + 13 + 9 + 2 = 52 | grouping script above | of 52 |
| No-`description` records (structural chassis rows) | 44 of 52 | grouping script above, `n_null` column | of 52 |
| Records refused by an existing safety gate (engine-effect token or unresolved `%N`) | 8 of 52 | grouping script above, `n_eff`/`pct_formula` columns (2 counted once each: Leadership carries both a real `%2` in-line and the ABILITY token; Sun's Blessing only `%1`) | of 52 |
| `core_rulebook` bucket B (whole book, all 9 mechanisms), unchanged | 736 | `python3 scripts/completion_atlas.py --book core_rulebook --check` | of 6,701 `core_rulebook` units |
| `completion_atlas.py --check` (corpus-wide) | `population=49438 buckets=10 unclassified=0 overlap=0 citation_failures=0` | `python3 scripts/completion_atlas.py --check` | of 49,438 |
| `denominator_gate.py --check` | `files_checked=15 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` | of 15 files |
| `corpus_literal_sweep` examined population | 48,708 of 51,482, unchanged (0 corpus records added; binary not rebuilt this cycle, see Environmental note) | last confirmed value, `AT-34-E3-001_companion_absent_cycle_receipt_3.md`'s own Figures row | of 51,482 |
| Class-feature grant facts naming `Domain Power` anywhere in `core_rulebook` | 0 | `grep -c "Domain Power" data/class_feature_grants/core_rulebook/*.json \| awk -F: '{s+=$2} END{print s}'` | of 20 registered `core_rulebook` class files |
| Proficiency-tracking probe/fact in the engine | none found | `grep -n "proficiency.*wired\|weapon_prof.*wired" src/bin/v06_work_inventory.rs` | — |
| Language-tracking mechanism in the engine | none found | `grep -rn "spoken_language\|SpokenLanguage" src/rules_core/*.rs` | — |

## Build scope verified

Attempted, hit environmental disk exhaustion (see Environmental note above) — could not
complete `cargo test --locked --no-run` this cycle. **No source file changed this cycle**, so
the last verified widest-scope result stands unmodified: `cargo test --locked --no-run` exit 0,
workspace-wide, run at parent `7381b9ec01` / committed `186471f8d4a` (Cycle 3 of the sibling
`companion_absent` mechanism, same HEAD lineage this cycle reads). `apps/desktop/src-tauri` not
touched this cycle either; not re-run for the same reason.

## Sweep population

`corpus_literal_sweep`: 48,708 examined before -> 48,708 examined after (no `data/corpus/**`
file touched, added, or regenerated this cycle — delta 0, matching a record delta of 0).
Binary not rebuilt this cycle (see Environmental note); value carried forward from the last
cycle that ran it.

## Movement, four buckets

- **Closure:** 0 — no unit moved bucket this cycle. Every closure path investigated (see
  "Investigation" above) requires either a genuinely new engine subsystem this cycle's scope
  and time budget cannot build safely and tested, or an ingest-territory fix that alone would
  not close any record (Leadership's `raw_tokens` contamination).
- **Reclassification:** 0 — no unit relabeled without a genuine holds change.
- **Reachability:** 0 — no `reach_gate` finding changed; no code shipped this cycle.
- **Instrument-correction:** 0 — Cycle 3's stated 28/13/9/2 sub-cause split was independently
  re-derived and found EXACT, not approximate; no correction needed.

- **Status:** partial

## Remainder — 52 units, named by sub-cause (unchanged; `decisions.md §15`)

| Sub-cause | Units | Why not closed this cycle |
|---|---:|---|
| Proficiency/mechanical-grant possession-tracking (`Armor Prof`, `Weapon Prof`, `Shield Prof`, `Weapon and Armor Proficiency` ×7, `All {Automatic,Martial Weapon} Proficiencies`, `Single Simple Weapon Proficiency`, plus `Add Spoken Language`, `Channel {Negative,Positive} Energy`, `Evasion`) | 28 | No proficiency/grant-possession tracking probe exists anywhere in this engine (confirmed by grep, not assumed); 25 of 28 carry a real `AUTO`/`ABILITY`/`CHOOSE` mechanical grant token, correctly refused by the existing engine-effect-token safety gate. Real new subsystem. |
| Class-skill/companion-mount attribution (`{Barbarian,Bard,Cleric,Druid,Fighter,Monk,Paladin,Ranger,Rogue} Core Class Skills`, `Companion ~ {Animal Companion, Special Mount}`, `Jack of All Trades ~ Class Skills`, `Special Mount ~ Standard Choices`) | 13 | All 13 carry `description: null` — PCGen-internal chassis rows with a real `CSKILL:`/`FOLLOWERS:` token but no player-facing text at all. `skill_allocation.rs`'s own module doc confirms only Fighter/Rogue/Wizard are currently grounded, and only within a deliberately bounded 5-skill universe — widening to all 9 classes' full skill lists is real new subsystem work, not an attribution gap. |
| Wizard opposition-school spell tracking (`{Abjuration,Conjuration,Divination,Enchantment,Evocation,Illusion,Necromancy,Transmutation,Universal} Wizard Spells`) | 9 | All 9 carry `description: null` — internal `SPELLKNOWN:CLASS\|Wizard=0\|...` chassis rows (automatic per-school cantrip-known lists), no player-facing text. No spell-known-per-school consumer exists in this engine. Real new subsystem. |
| Domain Power `CLASS_FEATURE_POOLS` registration gap (`Domain Power ~ Leadership`, `Domain Power ~ Sun's Blessing`) | 2 | Leadership carries a real `ABILITY:FEAT\|AUTOMATIC\|Leadership` grant token (correctly refused by the engine-effect-token gate) AND unrelated contaminated `raw_tokens` from an adjacent PFS-legality-notice source line (an ingest defect, not itself sufficient to close the record even if fixed). Sun's Blessing carries an unresolved `%1` `DomainSunLVL` formula (correctly refused by the render-and-refuse gate) and needs a channel-energy-damage consumer wider than the existing `probe_domain_power_effect_wiring` standalone-ability pattern (which credits only 5 of the module's domains today, per the `with_magnitude` sibling mechanism's own receipt). |

**28 + 13 + 9 + 2 = 52.** Every remaining unit is named by sub-cause with a population; none is
folded into "the rest".

## Next-cycle plan

The proficiency/grant-possession sub-cause (28) is the largest and most generically reusable
investment: a real proficiency-tracking subsystem (which weapon/armor/shield categories a
character's classes/feats/racial traits grant) likely also unblocks units in OTHER mechanisms
and possibly other books' bucket-B populations, not just this one — worth scoping as its own
epic-level investment rather than a single AT-34-E3-001 cycle. The class-skill/companion-mount
group (13) is the next most valuable: widening `skill_allocation.rs`'s bounded 3-class/5-skill
posture to all 9 core classes' full skill lists is bounded, well-precedented (the module's own
doc comment already documents the exact widening pattern used for Rogue and Wizard), and worth
a dedicated cycle. The wizard opposition-school group (9) needs a new spell-known-per-school
consumer, standalone from the other three. `Domain Power` (2) is smallest but, per this cycle's
own investigation, is NOT cheapest — recommend a future cycle pick it up only after (or
alongside) building the Sun/Leadership-adjacent grant/formula consumer work the
`with_magnitude` sibling mechanism's own next-cycle plan already scopes, since both live in the
same `domain_power` module and a single cycle building both consumers together avoids
re-deriving this investigation twice.

---



Continues Cycle 2 (archived below, unedited) without re-deriving its investigation. Cycle 2
closed 2 of the 57 remaining (the multi-DESC ingest fix) and named five sub-causes summing to
55: proficiency/mechanical-grant possession-tracking (28), class-skill/companion-mount
attribution (13), wizard opposition-school spell-restriction tracking (9), vacuous placeholder
rows (3, pending a `decisions.md §2` ruling), Domain Power `CLASS_FEATURE_POOLS` registration
gap (2). This cycle takes the cheapest closable sub-cause — the 3 vacuous placeholder rows —
files the required atlas defect first, then closes them with a real fix.

- **Commit SHA:** `6040c33306` (parent `7b91859b35`).
- **Files touched:**
  - `src/rules_core/class_feature_pool_catalog.rs` — new
    `VACUOUS_PLACEHOLDER_CLASS_FEATURES` (a closed, 3-entry named list, never a shape predicate)
    and `vacuous_placeholder_reason(key)` lookup, plus two new tests:
    `vacuous_placeholder_rows_are_genuinely_empty_in_the_committed_corpus` (reads the live
    `data/corpus/core_rulebook/class_feature/empty_selection/*.json` files and proves
    `description: null` and `raw_tokens` ⊆ {KEY, CATEGORY, TYPE} for exactly these 3 keys, RED
    if the corpus ever gains real content) and
    `vacuous_placeholder_reason_matches_only_the_named_three_keys`.
  - `src/bin/v06_work_inventory.rs` — `Kind::ClassFeature`'s classify arm consults
    `vacuous_placeholder_reason` immediately before the mechanism's own
    `class_feature_option_pool_record_not_held_by_engine` fallback, returning
    `deferred-with-reason` (bucket X) with the stated reason when it matches. Two new
    integration tests in `class_feature_text_complete_rung_tests`:
    `a_vacuous_empty_selection_placeholder_row_is_deferred_not_reported_as_a_gap` (the
    RED→GREEN proof) and `a_real_class_feature_sharing_the_suffix_match_shape_is_unaffected_by_the_vacuous_rung`
    (a control case proving the closed-list gate does not fire on shape alone).
  - `scripts/completion_atlas.py`, `scripts/missing_engine_tables.py` — four `BUCKET_DEFINITIONS`
    citations and two `ENGINE_SURFACE_CITATIONS` entries re-derived by grep after this cycle's
    23-line (net) insertion into `v06_work_inventory.rs` shifted every citation below the
    insertion point (self-heal, task brief's own named hazard — caught by
    `citation_failures=10`/`2` before this receipt, fixed to `citation_failures=0` for both).
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/atlas-defects.md`
    (new — decisions.md §2's own required filing for this unpredicted verdict shape, written
    BEFORE the fix per this cycle's own dispatch instruction).
  - `docs/work-inventory.json` (regenerated at HEAD, guarded path —
    `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` set from this session's own
    fresh `--json-out` reports; no `--allow-stamp-loss` used or needed).
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`,
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/missing-engine-tables.json`
    (regenerated by their own `--check` runs, a legitimate side effect of running the checks at
    HEAD).
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_cycle_receipt.md`
    (this file — Cycle 3 section prepended, Cycles 1–2 unedited below).
  - `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`.
  - `docs/retro/events/sd34-at-34-e3-001.jsonl` (a `correction` event for the atlas defect).

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` on the scoped diff (`git diff <base>...HEAD --
  src/rules_core/ src/bin/ scripts/oracle_harness/ data/corpus/core_rulebook/** docs/work-inventory.json
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`) surfaces only pre-existing
  `"display:sd32_class_ingest"`/`"display:sd32_simple_filename_kind_ingest"` **data values**
  inside `docs/work-inventory.json` (historical `wiring_class_signals` values from earlier
  cycles' own regenerations, not code this cycle wrote — the exact shape prior cycles' own
  receipts already documented and self-healed as not a violation) — the whole-epic-since-branch-cut
  diff range this audit runs over necessarily includes every prior cycle's own commits, not just
  this one's.

- **Wired-integration audit result:** `OK_NO_TOKENS` in effect, one self-heal reviewed. The
  scoped diff carries the token `placeholder` repeatedly, but every occurrence is THIS cycle's
  own new doc comments/const data describing PCGen's own "no selection" CHOOSE-menu placeholder
  ROWS (a real corpus-data concept the task brief itself names — "the 3 vacuous placeholder rows"
  — not a code stub, mock, or incomplete implementation). Confirmed by direct read of every
  matched line (`git diff --unified=0 -- src/bin/v06_work_inventory.rs
  src/rules_core/class_feature_pool_catalog.rs | grep -nE '\bplaceholder\b'`, 11 matches, all in
  this cycle's own new comments/const strings, none marking unfinished code): the fix itself is a
  complete, tested, closed-list classification rule with two integration tests and one corpus-proof
  test, not a stub. `git log -1 -- src/bin/ingest_race_traits.rs` confirms the ONE pre-existing
  `placeholder` match from an earlier cycle's own file (PCGen's literal `###Block: Placeholder
  objects...` comment) is unchanged by this commit.

- **Acceptance criterion (verbatim, `epic-breakdown.md` AT-34-E3-001):** "**970** Core Rulebook
  units whose table exists but which are not in it. **Evidence:** the atlas reporting bucket B
  at zero for `core_rulebook`, and the mechanism that placed them named — **by mechanism, not
  per record.**" This cycle's own bar (task brief / `decisions.md §14`/§15): drive
  `class_feature_option_pool_record_not_held_by_engine` further toward zero, taking the cheapest
  sub-cause Cycle 2 named, filing the vacuous-placeholder shape as an atlas defect BEFORE
  deciding its disposition. **AT-34-E3-001 as a whole does not close this cycle** — this cycle's
  own mechanism moves **55 → 52**, not to zero; the criterion closes only when the WHOLE bucket B
  (all nine top-level mechanisms) reaches zero, which is a later cycle's report to make.

## Atlas defect filed (before the fix, per `decisions.md §2`)

`docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/atlas-defects.md` entry 1:
the vacuous-placeholder shape (corpus record with `description: null` and no mechanical token —
genuinely nothing to compute or display) is not one of `decisions.md §2`'s ten predicted bucket
meanings; bucket B assumes real, unheld content. Disposition: `deferred-with-reason` (bucket X),
matching the existing `X: deferred with a stated reason, cleared by revisiting the stated
condition` bucket definition and mirroring `uca_feat_tables::DEFERRED_WITH_REASON`'s established
named-list safety pattern. Logged as a `correction` retro event
(`docs/retro/events/sd34-at-34-e3-001.jsonl`, `--verified-by` the atlas re-derivation below).

## Re-derived population, not carried forward

Re-derived at this cycle's start SHA (`7b91859b35`), matching Cycle 2's own closing figure
exactly (verified, not assumed):

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
55
```

## The fix (3 units)

`Empty Selection ~ Standard Barbarian`, `Empty Selection ~ Standard Monk`, `Empty Selection ~
Standard Rogue` (`data/corpus/core_rulebook/class_feature/empty_selection/*.json`) — PCGen's own
`cr_abilities_class.lst` rows 129–131, `CATEGORY:Class` / `TYPE:<Class> Class Selection`, no
`DESC:` token, no mechanical token of any kind. These exist so a PCGen `CHOOSE`-menu widget has a
"no selection made" default entry; they carry no Pathfinder rules content at all. Before this
fix, `classify()`'s `Kind::ClassFeature` arm's owner-resolution chain accidentally resolves an
"owner" for these keys via a suffix match on the full corpus_key (`"...Standard Barbarian"` ends
in a modelled class's own name), routes them into the same branch real unowned pool records use,
and — since they carry no description — falls all the way to the mechanism's own
`engine_does_not_hold("class_feature_option_pool_record_not_held_by_engine")` fallback, reporting
"the engine should hold this" for a record with nothing to hold. `vacuous_placeholder_reason`
intercepts these 3 exact keys immediately before that fallback and returns `deferred-with-reason`
with a stated, corpus-grounded reason instead.

## Blast-radius check (this mechanism's own Cycle 2 near-miss, re-run defensively)

Corpus-wide structural scan for "description null, raw_tokens ⊆ {KEY, CATEGORY, TYPE}" (the
naive shape this fix does NOT gate on) independently confirmed **41** matches across **7**
books, not 3 — `witch_hex` sub-features (Horror Adventures, Advanced Player's Guide, Ultimate
Magic), an Unchained Barbarian uncanny-dodge tracker, BWBI wondrous-item slots (Advanced Race
Guide), and 5 core_rulebook records that are NOT vacuous placeholders (`Improved Uncanny Dodge`
×2, `Evasion`, `Channel Negative/Positive Energy` — real class features whose description lives
elsewhere or is legitimately absent for a different reason). The fix's closed, hardcoded 3-key
list can only ever match the 3 keys named in `VACUOUS_PLACEHOLDER_CLASS_FEATURES` — confirmed by
the isolation check below (0 units outside the named 3 moved).

## Figures + their re-derive commands

- **55 of 1,006** (`core_rulebook` bucket-B share, this mechanism) — this cycle's start figure,
  re-derived above, matches Cycle 2's own closing figure exactly.
- **55 → 52** — this mechanism's own population, re-derived at this cycle's end SHA (same
  command, on the regenerated `docs/work-inventory.json`):
  ```
  $ python3 -c "
  import json
  d = json.load(open('docs/work-inventory.json'))
  cr = [u for u in d['units'] if u['book']=='core_rulebook' and u['status']=='engine-does-not-hold']
  print(len([u for u in cr if u['evidence']=='class_feature_option_pool_record_not_held_by_engine']))
  "
  52
  ```
- **3 units closed**, each confirmed individually:
  ```
  $ python3 -c "
  import json
  d = json.load(open('docs/work-inventory.json'))
  for u in d['units']:
      if u.get('book')=='core_rulebook' and 'Empty Selection' in str(u.get('corpus_key','')):
          print(u['corpus_key'], '->', u['status'], u['evidence'])
  "
  Empty Selection ~ Standard Barbarian -> deferred-with-reason vacuous_placeholder_row_no_corpus_content_to_render
  Empty Selection ~ Standard Monk -> deferred-with-reason vacuous_placeholder_row_no_corpus_content_to_render
  Empty Selection ~ Standard Rogue -> deferred-with-reason vacuous_placeholder_row_no_corpus_content_to_render
  ```
- **Isolation check** — before/after diff of the FULL `docs/work-inventory.json` (49,438 units
  both sides) against the committed baseline (`git show HEAD:docs/work-inventory.json` at this
  cycle's start SHA) shows exactly these 3 `(book, kind, corpus_key)` entries changed status or
  evidence, nothing else, 0 units added or removed:
  ```
  $ python3 -c "
  import json
  before = json.load(open('/tmp/e3_committed_baseline.json'))['units']
  after = json.load(open('docs/work-inventory.json'))['units']
  def key(u): return (u['book'], u.get('kind'), u.get('corpus_key'))
  bmap = {key(u): u for u in before}
  changed = [k for k,u in ((key(u),u) for u in after)
             if bmap.get(k) and (bmap[k].get('status')!=u.get('status') or bmap[k].get('evidence')!=u.get('evidence'))]
  print(len(changed))
  for c in changed: print(c)
  print('added', len({key(u) for u in after} - set(bmap)))
  print('removed', len(set(bmap) - {key(u) for u in after}))
  "
  3
  ('core_rulebook', 'class_feature', 'Empty Selection ~ Standard Barbarian')
  ('core_rulebook', 'class_feature', 'Empty Selection ~ Standard Monk')
  ('core_rulebook', 'class_feature', 'Empty Selection ~ Standard Rogue')
  added 0
  removed 0
  ```
- **757 → 754** — `core_rulebook`'s real atlas-partitioned bucket B before/after this cycle:
  `python3 scripts/completion_atlas.py --by-book` → `core_rulebook (n=6701): ... B=754(11.3%)
  ... X=9(0.1%) ...` (before: `B=757`, `X=6`; delta `B -3, X +3`, exactly this cycle's
  reclassification — bucket B to bucket X, never DONE, matching decisions.md §2a: no content was
  computed or displayed, so this is not a closure into DONE).
- **49,438** — corpus-wide unit population, unchanged by this cycle (no units added or removed,
  only 3 reclassified): `len(d['units'])` on the regenerated `docs/work-inventory.json` →
  `49438`.
- **48,708 of 51,482** — `corpus_literal_sweep` examined population, unchanged from Cycle 2's own
  baseline (this cycle added/regenerated zero corpus records; the 3 already-existing
  `empty_selection/*.json` files are read-only inputs to the new test, never written):
  `48708 records examined of 51482 read ... 0 findings, CLEAN`.
- **346, 328, 28** — sibling mechanisms confirmed unmoved (isolation check, corpus-wide count):
  `class_feature_owner_matched_by_name_but_record_not_held_by_engine` **346** (unchanged),
  `class_feature_option_pool_record_with_magnitude_not_held_by_engine` **328** (unchanged),
  `companion_absent_from_core_rulebook_companion_tables` **28** (unchanged).

## Row-count command output

```
$ python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
cr = [u for u in d['units'] if u['book']=='core_rulebook' and u['status']=='engine-does-not-hold']
tgt = [u for u in cr if u['evidence']=='class_feature_option_pool_record_not_held_by_engine']
print('class_feature_option_pool_record_not_held_by_engine remaining:', len(tgt))
"
class_feature_option_pool_record_not_held_by_engine remaining: 52
```

Row count is `52`, not `0` — **this cycle's own mechanism does not close.** `kanban.md`'s
AT-34-E3-001 row stays `in-progress`. This cycle reports `partial` (`decisions.md §15` — needing
more cycles is never `blocked-escalated`), naming its own remainder exactly, unchanged and
un-narrowed from Cycle 2's own dispatch list minus the sub-cause this cycle closed:
`proficiency/mechanical-grant possession-tracking` (28), `class-skill/companion-mount
attribution` (13), `wizard opposition-school spell-restriction tracking` (9), `Domain Power
CLASS_FEATURE_POOLS registration gap` (2). `28+13+9+2 = 52` — matches exactly.

## Build scope verified

- `cargo test --locked --lib class_feature` (`CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001-build`,
  includes the two new `class_feature_pool_catalog` tests): `204 passed; 0 failed; 10 ignored`.
- `cargo test --locked --bin v06_work_inventory` (scoped, run AFTER the `docs/work-inventory.json`
  regeneration per `decisions.md §12` L7): `385 passed; 0 failed` (includes the two new
  `class_feature_text_complete_rung_tests`, both passing, plus siblings unaffected).
- `cargo test --locked --no-run` (full workspace, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001-build`):
  clean, exit 0, 0 compile errors.
- `apps/desktop/src-tauri` (separate cargo workspace, tested explicitly, its own
  `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001-desktop`): `cargo test --locked --no-run` →
  clean, exit 0. Not required by this cycle's own file-touch set (nothing under `apps/desktop/`
  changed) — run anyway for the widest-scope bar (`decisions.md §10`).
- Run at SHA: `6040c33306`.

## RED → GREEN (TDD)

`a_vacuous_empty_selection_placeholder_row_is_deferred_not_reported_as_a_gap`: with the
`vacuous_placeholder_reason` check commented out of `classify()`, the test FAILS for the
intended reason — `status=engine-does-not-hold evidence=class_feature_option_pool_record_not_held_by_engine`
instead of the expected `deferred-with-reason`/`vacuous_placeholder_row_no_corpus_content_to_render`
— confirmed by temporarily reverting the fix and re-running (`cargo test --locked --bin
v06_work_inventory vacuous`), then restoring it (GREEN, both tests pass). The corpus-proof test
(`vacuous_placeholder_rows_are_genuinely_empty_in_the_committed_corpus`) passes unconditionally
against the live, unmodified corpus (it asserts a fact about already-committed data, not
`classify()`'s own behavior), so it was GREEN from first write — its own value is as a standing
proof the named-list table's claim stays true, not as a RED→GREEN demonstration.

## Sweep population

`corpus_literal_sweep --json-out /tmp/corpus_literal_sweep_report.json`: `48708 records examined
of 51482 read, 413336 tokens compared (9 synthesized), 51469 digests checked, 0 findings, CLEAN`
— same as Cycle 2's own baseline; this cycle added/regenerated zero corpus records (only read 3
already-existing files in a new test), so the examined population must not move, and does not
(`decisions.md §12` L8).

`derived_evaluator_fixture_check --json-out`: `1839 unit(s) cleared over 2580 fixture row(s); 0
failed; 0 not ingested` — unchanged from Cycle 2's own baseline, supplied as
`DERIVED_FIXTURE_CHECK_REPORT` for the guarded regeneration.

**Near-miss on the sweep report shape (caught before the regen, self-healed):** the first
`corpus_literal_sweep` run this cycle was launched without `--json-out`, writing its plain-text
summary to the path `CORPUS_LITERAL_SWEEP_REPORT` expected JSON at. The subsequent
`v06_work_inventory` regen correctly refused to write
(`refusing to write docs/work-inventory.json: this run would drop 7771 of the 9512 verification
stamp(s) ...`) — the stamp-loss guard (`decisions.md §12`'s own hazard) caught the malformed
report before any stamp was lost. Re-ran `corpus_literal_sweep -- --json-out <path>` (the correct
invocation) and the regeneration proceeded cleanly with the full stamp set retained. No
`--allow-stamp-loss` was used.

## Citation-drift self-heal (task brief's own named hazard)

`completion_atlas.py`'s `BUCKET_DEFINITIONS` (A, B, C, V citations) and
`missing_engine_tables.py`'s `ENGINE_SURFACE_CITATIONS` (companion, power) all drifted from this
cycle's own net +23-line insertion into `v06_work_inventory.rs`'s `classify()` function (the
debug-only lines added mid-cycle for investigation were removed before this receipt, landing the
net insertion at +23, not the original +26). Caught by running
`python3 scripts/completion_atlas.py --check` (`citation_failures=10` immediately after the code
edit) and `python3 scripts/missing_engine_tables.py --check` (`citation_failures=2`) **before**
writing this receipt, per the task brief's explicit warning. Each of the six drifted citations
(A: 10089→10109, B: 9787→9807, C: 10012→10032, V: 10740→10760, companion: 10089→10109,
power: 10168→10188) was independently re-derived by grepping the literal target content (not
computed from the diff hunk offset alone) and re-verified at content-match, not merely at line
number. Both gates are clean at this cycle's HEAD: `citation_failures=0` for both.

## Denominator gate

`python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` →
`files_checked=15 violations=0` (glob is top-level only; `atlas-defects.md`, being under
`artifacts/epic-3-core-rulebook/`, is outside its scope, unchanged from prior cycles' own count).

## PI gates

Not re-run this cycle — no deity/PI-adjacent record touched (the fix is a 3-key class_feature
lookup; `VACUOUS_PLACEHOLDER_CLASS_FEATURES`' own 3 reasons are plain PCGen mechanical
descriptions, no redacted/PI-marked text anywhere in this diff). Cycle 2's own defensive run
(`scripts/verify.sh --only site-public-status-pi-gate --only site-dashboard-pi-gate` → `PASS`
both) is the most recent confirmation on this branch; nothing in this cycle's diff can regress
either gate.

## Oracle pin

Not applicable — no figure in this receipt comes from the pinned PCGen oracle corpus; every
figure comes from the repo's own committed `data/corpus/` and `docs/work-inventory.json`.

## Status

- **Status:** partial

  Closes 3 of this cycle's own mechanism's 55 remaining units (55 → 52). Names every remaining
  unit by sub-cause, summing exactly to 52 (see "Row-count command output" above). This is not
  an operator-ruling request; `decisions.md §15` names exactly this shape as `partial`, not
  `blocked-escalated` — needing more cycles is never an escalation.

## Movement, four buckets

- **Closure:** 0 — no unit reached `DONE` (grounded/text-complete) this cycle. Per
  `decisions.md §2a`, a "genuinely nothing to compute, nothing to display" record does not earn
  a text-complete credit either (there is no text to render), so this is correctly NOT reported
  as closure.
- **Reclassification:** 3 — `Empty Selection ~ Standard {Barbarian, Monk, Rogue}` moved from
  bucket B (`engine-does-not-hold`) to bucket X (`deferred-with-reason`) — a genuine, verified
  correction of a mismatched verdict (the mechanism's own fallback claimed "the engine should
  hold this" for a record with no content to hold), not a cosmetic relabeling: the new verdict
  carries a real, corpus-grounded reason a future reader can act on, and the isolation check
  proves nothing else moved.
- **Reachability:** 0 — no previously-unreachable unit became reachable this cycle (these 3
  records were never player-facing content in the first place; nothing changed for a player).
- **Instrument-correction:** 1 — the atlas's own bucket-B taxonomy is corrected to admit a
  verdict shape it did not predict (logged as the `correction` retro event above and in
  `atlas-defects.md`); no NUMBER was wrong here (55 was the correct pre-cycle count both before
  and after this correction), so this is a taxonomy correction, not a figure correction.

## Notes

The real finding this cycle makes is procedural, matching this mechanism's own recurring
pattern (Cycle 2's near-miss was the same shape one level down): a record's OWNER-RESOLUTION path
can accidentally succeed via a coincidental suffix match even when the record is not really
"owned" by anything a Pathfinder rule recognizes — `"Empty Selection ~ Standard Barbarian"`
resolves an "owner" (`barbarian`) purely because the class name is a trailing substring of the
full key, not because PCGen's own `class` field (`"Empty Selection"`) says so. This is now a
closed, named exception (3 keys) rather than a widened predicate, so it cannot recur silently for
a fourth key sharing the same coincidental-suffix shape — a genuinely new such record would still
report `engine-does-not-hold`, correctly, until independently investigated and added to the named
list by hand.

A separate, non-code-affecting operational finding: the full `v06_work_inventory` regeneration
took roughly 11–12 minutes of wall-clock CPU-bound time on this box across BOTH the baseline
(unmodified) and fixed runs, with no observable difference attributable to this cycle's change
— confirmed by running the unmodified `HEAD` version of the binary first and observing the
identical CPU/RSS growth plateau-then-jump pattern at the same point (inside `gather_engine_facts`
/ the `class_feature_effect_wired` probe, before any `classify()` output is produced). This is
existing, pre-cycle cost, not a regression; noted here only because it was mistaken for a hang
mid-investigation and cost real time to rule out — a future cycle regenerating this file should
budget for it rather than assume a stall.

## Next-cycle plan

Unchanged from Cycle 2's own dispatch list, minus the sub-cause this cycle closed:

1. **Class-skill / companion-mount attribution (13 units)** — needs its own scoped investigation
   (widen `skill_allocation.rs`'s hand-kept lists, or a new attribution check).
2. **Domain Power `CLASS_FEATURE_POOLS` registration gap (2 units)** — flag for whichever cycle
   owns `class_feature_option_pool_record_with_magnitude_not_held_by_engine` (328 units); per
   Cycle 2's own note, registering `"Domain Power"` alone would not close either of THIS
   mechanism's own 2 units (`Leadership` has no computable formula at all; `Sun's Blessing` needs
   its own new formula in `domain_power.rs`) — the registration gap and this mechanism's 2 units
   are separate work sharing one registry.
3. **Wizard opposition-school + proficiency/grant tracking (37 units)** — the largest remaining
   share; genuinely new engine subsystems (spell-school restriction tracking, character-level
   proficiency-possession tracking, a non-choice-based class-feature effect-attribution probe).
   Recommend splitting further by sub-shape once scoped, rather than one oversized cycle.

---

# Cycle 2 — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`class_feature_option_pool_record_not_held_by_engine` mechanism)

Continues Cycle 1 (archived below, unedited) without re-deriving its investigation. Cycle 1
closed 6 of the original 63 and named all 57 remaining across seven sub-causes
(28+10+9+3+3+2+2). This cycle takes the cheapest of those seven — sub-cause 8, the
multi-`DESC:` ingest truncation (2 units) — and closes it.

- **Commit SHA:** `f98c6abddc` (parent `85ceae50aa`).
- **Files touched:**
  - `src/rules_core/cache_gen/class_feature.rs` — `desc_value` now joins a record's DESC
    segments into one description when doing so is safe (a genuine sequential continuation,
    no `PREVAREQ`/`PREVARGTEQ` choice-branch gate on any segment beyond the first), instead of
    keeping only the first segment unconditionally. A choice-branch-gated multi-DESC row
    (`Rage Power ~ Elemental Blood (Greater)`'s exact shape) is unchanged, byte-identical to
    this function's pre-fix behavior. Four new tests.
  - `src/rules_core/class_feature_pool_catalog.rs` — the multi-DESC render-and-refuse gate
    (`raw_tokens_carry_more_than_one_desc_segment`, unchanged, same test) is now paired with a
    new `shipped_description_is_the_already_regenerated_safe_multi_desc_join`, which recomputes
    the safe join directly from `raw_tokens` and requires it to EQUAL the already-shipped
    `data.description` — proving ingest has actually been re-run for this one record, rather
    than trusting the safe SHAPE alone. See "Discoveries" below for why the shape-alone version
    was tried and reverted. Three new tests (two integration, over the live regenerated corpus;
    one unit-level on the new function).
  - `data/corpus/core_rulebook/class_feature/martial_weapon_proficiency/martial_weapon_proficiency.json`,
    `data/corpus/core_rulebook/class_feature/octopus_wild_shape/poison.json` — regenerated via
    the guarded `--coordinates` path (`cargo run --locked --bin gen_cache_class_feature --
    --coordinates <file>`, 2 of 2 named coordinates matched, 18,043-unit corpus untouched
    elsewhere). Only `description` and `ingested_at` changed on each; every other field
    (`raw_tokens`, `wiring_class`, `pi_field`, `pi_marker`, ...) byte-identical.
  - `docs/work-inventory.json` (regenerated at HEAD, guarded path —
    `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` set from this session's own
    fresh sweep/fixture-check runs, no `--allow-stamp-loss` used or needed).
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    (regenerated by `completion_atlas.py --check`, a legitimate side effect of running the
    check at HEAD; reflects this cycle's own 2-unit closure).
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_cycle_receipt.md`
    (this file — Cycle 2 section prepended, Cycle 1 unedited below).
  - `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`
  - `docs/retro/events/sd34-at-34-e3-001.jsonl` (a `near_miss` event for the reverted
    corpus-wide relaxation attempt, below).

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` on the scoped diff — `git diff <base>...HEAD
  -- src/rules_core/ src/bin/ scripts/oracle_harness/ data/corpus/core_rulebook/**
  docs/work-inventory.json | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` surfaces
  only pre-existing `"display:sd32_class_ingest"`/`"display:sd32_simple_filename_kind_ingest"`
  **data values** inside `docs/work-inventory.json` (historical `wiring_class_signals` values
  from a prior cycle's own regeneration, not code this cycle wrote — the exact shape Cycle 1's
  own receipt already documented and self-healed as not a violation).

- **Wired-integration audit result:** `OK_NO_TOKENS` in effect. The scoped diff carries
  `"placeholder"` matches, all inside `src/bin/ingest_race_traits.rs` (PCGen's own literal
  `###Block: Placeholder objects...` comment); confirmed via `git log -1 -- src/bin/
  ingest_race_traits.rs` → `ae25d75d7d`, the `race_trait_absent_from_race_traits` sibling
  cycle, not this cycle's own edit. No new stub/mock/placeholder token in any file this cycle
  actually wrote.

- **Acceptance criterion (verbatim, `epic-breakdown.md` AT-34-E3-001):** "**970** Core Rulebook
  units whose table exists but which are not in it. **Evidence:** the atlas reporting bucket B
  at zero for `core_rulebook`, and the mechanism that placed them named — **by mechanism, not
  per record.**" This cycle's own bar (task brief / `decisions.md §14`/§15): drive
  `class_feature_option_pool_record_not_held_by_engine` further toward zero, taking the
  cheapest sub-cause Cycle 1 named. **AT-34-E3-001 as a whole does not close this cycle** — this
  cycle's own mechanism moves **57 → 55**, not to zero; five of the nine top-level mechanisms
  remain to close, and five sub-causes of this one mechanism remain (28+10+9+3+3, sub-cause 6
  the vacuous-placeholder shape still pending a `decisions.md §2` ruling).

## Re-derived population, not carried forward

Re-derived at this cycle's start SHA (`85ceae50aa`), matching Cycle 1's own closing figure
exactly (verified, not assumed):

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
57
```

## Discoveries — a corpus-wide blast radius, caught and reverted before commit

The first version of this cycle's fix relaxed `class_feature_pool_catalog.rs`'s multi-DESC
gate on SHAPE alone (skip refusal whenever no segment beyond the first carries a
`PREVAREQ`/`PREVARGTEQ` gate). Running the full `docs/work-inventory.json` regeneration and
diffing it against the committed baseline (`git show HEAD:docs/work-inventory.json`) surfaced
**188** status/evidence changes, not 2 — spanning `advanced_class_guide`, `advanced_players_guide`,
and other books, and mechanisms this cycle does not own (e.g.
`class_feature_owner_matched_by_name_but_record_not_held_by_engine`). The cause: many OTHER
corpus records share the same "multi-DESC, no choice-branch gate" SHAPE but have never been
regenerated by `cache_gen::class_feature::generate`'s new join logic — their shipped
`data.description` is still the OLD first-segment-only value. Gating on shape alone served
that stale, truncated text as `text-complete` for all of them — precisely the silent-truncation
defect this catalog exists to prevent, reopened at corpus scale.

**Reverted before commit.** The gate now requires an ingest-freshness PROOF, not just the safe
shape: `shipped_description_is_the_already_regenerated_safe_multi_desc_join` recomputes the
expected join directly from `raw_tokens` and only allows the record through when that
recomputed join EQUALS the already-shipped `data.description` — true only for a record
`cache_gen::class_feature::generate` has actually re-run since this fix landed. Every other
multi-DESC record (safe-shaped or not) stays refused, unchanged from before this cycle, until a
future cycle regenerates it. Re-running the full inventory regeneration with this corrected
gate confirmed exactly 2 status changes, both intended (see Figures). Retro `near_miss` event:
`docs/retro/events/sd34-at-34-e3-001.jsonl`.

## The fix (2 units)

`Martial Weapon Proficiency Output`'s two DESC segments and `Octopus Wild Shape ~ Poison`'s two
DESC segments (the second carrying a `|PRERULE:1,DisplayFullSpell` display-condition tail, not
a choice-branch gate) are both genuine sequential continuations with no mechanical reason for
the split. `cache_gen::class_feature::generate`'s `desc_value` now joins segments like these at
ingest time; the pool/standalone catalog's gate now recognizes a record that has been
regenerated this way and serves it. `Rage Power ~ Elemental Blood (Greater)`'s own regression
test (`elemental_blood_greater_is_refused_for_a_silently_truncated_multi_desc_row`) still passes
unchanged — its four `PREVAREQ`-gated alternative-element clauses are never eligible for the
join, at either layer.

## Figures + their re-derive commands

- **57 of 1,006** — this mechanism's share of `core_rulebook` bucket B at this cycle's start
  (Cycle 1's own closing figure). Command above; independently re-derived, matches exactly.
- **57 → 55** — this mechanism's own population, re-derived at this cycle's end SHA (same
  command, on the regenerated `docs/work-inventory.json`) → `55`.
- **2 units closed**, each confirmed individually:
  ```
  $ python3 -c "
  import json
  d=json.load(open('docs/work-inventory.json'))
  for k in ['Octopus Wild Shape ~ Poison','Martial Weapon Proficiency Output']:
      for u in d['units']:
          if u.get('book')=='core_rulebook' and u.get('corpus_key')==k and u.get('kind')=='class_feature':
              print(k,'->',u['status'],u['evidence'])
  "
  Octopus Wild Shape ~ Poison -> text-complete class_feature_pool_catalog_serves_a_rendered_description
  Martial Weapon Proficiency Output -> text-complete class_feature_standalone_catalog_serves_a_rendered_description
  ```
- **Isolation check** — before/after diff of the FULL `docs/work-inventory.json` against the
  committed baseline shows exactly these 2 `(book, kind, corpus_key)` entries changed status or
  evidence, nothing else:
  ```
  $ python3 -c "
  import json
  before = json.load(open('/tmp/e3_inventory_before.json'))['units']
  after = json.load(open('docs/work-inventory.json'))['units']
  def key(u): return (u['book'], u.get('kind'), u.get('corpus_key'))
  bmap = {key(u): u for u in before}
  changed = [k for k,u in ((key(u),u) for u in after)
             if bmap.get(k) and (bmap[k].get('status')!=u.get('status') or bmap[k].get('evidence')!=u.get('evidence'))]
  print(len(changed))
  for c in changed: print(c)
  "
  2
  ('core_rulebook', 'class_feature', 'Martial Weapon Proficiency Output')
  ('core_rulebook', 'class_feature', 'Octopus Wild Shape ~ Poison')
  ```
  (`/tmp/e3_inventory_before.json` = `git show HEAD:docs/work-inventory.json` at this cycle's
  start SHA, before this cycle's own regeneration.)
- **966** — `core_rulebook`'s real atlas-partitioned bucket B before/after this cycle:
  `python3 scripts/completion_atlas.py --book core_rulebook --check` → `B: 966` post-cycle
  (before: `968`, delta `-2`, exactly this cycle's closure). Sibling mechanisms confirmed
  unmoved (isolation check, corpus-wide count):
  `class_feature_owner_matched_by_name_but_record_not_held_by_engine` **346** (unchanged),
  `class_feature_option_pool_record_with_magnitude_not_held_by_engine` **333** (unchanged),
  `companion_absent_from_core_rulebook_companion_tables` **100** (unchanged),
  `race_trait_race_not_modelled` **132** (unchanged). `55+346+333+100+132 = 966` — matches
  exactly, no unnamed gap.
- **49,438** — corpus-wide unit population, unchanged by this cycle (no units added or removed,
  only 2 reclassified): `len(d['units'])` on the regenerated `docs/work-inventory.json` →
  `49438`.
- **48,708 of 51,482** — `corpus_literal_sweep` examined population, unchanged from Cycle 1's
  own baseline (this cycle edited 2 already-existing corpus files' `description` field; it
  added/removed zero corpus records, so the sweep's examined population must not move, and it
  did not): `48708 records examined of 51482 read ... 0 findings, CLEAN`.

## Row-count command output

```
$ python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
cr = [u for u in d['units'] if u['book']=='core_rulebook' and u['status']=='engine-does-not-hold']
tgt = [u for u in cr if u['evidence']=='class_feature_option_pool_record_not_held_by_engine']
print('class_feature_option_pool_record_not_held_by_engine remaining:', len(tgt))
"
class_feature_option_pool_record_not_held_by_engine remaining: 55
```

Row count is `55`, not `0` — **this cycle's own mechanism does not close.** `kanban.md`'s
AT-34-E3-001 row stays `in-progress`. This cycle reports `partial` (`decisions.md §15` — needing
more cycles is never `blocked-escalated`), naming its own remainder exactly: five sub-causes
Cycle 1 already named, unchanged and un-narrowed —
`proficiency/mechanical-grant tokens with no possession-tracking system` (28),
`class-skill/companion-mount attribution` (10 + 3 = 13, Cycle 1's sub-causes 3 and 5 combined
in its own next-cycle plan), `wizard opposition-school spell-restriction tracking` (9),
`vacuous placeholder rows` (3, pending a `decisions.md §2` ruling),
`Domain Power CLASS_FEATURE_POOLS registration gap` (2, shared with the 333-unit
`with_magnitude` sibling mechanism, not fixed here). `28+13+9+3+2 = 55` — matches exactly.

## Build scope verified

- `cargo test --locked --lib` (workspace lib): `2872 passed; 0 failed; 14 ignored`.
- `cargo test --locked --lib class_feature` (scoped, includes every new test this cycle added):
  `201 passed; 0 failed; 10 ignored`.
- `cargo test --locked --bin v06_work_inventory` (scoped): `376 passed; 0 failed` — unchanged
  from Cycle 1's own figure, run **after** the last write that could move a figure (the
  `docs/work-inventory.json` regeneration) — `decisions.md §12` L7.
- `cargo test --locked --no-run` (full workspace): clean, exit 0.
  `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001`.
- `apps/desktop/src-tauri` (separate cargo workspace, tested explicitly):
  `cargo test --locked --no-run` with its own
  `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001-desktop` — clean, exit 0.
- Run at SHA: `f98c6abddc`.

## Sweep population

`corpus_literal_sweep`: `48708 records examined of 51482 read, 0 findings, CLEAN` — same as
Cycle 1's own baseline; this cycle regenerated 2 already-existing corpus records' `description`
field only, adding/removing zero files, so the examined population must not move, and does not
(`decisions.md §12` L8).

`derived_evaluator_fixture_check`: `1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0
not ingested` — unchanged from Cycle 1's own baseline, supplied as `DERIVED_FIXTURE_CHECK_REPORT`
for the guarded regeneration.

## Citation-drift self-heal

Not applicable this cycle — no line was inserted or removed in `src/bin/v06_work_inventory.rs`
(this cycle touched `src/rules_core/cache_gen/class_feature.rs` and `src/rules_core/
class_feature_pool_catalog.rs` only, neither cited by `completion_atlas.py`'s
`BUCKET_DEFINITIONS` or `missing_engine_tables.py`'s `ENGINE_SURFACE_CITATIONS`). Both gates
re-run defensively: `python3 scripts/completion_atlas.py --check` → `citation_failures=0`;
`python3 scripts/missing_engine_tables.py --check` → `citation_failures=0`.

## Denominator gate

`python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` →
`files_checked=15 violations=0`.

## PI gates (Cycle 1's own precedent, re-run defensively)

`scripts/verify.sh --only site-public-status-pi-gate --only site-dashboard-pi-gate` → `PASS`
(both). Not directly implicated by this cycle's change (no deity/PI-adjacent record touched),
re-run to confirm no regression.

## Oracle pin

Not applicable — no figure in this receipt comes from the pinned PCGen oracle corpus; every
figure comes from the repo's own committed `data/corpus/` and `docs/work-inventory.json`. The
`--coordinates` regeneration reads the local PCGen checkout (`PCGEN_CORPUS_ROOT`), unpinned,
matching Cycle 1's own convention for this generator.

- **Status:** partial

  Closes 2 of this cycle's own mechanism's 57 remaining units (57 → 55). Names every remaining
  unit by sub-cause, summing exactly to 55 (see "Row-count command output" above). This is not
  an operator-ruling request; `decisions.md §15` names exactly this shape as `partial`, not
  `blocked-escalated` — needing more cycles is never an escalation.

## Movement, four buckets

- **Closure:** 2 — `Martial Weapon Proficiency Output`, `Octopus Wild Shape ~ Poison` moved from
  bucket B (`engine-does-not-hold`) to `text-complete` (DONE) via a real, tested, ingest-freshness-
  proven engine addition (the safe multi-DESC join) — the engine genuinely holds and serves
  these records' full, untruncated descriptions now, not a relabeling.
- **Reclassification:** 0 — no unit changed bucket without a genuine holds change; the
  isolation check confirms every sibling mechanism and every OTHER multi-DESC-shaped record
  corpus-wide stayed exactly where it was (see Discoveries — this is precisely what the
  ingest-freshness-proof gate was built to guarantee, after the shape-alone version failed it).
- **Reachability:** 0 — no previously-unreachable unit became reachable this cycle.
- **Instrument-correction:** 0 — no count changed because a measurement method was wrong.

## Notes

The real finding this cycle makes is procedural, not corpus-specific: a "safe shape" check
built for TWO already-regenerated records is not automatically safe for every OTHER record
sharing that shape corpus-wide, when the check runs against a catalog that reads pre-computed
`data.description` rather than raw source. The fix — require the shipped field to already equal
what the safe transformation would produce, not merely to look like a case where it's allowed —
generalizes: any future cycle that teaches an ingest-time transformation a new safe case must
verify newly-eligible records against their OWN regenerated `data.description`, not against the
shape of `raw_tokens` alone, or risk promoting every other record sharing that shape before its
own ingest has caught up.

## Next-cycle plan

Unchanged from Cycle 1's own dispatch list, minus the sub-cause this cycle closed:

1. **Vacuous placeholders (3 units)** — needs a `decisions.md §2` ruling on verdict shape
   first.
2. **Class-skill / companion-mount attribution (13 units)** — needs its own scoped
   investigation (widen `skill_allocation.rs`'s hand-kept lists, or a new attribution check).
3. **Wizard opposition-school + proficiency/grant tracking (37 units)** — the largest remaining
   share; genuinely new engine subsystems. Recommend splitting further by sub-shape once
   scoped.
4. **Domain Power `CLASS_FEATURE_POOLS` registration gap** — flag for whichever cycle owns
   `class_feature_option_pool_record_with_magnitude_not_held_by_engine` (333 units); not fixed
   here. Note for that cycle: registering `"Domain Power"` alone would not close either of THIS
   mechanism's own 2 Domain Power units (`Leadership` has no computable formula at all;
   `Sun's Blessing` needs its own new formula added to `domain_power.rs` before the
   registration would attribute anything to it) — the registration gap and this mechanism's 2
   units are two separate pieces of work that happen to share one registry.

---

# Cycle 1 (archived, unedited below)
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
- Run at SHA: `8e7aecc855`.

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
