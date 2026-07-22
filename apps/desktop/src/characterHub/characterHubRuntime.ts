import { loadListSavedCharacters } from '../boundary/loadListSavedCharacters';
import { loadCreateCharacter, type CreateCharacterRequest } from '../boundary/loadCreateCharacter';
import type { RecomputeCharacterRequest } from '../boundary/recomputeCharacter';
import { buildCharacterHubListSurface, type CharacterHubListSurface } from './buildCharacterHubListSurface';
import {
  buildCreateCharacterOutcomeSurface,
  type CreateCharacterOutcomeContext,
  type CreateCharacterOutcomeSurface,
} from './buildCreateCharacterOutcomeSurface';
import { CLASS_OPTIONS } from './characterHubModel';
import { hasTauriRuntime } from '../boundary/runtime';
import { buildPreviewListSurface } from './previewData';
import type { RuleSetId } from './LandingScreen';

/**
 * Maps the panel's active `RuleSetId` (the landing screen's rule-set
 * picker — `LandingScreen.tsx`) to the wire-level `ruleSystemId` the Rust
 * `resolve_rule_system_adapter` dispatch seam understands (SD-25 Criterion
 * 3.4: `"pf1"` resolves to the real `Pf1Adapter`; any other id resolves to
 * the governed `StubAdapter` seam — see
 * `governance/wired-integration-stubs-registry.md` entry 0002 — which
 * honestly errors rather than silently falling through to PF1 logic).
 * Pathfinder 1e is the only rule set with a real adapter today, so every
 * other `RuleSetId` intentionally passes through unchanged rather than
 * being rewritten to `"pf1"` — this is
 * the seam SD-25 Criterion 3.5's RED targets: before this function existed,
 * the panel had no concept of "the active adapter" at all, and every call
 * site that has since grown a `ruleSystemId` field (3.4's `appendToCharacter`
 * / `recomputeCharacter` / `reSaveCharacter`) had no UI caller to route it
 * through.
 */
export function resolveRuleSystemId(ruleSet: RuleSetId): string {
  return ruleSet === 'pathfinder-1e' ? 'pf1' : ruleSet;
}

/**
 * Pure request composer for `recompute_character` (mirrors
 * `composeCreateCharacterRequest`'s own split) — proves the panel really
 * routes a mutation call site through the active adapter rather than
 * hardcoding `"pf1"` inline at the call site. Backs `CharacterSheet.tsx`'s
 * "Recompute" menu action (SD-25 Criterion 3.5 register A3: a real UI
 * affordance wired to `recompute_character`, matching SD-24 Criterion 7.4's
 * Add-Weapon/Add-Armor/Add-Spell precedent).
 */
export function buildRecomputeCharacterRequest(
  characterId: string,
  ruleSet: RuleSetId
): RecomputeCharacterRequest {
  return {
    characterId,
    ruleSystemId: resolveRuleSystemId(ruleSet),
  };
}

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
