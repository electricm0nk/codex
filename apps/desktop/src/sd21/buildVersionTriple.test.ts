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

function readWorkflowStampVersion(): string {
  const text = readFileSync(
    join(repoRoot, '.github', 'workflows', 'publish-tester-release.yml'),
    'utf8'
  );
  const match = text.match(/VERSION="([^"]*\$\{GITHUB_RUN_NUMBER\}[^"]*)"/);
  assert(match !== null, 'publish-tester-release.yml must stamp a VERSION containing GITHUB_RUN_NUMBER');
  return match![1];
}

function verifiesAllThreeVersionFilesAgreeAndFollowTripleShape() {
  const pkg = readPackageJsonVersion();
  const tauri = readTauriConfVersion();
  const cargo = readCargoTomlVersion();

  assert(TRIPLE_RE.test(pkg), `package.json version "${pkg}" must be a <major>.<tranche>.<build> triple`);
  assertEqual(tauri, pkg, 'tauri.conf.json version must match package.json version');
  assertEqual(cargo, pkg, 'Cargo.toml version must match package.json version');

  // Anchor: this branch was promoted from tranche/4-1 to tranche/5 (SD-22
  // E8.27) and stays at tranche-base 5 while tranche/5 remains the active
  // branch — the tranche digit only advances when a new tranche/N branch is
  // cut for the next bundle, not automatically at a bundle's own closure
  // (../SD-22/decisions.md §2 + ../SD-21/decisions.md §18's tranche-promotion
  // rule; SD-22's Epic 7 closure-epilogue cycle initially bumped this to
  // tranche=6 in error and it was reverted). Each anchor here only holds
  // until the next tranche promotion lands — update alongside the version
  // bump, not as a follow-on fix.
  assert(pkg.startsWith('0.5.'), `version "${pkg}" must keep major=0, tranche=5 on tranche/5`);
}

function verifiesWorkflowStampMatchesTripleShapeNotLegacyScheme() {
  const stamp = readWorkflowStampVersion();
  assert(
    stamp.startsWith('0.5.'),
    `workflow stamp "${stamp}" must use the current 0.5.<build> shape, not a stale or legacy scheme`
  );
}

verifiesAllThreeVersionFilesAgreeAndFollowTripleShape();
verifiesWorkflowStampMatchesTripleShapeNotLegacyScheme();
