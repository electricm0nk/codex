# Shaman / Summoner / Witch — Comparative Re-Scoping of the Last 3 Untouched Classes

> Directed by the lead: don't assume the standing "all three blocked on a
> brand-new Familiar/Eidolon subsystem" verdict still holds — apply the
> same discipline that turned Investigator from "needs a new spell list,
> skip it" into a real buildable MVP, and check directly whether any of
> the three has a smaller real MVP hiding (e.g. a Shaman feature
> independent of its Familiar; one canonical Witch hex cheap enough to
> narrow to the way Oracle's Mystery/Curse and Warpriest's Blessing did).
> Re-verified every claim against the ACG/APG corpus and the real code.
> **Result: a genuine correction — Shaman and Witch are NOT actually
> blocked on the Familiar for a first MVP; each has an Oracle-Mystery-
> style narrowing to one flat, self-scoped, familiar-independent power.
> Only Summoner is genuinely subsystem-blocked.**

## Shared facts verified up front (all three)

- **All three chassis tables are already built**, exactly like
  Investigator's was: `rules_tables::{acg::class_shaman,
  apg::class_summoner, apg::class_witch}` each expose `class_table()` /
  `MAX_SUPPORTED_LEVEL = 20` / `HIT_DIE` (Shaman 8, Summoner 8, Witch 6),
  and `AcgClassId::Shaman` / `ApgClassId::{Summoner,Witch}` are registered
  with working `class_chassis_resolve`. So — as with Investigator — none
  of the three needs new chassis-table work; each needs only the gate
  widening (`is_supported_<class>_single_class` added to
  `has_supported_class_chassis`) plus feature grounding. Confirmed no
  `is_supported_shaman/summoner/witch_single_class` gate exists yet.
- **No Familiar stat-block exists anywhere in the codebase.** (The
  companion picture is more nuanced than a blanket "no pet code exists":
  a real, working Animal Companion Wolf stat block *is* built and reused —
  `ground_wolf_companion_stat_block` (`pilot_compute.rs:4835`) is grounded
  by both Druid's Nature Bond and Hunter's own closure via
  `ground_hunter_animal_companion_and_defer_the_rest`
  (`pilot_compute.rs:11574`, which calls it directly); what Hunter defers
  is companion *advancement past level 1*, not the stat block itself.) But
  that does **not** help Shaman or Witch: Shaman's Spirit Animal is a
  **Familiar**, not an Animal Companion — a mechanically distinct PF1
  subsystem (different progression table, different special-ability
  scaling), so it genuinely cannot reuse the Wolf companion code
  regardless of that code existing. The **Familiar subsystem itself is
  unbuilt**, and that — not any absence of companion code — is the correct
  reason Shaman's Spirit Animal and Witch's Familiar stay deferred. The
  MVPs below matter precisely because they **avoid the pet entirely**, the
  same way Oracle deferred its subsystem-heavy Mystery bonus-spell list.
- **None of the three reuses an existing spell list.** Shaman
  (`SPELLSTAT:WIS MEMORIZE:YES`, 9th-level prepared divine), Summoner
  (`SPELLSTAT:CHA MEMORIZE:NO`, spontaneous), and Witch (`SPELLSTAT:INT`)
  all carry **no `SPELLLIST:` reuse token** — each has its own fresh list.
  So spellcasting is a fresh-ingestion cost for all three (worse than
  Investigator, which at least had a reuse *target*, unbuilt though it
  was). Every MVP below therefore **defers spellcasting** and stays
  `Blocked`, like 8 of the 9 prior closures.

## Shaman (ACG) — real MVP, NOT blocked on the Familiar

- **Chassis** (`acg_classes.lst:221/225`, verified): HD 8, 3/4 BAB, good
  Will, poor Fort/Reflex, `SPELLSTAT:WIS MEMORIZE:YES` (prepared, 9th-level
  divine, `ROLE:Cleric`), `BONUS:DOMAIN|NUMBER|1|...TYPE.ShamanSpirit`.
- **10 real `KEY:Shaman ~ ...` records**: Class Skills, Weapon/Armor
  Proficiency, Manifestation, Orisons, Spirit, Spirit Animal, Spirit
  Magic (matching `named_features_expected` = 10).
- **Spirit** is domain-like — 10 primary spirits (Battle, Bones, Flame,
  Heavens, Life, Lore, Nature, Stone, Waves, Wind), the **same 10 as
  Oracle's Mysteries**, verified via `KEY:Shaman Spirit ~ <Name>`. The
  identical narrowing Oracle used applies: pick ONE canonical spirit,
  ground its flat power, defer the other nine.
- **CORRECTION (backend, pre-build verification, 2026-07-26): the
  original MVP pick, `Life Spirit ~ Healer's Touch`, is level-8-gated,
  not immediately available.** Its own `ABILITY` grant line carries
  `PREVARGTEQ:ShamanSpiritGreater,1`, and `ShamanSpiritGreater` only sets
  via `PREMULT:2,[PRECLASS:1,Shaman=8],...` — genuinely a level 8+
  feature, unlike Oracle's own Healing Hands which has no such gate. This
  doc's original "exact structural twin of Oracle's Life Mystery" framing
  overstated it; the +4 Heal formula itself was correctly transcribed,
  but the DESC-text citation didn't check the surrounding `ABILITY` grant
  condition. **Real fix, verified directly**: Life Spirit's OTHER
  immediately-granted power (no `PREVARGTEQ` at all on its own primary
  `KEY:Shaman Spirit ~ Life` record's grant line) is `Life Spirit ~
  Channel` — a real Channel Positive Energy variant, structurally
  identical to Cleric's own already-built Channel Energy: uses/day
  `1+CHA`, dice `(ShamanSpiritLVL+1)/2` (d6), DC
  `10+(ShamanSpiritLVL/2)+CHA`, all confirmed directly against the corpus.
  Independent of the Spirit Animal (Familiar) either way.
- **Spirit Animal** confirmed `TYPE:...Familiar` (not an Animal
  Companion) — stays deferred exactly as Oracle's subsystem parts did.
  Spirit Magic (spirit-granted bonus spells), Orisons, Manifestation
  (capstone), Healer's Touch (now correctly named as level-8-gated, not
  merely deferred-for-scope), and Hexes/Spirit Hexes (a large
  chooser-list, `KEY:Shaman Spirit Hex ~ ...`) all stay deferred.
- **Verdict**: a real Oracle-shaped MVP — chassis gate + Life Spirit's
  Channel (flat uses-per-day/dice/DC), defer the Familiar/spellcasting/
  other 9 spirits/hexes/Healer's Touch.
  Stays `Blocked`. Not blocked on the Familiar at all.

## Witch (APG) — real MVP, NOT blocked on the Familiar

- **Chassis**: `SPELLSTAT:INT`, HD 6, own fresh list. **7 real
  `KEY:Witch ~ ...` records** (smallest on the roster): Cantrips, Class
  Skills, Familiar, Familiar Touch Spells, Hex, Patron Spells, Weapon
  Proficiencies.
- **Hex is the marquee chooser-list** — 19 base hexes verified
  (`KEY:Witch Hex ~ <Name>`: Blight, Cackle, Cauldron, Charm, Coven,
  Disguise, Evil Eye, Flight, Fortune, Healing, Misfortune, Slumber,
  Tongues, Ward, …), plus separate Major Hex / Grand Hex tiers.
- **The flat, self-scoped, familiar-independent canonical hex exists:
  Ward.** Real record: "the warded creature receives a +%1 deflection
  bonus to AC and a +%2 resistance bonus on saving throws" with
  `BONUS:VAR|WitchWardBonus|2` (base, +1 at level 8, +1 at level 16 →
  flat +2 scaling to +4). Self-scoped (the witch may ward herself), a
  flat numeric value — the closest hex to Oracle's flat self-scoped
  narrowing. (Most other hexes — Evil Eye, Misfortune, Slumber, Cackle,
  Fortune — are opponent/ally-targeted with save-DC interactions, the
  same "opponent-dependent" wall Slayer's Quarry hit; Healing is a
  cure-spell effect needing spell resolution. Ward is the clean one.)
- **The Familiar is central to Witch *spellcasting*** (she prepares from
  the familiar), so deferring spellcasting AND the Familiar together is
  the honest posture — but the Ward hex itself does **not** require the
  familiar, so it grounds independently. Cantrips/Patron Spells (fresh
  spellcasting), Familiar/Familiar Touch Spells (the unbuilt Familiar
  subsystem), and the other 18 hexes stay deferred.
- **Verdict**: the cleanest of the three — chassis gate + Ward hex's flat
  +2/+2 (self-scoped), defer the Familiar/spellcasting/other hexes. Stays
  `Blocked`. Not blocked on the Familiar for a first MVP.

## Summoner (APG) — genuinely subsystem-blocked, no cheap MVP

- **Chassis**: `SPELLSTAT:CHA MEMORIZE:NO` (spontaneous), own short fresh
  list, HD 8. **17 real `KEY:Summoner ~ ...` records**: Eidolon, Aspect,
  Greater Aspect, Bond Senses, Life Link, Life Bond, Shield Ally, Greater
  Shield Ally, Maker's Call, Transposition, Merge Forms, Twin Eidolon,
  Gate, Summon Monster, Cantrips, Weapon/Armor Proficiency, Standard Class.
- **Nearly every feature of value is Eidolon-dependent**: Life Link /
  Life Bond (share HP with eidolon), Bond Senses (share its senses),
  Shield Ally / Greater Shield Ally (bonus *while near the eidolon*),
  Aspect / Greater Aspect (spend the *eidolon's* evolution pool on the
  self), Maker's Call / Transposition / Merge Forms / Twin Eidolon / Gate
  — all require the unbuilt Eidolon (a whole second stat block with an
  evolution point-buy system).
- **The only eidolon-independent fact is thin**: Summon Monster is a
  summon-SLA with `BONUS:VAR|SummonMonsterTimes|CHA+3` uses/day (and
  `SummonMonsterLVL|min(9,(SummonerLVL+1)/2)`) — a flat Panache-shaped
  pool, but its *effect* (summoning a creature) needs an unbuilt summon
  subsystem, so only the pool-size fact could ground. That plus chassis/
  saves is a thinner MVP than any closure to date (no self-scoped
  always-on power like Shaman's +4 Heal or Witch's Ward).
- **Verdict**: genuinely blocked. A first closure could only ground
  chassis + Summon Monster uses/day — below the value bar the prior
  closures cleared. Summoner is the class where the lead's "build the
  Eidolon subsystem deliberately, or declare full-build closures done"
  question actually applies.

## Recommendation

Two real, honest MVPs are available that the standing verdict missed —
**Witch (cleanest: Ward hex, flat +2/+2 self-scoped) and Shaman (Life
Spirit's Channel — corrected from the original Healer's Touch pick,
which turned out to be level-8-gated; Channel is Cleric-Channel-Energy-
shaped and immediately available)** — both landing WITHOUT any
Familiar-subsystem work, by narrowing to one canonical power exactly as
Oracle/Warpriest did. **Summoner is the genuine outlier**: no cheap
familiar/eidolon-independent power, only a thin Summon-Monster pool.

Caveat, stated honestly: both Witch and Shaman MVPs are **thin and stay
`Blocked`** (one flat power each; `named_features_wired` ~1, maybe 2 for
Shaman). Neither reaches `Computed` without its fresh own-list
spellcasting slice — a bigger data cost than Investigator's, since neither
reuses an existing list. So the real question for the lead is a
value-judgment, not a feasibility one: two more "chassis + one flat
power, stays Blocked" closures are buildable and honest, but whether
they clear the worth-it bar this late in the epic — versus declaring the
cheap-narrowing closures done and treating Familiar/Eidolon as a single
deliberate subsystem investment — is your call.

## Open questions for the lead

1. Build **Witch** next (Ward hex MVP, cleanest of the three), and/or
   **Shaman** (Life Spirit's Channel MVP — corrected from Healer's Touch,
   which turned out to be level-8-gated, not immediately available; see
   the correction above)? Both are real and honest but thin and stay
   `Blocked`; confirm they clear the value bar, or that you'd rather stop
   cheap-narrowing closures here.
2. For **Summoner**, confirm the read: no cheap MVP, only a thin
   Summon-Monster-pool closure or a deliberate Eidolon subsystem — is it
   worth a subsystem investment, or does Summoner stay untouched?
3. If Witch/Shaman are greenlit: is the canonical pick right (**Witch →
   Ward**, **Shaman → Life Spirit / Channel**), mirroring Oracle's
   Life-Mystery precedent, or is a different spirit/hex preferred?
4. Bigger-picture: is there appetite to fund **one** Familiar-or-Eidolon
   subsystem as a large deliberate slice (unblocking Shaman's Spirit
   Animal, Witch's Familiar, and Summoner's Eidolon together over time),
   or are we effectively done with new full-build closures for this epic
   after the cheap narrowings are exhausted?
