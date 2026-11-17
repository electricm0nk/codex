import type { CampaignTreeEntry } from '../../boundary/campaignFolder/types';

/** The tab's file (`src/campaign/folder.rs` `PARTY_RESOURCES_FILE`). Codex never parses it (Q14). */
export const PARTY_RESOURCES_PATH = 'Party Resources.md';

function markdownFiles(tree: readonly CampaignTreeEntry[]): string[] {
  return tree.flatMap((entry) => (entry.isDir ? markdownFiles(entry.children) : entry.path.toLowerCase().endsWith('.md') ? [entry.path] : []));
}

const stem = (path: string) => path.split('/').pop()!.replace(/\.md$/i, '');

/** Every Markdown filename stem in the folder, for `[[` completion. */
export function markdownStems(tree: readonly CampaignTreeEntry[]): string[] {
  return markdownFiles(tree).map(stem);
}

// shortcut: filename stems only; E2.9 adds frontmatter titles and Obsidian's full resolution rule.
/** The Markdown file a `[[link]]` names by its stem (case-insensitive), or null. */
export function linkTarget(tree: readonly CampaignTreeEntry[], target: string): string | null {
  const wanted = target.trim().toLowerCase();
  return markdownFiles(tree).find((path) => stem(path).toLowerCase() === wanted) ?? null;
}
