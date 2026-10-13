---
canonical: true
owner: operator
bundle_id: SD-38
slug: cyberpunk-red
canonical_branch: tranche-18
target_version: 0.18.0
date: 2026-10-13
status: draft — authored under operator-away defaults; operator rulings on licence and scope are on record
---

# SD-38 Content-Unit Inventory — Cyberpunk RED corebook v1.22

This file holds the bundle's **figures of record** (§2). Every other file cites them by ID
(`CUI F-n`) instead of restating a number without its command. It also holds the kind table that
`data/cyberpunk-red/corpus/MANIFEST.json` pins (§3), the garbled-page list (§4), the UNCLEAR list
with the ruling that resolves each item (§5), and the seed roster (§6).

**Source of every figure.** Seven read-only inventories written on 2026-10-13 from the PDF text.
They are copied verbatim into this package at `artifacts/cycle_0/inventory/` (`roles.md`,
`creation.md`, `skills.md`, `gear.md`, `cyberware.md`, `combat.md`, `netrunning.md`, plus the
scripts and data they run: `gen.py`, `sr.py`, `skills.tsv`, `stat_tables.md`). This file
**transcribes** their counts and commands. It does not re-derive them. Where a command named a
session scratch path, the path below names the package copy instead; the command is otherwise
unchanged.

**Variables used in commands.**

```bash
P=/home/ubuntu/workspace/inbox/cyberpunk-red/pages      # 459 per-page text files, outside the git tree
PDF=/home/ubuntu/workspace/inbox/cyberpunk-red/cpr-corebook-v122.pdf
INV=docs/release/SD-38-cyberpunk-red/artifacts/cycle_0/inventory
```

Commands that count page lines run with `cd "$P"` first. Commands that count inventory rows run from
the repo root. **No source file is ever copied into the git tree** (`decisions.md §11`): the corpus
JSON is transcribed from these pages, and only the JSON ships.

---

## §1 The page-offset rule

**`pNNN` = printed page number + 1.** Every page cite in this package is a `pNNN` file name.
In-book cross-references ("see page 45") are printed numbers: add 1 to find the file.

Checked by four inventories independently:

| Check | Command or observation | Inventory |
|---|---|---|
| `p100` prints `99` | `grep -E '^\s*[0-9]{2,3}\s*$' p100.txt \| tail -1` → `99` | gear §0 |
| `p113` prints `112`; `p359` prints `358` | footer read | cyberware header |
| `p169` prints `168` | footer read | combat header |
| `p204` prints `203` | footer read | netrunning header |
| Printed "Rockerboy page 54" is `p055`; "Reputation 193" is `p194`; "Getting Better 408" is `p409` | cross-reference read | creation header |

Blank files: `p107` (0 bytes, cyberware §6) and `p459` (0 bytes, `ls -la $P/p459.txt`). Every
corpus record carries `source_page` as a `pNNN` string (`decisions.md §11`).

---

## §2 Figures of record

Columns: **Value** as the inventory reported it; **Predicate** what is counted; **Command A** /
**Command B** the implementations. `[CMD]` = re-derivable by the command. `[READ]` = counted by
reading a page, no command (treat as an estimate until the owning lane re-derives it with two
implementations). `[HAND]` = hand count, **estimate**. Inside a table cell `\|` means `|`: unescape
before running.

### §2.1 Roles (`roles.md`)

