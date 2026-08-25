---
title: OGL/PI Field-Classification Blacklist (Shape B v1 License-Stripping)
stc_id: GOV-OGL-PI-BLACKLIST
canonical: false
owner: Todd Hintzmann
scope: SD-27 (Shape B v1 license-stripping, all in-scope and future-state books)
status: SIGNED-OFF — amended and operator-approved per decisions.md §19
review_state: signed_off
last_reviewed_at: 2026-08-23
canonical_source: docs/governance/ogl-pi-blacklist.md (this file)
related_artifacts:
  - src/rules_core/shape_b_v1.rs (the schema this blacklist feeds)
  - docs/release/SD-27-future-state-book-content-ingestion/decisions.md §17 (license-stripping doctrine)
  - docs/release/SD-27-future-state-book-content-ingestion/forward-scope-register.md §1.4 (initial blacklist source)
  - docs/release/SD-32-compute-library-and-cause-closure/decisions.md §19 (2026-08-23 operator sign-off: amendments 3a-3d approved, §19b/§19c rulings)
  - docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-3-closure-invariant/t9-pi-signoff-package.md (the review this sign-off acts on)
date: 2026-07-27
sign_off_date: 2026-08-23
---

# OGL/PI Field-Classification Blacklist

> ## ✅ SIGNED OFF (2026-08-23) — amended per `SD-32 decisions.md §19`, standing caution below still applies
>
> This document was a **content-licensing / legal classification draft**
> from its 2026-07-27 origin through SD-32 card 11's T9 review. The operator
> has now reviewed the audit at `t9-pi-signoff-package.md` and approved all
> four amendments in §2.3/§2.3a-c/§4 below (`SD-32 decisions.md §19`), so this file is
> **in force as amended** — it is no longer "not yet binding." `decisions.md
> §17` (SD-27) still frames Product Identity classification as requiring
> operator review, not silent automation; that review has now happened for
> the amendments below, but it does **not** retroactively bless every future
> book's data sight-unseen. Cycle E2.0.5 (this document's origin) landed the
> *schema* and the *initial* blacklist; it did not retro-fit any book's
> data. Per-book retro-fit cycles (E2.0.6+, and T9's own per-record review
> per `SD-32 decisions.md §18`/§19) apply this blacklist and must record
> what they found — including any field this draft missed — back into this
> file before that book's data is considered license-clean.
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
| `description` (PCGen `DESC:`/`SPECIALS:`/`SA:`/`BENEFIT:` free-text tags) | `companion`-kind ability rows | Summoner-eidolon-evolution, animal-companion-trick, and familiar-archetype rules text. Reviewed corpus-wide (443 originally-uncertain rows, full read): entirely generic game mechanic in every row inspected — no deity/place/NPC content found. Presumptively OGL under §1(d)/(e)'s mechanic exclusion. 360 of the 443 nonetheless remain `still_undecidable` here, not because content was found, but because the classifier could not positively rule out a lowercase creature-species reference or an unlisted capitalized token in each one — see §4.2 for what closes this. |
| `description` (same tags) | `monster_ability`-kind rows | Special-ability text routinely embeds the *owning creature's own name* (via `KEY:<Creature> ~ <Ability>` and/or the DESC prose itself, e.g. "a jinushigami wields..."). Requires per-record judgment tied to the referenced creature's own PI status, not the ability row's content in isolation — if the named creature is not part of the SRD's declared-Open monster list, the ability row carries the same PI exposure as the creature name. See `SD-32 decisions.md §19b` — the operator has ruled that the row's own declaration governs (no declaration = not PI by association alone); the 954 previously-undecidable units this row named are `clear` under that ruling. |

**Added by `SD-32 decisions.md §19a` amendment 3a (2026-08-23, operator-approved).** Verbatim source: `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-3-closure-invariant/t9-pi-signoff-package.md §3a`. Both `companion` and `monster_ability` had *no* field-classification entry at all before this amendment, which is why 802 of T9's original 1,344 uncertain units had nowhere to look.

### 2.3a Normalization rule for the term-list scan (all six kinds; `SD-32 decisions.md §19a` amendment 3b, 2026-08-23)

