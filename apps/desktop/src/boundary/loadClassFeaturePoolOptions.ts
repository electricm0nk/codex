import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over the option-pool `class_feature` reference
 * catalog (SD31-W22-POOLMEMBER-001).
 *
 * Invokes the `list_class_feature_pool_options` Tauri command
 * (`class_feature_pool_picker.rs`), which reads the already-ingested,
 * already-PI-screened `cache_gen::class_feature` JSON cache and serves every
 * record belonging to a registered option pool (Rogue Talent today) whose
 * description renders with nothing missing.
 *
 * Unlike {@link loadClassFeatureDescriptions}, this is not joined to a
 * character's `ExplanationDto`s — a pool member (a specific Rogue Talent) is
 * not granted directly by the class, so no engine explanation names it until
 * a character has actually chosen it. This is a browsable REFERENCE list —
 * every real option, shown whether or not it is currently selected —
 * modelled on `loadAlternateRacialTraits`'s own menu precedent.
 */
export interface ClassFeaturePoolOptionDto {
  book: string;
  poolGroup: string;
  key: string;
  name: string;
  description: string;
}

export async function loadClassFeaturePoolOptions(): Promise<ClassFeaturePoolOptionDto[]> {
  if (!hasTauriRuntime()) {
    return [];
  }

  try {
    return await invoke<ClassFeaturePoolOptionDto[]>('list_class_feature_pool_options');
  } catch (cause: unknown) {
    throw new Error(`Failed to load class feature pool options: ${formatError(cause)}`);
  }
}
