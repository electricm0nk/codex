import { assert } from '../testSupport/asserts';
import { restoreStarterCharactersMessage, starterSeedErrorsMessage } from './starterCharactersModel';

function verifiesRestoredNamesAreListed() {
  const message = restoreStarterCharactersMessage({ restored: ['Aldric Ironhand'], errors: [] });
  assert(message === 'Restored: Aldric Ironhand', `one name, got "${message}"`);
  const two = restoreStarterCharactersMessage({ restored: ['Aldric Ironhand', 'Brother Tomas Vane'], errors: [] });
  assert(two === 'Restored: Aldric Ironhand, Brother Tomas Vane', `two names, got "${two}"`);
}

function verifiesNothingToRestoreSaysSo() {
  const message = restoreStarterCharactersMessage({ restored: [], errors: [] });
  assert(message === 'All starter characters are already here.', `nothing restored, got "${message}"`);
}

function verifiesAFailedSeedIsNeverReportedAsSuccess() {
  const failedOnly = restoreStarterCharactersMessage({ restored: [], errors: ["starter character 'Aldric Ironhand' did not compute"] });
  assert(failedOnly.startsWith('Could not restore:'), `a failure is not success, got "${failedOnly}"`);
  assert(!failedOnly.includes('already here'), 'a failure never says everything is already here');
  assert(failedOnly.includes("'Aldric Ironhand' did not compute"), 'the real error is shown');

  const mixed = restoreStarterCharactersMessage({ restored: ['Elowen Ashgrave'], errors: ['boom'] });
  assert(mixed === 'Restored: Elowen Ashgrave. Could not restore: boom', `partial restore, got "${mixed}"`);
}

function verifiesLaunchSeedErrorsAreShown() {
  assert(starterSeedErrorsMessage([]) === null, 'no errors, no message');
  const message = starterSeedErrorsMessage(["starter character 'Aldric Ironhand' did not compute (blocking: [])"]);
  assert(
    message === "A starter character could not be created: starter character 'Aldric Ironhand' did not compute (blocking: [])",
    `launch error, got "${message}"`,
  );
}

verifiesRestoredNamesAreListed();
verifiesNothingToRestoreSaysSo();
verifiesAFailedSeedIsNeverReportedAsSuccess();
verifiesLaunchSeedErrorsAreShown();
console.log('starterCharactersModel.test.ts: all assertions passed');
