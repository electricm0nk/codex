# SD-38 inventory: Roles and Role Abilities (Cyberpunk RED corebook v1.22)

Source: `/home/ubuntu/workspace/inbox/cyberpunk-red/pages/pNNN.txt`. All page cites below are **pNNN file numbers**. Printed page = pNNN minus 1 in this range (p029 is printed 28). The book's own cross-references use printed numbers (the Role Ability index on p144 says "page 144" for Rockerboy, which is file p145).

Shorthand: PD = `/home/ubuntu/workspace/inbox/cyberpunk-red/pages`.

## 0. Global rules

| Rule | Text | File |
|---|---|---|
| Number of Roles | "There are ten Roles" | p030 |
| Starting Role Ability Rank | "Starting Characters automatically begin their Role Ability at Rank 4 (unless the GM ... decides to allow you to start at a higher Rank)". Also p030: "note that it starts with a value of 4". Creation flow: "Set your Role's Role Ability to 4" | p143-p144, p030, p042, p043 |
| Rank range | 1 to 10 for every Role Ability (rank tables all end at 10) | p145-p166, p411 |
| Advancement cost (IP) | Role Ability Rank N costs 60*N IP: ranks 1..10 = 60,120,180,240,300,360,420,480,540,600. Cannot skip ranks. | p411 |
| Skill IP cost (for contrast) | Typical 20*N; Difficult(x2) 40*N | p411 |
| Multiclass | Cannot start with two Roles. Can switch once the previous Role is at Rank 4+ by buying Rank 1 of the new Role Ability with IP. New Role starts at Rank 1. Locked from multiclassing again until the latest Role is Rank 4+. Previous Role can keep improving and keeps its features. | p144 |
| Multiclass example | JP Guyver: Solo Combat Awareness Rank 4 -> buys Nomad Moto Rank 1 with IP, picks a ground bike as first vehicle. | p144 |
| Where "IP" is introduced | "raise the Level of a Skill or the Rank of your Role Ability" | p409 |

Not stated in the text (UNCLEAR, do not invent): whether a multiclass purchase costs 60 IP (Rank 1 price) or something else. p144 says only "buying Rank 1 in the new Role with Improvement Points"; p411 table says Rank 1 = 60 IP. Probable reading is 60 IP; mark as inference.

## 1. Role table

Rank range is 1-10 for all. Blurb file = Role description page; ability file = rules pages.

| # | Role | Role Ability | Blurb file | Ability rules file(s) | Sub-tables in ability rules |
|---|---|---|---|---|---|
| 1 | Rockerboy | Charismatic Impact | p031 | p145-p146 | Yes: 1 ranks table (see 1a) |
| 2 | Solo | Combat Awareness | p032 | p147 | Yes: 1 option list (see 1b) |
| 3 | Netrunner | Interface | p033 | p148 | Yes: NET Actions table + Interface Abilities table (1c). Full netrunning rules pg "198" -> file p198-p199 (other domain) |
| 4 | Tech | Maker | p034 | p148 (intro, Field Expertise), p149 (Upgrade, Fabrication, Invention), p150 (cost table) | Yes: 4 specialties + upgrade list + DV/Time table (1d) |
| 5 | Medtech | Medicine | p035 | p150 (intro, Surgery, Pharmaceuticals), p151 (drug table, Cryosystem Operation table) | Yes (1e) |
| 6 | Media | Credibility | p036 | p152-p154 | Yes: Rumor table + ranks table (1f) |
| 7 | Exec | Teamwork | p037 | p154 (Signing Bonus, housing, insurance), p155 (Team Members, Loyalty), p156-p158 (team member stat tables) | Yes (1g) |
| 8 | Lawman | Backup | p038 | p159-p160 | Yes: ranks table (1h) |
| 9 | Fixer | Operator | p039 | p160 (intro), p161-p162 (ranks) | Yes: ranks table (1i) |
| 10 | Nomad | Moto | p040 | p162 (intro, familiarity, motorpool), p163 (rules, Motorpool table), p164-p166 (upgrade tables) | Yes (1j) |

Role list sentence: p030 (list); Role blurbs each begin on their own page, confirmed by heading regex (count 4.1 below).

