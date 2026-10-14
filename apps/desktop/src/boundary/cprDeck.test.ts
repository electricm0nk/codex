import { computeCyberpunkDeck } from './cprDeck';
import { loadCyberpunkCatalog } from './loadCyberpunkCatalog';
import { assertEqual } from '../testSupport/asserts';

/** No Tauri runtime under `tsx`: both loaders fail with a descriptive message. */
async function messageOf(run: () => Promise<unknown>): Promise<string> {
  try {
    await run();
  } catch (cause) {
    return cause instanceof Error ? cause.message : String(cause);
  }
  return '';
}

async function main() {
  assertEqual(
    await messageOf(() => loadCyberpunkCatalog('program')),
    'Tauri runtime not available for loading the Cyberpunk RED catalog',
    'catalog no-runtime failure is descriptive',
  );
  assertEqual(
    await messageOf(() =>
      computeCyberpunkDeck({
        deck: 'Cyberdeck (Standard Quality)',
        programs: [],
        hardware: [],
        extras: { bodyweightSuit: false, cyberarmDeck: false },
      }),
    ),
    'Tauri runtime not available for computing the Cyberpunk RED deck',
    'deck no-runtime failure is descriptive',
  );
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
