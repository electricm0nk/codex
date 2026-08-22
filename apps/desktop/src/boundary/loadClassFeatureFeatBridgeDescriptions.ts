import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { ClassFeatureDescriptionDto } from './loadClassFeatureDescriptions';

/**
 * Read-only desktop boundary over the `class_feature` → `feat` cross-
 * reference bridge (SD31-W29-CLASSFEATURE-FEATBRIDGE-001, THE-BOX §2.1 F2).
 *
 * Invokes the `list_class_feature_feat_bridge_descriptions` Tauri command
 * (`class_feature_feat_bridge.rs`), which serves the SAME
 * {@link ClassFeatureDescriptionDto} shape {@link loadClassFeatureDescriptions}
 * does, for a DISJOINT population: `class_feature` records with no local
 * `DESC:` text of their own whose entire content is a grant of an
 * already-separately-modelled `feat` (e.g. `Golden Legionnaire ~ Swift Aid`
 * carries no description, but its `ABILITY:FEAT|AUTOMATIC|Swift Aid` token
 * names a real, already-described feat). The two lists are meant to be
 * concatenated before being passed to `buildClassFeatureSurface` — see
 * `CharacterSheet.tsx`'s own call site — never called independently of
 * each other, and never both trusted to describe the same record (the
 * Rust side keeps the two populations disjoint by construction; see that
 * module's own doc comment).
 */
export async function loadClassFeatureFeatBridgeDescriptions(): Promise<ClassFeatureDescriptionDto[]> {
  if (!hasTauriRuntime()) {
    return [];
  }

  try {
    return await invoke<ClassFeatureDescriptionDto[]>('list_class_feature_feat_bridge_descriptions');
  } catch (cause: unknown) {
    throw new Error(`Failed to load class feature feat-bridge descriptions: ${formatError(cause)}`);
  }
}
