# Cyberpunk RED v1.22: Stats, Creation, Lifepath, Reputation, Improvement (inventory)

Source: `/home/ubuntu/workspace/inbox/cyberpunk-red/pages/pNNN.txt`. Page-file rule: **pNNN = printed page + 1** (checked: printed "Rockerboy page 54" is `p055`; printed "Reputation 193" is `p194`; printed "Getting Better 408" is `p409`). All page cites below are pNNN files. P = `/home/ubuntu/workspace/inbox/cyberpunk-red/pages`.

Verification marks: **[CMD]** = number re-derivable by the command shown. **[HAND]** = hand count from reading, no command; treat as ESTIMATE. **[UNCLEAR]** = text does not settle it.

---

## 1. The 10 STATs and derived values

STATs are "generally rated from 1 to 8, but can go higher" (p073). Four groups (p073). Allowed creation range depends on method:

| STAT | Abbr | Group | Streetrat/Edgerunner template range seen | Complete Package range |
|---|---|---|---|---|
| Intelligence | INT | Mental | 3-8 | 2-8 |
| Willpower | WILL | Mental | 3-8 | 2-8 |
| Cool | COOL | Mental | 3-8 | 2-8 |
| Empathy | EMP | Mental | 4-8 | 2-8 |
| Technique | TECH | Combat | 3-8 | 2-8 |
| Reflexes | REF | Combat | 5-8 | 2-8 |
| Luck | LUCK | Fortune | 5-8 | 2-8 |
| Body | BODY | Physical | 3-8 | 2-8 |
| Dexterity | DEX | Physical | 5-8 | 2-8 |
| Movement | MOVE | Physical | 5-7 | 2-8 |

Definitions on p073 (two-column page, group headings interleaved with entries; see section 7). The page names four groups (Mental, Combat, Fortune, Physical). Group labels for the 10 STATs come from the heading order on p073; EMP's group is **[UNCLEAR]** from the extracted text (its entry sits above the "Combat Group" heading, so Mental is the likely reading). No mechanical effect.

Template min/max per STAT are derived from the 100 template rows. **[CMD]** `python3 -I $S/gen.py` where S is the scratchpad dir. Output: `INT min 3 max 8, REF 5-8, DEX 5-8, TECH 3-8, COOL 3-8, WILL 3-8, LUCK 5-8, MOVE 5-7, BODY 3-8, EMP 4-8`. Complete Package limit "no STAT may be higher than 8 or lower than 2" is quoted on p079 and p042.

### Derived values (exact text)

| Value | Formula as printed | Page | Compute or print |
|---|---|---|---|
| Hit Points | "You have Hit Points equal to 10 + (5[BODY and WILL averaged, rounding up])" | p080 | **Compute** = 10 + 5 x ceil((BODY+WILL)/2) |
| HP lookup table | 9 rows (WILL 2-10) x 14 columns (BODY 2-15) = 126 cells | p080 | Check only. **[CMD]** all 126 cells match the formula, 0 mismatches (`gen.py`). Table does not cover WILL 11+ or BODY 16+; the formula does. |
| Seriously Wounded | "half of their total HP (rounded up)" (creation); wound table says "Less than 1/2 HP (round up)", -2 to all Actions, DV13 stabilization | p080, p187 | **Compute** = ceil(HP/2). Boundary wording (at-or-below vs below) is **[UNCLEAR]**: p080 calls it the "threshold", p187 says "Less than 1/2 HP". Print the number only. |
| Death Save | "Their Death Save is equal to their BODY Statistic" | p080 | **Compute** = BODY. Roll rule p189: "Roll a d10. If you roll under your BODY, you live ... If you roll a 10, you automatically fail"; penalty rises +1 per save. Rule text: print. |
| Humanity | "For every point of Empathy the Character has, they gain 10 points of Humanity" | p081 | **Compute** = EMP x 10 (table 1..10 -> 10..100 on p081) |
| EMP (current) | "This happens every time the tens place of your Humanity value is lowered ... a Character with 44 Humanity has an EMP of 4 until their Humanity is lowered to 39, at which point their EMP lowers to 3." p230: "50 Humanity to 46 Humanity ... EMP 5 to EMP 4" | p081, p230 | **Compute** = floor(current HUM / 10). Sheet keeps EMP as Maximum and Current (p121 "STATs that can have two different values (Maximum and Current)"). |
| Cyberpsychosis | "If your Character's HUM drops below zero" | p081 | Print rule text. |
| LUCK pool | "remaining LUCK Pool (which holds LUCK Points equal to your LUCK Statistic, and which refills at the beginning of each game session)" | p131; also p073 | Compute pool max = LUCK; current value is a play-time tracker (blank boxes). |
| MOVE (distance) | Move Action: "a number of m/yds equal to their MOVE x 2, or a number of squares ... equal to their MOVE" ; table row "Move up to as many m/yds as your MOVE STAT x 2 each Turn" | p128, p170, p127 | Compute m/yds = MOVE x 2 (only if the sheet prints it). Seriously-wounded rider: "-6 to MOVE (Minimum 1)" is Mortally Wounded (p187). |
| Initiative | "Your Initiative Value = REF + 1d10" | p127 | Sheet total = REF (the d10 is rolled at the table). |
| Skill Base | skill Level + linked STAT ("Skill Base") | p082, p121 | **Compute** per skill row: LVL + STAT. |
| Role Ability rank | "Starting Characters automatically begin their Role Ability at Rank 4 (unless the GM ... decides to allow you to start at ..." text cut) | p143, p042 | Set 4 for every method. **[UNCLEAR]** the GM exception text is cut off in p143; see `P/p143.txt` lines 51-52 and p144. |

Wound states (p187, relevant because HP derives them): Lightly Wounded = less than full HP, no effect, DV10. Seriously Wounded = less than 1/2 HP, -2 to all Actions, DV13. Mortally Wounded = less than 1 HP, -4 to all Actions, -6 to MOVE (min 1), Death Save each turn, DV15. Dead = one failed Death Save. Sheet also prints "-2 to all actions when seriously wounded; penalty applies to REF, DEX & MOVE" (p121, label text; wording scrambled, **[UNCLEAR]** which of REF/DEX/MOVE the -2 hits versus "all Actions" on p187). Do not simulate; print.

---

## 2. The three creation methods

Overview p041. Step charts: Streetrat + Edgerunner share one chart (p042), Complete Package has its own (p043). Shared steps: #1 pick Role and "Set your Role's Role Ability to 4"; #2 run general Lifepath and the Role-based Lifepath, pick Cultural Origin Language as a Skill at Rank 4; #3 STATs; #4 derived STATs; #5 skills; #6 weapons/armor; #7 outfit; #8 cyberware.

### 2a. Method 1: Streetrat (Templates) (p074-p078, p087-p088, p099, p104, p118)

- STATs: roll **1d10 once**, copy the whole row for the Role. "You may not move your STATs around" (p074). 10 Roles x 10 rows.
- Skills: fixed per Role (13 Basic + 7 Role skills = 20), copy levels (p087-p088). Plus Language (Cultural Origin) 4 free.
- Weapons/armor: fixed per Role, choose one where "or" (p099). Outfit/fashion: fixed per Role (p104). Cyberware: fixed per Role (p118). Plus "500eb to spend on other items or to keep" (p099).
- Row counts **[CMD]**: `cat P/p075.txt P/p076.txt P/p077.txt P/p078.txt | awk '/^ *[0-9]+( +[0-9]+){10} *$/ {c++} END{print c}'` -> `110`. That includes the 10 Edgerunner worked-example rows on p078 (a copy of the Solo table) so unique role rows are 100. Per-role row count from the parser (`gen.py`, keyed on the 10 `▶ Role` headers): `rows=[10,10,10,10,10,10,10,10,10,10]`.

