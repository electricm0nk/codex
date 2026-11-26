import { OVERRIDE_TOGGLES, type CustomOverrides, type HpMethod } from './customModel';
import { CYBERPUNK_GAME_SYSTEM } from './cyberpunkSheetModel';
import { STARFINDER_GAME_SYSTEM } from './starfinderSheetModel';

/** The four existing `Overrides` flags House Rules edit; the Custom tab no longer shows them (decisions.md §11, Q-3.4). */
export const HOUSE_RULE_FLAGS: ReadonlyArray<keyof CustomOverrides> = [
  'ignoreSlotLimits',
  'featsWithoutPrerequisites',
  'weaponsWithoutProficiency',
  'armorWithoutProficiency',
];

/** A house rule: a flag the engine reads, the HP method choice, or (both null) a notice naming the card that brings its engine. */
export interface HouseRule {
  label: string;
  flag: keyof CustomOverrides | null;
  choice: 'hpMethod' | null;
  notice: string | null;
}

/** The seven level-up hit point methods, in the intake's order (item 3); level 1 always takes the full die. */
export const HP_METHODS: ReadonlyArray<{ value: HpMethod; label: string }> = [
  { value: 'userRolled', label: 'User Rolled' },
  { value: 'random', label: 'Random' },
  { value: 'randomHalfFloor', label: 'Random, 50% floor' },
  { value: 'average', label: 'Average' },
  { value: 'averageRoundUp', label: 'Average, round up' },
  { value: 'maximum', label: 'Maximum' },
  { value: 'percentage', label: 'Percentage' },
];

/** What the level-up asks the player for under `method`: the die result, a percentage of the die maximum, or nothing. */
export function hitPointEntryFor(method: HpMethod | undefined): 'roll' | 'percent' | null {
  return method === 'userRolled' ? 'roll' : method === 'percentage' ? 'percent' : null;
}

const labelOf = (flag: keyof CustomOverrides): string => OVERRIDE_TOGGLES.find((toggle) => toggle.key === flag)?.label ?? flag;
const flagRule = (flag: keyof CustomOverrides): HouseRule => ({ label: labelOf(flag), flag, choice: null, notice: null });

/** The intake's six, in its order (decisions.md §11). */
const PATHFINDER_HOUSE_RULES: ReadonlyArray<HouseRule> = [
  flagRule('ignoreSlotLimits'),
  { label: 'Hit points at level-up', flag: null, choice: 'hpMethod', notice: null },
  flagRule('featsWithoutPrerequisites'),
  flagRule('weaponsWithoutProficiency'),
  flagRule('armorWithoutProficiency'),
  { label: 'Ambidextrous', flag: null, choice: null, notice: 'No off-hand penalty arrives with card E6.4.' },
];

/**
 * The house rules a character of this game system can set. Cyberpunk RED has none of the six; Starfinder
 * none either, since no flag has a Starfinder consumer (E0.5 rest-and-house-rules.md §5, Q-3.5).
 */
export function houseRulesFor(gameSystem: string): ReadonlyArray<HouseRule> {
  return gameSystem === CYBERPUNK_GAME_SYSTEM || gameSystem === STARFINDER_GAME_SYSTEM ? [] : PATHFINDER_HOUSE_RULES;
}

/** The Custom tab's Overrides group: every flag but the four House Rules own. */
export const CUSTOM_TAB_TOGGLES = OVERRIDE_TOGGLES.filter((toggle) => !HOUSE_RULE_FLAGS.includes(toggle.key));
