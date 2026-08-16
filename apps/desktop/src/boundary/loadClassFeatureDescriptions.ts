import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over the real corpus `DESC:` text for class
 * features (SD31-D7-PROSE-003).
 *
 * Invokes the `list_class_feature_descriptions` Tauri command
 * (`class_feature_descriptions.rs`), which reads the already-ingested,
 * already-PI-screened `cache_gen::class_feature` JSON cache directly from
 * `data/corpus/*\/class_feature/**\/*.json`. This is a SECOND, additive data
 * source alongside `ExplanationDto` — `ClassFeatureRow.detail`
 * (`classFeaturesModel.ts`) renders the engine's own computed derivation
 * text, never the rulebook prose; this call is what makes the rulebook prose
 * available at all.
 *
 * `classSlug`/`featureSlug` are `slug()` of the corpus record's owning class
 * name and feature name, in the exact algorithm
 * `v06_work_inventory.rs::slug` already uses to decide `Kind::ClassFeature`
 * doneness — `classFeaturesModel.ts`'s own join against `ExplanationDto.id`
 * reuses that same rule rather than inventing a second one.
 */
export interface ClassFeatureDescriptionDto {
  book: string;
  classSlug: string;
  featureSlug: string;
  key: string;
  name: string;
  description: string;
}

export async function loadClassFeatureDescriptions(): Promise<ClassFeatureDescriptionDto[]> {
  if (!hasTauriRuntime()) {
    return [];
  }

  try {
    return await invoke<ClassFeatureDescriptionDto[]>('list_class_feature_descriptions');
  } catch (cause: unknown) {
    throw new Error(`Failed to load class feature descriptions: ${formatError(cause)}`);
  }
}