### 1a. Rockerboy: Charismatic Impact (p145-p146)
- Sub-table: "Charismatic Impact Ranks" with 6 rank-band rows: 1-2, 3-4, 5-6, 7-8, 9, 10.
- Columns: Rank band | Venues You Can Play | Impact on a Single Fan (DV8) | Impact on a Small Group of Fans (DV10) | Impact on a Huge Group of Fans (DV12). Rows: 6. Cells are prose.
- Check: roll Charismatic Impact + 1d10 vs DV8 (single), DV10 (small group, up to 6), DV12 (huge). Failure bars asking the same favor for a week. (p145)
- Turn non-fans into fans: same roll vs the same DVs, only outside combat. (p145)
- Rank 1-2 huge group cell is joke text: "You're kidding, right? You don't have huge groups of fans yet." (p145)

### 1b. Solo: Combat Awareness (p147)
Pool = total Combat Awareness points, re-divided "when combat begins (before Initiative is rolled), anytime outside of combat, or in combat with an Action".
6 abilities (count 4.4):

| Ability | Cost / effect |
|---|---|
| Damage Deflection | 2 pts: first damage this Round -1; 4 pts: -2; 6: -3; 8: -4; 10: -5 (5 tiers) |
| Fumble Recovery | 4 pts: ignore critical failures (1s) on attacks (still treated as 1) |
| Initiative Reaction | each point +1 to Initiative rolls |
| Precision Attack | 3 pts: +1 to Attacks; 6 pts: +2; 9 pts: +3 (3 tiers) |
| Spot Weakness | each point +1 damage (before armor) on first successful Attack in a Round |
| Threat Detection | each point +1 to Perception Checks |

### 1c. Netrunner: Interface (p148)
- NET Actions per Turn table: columns Interface Rank | 1 to 3 | 4 to 6 | 7 to 9 | 10; one data row: NET Actions = 2 | 3 | 4 | 5. (4 columns, 1 row)
- Interface Abilities table: columns Ability | Data. Rows: 9 (Backdoor, Cloak, Control, Eye-Dee, Pathfinder, Scanner, Slide, Virus, Zap). Data is one-line prose each.
- Text p198 says without Interface "you cannot Netrun" and Interface sets Actions per Turn.

### 1d. Tech: Maker (p148-p150)
- Rule: each +1 Maker Rank = +1 Rank in two different Maker Specialties of the player's choice (p148; same in blurb p034).
- 4 specialties: Field Expertise (p148), Upgrade Expertise (p149), Fabrication Expertise (p149), Invention Expertise (p149-p150).
- Field Expertise: add specialty Rank to Basic Tech, Cybertech, Electronics/Security Tech, Weaponstech, Land/Sea/Air Vehicle Tech checks "for any Non-Maker Specialty purpose". At 1+ Rank may jury-rig as an Action; lasts 10 minutes per Rank. (p148)
- Upgrade Expertise: roll TECH + the item's repair TECH skill + specialty Rank + 1d10; item gets one upgrade only. Bullet list of 10 upgrade options (count 4.6). (p149)
- Fabrication Expertise: same roll form; materials one price category below the item (Super Luxury: half price). (p149)
- Invention Expertise: GM-adjudicated; lowest price category selectable is Expensive. (p149-p150)
- Table "Upgrade/Fabricate/Invent DV/Time" (p150): columns Cost | DV | Time; 7 rows: Cheap/Everyday 9 / 1 hour; Costly 13 / 6 hours; Premium 17 / 1 day; Expensive 21 / 1 week; Very Expensive 24 / 2 weeks; Luxury 29 / 1 month; Super Luxury 29 / 1 month per 10,000eb of Cost.
- Sidebar p149: "Fabricating a Cheap item costs 5eb."

### 1e. Medtech: Medicine (p150-p151)
- Rule: each Medicine Rank increase = allocate 1 point to one of 3 specialties: Surgery, Medical Tech (Pharmaceuticals), Medical Tech (Cryosystem Operation). (p150; blurb p035)
- Surgery: each point = +2 Surgery Skill, max 10. (p150)
- Pharmaceuticals: each point = +1 Medical Tech Skill (max 10); max 5 points in this specialty; each point also unlocks one pharmaceutical (synthesize on DV13 Medical Tech check; doses per hour from 200eb of materials equal to Medical Tech Skill). (p150)
- Cryosystem Operation: each point = +1 Medical Tech Skill (max 10 across both); max 5 points. (p151)
- Sidebar p150: "Your Medical Tech Skill Level equals your points in Pharmaceuticals plus your points in CryoSystem Operation."
- Pharmaceutical table (p151): columns Pharmaceutical | Effect; 5 rows: Antibiotic, Rapidetox, Speedheal, Stim, Surge.
- Cryosystem table (p151): columns Level | Benefit; 5 rows (levels 1-5): 1 = one Cryopump; 2 = Registered Cryotank Technician, 24/7 access to 1 Cryotank at facilities; 3 = 1 Cryotank in a room of choice; 4 = 2 more Cryotanks, Cryopump 2 charges, capacity 2; 5 = 3 more Cryotanks, Cryopump 3 charges, capacity 3.
- Which points the pharmaceutical unlock is allocated to: "Each time you allocate a point into Pharmaceuticals you also gain access to one of the following pharmaceuticals" (player choice among the 5; max 5 points = all 5). Inference; text does not say order.

