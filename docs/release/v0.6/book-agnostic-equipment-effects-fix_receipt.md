# Fix receipt — book-agnostic equipment AC/max-dex/spell-failure/weapon-mod resolution

- **Finding:** `docs/release/v0.6/book-agnostic-backend-gaps-scoping.md`, Finding 1 (the same
  finding PR #344 fixed for weight/cost; this is the equipment_effects.rs half of that finding,
  deliberately left out of #344's scope).
- **Branch:** `fix/equipment-effects-book-agnostic`, forked from `origin/develop` after v0.6/PR
  #343 merged.
- **Status:** built, tested, verified.

## Change

`src/rules_core/equipment_effects.rs`: `compute_equipment_effects` resolved every selection via
the already-book-agnostic `equipment_id_resolve`, then gated ALL per-item effects (AC bonus, max
Dex cap, spell failure chance, armor check penalty, skill bonus, ability-score bonus, weapon
enhancement bonus) behind a category lookup in the CRB-only compiled
`rules_tables::crb::equipment_tables()` store. A non-CRB item's key was never in that table, so the
item was `continue`'d out of the loop entirely -- not even a default/zero effect, the item
vanished from `per_item` altogether. This is a real, currently-shipping bug: it already affects any
already-shipped APG/ACG equipped item today, not just future ARG/PU gear (confirmed no per-book
dispatch exists anywhere in this file).

A second instance of the same pattern existed in `resolve_weapon_to_hit_bonus`: an equipped weapon
modifier's to-hit bonus was gated behind the same CRB-only category check before being applied.

## Fix

Every per-category resolver (`arms_armor::compute_arms_armor_effect`,
`general::compute_general_effect`, `magic_items::compute_magic_items_effect`,
`equipmods::compute_equipmods_effect`) was already confirmed book-agnostic -- each reads its own
token/bonus-chain shape directly off the resolved `EquipmentRecord`, independently of category, and
already returns `None`/default when the relevant tokens are absent. So the real fix removes the
category *gate* entirely: call all four resolvers directly against every resolved record, and
derive the `category` field (confirmed purely descriptive -- no branching on it anywhere
downstream, including `apps/desktop`'s own wire type, which treats it as a plain string) from
*which* resolver(s) actually matched, falling back to the CRB-only table only as a best-effort
label for the (already fully-correct, effect-wise) case where no category-defining token matched at
all.

For `resolve_weapon_to_hit_bonus`, the CRB-only category-membership check was strictly redundant
with `compute_equipmods_effect`'s own precise, already-book-agnostic check (it returns `None` if
the record doesn't carry a matching `BONUS:WEAPON|...|TYPE=Enhancement` chain) -- simply removed.

## Verification

- **Two new regression tests**:
  - `a_non_crb_armor_item_resolves_all_four_arms_armor_stats`: a synthetic-but-real-grammar
    ARG-tagged armor record (CRB's own verified token shape, explicitly labeled as not a literal
    single verbatim ARG source line since no clean single-record match exists in ARG's real source
    -- the book adds accessories/exotic weapons, not new base armor). Asserts AC bonus, max Dex,
    spell failure, and armor check penalty all resolve and aggregate correctly. Fails pre-fix (item
    silently dropped from `per_item` entirely), passes post-fix.
  - `a_non_crb_equipment_modifier_still_applies_its_tohit_bonus`: real ARG equipmods token grammar
    (`BONUS:WEAPON|TOHIT|1|TYPE=Enhancement`), confirms a non-CRB weapon modifier's to-hit bonus is
    no longer silently dropped.
- `cargo test --lib rules_core::equipment_effects` -- 23/23 passed (21 pre-existing + 2 new).
- Full workspace suite (`PCGEN_REPO_DIR` set): **5,353 passed / 2 failed**. Both pre-existing,
  environment-path-dependent (`/home/ubuntu/workspace/...`), identical baseline to every prior
  receipt this session. Zero regressions.

## Scope note

Left `RuleSetId::Crb` unchanged at both `equipment_id_resolve` call sites in this file, same as PR
#344 -- confirmed it only affects a discarded citation value, not resolution or the bugs this fixes.
Same larger `RuleSetId`/Finding 2 note applies: out of scope here.