| ID | Figure | Value | Predicate | Command A | Command B / status |
|---|---|---|---|---|---|
| F-1 | Roles | **10** | Role blurb pages, one per Role heading | `for r in ROCKERBOY SOLO NETRUNNER TECH MEDTECH MEDIA EXEC LAWMAN FIXER NOMAD; do grep -lE "^\s*$r\s*$" p0[2-4]*.txt; done \| wc -l` | p030 sentence "There are ten Roles" [READ] |
| F-2 | Role Ability stubs in blurbs | **10** | `▶ Role Ability:` lines | `grep -E '▶ Role Ability:' p0[3-4]*.txt \| awk 'END{print NR}'` | — |
| F-3 | Role Ability rule sections | **10** | `(<Role> Role Ability)` headings, p145–p162; the Lawman heading extracts as `(L awman Role Ability)` (p159), and p157 has one prose mention in an Exec team stat line (`Skills at +2: Interface (Netrunner Role Ability)`) | `cat p14[5-9].txt p15*.txt p16[0-2].txt \| awk '!/Skills at/{n+=gsub(/\((Rockerboy\|Solo\|Netrunner\|Tech\|Medtech\|Media\|Exec\|L ?awman\|Fixer\|Nomad) Role Ability\)/,"&")} END{print n}'` → 10 (auditor, 2026-10-13) | `python3 -c "import re,glob;fs=sorted(glob.glob('p14[5-9].txt')+glob.glob('p15?.txt')+glob.glob('p16[0-2].txt'));print(len({m.group(1).replace(' ','') for f in fs for l in open(f) if 'Skills at' not in l for m in re.finditer(r'\((Rockerboy\|Solo\|Netrunner\|Tech\|Medtech\|Media\|Exec\|L ?awman\|Fixer\|Nomad) Role Ability\)',l)}))"` → 10 distinct Roles. **Correction:** the inventory's command (`grep -E '\((…\|Lawman\|…) Role Ability\)' … \| wc -l`) also prints 10, but as 9 headings + the p157 prose line; it misses Lawman. Do not use it |
| F-4 | Solo Combat Awareness abilities | **6** | `▶` + ability-name headings on p147 (five of six sit mid-line, right of a sidebar column) | `awk '{n+=gsub(/▶ (Damage Deflection\|Fumble Recovery\|Initiative Reaction\|Precision Attack\|Spot Weakness\|Threat Detection)/,"&")} END{print n}' p147.txt` → 6 (auditor, 2026-10-13) | `python3 -c "import re;print(len(re.findall(r'▶ (Damage Deflection\|Fumble Recovery\|Initiative Reaction\|Precision Attack\|Spot Weakness\|Threat Detection)',open('p147.txt').read())))"` → 6. **Correction:** the inventory's line-anchored `grep -E '^\s*▶ (…)' p147.txt \| wc -l` prints **1**, not 6 |
| F-5 | Nomad vehicle upgrades | **19** | named upgrade labels p164–p166 | `cat p164.txt p165.txt p166.txt \| grep -oE '(Armored Chassis\|Bulletproof Glass\|Communications Center\|NOS\|Onboard Flamethrower\|Onboard Machine gun\|Seating Upgrade\|Security Upgrade\|Smuggling Upgrade\|Heavy Chassis\|Onboard Rocket Pod\|Vehicle Heavy Weapon Mount\|Onboard Melee Weapon\|Hover Upgrade\|AV-4 Engine Upgrade\|Combat Plow\|Enhanced Interface Plug Integration\|Deployable Spike Strip\|Housing Capacity):' \| awk '{c++}END{print c}'` | manual list 9+3+1+2+1+1+1+1 = 19 (agrees; the inventory flags its own `grep -o`) |
| F-6 | Tech Upgrade Expertise options | **10** | bullets on p149 | `grep -c '•' p149.txt` | — |
| F-7 | Interface Abilities | **9** | ability rows on p148 | `grep -E '^\s*(Backdoor\|Cloak\|Control\|Eye-Dee\|Pathfinder\|Scanner\|Slide\|Virus\|Zap)\s' p148.txt \| wc -l` | `awk '/^  (Backdoor\|Cloak\|Control\|Eye-Dee\|Pathfinder\|Scanner\|Slide\|Virus\|Zap) /{n++}END{print n}' p148.txt` (netrunning §1, "expect 9") |
| F-8 | Exec Team Member stat rows | **30** | 5 jobs × 6 rows × 9 STATs | `grep -E '^\s*[1-6]\s+[0-9]\s+[0-9]\s+[0-9]\s+[0-9]\s+[0-9]\s+[0-9]\s+[0-9]\s+[0-9]\s+[0-9]\s*$' p156.txt p157.txt p158.txt \| wc -l` | — |
| F-9 | Exec Team Member jobs | **5** | `▶ Company ` headings | `grep -E '▶ Company ' p156.txt p157.txt p158.txt \| wc -l` | — |
| F-10 | Lawman Backup rank bands | **6** | `▶ Backup Rank` rows | `grep -cE '^\s*▶ Backup Rank ' p159.txt p160.txt` → 3 + 3 | — |
| F-11 | Fixer Operator rank bands | **6** | `▶ Operator Rank` rows | `grep -cE '▶ Operator Ranks? [0-9]' p161.txt p162.txt` → 3 + 3 | — |
| F-12 | Media Credibility rank bands | **6** | `▶ Credibility Rank` rows | `grep -cE '▶ Credibility Ranks? [0-9]' p153.txt p154.txt` → 5 + 1 | — |
| F-13 | Rockerboy Charismatic Impact rank bands | **6** | `▶ Charismatic Impact Rank` rows | `grep -cE '▶ Charismatic Impact Ranks? [0-9]' p145.txt p146.txt` → 2 + 4 | — |
| F-14 | Maker DV/Time rows | **7** | price-category rows on p150 | `grep -E '^\s*(Cheap/Everyday\|Costly\|Premium\|Expensive\|Very Expensive\|Luxury\|Super Luxury)\s' p150.txt \| wc -l` | — |
| F-15 | Media Rumor table rows | **4** | Vague, Typical, Substantial, Detailed | [READ] p152 (interleaved; `grep -cE 'Rumor' p152.txt` → 10 is **not** the row count) | E1.1 re-derives |
| F-16 | Medtech pharmaceuticals | **5** | Antibiotic, Rapidetox, Speedheal, Stim, Surge | [READ] p151 (the anchored grep returns 1: effect text wraps) | E1.1 re-derives |
| F-17 | Medtech Cryosystem levels | **5** | levels 1–5 | [READ] p151 (`grep -E '^\s*[1-5]\s+With' p151.txt \| wc -l` → 2: rows wrap) | E1.1 re-derives |
| F-18 | Nomad Motorpool vehicle names | **14** | 4+3+3+4 across 4 rank bands | [READ] p163 | E1.1 re-derives |
| F-19 | Page files | **459** | files under `$P` | `ls /home/ubuntu/workspace/inbox/cyberpunk-red/pages \| wc -l` | re-run 2026-10-13 by the package author → 459 |

### §2.2 STATs, creation, Lifepath, reputation, improvement (`creation.md`)

`gen.py` and `sr.py` are at `$INV/`. `gen.py` also writes a side file `stat_tables.md` at its
original scratch path; that file is copied at `$INV/stat_tables.md`.

