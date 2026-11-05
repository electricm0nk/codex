import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { assert, assertEqual } from '../testSupport/asserts';
import type { PrintSheetPdfRequest, SaveSheetPdfRequest, SavedSheetPdf } from '../boundary/saveSheetPdf';
import {
  PRINT_OPTION_BOXES,
  campaignNotesFor,
  printFileName,
  runPrintAction,
  type PrintDialogDeps,
  type PrintOptions,
} from './PrintDialog';

/**
 * The Print dialog (operator rulings R3, R4, R5, R19): the paper (the setting, overridable once),
 * the three stacking option boxes, Save PDF (save-as, then the backend opens the file) and Print
 * (the backend writes a temp PDF and opens it in the system viewer). The flow is tested with its
 * native dialog and both commands injected; the sources are read to prove the three sheets host
 * the dialog and the browser print path is gone.
 */

const here = dirname(fileURLToPath(import.meta.url));
const srcRoot = join(here, '..');

interface Calls {
  saveDefault: string[];
  saved: SaveSheetPdfRequest[];
  printed: PrintSheetPdfRequest[];
}

function fakeDeps(overrides: Partial<PrintDialogDeps> = {}): { deps: PrintDialogDeps; calls: Calls } {
  const calls: Calls = { saveDefault: [], saved: [], printed: [] };
  const answer = (filePath: string): SavedSheetPdf => ({ filePath, bytes: 4096, warnings: [] });
  const deps: PrintDialogDeps = {
    hasRuntime: () => true,
    save: async (defaultFileName) => {
      calls.saveDefault.push(defaultFileName);
      return '/home/player/Aldric.pdf';
    },
    saveSheetPdf: async (request) => {
      calls.saved.push(request);
      return answer(request.filePath);
    },
    printSheetPdf: async (request) => {
      calls.printed.push(request);
      return answer(`/tmp/codex-print/${request.characterId}.pdf`);
    },
    campaigns: () => [
      { name: 'Rise of the Runelords', description: 'Sandpoint, autumn.', partyCharacterIds: ['char-aldric', 'char-elowen'] },
      { name: 'Skull and Shackles', description: 'At sea.', partyCharacterIds: ['char-elowen'] },
    ],
    ...overrides,
  };
  return { deps, calls };
}

const target = { characterId: 'char-aldric', displayLabel: 'Aldric Ironhand' };
const letterAll: PrintOptions = { paper: 'letter', spellbook: true, math: true, bio: true };

async function verifiesSavePdfAsksWhereThenSendsEveryOption() {
  const { deps, calls } = fakeDeps();
  const outcome = await runPrintAction('save', target, letterAll, deps);
  assertEqual(calls.saveDefault.join('|'), 'Aldric Ironhand.pdf', 'the save-as dialog offers the character name as a PDF');
  assertEqual(calls.saved.length, 1, 'one save');
  assertEqual(
    JSON.stringify(calls.saved[0]),
    JSON.stringify({
      characterId: 'char-aldric',
      paper: 'letter',
      spellbook: true,
      math: true,
      bio: true,
      campaigns: [{ name: 'Rise of the Runelords', description: 'Sandpoint, autumn.' }],
      filePath: '/home/player/Aldric.pdf',
    }),
    'the request carries the paper, the three boxes, the campaigns that list the character and the chosen path',
  );
  assertEqual(calls.printed.length, 0, 'Save PDF does not print');
  assertEqual(outcome.kind, 'Saved', 'saved');
  assertEqual(outcome.kind === 'Saved' ? outcome.message : '', 'Saved /home/player/Aldric.pdf (4096 bytes) and opened it.', 'the message names the file');
}

async function verifiesADismissedSaveDialogWritesNothing() {
  const { deps, calls } = fakeDeps({ save: async () => null });
  const outcome = await runPrintAction('save', target, letterAll, deps);
  assertEqual(outcome.kind, 'Cancelled', 'cancelled');
  assertEqual(calls.saved.length, 0, 'nothing saved');
}

async function verifiesPrintWritesTheTempCopyWithNoDialog() {
  const { deps, calls } = fakeDeps();
  const basic: PrintOptions = { paper: 'a4', spellbook: false, math: false, bio: false };
  const outcome = await runPrintAction('print', target, basic, deps);
  assertEqual(calls.saveDefault.length, 0, 'Print asks for no path');
  assertEqual(
    JSON.stringify(calls.printed[0]),
    JSON.stringify({ characterId: 'char-aldric', paper: 'a4', spellbook: false, math: false, bio: false, campaigns: [] }),
    'a basic print sends no boxes and no campaigns',
  );
  assertEqual(outcome.kind, 'Printed', 'printed');
  assertEqual(
    outcome.kind === 'Printed' ? outcome.message : '',
    'Opened /tmp/codex-print/char-aldric.pdf in the PDF viewer. Print from there.',
    'the message says where to print from',
  );
}

