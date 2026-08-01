# SD-29 — Bestiary-Line Book Ingestion

> **⚠️ STATUS: SCOPE PASS ONLY — NOT PLANNING-READY, NOT EXECUTION-READY ⚠️**
>
> This directory holds SD-29's **inherited scope**, authored 2026-08-01 by the tranche/7-1 debt
> cycle so that the work SD-27 routed forward has somewhere to land that is not a retro shard.
> It is **not** a dispatchable bundle. Two of the canonical ten chassis files exist; the other
> eight are deliberately absent, and §4 says which and why.
>
> **Read first:** `./forward-scope-register.md`. Per `spec-domain-bundle-authoring` v1.2.0 and
> SD-27's own recorded pitfall — *"Don't author the `scope-draft.md` until the operator accepts
> this register"* (`../SD-27-future-state-book-content-ingestion/forward-scope-register.md`,
> "Pitfalls") — the register is the artifact the operator signs off on, and everything downstream
> is derivative. Authoring a `scope-draft.md` here before that sign-off would invert the order the
> predecessor bundle explicitly warns against.

## 0. Bundle at a glance

- **Slug:** `SD-29-bestiary-line-book-ingestion`
- **Workchannel:** `SD-29 (Bestiary)` — the operator's dashboard routing, transcribed at
  `../SD-27-future-state-book-content-ingestion/epic-breakdown.md:150`.
- **Predecessor:** SD-27 (Future-State Book Content Ingestion), branch `tranche/7`.
- **Sibling bundles:** SD-28 (Ultimate line, 6 books), SD-30 (Adventure+, 4 books). Same
  `epic-breakdown.md` table, rows 149 and 151.
- **Books:** 7 — `bestiary_2`, `bestiary_3`, `bestiary_4`, `bestiary_5`, `bestiary_6`,
  `bonus_bestiary`, `monster_codex`. All seven are **Tier-1** per SD-27 `decisions.md §9`.
- **Registry entries:** `#0006`–`#0011` and `#0014` in
  `docs/governance/wired-integration-stubs-registry.md`, with stub manifests at
  `data/stubs/<book>.json` carrying `content_kind_counts: null`.
- **Epics / criteria:** not yet authored. See §4.

## 1. What this bundle ships

The resolution of **7** of SD-26's 19 registered `book_stub` entries into real Shape B v1 JSON
caches at `data/corpus/<book>/`, following the per-book 4-stage cycle (license → pre-build →
verify → parity) that SD-27 proved on Advanced Race Guide and Pathfinder Unchained.

**One of those seven closes a player-visible gap that nothing else can close.** Monster Codex is
the sole source, anywhere in the PCGen checkout, of the flag that grants
`Duergar ~ Spell-Like Ability ~ Invisibility` — the single remaining reach NO in the whole
project. That is derived, executable, and re-verified here rather than inherited; the full
derivation with commands is `forward-scope-register.md §1.2`.

## 2. What this bundle does not ship

- **The 6 Ultimate-line books** — SD-28's workchannel. Two live deferrals are blocked
  specifically on Ultimate Magic + Ultimate Combat and must **not** be pulled in here
  (`forward-scope-register.md §4.1`).
- **The 4 Adventure+ books** (`adventurers_guide`, `mythic_adventures`, `occult_adventures`,
  `horror_adventures`) — SD-30's workchannel.
- **Engine work.** No new class chassis, no formula interpreter (SD-27 `decisions.md §24`
  stands), no archetype engine, no companion/familiar stat-block engine.
- **The ~40 unrouted engine and UI deferrals** in `docs/retro/events/`. They are real; none of
  them is SD-29's by any documentary authority, and the register classifies them as candidates
  rather than quietly annexing them (`forward-scope-register.md §3`).

## 3. Why this directory exists now

SD-27 closed with its forward-routed work living **only** in append-only retro shards. 74
deferrals across 40 shard files is not a scope surface: it has no ordering, no ownership, no
class separation, and no operator sign-off point. The single deferral that names SD-29 by name
(`docs/retro/events/record-gaps.jsonl`) would have been findable only by grepping 52 files.

This directory converts that into the shape the repo's own doctrine expects, and — importantly —
**does not inflate it**. The honest headline is in the register: **exactly one** of the 74
deferrals routes itself to SD-29. Everything else here is either derived from the corpus, cited
from `epic-breakdown.md`, or explicitly marked as a candidate the operator must claim.

## 4. Chassis files deliberately not authored

Per `no-stub-mvp-doctrine.md`, eight empty template files would be eight stubs. They are named
here so their absence is a recorded decision rather than an oversight.

| File | Why not yet |
|---|---|
| `scope-draft.md` | Blocked on operator acceptance of the register — SD-27's own pitfall list. |
| `decisions.md` | A decision record with no decisions in it. Seeded by the first cycle. |
| `technical-design.md` | The Shape B v1 pipeline is SD-27's and unchanged; nothing new to design until the corpus survey in `forward-scope-register.md §1.3` is acted on. |
| `technical-requirements.md` | Derivative of the scope-draft. |
| `epic-breakdown.md` | Cycle count depends on the per-book shape, which §1.3 shows is **not** uniform across these 7 books. Writing it now would repeat the "~250-300 monsters each" error the register corrects. |
| `loop-instruction.md` | Copied and adapted from SD-27's at dispatch time, not before. |
| `progress.md` | Live cycle log; nothing has run. |
| `release-notes.md` | Generated at the closure epic. |
| `acceptance-and-verification.md` | Derivative of the criteria, which do not exist yet. |

`artifacts/` exists and is empty by design — per-cycle receipts land there.

## 5. Cross-reference

- `./forward-scope-register.md` — the planning entry point and the only load-bearing file here.
- `../SD-27-future-state-book-content-ingestion/` — predecessor; source of the 4-stage per-book
  cycle, Shape B v1, and the `epic-breakdown.md:150` workchannel routing.
- `../SD-27-future-state-book-content-ingestion/artifacts/cross-bundle-findings-2026-07-30.md` §5
  — the corpus facts that invalidate SD-29's original monster-count estimate.
- `tests/sd27_duergar_invisibility_sla_is_upstream_blocked.rs` — the executable proof of §1's
  claim. It goes **red** the day Monster Codex is ingested; that is how the finding closes.
- `docs/retro/events/*.jsonl` — the raw deferral log this register routes.
- `docs/governance/wired-integration-stubs-registry.md` — entries `#0006`–`#0011`, `#0014`.
