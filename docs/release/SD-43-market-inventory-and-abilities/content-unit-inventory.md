---
canonical: true
owner: operator
bundle_id: SD-43
slug: market-inventory-and-abilities
canonical_branch: tranche-43
target_version: 0.43.0
date: 2026-11-22
status: planning — every figure carries its command; re-run on tranche-43 @ 75e95b30c2 on 2026-11-22 unless marked "operator's screen" or "to be measured"
---

# SD-43 Content Unit Inventory

The units SD-43 must close, each with the card that closes it and, where it is a number, the command that
produced it and its denominator. A figure marked **to be measured** has no value yet and is not a target; the
named card produces it. Counts use `awk` or Python, never `grep -o`.

## §1 Intake items → cards (the closure unit is the item; an item closes when every card listed is `complete`)

| Item | Unit of work | Cards | Count of cards |
|---|---|---|---|
| 0 | graphify after SD-42 (steps 1–3 already done: `git worktree list` → 1 line; `ls site` → absent; codex-web updated 2026-11-21) | C2 | 1 |
| 1 | campaign folder relocation with content and skeleton | E6.1 | 1 |
| 2 | rest and recovery text: print, Biography tab, override | E0.5, E2.4, E2.5 | 3 |
| 3 | House Rules menu, six rules, per character and system | E0.5 (shared), E6.2, E6.3, E6.4 | 3 (+ E0.5) |
| 4 | custom items export/import | E6.5 | 1 |
| 5 | one-column gear list; container truth | E0.4, E1.1, E1.2 | 3 |
| 6 | double-click / Enter | E1.4 | 1 |
| 7 | whole-box drop target | E1.3 | 1 |
| 8 | Market button; Print and pricing leave the panels | E1.7 | 1 |
| 9 | sell window | E1.6 | 1 |
| 10 | stats in the Market | E1.9 | 1 |
| 11 | filters, no cap, counts | E0.2, E1.10, E1.11 | 3 |
| 12 | sell values; container sale | E1.5, E1.8 | 2 |
| 13 | report API client | E5.1 | 1 |
| 14 | Update menu layout | E5.3 | 1 |
| 15 | Anonymous selector + Handle | E5.2 | 1 |
| 16 + 19 | Developer tab slimmed; diagnostic moved | E5.4 | 1 |
| 17 + 21 | fixed layout (PF, SF, CPR); Show Progression | E2.1, E2.2 | 2 |
| 18 | Open campaign button | E2.3 | 1 |
| 20 | placeholder names; one all-kinds test; approval | E0.3, E4.1, E4.G, E4.2 | 4 |
| 22 / 22a / 22b | the Abilities tab (§2 below) | E0.1, E3.1–E3.11, E3.MC | 13 |
| 23 | minor = SD number; pointer | C1 (pointer), E7.3 (final notes, counted under closure); version files done in `147a3f71eb` (14 files: `git show --name-only --format= 147a3f71eb \| awk 'NF' \| awk 'END{print NR}'` → 14) | 1 |
| — | merge checks, ui-smoke, closure | E1.MC, E2.MC, E5.MC, E6.MC, E7.1–E7.4, C0 | 9 |

Sum of distinct cards: 56 (`awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'`). E0.5 serves items 2
and 3 and is counted once; E3.MC is counted under item 22.

## §2 The seven Abilities sub-tabs (items 22, 22a, 22b; `decisions.md §15`–`§20`)

The operator's frame is PCGen's seven sub-tabs, in this order. Each must, by E3.MC: show the picks granted per level
with their source, what can be selected (search, Qualified filter), what is selected, remaining picks per pool, the
rule text; enforce grants and prerequisites in the engine; save on the character; print. The Abilities tab is the renamed
Feats tab, with Feats as one of its seven sub-tabs (operator ruling Q-22.3, 2026-11-22; E3.1).

