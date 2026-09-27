/**
 * The Load list the desktop backend ACTUALLY serves on a fresh install (both starter seeds:
 * Aldric Ironhand, Human Fighter 3; Elowen Ashgrave, Human Wizard 5), read off the artifact
 * `character_hub::starter_seed_tests::the_starter_seed_list_wire_matches_the_committed_artifact`
 * pins (that Rust test fails on any drift between the live listing and this file).
 *
 * Test-only: nothing under `src/` outside `*.test.ts` imports this module.
 */

import { readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import type { ListSavedCharactersResponse } from '../boundary/loadListSavedCharacters';

const REPO_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), '../../../..');

export const STARTER_SEED_LIST_WIRE_PATH = join(
  REPO_ROOT,
  'docs/release/SD-36-consolidation/artifacts/epic-f/stage-f6/f6e-starter-seed-list-wire.json'
);

export function starterSeedListWire(): ListSavedCharactersResponse {
  return JSON.parse(readFileSync(STARTER_SEED_LIST_WIRE_PATH, 'utf8')) as ListSavedCharactersResponse;
}