| ID | Figure | Value | Predicate | Command A | Command B / status |
|---|---|---|---|---|---|
| F-20 | STATs | **10** | INT REF DEX TECH COOL WILL LUCK MOVE BODY EMP | column header of the template tables p075–p078 (`python3 -I $INV/gen.py` reads 10 columns) | p073–p074 STAT list [READ] |
| F-21 | Streetrat template rows incl. duplicates | **110** | rows of 11 integers on p075–p078 | `cat p075.txt p076.txt p077.txt p078.txt \| awk '/^ *[0-9]+( +[0-9]+){10} *$/ {c++} END{print c}'` | includes the 10-row Edgerunner example on p078 (a copy of Solo) |
| F-22 | Unique Streetrat template rows | **100** | 10 Roles × 10 rolls | `python3 -I $INV/gen.py` → `rows=[10,10,10,10,10,10,10,10,10,10]` | 110 − 10 duplicates (F-21) |
| F-23 | HP table cells | **126**, **0** mismatches against `10 + 5 × ceil((BODY+WILL)/2)` | 9 rows (WILL 2–10) × 14 columns (BODY 2–15), p080 | `python3 -I $INV/gen.py` | row count `awk '/^ *[0-9]+( +[0-9]+){14} *$/ {c++} END{print c}' p080.txt` → 9 |
| F-24 | Skills | **66**, of which **7** are (x2) | dot-leader skill entries p083–p086 | `cat p083.txt p084.txt p085.txt p086.txt \| awk '{n=gsub(/\.\.\.\.+ *(INT\|REF\|DEX\|TECH\|COOL\|WILL\|EMP)/,"&"); t+=n; x+=gsub(/\(x2\)\.\.\.\.+/,"&")} END{print t, x}'` → `66 7` | `sed -e 's/  */ /g' p121.txt \| grep -o -E "[A-Za-z/& ]+(\(x2\))? \((INT\|REF\|DEX\|TECH\|COOL\|WILL\|EMP)\)" \| wc -l` → 66 (`grep -o`: AGENTS.md caution; agrees with A and with F-37) |
| F-25 | Streetrat Role skill totals | **86** for each of **10** Roles | Level × 1, or × 2 for (x2), Cultural Language excluded | `python3 -I $INV/sr.py` prints `86` ten times | proves sums only, not column placement (`creation.md §2d`) |
| F-26 | General Lifepath printed tables | **15** | roll-instruction lines p046–p054 | `for p in 046 047 048 049 050 051 052 053 054; do grep -o -E "Roll 1d(6\|10)( and subtract 7)?\|1d10 and subtract 7\|1d10 or choose\|Roll 1d10" $P/p$p.txt \| wc -l; done` → `1 1 2 3 1 2 2 2 1` | 17 listed rows, 19 d10 columns [HAND] |
| F-27 | Role-based Lifepath rolled tables | **52** | `Roll 1d6`/`Roll 1d10` lines p055–p070 | `n=0; for p in 055 056 057 058 059 060 061 062 063 064 065 066 067 068 069 070; do c=$(grep -o -E "Roll 1d(6\|10)" $P/p$p.txt \| wc -l); n=$((n+c)); done; echo $n` | per page `creation.md §3b` |
| F-28 | General Lifepath rows | **186** (estimate) | 18 columns × 10 + Sweet Revenge 6 | [HAND] | E1.2 re-derives with two implementations |
| F-29 | Role Lifepath rows | **336** (estimate) | Rockerboy 28, Solo 24, Netrunner 36, Tech 40, Medtech 34, Media 24, Exec 40, Lawman 30, Fixer 34, Nomad 46 | [HAND] | E1.2 re-derives |
| F-30 | Basic skills | **13** | p086 list | [HAND] | same list p089, p091 |
| F-31 | Role skill lists | **10 × 20 = 200** rows (13 Basic + 7 Role) | p087–p090 | [HAND] | F-25 sums agree |
| F-32 | Complete Package STAT-point table | **5** rows (50/62/70/75/80) | p079 | [READ] | `echo $((5+7+8+3+6+7+6+8+8+4))` → 62 (worked example) |
| F-33 | IP cost table | **3** rows × **10** ranks | Typical 20N, Difficult 40N, Role Ability 60N | [READ] (`creation.md` cites p412, `roles.md` cites p411: **E1.2 reads both pages and records which holds the table**) | — |
| F-34 | Playstyle IP award table (GM) | **8** rows × **5** columns = 40 cells | p411–p412 | [READ]; cell text garbled | print-only |
| F-35 | Reputation levels | **10** | p194 "Who Knows About You" rows | [READ] | combat §8 lists the same 10 |
| F-36 | "Running Out of Cash" catches (GM option) | **7** | p120 | [HAND] | print-only |
| F-37 | Starting-kit Roles | **10** | `Weapons/Armor` column headers on p099 | `grep -o 'Weapons/Armor' p099.txt \| wc -l` → 10 (`grep -o`, single page; gear §8c) | 10 Role columns on p104 and p118 [READ] |

### §2.3 Skills (`skills.md`)

| ID | Figure | Value | Predicate | Command A | Command B / status |
|---|---|---|---|---|---|
| F-38 | Skill rows | **66** | one `Category\|Skill\|STAT` row per skill | `wc -l < $INV/skills.tsv` → 66 | per-skill check: `grep -cE "<name>( \(x2\))? *\.{3,} *<STAT>"` over p083–086 and over p132–143 → exactly 1 hit each, 66 of 66 (`skills.md §1`). A generic dot-leader regex finds only 59 of 66; do not count with it |
| F-39 | Skills per category | Awareness 5, Body 6, Control 4, Education 16, Fighting 4, Performance 2, Ranged Weapon 5, Social 9, Technique 15 (= 66) | 9 categories | `awk -F'\|' '{c[$1]++} END{for(k in c) print k, c[k]}' $INV/skills.tsv` | `creation.md §4` [HAND], same values |
| F-40 | Skills per linked STAT | COOL 8, DEX 8, EMP 2, INT 20, REF 9, TECH 16, WILL 3 (= 66) | column 3 | `awk -F'\|' '{print $3}' $INV/skills.tsv \| sort \| uniq -c` | — |
| F-41 | (x2) skills | **7** | `(x2)...` entries | `cat p083.txt p084.txt p085.txt p086.txt \| awk '/\(x2\)\.\.\./{n++}END{print n}'` | F-24 second count |
| F-42 | Difficulty Value rows | **7** (9/13/15/17/21/24/29) | p130 | [READ] (`grep -cE "^ *(Simple\|Everyday\|Difficult\|Professional\|Heroic\|Incredible\|Legendary) " p130.txt` returns ≥ 7; values paired by reading) | — |
| F-43 | Negative skill-modifier examples | **10** | p131 | [READ] | print-only |

