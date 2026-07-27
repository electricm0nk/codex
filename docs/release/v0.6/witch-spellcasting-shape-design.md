# Witch Spellcasting — Caster-Shape Design Pass

> Backend stopped before building #11's spellcasting piece: Witch's caster
> posture is a **third shape**, not a variant of either already-built one.
> Lead independently re-verified the corpus facts, so this pass builds on
> them rather than re-deriving.
>
> **Two findings. First, the third shape is real — and it is specifically a
> *hybrid*, which is why neither existing implementation fits. Second, and
> more usefully: "prepares from the familiar" needs no familiar object, so
> this is *not* a reason to build the Familiar's deferred Tier 2.**

## The three caster shapes, side by side

| class | tokens | knowledge source |
|---|---|---|
| Cleric / Shaman | `KNOWNSPELLS:LEVEL=0\|1\|2\|…\|9` | **the entire list**, auto-known at every level |
| Wizard / Arcanist / Investigator | `SPELLBOOK:YES` | **a recorded store**; only what's in it |
| **Witch** | **`KNOWNSPELLS:LEVEL=0` only** | **cantrips auto-known; levels 1-9 have no declared source at all** |

Witch has neither the full ladder nor a spellbook. That is the whole
finding, stated precisely:

> **Witch is Cleric-shaped at level 0 and Wizard-shaped at levels 1-9.**

It is a hybrid, not a third independent mechanism — which is exactly why
calling it "the same as Cleric/Druid" is half right, and why applying either
existing implementation uniformly would be wrong in one half.

## The key finding: the familiar-as-store needs no familiar

Backend named two paths — a familiar-as-spell-store mechanism, or a bounded
MVP that blocks on the missing store — and flagged this as arguably the
first real reason to build the Familiar's Tier 2. **It isn't, and the reason
is that this repo's "spellbook" is already not an object.**

`ground_wizard_prepared_spellbook` reads `spells_selected`, filtered by
`AcquisitionMode`. There is no spellbook item, no container entity, no
inventory coupling — the book is a *semantic reading of an existing list*.
And `AcquisitionMode`'s three variants map exactly onto Witch's three
knowledge roles:

| mode | Wizard | **Witch** |
|---|---|---|
| `Known` | recorded in the spellbook | **recorded in the familiar** |
| `Prepared` | prepared today from it | prepared today from it |
| `Granted` | — | **patron bonus spells** |

A spell store is a set of spell identities. **The familiar's stat block —
its HD, saves, Intelligence, skills — is irrelevant to storing them.** This
is the same result the Studied-Target and Familiar-master-benefit passes
reached: the creature is not an input to the mechanic. Third instance of the
pattern.

So Witch's spellcasting is implementable **today, reusing
`ground_wizard_prepared_spellbook`'s exact shape with the flavour
relabelled**, and the Tier-2 deferral is untouched by it.

## The honest gap, named rather than papered over

There is a real interpretive step here, and it should be explicit in the
code rather than implied.

**The corpus declares no knowledge source for Witch's levels 1-9.** No
`KNOWNSPELLS` ladder, no `SPELLBOOK:YES`. RAW says the familiar holds them;
the corpus does not encode that mechanically — consistent with the earlier
Familiar pass, which confirmed **zero `PRE` gates on `Witch ~ Cantrips` or
`Witch ~ Patron Spells` and zero `PREABILITY` references to
`Witch ~ Familiar` anywhere.**

So treating `spells_selected` + `AcquisitionMode::Known` as the store is a
**deliberate modelling choice, not a corpus-stated fact.** It is the right
one — it is the only container that exists, it matches RAW's intent, and it
reuses shipped code — but the explanation text should say so plainly, the
same way Sacred Weapon's evidentiary note and Swashbuckler's gate
substitution do.

## Patron spells — the `Granted` third

`Witch ~ Patron Spells` grants **12 `SPELLKNOWN:CLASS|Witch=<level>|<spell>`
records**, gated on `PREVARGTEQ:WitchPatronLVL,{2,4,6,8,10,…}` — structurally
identical to Bloodrager's bloodline bonus spells and Cleric's domain spells.
The patron itself is a chooser, so under the ratified Skill Focus precedent
these ground only with an explicit recorded patron choice, never a silently
seeded canonical one.

## Recommended scope

Backend's path (b), but stronger than "blocks on the missing store" — **the
store exists**:

1. **Cantrips**: Cleric-shaped. All level-0 Witch spells auto-known via
   `KNOWNSPELLS:LEVEL=0`.
2. **Levels 1-9**: Wizard-shaped. `Known` = recorded in the familiar,
   `Prepared` = prepared today, mirroring
   `unmet_wizard_spellbook_conditions` / `ground_wizard_prepared_spellbook`
   with no opposed-school burden (Arcanist's simpler variant).
3. **Slots/day**: reuse Cleric's CAST table — backend confirmed byte-identical
   through all 20 levels including L11.
4. **Save DC**: `10 + spell level + INT`.
5. **Patron spells**: deferred unless the patron chooser is recognised;
   named in the diagnostic either way.

**No Tier 2. No new architecture.**

## Build-time hazards

1. **The existing comment error is narrower than it looks — fix the comment,
   not the parser.** `spellcasting_class.rs:22-26` says Witch has "the same
   absent-signals prepared-casting shape as Cleric/Druid." The
   *absent-signals* half is correct and so is the parser: it derives posture
   from `MEMORIZE`/`SPELLBOOK` and **never reads `KNOWNSPELLS` for any
   class**. Only the editorial "same as Cleric/Druid" claim is wrong. Whether
   the posture enum should gain a third value is a separate question — do not
   "fix" the parser on the strength of a wrong comment.
2. **One class, two treatments.** Cantrips and levels 1-9 need *different*
   handling inside a single class — a first for this codebase. Applying
   either rule uniformly silently breaks the other half.
3. **The store is an interpretation.** See above — it must be stated in the
   explanation text, not assumed.
4. **Spell count: 326 is authoritative — already settled, do not re-open.**
   My comma-aware parse gave 324; task #33 ("fix remaining bracket-prereq
   gap, 324 → 326") closed the difference. The residual two were
   bracket-prereq forms my parser did not handle, on top of the comma-group
   forms the naive substring test missed. Use **326**. Worth knowing that the
   same substring-matching defect turned out to be present in seven
   already-shipped lists (tasks #24-#32 — Sorcerer 394→578, Druid 169→271,
   Bard 164→264, Cleric 236→301, Alchemist 104→147, Ranger 51→114, Paladin
   45→95), so any *new* list built here should use the parse form, never
   `grep -c "<Class>="`.
5. **Cleric's CAST table is the *slots* table, not knowledge.** Reusing it is
   correct and confirmed; do not let that reuse imply Cleric's knowledge
   model comes with it — that is precisely the conflation the wrong comment
   made.

## Open questions for the lead

1. **Confirm the store interpretation** — treat `spells_selected/Known` as
   the familiar's store, with the modelling choice stated explicitly in the
   explanation text? I recommend yes; it is the only container available and
   it reuses shipped code.
2. **Patron spells in or out of this slice?** They need the patron chooser
   recognised first. Cleanest as a follow-on, keeping this slice to the
   caster shape itself.
3. **Does the ingest parser's posture enum need a third value**, or is a
   comment fix sufficient? My read is comment-only: the parser's job is
   posture derivation from the tokens it reads, and it does that correctly.
   The hybrid distinction matters in the compute layer, which is where this
   design puts it.
