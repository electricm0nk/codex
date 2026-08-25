# Cycle — Gate 3 (closure invariant) / Card 11, lane T2a-residual (alias-tier batch)

- **Card ID:** `epic-2-cause-closure` (row 11; this receipt covers ONLY the T2a-residual
  alias-tier batch — the census that sized this work is
  `artifacts/gate-3-closure-invariant/card11-t2a-residual-census-census.md`)
- **Commit SHA:** (this cycle's commit — see `git log -1 --format=%H` after push, recorded in
  `progress.md`'s entry for this cycle)
- **Files touched:**
  - `src/rules_core/cache_gen/class_feature.rs` (new sixth resolution tier —
    `CATEGORY_LABEL_ALIASES` + `category_label_alias_owner`, wired into `generate()`'s chain
    after `corpus_class_owner`; +9 new tests, 2 new end-to-end `generate()` tests)
  - `data/corpus/**/class_feature/**/*.json` (12,382 regenerated records — data, not code; 814
    `data.class` values actually changed)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  (`git diff --unified=0 HEAD -- src/rules_core/cache_gen/class_feature.rs` — no
  `sd[0-9]+_`/`SD[0-9]+_`/`Sd[0-9]+`/`t_[0-9a-f]{8,}` matches)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff — no
  `STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack` tokens)
- **Acceptance criterion:** `acceptance-and-verification.md` AT-32-E2-001 — "Cause closure closes
  by class, not by instance." This cycle's scope, per the dispatch brief: card 11's T2a-residual
  sub-population (`decisions.md §13`), sized by
  `artifacts/gate-3-closure-invariant/card11-t2a-residual-census-census.md` at 2,640 units / 547
  labels / 18 books, 12 cycles.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`
  `PCGEN_ORACLE_SHA`) — fresh worktree, empty oracle slot, self-healed per §8 via
  `scripts/fetch-pcgen-oracle.sh --dest <repo-local pcgen slot>`; matches the pin exactly.
- **Status:** complete (this batch's own scope; row 11 stays `in-progress` — the residual after
  this batch is real, sized future work, not this cycle's to finish alone per the census's own
  12-cycle sizing)

## Re-deriving the population before trusting the census (`decisions.md §17a` bar)

Re-ran the census's own script at this cycle's start:

```
python3 scripts/sd32-t2a-residual-census.py
# -> total 12464  non-null-class 11904  dispatched 7620  corpus-declared-undispatched(T12-overlap-shape) 1644  residual-category-label(T2a-residual) 2640
```

Confirmed: **2,640 units, 547 distinct labels** (independently re-counted with a fresh script
against `docs/work-inventory.json`'s `kind == "class"` roster — got 548/2,642, a ~2-unit
class-roster-definition edge case, not a computation error; the census's own 547/2,640 is trusted
as the more carefully-audited figure and used throughout this receipt).

## The bottleneck, and why it was widened (`decisions.md §17`)

`POOL_TO_DISPATCHED_CLASS` (the existing 27-entry precedent) resolves a label ONLY when the
label's own text shares a suffix/prefix with the target class's own text (`"Rage Power"` ->
`"Barbarian"` via the registered word `"Rage Power"` itself). Zero of the 547 residual labels
matched it (confirmed by the census). Many of the highest-value labels in the residual — `"Ki
Power"` (80 units) -> `"Monk"`, `"Wild Talent"` (128) -> `"Kineticist"` — share NO text with their
real owning class at all; the relationship is provable only by reading the record's own `PRE:`/
`BONUS:`/`ABILITY:` tokens (`"PRE: 1,Monk=4"`, `"BONUS ... PREVARGTEQ:MonkLVL,10"`,
`"ABILITY:...PREVARGTEQ:WarpriestBlessingLVL..."`). A text-suffix table structurally cannot reach
these. Per `decisions.md §17`'s standing control ("if your scope is the bottleneck, widen it and
say so"), this cycle adds a sixth resolution tier, `CATEGORY_LABEL_ALIASES` /
`category_label_alias_owner`, keyed on the exact label text with no suffix fuzzing, verified
per-label the same way `CLASS_FEATURE_POOLS`' own 27 entries were built (`decisions.md §3`'s
fixture-check bar): reading every one of that label's corpus records' `TYPE:`, `PRE*:`, `BONUS`,
and `ABILITY` tokens (not a sample), confirming a single class, no cross-book or cross-class
collision.