### §2.4 Weapons, armour, gear, Night Market (`gear.md`)

| ID | Figure | Value | Predicate | Command A | Command B / status |
|---|---|---|---|---|---|
| F-44 | Price categories | **8** | p386 table rows | printed sentence "eight Price Categories" (p386) [READ] | 8 table rows [READ] |
| F-45 | Melee weapon types | **4** | Light/Medium/Heavy/Very Heavy | `grep -cE '^\s+(Light Melee\|Medium Melee\|Heavy Melee\|Very Heavy)\s' p093.txt p341.txt` → 4 and 4 | `grep -c -E "[0-9]d6 +[12] +(YES\|NO)" p176.txt` → 4 (combat §9) |
| F-46 | Ranged weapon types | **11** | `Alt. Fire Modes` lines | `grep -c 'Alt. Fire Modes' p095.txt p342.txt` → 11 and 11 | `grep -c "Alt. Fire Modes" p172.txt` → 11 (combat §9) |
| F-47 | Exotic weapons | **14** | price lines on p096 | `awk '/[0-9]eb \(/{c++} END{print c}' p096.txt` → 14 | `cat p348.txt p349.txt p350.txt \| awk '/^ *▶ /{c++} END{print c}'` → 16 = 14 + Smoke/Teargas ammo headings |
| F-48 | Ammunition types | **13** | ammo table rows p346 | `grep -cE '^\s+[A-Z][A-Za-z-]+ Ammunition\s+[0-9]' p346.txt` → 13 | `cat p346.txt p347.txt p348.txt \| awk '/^ *▶ .*Ammunition/{c++} END{print c}'` → 13 |
| F-49 | Weapon attachments | **8** | `▶` headings p344–p345 | `cat p344.txt p345.txt \| awk '/^ *▶ (Bayonet\|Drum\|Extended\|Grenade\|Infrared\|Shotgun\|Smartgun\|Sniping)/{c++} END{print c}'` → 8 | — |
| F-50 | Clip Chart rows | **10** | p345 | `sed -n '/Clip Chart/,/^ *Ammunition/p' p345.txt \| grep -cE '^ +(Medium\|Heavy\|Very\|SMG\|Shotgun\|Assault\|Sniper\|Grenade\|Rocket)'` → 10 | — |
| F-51 | Weapon-quality cost rows | **3** (50/100/500eb standard) | p343 | [READ] | brand example rows 12 [READ] (print-only) |
| F-52 | Armour types | **8** SP-bearing; **9** rows with the Bulletproof Shield | p351 (also p098, p186, identical numbers) | `sed -n 1,18p p351.txt \| awk '/[0-9]eb ?\(/{c++} END{print c}'` → 8 (misses the Shield) | `grep -E " (None\|-[24] REF)" p186.txt \| wc -l` → 8 (combat §9); Shield row by reading → 9 |
| F-53 | Gear, starter table | **47** | price strings p101–p102 | `awk '{n=gsub(/[0-9]eb ?\(/,"&"); c+=n} END{print c}' p101.txt p102.txt` → 47 | 53 − 6 extras (F-54) = 47 |
| F-54 | Gear, Master Gear List | **53** | price strings p352 | `awk '{n=gsub(/[0-9]eb ?\(/,"&"); c+=n} END{print c}' p352.txt` → 53 | every name found in p353–p356 by `grep -cF` (no "NO DESC") |
| F-55 | Fashion style families | **10**; **90** price cells | p357 (also p103) | `awk '/^ +(Bag Lady\|Gang Colors\|Generic Chic\|Bohemian\|Leisurewear\|Nomad\|Asia Pop\|Urban Flash\|Businesswear\|High Fashion)/{c++} END{print c}' p357.txt` → 10 | 10 × 9 columns = 90 (arithmetic) |
| F-56 | Street drugs | **5** | p358 header table | `sed -n 1,12p p358.txt \| awk '/(Costly\|Everyday\|Cheap)\)$/{c++} END{print c}'` → 5 | `cat p228.txt p229.txt p230.txt \| grep -c "Cost per Dose"` → 5 (combat §9) |
| F-57 | Services and Entertainment | **33** | p377 | `awk '/eb \(/{c++} END{print c}' p377.txt` → 33 | — |
| F-58 | Lifestyle tiers | **4** | p378 | `grep -cE '^\s+(Kibble\|Generic\|Good\|Fresh)' p378.txt` (approximate) | read 4 |
| F-59 | Real-estate rows | **11** | p379 | `sed -n 1,16p p379.txt \| awk '/^ +(Living\|Cube\|Cargo\|Studio\|Two-\|Corporate\|Upscale\|Luxury)/{c++} END{print c}'` → 11 | — |
| F-60 | Vehicles | **14** (5 land, 4 sea, 5 air) | p191–p192 | `cat p191.txt p192.txt \| grep -cE '[0-9]+ MOVE'` → 14 | `grep -c -E "[0-9]+ MOVE" p191.txt p192.txt` → 9 + 5 (combat §9) |
| F-61 | Night Market generator (GM) | type table **6** rows; d100 table **20** rows × 6 columns = 120 cells | p339–p340 | `cat p339.txt p340.txt \| grep -oE '^ *[0-9]+-[0-9]+' \| tr -d ' ' \| wc -l` → 20 (`grep -o`; awk run also printed 20) | print-only |

### §2.5 Cyberware (`cyberware.md`)

