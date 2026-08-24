# Cycle t12-class-feature-pool-population, cycle 13 — Gate 3 (closure invariant) / row 18 (`epic-8-pool-shaped-class-features`)

- **Card ID:** row 18 (`epic-8-pool-shaped-class-features`), governed by `decisions.md §27b`
  (*"EVERYTHING"*). Dispatched scope: cycle 12 named an open question rather than absorbing it —
  its own `resolve_pcgen_var_chain` fix reaches real Bloodrager `DEFINE`-backed formulas in
  isolation, but no group's tally moved, because `resolve_pool_member_sole_magnitude` has
  independent per-member refusals it did not fully trace. This cycle's scope: take one concrete
  unresolved member, trace it end to end, name the refusal, verify the real semantics against
  PCGen's own Java before changing behaviour, then re-run
  `pool_group_closure_census_across_all_six_pools` and report real movement, including zero.
- **Base:** worktree `HEAD` was `1bb523773d32705d1b7387fd4c494861523f55ba` (PR #374's tranche/11
  merge) at session start — `git merge-base --is-ancestor "$PIN" HEAD` returned FALSE (footgun 1,
  the "fresh worktree on a stale lineage" failure mode). Working tree was clean
  (`git status --porcelain` — 0 output). Fixed: `git fetch origin tranche/12` (tip
  `5639862146`, cycle 12's own commit), confirmed `git merge-base --is-ancestor "$PIN"
  origin/tranche/12` == 0 (true), then `git reset --hard origin/tranche/12` (a plain
  fast-forward onto the correct lineage, not a rebase). `BASE_OK` re-verified after
  (`git merge-base --is-ancestor "$PIN" HEAD` → 0).
- **Oracle:** not consulted directly this cycle (cycle 12's own file:line citations of
  `PlayerCharacter.java`/`VariableProcessor.java`/`DefineLst.java` already establish the relevant
  0-default semantics; this cycle's own findings are corpus-structural, not PCGen-formula-grammar
  questions, and were verified by direct corpus inspection instead — see §1).

## 1. Tracing cycle 12's own named open question to root cause, per member

A brief diagnostic (`cycle13_diagnose_per_member_refusal_reasons`, printed via `--nocapture`, then
**removed before commit** — the same methodology cycle 12's own live diagnostic used) categorized
every real member of Bloodrager Bloodline, Cleric Domain, Shaman Spirit and Warpriest Blessing
into four buckets: `empty` (own `bonus_vars` empty — refuses before `combined_vars` is even
built), `multi_terminal` (more than one non-cross-referenced target — the existing "refuse rather
than guess which one" rule), `single_unresolved` (exactly one terminal target, but
`resolve_pool_member_sole_magnitude` still returns `None`), and `resolved`.

```
Bloodrager Bloodline: empty=50 multi_terminal=25 single_unresolved=7  resolved=6
Cleric Domain:        empty=54 multi_terminal=41 single_unresolved=50 resolved=30
Shaman Spirit:        empty=26 multi_terminal=28 single_unresolved=8  resolved=10
Warpriest Blessing:   empty=74 multi_terminal=0  single_unresolved=0  resolved=0
```

Cleric's 50 and Shaman's 8 `single_unresolved` members are the dominant bucket by far. Printing
each one's own target/formula (also removed before commit) showed every one of them is a
**single-hop reference to a bare header variable that is never merged into `combined_vars`** —
e.g. `"Air Domain ~ Lightning Arc"`'s `LightningArcTimes|DomainAirTimes`, `"Bones Spirit ~
Shedding Form"`'s `ShamanSheddingFormRounds|ShamanSpiritLVL`.

### 1a. `ShamanSpiritLVL` — a real, class-attributed base record never reached

`data/corpus/advanced_class_guide/class_feature/shaman/spirit.json` (KEY `"Shaman ~ Spirit"`,
`data.class` = `"Shaman"`) carries the real `BONUS VAR|ShamanSpiritLVL|ShamanLVL`. Every real
per-spirit member's own formula (`ShamanSheddingFormRounds|ShamanSpiritLVL`, and 7 siblings
across Bones/Flame/Lore/Nature/Stone/Waves/Wind Spirit) references this bare identifier directly.
It was never merged because neither the per-group header lookup (`pool_group` is the SPECIFIC
spirit's own name, e.g. `"Bones Spirit"`, never the bare word `"Spirit"`) nor the existing
`"<registered_name> Tracker"` merge (only ever tries `"Spirit Tracker"`) ever asked for the bare
registered name itself.

### 1b. `DomainPowerTimes` — a real, class-INDEPENDENT base record never reached

`data/corpus/core_rulebook/class_feature/domains/domains.json` (KEY exactly `"Domains"`,
`data.class` genuinely **absent** — confirmed live, no `CLASS:` token anywhere in its
`raw_tokens`; real PCGen grants it generically via the Cleric class's own
`ABILITY:...|Domains` reference, `core_rulebook/class_feature/cleric/domains.json`, itself a
zero-BONUS/DEFINE pointer record) carries the real `BONUS VAR|DomainPowerTimes|3+WIS`. Every
Domain's own `<X>Times` header var chains through it
(`DomainAirTimes|DomainPowerTimes|TYPE=Domain`, and 20+ siblings). `grep -rl 'VAR|DomainPowerTimes|'
data/corpus/` confirms exactly ONE corpus record ever targets this name — merging it is a real,
unambiguous corpus fact, never a guess. It was never reached because the existing bare-key merge
clause required an EXACT match (`table.get(pool_group)`), and every caller ever asks for the
SINGULAR registered name `"Domain"`, never the record's own PLURAL key `"Domains"`.

### 1c. Multi-terminal refusal — traced, confirmed correct, left unchanged

Draconic Bloodrager Bloodline's own 9 real members were checked directly: 4 have empty
`bonus_vars`, and the other 5 (`Claws`: 5 independent targets — `DamageDice`/`DamageDie`/
`DamageBonus`/`SizeBase`/`SizeBonus`; `Breath Weapon`: 3 — `Times`/`Dice`/`DC`; `Draconic
Resistance`: 2 — `ResistBonus`/`NaturalArmor`; `Dragon Wings`: 2) each carry **genuinely
independent real magnitudes in one record**, none referencing another. This is the multi-terminal
rule working as designed (`resolve_pool_member_sole_magnitude` returns exactly ONE
`(target, value)` pair — there is no single "sole magnitude" to report for a member with several
real, unrelated numbers), not a resolver bug. Cycles 2/5 already proved disabling a refusal
produces a fabricated value; this cycle left the rule untouched.

### 1d. Bloodrager's remaining 7 `single_unresolved` members — traced, a real, different blocker

Each (Aberrant `Staggering Strike`, Destined `Fated Bloodrager`, Elemental `Elemental Form`/
`Elemental Strikes`, Fey `Confusing Critical`, Infernal `Hellfire Strike`, Undead `Frightful
Charger`) chains through its own `Bloodrager_<X>_BloodlineLVL`. Direct corpus check: this
identifier is **not** bound anywhere on a pure-Bloodrager header (every `"Bloodrager Bloodline ~
<X>"` record only `DEFINE`s it as `0`, never `BONUS:VAR` binds it), but it **is** bound on a
same-named, cross-class `"Eldritch Scion <X> Bloodline"` record (e.g.
`data/corpus/advanced_class_guide/class_feature/eldritch_scion_aberrant_bloodline/
eldritch_scion_aberrant_bloodline.json`, `BONUS VAR|Bloodrager_Aberrant_BloodlineLVL|
BloodragerBloodlineLVL`, `data.class` = `"Sorcerer"` — the Eldritch Scion Sorcerer archetype
that trades its own bloodline for a Bloodrager one). Because that record IS a real corpus
`class_feature`, `every_corpus_bound_bonus_var_target()` correctly marks the name "bound
elsewhere", so cycle 12's own 0-default safety property (cycles 2/5's proven contract) correctly
refuses rather than guess. No corpus record anywhere binds a PURE Bloodrager's own per-bloodline
`BloodlineLVL` directly to `BloodragerLVL`/`BloodragerBloodlineLVL` for these five bloodlines —
a real, named finding for a future cycle, not forced here.

## 2. Implemented, narrowly (`§17a`: two real merges, both corpus-verified before writing)

`pool_header_record_by_normalized_suffix` (`pilot_compute/mod.rs`) gained one new clause:

- **Bare, trailing-`s`-tolerant class-independent lookup** — after the existing exact bare-key
  clause, a new pass searches every bare (no `" ~ "`) corpus key whose own trailing-`s`-trimmed
  form equals the caller's own trailing-`s`-trimmed `pool_group`, still class-checked
  (`header.class == class || header.class.is_empty()`) exactly like the exact clause it extends.
  Reaches `"Domains"` when asked for `"Domain"` (§1b).

`resolve_pool_member_sole_magnitude`'s existing `registered_name_for_tracker` merge site gained a
second call: alongside the pre-existing `"<registered_name> Tracker"` lookup, it now ALSO merges
`pool_header_record_by_normalized_suffix(owning_class, registered_name, None)` — the bare
registered name itself. This reaches `"Shaman ~ Spirit"` directly (§1a, via the function's own
EXISTING exact-key clause) and, as a side effect confirmed correct and not merely convenient,
also reaches `"Warpriest ~ Blessings"` (`WarpriestBlessingLVL|WarpriestLVL`) via the function's
pre-existing `"<class> ~ "`-prefixed trailing-`s`-tolerant wildcard (unchanged, cycle 5's own
widening) — traced and confirmed real (§3), even though it does not flip any Warpriest group.

Neither change touches the multi-terminal rule (§1c) or the "bound elsewhere refuses"
0-default safety property (§1d) — both traced, both confirmed correct, both left exactly as
cycle 12 left them.

## 3. Verified before writing: no unintended cross-matches

Before landing, every other class/registered-name pair this module resolves through the same call
site was checked directly against the corpus for an unintended bare-key or trailing-`s` match:
`"Sorcerer ~ Bloodline"` / `"Bloodrager ~ Bloodline"` / `"Cleric ~ Domain"` do not exist as
corpus keys (no accidental match); `"Cavalier ~ Order"` DOES exist (binds `OrderAbilityLVL`) but
changes nothing — every real Cavalier Order member's own `bonus_vars` is empty (cycle 10's own
finding, re-confirmed unchanged by this cycle's census). `"Warpriest ~ Blessings"` merging
`WarpriestBlessingLVL` was likewise confirmed inert for the census (§1c/Warpriest's own §17a note
below) since every real member record carries zero `BONUS:VAR` tokens.

## 4. Tests, RED→GREEN (`§1a`)

3 new tests: `pool_header_lookup_reaches_a_bare_plural_class_independent_base_record` (proves the
new bare-key trailing-`s` clause reaches `"Domains"`'s real `DomainPowerTimes`),
`pool_header_lookup_reaches_the_class_wide_registered_name_base_record` (proves the widened
tracker-merge site reaches `"Shaman ~ Spirit"`'s real `ShamanSpiritLVL`),
`shaman_generic_spirit_pass_now_grounds_a_bare_registered_name_chain` (end-to-end through
`resolve_pool_member_sole_magnitude` itself: `"Bones Spirit ~ Shedding Form"` now resolves to
`("ShamanSheddingFormRounds", 5)` at character level 5, previously `None`).

Mutation-style proof: this cycle's own diff was fully reverted (`git diff -- src/rules_core/
pilot_compute/mod.rs > /tmp/.../cycle13.patch`, `git checkout -- src/rules_core/pilot_compute/
mod.rs` — never `git stash`, per this bundle's own hard rule), and
`pool_group_closure_census_across_all_six_pools` re-run against the reverted code reproduced
cycle 12's own exact pinned baseline (`Cleric Domain: 26/72`, `Shaman Spirit: 8/14`, all others
unchanged) — confirming this cycle's change, not something else, is what moves the numbers. The
diff was then reapplied (`git apply /tmp/.../cycle13.patch`) and re-verified green.

```bash
cargo test --locked --lib -- rules_core::pilot_compute::generic_pool_group_selection_wiring_tests::pool_group_closure_census_across_all_six_pools --nocapture
```
```
Sorcerer Bloodline: 31/53 groups carry a resolvable member
Bloodrager Bloodline: 5/12 groups carry a resolvable member
Cleric Domain: 34/72 groups carry a resolvable member
Shaman Spirit: 11/14 groups carry a resolvable member
Warpriest Blessing: 0/37 groups carry a resolvable member
Cavalier Order: 1/9 groups carry a resolvable member
```

**Real movement this cycle:** `Cleric Domain` 26/72 → 34/72 (+8, all via the `"Domains"` merge).
`Shaman Spirit` 8/14 → 11/14 (+3, all via the `"Shaman ~ Spirit"` merge — the diagnostic's own
`single_unresolved` bucket for this pool dropped 8 → 0). `Sorcerer Bloodline`, `Bloodrager
Bloodline`, `Warpriest Blessing`, `Cavalier Order` honestly UNCHANGED (31/53, 5/12, 0/37, 1/9) —
re-run and re-checked, not assumed.

```bash
cargo test --locked --lib -- rules_core::pilot_compute::
```
```
test result: ok. 955 passed; 0 failed; 0 ignored; 0 measured; 1843 filtered out; finished in 26.67s
```
(up from cycle 12's 951/951 — +4: 3 new tests above, plus the census re-derivation)

```bash
cargo test --locked --lib -- rules_core::derived_evaluator_fixture_check::
```
```
test result: ok. 121 passed; 0 failed; 0 ignored; 0 measured; 2677 filtered out; finished in 8.65s
```

```bash
cargo test --locked --lib -- oracle_dispatch_widening_safety_tests
```
```
test result: ok. 48 passed; 0 failed; 0 ignored; 0 measured; 2750 filtered out; finished in 4.22s
```
`a_mystery_pick_alone_grounds_no_tier_one_revelation` untouched, still green (Oracle Mystery
stays withdrawn per its own standing ruling).

```bash
cargo test --locked --lib -- cavalier
```
```
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 2782 filtered out; finished in 9.09s
```

## 5. Not attempted (named, per `§17`)

`classlevel("X","APPLIEDAS=NONEPIC")`'s real 2-argument form and Hunter Animal Focus: unchanged
from cycles 9–12. Cavalier's 8 real no-`BONUS:VAR` Orders: unchanged, confirmed again (§3).
Bloodrager's remaining 7 single-terminal members (§1d — a real, named, cross-class-record-owned
`BloodlineLVL` blocker) and Warpriest's 74 zero-`BONUS:VAR` members: named, not forced. Oracle
Mystery: stays withdrawn, `oracle_dispatch_widening_safety_tests::
a_mystery_pick_alone_grounds_no_tier_one_revelation` untouched. Rows 11/15 left as found
(`in-progress`/`complete` respectively), untouched. `apps/desktop`'s row 19/20 lanes not touched.
`data/corpus/**` untouched throughout (`git status --porcelain -- data/corpus` — 0 changes).

## 6. Audits

- **Identifier audit** (`grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'` against
  `git diff -- src/rules_core/pilot_compute/mod.rs`, scoped to this cycle's own diff): `OK_NO_
  BUNDLE_TAGS` — 0 hits (excluding the literal `SD-32` doc-comment references, which are not
  bundle-scoped identifiers).
- **Wired-integration audit** (`grep -inE "STUB|MOCK|placeholder|not.?yet.?implemented|todo|fixme|
  hack"`, same scope): `OK_NO_TOKENS` — 0 hits.
- **PI audit**: `pi_scrub.normalized_term_hits(...)` (imported via `scripts/pi_scrub.py`, never
  copied), scoped to `git diff --unified=0` of this cycle's own diff → `[]` (0 hits). `data/
  corpus/**` untouched throughout.

## 7. Kanban

`docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 18 only (cycle field 12 →
13, Notes appended). Verified: 21 distinct `^| N |` rows, 0 duplicates, row 18 parses to 7
real columns (9 raw pipe-split fields including the leading/trailing empty strings from the outer
pipes) before and after (backtick-aware parser). Rows 11 (`in-progress`) / 15 (`complete`)
confirmed untouched from their pre-cycle state.

## 8. `df -h /`

```
Filesystem      Size  Used Avail Use% Mounted on
/dev/sda1       968G  497G  472G  53% /
```