### 1f. Media: Credibility (p152-p154)
- Rumor table (p152): columns Rumor | Description | Passive DV | Active DV; 4 rows: Vague Rumor 7/13; Typical Rumor 9/15; Substantial Rumor 11/17; Detailed Rumor 13/21.
- Rumors rule: at least twice per week GM secretly rolls Credibility Rank + 1d10; beats any Passive DV = rumor of the highest DV beaten. Active search: STAT + Skill + 1d10 vs Active DV. (p152)
- Publishing: four attributes Access/Sources, Audience, Believability, Impact (p152-p153).
- Ranks table (p153-p154): 6 rank-band rows (1-2, 3-4, 5-6, 7-8, 9, 10); fields Access/Sources | Audience | Believability | Impact. Believability is numeric (see section 2).
- Believability bonus: one easily understood piece of verifiable evidence = +1 chance; more than 4 distinct pieces = +2; stack; "LUCK can never be spent on a Believability Check." (p153)
- Sidebar p154 "What if I Get Fired?" (Exec/Lawman) is flavor.

### 1g. Exec: Teamwork (p154-p158)
- Signing Bonus (Rank 1): Businesswear Jacket, Top, Bottom, Footwear, not resellable without suspicion. (p154)
- Corporate Housing: Rank 2 Corporate Conapt rent-free; Rank 7 Beaverville House in Executive Zone; Rank 10 Beaverville McMansion or Luxury Penthouse. Lifestyle still bought separately. (p154)
- Corporate Health Insurance: Rank 6 Trauma Team Silver; Rank 8 Trauma Team Executive. (p154)
- Team Members: p155 (rules) and p156-p158 (5 jobs with STAT tables).
- Loyalty table (p155): two side-by-side tables, Gaining Loyalty (Action | Gain) 6 rows: Compliment +1, Bonus >=200eb +4, Support vs Management +4, 20% cut +6, Paid time off +6, Risk physical harm +8; Losing Loyalty (Action | Loss) 6 rows: Gain no Loyalty with Team Member during an entire session (compliment overuse note, shown as the -1 row per column layout), Berate -2, Ignore contribution -4, Forget birthday -4, Broken promised perk -6, Throw under bus -6, Abandon under fire -8. Column pairing is garbled (see section 5); UNCLEAR which losing-row carries which loss value.
- Team Member job tables (p156-p158): 5 jobs, each 6 rows (1d6) x 9 STATs (INT REF DEX TECH COOL WILL MOVE BODY EMP) = 30 stat rows (count 4.8); each job also lists Cover Jobs, True Job, Skills at +2/+4/+6, Cyberware, Gear.
  - Company Bodyguard (p156), Company Covert Operative (p156-p157), Company Driver (p157), Company Netrunner (p157-p158), Company Technician (p158).
- Starting Loyalty: roll 1d6 + 1 (p156). Loyalty Save: GM rolls 1d6 under current Loyalty (p155). Loyalty caps at 10 between sessions (p155).
- Replacement cost after a loss: extra 200eb "hiring fees", new hire starts with Loyalty 1 (p155).

### 1h. Lawman: Backup (p159-p160)
- Call: as an Action, roll 1d10 equal or lower than Backup Rank. Then 1d6 = Rounds until arrival; a 6 on that d6 upgrades the backup to the next tier (at Rank 10, two groups arrive). Retry next Turn if nobody responds. (p159)
- Ranks table (p159-p160): 6 rank-band rows: 1-2, 3-4, 5 thru 7, 8, 9, 10. Columns: Combat Number | SP | HP | MOVE & BODY | description (who, how many, how they arrive, kit).

