/**
 * Settings, Print paper (operator ruling R3): the paper every printed sheet uses, A4 until the
 * player picks US Letter, remembered in localStorage like the theme mode. The Print dialog starts
 * from it and may override it for one print.
 */

export type PrintPaper = 'a4' | 'letter';

/** The papers the print backend lays out (`print/model.rs` `Paper`), in the order the app offers them. */
export const PRINT_PAPERS: ReadonlyArray<{ value: PrintPaper; label: string }> = [
  { value: 'a4', label: 'A4' },
  { value: 'letter', label: 'US Letter' },
];

export interface PrintPaperStorage {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
}

const PRINT_PAPER_KEY = 'codex.print.paper';

export function isPrintPaper(value: string): value is PrintPaper {
  return PRINT_PAPERS.some((paper) => paper.value === value);
}

export function getPrintPaper(storage?: PrintPaperStorage): PrintPaper {
  try {
    const stored = (storage ?? localStorage).getItem(PRINT_PAPER_KEY);
    return stored !== null && isPrintPaper(stored) ? stored : 'a4';
  } catch {
    return 'a4';
  }
}

export function setPrintPaper(paper: PrintPaper, storage?: PrintPaperStorage): void {
  try {
    (storage ?? localStorage).setItem(PRINT_PAPER_KEY, paper);
  } catch {
    /* non-persistent environments (e.g. private mode) still print on the paper picked in the dialog */
  }
}
