import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over the engine's per-class spell levels.
 *
 * Invokes the `list_class_spell_levels` Tauri command
 * (`src-tauri/src/class_spell_levels.rs`), which answers, for each
 * requested `class:<id>`, the real spell level of every spell on that
 * class's list.
 *
 * **Why a second command alongside `list_spells`.** A spell catalog
 * record's own `level` is the MINIMUM across every class named in its
 * corpus `CLASSES:` tag, not the level for any particular class.
 * `Hideous Laughter` is `CLASSES:Bard=1|Sorcerer,Wizard=2`, so the catalog
 * serves 1 and a Wizard's sheet read "Level 1" for a spell a Wizard learns
 * at 2. 67 of the 580 spells on the Wizard list are wrong that way, always
 * biased low. This command supplies the per-class answer that corrects it.
 *
 * Absence is reported, never filled: a class the engine has no ingested
 * list for comes back `known: false` with no entries, and the caller must
 * render that gap rather than fall back to the catalog level.
 */

export interface ClassSpellLevelDto {
  /** Matches `SpellCatalogEntryDto.key` exactly — the join key. */
  key: string;
  /** The spell's level for this class specifically, 0-9. */
  level: number;
}

export interface ClassSpellLevelsDto {
  /** Echoed back verbatim, including for a class the engine cannot answer for. */
  classId: string;
  /**
   * `false` when the engine has no ingested spell list for this class.
   *
   * Distinct from a known class whose list simply lacks a given spell:
   * `false` means no per-class level can be reported for ANY spell without
   * inventing one. Magus, Summoner and Oracle are the live examples — they
   * name themselves in real corpus `CLASSES:` tags, so their levels are
   * knowable, but nothing has ingested them.
   */
  known: boolean;
  /** Sorted by key. Always empty when `known` is `false`. */
  entries: ClassSpellLevelDto[];
}

export interface ClassSpellLevelsResponse {
  /** One entry per requested class id, in request order. */
  classes: ClassSpellLevelsDto[];
}

export async function loadClassSpellLevels(classIds: string[]): Promise<ClassSpellLevelsResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading per-class spell levels');
  }

  try {
    // Wrapped as `{ request: { classIds } }`, matching every other command
    // boundary here — the backend takes a named `ClassSpellLevelsRequest`
    // whose camelCase field naming is pinned by its own serde test.
    return await invoke<ClassSpellLevelsResponse>('list_class_spell_levels', {
      request: { classIds },
    });
  } catch (cause: unknown) {
    throw new Error(`Failed to load per-class spell levels: ${formatError(cause)}`);
  }
}
