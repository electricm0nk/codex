import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Desktop boundary over the Starfinder 1e catalogs (SD-37 E6.4):
 * `list_starfinder_catalog` (`apps/desktop/src-tauri/src/sf_catalog.rs`), which
 * serves the races, themes, classes, feats, spells and equipment of the
 * converted Starfinder package (`data/starfinder-1e/sheet_rules`). Every row,
 * name, description and total is the engine's answer; this file mirrors the
 * Rust DTOs field for field (`serde(rename_all = "camelCase")`).
 */

export type StarfinderCatalogKind = 'race' | 'theme' | 'class' | 'feat' | 'spell' | 'equipment';

export interface StarfinderCatalogRowDto {
  id: string;
  label: string;
  /** The row's value, the total it adds to and its condition, as the engine words it. */
  summary: string;
}

export interface StarfinderCatalogEntryDto {
  /** The package rule id (`core:feat:adaptive_fighting`). */
  id: string;
  name: string;
  /** The package's book id (`core`, `armory`, ...). */
  book: string;
  tags: string[];
  description: string | null;
  /** `prose` (the record's own words), `statBlock` or `fields`; null with no description. */
  descriptionTier: string | null;
  /** The record's rows that feed a sheet total. */
  rows: StarfinderCatalogRowDto[];
}

export interface StarfinderCatalogResponse {
  kind: StarfinderCatalogKind;
  /** The package root the rows were read from (repo-relative). */
  source: string;
  entries: StarfinderCatalogEntryDto[];
}

export async function loadStarfinderCatalog(kind: StarfinderCatalogKind): Promise<StarfinderCatalogResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading the Starfinder catalog');
  }

  try {
    return await invoke<StarfinderCatalogResponse>('list_starfinder_catalog', { kind });
  } catch (cause: unknown) {
    throw new Error(`Failed to load the Starfinder catalog: ${formatError(cause)}`);
  }
}
