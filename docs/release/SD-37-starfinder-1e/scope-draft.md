---
canonical: true
owner: operator
bundle_id: SD-37
date: 2026-10-02
status: draft — authored under operator-away defaults; operator sign-off pending (see README §6)
---

# SD-37 Scope — Starfinder 1e

## Operator intent (on record)

- **2026-09-07** (SD-35 `decisions.md §11`, verbatim): *"when we finish pathfinder1e - we do have
  other game systems to convert. Do[n't] start deleting our conversion engines and test oracles as
  soon as we finish pf1e, because right after that we are going to start on starfinder."*
- **2026-09-15** (SD-36 scope-draft, goal): *"a tight, clean codebase before UI work and before
  Starfinder; nothing of PCGen in live code, enforced by the build not by a script"*.
- **2026-10-02** (this bundle's brief): Starfinder 1e, thorough but with bounded scope. Prove it on
  one book, then go wide. The operator is away. Every scoping ruling below is an operator-away
  default (`decisions.md`).

## Bundle intent

Campaign Codex generates **Starfinder 1e character sheets** alongside PF1e. Sheet totals are
computed from converted data with a **generic** chassis; every other rule is printed
(`decisions.md §5`). PF1e must be **byte-identical** before and after. SD-37 also moves
`rules_tables` out of Rust into a data package, on a parallel track.

## Definition of Done

All of the following must hold. Each item maps to cards in `epic-breakdown.md`.

1. The SF oracle is admitted: the sparse pin covers `data/starfinder` and
   `system/gameModes/Starfinder`, and this is proven in a fresh clone (E0.1). The licence matrix
   holds a row for every SF book, and the SF PI term set is in place (E0.2). The SF denominator
   comes from a fail-closed sum (E0.3).
2. The rule package is partitioned by game system, with PF renders byte-identical for Aldric and
   Elowen (E1).
3. The schema is extended additively and published under `schemas/rules/` (E2).
4. The SF converter (SD-35 C2.1, adopted) reads all 8 in-scope books. The overloaded PCGen fields
   map through an SF-only table that has an oracle row for each field. Gates: `--check`, residue,
   PI, structural diff (E3).
5. SF sheet totals are computed generically: BAB, saves, HP, Stamina, Resolve, EAC, KAC,
   initiative, skills, spells 0–6, credits, bulk. No per-class modules. `StarfinderAdapter`
   replaces stub 0002 for SF (E4).
6. SF print-path content is in place: races, themes, class features, feats, spells, equipment,
   augmentations and drone (E5).
7. Desktop: the landing screen routes `starfinder-1e` to the real adapter, with SF creation, sheet,
   catalogs and level-up. No hand-kept tables. All six seeds open in the real app under an isolated
   data root (E6).
8. `rules_tables` is a data package: 252 importers re-pointed, the `.lst` citation burn-down done,
   PF parity held (E4a).
9. SF oracle parity holds with an explicit "not covered" list. Widest-scope verify passes in both
   workspaces. Closure runs in the corrected order (E7).

**Not in the Definition of Done:** starship sheets (planned deferral DEF-1, `decisions.md §17`);
the excluded books SSRGG and LPJ (`decisions.md §6`); and the PF correctness candidates in
`forward-scope-register.md`.

## Book scope

- **Proof book:** `paizo/core`.
- **Go wide:** `armory`, `character_operations_manual`, `pact_worlds`, `near_space`,
  `alien_archive{,_2,_3}`.
- **Excluded:** `starfinder_society_rules`, `lpj_design/infinite_space`.

Row counts are in `content-unit-inventory.md §2`: 12,733 in-scope LST rows, which are **not**
units. Units come from E0.3.

## Epic order

```
C0 → C1 → { E0 ∥ E1 } → E2 → E3 → { E4 ∥ E5 } → E6 → E7
                    E1 → E4a (parallel track, joins before E7.2)
```

## Cycle dispatch model

One long `Workflow` run (`workflow-instruction.md §2.4`). Tiers: Opus by default, Sonnet for
mechanical work and long waits, Haiku for housekeeping. Merge checks always run on Opus. Batches
are big (all of a homogeneous remainder per dispatch), and each wave gets one verify.

## Verification (end to end)

See `acceptance-and-verification.md`. In short: PF hash set equal; SF seeds equal to the SRD hand
values; `cargo test` at the widest scope in both workspaces; SF oracle parity with a stated "not
covered" list; the sparse-path fix proven in a fresh clone; CI green on `tranche/17`.
