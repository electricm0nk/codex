import type { FeatCatalogEntryDto } from '../boundary/listFeats';
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

const FEAT_BOOK_LABELS: Record<string, string> = { Crb: 'CRB', Apg: 'APG', Acg: 'ACG', Arg: 'ARG', Pu: 'PU' };

/**
 * Feats for the Manage dialog, with the draft character's verdicts. A feat that needs a target
 * (a weapon, skill or school) is listed but not selectable here: its target is chosen on the sheet.
 * A key the catalog lists twice (PU re-lists Endurance) is offered once.
 */
export function featItems(entries: FeatCatalogEntryDto[]): TransferItem[] {
  const seen = new Set<string>();
  const items: TransferItem[] = [];
  for (const entry of entries) {
    if (seen.has(entry.key)) {
      continue;
    }
    seen.add(entry.key);
    const verdict = entry.eligibility;
    const book = FEAT_BOOK_LABELS[entry.source] ?? entry.source;
    const lines = [entry.description ?? 'No description available.'];
    if (verdict !== undefined && verdict.met.length > 0) {
      lines.push(`Prerequisites met: ${verdict.met.join('; ')}.`);
    }
    if (verdict !== undefined && verdict.unmet.length > 0) {
      lines.push(`Prerequisites not met: ${verdict.unmet.join('; ')}.`);
    }
    if (verdict !== undefined && verdict.unverified.length > 0) {
      lines.push(`Could not be checked: ${verdict.unverified.join('; ')}.`);
    }
    const needsTarget = entry.chooserTargetKind !== null;
    const eligible = verdict === undefined || verdict.eligible;
    items.push({
      id: entry.key,
      label: `${entry.name} (${book})`,
      description: lines.join('\n\n'),
      qualified: eligible && !needsTarget,
      ...(needsTarget
        ? { unqualifiedReason: `Choose this feat's ${entry.chooserTargetKind?.toLowerCase()} target on the sheet after creation.` }
        : eligible
          ? {}
          : { unqualifiedReason: verdict?.unavailableReason ?? 'The character does not qualify.' }),
    });
  }
  return items;
}

/**
 * How many feats a new character picks: one at every odd character level (1st, 3rd, ...), plus a
 * human's bonus feat. Class bonus feats (a Fighter's, say) are class features chosen on the sheet.
 */
export function creationFeatSlots(characterLevel: number, raceId: string): number {
  if (characterLevel <= 0) {
    return 0;
  }
  return Math.ceil(characterLevel / 2) + (raceId === 'race:human' ? 1 : 0);
}
