/**
 * Live hook into Obsidian's real community theme catalog — the same JSON
 * index Obsidian itself reads to populate its "Community themes" browser.
 * We do not maintain our own theme list; this always reflects the current
 * public catalog.
 */

export interface ObsidianThemeCatalogEntry {
  name: string;
  author: string;
  /** `"owner/repo"` on GitHub. */
  repo: string;
  /** Screenshot filename relative to the repo root. */
  screenshot: string;
  modes: Array<'dark' | 'light'>;
  legacy?: boolean;
}

const CATALOG_URL = 'https://raw.githubusercontent.com/obsidianmd/obsidian-releases/HEAD/community-css-themes.json';

let cached: ObsidianThemeCatalogEntry[] | null = null;
let inflight: Promise<ObsidianThemeCatalogEntry[]> | null = null;

/** Fetches (and memoizes for the session) the full Obsidian community theme catalog. */
export async function loadObsidianThemeCatalog(): Promise<ObsidianThemeCatalogEntry[]> {
  if (cached) {
    return cached;
  }
  if (inflight) {
    return inflight;
  }
  inflight = fetch(CATALOG_URL)
    .then((response) => {
      if (!response.ok) {
        throw new Error(`Failed to load the Obsidian community theme catalog (HTTP ${response.status}).`);
      }
      return response.json() as Promise<ObsidianThemeCatalogEntry[]>;
    })
    .then((entries) => {
      cached = entries;
      return entries;
    })
    .finally(() => {
      inflight = null;
    });
  return inflight;
}

/**
 * `HEAD` reliably resolves to a GitHub repo's default branch on
 * raw.githubusercontent.com, so callers don't need a separate GitHub API
 * lookup (and its much lower unauthenticated rate limit) just to find it.
 */
export function rawFileUrl(repo: string, path: string, ref = 'HEAD'): string {
  return `https://raw.githubusercontent.com/${repo}/${ref}/${path}`;
}
