import { assert, assertEqual } from '../../testSupport/asserts';
import {
  GITHUB_ISSUE_OWNER,
  GITHUB_ISSUE_REPO,
  runBrowserHandoff,
  type BrowserHandoffDraft,
  type InvokeLike,
} from './browserHandoff';

const DRAFT: BrowserHandoffDraft = {
  title: 'Shell defect: preview crashes on load',
  markdownBody: '## Observed\n\nCrash.\n',
  labels: ['codex-shell-defect', 'channel:alpha'],
};

const BUILT_URL =
  'https://github.com/electricm0nk/codex/issues/new?title=Shell%20defect&body=x&labels=codex-shell-defect';

function runtimePresent(): boolean {
  return true;
}

async function testHappyPathConfirmsOnlyAfterRealOpen() {
  const calls: Array<{ cmd: string; args: Record<string, unknown> }> = [];
  const invokeImpl: InvokeLike = async (cmd, args) => {
    calls.push({ cmd, args });
    return { url: BUILT_URL, opened: true };
  };
  const outcome = await runBrowserHandoff(DRAFT, { invokeImpl, hasRuntimeImpl: runtimePresent });

  assertEqual(outcome.state.kind, 'confirmed', 'successful OS open confirms');
  assertEqual(outcome.ui.status, 'confirmed', 'ui state confirmed');
  assertEqual(outcome.ui.canClaimSubmitted, true, 'browser-open claim is allowed');
  assertEqual(outcome.ui.url, BUILT_URL, 'confirmed url surfaced');
  assertEqual(outcome.manualUrl, BUILT_URL, 'manual url mirrors the opened url');

  assertEqual(calls.length, 1, 'exactly one IPC call');
  assertEqual(calls[0].cmd, 'handoff_defect_report_to_browser', 'command name');
  const req = calls[0].args.req as Record<string, unknown>;
  assertEqual(req.owner, GITHUB_ISSUE_OWNER, 'owner pinned');
  assertEqual(req.repo, GITHUB_ISSUE_REPO, 'repo pinned');
  assertEqual(req.title, DRAFT.title, 'title forwarded');
  assertEqual(req.body, DRAFT.markdownBody, 'body forwarded verbatim');
  assert(Array.isArray(req.labels) && (req.labels as string[]).length === 2, 'labels forwarded');
}

async function testNoRuntimeFailsClosedWithoutIpc() {
  let invoked = false;
  const invokeImpl: InvokeLike = async () => {
    invoked = true;
    return { url: BUILT_URL, opened: true };
  };
  const outcome = await runBrowserHandoff(DRAFT, {
    invokeImpl,
    hasRuntimeImpl: () => false,
  });

  assertEqual(outcome.state.kind, 'failed', 'no runtime fails closed');
  assertEqual(outcome.ui.canClaimSubmitted, false, 'no claim without runtime');
  assert(
    (outcome.ui.reason ?? '').includes('desktop runtime'),
    `reason names the missing runtime, got: ${outcome.ui.reason}`
  );
  assertEqual(outcome.manualUrl, null, 'no url was ever built');
  assert(!invoked, 'IPC must not run without the desktop runtime');
}

async function testOpenFailureSurfacesManualUrlFallback() {
  const invokeImpl: InvokeLike = async () => {
    // Tauri rejects a command Result::Err with the serialized error payload.
    throw { kind: 'browser-open-failed', reason: 'xdg-open missing', url: BUILT_URL };
  };
  const outcome = await runBrowserHandoff(DRAFT, { invokeImpl, hasRuntimeImpl: runtimePresent });

  assertEqual(outcome.state.kind, 'failed', 'failed OS open never confirms');
  assertEqual(outcome.ui.canClaimSubmitted, false, 'no claim on failed open');
  assert(
    (outcome.ui.reason ?? '').includes('xdg-open missing'),
    `reason carries the OS failure, got: ${outcome.ui.reason}`
  );
  assertEqual(outcome.manualUrl, BUILT_URL, 'validated url preserved for manual filing');
}

async function testValidationRejectionFailsWithoutManualUrl() {
  const invokeImpl: InvokeLike = async () => {
    throw { kind: 'empty-title' };
  };
  const outcome = await runBrowserHandoff(DRAFT, { invokeImpl, hasRuntimeImpl: runtimePresent });

  assertEqual(outcome.state.kind, 'failed', 'validation rejection fails');
  assert(
    (outcome.ui.reason ?? '').includes('empty-title'),
    `reason names the validation failure, got: ${outcome.ui.reason}`
  );
  assertEqual(outcome.manualUrl, null, 'no validated url exists to fall back to');
}

async function testEmptyUrlResponseNeverConfirms() {
  const invokeImpl: InvokeLike = async () => ({ url: '', opened: true });
  const outcome = await runBrowserHandoff(DRAFT, { invokeImpl, hasRuntimeImpl: runtimePresent });

  assertEqual(outcome.state.kind, 'failed', 'empty url must not confirm');
  assertEqual(outcome.ui.canClaimSubmitted, false, 'no claim on empty url');
}

async function testOpenedFalseResponseNeverConfirms() {
  const invokeImpl: InvokeLike = async () => ({ url: BUILT_URL, opened: false });
  const outcome = await runBrowserHandoff(DRAFT, { invokeImpl, hasRuntimeImpl: runtimePresent });

  assertEqual(outcome.state.kind, 'failed', 'opened=false must not confirm');
  assertEqual(outcome.manualUrl, BUILT_URL, 'url still preserved for manual filing');
}

async function main() {
  await testHappyPathConfirmsOnlyAfterRealOpen();
  await testNoRuntimeFailsClosedWithoutIpc();
  await testOpenFailureSurfacesManualUrlFallback();
  await testValidationRejectionFailsWithoutManualUrl();
  await testEmptyUrlResponseNeverConfirms();
  await testOpenedFalseResponseNeverConfirms();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
