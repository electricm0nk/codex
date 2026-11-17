// Points each "Download latest" link at the newest release's asset for its OS.
// Every release is a pre-release, so GitHub's /releases/latest/download/ URL does not work;
// we ask the public API instead. With no JS, a blocked script, or a failed/rate-limited
// API call, each link keeps its static href to the releases page.
(function () {
  var API = 'https://api.github.com/repos/electricm0nk/codex/releases?per_page=10';

  // Newest non-draft release (API order is newest first) that has an asset ending in `suffix`.
  function assetUrl(releases, suffix) {
    for (var i = 0; i < releases.length; i++) {
      if (releases[i].draft) continue;
      var assets = releases[i].assets || [];
      for (var j = 0; j < assets.length; j++) {
        if (assets[j].name.slice(-suffix.length) === suffix) return assets[j].browser_download_url;
      }
    }
    return null;
  }

  if (typeof module !== 'undefined') { module.exports = { assetUrl: assetUrl, API: API }; return; }

  var links = document.querySelectorAll('a[data-asset-suffix]');
  if (!links.length || !window.fetch) return;
  fetch(API, { headers: { Accept: 'application/vnd.github+json' } })
    .then(function (r) { if (!r.ok) throw new Error('HTTP ' + r.status); return r.json(); })
    .then(function (releases) {
      Array.prototype.forEach.call(links, function (a) {
        var url = assetUrl(releases, a.getAttribute('data-asset-suffix'));
        if (url) a.href = url;
      });
    })
    .catch(function () { /* keep the releases-page fallback */ });
})();
