# Cycle t12-class-feature-pool-population — Gate 3 (closure invariant) / Card 11 (row 11's pool-shaped `class_feature` population)

- **Card ID:** 11 (`epic-2-cause-closure`) — the pool-shaped `class_feature` population sized by
  `t12-census-widening-followup_cycle-1_cycle_receipt.md` §5 (commit `cd60d08042`): ~1,913 group-qualified
  names, ~16,350 records, ~6,131 (raw estimate) magnitude-bearing, of which `class_feature_pool_catalog.rs`
  modeled only 2 groups (~71 records). This cycle's scope: (1) re-derive that estimate precisely, per
  `§17a`; (2) extend the catalog to serve the population generically, per `§17`/`§27b`.
- **Commit SHA:** see `git log -1` at push time (rebased onto `origin/tranche/12` before pushing, §5)
- **Base:** `cd60d08042a8008c65cd7c0f3f0de42696cabc1e` (== `origin/tranche/12` at cycle start; local worktree
  HEAD had drifted to an unrelated SHA at dispatch, reset to the pin before starting, confirmed by
  `git merge-base --is-ancestor`)
- **Files touched:**
  - `scripts/census_class_feature_pool_population.py` — **new**, committed (prior cycle's sizing script was
    ad-hoc/uncommitted; this one mirrors `class_feature_pool_catalog.rs::ENGINE_EFFECT_TOKEN_KEYS` exactly
    so its numbers are the SAME criterion the catalog itself uses, not an independent heuristic)
  - `src/rules_core/class_feature_pool_catalog.rs` — `REGISTERED_POOL_GROUPS` two-entry allowlist check
    replaced by `is_registered_pool_group` (universal `" ~ "`-group-qualified match); two new safety gates
    (`carries_unimplemented_marker`, `carries_class_specific_level_phrase`); module doc rewritten (scope
    section); 5 new tests, 1 pre-existing test rewritten for the new semantics (23 tests total, was 18)
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 11 cycle entry prepended (row
    left `in-progress` per dispatch instruction)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (working-tree diff on this cycle's own touched files,
  `git diff --unified=0` against the pre-cycle HEAD: `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|
  t_[0-9a-f]{8,})\b'` — 0 hits)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff scope, `grep -inE
  "STUB|MOCK|placeholder|not.?yet.?implemented|todo|fixme|hack"` — hits found are all doc-comment prose
  describing the `[not implemented]`/`[not enforced]` literal marker found IN CORPUS DATA that the new
  `carries_unimplemented_marker` guard refuses to serve — the same convention the prior cycle's receipt
  used ("0 hits outside this cycle's own doc-comment citations"). No actual stub/placeholder code.)
