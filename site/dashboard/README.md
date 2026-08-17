# Public project-status dashboard

`index.html` renders `PF1e-dashboard.json`. Both files live here together because the viewer fetches
its data by **relative** URL (`const JSON_URL = "PF1e-dashboard.json"`), so the page and its feed
must sit in the same directory.

## Refreshing the data

```
./scripts/publish-site-dashboard.sh            # regenerate the JSON in place
./scripts/publish-site-dashboard.sh --check    # fail if the committed copy is stale
```

The generator is `scripts/observer/pf1e_dashboard_producer.py` — the same producer the internal
operator dashboard uses. It previously wrote **only** to `$PF1E_JSON_PATH` (default
`~/swarm-observer/PF1e-dashboard.json`): outside the repo, unversioned, unreviewable, and lost with
the machine. This directory is the versioned copy.

**Never hand-edit `PF1e-dashboard.json`.** It is generated; an edit is overwritten on the next
refresh and is invisible in review. Every figure in it is derived from `docs/work-inventory.json` and
the producer's own `doneness_verdict()`.

## `index.html`

A verbatim copy of `scripts/observer/PF1e-dashboard.html`, the internal operator viewer, placed here
as a **starting point**. It is dense and built for operators rather than for the public, and is
expected to be replaced with a public-facing page. The only thing a replacement must preserve is the
data contract: read `PF1e-dashboard.json` from the same directory.

## What is in the feed — read this before treating it as public-safe

**CORRECTED 2026-08-17.** The 2026-08-16 assessment below this line was wrong in the way that
matters most for this section's own purpose: it checked the committed JSON text for the *strings*
`PI-REDACTED`/`Product Identity`/`NAMEISPI`/`DESCISPI` and found none, and read that as "no
declared-PI content." It was not — those strings are how `data/corpus/` marks a redaction; they were
never going to appear on a name this feed's own roster-building code pulled straight from a PCGen
`.lst` line with no PI screen at all. Wave-8 and wave-9 adversarial review found the real exposure by
cross-referencing published NAMES against the pinned oracle's own `NAMEISPI:YES` declarations instead
of scanning for redaction-marker text: **261 names in `site/dashboard/units/*.json` and 56 more in
this file's own `manifests`/roadmap content** (`OPEN-ISSUES.md` rows 141/149). Operator ruling,
Decision 12 (`SD-31-corpus-closure-grind/decisions.md §12`, 2026-08-17): **"withhold the name, keep
the row."** A public artifact may publish that a record exists and every derived figure about it, but
never a name its own corpus row declares Product Identity.

**Fixed in the producer, not by hand-trimming this file** (a hand-trim is undone by the next refresh —
this section's own 2026-08-16 warning about that pattern was correct even though its PI-safety claim
was not). `scripts/observer/pi_redaction.py` cross-references every published name against the pinned
PCGen oracle's own declaration — exact `(book, source_file, source_line)` coordinates where available
(`build_unit_shards`), the raw source line directly where the roster builder already holds it
(`_parse_lst_first_field`), and a full-oracle exact-match sweep as a defense-in-depth pass over the
whole assembled document and every shard. A declared name ships as `[redacted PI]`; the row, its
count, and every other figure about it are unaffected. A `verify.sh` stage,
`site-dashboard-pi-gate`, fails the build if a declared-PI name is ever found in the committed feed
or a shard — mutation-proven by seeding a leak in both and confirming both are caught.

`site/dashboard/units/` (the per-kind unit-search shards, ~38.5k rows) is committed here for the
first time as of the same fix — it was deliberately withheld until the redaction above existed
(row 141's requirement #4).

None of the ordinary internal-engineering-prose content this section originally described (decision
bodies, per-book open questions, agent snippets, local paths, monster stat-block `detail` strings) is
a licensing exposure on its own; that part of the 2026-08-16 assessment stands. The correction above
is specifically about published NAMES, the one thing PCGen's own PI declaration governs.

## Provenance

Landed 2026-08-16 on `tranche/11` at operator request: *"let's get a copy of the json out there along
with the html. I will replace the html with something suitable for the public."*
