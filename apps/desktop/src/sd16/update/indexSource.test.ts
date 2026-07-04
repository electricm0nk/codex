// SD16-E3-F3b-BACKFILL — indexSource URL-shape tests.
//
// AV-SCH-7 (channel index fetched from update-index branch, not GitHub
// Release scanning) is enforced by this module's URL helper. These
// tests pin the URL shape so any drift in the contract surfaces as a
// test failure rather than a silent misrouting at fetch time.
//
// The mirror of the URL shape also exists in the E6 surface
// (`apps/desktop/src/sd16/update/fetch.ts`). Both must agree; this
// test exercises the F3b half directly. The E6 surface is out of
// scope per the F3b write scope but the dual assertion here makes a
// downstream merge that drops the URL helper inspectable.

import { channelIndexFetchUrl, RAW_UPDATE_INDEX_CHANNELS_BASE_URL } from './indexSource';

function assert(cond: unknown, msg: string): asserts cond {
  if (!cond) throw new Error(`assertion failed: ${msg}`);
}

const EXPECTED_BASE =
  'https://raw.githubusercontent.com/electricm0nk/codex/update-index/channels';

// ---------- AV-SCH-7 ----------

export function test_av_sch_7_index_source_uses_update_index_branch_alpha(): void {
  assert(
    channelIndexFetchUrl('alpha') ===
      'https://raw.githubusercontent.com/electricm0nk/codex/update-index/channels/alpha.json',
    'alpha channel URL must be canonical',
  );
}

export function test_av_sch_7_index_source_uses_update_index_branch_beta(): void {
  assert(
    channelIndexFetchUrl('beta') ===
      'https://raw.githubusercontent.com/electricm0nk/codex/update-index/channels/beta.json',
    'beta channel URL must be canonical',
  );
}

export function test_av_sch_7_index_source_uses_update_index_branch_stable(): void {
  assert(
    channelIndexFetchUrl('stable') ===
      'https://raw.githubusercontent.com/electricm0nk/codex/update-index/channels/stable.json',
    'stable channel URL must be canonical',
  );
}

// ---------- URL base constant is exported ----------

export function test_base_url_constant_is_canonical(): void {
  assert(
    RAW_UPDATE_INDEX_CHANNELS_BASE_URL === EXPECTED_BASE,
    `RAW_UPDATE_INDEX_CHANNELS_BASE_URL must equal "${EXPECTED_BASE}" (got "${RAW_UPDATE_INDEX_CHANNELS_BASE_URL}")`,
  );
}

// ---------- Negative: unsupported channel throws ----------

export function test_unsupported_channel_throws_loudly(): void {
  let threw = false;
  try {
    channelIndexFetchUrl('gamma');
  } catch {
    threw = true;
  }
  assert(threw, 'unsupported channel label must throw, not silently 404 at fetch time');
}

// ---------- Sanity: no GitHub Releases API URL leaks into the helper ----------

export function test_no_github_releases_api_url_in_helper(): void {
  // AV-SCH-7 forbids "GitHub Release scanning" as the channel-index
  // source. The helper MUST NOT carry `/repos/.../releases` shapes.
  for (const ch of ['alpha', 'beta', 'stable'] as const) {
    const url = channelIndexFetchUrl(ch);
    assert(
      !url.includes('/repos/'),
      `channel URL must not invoke the GitHub Releases API: ${url}`,
    );
    assert(
      !url.includes('/releases/'),
      `channel URL must not contain a /releases path segment: ${url}`,
    );
    assert(
      url.includes('update-index'),
      `channel URL must point at the update-index branch: ${url}`,
    );
    assert(
      url.endsWith('.json'),
      `channel URL must end with .json so AV-SCH-8's regex passes: ${url}`,
    );
  }
}

function run(): void {
  test_av_sch_7_index_source_uses_update_index_branch_alpha();
  test_av_sch_7_index_source_uses_update_index_branch_beta();
  test_av_sch_7_index_source_uses_update_index_branch_stable();
  test_base_url_constant_is_canonical();
  test_unsupported_channel_throws_loudly();
  test_no_github_releases_api_url_in_helper();
  console.log('indexSource.test.ts: 6/6 AV-SCH-7 assertions passed');
}

run();
