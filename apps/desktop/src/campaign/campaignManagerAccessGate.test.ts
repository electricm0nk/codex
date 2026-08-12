import { computeCampaignManagerAccessGate } from './campaignManagerAccessGate';
import { assert, assertEqual } from '../testSupport/asserts';

const configured = computeCampaignManagerAccessGate(true);
assertEqual(configured.enabled, true, 'enabled once a local folder is configured');
assertEqual(configured.disabledHint, '', 'no hint needed once enabled');

const notConfigured = computeCampaignManagerAccessGate(false);
assertEqual(notConfigured.enabled, false, 'disabled until a local folder is configured');
assert(
  !/oauth|authorize|connect.*account/i.test(notConfigured.disabledHint),
  'E2.11: hint must not imply an OAuth/account-connection flow (OAuth is descoped for SD-21)'
);
assert(
  /local .*folder/i.test(notConfigured.disabledHint),
  'E2.11: hint should name the local-folder picker directly'
);

console.log('campaignManagerAccessGate.test.ts OK');
