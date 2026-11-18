import type { CampaignTreeEntry } from '../boundary/campaignFolder/types';
import type { WikiPage } from '../boundary/campaignFolder/wiki';
import { markdownStems } from './tabs/partyResourcesModel';
import { WIKI_DIR } from './mapsModel';

/** The Wiki tab's pure rules (SD-42 `decisions.md §15.4`); Rust reads, writes and resolves. */

export type WikiRow =
  | { kind: 'folder'; path: string; name: string; depth: number; pages: number }
  | { kind: 'page'; path: string; title: string; depth: number };

const parentOf = (path: string) => path.slice(0, path.lastIndexOf('/'));
const depthOf = (path: string) => path.split('/').length - 1;

/** `Wiki` and every folder under it (sorted, as `list_wiki_folders` returns them), each followed by its own pages. */
export function wikiRows(folders: readonly string[], pages: readonly WikiPage[]): WikiRow[] {
  return [WIKI_DIR, ...folders].flatMap((folder): WikiRow[] => {
    const own = pages.filter((page) => parentOf(page.path) === folder);
    const name = folder.split('/').pop()!;
    return [
      { kind: 'folder', path: folder, name, depth: depthOf(folder), pages: own.length },
      ...own.map((page): WikiRow => ({ kind: 'page', path: page.path, title: page.title, depth: depthOf(folder) + 1 })),
    ];
  });
}

/** `[[` completion: every wiki page title and every Markdown filename stem in the folder, once each. */
export function completionNames(pages: readonly WikiPage[], tree: readonly CampaignTreeEntry[]): string[] {
  return [...new Set([...pages.map((page) => page.title), ...markdownStems(tree)])].sort((a, b) => a.localeCompare(b));
}

/** What the screen says when `delete_wiki_folder` refused a folder that holds files. */
export function deleteRefusal(folder: string, files: readonly string[]): string {
  const count = `${files.length} file${files.length === 1 ? '' : 's'}`;
  return `${folder} was not deleted: it holds ${count} (${files.join(', ')}). Move or delete them first; Codex deletes only an empty folder.`;
}
