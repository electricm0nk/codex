import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read desktop boundary for the Buy Custom builder's choices (SD-41 `decisions.md §24`).
 *
 * Invokes the `list_custom_equipment_options` Tauri command: the special abilities and materials
 * whose record types name one of the base item's, and the wielder sizes the base can be made for.
 */

/** A wielder size a custom copy is made for (Rust `SizeCategory`); Buy Custom offers Small and Large. */
export type SizeCategory = 'Fine' | 'Diminutive' | 'Tiny' | 'Small' | 'Medium' | 'Large' | 'Huge' | 'Gargantuan' | 'Colossal';

export interface CustomOption {
  key: string;
  name: string;
  description: string | null;
  /** The enhancement it counts as (Flaming is +1); 0 for a material or a quality. */
  plus: number;
  /** Its price cannot be read from the record: a build holding it is priced by Negotiate. */
  unpriced: boolean;
  /** What the item must also be, in the record's words, when the base's own types do not show it. */
  prerequisite?: string;
}

export interface CustomEquipmentOptions {
  abilities: CustomOption[];
  materials: CustomOption[];
  /** Small and Large on arms, armor, shields and ammunition; empty for other gear. */
  sizes: SizeCategory[];
}

export async function listCustomEquipmentOptions(baseItemId: string): Promise<CustomEquipmentOptions> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for listing custom equipment options');
  }

  try {
    return await invoke<CustomEquipmentOptions>('list_custom_equipment_options', { request: { baseItemId } });
  } catch (cause: unknown) {
    throw new Error(`Failed to list custom equipment options: ${formatError(cause)}`);
  }
}
