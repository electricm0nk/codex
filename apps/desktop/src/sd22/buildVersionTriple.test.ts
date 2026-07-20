import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { assertEqual, assert } from '../testSupport/asserts';

// SD-22 E8.27: package.json, tauri.conf.json, and Cargo.toml must all carry
// the same `<major>.<tranche>.<build>` version triple, anchored to tranche/5
// (per decisions.md §2 + epic-breakdown.md criterion 27). Mirrors SD-21
// E5.25's buildVersionTriple.test.ts, re-anchored from tranche=4 to tranche=5.

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

  // Anchor: tranche-base moved to 6 on SD-22's closure-epilogue tranche
  // promotion (Epic 7, criterion 26; build resets to 0 on promotion) and
  // major stays 0 until first main-publish, per decisions.md §2 and
  // ../SD-21/decisions.md §18's tranche-promotion rule.
  assert(pkg.startsWith('0.6.'), `version "${pkg}" must move to major=0, tranche=6 on tranche/5 post-promotion`);
}

verifiesAllThreeVersionFilesAgreeAndFollowTripleShape();
