/**
 * The class roster and level-up wire rows the desktop backend ACTUALLY serves, read off the
 * artifacts `character_hub::tests::list_class_roster_wire_carries_hit_die_and_skill_ranks_for_every_census_class`
 * pins (that Rust test fails on any drift between the live commands and these files). Frontend
 * tests read them instead of a hand-written sample, so "the picker offers N classes" is N of the
 * real roster.
 *
 * Test-only: nothing under `src/` outside `*.test.ts` imports this module.
 */

import { readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import type { ClassCreationRosterResponse, LevelUpClassOptionsResponse } from '../characterHub/classRoster';

const REPO_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), '../../../..');
const WIRE_DIR = join(REPO_ROOT, 'docs/release/SD-36-consolidation/artifacts/epic-f/stage-f4-f5');

export const CLASS_ROSTER_WIRE_PATH = join(WIRE_DIR, 'f4c-class-roster-wire.json');
export const LEVEL_UP_FIGHTER6_WIRE_PATH = join(WIRE_DIR, 'f4c-level-up-fighter6-wire.json');

export function classRosterWire(): ClassCreationRosterResponse {
  return JSON.parse(readFileSync(CLASS_ROSTER_WIRE_PATH, 'utf8')) as ClassCreationRosterResponse;
}

export function levelUpFighter6Wire(): LevelUpClassOptionsResponse {
  return JSON.parse(readFileSync(LEVEL_UP_FIGHTER6_WIRE_PATH, 'utf8')) as LevelUpClassOptionsResponse;
}