> The term-list scan MUST case-fold and apply a bounded OCR-confusion
> normalization (at minimum: lowercase-l/uppercase-I/digit-1/exclamation-mark
> collapsed to one canonical character, matching the recorded
> lrori/Irori incident's error class; 0/o collapsed; rn folded to m) before
> substring matching, using WORD-BOUNDARY matching rather than bare
> substring. Word-boundary matching is required, not optional: a naive
> case-fold-only re-scan without it reopens a false-positive class where a
> short blacklist term (e.g. "Nex") collides with an ordinary English word
> ("next") the original case-sensitive scan never matched — found
> independently by two of the three review lanes and fixed the same way in
> both. The PCGen field delimiter "|" must NOT be included in any
> OCR-confusion table — folding it produces a false NEGATIVE on the Cayden
> CaiLean incident itself (confirmed by direct test).

Implemented in `scripts/sd32_t9_pi_review_feat_equipment.py` (`normalized_term_hit`) and `scripts/sd32_t9_pi_exposure_audit.py`'s sibling review scripts; tested in `scripts/tests/test_sd32_t9_pi_normalization_and_inheritance.py`.

### 2.3b `.COPY=`/`.MOD` inheritance rule (`SD-32 decisions.md §19a` amendment 3c, 2026-08-23)

> A PCGen `.COPY=`/`.MOD` row inherits its base item's declared
> NAMEISPI:YES/DESCISPI:YES status. A `.COPY=` derivative is mechanically
> the same named item as its base, with only cosmetic overrides
> (enhancement, price, name) — it is not new content. Resolve by
> same-file base-key lookup against the base's own declaration.

Resolves 5 units (`Gelugon Plate`, `Hellknight Half-Plate Barding`, `Hellknight Leather Barding`, `Hellknight Plate Barding`, `Maiden's Panoply` — all `adventurers_guide` equipment) from `clear` to `blocked`. Implemented in `scripts/sd32_t9_pi_review_feat_equipment.py` (`build_key_pi_index`/`find_base_item_pi`); tested in `scripts/tests/test_sd32_t9_pi_normalization_and_inheritance.py`.

### 2.3c Term-list additions (`SD-32 decisions.md §19a` amendment 3d, 2026-08-23)

`Aldori` (underlies the already-blocked "Aldori Dueling Sword", found via a feat prerequisite citation) and `Magaambya`/`Magaambyan` (a Golarion institution name, same shape as the existing place/nation terms) are added to the term list, bringing the review-script copies to 60 terms (`scripts/sd32_t9_pi_review_feat_equipment.py`; see `scripts/tests/test_sd32_t9_pi_normalization_and_inheritance.py`). Left undecided by the reviewing lane because the terms appear in mechanical `PREABILITY` prerequisite fields rather than a record's own name or flavour — the operator has now made that call (`SD-32 decisions.md §19a`): the citing record redacts too. **`src/rules_core/pi_screening.rs::PI_BLACKLIST_TERMS`** (production, 57 terms, asserted at `pi_screening.rs` line ~261) is not yet updated to 60 — that change belongs to the T9 onboarding cycle that actually transcribes corpus data under this amended blacklist, since bumping the production term list triggers cache regeneration across every already-shipped book, which is out of scope for this read-only sign-off-application cycle.

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

### Per-book override: Advanced Class Guide (added by cycle GE-01 wiring_class/PI-screening convergence, 2026-08-03)

- New term discovered not in §2 above: `Jarn` — classified PI (personal
  name; "creatures characters ... personas" per OGL §1(e)) because ACG's
  own E2.0.8 retrofit found this example NPC name embedded in
  `advanced_class_guide/spell/discern_next_of_kin.json`'s flavor text
  while sampling the book's real description text (recorded in that
  retrofit's own method note, but never folded back into this file or
  into `src/rules_core/pi_screening.rs`'s shared `PI_BLACKLIST_TERMS` —
  the gap this entry closes). Folded into the SHARED term list (not kept
  ACG-only), per this section's own instruction, so a future book
  carrying the same name is also caught. Verified before folding it in:
  `Jarn` occurs nowhere else in `data/corpus` or the PCGen source corpus,
  and no other blacklist term is a substring of it or vice versa, so
  adding it does not widen redaction anywhere beyond this one already-
  redacted record. `PI_BLACKLIST_TERMS` is now 55 terms (54 + this one).

### Per-book override: Inner Sea Gods (added by cycle SD31-W9-INTEGRATE-001, 2026-08-17)

- New terms discovered not in §2 above: `Cayden CaiLean` and `lrori` —
  both classified PI, because both are the pinned oracle's OWN
  miscapitalization/OCR-typo variant of an EXISTING §2 blacklist deity
  name (`Cayden Cailean`, `Irori`) that the exact-substring scan cannot
  see. `isg_spells.lst:46`'s `FACTSET:Deity|Cayden CaiLean` (capital `L`)
  shipped unredacted in `data/corpus/inner_sea_gods/spell/pick_your_poison.json`
  while all 51 of its correctly-spelled siblings redacted; `isg_spells.lst:8`'s
  own OCR of "Irori" as "lrori" (lowercase `L`) shipped unredacted in both
  `data.description` and `raw_tokens` in `abstemiousness.json`, and in the
  compiled, player-served `rules_tables::inner_sea_gods::spell_list::SPELL_LIST`
  table this book's spell catalog actually reads from. Found by an
  adversarial review of SD31-E6-F10-001's own screening (which correctly
  redacted the other 51 FACTSET deity hits and dropped 4 name-PI records)
  missing exactly these two oracle-typo forms. Folded into the SHARED term
  list (not kept ISG-only), per this section's own instruction. Verified
  before folding in: neither term occurs anywhere else in `data/corpus` or
  the PCGen source corpus outside these two records, and neither is a
  substring of, nor contains, any other blacklist term, so adding them
  does not widen redaction anywhere beyond these two records.
  `PI_BLACKLIST_TERMS` is now 57 terms (55 + these two). Both records
  re-screened and confirmed clean on disk and in the compiled table after
  the fix.

