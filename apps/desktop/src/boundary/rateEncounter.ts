import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read boundary over `rate_encounter` (v0.8 E-1, `encounter_rating.rs`),
 * the only source of every number the encounter builder shows. A party
 * member is a saved character (level resolved engine-side from the
 * on-disk build; an unresolvable id fails the whole rating) or a bare
 * level; a monster is a catalog key (resolved to its corpus CR; an unknown
 * key is an error) or a bare CR.
 */
export interface PartyMemberInput {
  characterId?: string;
  level?: number;
}
export interface EncounterMonsterInput {
  catalogKey?: string;
  challengeRating?: number;
}
export interface RateEncounterRequest {
  party: PartyMemberInput[];
  monsters: EncounterMonsterInput[];
}
export interface RatedPartyMemberDto {
  characterId?: string;
  /** The level the engine rated this member at. */
  level: number;
}
export interface RatedMonsterDto {
  catalogKey?: string;
  name?: string;
  challengeRating: number;
  /** The CR the engine actually rated with — read back, not recomputed. */
  crAsRated: number;
  outsideVerifiedRange: boolean;
  reason?: string;
}
export interface DifficultyScaleDto {
  tiers: number;
  collapsedFrom: number;
  note: string;
}
export interface RateEncounterResponse {
  /** `"easy" | "medium" | "hard" | "deadly"` — the engine's four tiers. */
  difficulty: string;
  averagePartyLevel: number;
  encounterLevel: number;
  elMinusApl: number;
  party: RatedPartyMemberDto[];
  monsters: RatedMonsterDto[];
  anyOutsideVerifiedRange: boolean;
  verifiedCrRange: [number, number];
  difficultyScale: DifficultyScaleDto;
  caveats: string[];
}

export async function rateEncounter(request: RateEncounterRequest): Promise<RateEncounterResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for rating an encounter');
  }
  try {
    return await invoke<RateEncounterResponse>('rate_encounter', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to rate the encounter: ${formatError(cause)}`);
  }
}
