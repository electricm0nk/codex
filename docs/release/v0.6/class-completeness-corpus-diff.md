# Class completeness corpus-diff (task #84)

Follow-on to #76/#79. Those audits asked **"is this diagnostic's text true?"**
This one asks the different question the Slayer trap exposed: **"does this class
have unbuilt features its diagnostic never mentions?"** A message going quiet
about a feature is not evidence the feature is done.

Docs-only. No code changed.

## Method

For each class: enumerate `KEY:<Class> ~ …` base records from the ingested
corpus, diff against the set of features the class's own diagnostics name, then
check grounding status in `pilot_compute.rs` for anything unmentioned.

**Plumbing records excluded, named rather than silently filtered:** `Class
Skills`, `Weapon and Armor Proficiency`, `Standard Class`, `IUS Yes`/`IUS No`,
`Output`, `Standard Bloodline Selection`. These are PCGen encoding artifacts, not
player-facing features.

### The trap inside this check

Grepping the crate for a feature name is **not** sufficient to call it grounded.
Hunter's `Swift Tracker`, `Woodland Stride` and `Master Hunter` all return hits —
and every hit belongs to **Ranger or Druid**:

```
class_feature.ranger.swift_tracker
class_feature.ranger.woodland_stride   class_feature.druid.woodland_stride
class_feature.ranger.master_hunter
```

Hunter's own versions are ungrounded. A name-match check would have scored three
of Hunter's gaps as covered. Every "grounded" call below was resolved to its
owning namespace.

## Results

| Class | Base records | Unmentioned **and** ungrounded | Verdict |
|---|---|---|---|
| Sorcerer | 7 | 0 | **Clean** |
| Barbarian | 13 | 1 | 1 gap |
| Bard | 9 | 1 | 1 gap |
| Warpriest | 18 | 2 | 2 gaps |
| Skald | 20 | 3 | 3 gaps |
| Alchemist | 25 | 6 | 6 gaps |
| Hunter | 21 | ~14 | **Worst — diagnostic covers a third of the class** |

Six of seven classes have at least one genuinely-unbuilt feature that appears
nowhere in their diagnostics. The trap is the norm, not the exception — but it
is not uniform, so it has to be run per class rather than assumed either way
(Sorcerer really is clean).

---

### Sorcerer — clean

All 7 records account for: Cantrips and Spells via the spell baseline, Eschew
Materials, Standard Bloodline and the Arcane bloodline ladder, plus the two
plumbing records. Nothing unmentioned and ungrounded. Consistent with #79's
finding that Sorcerer's diagnostic is the best-maintained one in the crate.

### Barbarian — 1 gap: Mighty Rage

`Greater Rage` **is** grounded, via the rage-tier mechanism (test
`single_class_barbarian_at_greater_rage_level_applies_the_higher_tier`).
`Mighty Rage` has **zero** references anywhere.

This one is live rather than theoretical: `MAX_SUPPORTED_BARBARIAN_LEVEL = 20`,
and Mighty Rage is the level-20 feature, so it is **reachable on a Computed
class**. Not bounded away by a level cap.

### Bard — 1 gap: Armored Casting

`Armored Casting` (arcane spell failure suppression in light armor) is
ungrounded and unmentioned.

**`Bardic Countersong` is *not* a gap, and the reason is worth copying.** It is
unmentioned by name, but Bard ships
`class_feature.bard.bardic_performance_execution.other_performances_not_modeled`
— an explicit catch-all record acknowledging the unmodelled performance types.
That is the honest way to handle a family you have not enumerated, and it is
exactly what Hunter, Alchemist and Skald lack.

### Warpriest — 2 gaps

`Bonus Feats` and `Orisons` are both ungrounded and unmentioned (0 references).
Note Inquisitor's diagnostic *does* name Orisons; Warpriest's does not, despite
both classes having the record.

### Skald — 3 gaps

`Uncanny Dodge`, `Improved Uncanny Dodge`, and `Master Skald` (capstone) are
ungrounded and unmentioned. Worth flagging because **Bloodrager's identical
Uncanny Dodge / Improved Uncanny Dodge pair was grounded in task #42** — so
there is an accepted in-repo precedent making Skald's cheap, the same shape as
the Bloodrager Indomitable Will finding.

### Alchemist — 6 gaps

Its diagnostic names only Discovery, Swift Alchemy and Swift Poisoning as
remaining. Also unmentioned and ungrounded: **Alchemy, Brew Potion, Instant
Alchemy, Persistent Mutagen, Poison Use, Grand Discovery.**

`Alchemy` is notable — Investigator ships
`investigator_alchemy_creation_bonus` for its own version, so this is a
generalization candidate rather than fresh work.

### Hunter — the worst case

Its diagnostic names Animal Companion, Wild Empathy and the Bull Animal Focus as
grounded, and the class-skill list, spellcasting, Nature Training, Precise
Companion and the 12 other Animal Focus options as remaining. **That accounts for
roughly a third of the class.**

Unmentioned, and verified ungrounded for Hunter specifically:

`Bonus Tricks`, `Hunter Tactics`, `Greater Empathic Link`, `Improved Empathic
Link`, `Speak With Master`, `Raise Animal Companion`, `Master Hunter`, `Swift
Tracker`, `Woodland Stride`, `Track`, `One With The Wild`, `Second Animal
Focus`, `Teamwork Feats`, `Orisons`.

Three of those (`Master Hunter`, `Swift Tracker`, `Woodland Stride`) are the
Ranger/Druid collisions described above — they look grounded and are not.

**This directly affects the #76 punch-list ranking.** I ranked Hunter as the
single closest class to a retirable catch-all, on the basis that everything its
diagnostic named was either in flight or cheap. That conclusion was drawn from
the diagnostic's own list, which turns out to omit ~14 features. **Hunter's
ranking should not be relied on until this is re-derived.**

## What this means for the audits already delivered

- #76 and #79's verdicts are unaffected — they answered whether the text is
  true, and they still do.
- **What is affected is any "closest to done" ranking built on those lists.**
  Hunter (#76's number 1) and Bloodrager (already corrected for Indomitable
  Will) both had their remaining-work estimates drawn from diagnostics that
  omit real features.
- The constructive fix is Bard's: where a class has an unenumerated family, ship
  an explicit *"others not modelled"* record rather than silence. Silence is
  what makes the omission invisible to every downstream reader, including
  future audits.

## Limits

- Covers the seven classes assigned. Not run on the other CRB classes
  (Cleric, Druid, Fighter, Paladin, Ranger, Rogue, Wizard, Monk) or the
  remaining #76 classes.
- "Ungrounded" means no reference resolving to that class's own namespace in
  `pilot_compute.rs`. A feature grounded in another module would not be seen,
  though no such case turned up in spot-checks.
- Corpus scope is the four ingested books only.
- I did not assess whether each gap is *worth* grounding — several
  (Brew Potion, Teamwork Feats) are likely feat-grant plumbing rather than
  magnitudes. This is an inventory of what is unaccounted for, not a build list.
