---
canonical: true
owner: god-emporer
status: planning-ready (chassis completed 2026-08-22 from SD-31 session)
date: 2026-08-22
---

# SD-32 Technical Requirements

## Pre-loop prerequisites

- `tranche/12` checked out, `git pull --ff-only` clean (cut from `tranche/11`'s tip; `decisions.md §1`).
- The PCGen oracle checkout at the pin (`scripts/pcgen-oracle-pin.env`) **in the repo-local slot**
  `artifacts/corpus/operator-supplied/pcgen` (git-ignored; `artifacts/corpus/README.md`), verified
  with `scripts/verify.sh --only preflight-oracle` after exporting the `workflow-instruction.md §2.1`
  env block (bootstrap with `scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"` if it fails).
  `$PCGEN_CORPUS_ROOT`/`$PCGEN_REPO_DIR` always resolve to that slot — never to `~/workspace/repos/pcgen`
  or any path outside the repo.
- `SD-31-corpus-closure-grind` content merged to develop — PR #374, merged 2026-08-22, verified **by
  content** (`git diff origin/develop b1b7f4290 -- src scripts data docs/retro
  docs/release/SD-31-corpus-closure-grind` empty), not by ancestry (`workflow-instruction.md §1` item 3).
  Cited, not re-verified per cycle.
- `cargo run --locked --bin v06_work_inventory` regenerates `docs/work-inventory.json` at cycle-0
  of any card that cites a figure from it — never transcribed stale.
- The pre-loop figures (`38,372 unit denominator`; `24,914 not-done`; `35.07% done`; ten
  semantic families; the 4 unbuilt books; the 77 prestige classes) are documented in
  `epic-breakdown.md`. A cycle that quotes any of them must re-derive against the live
  `docs/work-inventory.json` and quote the derivation command in the receipt — the SD-31-era
  figures are *the starting baseline*, not a frozen value.

## Normative requirements

- **Every Gate 2 engine emits values that clear `derived_evaluator_fixture_check`**, whose expected
  value is transcribed from bytes the engine never reads. An interpreted value with no fixture is
  not done (operator ruling §20, restated in `decisions.md §3`).
- **Every Gate 0 cycle cites the live PCGen oracle pin** (`scripts/pcgen-oracle-pin.env`,
  `PCGEN_ORACLE_SHA`) in its re-derive receipt, and reads the oracle from the repo-local slot
  (`artifacts/corpus/operator-supplied/pcgen`). A figure re-derived against an unstated oracle
  commit is not re-derived.
- **Every Gate 2 cycle quotes the corpus arithmetic family** the engine handles (F1..F10, the
  canonical vocabulary in `scripts/shape_ledger.py` / `artifacts/gate-1-shape-closure/
  family-vocabulary.md` — not `epic-breakdown.md Epic 1`, whose F1/F2/F3 rows are work items,
  not a family-count table) and the corpus units it claims to reach. A cycle that emits
  values for a family without quoting the family and unit count is out of protocol.
- **No shape is "handled" without proof width stated explicitly.** Every Gate 2 engine's
  `acceptance-and-verification.md` entry must name which corpus shapes its proof does **not**
  cover, alongside the family/units it does cover. The proof width is part of the criterion.
- **No unit leaves the `38,372` denominator without an operator-signed disposition** — either it
  is reachable behind a Gate 2 engine or it carries a forward-scope-register entry with operator
  sign-off (`risks-and-open-questions.md` tracks the four pending rulings, B1/B2/B4/B5).
- **`scripts/verify.sh` full passes before any cycle's commit** (mirrors SD-30's standing
  requirement at `SD-30-.../decisions.md §18`, AT-30-002). The closure invariant (Gate 3) is
  wired into `verify.sh` as a real stage, not a courtesy check.
- **The Epic 5 protective sweep (self-erasure check across Rust generators) runs before Gate 0.**
  Scaling engines over a generator that silently empties its own fixtures is the failure class
  Gate 2 depends on not existing. See `artifacts/HANDOFF.md` for the live precedent
  (`scripts/derive_derived_evaluator_fixtures.py` was destroying 2,110 fixture entries per run
  before the fix; 17 of the 29 generators — `ls src/bin/{gen_,ingest_,enrich_}*.rs | wc -l` — have
  never been checked for the same shape, per `epic-breakdown.md` Epic 5).

## Out of scope (technical)

- **Real-time execution engines** (RNG, opponent state, turn sequencing) — unchanged repo-wide
  constraint, not touched by this bundle.
- **PDF reader / scraping source extraction** — deliberately not built (`decisions.md §5`).
  Calibration would measure the wrong thing against the wrong source.
- **Speculative generalisation of the reader seam** — one worked example, generalised only when
  a second reader exists to test the abstraction against.
- **Prose-source ingestion research** — filed in `forward-scope-register.md` C3.x as research-grade
  forward scope. The discipline does not port unchanged from PCGen's pinned ground truth; the
  design work is not part of SD-32.
- **The form-interpreter PMMG build (the "Edge of the Sea" tranche** that SD-30's
  `state-goals-and-lessons.md §1.3` hazard 4 references) — out of scope, and a `verify.sh`
  finding every cycle.
- **The SD-31 anti-gaming apparatus itself** — read as doctrine, not modified. Cycle receipts that
  re-derive against `doneness_verdict()` import the table rather than reimplementing it; the
  generator/producer same-commit discipline (`SD-30-.../state-goals-and-lessons.md §1.3`) still
  binds any future touch.
