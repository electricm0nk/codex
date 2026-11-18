import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert } from '../testSupport/asserts';
import type { OpenedCampaign } from '../boundary/campaignFolder/types';
import { CAMPAIGN_MANAGER_ART } from '../characterHub/LandingScreen';
import { OPEN_CAMPAIGN_HINT, OpenCampaignBanner } from './CampaignManagerScreen';

const opened: OpenedCampaign = {
  campaignDir: '/tmp/campaigns/Rise',
  campaign: {
    schemaVersion: 2,
    name: 'Rise of the Runelords',
    ruleSetId: 'pathfinder-1e',
    ruleSetLabel: 'Pathfinder 1e',
    description: '',
    members: [],
    createdAt: '2026-11-18T00:00:00Z',
    updatedAt: '2026-11-18T00:00:00Z',
  },
  nonce: 1,
};

const banner = (value: OpenedCampaign | null) =>
  renderToStaticMarkup(createElement(OpenCampaignBanner, { opened: value, onOpen: () => {} }));

const cases: Array<[string, () => void]> = [
  [
    "open button carries the main screen's art",
    () => {
      const html = banner(opened);
      assert(html.includes('class="landing-banner"'), 'the landing screen ActionBanner');
      assert(html.includes(`background-image:${CAMPAIGN_MANAGER_ART}`), 'the Campaign Manager art');
      assert(html.includes('Open Rise of the Runelords'), 'titled with the campaign name');
      assert(!html.includes('disabled=""'), 'enabled with a selection');
    },
  ],
  [
    'open button spans the column',
    () => {
      assert(banner(opened).includes('width:100%'), 'width: 100%');
    },
  ],
  [
    'open button is disabled with the hint when nothing is selected',
    () => {
      const html = banner(null);
      assert(html.includes('disabled=""'), 'disabled');
      assert(html.includes(OPEN_CAMPAIGN_HINT), 'the hint');
    },
  ],
];

for (const [name, run] of cases) {
  run();
  console.log(`ok - ${name}`);
}
console.log(`campaignManagerScreen.test.ts: ${cases.length} cases passed`);
