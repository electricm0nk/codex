import { assert, assertEqual } from '../testSupport/asserts';
import type { ReportOutcome, ReportRequest } from '../boundary/sendReport';
import type { BrowserHandoffDraft, BrowserHandoffOutcome } from './browserHandoff';
import { bodyWithHandle, routeReport, type ReportRouteDeps } from './reportRoute';

const DRAFT: BrowserHandoffDraft = {
  title: 'Sheet crashes on load',
  markdownBody: '## Observed\n\nCrash.\n',
  labels: ['codex-shell-defect'],
};

const OPENED: BrowserHandoffOutcome = {
  state: { kind: 'confirmed', url: 'https://github.com/x/y/issues/new' },
  ui: { status: 'confirmed', canClaimSubmitted: true, url: 'https://github.com/x/y/issues/new' },
  manualUrl: 'https://github.com/x/y/issues/new',
};

const FAILED: BrowserHandoffOutcome = {
  state: { kind: 'failed', reason: 'open-failed: no browser' },
  ui: { status: 'failed', canClaimSubmitted: false, reason: 'open-failed: no browser' },
  manualUrl: null,
};

function fakeDeps(handoff: BrowserHandoffOutcome, available = true) {
  const sent: ReportRequest[] = [];
  const handedOff: BrowserHandoffDraft[] = [];
  const deps: ReportRouteDeps = {
    availability: async () => ({ available, reason: available ? null : 'This build has no report key.' }),
    send: async (request): Promise<ReportOutcome> => {
      sent.push(request);
      return { outcome: 'sent', message: 'sent for review' };
    },
    handoff: async (draft) => {
      handedOff.push(draft);
      return handoff;
    },
  };
  return { deps, sent, handedOff };
}

async function testAnonymousWithHandleCallsTheApiWithHandleFirst() {
  const { deps, sent, handedOff } = fakeDeps(OPENED);
  const outcome = await routeReport({ kind: 'bug', anonymous: true, handle: 'Gus', draft: DRAFT }, deps);
  assertEqual(sent.length, 1, 'Anonymous on calls the report API once');
  assertEqual(handedOff.length, 0, 'Anonymous on never opens the browser');
  assertEqual(sent[0].body.split('\n')[0], 'Gus', "body's first line is the handle");
  assertEqual(sent[0].kind, 'bug', 'kind passed through');
  assertEqual(sent[0].title, DRAFT.title, 'title passed through');
  assert(outcome.route === 'api' && outcome.sent && outcome.message === 'sent for review', 'success text is "sent for review"');
}

async function testAnonymousWithoutHandleSendsTheBodyUnchanged() {
  const { deps, sent } = fakeDeps(OPENED);
  await routeReport({ kind: 'enhancement', anonymous: true, handle: '   ', draft: DRAFT }, deps);
  assertEqual(sent[0].body, DRAFT.markdownBody, 'a blank Handle adds no line');
  assertEqual(bodyWithHandle('Gus\nEvil', 'b'), 'Gus Evil\nb', 'a Handle stays one line');
}

async function testAnonymousUnavailableStatesTheReasonAndSendsNothing() {
  const { deps, sent, handedOff } = fakeDeps(OPENED, false);
  const outcome = await routeReport({ kind: 'bug', anonymous: true, handle: 'Gus', draft: DRAFT }, deps);
  assertEqual(sent.length + handedOff.length, 0, 'no key: nothing is sent and nothing falls back');
  assert(outcome.route === 'api' && !outcome.sent && outcome.message.includes('no report key'), 'the reason is stated');
}

async function testAnonymousOffHandsOffToTheBrowser() {
  const { deps, sent, handedOff } = fakeDeps(OPENED);
  const outcome = await routeReport({ kind: 'bug', anonymous: false, handle: 'Gus', draft: DRAFT }, deps);
  assertEqual(handedOff.length, 1, 'Anonymous off opens the browser handoff');
  assertEqual(handedOff[0], DRAFT, 'the handoff gets the draft unchanged (no Handle line)');
  assertEqual(sent.length, 0, 'Anonymous off never calls the API');
  assert(outcome.route === 'browser' && outcome.githubUnavailable === null, 'no unavailable message on success');
}

async function testGithubUnavailableShowsAMessageAndNoApiCall() {
  const { deps, sent } = fakeDeps(FAILED);
  const outcome = await routeReport({ kind: 'bug', anonymous: false, handle: '', draft: DRAFT }, deps);
  assertEqual(sent.length, 0, 'GitHub unavailable: no silent fallback to the API');
  assert(
    outcome.route === 'browser' && outcome.githubUnavailable !== null && outcome.githubUnavailable.includes('turn on Anonymous'),
    'GitHub unavailable: the message tells the user to turn on Anonymous',
  );
}

async function main() {
  await testAnonymousWithHandleCallsTheApiWithHandleFirst();
  await testAnonymousWithoutHandleSendsTheBodyUnchanged();
  await testAnonymousUnavailableStatesTheReasonAndSendsNothing();
  await testAnonymousOffHandsOffToTheBrowser();
  await testGithubUnavailableShowsAMessageAndNoApiCall();
}

main().catch((error: unknown) => {
  console.error(error);
  process.exit(1);
});