| ID | Figure | Value | Predicate | Command A | Command B / status |
|---|---|---|---|---|---|
| F-62 | Cyberware items | **96** | numbered rows of the inventory's item table | `awk '/^[\|] [0-9]+ [\|]/{n++} END{print n}' $INV/cyberware.md` → 96 | per-category sum (F-63) = 96 |
| F-63 | Items per category | Fashionware 7, Neuralware 12, Cyberoptics 13, Cyberaudio 12, Internal 13, External 4, Cyberlimbs 30 (arm 19, leg 7, limb options 4), Borgware 5 | column 4, grouped: the sub-label in parentheses dropped, every `Cyberarm…`/`Cyberleg…`/`Cyberlimb…` label counted as Cyberlimbs | `awk -F'\|' '/^[\|] [0-9]+ [\|]/{k=$4; gsub(/^ +\| +$/,"",k); sub(/ \(.*/,"",k); if(k ~ /^Cyber(arm\|leg\|limb)/) k="Cyberlimbs"; c[k]++} END{for(k in c) print k, c[k]}' $INV/cyberware.md` → the 8 values at left (auditor, 2026-10-13) | **Correction:** the inventory's ungrouped `awk … {c[$4]++}` prints 18 raw labels (e.g. `Neuralware (Chipware) 6`), not the 8 figures; the arm/leg split 19/7 is not printed by either command [HAND] |
| F-64 | Items per Humanity-Loss preset | 0 = 12, 2 = 22, 3 = 25, 7 = 26, 14 = 11 (= 96) | column 7 | `awk -F'\|' '/^[\|] [0-9]+ [\|]/{print $7}' $INV/cyberware.md \| sort \| uniq -c` | — |
| F-65 | Cyberware types | **8** | p111 "There are 8 types of Cyberware." | [READ] | — |
| F-66 | Role cyberware packages, HL sums | Rockerboy 9, Solo 14, Netrunner 14, Tech 12, Medtech 12, Media 10, Lawman 10, Exec 12, Fixer 16, Nomad 14 | p118 printed sums vs item presets | preset arithmetic, `cyberware.md §4` table (each row sums to its printed value) | E2.5 parity fixture |
| F-67 | Therapy rows | **3** (Standard, Extreme, Addiction) | p231 | [READ] | — |
| F-68 | Cyberpsychosis bands | **5** | p233 | [READ] | print-only |
| F-69 | Trauma Humanity-Loss rows | **4** | p232 | [READ] | print-only |

### §2.6 Combat, health, vehicles (`combat.md`)

| ID | Figure | Value | Predicate | Command A | Command B / status |
|---|---|---|---|---|---|
| F-70 | Combat actions | **18** (estimate) | rows of p169–p170 "Combat Actions in Brief" | [READ]; a shell label check matched 17 | E1.6 re-derives with two implementations |
| F-71 | Single-shot range DV table | **8** rows × **8** bands = 64 cells (11 N/A) | p174 | [READ] (a row regex returned 9 including a wrapped label) | E1.6 checks against the page image (D-1) |
| F-72 | Autofire DV table | **2** rows × **5** bands | p174 | [READ] | — |
| F-73 | Brawling / Martial Arts damage bands | **4** each | p177, p179 | [READ] | — |
| F-74 | Martial Arts forms / special moves | **4** forms; **9** moves (1 shared + 4 × 2) | p180–p181 | [READ] (estimate) | E1.6 re-derives |
| F-75 | Cover materials | **6** (examples 24, estimate) | p183 (p184 examples) | awk over p183 (the inventory did not record the command: treat as [READ]) | — |
| F-76 | Critical injuries | **11** body + **11** head = 22 | p222–p223 (copy p188–p189) | `awk '/Critical Injuries to the Body/{f=1} /Critical Injuries to the Head/{f=0} f && /^ +(2\|3\|4\|5\|6\|7\|8\|9\|10\|11\|12) +[A-Z]/' p222.txt \| wc -l` → 11 | `awk '/Critical Injuries to the Head/{f=1} f && /^ +(2\|3\|4\|5\|6\|7\|8\|9\|10\|11\|12) +[A-Z]/' p222.txt p223.txt \| wc -l` → 11 |
| F-77 | Wound states | **4**; stabilization DVs **10/13/15** | p187 (= p221), p223 | [READ] | — |
| F-78 | Hazards | **9** | p181–p182 | [READ] | print-only |
| F-79 | Vehicle maneuvers | **7** | p193 | [READ] (estimate) | — |
| F-80 | Trauma Team | **2** plans; **5** members in **4** stat rows; hospital bill **4** rows | p224–p226 | [READ] | — |

### §2.7 Netrunning (`netrunning.md`)

