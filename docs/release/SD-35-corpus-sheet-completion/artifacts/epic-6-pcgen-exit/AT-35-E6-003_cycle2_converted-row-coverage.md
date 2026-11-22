# AT-35-E6-003 cycle 2 — what the catalog-mode renderer unblocks, and what it does not

Cycle 1 named the twelve surviving `apps/desktop/` files' blocker as **one buildable thing**: a
catalog-mode prose path, "after which the nine `render_pcgen_desc` call sites become a lookup".
This cycle built that path and then **tried the lookup on one catalog**. The renderer works. The
lookup does not yet, and the reason is not the renderer — it is **row coverage in the converted
package**. This file is the measurement, so the next cycle scopes the converter work rather than
rediscovering it.

`correction 1789093661118-at-35-e6-003-5d341d` records the corrected claim.

## 1. The renderer — built, and proved against the live corpus

`src/rules_core/sheet_rule_catalog.rs`. It renders a converted `SheetRule`'s `prose` with **no
character in hand**: a `ProsePiece::Slot` whose `Expr` is a constant still prints its number,
and a slot standing on a character term prints **the term's own words** rather than the
evaluator's characterless `0`.

The record cycle 1 named as the motivating defect, end to end:

| path | Advanced Race Guide, *Absorbing Inhalation* |
|---|---|
| ingest format (`arg_spells.lst`) | `...contained within you for up to %1 rounds...\|CASTERLEVEL` |
| `sheet_rule::evaluate`, no character | `...for up to **0** rounds...` — a wrong number |
| `sheet_rule_catalog::catalog_prose` | `...for up to **caster level** rounds...` — the rule's words |

`decisions.md §1` allows exactly three printed forms — a final number, dice in final form, or
the rule's words. An unsettled slot is the third, and this is the module that prints it.

**The gate is corpus-wide, not a fixture** (`decisions.md §4`):
`every_unsettled_slot_in_the_live_package_renders_as_words_not_as_the_characterless_zero` loads
the real `data/sheet_rules/` directory, finds every rule whose prose carries a slot no character
settles, and asserts that each one renders **differently** with and without a character. A rule
that rendered identically would be one whose unsettled slot still reaches a catalog screen as a
number nobody computed.

```
cargo test --locked --lib sheet_rule_catalog -j 6 -- --nocapture
```

## 2. The lookup — tried on `spell_catalog`, and reverted

`spell_catalog::serve_description` was swapped from the run-time ingest-format renderer to a
book-scoped `data/sheet_rules/` lookup at all 28 of its call sites, and the desktop suite was
run. **6 of 580 tests failed. Three of the six are real losses, not moved counts**, so the swap
was reverted rather than shipped (`AGENTS.md` rule 2; `workflow-instruction.md §8` lists
RED→GREEN not preserved as non-self-healable).

```
cd apps/desktop/src-tauri && cargo test --locked -j 6
-> test result: FAILED. 574 passed; 6 failed; 0 ignored; finished in 79.80s
```

| failing test | what it says | class |
|---|---|---|
| `spell_catalog::absorbing_inhalation_reads_as_prose_rather_than_as_a_pcgen_token` | pinned the OLD behaviour ("the caster-level formula is dropped, not guessed"); the new text reads *"for up to caster level rounds"* | **moved by this cycle's own deliberate change** — self-healable, and the new expectation is the criterion's goal |
| `spell_catalog::crb_acg_and_arg_records_are_always_fully_populated` | `Nondetection (self only) has no description` | **real loss** — the converted package carries `nondetection`, not the variant row |
| `spell_catalog::apg_records_missing_a_field_are_served_with_that_field_null` | `left: 20, right: 0` — 20 APG rows lose their description | **real loss** |
| `reach_gate::bare_records_are_exactly_the_recorded_findings` | `apg/spells: 3 record(s) now reach their surface carrying only a key`: the three `Threefold Aspect (<age>)` rows | **real loss, and user-visible** — the reach gate did its job |
| `spell_catalog::no_served_spell_description_carries_raw_pcgen_syntax` | `Teleport (CRB)` and `Ancestral Memory (ISWG)` carry an unsubstituted bare `%` | **converter defect** (§3) |
| `equipment_catalog::no_catalog_serves_a_description_carrying_raw_pcgen_syntax` | the same two records, through the cross-catalog sweep | same defect |

### Row coverage, measured

