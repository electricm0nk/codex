import { isValidElement, type ReactElement, type ReactNode } from 'react';
import { assert, assertEqual } from '../testSupport/asserts';
import type { CampaignMembershipDto } from '../boundary/setCharacterCampaigns';
import { SheetCampaignBanners } from './CharacterSheet';

// A real membership row: a UUID id (older rows) beside the absolute campaign folder.
const rise: CampaignMembershipDto = {
  campaignId: '7f3c2a10-4b1e-4c8a-9d2f-0a1b2c3d4e5f',
  campaignName: 'Rise of the Runelords',
  campaignDir: '/home/player/Campaigns/Rise of the Runelords',
  active: true,
};
const skull: CampaignMembershipDto = { campaignId: 'b2', campaignName: 'Skull & Shackles', campaignDir: '/home/player/Campaigns/Skull', active: true };
const old: CampaignMembershipDto = { campaignId: 'c3', campaignName: 'Old', campaignDir: '/home/player/Campaigns/Old', active: false };

function collect(node: ReactNode, into: ReactElement<Record<string, unknown>>[] = []) {
  if (Array.isArray(node)) {
    node.forEach((child) => collect(child, into));
  } else if (isValidElement(node)) {
    const element = node as ReactElement<Record<string, unknown>>;
    into.push(element);
    collect(element.props.children as ReactNode, into);
  }
  return into;
}

const clickables = (memberships: CampaignMembershipDto[], onOpen: (dir: string) => void) =>
  collect(SheetCampaignBanners({ memberships, onOpen })).filter((element) => typeof element.props.onClick === 'function');

// Open with a real membership row hands the handler the absolute folder, not the id.
{
  const seen: string[] = [];
  const [open] = clickables([rise], (dir) => seen.push(dir));
  assert(open !== undefined, 'one Open control for the membership');
  (open.props.onClick as () => void)();
  assertEqual(seen[0], rise.campaignDir, 'Open passes the campaign folder');
  assert(seen[0].startsWith('/'), 'the argument is an absolute path');
  console.log('ok - sheet campaign Open passes the absolute campaign folder');
}

// One full-width banner per active membership, stacked; an inactive one gets none.
{
  const seen: string[] = [];
  const opens = clickables([rise, skull, old], (dir) => seen.push(dir));
  assertEqual(opens.length, 2, 'one banner per active membership');
  opens.forEach((open) => (open.props.onClick as () => void)());
  assertEqual(JSON.stringify(seen), JSON.stringify([rise.campaignDir, skull.campaignDir]), 'each banner opens its own folder');
  opens.forEach((open, index) => assertEqual(open.props.title, `Open ${[rise, skull][index].campaignName}`, 'titled with the campaign name'));
  console.log('ok - one banner per active campaign');
}

assert(SheetCampaignBanners({ memberships: [old], onOpen: () => {} }) === null, 'no banners without an active campaign');
console.log('sheetCampaignOpen.test.ts: 3 cases passed');
