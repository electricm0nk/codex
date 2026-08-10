# Successor Forward-Scope Register — SD-29

> **Renamed at package consolidation (2026-08-01).** This file originated in the
> `SD-29-bestiary-2-3-4-5-content-ingestion` package under the name
> `forward-scope-register.md`. During consolidation into this directory it collided
> with a differently-scoped, differently-authored file of the same name that
> originated in this package (`./forward-scope-register.md`, authored by the
> `tranche/7-1` debt cycle) — that file routes work **into** SD-29 from the
> predecessor's retro log and is the operator sign-off gate; this file instead
> tracks work **downstream of** SD-29 (successor/retrofit dependencies). Both are
> real and neither supersedes the other; this one was renamed, not deleted, so
> both survive under distinct names. See `README.md` for the reconciliation note.

This register captures work downstream of SD-29. SD-29's successor bundle
(SD-30) is recorded as **Class 1** (named successor). Bundles that depend
on SD-29's bestiary outputs but aren't yet named land in **Class 2**.
SD-29-specific retrofits land in **Class 3**.

> **Note, 2026-08-10 (`decisions.md §38.5`).** The "successor" framing above predates the
> corpus-wide re-scope. SD-30 is no longer cleanly downstream of SD-29's output — its sixteen-book
> list is now a *subset* of SD-29's corpus-wide lane scope, a collision rather than a
> dependency. See `risks-and-open-questions.md` R-29-009/OQ-29-004; not resolved here.

## Class 0 — Doctrinal anchors (always-on)

| Anchor | Path | Note |
|--------|------|------|
| Per-book ingest pipeline | `docs/governance/book-ingestion-playbook.md` | Doctrine-of-record; pre-cycle trap-report is mandatory |
| Reach gate | `apps/desktop/src-tauri/src/reach_gate.rs` | Definition-of-done per `decisions.md §19`; gate's `OPEN_FINDINGS` carries the Bestiary-1-monster-surface prerequisite |
| Identifier discipline | `~/workspace/governance/identifier-discipline.md` | SD-29 inherits; Epic 1 enforces |
| Build-version scheme | `<major>.<tranche-base>.<build>` (2026-07-17 amendment) | SD-29 first concrete value `0.9.<build>` |
| Source STC chassis | `spec-domain-bundle-authoring` skill | 15-file shape per the modern chassis (SD-22 through SD-28) |
| Move-not-copy publish | `release-package-promotion` skill | Workspace tree removed on publish commit |
| Hermes board retirement | SD-28 `decisions.md §15a` (2026-08-01) | All post-2026-08-01 bundles are local-file only |

## Class 1 — Predecessor-deferred (named successor owns)

### C1.1 — Ingest cycle consumes `data/corpus/beastiary1/` from SD-22

**Owner:** SD-29 itself.

**What depends on SD-29:** no upstream dependency. SD-29 reads
`data/corpus/beastiary1/` as a reference shape for its own monster slices
but doesn't need it as a cycle dependency — Bestiary 1 records are
already canonical.

### C1.2 — DM Toolkit extension (consume SD-29's monster records)

**Owner:** SD-29 itself (Epic 8; was Epic 7 under the retired per-book numbering — `decisions.md §37`).

**What depends on SD-29:** the extension consumes Epic 5's monster/monster-ability chassis records
(was Epic 4 before the 2026-08-10 corpus-wide re-scope, `decisions.md §38`) for the encounter
builder + party-CR math. Epic 8 is gated on Epic 5's pilot cycle-batch (Bonus Bestiary) landing, and
can consume the remaining books incrementally as Epic 5's extension cycle-batches close — it does
not wait for every lane.

**Status:** Epic 8 is optional, operator-pinned; absent an explicit call it takes the safe default of a Class 3 (C3.1) retrofit per §C3.1 below, per `decisions.md §19`.

### C1.3 — `class_feature` (15,472 units corpus-wide) inherits `corpus-work-channels.md §9.1`'s per-class archetype funding

**Owner: ASSIGNED 2026-08-10 — SD-30** (`docs/release/SD-30-class-feature-archetype-bundle/`, renamed
via `git mv` from `SD-30-occult-and-companion-content-ingestion` the same day). Not SD-29. Previously
recorded as "not yet assigned an SD number"; the operator's 2026-08-10 directive closed that gap by
re-scoping SD-30 (whose old sixteen-book list this same directive dissolved, see OQ-29-004/R-29-009
above) into the `class_feature` bundle — the exact assignment this entry anticipated. **Widened from
90 units to 15,472 units by the 2026-08-10 corpus-wide re-scope** (`decisions.md §38.4`); originally
added by the kind-lane re-cut (`decisions.md §37.4`) at the retired seven-book, 90-unit figure.

**What depends on SD-29:** nothing — this is the reverse relationship. `class_feature` (15,472
units, 40.2% of the corpus) is Channel D per `../corpus-work-channels.md §3`/`§5.4`: blocked behind
the archetype mechanism and per-class chassis (SD-28 `§60`/`§63`), corpus-wide sizing funded
(`§9.1`) but not yet measured per-class. Explicitly excluded from every SD-29 lane, including the
now-corpus-wide Epic 4 (proven-path lane scopes only the settled-method kinds). Ingests once the
`§9.1` measurement reaches the relevant classes — tracked here so it is not silently dropped, and
not silently folded into a lane whose method does not fit it.

