import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { assertEqual, assert } from '../testSupport/asserts';

// SD-21 E5.25: package.json, tauri.conf.json, and Cargo.toml must all carry
// the same `<major>.<tranche>.<build>` version triple, and the workflow's
// publish-time stamp must write that same triple shape (not the older
// `0.0.<run>` scheme), so tester/published builds never silently regress to
// an old numbering scheme regardless of what the repo files say.

const appRoot = join(dirname(fileURLToPath(import.meta.url)), '..', '..');
const repoRoot = join(appRoot, '..', '..');

const TRIPLE_RE = /^\d+\.\d+\.\d+$/;

function readPackageJsonVersion(): string {
  const doc = JSON.parse(readFileSync(join(appRoot, 'package.json'), 'utf8'));
  return doc.version;
}

function readTauriConfVersion(): string {
  const doc = JSON.parse(readFileSync(join(appRoot, 'src-tauri', 'tauri.conf.json'), 'utf8'));
  return doc.version;
}

function readCargoTomlVersion(): string {
  const text = readFileSync(join(appRoot, 'src-tauri', 'Cargo.toml'), 'utf8');
  const match = text.match(/^version\s*=\s*"([^"]+)"/m);
  assert(match !== null, 'Cargo.toml must declare a top-level version field');
  return match![1];
}

const RUN_NUMBER_EXPANSION = '${GITHUB_RUN_NUMBER}';

function readWorkflowText(): string {
  return readFileSync(
    join(repoRoot, '.github', 'workflows', 'publish-tester-release.yml'),
    'utf8'
  );
}

function readWorkflowStampVersion(): string {
  const text = readWorkflowText();
  const matches = [...text.matchAll(/VERSION="([^"]*\$\{GITHUB_RUN_NUMBER\}[^"]*)"/g)];
  assert(
    matches.length > 0,
    'publish-tester-release.yml must stamp a VERSION containing GITHUB_RUN_NUMBER'
  );
  // A second stamping site would let two lanes drift apart the same way the
  // stamp and the repo files did; the publish lane has exactly one stamp.
  assertEqual(
    matches.length,
    1,
    'publish-tester-release.yml must define exactly one GITHUB_RUN_NUMBER-based VERSION stamp'
  );
  return matches[0][1];
}

function verifiesAllThreeVersionFilesAgreeAndFollowTripleShape() {
  const pkg = readPackageJsonVersion();
  const tauri = readTauriConfVersion();
  const cargo = readCargoTomlVersion();

  assert(TRIPLE_RE.test(pkg), `package.json version "${pkg}" must be a <major>.<tranche>.<build> triple`);
  assertEqual(tauri, pkg, 'tauri.conf.json version must match package.json version');
  assertEqual(cargo, pkg, 'Cargo.toml version must match package.json version');

  // Anchor: this branch was cut as tranche/14 (SD-34 bundle cut, from
  // tranche/13's tip 571307724f after SD-33's closure PR #377 merged to
  // develop `ea2b3396f2`; the prior anchor was tranche/13 for SD-33, and
  // tranche/11 for SD-31). The tranche digit only advances when a NEW
  // tranche/N branch is cut for the next bundle, not automatically at a
  // bundle's own closure while still on the same tranche branch
  // (../SD-22/decisions.md §2 + ../SD-21/decisions.md §18's
  // tranche-promotion rule; SD-34 decisions.md §11 pins this bundle's value
  // at `0.14.<build>`). Each anchor here only holds until the next tranche
  // promotion lands — update alongside the version bump, not as a
  // follow-on fix.
  assert(pkg.startsWith('0.14.'), `version "${pkg}" must keep major=0, tranche=14 on tranche/14`);
}

// The invariant is a *relationship*, not two independent literals: the
// workflow's publish-time stamp must reuse the repo version files' own
// `<major>.<tranche>` position and contribute only the build position, which
// GITHUB_RUN_NUMBER supplies. Asserting each side's prefix separately is what
// let the defect through — the repo files moved to 0.6.0 for the tranche/6
// cut while the stamp stayed at 0.5.<build>, and because each assertion only
// looked at its own file, the pair of them blessed the mismatch. Derive the
// expected stamp from package.json so a future tranche promotion can only
// ever move both together.
function verifiesWorkflowStampTrancheAgreesWithRepoVersionFiles() {
  const pkg = readPackageJsonVersion();
  const stamp = readWorkflowStampVersion();

  const parts = pkg.split('.');
  assertEqual(parts.length, 3, `package.json version "${pkg}" must have three segments`);
  const [major, tranche] = parts;

  assertEqual(
    stamp,
    `${major}.${tranche}.${RUN_NUMBER_EXPANSION}`,
    `workflow stamp "${stamp}" must reuse package.json's major.tranche (${major}.${tranche}) ` +
      'and take its build position from GITHUB_RUN_NUMBER'
  );
}

// Companion to the relationship check: the build position is owned by the run
// counter, so no literal build number may be stamped. This closes the escape
// hatch where someone "fixes" a stamp mismatch by pinning a triple.
function verifiesWorkflowNeverStampsAHardcodedBuildNumber() {
  const hardcoded = [...readWorkflowText().matchAll(/VERSION="(\d+\.\d+\.\d+)"/g)].map((m) => m[1]);
  assertEqual(
    hardcoded.length,
    0,
    `publish-tester-release.yml must not stamp a hardcoded version triple (found: ${hardcoded.join(', ')}); ` +
      'the build position comes from GITHUB_RUN_NUMBER'
  );
}

verifiesAllThreeVersionFilesAgreeAndFollowTripleShape();
verifiesWorkflowStampTrancheAgreesWithRepoVersionFiles();
verifiesWorkflowNeverStampsAHardcodedBuildNumber();
