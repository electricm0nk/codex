import { invokeCampaignFolder } from './invokeCampaignFolder';

/** One page under `Wiki/` (`src/campaign/wiki.rs` `Page`). */
export interface WikiPage {
  path: string;
  /** The frontmatter's `title`, else the filename stem. */
  title: string;
  type: string | null;
  tags: string[];
  /** SD-o: the file has a frontmatter block Codex cannot read (listed by filename, never rewritten). */
  frontmatterUnread: boolean;
}

/** One search result (`src/campaign/wiki.rs` `Hit`): title hits come first. */
export interface WikiSearchHit {
  path: string;
  title: string;
  titleMatch: boolean;
  snippet: string;
}

export function listWikiFolders(campaignDir: string): Promise<string[]> {
  return invokeCampaignFolder('list_wiki_folders', { campaignDir }, 'listing the wiki folders');
}

export function listWikiPages(campaignDir: string): Promise<WikiPage[]> {
  return invokeCampaignFolder('list_wiki_pages', { campaignDir }, 'listing the wiki pages');
}

/** Makes `<parent>/<name>`; resolves to its path. */
export function addWikiFolder(campaignDir: string, parent: string, name: string): Promise<string> {
  return invokeCampaignFolder('add_wiki_folder', { request: { campaignDir, path: parent, name } }, 'adding the folder');
}

/** Renames a folder in place; resolves to its new path. */
export function renameWikiFolder(campaignDir: string, folder: string, name: string): Promise<string> {
  return invokeCampaignFolder('rename_wiki_folder', { request: { campaignDir, path: folder, name } }, 'renaming the folder');
}

/** Deletes an empty folder; resolves to `[]`, or to the files it holds (nothing deleted). */
export function deleteWikiFolder(campaignDir: string, folder: string): Promise<string[]> {
  return invokeCampaignFolder('delete_wiki_folder', { request: { campaignDir, path: folder } }, 'deleting the folder');
}

/** "+": writes `<folder>/<name>.md` with the small frontmatter; resolves to its path. */
export function newWikiPage(campaignDir: string, folder: string, name: string): Promise<string> {
  return invokeCampaignFolder('new_wiki_page', { request: { campaignDir, path: folder, name } }, 'creating the page');
}

/** The file a `[[link]]` opens (title first, then filename stem), or null. */
export function resolveWikiLink(campaignDir: string, link: string): Promise<string | null> {
  return invokeCampaignFolder('resolve_wiki_link', { request: { campaignDir, path: link } }, 'following the link');
}

/** Every other Markdown file whose `[[links]]` reach `page`. */
export function wikiBacklinks(campaignDir: string, page: string): Promise<string[]> {
  return invokeCampaignFolder('wiki_backlinks', { request: { campaignDir, path: page } }, 'reading the backlinks');
}

/** Title hits, then text hits, over every `.md` in the campaign. */
export function searchCampaign(campaignDir: string, query: string): Promise<WikiSearchHit[]> {
  return invokeCampaignFolder('search_campaign', { request: { campaignDir, query } }, 'searching the campaign');
}
