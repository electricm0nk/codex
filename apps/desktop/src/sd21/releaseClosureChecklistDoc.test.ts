import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { assert } from '../testSupport/asserts';

// SD-21 E5.27: docs/SD-21/release-closure-checklist.md must exist and spell
// out the four-step version-bump process so a future closure cycle (or a
// human operator) has a single authoritative place to follow, rather than
// re-deriving the steps from scratch each release.

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), '..', '..', '..', '..');
const docPath = join(repoRoot, 'docs', 'SD-21', 'release-closure-checklist.md');

function verifiesChecklistDocCoversAllFourSteps() {
  let text: string;
  try {
    text = readFileSync(docPath, 'utf8');
  } catch {
    throw new Error(`expected docs/SD-21/release-closure-checklist.md to exist at ${docPath}`);
  }

  assert(text.includes('package.json'), 'checklist must name package.json in the version-bump step');
  assert(text.includes('tauri.conf.json'), 'checklist must name tauri.conf.json in the version-bump step');
  assert(text.includes('Cargo.toml'), 'checklist must name Cargo.toml in the version-bump step');
  assert(
    text.includes('publish-tester-release.yml'),
    'checklist must name the workflow stamp step'
  );
  assert(
    /build.label/i.test(text),
    'checklist must include a build-label format check step'
  );
  assert(/cargo check/.test(text), 'checklist must include the cargo check / Cargo.lock refresh step');
  assert(
    text.includes('feat(sd21): bump version to'),
    'checklist must specify the commit message shape'
  );
  assert(
    /<major>\.<tranche>\.<build>/.test(text),
    'checklist must document the <major>.<tranche>.<build> triple'
  );
}

verifiesChecklistDocCoversAllFourSteps();
