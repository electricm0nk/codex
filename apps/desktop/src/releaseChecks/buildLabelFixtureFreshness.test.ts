import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { assert } from '../testSupport/asserts';

// SD-22 E8.28: build-label test fixtures across the SD-11 tester-workbench
// surface must carry the current tranche's build-label shape, not a stale
// value left behind by a prior version bump (epic-breakdown.md criterion 28,
// file-touch partition in loop-instruction.md).

const appRoot = join(dirname(fileURLToPath(import.meta.url)), '..', '..');

const FIXTURE_FILES = [
  'src/testerWorkbench/loadTesterWorkbenchSurface.test.ts',
  'src/testerWorkbench/status/createWorkbenchStatus.test.ts',
  'src/testSupport/makeSurface.ts',
];

function readPackageJsonVersion(): string {
  const doc = JSON.parse(readFileSync(join(appRoot, 'package.json'), 'utf8'));
  return doc.version;
}

// The exact pre-bump literal these fixtures carried before the most recent
// version bump. Kept one bump behind on purpose: it is the literal a
// half-applied bump would leave behind, so it is the one worth naming
// explicitly. Updated to 0.8.0 by the tranche/9 cut (the version files moved
// 0.8.0 -> 0.9.0 because a NEW tranche/N branch was cut; a bundle's own
// closure on an unchanged tranche branch does not move the tranche digit).
// The prior value named here was 'Codex 0.5.96-test', from SD-23's Epic 7
// build-number increment. Some files also carry unrelated arbitrary version
// placeholders (e.g. '0.0.0-test') for isolated formatter tests — those
// aren't "the current build" fixture and must not be flagged here.
const STALE_LABEL = 'Codex 0.8.0-test';

function verifiesFixturesCarryCurrentTrancheBuildLabel() {
  const pkgVersion = readPackageJsonVersion();
  const expected = `Codex ${pkgVersion}-test`;

  for (const relPath of FIXTURE_FILES) {
    const text = readFileSync(join(appRoot, relPath), 'utf8');
    assert(!text.includes(STALE_LABEL), `${relPath} still carries the pre-bump build-label fixture "${STALE_LABEL}"`);
    assert(text.includes(expected), `${relPath} must carry the current tranche's build-label fixture "${expected}"`);
  }
}

verifiesFixturesCarryCurrentTrancheBuildLabel();
