# Choice-Picker UI Gap (Sorcerer/Cleric/Druid) — Scoping Plan

> Requested by the lead after the `headless-only` `characterHubModel.ts` fix
> (833ea89c): Sorcerer/Cleric/Druid's engines can all genuinely reach
> `Computed` now, but none of the three can be created that way through the
> real UI, because the specific choice each needs (Arcane bloodline +
> Arcane Bond for Sorcerer, a Good-domain pick for Cleric, an animal-
> companion nature bond for Druid) has no picker anywhere in the creation
> flow, and no field on the wire contract to carry it even if one existed.
> This doc scopes that gap — not just "what UI to build," but a cheaper
> option the existing codebase already ships that the lead's four questions
> didn't anticipate. Per standing discipline: scoping only, no code.

## What "the real creation flow" actually is today — the load-bearing fact this whole scoping turns on

`compose_character_input` (`apps/desktop/src-tauri/src/pf1_adapter.rs:268`)
is **not** a general character composer that translates arbitrary player
choices into a `CharacterInput`. It is a single, hardcoded, canonical
fixture — the same "GE-06 bounded deterministic posture" every class this
whole epic has been validated against — with the request supplying only
`raceId`/`classId`/`level`/`abilityScores`/`abilityBonusTarget`. Every feat,
skill allocation, and equipment selection is a fixed literal in that
function, regardless of anything the player does in the UI (confirmed by
reading `CreateCharacterForm.tsx` directly — it has no feat, skill, or
equipment picker at all; the only inputs are name/race/class/ability
scores). `compose_character_input` already has exactly one precedent for a
class needing an extra choice to reach `Computed`: Wizard.

```rust
if request.class_id == WIZARD_CLASS_ID {
    selected_choices.push(SelectedChoice {
        choice_set_id: "choice:wizard_school_specialization".to_owned(),
        selection_id: "school:evocation".to_owned(),
    });
    selected_choices.push(SelectedChoice {
        choice_set_id: "choice:wizard_opposed_schools".to_owned(),
        selection_id: "school:necromancy".to_owned(),
    });
    // ... transmutation too
}
```

This is silently applied to **every** Wizard, with **no picker anywhere in
the UI** — the function's own comment says so explicitly: "not a real
in-game 'pick your school' choice — that UI is separate, larger,
out-of-scope future work." This is the exact same shape of gap I'm scoping
for Sorcerer/Cleric/Druid, already shipped once, already accepted. It
changes the shape of this whole scoping question: there are genuinely two
different things "build the picker" could mean, of very different size,
and the cheaper one already has a working precedent in this codebase.

## The wire contract, precisely (answers question 2)

`CreateCharacterRequest` (both `composeCreateCharacterRequest.ts`'s
`CreateCharacterFormFields`/the Rust struct in `character_hub.rs:214`) has
exactly 8 fields: `characterId`/`displayLabel`/`raceId`/`classId`/`level`/
`abilityScores`/`abilityBonusTarget`/`savedAt`. No choices field exists.

The proven pattern to mirror already exists, one file over —
`LevelUpCharacterRequest` (`character_hub.rs:862`):

```rust
pub struct LevelUpCharacterRequest {
    pub character_id: String,
    pub class_id: String,
    #[serde(default)]
    pub additional_choices: Vec<SelectedChoiceDto>,
    // ...
}
```

...consumed at `level_up_character`'s handler by mapping straight into
`SelectedChoice` and appending to `chosen.selected_choices`. This is not a
new design — it is a real field, on a sibling request, already shipped,
already used in production (the hit-die-choice and feat-pick provenance
flows from earlier this session use exactly this mechanism). Extending
`CreateCharacterRequest` with the identical field, and having
`compose_character_input` accept and append it the same way
`level_up_character`'s handler does, is a small, low-risk, precedent-backed
change — **if** a real interactive picker is the path chosen (Path B
below). If the cheap default (Path A) is chosen instead, no wire-contract
change is needed at all — the existing per-class-`if` shape in
`compose_character_input` already does the job, same as Wizard's.

