# Cycle — SD-34 wave 38, Lane B — Sub-mechanism 5's top-2 classes (`divine_scion` 45, `phrenic_slayer` 43) scope-checked, NOT genuinely bounded for a single chassis cycle; 0 units closed, honestly reported

- **Commit SHA:** (this receipt's own commit, see report)
- **Files touched:** this receipt, `progress.md`, `kanban.md`,
  `docs/retro/events/sd34-wave38-laneb.jsonl` (new),
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
  (`derived_at` pointer refresh only, from running `completion_atlas.py --check`
  at this cycle's own HEAD — no bucket data changed). **No `src/`, `scripts/`,
  or `data/corpus/**` file touched.**
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — no code diff this cycle,
  N/A by construction.
- **Wired-integration audit result:** `OK_NO_TOKENS` — no code diff this
  cycle, N/A by construction.
- **Acceptance criterion (verbatim from this cycle's dispatch brief):** "Begin
  sub-mechanism 5['s] own corrected 634-unit magnitude-bearing remainder
  (`class_feature_of_unmodelled_corpus_class` shape, real new chassis needed
  per class) — largest classes first, per its own dispatch table. ... Pick
  the single largest class group from that table (re-derive the current top
  group fresh ...). ... build a real, tested chassis for it if the scope is
  genuinely bounded (BAB/save progression + the specific feature magnitude
  this shape needs). If the true scope turns out much larger than expected
  once you look closely, stop and report the real scope honestly rather than
  rushing an incomplete chassis."

## Step 0 — worktree self-heal

Assigned worktree started at `ea2b3396f2` (the SD-33 PR #377 merge commit),
far behind local `tranche/14`'s real tip (`fb149ce2b1`, wave 37's own
wave-end gate). Confirmed clean fast-forward (`git merge-base --is-ancestor
HEAD tranche/14` → true, working tree clean, no local commits to lose) and
rebased. Re-ran `completion_atlas.py --check` post-rebase before any analysis
— `population=49438 buckets=10 unclassified=0 overlap=0`, `D: 2661`,
`DONE: 25244` — matches wave 37's own final reported state exactly.

## Step 1 — re-derive sub-mechanism 5 fresh, confirm the top group unchanged

Re-ran wave 37 lane B's own exact `Counter` method against this cycle's HEAD:

```python
collections.Counter(u["evidence"].split(":", 1)[1] for u in units
                     if (u.get("evidence") or "").startswith(
                         "class_feature_of_unmodelled_corpus_class:"))
```

`700` units / `68` classes total (down 1 from wave 37's `701`/`68` — an
unrelated single-unit shift elsewhere in the corpus-wide evidence bucket,
outside the excluded-class set). Sub-mechanism 5 (excluding the same 10
already-mined classes): **`634` units / `60` classes — identical to wave
37's corrected figure**, same class roster (set-difference confirmed
empty). Top group unchanged: `divine_scion` (45, `inner_sea_magic` —
correcting wave 37's own receipt, which named the book `adventurers_guide`;
the actual `book` field on all 45 units is `inner_sea_magic`, confirmed
directly). `phrenic_slayer` (43, `ultimate_psionics`) is second.

## Step 2 — `divine_scion` (45 units): investigated, NOT bounded

Read `data/corpus/inner_sea_magic/class/divine_scion.json` (the class
chassis record: `HD:8`, `TYPE:PC.Prestige`, `MAXLEVEL:10`, 3/4 BAB, good
Will only — a completely standard formula shape, `classlevel("APPLIEDAS=
NONEPIC")`-based, identical in kind to the 61 conventional classes
`generic_class_chassis.rs` and the 7 CRB NPC/`Ex-*` classes
`crb_untabled_class_chassis.rs` already handle via
`PcgenFormulaEvaluator`) and all 45 `data/corpus/inner_sea_magic/
class_feature/divine_scion/*.json` records directly.

**The BAB/save half is cheap and reusable** — the exact same evaluator
pattern already proven on 61+7 other classes would resolve it in a few
lines. **The feature-magnitude half is not one formula, it is a whole
subsystem**: `magnitude_token_count` is `2` for 39 of the 45 units and `1`
for the other 6 — **every single one is magnitude-bearing, none are
zero-magnitude**, so none of them can ride the "text-only = complete"
promotion path that closed prior D-bucket waves' cheap wins. Reading the
actual content:

- **34 of the 45** are `Domain Specialization (<Domain>)` — one per
  Pathfinder cleric domain — and each one grants a **different** domain-specific
  spell-like ability via its own `SPELLS:` token (`Air` → `Fly 1/day`;
  confirmed by reading `domain_specialization_air.json` directly: `SPELLS =
  Innate|TIMES=1|CASTERLEVEL=TL|Fly,13+max(WIS,CHA)`). This is not "the
  specific feature magnitude this shape needs" (singular) — it is 34
  independent spell-like-ability grants, each its own small mechanism, on
  top of the base `Domain Specialization` ability-pool-selection record
  itself (`BONUS:ABILITYPOOL|Domain Specialization|1`) and its healing-scaling
  formula (`BONUS:VAR|DomainSpecBonus|2`).
- The remaining 11: `Deific Defense` (DR formula), `Divine Wrath` (spell
  damage formula), `Weapon and Armor Proficiency` (an explicit no-op —
  "gains no additional... proficiencies"), 4× `Opposition Alignment`
  (spell-resistance-check bonus, `DR:` token cross-references Deific
  Defense), 2× `True Scion` (permanent stat increase + scaling of THREE
  other features' own magnitudes at once — `BONUS:STAT|WIS|1` plus
  `VAR|DomainSpecBonus|2`, `VAR|DivineWrathBonus|1`,
  `VAR|DeificDefenseBonus|3`).

**Verdict: not genuinely bounded.** A real chassis here means building a
34-variant domain-specific spell-like-ability system (each domain its own
small piece: Air→Fly, and presumably 33 more, each different) plus five
more distinct mechanical shapes (a no-op record, a DR formula, a spell-damage
formula, an SR-check bonus, a two-tier True-Scion scaling layer) — genuine
Epic 4/5-scope work, not a bounded single-cycle build. Building only the
BAB/save half and registering the class would close **zero** of the 45
units (see Step 4) while adding real, untested surface area for no
measured payoff — declined.

## Step 3 — `phrenic_slayer` (43 units): investigated, its cheapest subset independently blocked

Read `data/corpus/ultimate_psionics/class/phrenic_slayer.json` (full BAB,
good Will, `MAXLEVEL:10`, same cheap formula shape) and all 43 magnitude-
bearing/zero-magnitude records under `class_feature/phrenic_slayer/` and
`class_feature/phrenic_slayer_favored_enemy/`.

**Magnitude distribution, unlike `divine_scion`, is genuinely mixed**: `31`
of 43 are `magnitude_token_count: 0` (the `Favored Enemy` creature-type
sub-choices — "Aberration", "Animal", ... "Vermin" — the exact zero-magnitude,
`" ~ "`-qualified pool-member SHAPE that closed "Order of the Dragon" in
wave 36 lane C). The remaining 12 carry real magnitude: the base `Favored
Enemy` formula record itself (`mag=2`), `Brain Nausea`/`Power Resistance`/
`Rebound Attack`/`Lucid Buffer` (`mag=2` each, real supernatural-ability
formulas), and 5 `Advance <X>` psionic-enhancement-choice records (`mag=1`
each, an `ABILITYPOOL` selection).

**The cheap 31-unit zero-magnitude subset was checked against the existing
promotion mechanism and independently blocked, correctly, by an
already-shipped gate — not by anything this cycle would need to build.**
`class_feature_pool_catalog.rs`'s `is_registered_pool_group` is already
universal (`key.contains(" ~ ")`, `REGISTERED_POOL_GROUPS` is a vestigial
doc-only constant no longer consulted by the filter — confirmed by reading
the function and its own doc comment), so these 31 records are already
candidates for `load_pool_catalog`'s render-and-refuse gate; a class-chassis
build was never the blocker for this specific subset. Direct read of all 31
corpus records (`data/corpus/ultimate_psionics/class_feature/
phrenic_slayer_favored_enemy/*.json`) found **every single one carries
exactly 2 `DESC:` rows**, one gated `!PRERULE:1,DisplayFullAbility` (short
form) and one gated `PRERULE:1,DisplayFullAbility` (long form — a house-rule
toggle PCGen's own source data branches on). `class_feature_pool_catalog.rs`'s
`raw_tokens_carry_more_than_one_desc_segment` gate — and its one documented
exception, `shipped_description_is_the_already_regenerated_safe_multi_desc_
join`, which explicitly excludes any segment carrying a `PREVAREQ`/
`PREVARGTEQ` gate but does **not** except a `PRERULE` gate — correctly
refuses to serve any of these 31: this engine tracks no per-character
`DisplayFullAbility` house-rule setting, so picking either DESC row would be
a genuine content fabrication, exactly the defect this gate exists to catch.
**This is not a missing chassis — it is a different, already-solved-correctly
problem** (a house-rule-toggle mechanism this engine does not have, shared
with every other `PRERULE`-branched record corpus-wide, not specific to this
class).

**The 12 magnitude-bearing records are blocked by the same caster/
manifester-level-stacking gap this codebase already names and defers
elsewhere.** Every one of them keys its formula off `PhrenicSlayerLVL`
(cheap — a straight `classlevel()`-style class-level counter, buildable
alongside the BAB/save chassis) **and/or** `PhrenicSlayerPrimeStat` (the
prestige class's own "prime manifesting stat" — a psionic multiclass-derived
fact this codebase does not compute) **and/or** an `ABILITYPOOL|Manifesting
Level Advancement|...` grant (a manifester-level-stacking mechanism).
`src/rules_core/pilot_compute/crb_untabled_class_chassis.rs`'s own doc
comment states this exact blocker in general terms: *"a full base-attack-
bonus/save chassis for six of the ten [CRB prestige classes] is deferred
pending a caster-level-stacking mechanism this codebase does not have yet"*
— `phrenic_slayer`'s prime-stat/manifester-stacking dependency is the same
shape, for a psionic class outside CRB.

**Verdict: not genuinely bounded either**, for a different reason than
`divine_scion` — not "too many small mechanisms" but "blocked on a real,
already-named, cross-cutting infrastructure gap this codebase defers on
purpose."

## Step 4 — one honesty check: would registering either class close ANYTHING?

Before concluding, checked whether simply widening `modelled_class_books()`
to include `divine_scion`/`phrenic_slayer` (even without full feature
wiring) would move any of the 88 units, since that alone is cheap. Read
`v06_work_inventory.rs`'s `Kind::ClassFeature` owner-resolved branch
(≈line 12736 onward): once an owner resolves, a magnitude-bearing record
(`text_only == false`) needs `grounded`/`grounded_strict` (a real
explanation-id match against something the engine actually computed) or
else falls to `engine_does_not_hold("class_feature_no_dedicated_magnitude_
id_matched_the_record_slug")` — **still D, evidence renamed, zero bucket
movement**. This is the exact same outcome wave 37 lane B already proved
for the 151-sibling `ranger_combat_style_feat` population (owner correctly
resolves, engine still doesn't hold any of them, 0 DONE). Registering
either class without building the underlying magnitude mechanisms would
therefore add real chassis code for a **confirmed zero-unit payoff** —
declined, per this cycle's own dispatch brief ("stop and report the real
scope honestly rather than rushing an incomplete chassis").

## A wider finding: sub-mechanism 5 is not one shape

Sampling `TYPE`/`MAXLEVEL` on 8 more classes from wave 37's own table
(`thrallherd`, `psychic_detective`, `cyphermage`, `twilight_talon`,
`golden_legionnaire`, `phantom`, `psychic_fist`, `asavir`) found the
**majority are genuine `TYPE:PC.Prestige`/`PC.Prestige.Psionic` classes**
(`thrallherd`, `cyphermage`, `psychic_fist`, `asavir` — the same
prestige-chassis-plus-caster-stacking shape as the two investigated above),
but **two carry no `data/corpus/*/class/<slug>.json` record at all**
(`twilight_talon`, `golden_legionnaire` — likely a name-collision
misattribution, the same shape wave 36 lane C already fixed for "Order of
the Dragon", not a genuine missing-chassis case), and one (`phantom`) is
filed `TYPE:Monster`, not a PC class at all — plausibly a bestiary-name
collision too. **Sub-mechanism 5's population is a mix of at least three
different disposition shapes, not the single "real new chassis needed per
class" framing wave 37's own receipt used.** Retro-logged as a correction
(below); not exhaustively re-classified this cycle (would require checking
all 60 classes individually — named as the real next-cycle work, not done
here).

## Disposition: 0 units closed, honestly reported

No `src/`, `scripts/`, or `data/corpus/**` file touched. No chassis built.
Consistent with this cycle's own dispatch brief's explicit escape hatch: a
well-scoped partial closure is better than a fabricated one, and a
zero-payoff chassis (Step 4) is worse than no chassis at all.

## Figures (every number, its command, its denominator)

- `population=49438 buckets=10 unclassified=0 overlap=0`, `D: 2661`,
  `DONE: 25244` — `python3 scripts/completion_atlas.py --check`, this
  cycle's own HEAD (post-rebase, before and after this cycle — no
  bucket-moving change made).
- `700` units / `68` classes total under
  `class_feature_of_unmodelled_corpus_class:*`, `634` units / `60` classes
  for sub-mechanism 5 (excluding the same 10-class set wave 35 established) —
  the `Counter` command quoted in Step 1, of `docs/work-inventory.json`'s
  `49438`-unit population, this cycle's own committed HEAD.
- `45` units for `divine_scion` (`inner_sea_magic`, correcting the book
  field from wave 37's `adventurers_guide`), magnitude distribution `{2: 39,
  1: 6}`, `0` at magnitude `0` — direct field read/`Counter` over the 45
  matching units, `docs/work-inventory.json`, this cycle's HEAD.
- `43` units for `phrenic_slayer` (`ultimate_psionics`), magnitude
  distribution `{0: 31, 1: 5, 2: 7}` — direct field read/`Counter`,
  `docs/work-inventory.json`, this cycle's HEAD.
- `31` of 43 `phrenic_slayer` records carry exactly 2 `DESC:` rows,
  `PRERULE:1,DisplayFullAbility`-branched — direct read of all 31
  `data/corpus/ultimate_psionics/class_feature/phrenic_slayer_favored_enemy/
  *.json` files, `desc_count` computed per file (script run this cycle,
  reproducible).
- `2` correction/deferral events retro-logged —
  `docs/retro/events/sd34-wave38-laneb.jsonl`.

## Row-count command output

```
$ grep -n "^| [0-9]* |" docs/release/SD-34-book-completion/kanban.md | tail -1
| 37 | `mine-bucket-d` | 3 | wave 32, lane C (no AT-34-E# card yet) | partial | ...
```
Same accumulating row 37 (`mine-bucket-d`) — this cycle appends its own
sentence, per house style established by waves 32/35/36/37. Status stays
`partial`: this cycle closed 0 units.

## Build scope verified

No `src/`, `Cargo.toml`, or `scripts/` file touched this cycle — no build or
test run required or performed, per `workflow-instruction.md §6` step 3 ("if
touched"). `apps/desktop/src-tauri` — not touched, not run, same reasoning.

## Sweep population

`corpus_literal_sweep` — not run: no `data/corpus/**` file added, changed,
or removed this cycle (N/A, 0 delta by construction, nothing to sweep).

## Oracle pin

Not applicable — no figure this cycle came from the pinned PCGen oracle
corpus; every figure is derived from `docs/work-inventory.json` and the
committed `data/corpus/**` records directly.

## Status

**complete** — the dispatched investigation (pick the largest sub-mechanism-5
class group, build a real chassis if bounded, else stop and report honestly)
is fully executed: both of the two largest groups were read in full, both
found genuinely unbounded for different, specific, named reasons, a
zero-payoff partial build was explicitly checked and declined, and the
finding is written up precisely enough for the next dispatch to act on
without re-deriving it.

## Movement, four buckets

- **Closure:** 0.
- **Reclassification:** 0.
- **Reachability:** 0.
- **Instrument-correction:** 1 (sub-mechanism 5's population is a mix of ≥3
  disposition shapes, not one uniform "real new chassis needed per class"
  shape — retro-logged) + 1 deferral (the two investigated classes' real
  blockers named and scoped for a future dispatch).

## Notes (judgment calls)

- **Why no partial BAB/save-only chassis was built for either class**: Step
  4 proves it would close zero units (owner resolution alone cannot promote
  a magnitude-bearing record without a real computed magnitude behind it —
  the exact mechanism wave 37 lane B already proved for the 151
  `ranger_combat_style_feat` siblings). Shipping untested new chassis code
  for a proven zero-unit payoff would itself be a form of the "rushing an
  incomplete chassis" this cycle's own brief warned against — the code
  would exist, but would misrepresent progress if reported as "chassis
  built" without naming that it closes nothing.
- **Why `phrenic_slayer`'s 31-unit zero-magnitude subset looked promising
  before the PRERULE finding**: it matches wave 36 lane C's own "Order of
  the Dragon" precedent shape almost exactly (zero-magnitude, `" ~ "`-
  qualified, real description, correct owner once resolvable) — the
  difference (a second, house-rule-branched DESC row) is a real, deliberate
  data-shape difference this cycle confirmed against every one of the 31
  records individually, not assumed from one sample.
- **The book-field correction** (`divine_scion` is `inner_sea_magic`, not
  `adventurers_guide` as wave 37 lane B's receipt stated) is minor and does
  not change any figure; noted for the record since "every figure you write
  down carries the command that produced it" (`AGENTS.md` rule 9) — worth
  flagging that even a receipt entirely dedicated to re-deriving this
  population still shipped one wrong field, caught only by this cycle's own
  direct read of the raw records rather than trusting the prior summary.

## Next-cycle plan

1. **`divine_scion`'s 34-domain spell-like-ability subsystem and
   `phrenic_slayer`'s caster/manifester-level-stacking dependency are both
   real, named, un-closed work** — genuinely Epic 4/5 scope, not a bounded
   single-cycle chassis. Whoever eventually builds a caster/manifester-
   level-stacking mechanism for prestige classes (the same milestone
   `crb_untabled_class_chassis.rs`'s own doc comment already names for 6 of
   CRB's 10) should fold `phrenic_slayer`'s 12 magnitude-bearing records in
   at the same time.
2. **A PRERULE house-rule-toggle mechanism** would unblock `phrenic_slayer`'s
   31-unit zero-magnitude Favored-Enemy subset (and likely other
   `PRERULE:DisplayFullAbility`-branched records corpus-wide — not
   inventoried this cycle) independently of any chassis work — a
   genuinely separate, possibly smaller, piece of infrastructure worth
   scoping on its own.
3. **Sub-mechanism 5's remaining 58 classes (546 of 634 units) need the
   same per-class TYPE/MAXLEVEL/record-existence check this cycle ran on 10
   of them** before any more chassis-building cycles are dispatched against
   "largest classes first" — the 8-class sample found the population splits
   into (a) genuine prestige classes likely sharing the same caster-stacking
   blocker (majority), (b) name-collision/no-class-record cases needing a
   matcher fix instead of a chassis (at least 2 confirmed: `twilight_talon`,
   `golden_legionnaire`), and (c) `TYPE` mismatches worth a second look
   (`phantom`, `psychic_detective`). A full per-class classification pass
   (60 classes, cheap — one JSON read each) would let the next dispatch
   pick a genuinely bounded target instead of re-discovering this same
   pattern one class at a time.