Slug the compiled tables' own keys and ask whether `data/sheet_rules/<book>/<kind>/<slug>.json`
exists. Denominator is the table key population named in each row.

| population | resolved | denominator | share |
|---|---:|---:|---:|
| spells, `crb/spell_list.rs` | 664 | 664 | 664 of 664 = 100 % |
| spells, `acg/spell_list.rs` | 144 | 144 | 144 of 144 = 100 % |
| spells, `advanced_race_guide/spell_list.rs` | 93 | 93 | 93 of 93 = 100 % |
| spells, `apg/spell_list.rs` | 293 | 297 | 293 of 297 = 98.7 % |
| feats, `*/feat_data/*.rs` (4 books) | 673 | 673 | 673 of 673 = 100 % |
| equipment, `*/equipment_data/*.rs` | 2727 | 3446 | 2727 of 3446 = 79.1 % |

Re-derive:

```bash
python3 - <<'PY'
import re,os,glob
def slug(name):
    out=[];pending=False
    for ch in name:
        if ch.isascii() and ch.isalnum():
            if pending and out: out.append('_')
            pending=False; out.append(ch.lower())
        else: pending=True
    return ''.join(out)
BOOK={'crb':'core_rulebook','apg':'advanced_players_guide','acg':'advanced_class_guide',
      'advanced_race_guide':'advanced_race_guide','ultimate_equipment':'ultimate_equipment',
      'ultimate_combat':'ultimate_combat','ultimate_magic':'ultimate_magic',
      'ultimate_intrigue':'ultimate_intrigue','pathfinder_unchained':'pathfinder_unchained'}
def report(pattern, kind):
    tot=hit=0; misses=[]
    for f in glob.glob(pattern):
        book=f.split('rules_tables/')[1].split('/')[0]
        bd=BOOK.get(book)
        if not bd: continue
        keys=re.findall(r'\bkey: "((?:[^"\\]|\\.)*)"', open(f).read())
        d=os.path.join('data/sheet_rules',bd,kind)
        have=set(os.listdir(d)) if os.path.isdir(d) else set()
        for k in keys:
            tot+=1
            if slug(k.replace('\\"','"'))+'.json' in have: hit+=1
            else: misses.append(f"{book}:{k}")
    print(kind, hit, '/', tot, 'misses:', misses[:12])
report('src/rules_core/rules_tables/*/spell_list.rs','spell')
report('src/rules_core/rules_tables/*/feat_data/*.rs','feat')
report('src/rules_core/rules_tables/*/equipment_data/*.rs','equipment')
PY
```

### The shape of every miss

The misses are **variant rows**, not absences. The compiled table carries a row per printed
variant; the converter writes one record per record.

| table key | converted record present? |
|---|---|
| `Threefold Aspect (Young Adult)` / `(Adulthood)` / `(Elderly)` | `advanced_players_guide/spell/threefold_aspect.json` — one record for three rows |
| `Wall of Thorms` | `advanced_players_guide/spell/wall_of_thorns.json` — the table key carries the source's own misspelling |
| `Nondetection (self only)` | `core_rulebook/spell/nondetection.json` |
| `Special Ability ~ Amorphous ~ Armor` (and 718 siblings) | under `equipment_modifier/`, not `equipment/` — a **kind** mismatch, not an absent record |

**None of these is closable by a matcher on the live side.** `feat_catalog`'s own standing rule
says it in this repo's words: *"a shared name never implies a shared thing — a fuzzy matcher
belongs, if built at all, in the caller, verified per-match against the owning record, never
silently folded into this function."* The fix is converter-side: either the converter emits the
variant rows the tables carry, or the tables carry the converted record's id. Both are
`src/pcgen_import/` work, which is where `decisions.md §11` puts it.

## 3. Two converter prose defects, found by pointing a catalog at the converted package

Neither is visible to the standing `data/sheet_rules/` token grep
(`grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` reads `0`, and
correctly so — neither string is in that pattern set).

1. **`inner_sea_world_guide:spell:ancestral_memory`** — the converted prose carries the literal
   `(70+CASTERLEVEL)%`. **An unconverted PCGen variable name in player-facing prose.**
   `correction 1789093674266-at-35-e6-003-32a3f3`.
