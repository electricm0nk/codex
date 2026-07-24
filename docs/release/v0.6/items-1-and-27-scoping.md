# Items 1 + 27 — Equipment-Attachment Schema & Posture-Gate Widening: Scoping Plan

> Operator-directed (2026-07-24): widen `unmet_combat_posture_conditions`/
> `unmet_selected_skill_posture_conditions` to accept any equipment loadout,
> not just the one hardcoded posture (item 27), and add a real
> equipment-attachment schema field so multi-weapon attack-bonus math
> resolves instead of staying honestly absent (item 1). Scoped together per
> instruction, since both need the same underlying capability. Design/scoping
> only — no code in this pass. Same shape as
> `item-1-architecture-wall-design.md`; read that first for the
> already-established headless/corpus-aware background this plan builds on.

## The central finding: these two items are no longer independent

The original item-1 design pass concluded bridging the headless/corpus-aware
wall would buy nothing today, because the posture gate's *exact-item-id*
check is a stricter, separate bottleneck — bridging the wall doesn't widen
which loadouts reach `Computed` on its own. That conclusion assumed the
posture gate would stay exact-match. It won't. The operator has now decided
the gate itself should widen (item 27). Once it does, "does this equipped
item have known, corpus-derived math" becomes a real question the
`Computed`/`Blocked` decision must answer for arbitrary gear — which means
the corpus-awareness question this doc's predecessor shelved is back, this
time as a genuine, load-bearing prerequisite rather than a nice-to-have.
Sequencing both items as one plan avoids scoping the same underlying need
twice.

## Part A — Equipment-attachment schema (item 1)

### Real schema shape, verified against PCGen's own character-file convention

Before inventing a schema, I checked how PCGen itself represents an enchanted
weapon in a real `.pcg` file (`code/testsuite/PCGfiles/pf_Paladin.pcg`):

```
EQUIPNAME:Flail +1 (Heavy)|...|CUSTOMIZATION:[BASEITEM:Flail (Heavy)|DATA:NAME=Flail +1 (Heavy)$EQMOD=DISARM.STEEL.TRIP.PLUS1W]
```

This is **not** two separate flat entries with a cross-reference — it's a
**single** equipment entry that names its own base item and carries its own
applied modifiers inline (`EQMOD=`, dot-separated). My first instinct going in
(a separate `attached_to: Option<String>` field referencing another
selection's id, needing a new unique-instance-id scheme to disambiguate two
identical base items) would have been a heavier, non-PCGen-faithful design.
The real, verified shape is simpler:

```rust
pub struct EquipmentSelection {
    pub item_id: String,
    pub equipped_or_active: bool,
    pub active_state: ActiveState,
    /// NEW: item_ids of equipmods-category items applied to this specific
    /// selection (e.g. "Special Ability ~ +1 ~ Weapon" on a Longsword
    /// selection), mirroring PCGen's own CUSTOMIZATION:EQMOD= convention.
    /// No separate top-level equipment_selections entry for an applied
    /// equipmod -- it lives on the weapon/armor selection it modifies.
    pub applied_modifiers: Vec<String>,
}
```

No new id/reference scheme needed. `compute_equipment_effects` (already
resolves `equipmods`-category records and their `weapon_enhancement_bonus`)
would look up each selection's `applied_modifiers` directly instead of
scanning the flat list for a same-category item and guessing which weapon it
belongs to — closing the ambiguity item 1's original pass identified, for
real, for any number of weapons.

### Real blast radius, counted not estimated

