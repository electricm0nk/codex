---
canonical: true
purpose: Structural work that unblocks many units at once. Ranked by units-per-unit-of-effort, not
  by size. This is where wave targets come from.
---

# Levers

| # | Lever | Unblocks | Status |
|---|---|---|---|
| L0 | **Prestige-class entry-requirement gating.** Does not exist anywhere in the codebase. 77 of the 157 not-done classes need it — every CRB prestige class (Arcane Archer, Assassin, Duelist, Shadowdancer, Eldritch Knight...) plus APG's and Ultimate Psionics'. The single largest structural gap found by the wave-27 census. | 77 classes, and every feature they grant | NOT STARTED |
| L1 | **Class chassis.** Only 11 classes can be built (the CRB base classes). `class` is 28 done of 185. Every prestige class, every ACG/APG class, every archetype hangs off this. Wave 26 proved features for unbuildable classes compute correctly and reach nobody. | gates `class_feature`, which is 60% of everything remaining | **S1 SWEPT, zero Monk-shaped left.** Remaining work is L0 (77 prestige), ≥20 untabled base classes (corrected wave 28, was 18 — see `sweeps.md`), 28 unstarted books. **`class` (157 units) and `race` (60 units) were the two kinds NO wave-28 lane examined — `race` in particular has no census on file at all; worth a dedicated look before the next class-chassis push.** |
| L2 | **Domain-power grounding past its allowlist.** `ground_or_block_cleric_domain_power` grounded only Good and Healing because each domain needed a hand-written function. Wave 25 salvage added War and Strength. With the interpreter, the allowlist should become unnecessary. | every Cleric/Inquisitor domain in every book; 15 of Bestiary 6's 18 class_feature units alone | IN PROGRESS |
| L3 | **Monster-chassis ↔ companion-ability bridge.** Named by the Bestiary 6 ledger as a general fix appearing across many books. | 28 units (sized wave 28 via a grep-based exact-string cross-reference of `companion::external_ability_refs` — 377 distinct strings — against `monster_ability` corpus_keys; down from "unknown") | NOT STARTED — now sized |
| L4 | **Race-trait flat-override compute seam.** Landed for three races in wave 25 salvage; race_trait moved for the first time in six waves. | race_trait, 234 units currently correctly held back with a proven seam pattern (G6, wave-28 census) — 21 races/heritage variants named individually, each needs its own `explain_<race>_flat_override_race_trait` seam, per-record-verified before crediting (post-Undine discipline) | IN PROGRESS |
| L5 | **Option-pool catalog scaling.** Produced +109 from Rage Powers alone — the largest single class_feature win so far. ~10,000 units sit in pools no grant parser can reach because they are chosen, not granted. Constrained by ruling 18: open pools only. | up to ~10,000, minus exclusive pools; wave 28 census: class_feature's own G1 alone is 3,347 units/817 pool names, only 6 names (161 units) classified OPEN vs EXCLUSIVE so far | PARTIALLY BUILT (2 of 27 pools registered) |
| L6 | **The 36 grant facts with no DESC at all.** Their rules text lives on the class progression table, not a per-feature description. No `%N` mechanism will ever reach them. | **corrected wave 28: the real population is ≥1,764 within `class_feature` alone** (class_feature's own G3, magnitude-bearing units with no consumer function, is 2,583; of these, a shallow-but-unverified scan suggests roughly 1,764 lack a %N-reachable description entirely — flagged as needing a real re-derivation, not the "36 known" figure this lever was filed at) | NOT ANALYSED |
| L7 | **(new, wave 28) Template-application mechanism.** PF1e creature/familiar TEMPLATES (Ogrekin, Animated Object, Dread Lord, Celestial/Fiendish, Mana Wastes Mutant, Fungal Creature, Petitioner...) have no single stat block for the current per-creature chassis model to hold. | **≥479 units** (corrected wave 28 via adversarial review from a filed ≥547 — 430 monster_ability + 49 companion, only 2 of the pile's 3 kinds checked, so this is a floor) | NOT STARTED — a genuinely new engine mechanism, not data transcription |
| L8 | **(new, wave 29) Class-feature description reference surface — the real blocker behind THE-BOX §2.1 F1.** Wave 29's lane 3 was dispatched to widen `classify()` to accept `class_feature_descriptions.rs`'s render catalog (`list_class_feature_descriptions`) as proof-of-holds for G1/G2. Re-confirmed by direct code read (not by inspection alone) that this cannot reach a player today: `classFeaturesModel.ts`'s `buildClassFeatureSurface` only ever creates a row by iterating `ExplanationDto[]`; `descriptions` is consulted ONLY as enrichment (`corpusDescription`) on a row an explanation already created. Unlike `list_class_feature_pool_options` → `ClassFeaturePoolReferenceSection` (`CharacterSheet.tsx`), which renders its FULL catalog independent of held explanations, no equivalent standalone browsable surface exists for `list_class_feature_descriptions`. A locking regression test now pins this (`classFeaturesModel.test.ts`, `verifiesADescriptionWithNoMatchingExplanationProducesNoRowAtAllRegardlessOfHowManyDescriptionsExist`): 0 rows for 0 explanations regardless of description count. Widening `classify()`'s eligibility without this surface would credit `done` for text no player screen shows — the exact Decision 7 condition 3 / DoD-8 violation the wave-29 dispatch warned against. Same shape as L7: a new render surface, not a data-transcription task. | up to ≤2,763 (THE-BOX §2.1 F1 ceiling, itself gated further behind Ruling §18's OPEN/EXCLUSIVE pool split for the ~1,656 G1-shaped units inside it) | NOT STARTED — new, unscoped, uncosted frontend surface required before any `classify()` widening on this evidence is honest |

### L5 sub-finding (wave 29 lane 2) — a method for the OPEN/EXCLUSIVE axis, applied to the 27 `CLASS_FEATURE_POOLS` entries

Ruling §18 makes OPEN-vs-EXCLUSIVE load-bearing for any pool-catalog widening, and THE-BOX (wave
28) found 811 of 817 distinct option-pool group names never classified on that axis. This lane was
not asked to classify all 817, only to establish the method and classify as many as it can defend.

**The method:** for each pool, read the class's own selection mechanic (not the corpus row, which
does not encode this) against one question — *does a character gain ADDITIONAL instances of this
choice over the course of leveling, keeping every earlier pick (OPEN), or is the choice made
EXACTLY ONCE, permanently, at a fixed level (EXCLUSIVE)?* This is the same distinction Ruling §18
draws for Cleric Domain ("once a character takes the Void domain, Scalykind's powers are not
available to them").

**Applied to all 28 `CLASS_FEATURE_POOLS` entries** (this file's own registry, not the wider
817-name census — a bounded, defensible slice of it; `Arcane School` and `Focused Arcane School`
share one table row below since both are Wizard's same EXCLUSIVE choice). **Correction against this
lane's own dispatch brief**, which cited "27-entry pool table" — `awk '/const CLASS_FEATURE_POOLS/,
/^\];/' src/bin/v06_work_inventory.rs | grep -c '^\s*("'` counts 28, not 27; re-derived rather than
trusted, per the standing instruction. Self-derived from standard, widely-published PF1e
class-mechanic text, **not per-group re-verified against the pinned oracle** — flag this the same
way `class_feature_pool_group_matches`'s own false-suffix list demands: treat as PROPOSED,
spot-check before any pool moves from this list into `REGISTERED_POOL_GROUPS` (the render catalog
Ruling §18 actually gates).

| Pool | Owner | Axis | Why |
|---|---|---|---|
| Rage Power | barbarian | **OPEN** | operator-ruled already, §18 |
| Unchained Rage Power | unchained_barbarian | **OPEN** | same mechanic as Rage Power |
| Discovery | alchemist | **OPEN** | gains a new discovery at multiple levels, keeps all |
| Grand Discovery | alchemist | **EXCLUSIVE** | exactly one, at 20th level, permanent |
| Rogue Talent | rogue | **OPEN** | operator-ruled already, §18 |
| Advanced Talents | rogue | **OPEN** | same repeatable-pick mechanic, unlocked at 10th |
| Hex | witch | **OPEN** | gains a new hex at multiple levels, keeps all |
| Revelation | oracle | **OPEN** | gains a new revelation at multiple levels, keeps all |
| Mercy | paladin | **OPEN** | gains a new mercy at multiple levels, keeps all |
| Investigator Talent | investigator | **OPEN** | repeatable, same shape as Rogue Talent |
| Slayer Talent | slayer | **OPEN** | repeatable, same shape as Rogue Talent |
| Judgment | inquisitor | **OPEN** | gains an additional judgment KNOWN at higher levels (only one active at a time, but the known set grows and old ones are never lost) |
| Inquisition | inquisitor | **EXCLUSIVE** | chosen once at 1st level, permanent |
| Blessing | warpriest | **EXCLUSIVE** | both blessings are chosen once at 1st level (tied to deity), never gains more |
| Evolution | summoner | **OPEN** | eidolon gains new evolutions at multiple levels |
| Bloodline | sorcerer | **EXCLUSIVE** | chosen once at 1st level, permanent |
| Bloodrager Bloodline | bloodrager | **EXCLUSIVE** | same shape as Sorcerer Bloodline |
| Domain | cleric | **EXCLUSIVE** | Ruling §18's own worked example |
| Order | cavalier | **EXCLUSIVE** | chosen once at 1st level, permanent |
| Mystery | oracle | **EXCLUSIVE** | chosen once at 1st level; determines the whole Revelation list |
| Curse | oracle | **EXCLUSIVE** | chosen once at 1st level, permanent |
| Spirit | shaman | **EXCLUSIVE** | chosen once at 1st level; determines hex/spell access |
| Animal Focus | hunter | **OPEN** | additional focuses become known at higher levels, all retained (only the ACTIVE focus is exclusive per use, the KNOWN pool is open) |
| Favored Enemy | ranger | **OPEN** | gains an additional favored enemy at multiple levels |
| Favored Terrain | ranger | **OPEN** | gains an additional favored terrain at multiple levels |
| Versatile Performance | bard | **OPEN** | gains an additional linked performance at multiple levels |
| Arcane School / Focused Arcane School | wizard | **EXCLUSIVE** | chosen once at 1st level, permanent |

**Net: 16 OPEN, 12 EXCLUSIVE** (28 registry entries, 27 table rows above). 2 of the 16 OPEN are
already operator-ruled (§18: Rogue Talent, Rage Power); the other 14 OPEN and all 12 EXCLUSIVE are
newly classified this wave. Every EXCLUSIVE entry (Grand Discovery, Inquisition, Blessing,
Bloodline, Bloodrager Bloodline, Domain, Order, Mystery, Curse, Spirit, Arcane School, Focused
Arcane School) must never gain the browsable reference-catalog pattern per §18. Not code-changed
this wave (no entry here was added to `REGISTERED_POOL_GROUPS` or otherwise wired to a
`done`/`text-complete` credit) -- this is the classification groundwork §18 asks for, filed for the
next pool-widening cycle to consume, not a widening itself.

**Scope note:** this covers the 28 entries (27 distinct group names) already registered in this
probe's own pool table, out of the 817 distinct group names THE-BOX's G1 census found; the other
~790 names still need the same per-pool mechanic read before any of them can be registered
anywhere.
| L9 | **(new, wave 29, CORRECTED by wave 29 integration after adversarial review — do not consume as originally filed) F2 feat-bridge render path is BUILT but reaches ZERO of the class_feature records it was sized against; a classify()-side hook alone would be a manufactured-credit trap.** `apps/desktop/src-tauri/src/class_feature_feat_bridge.rs` (new) + `feat_catalog::feat_description_by_exact_name` (new, reused render path) serve a `class_feature` record's matched feat's own already-verified description through the SAME `ClassFeatureDescriptionDto`/`list_class_feature_descriptions` render path (`CharacterSheet.tsx` fetches and concatenates both). Corpus-wide, narrow (three refusals — second mechanical token, non-feat `ABILITY`, placeholder/compound grant target — all pinned, all mutation-proven RED) this identifies **471** records, not THE-BOX's filed 431 (the census's `type_facet`-"bonusfeat"-substring proxy both missed real matches and over-counted 25 the narrow refusals correctly exclude). **The lane's own `on_screen_evidence` claim was FALSE, and adversarial review measured the real reach directly: `findCorpusDescription` (`classFeaturesModel.ts`) only attaches a description when `d.classSlug === classToken`, and `classToken` is only ever set from the character's HELD classes — of the 471 bridged records' 25 distinct group slugs (`ranger_combat_style_feat` 143, `monk_bonus_feat` 121, `golden_legionnaire` 4, ...), exactly ONE is even a holdable class token (`bard`, 1 record, "Dawnflower Dervish ~ Dervish Dance"), and that one has no engine `ExplanationDto` to attach to either (no `dervish_dance` reference anywhere in `src/`/`src-tauri/src/`) — so the TRUE reachable count is 0 of 471, not "up to 471."** Wiring a classify()-side hook onto this catalog alone, as originally filed here, would bank up to 471 units as `done` for text no character sheet can show — the exact Decision 7 condition 3 / DoD-8 / Decision 1(a) violation this wave's own dispatch warned lane 3 against, and lane 3 correctly declined for the sibling F1 lever (L8, above) on identical grounds. **Also flagged, not yet ruled**: independent of reachability, a `class_feature` record whose sole content is `ABILITY:FEAT|AUTOMATIC|<name>` is a live mechanical grant the engine never applies (it only lends the granted feat's prose) — this may fail Decision 7 conditions 1/2 on its own terms even once reachability is fixed; needs an operator ruling before any credit, see `needs_operator_ruling` in `progress.md`'s wave 29 receipt. **Real work preserved**: the render path itself, its 3 refusal guards, and the 471-record identification are all independently re-verified (515/515 desktop tests, 100/100 frontend, exact-name join collision-checked: 3 of 471 targets name a feat defined in >1 book, 0 disagree in rendered text). What must NOT happen: treating "add the classify() hook" as the only remaining step, or banking any of the 471 before a real reachability path (a new browsable surface, matching L8, or per-character-held-class scoping) exists. Board unchanged this wave: 13,456/38,372, confirmed by guarded regen, `docs/work-inventory.json` reverted after measuring. | 0 of 471 reachable today; up to 471 IF AND ONLY IF a real reachability surface (not just a classify() hook) is built AND the mechanical-grant question above is ruled in its favour | RENDER PATH BUILT, REACHABILITY MISSING, DONENESS HOOK MUST NOT BE ADDED UNTIL REACHABILITY EXISTS |
| L10 | **(new, wave 29, unit figure CORRECTED by wave 29 integration — see below) Book onboarding — no compiled `RuleSetId` at all.** THE-BOX.md §3 item #3: `adventurers_guide`/`inner_sea_magic`/`inner_sea_temples`/`inner_sea_taverns` had no `RuleSetId`, so `v06_work_inventory::classify`'s book-level gate (`engine_book_for` -> `rule_set_for` -> `None`) short-circuited EVERY one of each book's units, all kinds, to `not-started`/`no_compiled_rule_set_for_book` regardless of any per-kind work. **First data point, wave 29: `adventurers_guide` registered via its spell family** (`RuleSetId::AdventurersGuide`, 45 of 49 base `ag_spells.lst` declarations, `src/bin/ingest_adventurers_guide_spells.rs`) — real cost was compile-graph wiring (RuleSetId variant + 2 exhaustive-match arms it forced open in `v06_work_inventory.rs`/`v06_content_state_dump.rs` + `reach_gate.rs` claim + `spell_catalog.rs`/`corpus_ingest_diagnostic.rs` count-pin updates), NOT content — the 45-entry transcription itself took minutes once the `ultimate_wilderness` ingest-binary template was copied. Guarded regen (sweep+fixture reports, `docs/work-inventory.json` reverted after measuring): all 973 `adventurers_guide` units moved off the book-level gate. **CORRECTED by wave 29 integration**: the lane's own filed "+5 `done`" is wrong. Adversarial review found 3 of the 5 credited `class_feature` units (`giant_stalker_defense`, `topple_giant`, `underfoot`) are archetype-gated Rage Power options (`PREABILITY = 1,CATEGORY=Archetype,...`) served wholesale through `class_feature_pool_catalog`, violating Ruling §18 — the same integration cycle fixed the catalog itself (`has_no_prerequisite_token`, `class_feature_pool_catalog.rs`) to refuse any `PRE*`-gated record corpus-wide, which naturally withdraws these 3 (they no longer reach the catalog at all) while leaving the 2 genuinely-unconditional Rogue Talent units (`Careful Stab`, `Hairpin Trick`, no `PRE` token) banked. **Net from this lever: +2 `done`, not +5.** +152 reclassified between not-done verdicts stands unchanged (106 class_feature→unmeasurable, 45 spell→held, 1 class_feature→deferred, **reported separately, not a doneness gain**); the remaining 816 moved `not-started`→`not-ingested` (same doneness bucket, now honestly attributed per-kind instead of blanket-unattempted). | `adventurers_guide` class_feature 699 + spell 49 + feat 81 + equipment 116 + class 25 + race 3 = 973 registered this wave; `inner_sea_magic` (class_feature 218 + share), `inner_sea_temples` (64, whole book, no data/corpus tree at all yet), `inner_sea_taverns` (class_feature 11 + share) — still fully NOT STARTED, ≥1,300 more units behind the same gate shape | PARTIALLY BUILT — 1 of 4 books' rule set registered (spell family only; this book's feat/equipment/class_feature-chassis families and all 3 remaining books are still future work) |

## Levers that turned out not to be

* **Bulk description ingest** (wave 19 thesis) — REFUTED. "not-ingested" means the engine emits no explanation naming the record, not that text is missing. The records already exist with real prose.
* **Generic roster without grant data** (wave 20) — REFUTED, returned GAMED. The emission loop is generic; the data it needs was not.
