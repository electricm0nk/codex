/**
 * Data-driven catalogues for the Character Hub Phase 1 golden-path builder.
 *
 * These are structured as arrays/lookups (not hardcoded UI branches) so
 * later phases can widen them (more classes, more races, more game systems)
 * without a component redesign.
 */

export interface RaceOption {
  id: string;
  label: string;
}

/** Races with real, tested fixtures proving a `Computed` receipt. */
export const RACE_OPTIONS: RaceOption[] = [
  { id: 'race:human', label: 'Human' },
  { id: 'race:half-elf', label: 'Half-Elf' },
  { id: 'race:half-orc', label: 'Half-Orc' },
  { id: 'race:gnome', label: 'Gnome' },
];

export interface ClassOption {
  id: string;
  label: string;
  enabled: boolean;
}

/** Today exactly one entry — the only class the compute engine fully supports. */
export const CLASS_OPTIONS: ClassOption[] = [{ id: 'class:fighter', label: 'Fighter', enabled: true }];

export const LEVEL_OPTIONS: number[] = [1, 2, 3];

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
export const GOLDEN_PATH_LOADOUT_SUMMARY = {
  feats: ['Power Attack', 'Dodge', 'Weapon Focus (Longsword)'],
  skills: ['Climb 1 rank', 'Intimidate 1 rank', 'Swim 1 rank'],
  equipment: ['Longsword (equipped)', 'Chain Shirt (equipped)', 'No shield'],
};