| ID | Figure | Value | Predicate | Command A | Command B / status |
|---|---|---|---|---|---|
| F-81 | Non-ability NET Actions | **4** (Jack In/Out, Use Interface Ability, Activate/Deactivate Program, Miscellaneous) | p199 | [READ] | Interface Abilities are F-7 |
| F-82 | NET Actions per Turn bands | **4** (1–3 = 2, 4–6 = 3, 7–9 = 4, 10 = 5) | p148, p198 | [READ] | — |
| F-83 | Programs (non-Demon) | **27** | `Icon:` lines | `cat p369.txt p370.txt p371.txt p372.txt \| awk '/Icon:/{n++}END{print n}'` → 27 | `cat p204.txt p205.txt p207.txt p208.txt \| grep -c "Icon: [A-Z]"` → 28 incl. one legend line = 27 |
| F-84 | Programs per class | Boosters 4, Defenders 3, Attackers 8, Black ICE 12 (= 27) | — | Boosters `cat p369.txt \| awk '/Booster +0 +0/{n++}END{print n}'` → 4; Black ICE `cat p207.txt p208.txt \| awk '/^ *(Asp\|Giant\|Hellhound\|Kraken\|Liche\|Raven\|Scorpion\|Skunk\|Wisp\|Dragon\|Killer\|Sabertooth) /{n++}END{print n}'` → 12 | Defenders 3, Attackers 8 [HAND] |
| F-85 | Demons | **3** | `Icon:` lines p213 | `awk '/Icon:/{n++}END{print n}' p213.txt` → 3 | — |
| F-86 | Cyberdecks | **3** (5/7/9 slots) | p197, p352, p354 | [READ] | — |
| F-87 | NET hardware | **6**; misc upgrades **2** | p209 | `cat p209.txt \| awk '/^ +(Backup Drive\|DNA Lock\|Hardened\|Insulated\|KRASH Barrier\|Range)/{n++}END{print n}'` → 6 | — |
| F-88 | NET Architecture GM tables | difficulty 3 rows × 4; lobby **6**; body **16** rows × 4 = 64 cells; home-security **3** rows; defenses **18** | p210–p219 | lobby `awk '/Lobby Floor/{s=1} /All Other Floors/{s=0} s && $1 ~ /^[1-6]$/ {n++} END{print n}' p212.txt` → 6; body `awk '/All Other Floors/{s=1} s && $1 ~ /^[0-9]+$/ && $1>=3 && $1<=18 {n++} END{print n}' p212.txt` → 16; defenses `cat p214.txt p215.txt p216.txt p217.txt \| grep -c "Electronics/Security Tech, [0-9]* \?min"` → 18 | print-only |

### §2.8 GM tables with no inventory yet

The operator's scope ruling names **Mooks & Grunts stat blocks, Encounters and Beat Charts** as
content units (print-only). **None of the seven inventories covers them.** They have no figure yet.
Card **E1.8** inventories them first (page range, counts, two implementations each) and appends
F-89 onward to this table in its own commit, before it transcribes them. Until E1.8 lands, any
"N of M" figure that includes them is provisional.

---

## §3 Kinds pinned by `MANIFEST.json`

`data/cyberpunk-red/corpus/MANIFEST.json` holds one entry per kind: `kind`, `file`, `expected`
(the figure below), `figure` (the `F-n` it comes from), `print_only` (true for GM tables), and
`nested` (expected counts of sub-tables inside records, where a kind has them). Card **E1.0**
writes it; each transcription lane turns its own kinds green. One file per kind:
`data/cyberpunk-red/corpus/<kind>.json`, a JSON array of records (`decisions.md §11`).

| Kind | Expected records | Figure | Lane | Compute / print |
|---|---|---|---|---|
| `stat` | 10 | F-20 | E1.2 | compute |
| `role` | 10 | F-1 | E1.1 | compute (rank) |
| `role_ability` | 10 (nested: CA 6 F-4, Maker DV 7 F-14, upgrades 10 F-6, Interface 9 F-7, NET actions bands 4 F-82, pharmaceuticals 5 F-16, cryosystem 5 F-17, rumor 4 F-15, rank bands 6 each F-10–F-13, team jobs 5 F-9 with 30 stat rows F-8, motorpool 14 F-18, Nomad upgrades 19 F-5) | F-3 | E1.1 | compute rank-derived values; print text |
| `streetrat_template` | 100 | F-22 | E1.2 | compute (STAT source) |
| `role_skill_list` | 10 (20 skills each, F-31) | F-31 | E1.2 | compute (Streetrat levels, Edgerunner list) |
| `complete_package_points` | 5 | F-32 | E1.2 | compute (62) |
| `lifepath_table` | 67 = 15 general + 52 Role | F-26, F-27 (rows F-28/F-29 are estimates; E1.2 replaces them) | E1.2 | print (Cultural Origin → Language 4 is compute) |
| `starting_kit` | 10 (weapons/armour, outfit, cyberware per Role) | F-37 | E1.2 | data for the creation kit |
| `ip_cost` | 3 rows × 10 | F-33 | E1.2 | compute |
| `ip_award` | 8 (GM) | F-34 | E1.2 | print-only |
| `reputation_level` | 10 | F-35 | E1.2 | print text; Facedown base computed |
| `skill` | 66 | F-38 | E1.3 | compute |
| `difficulty_value` | 7 | F-42 | E1.3 | print |
| `skill_modifier` | 10 | F-43 | E1.3 | print |
| `price_category` | 8 | F-44 | E1.4 | data |
| `weapon` | 29 = 4 melee + 11 ranged + 14 exotic | F-45, F-46, F-47 | E1.4 | print stats; range row lookup |
| `ammo` | 13 | F-48 | E1.4 | print |
| `attachment` | 8 | F-49 | E1.4 | print |
| `clip_chart` | 10 | F-50 | E1.4 | lookup |
| `weapon_quality` | 3 | F-51 | E1.4 | print |
| `armor` | 9 (8 + Bulletproof Shield) | F-52 | E1.4 | compute SP + penalty |
| `gear` | 53 (`in_starter_table` true on 47) | F-54, F-53 | E1.4 | print |
| `fashion` | 10 (90 price cells) | F-55 | E1.4 | print |
| `drug` | 5 | F-56 | E1.4 | print |
| `service` | 33 | F-57 | E1.4 | print |
| `lifestyle` | 4 | F-58 | E1.4 | print |
| `housing` | 11 | F-59 | E1.4 | print |
| `vehicle` | 14 | F-60 | E1.4 | print |
| `night_market_table` | 2 (6 + 20 rows) | F-61 | E1.4 | print-only (GM) |
| `cyberware` | 96 | F-62 | E1.5 | compute HL, slots, SP/BODY/Initiative effects |
| `therapy` | 3 | F-67 | E1.5 | compute recovery cap (D-5) |
| `cyberpsychosis_band` | 5 | F-68 | E1.5 | print |
| `humanity_trauma` | 4 | F-69 | E1.5 | print (GM) |
| `action` | 18 (estimate) | F-70 | E1.6 | print |
| `range_dv` | 10 = 8 single-shot + 2 autofire | F-71, F-72 | E1.6 | lookup (D-1) |
| `damage_band` | 8 = 4 brawling + 4 martial arts | F-73 | E1.6 | compute (BODY lookup) |
| `martial_arts` | 9 moves in 4 forms + shared | F-74 | E1.6 | print |
| `cover` | 6 | F-75 | E1.6 | print |
| `critical_injury` | 22 | F-76 | E1.6 | print; Death Save penalty count computed |
| `wound_state` | 4 | F-77 | E1.6 | compute thresholds |
| `hazard` | 9 | F-78 | E1.6 | print |
| `vehicle_maneuver` | 7 | F-79 | E1.6 | print |
| `trauma_team` | 2 plans + 4 member rows + 4 hospital rows | F-80 | E1.6 | print |
| `interface_ability` | 9 | F-7 | E1.7 | print (formula) |
| `net_action` | 4 | F-81 | E1.7 | print |
| `program` | 27 | F-83 | E1.7 | slots computed; stats printed |
| `demon` | 3 | F-85 | E1.7 | print-only (GM) |
| `cyberdeck` | 3 | F-86 | E1.7 | compute slots |
| `net_hardware` | 8 = 6 hardware + 2 misc upgrades | F-87 | E1.7 | compute slots |
| `net_architecture` | 5 tables (difficulty, lobby, body, home security, defenses) | F-88 | E1.7 | print-only (GM) |
| `mooks_grunts`, `encounter`, `beat_chart` | set by E1.8 (§2.8) | — | E1.8 | print-only (GM) |