Streetrat STAT arrays (all 10 Roles, all 10 rolls). Column order as printed: INT REF DEX TECH COOL WILL LUCK MOVE BODY EMP. Row sum shown is derived, not printed. Pages: Rockerboy/Solo/Netrunner p075; Tech/Medtech/Media p076; Lawman/Exec/Fixer p077; Nomad p078.


**Rockerboy** (p075-p078; roll = 1d10 row)

| Roll | INT | REF | DEX | TECH | COOL | WILL | LUCK | MOVE | BODY | EMP | Sum |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 7 | 6 | 6 | 5 | 6 | 8 | 7 | 7 | 3 | 8 | 63 |
| 2 | 3 | 7 | 7 | 7 | 7 | 6 | 7 | 7 | 5 | 8 | 64 |
| 3 | 4 | 5 | 7 | 7 | 6 | 6 | 7 | 7 | 5 | 8 | 62 |
| 4 | 4 | 5 | 7 | 7 | 6 | 8 | 7 | 6 | 3 | 8 | 61 |
| 5 | 3 | 7 | 7 | 7 | 6 | 8 | 6 | 5 | 4 | 7 | 60 |
| 6 | 5 | 6 | 7 | 5 | 7 | 8 | 5 | 7 | 3 | 7 | 60 |
| 7 | 5 | 6 | 6 | 7 | 7 | 8 | 7 | 6 | 3 | 6 | 61 |
| 8 | 5 | 7 | 7 | 5 | 6 | 6 | 6 | 6 | 4 | 8 | 60 |
| 9 | 3 | 5 | 5 | 6 | 7 | 8 | 7 | 5 | 5 | 7 | 58 |
| 10 | 4 | 5 | 6 | 5 | 8 | 8 | 7 | 6 | 4 | 7 | 60 |

Row sums for Rockerboy: min 58, max 64 (derived with sum of the 10 values per row)

**Solo** (p075-p078; roll = 1d10 row)

| Roll | INT | REF | DEX | TECH | COOL | WILL | LUCK | MOVE | BODY | EMP | Sum |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 6 | 7 | 7 | 3 | 8 | 6 | 5 | 5 | 6 | 5 | 58 |
| 2 | 7 | 8 | 6 | 3 | 6 | 6 | 7 | 5 | 6 | 6 | 60 |
| 3 | 5 | 8 | 7 | 4 | 7 | 7 | 6 | 7 | 8 | 5 | 64 |
| 4 | 5 | 8 | 6 | 4 | 6 | 7 | 6 | 5 | 7 | 6 | 60 |
| 5 | 6 | 6 | 7 | 5 | 7 | 6 | 7 | 6 | 8 | 4 | 62 |
| 6 | 7 | 7 | 6 | 5 | 7 | 6 | 6 | 7 | 7 | 5 | 63 |
| 7 | 7 | 7 | 6 | 5 | 6 | 7 | 7 | 6 | 6 | 6 | 63 |
| 8 | 7 | 8 | 7 | 5 | 6 | 6 | 5 | 6 | 8 | 4 | 62 |
| 9 | 7 | 7 | 6 | 4 | 6 | 6 | 6 | 5 | 6 | 5 | 58 |
| 10 | 6 | 6 | 8 | 5 | 6 | 6 | 5 | 6 | 6 | 5 | 59 |

Row sums for Solo: min 58, max 64 (derived with sum of the 10 values per row)

**Netrunner** (p075-p078; roll = 1d10 row)

| Roll | INT | REF | DEX | TECH | COOL | WILL | LUCK | MOVE | BODY | EMP | Sum |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 5 | 8 | 7 | 7 | 7 | 4 | 8 | 7 | 7 | 4 | 64 |
| 2 | 5 | 6 | 7 | 5 | 8 | 3 | 8 | 7 | 5 | 5 | 59 |
| 3 | 5 | 6 | 8 | 6 | 6 | 4 | 7 | 6 | 7 | 4 | 59 |
| 4 | 5 | 7 | 7 | 7 | 7 | 5 | 8 | 6 | 5 | 5 | 62 |
| 5 | 5 | 8 | 8 | 5 | 7 | 3 | 7 | 5 | 5 | 6 | 59 |
| 6 | 6 | 6 | 6 | 7 | 8 | 4 | 7 | 7 | 6 | 6 | 63 |
| 7 | 6 | 6 | 6 | 7 | 6 | 5 | 7 | 7 | 7 | 6 | 63 |
| 8 | 5 | 7 | 8 | 6 | 8 | 4 | 8 | 5 | 7 | 4 | 62 |
| 9 | 7 | 6 | 7 | 7 | 6 | 3 | 6 | 5 | 6 | 5 | 58 |
| 10 | 7 | 8 | 6 | 6 | 6 | 4 | 7 | 7 | 5 | 6 | 62 |

Row sums for Netrunner: min 58, max 64 (derived with sum of the 10 values per row)

**Tech** (p075-p078; roll = 1d10 row)

| Roll | INT | REF | DEX | TECH | COOL | WILL | LUCK | MOVE | BODY | EMP | Sum |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 6 | 7 | 7 | 8 | 4 | 4 | 5 | 5 | 7 | 6 | 59 |
| 2 | 7 | 6 | 6 | 7 | 5 | 3 | 7 | 7 | 5 | 5 | 58 |
| 3 | 8 | 6 | 5 | 7 | 5 | 4 | 7 | 7 | 5 | 7 | 61 |
| 4 | 7 | 8 | 7 | 8 | 4 | 4 | 6 | 5 | 6 | 7 | 62 |
| 5 | 6 | 6 | 7 | 6 | 4 | 3 | 7 | 7 | 6 | 6 | 58 |
| 6 | 8 | 7 | 5 | 6 | 3 | 3 | 7 | 6 | 6 | 7 | 58 |
| 7 | 8 | 6 | 7 | 8 | 4 | 4 | 7 | 6 | 7 | 6 | 63 |
| 8 | 8 | 8 | 7 | 8 | 5 | 4 | 6 | 5 | 6 | 6 | 63 |
| 9 | 6 | 6 | 7 | 8 | 3 | 3 | 5 | 7 | 7 | 7 | 59 |
| 10 | 8 | 8 | 5 | 6 | 4 | 4 | 6 | 5 | 6 | 6 | 58 |

Row sums for Tech: min 58, max 63 (derived with sum of the 10 values per row)

**Medtech** (p075-p078; roll = 1d10 row)

| Roll | INT | REF | DEX | TECH | COOL | WILL | LUCK | MOVE | BODY | EMP | Sum |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 7 | 5 | 6 | 7 | 5 | 3 | 8 | 5 | 5 | 7 | 58 |
| 2 | 6 | 7 | 7 | 7 | 4 | 4 | 6 | 7 | 7 | 7 | 62 |
| 3 | 6 | 5 | 5 | 8 | 5 | 3 | 8 | 5 | 7 | 8 | 60 |
| 4 | 8 | 7 | 6 | 8 | 3 | 5 | 6 | 6 | 5 | 7 | 61 |
| 5 | 6 | 7 | 5 | 7 | 5 | 5 | 8 | 7 | 6 | 8 | 64 |
| 6 | 8 | 5 | 5 | 8 | 5 | 5 | 6 | 6 | 5 | 6 | 59 |
| 7 | 8 | 6 | 5 | 8 | 5 | 4 | 8 | 5 | 7 | 7 | 63 |
| 8 | 6 | 5 | 7 | 7 | 3 | 5 | 8 | 5 | 5 | 8 | 59 |
| 9 | 6 | 6 | 7 | 7 | 5 | 4 | 6 | 6 | 5 | 6 | 58 |
| 10 | 8 | 7 | 6 | 6 | 3 | 4 | 8 | 7 | 6 | 7 | 62 |

