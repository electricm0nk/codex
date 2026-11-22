# SD-14 Character Lifecycle, Persistence, and Upgrade-Safe Revision Risks and Open Questions

## Purpose
This file quarantines the unresolved decisions and risk posture for SD-14 so the main contract stays concrete without pretending the open questions are already solved.

## Active risks

### R1 — Storage technology drift disguised as architecture
Risk:
- implementation may jump directly into a convenient local format and later discover it cannot support explicit revision, compatibility, diagnostics, or recovery posture cleanly

Why it matters:
- the first persistence format becomes folklore quickly; changing it later without a declared migration posture risks tester data loss

Required guardrail:
- later handoffs must preserve the logical contract first and choose storage technology second

### R2 — Derived snapshot inflation
Risk:
- implementers may serialize convenient computed snapshots and treat them as canonical character truth

Why it matters:
- this would freeze accidental current behavior as saved truth and make later recomputation drift impossible to classify honestly

Required guardrail:
- authoritative-versus-derived separation must remain explicit in all later handoffs and tests

### R3 — Silent migration optimism
Risk:
- version/content drift may be “handled” by silently coercing, dropping, or recomputing unsupported state

Why it matters:
- silent success is counterfeit durability; testers lose trust precisely when saved state seems to survive until it matters

Required guardrail:
- blocked/read-only/migrated states must stay explicit and evidence-bearing

### R4 — Scope creep into roster, campaign, or cloud features
Risk:
- because lifecycle language is broad, later work may smuggle in library breadth, sharing, accounts, or world-state persistence

Why it matters:
- the saved-character continuity lane would become a product rewrite instead of a bounded continuity contract

Required guardrail:
- local-first single-user saved-character continuity remains the only authority for this lane

## Open questions

### Q1 — What is the minimum truthful persisted unit for the first executable slice?
Current posture:
- the packet requires a saved-character artifact plus some catalog/indexable surface, but does not yet force whether the first slice exposes a single-slot workflow, a bounded local library, or a slightly richer local catalog

Why unresolved:
- the strategic spec domain intentionally left this open; the current repo still lacks a live character-save boundary to constrain the choice

Allowed interim posture:
- later execution may choose the narrowest option that still preserves create/open/save/reopen/duplicate/archive/delete truth honestly and does not counterfeit breadth

### Q2 — Which derived fields, if any, should be cached inside the saved artifact?
Current posture:
- the contract requires authoritative user-authored state and permits subordinate caches, but does not yet freeze the cache shape

Why unresolved:
- the right cache surface depends on later runtime/UI seams and compatibility behavior

Allowed interim posture:
- caches may be omitted or invalidated freely as long as authoritative state remains complete and reopen remains honest

### Q3 — What minimum autosave / backup depth is required?
Current posture:
- the packet requires autosave/recovery posture but does not yet fix exact depth, retention count, or UI wording

Why unresolved:
- loss-tolerance depends on later execution cost and UX evidence

Allowed interim posture:
- later handoffs must name exact autosave depth and recovery semantics before implementation

### Q4 — What is the exact missing-dependency posture when content packages evolve?
Current posture:
- the packet requires explicit classification and diagnostics but does not yet fix whether partial read-only reconstruction is required for every missing-content case

Why unresolved:
- the right answer depends on the later canonical content/versioning seam and how much unsupported state can still be inspected safely

Allowed interim posture:
- if safe editable reopen cannot be proven, read-only or blocked posture is preferred over optimistic coercion

### Q5 — How should rollback under SD-12 interact with saved-character compatibility history?
Current posture:
- the packet requires SD-12 coupling but does not yet define the exact user-visible downgrade/recovery language or history retention semantics

Why unresolved:
- this depends on later bounded release-truth and recovery-consumer seams from SD-12

Allowed interim posture:
- later handoffs must preserve explicit classification and recovery evidence; no silent downgrade-state mutation is permitted

## Forbidden assumptions
- do not assume that storage format choice is a trivial local concern
- do not assume migration is always possible
- do not assume archived/deleted state can be treated as operationally identical
- do not assume save-file attachment capability in SD-11 means save files already exist in the runtime
- do not assume GE-08 package lifecycle semantics can be copied directly onto character lifecycle without review

## Shortest path back into closure
If later work needs to reopen this packet for readiness repair, the shortest path is:
1. verify whether repo truth has added a real character save/load seam
2. verify whether SD-12 has added new rollback/recovery authority that changes saved-state expectations
3. patch the relevant artifact contract(s)
4. re-audit whether the epic breakdown still routes the truth honestly