**Sum check.** E1.0 writes a check that sums `expected` over every MANIFEST entry and compares it
with the sum of this table's column (estimates included, flagged). It fails closed on a missing
kind, an extra kind or a mismatch (Sum the piles, `workflow-instruction.md §12`).

---

## §4 Pages that need hand transcription (garbled extraction)

A lane transcribes these pages from the page image, not from `pNNN.txt` alone:

```bash
pdftoppm -f N -l N -r 150 -png "$PDF" out      # N = the pNNN number; view out-NNN.png with the Read tool
pdftotext -f N -l N -layout -x X -y Y -W W -H H "$PDF" -   # a column crop
```

| Inventory | Pages (severity per the inventory) | Lane |
|---|---|---|
| roles | **p149, p150, p152, p155, p164–p166** (high); p146, p151, p154, p159, p160 (medium); p156–p158 skill lines (low-medium) | E1.1 |
| creation | **p087–p088, p099, p104, p118, p121** (high); p073, p079–p080, p083–p086, p089–p090, p230, p052, p410–p412; Lifepath sidebars p058, p060, p065, p070; branch tags p055, p057, p059, p061, p067, p069 | E1.2 |
| skills | p083–p086 (column interleave), p129, p130 (DV values on the wrong line), p131, p141 (repair sidebar) | E1.3 |
| gear | **p099, p103, p104, p339–p340, p357, p121** (high); p095, p096, p098, p352, p353–p356, p358, p191–p192 (medium); data defects p379 ("50,0000eb"), p102 ("10b"), p351 ("100eb(Premium)") | E1.4 |
| cyberware | p112–p117 (cells interleaved; do not parse mechanically), p359–p368 (cells wrap), p118, p231 (therapy, confirm against the image before wiring D-5), p233 | E1.5 |
| combat | p172, p174 (D-1 mapping check), p176, p186, p187/p221, p188–p189 / p222–p223 (Quick Fix / Treatment merged), p191–p192, p225, p231 | E1.6 |
| netrunning | p197, p198–p201, p204–p205, p207–p208, p209, p210, p212, **p213–p217** (defense cells split; verify by hand), p218–p219 | E1.7 |

---

## §5 UNCLEAR items and the ruling that resolves each

Resolution key: **D-n** = the brief's operator-away DEFAULT n (`decisions.md §20`); **A-n** =
an author default **not in the brief**, flagged for operator review (`decisions.md §21`);
**IMG** = settled by the owning lane reading the page image (§4), no ruling needed; **PRINT** = no
computed value depends on it, printed as written.

