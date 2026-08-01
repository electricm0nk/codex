---
title: OGL/PI Field-Classification Blacklist (Shape B v1 License-Stripping)
stc_id: GOV-OGL-PI-BLACKLIST
canonical: false
owner: Todd Hintzmann
scope: SD-27 (Shape B v1 license-stripping, all in-scope and future-state books)
status: DRAFT — operator-reviewable, not unilaterally binding
review_state: pending_operator_sign_off
last_reviewed_at: 2026-07-27
canonical_source: docs/governance/ogl-pi-blacklist.md (this file)
related_artifacts:
  - src/rules_core/shape_b_v1.rs (the schema this blacklist feeds)
  - docs/release/SD-27-future-state-book-content-ingestion/decisions.md §17 (license-stripping doctrine)
  - docs/release/SD-27-future-state-book-content-ingestion/forward-scope-register.md §1.4 (initial blacklist source)
date: 2026-07-27
---

# OGL/PI Field-Classification Blacklist

> ## ⚠️ DRAFT — OPERATOR SIGN-OFF REQUIRED, NOT SELF-EXECUTING
>
> This document is a **content-licensing / legal classification draft**, not
> a binding automated policy. `decisions.md §17` frames Product Identity
> classification as requiring operator review, not silent automation —
> nothing in this repository treats this file as authoritative until an
> operator has reviewed and accepted a given book's classification pass.
> Cycle E2.0.5 (this document's origin) only lands the *schema* and the
> *initial* blacklist; it does not retro-fit any book's data. Per-book
> retro-fit cycles (E2.0.6+) apply this blacklist and must record what they
> found — including any field this draft missed — back into this file
> before that book's data is considered license-clean.
>
> **If you are a future cycle (human or agent) reading this file to decide
> what to redact:** treat every classification below as a starting
> hypothesis, not a verified legal fact. When a real field's content
> doesn't obviously fit a bucket, stop and ask the operator rather than
> guessing.

## 1. Source of the "Product Identity" definition

The Open Game License v1.0a, Section 1(e), defines Product Identity. This
document quotes it as fetched from a live mirror during authoring
(`https://opengamingfoundation.org/ogl.html`, 2026-07-27) — **not** typed
from memory, though the text below is also the well-known, stable,
unchanged-since-2000 wording that matches this author's own prior
knowledge of the license. Both sources agree; the fetched text is quoted
verbatim below for traceability:

> **1(e) "Product Identity"** means product and product line names, logos
> and identifying marks including trade dress; artifacts; creatures
> characters; stories, storylines, plots, thematic elements, dialogue,
> incidents, language, artwork, symbols, designs, depictions, likenesses,
> formats, poses, concepts, themes and graphic, photographic and other
> visual or audio representations; names and descriptions of characters,
> spells, enchantments, personalities, teams, personas, likenesses and
> special abilities; places, locations, environments, creatures,
> equipment, magical or supernatural abilities or effects, logos,
> symbols, or graphic designs; and any other trademark or registered
> trademark clearly identified as Product identity by the owner of the
> Product Identity, and which specifically excludes the Open Game
> Content.

Everything **not** Product Identity, in a work that has been declared
"Open Game Content" by its publisher (Paizo's Pathfinder RPG Reference
Document / SRD declares its core game mechanics Open Content), is
OGL-inlinable: "the game mechanic itself and any elements which are
necessary to it including... rules, formulae, ... including... spell
level names ... regardless of format" (OGL v1.0a §1(d)/§1(e) taken
together).

## 2. Real field names in this codebase, classified

Field names below are grepped from the actual v0 payload structs this
codebase ships today (`src/rules_core/rules_tables/crb/json_cache.rs`,
`race_tables.rs`, `feats.rs`, `beastiary1/mod.rs`,
`beastiary1/equipment_tables.rs`), not invented for this document.

### 2.1 PI-blacklisted (per `decisions.md §17`'s initial list)

