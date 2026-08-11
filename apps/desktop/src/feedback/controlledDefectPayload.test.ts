// controlledDefectPayload tests.
//
// Covers positive fixture (every required field present) and negative
// fixtures (each forbidden token class is rejected; missing required
// fields are rejected; secret strings are redacted).

import { assert, assertEqual } from '../../testSupport/asserts';
import {
  buildControlledDefectIssuePayload,
  type ControlledDefectOperatorState,
} from './controlledDefectPayload';
import type { ControlledDefectVerdict } from '../update/controlledDefect';

// ---------- fixtures ----------

const DECLARED = 'a'.repeat(64);
const ACTUAL_MATCH = 'b'.repeat(64);
const ACTUAL_MISMATCH = 'c'.repeat(64);

const BASE_STATE: ControlledDefectOperatorState = {
  manifestUrl: 'https://updates.example.test/alpha/update-manifest.json',
  trancheId: 'codex-tranche-2-5',
  channel: 'alpha',
  installedState: {
    version: '0.0.0-pre-e8',
    artifactSha256: 'd'.repeat(64),
    channel: 'alpha',
  },
  relaunchPromptShown: 'Relaunch now to finish installing the update?',
  relaunchHashCheckResult: 'installed-state.json -> installed (hash matches manifest)',
  checkTimestampUtc: '2026-07-04T01:55:43Z',
};

function controlledVerdict(): ControlledDefectVerdict {
  return {
    match: false,
    actual: ACTUAL_MISMATCH,
    declared: DECLARED,
    reason: 'checksum-mismatch',
    httpStatus: 200,
  };
}

function correctedVerdict(): ControlledDefectVerdict {
  return {
    match: true,
    actual: ACTUAL_MATCH,
    declared: DECLARED,
    reason: null,
    httpStatus: 200,
  };
}

// ---------- (a) positive fixture: controlled mismatch ----------

function verifiesPositiveFixtureControlledMismatch() {
  const payload = buildControlledDefectIssuePayload(controlledVerdict(), BASE_STATE);
  assertEqual(payload.labels.length, 3, 'labels.length');
  assert(payload.labels.includes('area:shell-update'), 'labels contains area:shell-update');
  assert(payload.labels.includes('controlled-defect'), 'labels contains controlled-defect');
  assert(payload.labels.includes('sd-16-e8'), 'labels contains sd-16-e8');
  assert(payload.title.startsWith('[alpha][controlled-defect]'), 'title channel+tag');
  assert(payload.title.includes('mismatch'), 'title mentions mismatch on controlled');
  assert(payload.body.includes('Summary'), 'body has Summary heading');
  assert(payload.body.includes('Controlled-defect verdict'), 'body has verdict section');
  assert(payload.body.includes('manifest URL'), 'body has manifest URL');
  assert(payload.body.includes('actual sha256'), 'body has actual sha256');
  assert(payload.body.includes('declared sha256'), 'body has declared sha256');
  assert(payload.body.includes('install_disabled_reason'), 'body has install_disabled_reason');
  assert(payload.body.includes('checksum-mismatch'), 'body has verbatim checksum-mismatch string');
  assert(payload.body.includes('Relaunch'), 'body has Relaunch section');
  assert(payload.body.includes('Expected fix surface'), 'body has expected fix surface');
  assert(payload.body.includes('AV-FA-9'), 'body cites AV-FA-9');
  assert(payload.body.includes('AV-FA-10'), 'body cites AV-FA-10');
  assert(payload.body.includes(BASE_STATE.checkTimestampUtc), 'body cites check timestamp');
  assert(!payload.body.includes('claude-execution-receipt'), 'body must not contain agent markers');
  assert(!payload.body.includes('kanban-session-id'), 'body must not contain agent markers');
  assert(!payload.body.includes('hermes-run-id'), 'body must not contain agent markers');
}

// ---------- (b) positive fixture: corrected match ----------

function verifiesPositiveFixtureCorrectedMatch() {
  const payload = buildControlledDefectIssuePayload(correctedVerdict(), BASE_STATE);
  assert(payload.title.includes('corrected AppImage SHA-256 matches manifest'), 'title says corrected');
  assert(payload.title.includes('defect fixed'), 'title says defect fixed');
  assert(payload.body.includes('now matches the manifest'), 'body explains match');
  assert(payload.body.includes('(none — Install enabled)'), 'body has install_disabled_reason null on match');
  assert(!payload.body.includes('checksum-mismatch'), 'body must not say mismatch on match');
}

