import { createReferenceListKey } from './referenceListKey';

function assertEqual<T>(actual: T, expected: T, message: string) {
  if (actual !== expected) {
    throw new Error(`${message}: expected ${String(expected)}, got ${String(actual)}`);
  }
}

function assertNotEqual<T>(actual: T, expected: T, message: string) {
  if (actual === expected) {
    throw new Error(`${message}: both values were ${String(actual)}`);
  }
}

function main() {
  const firstKey = createReferenceListKey('rule:guard-stance-ac', 0);
  const duplicateLabelKey = createReferenceListKey('rule:guard-stance-ac', 1);

  assertEqual(firstKey, 'rule:guard-stance-ac-0', 'first key includes the index');
  assertNotEqual(firstKey, duplicateLabelKey, 'duplicate labels get unique keys');
}

main();
