# Forward-Scope Register — SD-28

This register captures work downstream of SD-28 that either depends on
SD-28's ingest outputs or revisits SD-28's contracts as a future bundle.
SD-28's successor bundles (SD-29 / SD-30) are recorded here as **Class 1**
(predecessor-deferred, in-scope for named successor). Bundles that depend
on SD-28's book-list completion but are not yet named land in **Class 2**
(future-acquired, deferred). SD-28-specific retrofits land in **Class 3**
(if/when operator requests).

## Class 0 — Doctrinal anchors (always-on)

| Anchor | Path | Note |
|--------|------|------|
| Per-book ingest pipeline | `docs/governance/book-ingestion-playbook.md` | Doctrine-of-record per the playbook; pre-cycle trap-report is mandatory |
| Reach gate | `apps/desktop/src-tauri/src/reach_gate.rs` | Definition-of-done per `decisions.md §18`; gate's `OPEN_FINDINGS` carries the APG/ACG equipment-surface prerequisite |
| Identifier discipline | `~/workspace/governance/identifier-discipline.md` | SD-28 inherits; Epic 1 enforces |
| Build-version scheme | `<major>.<tranche-base>.<build>` (2026-07-17 amendment) | SD-28 first concrete value `0.8.<build>` |
| Source STC chassis | `spec-domain-bundle-authoring` skill | 12-file shape |
| Move-not-copy publish | `release-package-promotion` skill | Workspace tree removed on publish commit |

## Class 1 — Predecessor-deferred (named successor owns)

### C1.1 — Bestiary 2-5 cycle pattern

**Owner:** SD-29 (`./../SD-29-bestiary-2-3-4-5-content-ingestion/` — repo-resident canonical home; workspace source-of-record removed on publish per move-not-copy doctrine).

**What depends on SD-28:** SD-29 inherits SD-28's per-book-ingest pipeline shape.
The cycle pattern (per-monster-block, reach-gate, trap-report) is established
by SD-22's Bestiary 1 ingest and refined by SD-28's per-class / per-equipment
cycles. No API changes — SD-29 reads SD-28's pipeline surface as the
documented shape, not as a code dependency.

**Cross-bundle doc:** SD-29's `loop-instruction.md` cites SD-28's
`loop-instruction.md` for the per-cycle base procedure.

### C1.2 — Cross-bundle class overlap (SD-30)

**Owner:** SD-30 (`./../SD-30-occult-and-companion-content-ingestion/` — repo-resident canonical home; workspace source-of-record removed on publish per move-not-copy doctrine).

**What depends on SD-28:** Classes shared between Ultimate Intrigue and
Occult Adventures (Occultist, Spiritualist, Medium, Mesmerist) have their
canonical class definition owned by SD-30 per `decisions.md §5`. SD-28
references the canonical class id from SD-30's progress; SD-28 does not
redefine.

**Cross-book conflict rule applies after both bundles land:** when SD-30's
Occult Adventures definition contradicts SD-28's Ultimate Intrigue
definition on the same class, `decisions.md §16` resolves it (newer book =
doctrine, older book = errata). The class-grant case is the only exception
to §16 (preserved from SD-22's doctrine).

## Class 2 — Future-acquired (deferred)

### C2.1 — Dreamscarred Press corpus expansion

If operator acquires additional Dreamscarred Press books beyond
`ultimate_psionics` (e.g., `psionics_unleashed`, `psionics_expanded` — both
confirmed in the corpus per `decisions.md §17`), a future bundle (or an
in-cycle retrofit) covers their ingest. SD-28 does not lock these in.

### C2.2 — Bulk-modification retrofit

If operator requests a bulk-modification pass across all ingested U-line
records (per `decisions.md §17a` — "bulk modifications deferred"), that
pass is a separate bundle. SD-28 preserves the per-cycle one-record-at-a-
time discipline and does not bulk-edit.

### C2.3 — Post-tranche consumer

The next tranche after `tranche/8` (whatever it becomes — `tranche/9`,
`tranche/9-1`, etc.) is out of scope here; it inherits the post-`develop`
merge of SD-28's closure work.

## Class 3 — Retrofit (operator-on-request)

### C3.1 — UE equipment catalog widening

`apps/desktop/src-tauri/src/equipment_catalog.rs` reads CRB alone; APG/ACG
already-ingested equipment reaches no surface today (per
`reach_gate.rs OPEN_FINDINGS`). UE adds the largest equipment book in
the corpus — the catalog widening must complete before UE's cycles reach
the gate. The remedy was proposed in SD-28's earlier stub as either an
in-scope epic or a named prerequisite outside it; `decisions.md §10`
(this version of the bundle) supersedes that with `§18` — the reach gate
is the definition of done; engine or widening where strictly necessary;
UE's cycling pauses on `decision-blocked` if the surface remains absent.

**Operator decision this surfaces:** is the catalog widening a
precycle prerequisite outside SD-28 or a SD-28-owned retrofit? `decisions.md
§10` (the prior version) marked this operator-pending; this retrofit entry
preserves the question without forcing the answer.

### C3.2 — Ultimate Psionics third-party tier license retro-fit

The pre-cycle verification per `decisions.md §17` validates licensing at
upsionics ingest start. If the verification surfaces records whose
licensing annotations don't match open-content tier (e.g., a record
annotated `OGL` but matching PSPF PI patterns), the affected records
drop from the per-cycle scope. A retrofit bundle may revisit; SD-28
records the dropped records as cycle findings, not blockers.

## Review trigger

Reopen SD-28's forward-scope register when:

- A successor bundle (named or un-named) reaches into the seven books' ingest outputs.
- A new U-line book arrives in the corpus.
- The bulk-modification retrofit is operator-authorized.
- The post-`tranche/8` consumer is operator-named.
- Operator requests Class 3.x retrofits.

Closed-form: the bundle closes when Epic 10 (Closure Epilogue) fires.
