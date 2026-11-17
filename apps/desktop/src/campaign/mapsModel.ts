import type { CampaignTreeEntry } from '../boundary/campaignFolder/types';
import { linkTarget } from './tabs/partyResourcesModel';

/** The Maps tab's pure rules (SD-42 `decisions.md §15.3`); Rust reads and writes the pins. */

export const WIKI_DIR = 'Wiki';

/** Where a click landed, as fractions of the image's box (`§21` Q-E), clamped and rounded to 4 places. */
export function clickFraction(clientX: number, clientY: number, rect: { left: number; top: number; width: number; height: number }): { x: number; y: number } {
  const part = (offset: number, size: number) => Math.round(Math.min(1, Math.max(0, size > 0 ? offset / size : 0)) * 10000) / 10000;
  return { x: part(clientX - rect.left, rect.width), y: part(clientY - rect.top, rect.height) };
}

export type PinTarget = { kind: 'open'; path: string } | { kind: 'missing'; name: string; createPath: string } | { kind: 'none' };

/** The file a pin's page names (`[[Page|alias]]`, `[[Page#heading]]` resolve to `Page`), or the create offer (SD-p). */
export function pinTarget(tree: readonly CampaignTreeEntry[], page: string): PinTarget {
  const name = page.split(/[|#]/)[0].trim();
  if (!name) return { kind: 'none' };
  const path = linkTarget(tree, name);
  return path ? { kind: 'open', path } : { kind: 'missing', name, createPath: `${WIKI_DIR}/${name}.md` };
}

// shortcut: the page "Create" writes has no `type` (no folder picked); E2.9's "+" owns the per-folder page.
/** A new wiki page: `§15.4`'s small frontmatter and a heading. */
export function newPageText(name: string): string {
  return `---\ntitle: ${JSON.stringify(name)}\ntags: []\n---\n# ${name}\n`;
}

/** The Markdown pages under `Wiki/` by name, for the "Link to wiki page" list. */
export function wikiPages(tree: readonly CampaignTreeEntry[]): string[] {
  const walk = (entries: readonly CampaignTreeEntry[]): string[] =>
    entries.flatMap((entry) => (entry.isDir ? walk(entry.children) : /\.md$/i.test(entry.name) ? [entry.name.replace(/\.md$/i, '')] : []));
  const wiki = tree.find((entry) => entry.isDir && entry.path === WIKI_DIR);
  return wiki ? walk(wiki.children).sort((a, b) => a.localeCompare(b)) : [];
}