## 21 labels verified and closed — 814 units

| Label | Units | Real owner | Evidence read (representative) |
|---|---:|---|---|
| Wild Talent | 128 | Kineticist (undispatched) | `PRE: KineticistLVL_Fire,10` — occult_adventures only |
| Refined Education | 94 | Rogue | `BONUS: VAR\|CraftRefinedEducationUnlock\|1\|PRECLASS:1,Rogue=4` — ultimate_intrigue only |
| Ki Power | 80 | Monk | `PRE: 1,Monk=4` — ultimate_magic (+2 advanced_race_guide, same shape) |
| Master of Many Styles | 53 | Monk | `PRE: [PREABILITY...Master of Many Styles~Djinni Style], MonkBonusFeatLVL,1` — ultimate_combat only |
| Implement School Focus Power | 48 | Occultist (undispatched) | `PRE: OccultistSchool_Abjuration=true, OccultistLVL,3` — occult_adventures only |
| Pack Lord | 40 | Druid | `TYPE: DruidClassFeatures.SpecialQuality` / `.PackMembers` — ultimate_magic only |
| Adaptation | 39 | Ranger | `TYPE: ClassFeatures.RangerClassFeatures.AdaptationSelection` — advanced_players_guide only |
| Blessings | 37 | Warpriest | `ABILITY:...PREVARGTEQ:WarpriestBlessingLVL,...` — every one of 74 checked ABILITY tokens, both books (advanced_class_guide 33, ultimate_wilderness 4) |
| Favored Enemy Bonus | 37 | Ranger | `TYPE: RangerClassFeatures.FavoredEnemyBonus` — core_rulebook/ultimate_intrigue/inner_sea_intrigue |
| Infiltrator | 31 | Ranger | `TYPE: ClassFeatures.RangerClassFeatures.SpecialQuality.AdaptationCreature...` — advanced_players_guide only |
| Wildcat | 28 | Monk | `BONUS: VAR\|WildcatDisarmBonus\|1\|PREVARGTEQ:MonkLVL,10` — advanced_class_guide only |
| Hunter's Tricks | 26 | Ranger | `TYPE: ClassFeatures.RangerClassFeatures.SpecialQuality.Extraordinary.HuntersTrickChoice` — advanced_players_guide only |
| Packmaster | 20 | Hunter | `PRE: AnimalCompanionLVLI,HunterLVL` — advanced_class_guide only |
| Packmaster Follower | 20 | Hunter | paired 1:1 by key/book with `Packmaster` (`advanced_class_guide` only, no other book) |
| Beastmaster | 20 | Ranger | `PRE: AnimalCompanionLVLI,RangerLVL` — advanced_players_guide only |
| Beastmaster Follower | 20 | Ranger | paired 1:1 by key/book with `Beastmaster` (`advanced_players_guide` only) |
| Maneuver Master | 20 | Monk | `PRE: MonkBonusFeatLVL,1` — ultimate_combat only |
| Wildblooded | 20 | Sorcerer | `PRE: 1,CATEGORY=Special Ability,Sorcerer Bloodline ~ <X>` — ultimate_magic only |
| Favored Terrain Bonus | 18 | Ranger | `TYPE: RangerClassFeatures.FavoredTerrainBonus` — advanced_players_guide/core_rulebook |
| Terrain Mastery | 18 | Ranger | `PRE: 1,CATEGORY=Special Ability,Favored Terrain ~ <X>` — advanced_players_guide/ultimate_combat |
| Terrain Dominance | 17 | Ranger | `PRE: 1,CATEGORY=Special Ability,Terrain Mastery ~ <X>` — advanced_players_guide only |

