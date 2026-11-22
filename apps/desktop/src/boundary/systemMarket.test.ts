import { buySystemMarket, listSystemMarket, loadSystemMarketState, sellSystemMarket } from './systemMarket';
import { assertEqual } from '../testSupport/asserts';

/**
 * No Tauri runtime is available under `tsx`, so this exercises the no-runtime failure path every
 * boundary's test exercises. The lists, the prices and the money are proved in `sf_adapter.rs` and
 * `cpr_adapter.rs` (`market_shell`).
 */
async function failure(run: () => Promise<unknown>): Promise<string> {
  try {
    await run();
  } catch (cause) {
    return cause instanceof Error ? cause.message : String(cause);
  }
  return 'did not throw';
}

async function main() {
  const request = { characterId: 'c', key: 'k', priceMode: 'standard' as const, savedAt: 't' };
  for (const system of ['starfinder', 'cyberpunk'] as const) {
    const expected = `Tauri runtime not available for the ${system} Market`;
    assertEqual(await failure(() => listSystemMarket(system, 'weapon')), expected, 'list');
    assertEqual(await failure(() => loadSystemMarketState(system, 'c')), expected, 'state');
    assertEqual(await failure(() => buySystemMarket(system, request)), expected, 'buy');
    assertEqual(await failure(() => sellSystemMarket(system, request)), expected, 'sell');
  }
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
