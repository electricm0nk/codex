# SD-31 Technical Requirements

## Pre-loop prerequisites

- `tranche/11` checked out, `git pull --ff-only` clean (cut from `tranche/10`'s tip 2026-08-15,
  `decisions.md §6`).
- The PCGen oracle checkout at the pin (`scripts/pcgen-oracle-pin.env`), verified with
  `scripts/verify.sh --only preflight-oracle` (bootstrap with `scripts/fetch-pcgen-oracle.sh` if it
  fails). Resolve via `$PCGEN_CORPUS_ROOT`/`$PCGEN_REPO_DIR`, never a literal local path.
- `SD-30-class-feature-archetype-bundle`'s Epics 1, 2 and 3 `COMPLETE` — all three closed 2026-08-14,
  cited not re-verified per cycle.
- `scripts/reachability_audit.py` (Epic 0) exists and has a committed baseline run before any other
  card is claimed (`decisions.md §4`). **Clarified 2026-08-15 (launch-readiness remediation Step 5,
  drift D3):** this is listed under "pre-loop prerequisites" but cannot literally precede the loop —
  the script is Epic 0's own deliverable (`README.md` "Source STC contents"), and Epic 0 is a card
  inside this loop, not outside it. Read this bullet as **"before any card other than Epic 0's own
  first cycle"**: the loop's very first claimed card must be an Epic 0 card, that cycle builds the
  script, proves it can fail, and commits the baseline run — only after that does the "before any
  other card" gate apply to everything that follows. It is not yet in `scripts/verify.sh` (confirmed
  `--list` this cycle carries no `reachability`-named stage) and is not assumed to be until Epic 0-F1
  lands it.
- `cargo run --locked --bin v06_work_inventory` regenerates `docs/work-inventory.json` at cycle-0 of
  any card that cites a figure from it — never transcribed stale.

## Normative requirements

- Every ingested record satisfies the reach-gate prime rule (`AT-31-002`).
- Every ingest cycle in Epics 5/6/7 cites the PI-gate `COMPLETE` receipt for the specific book before
  claiming it, and the production path calls the documented readers (`AT-31-003`, `G1.4`/`G1.5`).
- No `race`/`race_trait` ingest cycle claims a book whose races are not covered by a landed Epic 1
  chassis batch; no `unknown`-bucket seed (Epic 3-F4, Epic 5-F3) claims before Epic 2 is `COMPLETE`
  (`AT-31-101`).
- Every Epic 6 card records its raw-vs-workable split with command before planning cycles (`AT-31-004`).
- No unit leaves the 100 % denominator without an operator-signed Structural Exclusion Register entry
  (`AT-31-100`).
- No blended per-class measurement figure (`AT-31-001`).
- `scripts/verify.sh` full passes before any cycle's commit, mirroring SD-30's own standing requirement
  (`SD-30-.../decisions.md §18`, AT-30-002).

## Out of scope (technical)

- The PI-screening gate's own implementation — that is SD-30's Epic 3, consumed not built here.
- The dashboard producer's `doneness_verdict()` table and consumer-delta probes — SD-30's Epic 0.
- Real-time execution engines (RNG, opponent state, turn sequencing) — unchanged repo-wide constraint.