| Field name (real, this codebase) | Where it appears | PI category (OGL §1(e)) | Notes |
|---|---|---|---|
| `deity`, `deity_name` | Not yet a discrete struct field anywhere in this codebase (deity names currently appear only as free text inside `SpellCacheData.description`/spell-list `description`, e.g. "Atonement", "Commune"). Listed here pre-emptively for any future discrete deity field. | "names and descriptions of characters ... personas" | Product Identity in CRB; a per-book override (§3) may reclassify for setting-neutral bestiaries. |
| `npc`, `npc_name` | Not yet a discrete field in this codebase's v0 payloads. Listed pre-emptively. | "creatures characters ... personas, likenesses" | PI in most CRB/APG/ACG/Bestiary-1 content. |
| `monster_name` (non-bestiary context) | `MonsterStatBlock.name` (`beastiary1/mod.rs`) is the one real field this maps to today — but see the per-book override below: **within Bestiary 1 itself, classic SRD monster names (e.g. "Goblin", "Owlbear") were declared Open Game Content by the original d20 SRD**, so `MonsterStatBlock.name` in this book is presumptively OGL, not PI, pending an explicit per-record check. | "creatures characters" | The blacklist entry is for *non-bestiary* uses of a monster's proper name (e.g. a unique named NPC monster in a module/adventure), not the generic species name in a core bestiary. |
| `place_name`, `faction_name`, `deity_portfolio` | Not yet discrete fields anywhere in this codebase. Listed pre-emptively. | "places, locations, environments" / "personas" | |
| `art_url`, `fiction_text`, `book_cover` | Not present in this codebase (no art/fiction ingestion exists yet). Listed pre-emptively for future-state book ingestion (SD-27's 19 stub books may carry narrative/art content). | "artwork ... graphic, photographic and other visual or audio representations" | |
| `monster_description` (flavor prose, not the mechanical stat block) | No discrete "flavor vs. mechanical" split exists in `MonsterStatBlock` today — its fields (`challenge_rating`, `size`, `speed_ft`, `race_type`, `race_subtype`, `natural_attacks`) are all mechanical/numeric and are OGL-inlinable (see §2.2). If/when a future cycle adds a flavor-text field to `MonsterStatBlock`, it lands here. | "stories, storylines, plots, thematic elements ... dialogue" | |

### 2.2 OGL-inlinable (game mechanics/procedures/formulae) — real fields, this codebase

| Field name | Struct | Why OGL |
|---|---|---|
| `class_id`, `maxlevel`, `bab`, `save_fort`, `save_ref`, `save_will` | `ClassCacheData` | Core class names and their progression tables (BAB/save formulas) are declared Open Game Content in the original d20 SRD and Pathfinder's own Reference Document. |
| `key`, `school`, `level` | `SpellCacheData` | Spell name, school, and level are game-mechanical classifiers; Pathfinder's core spell list was itself released as Open Game Content. |
| `key`, `category`, `cost_gp`, `weight_lbs` | `EquipmentCacheData` | Cost/weight/category are numeric game-mechanical data, explicitly OGL per §1(e)'s exclusion for "the game mechanic itself." |
| `race_id`, `trait_name`, `value`, `detail` | `RaceTraitEntry` | Core race trait mechanics (numeric bonuses, mechanical trait text) for the SRD's declared-Open core races. |
| `key`, `category`, `name`, `effect` (the `FeatEffectBonus` qualifiers) | `FeatTableEntry` | Feat name + numeric/formula `BONUS:` mechanics — game-mechanical, OGL. |
| `challenge_rating`, `size`, `speed_ft`, `race_type`, `race_subtype`, `source_page`, `natural_attacks` | `MonsterStatBlock` | Numeric/mechanical stat-block data — OGL per §1(e)'s explicit mechanic exclusion, independent of whether the monster's `name` is PI in a given context. |
| `damage_dice` (on `NaturalAttack`) | `NaturalAttack` | Dice-expression mechanic, OGL. |

### 2.3 Fields requiring per-record judgment (not blanket-classifiable)

These field names hold free text that *may* contain embedded PI (a named
deity, a named unique NPC, a place name) even though the field itself is
mechanically OGL-shaped. A blanket "OGL" or "PI" classification at the
field-name level is not honest for these — the per-book retro-fit cycles
(E2.0.6+) must inspect **values**, not just field names, and either leave
them OGL (no PI content found) or redact the specific offending
sub-string/whole-value to `"[redacted PI]"` with `pi_field` naming this
field.

| Field name | Struct | Why it needs per-value review |
|---|---|---|
| `description` | `SpellCacheData`, `EquipmentCacheData`, `FeatTableEntry` | Spell/equipment/feat flavor and rules text sometimes references "your deity" generically (OGL — no proper name) but could reference a specific named deity, NPC, or place in some entries (PI). CRB's real `Atonement`/`Commune`/`Miracle` spell descriptions found during this cycle's authoring use only the generic "your deity," not a proper name — but this is not proven exhaustively for every record in every book. |
| `detail` | `RaceTraitEntry` | Could carry a named homeland/place in some race entries. |

## 3. Redistribution posture (feeds the per-book `LICENSE.json`)

Once a book's records are fully classified, its overall redistribution
posture is one of:

- **`ogl-notice-attached`** — the book's inlined OGL content ships with the
  required OGL §15 "Section 15 Copy of this License" notice attached; no
  PI values are inlined (all redacted to markers or omitted at the field
  level).
- **`cc-by-compatible`** — reserved for any future book whose Open Game
  Content is dual-released under a Creative Commons license (not
  applicable to any of the 4 currently in-scope Paizo books as of this
  cycle).
- **`pi-present-restricted`** — an interim, non-shippable state: PI values
  exist in the on-disk cache un-redacted. No book should be in this state
  after its retro-fit cycle closes; it exists as a value so a `LICENSE.json`
  can honestly describe an in-progress book.

## 4. Per-book override template

Paizo's own PI declarations vary per book (a name may be PI in the Core
Rulebook and Open Content in a later, setting-neutral release, or vice
versa). Each book's retro-fit cycle should append a dated section here,
not silently diverge from this shared file:

