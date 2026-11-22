// Index source URL resolver.
//
// Resolves the canonical raw URL for a channel's index file on the
// protected `update-index` branch on origin. The shell's Install
// enabler uses this function to fetch the channel-index pointer; the
// release lane (E4) writes to that branch, and the URL shape is locked
// at `AV-SCH-7` ("Channel index fetched from update-index, not GitHub
// Release scanning").
//
// This function is intentionally a pure string shape: no `fetch`, no
// `gh api`, no GitHub Releases. The `fetch.ts` module (E6-F3a) consumes
// this URL via its `defaultFetchImpl` indirection. Keeping the URL
// helper pure means the TS lane can pin the URL shape in isolation.
//
// This module also accepts the E6 surface's `ChannelLabel` shape so
// callers can pass `'alpha' | 'beta' | 'stable'` without casting.

const REPO_OWNER = 'electricm0nk';
const REPO_NAME = 'codex';
const UPDATE_INDEX_BRANCH = 'update-index';
const CHANNELS_SUBDIR = 'channels';

const RAW_BASE_URL = `https://raw.githubusercontent.com/${REPO_OWNER}/${REPO_NAME}/${UPDATE_INDEX_BRANCH}/${CHANNELS_SUBDIR}`;

const SUPPORTED_CHANNELS = ['alpha', 'beta', 'stable'] as const;
export type ChannelLabel = (typeof SUPPORTED_CHANNELS)[number];

function isChannelLabel(value: string): value is ChannelLabel {
  return (SUPPORTED_CHANNELS as ReadonlyArray<string>).includes(value);
}

/**
 * Returns the raw `update-index` URL for the requested channel's index
 * file. Throws if the channel label is not one of the three supported
 * values, because constructing a URL with an unsupported label would
 * silently 404 at fetch time and confuse the UI's "why did Check
 * fail?" diagnostic.
 */
export function channelIndexFetchUrl(channel: string): string {
  if (!isChannelLabel(channel)) {
    throw new Error(
      `update/indexSource: unsupported channel "${channel}". ` +
        `Supported: ${SUPPORTED_CHANNELS.join(', ')}.`,
    );
  }
  return `${RAW_BASE_URL}/${channel}.json`;
}

/** Exported so the test lane can pin the URL shape without depending on the fetcher. */
export const RAW_UPDATE_INDEX_CHANNELS_BASE_URL = RAW_BASE_URL;
