import { OVERRIDE_TOGGLES, type CustomOverrides } from './customModel';
import { CYBERPUNK_GAME_SYSTEM } from './cyberpunkSheetModel';
import { STARFINDER_GAME_SYSTEM } from './starfinderSheetModel';

/** The four existing `Overrides` flags House Rules edit; the Custom tab no longer shows them (decisions.md §11, Q-3.4). */
export const HOUSE_RULE_FLAGS: ReadonlyArray<keyof CustomOverrides> = [
  'ignoreSlotLimits',
  'featsWithoutPrerequisites',
  'weaponsWithoutProficiency',
  'armorWithoutProficiency',
];

/** A house rule: a flag the engine reads, or (flag null) a notice naming the card that brings its engine. */
export interface HouseRule {
  label: string;
  flag: keyof CustomOverrides | null;
  notice: string | null;
}

const labelOf = (flag: keyof CustomOverrides): string => OVERRIDE_TOGGLES.find((toggle) => toggle.key === flag)?.label ?? flag;
const flagRule = (flag: keyof CustomOverrides): HouseRule => ({ label: labelOf(flag), flag, notice: null });

/** The intake's six, in its order (decisions.md §11). */
const PATHFINDER_HOUSE_RULES: ReadonlyArray<HouseRule> = [
  flagRule('ignoreSlotLimits'),
  { label: 'Hit points at level-up', flag: null, notice: 'The seven level-up hit point methods arrive with card E6.3.' },
  flagRule('featsWithoutPrerequisites'),
  flagRule('weaponsWithoutProficiency'),
  flagRule('armorWithoutProficiency'),
  { label: 'Ambidextrous', flag: null, notice: 'No off-hand penalty arrives with card E6.4.' },
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