| Sub-tab | Today (read from code, `decisions.md §15`) | Engine card(s) | Records (to be measured by E0.1) |
|---|---|---|---|
| Feats | picker, `feats.budget.*`, overrides — unchanged | none (kept) | E0.1 |
| Racial Abilities | read-only list; alternates only at creation | E3.6 | E0.1 |
| Traits | add/remove, no budget | E3.2, E3.3 | E0.1 |
| Archetype | no UI; engine `choice:archetype` | E3.7 | E0.1 |
| Background | none found | E3.9 (data-gap sentence if E0.1 finds no records) | E0.1 |
| Class Features (every class- or archetype-granted choice + favored class; §2a) | read-only lists for two pools; nothing recorded | E3.4, E3.5, E3.7, E3.8 (+ discovered cards per E0.1 mechanism) | denominator: E0.1's class-granted-choice total (§2a). Reference only: 1,913 groups / 16,348 group-qualified records / 7,328 text-only (`python3 scripts/census_class_feature_pool_population.py`) — may exclude level-1 picks (open check) |
| Misc | unknown | E3.9 | E0.1 |

### §2a The Class Features unit is every class-granted choice (intake §22c, operator 2026-11-22, `c9994e01bd`)

**The unit of work is every choice a class or archetype grants the player** — talents, tricks, powers,
discoveries, hexes, arcana, revelations, mercies, bloodlines, domains, schools, mysteries, orders and every other
shape the books define. The pools named below and in the intake are **examples, not the denominator.**

| Figure | Value | Source |
|---|---|---|
| class-granted choices, total and by mechanism (catalog pool group / level-1 pick / archetype-granted / other) | **to be measured** by E0.1 (`abilities-census.json` `class_granted_choices`, two implementations agreeing) | E0.1 |
| reachable in the UI (selectable + recorded after reload), per mechanism | **to be measured** at E3.MC; must equal E0.1's per-mechanism total minus choices covered by discovered cards, and every discovered card closes before E7.2 | E3.MC |
| not reachable | a number to close, never an exemption (target 0 at closure) | E3.MC |

**Open check (intake §22c, not yet run):** whether choices that are not modeled as a catalog pool today (e.g. a
bloodline, domain, school, mystery or order picked at level 1) are inside or outside the 1,913-group /
16,348-record figure below. That figure counts `' ~ '`-grouped `class_feature` records only, so it is **not**
assumed to be the denominator; E0.1 answers the question and counts any outside mechanism separately.

Per-pool reference counts (examples only; `key.split(' ~ ')[0]` over `glob('data/corpus/*/class_feature/**/*.json', recursive=True)`,
`decisions.md §31` C-2): Rage Power 170, Rogue Talent 130, Discovery 129, Magus Arcana 57, Arcanist Exploit 46,
Slayer Talent 46, Ninja Trick 45 (re-run 2026-11-22 for Ninja Trick: 45), Mercy 15. **Data-gap list:** to be
measured by E0.1 (`artifacts/epic_0/abilities-census.json`); `forward-scope-register.md §3` cites it.
`ABILITYCATEGORY` lines at the pin (intake: 2,616 lines / 2,430 names): **not re-derived**, E0.1 re-runs it.

## §3 Placeholder item names (item 20; `decisions.md §22`)

Predicate: `re.search(r'[\[\]%]|\bNAME\b', name)` on `data.name`. Re-run 2026-11-22:

```bash
python3 -I -c "
import json,re,glob
pat=re.compile(r'[\[\]%]|\bNAME\b')
def scan(files):
    recs=named=hits=0; kinds={}
    for f in files:
        d=json.load(open(f))
        if not (isinstance(d,dict) and isinstance(d.get('data'),dict)): continue
        recs+=1; nm=d['data'].get('name')
        if not isinstance(nm,str): continue
        named+=1
        if pat.search(nm): hits+=1; k=f.split('/')[3]; kinds[k]=kinds.get(k,0)+1
    return recs,named,hits,kinds
print(scan(glob.glob('data/corpus/*/equipment/*.json')))
print(scan(glob.glob('data/corpus/*/*/**/*.json',recursive=True)))"
```