Row sums for Medtech: min 58, max 64 (derived with sum of the 10 values per row)

**Media** (p075-p078; roll = 1d10 row)

| Roll | INT | REF | DEX | TECH | COOL | WILL | LUCK | MOVE | BODY | EMP | Sum |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 6 | 6 | 5 | 5 | 8 | 7 | 5 | 7 | 5 | 7 | 61 |
| 2 | 8 | 7 | 7 | 3 | 6 | 6 | 6 | 5 | 6 | 8 | 62 |
| 3 | 6 | 7 | 7 | 5 | 6 | 8 | 5 | 5 | 5 | 7 | 61 |
| 4 | 6 | 5 | 7 | 5 | 6 | 7 | 5 | 5 | 6 | 6 | 58 |
| 5 | 6 | 6 | 7 | 4 | 8 | 7 | 6 | 7 | 5 | 8 | 64 |
| 6 | 7 | 5 | 5 | 4 | 8 | 7 | 6 | 7 | 5 | 8 | 62 |
| 7 | 8 | 5 | 6 | 3 | 7 | 6 | 6 | 5 | 6 | 7 | 59 |
| 8 | 6 | 5 | 6 | 5 | 6 | 8 | 6 | 6 | 7 | 8 | 63 |
| 9 | 7 | 7 | 5 | 4 | 6 | 7 | 6 | 5 | 6 | 7 | 60 |
| 10 | 7 | 6 | 6 | 3 | 7 | 6 | 7 | 6 | 7 | 6 | 61 |

Row sums for Media: min 58, max 64 (derived with sum of the 10 values per row)

**Lawman** (p075-p078; roll = 1d10 row)

| Roll | INT | REF | DEX | TECH | COOL | WILL | LUCK | MOVE | BODY | EMP | Sum |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 5 | 6 | 7 | 5 | 7 | 8 | 5 | 6 | 5 | 6 | 60 |
| 2 | 6 | 6 | 6 | 5 | 6 | 8 | 5 | 7 | 5 | 5 | 59 |
| 3 | 5 | 7 | 7 | 7 | 6 | 7 | 5 | 5 | 7 | 6 | 62 |
| 4 | 6 | 6 | 7 | 6 | 6 | 8 | 5 | 7 | 7 | 6 | 64 |
| 5 | 6 | 6 | 7 | 6 | 7 | 7 | 6 | 5 | 5 | 6 | 61 |
| 6 | 7 | 6 | 5 | 5 | 7 | 8 | 5 | 6 | 7 | 4 | 60 |
| 7 | 7 | 8 | 7 | 5 | 6 | 8 | 7 | 6 | 5 | 4 | 63 |
| 8 | 5 | 6 | 6 | 5 | 6 | 8 | 5 | 7 | 6 | 4 | 58 |
| 9 | 7 | 7 | 5 | 5 | 7 | 7 | 6 | 5 | 5 | 6 | 60 |
| 10 | 6 | 6 | 5 | 6 | 8 | 7 | 5 | 7 | 6 | 6 | 62 |

Row sums for Lawman: min 58, max 64 (derived with sum of the 10 values per row)

**Exec** (p075-p078; roll = 1d10 row)

| Roll | INT | REF | DEX | TECH | COOL | WILL | LUCK | MOVE | BODY | EMP | Sum |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 8 | 5 | 5 | 3 | 8 | 6 | 6 | 5 | 5 | 7 | 58 |
| 2 | 8 | 6 | 6 | 4 | 7 | 6 | 7 | 7 | 5 | 7 | 63 |
| 3 | 8 | 7 | 6 | 3 | 8 | 6 | 7 | 6 | 4 | 5 | 60 |
| 4 | 8 | 5 | 7 | 5 | 6 | 5 | 6 | 5 | 5 | 7 | 59 |
| 5 | 7 | 7 | 6 | 5 | 8 | 5 | 7 | 7 | 5 | 6 | 63 |
| 6 | 5 | 7 | 7 | 3 | 6 | 7 | 6 | 5 | 5 | 7 | 58 |
| 7 | 6 | 6 | 7 | 5 | 8 | 7 | 6 | 7 | 4 | 6 | 62 |
| 8 | 6 | 7 | 7 | 3 | 7 | 5 | 7 | 5 | 5 | 7 | 59 |
| 9 | 7 | 6 | 7 | 5 | 7 | 5 | 7 | 6 | 5 | 5 | 60 |
| 10 | 7 | 7 | 5 | 5 | 8 | 6 | 6 | 7 | 4 | 7 | 62 |

Row sums for Exec: min 58, max 63 (derived with sum of the 10 values per row)

**Fixer** (p075-p078; roll = 1d10 row)

| Roll | INT | REF | DEX | TECH | COOL | WILL | LUCK | MOVE | BODY | EMP | Sum |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 8 | 5 | 7 | 4 | 6 | 5 | 8 | 5 | 5 | 8 | 61 |
| 2 | 8 | 5 | 5 | 5 | 6 | 7 | 8 | 7 | 5 | 7 | 63 |
| 3 | 6 | 6 | 6 | 4 | 5 | 6 | 8 | 6 | 3 | 8 | 58 |
| 4 | 7 | 7 | 5 | 5 | 7 | 6 | 7 | 7 | 5 | 8 | 64 |
| 5 | 8 | 6 | 6 | 3 | 6 | 5 | 8 | 7 | 5 | 6 | 60 |
| 6 | 8 | 7 | 5 | 5 | 6 | 7 | 7 | 5 | 3 | 6 | 59 |
| 7 | 8 | 6 | 6 | 5 | 6 | 5 | 6 | 7 | 5 | 8 | 62 |
| 8 | 6 | 6 | 7 | 4 | 7 | 6 | 7 | 7 | 4 | 7 | 61 |
| 9 | 8 | 7 | 7 | 5 | 5 | 5 | 7 | 6 | 5 | 7 | 62 |
| 10 | 6 | 5 | 6 | 5 | 5 | 6 | 8 | 6 | 4 | 7 | 58 |

Row sums for Fixer: min 58, max 64 (derived with sum of the 10 values per row)

**Nomad** (p075-p078; roll = 1d10 row)

| Roll | INT | REF | DEX | TECH | COOL | WILL | LUCK | MOVE | BODY | EMP | Sum |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 6 | 6 | 8 | 3 | 6 | 7 | 6 | 6 | 6 | 4 | 58 |
| 2 | 5 | 7 | 6 | 5 | 8 | 8 | 8 | 7 | 5 | 4 | 63 |
| 3 | 5 | 8 | 6 | 3 | 8 | 7 | 6 | 5 | 6 | 5 | 59 |
| 4 | 5 | 8 | 7 | 4 | 8 | 6 | 7 | 7 | 7 | 5 | 64 |
| 5 | 6 | 6 | 6 | 3 | 6 | 7 | 6 | 7 | 7 | 4 | 58 |
| 6 | 7 | 6 | 8 | 4 | 6 | 7 | 6 | 5 | 6 | 5 | 60 |
| 7 | 6 | 7 | 8 | 4 | 6 | 6 | 7 | 5 | 7 | 5 | 61 |
| 8 | 5 | 7 | 8 | 3 | 8 | 6 | 7 | 5 | 5 | 5 | 59 |
| 9 | 6 | 7 | 6 | 4 | 8 | 6 | 6 | 6 | 6 | 6 | 61 |
| 10 | 5 | 6 | 7 | 4 | 7 | 8 | 7 | 7 | 7 | 4 | 62 |

Row sums for Nomad: min 58, max 64 (derived with sum of the 10 values per row)

