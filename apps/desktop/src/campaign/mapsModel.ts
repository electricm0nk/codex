import type { CampaignTreeEntry } from '../boundary/campaignFolder/types';

/** The Maps tab's pure rules (SD-42 `decisions.md §15.3`); Rust reads and writes the pins. */

export const WIKI_DIR = 'Wiki';

/** Where a click landed, as fractions of the image's box (`§21` Q-E), clamped and rounded to 4 places. */
export function clickFraction(clientX: number, clientY: number, rect: { left: number; top: number; width: number; height: number }): { x: number; y: number } {
  const part = (offset: number, size: number) => Math.round(Math.min(1, Math.max(0, size > 0 ? offset / size : 0)) * 10000) / 10000;
  return { x: part(clientX - rect.left, rect.width), y: part(clientY - rect.top, rect.height) };
}

export type PinTarget = { kind: 'open'; path: string } | { kind: 'missing'; name: string } | { kind: 'none' };

/** The page name a pin or link names: `Page|alias`, `Page#heading` → `Page`. */
export function pageName(page: string): string {
  return page.split(/[|#]/)[0].trim();
}

/**
 * What a pin's page opens: the file `resolve_wiki_link` found (`resolved`, title first, then
 * filename stem), else the create offer (SD-p; "+" writes it under `Wiki/`), or nothing for no page.
 */
export function pinTarget(page: string, resolved: string | null): PinTarget {
  const name = pageName(page);
  if (!name) return { kind: 'none' };
  return resolved ? { kind: 'open', path: resolved } : { kind: 'missing', name };
}

/** The Markdown pages under `Wiki/` by name, for the "Link to wiki page" list. */
export function wikiPages(tree: readonly CampaignTreeEntry[]): string[] {
  const walk = (entries: readonly CampaignTreeEntry[]): string[] =>
    entries.flatMap((entry) => (entry.isDir ? walk(entry.children) : /\.md$/i.test(entry.name) ? [entry.name.replace(/\.md$/i, '')] : []));
  const wiki = tree.find((entry) => entry.isDir && entry.path === WIKI_DIR);
  return wiki ? walk(wiki.children).sort((a, b) => a.localeCompare(b)) : [];
}