async function verifiesWarningsAndFailuresAreShown() {
  const warned = fakeDeps({
    saveSheetPdf: async (request) => ({ filePath: request.filePath, bytes: 10, warnings: ['page 2 overflows'] }),
  });
  const outcome = await runPrintAction('save', target, letterAll, warned.deps);
  assert(outcome.kind === 'Saved' && outcome.message.endsWith(' Warnings: page 2 overflows.'), 'a Typst warning is shown, not dropped');

  const refused = fakeDeps({
    printSheetPdf: async () => {
      throw new Error('Made /tmp/codex-print/char-aldric.pdf, but it could not be opened: no viewer');
    },
  });
  const failed = await runPrintAction('print', target, letterAll, refused.deps);
  assertEqual(failed.kind, 'Failed', 'a refused print fails');
  assertEqual(
    failed.kind === 'Failed' ? failed.message : '',
    'Made /tmp/codex-print/char-aldric.pdf, but it could not be opened: no viewer',
    'with the backend\'s words',
  );

  const offline = fakeDeps({ hasRuntime: () => false });
  const noRuntime = await runPrintAction('save', target, letterAll, offline.deps);
  assertEqual(noRuntime.kind, 'Failed', 'no runtime fails');
  assertEqual(offline.calls.saveDefault.length, 0, 'and opens no dialog');
}

function verifiesCampaignNotesAndFileNames() {
  const { deps } = fakeDeps();
  assertEqual(campaignNotesFor('char-elowen', deps.campaigns()).map((note) => note.name).join('|'), 'Rise of the Runelords|Skull and Shackles', 'both campaigns list Elowen');
  assertEqual(campaignNotesFor('char-nobody', deps.campaigns()).length, 0, 'a character no campaign lists sends none');
  assertEqual(printFileName('Aldric Ironhand'), 'Aldric Ironhand.pdf', 'name kept');
  assertEqual(printFileName('V/../'), 'V.pdf', 'path characters dropped');
  assertEqual(printFileName('...'), 'character.pdf', 'an empty name falls back');
  assertEqual(PRINT_OPTION_BOXES.map((box) => box.label).join('|'), 'Spellbook|Show the math|Bio and campaign notes', 'the three boxes of R4, in order');
}

function collectSources(dir: string): string[] {
  const files: string[] = [];
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const full = join(dir, entry.name);
    if (entry.isDirectory()) {
      files.push(...collectSources(full));
    } else if (/\.(ts|tsx)$/.test(entry.name)) {
      files.push(full);
    }
  }
  return files;
}

function verifiesTheThreeSheetsHostTheDialogAndTheBrowserPrintPathIsGone() {
  for (const sheet of ['CharacterSheet.tsx', 'StarfinderCharacterSheet.tsx', 'CyberpunkCharacterSheet.tsx']) {
    const source = readFileSync(join(here, sheet), 'utf8');
    assert(source.includes("from './PrintDialog'"), `${sheet} imports the Print dialog`);
    assert(/<PrintDialog\b/.test(source), `${sheet} renders the Print dialog`);
  }
  // Built from parts so this file is not itself a hit of the card's acceptance grep.
  const retired = ['window' + '.print', 'print' + 'Layout'];
  const hits = collectSources(srcRoot).filter((file) => {
    const text = readFileSync(file, 'utf8');
    return retired.some((word) => text.includes(word));
  });
  assertEqual(hits.join(', '), '', 'no source under src/ calls the browser print or reads the CSS print layout');
  assert(!existsSync(join(here, 'print' + 'Layout.ts')), 'the CSS print layout file is deleted');
}

async function main() {
  await verifiesSavePdfAsksWhereThenSendsEveryOption();
  await verifiesADismissedSaveDialogWritesNothing();
  await verifiesPrintWritesTheTempCopyWithNoDialog();
  await verifiesWarningsAndFailuresAreShown();
  verifiesCampaignNotesAndFileNames();
  verifiesTheThreeSheetsHostTheDialogAndTheBrowserPrintPathIsGone();
  console.log('PrintDialog.test.ts: all assertions passed');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
