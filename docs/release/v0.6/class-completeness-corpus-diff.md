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
| Barbarian | 13 | 0 | **Clean** (corrected — see below) |
| Bard | 9 | 1 | 1 gap |
| Warpriest | 18 | 1 | 1 gap (corrected — Orisons withdrawn) |
| Skald | 20 | 3 | 3 gaps |
| Alchemist | 25 | 6 | 6 gaps |
| Hunter | 21 | ~14 | **Worst — diagnostic covers a third of the class** |

Five of seven classes have at least one genuinely-unbuilt feature that appears
nowhere in their diagnostics. The trap is common but not universal, so it has to
be run per class rather than assumed either way.

### Correction: absence-of-grep is a candidate, not a finding

The first version of this table listed Barbarian's **Mighty Rage** as a gap on
the strength of a zero-hit `grep`. **That was a false positive.** Mighty Rage is
grounded — `barbarian_rage_tier` returns `(8, 8, 4, "Mighty Rage")` above
`BARBARIAN_MIGHTY_RAGE_LEVEL`. The corpus record name does not map onto an id
name, so a snake_case search finds nothing while the feature is fully built.
Barbarian is clean.

This is the same false-positive shape featmate hit on Bloodrager's
Greater/Tireless/Mighty Bloodrage tiers, and it cuts the opposite way from the
#76/#79 lesson. Both cautions apply at once:

- **Don't trust a diagnostic's claim** that something is unbuilt (#76/#79).
- **Don't trust a raw grep-for-absence** that something is unbuilt (this).

### The three-way check every "gap" below now satisfies

1. **snake_case search** (`mighty_rage`) — catches id/function names.
2. **Literal-name search, case-insensitive** (`Mighty Rage`) — catches string
   literals and tier labels, which is what the snake_case pass misses.
3. **Owning-namespace resolution** — a hit only counts if it resolves to *that
   class's* ids or functions. Skald's "Uncanny Dodge" returns 105 literal hits,
   all of them Barbarian, Rogue or Bloodrager; Skald's own id and function sets
   contain no Uncanny Dodge at all, so the gap is real.

All remaining gaps below were re-verified against the owning class's full id and
function set, not a name search. Two cases were checked specifically because
they had a tier-mechanism shape that could have hidden a grounding, and both
held: Alchemist's **Persistent Mutagen** (level 14, doubles duration) is a real
gap because `alchemist_mutagen_duration_minutes` is a flat `level * 10` with no
level-14 branch; and Hunter's Master Hunter / Swift Tracker / Woodland Stride
resolve entirely to `class_feature.ranger.*` and `class_feature.druid.*`.

### Method postmortem: this doc needed three correction passes

Worth recording plainly, because the pattern is the argument for changing the
method rather than trying harder:

1. **Mighty Rage** — a real grounding missed because the corpus name lives in
   code as a capitalised string literal, not a snake_case id.
2. **Orisons** — a real grounding missed because *no id contains the word at
   all*; it is the level-0 entry of a format-constructed spells-per-day table.
3. Both were false *gaps*; the Hunter/Ranger namespace collisions were the
   mirror-image risk of false *coverage*.

All three trace to the same root: this audit started from
**"grep the feature name and see what comes back."** The structurally safer
order, which #79 used and this pass did not, is **"enumerate the ids in the
class's own namespace, then join against the corpus record list."** That removes
false coverage entirely and reduces false gaps to one residual case — a feature
whose id is table-constructed or shared, which you close by reading the
constructing function rather than by grepping harder.

Adopting that order for any future completeness pass. The remaining gaps in this
doc were re-verified against both the shared-id set (30 ids with no class name
in them) and the format-constructed id set (63), and none of them is covered.

### On enumerating from the file rather than memory

This audit's corpus side was derived by parsing `KEY:<Class> ~ …` out of the
`.lst` files directly, never from recall of what a class "should" have. That
check passes as run — recording it because the failure mode (a name that passes
a sniff test and fails the file check) is invisible in the output if you don't
state the source.

---

### Sorcerer — clean

All 7 records account for: Cantrips and Spells via the spell baseline, Eschew
Materials, Standard Bloodline and the Arcane bloodline ladder, plus the two
plumbing records. Nothing unmentioned and ungrounded. Consistent with #79's
finding that Sorcerer's diagnostic is the best-maintained one in the crate.

### Barbarian — clean (corrected)

Originally reported as a Mighty Rage gap. **Withdrawn.** Both `Greater Rage` and
`Mighty Rage` are grounded through the same tier mechanism —
`barbarian_rage_tier` returns `(6, 6, 3, "Greater Rage")` and
`(8, 8, 4, "Mighty Rage")` at their respective level gates. The zero-hit
`grep` for `mighty_rage` reflected the naming convention, not the state of the
code.

### Bard — 1 gap: Armored Casting

`Armored Casting` (arcane spell failure suppression in light armor) is
ungrounded and unmentioned.

**`Bardic Countersong` is *not* a gap, and the reason is worth copying.** It is
unmentioned by name, but Bard ships
`class_feature.bard.bardic_performance_execution.other_performances_not_modeled`
— an explicit catch-all record acknowledging the unmodelled performance types.
That is the honest way to handle a family you have not enumerated, and it is
exactly what Hunter, Alchemist and Skald lack.

### Warpriest — 1 gap (corrected from 2)

`Bonus Feats` is ungrounded and unmentioned.

**`Orisons` is withdrawn as a gap.** It is grounded, as the level-0 entry of the
spells-per-day table: `warpriest_base_spells_per_day` returns `Some(3)` at
index 0 for level 1 and `Some(4)` for levels 2-3, emitted as
`class_spell.acg.warpriest.base_spells_per_day.spell_level_0` and asserted by an
existing test. **No id anywhere contains the string "orisons"** — this is the
residual failure mode of a name-based search, and the reason the id-set-first
method is structurally safer.

Honest qualification: what is grounded is the orison *slot count*. The
never-expended semantics of the Orisons class feature are not modelled, so this
is the established "pool size grounded, execution not" idiom rather than a
complete feature. It is not, however, an unaccounted-for gap.

Hunter's own `Orisons` record stays a genuine gap for now — Hunter has no
spells-per-day table at all until task #44 lands, so nothing grounds its
level-0 count.

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