| Figure | Value | Denominator |
|---|---|---|
| equipment placeholder names | **111** | of **3,805** equipment records |
| all-kinds placeholder names | **201** (equipment 111, class_feature 72, ability 16, skill 1, feat 1) | of **51,474** records with a `data` object (48,941 of which have a string `data.name`; 51,523 JSON files) |
| distinct equipment name strings | 18 | of the 111 hits (`decisions.md §22`) |
| by book (equipment) | Advanced Player's Guide 60, Ultimate Psionics 50, Ultimate Combat 1 | of 111 (`decisions.md §22`, not re-run here) |
| `label` / `OUTPUTNAME`, `_settled`, `rules_tables`, `resources/corpus_bundle/` | **to be measured** | E0.3 prints each, zero or not |

## §4 Market counts (item 11)

| Figure | Value | Source |
|---|---|---|
| Magic / Gear / Weapons shown | 3,680 / 1,624 / 480 | **operator's screen** — not derived by us; E0.2's test must reproduce them or stop |
| Armor / Shields shown | "under 200" each | operator's screen; exact values **to be measured** by E0.2 |
| `built_catalog().entries.len()` and dropped rows by reason | **to be measured** | E0.2 (`cargo test --test market_counts -- --nocapture`) |
| `data/equipment_types.json` names (a different set from the Market's source) | 10,674 | `python3 -c "import json;print(len(json.load(open('data/equipment_types.json'))['types']))"` |
| `MARKET_ROW_LIMIT` | 200 (`itemPickerFilter.ts:362`) | `grep -n MARKET_ROW_LIMIT apps/desktop/src/characterHub/itemPickerFilter.ts` |

## §5 Rest and recovery (item 2)

| Figure | Value | Command |
|---|---|---|
| corpus records naming a rest phrase | 23 (class_feature 14, ability 2, equipment 2, companion 1, feat 1, monster_ability 1, race_trait_generic 1, spell 1) | `grep -rlis "hours of rest\|night's rest\|8 hours of sleep\|eight hours of rest" data/corpus --include=*.json \| awk -F/ '{print $4}' \| sort \| uniq -c` |
| class records (E0.5's denominator; all class records, base and others) | 168 | `ls data/corpus/*/class/*.json \| awk 'END{print NR}'` |
| rest-table rows | **to be measured** | E0.5 lists, E2.4 writes; every row cited |

## §6 House Rules (item 3)

| Figure | Value | Command |
|---|---|---|
| rules requested | 6 | intake item 3 |
| already flags in `Overrides` | 4 of the 9 flags (`ignore_slot_limits`, `feats_without_prerequisites`, `weapons_without_proficiency`, `armor_without_proficiency`) | `sed -n '/pub struct Overrides/,/^}/p' apps/desktop/src-tauri/src/character_custom.rs \| grep -cE 'pub [a-z_]+:'` → 9 |
| new | 2 (HP method, Ambidextrous) | `decisions.md §11` |
| HP methods | 7 | intake item 3 |
| rules applying to SF / CPR | **to be measured** (E0.5) / 0 (operator) | E0.5 |

## §7 Release

| Figure | Value | Command |
|---|---|---|
| version files at `0.43.0` | package.json, tauri.conf.json, Cargo.toml, Cargo.lock entry | `acceptance-and-verification.md` G-R |
| publish runs so far (the next build is `0.43.<run>`, `<run>` ≥ 169) | newest run 168 | `gh run list -w publish-tester-release.yml -L 1 --json number` |
| `current-release.json` tranche | 22 (C1 makes it 43) | `python3 -c "import json;print(json.load(open('docs/release/current-release.json'))['tranche'])"` |
| CI secret `CODEX_REPORT_KEY` | present (1), created 2026-11-22 | `gh secret list -R electricm0nk/codex \| awk '$1=="CODEX_REPORT_KEY"' \| awk 'END{print NR}'` |

## §8 What this inventory does not count

The engine's per-sub-tab gap (E0.1); pool-size token shapes and `PRE*` shapes (E0.1, E3.4, E3.5); the Market's
dropped rows by reason (E0.2); which gear symptoms are data bugs (E0.4); the number of ui-smoke rows E7.1 adds;
any Starfinder or Cyberpunk RED ability data (out of scope).
