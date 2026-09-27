/**
 * The feat skill-bonus fold the desktop backend ACTUALLY serves on the sheet
 * (`LoadSavedCharacterResponse.featSkillBonuses`) for the census fixture (Human Fighter 1) with
 * Alertness as its level-1 feat, read off the artifact
 * `character_hub::tests::feat_skill_bonuses_wire_for_the_census_fixture_with_alertness_matches_the_committed_artifact`
 * pins (that Rust test fails on any drift between the live fold and this file).
 *
 * Test-only: nothing under `src/` outside `*.test.ts` imports this module.
 */

import { readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import type { FeatSkillBonusesDto } from '../boundary/loadSavedCharacterDetail';

const REPO_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), '../../../..');

export const FEAT_SKILL_BONUS_WIRE_PATH = join(
  REPO_ROOT,
  'docs/release/SD-36-consolidation/artifacts/epic-f/stage-f6/f6b-feat-skill-bonus-wire.json'
);

export function featSkillBonusWire(): FeatSkillBonusesDto {
  return JSON.parse(readFileSync(FEAT_SKILL_BONUS_WIRE_PATH, 'utf8')) as FeatSkillBonusesDto;
}
