# SD-27 — Forward-Scope Register

> **Status:** planning-ready. Loader action: read this register first, then `scope-draft.md`. The `scope-draft.md` is the bundle's committed scope; this register is the disagreement surface that the operator signs off on before authoring tightens.
>
> **Authored:** 2026-07-25 (god-emporer, operator directive 2026-07-25).
> **Predecessor:** SD-26 (closure-ready per `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/`, PR #338 awaiting operator merge).
> **Sidecar:** v0.6-alpha release-swarm (active on `tranche/6`, not yet closed).

This register separates predecessor-deferred work into three classes. The class determines whether SD-27 owns it (class 1), sequences it (class 2), or stays clear of it (class 3). Per `spec-domain-bundle-authoring` v1.2.0 §"Forward-scope extraction from predecessor bundle/lane docs" — the three-class separation is load-bearing; do not collapse it into a flat backlog.

---

## Class 1 — Committed payloads (with documentary citations)

These are the predecessor's explicit, by-name successor bindings. The strongest authority. Non-negotiable.

### 1.1 The 19 future-state book `book_stub` entries

**Source:** SD-26 Epic 4 (criteria 4.1-4.20), `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/release-notes.md:29-31`, `docs/governance/wired-integration-stubs-registry.md` entries #0003-#0021, `data/stubs/*.json` (19 files).

**What is committed.** SD-26 registered 19 PF1 sourcebooks as `book_stub` entries in the Stubs Registry and wrote `data/stubs/<book>.json` manifests for each, with `content_kind_counts: null` (honest gap) and a `planned_resolution_bundle` field pointing at SD-27. **Per operator directive 2026-07-27, Beginner Box and Core Essentials have been removed from scope** (redundant to other tomes; will not be brought in); their registry slots (#0005 and #0012) and stub manifests, if they exist on disk, are out-of-scope and may be deleted by the closure epilogue with operator authorization. The 19 remaining books are:

| # | Book | Stub manifest | Registry entry |
|---|------|---------------|----------------|
| 1 | advanced_race_guide | `data/stubs/advanced_race_guide.json` | #0003 |
| 2 | adventurers_guide | `data/stubs/adventurers_guide.json` | #0004 |
| 3 | bestiary_2 | `data/stubs/bestiary_2.json` | #0005 |
| 4 | bestiary_3 | `data/stubs/bestiary_3.json` | #0006 |
| 5 | bestiary_4 | `data/stubs/bestiary_4.json` | #0007 |
| 6 | bestiary_5 | `data/stubs/bestiary_5.json` | #0008 |
| 7 | bestiary_6 | `data/stubs/bestiary_6.json` | #0009 |
| 8 | bonus_bestiary | `data/stubs/bonus_bestiary.json` | #0010 |
| 9 | horror_adventures | `data/stubs/horror_adventures.json` | #0011 |
| 10 | monster_codex | `data/stubs/monster_codex.json` | #0012 |
| 11 | mythic_adventures | `data/stubs/mythic_adventures.json` | #0013 |
| 12 | occult_adventures | `data/stubs/occult_adventures.json` | #0014 |
| 13 | pathfinder_unchained | `data/stubs/pathfinder_unchained.json` | #0015 |
| 14 | ultimate_campaign | `data/stubs/ultimate_campaign.json` | #0016 |
| 15 | ultimate_combat | `data/stubs/ultimate_combat.json` | #0017 |
| 16 | ultimate_equipment | `data/stubs/ultimate_equipment.json` | #0018 |
| 17 | ultimate_intrigue | `data/stubs/ultimate_intrigue.json` | #0019 |
| 18 | ultimate_magic | `data/stubs/ultimate_magic.json` | #0020 |
| 19 | ultimate_wilderness | `data/stubs/ultimate_wilderness.json` | #0021 |

**Each `book_stub` entry carries the documentary pointer**: `planned_resolution_bundle: "SD-27+ (unscheduled)"` (with the discrepancy against `decisions.md:102`'s `"SD-27"` — see §1.2 below).

**SD-27's obligation.** Resolve each stub into a real Shape B JSON cache at `data/corpus/<book>/` (per SD-26's Epic 3 schema, `decisions.md §7`), with the per-book content funnels populated where the source LST corpus supports it. The `content_kind_counts` field gets a real value rather than `null`. The Stubs Registry entry's `Status` flips from "Registered stub" to "Resolved" with a date and the bundle's cycle receipt pointer.

**Out-of-scope for SD-27 (this class).** Designing new rules or new chassis for these books. SD-27 is content ingestion + parity baseline, not engine design. The 19 books' rule systems are PF1 continuations of the CRB chassis already wired in SD-22/SD-26; no new class engines, no new data shapes, no new rule mechanics.

### 1.2 The `planned_resolution_bundle` label discrepancy

**Source:** `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/decisions.md:102` ("SD-27") vs. all 19 `data/stubs/*.json` files + `wired-integration-stubs-registry.md` entries #0003-#0021 ("SD-27+ (unscheduled)").

**What is committed.** SD-26's own closure-readiness report and Epic 6 final-criterion scan explicitly forwarded this as a real, unresolved operator decision. Cited verbatim: `decisions.md:102` says the literal `"SD-27"`; the landed stubs and registry entries say `"SD-27+ (unscheduled)"`. Two internally consistent sets, mutually contradictory.

**SD-27's obligation.** Resolve the discrepancy as the bundle's first cycle. Two acceptable resolutions:

- **(a)** Rename all 19 `data/stubs/*.json` files + the 19 `wired-integration-stubs-registry.md` entries to `"SD-27"`. Update `risks-and-open-questions.md Q2` to record the rename. Update `decisions.md:102` to reflect the choice.
- **(b)** Keep all 19 entries at `"SD-27+ (unscheduled)"` and rename `decisions.md:102` to `"SD-27+ (unscheduled)"` to match.

**Per-cycle blocking decision.** Whichever way the operator pulls, the bundle's first cycle (1.1) lands the resolved label across all 20 surfaces (19 stubs + 1 `decisions.md`) before any further cycle dispatches. The lead does not pick a side; both are internally consistent.

### 1.3 SD-22's deferred OGL/PI license-stripping (operator directive 2026-07-25)

**Source:** SD-22's own closure-readiness report deferred "Product Identity handling per OGL" as out-of-scope; the 4 in-scope books (CRB, APG, ACG, Bestiary 1) were ingested with inlined OGL-licensed content but no PI-stripping pass. The operator's 2026-07-25 OGL review confirmed this is a real liability that must be closed before any of the 19 future-state books fan out.

**What is committed.** SD-27 ships Shape B v1, a license-aware extension of SD-26's Shape B:

- **Per-record `license` field:** `"OGL" | "PI" | "PI-REDACTED"`. Every Shape B record carries exactly one of these.
- **Per-record `pi_field` and `pi_marker` fields** for PI-tagged records. `pi_marker: "redacted"` is the only acceptable marker for the first cut.
- **Per-book `LICENSE.json`** at `data/corpus/<book>/LICENSE.json`. Declares the OGL/PI split, the redaction policy, the redistribution posture (CC-BY-compatible, OGL-notice-attached, etc.).
- **PI-blacklist:** the per-field list of what's PI vs OGL (initial: `deity`, `deity_name`, `npc`, `npc_name`, `monster_name` (non-bestiary), `place_name`, `faction_name`, `deity_portfolio`, `art_url`, `fiction_text`, `book_cover`, `monster_description` (flavor)). Per-book, not per-record — Paizo's PI list varies by book.
- **Redaction-to-marker policy:** PI-tagged field values become `"[redacted PI]"`, not omitted. Preserves schema; downstream code reads one branch per field.

**Why class 1 (not class 2).** This is not a candidate for SD-27 to skip. The 19 future-state books (also OGL) inherit the same Shape B pattern; without the license-stripping pre-flight, all 19 books ingest with inlined PI. The 4 in-scope books' SD-22 shape is the load-bearing precedent.

**5th dual-audit (PI-blacklist grep).** The license-stripping cycles run a 5th audit on top of the standard 4-grep dual-audit: for every record in `data/corpus/<book>/`, every field value that matches a PI-blacklist pattern must have `license: "PI" | "PI-REDACTED"` and `pi_marker: "redacted"`. A record with a PI-matching value and `license: "OGL"` is a license-defect and the cycle fails.

**The 4 in-scope books' retro-fit is rolled into SD-27** (cycles 2.0.6-2.0.9, one cycle per in-scope book, file-disjoint, parallel-safe). The retro-fit is NOT a separate bundle — per the operator's 2026-07-25 directive, the in-scope books' liability must close before any future-state book fans out, and the per-book cycles (2.1-2.2) inherit the new shape.

**Per-cycle blocking decision.** Cycle 2.0.5 lands the schema + the initial PI-blacklist. Cycles 2.0.6-2.0.9 apply the blacklist against the in-scope books; discovered-PI-fields that are not in the initial blacklist get added to the blacklist (one source-of-truth file, versioned per cycle). Cycle 2.0.10's dual-audit gate verifies the blacklist is exhaustive across all 23 books.

---

## Class 2 — Structurally implied by closure gaps (with predecessor item numbers)

These are items the predecessor's own closure report or final-state summary identified as remaining, but did not commit to a specific bundle. SD-27 must decide whether to own, sequence, or carve them out.

### 2.1 Equipment-attachment schema (v0.6 risks item 1, SD-26 follow-up)

**Source:** `docs/release/v0.6/risks-and-open-questions.md:54-64`, `docs/release/v0.6/item-1-architecture-wall-design.md` (whole doc).

**What is implied.** `EquipmentSelection` is a flat `{item_id, equipped_or_active, active_state}` shape with no concept of one modifier attaching to another. The headless/corpus-aware architecture wall blocks real attack-bonus and armor-check-penalty math. The v0.6 swarm designed three fix shapes (a, b, c) and the operator chose to widen the posture gate to accept any equipment (item 27); the design and sequencing plan is in active scoping.

**Currently-in-progress filter.** v0.6 is actively scoping this — `SWARM_REPORT.md:54-63` records the operator's decision and the active wave of backend work. **NOT class-1 for SD-27.** This is v0.6 work, not deferred to SD-27.

**SD-27's obligation.** Defer to v0.6 close. If v0.6 finishes with a settled equipment-attachment schema, SD-27's per-book caches can use it. If v0.6 finishes with a deferred/open architecture decision, SD-27's per-book content ingestion does not depend on it (the stub → cache pipeline is content-shape, not equipment-attachment). Re-evaluate at v0.6 close.

### 2.2 Feat-effects engine (v0.6 risks item 17)

**Source:** `docs/release/v0.6/risks-and-open-questions.md:226-263`. Toughness + 3 save-boosting feats landed in v0.6 (`f38e9f33`, `53ddd1ce`).

**Currently-in-progress filter.** v0.6 closed item 17 at the bounded 4-feat scope. The remaining feat catalog (185 CRB feats, ~500+ across APG+ACG+B1) is not yet sequenced. **NOT class-1 for SD-27** — the per-book content ingestion for the 19 future-state books does not include engine-level feat-effects wiring beyond what is already in scope of the per-book chassis.

**SD-27's obligation.** Stays clear. The 19 books' feat catalogs are ingested as content (the `feats` corpus), not as engine effects. Per-feat effects continue to be a separate, slower program-long workstream.

### 2.3 Class-skill-list recognition beyond Fighter/Wizard/Rogue (v0.6 risks item 1 sub-problem)

**Source:** `docs/release/v0.6/risks-and-open-questions.md:42-44`.

**Currently-in-progress filter.** v0.6 is widening class-skill recognition work as part of the broader class/race breadth effort. The lead's response confirms it's a "labor-volume hand-authoring problem, not an architecture wall."

**SD-27's obligation.** Defer. Class-skill grounding is engine work, not content ingestion. Out of SD-27's lane.

### 2.4 Multiclass / class-chassis breadth (8 of 11 CRB classes)

**Source:** `docs/release/v0.6/SWARM_TASKS.md:81`, `docs/release/v0.6/SWARM_REPORT.md:38-41`.

**Currently-in-progress filter.** v0.6 is actively working on this — the operator confirmed on 2026-07-25 that Bard and Bardic Performances are in progress. Backend work is ongoing through Bard, Cleric, Druid, Barbarian, Monk, Paladin, Ranger, Sorcerer.

**SD-27's obligation.** Defer. v0.6 owns the chassis breadth work. SD-27 does not author class engines. **This is the critical class-2 item that v0.6's live work most directly overlaps with.** SD-27's content ingestion for the 19 books is independent of which CRB classes have chassis support — `data/corpus/<book>/` is data, not engine code.

### 2.5 Companion / animal / familiar stat-block engine

**Source:** `docs/release/v0.6/risks-and-open-questions.md:107`.

**Currently-in-progress filter.** Not in v0.6 scope. The Pets tab stays as a "coming soon" stub. The operator accepted this as a non-goal for the alpha.

**SD-27's obligation.** Stays clear. No stat-block engine exists for companions. The 19 books' content is character-centric; companion stat blocks are not in any of the SD-26 future-state books' content scope.

### 2.6 Starting wealth for non-CRB classes

**Source:** `docs/release/v0.6/risks-and-open-questions.md:143-145`, `docs/release/v0.6/SWARM_REPORT.md:39-41`.

**Currently-in-progress filter.** v0.6 implemented starting wealth for the 11 CRB classes (`0dbf67ad`). The 12 APG/ACG classes in the operator's table are blocked because the chassis is not yet supported for those classes.

**SD-27's obligation.** Defer. Class identity is prerequisite for starting wealth. The 19 future-state books' classes are not the same classes as the 12 APG/ACG ones — the 19 books are Bestiary-2-6, Ultimate-line, etc. — and starting wealth for those classes is a separate engine workstream.

---

## Class 3 — Candidate only (no documentary backing from predecessor)

These are real but require their own justification in SD-27. Cannot be carried forward on the predecessor's authority alone.

### 3.1 Equipment-attachment schema implementation (v0.6 scoping carried to engine work)

**Source of claim:** v0.6 design pass (`docs/release/v0.6/item-1-architecture-wall-design.md`).

**Why class-3, not class-1.** v0.6 has not yet committed to a fix shape. The lead's note "operator decides items 1/18/27 — new wave of backend work" (`docs/release/v0.6/risks-and-open-questions.md:64`) puts the decision back in the v0.6 lane. Until v0.6 closes, this is a candidate.

**If SD-27 wanted to claim it.** Would need a separate scope-draft cycle and a discrete class-skill-list engine pass. Not realistic as part of the 19-book ingest.

### 3.2 Arcane-school selector (v0.6 risk item 1 follow-up)

**Source of claim:** `docs/release/v0.6/risks-and-open-questions.md:48` ("A real UI-facing arcane-school selector (frontend's Option A) is a separate, larger feature — **backlog, not blocking**").

**Why class-3.** Backlog, not committed. No commitment from v0.6 to a specific bundle.

**If SD-27 wanted to claim it.** Would need explicit operator greenlight. Out of scope here.

### 3.3 Temporary HP / favored-class-bonus HP wiring (v0.6 risks item 4 sub-items)

**Source of claim:** `docs/release/v0.6/risks-and-open-questions.md:99-100`.

**Why class-3.** v0.6 explicitly flagged these as deferred, not committed. Out of scope for content ingestion.

### 3.4 Unequip / remove-equipment UI affordance (v0.6 risks item 28)

**Source of claim:** `docs/release/v0.6/risks-and-open-questions.md:295`.

**Why class-3.** v0.6 documented this as a future UI-affordance backlog, not a content or engine item.

---

## Class 0 — Prerequisite (resolve before any cycle dispatches)

The lead cannot pick a class-1b split. The operator must.

### 0.1 Bundle label

`SD-27` vs. `SD-27+ (unscheduled)`. Already named in class 1.2. Promoted to class-0 here because the wiring is **a per-cycle blocking decision** — every cycle that touches a `data/stubs/*.json` file or a registry entry block on it. The bundle's first cycle (1.1) lands the resolved label.

### 0.2 v0.6 close

v0.6's terminal state matters for SD-27's file-touch partition. **Filetouch partition during v0.6 close:** SD-27 cycles must not touch `src/rules_core/pilot_compute.rs`, `src/rules_core/rules_tables/<book>/` for any not-yet-resolved book, or `data/corpus/<book>/` for any book other than CRB/APG/ACG/Bestiary 1. Ingestion cycles only write to `data/corpus/<book>/` for the 19 future-state books and to `docs/governance/wired-integration-stubs-registry.md`. The partition is enforceable via the same worktree-based isolation SD-26 Epic 3 used.

### 0.3 CG-03 (Human ability-modifier bug)

`src/rules_core/pilot_compute.rs:4743-4767` still incorrectly derives modifiers from raw chosen score, never applying the Human +2 racial bonus. v0.6 is working on this; the SD-27 parity baseline for `pf_<book>_*` fixtures will inherit the current 7-of-9 match rate (with the 2-climb/swim mismatch from CG-03) unless v0.6 closes it. SD-27 does not own CG-03 but does inherit the consequence: the parity baseline for each of the 19 books is "match rate at the time of cycle close," not "9-of-9 fully oracle-checked."

---

## Class excluding — operator passed over deliberately

These are items that have been named in conversation or noted in v0.6 docs but should not enter SD-27. Documented so the operator can confirm the carve-out rather than re-litigate.

- **Animal companions / Pets / familiars** — `docs/release/v0.6/risks-and-open-questions.md:107`. Operator confirmed non-goal for alpha. Carved out: not v0.6, not SD-27, future program.
- **Parameterized feats (Skill Focus with a chosen skill, Teamwork feats, etc.)** — `docs/release/v0.6/risks-and-open-questions.md:209-230`. Carved out until a general feat-effects engine exists.
- **Temporary HP / favored-class-bonus HP** — `docs/release/v0.6/risks-and-open-questions.md:99-100`. Carved out: durability implementation is at the character-record level, not the per-book content level.
- **Multiclass durability-level ordering** — `docs/release/v0.6/SWARM_REPORT.md:485-505`. Carved out: Shape B JSON cache does not encode level-ordering ambiguity.

---

## Pick-three summary

The operator's three lever decisions for SD-27:

1. **Class 1 — accept in full.** The 19 future-state book stubs are the canonical payload. SD-27 commits to resolving each into a real Shape B JSON cache.
2. **Class 2 — defer to v0.6 close.** Equipment-attachment, feat-effects, class-skill, multiclass, starting wealth, companion all-stay clear. Re-evaluate at v0.6 close.
3. **Class 3 — drop.** Arcane-school selector, temporary HP, unequip UI all drop. Not in scope of this bundle.

The bind points the operator must explicitly resolve before cycle dispatch:

- **0.1** — Bundle label (`SD-27` vs. `SD-27+ (unscheduled)`). One cycle, calendar-day 1.
- **0.2** — v0.6 close coordination. The 19-book cycles can run in parallel with v0.6's class work provided the file-touch partition holds.
- **0.3** — CG-03 disposition. SD-27's parity baselines inherit the current 7-of-9 baseline. Not a blocker; a documentation point.

---

## Pitfalls (do not repeat)

- **Don't list v0.6 in-progress work as class-1.** Bard + Bardic Performances are v0.6's lane, not SD-27's. The "currently-in-progress" filter exists precisely to prevent this.
- **Don't fold the operator-passed-over items into class-1.** Animal companions, parameterized feats, temporary HP, multiclass durability ordering: each has been explicitly deferred by the operator. SD-27 inheriting them would be a scope takeover.
- **Don't promote the bundle label to a phantom compromise.** Either `SD-27` or `SD-27+ (unscheduled)` — not `"SD-27 (or 27+)"` or other hedging. Pick one, propagate, done.
- **Don't author the `scope-draft.md` until the operator accepts this register.** The register is the artifact the operator signs off on; everything downstream is derivative.
