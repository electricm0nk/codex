// Runs every self-executing src/**/*.test.ts script (via tsx) and every
// scripts/ui-smoke/lib/*.test.mjs script (tsx runs plain .mjs too) and reports
// a summary. Each test file exits non-zero on its first failed assertion.
import { spawnSync } from 'node:child_process';
import { readdirSync } from 'node:fs';
import { join, dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const appRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const srcRoot = join(appRoot, 'src');

function collectTestFiles(dir) {
  const files = [];
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const full = join(dir, entry.name);
    if (entry.isDirectory()) {
      files.push(...collectTestFiles(full));
    } else if (entry.name.endsWith('.test.ts')) {
      files.push(full);
    }
  }
  return files.sort();
}

// The ui-smoke harness's own self-executing Node tests (scripts/ui-smoke/lib/
// *.test.mjs) run here too -- including the app-data isolation guard that
// keeps a smoke run off the operator's real character store (SD-36 F6d).
const harnessLibRoot = join(appRoot, 'scripts', 'ui-smoke', 'lib');
const harnessTestFiles = readdirSync(harnessLibRoot)
  .filter((name) => name.endsWith('.test.mjs'))
  .sort()
  .map((name) => join(harnessLibRoot, name));

const testFiles = [...collectTestFiles(srcRoot), ...harnessTestFiles];
if (testFiles.length === 0) {
  console.error('No test files found under src/ or scripts/ui-smoke/lib/.');
  process.exit(1);
}

const tsxBin = join(appRoot, 'node_modules', '.bin', 'tsx');
const failures = [];
for (const file of testFiles) {
  const result = spawnSync(tsxBin, [file], { stdio: 'inherit', cwd: appRoot });
  const label = file.slice(appRoot.length + 1);
  if (result.status === 0) {
    console.log(`PASS ${label}`);
  } else {
    console.error(`FAIL ${label}`);
    failures.push(label);
  }
}

console.log(`\n${testFiles.length - failures.length}/${testFiles.length} test files passed.`);
if (failures.length > 0) {
  process.exit(1);
}
