import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from '../runtime';

/** Shared invoke wrapper: one no-runtime message and one error prefix per action. */
export async function invokeCampaignFolder<T>(
  command: string,
  args: Record<string, unknown>,
  action: string
): Promise<T> {
  if (!hasTauriRuntime()) {
    throw new Error(`Tauri runtime not available for ${action}`);
  }
  try {
    return await invoke<T>(command, args);
  } catch (cause: unknown) {
    throw new Error(`Failed ${action}: ${formatError(cause)}`);
  }
}
