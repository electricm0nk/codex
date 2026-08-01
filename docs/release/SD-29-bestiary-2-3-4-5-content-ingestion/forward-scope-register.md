# Forward-Scope Register — SD-29

This register captures work downstream of SD-29. SD-29's successor bundle
(SD-30) is recorded as **Class 1** (named successor). Bundles that depend
on SD-29's bestiary outputs but aren't yet named land in **Class 2**.
SD-29-specific retrofits land in **Class 3**.

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

### C1.2 — DM Toolkit extension (consume Bestiary 2-5)

**Owner:** SD-29 itself (Epic 7).

**What depends on SD-29:** the extension consumes Epic 3-6's monster
records for the encounter builder + party-CR math. Cycles do not
interleave with Epic 7 until Epic 3-6 close.

**Status:** Epic 7 is in scope, gated at cycle-0 per `decisions.md §18`.

## Class 2 — Future-acquired (deferred)

### C2.1 — Bestiary 6 + Bonus Bestiary drop-in

The 07-30 scope-draft flagged that Bestiary 5 has no `monster` records
(player-options dataset). Bestiary 6 + Bonus Bestiary are listed in the
07-30 scope-draft as drop-in replacements for Bestiary 5. Cycle-0
trap-report + inventory runs first; the swap fires only if operator
prefers B6 + Bonus over B5's player-options cycles.

### C2.2 — Monster catalog command and browser

Bestiary 1's 41 ingested monsters reach no surface today (per
`reach_gate.rs OPEN_FINDINGS`). The remedy proposed by the 07-30
scope-draft is "a monster catalog command and browser, mirroring
`spell_catalog.rs` + `SpellCatalogScreen.tsx`." This is a separate
surface-building epic. SD-29's Epic 7 (DM Toolkit extension) is the
nearest existing consumer; the surface-build epic itself is deferred.

### C2.3 — Bulk-modification retrofit

If operator requests a bulk-modification pass across ingested records
(per `decisions.md §17` — "bulk modifications deferred"), that pass is a
separate bundle. SD-29 preserves the per-cycle one-record-at-a-time
discipline.

## Class 3 — Retrofit (operator-on-request)

### C3.1 — DM Toolkit extension as retrofit

If Epic 7 (DM Toolkit extension) does not land inside SD-29, it
surfaces as a Class 3 retrofit: separate bundle that consumes
`beastiary<N>/` slices. Operator-pinned per-cycle at Epic 5/6 closure.

### C3.2 — Bestiary 6 + Bonus Bestiary ingestion

If operator prefers Bestiary 6 + Bonus Bestiary over Bestiary 5 (per
`decisions.md §18`), a retrofit bundle adds them. Cycle-0 trap-report +
inventory produces the per-book shape finding.

## Review trigger

Reopen SD-29's forward-scope register when:

- A successor bundle reaches into SD-29's ingest outputs.
- A new bestiary arrives in the corpus.
- The bulk-modification retrofit is operator-authorized.
- The post-`tranche/9` consumer is operator-named.
- Operator requests Class 3.x retrofits.

Closed-form: the bundle closes when Epic 8 (Closure Epilogue) fires.
