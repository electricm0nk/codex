---
title: v0.8 — Scout gap audit — character creation & level-up player journey
status: draft
author: scout (Fable 5.1 teammate)
date: 2026-09-01
branch: tranche/14-ui
scope_read: apps/desktop/src/characterHub/**, apps/desktop/src/boundary/**, apps/desktop/src-tauri/src/{main.rs,character_hub.rs} (read-only), repo-root src/rules_core (grep-only, for tagging)
---

# Scout gap audit — the player's journey through `characterHub/`

This is a **player-journey walkthrough**, not a code-coverage diff (brief §3.1). I sat in the
seat of someone rolling a new PF1e character and walked
race → class → ability scores → skills → feats → traits → languages → equipment → spells → review,
then levelled the character up, asking at each step: (1) does the UI offer every selection a
PF1e sheet needs here, and (2) once a choice is made, does the sheet show what the character
now qualifies for?

Tags: `[frontend]` = `apps/desktop/src/**`; `[backend]` = `apps/desktop/src-tauri/src/**`,
and **only ever** "wire an existing engine capability through Tauri" (brief §4.2). Anything
that would need repo-root `src/` edited is in `## Open blockers`, not the punch list.
Where a gap's shape is genuinely undecided it is in `## Open questions` rather than designed here.

Not treated as pre-approved scope (brief §3.1) — the orchestrator triages.

## How to read the numbers in this file

Per AGENTS.md rule 9, every count below carries the command that produced it:

| Figure | Command |
|---|---|
| 31 classes in the creation picker | `grep -c "^  { id: 'class:" apps/desktop/src/characterHub/characterHubModel.ts` |
| 36 skill rows on the sheet | `awk '/^export const SKILLS/,/^\];/' apps/desktop/src/characterHub/skillsModel.ts \| grep -c "name:"` |
| 12 classes with a hand-authored class-skill list | `awk '/^const CLASS_SKILLS/,/^};/' apps/desktop/src/characterHub/skillsModel.ts \| grep -c "'class:"` |
| 3 skills with a real engine modifier | `grep -n "SelectedSkillModifiersDto" -A4 apps/desktop/src/boundary/loadCreateCharacter.ts` (climb/intimidate/swim) |
| 5 skill ids the engine allocator recognises | `sed -n 265,272p src/rules_core/skill_allocation.rs` |
| 0 `@media print` rules | `grep -c "@media print" apps/desktop/src/theme.css apps/desktop/index.html` |
| 45+ `choice:*` slots the engine reads | `grep -rhoE 'choice:[a-z_0-9]+' src/ --include=*.rs \| sort \| uniq -c \| sort -rn` |
| 0 `favored_class` / `ability_score_increase` hits in the engine | `grep -rlE "favored_class\|ability_score_increase" src/rules_core/*.rs` |

## What is already wired (so nobody re-audits it)

Verified by reading the call path from affordance → boundary wrapper → registered Tauri command
(`apps/desktop/src-tauri/src/main.rs:162-260`): corpus-derived race roster with fixed and
floating ability adjustments; manual / point-buy / pool / straight ability generation; alternate
racial traits resolved live by the engine with mutual lock-outs; 53 computing character traits with
per-trait skill choice; 31 classes at levels 1-20; HP preview; create → engine `Computed`/`Blocked`
outcome with real diagnostics; level-up dialog previewing engine grants; odd-level feat pick on
level-up through the real feat picker; feat picker greyed-with-reason by engine eligibility, with
the two-step chooser-target flow (weapon / skill / school); feat removal with dependency refusal;
skill-allocation dialog persisting via `set_skill_allocations`; spell picker narrowed to the routed
class's own list with per-class levels, save DC and spells-per-day from engine records; equipment
purchase charging real money, equipmod attachment, drop, encumbrance, AC-by-source, per-weapon
damage facets; pets stat block; class features with corpus prose; racial traits with engine prose;
bio / money / HP-durability / portrait sidecars; clone, recompute, export (Load screen), import.

---

## Punch list (ordered by where in the journey the player hits it)

### A. Identity and setup (`CreateCharacterForm.tsx` top of form)

1. `[frontend]` **Player name is collected and thrown away.** `playerName` is component state only
   (`CreateCharacterForm.tsx:264`); it is never sent to `composeCreateCharacterRequest` nor to
   `update_character_bio`, and `BioFields`/`CharacterBioDto` have no slot for it. A player types
   their name and it appears nowhere on the sheet. (Needs a `[backend]` sibling: add `playerName`
   to the bio sidecar DTO in `character_hub.rs` `UpdateCharacterBioRequest`/`load_character_bio`.)
2. `[frontend]` **Alignment, Deity, Sex, Age, Eyes, Hair, Height, Weight entered at creation are
   never persisted.** All eight are form state (`CreateCharacterForm.tsx:274-281`) but
   `composeCreateCharacterRequest` carries none of them and the form never calls
   `updateCharacterBio` after a `saved` outcome. The sheet's Details panel opens blank and the
   player re-types everything. The `update_character_bio` command already accepts exactly these
   eight fields.
3. `[frontend]` **Deity is a free-text box.** The corpus carries deity records
   (`data/corpus/core_rulebook/deity/`) and `list_reference_library_catalog` already serves the
   `deity` kind (`reference_library_catalog.rs:93`). A player has no list to pick from and no
   alignment/domain hint. Wire a picker off the existing catalog command.
4. `[frontend]` **Class support text is stale.** Every one of the 31 classes is `full`, so the
   "(blocked today — needs a choice picker not yet built)" `headless-only` suffix never renders,
   while the *actual* headless behaviour (item 12 — canonical bloodline/domain/school silently
   seeded) is not told to the player at all. The description line under the Class select should
   say what is auto-chosen for them.

### B. Race

5. `[frontend]` **Picking a race shows no racial-trait list, only its alternates.** The
   `resolve_race_alternate_selection` response the form already fetches on every race change
   (`CreateCharacterForm.tsx:427-444`) carries `appliedTraits` including the standard/default
   rows, but only the alternate rows are rendered. A player choosing Dwarf vs. Gnome cannot see
   Hardy / Stonecunning / Weapon Familiarity before committing. (Question 1 of the brief: "does
   picking a race immediately show what it grants?" — no.)
6. `[frontend]` **Human bonus feat / Monk bonus feat / Fighter level-1 bonus feat are never
   offered to the player.** The engine reads `choice:human_bonus_feat`, `choice:monk_bonus_feat`,
   `choice:fighter_bonus_feat` (choice-slot grep, table above) and `compose_character_input`
   seeds Dodge into the human slot (`featsTabModel.ts:20-27` traces this). The creation form has
   no feat step at all. After creation the Feats tab can add feats, but nothing tells the player
   "you have a racial bonus feat to spend". Needs `[backend]` sibling: a create-time way to pass
   `additional_choices` (the Rust `CreateCharacterRequest` at `character_hub.rs:400` has no such
   field today; `LevelUpCharacterRequest` does).
7. `[frontend]` **Age category effects are previewed but not saved.** `calculatedScore` adds
   `ageEffectForAbility` (`CreateCharacterForm.tsx:332`) so a Venerable character sees STR −6 in
   the Calculated column, but `handleSubmit` submits `raw + racial + floating` only
   (`:579-584`). The saved character's scores disagree with what the form showed. Either stop
   previewing it or persist it — see Open question Q3 (the engine has no aging model; TS is
   currently the only place this rule lives, which §3.3 forbids).
8. `[frontend]` **Half-Elf / Half-Orc floating +2 is baked client-side and the
   `abilityBonusTarget` is inferred by "whichever ability got the most points"**
   (`deriveAbilityBonusTarget`, `:545-555`). Works for the +2 case but silently misreports if a
   race ever has 2+ floating points split across abilities. Low priority; note only.

### C. Class

9. `[frontend]` **Choosing a class previews nothing but HP.** BAB, base saves, skill points per
   level, class skills and weapon/armor proficiencies are not shown before the pick.
   `list_class_catalog` (`loadClassCatalog.ts`) already returns BAB/Fort/Ref/Will per class per
   level; class skill points live in `characterProgression.ts`. Show them next to the select.
10. `[frontend]` **Only one class at creation.** `CreateCharacterRequest.classId` is a single
    string; a player rolling a Fighter 3 / Rogue 2 at level 5 cannot. Multiclass is reachable
    only by creating then levelling up one level at a time. See Open question Q1.
11. `[frontend]` **Creating at level N > 1 skips every per-level choice.** A Level 10 Fighter
    created directly gets the engine-seeded Power Attack + Weapon Focus and Climb/Intimidate/Swim
    at 1 rank each and nothing else — no feats for levels 3/5/7/9, no bonus combat feats, no
    skill ranks, no spells, no rogue talents. The level picker (`:695-708`) just sets the number.
    The level-up path (`LevelUpDialog` → `handleLevelUpAccept`) already routes feat picks; the
    creation path does not. See Open question Q2 for shape.
12. `[backend]` **No create-time channel for class choices the engine reads.** The engine reads
    `choice:cleric_domain`, `choice:sorcerer_bloodline`, `choice:wizard_school_specialization`,
    `choice:wizard_opposed_schools`, `choice:druid_nature_bond`, `choice:oracle_mystery`,
    `choice:oracle_curse`, `choice:ranger_combat_style`, `choice:ranger_favored_enemy`,
    `choice:cavalier_order`, `choice:witch_hex`, `choice:shaman_spirit`,
    `choice:arcanist_metamagic_knowledge`, `choice:bloodrager_bloodline`,
    `choice:inquisitor_domain`, `choice:inquisitor_judgment`, `choice:warpriest_blessing`,
    `choice:alchemist_discovery`, `choice:summoner_eidolon_evolution`,
    `choice:bard_versatile_performance`, `choice:psychic_discipline` (choice-slot grep above),
    and `compose_character_input` silently seeds a canonical default (Arcane bloodline, Good
    domain, wolf companion — `characterHubModel.ts:273-282`). The Rust `CreateCharacterRequest`
    has no `additional_choices` field. Add one (same `SelectedChoiceDto` shape
    `LevelUpCharacterRequest` already uses) plus, where no list command exists yet, a
    `list_*_options` command for each pool (`list_class_feature_pool_options` covers some pools
    today — see `ClassFeaturePoolReferenceSection`, which renders them read-only).
13. `[frontend]` **Class-choice pickers (domain, bloodline, school, nature bond, mystery,
    curse, combat style, favored enemy, order, patron, spirit, exploit, blessing, discipline).**
    Depends on 12 landing first. A Cleric today cannot pick her domains; a Sorcerer is always
    Arcane. This is the single biggest "what does picking this class let me choose" gap.
14. `[frontend]` **Companion species picker is missing although the wire already supports it.**
    Rust `CreateCharacterRequest.companion_species: Option<String>` exists
    (`character_hub.rs:444`) and `list_companion_catalog` is registered, but the TS request
    (`loadCreateCharacter.ts`) never sends it and no picker exists. A Druid always gets a Wolf.
15. `[frontend]` **Alignment is a free pick with no class hint.** Nothing tells a player that
    Paladin needs LG, Monk lawful, Barbarian non-lawful. Engine has no alignment rule (grep
    `alignment` in `src/rules_core` hits only `equipment_effects.rs`/`support_state_matrix.rs`),
    so surfacing this as a *rule* is Open blocker B7; surfacing the corpus `PREALIGN` text as
    informational prose beside the picker would be `[backend]` (expose the class record's prereq
    text) then `[frontend]`.

### D. Ability scores

16. `[backend]` **The sheet reconstructs ability scores from modifiers.** `PilotSnapshotDto`
    carries only `abilityModifiers`; `AbilitiesPanel` prints `scoreFromModifier` = `10 + 2·mod`
    (`CharacterSheet.tsx:418-420`), so every odd score displays one lower (17 → 16). Expose the
    real submitted/computed scores on `load_saved_character` (they are in the persisted
    `authoritative_character_input`). Then `[frontend]` render them.
17. `[frontend]` **No ripple from ability scores at creation beyond HP.** Changing CON shows HP
    move; changing DEX/INT/WIS shows nothing (AC, saves, skill points, bonus languages, spell
    DCs). Question 2 of the brief for this step is answered "no". See Open question Q4 for whether
    a pre-save preview compute is wanted.
18. `[frontend]` **Ability score increase at character levels 4/8/12/16/20 is a label only.**
    `generalBenefits` emits the string "Ability score increase"; no picker, no persistence, and
    the engine has no slot for it (grep table). UI half is a picker + `additional_choices`
    entry; the engine half is Open blocker B2.

### E. Skills

19. `[backend]` **Persisted skill allocations are never read back.** `LoadSavedCharacterResponse`
    has no skill-allocation field; `CharacterSheet` seeds `useState({...DEFAULT_SKILL_ALLOCATION})`
    (Climb/Intimidate/Swim = 1) on every open (`:3363`). A player allocates ranks, closes the
    sheet, reopens, and sees the default again while the engine still holds their real
    allocation. Expose `chosen.skill_allocations` on `load_saved_character`; then `[frontend]`
    seed from it.
20. `[frontend]` **No skill allocation at creation.** The only way to spend level-1 skill
    points is the sheet's dialog after the fact. `set_skill_allocations` exists; the create form
    could call it after a `saved` outcome, or the flow in Q2 could carry it.
21. `[frontend]` **Level-up never prompts to spend the new skill points.** `LevelUpDialog` shows
    "Skill points: N" and `LevelUpCharacterRequest.skill_allocations` exists
    (`levelUpCharacter.ts`) but is deliberately omitted (`CharacterSheet.tsx:3155-3160`). The
    player must remember to open the dialog afterwards; nothing flags unspent points except the
    small "N unallocated" chip.
22. `[backend]` **33 of 36 skill totals are computed in TypeScript.** `SkillsPanel` uses
    `skillModifier(abilityMod, ranks, classSkill)` for everything except climb/intimidate/swim
    (`CharacterSheet.tsx:562-566`), and `isClassSkill` reads a hand-authored 12-class table.
    Armor check penalty, racial skill bonuses, trait bonuses, feat bonuses (Skill Focus) and size
    are therefore all absent from the displayed number. Wiring the 5 skills the engine allocator
    recognises (`skill:climb/swim/intimidate/diplomacy/disable_device`) is `[backend]`; the other
    31 are Open blocker B3.
23. `[backend]` **Class-skill status is a 12-class TS table.** `CLASS_SKILLS` in `skillsModel.ts`
    covers the 11 CRB classes + Arcanist; the other 19 classes (APG/ACG/PU) see *every* skill as
    cross-class — wrong max ranks, wrong cost, wrong +3. The corpus class records carry `CSKILL`.
    Expose class-skill membership per held class through Tauri. (If the engine has no
    class-skill reader beyond the 5 cited skills, this moves to Open blockers — `backend` to
    confirm on first read.)
24. `[frontend]` **Craft / Perform / Profession / Knowledge have no specialty.** `SKILLS` has one
    `Craft`, one `Perform`, one `Profession` row; a Bard with Perform (Oratory) and Perform
    (Sing) cannot record both. Also `skillIdFor` mangles `Knowledge (Arcana)` →
    `skill:knowledge_arcana` by an inferred convention the backend never confirmed
    (`skillsModel.ts:106-116`). See Open question Q5.
25. `[frontend]` **Skill dialog shows ranks/points but never "which skills are class skills for
    the class I'm about to take"** at level-up, so multiclassing into Rogue gives no visible
    "Disable Device is now a class skill" ripple.

### F. Feats

26. `[frontend]` **No feat budget is shown.** The Feats tab has "Add Feat" with no "N of M feat
    slots used" — the player can add 20 feats to a level 1 Fighter and nothing says so. Slot
    count is a rule (odd levels + racial + class bonus), so the count must come from the engine
    (`preview_level_up`'s grants + a creation-time equivalent); see Open question Q6 on whether an
    engine "feat slots" record exists to wire.
27. `[frontend]` **Level-up pick lists are rendered as text, not picked.** `LevelUpDialog`
    prints "Pick 1 rogue talent" from `enginePlan.pickFromLists` but ignores the `candidates`
    array (`previewLevelUp.ts` `LevelUpPickCandidateDto`), and `handleLevelUpAccept` sends only
    the `hp:average` choice. The engine reads `choice:rogue_talent`, `choice:barbarian_rage_power`,
    `choice:witch_hex`, `choice:alchemist_discovery`, `choice:investigator_talent`,
    `choice:oracle_revelation` etc. (grep table), and `LevelUpCharacterRequest.additional_choices`
    is the channel. A Rogue levelling to 2 gets no talent. Note the `local_store.rs` two-colon-
    segment persistence check that bit feat keys (`CharacterSheet.tsx:3183-3196`) — candidate ids
    must satisfy it; `backend` to confirm the id grammar before `frontend` builds the picker.
28. `[frontend]` **Level-1 feats are engine-seeded, not chosen.** Every new character holds
    `feat:power_attack` + `feat:weapon_focus` (`featsTabModel.ts:20-27`) — a Wizard 1 has Power
    Attack. The create flow has no feat step. UI half: a post-create level-1 feat picker (the
    Feats tab's Add/Remove already work) or the Q2 flow. Stopping the seed itself is engine
    (`pf1_adapter.rs` `compose_character_input`) → Open blocker B4.
29. `[frontend]` **After a feat is added, nothing shows what it unlocked.** Adding Power Attack
    doesn't highlight that Cleave is now eligible; the picker re-evaluates on next open only.
    Presentational: re-run `list_feats_for_character` after a successful add and show a
    "now eligible" delta. (Question 2 of the brief for this step.)
30. `[frontend]` **Weapon-target picker offers the whole weapon list, not the proficient ones.**
    `listWeaponTargets` is unfiltered; Weapon Focus in a weapon the character can't use is
    accepted. Whether the engine can filter by proficiency is tied to item 41.

### G. Traits

31. `[frontend]` **No trait cap or category rule is surfaced.** `toggleTrait` has no limit
    (`CreateCharacterForm.tsx:468-492`): 10 traits can be checked. PF1's "two traits, no two from
    the same category, a drawback buys a third" is nowhere — and the engine has no cap either
    (`trait_effects.rs:339` explicitly allows stacking). Trait category is not even shown. The
    rule half is Open blocker B8; showing the corpus category on each row is `[backend]` (expose
    `CATEGORY` on `CharacterTraitOptionDto`) then `[frontend]`.
32. `[frontend]` **Selected traits are not rendered on the sheet.** `selectedTraits` is loaded
    and carried through every refresh (`grep selectedTraits CharacterSheet.tsx` shows only
    carry-forward lines, zero render sites). The player picks Reactionary and never sees it
    again except as a number folded into a skill total they also can't see (item 22).
33. `[backend]` **Traits cannot be added or removed after creation.** No Tauri command exists
    (`main.rs:162-260`); the state is already in `ChosenCharacterState.selected_traits`. Wrap
    add/remove the way `add_feat_selection`/`remove_feat_selection` do.

### H. Languages

34. `[backend]` **Languages have no data path at all.** The sheet prints
    `Common, +N bonus language slot(s) (not yet selectable)` from a TS function
    (`CharacterSheet.tsx:542-548`). Racial languages (Dwarven, Elven…) from the race's
    `Languages` trait, the bonus-language list per race, and Linguistics ranks are all absent.
    The corpus has `language` records (`data/corpus/*/language/`) and
    `list_reference_library_catalog` serves the `language` kind. Expose (a) the race's automatic
    and bonus languages, (b) a persisted language selection on the character. Whether the
    engine computes the *slot count* is Open question Q7.
35. `[frontend]` **Language picker** — depends on 34.

### I. Equipment

36. `[frontend]` **Gear tab can only add arms & armor.** Both pickers hard-code
    `WEAPONS_AND_ARMOR_CATEGORY = 'ArmsArmor'` (`CharacterSheet.tsx:210`); the catalog also
    serves `General`, `MagicItems`, `Cloth`, `Backpack`… (equipmentCatalog grep). A player
    cannot buy a backpack, rope, rations, a holy symbol, a spell component pouch, or a potion.
37. `[frontend]` **Picker rows don't show cost, weight or proficiency before purchase.** The
    purchase charges real gold atomically, so the player finds out the price by their balance
    dropping. `list_equipment` entries carry cost/weight (the Gear tab shows them post-purchase).
38. `[frontend]` **No quantity.** 20 arrows = 20 picks. See Open question Q8.
39. `[backend]` **Everything bought is `EquippedActive`; nothing can be stowed or unequipped.**
    `handleAddEquipment` hard-codes the state; no `set_equipment_active_state` command exists,
    though the engine's `EquipmentSelection` carries the enum. A character buying two suits of
    armor wears both. Wrap a state-change command; then `[frontend]` toggle.
40. `[frontend]` **Melee attack bonus is computed but never shown.** `PilotSnapshotDto.
    baselineMeleeAttackBonus` arrives on every load and `AttackPanel` renders BAB/CMB/CMD only.
    Ranged attack bonus and per-weapon attack totals are engine gaps (Open blocker B5).
41. `[backend]` **Weapon proficiency is a 5-entry TS set.** `MARTIAL_WEAPON_CLASSES` in
    `characterProgression.ts:218` marks 26 of 31 classes as simple-only and ignores racial weapon
    familiarity and Martial Weapon Proficiency (the engine reads
    `choice:martial_weapon_proficiency_target`). The corpus class records carry proficiency
    grants (the Unchained Barbarian comment cites them). Expose proficiency per character.
42. `[frontend]` **Speed ignores armor and load.** `SpeedPanel` prints the race roster's base
    speed; medium/heavy armor and medium/heavy load reduce it and the Defense tab already shows
    the load tier. Whether the engine emits an adjusted speed record is Open question Q9.
43. `[frontend]` **Weapons tab "Print" button duplicates the menu Print** and both call
    `window.print()` with zero print CSS (grep table) — the three-column app chrome, the Menu bar
    and the collapsed progression rail all print. Part of item 55.

### J. Spells

44. `[frontend]` **Multiclass casters cannot choose which class learns a spell.**
    `resolveSpellRouting` prefers Wizard then `heldClasses[0]` (`CharacterSheet.tsx:2975`); a
    Cleric 3 / Wizard 2 can only ever add Wizard spells. The picker needs a source-class chooser.
45. `[frontend]` **"Add Spell" is offered to non-casters and offers them the whole catalog.**
    For a single-class Fighter, routing picks `class:fighter`, `list_class_spell_levels` reports
    `known: false`, and `buildSpellPickerOffering` falls back to all 1185 records. Hide/disable
    the affordance when no held class has a known list (the `known` flag is the engine's signal;
    Magus/Summoner/Oracle are the honest exception).
46. `[frontend]` **Known vs. prepared is not a player action.** Every add is `Known` (or the
    Wizard bootstrap's atomic Known+Prepared); there is no "prepare for today" / "unprepare",
    and `add_spell_selection` already accepts `acquisitionMode: 'Prepared'`. Spontaneous casters'
    known-spell limits and daily slot usage are Open question Q10.
47. `[frontend]` **Casters pick no spells at creation.** A Wizard's starter spellbook is engine-
    seeded silently; a Sorcerer 1 is created with zero known spells and nothing says "you may
    know 4 cantrips and 2 first-level spells". The Spells tab works after the fact. Part of Q2.
48. `[frontend]` **After a level-up, the Spells tab doesn't say what became available.**
    Spells-per-day rows update, but nothing highlights "Level 2 spells now castable" or the
    Wizard's two free spells per level. (Question 2 of the brief for casters.)
49. `[backend]` **Caster level is a 6-class TS set.** `CASTER_CLASSES` in
    `characterProgression.ts:396` lists Wizard/Sorcerer/Cleric/Druid/Bard/Arcanist; Paladin,
    Ranger, Oracle, Witch, Summoner, Inquisitor, Alchemist, Magus… read "—". The engine grounds
    `class_spell.*` records per caster; expose caster level (or `backend` confirms no such
    record exists → Open blockers).
50. `[frontend]` **Cantrips/orisons, domain spells and school spells are undistinguished** in
    both the picker and the tab. Open question Q11.

### K. Level-up (`LevelUpDialog.tsx`, `handleLevelUpAccept`)

51. `[frontend]` **HP per level is silently `hp:average`.** No roll/average/max choice; the
    sheet's HP is `maxHitPoints` in TS (`characterProgression.ts:421`) — rules in TS — while the
    engine's `load_character_durability` supports single-class Fighter/Wizard/Rogue only
    (`CharacterSheet.tsx:2605-2612`). The choice slot is recorded but "nothing reads it"
    (`:3141-3146`). Offering a choice that changes nothing would be a stub; the engine half is
    Open blocker B1.
52. `[frontend]` **Initiative is `dexMod`.** `InitiativeHpPanel initiative={dexMod}`; Improved
    Initiative and the `trait.standalone.initiative_bonus` record the engine already emits are
    ignored. Summing them here is rules-in-TS; a single engine initiative total is Open
    blocker B6. Rendering the trait record's existence as a line is `[frontend]`.
53. `[frontend]` **Level-up accepted → no "what changed / what you now qualify for" surface.**
    The dialog previews grants; after accept the sheet just refreshes. No diff, no "new feats
    eligible", no "unspent skill points / unpicked talent" checklist. Open question Q12.
54. `[frontend]` **XP is not tracked.** No XP field, no "next level at" — a table player's most
    basic level-up trigger. Pure sidecar data like bio/money (`[backend]` sibling to add an
    `xp` field to the bio or money sidecar).

### L. Review / output (the sheet as a thing to play from)

55. `[frontend]` **Print produces the app, not a sheet.** No `@media print` rules exist (grep
    table); the menu bar, tab strip, buttons and collapsed rail print. A dice-at-the-table
    product needs a printable sheet.
56. `[frontend]` **Export is on the Load screen only**, not in the sheet's ☰ menu
    (`menuItems`, `CharacterSheet.tsx:3554-3558`), so a player reviewing their finished
    character has to close it to export it.
57. `[frontend]` **`Overrides` tab is a "coming soon" placeholder** (`TABS` includes it;
    fallthrough at `:3963`). A visible tab with no behaviour — no-stub doctrine. Remove it or
    wire it.
58. `[frontend]` **Defense tab prints "Save modifiers by source — coming soon."**
    (`:1432`). `snapshot.baseSaves` and `totalSaves` both arrive today; render base vs. total
    per save and drop the placeholder.
59. `[frontend]` **Conditions / active states have no toggle.** The engine models activation
    (`activation=rage:active`, `src/rules_core/character_input.rs:844`) and the Barbarian's
    "not raging" posture is documented as the default; no UI lets a player rage. `backend` to
    confirm `recompute_character` accepts an activation before `frontend` adds a toggle.

---

## Open questions (shape undecided — orchestrator's call)

- **Q1 — Multiclass at creation (item 10).** Two shapes: (a) creation stays single-class and
  a post-create "add class levels" wizard drives `level_up_character` N times; (b)
  `CreateCharacterRequest` grows a class list. (a) is pure `[frontend]`; (b) is `[backend]`+
  engine-adjacent. Which?
- **Q2 — Creating above level 1 (items 11, 20, 28, 47).** Same fork: (a) always create at level
  1 then run the real level-up flow N−1 times, collecting feats/skills/spells/talents each step;
  (b) a batch "level N choices" step. (a) reuses everything that exists.
- **Q3 — Aging (item 7).** Drop the age preview from ability scores, or persist age and ask the
  engine? The engine has no aging model (grep `aging|venerable` in `src/rules_core` → no rules
  hit), so persisting would be TS-authored rules.
- **Q4 — Pre-save ripple (item 17).** Is a "preview compute" of an unsaved build wanted, or is
  "create, then see the sheet" acceptable? `recompute_character` only takes a saved id.
- **Q5 — Subskills (item 24).** Craft/Perform/Profession/Knowledge specialties: separate rows,
  or a specialty field on the row? And the `skill:knowledge_arcana` id convention is
  unconfirmed against the engine.
- **Q6 — Feat budget (item 26).** Does any engine record state total feat slots for a build? If
  not, this is an engine gap, not a UI one.
- **Q7 — Bonus language slot count (item 34).** Is `INT modifier` slots computed anywhere in
  the engine, or does the count stay client-side prose?
- **Q8 — Quantity (item 38).** Stackable items: a `quantity` on `EquipmentSelection`, or N
  selections?
- **Q9 — Speed (item 42).** Does the engine emit an armor/load-adjusted speed record?
- **Q10 — Daily spell state (item 46).** Should the sheet track slots used today (session
  state) or only the static prepared list?
- **Q11 — Spell categories (item 50).** How should domain/school/cantrip rows be distinguished?
- **Q12 — Post-level-up delta (item 53).** A one-shot "what changed" panel, or a persistent
  "unfinished choices" checklist on the sheet?

## Open blockers (require repo-root `src/` — escalate per AGENTS.md Blocker Discipline)

- **B1 — HP per level and HP total.** `rules_core::durability` covers single-class
  Fighter/Wizard/Rogue; the level-up hit-die choice is not consumed. Until the engine computes
  HP for every class/multiclass and reads the hit-die choice, the sheet's HP is TS math
  (`characterProgression.ts:maxHitPoints`). Blocks items 51 and the §3.3 rule for HP.
- **B2 — Ability score increases at 4/8/12/16/20.** No engine choice slot or record
  (`grep -rl ability_score_increase src/rules_core` → none). Blocks item 18.
- **B3 — Per-skill modifiers for all 36 skills.** `skill_allocation.rs` recognises 5 skill
  ids and emits `skill.selected_modifier.unsupported` for the rest. Blocks item 22 beyond the
  first 5.
- **B4 — Level-1 feat seeding.** `pf1_adapter.rs` `compose_character_input` always seeds
  Power Attack + Weapon Focus. Blocks the "clean" half of item 28.
- **B5 — Per-weapon attack totals, ranged attack bonus, summed damage.** Documented engine
  refusal (`contract.rs` `PilotReceipt::weapon_damage`). Blocks the rest of item 40.
- **B6 — Initiative total.** Only `trait.standalone.initiative_bonus` exists; no
  `combat.initiative` record. Blocks item 52.
- **B7 — Class alignment prerequisites.** No alignment rule in `src/rules_core`. Blocks the
  rule half of item 15.
- **B8 — Trait count / category cap.** `trait_effects.rs` deliberately allows stacking. Blocks
  the rule half of item 31.
- **B9 — Fighter bonus combat feat candidates.** `level_up/fighter.rs` leaves `pick_from_lists`
  empty for its ten slots (documented in `characterProgression.ts:331-343`). Blocks even-level
  Fighter feat picks in item 27.
- **B10 — Starting wealth for non-CRB classes.** `rules_core::money::starting_wealth_gp`
  covers the 11 CRB classes (`character_hub.rs:1222-1224`); the 20 APG/ACG/PU classes start at
  0 gp and can buy nothing. (`money.rs` is in the engine crate: `ls src/rules_core/money.rs`.)
- **B11 — Favored class bonus.** No engine concept (`grep -rl favored_class src/rules_core` →
  none).
- **B13 — Choice-trait characters cannot be saved at all.** `trait_effects.rs:517`
  `trait_skill_choice_id` emits a 3-segment id (`trait_choice:trait:trait_criminal`) while
  `local_store.rs:254` rejects any `choice_set_id` that is not exactly 2 segments. Every
  `SKILL_CHOICE_TRAIT_BONUSES` / `FAMILY_CHOICE_TRAIT_BONUSES` trait (Criminal, Fiend Blood,
  Harvester...) fails to save when its skill is chosen — and the create form sends that payload
  today (`CreateCharacterForm.tsx:613`). Pre-existing, user-facing, not sprint-caused. Found by
  `backend` during B-4, confirmed by the orchestrator. Full write-up with both candidate one-line
  fixes: `engine-handoff-trait-choice-save.md`. Operator ruling 2026-09-01: hand to the concurrent
  engine session rather than edit `src/` from two sessions at once; B-4 ships flat-traits-only.
  **Corrects this file's own "already wired" line** crediting per-trait skill choice — true for
  flat traits, false for the choice-trait subset.

- **B12 — Archetypes and prestige classes.** The picker offers 31 base classes; the engine has
  6 files mentioning "archetype" (`grep -rli archetype src/rules_core/*.rs | wc -l`) but no
  selectable archetype surface reaches Tauri. Not a v0.8 UI ticket; recorded so it is not
  rediscovered.

## Summary counts

| Tag | Count | Command |
|---|---|---|
| `[frontend]` | 49 | `grep -cE '^[0-9]+\. `\[frontend\]`' docs/release/v0.8/scout-gap-audit.md` |
| `[backend]` | 10 | `grep -cE '^[0-9]+\. `\[backend\]`' docs/release/v0.8/scout-gap-audit.md` |
| Open questions | 12 | `grep -c '^- \*\*Q' docs/release/v0.8/scout-gap-audit.md` |
| Open blockers | 12 | `grep -c '^- \*\*B' docs/release/v0.8/scout-gap-audit.md` |

Items that need a `[backend]` command *before* a `[frontend]` picker (1, 6, 12→13, 19, 33,
34→35, 39) are tagged by the half that must land first, with the sibling named inline, per
brief §4.1's "two tickets, sequenced" rule.

Not done (deliberately, per the spawn brief): the §3.5 DM Toolkit stretch pass.
