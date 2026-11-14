import { useEffect, useState, type CSSProperties } from 'react';
import { save } from '@tauri-apps/plugin-dialog';
import { hasTauriRuntime } from '../boundary/runtime';
import {
  printSheetPdf,
  saveSheetPdf,
  type CampaignNoteDto,
  type PrintSectionsDto,
  type PrintSheetPdfRequest,
  type SaveSheetPdfRequest,
  type SavedSheetPdf,
} from '../boundary/saveSheetPdf';
import { getCampaigns, type Campaign } from '../campaign/campaignModel';
import { PRINT_PAPERS, getPrintPaper, isPrintPaper, type PrintPaper } from '../settings/printPaper';
import { getPrintSections, setPrintSections } from '../settings/printSections';

/**
 * The Print dialog every character sheet opens (SD-39 E8.4; operator rulings R3, R4, R5, R19).
 *
 * The paper starts from Settings, Print paper, and may be changed for this print only. The section
 * boxes (SD-40 `decisions.md §23.4`, §24.3.7, §24.9) pick what prints, each section on its own pages,
 * and are remembered per character (`settings/printSections.ts`). **Save PDF** asks
 * where with the native save-as dialog, and the backend writes the sheet there and opens it.
 * **Print** has the backend write the same PDF to a temp file and open it in the system PDF viewer;
 * the player prints from there. The sheet is rendered by the Rust print pipeline from the saved
 * character, so what prints is what the engine computed, never what the page shows.
 */

export interface PrintOptions {
  paper: PrintPaper;
  sections: PrintSectionsDto;
}

interface PrintSectionBox {
  key: keyof PrintSectionsDto;
  label: string;
  testId: string;
  /** Only the Pathfinder sheet prints this section; Starfinder and Cyberpunk RED have no such box. */
  pathfinderOnly: boolean;
}

const box = (key: keyof PrintSectionsDto, label: string, pathfinderOnly: boolean): PrintSectionBox => ({
  key,
  label,
  testId: `print-section-${key}`,
  pathfinderOnly,
});

/** The eight section boxes, in box order, labelled with the section titles (`decisions.md §24.3.7`, §24.9). */
export const PRINT_SECTION_BOXES: ReadonlyArray<PrintSectionBox> = [
  box('main', 'Primary (page 1)', false),
  box('feats', 'Abilities and Feats', true),
  box('inventory', 'Inventory', true),
  box('spellbook', 'Spellbook', false),
  box('bio', 'Bio', false),
  box('progression', 'Progression', true),
  box('math', 'Math', false),
  box('notes', 'Notes', true),
];

/** The boxes a sheet shows: all eight on Pathfinder; 1, 4, 5 and 7 on Starfinder and Cyberpunk RED. */
export function boxesFor(pathfinder: boolean): ReadonlyArray<PrintSectionBox> {
  return PRINT_SECTION_BOXES.filter((section) => pathfinder || !section.pathfinderOnly);
}

/** What the request carries: the ticks of the boxes shown; a box the sheet does not show is off. */
export function sectionsToSend(ticks: PrintSectionsDto, pathfinder: boolean): PrintSectionsDto {
  const shown = new Set(boxesFor(pathfinder).map((section) => section.key));
  return Object.fromEntries(PRINT_SECTION_BOXES.map((section) => [section.key, shown.has(section.key) && ticks[section.key]])) as unknown as PrintSectionsDto;
}

/** The line under the boxes: what will print, or why the buttons are off. */
export function printSummary(sections: PrintSectionsDto): string {
  const ticked = PRINT_SECTION_BOXES.filter((section) => sections[section.key]).map((section) => section.label);
  return ticked.length === 0 ? 'Tick at least one section to print.' : `Prints: ${ticked.join(', ')}.`;
}

export interface PrintDialogDeps {
  hasRuntime: () => boolean;
  /** Native save dialog; resolves `null` when the player dismisses it. */
  save: (defaultFileName: string) => Promise<string | null>;
  saveSheetPdf: (request: SaveSheetPdfRequest) => Promise<SavedSheetPdf>;
  printSheetPdf: (request: PrintSheetPdfRequest) => Promise<SavedSheetPdf>;
  /** The campaign store; only the campaigns whose party lists the character are sent. */
  campaigns: () => ReadonlyArray<Pick<Campaign, 'name' | 'description' | 'partyCharacterIds'>>;
}

