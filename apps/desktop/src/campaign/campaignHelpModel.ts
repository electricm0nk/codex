import type { CampaignTreeEntry } from '../boundary/campaignFolder/types';

/**
 * The campaign help screen's text and folder tree (SD-42 `decisions.md §13.3`, operator Q12, Q13,
 * Q19, Q21/Q22). Pure data so the test can hold the sentences the rulings require.
 */

export const OBSIDIAN_URL = 'https://obsidian.md';

export interface HelpSection {
  id: string;
  title: string;
  paragraphs: string[];
}

export const CAMPAIGN_HELP_SECTIONS: HelpSection[] = [
  {
    id: 'folder',
    title: 'A campaign is a folder',
    paragraphs: [
      'A campaign is a plain folder of files on your computer: campaign.json for its name and members, and Markdown files for party resources, the adventure log, maps and the wiki.',
      'Codex never syncs anything. It reads and writes the folder on this computer only. To play together, share the folder with a sync tool you already use, and point each player\'s Codex at their own copy.',
    ],
  },
  {
    id: 'google-drive',
    title: 'Worked example: Google Drive',
    paragraphs: [
      '1. Install Google Drive for desktop and let it sync a folder on your computer.',
      '2. In Codex, press New campaign… and create the campaign inside that synced Google Drive folder.',
      '3. In Google Drive, share the campaign folder with your players.',
      '4. Each player adds the shared folder to their own Drive, lets it sync, then presses Open folder… in Codex and picks their synced copy.',
    ],
  },
  {
    id: 'any-tool',
    title: 'Any sync tool works',
    paragraphs: [
      'Google Drive is only an example: any sync tool that copies a folder between computers works, and so does a LAN share or a USB stick.',
      'Dropbox: create the campaign inside your Dropbox folder and share that folder from Dropbox.',
      'Syncthing: add the campaign folder as a Syncthing folder and share it with each player\'s device; it syncs directly, with no cloud account.',
    ],
  },
  {
    id: 'structure',
    title: 'What is in the folder',
    paragraphs: [
      'This is the folder structure. With a campaign open it shows that campaign\'s folder as it is right now; otherwise it shows what a new campaign gets.',
    ],
  },
  {
    id: 'editors',
    title: 'Edit the files anywhere',
    paragraphs: [
      'Edit these files in Obsidian or any Markdown editor. Codex reads them as they are and keeps your formatting. Obsidian is free at https://obsidian.md.',
    ],
  },
  {
    id: 'shared',
    title: 'Everything is shared',
    paragraphs: [
      'Everything in this folder is shared: every player who syncs it can read and change every file. Codex has no secrets yet.',
      'GMs: keep secret notes in a separate folder that you do not share.',
    ],
  },
  {
    id: 'characters',
    title: 'Characters, master and stand-in',
    paragraphs: [
      'The Characters folder holds a backup copy of each party member\'s character, so the GM can see the party and other players can cover for someone who is away.',
      'Your own copy on your computer is the master. Every save also updates the backup in Characters. The backup remembers which computer holds the master.',
      'If you open someone\'s backup on another computer, you can play as a stand-in (the default): your changes stay on that computer and never overwrite the backup. Or you can move the master there, and that computer becomes the one that saves the backup.',
      'If the backup was changed by someone else since your last save, Codex does not overwrite it: it tells you who changed it and when, and your own save still goes through.',
    ],
  },
];

function dir(path: string, children: CampaignTreeEntry[] = []): CampaignTreeEntry {
  return { name: path.split('/').pop() ?? path, path, isDir: true, children };
}

function file(path: string): CampaignTreeEntry {
  return { name: path.split('/').pop() ?? path, path, isDir: false, children: [] };
}

/**
 * What "New campaign…" creates (`src/campaign/folder.rs` `LAYOUT_DIRS` + `campaign.json` +
 * `Party Resources.md` + the `.codex/nonce` counter), in the order `list_campaign_tree` sorts.
 * shortcut: a hand-kept mirror of `LAYOUT_DIRS`, pinned by the root test `campaign_folder_help_template_matches_the_layout`; read it from a command if the layout starts changing.
 */
export const TEMPLATE_TREE: CampaignTreeEntry[] = [
  dir('.codex', [file('.codex/nonce')]),
  dir('Adventure Log'),
  dir('Attachments'),
  dir('Characters'),
  dir('Custom Items'),
  dir('Maps'),
  file('Party Resources.md'),
  dir('Wiki', [dir('Wiki/Factions'), dir('Wiki/Items'), dir('Wiki/NPCs'), dir('Wiki/Places')]),
  file('campaign.json'),
];

/** The tree as indented lines (two spaces per level, folders end in `/`); `null` = no campaign open. */
export function helpTreeLines(entries: CampaignTreeEntry[] | null): string[] {
  const tree = entries ?? TEMPLATE_TREE;
  if (tree.length === 0) {
    return ['(this folder is empty)'];
  }
  const lines: string[] = [];
  const walk = (level: CampaignTreeEntry[], depth: number) => {
    for (const entry of level) {
      lines.push(`${'  '.repeat(depth)}${entry.name}${entry.isDir ? '/' : ''}`);
      walk(entry.children, depth + 1);
    }
  };
  walk(tree, 0);
  return lines;
}
