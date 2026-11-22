# Inquisitor (#45) — Remaining Features

> Brief framed Inquisitor as "thin, largely chooser-shaped remainders."
> **Enumerating fresh contradicts both halves: the dominant remaining item is
> spellcasting — 219 spells, 100% already ingested — which is neither thin
> nor chooser-shaped.** The genuinely chooser-shaped part is small.
>
> A second finding worth carrying beyond this class: **my own roster survey
> undercounted Inquisitor**, because one of its grounded features has no
> `class_feature.*` id at all.

## Correction to the survey method: consumer-integrated features are invisible to id counts

Survey (#40) counted Inquisitor at 4 real grounded features. It is **5**.

**Stern Gaze is grounded** — `inquisitor_stern_gaze_intimidate_bonus` /
`active_inquisitor_stern_gaze_bonus` (`pilot_compute.rs:10159`, `:10170`) —
but it produces **no explanation record**, because it feeds the *computed
Intimidate skill modifier* directly. It is a consumer integration, not a
standalone fact.

**So id-based surveys systematically undercount features that landed as
total-integrations rather than standalone records.** That is a real
limitation of the #40 methodology, affecting any class whose wins went into
computed totals — and it biases *against* exactly the highest-value kind of
work. Worth noting on item 60/63 alongside the prefix rule.

## Grounded today (5)

Judgment (execution + uses/day), Monster Lore (`= WIS`), Cunning Initiative
(`BONUS:COMBAT|INITIATIVE|WIS`), Track (`TrackLVL = InquisitorLVL`), and
Stern Gaze (`max(1, InquisitorLVL/2)` on Intimidate + Sense Motive).

## The dominant remaining item: spellcasting

Not mentioned in the brief, and it is the bulk of what is left.

| measure | value |
|---|---|
| unique spells | **219** |
| **ingested in this repo** | **219 — zero missing** |
| levels | 0-6 (15/38/43/44/35/24/20) |
| posture | `SPELLSTAT:WIS`, `MEMORIZE:NO` → **spontaneous** |
| list source | **its own** — no `SPELLLIST:` reuse token |

Ceiling cross-check: 220 raw lines contain the literal "Inquisitor" against
219 parsed, a one-line gap consistent with an incidental non-`CLASSES:`
mention. Clean.

**Shape:** exactly the Witch job — build an `inquisitor_spell_list.rs`
module over already-ingested spells — combined with the **Oracle**
spontaneous validation shape (`MEMORIZE:NO`), *not* the Alchemist/
Investigator prepared machinery. Same two-part pattern just scoped for
Hunter (#43), except Inquisitor needs its own module because it has no
`SPELLLIST` union to borrow.

Currently **entirely unbuilt** — zero `class_spell.*inquisitor*` ids.

## Flat and groundable now

**Bane** — `BONUS:VAR|InquisitorBanePool|InquisitorLVL` → rounds/day pool.
Flat, self-scoped, the Fervor/Panache idiom. Small but clean.

## Genuinely chooser-shaped

**Sacred Judgment / Profane Judgment** — each grants four additional judgment
types (Healing, Resiliency, Resistance, Smiting) via
`ABILITY:…|AUTOMATIC|Judgment ~ <type>`. These extend the **already-built**
Judgment machinery rather than needing new mechanism, so a canonical
narrowing here is cheaper than most: the execution path exists, only the
type table widens.

## No numeric magnitude — correctly deferred

Orisons (folds into spellcasting, same reasoning as Arcanist's Cantrips),
Detect Alignment, Solo Tactics, Discern Lies, Stalwart, Exploit Weakness,
Slayer, True Judgment, Class Skills.

**One honest sub-distinction:** Second Judgment, Third Judgment and Greater
Bane carry no corpus token, but they are **not semantic no-ops** the way
Nature Training is — they multiply existing mechanics (use two/three
judgments at once; double Bane's bonus). They are DESC-only rather than
content-free, so if they are ever wanted the magnitude must come from a
primary source, not from the corpus.

## Nothing is subsystem-blocked

No item here needs an absent engine.

## Recommendation

1. **Spellcasting** — the real prize. 219 records, fully ingested, own
   module (Witch shape) + Oracle spontaneous validation. Largest single
   remaining win and independent of everything else.
2. **Bane pool** — trivial, fold in wherever convenient.
3. **Sacred/Profane Judgment** — optional canonical widening of shipped
   Judgment machinery.

**Honest expectation:** spellcasting alone takes Inquisitor from 5 grounded
features to 6 and closes its largest gap; it stays Blocked on
`other_features_deferred` (Solo Tactics, Bane's greater tiers, the extra
judgment slots).

## Open question

**Is #45 the right container for this?** The brief scoped "remaining
chooser-shaped features," but the dominant finding is a spell-list build
that is neither chooser-shaped nor small — and it is the same shape as
Hunter's #44. It may be cleaner as its own task alongside #44, leaving #45
for Bane plus the judgment-type widening. Your call on splitting.
