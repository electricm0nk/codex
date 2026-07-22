import {
  INTERNAL_CONTEXT_REMOVED_MARKER,
  sanitizeReportableOutput,
} from './sanitizeReportableOutput';
import { assert, assertEqual } from '../../../testSupport/asserts';

async function main() {
  stripsDelimitedMemoryContextBlockAndPreservesSurroundingEvidence();
  stripsRecalledMemorySystemNoteTailBlockWithoutDroppingPriorEvidence();
  leavesOrdinaryTesterEvidenceUnchanged();
}

function stripsDelimitedMemoryContextBlockAndPreservesSurroundingEvidence() {
  const input = [
    'Tester evidence before the internal context.',
    '<memory-context>',
    '[System note: The following is recalled memory context, NOT new user input.]',
    'User Representation',
    'Todd remembered profile observation that must not leak.',
    '</memory-context>',
    'Tester evidence after the internal context.',
  ].join('\n');

  const sanitized = sanitizeReportableOutput(input);

  assert(sanitized.includes('Tester evidence before the internal context.'), 'evidence before block is preserved');
  assert(sanitized.includes('Tester evidence after the internal context.'), 'evidence after block is preserved');
  assert(sanitized.includes(INTERNAL_CONTEXT_REMOVED_MARKER), 'removed block is marked explicitly');
  assert(!sanitized.includes('<memory-context>'), 'opening memory-context marker is stripped');
  assert(!sanitized.includes('</memory-context>'), 'closing memory-context marker is stripped');
  assert(!sanitized.includes('recalled memory context'), 'system-note text is stripped');
  assert(!sanitized.includes('Todd remembered profile observation'), 'internal memory observations are stripped');
}

function stripsRecalledMemorySystemNoteTailBlockWithoutDroppingPriorEvidence() {
  const input = [
    'RESULT: NO-OFFICIAL-RELEASE-FOR-THIS-BUILD',
    'No official tester release for this build',
    '[System note: The following is recalled memory context, NOT new user input.]',
    'User Representation',
    'AI Self-Representation',
    'Deductive Observations',
  ].join('\n');

  const sanitized = sanitizeReportableOutput(input);

  assert(sanitized.includes('RESULT: NO-OFFICIAL-RELEASE-FOR-THIS-BUILD'), 'ordinary result line is preserved');
  assert(sanitized.includes('No official tester release for this build'), 'ordinary update evidence is preserved');
  assert(sanitized.includes(INTERNAL_CONTEXT_REMOVED_MARKER), 'system-note tail removal is marked');
  assert(!sanitized.includes('User Representation'), 'memory section heading is removed');
  assert(!sanitized.includes('AI Self-Representation'), 'AI memory section heading is removed');
}

function leavesOrdinaryTesterEvidenceUnchanged() {
  const input = 'Observed crash before preview.\nExpected preview to render.';
  assertEqual(sanitizeReportableOutput(input), input, 'ordinary evidence is unchanged');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