| Rank | Combat Number | SP | HP | MOVE & BODY | Backup |
|---|---|---|---|---|---|
| 1-2 | 8 | 7 | 20 | 4 | Corporate Security, 4 rent-a-cops on foot; Heavy Pistols, Kevlar |
| 3-4 | 10 | 7 | 25 | 5 | Local Beat Cops, 4, in 2 Compact Groundcars; Heavy Pistols, Kevlar |
| 5-7 | 14 | 13 | 35 | 4 | Sheriff's Department, 2, High Performance Groundcar; Heavy Pistols, Assault Rifles, Heavy Armorjack |
| 8 | 16 | 15 | 50 | 6 | Recovery Zone Marshal, 1, Superbike; Very Heavy Pistol, Assault Rifle, Grenade Launcher, Flak |
| 9 | 15 | 18 | 35 | 4 | C-SWAT, 2, AV-4; assault rifles, rocket launchers, Metalgear |
| 10 | 14 | 11 | 35 | 6 | National LE/Interpol/FBI/Netwatch, 2, AV-4; Very Heavy Pistols, Assault Rifles, Light Armorjack; persist after the fight; extra skills list (15 skills) usable at Combat Number |

Rank 9 Combat Number 15 is lower than Rank 8 (16) and Rank 10 is 14: this is how the text reads. Flagged as UNCLEAR (possible book oddity), transcribe verbatim.

### 1i. Fixer: Operator (p160-p162)
- Four named columns per rank band: Contacts & Clients, Reach, Haggle, Grease.
- Ranks table: 6 rank-band rows: 1-2, 3-4, 5-6, 7-8, 9, 10 (p161-p162).
- Haggle contest: COOL + Trading + Operator Rank + 1d10 vs their COOL + Trading + (their Operator Rank if Fixer) + 1d10. Success = 1 deal of your Operator Rank or lower; 1 Fixer deal per transaction. (p160)
- Reach by rank: 1-2 Cheap/Everyday; 3-4 up to Expensive; 5-6 Night Market once per month, up to Super Luxury while there; 7-8 up to Very Expensive; 9 up to Luxury plus optional Midnight Market inside a Night Market; 10 up to Super Luxury piece by piece. (p161-p162)
- Reach order oddity: rank 9 is "up to Luxury" and rank 10 "up to Super Luxury"; rank 7-8 "up to Very Expensive". Rank 5-6 Night Market is the only band with the market rule. Night Market rules at pg 338 -> file p339 (other domain).

### 1j. Nomad: Moto (p162-p166)
- Vehicle Familiarity: add Moto Rank to Drive Land Vehicle, Pilot Air Vehicle, Pilot Sea Vehicle, Air Vehicle Tech, Land Vehicle Tech, Sea Vehicle Tech checks. (p162)
- Family Motorpool: per Moto Rank increase, either add a stock vehicle of Moto Rank or lower to the permitted pool, or add one upgrade of Moto Rank or lower to an already permitted vehicle. (p162; blurb p040)
- One Family Vehicle out at a time; swap overnight if Family is near; destroyed vehicle repaired in a week for 500eb. Rank 10: all vehicles out at once; further vehicles at market price; further upgrades 1,000eb each. (p163)
- Motorpool table (p163): columns Rank | Family Vehicle; 4 rows: 1-4: Compact Groundcar, Gyrocopter, Jetski, Roadbike; 5-6: Helicopter, High Performance Groundcar, Speedboat; 7-8: AV-4, Cabin Cruiser, Superbike; 9-10: Aerozep, AV-9, Super Groundcar, Yacht. Total vehicle names 4+3+3+4 = 14.
- Upgrade tables (p164-p166): 8 groups, 19 upgrades total (count 4.5). Columns: Rank Required | Upgrade (name: description). Upgrades priced "Very Expensive for those attainable at Moto Rank 1, Luxury for those not attainable at Rank 1" (p164).
  - All Vehicles (p164): Armored Chassis 5; Bulletproof Glass 1; Communications Center 1; NOS 1; Onboard Flamethrower 1; Onboard Machine gun 1; Seating Upgrade 1; Security Upgrade 5; Smuggling Upgrade 1. (9)
  - All Vehicles Except Bikes, Jetskis, Gyrocopters (p165): Heavy Chassis 1; Onboard Rocket Pod 5; Vehicle Heavy Weapon Mount 5. (3)
  - All Land and Sea Vehicles (p165): Onboard Melee Weapon 1. (1)
  - All Land Vehicles (p165): Hover Upgrade 5; AV-4 Engine Upgrade 7. (2)
  - All Land and Sea Vehicles Except Bikes and Jetskies (p165): Combat Plow 1. (1)
  - All Bikes (p166): Enhanced Interface Plug Integration 5. (1)
  - All Groundcards [sic, "Groundcars"] (p166): Deployable Spike Strip 1. (1)
  - Aerozep, AV-4, Cabin Cruiser, Groundcars, Yacht (p166): Housing Capacity 1. (1)
  - The Rank Required values are read off a column that pdftotext separated from the names (section 5). The pairings above are by line order and positions; they are consistent with the rule text (Rank 5 for armored/security/rocket/heavy mount/hover/bike plug, Rank 7 for AV-4 engine) but mark them VERIFY AGAINST PDF IMAGE before shipping.
