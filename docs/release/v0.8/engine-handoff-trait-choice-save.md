---
title: Engine handoff — choice-trait characters cannot be saved
status: open
severity: user-facing data loss (save refused)
found_by: backend teammate, v0.8 UI sprint, during ticket B-4
found_on: 2026-09-01
owner: the concurrent session that owns repo-root src/
scope_note: repo-root src/ is out of scope for the v0.8 UI sprint (agent-team-ui.md §4.2), so this
  session did NOT attempt either fix. Operator ruling was to hand it off rather than edit the
  engine crate from two sessions at once.
---

# Choice-trait characters cannot be saved at all

## Impact

A player who picks any `%LIST` trait — every trait in `SKILL_CHOICE_TRAIT_BONUSES` and
`FAMILY_CHOICE_TRAIT_BONUSES`, e.g. **Criminal**, **Fiend Blood**, **Harvester** — and then chooses
the skill that trait asks for **cannot save the character at all**. The create call fails outright.
This is not a display bug or a degraded path; the character is lost at the save boundary.

This is **pre-existing on `develop`'s engine**, not introduced by the v0.8 UI sprint. The desktop
create form already sends the offending payload today: `CreateCharacterForm.tsx:613` passes
`traitSkillChoices` on every create.

Note for whoever reads the v0.8 gap audit alongside this: `scout-gap-audit.md`'s "already wired"
section credits "53 computing character traits with per-trait skill choice." That is correct for
flat traits and **wrong for the choice-trait subset** — those cannot round-trip.

## Mechanism

Two engine invariants disagree about how many colon-segments a `choice_set_id` may have.

1. `src/rules_core/trait_effects.rs:517` builds the id by prefixing a trait id that *already*
   carries a `trait:` prefix:

       pub fn trait_skill_choice_id(trait_id: &str) -> String {
           format!("trait_choice:{trait_id}")
       }

   For `trait:trait_criminal` this yields `trait_choice:trait:trait_criminal` — **three** segments.

2. `src/saved_character/local_store.rs:254` rejects any `choice_set_id` that is not **exactly two**:

       if choice.choice_set_id.split(':').count() != 2 {
           return Err(... "must have exactly two colon-segments to round-trip
                          through the fixture grammar" ...)

   (The neighbouring `selection_id` check uses `< 2`, not `!= 2`.)

## Reproduction

`backend` reproduced it directly against the real create path with a throwaway probe (since
deleted), calling `create_character_at_root` with the exact payload the form sends today:

    selectedTraits:     ["trait:trait_criminal"]
    traitSkillChoices:  [{ "trait_choice:trait:trait_criminal": "skill:intimidate" }]

Result:

    selected choice choice_set_id 'trait_choice:trait:trait_criminal' must have exactly
    two colon-segments to round-trip through the fixture grammar

The orchestrator independently confirmed both code sites by direct read before escalating.

## Two candidate fixes (neither applied)

**(a) `src/saved_character/local_store.rs:254`** — relax `!= 2` to `< 2`, matching the
`selection_id` check directly above it. One line; does not change any persisted id format, so no
existing test pin needs updating. **Prerequisite:** confirm the fixture-grammar parser
(`character_input.rs` ~:297) actually round-trips a 3-segment id before choosing this — if it
splits on the first colon and keeps the remainder, this is the clean fix.

**(b) `src/rules_core/trait_effects.rs:517`** — emit two segments by stripping the redundant
prefix:

    format!("trait_choice:{}", trait_id.trim_start_matches("trait:"))

The reader at `:542` uses the same function, so it stays self-consistent. **But** this changes a
persisted id format, so it needs the pins at `trait_picker.rs:888,914` and
`composeCreateCharacterRequest.test.ts:141,146` updated, and it raises a migration question for any
already-saved character holding the old 3-segment id.

`backend` recommends (a) if the parser copes, else (b). The v0.8 team has no opinion beyond that —
this is the engine owner's call.

## Definition of done

Two tests already exist in `apps/desktop/src-tauri/src/character_hub.rs` and are **deliberately left
red** as the acceptance criteria. They require no edit and should go green untouched once the engine
accepts the id:

- `add_trait_selection_at_root_records_the_skill_choice_for_a_choice_trait`
- `remove_trait_selection_at_root_removes_the_trait_and_its_skill_choice`

Current state: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml trait_selection_at_root`
→ 6 passed, 2 failed (exactly the two above). B-4's flat-trait path is complete and green.
