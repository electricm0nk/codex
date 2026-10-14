import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Desktop boundary over Cyberpunk RED creation (SD-38 E3.1):
 * `preview_cyberpunk_character` and `create_cyberpunk_character`
 * (`apps/desktop/src-tauri/src/cpr_creation.rs`). The STATs come from the Role's template
 * (Streetrat row rolled or picked; Edgerunner one 1d10 per STAT) or the player's Complete
 * Package allocation, and every number is the engine's `cpr.` row. The dice come from one
 * seedable roller: send the preview's `seed` back on create to keep the previewed dice.
 */

export type CyberpunkCreationMethod = 'streetrat' | 'edgerunner' | 'completePackage';

export interface CyberpunkCreationRequest {
  /** A `role` record name, e.g. `Solo`. */
  role: string;
  method: CyberpunkCreationMethod;
  /** The roller's seed (0 to 4294967295); absent, Rust draws one and returns it. */
  seed?: number | null;
  /** Streetrat only: the picked template row, 1-10; absent, it is rolled on 1d10. */
  templateRoll?: number | null;
  /** Complete Package only: the allocation keyed by STAT id (`int` ... `emp`). */
  stats?: Record<string, number> | null;
}

export interface CyberpunkCreateRequest {
  characterId: string;
  displayLabel: string;
  savedAt: string;
  creation: CyberpunkCreationRequest;
}

export interface CyberpunkRoll {
  /** `streetrat_row`, or `stat.<id>` for an Edgerunner roll. */
  table: string;
  die: string;
  result: number;
}

export interface CyberpunkStatValue {
  id: string;
  label: string;
  value: number;
}

export interface CyberpunkCreationRow {
  id: string;
  value: number;
  detail: string;
}

export interface CyberpunkCreationProblem {
  id: string;
  message: string;
  claimBlocking: boolean;
}

export interface CyberpunkCreationPreview {
  role: string;
  method: CyberpunkCreationMethod;
  seed: number;
  rolls: CyberpunkRoll[];
  templateRoll: number | null;
  stats: CyberpunkStatValue[];
  statPointsSpent: number | null;
  statPointsTotal: number | null;
  /** The engine's `cpr.stat.*` and `cpr.derived.*` rows; empty while a problem stands. */
  explanations: CyberpunkCreationRow[];
  problems: CyberpunkCreationProblem[];
}

export interface CyberpunkCharacterSummary {
  characterId: string;
  displayLabel: string;
  gameSystem: string;
  schemaVersion: number;
  savedAt: string;
  /** Empty: a Cyberpunk RED character has no Pathfinder race or class levels. */
  raceId: string;
  classSummary: string;
}

export type CyberpunkCreateResponse =
  | { kind: 'Saved'; summary: CyberpunkCharacterSummary; seed: number; explanations: CyberpunkCreationRow[] }
  | { kind: 'Blocked'; diagnostics: CyberpunkCreationProblem[] };

function wire(request: CyberpunkCreationRequest) {
  return {
    role: request.role,
    method: request.method,
    seed: request.seed ?? null,
    templateRoll: request.templateRoll ?? null,
    stats: request.stats ?? null,
  };
}

export async function previewCyberpunkCharacter(request: CyberpunkCreationRequest): Promise<CyberpunkCreationPreview> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for previewing a Cyberpunk RED character');
  }

  try {
    return await invoke<CyberpunkCreationPreview>('preview_cyberpunk_character', { request: wire(request) });
  } catch (cause: unknown) {
    throw new Error(`Failed to preview the Cyberpunk RED character: ${formatError(cause)}`);
  }
}

export async function createCyberpunkCharacter(request: CyberpunkCreateRequest): Promise<CyberpunkCreateResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for creating a Cyberpunk RED character');
  }

  try {
    return await invoke<CyberpunkCreateResponse>('create_cyberpunk_character', {
      request: {
        characterId: request.characterId,
        displayLabel: request.displayLabel,
        savedAt: request.savedAt,
        creation: wire(request.creation),
      },
    });
  } catch (cause: unknown) {
    throw new Error(`Failed to create the Cyberpunk RED character: ${formatError(cause)}`);
  }
}