## Two real paths, not one — sizing both

### Path A — mirror Wizard's silent canonical default (cheapest)

Add three class-conditional blocks to `compose_character_input`, each
pushing one fixed, canonical choice (or two, for Sorcerer), the same shape
as the existing Wizard block:

- Sorcerer: `choice:sorcerer_bloodline -> bloodline:arcane` +
  `choice:sorcerer_arcane_bond -> <familiar or bonded object — pick one
  canonical option>`.
- Cleric: `choice:cleric_domain -> domain:good`.
- Druid: `choice:druid_nature_bond -> bond:animal_companion` (no species
  choice needed — Wolf is the only species this codebase's Druid/Hunter
  seam ever grounds, chosen automatically once the bond-type is
  recognized, confirmed directly in `pilot_compute.rs`).

**Size**: ~10-15 lines of Rust per class, zero wire-contract change, zero
new frontend surface, zero new component. Same order of magnitude as the
Wizard block it mirrors. Ships all three classes as genuinely creatable
through the existing UI immediately.

**Real cost, stated plainly, not glossed over**: every Sorcerer created
this way is silently an Arcane-bloodline-with-a-familiar Sorcerer; every
Cleric a Good-domain Cleric; every Druid an animal-companion Druid — no
player choice, same limitation Wizard already has and this codebase has
already accepted as fine for this stage. This should be a conscious,
named decision (documented in `characterHubModel.ts`'s own support-level
copy, same honesty bar as the `headless-only` fix itself), not something
that ships quietly and gets discovered later as a second false claim.

### Path B — a real interactive picker (the one the lead's questions assume)

Requires, in order:
1. Extend `CreateCharacterRequest`/`CreateCharacterFormFields` with an
   `additionalChoices`-shaped field (mirroring `LevelUpCharacterRequest`
   exactly — low risk, proven shape).