Every label checked across ALL its records (not a sample) for TYPE/PRE/BONUS/ABILITY-token
consistency and single-book-or-single-class agreement before being added to
`CATEGORY_LABEL_ALIASES`; commands are the inline Python snippets in this cycle's own working
notes, re-derivable against the pinned oracle via the same corpus-walk pattern the census script
uses (`data/corpus/*/class_feature/**/*.json`, filter on `data.class == "<label>"`, read
`data.raw_tokens`).

## Two labels deliberately NOT mapped, with the reason (`decisions.md §1a`/§15-adjacent discipline)

- **`Domain Power` (172 units, the single largest group).** Verified genuinely multi-owner:
  158/172 records' `DESC` text names no class at all, the 14 that do split 13 Cleric-only + 1
  Cleric+Druid, and the `PRE:`/`TYPE:` tokens are generic (`DomainLawLVL`,
  `SpecialQuality.DomainPower`) shared by every class with domain access (Cleric, Inquisitor's
  Inquisition, Warpriest's Blessing-domain hybrid, Paladin's Sacred Servant archetype). There is
  no per-record corpus signal that says which class granted a given record. Forcing this into
  `CATEGORY_LABEL_ALIASES` the way `"Rage Power" -> "Barbarian"` works would be exactly the
  anti-gaming failure `decisions.md §1a` names: a relabelled shape, not a closed one. Pinned by a
  standing test (`category_label_alias_owner_refuses_the_known_multi_owner_and_not_class_owned_labels`)
  so a future edit cannot silently reintroduce a single-class mapping without a reviewer noticing
  the assertion break. **Reported open, 172 units, reason: genuinely multi-owner, no per-record
  signal in this generator's inputs; closing it needs either (a) a source beyond `TYPE:`/`PRE*:`
  tokens — cross-referencing which specific class build each domain-power-granting deity/domain
  entry is attached to in the PCGen source tree, which this generator does not currently read at
  all — or (b) an operator ruling on whether "shared across domain-access classes" is itself an
  acceptable disposition, distinct from "closed by class."**
- **`Demonic Obedience` (42 units).** Verified NOT class-owned at all: every one of 42 records'
  `PRE:` tokens names a demon lord (`Shivaska`, `Jubilex`, ...), never a class or class-shaped
  variable — a deity-obedience feat line, structurally outside any PC class chassis (comparable to
  a boon feat). Correctly excluded, not silently dropped — same standing test pins the exclusion.
  **Reported open, 42 units, reason: not class-owned; the correct closure for this shape is
  proving it should stay a category label (or be re-typed to a different `kind` than
  `class_feature`), which is a disposition this cycle records but does not itself execute (out of
  this cycle's granted scope — `kind` re-typing touches `v06_work_inventory.rs`, not this
  generator).**

## Consumer-conflict audit (re-run, not assumed)

```
grep -rn 'data\["class"\]\|data\.get("class")\|\.class ==\|data\.class' src/ apps/desktop/src-tauri/src/
```

Same four readers the T2a+T12 cycle and the census both found: `class_feature_pool_catalog.rs`
(already fixed, reads `key`-split, not `data.class`), `class_feature_descriptions.rs`,
`class_feature_grant_consumer.rs`, `class_feature_feat_bridge.rs` — all three treat `data.class`
as "the record's real owning class" and only benefit from a more accurate value. **No new
consumer-conflict hazard.**

## Regeneration discipline