### Per-scope note: T9 PI sign-off (SD-32 card 11, `decisions.md §19`, 2026-08-23)

- **Two new terms added to the shared list** (§2.3c above): `Aldori`, `Magaambya`/`Magaambyan`.
  `PI_BLACKLIST_TERMS` is now 60 terms in the script-side copies (57 + these three: `Aldori`, `Magaambya`, `Magaambyan`).
- **`.COPY=`/`.MOD` inheritance rule added** (§2.3b): resolves 5 previously-`clear` T9
  equipment units to `blocked` — see that section for the named records.
- **§2.3 gained two new kind entries** (`companion`, `monster_ability`) with the
  normalization rule (§2.3a) applied to all six kinds' term scan, not only the kind that
  found the original incident.
- This sign-off does not itself redact any T9 record — a separate onboarding cycle
  (SD-32 card 11, not yet dispatched at sign-off time) applies these rules to the
  corpus and writes each affected book's `LICENSE.json` per §5 below.

### Per-book override: Inner Sea Gods, equipment (added by cycle `t9-onboarding`/`pi-key-rawtokens-screen` follow-up, 2026-08-23)

- **One new term added to the shared list** (the term itself is deliberately
  not repeated here — see `src/rules_core/pi_screening.rs::PI_BLACKLIST_TERMS`'s
  own trailing entry, which is this document's actual canonical source, not
  illustrative text) — classified PI (a deity name, "names and descriptions
  of characters... personas" per OGL §1(e)), the pinned oracle's OWN
  lowercase-possessive spelling of an already-blacklisted deity name (index
  9 of the same array). Found by the corpus-wide `data.key`/`data.raw_tokens`
  screen (`scripts/pi_key_rawtokens_audit.py`, `pi-key-rawtokens-screen`
  cycle) confirming `decisions.md §19a`'s own case-fold-normalized Python
  scan against the SIGNED-OFF list — the production Rust
  `pi_screening.rs::classify_field` this codebase's `equipment` generator
  (`src/bin/gen_equipment_gap_tables.rs`) actually runs at ingest time has NO
  case-fold normalization, so it never caught the oracle's lowercase
  variant. `isg_equip.lst:232` (the `Wayfinder Of Zephyrs` record's `DESC`
  token) shipped unredacted in both
  `data/corpus/inner_sea_gods/equipment/wayfinder_of_zephyrs.json`'s
  `data.description` and its `raw_tokens[DESC]` copy until this fix. Same
  shape as the Inner Sea Gods override immediately above (an oracle
  spelling/casing variant of an existing blacklisted deity), same
  resolution — fold the exact variant into the shared list rather than
  changing the scan's general matching rule. Verified before folding in:
  this variant (any case) occurs in exactly one PCGen source file at
  exactly two lines — one already excluded via that record's own
  `NAMEISPI:YES` declaration (line 20, an altar item naming the same deity),
  the other this leak (line 232) — so adding it does not widen redaction
  anywhere beyond this one record. **A whole-list case-fold was considered
  and rejected** for the Rust production copy: `PI_BLACKLIST_TERMS` includes
  a 3-letter term prone to colliding with an ordinary English word once
  case-folded (the same shape `§2.3a`'s own word-boundary guard exists to
  prevent, above), and a case-fold without that guard would reopen the
  identical collision class in the Rust copy — which deliberately has NO
  word-boundary guard at all, because real corpus identifiers concatenate a
  PI term into another identifier
  with no separator (e.g. a class-feature key ending `...LVL` immediately
  after a deity/place name). A single, verified, narrow term addition
  carries none of that risk. `PI_BLACKLIST_TERMS` (Rust production copy) is
  now 61 terms (60 + this one) — one ahead of the script-side copies' 60
  until a future sign-off cycle folds this addition into `§2.3c` the way
  the three `§19a` additions were. Re-derive:
  `cargo test --locked --lib rules_core::pi_screening::tests::term_list_matches_the_reference_copy_plus_the_documented_acg_addition`.

No other entries yet — cycles 2.0.6-2.0.9 (CRB, APG, ACG, Bestiary 1 retro-fits)
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
