import { invokeCampaignFolder } from './invokeCampaignFolder';

/**
 * Copies a dragged-in image into the campaign's `Attachments/` (`save_attachment`); a name
 * already taken gets `-2`, `-3`… Resolves to the path inside the campaign, e.g. `Attachments/map-2.png`.
 */
export function saveAttachment(campaignDir: string, name: string, bytes: Uint8Array): Promise<string> {
  return invokeCampaignFolder(
    'save_attachment',
    { request: { campaignDir, name, bytes: Array.from(bytes) } },
    `saving ${name} to Attachments`
  );
}
