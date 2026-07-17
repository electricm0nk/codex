import { rawFileUrl, type ObsidianThemeCatalogEntry } from './obsidianThemeCatalog';

/**
 * Install / apply / persist real Obsidian community themes.
 *
 * Installing follows the same steps Obsidian itself takes: resolve the
 * theme repo's default branch via the GitHub API, then fetch `theme.css`
 * from the root of that branch. The fetched CSS is the theme author's
 * actual stylesheet — nothing is reinterpreted.
 *
 * Applying works by injecting that CSS verbatim into the document (Obsidian
 * community themes scope their variables under `.theme-dark` / `.theme-light`
 * on `<body>`, so we mirror that convention) plus a small bridge stylesheet
 * that maps our own `--color-*` design tokens onto Obsidian's variable names.
 * Any theme that defines the standard Obsidian variables therefore "just
 * works" without a per-theme mapping.
 *
 * Persistence is localStorage today (installed theme CSS + the active
 * selection). This is enough to survive reloads; moving it to real file
 * storage is future backend work, same as other not-yet-persisted settings.
 */

export interface InstalledTheme {
  /** The catalog repo id, e.g. `"kepano/obsidian-minimal"` — stable and unique. */
  id: string;
  name: string;
  author: string;
  repo: string;
  modes: Array<'dark' | 'light'>;
  css: string;
  installedAt: string;
}

const INSTALLED_KEY = 'codex.theme.community.installed';
const ACTIVE_KEY = 'codex.theme.community.active';

export function getInstalledThemes(): InstalledTheme[] {
  try {
    const raw = localStorage.getItem(INSTALLED_KEY);
    return raw ? (JSON.parse(raw) as InstalledTheme[]) : [];
  } catch {
    return [];
  }
}

function saveInstalledThemes(themes: InstalledTheme[]): void {
  try {
    localStorage.setItem(INSTALLED_KEY, JSON.stringify(themes));
  } catch {
    /* non-persistent environments (e.g. private mode) still get the theme applied this session */
  }
}

// `HEAD` resolves the default branch for the overwhelming majority of repos;
// `main`/`master` are a defensive fallback for the rare case it doesn't.
const BRANCH_FALLBACKS = ['HEAD', 'main', 'master'];

async function fetchThemeCss(repo: string): Promise<string> {
  let lastStatus = 0;
  for (const ref of BRANCH_FALLBACKS) {
    const response = await fetch(rawFileUrl(repo, 'theme.css', ref));
    if (response.ok) {
      return response.text();
    }
    lastStatus = response.status;
  }
  throw new Error(`No theme.css found at the root of ${repo} (last HTTP ${lastStatus}). The repo may have moved or renamed the file.`);
}

export async function installTheme(entry: ObsidianThemeCatalogEntry): Promise<InstalledTheme> {
  const css = await fetchThemeCss(entry.repo);

  const installed: InstalledTheme = {
    id: entry.repo,
    name: entry.name,
    author: entry.author,
    repo: entry.repo,
    modes: entry.modes,
    css,
    installedAt: new Date().toISOString(),
  };

  saveInstalledThemes([...getInstalledThemes().filter((theme) => theme.id !== installed.id), installed]);
  return installed;
}

export function uninstallTheme(id: string): void {
  saveInstalledThemes(getInstalledThemes().filter((theme) => theme.id !== id));
  if (getActiveThemeId() === id) {
    setActiveThemeId(null);
  }
}

export function getActiveThemeId(): string | null {
  try {
    return localStorage.getItem(ACTIVE_KEY);
  } catch {
    return null;
  }
}

/** Selects the active theme (or `null` to revert to the built-in Wasp palette) and applies it immediately. */
export function setActiveThemeId(id: string | null): void {
  try {
    if (id) {
      localStorage.setItem(ACTIVE_KEY, id);
    } else {
      localStorage.removeItem(ACTIVE_KEY);
    }
  } catch {
    /* still applies for this session even if it can't persist */
  }
  applyActiveTheme();
}

const STYLE_TAG_ID = 'codex-community-theme-css';
const BRIDGE_TAG_ID = 'codex-community-theme-bridge';

// `body.theme-dark`/`body.theme-light` (element + class) always outrank our
// `:root` token definitions (pseudo-class only) in CSS specificity, so this
// bridge safely overrides the built-in Wasp tokens only while a community
// theme is actually active and defines the corresponding Obsidian variable.
//
// The fallback in each `var(..., fallback)` MUST be `--wasp-color-*`, not
// `--color-*` itself: `--color-accent: var(--interactive-accent, var(--color-accent))`
// looks like a reasonable "fall back to our own value" but is actually a
// self-referencing custom property. Per the CSS spec a cyclic custom-property
// reference resolves to the guaranteed-invalid value (silently empty) rather
// than the inherited value — this shipped once and silently voided every
// `--color-*` token app-wide, making every themed surface render fully
// transparent. `--wasp-color-*` (theme.css) is a separate token namespace
// nothing ever overrides, so referencing it here carries no such risk.
const BRIDGE_CSS = `
body.theme-dark, body.theme-light {
  --color-page: var(--background-secondary, var(--wasp-color-page));
  --color-surface: var(--background-primary, var(--wasp-color-surface));
  --color-surface-2: var(--background-modifier-form-field, var(--wasp-color-surface-2));
  --color-border: var(--background-modifier-border, var(--wasp-color-border));
  --color-border-strong: var(--interactive-accent, var(--wasp-color-border-strong));
  --color-accent: var(--interactive-accent, var(--wasp-color-accent));
  --color-accent-hover: var(--interactive-accent-hover, var(--wasp-color-accent-hover));
  --color-on-accent: var(--text-on-accent, var(--wasp-color-on-accent));
  --color-text: var(--text-normal, var(--wasp-color-text));
  --color-text-secondary: var(--text-muted, var(--wasp-color-text-secondary));
  --color-text-muted: var(--text-faint, var(--wasp-color-text-muted));
  --color-text-faint: var(--text-faint, var(--wasp-color-text-faint));
  --color-link: var(--text-accent, var(--wasp-color-link));
}
`;

/** Re-injects (or removes) the active community theme's CSS. Call on boot and whenever the selection changes. */
export function applyActiveTheme(): void {
  const activeId = getActiveThemeId();
  const theme = activeId ? getInstalledThemes().find((entry) => entry.id === activeId) : null;

  const existingStyle = document.getElementById(STYLE_TAG_ID);
  const existingBridge = document.getElementById(BRIDGE_TAG_ID);

  if (!theme) {
    existingStyle?.remove();
    existingBridge?.remove();
    return;
  }

  const styleTag = existingStyle ?? document.createElement('style');
  styleTag.id = STYLE_TAG_ID;
  styleTag.textContent = theme.css;
  if (!existingStyle) {
    document.head.appendChild(styleTag);
  }

  const bridgeTag = existingBridge ?? document.createElement('style');
  bridgeTag.id = BRIDGE_TAG_ID;
  bridgeTag.textContent = BRIDGE_CSS;
  if (!existingBridge) {
    document.head.appendChild(bridgeTag);
  }
}

/**
 * Obsidian community CSS keys off `body.theme-dark` / `body.theme-light`.
 * Keep these in sync with our own dark/light mode regardless of whether a
 * community theme is active, so switching modes always resolves correctly.
 */
export function applyObsidianModeClass(mode: 'dark' | 'light'): void {
  if (typeof document === 'undefined') {
    return;
  }
  document.body.classList.toggle('theme-dark', mode === 'dark');
  document.body.classList.toggle('theme-light', mode === 'light');
}
