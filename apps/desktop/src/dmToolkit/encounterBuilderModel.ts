import type { RateEncounterRequest } from '../boundary/rateEncounter';

/**
 * The encounter builder's state (v0.8 E-2) and the request it composes.
 * Pure: no rating, no levels for saved characters (the engine resolves
 * those from the id), no CRs (a monster goes as its catalog key). Levels
 * typed by hand are bounded to the engine's `u8` domain, 1–20.
 */
export type PartyMember = { kind: 'saved'; characterId: string } | { kind: 'typed'; level: number };
export interface MonsterRow {
  catalogKey: string;
  count: number;
}
export interface EncounterBuilder {
  party: PartyMember[];
  monsters: MonsterRow[];
}

export function emptyBuilder(): EncounterBuilder {
  return { party: [], monsters: [] };
}
export function addSavedMember(b: EncounterBuilder, characterId: string): EncounterBuilder {
  return { ...b, party: [...b.party, { kind: 'saved', characterId }] };
}
export function addTypedMember(b: EncounterBuilder, level: number): EncounterBuilder {
  if (!Number.isInteger(level) || level < 1 || level > 20) {
    return b;
  }
  return { ...b, party: [...b.party, { kind: 'typed', level }] };
}
export function removeMember(b: EncounterBuilder, index: number): EncounterBuilder {
  return { ...b, party: b.party.filter((_, i) => i !== index) };
}
export function addMonster(b: EncounterBuilder, catalogKey: string): EncounterBuilder {
  const existing = b.monsters.find((row) => row.catalogKey === catalogKey);
  return existing ? setMonsterCount(b, catalogKey, existing.count + 1) : { ...b, monsters: [...b.monsters, { catalogKey, count: 1 }] };
}
export function setMonsterCount(b: EncounterBuilder, catalogKey: string, count: number): EncounterBuilder {
  if (!Number.isInteger(count) || count < 1) {
    return removeMonster(b, catalogKey);
  }
  return { ...b, monsters: b.monsters.map((row) => (row.catalogKey === catalogKey ? { ...row, count } : row)) };
}
export function removeMonster(b: EncounterBuilder, catalogKey: string): EncounterBuilder {
  return { ...b, monsters: b.monsters.filter((row) => row.catalogKey !== catalogKey) };
}

/** `null` while there is nothing to rate; otherwise exactly what `rate_encounter` takes. */
export function toRequest(b: EncounterBuilder): RateEncounterRequest | null {
  if (b.party.length === 0 || b.monsters.length === 0) {
    return null;
  }
  return {
    party: b.party.map((member) => (member.kind === 'saved' ? { characterId: member.characterId } : { level: member.level })),
    monsters: b.monsters.flatMap((row) => Array.from({ length: row.count }, () => ({ catalogKey: row.catalogKey }))),
  };
}
