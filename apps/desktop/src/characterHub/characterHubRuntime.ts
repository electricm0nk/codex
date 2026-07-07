import { loadListSavedCharacters } from '../boundary/loadListSavedCharacters';
import { loadCreateCharacter, type CreateCharacterRequest } from '../boundary/loadCreateCharacter';
import { buildCharacterHubListSurface, type CharacterHubListSurface } from './buildCharacterHubListSurface';
import {
  buildCreateCharacterOutcomeSurface,
  type CreateCharacterOutcomeSurface,
} from './buildCreateCharacterOutcomeSurface';

/** Thin wrapper composing the real boundary loaders with the pure mappers. */
export async function loadCharacterHubListSurfaceRuntime(): Promise<CharacterHubListSurface> {
  const snapshot = await loadListSavedCharacters();
  return buildCharacterHubListSurface(snapshot);
}

export async function createCharacterRuntime(
  request: CreateCharacterRequest
): Promise<CreateCharacterOutcomeSurface> {
  const outcome = await loadCreateCharacter(request);
  return buildCreateCharacterOutcomeSurface(outcome);
}
