import { conditionalBonusLines } from './conditionalBonusesModel';
import type { ConditionalNoteDto, ConditionalNotesDto, FeatSkillBonusesDto } from '../boundary/loadSavedCharacterDetail';
import { NO_FEAT_SKILL_BONUSES } from '../boundary/loadSavedCharacterDetail';
import { assertEqual } from '../testSupport/asserts';

// SD-40 E2.25 (decisions.md §24.3.8): the sheet's Saving Throws box lists, under the totals, the
// same lines the printed Conditional Bonuses box prints (print/blocks/core.rs `conditional_bonuses`),
// in its order, in the engine's words. The model orders and de-duplicates; it adds no number.

const note = (label: string, value: string, condition: string, ruleId: string): ConditionalNoteDto => ({ label, value, condition, ruleId });
const none = (): ConditionalNotesDto => ({
  hitPoints: [],
  initiative: [],
  fortitude: [],
  reflex: [],
  will: [],
  armorClass: [],
  combatManeuverBonus: [],
  combatManeuverDefense: [],
});

// Aldric (Human Fighter 3): Bravery on Will only.
const bravery = note('Bravery', '+1', 'vs. fear', 'core_rulebook:class_feature:bravery');
assertEqual(conditionalBonusLines({ ...none(), will: [bravery] }, NO_FEAT_SKILL_BONUSES).join('|'), 'Will: Bravery: +1 vs. fear', 'Aldric: one Will line naming Bravery');

// A Dwarf's Hardy is on all three saves: one line, under `Saves`; a rule on two saves names both.
const hardy = note('Hardy', '+2', 'vs. poison, spells, and spell-like abilities', 'core_rulebook:race_trait:dwarf_hardy');
const lucky = note('Lucky', '+1', '', 'x:race_trait:lucky');
assertEqual(
  conditionalBonusLines({ ...none(), fortitude: [hardy, lucky], reflex: [hardy], will: [hardy, lucky] }, NO_FEAT_SKILL_BONUSES).join('|'),
  'Saves: Hardy: +2 vs. poison, spells, and spell-like abilities|Fortitude / Will: Lucky: +1',
  'a rule on several saves prints once',
);

// Every kind, in the printed box's order: HP, Initiative, saves, AC, CMB, CMD, then skills.
const skills: FeatSkillBonusesDto = {
  ...NO_FEAT_SKILL_BONUSES,
  situational: [{ skill: 'knowledge_arcana', ruleId: 'a', label: 'Arcane Lore', reason: '+2 when identifying spells' }],
  unknown: [{ skill: 'perception', ruleId: 'b', label: 'Keen Eye', reason: 'no stated value' }],
};
const every: ConditionalNotesDto = {
  hitPoints: [note('Toughness', '+3', 'when stable', 'h')],
  initiative: [note('Alert', '+2', 'in darkness', 'i')],
  fortitude: [],
  reflex: [note('Evasion', '', 'no damage on a save', 'r')],
  will: [],
  armorClass: [note('Dodge Trait', '+1', 'vs. giants', 'a')],
  combatManeuverBonus: [note('Relentless', '+1', 'to bull rush', 'cmb')],
  combatManeuverDefense: [note('Stability', '+4', 'against bull rush or trip', 'cmd')],
};
assertEqual(
  conditionalBonusLines(every, skills).join('|'),
  [
    'HP: Toughness: +3 when stable',
    'Initiative: Alert: +2 in darkness',
    'Reflex: Evasion: no damage on a save',
    'AC: Dodge Trait: +1 vs. giants',
    'CMB: Relentless: +1 to bull rush',
    'CMD: Stability: +4 against bull rush or trip',
    'Skills: Arcane Lore on Knowledge (Arcana): +2 when identifying spells',
    'Skills: Keen Eye on Perception: no stated value',
  ].join('|'),
  'every kind, in the printed order',
);

// Nothing stated (and an older response with no conditionalNotes at all).
assertEqual(conditionalBonusLines(none(), NO_FEAT_SKILL_BONUSES).join('|'), 'None stated.', 'empty → None stated.');
assertEqual(conditionalBonusLines(undefined, NO_FEAT_SKILL_BONUSES).join('|'), 'None stated.', 'absent → None stated.');

console.log('conditionalBonusesModel.test.ts: all assertions passed');
