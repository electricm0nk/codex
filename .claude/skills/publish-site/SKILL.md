---
name: publish-site
description: Pointer only. The public site (campaign-codex.org) no longer lives in this repo; it lives in electricm0nk/codex-web. Use when asked to publish, ship, or push the public site, or to check whether the site is safe to publish.
---

# Publishing the public site

The public site moved out of this repo. It lives in the private repo
**`electricm0nk/codex-web`** (its `site/` tree, deployed to the Cloudflare Pages project
`codex` by that repo's own `.github/workflows/deploy-site.yml`). This repo has no `site/`
tree and no deploy lane any more.

**Publish = open a PR to `electricm0nk/codex-web`'s `main`; the operator merges.** Do not
re-add `site/` or a deploy workflow here — two repos deploying to one Pages project means
whichever deploys last wins.

**Known gap:** the codex Product Identity gates that used to run on the site content
(`site-dashboard-pi-gate`, `site-public-status-pi-gate`, the frozen-status and asset-stamp
checks in `scripts/verify.sh`) were removed with it and **no longer run on site content**.
Nothing in this repo screens what codex-web publishes; any PI review of the site happens in
codex-web or by hand.
