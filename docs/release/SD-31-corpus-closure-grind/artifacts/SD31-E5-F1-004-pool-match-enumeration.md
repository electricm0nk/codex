# SD31-E5-F1-004 — `CLASS_FEATURE_POOLS` pool-name matching, per-entry enumeration

Card: `SD31-E5-F1-004` (`sd31-pool-match`). Fixes row 168 (pool-name matching) and row 181
(`slug()` apostrophe handling), `docs/release/SD-31-corpus-closure-grind/artifacts/OPEN-ISSUES.md`.

This is the evidence `class_feature_pool_group_matches_enumerates_the_real_corpus_against_every_pool_entry`
(`src/bin/v06_work_inventory.rs`, `class_feature_consumer_delta_tests` module) transcribes verbatim —
that test is a **permanent, always-run** regression test (not `#[ignore]`d), pinning both the
newly-reachable floor and the still-excluded ceiling named below. Regenerate this table with:

```
cargo test --locked --bin v06_work_inventory -- \
  class_feature_pool_group_matches_enumerates_the_real_corpus_against_every_pool_entry --nocapture
```

## 1. Re-derived sizing (row 168's own figure, reproduced independently a third time)

```
python3 -c "
import json, collections
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('kind')=='class_feature' and u.get('status') in ('not-ingested','not-started','unknown')]
dead=['Grand Discovery','Advanced Talents','Hex','Revelation','Blessing','Bloodline','Domain','Order','Mystery','Curse','Spirit','Animal Focus','Arcane School']
groups=[(u.get('corpus_key') or '').split(' ~ ')[0] for u in U if ' ~ ' in (u.get('corpus_key') or '')]
c=collections.Counter(next((w for w in dead if g!=w and g.endswith(' '+w)), None) for g in groups)
del c[None]
print(sum(c.values()), c.most_common())
"
```

→ **1562**, `[('Bloodline', 452), ('Domain', 422), ('Mystery', 234), ('Hex', 220), ('Spirit', 88),
('Blessing', 82), ('Animal Focus', 35), ('Arcane School', 17), ('Order', 9), ('Revelation', 3)]`

Matches wave 10's row 168 figure exactly (two independent implementations there — JSON/`dict` and
`awk` — a third here). Note only **10** of the 13 "dead" words carry any not-done population;
`Grand Discovery`, `Advanced Talents`, and `Curse` (3 units, folded into `Revelation`'s bucket by
this particular word list) show zero or near-zero because of §3 below.

## 2. Per pool-entry: which real corpus groups it now matches (fix applied)

The word-boundary suffix rule (`group == registered` OR `group.strip_suffix(" "+registered)` is
`Some`), gated by (a) no cross-class prefix token and (b) not in
`CLASS_FEATURE_POOL_FALSE_SUFFIX_MATCHES`:

