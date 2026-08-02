import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { assertEqual, assert } from '../testSupport/asserts';

// SD-22 E8.27: package.json, tauri.conf.json, and Cargo.toml must all carry
// the same `<major>.<tranche>.<build>` version triple, anchored to tranche/7
// (per decisions.md §2 + epic-breakdown.md criterion 27). Mirrors SD-21
// E5.25's buildVersionTriple.test.ts and the SD-21 counterpart at
// src/sd21/buildVersionTriple.test.ts (which is anchored one tranche ahead
// as the canonical source), re-anchored from tranche=6 to tranche=7 to
// match the SD-27 bundle closure that landed in this commit (the v0.6.120
// publish at 5c432a1b missed this advance).

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

  // Anchor: tranche moves to 7 (SD-27 bundle closure on tranche/7, which
  // is the same advance the v0.6.120 publish at 5c432a1b missed). Major
  // stays 0 until first main-publish, per decisions.md §2's build-version
  // responsibility note. The tranche digit only advances when a new
  // tranche/N branch is cut for the next bundle — not automatically at a
  // bundle's own closure while still on the same tranche branch (an
  // earlier Epic 7 closure-epilogue cycle bumped this to tranche=6 in
  // error and it was reverted; this time the bump is intentional, driven
  // by the SD-27 closure landing on tranche/7).
  assert(pkg.startsWith('0.7.'), `version "${pkg}" must move to major=0, tranche=7 on tranche/7`);
}

verifiesAllThreeVersionFilesAgreeAndFollowTripleShape();
