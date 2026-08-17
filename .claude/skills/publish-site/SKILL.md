---
name: publish-site
description: Publish the site/ directory from a work branch to the deploy branch, which pushes it live to Cloudflare Pages. Use when asked to publish, ship, or push the public site, refresh what campaign-codex.org shows, or get status/dashboard data in front of the public. Also use when asked to check whether the site is safe to publish.
---

# Publishing the public site

`site/**` is deployed to Cloudflare Pages by `.github/workflows/deploy-site.yml` on every
push to `main`, **with no build step**. The committed files ARE the published artifact.

**Merging to `main` is the publish action.** Everything else is preparation.

## The one thing to hold in mind

Publishing is not reversible in the way a bad commit is. Once `site/` is on `main` it is
served, crawled, indexed and cached. Deleting it afterwards removes it from the origin, not
from Google, not from the Internet Archive, not from anyone's cache.

The specific hazard in this repo is **Paizo Product Identity** — deity names, certain class
and item identities — which the corpus marks with `NAMEISPI:`/`DESCISPI:` and which we are
licensed to *use* but not to *republish as our own*. Operator ruling `decisions.md §12` is
the rule: **withhold the name, keep the row.** A public artifact may say a record exists and
publish every derived figure about it, but never a name its own row declares as PI.

PI screening in this program has failed in **five of the last ten waves**, every time on
something an earlier check was structurally blind to — typo variants past an exact-substring
scan, names in `raw_tokens` nobody thought to screen, `.MOD`-declared names a
coordinate-keyed check never looked up. **Assume the current check has a blind spot and go
looking for it.** That posture is the job; the script is only the mechanics.

## Do this

```bash
./scripts/publish-site-to-main.sh --dry-run          # always first
./scripts/publish-site-to-main.sh                    # opens a PR; does not merge
./scripts/publish-site-to-main.sh --from tranche/12 --to main
```

The script refuses to proceed unless it can prove things are safe. It:

1. requires a clean tree and an authenticated `gh`;
2. **discovers** every `site`/`pi` stage in `scripts/verify.sh` and runs them all — discovered
   rather than hardcoded, so a gate added later is picked up instead of silently skipped, and
   finding *no* stages is itself a refusal;
3. runs an **independent declared-PI sweep over the exact publish payload**, not over what a
   gate chose to look at;
4. shows the file list and diff against the deploy branch;
5. lands `site/` as a **tree**, one clean commit on a fresh branch off the deploy branch;
6. pushes and opens a PR — **never merges**.

`main` is a protected branch, so a PR is required regardless.

### Why a tree, not cherry-picks

Site content lives on a long-running work branch (`tranche/N`) interleaved with engine
commits. Cherry-picking the site commits drags unrelated work onto the deploy branch and
invites conflicts on every subsequent publish. Taking the tree gives a deploy branch whose
history is a clean sequence of "here is what the site looked like at time T".

**Only `site/` is published.** The generators that produce it stay on the work branch and
reach `main` through the normal promotion path. Cloudflare needs the artifact, not the
toolchain.

## Before you let it merge

The script's gates are necessary, not sufficient. Add these:

- **Read the payload yourself.** Spot-check the actual JSON going up, not a summary of it. If
  a subagent reported "clean", that is a claim to verify, not a result to accept.
- **Check every field, not just `name`.** Labels, titles, type facets, and anything built by
  string concatenation. The known residual gap is a declared-PI name embedded inside a longer
  derived string — exact-leaf matching cannot see it.
- **Prove the gate can fail.** Seed a declared-PI name into the payload, run the gate, confirm
  it goes red, then remove the seed. A gate that cannot fail is worse than no gate, and this
  repo has shipped several.
- **Check the data is current.** Cloudflare serves what is committed. Stale JSON publishes a
  stale number as fact. Run `./scripts/publish-site-dashboard.sh` first if the feeds have not
  been refreshed this cycle.
- **Say what is going.** Report the file count, the redaction count, and the gate results
  before the operator merges. They are deciding, not rubber-stamping.

## The reviewed substring allow-list

`scripts/site/pi_substring_allowlist.py` is a short, hand-reviewed list of exact `(name,
book)` pairs that DO embed a declared-PI word (`Ulfen Guard`, `Shackles of Compliance`) but
were read, one at a time, and judged mundane rather than a genuine PI disclosure — see the
file's own module docstring for the full rationale (SITE-PI-ALLOWLIST-001, 2026-08-17
operator ruling).

**It must stay short.** Every entry is a place a real leak could hide behind "we looked at
that once" — the operator's own words when accepting this design. Before it grows:

- Read the actual corpus row, not just the name. A description that is substantively *about*
  the declared-PI thing (a deity's clergy, specific setting lore) is a redaction, not an
  allow-list entry — see `Death (Pharasma)`, the one name this design redacts rather than
  excuses.
- Every entry needs a reason a reviewer with no other context can check; `test_build_public_status.py`
  fails the build if one is blank.
- Re-read the EXISTING entries whenever you add a new one, not just the new one.

If a publish's PI sweep finds a fresh hit, the fix is almost never "add it to the allow-list
and move on" — read the row first.

## Keeping it self-maintaining

The published data must be **generated, never hand-maintained** — a hand-edit is silently
undone by the next refresh and is invisible in review. The generators are wired into
`scripts/publish-site-dashboard.sh`, and `verify.sh` carries staleness gates that fail when a
committed feed drifts from what the generator would produce.

If you find yourself editing a JSON file under `site/` by hand, stop: fix the generator.

## When it goes wrong

- **A gate fails** — that is the gate working. Do not pass `--skip-gates`; fix the finding.
  (`--skip-gates` additionally requires `PUBLISH_SITE_I_ACCEPT_THE_RISK=1`, exists only for a
  machine that genuinely cannot run the gates, and is never a way past a red one.)
- **`gh pr create` returns nothing** — the branch is still pushed; open the PR by hand.
- **Something is already published that should not be** — treat it as an incident. Remove it
  from `main` immediately so it stops being served, then tell the operator plainly what was
  exposed, for how long, and that removal does not undo indexing.
