# SD-27 — References Index

> **Corrected 2026-07-27.** Every entry below was resolved against the live repo. Entries that do not
> exist repo-locally are now stated as such, with where they actually live — the previous version cited
> five `docs/governance/*.md` files and five `~/.hermes/...` skill paths that resolve nowhere on this
> machine as written.

## 1. Repo-local doctrine (REPO-LOCAL CANONICAL — verified present)

- `docs/governance/loop-instruction-template.md` — canonical loop-instruction template (operator-pinned 2026-07-21).
- `docs/governance/no-stub-mvp-doctrine.md` — wired-integration parent doctrine.
- `docs/governance/wired-integration-stubs-registry.md` — operator-granted stub exceptions; carries the **21** `book_stub` entries SD-26 registered (`#0003`–`#0023`). Entries `#0005` (beginner_box) and `#0012` (core_essentials) are out-of-scope per operator directive 2026-07-27 but still present.

**Not repo-local** — these were previously listed here as if they were:

- `docs/governance/identifier-discipline.md` — **does not exist.** The doctrine is the machine-local skill `identifier-discipline`; the runnable gate is vendored at `scripts/identifier-discipline-audit.sh`.
- `docs/governance/spec-domain-lifecycle.md` — **does not exist.** The doctrine is the skill `spec-domain-lifecycle-routing`.
- `docs/governance/ogl-pi-blacklist.md` — **does not exist yet, by design.** Cycle 2.0.5 creates it; it is an output, not a prerequisite.

## 2. Repo-local runnable gates

- `scripts/sd27-workflow.py` — dispatch-state driver + the only sanctioned writer to the reporting JSON. Contract: `loop-instruction.md §8`.
- `scripts/identifier-discipline-audit.sh` — identifier-discipline half of the dual-audit gate → `OK_NO_BUNDLE_TAGS`.
- `scripts/wired-integration-audit.sh` — four-check wired-integration audit → `AUDIT PASSED`.
- `scripts/architecture-truth-up.sh` — E4.2 architecture truth-up gate.
- `scripts/graphify-update.sh` — E4.2 graphify gate. Replaces the previously-cited `graphify cluster-only`; no `graphify` binary is on PATH.
- `scripts/pcgen-run-character.sh`, `scripts/pcgen-normalize-output.py` — SD-26's PCGen pipeline, consumed unmodified by E3.x.
- `src/oracle_validation/comparator.rs` — SD-26's parity comparator, consumed unmodified.
- `src/bin/sd26_gen_core_rulebook_cache.rs` — the codegen precedent `src/bin/sd27_gen_book_cache.rs` is modelled on.

## 3. Machine-local skills (EXTERNAL — real, but outside this repo)

These resolve under **`$HERMES_HOME/profiles/god-emporer/skills/`** — on this host
`/home/todd/hermes-home/.hermes/profiles/god-emporer/skills/`. They are **not** at `~/.hermes/...` as
previously cited, and they are **not** repo-local. No cycle depends on them being present: the runnable
halves of the two audit skills are vendored into `scripts/` (§2).

- `orchestration/workflow-orchestrated-dispatch/SKILL.md` — the canonical dispatch shape.
- `devops/wired-integration-discipline/SKILL.md` — dual-audit gate partner; the four-check audit is per-cycle.
- `devops/identifier-discipline/SKILL.md` — dual-audit gate partner. Inline-only; ships no script.
- `devops/kanban-claude-code-execution-receipt/SKILL.md` — receipt schema.
- `devops/release-package-promotion/SKILL.md` — workspace → repo publish.
- `dual-canonical-doctrine/SKILL.md` — workspace-citation + repo-local canonical pattern.
- `release-swarm-observer/scripts/pf1e_dashboard_producer_orchestrator_helper.py` — **the sanctioned dashboard writer.** `scripts/sd27-workflow.py` delegates to it; nothing else may write that JSON.
- `release-swarm-observer/scripts/pf1e_dashboard_producer.py` — regenerates the dashboard from v0.6 markdown; preserves manifest `items`/`stats`, reseeds `scope`/`workchannel`.

## 4. Sibling bundles (REPO-LOCAL)

- `../../SD-26-ingest-strategy-and-rule-system-plumbing/` — Tier-1 launch-gate dependency. Ships the Shape B v0 schema, the `book_stub` registry kind, and the 21 future-state book stubs. SD-27 consumes all three and adds the v1 license-aware schema bump (cycle 2.0.5).
- `../../SD-25-ui-evaluation-defect-closure/` — closed predecessor; Tier-1 launch-gate dependency for SD-26.
- `../../SD-24-beta-readiness-and-multiclass/` — closed predecessor.
- `../../SD-23-character-mutation-and-wired-integration/` — closed; canonical cycle-receipt shape.
- `../../v0.6/` — active sidecar; the file-touch partition exists because of it.

## 5. Cross-cutting operator directives

- **2026-07-25 17:39:26** — shape cross for SD-26 (inherited by SD-27); JSON cache is repo-resident at `data/corpus/<book>/`.
- **2026-07-25 18:00:00** — SD-27 ships only 2 future-state books per the "tune, then go wide" model; the rest deferred.
- **2026-07-25 (OGL review)** — per-record `license` field on every Shape B v1 record; 5th dual-audit (PI-blacklist grep); redaction-to-marker policy (`"PI"` → `"PI-REDACTED"` with `[redacted PI]` value).
- **2026-07-25 18:30:00** — per-cycle tier model; Sonnet default, free/discounted operator-authorized for per-book cycle bodies.
- **2026-07-25 19:00:00** — SD-27 does not author class engines; v0.6 owns chassis breadth.
- **2026-07-27** — Beginner Box and Core Essentials removed from scope (redundant to other tomes). Bundle book-count: 21 → 19 future-state. Their stubs and registry slots remain on disk, out-of-scope rather than deleted.
- **2026-07-27** — the in-scope pair is **Advanced Race Guide + Pathfinder Unchained**, matching the operator's `SD-27 (ARG + PU)` dashboard workchannel. Adventurer's Guide is routed to SD-30. An earlier draft of this bundle named ARG + Adventurer's Guide throughout; that pairing is superseded.

## 6. External (outside the repo)

- `$PCGEN_DATA_ROOT/` (default `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/`) — the PF1 book directories. Verified present 2026-07-27: `advanced_race_guide` (23 `.lst`), `pathfinder_unchained` (11 `.lst`). `scripts/sd27-workflow.py preflight` asserts both.
- `~/workspace/repos/pcgen/gradlew` + `code/testsuite/base-xml.ftl` — PCGen Gradle headless route.
- `$PF1E_JSON_PATH` (default `/home/todd/hermes-home/swarm-observer/PF1e-dashboard.json`) — the operator's reporting dashboard; manifest `sd27_book_pre_build`.

## 7. Repo-local test fixtures

- `tests/fixtures/rules_core/pf1_*_level*_*.txt` — deterministic character inputs (cache inputs).
- `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt` — SD-26's pilot case for E2 verification; SD-27's E3.x cycles mirror this pattern for ARG and PU.
