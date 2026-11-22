# Stage F1c — render receipt (merge-readiness blockers 1 and 2)

Spec: `docs/release/SD-36-consolidation/epic-f-class-completion.md` §3 (F1). This receipt closes the
two blockers of the F1c merge-readiness check: (1) D2 split lines that stopped printing, and
(2) rendered value changes vs tranche/16 with no cited receipt. It supersedes, for rendered
`LINE|` values, the "Changed sheet values without a PF1 citation: none" statements in
`fixture-receipts.md` — those were about test pass/fail; no render comparison had been run.

## 0. Denominator and method

- **Population: 249 builds.** Every non-prestige class in `census-f1c.json` (61) at levels 1, 5,
  10 and 20, each capped at the class's own `max_level` and de-duplicated (244), plus the 5 builds
  of the merge-readiness check's 15 that are not already in that list (`psion:3`,
  `fighter:6+duelist:1`, `fighter:6+golden_legionnaire:1`, `fighter:6+wizard:4`,
  `cleric:5+rogue:3`). List:
  `python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/f1c_render_population.py builds`.
- **Before:** tranche/16 code + tranche/16 data — a detached worktree at `8057263014`, its own
  `class_census` release build (the baked data path was checked with `strings`: it points at that
  worktree). **After:** this branch's code + data, its own release build.
- **Render:** `f1c_render_population.py render <class_census binary> <outdir>` runs
  `class_census --sheet-dump <build> --with-sheet-rules` per build. All 249 dumps exit 0 on both
  trees.
- **Classify:** `f1c_render_classify.py <before> <after> <out>` (every `LINE|` row by rule id:
  added, removed, changed `printed`, changed condition; `EXPL|` value deltas; same-build duplicate
  scan) and `f1c_render_pairs.py` (pairs each D2 relocation and buckets every delta by
  mechanism). Outputs: `render/render-classify.json`, `render/render-pairs.json`. Added-line
  attribution: `f1c_render_attribute.py` → `render/added-attribution.txt`. D2 static scan:
  `f1c_d2_line_gate_scan.py` → `render/d2-line-gate-scan.json`.

`LINE|` rows: 7,032 on tranche/16, 9,088 on F1c. Same-build duplicate copies: 16 before, 16
after (no new duplicate). Build dumps that failed: 0 of 249 on each tree.

## 1. Blocker 1 — root fix (render_sheet, `src/rules_core/sheet_rule.rs`)

Two rules, neither names a class or a record:

1. **A sibling's line gate is its own terms, not its principal's holding condition**
   (`line_gate`). A principal's `applies` (a class's entry requirements, a feat's prerequisites)
   is settled by the principal being held; render never re-checked it on the principal line, on
   tranche/16 or now. D2 copies it onto every sibling it splits off, so render now drops that copy
   (recognised structurally: every decided term of the principal's gate appears among the
   sibling's `All` terms) and decides the line by what remains. A `Situational` term of the copy
   is kept ("when active" stays on every rage line).
2. **An undecided fact prints as a condition, never drops** (`Evaluator::undecided_leaf`,
   render only). A `Holds` leaf over a fact the character record does not carry — alignment,
   deity, gender, age category, languages, class skills (the list in
   `CharacterFacts::from_character`'s own doc) — reads `Situational(<the leaf in words>)` on a
   sibling's line gate instead of `Exclude`; its negation is undecided too. The held-set
   fixpoint, the variable fold, the proficiency reader and every other gate consumer keep the
   two-valued reading, so no total moves. Proficiency is not in the list: the sheet answers it
   through its own proficiency reader.

RED first: `a_line_gated_on_an_uncarried_fact_prints_with_its_condition_never_drops` and
`a_sibling_line_is_gated_by_its_own_terms_not_its_principals_holding_condition` (both failed on
the branch head, pass now). `a_sibling_line_prints_only_when_its_own_gate_includes` was updated:
it pinned "Climb has one line" from before D2 moved the class-skill +3 into `#bonus0`, which is
the exact loss this blocker names; it now requires the `+3` line with its class-skill condition.

**Static scan of all 1,108 D2 siblings** (`render/d2-line-gate-scan.json`): after rule 1, 262
line gates are unconditional (print whenever held, as tranche/16 printed the principal line), 118
still read an undecided fact (print with that condition), and 728 are decided by facts the
record carries (the line's own condition). Undecided leaves in the ORIGINAL sibling gates:
ClassSkill 102, Alignment 25, AlignmentMatchesDeity 2, Deity 10, DeityInPantheon 1, Language 5,
AgeCategory 4, Gender 1 (per sibling, per leaf kind).

**The cited cases, rendered:** `fighter:1` prints `Climb +3`, `Intimidate +3`, `Swim +3`, each
with "requires <skill> as a class skill" (CRB Skills: +3 on a class skill with at least 1 rank).
`paladin:1` prints `Paladin (base attack) +1`, `antipaladin:1` `Antipaladin (base attack) +1`,
`druid:1` `+0`, `hunter:1` `+0`, `shifter:1` `+1` — each unconditional (the alignment requirement
is the class's entry requirement, settled by the class being held).

### 1b. Found by the wide render and fixed at the root: counted-as rules folded their bonuses

`HeldSet::holds` includes counted-as rules (the source's SERVESAS); the variable fold used it, so
a rule the character only counts as holding contributed its own bonuses. Unchained Rogue ~
Trapfinding serves as the CRB Rogue ~ Trapfinding and both fed "Trapfinding Bonus": `+10` at
unchained rogue 5 (PF1 and the chassis `EXPL`: `+2`). Fix: `HeldSet::holds_itself` (held, never
merely counted-as) decides whose declarations and contributions fold; gates still read `holds`.
RED first: `a_counted_as_rule_satisfies_a_gate_but_contributes_nothing_to_a_variable`.

## 2. Every LINE delta vs tranche/16, by bucket (249 builds)

| Bucket | Rows | Meaning |
|---|---:|---|
| D2 relocated | 1,276 pairs | principal printed X, now prints its text; a new sibling prints the same X under the same label (+ target suffix). Condition unchanged in 529; class-skill condition added in 747 (3 skills x 249 builds, rule 2) |
| D2 folded | 8 | `rage` "Rage (Will save) +1 when active" now appears once, printed by `barbarian_rage#bonus0` (the stage-5 duplicate fold); barbarian and ex-barbarian x4 levels |
| D2 line gate decides false | 21 | §4 |
| Value changed, same id | 298 | §3 |
| Added, not a relocation | 789 | §5 |
| Removed id | 9 | §6 |
| EXPL value changed | 8 | §7 |

## 3. Changed printed values (298 rows, 44 distinct (id, before, after) groups), each with mechanism and PF1 citation

Mechanisms: **D7** always-held global abilities seed every walk (F1c-4, `ea4d64eca8`); **D2** line
split (the condition that belongs to one line no longer gates the record); **D3** Unchained class
records + TakenOnClass (the Unchained class holds its base class line, so the base class's level
variables fold); **CA** the counted-as fix above.

| Line | tranche/16 -> F1c | Builds | Mechanism | PF1 |
|---|---|---|---|---|
| Weapon Focus | +0 -> +1 | 249 of 249 | D7 (the CRB `Default` global declares and contributes "Weapon Focus To Hit") | CRB Feats, Weapon Focus: +1 on attack rolls with the selected weapon |
| Adept, Bard, Cleric, Sorcerer, Wizard (caster level) | +0 -> class level (+1/+5/+10/+20; wizard +4 at fighter 6/wizard 4) | 22 | D7 (`Default` declares the Caster Level variables) | CRB class spell sections: caster level = class level |
| Ranger (caster level) | +0 -> +2/+7/+17 at 5/10/20 | 3 | D7 | CRB Ranger, Spells: caster level = ranger level - 3 |
| Armor Training (accheck) | +0 -> +1 (fighter 5; fighter 6/wizard 4), +2 (fighter 10) | 3 | D2 (`fighter_class`, the CRB "Fighter" record, is held now that the level-20 Weapon Mastery condition gates only its own line; it feeds "Armor Training LVL") | CRB Fighter, Armor Training: armor check penalty reduced by 1 at 3rd level and every four levels thereafter (7th, 11th, 15th) |
| Danger Sense (unchained barbarian, unchained rogue) | +0 -> +1/+3/+6 at 5/10/20 | 6 | D3 + CA | Pathfinder Unchained, Barbarian and Rogue, Danger Sense: +1 at 3rd level, +1 every 3 levels thereafter |
| Unchained Monk Bonus Feat | +1 -> +2/+4/+6 at 5/10/20 | 3 | D3 ("Monk Bonus Feat LVL" folds the class line) | Pathfinder Unchained, Monk, Bonus Feats: 1st, 2nd, and every 4 levels thereafter |
| Finesse Training (unchained rogue finesse damage choice picks) | +0 -> +1 (5, 10), +3 (20) | 3 | D3 | Pathfinder Unchained, Rogue, Finesse Training: a weapon at 3rd level, again at 11th and 19th |
| Rogue's Edge | +0 -> +1/+2/+4 at 5/10/20 | 3 | D3 | Pathfinder Unchained, Rogue, Rogue's Edge: 5th level and every 5 levels thereafter |
| Trapfinding (Perception; Disable Device) | +2 -> +1 (1), +5 (10), +10 (20); unchanged +2 at 5 | 6 | D3 + CA (before CA it read +10 at level 5) | Pathfinder Unchained, Rogue, Trapfinding: 1/2 rogue level (minimum 1); chassis `EXPL` agrees |

## 4. Lines whose own gate now decides false (21 rows, 6 ids)

tranche/16 printed these principals WITHOUT checking their gate; D2 moved the value and its gate
onto one sibling, and the gate is decided by facts the record carries:

| Line (tranche/16 print) | Builds | Gate now read | Verdict |
|---|---|---|---|
| Shifter Claws `+1` | shifter 1/5/10/20 | `Shifter LVL >= 17` on `#weapon0` "Shifter Claw critmultadd"; prints `+1` at shifter 20 | Correct: Ultimate Wilderness, Shifter Claws: critical multiplier x3 at 17th level. tranche/16 printed it at every level |
| Swashbuckler Finesse (attack) `+0` | swashbuckler 1/5/10/20 | `not Holds Weapon Finesse`; the record counts as Weapon Finesse | Correct: ACG, Swashbuckler Finesse grants the benefits of Weapon Finesse, so the Str-to-Dex fallback line does not apply |
| Infusion (kineticist infusion or wild talent aether picks) `+1` | kineticist 1/5/10/20 | `Element Tracker Aether >= 2`; the sweep kineticist has chosen no element | Correct: the extra pick needs the same element chosen twice (Occult Adventures, Expanded Element). tranche/16 printed it for every kineticist |
| Armor Training (armor training choice picks) `+1` | fighter 5; fighter 6/wizard 4 | `Armor Training LVL >= 7` on `#bonus0`; prints at fighter 10 | Correct: the choice pick starts at 7th level |
| Summoner Summon Monster I `0` (APG summoner) | summoner 1/5/10/20 | `#spell0` gated on "Summoner Summon Monster LVL" = 1 | tranche/16 printed a wrong `0`; PF1 (APG Summoner, Summon Monster I: 3 + Cha modifier per day) is not supported by the data, so no number prints now. Remainder R2 |
| Summoner Summon Monster I `3` (Unchained summoner) | unchained summoner 5/10/20 | tier gate `min((L+1)/2, 9) = N` | Level 5 prints Summon Monster III `3` and level 20 Summon Monster IX `3` (Pathfinder Unchained, Summoner, Summon Monster: tier (level+1)/2, uses 3 + Cha); tranche/16 printed tier I at every level. Level 10 prints no tier: remainder R1 |

## 5. Added lines that are not relocations (789 rows, 104 ids)

187 carry a value, 602 print rule text only. By cause (`render/added-attribution.txt`; hold state
read from the tranche/16 `HELD|` rows of the same build):

- **Held on tranche/16 but gated off (103 rows).** By rule 1 (the class's entry requirements
  were copied onto the line): base saves and caster level of paladin, antipaladin, druid, hunter,
  shifter, summoner (APG) and duelist (in fighter 6 / duelist 1). By D2 (the line's own gate reads
  "Armor Training LVL" / "Fighter CFP Level", which fold now that `fighter_class` is held): Armor
  Training maxdex tier 1 (fighter 10) and Weapon Training II picks (fighter 10). By D3 (the tier
  variable folds for the Unchained summoner): Summon Monster III (5) and IX (20). Checked against the class
  tables: paladin and antipaladin Fort/Will good, Ref poor, caster level = level - 3 (CRB Paladin;
  APG Antipaladin); druid Fort/Will good, Ref poor, caster level = level (CRB Druid); hunter
  Fort/Ref good, Will poor (ACG Hunter); shifter Fort/Ref good, Will poor (Ultimate Wilderness
  Shifter); summoner Will good, Fort/Ref poor (APG Summoner); duelist Ref +1, Fort/Will +0 at 1st
  (CRB Duelist); fighter armor training maxdex +1 from 3rd, weapon training II at 9th (CRB
  Fighter). `Paladin (domains) +0` also prints (the record's domain count is 0).
- **Folded as a duplicate on tranche/16 (3 rows).** `Climb +3` from Skill Focus
  (`skill:climb#bonus2`) for cavalier 5/10/20, which hold Skill Focus: it printed the same text as
  the old principal line and was dropped by the duplicate fold (CRB Skill Focus: +3).
- **Not held on tranche/16 (683 rows), held through F1c's converter and engine changes:**
  - proficiency records reached by D1 type grants — `weapon_prof_simple/martial/auto`,
    `armor_prof_light/medium/heavy`, `shield_prof`, `shield_prof_tower`, `shield_prof_buckler`
    (423 rows, all text);
  - D3 / TakenOnClass — the four `pathfinder_unchained:class:*` records and their class features,
    the base class lines they are taken on (`core_rulebook:class:rogue`, `barbarian`, `monk`,
    `advanced_players_guide:class:summoner`, with base attack and saves: rogue 3/4 BAB with good
    Reflex, barbarian full BAB with good Fort, summoner 3/4 BAB with good Will — the Pathfinder
    Unchained Rogue, Barbarian and Summoner tables), Unchained Monk base attack full and Fort/Ref
    good, Will poor (Pathfinder Unchained, Monk table), `monk_unarmed_damage_lvl_20_medium` `2d10`
    at unchained monk 20 (Pathfinder Unchained, Monk, Unarmed Strike: 2d10 at 20th for a Medium
    monk);
  - D2 — `fighter_class`, `medium_class`, `mesmerist_class`, `psychic_class`, `spiritualist_class`
    (the one-line condition no longer gates the record); in the fighter 6 mixes, fighter
    bonus-feat picks `+4`, weapon training I picks `+1`, armor training maxdex/accheck `+1` and
    bravery, whose grant conditions read "Fighter CFP Level", declared by `fighter_class` (CRB
    Fighter: bonus feats at 1st, 2nd and every even level; weapon training at 5th; armor training
    and bravery at 2nd-3rd);
  - D8 — `summoner_standard_class` (the Summoner Class Selection pick, `641691e283`);
  - other F1c grant-edge or gate changes, text only (`added-attribution.txt` names the bucket of
    each): `ki_pool_tracker`, `ki_stat_choice_*`, samurai `determined`/`resolute`/`unstoppable`,
    Golden Legionnaire Defy Danger;
  - Golden Legionnaire (base attack) `+1` at 1st (Adventurer's Guide, Golden Legionnaire: full
    base attack), `shifter_claw#weapon1` "wieldcategory" `+0`.

## 6. Removed ids (9 rows)

- `monk_unarmed_damage_lvl_1` and `..._lvl_1_medium` `1d6` at unchained monk 1/5/10/20 (8 rows):
  tranche/16 read "Monk LVL" as 0 for the Unchained monk and printed the level-1 die at every
  level (wrong at 5, 10 and 20: 1d8, 1d10, 2d10). With D3 the Unchained monk reads its level like
  the CRB monk; level 20 prints `2d10`, levels 1, 5 and 10 print no tier line — exactly as the CRB
  monk already does on tranche/16 (remainder R1). The chassis `EXPL` grounds the die at every
  level (`unarmed_strike_damage_die`).
- `unchained_monk_timeless_body` at unchained monk 20 (text): still held; the CRB `timeless_body`
  is now held at the same build (a new grant edge) and prints the identical "Timeless Body" text
  line, so the duplicate fold keeps one (the CRB id sorts first).

## 7. EXPL value changes (8 rows)

`combat.baseline_melee_attack_bonus` and `combat.weapon_attack_bonus.longsword` for commoner
1/5/10/20: 5 -> 1, 7 -> 3, 10 -> 6, 15 -> 11 (mechanism D6: the commoner's one-simple-weapon pick
is converted, so the Longsword reads nonproficient). CRB NPC Classes, Commoner: proficient with
one simple weapon, no other weapons; CRB Combat: -4 on attack rolls with a weapon you are not
proficient with. No other `EXPL` value changed in the 249 builds, so no computed total moved
because of rules 1, 2 or the counted-as fix. Census (`class_census --json`): identical to
`census-f1c.json` apart from `generated_at` — 61 of 61 non-prestige Computed, 185 of 185 mix-panel
builds Computed, 11 prestige mixes Unknown. Proficiency reader: 135 classes, 42 static, 93 walked,
76 Known, 17 Unknown (unchanged).

## 8. Named remainders (by mechanism)

- **R1 — exact division inside an equality tier gate.** `min(5, MonkUnarmedDamageLVL/4) = N` and
  `min((Summoner_CFP_Level+1)/2, 9) = N` are exact rationals (SD-35 ruling A3: PCGen carries
  doubles; its `PREVAREQ` compares floats), so a level that is not a multiple of 4 (monk) or an
  even summoner level matches no tier. Rendered: CRB monk 1/5/10 and Unchained monk 1/5/10 print
  no unarmed-damage tier line (the CRB monk rows are unchanged from tranche/16), Unchained summoner
  10 prints no Summon Monster tier. The chassis `EXPL` grounds the die and tier.
- **R2 — the APG Summoner's Summon Monster rows read variables only Pathfinder Unchained declares**
  (`apg_abilities_class.lst:818`-: `SummonerSummonMonsterLVL`/`Times`, while the APG record
  DEFINEs `SummonMonsterLVL`/`Times`). Source-data defect; the line prints its name without a
  number (Unknown), never tranche/16's wrong `0`.
- **R3 — the duplicate fold hides stacking tier lines.** Armor Training maxdex tiers are four
  identical "+1 while armor is equipped" siblings; the stage-5 fold keeps one, so fighter 10 and
  fighter 20 show one `+1` line where PF1 gives +2 and +4 (the chassis `EXPL` grounds the tier).
  Summing identical sibling lines was tried and rejected: across the 249 builds it doubled lines
  that repeat the SAME bonus (warpriest caster level, swashbuckler weapon training, witch hex
  count), 30 wrong values against 2 corrected.

## 9. Gates

| Gate | Command | Result |
|---|---|---|
| Root tests | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` | EXIT=0; 287 test binaries; 6,311 passed / 0 failed / 28 ignored (6,308 + 3 new) |
| Root clippy | `cargo clippy --locked -j 8 --all-targets -- -D warnings` | EXIT=0 |
| Desktop tests | `cargo test --locked -j 8 --no-fail-fast --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8` | EXIT=0; 613 passed / 0 failed / 0 ignored of 613 |
| Ingest tests | `cargo test --locked -j 8 -p codex-ingest --no-fail-fast -- --test-threads=8` | EXIT=0; 167 test-result lines; 1,748 passed / 0 failed / 43 ignored |
| Residue gate | `python3 scripts/pcgen_residue_gate.py --check --closure` | PASS |

No converter code changed in this step, so the package was not regenerated:
`data/sheet_rules/_report.json` stays `records=49450 converted=49450`, `data/corpus/**` and
`site/**` are untouched.
