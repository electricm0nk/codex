import type { CharacterHubListSurface } from './buildCharacterHubListSurface';
import type { LoadSavedCharacterResponse } from '../boundary/loadSavedCharacterDetail';

/**
 * Sample data for the browser preview (localhost:1420) where the Tauri backend
 * is absent, so `list_saved_characters` / `load_saved_character` cannot run.
 * This keeps the Load → sheet flow walkable for UI work; the real desktop app
 * never uses it (guarded by `hasTauriRuntime()`).
 */

export const PREVIEW_CHARACTER_ID = 'preview:aldric-ironhand';

// Aldric is a Fighter 3 / Wizard 1 multiclass so the sheet exercises both the
// per-class level rail and the multiclass "Next" options.
const PREVIEW_CLASS_SUMMARY = 'class:fighter:3,class:wizard:1';

export function buildPreviewListSurface(): CharacterHubListSurface {
  return {
    rows: [
      {
        characterId: PREVIEW_CHARACTER_ID,
        displayLabel: 'Aldric Ironhand',
        gameSystemLabel: 'Pathfinder 1st Edition',
        raceLabel: 'Human',
        classSummary: PREVIEW_CLASS_SUMMARY,
        savedAtLabel: 'Preview',
        campaign: 'The Trouble with Trolls',
      },
    ],
    isEmpty: false,
    emptyStateMessage: null,
    unreadableNotice: 'Preview mode — no desktop backend detected; showing a sample character.',
  };
}

