/**
 * Shared assertion helpers for the self-executing *.test.ts scripts.
 *
 * Every test file previously carried its own identical copy of these; they
 * live here so the failure format stays uniform and the copies cannot drift.
 */

export function assertEqual<T>(actual: T, expected: T, message: string) {
  if (actual !== expected) {
    throw new Error(`${message}: expected ${String(expected)}, got ${String(actual)}`);
  }
}

export function assert(condition: boolean, message: string) {
  if (!condition) {
    throw new Error(message);
  }
}