- Nomad housing: p106 says "Nomads have an additional Housing option, due to their Moto Role Ability. See Pg. 165" (file p166 Housing Capacity -> "Kombi"). Housing rules otherwise at p105-p106 (other domain).

## 2. Sheet-visible computed values (rank-dependent numbers)

| Role | Value | Rule | File |
|---|---|---|---|
| All | Role Ability Rank | Starts at 4; 1-10 | p143-p144 |
| Rockerboy | Charismatic Impact check | Rank + 1d10 vs DV8/10/12 (fixed DVs, not rank-dependent) | p145 |
| Rockerboy | Venue tier, favor tier | Looked up by rank band (prose) | p145-p146 |
| Solo | Combat Awareness points | = Rank (pool total, divided among abilities) | p147 |
| Solo | Per-ability sheet numbers | Damage Deflection -1..-5 at 2/4/6/8/10 pts; Fumble Recovery 4 pts; Initiative +1/pt; Precision Attack +1/+2/+3 at 3/6/9 pts; Spot Weakness +1 dmg/pt; Threat Detection +1 Perception/pt. Allocation is player-chosen (a loadout, not a derived value) | p147 |
| Netrunner | NET Actions per Turn | Rank 1-3 = 2; 4-6 = 3; 7-9 = 4; 10 = 5 | p148 |
| Netrunner | Interface Skill value (in Team Member Netrunner) | "Skills at +2: Interface" for the NPC netrunner | p157 |
| Tech | Maker specialty Ranks | Total specialty ranks = 2 per Maker Rank gained (player-chosen split among 4 specialties) | p148 |
| Tech | Field Expertise bonus | + specialty Rank to six Tech skill checks (non-Maker purposes); jury-rig lasts 10 min x Rank | p148 |
| Tech | Upgrade/Fabricate/Invent check | TECH + skill + specialty Rank + 1d10 | p149 |
| Medtech | Medicine specialty points | 1 point allocated per Medicine Rank (player-chosen among 3) | p150 |
| Medtech | Surgery Skill | = 2 x points in Surgery, max 10 | p150 |
| Medtech | Medical Tech Skill | = Pharmaceuticals points + Cryosystem Operation points, max 10 (each specialty max 5 points) | p150-p151 |
| Medtech | Pharmaceuticals known | = Pharmaceuticals points (up to 5) | p150 |
| Medtech | Cryopump/Cryotank holdings | Table by Cryosystem Operation points 1-5 | p151 |
| Medtech | Doses synthesized per hour | = Medical Tech Skill, from 200eb materials | p150 |
| Media | Believability | Chance out of 10: Rank 1-2 = 2; 3-4 = 3; 5-6 = 4; 7-8 = 5; 9 = 6; 10 = 7. +1 for one verifiable evidence, +2 for >4 pieces | p153-p154 |
| Media | Rumor roll | Credibility Rank + 1d10 vs passive DVs 7/9/11/13, twice per week | p152 |
| Media | Access/Audience/Impact | Prose by rank band | p153-p154 |
| Exec | Team Member count | Rank 3 = 1; Rank 5 = 2; Rank 9 = 3 (max 3) | p155 |
| Exec | Housing tier | Rank 2 Conapt; Rank 7 Beaverville House; Rank 10 McMansion/Penthouse | p154 |
| Exec | Insurance tier | Rank 6 Trauma Team Silver; Rank 8 Executive | p154 |
| Exec | Team Member Loyalty | Starting 1d6+1; cap 10 between sessions | p155-p156 |
| Lawman | Backup success threshold | d10 <= Rank | p159 |
| Lawman | Backup tier stats | Combat Number, SP, HP, MOVE and BODY by rank band (table 1h) | p159-p160 |
| Fixer | Haggle bonus in contest | + Operator Rank in the contest roll; deal size <= Rank | p160 |
| Fixer | Reach (price category cap) | By band: Cheap/Everyday, Expensive, Super Luxury (Night Market), Very Expensive, Luxury, Super Luxury | p161-p162 |
| Fixer | Grease cultures | Rank 1-2: local; 3-4: +1 culture and 1 language at Skill 4; 5-6: 3 total; 7-8: 6 total; 9, 10: prose | p161-p162 |
| Fixer | Haggle price effect | Rank 1-2: 10% more/less; 9: 20% more/less; others prose | p161-p162 |
| Nomad | Vehicle Familiarity | + Moto Rank to 6 vehicle skills | p162 |
| Nomad | Family Vehicles | Allowed vehicle types by rank band (table 1j); 1 pick or upgrade per Rank gained | p162-p163 |
| Nomad | Max upgrade rank | Upgrade availability by Rank Required (1, 5, 7) | p164-p166 |

