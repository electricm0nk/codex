import { dmConsoleFileName, runDmConsoleExport } from './dmConsoleExportFlow';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * v0.8 D-5: the export flow with dialog and boundary injected, so the test
 * proves the real command is called with the chosen path and the rendered
 * HTML — the same shape `characterExport.test.ts` pins for characters.
 */
assertEqual(dmConsoleFileName('Night City: Act 2!'), 'Night City Act 2 console.html', 'filesystem-safe name');
assertEqual(dmConsoleFileName('***'), 'campaign console.html', 'fallback when nothing safe remains');

async function main() {
  const calls: Array<{ filePath: string; html: string }> = [];
  const deps = {
    hasRuntime: () => true,
    save: async () => '/tmp/console.html',
    exportDmConsole: async (req: { filePath: string; html: string }) => {
      calls.push(req);
    },
  };
  const ok = await runDmConsoleExport({ campaignName: 'Night City', html: '<!doctype html>x' }, deps);
  assertEqual(ok.kind, 'Exported', 'happy path');
  assertEqual(calls.length, 1, 'the real command ran once');
  assertEqual(calls[0].filePath, '/tmp/console.html', 'to the chosen path');
  assertEqual(calls[0].html, '<!doctype html>x', 'with the rendered html verbatim');
  assert(ok.kind === 'Exported' && ok.message.includes('/tmp/console.html'), 'status names the file');

  assertEqual((await runDmConsoleExport({ campaignName: 'x', html: 'y' }, { ...deps, save: async () => null })).kind, 'Cancelled', 'dismissal is not an error');
  assertEqual(calls.length, 1, 'cancel does not write');
  assertEqual((await runDmConsoleExport({ campaignName: 'x', html: 'y' }, { ...deps, hasRuntime: () => false })).kind, 'Failed', 'browser preview cannot export');
  const thrown = await runDmConsoleExport({ campaignName: 'x', html: 'y' }, { ...deps, exportDmConsole: async () => { throw new Error('disk full'); } });
  assert(thrown.kind === 'Failed' && thrown.message === 'disk full', 'errors surface verbatim');
  console.log('dmConsoleExportFlow tests passed');
}
void main();
