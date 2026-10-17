import { readFileSync } from 'node:fs';
import type { LoadSavedCharacterResponse } from './loadSavedCharacterDetail';
import { saveCyberpunkCriticals } from './cprSheet';
import { assertEqual } from '../testSupport/asserts';

/**
 * SD-38 E4.5: the sheet's critical-injuries checklist saves through `save_cyberpunk_criticals`
 * (`apps/desktop/src-tauri/src/cpr_adapter.rs`). This proves the wire shape the Rust command
 * deserialises (`CprCriticalsSaveRequest`: `{ request: { characterId, criticals: [{ location, name }] } }`)
 * and that the answer is handed back as the reloaded sheet. The save itself, the refusal and the
 * moved Base Death Save Penalty row are proved against the real adapter in Rust.
 */
async function messageOf(run: () => Promise<unknown>): Promise<string> {
  try {
    await run();
  } catch (cause) {
    return cause instanceof Error ? cause.message : String(cause);
  }
  return '';
}

async function main() {
  // No Tauri runtime under `tsx`: the save fails with a descriptive message.
  assertEqual(
    await messageOf(() => saveCyberpunkCriticals('cpr-solo-kitted', [])),
    'Tauri runtime not available for saving the Cyberpunk RED critical injuries',
    'no-runtime failure is descriptive',
  );

  // A Tauri runtime whose IPC records the call and answers with a real load response.
  const answer = JSON.parse(
    readFileSync(new URL('../characterHub/__tests__/cyberpunkSheetFixtures/cpr-solo-kitted.json', import.meta.url), 'utf8'),
  ) as LoadSavedCharacterResponse;
  const calls: Array<{ cmd: string; args: unknown }> = [];
  const globals = globalThis as Record<string, unknown>;
  globals.window = {
    __TAURI_INTERNALS__: {
      invoke: async (cmd: string, args: unknown) => {
        calls.push({ cmd, args });
        if (cmd !== 'save_cyberpunk_criticals') {
          throw new Error(`unexpected command ${cmd}`);
        }
        return answer;
      },
      transformCallback: () => 0,
    },
  };
  try {
    const saved = await saveCyberpunkCriticals('cpr-solo-kitted', [
      { location: 'body', name: 'Collapsed Lung' },
      { location: 'head', name: 'Whiplash' },
    ]);
    assertEqual(calls.length, 1, 'one IPC call');
    assertEqual(calls[0].cmd, 'save_cyberpunk_criticals', 'the command registered in main.rs');
    assertEqual(
      JSON.stringify(calls[0].args),
      JSON.stringify({
        request: {
          characterId: 'cpr-solo-kitted',
          criticals: [
            { location: 'body', name: 'Collapsed Lung' },
            { location: 'head', name: 'Whiplash' },
          ],
        },
      }),
      'the request is the shape CprCriticalsSaveRequest deserialises',
    );
    assertEqual(saved, answer, 'the answer is the reloaded sheet');

    // A refusal from the engine reaches the caller with its reason.
    (globals.window as { __TAURI_INTERNALS__: { invoke: unknown } }).__TAURI_INTERNALS__.invoke = async () => {
      throw 'Paper Cut is not on the Critical Injuries to the Body table (p. 221-222)';
    };
    assertEqual(
      await messageOf(() => saveCyberpunkCriticals('cpr-solo-kitted', [{ location: 'body', name: 'Paper Cut' }])),
      'Failed to save the Cyberpunk RED critical injuries: Paper Cut is not on the Critical Injuries to the Body table (p. 221-222)',
      'the engine refusal is passed on',
    );
  } finally {
    delete globals.window;
  }
  console.log('cprSheet: save_cyberpunk_criticals request shape, answer and refusal checked');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