export type PrintOutcome =
  | { kind: 'Saved'; message: string }
  | { kind: 'Printed'; message: string }
  | { kind: 'Cancelled' }
  | { kind: 'Failed'; message: string };

/** Filesystem-safe default name: the character's name, as the Export file is named. */
export function printFileName(displayLabel: string): string {
  const safeName = displayLabel.replace(/[^a-z0-9-_ ]/gi, '').trim() || 'character';
  return `${safeName}.pdf`;
}

/** The campaigns whose party holds the character, as the print's Bio and campaign notes take them (R17). */
export function campaignNotesFor(
  characterId: string,
  campaigns: ReadonlyArray<Pick<Campaign, 'name' | 'description' | 'partyCharacterIds'>>,
): CampaignNoteDto[] {
  return campaigns
    .filter((campaign) => campaign.partyCharacterIds.includes(characterId))
    .map((campaign) => ({ name: campaign.name, description: campaign.description }));
}

function warningsSuffix(saved: SavedSheetPdf): string {
  return saved.warnings.length === 0 ? '' : ` Warnings: ${saved.warnings.join('; ')}.`;
}

export async function runPrintAction(
  action: 'save' | 'print',
  target: { characterId: string; displayLabel: string },
  options: PrintOptions,
  deps: PrintDialogDeps,
): Promise<PrintOutcome> {
  if (!deps.hasRuntime()) {
    return { kind: 'Failed', message: 'Printing requires the desktop runtime.' };
  }
  try {
    const request: PrintSheetPdfRequest = {
      characterId: target.characterId,
      paper: options.paper,
      sections: options.sections,
      // Only the Bio section prints them; a print without that box sends none.
      campaigns: options.sections.bio ? campaignNotesFor(target.characterId, deps.campaigns()) : [],
    };
    if (action === 'print') {
      const printed = await deps.printSheetPdf(request);
      return { kind: 'Printed', message: `Opened ${printed.filePath} in the PDF viewer. Print from there.${warningsSuffix(printed)}` };
    }
    const filePath = await deps.save(printFileName(target.displayLabel));
    if (!filePath) {
      return { kind: 'Cancelled' };
    }
    const saved = await deps.saveSheetPdf({ ...request, filePath });
    return { kind: 'Saved', message: `Saved ${saved.filePath} (${saved.bytes} bytes) and opened it.${warningsSuffix(saved)}` };
  } catch (cause: unknown) {
    return { kind: 'Failed', message: cause instanceof Error ? cause.message : String(cause) };
  }
}

/** Production wiring: the real save-as dialog, runtime check, both commands and the campaign store. */
export const DESKTOP_PRINT_DEPS: PrintDialogDeps = {
  hasRuntime: hasTauriRuntime,
  save: (defaultFileName) => save({ title: 'Save PDF', defaultPath: defaultFileName, filters: [{ name: 'PDF', extensions: ['pdf'] }] }),
  saveSheetPdf,
  printSheetPdf,
  campaigns: getCampaigns,
};

const buttonStyle: CSSProperties = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  color: 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '0.9rem',
  padding: '0.45rem 0.9rem',
};

