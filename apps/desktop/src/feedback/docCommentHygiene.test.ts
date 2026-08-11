/**
 * Doc-comment hygiene sweep for feedback/*: no bare SD-16/SD16 tranche
 * tags and no AV-PAY-N audit-ID tags may remain in source comments in this
 * directory (SD-21 Epic 1, criterion E1.3). These files describe behaviour
 * directly in prose instead of pointing at an external tranche/audit-ID
 * ledger.
 *
 * The `sd-16-e8` GitHub *label* string in controlledDefectPayload.ts /
 * .test.ts is real external taxonomy (an actual label applied to filed
 * issues), not a doc-comment tag, so it is deliberately excluded from this
 * sweep rather than rewritten. Likewise, a literal source
 * path mentioned in a rendered issue body is a real directory reference
 * (the directory is now named `feedback/` for what it does), not a
 * tranche/audit-ID tag and is not
 * flagged.
 */
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { assert } from '../../testSupport/asserts';

const here = dirname(fileURLToPath(import.meta.url));

const FILES = [
  'controlledDefectPayload.ts',
  'controlledDefectPayload.test.ts',
  'submissionState.ts',
  'submissionState.test.ts',
  'submissionUiState.ts',
  'submissionUiState.test.ts',
];

const FORBIDDEN_PATTERNS: readonly RegExp[] = [/\bSD-?16\b(?!\/)/i, /\bAV-PAY-\d+\b/];

function verifiesNoTrancheOrAuditIdTagsRemainInComments() {
  for (const name of FILES) {
    const raw = readFileSync(join(here, name), 'utf8');
    // The `sd-16-e8` GitHub label literal is real data, not a doc-comment
    // tag — strip it before scanning so it can't produce a false positive.
    const text = raw.replace(/sd-16-e8/gi, '');
    for (const pattern of FORBIDDEN_PATTERNS) {
      assert(
        !pattern.test(text),
        `${name}: found forbidden doc-comment tag matching ${pattern} — rewrite as plain prose`,
      );
    }
  }
}

verifiesNoTrancheOrAuditIdTagsRemainInComments();

console.log('docCommentHygiene.test.ts: 6/6 feedback/ files clean of SD-16/AV-PAY-N tags');