`cargo run --locked --bin gen_cache_class_feature` (env: `PCGEN_CORPUS_ROOT` pointed at the
repo-local pinned oracle slot) regenerated 12,384 records across the generator's 21 in-scope
books. Every regenerated file's diff was checked field-by-field against its pre-image
(`ingested_at`/`data.class` stripped before compare) — script:
`/tmp/.../diff_check.py` (ad hoc, not committed; the comparison logic is inline Python, same
shape as the T2a+T12 receipt's own per-file diff check). **12,382 of 12,384 changed ONLY `class`
and/or `ingested_at`.** **2 files diverged in other fields too** —
`adventurers_guide/enlightened_bloodrager/bloodline_feat.json` and
`core_rulebook/draconic_bloodline/draconic_bloodline.json` — the SAME pre-existing
citation-line-drift pair the T2a+T12 receipt already logged as an incident
(`class-feature-citation-line-drift`), unrelated to this cycle. Both reverted to HEAD before
committing.

Of the 12,382 clean regenerations, **814 records' `data.class` value actually changed** (table
above). `corpus_literal_sweep` ran clean afterward: `26538 records examined ... 0 findings,
CLEAN`.

## RED -> GREEN

`.or_else(|| category_label_alias_owner(group, corpus_class_names))` mutated to
`.or_else(|| None::<String>)`:

```
cargo test --locked --lib cache_gen::class_feature::tests::generate_writes_the_alias_owner_for_a_text_free_category_label
# FAILED: left: Some("Ki Power")  right: Some("Monk")   -- failed for the intended reason
```

Reverted; green again. Module suite (`cache_gen::class_feature::`): 32/32 pass (9 new).

## Suites run

- `cargo test --locked --lib` — **2,402/2,402 pass, 0 failed, 13 ignored.**
- `cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml` (separate cargo
  workspace) — **518/518 pass, 0 failed.**

## Pinned-count sweep

`grep -rn "2,775\|2775\|2,640\|2640" --include='*.rs' --include='*.py'` across `src/`, `tests/`,
`scripts/`, `apps/` — no hits outside this bundle's own docs (`decisions.md`, `kanban.md`,
`progress.md`, the census memo, the census script) and one unrelated SD-19 doc. No test or script
pins the exact 5,678/4,284/2,640 population figures as an assertion, so this cycle's 814-unit
shift does not leave any other file's hardcoded count red. The pre-fix module doc-comment prose
citing "5,678" (this file, line ~476) is historical narration of the T2a+T12 cycle's own
before/after, not a pinned assertion — left as-is; it describes what that PRIOR cycle found, not
this cycle's population.

**Note for the next lane / Gate 3 owner:** this batch moves the corpus's `data.class` distribution
(814 records), which may move `shape_ledger.py`'s F-family counts and Gate 3's `no_record` budget.
Per this cycle's explicit instruction, the budget constants were **not** touched. Re-derive with
`python3 scripts/shape_ledger.py` and `scripts/verify.sh --only gate3` before the next Gate-3-owning
cycle trusts either figure.

## What this batch closes, and what remains open, honestly

**Closed:** 814 of 2,640 T2a-residual units (30.8%), across 21 verified single-owner labels, by
the same generator that produces every record in scope — corpus-wide, not sampled.

**Not closed, reported with reason, not fabricated:**
- 214 units (`Domain Power` 172 + `Demonic Obedience` 42) verified and deliberately excluded —
  see the section above for the exact per-label reason and what would be needed to close each.
- 2,640 − 814 − 214 = **1,612 units across the remaining ~525 labels**, not yet individually
  verified by this batch. The census's own cost model (per-group `TYPE:`/`PRE*:` verification,
  batchable several-per-cycle since they share one file) still applies; this batch is one
  instalment of the census's 12-cycle sizing, not its close.

- **Discovery forwards:** none requiring a new card.
- **Next-cycle plan:** continue `CATEGORY_LABEL_ALIASES` verification through the remaining ~525
  labels (the census's per-book table names where they live), or escalate `Domain Power`'s
  multi-owner disposition to the operator if a work lane wants it resolved before row 11 can
  close — it cannot be closed by this generator's current inputs alone.

`df -h /`: recorded at end of turn, see final report.
