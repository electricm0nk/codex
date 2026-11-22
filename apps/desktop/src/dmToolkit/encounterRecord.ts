import type { RateEncounterResponse } from '../boundary/rateEncounter';
import { describeTier, formatCr } from './encounterViewModel';

/**
 * A rated encounter as structured text for a Scene record's `encounter`
 * field (v0.8 G-4). Written entirely from the `rate_encounter` response:
 * tier with its rulebook gloss, the unverified disclosure when the engine
 * set it, APL/EL/EL−APL verbatim, the party at the levels the engine
 * rated, every monster with its corpus CR and the CR it was rated at plus
 * the engine's reason, the scale note and every caveat. It is a record of
 * what the engine said at `ratedAt` — never re-rated on save or export.
 */
export function formatEncounterRecord(
  response: RateEncounterResponse,
  savedCharacterNames: Record<string, string>,
  ratedAt: string,
): string {
  const range = `CR ${response.verifiedCrRange[0]}–${response.verifiedCrRange[1]}`;
  const tierLine = response.anyOutsideVerifiedRange
    ? `Difficulty: ${describeTier(response.difficulty)} — UNVERIFIED: one or more creatures are outside the engine’s verified ${range} table; see the marked monsters.`
    : `Difficulty: ${describeTier(response.difficulty)} — every creature inside the engine’s verified ${range} table.`;
  const party = response.party
    .map((member) => (member.characterId ? `${savedCharacterNames[member.characterId] ?? member.characterId} (level ${member.level})` : `level ${member.level}`))
    .join(', ');
  const grouped = new Map<string, { count: number; monster: RateEncounterResponse['monsters'][number] }>();
  for (const monster of response.monsters) {
    const key = monster.catalogKey ?? `cr:${monster.challengeRating}`;
    const existing = grouped.get(key);
    if (existing) {
      existing.count += 1;
    } else {
      grouped.set(key, { count: 1, monster });
    }
  }
  const monsters = [...grouped.values()].map(({ count, monster }) => {
    const name = monster.name ?? monster.catalogKey ?? `CR ${formatCr(monster.challengeRating)} creature`;
    const corpus = formatCr(monster.challengeRating);
    const rerated = Math.abs(monster.challengeRating - monster.crAsRated) >= 0.01;
    const cr = rerated ? `CR ${corpus}, rated as CR ${monster.crAsRated}` : `CR ${corpus}`;
    const flag = monster.outsideVerifiedRange ? ` ⚠ ${monster.reason ?? 'outside the verified table'}` : '';
    return `- ${count} × ${name} — ${cr}${flag}`;
  });
  const signed = response.elMinusApl < 0 ? String(response.elMinusApl) : `+${response.elMinusApl}`;
  return [
    tierLine,
    `APL ${response.averagePartyLevel} · EL ${response.encounterLevel} · EL−APL ${signed}`,
    `Party: ${party}`,
    'Monsters:',
    ...monsters,
    `Scale: ${response.difficultyScale.tiers} tiers, collapsed from the rulebook’s ${response.difficultyScale.collapsedFrom}. ${response.difficultyScale.note}`,
    ...response.caveats.map((caveat) => `Caveat: ${caveat}`),
    `Rated ${ratedAt} by the engine; not re-rated on export.`,
  ].join('\n');
}
