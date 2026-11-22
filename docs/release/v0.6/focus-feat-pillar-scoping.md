# Focus-feat pillar scoping — school-keyed spell DCs vs per-weapon attack totals

**Task #69. Investigation only — nothing built, no decision taken.** Written for a
decision checkpoint, same shape as the Summoner evolution-economy call.

## The question

Four Focus-family feats are grounded as standalone facts but wired into nothing:
Spell Focus and Greater Spell Focus on one side; Weapon Focus, Weapon
Specialization, Greater Weapon Specialization and Improved Critical on the other.

Task #34 established that this is **not** a chooser-mechanism gap. The chooser
pattern is already built and reused four times — Skill Focus, Spell Focus, Weapon
Focus, and Master Craftsman all use `choice:<feat>_target` plus a typed
`selection_id`. What each of these feats lacks is a **destination**: a computed
total to modify.

So the real question is whether this engine wants two new pillars.

## Headline finding: these are not siblings

The brief framed them as a pair. They are not remotely the same size.

| | school-keyed spell DC | per-weapon attack total |
|---|---|---|
| target data modelled? | **yes** — spells carry school | **no** — no weapon records at all |
| identity type exists? | **yes** — `Pf1SchoolId` | no |
| related total exists? | **yes** — DCs, keyed by spell level | base attack bonus only |
| shape of work | **re-key an existing total** | **ingest a new content domain, then build** |
| rough size | small–moderate | large |

## Side A: school-keyed spell save DCs

**Most of this already exists.** The gap is a join, not a build.

- `crb::spell_list::SpellListEntry` carries `pub school: Pf1SchoolId` —
  non-optional, **670 school assignments in the CRB table alone**. APG's carries
  `Option<Pf1SchoolId>`.
- `Pf1SchoolId` is a real enum with `from_corpus_str`, already used for school
  identity elsewhere.
- Since task #66, school-keyed *feature* ids exist —
  `class_feature.school.abjuration.*` — so the school concept has a foothold in
  computed output, which was **not** true when #34 first scoped this.
- Spell save DC totals already exist for four classes:
  `class_chassis.{cleric,druid,wizard,skald}.spell_save_dc.spell_level_{n}`.

**What's missing:** those DCs are keyed by *spell level*, not by spell. Spell
Focus grants +1 to the DC of spells *of a chosen school*, and a level-keyed total
has no school to filter on. Both operands exist; nothing joins them.

**What building it would mean:** emit DC records keyed per-spell (or per
school-and-level) for spells the character can actually cast, then let Spell
Focus's already-built fact modify the matching subset. The likely cost is not the
arithmetic — it is **cardinality**: a Wizard's prepared list could turn a handful
of level-keyed records into dozens of per-spell records in every receipt. That is
a product question about receipt readability as much as an engineering one.

**Unblocks:** Spell Focus (+1), Greater Spell Focus (+2, stacking).

**Risk:** low-to-moderate, and mostly contained. No new corpus ingestion, no new
input state, no new identity type. The main hazard is receipt bloat, which is
reversible.

## Side B: per-weapon attack totals

**This is a genuinely greenfield pillar, and larger than the brief implies.**

- There are **zero weapon records in the equipment tables** — `grep -c 'Weapon'
  crb/equipment_tables.rs` returns 0. The engine has ingested armor, not weapons.
- Consequently there is **no weapon identity** to key on. `Improved Critical`
  needs a weapon's crit range; `Weapon Specialization` needs its damage. Neither
  datum is present.
- `CharacterInput` does carry `equipment_selections: Vec<EquipmentSelection>`
  with an `equipped_or_active` flag, so the *input seam* exists — but nothing
  downstream computes from a weapon selection.
- 32 attack-shaped ids exist. They are class base-attack progressions
  (`class_chassis.<class>.base_attack_bonus`), situational flats
  (`smite_evil_attack_bonus`, `favored_enemy_2_attack_damage_bonus`,
  `flurry_of_blows_attack_bonus`), or natural-attack forms on the Eidolon
  (`bite_attack`, `hoof_attack`). **Zero ids key on both attack and weapon** —
  verified by enumerating every id, not by pattern-matching prose.
