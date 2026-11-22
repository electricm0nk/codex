import { openUrl } from '@tauri-apps/plugin-opener';
import { formatError, hasTauriRuntime } from './runtime';

/** Opens a web link in the system browser (`tauri-plugin-opener`, capability `opener:default`). */
export async function openExternal(url: string): Promise<void> {
  if (!hasTauriRuntime()) {
    throw new Error(`Tauri runtime not available for opening ${url}`);
  }
  try {
    await openUrl(url);
  } catch (cause: unknown) {
    throw new Error(`Failed opening ${url}: ${formatError(cause)}`);
  }
}