- **Acceptance criterion (this cycle's stated scope):** re-derive the pool population precisely and
  report the honest number in both directions (§17a); extend the catalog generically, group-driven, not
  per-object (§17); close whole groups where the safety gates prove it is correct to (§27b: "novelty of
  shape is grounds for sizing, not exclusion").
- **Corpus SHA:** no oracle re-fetch needed this cycle — all work reads `data/corpus/` (already-ingested,
  committed cache), never the raw PCGen oracle directly; `PCGEN_ORACLE_SHA` unchanged from the prior
  cycle's bootstrap (`7f818006e371188e5717fd18d74d18a420747fc6`), confirmed present at
  `docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/`.
- **Status:** complete (this cycle's own scope: re-derivation + generic text-only widening). The
  numeric-magnitude residual (5,981 records, real compute functions) is sized, not closed — next cycle.
- **Discovery forwards:** `## DISCOVERED` — 17 `occult_adventures` `class_feature` records
  (`Sha'ir`/`Necroccultist`/`Toxitician`/... pool groups) carry a literal `[not implemented]`/
  `[not enforced]` stub marker baked directly into `data.description` by upstream ingestion
  (`cache_gen::class_feature::generate`, outside this cycle's file territory) — a real no-stub-doctrine
  defect this cycle's widening would otherwise have shipped onto a player's sheet verbatim. Mitigated at
  this catalog's own boundary (`carries_unimplemented_marker`); the root cause in `cache_gen` is
  unaddressed and should be a named line item for whichever cycle next touches that ingestion path.
- **Next-cycle plan:** close the 5,981-record numeric-magnitude residual by classifying into the four
  compute shapes (flat/constant · level-scaled linear/floor-division · level+ability_modifier ·
  ability_modifier-only), largest groups first: Warpriest Bonus Feat (432 records, 38 numeric-magnitude),
  Domain Power (172/148, partially already closed via `decisions.md §23a`'s generator-input extension —
  reconcile against that before assuming all 148 are still open), Aegis (126/88), Inquisitor Domain
  (124/106), Refined Education (94/94), Social Grace (85/85), Medium (85/42), Shaman Spirit Hex (59/46).
  Group by SOURCE FILE per class, per the standing lesson.

---

## 1. Re-derivation, precise, both directions (§17a)

**Method:** `scripts/census_class_feature_pool_population.py` (new, committed) scans every
`data/corpus/*/class_feature/**/*.json` record whose `key` is `" ~ "`-group-qualified, and classifies each
using the EXACT SAME criterion `class_feature_pool_catalog.rs::ENGINE_EFFECT_TOKEN_KEYS` uses (not an
independent heuristic — the prior cycle's own raw estimate used a looser "has BONUS or %N-substituted
DESC" proxy, which this script deliberately supersedes with the real gate).

```bash
python3 scripts/census_class_feature_pool_population.py
```
```
class_feature pool-shaped population census (decisions.md §17/§17a)
  files scanned                                18076
  malformed JSON                                    0
  distinct ' ~ '-group-qualified names           1913
  total group-qualified records                 16350
  catalog-servable text-only (no engine token)    7423
  any engine-effect token (ABILITY/CSKILL/SELECT/AUTO/SAB/BONUS/DEFINE/ADD/SPELLS/DR/SR)
    records                                       8927
  numeric magnitude (BONUS/DEFINE var math, or %N-substituted DESC)
    records                                        6306
  already modeled elsewhere (groups: Domain Power, Inquisitor Domain, Rage Power, Rogue Talent)
    records                                         596
    of which numeric-magnitude                      325
  RESIDUAL numeric-magnitude needing compute       5981
```

**Instrument validation (§17a: validate before trusting)** — checked against two independently-known
values before trusting the tool's output on the full corpus:
- `Domain Power`: 172 total records — matches `decisions.md §23a`'s own stated population exactly.
- `Rage Power`: 170 total records — matches `class_feature_pool_catalog.rs`'s own module doc ("scanning
  all 170 real Rage Power records") exactly.

Both known cases reproduce exactly; the instrument is trusted for the rest of the corpus.

**Which direction the prior estimate moved:** the group count (1,913) and total-record count (16,350)
both reproduce the prior cycle's raw estimate almost exactly (previously reported as "~1,913"/"~16,350" —
now exact, not approximate). The "magnitude-bearing" figure moved UP from the prior cycle's own
"~6,131" heuristic to a precisely-derived **6,306** once measured against the catalog's real gate — a
modest correction in the direction of MORE work, not less, consistent with this bundle's own pattern of
raw estimates undercounting rather than overcounting. A SEPARATE, broader figure — "any engine-effect
token" (8,927) — is reported distinctly and must not be conflated with "needs a compute function": most
of the gap between 6,306 and 8,927 is `ABILITY`/`SELECT`/`AUTO` grant/choice tokens (boolean mechanics,
not scaled magnitudes), a different closure shape entirely.

**Double-counting check:** `Domain Power`, `Inquisitor Domain`, `Rogue Talent`, and `Rage Power` are
already-modeled elsewhere (the first two via `decisions.md §23a`'s generator-input extension; the last two
via this catalog's own pre-existing `REGISTERED_POOL_GROUPS`) — 596 records, 325 numeric-magnitude,
excluded from the residual to avoid re-counting work already done. **Residual, precise: 5,981
numeric-magnitude records genuinely need a real compute function.**

## 2. The mechanism, extended generically (§17, not per-object)

`class_feature_pool_catalog.rs`'s `REGISTERED_POOL_GROUPS` was a two-entry literal-string allowlist
(`"Rogue Talent"`, `"Rage Power"`). Every OTHER record this catalog refuses is refused by a safety gate
that is already group-name-agnostic (render-and-refuse for an unresolved `%N`, the engine-effect-token
refusal, the archetype-lock refusal, the multi-`DESC:` refusal, the bare-`%N`-no-pipe-tail refusal) — none
of those gates ever consulted the group's name for correctness, so the allowlist was pure scope-narrowing,
not a safety property (confirmed by reading `load_pool_catalog`'s own filter chain, `class_feature_pool_
catalog.rs` lines ~396–450 pre-cycle).

**Change:** `is_registered_pool_group(key)` replaces the allowlist check — `true` for any key containing
`" ~ "`. `decisions.md §17` ("serve the population as a class... group-driven, config-shaped, not one
module per pool") and `§27b` ("EVERYTHING. No carve-outs survive... novelty of shape is grounds for
sizing, not exclusion") both directly forbid keeping a hand-curated allowlist once its cost (a per-pool
"spot-check" the module's own prior doc cited) is weighed against those rulings.

**Two NEW generic safety gates**, added because the widening exposed two real risks the two-pool-only
scope had never encountered:

1. **`carries_unimplemented_marker`** — 17 `occult_adventures` records (`Sha'ir ~ Jin`, `Necroccultist ~
   Necromantic Bond`, `Toxitician ~ ...`, and 14 others) carry a literal `[not implemented]` or
   `[not enforced]` marker baked directly into `data.description` by upstream ingestion. Confirmed real,
   not synthetic: `grep -rl '\[not implemented\]' data/corpus/*/class_feature/**/*.json | wc -l` → 16;
   `grep -rl '\[not enforced\]' ...` → 1. None of the pre-existing gates (render-and-refuse, engine-effect
   token, multi-DESC, bare-%N) would have caught this — it is syntactically clean prose that happens to
   start with a bracketed editorial note. Serving it as `text-complete` would have shipped the literal
   string `"[not implemented]"` onto a player's character sheet — exactly the defect
   `docs/governance/no-stub-mvp-doctrine.md` exists to catch, and one this cycle's own widening would have
   introduced if left unguarded (the two original groups never reached `occult_adventures`).
2. **`carries_class_specific_level_phrase`** — generalizes `CLASS_LEVEL_SCALED_SHEET_VALUE_EXCLUDED_KEYS`
   (a 16-entry, hand-verified, Rage-Power-only denylist from SD-31 wave 23's review) to every newly-admitted
   group. That review found records whose prose scales on `"barbarian level"` (a class-specific phrasing
   `wiring_class.rs`'s shared `prose_scaling_phrases` list does not recognise) AND applies to a value this
   engine already computes elsewhere — serving those as plain rendered text would misreport a
   genuinely-needs-computation record as done. Hand-verifying that same precondition for ~1,900 newly
   admitted groups is infeasible in one cycle, so the new guard is deliberately the CONSERVATIVE half
   alone: refuse ANY record whose description names its own owning class immediately followed by
   "level"/"levels", regardless of whether the engine happens to compute the referenced value. A false
   refusal costs nothing (the record stays `not_ingested`, unchanged from before this cycle); a false
   `text-complete` would be a new, wrong answer (`§1a`).

**Result, measured live against the real corpus** (`the_widened_catalog_serves_far_more_than_the_original_
two_groups`, `cargo test --nocapture`):

```
class_feature_pool_catalog: 3975 entries across 1057 groups
```

Before this cycle: ~71 records across 2 groups. **The catalog now serves 3,975 real, rendered,
gate-verified descriptions across 1,057 pool groups** — the SAME `v06_work_inventory.rs::classify()`
consumer wiring as before (no new consumer code needed; the "no owner resolved" and "owner resolved"
branches both already consulted `class_feature_pool_catalog_holds` from the prior wave's work), so every
one of these 3,975 records now reaches `text-complete` through the existing, already-tested pipeline.

This closes the majority of the population's TEXT-ONLY bucket (7,423 candidate records per the census
above; 3,975 actually clear every safety gate — the gap is exactly what the render-and-refuse, engine-
effect-token, archetype-lock, multi-DESC, bare-%N, unimplemented-marker, and class-level-phrase gates
correctly withhold pending either upstream data fixes or real computation).

## 3. Tests (§1a — no relabelled shape)

23 tests in `class_feature_pool_catalog.rs` (was 18), all passing against the LIVE corpus (not a fixture):
```bash
cargo test --locked --lib -- class_feature_pool_catalog
```
```
test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 2706 filtered out
```

New tests, non-vacuous against the live corpus:
- `only_group_qualified_keys_are_ever_served_but_every_group_is_now_eligible` — replaces the old
  `unregistered_pool_groups_are_never_served` (whose assertion the widening deliberately inverts); proves
  every served key still carries `" ~ "` AND a newly-widened group (`Vigilante Talent`) now appears.
- `loads_a_real_clean_vigilante_talent_from_a_newly_widened_group` — `inner_sea_intrigue: Vigilante Talent
  ~ Turnabout`, a real corpus record from a group that was NOT one of the original two, proven served with
  its real rendered description.
- `carries_unimplemented_marker_catches_both_bracket_shapes` + `a_record_carrying_a_literal_
  unimplemented_marker_is_refused_by_the_live_catalog` — the latter proves the LIVE catalog refuses
  `occult_adventures: Sha'ir ~ Jin`, the real record carrying the real marker.
- `carries_class_specific_level_phrase_generalizes_the_rage_power_denylist` — unit-level proof the new
  guard fires on the owning class's own name and not on an unrelated class's name in the same phrase shape.
- `the_widened_catalog_serves_far_more_than_the_original_two_groups` — the live-corpus floor assertion
  (>500 records, >50 groups) proving this is real, large-scale, non-fixture-only work.

**Downstream consumer regression check** (`v06_work_inventory.rs`'s own pool-catalog consumer tests, which
use their own fixture `EngineFacts` rather than the live corpus and are therefore unaffected by which real
records now populate the catalog):
```bash
cargo test --locked --bin v06_work_inventory -- pool
```
```
test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 340 filtered out
```

**Pre-existing, unrelated red confirmed NOT caused by this cycle** — a full `--bin v06_work_inventory` run
surfaced two failures (`e14_harness_tests::a_key_two_books_share_grounds_only_the_book_whose_corpus_was_
read`, `race_trait_grounding_tests::the_t2b_residual_population_is_never_ingested_not_a_matcher_miss`),
neither touching `class_feature`/pool anything. Verified pre-existing by temporarily restoring the
pre-cycle `class_feature_pool_catalog.rs` (`git show HEAD:<path>`, never `git stash`) and re-running: BOTH
fail identically against the unmodified HEAD. Restored this cycle's file immediately after
(`git status --porcelain` confirmed only the two intended files touched throughout). Not this cycle's
territory (equipment probe / T2b race-trait census); left for the unowned-reds lane per the dispatch's own
territory note.

## 4. Sweep (§3)

```bash
grep -rn "REGISTERED_POOL_GROUPS\b" tests/ src/ scripts/ apps/ 2>/dev/null | grep -v class_feature_pool_catalog.rs
```
No hit outside the one file this cycle touched — the constant is documented as no-longer-consulted for
filtering but referenced nowhere else that would need updating.
```bash
grep -rn "71 magnitude-bearing\|~1,913\|~16,350\|~6,131\|2 groups" tests/ src/ scripts/ apps/ docs/release/SD-32-compute-library-and-cause-closure/*.md 2>/dev/null
```
Only the prior cycle's own receipt and this cycle's kanban entry cite those figures; no test/src pin
depends on them as a hard assertion.

## 5. Scope discipline

Did not attempt: closing the 5,981-record numeric-magnitude residual (real compute functions, sized in §1,
next cycle's scope). Did not touch `data/corpus/**/monster_ability/**`, `scripts/transcribe_monster_
tables.py`, `monster_chassis.rs` (sibling `monster_ability` lane); `tests/sd26_cache_core_rulebook.rs`,
`tests/pi_screening_regeneration_round_trip.rs` (sibling unowned-reds lane); `kanban.md` row 15 or
`progress.md` (sibling closure-readiness-audit lane, coordinated by leaving row 11 alone otherwise). Did
not touch `data/corpus/**` at all (`git status --porcelain -- data/corpus` — 0 changes throughout). Did not
fix the `cache_gen::class_feature::generate` root cause of the `[not implemented]` marker (ingest
territory, out of this cycle's file scope) — mitigated at this module's own boundary instead, root cause
named as a discovery-forward.

`df -h /`: reported in the dispatch's final report.
