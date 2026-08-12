import { adjustCharacterMoney, gpToCopper, loadCharacterMoney } from './characterMoney';
import { assertEqual } from '../testSupport/asserts';

/**
 * risks-and-open-questions.md item 25: `gpToCopper` is real conversion
 * logic (the frontend's own gp-input round trip into the wire's canonical
 * copper delta), zero coverage before this.
 */
function verifiesGpToCopperConvertsAtTheStandardRatio() {
  assertEqual(gpToCopper(1), 100, '1 gp is 100 copper');
  assertEqual(gpToCopper(0), 0, '0 gp is 0 copper');
  assertEqual(gpToCopper(2.5), 250, 'a fractional gp amount converts exactly when it lands on a whole copper');
}

function verifiesGpToCopperRoundsAFractionalCopperResult() {
  assertEqual(gpToCopper(1.234), 123, 'a gp amount that would produce a fractional copper value rounds to the nearest whole copper');
}

/**
 * No Tauri runtime is available under `tsx`. `loadCharacterMoney` resolves
 * to a zero balance rather than throwing (matching the Rust command's own
 * "no money.json saved yet" default), the one behavior specific to this
 * file vs. every other boundary loader — see `characterBio.test.ts` for
 * the same asymmetric shape.
 */
async function testLoadWithNoRuntimeResolvesToZeroBalance() {
  const money = await loadCharacterMoney('char-test');
  assertEqual(
    JSON.stringify(money),
    JSON.stringify({ totalCopper: 0, platinum: 0, gold: 0, silver: 0, copper: 0 }),
    'loadCharacterMoney outside a Tauri runtime resolves to a zero balance, not an error'
  );
}

async function testAdjustThrowsDescriptiveErrorWithNoRuntime() {
  let thrown: unknown;
  try {
    await adjustCharacterMoney('char-test', 500);
  } catch (cause) {
    thrown = cause;
  }
  const message = thrown instanceof Error ? thrown.message : String(thrown);
  assertEqual(
    message,
    'Tauri runtime not available for adjusting character money',
    'no-runtime failure is descriptive'
  );
}

async function main() {
  verifiesGpToCopperConvertsAtTheStandardRatio();
  verifiesGpToCopperRoundsAFractionalCopperResult();
  await testLoadWithNoRuntimeResolvesToZeroBalance();
  await testAdjustThrowsDescriptiveErrorWithNoRuntime();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