export function buildPreviewDetail(): LoadSavedCharacterResponse {
  return {
    // The preview character takes no alternate racial trait. Inventing one
    // would put a swap on a sample sheet that no engine resolved.
    selectedAlternateTraitKeys: [],
    // AT-34-E4-002: the preview character takes no traits either, same
    // "no fabricated default" reasoning as the alternate-trait comment
    // above.
    selectedTraits: [],
    // And no racial-trait resolution: every sentence in one is *rendered* by
    // the engine from the corpus row's own `DESC:` tokens against this
    // character's feats. Hand-writing a sample here would put corpus-looking
    // rules prose on screen that no corpus produced — the fixture-in-a-
    // production-path failure `docs/governance/no-stub-mvp-doctrine.md`
    // prohibits. The preview says the section is unavailable instead.
    resolvedRacialTraits: null,
    summary: {
      characterId: PREVIEW_CHARACTER_ID,
      displayLabel: 'Aldric Ironhand',
      gameSystem: 'pf1',
      schemaVersion: 1,
      savedAt: 'preview',
      raceId: 'race:human',
      classSummary: PREVIEW_CLASS_SUMMARY,
    },
    snapshot: {
      // Fighter 3 / Wizard 1: BAB +3, Fighter good Fort + Wizard good Will.
      abilityModifiers: { strength: 3, dexterity: 1, constitution: 2, intelligence: 2, wisdom: 1, charisma: -1 },
      baseAttackBonus: 3,
      baseSaves: { fortitude: 3, reflex: 1, will: 3 },
      baselineMeleeAttackBonus: 6,
      baselineArmorClass: 14,
      totalSaves: { fortitude: 5, reflex: 2, will: 4 },
      selectedSkillModifiers: { climb: 6, intimidate: 3, swim: 6 },
    },
    diagnostics: [],
    corpusDerived: {
      schoolCoverage: [
        { school: 'Abjuration', spells: ['Alarm'], grounded: true },
        { school: 'Illusion', spells: ['Blur'], grounded: true },
      ],
      equippedItems: [
        {
          itemId: 'item:longsword',
          equipmentRecordName: 'Longsword',
          equipmentRecordKey: 'Longsword (Base)',
          grounded: true,
          appliedModifiers: [
            {
              itemId: 'item:masterwork',
              equipmentRecordName: 'Masterwork',
              equipmentRecordKey: 'Masterwork',
              grounded: false,
              appliedModifiers: [],
            },
          ],
        },
        {
          itemId: 'item:chain_shirt',
          equipmentRecordName: 'Chain Shirt',
          equipmentRecordKey: 'Chain Shirt (Base)',
          grounded: false,
          appliedModifiers: [],
        },
      ],
      equipmentEffects: {
        // Real CRB values for Aldric's two equipped records: the Chain
        // Shirt's own ACCHECK:-2 / MAXDEX:4 / SPELLFAILURE:20 / +4 armor
        // bonus, and a Longsword, which contributes no armor stats at all.
        perItem: [
          { itemId: 'item:longsword', equipmentRecordKey: 'Longsword (Base)', category: 'ArmsArmor' },
          {
            itemId: 'item:chain_shirt',
            equipmentRecordKey: 'Chain Shirt (Base)',
            category: 'ArmsArmor',
            armorClassBonus: 4,
            maxDex: 4,
            spellFailure: 20,
            armorCheckPenalty: -2,
          },
        ],
        armorClassDelta: 4,
        armorCheckPenaltyTotal: -2,
        maxDexCap: 4,
        attackBonusDelta: 0,
      },
      // Aldric's Strength modifier is +3 (score 16), whose real load.lst
      // row is LOAD:16|230 — light 76 / medium 153 / heavy 230. He carries
      // a Longsword (4 lb, 15 gp) and a Chain Shirt (25 lb, 100 gp), all
      // real CRB WT:/COST: values, so 29 lb is comfortably a light load and
      // imposes no penalties of its own.
      encumbrance: {
        totalCarriedWeightLbs: 29,
        totalCarriedCostGp: 115,
        lightMaxLbs: 76,
        mediumMaxLbs: 153,
        heavyMaxLbs: 230,
        level: 'Light',
        loadArmorCheckPenalty: 0,
        perItem: [
          { itemId: 'item:longsword', weightLbs: 4, costGp: 15 },
          { itemId: 'item:chain_shirt', weightLbs: 25, costGp: 100 },
        ],
        unresolvedItemIds: [],
      },
      unresolvedSpellIds: [],
      unresolvedEquipmentItemIds: [],
    },
    selectedFeats: ['feat:power_attack', 'feat:dodge', 'feat:weapon_focus'],
  // Mirrors the real seeded Fighter loadout: `compose_character_input`
  // records Weapon Focus's target through the Fighter bonus-feat slot, so
  // the preview shows a resolved target rather than an untargeted feat.
  chosenFeatTargets: [{ featId: 'feat:weapon_focus', targetKind: 'Weapon', targets: ['longsword'] }],
    spellsSelected: [],
    // Real records the engine emits for a Fighter 3 / Wizard 1, copied
    // verbatim from its own output shape (id, value and the citation text
    // it writes) so the browser preview exercises the same rendering path
    // the desktop app does — not a prettified stand-in.
    explanations: [
      {
        id: 'class_feature.fighter.bravery',
        value: 1,
        detail:
          'Fighter level 2 Bravery (cr_abilities_class.lst Fighter; +1 at level 2 and another +1 ' +
          'every 4 Fighter levels thereafter): grants +1 to Will saves against fear. This is a ' +
          'flat, non-fabricated bonus magnitude only — no fear-condition or Will-save-resolution ' +
          'engine exists anywhere in this codebase, so this bonus is never folded into the ' +
          'unconditional Will save total',
      },
      {
        id: 'class_chassis.wizard.scribe_scroll',
        value: 1,
        detail:
          'Wizard level 1 Scribe Scroll bonus feat from the PF1 Core Rulebook Wizard class table: ' +
          'granted once, at 1st level',
      },
      {
        id: 'class_spell.wizard.total_spells_per_day.spell_level_0',
        value: 3,
        detail:
          'Wizard level 1 spells per day for spell level 0 from the PF1 Core Rulebook Wizard ' +
          'class table: 3 (cantrips take no Intelligence bonus spells)',
      },
      {
        id: 'class_spell.wizard.total_spells_per_day.spell_level_1',
        value: 2,
        detail:
          'Wizard level 1 spells per day for spell level 1: 1 base from the PF1 Core Rulebook ' +
          'Wizard class table + 1 Intelligence bonus spell (Intelligence modifier +2) = 2',
      },
    ],
    // The engine's real breakdown for Aldric's equipped Longsword. The
    // Chain Shirt is deliberately absent: it is not a weapon, and the
    // engine omits non-weapons entirely rather than listing them with
    // zeroed facets.
    weaponDamage: [
      {
        weaponItemId: 'item:longsword',
        weaponRecordKey: 'Longsword (Base)',
        baseDice: { count: 1, dieSize: 8 },
        strDamageModifier: 3,
        wieldCategory: 'OneHanded',
        enhancementAttackBonus: null,
        enhancementDamageBonus: null,
        criticalThreatRange: [19, 20],
        criticalMultiplier: 2,
        featEffects: [],
      },
    ],
  };
}
