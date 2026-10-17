import { invoke } from '@tauri-apps/api/core';
import type { LoadSavedCharacterResponse } from './loadSavedCharacterDetail';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Desktop boundary over the Cyberpunk RED sheet's saves (SD-38 E4.5):
 * `save_cyberpunk_criticals` (`apps/desktop/src-tauri/src/cpr_adapter.rs`). The checklist sends
 * every Critical Injury it holds ticked; the adapter composes the sheet with them (refusing an
 * injury not on its table, or one recorded twice, before anything is written), saves them with
 * the character's other choices and answers with the reloaded sheet, whose Base Death Save
 * Penalty row the engine has moved. Nothing here counts a penalty.
 */

/** One Critical Injury by its table (`critical_injury.json` `location`) and name. */
export interface CyberpunkCritical {
  location: 'body' | 'head';
  name: string;
}

export async function saveCyberpunkCriticals(
  characterId: string,
  criticals: readonly CyberpunkCritical[],
): Promise<LoadSavedCharacterResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for saving the Cyberpunk RED critical injuries');
  }

  try {
    return await invoke<LoadSavedCharacterResponse>('save_cyberpunk_criticals', {
      request: {
        characterId,
        criticals: criticals.map((critical) => ({ location: critical.location, name: critical.name })),
      },
    });
  } catch (cause: unknown) {
    throw new Error(`Failed to save the Cyberpunk RED critical injuries: ${formatError(cause)}`);
  }
}