## 3. Starting gear and starting cash per Role

Not in the Roles chapter (p031-p040 have only blurbs and the Role Ability stub). It lives here (pointers only, not transcribed by this inventory; belongs to another domain):

| Item | Where | Notes |
|---|---|---|
| Streetrat / Edgerunner extra cash | p042 ("500eb to spend on extra stuff"), p099 | 500eb, not per Role |
| Complete Package budget | p043 | 2550eb general, 800eb Fashion/Fashionware only |
| Starting weapons and armor per Role | p099 (Rockerboy, Solo, Netrunner, Tech, Medtech, Media, Lawman, Exec, Fixer, Nomad), p100 (worked example) | Role-indexed tables; some entries are "X or Y" choices |
| Starting cyberware per Role | p118 | Role-indexed; Humanity Loss per Role (Rockerboy 9, Solo 14, Netrunner 14, Tech 12, Medtech 12, Media 10, Lawman 10, Exec 12, Fixer 16, Nomad 14) with EMP effect |
| Starting skills per Role (Streetrat templates) | p087; Edgerunner: p089 | Role-indexed |
| Template options Role by Role | p075 (Rockerboy ...) | Statistics templates; unverified by me beyond header |
| Outfit, gear, lifestyle | p104-p106 | |
| Exec Signing Bonus suit | p154 | Role-ability gear granted at Teamwork Rank 1 |
| Exec team member kit | p156-p158 | NPC kit, not PC kit |

## 4. Counts with derivation commands

Run from `PD` (`cd /home/ubuntu/workspace/inbox/cyberpunk-red/pages`).