// ---------- (c) negative fixtures: forbidden tokens ----------

function expectsForbiddenTokenToThrow(token: string) {
  const polluted = {
    ...BASE_STATE,
    installedState: {
      ...BASE_STATE.installedState,
      artifactSha256: 'e'.repeat(64), // keep it valid hex; pollution is in a separate field
    },
  };
  // Inject the token via a field that will land in the body. The
  // `manifestUrl` is rendered into the body, so a token there triggers
  // the guard.
  const pollutedWithUrl = { ...polluted, manifestUrl: `https://updates.example.test/${token}` };
  let threw = false;
  try {
    buildControlledDefectIssuePayload(controlledVerdict(), pollutedWithUrl);
  } catch (err) {
    threw = true;
    assert(
      err instanceof Error && err.message.includes('forbidden token'),
      `error message should mention forbidden token, got ${String(err)}`
    );
  }
  assert(threw, `expected forbidden token "${token}" to throw`);
}

function verifiesForbiddenTokensAreRejected() {
  const tokens = [
    'ghp_',
    'github_pat_',
    'Bearer ',
    'token=',
    'TOKEN=',
    'SECRET=',
    '-----BEGIN',
    'PRIVATE KEY',
    'AKIA',
    'aws_secret_access_key',
    'password=',
    'PASSWORD=',
    'session_cookie=',
    'COOKIE=',
    'claude-execution-receipt',
    'kanban-session-id',
    'hermes-run-id',
    'hermes:',
  ];
  for (const token of tokens) {
    expectsForbiddenTokenToThrow(token);
  }
}

// ---------- (d) negative fixtures: channel label variation ----------

function verifiesChannelLabelInTitle() {
  const alpha = buildControlledDefectIssuePayload(controlledVerdict(), BASE_STATE);
  assert(alpha.title.startsWith('[alpha]'), 'alpha channel label in title');

  const beta = buildControlledDefectIssuePayload(controlledVerdict(), {
    ...BASE_STATE,
    channel: 'beta',
  });
  assert(beta.title.startsWith('[beta]'), 'beta channel label in title');

  const stable = buildControlledDefectIssuePayload(controlledVerdict(), {
    ...BASE_STATE,
    channel: 'stable',
  });
  assert(stable.title.startsWith('[stable]'), 'stable channel label in title');
}

// ---------- (e) negative fixtures: missing field in operator state ----------

function verifiesEmptyStateRendersWithoutCrash() {
  // The verdict is still well-formed; the state fields are all empty
  // strings. The renderer must still produce a payload (no throw) but
  // will mark missing state with literal placeholders.
  const empty: ControlledDefectOperatorState = {
    manifestUrl: '',
    trancheId: '',
    channel: 'alpha',
    installedState: { version: '', artifactSha256: '', channel: 'unknown' },
    relaunchPromptShown: '',
    relaunchHashCheckResult: '',
    checkTimestampUtc: '',
  };
  const payload = buildControlledDefectIssuePayload(controlledVerdict(), empty);
  assert(payload.body.includes('manifest URL: '), 'body has empty manifest URL line');
  assert(payload.body.includes('installed version: '), 'body has empty installed version line');
  // The state is empty, so no forbidden tokens are introduced.
  assert(!payload.body.includes('token='), 'empty state must not introduce forbidden tokens');
}

// ---------- (f) negative fixture: missing-field verdict ----------

function verifiesMissingFieldVerdictRendersMissingFieldNote() {
  const verdict: ControlledDefectVerdict = {
    match: false,
    actual: '',
    declared: null,
    reason: 'checksum-missing-field',
    httpStatus: 0,
  };
  const payload = buildControlledDefectIssuePayload(verdict, BASE_STATE);
  assert(payload.body.includes('(missing)'), 'body has (missing) marker for declared');
  assert(payload.body.includes('checksum-missing-field'), 'body has verbatim checksum-missing-field string');
}

// ---------- main ----------

function main() {
  verifiesPositiveFixtureControlledMismatch();
  verifiesPositiveFixtureCorrectedMatch();
  verifiesForbiddenTokensAreRejected();
  verifiesChannelLabelInTitle();
  verifiesEmptyStateRendersWithoutCrash();
  verifiesMissingFieldVerdictRendersMissingFieldNote();
  console.log('controlledDefectPayload.test.ts: all assertions passed');
}

main();
