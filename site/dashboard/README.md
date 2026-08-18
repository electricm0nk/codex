# Internal operator dashboard (not the public status page)

`index.html` renders `PF1e-dashboard.json` here for the internal operator viewer. Both files live
together because the viewer fetches its data by **relative** URL (`const JSON_URL =
"PF1e-dashboard.json"`), so the page and its feed must sit in the same directory.

**The public status page is `site/status.html`, and it does not read anything in this
directory at request time.** It fetches `site/status-data.json` and
`site/status-data/<book_id>.json` — the narrow, allow-listed projection this README asked
for below, now built as `scripts/site/build_public_status.py`. That script reads
`site/dashboard/units/*.json` (the per-kind unit ledgers — name/book/status/wiring_class only,
regenerated locally by `publish-site-dashboard.sh` and not committed) and a curated
book-title/kind-label allow-list, and computes every figure with the producer's own
`doneness_verdict()`. It never reads `PF1e-dashboard.json` itself, so nothing in the "What is
in the feed" section below — agent snippets, decisions, session prose, `usage` — can reach the
public page structurally, regardless of what ends up in this file.

`./scripts/publish-site-dashboard.sh` (no flags) now regenerates both: the internal feed below,
and the public projection, in one run. `--check` verifies both are current.

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

A verbatim copy of `scripts/observer/PF1e-dashboard.html`, the internal operator viewer. It is
dense and built for operators, not the public — kept here for operator use, not linked from the
public site. `site/status.html` is the public-facing replacement referenced below; it does not
share code or a data contract with this viewer.

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
whole assembled document and every shard, PLUS (added SD31-W13-INTEGRATE-001, after that sweep was
found to miss three `.MOD`-declared names it should have caught) a per-book exact-match pass over
every shard's own `fields`/`rows` schema, closing the gap where a name is declared PI in one book and
legitimately not in another.

**CORRECTED AGAIN 2026-08-17 (FIX-DASHBOARD-PI).** The paragraph above's own closing claim was wrong:
`Bow of Erastil`, `Legendsbane` and `Witherfang` were **not** caught and fixed by SD31-W13-INTEGRATE-001
— they were still shipping, raw, as `Composite Longbow (Base).COPY=Bow of Erastil`,
`Dagger (Base).COPY=Legendsbane` and `Kukri (Base).COPY=Witherfang` in the committed feed at the moment
this correction was written. Every check up to this point — the coordinate-based redaction, the
per-book pass, and `site-dashboard-pi-gate` itself — was **EXACT-match only**: it asks "does this
string equal a declared-PI name," never "does this string *contain* one." All three names above are
created by a PCGen `.COPY=` directive whose OWN row carries no `NAMEISPI:YES` token at all (the
declaration lives on a separate `.MOD` row for the same object, several lines later in the same file);
an exact-match check has no string to match against, so it reported CLEAN — correctly, by its own
definition — while these names, `Helm of the Serpent King`, several `Rivethun`-prefixed spells, and
four built-up `unit_index` category labels (`Varisian Pilgrim Domain`, `Tattooed Sorcerer Varisian
Tattoo`, `Pathfinders Past Focus`, plus a fourth judged mundane — see below) all shipped. An
independent word-boundary sweep over everything under `site/` found 89 such leaks, almost all here.
**The lesson that matters more than any single name:** a gate reporting CLEAN is only as strong as what
it checks for, and this repo had already re-learned the exact-vs-word-boundary distinction once, for
`site/status-data*` (`SITE-PI-ALLOWLIST-001`) — that fix was never carried over to this feed until now.

Fixed with the SAME technique (and the SAME shared, reviewed allow-list,
`scripts/site/pi_substring_allowlist.py` — not a second list) `SITE-PI-ALLOWLIST-001` already proved
for `site/status-data*`: `pi_redaction.find_declared_pi_word_matches`, a freestanding-WORD embed check
(book-scoped union global), gated by allow-list entries reviewed one name at a time against the actual
corpus row. `_PiScreen` (`pf1e_dashboard_producer.py`) applies it in `_parse_lst_first_field` (closing
the `.COPY=` gap above — a public roster must never leak PCGen's own patch-directive syntax either,
independent of PI status), in `build_unit_shards`'s row `name` field, and in every `categories[*]`/
`school_categories[*]` category label. `site_dashboard_pi_gate.py` runs the same word-boundary check,
independently, over the top-level feed's book rosters, every shard, and every category label —
mutation-proven by seeding a declared-PI name into each of those three shapes and confirming all three
go RED, then confirming a clean `git status --porcelain` after every seed was removed. A declared name
still ships as `[redacted PI]`; the row, its count, and every other figure about it are unaffected. The
former "KNOWN RESIDUAL GAP" note about `categories[*].label` being invisible to exact-leaf matching is
closed; do not re-add it without re-deriving whether it is still true.

`site/dashboard/units/` (the per-kind unit-search shards, ~38.5k rows) is committed here for the
first time as of the SD31-W13-INTEGRATE-001 fix — it was deliberately withheld until the redaction
above existed (row 141's requirement #4).

None of the ordinary internal-engineering-prose content this section originally described (decision
bodies, per-book open questions, agent snippets, local paths, monster stat-block `detail` strings) is
a licensing exposure on its own; that part of the 2026-08-16 assessment stands. The correction above
is specifically about published NAMES, the one thing PCGen's own PI declaration governs.

## Provenance

Landed 2026-08-16 on `tranche/11` at operator request: *"let's get a copy of the json out there along
with the html. I will replace the html with something suitable for the public."*
