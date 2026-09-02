---
title: v0.8 — iteration 2 tickets
status: active
date: 2026-09-02
---

# Iteration 2

Operator asked for another iteration. These are chosen by the orchestrator on value, and every one
is dispatchable today: no engine edit, no operator decision pending. Ordered by player impact.

## Theme 1 — close the level-up hole (audit item 27, the biggest remaining gap)

Scout's audit: *"A Rogue levelling to 2 gets no talent."* `LevelUpDialog` prints
`enginePlan.pickFromLists` as **text** and ignores the `candidates` array; `handleLevelUpAccept`
sends only `hp:average`. So every rogue talent, rage power, hex, discovery, revelation and
investigator talent is silently skipped at level-up. This is the single largest thing a player
loses today.

It is dispatchable because the wire already carries what a picker needs — verified before cutting:
`LevelUpPickCandidateDto { id, name }` exists at `character_hub.rs:1904` and reaches TS at
`previewLevelUp.ts:39`, and `LevelUpCharacterRequest.additional_choices` already exists.

- **G-1 `[backend]` — confirm the candidate id grammar round-trips, and fix it if it doesn't.**
  Scout flagged the risk: `local_store.rs` rejects any `choice_set_id` that isn't exactly two
  colon-segments — the same defect that makes choice-traits unsaveable (B13). **Before `frontend`
  builds a picker, establish by test that a candidate id chosen at level-up actually persists and
  reloads.** If it doesn't, that is a second instance of B13's grammar bug and an engine handoff —
  report it, don't work around it. Verify: `npm run tauri:check`, full `cargo test`, 3-red bar.
- **G-2 `[frontend]` — render `pickFromLists` as real pickers.** Depends on G-1's verdict. Each
  list becomes a selection control over its `candidates`, and the chosen ids ride in
  `additional_choices` alongside the existing `hp:average`. Where G-1 finds a list unsupported,
  say so in place rather than offering a control that cannot persist. Verify: typecheck, tests.

## Theme 2 — tell the player what a choice just unlocked (audit item 29)

- **G-3 `[frontend]` — after a feat is added, show what it made eligible.** Adding Power Attack
  doesn't reveal that Cleave is now available; the picker only re-evaluates on next open. Re-run
  `list_feats_for_character` after a successful add and surface the delta ("Now eligible: Cleave,
  Improved Bull Rush"). Purely presentational — the eligibility judgement stays entirely in the
  engine; the UI diffs two engine answers and never decides eligibility itself. This is question 2
  of the brief's original mandate — *does the sheet show what a choice qualifies you for?* — and it
  is the cheapest place to answer it. Verify: typecheck, tests.

## Theme 3 — connect the encounter builder to the campaign (DM Toolkit)

The encounter builder and the DM Toolkit console were built in the same session and don't know
about each other. A DM who rates an encounter has nowhere to put it.

- **G-4 `[frontend]` — save a rated encounter onto a Scene record.** From the builder, attach the
  current party/monster selection and its rating to a Scene as structured text in that record's
  body or a field, so it persists, exports, and appears in the GM copy. Keep it honest: store what
  the engine returned, including any unverified-CR caveat, so an exported encounter carries the
  same disclosure the screen does. Do **not** re-rate on export or store a tier without its
  caveats. Verify: typecheck, tests.

## Explicitly not in this iteration

Anything needing an operator decision (the `'draft'` visibility state, per-player audiences,
hierarchy, a Map kind, encounter XP budgets/treasure) and anything gated on the five filed engine
handoffs. Those stay for v0.9.