`EquipmentSelection { ... }` is constructed as an explicit struct literal
(no `Default` today) at **67 sites**: 14 in `src/rules_core`, 11 in
`apps/desktop/src-tauri`, **42 in `tests/**` (63%)**. `ChosenCharacterState`
(the alternative location I considered for this field, to avoid touching
`EquipmentSelection` at all) has **74** construction sites — checked before
assuming it would be cheaper; it isn't, both are the same order of magnitude,
and neither derives `Default` today so neither avoids the compile break.
Rust has no way to add a required field to a struct without updating every
explicit literal construction of it within the same crate (`#[non_exhaustive]`
only affects external-crate boundaries, which doesn't apply here) — this is
a real, unavoidable cost, not a design choice to optimize around. The
honest plan: add `Default` for `ActiveState` (a sensible default variant,
`Absent`, already exists) so *future* code can use `..Default::default()`,
but the ~67 *existing* sites still need a one-line mechanical addition
(`applied_modifiers: Vec::new()`), the same coordinated-bulk-update shape
already used this session for smaller schema additions (e.g. `spells_selected`
onto `ChosenCharacterState`) — just at a larger scale, and with real
cross-team dependency since 42 of the 67 sites are QA-owned test fixtures.

### Frontend affordance needed (not backend's to build, but a real dependency)

Today a player adds "Longsword" and "+1 Enhancement to Weapon" as two
independent picks from the same flat equipment picker. Populating
`applied_modifiers` needs a genuinely new UI affordance — attach a picked
modifier to a specific already-equipped weapon, not just add it to a flat
list. Without this, the schema field would exist but nothing would ever
populate it. Flagging this as a cross-cutting dependency the sequencing plan
below accounts for, not something backend can land alone and call done.

## Part B — Posture-gate widening (item 27)

### Why this needs corpus access after all

`unmet_combat_posture_conditions`/`unmet_selected_skill_posture_conditions`
live in the headless layer (no corpus parameter). "Accept any equipment
loadout" means the gate must decide, for arbitrary gear, whether real math
exists for whatever pillar it contributes to (AC/ACP for armor, the
now-per-weapon attack-bonus for weapons) — an inherently corpus-aware
question, since that's where `compute_equipment_effects` lives. Three
candidate approaches, in order of how invasive each is:

| | Mechanism | Cost | Risk |
|---|---|---|---|
| **(i) Extend the static bootstrap table** | `rules_tables::crb::equipment_tables()` already carries `category` (ArmsArmor/General/MagicItems/Equipmods) headlessly, but not the weapon-vs-armor distinction *within* ArmsArmor (that needs the real corpus record's `DAMAGE:` token). Add a finer static field (e.g. `arms_armor_kind: Weapon \| Armor \| Shield`), authored once from the corpus at data-generation time, same precedent as `cost_gp`/`weight_lbs` already being static-authored fields | Small-medium: a data-authoring pass over the existing CRB arms/armor record set (finite, already enumerated), no runtime corpus dependency added | Ceiling: this only gets identity/category right headlessly. It does NOT get real AC/ACP/attack-bonus math into the claim-gated pillars — those still need corpus, so the gate could recognize "this is a weapon" without knowing its real to-hit bonus. Only closes half the problem. |
| **(ii) Move `Computed`/`Blocked` to the corpus-aware layer** (shape (b) from the original design doc) | Derive the claim-gating verdict from `compute_pilot_with_corpus`'s output (which already resolves real per-item math via `compute_equipment_effects`) instead of the headless receipt | Large: needs a parallel diagnostic-derivation pass mirroring the headless gate's logic, and every Tauri command's control flow changed from "check the headless receipt" to "check the corpus-aware one" | Real behavioral change to the crate's central save/mutate invariant. Bounded if done as an *additive* second path (see recommendation below), not a replacement. |
| **(iii) Thread corpus through the full headless chain** (shape (a)) | Give `build_pilot_headless_receipt` a corpus parameter, pass it down | Large: ~19+37+100ish call sites (already sized in the original doc) | Same ceiling as (ii) for actual payoff, larger mechanical footprint. Not recommended given (ii) already has a corpus-aware layer built and populated. |

**Recommendation: (ii), built additively, not as a replacement.** The
existing exact-posture check already works today for Fighter/Wizard/Rogue —
it should stay exactly as-is as a zero-risk fast path. A **new**, separate
validation function (corpus-aware, living alongside `compute_pilot_with_corpus`)
handles any OTHER loadout: for each `EquippedActive` selection, resolve
against the corpus; if it resolves and its category has known math
(arms_armor → `compute_equipment_effects` already gives real AC/ACP, and now
per-weapon attack-bonus via Part A's schema), count it supported. `Computed`
requires every equipped item to be either corpus-resolved-with-known-math or
absent — never requires a feat to have a known effect (see the policy
question below). This is additive: nothing that reaches `Computed` today
would ever stop reaching it.

### A real policy question this plan surfaces, not decides

Should `Computed` ever be blocked by a *feat* with no known effect (e.g. a
Fighter selecting a feat the 4-feat-effects engine doesn't recognize)? Every
precedent this session established (items 14/17/18/20/21) says no — an
unknown feat effect is a completeness gap, not a correctness bug, and has
never blocked `Computed` anywhere in this crate. I'm stating this as the
plan's working assumption, but flagging it explicitly rather than silently
assuming it extends to the widened posture gate too, since item 27 is itself
a "how much variation is acceptable" philosophy call — the same category of
decision, made explicit rather than inferred.

## Bounded sub-task sequence

Each lands as its own cycle with its own RED→GREEN tests, verified
independently before the next starts — the same incremental-widening
discipline already used for the Rogue chassis widening and the feat-effects
engine this session:

1. **Schema field** (`applied_modifiers` on `EquipmentSelection`) — data-model only, no behavior change. Coordinate the ~67-site mechanical update with QA before landing (42 of the sites are theirs).
2. **Per-weapon attack-bonus resolution** — extend `compute_equipment_effects`'s `attack_bonus_delta` to read `applied_modifiers` directly instead of the "exactly one weapon equipped" heuristic Part A of the earlier shape (c) work used. Closes item 1's 2+-weapon gap for real. Bounded, testable in isolation against the new field.
3. **Static category data** (approach (i) above) — a data-authoring pass giving `equipment_tables()` a weapon/armor/shield distinction headlessly, if the corpus-aware-layer approach (ii) below still wants a cheap headless pre-filter; otherwise skippable, since (ii) resolves this via the corpus directly.
4. **Corpus-aware `Computed` verdict** (approach (ii)) — the new, additive validation path. Built and tested standalone before touching any command.
5. **Command-layer wiring** — `create_character`/`level_up_character`/etc. accept `Computed` from *either* the existing exact-posture path or the new general path. Highest-risk step; needs the fullest test coverage (every currently-`Computed` build must still compute identically).
6. **Frontend equipment-modifier-attachment UI** — not backend's to build, but sequenced here since sub-task 2 has no real user-facing value until players can actually attach a modifier to a weapon through the UI.

## Open questions for the operator/lead before implementation starts

- Confirm the feat-completeness policy above (never block `Computed` on an
  unknown feat effect) extends to the widened posture gate, or should be
  reconsidered now that "arbitrary loadout" is in scope.
- Confirm approach (ii) (additive corpus-aware verdict, exact-posture path
  kept as-is) over a full replacement of the existing gate — the additive
  shape is recommended here as lower-risk, but replacing it outright would
  eventually be cleaner if the team is comfortable re-verifying the
  Fighter/Wizard/Rogue posture against the new path too.
- Whether sub-task 3 (static category data) is worth doing at all, or
  whether going straight to the corpus-aware layer (which already resolves
  everything sub-task 3 would provide, just at runtime instead of
  build-time) makes it redundant.
