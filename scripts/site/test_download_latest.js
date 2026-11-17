// Self-check for site/download-latest.js asset picking. Run: node scripts/site/test_download_latest.js
const assert = require('node:assert');
const { assetUrl } = require('../../site/download-latest.js');
const a = (n) => ({ name: n, browser_download_url: 'u/' + n });
const releases = [
  { draft: true, assets: [a('Codex_9_amd64.AppImage')] },
  { draft: false, assets: [a('Codex_2_amd64.AppImage'), a('Codex_2_amd64.AppImage.sig')] },
  { draft: false, assets: [a('Codex_1_amd64.AppImage'), a('Codex_1_aarch64.dmg'), a('Codex_1_x64-setup.exe')] },
];
assert.strictEqual(assetUrl(releases, '.AppImage'), 'u/Codex_2_amd64.AppImage'); // skips draft, not .sig
assert.strictEqual(assetUrl(releases, '.dmg'), 'u/Codex_1_aarch64.dmg'); // falls back to an older release
assert.strictEqual(assetUrl(releases, '-setup.exe'), 'u/Codex_1_x64-setup.exe');
assert.strictEqual(assetUrl(releases, '.deb'), null);
assert.strictEqual(assetUrl([], '.dmg'), null);
console.log('download-latest: 5 checks passed');