Observation: the printed text never says what the template point total is; derived row sums above range wider than Complete Package's 62, so template rows are not constrained to 62. Mark: **derived, not printed**.

### 2b. Method 2: Edgerunner (Fast and Dirty) (p078, p089-p090)

- STATs: use the same Role template table, but "roll 1d10 for each STAT individually, comparing the roll for that STAT against the value on the column for that STAT" (p078). So each of the 10 STATs is an independent 1d10 row lookup within its own column; 10 rolls. Worked example reuses the Solo table.
- Skills: the Role's fixed 20-skill list (13 Basic + 7 Role). "Edgerunners get **86 Skill points**" to distribute; "No Skill can be higher than 6 or lower than 2"; the 13 Basic skills "must be at least Level 2"; "(x2) cost two Skill points to increase in Level by one"; Cultural-Origin Language 4 free (p089). Hot tip text: "each Role has 20 Skills allotted".
- Cost rule from the worked example (p090): cost = Level x (1 or 2), counted from Level 0. Example: "Autofire (x2) Level 2 Cost 4"; "Language (Based on Cultural Origin) 4 Cost 0"; total 86.
- Weapons, armor, outfit, fashion, cyberware: identical to Streetrat; same 500eb.

### 2c. Method 3: Complete Package (Calculated) (p079, p091, p105-p106, p117-p118)

- STAT points by rank of character (p079). Table has 5 rows:

| Rank of Character | STAT Points |
|---|---|
| Minor Supporting Character | 50 |
| Starting Character | 62 |
| Major Supporting Character | 70 |
| Minor Hero | 75 |
| Major Hero | 80 |

  "normally 62"; no STAT above 8 or below 2. Points are spent 1 per STAT point (a flat even split is 6 each = 60, 2 spare; example shows a 62-point build 5 7 8 3 6 / 7 6 8 8 4 = 62). Example check: 5+7+8+3+6+7+6+8+8+4 = 62. **[CMD]** `echo $((5+7+8+3+6+7+6+8+8+4))` -> 62.
- Skills: 86 points, any skill, max Level 6; Basic skills at least 2; (x2) skills cost double; Cultural language 4 free (p091). The worked example shows the Basic floor costing 26 (13 x 2) and "remaining 60" points (86 - 26 = 60, ties to the text "the remaining 60").
- Money: **2,550eb** for weapons, armor, gear, cyberware (p105); unspent is kept. **800eb extra only for Fashion or Fashionware**; unspent "goes away" (p106). Example spent 380, "Left Over (and lost): 420".
- Lifestyle: non-Exec start in a rented Cargo Container, Kibble Lifestyle, first month rent and kibble free, "you owe 1100eb on the 1st of next month, or you're getting evicted on the 2nd". Exec starts rent-free in a Corporate Conapt, Good Prepak Lifestyle, "600eb on the first of every month". Nomads have an extra housing option via the Moto Role Ability (p106 cites Pg. 165; that is `p166`).
- Optional "Running Out of Cash" (p119-p120): at GM discretion, join Covert Military, Organized Crime, or a Corporation. Each grants "1500eb in cyberware in addition to an installation of mandatory Neural Link cyberware, free-of-charge", conditional on the whole table taking it. Catches listed p120: Hostages, Blackmail, Sabotage Cybernetics, Monitored, Company Safeguard, Remote Detonator, Command Kill (7 catches **[HAND]**). Print-only.

### 2d. Starting skills per Role (Streetrat levels, p087-p088)

Basic 13 are common to all (order: Athletics, Brawling, Concentration, Conversation, Education, Evasion, First Aid, Human Perception, Language (Streetslang), Local Expert (Your Home), Perception, Persuasion, Stealth). Levels:

| Role | Ath | Braw | Conc | Conv | Edu | Eva | FAid | HPer | Lang | LocEx | Perc | Pers | Stlth |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Rockerboy | 2 | 6 | 2 | 2 | 2 | 6 | 6 | 6 | 2 | 4 | 2 | 6 | 2 |
| Solo | 2 | 2 | 2 | 2 | 2 | 6 | 6 | 2 | 2 | 2 | 6 | 2 | 2 |
| Netrunner | 2 | 2 | 2 | 2 | 6 | 6 | 2 | 2 | 2 | 2 | 2 | 2 | 6 |
| Tech | 2 | 2 | 2 | 2 | 6 | 6 | 6 | 2 | 2 | 2 | 2 | 2 | 2 |
| Medtech | 2 | 2 | 2 | 6 | 6 | 6 | 2 | 6 | 2 | 2 | 2 | 2 | 2 |
| Media | 2 | 2 | 2 | 6 | 2 | 6 | 2 | 6 | 2 | 6 | 6 | 6 | 2 |
| Lawman | 2 | 6 | 2 | 6 | 2 | 6 | 2 | 2 | 2 | 2 | 2 | 2 | 2 |
| Exec | 2 | 2 | 2 | 6 | 6 | 6 | 2 | 6 | 2 | 2 | 2 | 6 | 2 |
| Fixer | 2 | 2 | 2 | 6 | 2 | 6 | 2 | 6 | 4 | 6 | 2 | 4 | 2 |
| Nomad | 2 | 6 | 2 | 2 | 2 | 6 | 6 | 2 | 2 | 2 | 4 | 2 | 6 |

Role skills (7 per Role) and Streetrat level:

| Role | Role skills (level) |
|---|---|
| Rockerboy | Composition 6, Handgun 6, Melee Weapon 6, Personal Grooming 4, Play Instrument (choose 1) 6, Streetwise 6, Wardrobe & Style 4 |
| Solo | Autofire (x2) 6, Handgun 6, Interrogation 6, Melee Weapon 6, Resist Torture/Drugs 6, Shoulder Arms 6, Tactics 6 |
| Netrunner | Basic Tech 6, Conceal/Reveal Object 6, Cryptography 6, Cybertech 6, Electronics/Security Tech (x2) 6, Handgun 6, Library Search 6 |
| Tech | Basic Tech 6, Cybertech 6, Electronics/Security Tech (x2) 6, Land Vehicle Tech 6, Shoulder Arms 6, Science (choose 1) 6, Weaponstech 6 |
| Medtech | Basic Tech 6, Cybertech 4, Deduction 6, Paramedic (x2) 6, Resist Torture/Drugs 4, Science (choose 1) 6, Shoulder Arms 6 |
| Media | Bribery 6, Composition 6, Deduction 6, Handgun 6, Library Search 4, Lip Reading 4, Photography/Film 4 |
| Lawman | Autofire (x2) 6, Criminology 6, Deduction 6, Handgun 6, Interrogation 6, Shoulder Arms 6, Tracking 6 |
| Exec | Accounting 6, Bureaucracy 6, Business 6, Deduction 6, Handgun 6, Lip Reading 6, Personal Grooming 4 |
| Fixer | Bribery 6, Business 6, Forgery 6, Handgun 6, Pick Lock 4, Streetwise 6, Trading 6 |
| Nomad | Animal Handling 6, Drive Land Vehicle 6, Handgun 6, Melee Weapon 6, Tracking 6, Trading 6, Wilderness Survival 6 |

Row counts: 10 Roles x 20 skills = 200 rows; Edgerunner lists are the same 200 skill names without levels (p089-p090). **[CMD]** the Edgerunner list appears twice per Role page group; no command used for the 200 (**[HAND]**).

**Transcription check (this is the evidence the above is right, and its limit).** p087-p088 are column-interleaved (see section 7). I reconstructed them by hand and checked each Role total with the printed cost rule (Level x1, or x2 for (x2) skills; Cultural Language excluded): every one of the 10 Roles totals exactly **86**. **[CMD]** `python3 -I $S/sr.py` prints `86` for all ten. That proves the sums match; it does **not** prove each level sits in the right Role column. The per-skill column assignment for the garbled rows (Conceal/Reveal Object, Electronics/Security Tech, Cybertech split between Tech and Medtech) rests on the Edgerunner lists on p089 matching as sets, which they do. A misplacement between two Roles that preserves each Role's sum would pass. Re-extract with `pdftotext -x/-y` crops before shipping.

