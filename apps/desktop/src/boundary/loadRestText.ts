import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/** One cited rest rule, as the engine's table holds it (`rest_text::RestRow`). */
export interface RestRowDto {
  id: string;
  system: string;
  applies: string[];
  rule: string;
  text: string;
  book: string;
  page: string;
  record?: string | null;
}

/** What the print block prints: the table's rows, the GM's override, or the sentence naming what the table lacks. */
export interface RestTextDto {
  rows: RestRowDto[];
  overrideText: string | null;
  gap: string | null;
}

/** Reads the rest and recovery text the print block prints; the Biography tab shows it read-only. */
export async function loadRestText(characterId: string): Promise<RestTextDto> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading the rest text');
  }
  try {
    return await invoke<RestTextDto>('load_rest_text', { request: { characterId } });
  } catch (cause: unknown) {
    throw new Error(`Failed to load the rest text: ${formatError(cause)}`);
  }
}
