import { loadListSavedCharacters } from '../boundary/loadListSavedCharacters';
import { loadCreateCharacter, type CreateCharacterRequest } from '../boundary/loadCreateCharacter';
import { buildCharacterHubListSurface, type CharacterHubListSurface } from './buildCharacterHubListSurface';
import {
  buildCreateCharacterOutcomeSurface,
  type CreateCharacterOutcomeContext,
  type CreateCharacterOutcomeSurface,
} from './buildCreateCharacterOutcomeSurface';
import { CLASS_OPTIONS } from './characterHubModel';
import { hasTauriRuntime } from '../boundary/runtime';
import { buildPreviewListSurface } from './previewData';

/** Thin wrapper composing the real boundary loaders with the pure mappers. */
export async function loadCharacterHubListSurfaceRuntime(): Promise<CharacterHubListSurface> {
  // Browser preview (no desktop backend): surface a sample character so the
  // Load → sheet flow stays walkable without the Tauri runtime.
  if (!hasTauriRuntime()) {
    return buildPreviewListSurface();
  }
  const snapshot = await loadListSavedCharacters();
  return buildCharacterHubListSurface(snapshot);
}

function outcomeContextFromRequest(request: CreateCharacterRequest): CreateCharacterOutcomeContext {
  const classOption = CLASS_OPTIONS.find((option) => option.id === request.classId);
  return {
    raceId: request.raceId,
    classId: request.classId,
    classLabel: classOption?.label ?? request.classId,
    supportLevel: classOption?.supportLevel ?? 'none',
  };
}

export async function createCharacterRuntime(
  request: CreateCharacterRequest
): Promise<CreateCharacterOutcomeSurface> {
  const outcome = await loadCreateCharacter(request);
  return buildCreateCharacterOutcomeSurface(outcome, outcomeContextFromRequest(request));
}
