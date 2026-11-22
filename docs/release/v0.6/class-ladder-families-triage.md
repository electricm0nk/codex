# Class-Specific Power Ladders — Sibling Triage (#62)

> First-pass triage, not build-ready scoping. Families were **discovered
> empirically** rather than checked off the brief's list: every
> `KEY:<prefix> ~ <power>` record grouped by the prefix's trailing noun, then
> counted for magnitude-bearing records (excluding `…LVL` plumbing).
>
> That surfaced four families the brief didn't name — Hexes, Spirits,
> Favored Enemy, Favored Terrain — and one clear winner.
>
> **#1 is Cleric Domains: bigger than Bloodlines *and* the only family so far
> with one genuinely shared namespace across three classes.**

## The ranked table

| family | records | magnitude-bearing | namespace | classes | grounded today |
|---|---|---|---|---|---|
| **Domain** | **292** | **159** | **one shared** (34 domains) | **Cleric + Inquisitor + Druid** | 10 ids, Cleric-only |
| Bloodline | 244 | 149 | **parallel** | Sorcerer \| Bloodrager | 7 ids — scoped (#60/#61) |
| School | 177 | 96 | likely shared | Wizard + Arcanist | 12 ids |
| Hex | 143 | 82 | — | Witch + Shaman | 6 ids |
| Mystery | 130 | 85 | one | Oracle | 11 ids |
| Spirit | 70 | 63 | one | Shaman | 16 ids |
| Favored Enemy | 31 | 31 | one | Ranger | 19 ids |
| Companion | 22 | 22 | shared | Druid/Hunter/Cavalier | — |
| Favored Terrain | 18 | 18 | one | Ranger | 11 ids |

(Entries like `Bonus`, `Selection`, `Times`, `Base` and bare class names are
grouping artifacts of the discovery method, not real families, and are
excluded. Subdomain/Subschool are almost entirely plumbing — 2 magnitude
records each.)

## #1 — Cleric Domains. The strongest candidate found so far.

**Two things make it better than Bloodlines**, which was the previous
largest:

1. **Bigger** — 159 magnitude-bearing records vs Bloodlines' 149, from 292
   total records.
2. **Genuinely shared, and this one I checked specifically** because the
   Bloodlines classification was wrong the same way. Domains are **one
   namespace**: 34 distinct `<Name> Domain ~ <Power>` prefixes (Air, Animal,
   Artifice, Chaos, Charm, Community, Darkness…), with **no per-class
   variants**. Contrast Bloodlines' `<Name> Bloodline ~` versus
   `<Name> Bloodrager Bloodline ~`.
3. **Three classes draw on it** — separate pool grants confirmed for
   `Inquisitor Domain Choice`, `Druid Domain`, and Cleric's own.

So this is the first family since Rage Powers with real multi-class
leverage, and it is an order of magnitude larger.

**Existing coverage is partial and Cleric-scoped.** Ten grounded ids —
`class_chassis.cleric.domain_choice`, `…domain_power_good_touch_of_good_*`,
`…domain_power_healing_rebuke_death_uses_per_day`, `…domain_spell_slot` —
so Good and Healing domains have real magnitudes, and the domain-choice
machinery works.

**But the machinery is namespaced `class_chassis.cleric.*`, not
class-agnostic.** That differs from the familiar machinery
(`class_feature.familiar.*`), which Arcanist can reuse directly. Extending
domains to Inquisitor and Druid therefore needs a **generalisation pass
first** — which is exactly the kind of one-time investment that pays across
three classes, and is the main reason this deserves a dedicated scope rather
than a quick pick.

## #2-3 — Schools, then the partially-covered trio

**Arcane Schools (177 / 96)** — `Arcanist School`, `Arcanist School Power`
and `Arcanist Savant Opposition School` pools confirm **Arcanist draws on
Wizard's schools**, so this is a genuine two-class family. Second-ranked on
size and sharing. (Namespace not yet verified — treat "shared" as probable,
not confirmed, until it is.)

**Hexes (143 / 82, Witch + Shaman)**, **Mysteries (130 / 85, Oracle)** and
**Spirits (70 / 63, Shaman)** all have real remaining depth but **already
carry partial coverage** from this session's class work (Ward hex, Life
Mystery, Life Spirit). They are widenings of existing threads rather than
new seams.

## Smaller, cleaner candidates worth knowing about

**Favored Enemy (31/31) and Favored Terrain (18/18)** are notable for a
different reason: **100% of their records carry magnitudes** — no tokenless
filler at all, the only families where that is true. Both are Ranger-only
and already have substantial coverage (19 and 11 ids), so they are close to
done rather than open seams — but if a small, high-density target is ever
wanted, they are the cleanest on the roster.

## Recommendation

**Scope Cleric Domains as the next dedicated ladder workstream**, after the
Draconic slice (#61) lands. Suggested shape: generalise the Cleric-scoped
domain machinery first, then canonical-narrow one domain — the generalisation
is what converts this from a one-class widening into three-class leverage.

**Do not** treat Schools/Hexes/Mysteries/Spirits as parallel candidates yet;
they rank below Domains on every axis, and three of them are widenings of
threads this session already opened.

## Method note

The empirical discovery mattered. Checking only the brief's named list would
have missed Hexes, Spirits, Favored Enemy and Favored Terrain entirely, and
would not have surfaced that Domains — not on the list as the top
candidate — outranks everything scoped so far. Same principle as
enumerate-the-prefixes: **derive the categories from the data, don't supply
them.**
