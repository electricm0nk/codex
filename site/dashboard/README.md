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

Checked 2026-08-16 against the committed copy (1.3 MB, 274 free-text strings over 120 chars):

- **No Paizo Product Identity text.** `PI-REDACTED` and `Product Identity` appear **zero** times. The
  two `NAMEISPI`, one `DESCISPI` and one `raw_tokens` occurrences are field *names* discussed inside
  engineering prose, not declared-PI content.
- **It does carry internal engineering prose** — resolved and open decision bodies (80 + 13),
  decision titles and call text, per-book open questions, agent snippets, and session-progress lines.
- **It carries some local and operational strings** — `worktree` ×68, `/home/ubuntu` ×3, `session_`
  ×4, agent entries ×15.
- Monster `detail` strings are factual stat summaries including source page references, e.g.
  `CR 1 · size M · Undead · speed 30 ft · p.146 · natural attacks: Claw 1d6, Bite 1d6`.

None of that is a licensing exposure. The operational strings simply were not written for a public
audience. **If the public page should show figures only, the right fix is a narrow allow-listed
projection generated as its own file — not a hand-trim of this one**, which would be silently undone
by the next refresh.

## Provenance

Landed 2026-08-16 on `tranche/11` at operator request: *"let's get a copy of the json out there along
with the html. I will replace the html with something suitable for the public."*
