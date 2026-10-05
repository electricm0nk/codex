import { loadListSavedCharacters, type ListSavedCharactersArgs } from '../boundary/loadListSavedCharacters';
import { loadCreateCharacter, type CreateCharacterRequest } from '../boundary/loadCreateCharacter';
import type { RecomputeCharacterRequest } from '../boundary/recomputeCharacter';
import { buildCharacterHubListSurface, type CharacterHubListSurface } from './buildCharacterHubListSurface';
import {
  buildCreateCharacterOutcomeSurface,
  type CreateCharacterOutcomeContext,
  type CreateCharacterOutcomeSurface,
} from './buildCreateCharacterOutcomeSurface';
import { findClassOption } from './classCatalog';
import { hasTauriRuntime } from '../boundary/runtime';
import { buildPreviewListSurface } from './previewData';
import { RULE_SETS, type RuleSetId } from './LandingScreen';

/**
 * Maps the panel's active `RuleSetId` (the landing screen's rule-set
 * picker — `LandingScreen.tsx`) to the wire-level `ruleSystemId` the Rust
 * `resolve_rule_system_adapter` dispatch seam understands
 * (`apps/desktop/src-tauri/src/rule_system_adapter.rs`): `"pf1"` resolves to
 * `Pf1Adapter`, `"starfinder-1e"` to `StarfinderAdapter` (SD-37 E4.6), and
 * any other id to the governed `StubAdapter` seam — see
 * `docs/governance/wired-integration-stubs-registry.md` entry 0002 — which
 * honestly errors rather than silently falling through to PF1 logic.
 * Pathfinder 1e's landing id differs from its wire id, so it is the one id
 * rewritten; every other `RuleSetId` passes through unchanged rather than
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

/**
 * The `list_saved_characters` arguments for the active rule set: the Load
 * list asks the active system's adapter for its characters
 * (`StarfinderAdapter` lists Starfinder saves only).
 */
export function buildListSavedCharactersArgs(ruleSet: RuleSetId): ListSavedCharactersArgs {
  return { ruleSystemId: resolveRuleSystemId(ruleSet) };
}

/** Whether "New Character" opens a creation flow for the active rule set. */
export interface CharacterCreationGate {
  enabled: boolean;
  disabledHint?: string;
}

/**
 * The creation form on this screen builds Pathfinder 1e characters only, so it
 * is open for Pathfinder and closed, with its reason, for every other rule set
 * — a Starfinder selection never opens the Pathfinder form.
 */
export function characterCreationGate(ruleSet: RuleSetId): CharacterCreationGate {
  if (ruleSet === 'pathfinder-1e') {
    return { enabled: true };
  }
  const name = RULE_SETS.find((candidate) => candidate.id === ruleSet)?.name ?? ruleSet;
  return { enabled: false, disabledHint: `${name} character creation is not available on this screen yet` };
}

/**
 * Thin wrapper composing the real boundary loaders with the pure mappers.
 * With no rule set the listing is Pathfinder's (every caller outside the
 * character hub: campaigns, the encounter builder, the trait picker).
 */
export async function loadCharacterHubListSurfaceRuntime(ruleSet?: RuleSetId): Promise<CharacterHubListSurface> {
  // Browser preview (no desktop backend): surface a sample character so the
  // Load → sheet flow stays walkable without the Tauri runtime.
  if (!hasTauriRuntime()) {
    return buildPreviewListSurface();
  }
  const snapshot = await loadListSavedCharacters(ruleSet === undefined ? undefined : buildListSavedCharactersArgs(ruleSet));
  return buildCharacterHubListSurface(snapshot);
}

function outcomeContextFromRequest(request: CreateCharacterRequest): CreateCharacterOutcomeContext {
  const classOption = findClassOption(request.classId);
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
