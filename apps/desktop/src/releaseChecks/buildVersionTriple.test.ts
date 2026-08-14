import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { assertEqual, assert } from '../testSupport/asserts';

// SD-28 Epic 11 (Build Version Numbering): package.json, tauri.conf.json, and
// Cargo.toml must all carry the same `<major>.<tranche>.<build>` version triple,
// anchored to tranche/8 per Decision 15 (operator-pinned 2026-08-01). Mirrors
// SD-21 E5.25's buildVersionTriple.test.ts, updated from tranche/6 anchor to
// tranche/8 per the new branch cut.

const appRoot = join(dirname(fileURLToPath(import.meta.url)), '..', '..');

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

function verifiesAllThreeVersionFilesAgreeAndFollowTripleShape() {
  const pkg = readPackageJsonVersion();
  const tauri = readTauriConfVersion();
  const cargo = readCargoTomlVersion();

  assert(TRIPLE_RE.test(pkg), `package.json version "${pkg}" must be a <major>.<tranche>.<build> triple`);
  assertEqual(tauri, pkg, 'tauri.conf.json version must match package.json version');
  assertEqual(cargo, pkg, 'Cargo.toml version must match package.json version');

  // Anchor: tranche moves to 9 (tranche/9 is the active branch for this
  // bundle, cut per operator directive 2026-08-01) and major stays 0 until
  // first main-publish, per that bundle's decisions.md §14 build-version
  // specification (`0.9.<build>`). The tranche digit only advances when a new
  // tranche/N branch is cut for the next bundle — not automatically at a
  // bundle's own closure while still on the same tranche branch.
  assert(pkg.startsWith('0.9.'), `version "${pkg}" must keep major=0, tranche=9 on tranche/9`);
}

verifiesAllThreeVersionFilesAgreeAndFollowTripleShape();
