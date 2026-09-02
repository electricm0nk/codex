---
title: v0.8 stretch — DM Toolkit v1 build brief
status: active
scope: apps/desktop/src/dmToolkit/** (new), apps/desktop/src-tauri (one export command)
date: 2026-09-01
---

# DM Toolkit v1 — build brief

Operator directive, 2026-09-01: **build it now, in this window.** Something real to react to beats
a clean spec, and v0.9 adjusts from a working artefact rather than starting cold. This supersedes
the closeout's "defer to v0.9" recommendation and §4.1's DM-Toolkit-out-of-scope line — the
operator owns that call.

## The six open questions, decided by the orchestrator

`scout` left Q-DM1..Q-DM6 for the operator. Blocking on them would spend the window, so they are
decided here as defaults. **Every one is a v0.9 adjustment point — that is the deliverable.**

- **Q-DM5 (system-agnostic vs PF1-first) → SYSTEM-AGNOSTIC, prose and links only.** This is the
  load-bearing decision. NoDA is a Cyberpunk RED campaign whose stat blocks came from no rules
  engine, and `scout` found the value is the record model and the 440 cross-links, not the
  numbers. Going prose-first means **no engine dependency at all**, so blocker B14 does not gate
  this build and nothing touches repo-root `src/`. Records carry free-text stat blocks the DM
  types; nothing is computed. Linking a record to a saved Codex character is a v0.9 question.
- **Q-DM1 (storage) → localStorage as source of truth, mirroring `campaignModel.ts`.** That module
  already establishes the pattern in this app (localStorage is truth, the folder is a mirror). Not
  the right long-term answer for hundreds of linked records — `scout` is right that it is neither
  queryable nor safe — but it is real persistence, needs no new Tauri surface, and keeps v1 inside
  one lane. File-backed storage is the first v0.9 decision.
- **Q-DM2 (record model) → the six NoDA kinds**: World, Timeline, Place, Person, Scene, Rule.
  A record is: id, kind, title, one-line summary, Markdown body, typed links to other records, and
  per-kind extra fields kept deliberately thin in v1.
- **Q-DM3 (export) → standalone single-file HTML, read-only, no embedded images in v1.** This is
  the capability the operator actually uses at the table, so it ships in v1 rather than waiting.
- **Q-DM4 (Campaign Manager seam) → build BESIDE it. Touch nothing existing.** The console is a new
  `dmToolkit/` subtree scoped per campaign. The four Markdown tabs stay exactly as they are.
  Absorb-or-replace is a v0.9 decision once the operator has both in front of them.
- **Q-DM6 (initiative tracking) → OUT.** NoDA does not do it; the stub promised it. The stub's
  wording gets corrected rather than honoured.

## Non-negotiables (unchanged)

Everything in `AGENTS.md` and the sprint brief still applies: TDD red-first, no stubs
(`wired-integration-discipline`), no teammate commits, `qa` verifies before the orchestrator
commits. **The no-stub doctrine matters more here than anywhere else in the sprint** — this is a
demo the operator will judge, and a button that looks real and does nothing is worse than an
absent one. Every affordance shipped must persist, reload, and export for real.

`StubScreen.tsx` itself stays (Manage Party still uses it); only the DM Toolkit route stops
pointing at it.

## Visual bar

Lift the visual language from `NoDA_Campaign_Console.html` — master/detail, list left with
per-tab search, record right, cross-links as inline buttons. Match the app's existing theme tokens
(`var(--color-*)`), not NoDA's palette. This is a thing the operator will look at and react to; it
should not look like a wireframe.

## Tickets

**B-12 `[backend]` — `export_dm_console` command.** Takes `{ fileName, html }`, writes the HTML
verbatim to a path the user chooses. Mirror how character export already works
(`characterExport.ts` → plugin-dialog `save` → boundary → real command). Verify with
`npm run tauri:check` and `cargo test`. Bar: 3 known reds.

**D-1 `[frontend]` — record store.** `dmToolkit/dmRecordModel.ts`: the six kinds, typed links,
CRUD, localStorage persistence keyed per campaign, mirroring `campaignModel.ts`'s conventions.
Pure model + tests, no UI. This is the foundation — get the shape right.

**D-2 `[frontend]` — console shell.** Master/detail with the six tabs, per-tab search, record
list left, record detail right. Reads real records from D-1. An empty kind says so honestly.

**D-3 `[frontend]` — record create/edit/delete.** Real forms writing through D-1. Must survive a
reload — that is the acceptance test, not a rendered form.

**D-4 `[frontend]` — cross-links.** Link a record to others; render links as buttons that focus
the target record, the way NoDA's 440 links do. Deleting a linked record must not strand a
dangling link — decide and test the behaviour.

**D-5 `[frontend]` — export to standalone HTML.** Render all records to one self-contained file
(inline CSS, no external fetches, links working inside the page) and write it through B-12.

**D-6 `[frontend]` — route the DM Toolkit entry point at the console** instead of `StubScreen`,
and correct the landing copy so it no longer promises initiative tracking.

Verification for every D ticket: `npm run typecheck` (0 errors) and `npm test`.
