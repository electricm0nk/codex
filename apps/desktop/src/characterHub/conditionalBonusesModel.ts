import type { ConditionalNoteDto, ConditionalNotesDto, FeatSkillBonusesDto } from '../boundary/loadSavedCharacterDetail';

/**
 * SD-40 E2.25 (`decisions.md §24.3.8`): the lines the sheet's Saving Throws box lists under its
 * totals, the same entries in the same order as the printed Conditional Bonuses box
 * (`print/blocks/core.rs` `conditional_bonuses`): HP, Initiative, the saves (a rule on all three
 * once under `Saves`, else under the saves that hold it), AC, CMB, CMD, then what a feat states on
 * a skill and no total adds. Each line is `<what>: <label>: <value> <condition>` in the engine's
 * words; nothing is added or worked out here. None → `None stated.`.
 */
export function conditionalBonusLines(notes: ConditionalNotesDto | undefined, skills: FeatSkillBonusesDto): string[] {
  const lines: string[] = [];
  const noted = (what: string, list: ConditionalNoteDto[] | undefined) => (list ?? []).forEach((note) => lines.push(`${what}: ${noteWords(note)}`));
  noted('HP', notes?.hitPoints);
  noted('Initiative', notes?.initiative);
  const saves: Array<[string, ConditionalNoteDto[]]> = [
    ['Fortitude', notes?.fortitude ?? []],
    ['Reflex', notes?.reflex ?? []],
    ['Will', notes?.will ?? []],
  ];
  const printed = new Set<string>();
  for (const [, list] of saves) {
    for (const note of list) {
      if (printed.has(note.ruleId)) continue;
      printed.add(note.ruleId);
      const held = saves.filter(([, other]) => other.some((n) => n.ruleId === note.ruleId)).map(([name]) => name);
      lines.push(`${held.length === saves.length ? 'Saves' : held.join(' / ')}: ${noteWords(note)}`);
    }
  }
  noted('AC', notes?.armorClass);
  noted('CMB', notes?.combatManeuverBonus);
  noted('CMD', notes?.combatManeuverDefense);
  for (const line of [...skills.situational, ...skills.unknown]) lines.push(`Skills: ${line.label} on ${skillName(line.skill)}: ${line.reason}`);
  return lines.length > 0 ? lines : ['None stated.'];
}

function noteWords(note: ConditionalNoteDto): string {
  return `${note.label}: ${[note.value, note.condition].filter((part) => part !== '').join(' ')}`;
}

/** The print's `skill_name` (`print/blocks/skills.rs`): `knowledge_arcana` → `Knowledge (Arcana)`. */
function skillName(slug: string): string {
  for (const family of ['knowledge', 'craft', 'perform', 'profession']) {
    if (slug.startsWith(`${family}_`)) return `${wordsOf(family)} (${titleWords(slug.slice(family.length + 1))})`;
  }
  return titleWords(slug);
}

function titleWords(slug: string): string {
  return slug
    .split(/[_ ]/)
    .filter((word) => word !== '')
    .map((word, index) => (index > 0 && word === 'of' ? word : wordsOf(word)))
    .join(' ');
}

function wordsOf(slug: string): string {
  const spaced = slug.replace(/_/g, ' ');
  return spaced.charAt(0).toUpperCase() + spaced.slice(1);
}
