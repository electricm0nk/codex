// Runs every self-executing src/**/*.test.ts script via tsx and reports a
// summary. Each test file exits non-zero on its first failed assertion.
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

const testFiles = collectTestFiles(srcRoot);
if (testFiles.length === 0) {
  console.error('No *.test.ts files found under src/.');
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
