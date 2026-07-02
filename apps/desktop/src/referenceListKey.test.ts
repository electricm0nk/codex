import { createReferenceListKey } from './referenceListKey';
import { assertEqual } from './testSupport/asserts';

function assertNotEqual<T>(actual: T, expected: T, message: string) {
  if (actual === expected) {
    throw new Error(`${message}: both values were ${String(actual)}`);
  }
}

function main() {
  const firstKey = createReferenceListKey(
    'rule:guard-stance-ac',
    'rule:guard-stance-ac'
  );
  const duplicateLabelKey = createReferenceListKey(
    'rule:guard-stance-ac',
    'rule:guard-stance-ac:variant'
  );

  assertEqual(
    firstKey,
    'rule:guard-stance-ac-rule:guard-stance-ac',
    'first key includes the discriminator'
  );
  assertNotEqual(firstKey, duplicateLabelKey, 'duplicate labels get unique keys');
}

main();
