/**
 * The `list_class_facts` wire the desktop backend ACTUALLY serves for every roster class at
 * levels 1 and 7, read off the artifact `class_facts::tests::class_facts_wire_for_every_roster_class_matches_the_committed_artifact`
 * pins (that Rust test fails on any drift between the live command and this file).
 *
 * Test-only: nothing under `src/` outside `*.test.ts` imports this module.
 */

import { readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import type { ClassFactsDto, ListClassFactsResponse } from '../boundary/listClassFacts';

const REPO_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), '../../../..');

export const CLASS_FACTS_WIRE_PATH = join(
  REPO_ROOT,
  'docs/release/SD-36-consolidation/artifacts/epic-f/stage-f6/f6a-class-facts-wire.json'
);

export function classFactsWire(): ListClassFactsResponse {
  return JSON.parse(readFileSync(CLASS_FACTS_WIRE_PATH, 'utf8')) as ListClassFactsResponse;
}

/** The served facts for `classId` at `level` (1 or 7); throws when the wire has no such row. */
export function classFactsFor(classId: string, level: 1 | 7): ClassFactsDto {
  const row = classFactsWire().classes.find((entry) => entry.classId === classId && entry.level === level);
  if (!row) {
    throw new Error(`no served class facts for ${classId} level ${level}`);
  }
  return row;
}
