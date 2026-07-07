/**
 * Data-driven catalogues for the Character Hub picker.
 *
 * These are structured as arrays/lookups (not hardcoded UI branches) so
 * later phases can widen them (more races, more game systems) without a
 * component redesign.
 */

export interface RaceOption {
  id: string;
  label: string;
}

/** The full PF1 core rulebook race roster. */
export const RACE_OPTIONS: RaceOption[] = [
  { id: 'race:human', label: 'Human' },
  { id: 'race:dwarf', label: 'Dwarf' },
  { id: 'race:elf', label: 'Elf' },
  { id: 'race:gnome', label: 'Gnome' },
  { id: 'race:half-elf', label: 'Half-Elf' },
  { id: 'race:half-orc', label: 'Half-Orc' },
  { id: 'race:halfling', label: 'Halfling' },
];

/**
 * `full` — reaches `Computed` for any race in `RACE_OPTIONS`.
 * `partial-human-only` — the compute engine's named class seam only fires
 * for `race:human`; every other race falls back to the same 4 generic
 * diagnostics as a `none` class.
 * `none` — no dedicated compute seam exists; every race produces the same 4
 * generic diagnostics.
 */
export type ClassSupportLevel = 'full' | 'partial-human-only' | 'none';

export interface ClassOption {
  id: string;
  label: string;
  supportLevel: ClassSupportLevel;
  levelOptions: number[];
}

/** The full PF1 core rulebook class roster. */
export const CLASS_OPTIONS: ClassOption[] = [
  { id: 'class:fighter', label: 'Fighter', supportLevel: 'full', levelOptions: [1, 2, 3] },
  { id: 'class:paladin', label: 'Paladin', supportLevel: 'partial-human-only', levelOptions: [1] },
  { id: 'class:ranger', label: 'Ranger', supportLevel: 'partial-human-only', levelOptions: [1] },
  { id: 'class:sorcerer', label: 'Sorcerer', supportLevel: 'partial-human-only', levelOptions: [1] },
  { id: 'class:wizard', label: 'Wizard', supportLevel: 'partial-human-only', levelOptions: [1] },
  { id: 'class:bard', label: 'Bard', supportLevel: 'partial-human-only', levelOptions: [1] },
  { id: 'class:barbarian', label: 'Barbarian', supportLevel: 'partial-human-only', levelOptions: [1] },
  { id: 'class:rogue', label: 'Rogue', supportLevel: 'none', levelOptions: [1] },
  { id: 'class:cleric', label: 'Cleric', supportLevel: 'none', levelOptions: [1] },
  { id: 'class:druid', label: 'Druid', supportLevel: 'none', levelOptions: [1] },
  { id: 'class:monk', label: 'Monk', supportLevel: 'none', levelOptions: [1] },
];

const DEFAULT_LEVEL_OPTIONS: number[] = [1];

export function getLevelOptionsForClass(classId: string): number[] {
  return CLASS_OPTIONS.find((option) => option.id === classId)?.levelOptions ?? DEFAULT_LEVEL_OPTIONS;
}

export function describeClassSupportLevel(supportLevel: ClassSupportLevel, classLabel: string): string {
  switch (supportLevel) {
    case 'full':
      return `${classLabel} is fully computed at every level offered here.`;
    case 'partial-human-only':
      return `${classLabel} is only computed for Human today — other races show what's still missing.`;
    case 'none':
      return `${classLabel} isn't computed by the engine yet.`;
  }
}

export const DEFAULT_ABILITY_SCORES = {
  strength: 16,
  dexterity: 14,
  constitution: 14,
  intelligence: 10,
  wisdom: 12,
  charisma: 8,
};

export const GAME_SYSTEM_LABELS: Record<string, string> = {
  pf1: 'Pathfinder 1st Edition',
};

/** The fixed feat/skill/equipment loadout, shown to the user rather than hidden. */
export const FIXED_LOADOUT_SUMMARY = {
  feats: ['Power Attack', 'Dodge', 'Weapon Focus (Longsword)'],
  skills: ['Climb 1 rank', 'Intimidate 1 rank', 'Swim 1 rank'],
  equipment: ['Longsword (equipped)', 'Chain Shirt (equipped)', 'No shield'],
};
