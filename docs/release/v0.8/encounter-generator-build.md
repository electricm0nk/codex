---
title: v0.8 stretch — Encounter generator v1 (strawman) build brief
status: active
scope: apps/desktop/src-tauri (one command), apps/desktop/src/dmToolkit (one screen)
date: 2026-09-02
---

# Encounter generator v1 — strawman

Operator directive 2026-09-02: build the party-strength encounter generator as a strawman, same
terms as the DM Toolkit — something to react to, adjusted in v0.9.

## This is a BRIDGE, not engine work — B14 was too pessimistic

`scout`'s B14 said rating encounters against real monsters "needs `encounters.rs` to take the
Bestiary record, not a CR, and needs a Tauri command. Both are engine + backend, neither exists."
The second half is right; **the first half is wrong for a strawman**, verified by reading:

- `src/rules_core/encounters.rs` — `Encounter::new(party: &[CharacterSnapshot], monsters:
  &[MonsterRef]) -> EncounterResult { difficulty, average_party_level, encounter_level }` is
  **pub**. `CharacterSnapshot::new(level: u8)` and `MonsterRef::new(challenge_rating: f32)` are pub.
- `apps/desktop/src-tauri/src/monster_catalog.rs:401` — the catalog DTO **already carries
  `challenge_rating: f32`**, parsed from the corpus.
- `main.rs` — `grep -c encounter` → **0**. The command is the only missing piece.

CR is exactly what PF1 encounter math consumes, so catalog monster → `MonsterRef` is a mapping, not
a new engine capability. **No repo-root `src/` edit. §4.2 is not breached.** B14 stands only for the
richer monster shape a fuller feature would want.

## Two honesty requirements — these are the whole risk

The engine's own doc comments disclose two limits. **Both must reach the DM, not be smoothed over.**

1. **`VERIFIED_XP_TABLE` covers CR 1–10 only** (`encounters.rs`). What the engine does for CR 0,
   fractional CRs (1/8, 1/4, 1/2 — common for low-level encounters) and CR 11+ must be established
   by reading, and whatever it does must be surfaced. If a CR outside the verified range yields an
   approximation, the UI says so on that encounter. Do not present an unverified number as verified.
2. **Difficulty is FOUR tiers** (Easy/Medium/Hard/Deadly) **collapsed from the rulebook's five** —
   the module doc calls this the "Difficulty tier collapse". A DM who knows PF1 will expect five.
   Surface the deviation where the rating is shown rather than letting them assume a rulebook tier.

A generator that quietly rounds is worse than none: the DM takes it to the table and a "Medium"
fight kills the party.

## Tickets

**E-1 `[backend]` — `rate_encounter` command.** Wrap `Encounter::new`. Take party member levels and
monster selections (catalog ids resolved to CR through `monster_catalog`, or explicit CRs — your
call, but a caller must be able to name a real monster). Return difficulty, average party level,
encounter level, plus **an explicit flag/reason when any CR falls outside the verified table** so
the UI can disclose it. Register in `main.rs`. Establish and report what the engine actually does
with CR 0, fractional CRs and CR 11+; if it silently misrates, that is a blocker to report, not to
paper over. Verify: `npm run tauri:check`, full `cargo test`. Bar: 3 known reds.

**E-2 `[frontend]` — Encounter builder in the DM Toolkit.** A new screen/section: build a party
(from saved characters' real levels where available, else typed levels), add monsters from
`list_monster_catalog` with their CR shown, and see the rating update as the party or monster list
changes. Show average party level and encounter level alongside the tier, and **surface both
honesty caveats above**. No fabricated numbers: every value comes from E-1. Verify: `npm run
typecheck` (0 errors), `npm test`.

Not in v1: XP budgeting/awards, treasure, terrain, encounter saving/persistence, linking an
encounter to a Scene record. All v0.9 questions once the operator has reacted to this.