## Class 2 — Future-acquired (deferred)

### C2.1 — Bestiary 6 + Bonus Bestiary drop-in [SUPERSEDED — decisions.md §34, 2026-08-02]

**Superseded.** Bestiary 6 and Bonus Bestiary are no longer contingent
swap-in candidates for Bestiary 5 — `decisions.md §34` (operator directive
2026-08-02) commits all seven books, including these two, as in-scope
alongside Bestiary 5, not as a replacement for it. **Further superseded,
2026-08-10 (`decisions.md §37`):** the "Epics 11/12" per-book epics this
note pointed to are themselves retired. Bestiary 6's and Bonus Bestiary's
units are now distributed across the kind lanes (`epic-breakdown.md`
Epics 4-7) the same as every other book's.

**Original text, preserved as historical record:** The 07-30 scope-draft
flagged that Bestiary 5 has no `monster` records (player-options dataset).
Bestiary 6 + Bonus Bestiary are listed in the 07-30 scope-draft as drop-in
replacements for Bestiary 5. Cycle-0 trap-report + inventory runs first;
the swap fires only if operator prefers B6 + Bonus over B5's
player-options cycles.

### C2.2 — Monster catalog command and browser [RETIRED — 2026-08-01]

**Retired.** The monster catalog command and browser this item deferred
have shipped: the `("beastiary1", "monsters")` arm of `reach_gate.rs`
(`:986` as of 2026-08-10; was `:840`) carries an executed reach claim
in place of the old `OPEN_FINDINGS` entry;
`apps/desktop/src-tauri/src/monster_catalog.rs`'s `list_monster_catalog`
command is registered (`main.rs:57,197`); `MonsterCatalogScreen.tsx` is
routed via `CharacterHubPage.tsx:104-105`, reachable from a "Browse Monster
Catalog" button at `LandingScreen.tsx:353`. The deferred work this item
named no longer exists as an open item.

The surviving related item is the `beastiary1/race_traits` Duergar
`Spell-Like Ability ~ Invisibility` record — upstream-blocked on
`monster_codex`. *(Corrected 2026-08-10: it is no longer the sole
`OPEN_FINDINGS` entry — seven `<book>/archetypes` gaps recorded at SD-28
closure sit alongside it (SD-28 `decisions.md §60`/`§63`); those belong to
SD-30's class_feature/archetype bundle.)* That record is
now expected to be addressed by Epic 5's Monster Codex cycle-batch
(Race-Trait Lane; was Epic 13 under the retired per-book numbering —
`decisions.md §37`; `monster_codex` is in scope per `decisions.md §34`) —
see `epic-breakdown.md` Epic 5 and `forward-scope-register.md §1.2`.

### C2.3 — Bulk-modification retrofit

If operator requests a bulk-modification pass across ingested records
(per `decisions.md §17` — "bulk modifications deferred"), that pass is a
separate bundle. SD-29 preserves the per-cycle one-record-at-a-time
discipline.

## Class 3 — Retrofit (operator-on-request)

### C3.1 — DM Toolkit extension as retrofit

If Epic 8 (DM Toolkit extension; was Epic 7) does not land inside SD-29,
it surfaces as a Class 3 retrofit: separate bundle that consumes
`<book>/` monster slices. Operator-pinned per-cycle at Epic 5's pilot
cycle-batch closure (was Epic 4's under the retired seven-book kind-lane
numbering, `decisions.md §38`; was "Epics 5 and 6" under the retired per-book
numbering).

### C3.2 — Bestiary 6 + Bonus Bestiary ingestion [SUPERSEDED — decisions.md §34, 2026-08-02]

**Superseded.** This is no longer an operator-on-request retrofit —
`decisions.md §34` commits Bestiary 6 and Bonus Bestiary as in-scope
content inside SD-29 itself, not a separate bundle. **Further superseded,
2026-08-10:** their units land via the kind lanes (`epic-breakdown.md`
Epics 4-7), not the "Epics 11/12" per-book epics this note previously
named.

**Original text, preserved as historical record:** If operator prefers
Bestiary 6 + Bonus Bestiary over Bestiary 5 (per `decisions.md §18`), a
retrofit bundle adds them. Cycle-0 trap-report + inventory produces the
per-book shape finding.

## Review trigger

Reopen SD-29's forward-scope register when:

- A successor bundle reaches into SD-29's ingest outputs.
- A new bestiary arrives in the corpus.
- The bulk-modification retrofit is operator-authorized.
- The post-`tranche/9` consumer is operator-named.
- Operator requests Class 3.x retrofits.
- SD-28's per-class archetype measurement (`§9.1`) reaches the classes behind SD-29's deferred
  `class_feature` units (C1.3).

Closed-form: the bundle closes when Epic 11 (Closure Epilogue) fires.
