---
title: Engine handoff — only one wizard specialization can produce a saveable character
status: open
severity: feature gap (8 of 9 wizard schools are unreachable; a player gets Evocation regardless)
found_by: backend teammate, v0.8 UI sprint, during ticket B-11
found_on: 2026-09-01
owner: the concurrent session that owns repo-root src/
scope_note: repo-root src/ is out of scope for the v0.8 UI sprint (agent-team-ui.md §4.2).
  Nothing was changed in the engine; this is the finding plus the command that proves it.
---

# Only Evocation/{Necromancy, Transmutation} produces a saveable wizard

## The finding

A level-1 wizard can be saved with exactly **one** specialization configuration:

    specialty: Evocation    opposed: Necromancy + Transmutation

All eight other schools — Abjuration, Conjuration, Divination, Enchantment, Illusion,
Necromancy, Transmutation, and the Universalist — **cannot be saved at all**. They emit their
school-power explanation rows, so they read as supported from the outside, but
`unmet_wizard_spellbook_conditions` claim-blocks the save with the engine's own sentence:

> prepared spellbook grounding requires the canonical Evocation specialization
> (opposed Necromancy/Transmutation)

Free opposed pairs are also not selectable: Evocation computes with that one pair and no other.

Today this is invisible to players because `compose_character_input` silently seeds the canonical
Evocation/Necromancy/Transmutation triple, so every wizard is that wizard. The moment a UI offers
the choice, eight of nine options become dead ends.

## Why this is trustworthy

It was established by **executing the engine**, not by reading it, and it corrected an earlier
claim made by this same sprint:

- `docs/release/v0.8/choice-pool-resolver-audit.md` (B-7) originally reported **five** grounded
  specialty+opposed triples, inferred from which combinations produced grounded explanation rows.
  That inference was wrong — grounding an explanation row is not the same as passing the save gate.
  The audit has been amended in place.
- B-11 probes every combination (8 specialties × C(7,2) pairs + universalist = 169 computes)
  through `compose_character_input` → `resolve_unified_pilot_snapshot`, the same path a save uses.
- The result is pinned by a test that drives the real `create_character_at_root`:
  `supported_means_the_real_create_path_saves_and_unsupported_means_it_blocks`
  (Evocation → `Saved`, Abjuration → `Blocked`).
- `qa`, on a different model, independently confirmed the parity test calls the production save
  function rather than a lookalike, that no hardcoded supported-combo list exists in the bridge,
  and that the diagnostic id `class_spell.wizard.prepared_spellbook.unsupported` is pre-existing
  engine code at `src/rules_core/pilot_compute/mod.rs:45484`.

## What already exists to build on

`list_wizard_school_options` (`apps/desktop/src-tauri/src/wizard_school_picker.rs`, committed
fcb359b4ef) returns all nine schools with, per school, a `supported` flag, the exhaustive list of
opposed pairs that compute, and the engine's blocking diagnostic verbatim.

Crucially it **discovers** support by probing rather than declaring it: no list of schools or
supported combinations exists in the bridge, and no branch is keyed on which school is which. **If
the engine widens this gate, the command widens automatically with no code change** — and its
parity test will start reporting the new combinations as `Saved`.

## Why no UI was built

A picker offering nine schools where eight are permanently greyed presents dead ends as live
choices pending player action. That is dishonest even with an honest reason string on each row, and
it is the "resolves but does not compute" stub-shape this sprint's own audit warned about. The
orchestrator declined to dispatch the frontend half; `qa` independently reached the same conclusion.

The UI half becomes worth building the moment the engine supports more than one configuration —
and the command needs no change when it does.

## The ask

Widen `unmet_wizard_spellbook_conditions` (and whatever prepared-spellbook grounding it guards) so
specializations beyond the canonical Evocation triple can produce a saveable character. Related
question worth asking at the same time: whether the same "grounds an explanation row but blocks the
save" split affects other class choices — B-7 found the identical pattern for Cleric domains, where
roughly 66 of 73 domains claim-block, and for Bloodrager bloodlines.
