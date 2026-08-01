# Forward-Scope Register — SD-30

This register captures work downstream of SD-30. SD-30 ships the final
content-ingest slot on the post-2026-08-01 trio (SD-28, SD-29, SD-30).
SD-30's successor bundles would consume SD-30's outputs, but the next
post-tranche bundle isn't yet named; it's recorded here as **Class 1**.
Bundles that depend on the four deferred books (NPC Codex, Planar Adventures,
Occult Origins, Haunted Heroes) live in **Class 2** as deferred.
SD-30-specific retrofits land in **Class 3**.

## Class 0 — Doctrinal anchors (always-on)

| Anchor | Path | Note |
|--------|------|------|
| Per-book ingest pipeline | `docs/governance/book-ingestion-playbook.md` | Doctrine-of-record; pre-cycle trap-report is mandatory |
| Reach gate | `apps/desktop/src-tauri/src/reach_gate.rs` | Definition-of-done per `decisions.md §18` (prime rule); gate's `OPEN_FINDINGS` carries missing-surface prerequisites |
| Identifier discipline | `~/workspace/governance/identifier-discipline.md` | SD-30 inherits; Epic 1 enforces |
| Build-version scheme | `<major>.<tranche-base>.<build>` (2026-07-17 amendment) | SD-30 first concrete value `0.10.<build>` |
| Source STC chassis | `spec-domain-bundle-authoring` skill | 13-file shape per the modern chassis |
| Move-not-copy publish | `release-package-promotion` skill | Workspace tree removed on publish commit |
| Hermes board retirement | SD-28 `decisions.md §15a` (2026-08-01) | All post-2026-08-01 bundles are local-file only |
| "Recently published takes precident" | SD-30 `decisions.md §16` (operator directive 2026-08-01) | Cross-bundle precedence for SD-28/SD-29's already-published surfaces |

## Class 1 — Predecessor-deferred (named successor owns)

### C1.1 — Post-tranche consumer

**Owner:** not yet named.

**What depends on SD-30:** SD-30 is the third post-2026-08-01 bundle
(SD-28, SD-29, SD-30). The next bundle would consume SD-30's
content-source-ingest outputs (occult + mythic + Monster Codex + Inner Sea
+ Book of the Damned cycles). Operator-pinned when the next bundle
opens.

### C1.2 — Class-grant overlap with SD-28

**Owner:** SD-30.

**What depends on SD-30:** SD-30 owns canonical class definitions for the
four shared classes (Occultist, Spiritualist, Medium, Mesmerist) that
appear in both Ultimate Intrigue (SD-28's territory) and Occult
Adventures (SD-30's territory). SD-28's Epic 6 references the canonical
class id from SD-30's progress; SD-30 does not redefine.

## Class 2 — Future-acquired (deferred)

### C2.1 — NPC Codex

The NPC Codex is a real Paizo product; the corpus directory is not
under `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/`
as of 2026-08-01. Per the 2026-08-01 absent-book rule, NPC Codex drops
from scope. A future bundle (or runtime operator directive) may acquire
the LST data and bring it in.

### C2.2 — Planar Adventures

Same disposition as C2.1 — Planar Adventures is a real Paizo product
without a `planar_adventures/` corpus directory. Deferred.

### C2.3 — Occult Origins

Carried forward from the 07-30 stub; `occult_origins/` corpus
directory is not present. Future bundle may acquire.

### C2.4 — Haunted Heroes Handbook

Same disposition as C2.3 — `haunted_heroes/` corpus directory is not
present. Carried forward from the 07-30 stub as a softcover companion.

## Class 3 — Retrofit (operator-on-request)

### C3.1 — Mythic Adventures reach-surface prerequisite

Mythic Adventures' reach surfaces are existential (the mythic path
mechanics + tier features + monster stat blocks all require consumer
integration). Per `decisions.md §18` reach-gate = DoD, cycles pause on
`decision-blocked` if no consumer surface reaches the gate.

The remedy is either (a) a campaign-tool consumer epic inside SD-30, or
(b) a separate bundle that consumes Mythic Adventures' records. The
operator decides per cycle.

### C3.2 — Occult Adventures psychic-discipline consumer surface

Occult Adventures' psychic-discipline mechanics (`psychic_discipline_*`
records) require a class-feature consumer surface to satisfy reach.
`reach_gate.rs OPEN_FINDINGS` flags missing surfaces per the per-cycle
audit.

### C3.3 — Inner Sea series campaign-tool surface

The Inner Sea series (×9 modules) is primarily campaign-setting data
(traits, regions, factions). Per-book ingest produces canonical records;
the cycle's reach gate may flag missing consumer integration (e.g.,
a campaign-setup wizard surface). Per-cycle gap filing.

## Review trigger

Reopen SD-30's forward-scope register when:

- A successor bundle reaches into SD-30's outputs.
- A class-grant resolution fires for the four shared classes.
- Operator authorizes NPC Codex / Planar Adventures / Occult Origins /
  Haunted Heroes retrofit.
- A missing-surface gap is recorded in `reach_gate.rs OPEN_FINDINGS`
  for Occult / Mythic / Inner Sea records.
- The post-`tranche/10` consumer is operator-named.

Closed-form: the bundle closes when the Closure Epilogue fires.