| # | Count | Value | Command |
|---|---|---|---|
| 4.1 | Role blurb pages (one per Role heading) | 10 | `for r in ROCKERBOY SOLO NETRUNNER TECH MEDTECH MEDIA EXEC LAWMAN FIXER NOMAD; do grep -lE "^\s*$r\s*$" p0[2-4]*.txt; done \| wc -l` (files p031..p040) |
| 4.2 | Role Ability stubs in blurbs | 10 | `grep -E '▶ Role Ability:' p0[3-4]*.txt \| awk 'END{print NR}'` |
| 4.3 | Role Ability sections in the ability chapter | 10 (Interface and Maker share p148) | `grep -E '\((Rockerboy\|Solo\|Netrunner\|Tech\|Medtech\|Media\|Exec\|Lawman\|Fixer\|Nomad) Role Ability\)' p14[5-9].txt p15*.txt p16[0-2].txt` gives the 10 headings (plus 2 prose mentions on p154 and p157 and one wrapped heading on p150 that the same regex catches: Medicine) |
| 4.4 | Solo Combat Awareness abilities | 6 | `grep -E '^\s*▶ (Damage Deflection\|Fumble Recovery\|Initiative Reaction\|Precision Attack\|Spot Weakness\|Threat Detection)' p147.txt \| wc -l` |
| 4.5 | Nomad upgrades | 19 | `cat p164.txt p165.txt p166.txt \| grep -oE '(Armored Chassis\|Bulletproof Glass\|Communications Center\|NOS\|Onboard Flamethrower\|Onboard Machine gun\|Seating Upgrade\|Security Upgrade\|Smuggling Upgrade\|Heavy Chassis\|Onboard Rocket Pod\|Vehicle Heavy Weapon Mount\|Onboard Melee Weapon\|Hover Upgrade\|AV-4 Engine Upgrade\|Combat Plow\|Enhanced Interface Plug Integration\|Deployable Spike Strip\|Housing Capacity):' \| awk '{c++}END{print c}'` printed 19. Caveat: the first match must include Armored Chassis, which is on a line starting with the digit 5 (`5  Armored Chassis:`); the command above matches it with -o. Names list was typed by hand from reading. grep -o caveat in AGENTS.md rule: second implementation, manual list count: 9+3+1+2+1+1+1+1 = 19 (agrees). |
| 4.6 | Tech Upgrade Expertise bullets | 10 | `grep -c '•' p149.txt` printed 10 (all bullets on p149 are that list) |
| 4.7 | Interface Abilities | 9 | `grep -E '^\s*(Backdoor\|Cloak\|Control\|Eye-Dee\|Pathfinder\|Scanner\|Slide\|Virus\|Zap)\s' p148.txt \| wc -l` printed 9 |
| 4.8 | Exec Team Member stat rows | 30 (5 jobs x 6) | `grep -E '^\s*[1-6]\s+[0-9]\s+[0-9]\s+[0-9]\s+[0-9]\s+[0-9]\s+[0-9]\s+[0-9]\s+[0-9]\s+[0-9]\s*$' p156.txt p157.txt p158.txt \| wc -l` printed 30 |
| 4.9 | Exec Team Member jobs | 5 | `grep -E '▶ Company ' p156.txt p157.txt p158.txt \| wc -l` (Bodyguard, Covert Operative, Driver, Netrunner, Technician) |
| 4.10 | Backup rank-band rows | 6 | `grep -cE '^\s*▶ Backup Rank ' p159.txt p160.txt` printed 3 and 3 |
| 4.11 | Operator rank-band rows | 6 | `grep -cE '▶ Operator Ranks? [0-9]' p161.txt p162.txt` printed 3 and 3 |
| 4.12 | Credibility rank-band rows | 6 | `grep -cE '▶ Credibility Ranks? [0-9]' p153.txt p154.txt` printed 5 and 1 |
| 4.13 | Charismatic Impact rank-band rows | 6 | `grep -cE '▶ Charismatic Impact Ranks? [0-9]' p145.txt p146.txt` printed 2 and 4 |
| 4.14 | Maker DV/Time rows | 7 | `grep -E '^\s*(Cheap/Everyday\|Costly\|Premium\|Expensive\|Very Expensive\|Luxury\|Super Luxury)\s' p150.txt \| wc -l` printed 7 |
| 4.15 | Rumor table rows | 4 | read directly from p152 (Vague, Typical, Substantial, Detailed). ESTIMATE-free but not command-derived: table text is in two interleaved columns. `grep -cE 'Rumor' p152.txt` printed 10 and is not the row count. |
| 4.16 | Pharmaceuticals | 5 | read from p151 table; `grep -E '^\s*(Antibiotic\|Rapidetox\|Speedheal\|Stim\|Surge)\s' p151.txt` gives 1 because effect text wraps; the 5 names were confirmed by eye. |
| 4.17 | Cryosystem levels | 5 | read from p151; `grep -E '^\s*[1-5]\s+With' p151.txt \| wc -l` printed 2 (wrapped rows); 5 by reading. |
| 4.18 | Motorpool vehicle names | 14 | read from p163 (4+3+3+4) |
| 4.19 | Total PDF pages in corpus dir | 459 | `ls /home/ubuntu/workspace/inbox/cyberpunk-red/pages \| wc -l` |

Role Ability rank-band row counts all equal 6 except tables with other shapes: NET Actions (4 bands: 1-3, 4-6, 7-9, 10), Motorpool (4 bands: 1-4, 5-6, 7-8, 9-10), Medtech Cryosystem (5 levels).

## 5. Text-extraction quality (pdftotext) notes for hand transcription

| File | Problem | Severity |
|---|---|---|
| p145 | Sidebar ("Rockerboys take their name from Rockerboy Manson...") interleaved with body column | low; table text itself is clean |
| p146 | "Charismatic Impact in Action" sidebar is interleaved line by line with the rank 5-6 and 7-8 rows, Rank 9 and 10 text | medium; re-sequence by band |
| p147 | "Combat Awareness in Action" sidebar left column interleaved with main text; ability text is clean | low |
| p148 | Two tables clean; Field Expertise text clean | low |
| p149 | Three columns interleaved (Upgrade Expertise bullets, roll text, sidebar quote, Fabrication/Invention); read as 2 columns | high; transcribe by hand |
| p150 | Two columns interleaved (Invention end / Medicine intro / DV-Time table / Surgery / Pharmaceuticals); sidebar "Your Medical Tech Skill Level equals..." inside Pharmaceuticals text | high |
| p151 | Drug table rows have effect text wrapped over several lines; cryosystem table rows wrap; sidebar "You'll find more information on Cryopumps..." inside | medium |
| p152 | Rumor table: columns Passive DV / Active DV / Rumor / Description interleaved; sidebar "Big Impact" interleaved | high; DVs (7/13, 9/15, 11/17, 13/21) are by position |
| p153 | Clean ranks text; "Impact" paragraph clean | low |
| p154 | Sidebar "What if I Get Fired?" interleaved with Teamwork text (Housing and Insurance) | medium |
| p155 | Loyalty table: Gaining and Losing columns interleaved; some row pairings broken (e.g. "-1" row has note about compliment overuse; "-2" row ordering) | high; hand transcribe, pairing UNCLEAR |
| p156-p158 | Team Member stat tables: parse cleanly (30 rows, 9 STATs each, regex confirmed) but Skills line wraps across pages (Covert Operative p156/p157, Netrunner p157/p158). p158 shows "Skills at +6.:" with stray period | low-medium |
| p159 | Sidebar "Backup in Action" interleaved with body | medium |
| p160 | Sidebar about treaties between law enforcement interleaved with Rank 8-9 rows | medium |
| p161-p162 | Operator rank text clean; page 162 starts mid-table | low |
| p163 | Motorpool table clean: row "9 to 10" ends with trailing comma "Yacht," | low |
| p164-p166 | Rank Required column is separated from the Upgrade text (rank digits appear on the first line, a middle line, or last line of the row, centered vertically) | high; Rank pairing requires PDF image check |
| p166 | "All Groundcards" heading typo in source | note |

