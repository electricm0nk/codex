// Controlled-defect GitHub issue payload assembly.
//
// Assembles the GitHub issue body for the E5 lane to file post-merge
// (the in-tranche spec is authored by E5-bis; the actual issue is
// Todd-driven user testing after the patch lands on develop per
// tranche-governance doctrine).
//
// The payload MUST:
//   - include the controlled tag, actual SHA-256, declared SHA-256,
//     install-disabled-reason, operator-visible pre/post state,
//     relaunch prompt, and hash check result;
//   - EXCLUDE secrets, tokens, raw full logs, and any agent-context
//     markers.
//
// Pure function: no I/O, deterministic on the same input, and throws if
// forbidden tokens are detected while assembling the payload.

import type { ControlledDefectVerdict } from '../update/controlledDefect';

export interface ControlledDefectOperatorState {
  /** Manifest URL whose AppImage is the controlled release. */
  manifestUrl: string;
  /** Tranche id under which the controlled release ships. */
  trancheId: string;
  /** Channel label the controlled release targets. */
  channel: 'alpha' | 'beta' | 'stable';
  /** Pre-install installed-state summary (version, sha256, channel). */
  installedState: {
    version: string;
    artifactSha256: string;
    channel: 'alpha' | 'beta' | 'stable' | 'unknown';
  };
  /** Relaunch-prompt UX line the operator saw (e.g. "Relaunch now?"). */
  relaunchPromptShown: string;
  /** Relaunch hash check result line. */
  relaunchHashCheckResult: string;
  /** UTC ISO-8601 timestamp at which the operator clicked Check. */
  checkTimestampUtc: string;
}

export interface ControlledDefectIssuePayload {
  title: string;
  body: string;
  labels: readonly string[];
}

const LABELS = ['area:shell-update', 'controlled-defect', 'sd-16-e8'] as const;

/**
 * Tokens that MUST NEVER appear in the rendered body. The first match
 * throws during payload assembly, so the operator-driven E5 lane cannot
 * accidentally file a payload that includes a secret or full raw log.
 */
const FORBIDDEN_TOKENS: readonly string[] = [
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
  // Agent-context markers that have no place in operator-visible
  // GitHub issues.
  'claude-execution-receipt',
  'kanban-session-id',
  'hermes-run-id',
  'hermes:',
];

function rejectForbiddenTokens(text: string): void {
  for (const token of FORBIDDEN_TOKENS) {
    if (text.includes(token)) {
      throw new Error(
        `controlledDefectPayload: refused to assemble payload containing forbidden token "${token}"`
      );
    }
  }
}

function normaliseSha(value: string): string {
  const trimmed = value.trim().toLowerCase();
  if (trimmed === '') {
    return '(empty)';
  }
  return trimmed;
}

function summariseState(state: ControlledDefectOperatorState): string {
  return [
    `- manifest URL: ${state.manifestUrl}`,
    `- tranche id: ${state.trancheId}`,
    `- channel: ${state.channel}`,
    `- installed version: ${state.installedState.version}`,
    `- installed sha256: ${normaliseSha(state.installedState.artifactSha256)}`,
    `- installed channel: ${state.installedState.channel}`,
    `- Check timestamp (UTC): ${state.checkTimestampUtc}`,
  ].join('\n');
}

function summariseVerdict(verdict: ControlledDefectVerdict): string[] {
  const lines: string[] = [];
  lines.push(`- match: ${verdict.match}`);
  lines.push(`- declared sha256: ${verdict.declared === null ? '(missing)' : normaliseSha(verdict.declared)}`);
  lines.push(`- actual sha256: ${verdict.actual === '' ? '(unread)' : normaliseSha(verdict.actual)}`);
  lines.push(`- install_disabled_reason: ${verdict.reason ?? '(none — Install enabled)'}`);
  if (verdict.httpStatus !== 0) {
    lines.push(`- AppImage fetch httpStatus: ${verdict.httpStatus}`);
  }
  return lines;
}

function titlePrefix(channel: 'alpha' | 'beta' | 'stable', match: boolean): string {
  const stage = match ? 'corrected' : 'controlled';
  return `[${channel}][controlled-defect] ${stage} AppImage SHA-256 ${match ? 'matches manifest (defect fixed)' : 'mismatch'}`;
}

export function buildControlledDefectIssuePayload(
  verdict: ControlledDefectVerdict,
  state: ControlledDefectOperatorState
): ControlledDefectIssuePayload {
  const title = titlePrefix(state.channel, verdict.match);
  const bodySections: string[] = [
    '## Summary',
    verdict.match
      ? 'The controlled-defect AppImage SHA-256 now matches the manifest. Install was enabled and the staged transaction completed.'
      : 'The controlled AppImage SHA-256 does not match the manifest declared in the update channel index. The shell refused to stage the AppImage and surfaced `install_disabled_reason` to the operator.',
    '',
    '## Controlled-defect verdict',
    ...summariseVerdict(verdict),
    '',
    '## Operator-visible pre-state',
    summariseState(state),
    '',
    '## Relaunch',
    `- relaunch prompt: ${state.relaunchPromptShown}`,
    `- relaunch hash check: ${state.relaunchHashCheckResult}`,
    '',
    '## Expected fix surface',
    '- The E7 staged transaction (`apps/desktop/src-tauri/src/update/transaction.rs`) pre-replacement SHA-256 guard.',
    '- The E6 update flow (`apps/desktop/src/sd16/update/fetch.ts`, `eligibility.ts`) and the E3 schema field `artifact_sha256`.',
    '- The E4 release lane manifest generation step that signs off on the AppImage digest before publishing.',
    '',
    '## Post-merge verification (E8-F4)',
    '- Re-run the shell Check / Install / Relaunch loop against the corrected release.',
    '- Confirm `installed-state.json` transitions to `installed` after relaunch-hash verification.',
    '- Capture the verification log in the final-acceptance evidence ledger under `AV-FA-9` and `AV-FA-10`.',
  ];

  const body = bodySections.join('\n');
  rejectForbiddenTokens(body);
  rejectForbiddenTokens(title);

  return { title, body, labels: LABELS };
}