```markdown
### Per-book override: <Book Name> (added by cycle <cycle-id>, <date>)

- <field_name>: reclassified <OGL -> PI | PI -> OGL> because <cited reason,
  e.g. "this book's own Product Identity section (page N) explicitly
  excludes/includes X">.
- New field discovered not in §2 above: <field_name> — classified
  <OGL | PI> because <reason>.
```

No entries yet — cycles 2.0.6-2.0.9 (CRB, APG, ACG, Bestiary 1 retro-fits)
append here as they run.

## 5. Per-book `LICENSE.json` template

Each book directory under `data/corpus/<book>/` gets a `LICENSE.json`
declaring its license split, redaction policy, and redistribution
posture. Shape (documented here; no literal template file is checked in
under `data/corpus/` because no book has yet been retro-fitted — the
first retro-fit cycle, E2.0.6, is what actually writes
`data/corpus/core_rulebook/LICENSE.json` from this shape):

```json
{
  "book": "core_rulebook",
  "license_declaration": {
    "open_game_content": "OGL 1.0a (Wizards of the Coast), inlined verbatim per §2.2 above",
    "product_identity_source": "Paizo Pathfinder Roleplaying Game Core Rulebook, OGL §15 Product Identity section",
    "product_identity_note": "Named deities, NPCs, and unique places are Product Identity per the book's own OGL Section 15 declaration; core class/race/feat/spell/equipment MECHANICS are Open Game Content."
  },
  "redaction_policy": {
    "marker": "[redacted PI]",
    "schema_preserving": true,
    "pi_field_recorded": true,
    "blacklist_source": "docs/governance/ogl-pi-blacklist.md",
    "blacklist_version_reviewed": "2026-07-27"
  },
  "redistribution_posture": "ogl-notice-attached",
  "classified_at": "<ISO-8601 timestamp of the retro-fit cycle that wrote this file>",
  "classified_by_cycle": "<cycle id, e.g. E2.0.6>",
  "operator_sign_off": {
    "signed_off": false,
    "signed_off_at": null,
    "note": "Set true only after an operator has reviewed this book's classification pass, per this document's DRAFT header."
  }
}
```

## 6. Cross-references

- `docs/release/SD-27-future-state-book-content-ingestion/decisions.md §17`
  — the license-stripping doctrine this document implements.
- `src/rules_core/shape_b_v1.rs` — the `License`/`pi_field`/`pi_marker`
  schema this blacklist's classifications feed.
- `docs/release/SD-27-future-state-book-content-ingestion/forward-scope-register.md §1.4`
  — the initial blacklist's origin.
- `tests/sd27_license_stripping_shape_v1.rs` — the dual-audit gate proving
  the schema (not yet the per-book data) is sound.
