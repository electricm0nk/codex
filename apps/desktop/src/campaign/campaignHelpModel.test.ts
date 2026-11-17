import { CAMPAIGN_HELP_SECTIONS, OBSIDIAN_URL, TEMPLATE_TREE, helpTreeLines } from './campaignHelpModel';
import type { CampaignTreeEntry } from '../boundary/campaignFolder/types';
import { assert, assertEqual } from '../testSupport/asserts';

const allText = CAMPAIGN_HELP_SECTIONS.map((section) => [section.title, ...section.paragraphs].join('\n')).join('\n');

function says(phrase: string): () => void {
  return () => assert(allText.includes(phrase), `the help screen says "${phrase}"`);
}

function dir(name: string, path: string, children: CampaignTreeEntry[] = []): CampaignTreeEntry {
  return { name, path, isDir: true, children };
}

function file(name: string, path: string): CampaignTreeEntry {
  return { name, path, isDir: false, children: [] };
}

const cases: [string, () => void][] = [
  ['the worked example is Google Drive', says('Google Drive')],
  ['any sync tool works', says('any sync tool')],
  ['Dropbox has its own line', says('Dropbox')],
  ['Syncthing has its own line', says('Syncthing')],
  [
    'the Obsidian link is https://obsidian.md',
    () => {
      assertEqual(OBSIDIAN_URL, 'https://obsidian.md', 'the link the button opens');
      says('https://obsidian.md')();
      says('Obsidian or any Markdown editor')();
    },
  ],
  ['everything in the folder is shared (Q19)', says('Everything in this folder is shared')],
  ['the Characters folder, stand-in and master are explained', () => {
    says('Characters')();
    says('stand-in')();
    says('master')();
  }],
  ['Codex never syncs', says('Codex never syncs')],
  [
    'seven sections, in the decisions.md §13.3 order',
    () => {
      assertEqual(CAMPAIGN_HELP_SECTIONS.length, 7, 'section count');
      assertEqual(
        CAMPAIGN_HELP_SECTIONS.map((section) => section.id).join(','),
        'folder,google-drive,any-tool,structure,editors,shared,characters',
        'section order'
      );
    },
  ],
  [
    'no open campaign: the template tree (the layout a new campaign gets)',
    () => {
      const lines = helpTreeLines(null);
      for (const expected of [
        'campaign.json',
        'Party Resources.md',
        'Adventure Log/',
        'Maps/',
        'Wiki/',
        '  NPCs/',
        '  Places/',
        '  Factions/',
        '  Items/',
        'Characters/',
        'Attachments/',
      ]) {
        assert(lines.includes(expected), `template line "${expected}" in ${JSON.stringify(lines)}`);
      }
      assertEqual(helpTreeLines(null).join('\n'), helpTreeLines(TEMPLATE_TREE).join('\n'), 'null renders the template');
    },
  ],
  [
    'an open campaign: the live tree, nested, as the folder holds it',
    () => {
      const live = [
        file('Party Resources.md', 'Party Resources.md'),
        dir('Wiki', 'Wiki', [dir('Gods', 'Wiki/Gods', [file('Desna.md', 'Wiki/Gods/Desna.md')])]),
      ];
      assertEqual(helpTreeLines(live).join('|'), 'Party Resources.md|Wiki/|  Gods/|    Desna.md', 'live lines');
      assertEqual(helpTreeLines([]).join('|'), '(this folder is empty)', 'an empty folder says so');
    },
  ],
];

for (const [name, run] of cases) {
  run();
  console.log(`ok - ${name}`);
}
console.log(`campaignHelpModel.test.ts: ${cases.length} cases passed`);
