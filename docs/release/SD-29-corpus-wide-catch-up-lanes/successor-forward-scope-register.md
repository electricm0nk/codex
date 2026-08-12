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

**Status: SUPERSEDED BY C3.1 — ruled 2026-08-11.** The safe default this entry anticipated was
taken: `epic-8-toolkit` is `decision-blocked`, Epic 8 does not land inside SD-29, and ownership
moves out of this bundle to the Class 3 retrofit at **§C3.1** (now ACTIVE), which carries the
evidence. SD-29 is no longer the owner of the DM Toolkit extension. Preserved above as the record
of what was planned, per `decisions.md §19`.

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

### C3.1 — DM Toolkit extension as retrofit — **ACTIVE (ruled 2026-08-11)**

**Status: ACTIVE.** This is no longer conditional. Card `epic-8-toolkit` was ruled
`decision-blocked` on 2026-08-11 under `loop-instruction.md` UNATTENDED MODE item 4 (the bundle's
one sanctioned instance). Epic 8 does **not** land inside SD-29; the DM Toolkit extension is a
Class 3 retrofit owned by a successor bundle. Full evidence in `progress.md`, cycle
`SD29-E8-F1-001`.

**The criterion and why it was not met.** `epic-breakdown.md` Epic 8 and `loop-instruction.md`
"Epic ordering" make Epic 8 in-scope only if a lane cycle *needed* the consumer surface to satisfy
its reach claim. It did not. Both reach claims Epic 5's pilot landed assess an
already-shipped surface:

```
git show origin/worktree-wf_3516060a-756-9:apps/desktop/src-tauri/src/reach_gate.rs \
  | awk '/^fn bonus_bestiary_(monsters|monster_abilities)_reach/,/^}/' \
  | grep -o 'assess("[a-z_]*"' | sort -u
```
→ `assess("list_monster_catalog"` — one distinct surface, and it is the monster catalog that
shipped under SD-22/C2.2, not a toolkit. Zero of the pilot's claims route through an encounter
builder or party-CR screen. No `OPEN_FINDINGS` entry names a DM-toolkit surface as its remedy
either; the seven standing entries all name an archetype picker (SD-30's). `epic-breakdown.md`
Epic 8 makes the same point from the other side: the `OPEN_FINDINGS` Bestiary-1-monster-surface
prerequisite Epic 8 would have satisfied "already [was] independently satisfied", so deferring
costs the reach gate nothing.

**What the retrofit inherits, and what it must build.** The engine half already exists and is
untouched by SD-29 — `src/rules_core/encounters.rs` (DM-toolkit encounter-difficulty computation,
landed by SD-22 Epic 6, criterion 18) and `src/rules_core/party_cr.rs`. What is missing is the
entire consumer path, which is why this is a bundle and not a cycle:

- **No IPC command.** `grep -n 'invoke_handler' -A 60 apps/desktop/src-tauri/src/main.rs | grep -icE 'encounter|party_cr'` → **0**. Neither module is reachable from the front end at all.
- **No screen.** `CharacterHubPage.tsx:112-120` renders the `dm-toolkit` mode as a `StubScreen`
  reading "Encounter building, initiative tracking, and other GM-side tools. Not built yet." That
  is an honest labelled placeholder, not a no-stub-doctrine violation — but it is also the whole of
  the surface today.

So landing Epic 8 inside SD-29 would mean building a new command, a new screen, and new reach
claims to satisfy no lane's requirement — scope no cycle needs, taken on at the cost of the lanes
that do. The safe default is the retrofit.

**Consequences to honour downstream.** `epic-10-review`'s dependency on `epic-8-toolkit` is
written "(COMPLETE or `decision-blocked`)" and is therefore satisfied; Epic 10 is not held. Class 1
entry **C1.2** ("Owner: SD-29 itself (Epic 8)") is superseded by this ruling — see the note added
there. Nothing in Epic 5's chassis is wasted: the monster/monster_ability records the retrofit
consumes land regardless, and the retrofit can start whenever a successor bundle is named.

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

### C3.3 — Retroactive magnitude-fidelity sweep over already-landed `static` units — **OPEN (routed 2026-08-12)**

**Routed by `decisions.md §46`, which split one instrument in two.** The per-cycle half is in SD-29
now: `epic-breakdown.md` Epic 4's three feature seeds each carry a magnitude-fidelity acceptance
criterion, so no *newly landed* `static` record escapes an exact round-trip check against its corpus
literal. This entry is the other half — the sweep over records that landed **before** that criterion
existed — and it is deliberately not SD-29's work.

**What it owns.** The `static` + `ingested-magnitude` backlog: **4,582 units** against the committed
inventory (`generated_at 2026-08-11T22:28:28Z`), all six of them in Epic 4's kinds
(`equipment`, `equipment_modifier`, `spell`, `feat`, `race`, `class`) — the command is in
`epic-breakdown.md` Epic 4. Re-derive before starting; Epic 4's own cycles will have moved the
figure, downward for records that land under the new criterion and upward for ones they add.

**Why it is a bundle and not a cycle.** The per-cycle criterion is nearly free because the cycle
already holds both the corpus row and the record it just wrote. The sweep holds neither, so it must
build the corpus-side reader standalone across every magnitude-bearing field (`COST`, `WT`, `AC`,
`DAMAGE`, `RANGE`, and `BONUS` literals) over ~7,487 `static` units. Two constraints make it real
work rather than a rollup:

- **`.MOD` base-name resolution MUST mirror the determinator's** (`CATEGORY=<x>|<Base>.MOD` →
  `<Base>`; `CLASS:<Base>.MOD` → `<Base>`) or the sweep and the inventory will disagree about which
  record a row belongs to and the check will silently skip rows — the identical hazard
  `wiring-class-determination.md` records for the closure pass.
- **It is a new test binary in the `reach_gate.rs` family**, not an extension of it. The reach gate
  asserts presence in an IPC response; this asserts textual fidelity to a corpus line. Sharing the
  inventory-construction machinery is welcome, sharing the assertion is not.

**What it unblocks, and what it does not.** It is the missing `DONE` verdict word for the `static`
class — `wiring-class-determination.md`'s table records the status column for `static` as
"(none — currently `ingested-magnitude`)". Until it exists, those units cannot be reported as
complete by any instrument, which is the dashboard defect the operator raised on 2026-08-12: a
finished `static` unit renders identically to unfinished `computed` work. It does **not** address
`derived`, whose bar is an evaluator-vs-fixture check over a further 1,000 `ingested-magnitude`
units and whose status column is equally "(none)" — a sibling gap, not this entry's scope, and
unrouted as of this writing.

**Not blocking anything.** No lane, epic, or reach claim in SD-29 waits on this. Deferring it costs
the bundle nothing; running it inside the bundle would cost the lanes cycles they need.

## Review trigger

Reopen SD-29's forward-scope register when:

- A successor bundle reaches into SD-29's ingest outputs.
- A new bestiary arrives in the corpus.
- The bulk-modification retrofit is operator-authorized.
- The post-`tranche/9` consumer is operator-named.
- Operator requests Class 3.x retrofits.
- The retroactive magnitude-fidelity sweep is operator-authorized, or the `derived` class's
  evaluator-vs-fixture bar is routed to an owner (C3.3).
- SD-28's per-class archetype measurement (`§9.1`) reaches the classes behind SD-29's deferred
  `class_feature` units (C1.3).

Closed-form: the bundle closes when Epic 11 (Closure Epilogue) fires.