Printed worked example for Solo (p088): Athletics 2, Brawling 2, Concentration 2, Conversation 2, Education 2, Evasion 6, First Aid 6, Human Perception 2, Language (Streetslang) 2, Local Expert (Your Home) 2, Perception 6, Persuasion 2, Stealth 2, Autofire 6, Handgun 6, Interrogation 6, Melee Weapon 6, Resist Torture/Drugs 6, Shoulder Arms 6, Tactics 6. This matches the Solo row above (good independent check on one Role).

### 2e. Starting gear per Role (Streetrat/Edgerunner)

These three tables are garbled in the shipped text (section 7). I re-extracted them from the PDF with column crops (`pdftotext -f N -l N -x X -y Y -W W -H H` and `-bbox` word positions), not from `pages/`. Treat as **[CMD-reproducible only via the PDF]**.

**Weapons and armor (p099)** ("pick one and only one when offered an option"):

| Role | Items |
|---|---|
| Rockerboy | Very Heavy Pistol; Basic VH Pistol Ammunition x50; Heavy Melee Weapon or Flashbang Grenade **[UNCLEAR: the "or" wraps a line]**; Teargas Grenade x2; Light Armorjack Body Armor (SP11); Light Armorjack Head Armor (SP11) |
| Solo | Assault Rifle; Very Heavy Pistol; Heavy Melee Weapon or Bulletproof Shield; Basic VH Pistol Ammunition x30; Basic Rifle Ammunition x70; Light Armorjack Body (SP11); Light Armorjack Head (SP11) |
| Netrunner | Very Heavy Pistol; Basic VH Pistol Ammunition x30; Light Armorjack Body (SP11); Light Armorjack Head (SP11) |
| Tech | Shotgun or Assault Rifle; Basic Shotgun Shell Ammunition x100 or Basic Rifle Ammunition x100; Flashbang Grenade; Light Armorjack Body; Light Armorjack Head |
| Medtech | Shotgun or Assault Rifle; Basic Shotgun Shell Ammunition x100 or Basic Rifle Ammunition x100; Incendiary Shotgun Shell Ammunition x10 or Incendiary Rifle Ammunition x10; Smoke Grenade x2; Light Armorjack Body; Light Armorjack Head; Bulletproof Shield |
| Media | Heavy Pistol or Very Heavy Pistol; Basic H Pistol Ammunition x50 or Basic VH Pistol Ammunition x50; Light Armorjack Body; Light Armorjack Head |
| Lawman | Assault Rifle or Shotgun; Heavy Pistol; Basic Rifle Ammunition x100 or Basic Shotgun Shell Ammunition x100 or Basic Slug Ammunition x100; Basic H Pistol Ammunition x30; Bulletproof Shield or Smoke Grenade x2; Light Armorjack Body; Light Armorjack Head |
| Exec | Very Heavy Pistol; Basic VH Pistol Ammunition x50; Light Armorjack Body; Light Armorjack Head |
| Fixer | Heavy Pistol or Very Heavy Pistol (twice, two separate picks); Light Melee Weapon; Basic H Pistol Ammunition x100 or Basic VH Pistol Ammunition x100; Light Armorjack Body; Light Armorjack Head |
| Nomad | Heavy Pistol or Very Heavy Pistol; Basic H Pistol Ammunition x100 or Basic VH Pistol Ammunition x100; Heavy Melee Weapon or Heavy Pistol; Light Armorjack Body; Light Armorjack Head |

Other lines to check: Netrunner has no melee/grenade line; Solo example output on p100 resolves to Assault Rifle, Very Heavy Pistol, Heavy Melee Weapon, Basic VH Pistol Ammo x30, Basic Rifle Ammo x70, Light Armorjack Body and Head (matches). Armor "Light Armorjack ... (SP11)" is Body and Head as separate items.