- One is worth naming because it is the nearest existing destination:
  **`combat.baseline_melee_attack_bonus`** is a real, general melee attack total.
  It is not per-weapon — it has no weapon to be "per" — but it means the
  arithmetic seam for an attack total already exists. A per-weapon total would
  extend that rather than invent it, which lowers the *last* step's cost without
  changing the ingest cost that dominates.
- The one precedent for computing an equipment effect is armor — the Chain Shirt
  armour-check penalty — and that is a single hard-coded item, not a table-driven
  ingest.

**What building it would mean, in order:** ingest weapon records from the corpus
(damage die, crit range/multiplier, type, proficiency group) → model which weapon
is wielded → compute a per-weapon attack total from BAB + ability + modifiers →
*then* let the four already-built weapon feats modify it.

Only the last step is feat-effects work. The first three are a new content
pillar comparable in size to a class-family ingest.

**Unblocks:** Weapon Focus, Weapon Specialization, Greater Weapon Specialization,
Improved Critical — plus, further out, Fighter's Weapon Training/Mastery, which
are currently grounded as bare flats with the same missing destination.

**Risk:** high, and it is the *ingest* that carries it, not the feats. Weapon
data is where per-item detail (special materials, enhancement bonuses, size
adjustments) tends to sprawl.

## Third option, per task #68

Operator's #68 correction — zero-magnitude feats need their **description
rendered**, not a fabricated number — applies directly here, and should be
treated as a real option rather than a fallback.

For any feat whose pillar is not built, the honest product answer may be: show
the player what the feat *does*, in its own words, instead of leaving it silently
deferred. That is a UI change, not an engine change, and it is available
immediately for all six feats regardless of what is decided below.

This matters most for **Side B**, where the pillar is genuinely expensive: a
player reading "Weapon Focus (longsword): +1 on attack rolls with this weapon" on
the sheet may be most of the real value, at a fraction of the cost of computing
it.

## Recommendation

**Split the decision. They are not one call.**

1. **Side A (school-keyed DCs) — worth building, and cheaper than it looks.**
   The data is already there and #66 has since put school identity into computed
   output. Recommend scoping a bounded slice next: one class (Wizard, which has
   both the DC total and the deepest school interaction), Spell Focus only,
   Greater Spell Focus deferred. Settle the receipt-cardinality question on that
   slice before generalising.

2. **Side B (per-weapon attack totals) — do not build as a Focus-feat task.**
   It is a weapon-ingest pillar wearing a feat-effects hat. If the engine wants
   weapons, that should be its own decision on its own merits — the four feats
   are a beneficiary, not a justification. Recommend #68's
   render-the-description answer for these four in the meantime.

3. **Either way, apply #68 now** to all six feats. It is independent of both
   decisions and immediately useful.

## What was verified directly

Every claim above was checked against the current tree (`ce16b82e`), not carried
from #34's earlier pass — and one thing had genuinely changed: **#34 reported
zero school-keyed ids; that is no longer true** since #66. The rest held.

- Spell DC id shapes: enumerated, all `spell_level_{n}`-keyed, no school-keyed DC.
- `school` field on spell entries: confirmed present, 670 CRB assignments.
- School-keyed feature ids: confirmed present post-#66.
- Weapon records in equipment tables: confirmed **zero**.
- Attack-shaped ids: all 32 enumerated individually; confirmed none keys on a
  weapon. A first, looser grep suggested 33 matches and was wrong — it was
  matching prose in doc comments, not ids. Recorded because the same
  pattern-too-loose error is what would make this doc's central claim false.
- `combat.baseline_melee_attack_bonus`: confirmed present.
- `equipment_selections` on `CharacterInput`: confirmed present but with no
  computing consumer.
