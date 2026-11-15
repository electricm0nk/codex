import type { PrintSectionsDto } from '../boundary/saveSheetPdf';
import type { PrintPaperStorage } from './printPaper';

/**
 * The Print dialog's ticks, remembered per character on this machine (SD-40 `decisions.md §23.4`,
 * §24.9), in localStorage under `codex.print.sections.<characterId>` the way `printPaper.ts` keeps
 * the paper. Not saved with the character: an export or a clone starts from the first-time ticks.
 */

/** First time, or a stored value that does not parse: Primary, Abilities and Feats, Inventory. */
export const FIRST_TIME_SECTIONS: PrintSectionsDto = {
  main: true,
  feats: true,
  inventory: true,
  spellbook: false,
  bio: false,
  progression: false,
  math: false,
  notes: false,
};

const keyFor = (characterId: string) => `codex.print.sections.${characterId}`;

function parseSections(stored: string): PrintSectionsDto | null {
  let value: unknown;
  try {
    value = JSON.parse(stored);
  } catch {
    return null;
  }
  if (typeof value !== 'object' || value === null || Array.isArray(value)) {
    return null;
  }
  const record = value as Record<string, unknown>;
  const keys = Object.keys(FIRST_TIME_SECTIONS) as Array<keyof PrintSectionsDto>;
  if (!keys.every((key) => typeof record[key] === 'boolean')) {
    return null;
  }
  return Object.fromEntries(keys.map((key) => [key, record[key]])) as unknown as PrintSectionsDto;
}

export function getPrintSections(characterId: string, storage?: PrintPaperStorage): PrintSectionsDto {
  try {
    const stored = (storage ?? localStorage).getItem(keyFor(characterId));
    return (stored !== null && parseSections(stored)) || { ...FIRST_TIME_SECTIONS };
  } catch {
    return { ...FIRST_TIME_SECTIONS };
  }
}

export function setPrintSections(characterId: string, sections: PrintSectionsDto, storage?: PrintPaperStorage): void {
  try {
    (storage ?? localStorage).setItem(keyFor(characterId), JSON.stringify(sections));
  } catch {
    /* non-persistent environments still print the boxes ticked in the dialog */
  }
}