**Outfit / gear / fashion (p104)** (rows reconstructed by word x-positions; "Agent" appears for 9 Roles, **Exec's column prints no Agent [UNCLEAR: printed omission or extraction drop]**):

| Role | Items |
|---|---|
| Rockerboy | Agent; Computer; Electric Guitar or Bug Detector; Glow Paint x5; Pocket Amp; Radio Scanner/Music Player; Video Camera; Generic Chic: Jacket, Jewelry x3, Top x4; Leisurewear: Jewelry, Mirrorshades, Footwear; Urbanflash: Bottoms, Top |
| Solo | Agent; Leisurewear: Footwear x2, Jacket x3, Mirrorshades, Bottoms x2, Top x2 |
| Netrunner | Agent; Cyberdeck (7 Slots); Virtuality Goggles; Program: Armor; Program: Sword; Program: See Ya or Eraser; Program: Sword or Vrizzbolt; Program: Worm or Sword; Generic Chic: Top x10; Leisurewear: Footwear x2, Jewelry, Bottoms x2; Urban Flash: Jacket |
| Tech | Agent; Anti-Smog Breathing Mask; Disposable Cell Phone; Duct Tape x5; Flashlight; Road Flare x6; Tech Bag; Generic Chic: Bottoms x8, Tops x10; Leisurewear: Footwear x2 |
| Medtech | Agent; Airhypo; Handcuffs; Flashlight; Generic Chic: Jacket x3; Glow Paint; Medtech Bag; Leisurewear: Footwear, Bottoms x3, Top x5 |
| Media | Agent; Audio Recorder; Binoculars; Disposable Cellphone x2 or Grapple Gun; Flashlight; Computer; Radio Scanner/Music Player; Scrambler/Descrambler; Video Camera; Generic Chic: Footwear, Bottoms, Top; Leisurewear: Jacket; Urbanflash: Mirrorshades |
| Lawman | Agent; Flashlight; Handcuffs x2; Radio Communicator; Road Flare x10; Generic Chic: Jacket, Bottoms x2, Top x3; Leisurewear: Footwear x2, Jacket x2, Bottoms x2, Mirrorshades, Top x2 |
| Exec | Radio Communicator x4; Scrambler/Descrambler; Businesswear: Footwear, Jacket, Bottoms, Mirrorshades, Top, Jewelry x2 |
| Fixer | Agent; Bug Detector; Computer; Disposable Phone x2; Generic Chic: Contacts, Jewelry; Leisurewear: Mirrorshades; Urbanflash: Footwear, Jacket, Bottoms, Top |
| Nomad | Agent; Anti-Smog Breathing Mask; Duct Tape; Flashlight; Grapple Gun; Inflatable Bed & Sleep-Bag; Medtech Bag; Radio Communicator x2; Rope; Techtool; Tent and Camping Equipment; Bohemian: Jewelry; Nomad Leathers: Top x4, Bottom x2, Footwear x2, Jacket, Hat |

Two possible extraction slips to confirm against the PDF image: Rockerboy "Jewelry x3" attaches to Jewelry and not to Jacket; Medtech holds "Handcuffs".

**Cyberware (p118)** (re-extracted by word x-positions; "pick one" where "or"):

| Role | Cyberware | Printed Humanity Loss | Printed EMP change |
|---|---|---|---|
| Rockerboy | Audio Recorder; Chemskin; Cyberaudio Suite; Tech Hair | 9 | -1 |
| Solo | Biomonitor; Neural Link; Sandevistan Speedware or Wolvers | 14 | -2 |
| Netrunner | Interface Plugs; Neural Link; Shift Tacts | 14 | -2 |
| Tech | Cybereye; MicroOptics; Skinwatch; Tool Hand | 12 | -2 |
| Medtech | Biomonitor; Cybereye; Nasal Filters or Toxin Binders; TeleOptics | 12 | -2 |
| Media | Amplified Hearing or Voice Stress Analyzer; Cyberaudio Suite; Light Tattoo | 10 | -2 |
| Lawman | Hidden Holster; Subdermal Pocket | 10 | -2 |
| Exec | Biomonitor or Tech Hair; Cyberaudio Suite; Internal Agent; Toxin Binders or Nasal Filters | 12 | -2 |
| Fixer | Cyberaudio Suite; Internal Agent; Subdermal Pocket; Voice Stress Analyzer or Amplified Hearing | 16 | -2 |
| Nomad | Interface Plugs or Wolvers; Neural Link | 14 | -2 |

**Printed-value conflict (must not be trusted blind): Media and Lawman print HL 10 with "-2 EMP".** Using the p081/p230 rule (EMP = tens place of Humanity, Humanity = EMP x 10 at start), losing 10 from any starting multiple of 10 lowers EMP by exactly 1, not 2. HL 9 is "-1" (correct), HL 12/14/16 are "-2" (correct), HL 10 as "-2" is not. **Recommendation: compute EMP from Humanity, do not print the "(-2 EMP)" strings.** **[CMD]** reasoning check only: `for hl in 9 10 12 14 16; do echo "$hl $((50/10 - (50-hl)/10))"; done` -> `9 1, 10 1, 12 2, 14 2, 16 2` (EMP 5 start).

The per-item HL values behind these sums are on p112-p117 (Cyberware chapter), outside this domain; I did not verify each role's HL sum against its items.

---

## 3. Lifepath (p045-p054 general; p054-p070 Role-based)

p045 explains the Lifepath as "a flowchart of 'plot complications'", "intended primarily as a guide; if you encounter something you don't think fits ... feel free to change the path". Each table is "Roll 1d10 or choose one" (or 1d6 for Role tables). Only one result is mechanical: **Cultural Origin -> pick a Language and take it at Skill Rank 4** (p046, p042). Everything else is background text with no stat or skill effect.

### 3a. General Lifepath: 15 printed tables, 19 d10 columns

**[CMD]** printed roll-instruction lines per page: `for p in 046 047 048 049 050 051 052 053 054; do grep -o -E "Roll 1d(6|10)( and subtract 7)?|1d10 and subtract 7|1d10 or choose|Roll 1d10" P/p$p.txt | wc -l; done` -> `1 1 2 3 1 2 2 2 1` = 15. Columns (19) and rows (186) are **[HAND]**.

| # | Table | Dice | Rows | Page | Mechanical? |
|---|---|---|---|---|---|
| 1 | Cultural Origins (region -> language list) | 1d10 | 10 regions, 5-9 languages each | p046 | **Yes**: Language Skill Rank 4 in a chosen language |
| 2 | Your Personality | 1d10 | 10 | p047 | print |
| 3 | Clothing Style | 1d10 | 10 | p048 | print |
| 4 | Hairstyle (same box as 3) | 1d10 | 10 | p048 | print |
| 5 | Affectation You Are Never Without | 1d10 | 10 | p048 | print |
| 6 | What Do You Value Most? | 1d10 | 10 | p049 | print |
| 7 | How Do You Feel About Most People? | 1d10 | 10 | p049 | print |
| 8 | Most Valued Person | 1d10 | 10 | p049 | print |
| 9 | Most Valued Possession | 1d10 | 10 | p049 | print |
| 10 | Original Family Background (name + paragraph) | 1d10 | 10 | p050 | print |
| 11 | Childhood Environment | 1d10 | 10 | p051 | print |
| 12 | Family Crisis | 1d10 | 10 | p051 | print |
| 13 | Friends: count = 1d10 minus 7 (min 0); each friend rolls Relationship table | 1d10, 1d10 | 10 | p052 | print |
| 14 | Enemies: count = 1d10 minus 7 (min 0); each enemy rolls three columns: Enemy, What Caused It / Who's Been Wronged (choose), What Can They Throw at You | 1d10, 3 cols | 10 each (30 cells) | p052 | print (includes dice-in-text "1d6/2", "1d10/2", "1d10 + 5") |
| 15 | Sweet Revenge! | 1d10 with ranges | **6 rows**: 1-2, 3-4, 5-6, 7-8, 9, 10 | p053 | print |
| 16 | Tragic Love Affair(s): count = 1d10 minus 7 (min 0); then What Happened? | 1d10, 1d10 | 10 | p053 | print |
| 17 | Your Life Goals | 1d10 | 10 | p054 | print |

(Rows 3 and 4 share one printed box, as do rows 6 and 7, which is why 17 listed rows come from 15 printed tables; the enemy table has 3 columns, so the d10 columns number 19: Cultural, Personality, Clothing, Hair, Affectation, Value, Feel, Person, Possession, Family, Environment, Crisis, Friend, Enemy, Cause, Threat, Revenge, Love, Goals.)

Count rule ("subtract 7, minimum 0") is on p052 and p053. Rows: 18 columns x 10 + Sweet Revenge 6 = 186 **[HAND]**.

Sheet fields for the Lifepath answers exist on p121 (Cultural Origins, Personality, Clothing Style, Hairstyle, Affectation, What Do You Value Most, Feelings About People, Most Valued Person, Most Valued Possession, Family Background, Childhood Environment, Family Crisis, Life Goals; friends/enemies/love are free-text in a Lifepath notes area). Names scrambled in the extraction; **[UNCLEAR]** whether friends/enemies/love affairs have dedicated sheet boxes.

### 3b. Role-based Lifepath (10 Roles, 52 rolled tables)

Index p054: Rockerboy p054 (printed) -> `p055`, then Solo `p056`, Netrunner `p057-p058`, Tech `p059-p060`, Medtech `p061-p062`, Media `p063`, Exec `p064-p065`, Lawman `p066`, Fixer `p067-p068`, Nomad `p069-p070`. All print-only (no mechanical effect). Exception noted on p060: the rolled **Workspace** (Netrunner, Tech, Medtech roll one) is *not* Housing; sleeping there "counts as if they had slept in a vehicle" for the good-night's-sleep check. That is a play-time rule, print only.

Rolled-table count **[CMD]**: `n=0; for p in 055 056 057 058 059 060 061 062 063 064 065 066 067 068 069 070; do c=$(grep -o -E "Roll 1d(6|10)" P/p$p.txt | wc -l); n=$((n+c)); done; echo $n` -> `52`. Per page: p055 4, p056 4, p057 3, p058 3, p059 3, p060 3, p061 3, p062 2, p063 4, p064 3, p065 3, p066 5, p067 3, p068 2, p069 4, p070 3.

| Role | Tables (dice, rows) | Pages |
|---|---|---|
| Rockerboy | Kind (1d10, 10); Who's Gunning (1d6, 6); Where You Perform (1d6, 6); Group or Solo Act (choose); Once in a Group? (choose, only for solo); Why You Left (1d6, 6) | p055 |
| Solo | Kind (1d6, 6); Moral Compass (1d6, 6); Who's Gunning (1d6, 6); Operational Territory (1d6, 6) | p056 |
| Netrunner | Kind of Runner (1d6, 6); Partner or Alone (choose); Workspace (1d6, 6); Partner Who (1d6, 6); Other Clients (1d6, 6); Where Programs From (1d6, 6); Who's Gunning (1d6, 6) | p057-p058 |
| Tech | Kind (1d10, 10); Workspace (1d6, 6); Partner or Alone (choose); Partner Who (1d6, 6); Main Clients (1d6, 6); Where Supplies From (1d6, 6); Who's Gunning (1d6, 6) | p059-p060 |
| Medtech | Kind (1d10, 10); Partner or Alone (choose); Partner(s) (1d6, 6); Workspace (1d6, 6); Main Clients (1d6, 6); Where Supplies From (1d6, 6) | p061-p062 |
| Media | Kind (1d6, 6); How Work Reaches Public (1d6, 6); Ethics (1d6, 6); Story Types (1d6, 6) | p063 |
| Exec | Corp Kind (header "1d6", **10 rows** [UNCLEAR: d6 vs d10]); Division (1d6, 6); How Good/Bad (1d6, 6); Where Based (1d6, 6); Who's Gunning (1d6, 6); State with Boss (1d6, 6) | p064-p065 |
| Lawman | Position (1d6, 6); Jurisdiction (1d6, 6); How Corrupt (1d6, 6); Who's Gunning (1d6, 6); Major Target (1d6, 6) | p066 |
| Fixer | Kind (1d10, 10); Partner or Alone (choose); Partner Who (1d6, 6); Office (1d6, 6); Side Clients (1d6, 6); Who's Gunning (1d6, 6) | p067-p068 |
| Nomad | Pack Size (1d6, 6); Land/Air/Sea (choose); If Land (1d10, 10); If Air (1d6, 6); If Sea (1d6, 6); Your Job in Pack (1d6, 6); Pack Philosophy (1d6, 6); Who's Gunning (1d6, 6) | p069-p070 |

Row totals per Role **[HAND]**: Rockerboy 28, Solo 24, Netrunner 36, Tech 40, Medtech 34, Media 24, Exec 40, Lawman 30, Fixer 34, Nomad 46; sum **336**. Choose-only boxes (7 of them) have no table rows. These are ESTIMATE-grade; there is no machine check.

---

## 4. Skills at creation, mandatory floor, cost, money, cyberware (summary)

| Item | Streetrat | Edgerunner | Complete Package | Page |
|---|---|---|---|---|
| Skill points | fixed levels (each Role totals 86 by cost rule) | **86** | **86** | p087-p091 |
| Skills available | the Role's fixed 20 | the Role's fixed 20 | any of 66 | p089, p091 |
| Level range | as printed (2, 4, or 6) | 2-6 per listed skill | max 6; 13 Basic at least 2 | p089, p091 |
| (x2) cost | counted in the 86 | 2 points per Level | 2 points per Level | p082, p089, p091 |
| Free Cultural Language | 4 | 4 | 4 | p042, p046 |
| Role Ability rank | 4 | 4 | 4 | p042-p043, p143 |
| Extra money | 500eb to spend or keep | 500eb | 2,550eb (all gear and cyberware) + 800eb fashion only | p099, p105, p106 |
| Gear | fixed | fixed | bought | p099, p104, p105 |
| Cyberware | fixed per Role | fixed per Role | bought from the 2,550eb (or +1,500eb and free Neural Link via employer option) | p118, p119 |
| Cyberware slot limit | n/a | n/a | "you can still only implant 7 Option Slots worth" for Fashionware/Internal/External (p112) | p112 |
| Humanity loss at creation | preset per Role (section 2e) | same | preset per item; "At Character Generation, Humanity Loss is preset" | p112, p118 |
| Installation cost | included | included | "Installation surgery is always included at no extra charge, including at Character Generation" | p111 |
| Lifestyle | Cargo Container/Kibble (Exec: Good Prepak) | same | same | p106 |

Basic Skills (13) "must start with at least a value of 2" (p086). 13 x 2 = 26 points spent before anything else for Edgerunner/Complete Package.

Master skill list: **66 skills in 9 categories, 7 marked (x2)**. **[CMD]** `cat P/p083.txt P/p084.txt P/p085.txt P/p086.txt | awk '{n=gsub(/\.\.\.\.+ *(INT|REF|DEX|TECH|COOL|WILL|EMP)/,"&"); t+=n; x+=gsub(/\(x2\)\.\.\.\.+/,"&")} END{print t, x}'` -> `66 7`. Second implementation (sheet labels on p121): `sed -e 's/  */ /g' P/p121.txt | grep -o -E "[A-Za-z/& ]+(\(x2\))? \((INT|REF|DEX|TECH|COOL|WILL|EMP)\)" | wc -l` -> `66`.

Category counts **[HAND]**, from reading p083-p086 (they sum to 66): Awareness 5, Body 6, Control 4, Education 16, Fighting 4, Performance 2, Ranged Weapon 5, Social 9, Technique 15. (x2): Pilot Air Vehicle, Martial Arts, Autofire, Heavy Weapons, Demolitions, Electronics/Security Tech, Paramedic. Stealth is a Body skill (it is the first entry of the right column on p083, which reads as part of Awareness in the raw text but the sheet on p121 puts it under Body).

Linked STATs for each skill are printed with dot leaders on p083-p086 and on the p121 sheet labels; the full list lives in the Roles/skills inventory if another teammate owns it. Not repeated here.

Skills with a "choose" specialization: Language, Local Expert, Science, Play Instrument, Martial Arts (form), Streetslang is the default Language rows. The Edgerunner/Streetrat lists carry "Play Instrument (choose 1)" and "Science (choose 1)" as printed.

Basic 13: on the sheet, "Streetslang" sits next to the Language row and "Your Home" next to Local Expert (p121).

---

## 5. Reputation and Improvement Points

### Reputation (p194-p195)

- Scale: Levels 1-10, 10 rows, each a "Who Knows About You" sentence (p194). "Most Cyberpunk RED Characters start with a Reputation of 0" (p194).
- Awarding: GM assigns a Level for a defining deed. "A new deed's Reputation will only replace the old one if the Level is higher." A negative deed (cowardice, desertion, betrayal) "can replace the Reputation of a positive event" (p194).
- Recognition: "Characters roll 1d10. A Character that rolls under the Reputation Level of the person they are meeting has heard of them" (p194).
- Facedown (p195): both sides roll **COOL + Reputation + 1d10**; "If your defining Reputation Event is for cowardice your Reputation Level is treated as a negative number." Tie: nothing happens. Loser backs down or takes -2 to future Actions against that opponent until beaten once.
- Sheet fields: Reputation box with "events" (p121 labels "REPUTATION ... EVENTS"). **[UNCLEAR]** whether the sheet has a single numeric Reputation field plus a free-text events list; the extraction scrambles it.
- Print vs compute: Reputation Level and the defining event are user-entered; the only candidate derived total is Facedown = COOL + Reputation (sign flips if the event is cowardice). Recommendation per the standing sheet rule: compute it only if the sheet prints a Facedown total; otherwise print Level, event, and the Facedown rule text.

### Improvement Points (p409-p412)

- Intro (p409): "Players can improve their Skills (or begin new ones) and Role Abilities by accumulating ... Improvement Points (I.P.)". IP can raise a Skill Level or a Role Ability Rank. **There is no IP cost for raising STATs in p409-p412.** **[CMD]** `grep -c -i "stat" P/p409.txt P/p410.txt P/p411.txt P/p412.txt` -> `0` on all four pages. This covers only those four pages, not the whole book.
- Award tables (p411-p412): Playstyle Improvement Point Table, 8 rows (I.P. Earned 10,20,30,40,50,60,70,80) x 5 columns (Group, Warrior, Socializer, Explorer, Roleplayer) = 40 cells. Garbled by two-column reading (section 7). Playstyle quiz on p410. All GM-run; print-only.
- Spend (p412):

| Level / Rank reached | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |
|---|---|---|---|---|---|---|---|---|---|---|
| Typical Skill I.P. cost | 20 | 40 | 60 | 80 | 100 | 120 | 140 | 160 | 180 | 200 |
| Difficult (x2) Skill I.P. cost | 40 | 80 | 120 | 160 | 200 | 240 | 280 | 320 | 360 | 400 |
| Role Ability Rank I.P. cost | 60 | 120 | 180 | 240 | 300 | 360 | 420 | 480 | 540 | 600 |

  Printed formulas: skill cost = 20 x next Level (40 x for (x2)), Role Ability = 60 x next Rank. "You can't skip Levels/Ranks." The table header reads "Level ... I.P. Cost" and the text says cost is "based on the Skill or Ability's next Level/Rank", so the number in the column is the cost to reach that Level. **[UNCLEAR]** only whether Level 1 means "buy a new skill at Level 1"; the text says "(or begin new ones)", which supports yes.
- Sheet: an Improvement Points box (p121: "IMPROVEMENT POINTS ... out of"). Spent vs total shown as "out of" — likely current/total **[UNCLEAR]**. The only compute candidate is "IP cost to next level" per skill, but that is not a sheet total; recommend print-only.

---

## 6. Counts with commands (summary)

| Figure | Value | Command / status |
|---|---|---|
| Streetrat STAT rows incl. duplicates | 110 | `cat p075..p078 \| awk '/^ *[0-9]+( +[0-9]+){10} *$/ {c++} END{print c}'` |
| Unique template rows (10 Roles x 10) | 100 | `python3 -I $S/gen.py` prints `rows=[10,10,10,10,10,10,10,10,10,10]` |
| HP table cells vs formula | 126 / 0 mismatches | `gen.py`; row count `awk '/^ *[0-9]+( +[0-9]+){14} *$/ {c++} END{print c}' p080.txt` -> 9 |
| Skills | 66 (7 x2) | awk on p083-p086 and sed/grep on p121 (two implementations agree) |
| Streetrat Role totals = 86 each | 10 / 10 | `python3 -I $S/sr.py` |
| General Lifepath printed tables | 15 | grep loop above |
| Role Lifepath rolled tables | 52 | grep loop above |
| Role Lifepath rows, general rows (186), Role rows (336) | | **[HAND] ESTIMATE** |
| Basic skills | 13 | listed p086; **[HAND]** |
| Role skills per Role | 7 (Basic 13 + 7 = 20) | **[HAND]** from p087-p088, p089-p090 |

`$S` = `/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/f61b82f6-6418-42af-8873-d3df5da582e6/scratchpad` (scripts `gen.py`, `sr.py` live there; `gen.py` also wrote `stat_tables.md`).

---

## 7. Text-extraction quality

| Page file | Problem | How handled |
|---|---|---|
| p073 | Two-column page; group headings (Mental/Combat/Fortune/Physical) land mid-column; reading order broken | Read column by column; STAT list matches the 10-name block on p074 |
| p074 | STAT block header clean; example table duplicates Solo | fine |
| p075-p078 | Clean tabular rows (11 integers) | parsed by regex |
| p078 | Edgerunner example repeats Solo table (10 duplicate rows) | counted out |
| p079-p080 | Two-column flow interleaves text; HP table has one header word "BODY" and a left label "WILL" separated | parsed 9 x 14 numeric rows; verified against formula |
| p081 | Clean | fine |
| p083-p086 | Two-column skill list: descriptions of one skill interleave with the next; Stealth sits at the head of the right column on p083 | counted by the dot-leader pattern, cross-checked with p121 sheet labels |
| **p087-p088** | Streetrat skill grid: 5 Roles per page, columns interleaved, long names wrap across rows ("Conceal/Reveal Object", "Electronics/Security Tech (x2)"), Language/Local Expert rows wrap | hand-reconstructed; every Role total is 86 (cost rule); **per-column placement not independently proven** |
| p089-p090 | Edgerunner skill lists: same interleave, wrapped names, (x2) on separate line | same method; sets match p087 |
| **p099** | Weapons/armor grid with 5 columns; lines of different Roles interleave in the shipped text | re-extracted from the PDF by column crop |
| **p104** | Outfit/fashion grid interleaved | re-extracted from the PDF by word x-position |
| **p118** | Cyberware grid, plus a sidebar | re-extracted from the PDF by word x-position |
| p102 | Gear table fine, but one cost prints "10b (Cheap)" (typo for 10eb) | note only |
| p103 | Fashion table columns clean | fine |
| p110/p111 | Fine | fine |
| p121 | Character sheet labels scrambled; field order unknown | used only as a label inventory |
| p230 | File content is a different page order (drug text interleaved with therapy text, header "TRAUMA TEAM"); Humanity rule is the left column | read the Humanity rule from the left column |
| p054-p070 | Sidebars (Lifepath Example, About Workspaces, Execs and Lawmen in a Punk Game, Nomads in a City Game) interleave with tables on p058, p060, p065, p070 | tables read from the left; sidebar text ignored |
| p055, p057, p059, p061, p067, p069 | Branch tags ("group", "solo", "work alone", "got a partner", "air", "sea") float in text and are not attached to tables | the branch logic is implied by the heading text; verify against the PDF |
| p052 | Enemies table: three table columns plus the first column; row text wraps across 2-3 lines | rows paired by number |
| p410-p412 | Playstyle table: 5 columns of wrapped paragraphs interleaved | cell text not reliable; only the 10..80 row labels are |

Page-offset caveat: the in-book cross-references ("see page 45") are printed pages; add 1 for the pNNN file.

---

## 8. Print vs compute split (standing rule: only sheet-total-feeding values are computed; rules are printed)

**Compute (feed sheet totals):**
- Ten STAT values (entered or selected), with EMP kept as maximum and current.
- HP = 10 + 5 x ceil((BODY+WILL)/2).
- Seriously Wounded threshold = ceil(HP/2).
- Death Save = BODY.
- Humanity max = EMP x 10; current EMP = floor(current Humanity / 10).
- Skill Base = Level + linked STAT, for each of the 66 skills.
- Role Ability rank starts at 4.
- Fixed Streetrat levels, Language rank 4 for the chosen Cultural Origin language.
- For Complete Package/Edgerunner: skill point spend = sum of Level x (1 or 2) must equal 86 less the free Cultural Language; STAT points = 62 (or the chosen rank value).
- Optional, only if the sheet prints them: MOVE distance = MOVE x 2 m/yds; Initiative base = REF; LUCK pool = LUCK; Facedown base = COOL + Reputation.

**Print only (text rules, no engine):**
- All Lifepath results (except the Cultural Origin language, which creates a skill row).
- All Lifepath tables, Friends/Enemies/Love-affair counts and results, Sweet Revenge.
- Role-based Lifepath tables and the Workspace/Housing note.
- Reputation scale text, recognition roll, Facedown rule, Reputation event.
- Wound-state text, Death Save procedure, cyberpsychosis note.
- Improvement Point award tables, playstyle quiz, IP cost tables (print the table; do not compute progress).
- Complete Package "Running out of Cash" options and their catches.
- Fixed gear/fashion/cyberware lists per Role (data for the starting inventory; gear item rules live elsewhere).
- Printed Humanity-loss "EMP" parentheticals on p118: **do not print**; they conflict with the formula for HL 10.

---

## 9. Open items for the lead

1. **Printed-value conflict:** Media and Lawman cyberware lists print "10 Humanity Loss (-2 EMP)", which contradicts the Humanity rule; compute instead.
2. **Exec outfit prints no Agent;** confirm against the PDF image.
3. **Rockerboy weapon line** "Heavy Melee Weapon or Flashbang Grenade" is an inferred pairing across a line wrap.
4. **Exec Corp Kind table** says 1d6 but has 10 rows.
5. **Not covered by this proof:** the 100 template rows and 86-sum check do not prove that each skill level sits under the right Role column; the gear and cyberware tables were re-extracted by column crop and not visually diffed against the PDF; Lifepath row counts are hand counts.
