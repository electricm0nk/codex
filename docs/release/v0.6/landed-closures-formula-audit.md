# Landed ACG/APG Closures — Formula Audit (Read-Only Verification Pass)

> Directed by the lead after the Investigator "swapped floors" catch and
> the Familiar-vs-Animal-Companion mislabel: do a systematic pass over
> every already-landed ACG/APG class-specific closure, independently
> re-deriving each numeric formula directly from its own real corpus
> record (not from a doc's claim, not by copying a sibling class's
> formula) and confirming it matches what's implemented in
> `pilot_compute.rs`. Read-only — report discrepancies for the lead's
> review, don't propose fixes. This doc records the full pass.

## Headline

**One real shipped correctness bug found** (Warpriest Sacred Weapon dice
count — reported to the lead immediately on discovery, per the "don't
batch shipped bugs" instruction). Everything else re-derived matches the
corpus exactly, with two evidentiary/scope notes worth recording (the
Arcanist/Warpriest spell-slot tables are not corpus-derivable; two values
are correct-per-RAW but lack a corpus `BONUS` backing).

## FINDING 1 — Warpriest Sacred Weapon dice COUNT uses the wrong branch's denominator (REAL BUG)

- **Corpus** (`acg_abilities_class.lst`, `KEY:Warpriest ~ Sacred Weapon`),
  the Medium-weapon branch `PREBASESIZEEQ:M` — the branch that applies to
  this codebase's Longsword/Medium fixture:
  `BONUS:VAR|WarpriestSacredWeaponBaseDice|1+min(1,WarpriestSacredWeaponLVL/15)`
  → **`/15`**.
- **Code** (`pilot_compute.rs:9886`, `warpriest_sacred_weapon_base_dice`):
  `let dice_count = 1 + 1.min(level / 20);` → **`/20`**.
- The `/20` is the corpus's *smaller-than-Medium* branch
  (`PREBASESIZELT:M`, `1+min(1,…LVL/20)`) — a sibling-branch transcription
  error. The function's own doc comment (line 9866) also cites `/20` as if
  corpus-verified, so a doc-vs-code check would pass; only re-deriving from
  the raw corpus catches it. (Dice **size** is correct: the code's
  `if(<5,6,if(<10,8,if(<15,10,if(<20,6,8))))` matches the EQ:M branch
  exactly. Only the **count** denominator is wrong.)
- **Impact — active, not latent.** `level` is the real character level
  (1–20, passed through `compute_acg_class_chassis`); Warpriest's
  `MAX_SUPPORTED_LEVEL = 20`. Correct Medium progression: 1d6 (1–4),
  1d8 (5–9), 1d10 (10–14), **2d6 (15–19)**, 2d8 (20). The code emits
  **1d6 at levels 15–19** (dice count 1 vs 2). The value surfaces in the
  `class_feature.acg.warpriest.sacred_weapon_base_damage_die` explanation
  detail text (`{count}d{size}`).
- **Severity: low–moderate.** A flat informational explanation, not
  integrated into any attack/damage total (this engine computes none), and
  divergent only in the 15–19 band. But it is a factual error in a formula
  the code claims corpus-verified. **Fix would be `/20` → `/15`** in both
  the function body and its doc comment (lead's call — I'm read-only).

## FINDING 2 — Arcanist & Warpriest per-day spell tables are NOT corpus-derivable (scope note, not a bug)

Both classes declare only a shared spell **list** (`SPELLLIST:1|Wizard`
for Arcanist, `SPELLLIST:1|Cleric` for Warpriest) — **neither carries any
per-level `CAST:`/`KNOWN:` rows anywhere in the ACG corpus** (verified:
`grep -rE "Arcanist|Warpriest" advanced_class_guide/*.lst | grep "CAST:[0-9]"`
→ empty; each class block is 3 lines with no level table). So their
spells-per-day tables — hand-transcribed in code as:

- Arcanist (`arcanist_base_spells_per_day`): L1 `[4,2]`, L2 `[5,2]`,
  L3 `[5,3]` (bounded 1–3);
- Warpriest (`warpriest_base_spells_per_day`): claimed to match Cleric at
  1–2, diverge at 3+;

**cannot be re-derived from the corpus.** They rest on external-source
transcription (the published tables), which both closures' own report rows
explicitly acknowledge ("own independently-verified per-day table",
"verified via two independent sources"). This is consistent with what was
claimed — I simply cannot add a corpus confirmation. **Recommendation:**
if not already done against a durable source, the Arcanist `4/2`-at-level-1
figure is worth a second look — it is notably richer than the Wizard
table it shares a list with (Wizard L1 `3/1`), and "prepared caster that
prepares more low-level slots than a Wizard" is a surprising-enough claim
to be worth a citation the lead trusts. Flagging as the one numeric claim
in the audit I could neither confirm nor refute from the corpus, not as a
known error.

## FINDING 3 — Two values are correct-per-RAW but have no corpus `BONUS` backing (evidentiary note)

Neither is a discrepancy (the corpus is silent, not contradictory), but
both are a weaker evidentiary path than a literal `BONUS:VAR`, worth
recording the same way the Swashbuckler closure honestly flagged Panache:

- **Swashbuckler Panache** — `max(1, CHA)` (`swashbuckler_panache_max`).
  The base class record carries no `BONUS:VAR`; the value comes from the
  DESC text. Already flagged in that closure's own report — recorded here
  for completeness, matches the DESC ("Charisma modifier, minimum 1").
- **Skald Inspired Rage AC penalty** — `SKALD_INSPIRED_RAGE_ARMOR_CLASS_PENALTY = -1`.
  The `KEY:Skald ~ Inspired Rage` corpus record carries only the Str/Con
  and Will `BONUS:VAR` tokens — **no AC-penalty `BONUS` at all**. The `-1`
  is correct per published Inspired Rage RAW ("–1 penalty to Armor
  Class"), and is correctly differentiated from Bloodrager/Barbarian's
  `-2`, but is rule-derived, not corpus-backed. Low stakes (this engine
  computes no player AC total, so it is informational only).

## Everything else — re-derived from the corpus, exact matches

Each below was extracted from the class's own real `BONUS:VAR` (or
equivalent) record and compared to the implementing code; all match.

| Class | Feature | Corpus | Code | Match |
|---|---|---|---|---|
| **Slayer** | Sneak Attack dice | `SneakAttackDice\|SlayerLVL/3` | `level/3` | ✓ |
| | Trap Sense | `max(1,SlayerTrapSenseLVL/3)` | `(level/3).max(1)` | ✓ |
| | Trapfinding | `SlayerTrapfindingLVL/2` | `level/2` | ✓ |
| | Track | `max(SlayerTrackLVL/2,1)` | `(level/2).max(1)` | ✓ |
| **Swashbuckler** | Charmed Life uses/day | `((SwashbucklerLVL-2)/4)+3` (≥2nd lvl) | `((level-2)/4)+3`, `None` below 2 | ✓ |
| | Charmed Life bonus | `SwashbucklerCharmedLifeBonus\|CHA` | CHA | ✓ |
| | Nimble dodge | `(SwashbucklerLVL+1)/4` | `(level+1)/4` | ✓ |
| **Warpriest** | Blessing uses/day | `(WarpriestBlessingLVL/2)+3` | `level/2+3` | ✓ |
| | Blessing save DC | `(WarpriestBlessingLVL/2)+10+WIS` | `level/2+10+WIS` | ✓ |
| | Sacred Weapon dice **size** | EQ:M `if(<5,6,if(<10,8,if(<15,10,if(<20,6,8))))` | identical | ✓ |
| | Destructive Attacks | (Cleric Touch-of-Good shape) | `(level/2).max(1)` | ✓ |
| **Arcanist** | Arcane Reservoir max | `MaxArcanistReservoirSize\|3+ArcanistLVL` | `3+level` | ✓ |
| | Reservoir daily fill | `ArcanistReservoirSize\|3+ArcanistLVL/2` | `3+level/2` | ✓ |
| **Brawler** | AC bonus | `(LVL>3)+(LVL>8)+(LVL>12)+(LVL>17)` | identical | ✓ |
| **Inquisitor** | Justice judgment attack | `1+InqJudgeJusticeLVL/5` | `1+level/5` | ✓ |
| **Alchemist** | Mutagen duration | `AlchemistMutagenLVL*10` | `level*10` | ✓ |
| | Mutagen stat bonus / penalty / nat. armor | `4` / `-2` / `2` | `4` / `-2` / `2` | ✓ |
| **Bloodrager** | Bloodrage Str/Con, save, AC | base `4` / `2` / `-2`; +2/+1 per Greater(11)/Mighty(20) stage | `(4,2)`,`(6,3)`,`(8,4)`; AC `-2` | ✓ |
| | Bloodrage rounds/day | `2+ConMod+(2*BloodrageLVL)` | `2+Con+2*level` | ✓ |
| **Skald** | Inspired Rage Str/Con | `2+floor(SkaldLVL/8)*2` | `2+(level/8)*2` | ✓ |
| | Inspired Rage Will | `1+(SkaldLVL/4)*1` | `1+level/4` | ✓ |
| | Spells per day | (Bard table) | byte-identical to `bard_base_spells_per_day` L1–10 | ✓ |
| | Spells known | (Bard table) | reuses `bard_spells_known_table` directly | ✓ |
| **Hunter** | Animal Companion | Wolf, effective level = hunter level | `ground_wolf_companion_stat_block(…, level)` (1:1) | ✓ |
| **Cavalier** | Mount | Horse, AC 10 + nat. armor 4 = 14; Str 16 / Con 15 | `HORSE_COMPANION_NATURAL_ARMOR=4` (AC 14), Str 16 / Con 15 | ✓ |

Notes on the matches:
- **Slayer** is fully clean, including the internal floor asymmetry the
  Investigator scoping flagged: Trapfinding `level/2` (no floor) vs Track
  `max(level/2,1)` (floor) vs Trap Sense `max(1,level/3)` (floor) — all
  three transcribed correctly per their own records.
- **Bloodrager** tiers correctly compose the base record (`4`/`2`) with the
  separate Greater/Mighty Bloodrage stage records (each `+2`/`+1`), giving
  `4/2 → 6/3 → 8/4` at levels 1/11/20 — not a flat value miscopied.
- **Hunter/Cavalier** companion stat blocks reuse the already-corpus-
  verified Wolf/Horse math from their Druid/Cavalier closures; both are
  bounded to companion level 1 (advancement deferred), matching the
  reports.

## Chassis tables (BAB / saves / HD) — re-derived from the corpus, all 11 clean

A second pass re-derived each landed class's chassis directly from its own
`CLASS:<Name>` record's `BONUS:COMBAT|BASEAB` / `BONUS:SAVE|BASE.*` / `HD:`
tokens and compared to its `rules_tables::{acg,apg}::class_<name>` table
(BAB formula, `HIT_DIE`, and the good/poor assignment of each of the three
saves). This surface is also hand-transcribed, and the Sacred Weapon bug
shows transcription errors reach production — so it was worth actually
re-deriving rather than trusting the chassis tests. **All 11 match the
corpus exactly:**

| Class | HD | BAB | Fort | Ref | Will | Match |
|---|---|---|---|---|---|---|
| Skald | 8 | 3/4 | good | poor | good | ✓ |
| Bloodrager | 10 | full | good | poor | poor | ✓ |
| Brawler | 10 | full | good | good | poor | ✓ |
| Hunter | 8 | 3/4 | good | good | poor | ✓ |
| Arcanist | 6 | 1/2 | poor | poor | good | ✓ |
| Warpriest | 8 | 3/4 | good | poor | good | ✓ |
| Slayer | 10 | full | good | good | poor | ✓ |
| Swashbuckler | 10 | full | poor | good | poor | ✓ |
| Alchemist | 8 | 3/4 | good | good | poor | ✓ |
| Cavalier | 10 | full | good | poor | poor | ✓ |
| Inquisitor | 8 | 3/4 | good | poor | good | ✓ |

(BAB formulas: full = `level`, 3/4 = `level*3/4`, 1/2 = `level/2`; good
save = `level/2+2`, poor = `level/3` — every code table uses these exactly,
with the good/poor save assignment matching each corpus record's
`BONUS:SAVE|BASE.<Save>` grouping.)

## Scope of this pass

Covered every numeric formula AND the BAB/save/HD chassis tables across the
11 landed ACG/APG class-specific closures (Skald, Bloodrager, Brawler,
Hunter, Cavalier, Alchemist, Inquisitor, Arcanist, Warpriest, Slayer,
Swashbuckler). Not covered (out of "numeric formula" scope): diagnostic
wording and the choice-recognition/activation-gating control flow (verified
by each closure's own tests). The one bug and two notes above are the
complete findings across both the feature formulas and the chassis tables.

## For the lead

1. **Warpriest Sacred Weapon dice count (`/20` → `/15`)** — confirmed real
   shipped bug, active at levels 15–19, already flagged to you separately.
   Fix at `pilot_compute.rs:9886` and its doc comment (9866).
2. **Arcanist `4/2` spell-slot table** — not corpus-derivable; worth a
   source double-check given it exceeds the Wizard table it shares a list
   with. Not asserted wrong.
3. Everything else re-derived matches the corpus exactly.
