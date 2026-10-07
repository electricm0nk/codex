import type { CharacterTraitOptionDto } from '../boundary/loadCharacterTraits';
import type { AlternateTraitRow } from './alternateTraitSelection';
import type { RacialTraitRow } from './racialTraitsModel';
import type { TransferItem } from './transferListModel';

/** PF1 gives a new character two traits. */
export const CHARACTER_TRAIT_LIMIT = 2;

/**
 * Racial traits for the Manage dialog: every alternate racial trait is a selectable option (one the
 * current selection locks out is listed as not qualified, with the backend's reason), and the
 * standard traits the race grants are innate (shown, never moved). Prose is the engine's.
 */
export function alternateTraitItems(rows: AlternateTraitRow[], innate: RacialTraitRow[]): TransferItem[] {
  const alternates = rows.map((row): TransferItem => {
    const replaces =
      row.alternate.replaces.length > 0
        ? `Replaces ${row.alternate.replaces.map((link) => link.name).join(', ')}.`
        : `Replaces nothing in the loaded books (${row.alternate.setsFlags.join(', ')}).`;
    const incomplete =
      row.droppedArgs.length > 0 ? `\n\nThe engine could not resolve ${row.droppedArgs.join(', ')}, so this description is incomplete.` : '';
    return {
      id: row.alternate.key,
      label: `${row.alternate.name} (${row.alternate.book}${row.alternate.sourcePage === null ? '' : ` ${row.alternate.sourcePage}`})`,
      description: `${row.description}\n\n${replaces}${incomplete}`,
      qualified: row.disabledReason === null,
      ...(row.disabledReason === null ? {} : { unqualifiedReason: row.disabledReason }),
    };
  });
  const granted = innate.map(
    (row): TransferItem => ({
      id: `innate:${row.key}`,
      label: row.name,
      description: row.text,
      qualified: true,
      innate: true,
    })
  );
  return [...alternates, ...granted];
}

/** The numbers a character trait gives, as a short phrase (`+1 Acrobatics`, `+1 Will save`). */
export function traitBonusSummary(option: CharacterTraitOptionDto): string {
  const signed = (value: number) => (value >= 0 ? `+${value}` : String(value));
  const isChoiceBased = option.skillOptions.length > 0;
  const parts: string[] = [];
  if (option.abilitySubstitution !== null) {
    const flat = option.abilitySubstitution.flatBonus;
    parts.push(`${option.skills.join(', ')} (ability-based${flat !== 0 ? `, +${flat} flat` : ''})`);
  } else if (option.skills.length > 0 || option.save !== null || isChoiceBased) {
    const what = isChoiceBased
      ? `choice of ${option.skillOptions.map((choice) => choice.name).join(', ')}`
      : option.save !== null
        ? `${option.save} save`
        : option.skills.join(', ');
    parts.push(`${signed(option.bonus)} ${what}`);
  }
  if (option.otherPillars.length > 0) {
    parts.push(option.otherPillars.map((pillar) => `${signed(pillar.bonus)} ${pillar.label}`).join(', '));
  }
  return parts.join('; ');
}

/** Character traits for the Manage dialog. Every one offered genuinely computes, so all are qualified. */
export function characterTraitItems(options: CharacterTraitOptionDto[]): TransferItem[] {
  return options.map((option) => {
    const summary = traitBonusSummary(option);
    return {
      id: option.id,
      label: option.name,
      description: summary ? `${summary}\n\n${option.description}` : option.description,
      qualified: true,
    };
  });
}
