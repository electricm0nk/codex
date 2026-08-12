# Tasks #11-15 — Stale-Framing Sweep

> After four task descriptions in a row proved wrong in the same direction
> (#1, #10, #6, #5 — each inheriting a "too expensive" framing that predates
> either the corrected standalone-grounding bar or a corpus re-derivation),
> the lead asked for the same check on the remaining pending rows before any
> of them get used as a build brief. Not full scoping docs — a verdict each.

**Result: 3 of 5 are genuinely stale, 1 is partly stale, 1 is correct.**

---

## #11 Witch — STALE. "Build spellcasting (own list)" understates nothing; it overstates the cost.

Witch's spell list is **324 unique records** (212 `.MOD` grafts + 112 new),
spell levels 0-9. **All 324 are already present in this repo's ingested
corpus — zero missing.** That is better coverage than any list checked this
segment (Bloodrager was 110 of 183).

> **Count corrected 2026-07-27 (was 249).** My first pass tested
> `"Witch=" in line`, which misses every comma-grouped class list where
> Witch is not the element immediately before the `=` — e.g.
> `CLASSES:Witch,Wizard=3`. That dropped 75 records (~30%). The lead's own
> independent count (252) was closer but low for the same reason. The
> figures here come from a real `CLASSES:` parse: split on `|`, `rpartition`
> each group on `=`, then test membership in the comma-separated name list.
> **The conclusion was unaffected — zero missing either way — but a right
> conclusion does not excuse wrong specifics.**

So there is no ingestion cost at all. The real work is a `witch_spell_list.rs`
module mapping already-ingested spells to Witch levels — structurally
identical to `alchemist_spell_list.rs` (104 records), just ~3.1× the rows.
The standing "no reusable spell list exists for Witch" note is *true but
misleading*: it means no other class's built list can be borrowed, not that
the spells need ingesting.

Chassis: `HD:6`, `SPELLSTAT:INT`, `KNOWNSPELLS:LEVEL=0` (all orisons known),
no explicit `MEMORIZE` token — prepared-shape by PCGen default, worth
confirming at build time rather than assuming.

**The Familiar half of the description is still correct** — genuinely
unbuilt. But note the Summoner/Eidolon pass just established the shape a
bounded companion-creature MVP takes (fixed canonical form, chassis math as
standalone records, defer the customization economy); a Familiar MVP could
follow it rather than needing a design pass from scratch.

## #12 Shaman — STALE, same shape as Witch.

**304 unique records** (267 `.MOD` grafts + 37 new), levels 0-9, **283 of
304 already ingested, 21 missing** (93% coverage — better than Bloodrager's
60%, which was already ruled acceptable). `HD:8`, `SPELLSTAT:WIS`,
`MEMORIZE:YES` (prepared). *(Count corrected from 281 by the same
comma-group parse fix noted under #11; conclusion unchanged.)*

"Build fresh spellcasting" implies from-scratch ingestion. It is the same
already-ingested-spells / build-the-list-module job as Witch, with the 22
unreachable spells routed through the existing unresolved-selection idiom.
Familiar half correct, same Eidolon-precedent note as above.

## #13 Slayer — PARTLY STALE. The two halves are independent; only one needs the design pillar.

The description sequences it as "design opponent-tracking pillar for Studied
Target, **then** ground Slayer Talents." Those are unrelated. **Slayer
Talents is a chooser, not an opponent-tracked mechanic** — narrowable to one
canonical pick today, exactly as Animal Focus / Oracle's Mystery / Cavalier's
Order were, with no design work needed. Gating it behind the pillar is a
sequencing error that would keep a tractable slice blocked.

The Studied Target half is **correctly blocked**, and I want to be explicit
that this is not stale framing: its magnitude
(`SlayerStudiedTargetBonus = SlayerLVL/5+1` → +1/+2/+3/+4/+5 at levels
1/5/10/15/20) is real, but it applies *only* against a studied opponent, and
the lead already ruled deliberately that opponent-conditioned bonuses stay
deferred rather than being grounded as "conditional" facts. That ruling
still holds.

One thin exception worth naming: `SlayerStudiedTargetProgression =
(LVL>0)+(LVL>6)` is a **capacity** (how many targets can be studied at once
— 1 at 1st, 2 at 7th), which is self-scoped in the same way a uses/day pool
is. Groundable, but genuinely minor on its own.

## #14 Swashbuckler — STALE, and the sharpest find of the sweep. The "mechanism to design" already exists.

`KEY:Swashbuckler ~ Swashbuckler Finesse` carries:

```
BONUS:VAR|CombatFeatIntRequirement|max(CHASCORE,INTSCORE)|TYPE=Base
```

**That is the same variable and the same idiom as Brawler's Cunning**
(`BONUS:VAR|CombatFeatIntRequirement|max(13,INTSCORE)|TYPE=Base`) — which is
already built and grounded as
`brawler_cunning_effective_intelligence_score` (`pilot_compute.rs:14344`,
wired at `:14431`).

So "design feat-prereq-substitution mechanism for Finesse" describes work
that was already done for a different class, against the identical corpus
variable. This is a **reuse, not a design problem** — the same category as
Hunter reusing Druid's companion math.

Honest bound: only the *prerequisite-substitution* half reuses. The other
half ("gains the benefits of the Weapon Finesse feat with light or
one-handed piercing melee weapons") has no token and would need real
attack-mechanics work — that part stays deferred. And "then ground Deeds"
is, like Slayer Talents, an independent chooser that does not depend on the
Finesse question at all.

## #15 Monk / Deflect Arrows — CORRECT. Genuinely blocked, leave as-is.

The CRB `Deflect Arrows` feat record carries **zero `BONUS` tokens of any
kind** — only `TYPE:Combat`, its two prerequisites (Improved Unarmed Strike;
Dex 13), and DESC text. There is no magnitude to ground even under the
corrected bar, and the ability is purely reactive against an incoming ranged
attack, which this engine models nowhere.

This is a genuine no-op *and* a genuine subsystem gap — the one row of the
five whose framing survives contact with the corpus unchanged.

---

## Suggested re-routing

| task | verdict | what it actually is |
|---|---|---|
| #11 Witch | stale | build a spell-list module from **324** already-ingested spells (`alchemist_spell_list.rs` shape), 100% reachable; Familiar still deferred |
| #12 Shaman | stale | same, **304** records / **283** reachable (93%); Familiar still deferred |
| #13 Slayer | partly stale | split it — Slayer Talents is narrowable now; Studied Target correctly stays deferred |
| #14 Swashbuckler | stale | Finesse's prereq-substitution is a direct reuse of built Brawler's Cunning code; Deeds is an independent chooser |
| #15 Monk | correct | genuinely blocked, no change |

Three of these (Witch's list, Shaman's list, Swashbuckler's Finesse reuse)
are ready to scope properly on request; #13's split needs only a description
rewrite, not a scoping pass.
