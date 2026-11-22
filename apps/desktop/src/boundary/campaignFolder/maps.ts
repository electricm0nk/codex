import { invokeCampaignFolder } from './invokeCampaignFolder';

/** One pin (`src/campaign/maps.rs` `Pin`): fractions of the image's size, a label, a wiki page name. */
export interface MapPin {
  x: number;
  y: number;
  label: string;
  page: string;
}

/** One map as `list_maps` reads it: an image with its notes file, or a notes file with no image yet. */
export interface MapEntry {
  title: string;
  image: string | null;
  notes: string;
  notesExists: boolean;
  pins: MapPin[];
  /** False when the notes file's pins cannot be read: shown, never rewritten. */
  pinsOk: boolean;
}

export function listMaps(campaignDir: string): Promise<MapEntry[]> {
  return invokeCampaignFolder('list_maps', { campaignDir }, 'listing the maps');
}

/** Writes the pins into the map's notes frontmatter (`set_map_pins`); every other byte stays. */
export function setMapPins(campaignDir: string, notes: string, pins: MapPin[]): Promise<void> {
  return invokeCampaignFolder('set_map_pins', { request: { campaignDir, notes, pins } }, 'saving the pins');
}

/** "Add map…": copies the picked image into `Maps/` and creates its notes; resolves to the image's path. */
export function addMap(campaignDir: string, source: string): Promise<string> {
  return invokeCampaignFolder('add_map', { request: { campaignDir, source } }, 'adding the map');
}
