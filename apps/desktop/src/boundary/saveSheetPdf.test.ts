import { printSheetPdf, saveSheetPdf, type PrintSectionsDto } from './saveSheetPdf';
import { assertEqual } from '../testSupport/asserts';

/**
 * No Tauri runtime is available under `tsx`, so this exercises the no-runtime failure path of both
 * print commands' boundaries, as every other boundary test does (see `exportCharacter.test.ts`).
 */
async function messageOf(run: () => Promise<unknown>): Promise<string> {
  try {
    await run();
  } catch (cause) {
    return cause instanceof Error ? cause.message : String(cause);
  }
  return 'did not throw';
}

async function main() {
  const options = { paper: 'a4' as const, spellbook: false, math: false, bio: false, campaigns: [] };
  assertEqual(
    await messageOf(() => saveSheetPdf({ characterId: 'char-test', filePath: '/tmp/sheet.pdf', ...options })),
    'Tauri runtime not available for saving a sheet PDF',
    'Save PDF without the runtime says so',
  );
  assertEqual(
    await messageOf(() => printSheetPdf({ characterId: 'char-test', ...options })),
    'Tauri runtime not available for printing a sheet',
    'Print without the runtime says so',
  );
  const sections: PrintSectionsDto = {
    main: true,
    feats: false,
    inventory: false,
    spellbook: false,
    bio: false,
    progression: false,
    math: false,
    notes: true,
  };
  assertEqual(
    await messageOf(() => printSheetPdf({ characterId: 'char-test', ...options, sections })),
    'Tauri runtime not available for printing a sheet',
    'A request carrying sections is the same command',
  );
  console.log('saveSheetPdf.test.ts: all assertions passed');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