2. `compose_character_input` accepts and appends it (mirrors
   `level_up_character`'s handler; also low risk).
3. New frontend UI: at minimum, a class-conditional section in
   `CreateCharacterForm.tsx` (there is no existing conditional-per-class UI
   today beyond the read-only support-level text — this is genuinely new
   surface, not an extension of something that already flexes per class).
4. The three classes are NOT interchangeable content for that UI (directly
   answers question 1):
   - **Sorcerer** needs two *linked* choices — a bloodline pick, then (only
     for Arcane) an Arcane Bond sub-choice (familiar vs. bonded object).
     Only Arcane bloodline is computed; every other bloodline (Draconic,
     Fey, ...) would need to either be hidden from the list or shown with
     an honest "not yet computed, will stay Blocked" affordance — the same
     honesty bar as `headless-only`'s own copy, now needed per-option
     inside a picker rather than per-class.
   - **Cleric** needs one choice from a large real domain list, of which
     only Good actually computes (Good+Healing together still blocks on
     Rebuke Death — a second, narrower gate even within a "supported"
     pick). Same "most options are honestly unsupported" problem, arguably
     worse than Sorcerer's since Clerics conventionally pick 2 domains.
   - **Druid** needs one binary choice (animal companion vs. domain), and
     of the two, only "animal companion" computes — no domain-type nature
     bond is grounded (per `SWARM_REPORT.md`'s "domain-type nature bond
     still falls through to the catch-all"). No species picker needed.
   - **Do not assume every animal-companion-shaped class needs this UI** —
     confirmed directly in `pilot_compute.rs`: Hunter's and Cavalier's own
     companions are **unconditional** (automatic per the PF1 rules text
     itself, never framed as a choice), so neither needs a picker or a
     `selected_choices` entry at all, unlike Druid's genuine either/or
     Nature Bond. A generic picker built for Druid's shape would be the
     wrong tool for Hunter/Cavalier and should not be forced onto them.
5. A genuinely shared, reusable shape only exists at the mechanical level
   (a restricted-list choice, optionally with a linked sub-choice) — the
   *content* behind each (which options exist, which are honestly
   computed vs. not) is class-specific data, not shared. A single fully
   generic `{choiceSetId, options: [{selectionId, label, computed}]}`
   component could render all three, but each class still needs its own
   options array authored and kept honest as engine coverage changes —
   this is real, ongoing content-maintenance surface, not a one-time cost.

**Size**: genuinely multi-part — wire contract (small), backend
consumption (small), one new reusable-ish picker component (medium, new
surface), plus per-class option data authored honestly for 3 different
shapes (Sorcerer's linked pair, Cleric's single-with-sub-gate, Druid's
binary) (medium-to-large, ongoing). Not a bounded single-cycle slice the
way Path A is.

## Answering the four questions directly

1. **Generic vs. bespoke picker**: mechanically, one generic component
   could work (a restricted-list choice, sometimes with a linked
   sub-choice) — but only if scoped as Path B. The *option content* isn't
   shareable across the three classes, and Hunter/Cavalier prove not every
   companion-shaped class even needs this UI at all. Path A sidesteps this
   question entirely by not building a picker yet.
2. **Wire-contract change**: yes, on `CreateCharacterRequest`, mirroring
   `LevelUpCharacterRequest`'s existing `additional_choices` field exactly
   — a proven shape, not a new design. `compose_character_input` has no
   consumption path for anything beyond race/class/level/ability-scores
   today; it would need the same append-on-request logic
   `level_up_character`'s handler already has. This is only needed for
   Path B — Path A needs no wire change at all.
3. **UX placement**: no conditional-per-class UI exists in
   `CreateCharacterForm.tsx` today beyond the read-only support-level
   text — this would be genuinely new surface, most naturally an inline
   section directly below the Class dropdown (same "only show what's
   relevant to the selected class" pattern the support-level text already
   establishes), not a new wizard step or modal, given the low field count
   involved. Only relevant for Path B.
4. **Scope size**: Path A is one bounded slice covering all three classes
   (~30-45 lines of Rust total, no frontend change, ships immediately).
   Path B needs splitting — the wire-contract/schema piece first (small,
   shared across all three), then each class's own picker content
   separately (Sorcerer's linked-choice shape, Cleric's large-list-mostly-
   unsupported shape, and Druid's binary shape are different enough that
   bundling them risks the same "not actually one bounded task" trap this
   swarm's own discipline exists to catch).

## Recommendation

Start with **Path A** as its own tiny bounded slice — it is backend-only,
mirrors a shipped precedent exactly, needs no frontend-owner involvement
up front beyond the follow-on `characterHubModel.ts` label update (which
would then honestly become `full`, or possibly `full-except-human-level-1`
if the same shared hybrid-level-1 gate turns out to apply — needs
re-checking per class, not assumed), and immediately makes the "N of 27
classes reach Computed" claim true of the real product, not just headless
tests. Frame Path A's real cost (no player choice, silently canonical)
honestly in that same label update, the same way `headless-only`'s own
copy names its cause rather than glossing it.

Treat **Path B** (the real interactive picker) as separate, larger,
future work — explicitly the same "separate, larger, out-of-scope" framing
the existing Wizard-school code comment already uses for its own case.
Worth scoping properly (per-class option content, the "some options are
honestly unsupported" UX problem, Hunter/Cavalier's non-need for it) if
and when the team decides silent canonical defaults aren't good enough
for launch — but that is a product decision, not something to default into
by continuing past Path A without a conscious call.

This is entirely frontend/backend-adjacent glue, not new engine work —
backend's own three closure docs (`sorcerer-arcane-bloodline-closure-
scoping.md`, `druid-animal-companion-closure-scoping.md`, and Cleric's
equivalent landing) never touched the creation-UI question at all, so
there's no engine-side design risk here to coordinate on. Path A can start
frontend-only (small Rust change, reviewed the same way any other
`pf1_adapter.rs` PR would be) without waiting on backend's own queue.
