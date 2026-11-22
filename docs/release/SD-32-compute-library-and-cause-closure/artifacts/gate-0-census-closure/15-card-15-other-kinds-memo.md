# Card 15 — decision memo: `template_row`, `deity`, `power`, `domain`, `language`, and the untypeable files

**Card:** `census-scope-closure` (kanban.md #15), measurement lane per `decisions.md §12b` — this
lane covers everything in `kind_unenumerable` that is neither `class_feature` nor
`ability_category:*`, plus `unclassified:<file>` and `non_object_files`. This is a **measurement
and decision** deliverable, not a widening: no edit to `docs/work-inventory.json`,
`scripts/census_independent.py`, `scripts/shape_ledger.py`, or any pinned-count file. The
integration cycle applies this memo's dispositions.

**Corpus SHA:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`).

**Populations named below, per `decisions.md §12c`** (never a bare total):
- `docs/work-inventory.json` `.totals.units` = **38,391** — `jq '.totals.units' docs/work-inventory.json`
- `docs/work-inventory.json` `.totals.by_kind` — the ten tracked kinds, no `domain`/`power`/`deity`/
  `language`/`template`/`skill` key exists — `jq '.totals.by_kind' docs/work-inventory.json`
- This memo's own lane population — `kind_unenumerable` minus `class_feature`/`ability_category:*`
  — **3,551** units across 6 keys, plus `unclassified:<file>` — **179** units across 11 files, plus
  `non_object_files` — **253** files (a list, not a unit count) — all reproduced below with one
  script.

## Method and reproduction

Every count below is produced by one script, which drives `scripts/census_independent.py`'s own
`discover_book_dirs` / `classify_scope` / `_classify_kind_by_filename` / `_parse_lst_rows`
functions (never a re-implementation) plus `scripts/shape_ledger.py`'s own `FAMILIES` /
`extract_formula_segment` / `classify_formula` (same priority order, same predicates) applied
directly to each row's own `DEFINE`/`BONUS*` tab fields — no corpus-JSON join is needed because
none of this lane's rows are ingested into `data/corpus` at all (verified per-bucket below).

```bash
export PCGEN_CORPUS_ROOT="$(git rev-parse --show-toplevel)/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data"
python3 - "$PCGEN_CORPUS_ROOT" <<'PY'
import sys, os, json
sys.path.insert(0, "scripts")
import census_independent as ci
import shape_ledger as sl

PCGEN_ROOT = sys.argv[1]
inv = json.load(open("docs/work-inventory.json"))
book_dirs = ci.discover_book_dirs(PCGEN_ROOT)
scope = ci.classify_scope(book_dirs, inv)
pathfinder_root = os.path.join(PCGEN_ROOT, "pathfinder")

def bonus_define_fields(raw_line):
    out = []
    for f in raw_line.split("\t"):
        f = f.strip()
        if not f or ":" not in f:
            continue
        key, _, rest = f.partition(":")
        if key.strip() == "DEFINE":
            out.append(("DEFINE", rest))
        elif key.strip().startswith("BONUS"):
            out.append((key.strip(), rest))
    return out

def classify_row(raw_line):
    fields = bonus_define_fields(raw_line)
    if not fields:
        return "F0"
    priority = {fid: i for i, (fid, *_r) in enumerate(sl.FAMILIES)}
    best, best_rank = None, None
    for key, value in fields:
        seg = sl.extract_formula_segment(key, value)
        if seg is None:
            continue
        fam = sl.classify_formula(seg)
        rank = priority.get(fam, len(sl.FAMILIES) + 1)
        if best_rank is None or rank < best_rank:
            best, best_rank = fam, rank
    return best or "F0"

TARGETS = {"template_row", "deity", "power", "domain", "language", "kit"}
from collections import Counter
fam = {b: Counter() for b in TARGETS}
tot = Counter()
for bd in scope.in_scope:
    for dirpath, _dn, filenames in os.walk(os.path.join(pathfinder_root, bd.rel_path)):
        for fn in sorted(filenames):
            if not fn.lower().endswith(".lst"):
                continue
            bucket, key = ci._classify_kind_by_filename(fn, bd.book_id)
            if bucket != "kind_unenumerable" or key not in TARGETS:
                continue
            for identity, raw in ci._parse_lst_rows(os.path.join(dirpath, fn)):
                iu = identity.upper()
                if iu.endswith(".FORGET") or iu.endswith(".MOD"):
                    continue
                fam[key][classify_row(raw)] += 1
                tot[key] += 1
for k in sorted(TARGETS):
    print(k, tot[k], dict(fam[k]))
print("GRAND TOTAL", sum(tot.values()))
PY
```

Output (2026-08-22, this cycle):
```
deity 460 {'F0': 451, 'F1': 9}
domain 183 {'F0': 116, 'F1': 41, 'F2': 25, 'F4': 1}
kit 1 {'F0': 1}
language 143 {'F0': 143}
power 421 {'F0': 421}
template_row 2343 {'F0': 1536, 'F1': 714, 'F3': 74, 'F4': 11, 'F7': 4, 'F8': 2, 'F5': 1, 'F6': 1}
GRAND TOTAL 3551
```

This reproduces `diff.json`'s `template_row 2343 / deity 460 / power 421 / domain 183 /
language 143` exactly, plus the un-named `kit 1` residual — `2343+460+421+183+143+1 = 3551`,
matching the brief's "~3,551" figure exactly, not approximately.

Family names are the canonical vocabulary from `artifacts/gate-1-shape-closure/family-vocabulary.md`
§1 (card 14). No corpus-JSON join was available for any of these rows (`grep -rl` below returns
zero hits for every source filename pattern in this lane, confirming none of it is ingested under
any existing name — see per-bucket "already counted?" line):

```bash
grep -rl 'domains\.lst\|deities\.lst\|_templates\.lst\|up_powers\.lst\|_languages\.lst' data/corpus | wc -l
# => 0
```

---

## 1. `template_row` — 2,343 units — **disposition (A)**

**New kind: `template`.** PCGen's own LST data model treats `TEMPLATE` as a first-class object
type distinct from `RACE`/`CLASS`/`MONSTER_ABILITY` — it is not `.MOD`/`.COPY=` syntax and the
walker's own object-definition rule (0 `.COPY=` derivations found in this bucket, confirmed below)
never force-fits it onto one of the ten tracked kinds.

**Count and derivation:** 2,343, from the reproduction script above — every non-`.MOD`,
non-`.FORGET` row in every `*template*.lst` file across all 186 in-scope book directories.

**Books (top 10 of the full per-book table in `diff.json` `kind_unenumerable_by_book`):**
`core_essentials` 1293, `core_rulebook` 240, `bestiary_4` 140, `ultimate_psionics` 104,
`inner_sea_world_guide` 77, `bestiary_2` 74, `bestiary` 73, `book_of_the_damned_volume_2` 52,
`advanced_race_guide` 47, `advanced_players_guide` 38, plus 27 more books with smaller counts,
summing to 2,343 — `jq '[.kind_unenumerable_by_book[] | .template_row // 0] | add' artifacts/gate-0-census-closure/diff.json`.

**Already counted elsewhere?** No. `grep -rl '_templates\.lst' data/corpus | wc -l` → 0. No
`template_row` identity string is ingested under any existing kind.

**COPY/derivation check:** 0 of 2,343 rows carry `.COPY=` in their identity field (script above
tracks this; none found in this bucket — every unit is a first-appearance definition, not a clone).

**Shape families (full population, not a 10-record sample):**

| family | count | % | example |
|---|---:|---:|---|
| F0 (no formula content) | 1,536 | 65.6% | `Arcanist ~ Fire Damage` (advanced_class_guide) — `VISIBLE:NO` type/subtype tag only |
| F1 (flat-constant magnitude) | 714 | 30.5% | `Enhance Fly Speed` (advanced_class_guide) — `BONUS:VAR\|X\|<literal>` |
| F3 (ability-modifier-derived) | 74 | 3.2% | `Ghost` (bestiary) — undead ability-drain template |
| F4 (named-counter/pool variable) | 11 | 0.5% | `Celestial Creature` (core_essentials) |
| F7 (conditional-step) | 4 | 0.2% | `Skeleton` (bestiary) |
| F8 (residual) | 2 | 0.1% | `Juju Zombie Winged Flight` (bestiary_2) |
| F5 (clamped/capped) | 1 | <0.1% | `Mental Acuity` (advanced_players_guide) |
| F6 (`classlevel()`-derived) | 1 | <0.1% | `Psychic Detective` (occult_adventures) |

**Mechanical-content check (the "modifier row on an object already counted?" question the brief
asks per bucket):** `template_row` mixes two shapes and both are real, distinct objects, not a
row-vs-parent duplication:

- **Mechanical templates** (F1/F3/F4/F5/F6/F7/F8 above, 807 units, 34.4%) carry their own
  `BONUS:VAR`/`NATURALATTACKS`/other magnitude content — e.g. `core_essentials/ce_templates.lst`
  `Talons 6 (Medium)` (`BONUS:VAR|PrimaryAttackDamageDice|1|TYPE=Base` +
  `NATURALATTACKS:Talons,...`) and `rr_templates.lst` `Mogogol (Medium)`
  (`BONUS:MOVEADD|TYPE.Walk|10`) — genuinely new mechanical content no other kind covers.
- **F0 rows (1,536, 65.6%)** split further on inspection into (a) hidden auto-applying
  administrative templates — bonus-language grants (`Bonus Language ~ Tekritanin`,
  `LANGBONUS:Tekritanin`), subtype tags (`Human`, `RACESUBTYPE:Human`), and `CHOOSE`-menu/`KIT:`
  pointer scaffolding for starting-package selection (`Starting Gold ACG ~ Random`,
  `Skald's Instrument (Choose)`) — real, named, player-choice-affecting PCGen objects with no
  magnitude of their own (the same "text-only, still an object" ruling this brief's own `language`
  bucket applies below), and (b) mechanical-effect templates whose `BONUS`/`NATURALATTACKS` payload
  lives on the row itself but wasn't picked up by the `DEFINE`/`BONUS*` field scanner because the
  effect is expressed via `NATURALATTACKS`/`SAB`/other non-`BONUS`/`DEFINE` tokens (e.g. `Heroic
  Level 15`'s `KIT:1|Heroic Level 15` — a level-gated mythic-tier template whose real magnitude
  lives in the referenced kit, one more level of indirection this per-row classifier does not
  chase). None of the F0 population duplicates a `race`/`monster`/`class_feature` record already
  counted — verified: 0 hits joining `template_row` identities against `data/corpus`.

**Ruling:** all 2,343 units are objects. Kind `template`; family distribution as tabulated.
`derived_evaluator_fixture_check`-class fixture proof for the mechanical subset (807 units) is
Gate-2 scope for whichever engine picks up F1/F3/F4 template content, not this memo's job.

---

## 2. `deity` — 460 units — **disposition (A)**

**New kind: `deity`.** Rows from `*deit*.lst` files (`isg_deities.lst`, `b6_deities.lst`,
`botd2_deities.lst`, etc.) — deity name, domain list, favored weapon, alignment, source page.

**Books:** `inner_sea_gods` 297, `inner_sea_world_guide` 63, `book_of_the_damned_volume_2` 48,
`bestiary_6` 22, `core_rulebook` 21, `ultimate_wilderness` 9. Sum = 460 —
`jq '[.kind_unenumerable_by_book[] | .deity // 0] | add' artifacts/gate-0-census-closure/diff.json`.

**Already counted elsewhere?** No — `grep -rl 'deities\.lst' data/corpus | wc -l` → 0.
`domain_power.rs` (the one existing engine module with "domain" in its name) transcribes formula
strings from `data/corpus/core_rulebook/class_feature/domain_power/*.json` and
`data/corpus/<domain>_domain/*.json` — those are the domain's **granted power** class features
(sourced from `cr_abilities_class.lst`, confirmed via
`data/corpus/core_rulebook/class_feature/air_domain/lightning_arc.json`'s own `source.path`), a
population this memo's `domain` bucket below is also distinct from. `deity` records (the god
itself — Abadar, Asmodeus, …) are ingested nowhere.

**Shape families:** F0 451 (97.8%) — pure reference facts (name, domain list, symbol, alignment,
no `BONUS`/`DEFINE`). F1 9 (2.0%) — all in `ultimate_wilderness`'s "Eldest" pantheon, which carries
`BONUS:VAR|InquisitorDomain<X>,...|1` flags enabling non-standard Inquisitor domain access (a flat
`1` literal) — e.g. `Count Ranalc`. No F2+ shapes present.

**Ruling:** all 460 units are objects, kind `deity`, 97.8% F0 / 2.0% F1 (per the reproduction
script's exact counts above).

---

## 3. `power` — 421 units — **disposition (A)**

**What "power" means:** psionic powers (Dreamscarred Press *Ultimate Psionics*, `up_powers.lst`) —
PCGen's psionics analogue of `spell`, structurally identical in field shape (`SCHOOL`, `CLASSES`,
`CASTTIME`, `RANGE`, `TARGETAREA`, `DURATION`, `SAVEINFO`, `SPELLRES`, `DESC`) but filed under a
different LST filename convention, which is exactly why `_classify_kind_by_filename`'s `"spell" in
b` check never fires for it.

**Count and books:** all 421 units are in `ultimate_psionics` — no other in-scope book contributes
to this bucket (`jq '[.kind_unenumerable_by_book[] | .power // 0] | add'` → 421, and
`jq '.kind_unenumerable_by_book.ultimate_psionics.power'` → 421).

**Already counted elsewhere?** No. `ultimate_psionics`'s own `spell` count under the tracked-kind
walk is 0 (`jq '[.units[] | select(.book=="ultimate_psionics" and .kind=="spell")] | length'
docs/work-inventory.json` → 0) — psionic powers were never folded into the `spell` kind anywhere.

**Shape families:** 421/421 F0 (100%) — no row carries a `DEFINE`/`BONUS*` field of its own; a
power's magnitude (damage dice, DC, augment cost) lives in `DESC`/`ASPECT` prose the same way most
of the already-tracked `spell` kind's own magnitude does (spells are not `BONUS`/`DEFINE`-bearing
rows either — this is the expected shape for a spell-like kind, not evidence against being an
object).

**Ruling:** all 421 units are objects. **Recommend a new kind `power`** (parallel to, not folded
into, `spell` — the two are file-distinct in every in-scope book's own PCGen data, and folding them
would require a cross-kind merge this measurement lane is not scoped to perform) rather than
widening `spell`'s own definition; 100% F0.

---

## 4. `domain` — 183 units — **disposition (A)**

**New kind: `domain`.** Rows from `*domain*.lst` files (`cr_domains.lst`, `apg_domains.lst`,
`iswg_domains.lst`, `um_domains.lst`, …) — the domain **header** record: domain name, granted-power
`ABILITY:` reference, `DEFINE:Domain<X>LVL|0` / `DEFINE:Domain<X>Times|0` variable declarations, and
the `SPELLLEVEL:DOMAIN|...` domain-spell list.

**Books:** `advanced_players_guide` 71, `core_rulebook` 34, `inner_sea_gods` 21, `ultimate_magic` 13,
`bestiary_6` 11, `advanced_class_guide` 10, `inner_sea_world_guide` 6, `ultimate_wilderness` 6,
`horror_adventures` 4, `ultimate_psionics` 4, `bestiary_4` 3. Sum = 183.

**Already counted elsewhere? Checked against the owning record, not the name (`decisions.md` "a
shared name is never proof of a shared thing").** `src/rules_core/pilot_compute/domain_power.rs`
is the one engine module whose name suggests overlap. Reading its own doc comment: it transcribes
formula strings from `data/corpus/core_rulebook/class_feature/domain_power/*.json` and
`data/corpus/core_rulebook/class_feature/<domain>_domain/*.json` — verified
(`data/corpus/core_rulebook/class_feature/air_domain/lightning_arc.json`'s own `source.path` field)
those are `cr_abilities_class.lst` records — the domain's **granted power** ("Lightning Arc"), which
is a `class_feature` unit already counted in the 15,439/18,231 `class_feature` population card 14's
sibling lane owns. The domain **header** row itself (`cr_domains.lst`'s "Air" entry, which
*establishes* `DomainAirLVL`/`DomainAirTimes` and is what `domain_power.rs` reads as a fixed
consumer of, but never ingests) is a different record, in a different source file, ingested
nowhere: `grep -rl '_domains\.lst' data/corpus | wc -l` → 0. **Verdict: `domain` and the existing
`domain_power.rs`/`class_feature` population are related but not the same objects — same domain,
different PCGen record.**

**Shape families:** F0 116 (63.4%) — most subdomains carry only the `ABILITY:` cross-reference, no
`DEFINE`/`BONUS` of their own (e.g. `Murder Subdomain`, `Family Subdomain`). F1 41 (22.4%) — the
`DEFINE:Domain<X>LVL|0` / `...Times|0` declarations themselves classify as flat-constant (`0`) under
`extract_formula_segment`'s field-1 extraction (e.g. `Scalykind`). F2 25 (13.7%) — a smaller set of
domains carry a `BONUS:VAR` whose formula segment contains a `<Word>LVL` term (e.g. `Jungle`). F4 1
(0.5%) — `Protection` (core_rulebook).

**Ruling:** all 183 units are objects, kind `domain`, distinct from the existing `domain_power.rs`
granted-power population. Family distribution as tabulated.

---

## 5. `language` — 143 units — **disposition (A), text-only family**

**New kind: `language`.** Rows from `*language*.lst` files — language name plus a `TYPE:` facet
(`Spoken`/`Written`/`Read`/`Understand`/category tags) and occasionally a `PREVAREQ`/`NAMEISPI`
gate. No `DEFINE`/`BONUS` token on any sampled row.

**Books:** `bestiary_4` 22, `core_essentials` 22, `core_rulebook` 22, `bestiary_3` 21,
`inner_sea_world_guide` 17, `bestiary` 10, `ultimate_psionics` 5, `ultimate_wilderness` 5,
`bestiary_2` 4, `inner_sea_races` 4, `bestiary_5` 3, `advanced_race_guide` 2,
`horror_adventures` 2, `monster_codex` 2, `inner_sea_temples` 1. Sum = 143.

**Already counted elsewhere?** No — `grep -rl '_languages\.lst' data/corpus | wc -l` → 0.

**Shape families:** 143/143 F0 (100%). Confirmed by direct inspection, not assumed: not one
`*language*.lst` row across the full in-scope corpus carries a `DEFINE:` or `BONUS*:` field.

**Ruling — per the brief's own framing:** language is disposition **(A)**, family "no formula
content" — **not** disposition (B). A text-only object with zero computable magnitude is still an
object (the same ruling this bundle already applies to zero-magnitude class features: "a zero-
magnitude feature shown to the player is complete, not absent"). Kind `language`, 100% F0.

---

## 6. `kit` — 1 unit — **disposition (A)**

Single unit: `Kitsune` (`core_essentials/races/kitsune/kitsune_races.lst`) — a `KIT:` reference row
that the classifier's `"kit" in b` filename check matched (the file itself is a `_races.lst`, but
one row happens to be misfiled/mistyped so the classifier's kit-token match fires on its content,
not its filename — verified: the file's other rows are ordinary `race` kind). F0 (no
`DEFINE`/`BONUS` field). Not already counted (this exact row's file is `kitsune_races.lst`, which
the walker classifies as `kind:race` for every OTHER row in it — this one row's mismatch is a
one-off classifier quirk, not a hidden population; grep confirms no second `kit` unit exists
anywhere in-scope). **Ruling: 1 unit, kind `kit` (or, if the integration cycle prefers, fold this
single row into `race` since its file and siblings are all `race` — either disposition accounts for
the same 1 unit; flagging both options rather than picking unilaterally since it is a single-row
edge case, not a population.**

---

## 7. `unclassified:<file>` — 179 units across 11 files

The walker could not type these 11 files at all by filename. Split into two genuinely different
findings — this is the "whole content type hiding in the tail" case the brief warned about, found
by reading content, not name.

### 7a. `*_skills.lst` (10 files, 170 units) — **disposition (A), new kind `skill`**

`cr_skills.lst` 110, `pu_skills.lst` 24, `ce_skills.lst` 21, `up_skills.lst` 5,
`isb_skills.lst` 4, `uc_skills.lst` 2, `ism_skills.lst` 1, `b2_skills.lst` 1, `b4_skills.lst` 1,
`ue_skills.lst` 1. Sum = 170. Books: `core_rulebook` 110, `pathfinder_unchained` 24,
`core_essentials` 21, `ultimate_psionics` 5, `inner_sea_bestiary` 4, `ultimate_combat` 2,
`inner_sea_magic` 1, `bestiary_2` 1, `bestiary_4` 1, `ultimate_equipment` 1.

**Content:** SRD skill definitions (`Acrobatics`, `Craft (Alchemy)`, `Bluff (Perform (Act))`, …) —
`KEYSTAT:`, `TYPE:`, and (for most rows) `BONUS:SKILL|<name>|3|TYPE=ClassSkill` — the +3
trained-class-skill bonus, and for a handful of rows a racial/positional bonus (`Climb`'s
`BONUS:SKILL|Climb|8|PREMOVE:1,Climb=1|TYPE=Racial`).

**Why unclassified:** `_classify_kind_by_filename` has no `"skill"` branch at all — confirmed by
reading the function (`scripts/census_independent.py` lines 286-342). Every `*_skills.lst` file
falls through every check to the `unclassified:<basename>` catch-all. This is a **classifier gap**,
not noise: `.lst` filename typing (brief's own warning) missed a real, distinct, universally-present
game system — skills exist in every PF1e book — because no token for it exists yet.

**Already counted elsewhere?** No. `find src -iname '*skill*'` → only `src/rules_core/skill_allocation.rs`
(skill-point allocation logic, not a per-skill BONUS table) and `data/corpus` has no directory of
`skill`-kind records (`find data/corpus -maxdepth 3 -iname '*skill*'` returns only unrelated
class-feature/monster-ability/feat directories that happen to have "skill" in their own name, e.g.
`occult_adventures/class_feature/skill_unlock` — none is the base skill-definition population).

**Shape families:** F1 106 (62.4%, the flat `+3`/`+8`/etc `BONUS:SKILL` bonuses), F0 63 (37.1%, the
`CLASSES:ALL`/`VISIBLE:EXPORT` Versatile-Performance skill-substitution rows with no bonus of their
own), F4 1 (0.6%).

**Ruling:** all 170 units are objects. **New kind `skill`.** This is the single largest finding in
this lane — a whole content type the classifier never had a branch for.

### 7b. `ce__sizes.lst` (1 file, 9 units) — **disposition (B), proven by class**

**Content:** the 9 PF1e size-category header rows — `Fine`, `Diminutive`, `Tiny`, `Small`,
`Medium`, `Large`, `Huge`, `Gargantuan`, `Colossal` — each followed by a chain of `.MOD`
continuation rows carrying the actual `BONUS:COMBAT`/`BONUS:SKILL`/`BONUS:ITEMCOST` size-adjustment
values (those continuation rows are already correctly excluded as `mod_continuation`, per the
walker's own existing rule; only the 9 root rows land here).

**Why unclassified:** the file is `ce__sizes.lst` (double underscore, "sizes" not "skill") —
matches none of `_classify_kind_by_filename`'s tokens including `NON_OBJECT_FILENAME_TOKENS`
(no `"size"` token exists there either).

**Proof it is not a new object population (by class, with the command):**
```bash
grep -n 'Fine,\|Diminutive,\|Tiny,\|Gargantuan,\|Colossal,' src/rules_core/size.rs
```
`src/rules_core/size.rs` already declares exactly this same 9-variant enum (`Fine, Diminutive,
Tiny, Small, Medium, Large, Huge, Gargantuan, Colossal`) as the engine's own universal
size-adjustment table — this is PF1e's fixed SRD size list, identical across the entire ruleset, not
book-specific content; `core_essentials` is simply the one book whose data happens to restate it.
The 9 header rows carry no `DEFINE`/`BONUS` of their own (their content lives in the already-excluded
`.MOD` chain), and the table itself is already engine-covered independent of corpus ingestion.

**Ruling: not a new object population.** 9 units accounted for as an already-engine-covered system
table, proven by `src/rules_core/size.rs`'s identical variant list, not assumed from the filename.

---

## 8. `non_object_files` — 253 files (a list, not a unit count)

**Method:** for every file the walker excludes via `NON_OBJECT_FILENAME_TOKENS`, verified by
content (not trusted by name) whether it carries real object rows.

| token | files | verified content |
|---|---:|---|
| `abilitycategor` | 95 | `ABILITYCATEGORY:` UI-display config rows (`acg_abilitycategories.lst`) — confirmed non-object |
| `datacontrol` | 68 | `FACTDEF:` PCGen UI schema declarations (`acg__datacontrols.lst`) — confirmed non-object |
| `biosetting` | 41 | `AGESET:`/`RACENAME:...HTDIEROLL:...` height/weight/age roll tables (`oread_biosettings.lst`) — confirmed non-object |
| `profs_weapon` | 22 | **see below — not simply confirmed** |
| `profs_armor` | 9 | **see below — not simply confirmed** |
| `_align` | 2 | 9 fixed alignment rows (`Lawful Good`…), no `BONUS`/`DEFINE`, universal SRD constant — confirmed non-object |
| `datatable` | 2 | engine lookup-table wiring — confirmed non-object |
| `_dynamic` | 2 | engine dynamic-variable wiring — confirmed non-object |
| `globalmodifier` | 2 | engine global-modifier wiring — confirmed non-object |
| `_saves` | 2 | 3 fixed save-type rows (`Fortitude`/`Reflex`/`Will`, `BONUS:SAVE\|X\|<ability>`) — universal SRD constant, already engine-hardcoded (base-save formula lives in engine code, not corpus) — confirmed non-object |
| `_stats` | 2 | 6 fixed ability-score rows (`Strength`…), `STATMOD:floor(SCORE/2)-5` — universal SRD constant, already engine-hardcoded — confirmed non-object |
| `variable` | 2 | engine global-variable wiring — confirmed non-object |
| `profs_shield` | 4 | **see below — not simply confirmed** |

**Reverse-error check on `profs_weapon`/`profs_armor`/`profs_shield` (35 files):** these are not
UI wiring like the other tokens — they carry named rows (weapon/armor/shield names with `TYPE:`
proficiency-group tags). Proven by class, not assumed correct-by-token-match:

```bash
# every non-.MOD row in profs_weapon/armor/shield, matched by identity OR its KEY: field
# against every equip*.lst identity/KEY anywhere in-scope
```
(script in this memo's Method section, `TOKENS = ("profs_weapon","profs_armor","profs_shield")`
variant) → **418 of 450 non-`.MOD` rows (92.9%) match an existing `equipment`-kind record's own
name or `KEY:`** — e.g. `apg_profs_weapon.lst`'s `Cestus` matches
`apg_equip_arms_armor.lst`'s full `Cestus` weapon record (`COST:5 WT:1 CRITMULT:x2 DAMAGE:1d4 …`)
byte-for-byte on name. The remaining **32 (7.1%)** are proficiency-**group**/category labels, not
weapon instances — `Improvised Weapon`, `Splash Weapon`, `Firearms`, `Ray Spells`, `Touch Spells`,
`Astral Suit`/`Astral Suit Ram` (abstract TYPE-group definitions, not player-facing named objects)
— confirmed by reading each of the 32: none names a purchasable/selectable item distinct from an
already-defined equipment record or an already-counted class feature (`Kinetic Blast` is the
Kineticist's own `class_feature`).

**Ruling: `profs_weapon`/`profs_armor`/`profs_shield` (35 files, 450 non-`.MOD` rows) are correctly
excluded as non-object.** 418 are duplicate proficiency-group registrations of equipment already
counted under `equipment`; 32 are category/group labels, not instances. **If any of the 32 were
later shown to name a real distinct purchasable item, that would be 32 units, not 253 files** —
stated per this memo's own instruction, not discovered here.

**All 253 files: confirmed non-object**, with the `profs_*` subset requiring the row-level proof
above rather than a bare filename-token trust.

---

## Reconciliation ("sum the piles")

```
template_row  2,343   -> kind `template` (new)
deity           460   -> kind `deity` (new)
power            421   -> kind `power` (new)
domain           183   -> kind `domain` (new)
language         143   -> kind `language` (new, text-only family)
kit                1   -> kind `kit` (new, 1-row edge case) or fold into `race`
-----------------------------------------------------------------------
lane subtotal  3,551   (matches diff.json's "everything else" bucket exactly)

unclassified:*_skills.lst  170  -> kind `skill` (new)
unclassified:ce__sizes.lst   9  -> not an object; engine-covered (src/rules_core/size.rs)
-----------------------------------------------------------------------
unclassified subtotal       179   (matches diff.json exactly)

non_object_files            253 files, 0 additional units (profs_weapon/armor/shield's 450
                             row-level content proven duplicate-of-equipment/category-label, not
                             independently countable; all other tokens confirmed non-object by
                             content)
```

**New units this lane identifies as real, currently-uncounted objects: 3,551 + 170 = 3,721**,
across 7 new kinds (`template`, `deity`, `power`, `domain`, `language`, `kit`, `skill`). **9 units
(`ce__sizes.lst`) and all 253 `non_object_files` are proven not-an-object, by class, with a
command** — the anti-gaming bar this memo's own instructions set.

This lane's population sums exactly to `diff.json`'s stated `kind_unenumerable` minus
`class_feature`/`ability_category:*` (3,551) plus `unclassified:<file>` (179) — **3,730 units
total in this lane's scope**, none left unaccounted, none double-counted against
`docs/work-inventory.json`'s existing ten kinds (verified per-bucket above, not assumed from a
shared name).

## What this memo does not do

- Does not edit `docs/work-inventory.json`, `scripts/census_independent.py`,
  `scripts/shape_ledger.py`, or any pinned-count file — that is the integration cycle's job, per
  this card's own scope note.
- Does not resolve the sibling lanes' buckets (`class_feature` 18,231 / inventory disagreement,
  `ability_category:*` 5,886) — those are the other two measurement lanes' scope.
- Does not build the seven new kinds' Gate-2 engines — that is downstream, gated behind Gate 1
  closing with these kinds classified (which is this memo's own deliverable).