export function PrintDialog(props: {
  characterId: string;
  displayLabel: string;
  onClose: () => void;
  /** False on a Starfinder or Cyberpunk RED sheet, which shows boxes 1, 4, 5 and 7 only. */
  pathfinder?: boolean;
  deps?: PrintDialogDeps;
}) {
  const deps = props.deps ?? DESKTOP_PRINT_DEPS;
  const { onClose } = props;
  const pathfinder = props.pathfinder ?? true;
  const [paper, setPaper] = useState<PrintPaper>(getPrintPaper);
  const [ticks, setTicks] = useState<PrintSectionsDto>(() => getPrintSections(props.characterId));
  const sections = sectionsToSend(ticks, pathfinder);
  const nothingTicked = PRINT_SECTION_BOXES.every((section) => !sections[section.key]);
  const [busy, setBusy] = useState<'save' | 'print' | null>(null);
  const [message, setMessage] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        onClose();
      }
    };
    window.addEventListener('keydown', onKeyDown);
    return () => window.removeEventListener('keydown', onKeyDown);
  }, [onClose]);

  async function run(action: 'save' | 'print') {
    setBusy(action);
    setMessage(null);
    setError(null);
    try {
      const outcome = await runPrintAction(action, { characterId: props.characterId, displayLabel: props.displayLabel }, { paper, sections }, deps);
      if (outcome.kind === 'Saved' || outcome.kind === 'Printed') {
        setMessage(outcome.message);
      } else if (outcome.kind === 'Failed') {
        setError(outcome.message);
      }
    } finally {
      setBusy(null);
    }
  }

  return (
    <div
      role="presentation"
      onClick={onClose}
      style={{ alignItems: 'center', backgroundColor: 'rgba(0, 0, 0, 0.55)', display: 'flex', inset: 0, justifyContent: 'center', padding: '2rem', position: 'fixed', zIndex: 1000 }}
    >
      <div
        role="dialog"
        aria-modal="true"
        aria-label="Print sheet"
        data-testid="print-dialog"
        onClick={(event) => event.stopPropagation()}
        style={{
          backgroundColor: 'var(--color-surface)',
          border: '1px solid var(--color-border)',
          borderRadius: 12,
          boxShadow: '0 24px 60px rgba(0, 0, 0, 0.55)',
          display: 'flex',
          flexDirection: 'column',
          gap: '0.9rem',
          maxWidth: '96vw',
          padding: '1.25rem 1.5rem',
          width: 480,
        }}
      >
        <header style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between' }}>
          <h2 style={{ fontSize: '1.1rem', margin: 0 }}>Print {props.displayLabel}</h2>
          <button
            type="button"
            aria-label="Close print"
            onClick={onClose}
            style={{ background: 'none', border: 'none', color: 'var(--color-text-muted)', cursor: 'pointer', fontSize: '1.3rem', lineHeight: 1 }}
          >
            ×
          </button>
        </header>

        <label style={{ alignItems: 'center', display: 'flex', gap: '0.75rem' }}>
          <span style={{ fontWeight: 600 }}>Paper</span>
          <select
            aria-label="Paper"
            value={paper}
            onChange={(event) => {
              const next = event.target.value;
              if (isPrintPaper(next)) {
                setPaper(next);
              }
            }}
            style={{ ...buttonStyle, cursor: 'pointer' }}
          >
            {PRINT_PAPERS.map((paper) => (
              <option key={paper.value} value={paper.value}>
                {paper.label}
              </option>
            ))}
          </select>
          <span style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem' }}>From Settings, Print; a change here is for this print only.</span>
        </label>

        <div role="group" aria-label="Sections to print" style={{ border: '1px solid var(--color-border)', borderRadius: 8, display: 'flex', flexDirection: 'column', gap: '0.4rem', padding: '0.6rem 0.9rem' }}>
          {boxesFor(pathfinder).map((section) => (
            <label key={section.key} style={{ alignItems: 'center', display: 'flex', gap: '0.5rem' }}>
              <input
                type="checkbox"
                aria-label={`Print ${section.label}`}
                data-testid={section.testId}
                checked={ticks[section.key]}
                onChange={(event) => {
                  const next = { ...ticks, [section.key]: event.target.checked };
                  setTicks(next);
                  setPrintSections(props.characterId, next);
                }}
              />
              <span style={{ fontWeight: 600 }}>{section.label}</span>
            </label>
          ))}
          <p data-testid="print-dialog-summary" style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: 0 }}>
            {printSummary(sections)}
          </p>
        </div>

        {message === null ? null : (
          <p role="status" data-testid="print-dialog-status" style={{ fontSize: '0.85rem', margin: 0, overflowWrap: 'anywhere' }}>
            {message}
          </p>
        )}
        {error === null ? null : (
          <p role="alert" data-testid="print-dialog-error" style={{ color: 'var(--color-error)', fontSize: '0.85rem', margin: 0, overflowWrap: 'anywhere' }}>
            {error}
          </p>
        )}

        <div style={{ display: 'flex', gap: '0.6rem', justifyContent: 'flex-end' }}>
          <button type="button" onClick={onClose} style={buttonStyle}>
            Cancel
          </button>
          <button type="button" disabled={busy !== null || nothingTicked} onClick={() => void run('save')} style={buttonStyle}>
            {busy === 'save' ? 'Saving…' : 'Save PDF'}
          </button>
          <button type="button" disabled={busy !== null || nothingTicked} onClick={() => void run('print')} style={buttonStyle}>
            {busy === 'print' ? 'Printing…' : 'Print'}
          </button>
        </div>
      </div>
    </div>
  );
}
