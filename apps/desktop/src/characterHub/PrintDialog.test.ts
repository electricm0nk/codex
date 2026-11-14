import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { assert, assertEqual } from '../testSupport/asserts';
import type { PrintSheetPdfRequest, SaveSheetPdfRequest, SavedSheetPdf } from '../boundary/saveSheetPdf';
import { FIRST_TIME_SECTIONS } from '../settings/printSections';
import {
  PRINT_SECTION_BOXES,
  boxesFor,
  campaignNotesFor,
  printFileName,
  printSummary,
  runPrintAction,
  sectionsToSend,
  type PrintDialogDeps,
  type PrintOptions,
} from './PrintDialog';

/**
 * The Print dialog (operator rulings R3, R5, R19; SD-40 `decisions.md §23.4`, §24.3.7, §24.9): the
 * paper (the setting, overridable once), the eight section boxes in box order (four on a Starfinder
 * or Cyberpunk RED sheet), remembered per character (`printSections.test.ts`), Save PDF (save-as, then the backend opens the file) and Print
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
const every = { main: true, feats: true, inventory: true, spellbook: true, bio: true, progression: true, math: true, notes: true };
const letterAll: PrintOptions = { paper: 'letter', sections: every };

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
      sections: every,
      campaigns: [{ name: 'Rise of the Runelords', description: 'Sandpoint, autumn.' }],
      filePath: '/home/player/Aldric.pdf',
    }),
    'the request carries the paper, the eight sections, the campaigns that list the character and the chosen path',
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
  const basic: PrintOptions = { paper: 'a4', sections: FIRST_TIME_SECTIONS };
  const outcome = await runPrintAction('print', target, basic, deps);
  assertEqual(calls.saveDefault.length, 0, 'Print asks for no path');
  assertEqual(
    JSON.stringify(calls.printed[0]),
    JSON.stringify({ characterId: 'char-aldric', paper: 'a4', sections: FIRST_TIME_SECTIONS, campaigns: [] }),
    'a first-time print sends sections 1+2+3 and, without Bio, no campaigns',
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
}

function verifiesTheEightBoxesInBoxOrder() {
  assertEqual(
    PRINT_SECTION_BOXES.map((box) => box.label).join('|'),
    'Primary (page 1)|Abilities and Feats|Inventory|Spellbook|Bio|Progression|Math|Notes',
    'eight boxes labelled with the section titles, in box order',
  );
  assertEqual(
    PRINT_SECTION_BOXES.map((box) => box.testId).join('|'),
    [
      'print-section-main',
      'print-section-feats',
      'print-section-inventory',
      'print-section-spellbook',
      'print-section-bio',
      'print-section-progression',
      'print-section-math',
      'print-section-notes',
    ].join('|'),
    'each box addressable by data-testid print-section-<key>',
  );
  assertEqual(boxesFor(true).length, 8, 'a Pathfinder sheet shows all eight');
  assertEqual(boxesFor(false).map((box) => box.label).join('|'), 'Primary (page 1)|Spellbook|Bio|Math', 'Starfinder and Cyberpunk RED show 1, 4, 5, 7');
  const source = readFileSync(join(here, 'PrintDialog.tsx'), 'utf8');
  assert(source.includes('data-testid={section.testId}'), 'the dialog renders each box with its data-testid');
  assert(!source.includes('<legend'), 'the old Include fieldset is gone');
}

function verifiesWhatIsSentAndTheSummary() {
  assertEqual(JSON.stringify(sectionsToSend(every, true)), JSON.stringify(every), 'Pathfinder sends every tick');
  assertEqual(
    JSON.stringify(sectionsToSend(every, false)),
    JSON.stringify({ ...every, feats: false, inventory: false, progression: false, notes: false }),
    'a Starfinder or Cyberpunk RED print sends only its four boxes',
  );
  assertEqual(printSummary(FIRST_TIME_SECTIONS), 'Prints: Primary (page 1), Abilities and Feats, Inventory.', 'the first-time ticks, named');
  assertEqual(printSummary({ ...FIRST_TIME_SECTIONS, bio: true }), 'Prints: Primary (page 1), Abilities and Feats, Inventory, Bio.', 'in box order');
  const nothing = { ...FIRST_TIME_SECTIONS, main: false, feats: false, inventory: false };
  assertEqual(printSummary(nothing), 'Tick at least one section to print.', 'nothing ticked says why the buttons are off');
  assertEqual(printSummary(sectionsToSend({ ...nothing, feats: true }, false)), 'Tick at least one section to print.', 'a hidden box ticked is not a print');
  const source = readFileSync(join(here, 'PrintDialog.tsx'), 'utf8');
  assert(/disabled=\{busy !== null \|\| nothingTicked\}/.test(source), 'nothing ticked disables Save PDF and Print');
  for (const sheet of ['StarfinderCharacterSheet.tsx', 'CyberpunkCharacterSheet.tsx']) {
    assert(readFileSync(join(here, sheet), 'utf8').includes('pathfinder={false}'), `${sheet} shows the four boxes`);
  }
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
  verifiesTheEightBoxesInBoxOrder();
  verifiesWhatIsSentAndTheSummary();
  verifiesTheThreeSheetsHostTheDialogAndTheBrowserPrintPathIsGone();
  console.log('PrintDialog.test.ts: all assertions passed');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