| # | Item | Source | Resolution |
|---|---|---|---|
| U-1 | Range DV table has 8 rows; weapon table has 11 types | combat §2b | **D-1** (verify against the p174 image) |
| U-2 | Netrunner brain damage: what it is applied against | netrunning §4 | **D-2** |
| U-3 | Air Pistol: Very Heavy (p096) vs Medium (p348) | gear §19.1 | **D-3** |
| U-4 | Luxury and Super Luxury both DV29 (p141, p150) | skills §2, roles §1d | **D-4** (image check) |
| U-5 | Therapy max-Humanity penalty: per piece, Standard Hand/Foot | cyberware §2 | **D-5** |
| U-6 | Media/Lawman "10 Humanity Loss (−2 EMP)" strings | creation §2e, §9 | **D-6** |
| U-7 | Starting Maker / Medicine specialty points at Rank 4 | roles §7.1 | **D-7** |
| U-8 | Lawman Backup Combat Number 16/15/14 at Ranks 8/9/10 | roles §1h, §7.3 | **D-8** |
| U-9 | Exec Corp Kind table header "1d6" with 10 rows | creation §3b | **D-9** |
| U-10 | "50,0000eb" Luxury Penthouse; High Fashion Jewelry 50,000eb "(Luxury)" | gear §12, §9 | **D-10** |
| U-11 | Cyberware slot enforcement | cyberware §7 | **D-11** (enforce) |
| U-12 | Encumbrance | gear §15 (`grep -il encumbr p*.txt` → no files) | **D-12** (none) |
| U-13 | Multiclass: IP price of Rank 1 in the new Role | roles §0, §7.6 | **A-1** |
| U-14 | Starting Moto picks, Exec team, Cryosystem points at Rank 4 | roles §7.2 | **A-2** (same reading as D-7) |
| U-15 | Humanity Loss after creation is dice; the app does not roll dice | cyberware §2; scope (dice engine not in DoD) | **A-3** (rolled by the creation roller or entered by the player, range-checked) |
| U-16 | EMP for negative Humanity | cyberware §2 | **A-4** (clamp at 0, per p233 "Equal to 0") |
| U-17 | Option slots of extra Cybereyes (MultiOptic) and extra Cyberarms (Shoulder Mount) | cyberware §3 | **A-5** (each foundation carries its own printed slot count) |
| U-18 | Attachment slot cost "not stated" for 5 of 8 attachments | gear §4 | **A-6** (1 slot, p112-style "unless otherwise noted" reading, flagged) |
| U-19 | Bodyweight Suit slot and Cyberarm deck slot on one deck | netrunning §3 | **A-7** (both apply; nothing prohibits it) |
| U-20 | Linear Frame Sigma (gear exoskeleton) needs 1 or 2 Interface Plugs (p355 says both) | gear §19.2 | **A-8** (record both, `transcription_note`; enforce neither) |
| U-21 | Starting cash shown on the sheet | gear §18; brief DoD item 1 ("cash") | **A-9** (computed: budget − purchases at creation) |
| U-22 | `cpr/mod.rs` declarations edited by two parallel engine lanes | design | **A-10** (lane anchors; SD-p) |
| U-23 | Where a CPR character's input lives in the save envelope | design | **A-11** (additive optional field) |
| U-24 | Seriously Wounded boundary: "threshold" (p080) vs "less than 1/2 HP" (p187) | creation §1 | PRINT the computed threshold number `ceil(HP/2)`; no wound simulation |
| U-25 | Death Save: does the first save carry a penalty | combat §5 | PRINT the rule; compute Death Save = BODY and the Base Death Save Penalty count from recorded criticals |
| U-26 | Zero HP: "Death State" (p080) vs Mortally Wounded "less than 1 HP" (p187) | combat §5 | PRINT the combat-chapter wording |
| U-27 | Seriously Wounded −2 hits REF/DEX/MOVE (sheet label p121) or "all Actions" (p187) | creation §1 | PRINT both texts; no computed penalty |
| U-28 | Loyalty "losing" table row pairing | roles §7.4 | IMG (p155) |
| U-29 | Nomad upgrade Rank Required pairing | roles §7.5 | IMG (p164–p166) |
| U-30 | Exec outfit prints no Agent; Rockerboy "Heavy Melee Weapon or Flashbang" wrap; Rockerboy "Jewelry x3"; Medtech "Handcuffs" | creation §2e, §9 | IMG (p099, p104) |
| U-31 | Critical-injury Quick Fix / Treatment merged cells | combat §4g | IMG (p222–p223) |
| U-32 | Trauma Team "MOVE & BODY" single number | combat §5 | PRINT as printed (one number) |
| U-33 | Karate / Taekwondo special moves with no printed DV | combat §3e | PRINT |
| U-34 | Hardened Shielding slot count not stated | cyberware §1 | 1 slot by the p112 default rule ("most cyberware options require one slot"), cited in the record |
| U-35 | Linear Frame Beta prerequisite "Two Grafted Muscle" vs "2 Grafted Muscles" | cyberware §4 | PRINT both; same meaning |
| U-36 | Lobby table die type not printed (1–6) | netrunning §5 | PRINT as printed |
| U-37 | IP cost table page: p411 (roles) or p412 (creation) | §2.2 F-33 | IMG/read (E1.2 records the page) |
| U-38 | STAT group of EMP (Mental likely) | creation §1 | PRINT (no mechanical effect) |
| U-39 | Shield with a two-handed weapon | gear §7 | PRINT the rule text |
| U-40 | Head/body armour price split | combat §4b | each location is bought separately at the table price (p097 rule "purchased individually"); print |

---

## §6 Seeds (merge-check roster)

| Seed | System | Build | Exercises |
|---|---|---|---|
| Aldric | PF1e | Fighter 3 (existing starter) | PF byte-identical render |
| Elowen | PF1e | Wizard 5 (existing starter) | PF byte-identical render incl. spells |
| SF-Soldier-3, SF-Mystic-5, SF-Technomancer-5, SF-Envoy-3 | SF1e | SD-37 seeds | SF printed lines byte-identical |
| CPR-Solo-Streetrat | CPR | Solo, Streetrat template (roll row named in E7.1) | template STATs, fixed skills, Combat Awareness 4, Solo kit, Solo cyberware (HL 14) |
| CPR-Netrunner-CP | CPR | Netrunner, Complete Package (62 STAT points, 86 skill points) | allocator paths, Interface 4 → 3 NET Actions, deck slots, bought cyberware |
| CPR-Medtech-Edgerunner | CPR | Medtech, Edgerunner per-STAT rolls | per-STAT roll path, Medicine specialty points (D-7), Medtech kit (HL 12) |

The CPR seeds are restorable through the existing restore-starters mechanism
(`apps/desktop/src-tauri/src/character_hub.rs:2471` `restore_starter_characters_at`; command
registered at `apps/desktop/src-tauri/src/main.rs:278`). **E7.1** writes their full builds to
`artifacts/epic_7/cpr-seed-builds.md` (every STAT, skill level, Lifepath answer, kit pick,
cyberware piece and cash) before generalising the starter list, and every CPR seed number on the
sheet must equal a book-internal value or a hand-computed value shown with its arithmetic.
