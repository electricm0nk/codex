import { exportFileName, runCharacterExport } from './characterExport';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * F-8 (scout audit item 56): Export existed only on the Load screen, so a
 * player looking at their finished sheet had to back out to save a file.
 * The sheet's ☰ menu now carries the same real export (`export_character`
 * builds the payload server-side from the on-disk build). This pins the
 * flow's decisions with the dialog and boundary call injected, so the test
 * proves the real wrapper is invoked with the chosen path — not a "would
 * have exported" string.
 */

assertEqual(exportFileName('Thrain Ironfist (Copy)'), 'Thrain Ironfist Copy.json', 'strips unsafe characters');
assertEqual(exportFileName('!!!'), 'character.json', 'falls back when nothing safe remains');

async function main() {
  const calls: Array<{ characterId: string; filePath: string }> = [];
  const deps = {
    hasRuntime: () => true,
    save: async () => '/tmp/out.json',
    exportCharacter: async (req: { characterId: string; filePath: string }) => {
      calls.push(req);
    },
  };

  const ok = await runCharacterExport({ characterId: 'c1', displayLabel: 'Thrain' }, deps);
  assertEqual(ok.kind, 'Exported', 'happy path reports Exported');
  assertEqual(calls.length, 1, 'the real export wrapper ran exactly once');
  assertEqual(calls[0].characterId, 'c1', 'with the open character');
  assertEqual(calls[0].filePath, '/tmp/out.json', 'to the path the dialog returned');
  assert(ok.kind === 'Exported' && ok.message.includes('/tmp/out.json'), 'status names the file');

  const cancelled = await runCharacterExport({ characterId: 'c1', displayLabel: 'Thrain' }, { ...deps, save: async () => null });
  assertEqual(cancelled.kind, 'Cancelled', 'dialog dismissal is not an error');
  assertEqual(calls.length, 1, 'cancel does not export');

  const noRuntime = await runCharacterExport({ characterId: 'c1', displayLabel: 'Thrain' }, { ...deps, hasRuntime: () => false });
  assertEqual(noRuntime.kind, 'Failed', 'browser preview cannot export');
  assertEqual(calls.length, 1, 'no runtime does not export');

  const thrown = await runCharacterExport(
    { characterId: 'c1', displayLabel: 'Thrain' },
    { ...deps, exportCharacter: async () => { throw new Error('disk full'); } },
  );
  assert(thrown.kind === 'Failed' && thrown.message === 'disk full', 'boundary errors surface verbatim');

  console.log('characterExport tests passed');
}

void main();