Pages with clean tables: p148 (NET Actions), p150 (cost table rows), p153-p154 (ranks), p156-p158 (stats), p161-p162, p163 (Motorpool), p411 (IP costs).

## 6. Print vs compute split (per standing sheet rule)

Compute (feeds a sheet total or a number on the sheet):
- Role Ability Rank (start 4, range 1-10, IP cost 60*N next rank).
- Solo: total Combat Awareness points = Rank. Per-ability values only when the player picks a loadout (store the allocation; cost tables for validation).
- Netrunner: NET Actions per Turn from Rank (2/3/4/5).
- Tech: specialty rank total = 2 per Maker Rank; Field Expertise bonus applies to six Tech skill checks (this is a skill-check modifier, so it feeds those skill rows).
- Medtech: Surgery Skill = 2 per Surgery point (max 10); Medical Tech Skill = Pharm points + Cryo points (max 10); doses/hour = Medical Tech Skill.
- Media: Believability chance (2..7 by band).
- Exec: Team Member count (0, 1 at Rank 3, 2 at Rank 5, 3 at Rank 9).
- Lawman: Backup roll threshold (= Rank) and the six-tier backup stat block (Combat Number, SP, HP, MOVE/BODY).
- Fixer: Operator Rank as a bonus in Haggle contest; Reach price-category cap by band.
- Nomad: Moto Rank bonus to six vehicle skills.
- Multiclass: independent Rank per Role Ability (more than one Role Ability on the sheet; each rank sets its own derived values).

Print verbatim (rule text, no engine):
- Charismatic Impact rank table (venues, impacts), fan/DV rules.
- Combat Awareness ability descriptions (only the allocation and arithmetic need computing).
- Interface Abilities table (9 one-line descriptions).
- Maker specialties text (Field/Upgrade/Fabrication/Invention), upgrade option list (10), DV/Time table.
- Medicine specialties text, pharmaceutical effects table, cryosystem benefits table.
- Credibility Access/Audience/Impact prose, Rumor table.
- Teamwork perks: Signing Bonus, Housing and Insurance by rank (print the tier for the player's rank), Loyalty rules and tables, Team Member stat blocks (these are rolled/NPC data, likely per-team-member notes).
- Backup descriptions (who arrives and how), Rank 10 extra skills list.
- Operator rank prose (Contacts, Haggle effects, Grease).
- Moto Family Motorpool by rank band, upgrade list (19) with rank required, Rank 10 leadership rule.
- Multiclassing rule text.

## 7. Open items marked UNCLEAR / unverified

1. Starting number of Maker/Medicine specialty points at creation: text gives only "when a Tech/Medtech increases a Rank". Rank 4 starts implies 4 steps, but the book does not state how starting specialty ranks are assigned in creation. Quote: "Whenever a Tech increases their Maker Rank by one, they gain one rank in two different Maker Specialties" (p034, p148). UNCLEAR whether creation grants 8 specialty ranks.
2. Same question for Moto (starting vehicle at Rank 4), Exec (starting team: Rank 4 gives 1 Team Member by the Rank 3 rule), Cryosystem (4 points).
3. Backup Rank 8 > 9 > 10 Combat Number (16, 15, 14): verbatim reading.
4. Loyalty "losing" table row pairing (p155).
5. Nomad upgrade rank assignments (p164-p166).
6. Multiclass IP price (see section 0).