2. **`core_rulebook:spell:teleport`** — one of the record's three `d%` percentile-dice
   notations is written `d %`, with an inserted space, which the live leak checker's
   `is_percentile_dice_notation` no longer recognises.
   `correction 1789093674392-at-35-e6-003-3300bc`.

Re-derive both:

```bash
python3 - <<'PY'
import json,re
for p in ['data/sheet_rules/core_rulebook/spell/teleport.json',
          'data/sheet_rules/inner_sea_world_guide/spell/ancestral_memory.json']:
    txt=''.join(pc['Text'] for seg in json.load(open(p))[0]['prose']
                for pc in seg['pieces'] if isinstance(pc,dict) and 'Text' in pc)
    for m in re.finditer('%',txt): print(p, repr(txt[max(0,m.start()-70):m.start()+40]))
PY
```

## 4. What the next cycle needs, named as mechanisms

1. **Variant-row coverage** in the converter, or a converted-record id on the compiled table
   rows — the 4 APG spell keys, CRB's `Nondetection (self only)`, and the 719 equipment rows
   whose converted twin sits under `equipment_modifier/` rather than `equipment/`.
2. **The two prose defects above**, on the converter side.
3. Only then the nine `render_pcgen_desc` call sites, one catalog at a time, each with its own
   before/after row-coverage count so a loss cannot ship silently.
4. Separately, and **not** blocked on the above: the three `raw_tokens` readers
   (`race_trait_picker.rs`, `class_feature_feat_bridge.rs`,
   `reference_library_catalog.rs::mechanical_summary`) and the two run-time condition
   translators (`companion_catalog::serve_desc_condition`,
   `intelligent_item_catalog::translate_condition`), which need `SheetRule.applies`/`grants`
   rather than prose.
5. `apps/desktop/src/characterHub/raceCreationCoverage.test.ts` is blocked on a **different**
   converter gap: a `race_trait` record's `BONUS:STAT` does not convert to a `SheetRule.target`
   /`value`. `core_rulebook:race_trait:dwarf_ability_scores` carries the adjustment only in its
   `label` (`"+2 Constitution, +2 Wisdom, -2 Charisma"`); only 6 of the 217 `core_rulebook`
   `race_trait` records carry a `target` at all, and all six are the standalone `2_<ability>`
   rows. Moving that test to the converted package today would **delete** its ability-adjustment
   assertion, which is the one thing it was written to protect
   (`BONUS:STAT|CON,WIS|2` read only up to the comma, for months).

   ```bash
   python3 - <<'PY'
   import json,os
   d='data/sheet_rules/core_rulebook/race_trait'
   n=t=0
   for f in os.listdir(d):
       for r in json.load(open(os.path.join(d,f))):
           n+=1; t+= 1 if r.get('target') else 0
   print('race_trait records',n,'carrying a target',t)
   PY
   ```

## 5. A fourth defect, found by reading the converter's own count against the loader's

`sheet_rule_convert -- --check` reports `rules=69344`. The corpus-wide gate this cycle added
prints the number the **loader** actually holds: `rules=68976`. **368 fewer.**

```
python3 - <<'PY'
import json,os,collections
ids=collections.Counter()
for book in os.listdir('data/sheet_rules'):
    bd=os.path.join('data/sheet_rules',book)
    if not os.path.isdir(bd) or book.startswith('_'): continue
    for root,_,fs in os.walk(bd):
        for f in fs:
            if f.endswith('.json'):
                for r in json.load(open(os.path.join(root,f))): ids[r['id']]+=1
dups={k:v for k,v in ids.items() if v>1}
print('distinct',len(ids),'objects',sum(ids.values()),
      'ids written more than once',len(dups),'extra objects',sum(v-1 for v in dups.values()))
PY
-> distinct 68976 objects 69344 ids written more than once 305 extra objects 368
```

**305 rule ids are written more than once, and every one is a `#natural<N>` sibling suffix** —
`inner_sea_world_guide:monster:treerazer#natural0` appears three times,
`bestiary_3:template:imperial_dragon_attacks_colossal#natural0` twice. The converter's sibling
suffix collides for a record carrying several natural attacks at the same index, and
`SheetRulePackage::insert_rule` keeps the last. Nothing reports it: the converter counts objects
written, the loader counts distinct ids, and no gate compares the two.

`correction 1789094375625-at-35-e6-003-af8026`. **A receipt quoting `rules=69344` as the live
rule population is quoting the wrong denominator** — the live side holds 68,976.
