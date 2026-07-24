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
      equipmentEffects: { armorClassDelta: 4, armorCheckPenaltyTotal: -2, maxDexCap: 4, attackBonusDelta: 0 },
      unresolvedSpellIds: [],
      unresolvedEquipmentItemIds: [],
    },
    selectedFeats: ['feat:power_attack', 'feat:dodge', 'feat:weapon_focus'],
    spellsSelected: [],
  };
}
