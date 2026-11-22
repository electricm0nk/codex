import { loadReportAvailability, sendReport } from './sendReport';
import { assert } from '../testSupport/asserts';

/** No Tauri runtime under `tsx`: both calls fail loudly, never a fake "sent". */
async function testNoRuntimeNeverReportsSuccess() {
  for (const [label, call] of [
    ['availability', () => loadReportAvailability()],
    ['send', () => sendReport({ kind: 'bug', title: 't', body: 'b' })],
  ] as const) {
    let thrown: unknown;
    try {
      await call();
    } catch (cause) {
      thrown = cause;
    }
    assert(thrown instanceof Error && thrown.message.includes('Tauri runtime not available'), `${label}: no-runtime failure is descriptive`);
  }
}

testNoRuntimeNeverReportsSuccess().catch((error: unknown) => {
  console.error(error);
  throw error;
});
