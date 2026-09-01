import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { assert } from '../testSupport/asserts';

// SD-22 E8.28: build-label test fixtures across the SD-11 tester-workbench
// surface must carry the current tranche's build-label shape, not a stale
// value left behind by a prior version bump (epic-breakdown.md criterion 28,
// file-touch partition in loop-instruction.md).

const appRoot = join(dirname(fileURLToPath(import.meta.url)), '..', '..');

// SD-30 Epic 7 (SD30-E7-F1-001, re-dispatch): the version-bump-to-0.10.0 cycle
// found this list incomplete. `src/testSupport/makeSurface.ts` is the single
// source of truth for the test surface fixture (its own doc comment says so),
// and four more test files assert its `buildLabel` output as a hardcoded
// literal downstream of `makeSurface()` rather than deriving it — they broke
// silently the moment `makeSurface.ts` moved to 0.10.0 until added here.
// Widened to the full consumer surface so a future bump cannot miss them
// again the same way. Confirmed still complete at the 0.10.0 -> 0.11.0
// tranche/11 bump (SD31-E10-F1-001): all 7 entries below were re-grepped
// fresh against the live tree rather than trusted from this list.
// The 0.11.0 -> 0.12.0/0.13.0 tranche cuts each missed this fixture entirely
// (only caught here at the 0.11.0 -> 0.14.0 tranche/14 bump, SD-34
// AT-34-E6-001, gate-remediation wave 23) — all 7 entries re-grepped fresh
// against the live tree again rather than trusted from this list.
const FIXTURE_FILES = [
  'src/testerWorkbench/loadTesterWorkbenchSurface.test.ts',
  'src/testerWorkbench/status/createWorkbenchStatus.test.ts',
  'src/testSupport/makeSurface.ts',
  'src/operatorTriage/buildOperatorTriageDraft.test.ts',
  'src/testerWorkbench/feedback/evidence/captureFeedbackEvidence.test.ts',
  'src/testerWorkbench/feedback/enhancement/composeEnhancementRequest.test.ts',
  'src/testerWorkbench/feedback/bug/composeBugReport.test.ts',
];

function readPackageJsonVersion(): string {
  const doc = JSON.parse(readFileSync(join(appRoot, 'package.json'), 'utf8'));
  return doc.version;
}

// The exact pre-bump literal these fixtures carried before the most recent
// version bump. Kept one bump behind on purpose: it is the literal a
// half-applied bump would leave behind, so it is the one worth naming
// explicitly. Updated to 0.11.0 by SD-34's tranche/14 version-bump cycle
// (AT-34-E6-001; the version files moved 0.11.0 -> 0.14.0 on the
// already-cut tranche/14 branch — a bundle's own closure does not move the
// tranche digit, only a NEW tranche/N cut does; two intervening tranches,
// tranche/12 and tranche/13, each missed this fixture list, which is exactly
// why this test exists). The prior value named here was 'Codex 0.10.0-test',
// from the tranche/11 cut. Some files also carry unrelated arbitrary version
// placeholders (e.g. '0.0.0-test') for isolated formatter tests — those
// aren't "the current build" fixture and must not be flagged here.
const STALE_LABEL = 'Codex 0.11.0-test';

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
