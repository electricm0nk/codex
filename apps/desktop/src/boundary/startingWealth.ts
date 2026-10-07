import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/** A class's maximum starting money. `maxGp` is `null` (with the reason in `note`) when none is published. */
export interface StartingWealthDto {
  maxGp: number | null;
  note: string | null;
}

export async function loadStartingWealth(classId: string): Promise<StartingWealthDto> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading starting wealth');
  }
  try {
    return await invoke<StartingWealthDto>('starting_wealth_for_class', { classId });
  } catch (cause: unknown) {
    throw new Error(`Failed to load the class's starting money: ${formatError(cause)}`);
  }
}