| pool entry | owner | matched groups | count |
|---|---|---|---:|
| Rage Power | barbarian | Rage Power, Unchained Rage Power | 2 |
| Unchained Rage Power | barbarian | Unchained Rage Power | 1 |
| Discovery | alchemist | Alchemist Discovery, Discovery, Ice Chemist Discovery | 3 |
| Grand Discovery | alchemist | *(none — see §3)* | 0 |
| Rogue Talent | rogue | Psionic Advanced Rogue Talent, Psionic Rogue Talent, Rogue Talent, Unchained Rogue Talent | 4 |
| Advanced Talents | rogue | *(none — see §3)* | 0 |
| Hex | witch | Gain Witch Hex, Witch Grand Hex, Witch Hex, Witch Major Hex | 4 |
| Revelation | oracle | Soothsayer Revelation | 1 |
| Mercy | paladin | Mercy | 1 |
| Investigator Talent | investigator | Investigator Talent | 1 |
| Slayer Talent | slayer | Slayer Talent | 1 |
| Judgment | inquisitor | Judgment, Profane Judgment, Sacred Judgment | 3 |
| Inquisition | inquisitor | Black Powder Inquisition, Inquisition, Spellkiller Inquisition | 3 |
| Blessing | warpriest | 37 groups (Air/Animal/Artifice/…/Wildfire Blessing) | 37 |
| Evolution | summoner | Evolution | 1 |
| Bloodline | sorcerer | 63 groups (Aberrant/…/Warped Bloodline, incl. 10 Eldritch Scion Bloodlines) | 63 |
| Bloodrager Bloodline | bloodrager | 12 groups (Aberrant/…/Verdant Bloodrager Bloodline) | 12 |
| Domain | cleric | 74 groups (Air/…/Zeal Domain) | 74 |
| Order | cavalier | Cavalier Order | 1 |
| Mystery | oracle | 22 groups (Ancestor/…/Wood Mystery, incl. Oracle's Mystery) | 22 |
| Curse | oracle | Oracle's Curse | 1 |
| Spirit | shaman | 21 groups (Battle/…/Wood Spirit, incl. Secondary/Wandering variants) | 21 |
| Animal Focus | hunter | Hunter Animal Focus, Scarab Stalker Animal Focus | 2 |
| Favored Enemy | ranger | Basic Favored Enemy, Favored Enemy | 2 |
| Favored Terrain | ranger | Basic Favored Terrain, Common Favored Terrain, Favored Terrain | 3 |
| Versatile Performance | bard | Versatile Performance | 1 |
| Arcane School | wizard | Focused Arcane School | 1 |
| Focused Arcane School | wizard | Focused Arcane School | 1 |

**249 distinct corpus groups** now match a pool entry that could not before the fix (exact-match
only ever reached a group whose text equalled the registered word verbatim — the full list is
pinned in the regression test's `newly_matched` assertion set).

## 3. `Grand Discovery` and `Advanced Talents`: real, but not a suffix-boundary defect

Direct corpus read: **no `class_feature` corpus key anywhere has a GROUP prefix ending in
`" Grand Discovery"` or `" Advanced Talents"`** (checked via `docs/work-inventory.json`, zero
hits both ways). These two pools' actual members are filed under the *base* pool's own group text
instead — e.g. the Alchemist's Grand Discoveries (`Awakened Intellect`, `Fast Healing`,
`Enhance Potion`, `Eternal Youth`, `Poison Touch`, `True Mutagen`, …) are corpus keys
`Discovery ~ Awakened Intellect`, `Discovery ~ Fast Healing`, etc. — group `Discovery`, not
`Grand Discovery`. Likewise every Rogue archetype's "Advanced Talents" replacement feature is
filed as `Rogue Talent ~ Advanced Talent` / `Unchained Rogue Talent ~ Advanced Talent` (singular,
member not group). **Both pools' real membership already grounds through the `Discovery` /
`Rogue Talent` entries these two are redundant with** — zero units are stuck on this, and no
further matcher change reaches them because there is nothing left in the corpus with that group
shape to reach. Not fixed because there is nothing to fix; noted so a future cycle does not
re-open this as if it were still row 168's shape.

## 4. Groups the fix's own guards correctly refuse (eyeballed, one record read each)

18 groups satisfy the bare suffix shape but are excluded — 8 by
`CLASS_FEATURE_POOL_FALSE_SUFFIX_MATCHES` (each cited with its own corpus-record evidence in that
constant's doc comment: `Heretical Revelation`, `Shifter's Blessing`, `Spider's Blessing`,
`Zevgavizeb's Blessing`, `Totem Spirit`, `Inspired Discovery`, `Mutation Warrior Discovery`,
`Merciful Healer Mercy`, `Take Inquisition`), and 10 more by the live cross-class-prefix-token
guard (checked against this run's own `modelled_class_books()`, not a hand list):

| excluded group | why | spot-checked against |
|---|---|---|
| Druid Domain | `druid` is a different modelled class than `cleric` (Domain's owner) | RAW: Druid has its own Nature Bond domain option, distinct chassis |
| Inquisitor Domain | `inquisitor` is a different modelled class than `cleric` | Inquisitor trades a domain via a *cross-class* option (see `Take Inquisition`, below) |
| Samurai Order | `samurai` is a different modelled class than `cavalier` (Order's owner) | `class_books.get("samurai") == Some("ultimate_combat")`, pinned by an existing test |
| Shaman Hex | `shaman` is a different modelled class than `witch` (Hex's owner) | Shaman Hexes are ACG's own distinct subsystem, not Witch's Hex pool |
| Shaman Spirit Hex | same | same |
| Shaman Wandering Hex | same | same |
| Mutagenic Mauler Brawler Discovery | `brawler` is a different modelled class than `alchemist` (Discovery's owner) | Brawler archetype's own mutagen-flavored talent, not an Alchemist discovery |
| Skald Versatile Performance | `skald` is a different modelled class than `bard` (Versatile Performance's owner) | `advanced_class_guide/ce_skills.lst:25` cites `Skald Versatile Performance ~ Oratory` as its own distinct `PREABILITY` chain, separate from `Versatile Performance ~ Oratory` |
| Phrenic Slayer Favored Enemy | `slayer` token collides with the modelled `slayer` class, even though the real source is the unrelated Dreamscarred Press *prestige* class "Phrenic Slayer" | `psionics_unleashed_abilities_classes_prestige.lst:42-61`: `TYPE:SlayerFavoredEnemy...`, a prestige-class-only favored-enemy variant, confirmed NOT Ranger's own Favored Enemy pool |
| Take Inquisition | already in the false-suffix list; the cross-class guard would ALSO catch it (`Cleric`/`Druid` tokens) if it were not | `PRECLASS:1,Cleric=1` — a cross-class option letting a Cleric trade a domain, not Inquisitor's own progression |

Every one of the 18 is a real, different subsystem or a different modelled class's own feature —
**zero of the 18 are groups that should have matched but were wrongly excluded.** This is the
"eyeball the list for anything that does not belong" check the card names explicitly; nothing did.

## 5. Movement in both directions (the card's explicit ask)

- **Gains**: 249 corpus groups (§2) now recognised, versus the pre-fix exact-only set (which only
  ever matched a group whose full text equalled a registered word verbatim — of the 28 entries,
  only `Mercy`, `Investigator Talent`, `Slayer Talent`, `Evolution`, `Versatile Performance`,
  `Focused Arcane School` had any such exact-shaped group at all).
- **Losses**: **zero.** The suffix rule is a pure superset of exact-match (`group == registered`
  is checked first and unconditionally), and the two guards (cross-class prefix, false-suffix
  list) only ever *withhold* a newly-reachable suffix match — they cannot cause a previously exact
  match to stop matching, because neither guard runs when `group == registered` short-circuits
  true. Confirmed by `exact_match_is_unaffected` and `owner_class_name_as_its_own_prefix_is_allowed`
  (`v06_work_inventory.rs`).
- This is consistent with the mandate's demand to report a matcher that "only ever gains units" as
  a finding about the matcher: here the pure-gain direction is a property of the fix's own
  structure (superset relation), not an unexamined asymmetry — the 18-group refusal list in §4 is
  the evidence the gain was NOT admitted unconditionally.

## 6. What this does NOT yet mean for the board

Recognising a group (`class_feature_pool_group_matches` returning `true`) only removes the
`NoChoiceSlotOffersIt` outcome at `probe_class_feature_key`/`probe_class_feature_effect_wiring`.
Whether a given unit then reaches `Wired`/`done` depends on `probe_class_feature_key`'s own
consumer-delta test (does selecting THIS pool member move a fact the sheet renders, distinguishably
from another member of the same pool) — see the board-delta figures in `progress.md`'s receipt for
this cycle, which are lower than 1,562 because most of the newly-recognised population still lands
on `NoChoiceSlotOffersIt`→`NoConsumerDelta` (declined as not-attributable) rather than `Wired`, per
the `--class-feature-probe` ceiling report run this cycle.

**The board's own `classify()` path is separate from that probe** and is what actually decides a
unit's persisted `status` in `docs/work-inventory.json` (`class_feature_exact_suffix_grounded` /
`suffix_stripped_grounded`, gated by [`class_feature_pool_group_matches`] the same way). The
guarded regen this cycle moved **38 units total, all `class_feature`, all forward, zero
regressions**: 35 from the pool-match fix (10 Shaman Spirit + 10 Secondary Shaman Wandering Spirit
+ 10 Shaman Wandering Spirit + 3 Witch Hex + 2 Sorcerer Bloodline) and 3 from the row-181
apostrophe fix (`Ranger ~ Hunter's Bond`, `Druid ~ Resist Nature's Lure`,
`Unchained Summoner ~ Maker's Call`). Board `done`: 11,229 → 11,267 (+38); every other kind
unchanged (equipment held exactly at 4,372, confirming §7 below's stamp-loss near-miss was fully
corrected before this regen ran). Re-derive:
```
python3 -c "
import json, sys, collections
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d = json.load(open('docs/work-inventory.json'))
U = [u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
c = collections.Counter(P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind')) for u in U)
print(len(U), dict(c), round(100*c['done']/len(U),4))
"
```

## 7. `slug()` scoping correction (near-miss, self-caught before commit)

The row-181 apostrophe fix was FIRST attempted by editing the shared `slug()` function directly
(the same function [`unit_id`] uses to build every kind's persisted id, not only
`class_feature`). The guarded regen's own stamp-preservation check correctly refused that first
attempt: swallowing the apostrophe in `slug()` renamed every apostrophe-bearing `equipment` id too
(`assassin_s_dust` → `assassins_dust`, etc.), which would have dropped 304 of 6502
literal-verified/fixture-verified stamps under **completely unrelated units this card was never
asked to touch** — a real, if inadvertent, widening of the change's blast radius past what row 181
asked for. Corrected by introducing `class_feature_engine_join_slug` — a copy of the apostrophe-
swallowing behaviour used ONLY at the two class_feature join sites
(`class_feature_exact_suffix_grounded`'s `feature_slug`, and the pool-member `selection_id` inside
`probe_class_feature_key`) — and reverting `slug()`/`unit_id` to their original, unmodified
behaviour. Re-running the guarded regen after the correction produced the clean +38 result in §6
above with zero stamp loss (confirmed: the regen wrote successfully with no
"refusing to write" refusal, and `equipment`'s `done` count is unchanged at 4,372 before/after).
No units outside `class_feature` moved in either version of the fix; the near-miss was caught
before it ever reached a commit.
