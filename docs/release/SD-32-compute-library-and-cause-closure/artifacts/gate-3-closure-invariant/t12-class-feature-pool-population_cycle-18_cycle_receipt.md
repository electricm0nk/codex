# Cycle t12-class-feature-pool-population, cycle 18 — Gate 3 (closure invariant) / row 18 (`epic-8-pool-shaped-class-features`)

- **Card ID:** row 18 (`epic-8-pool-shaped-class-features`), governed by `decisions.md §27b`
  (*"EVERYTHING"*). Dispatched scope: establish the honest remaining shape of the census's own
  103 never-classified-as-a-whole unresolved groups (94/197 resolve), report the three counts,
  then close whatever the real bug bucket contains, largest shape first.
- **Base:** worktree started on `1846190eef` (footgun 4 — a stale lineage, `origin/tranche/12`'s
  own PR #374 merge commit, NOT a descendant of the pinned `PIN=163dc5f3f05e80dc734dfb7419d7b59258cea3f7`).
  Fixed: `git reset --hard "$PIN"` — `origin/tranche/12`'s tip was exactly `$PIN` at reset time
  (row 20's own cycle-13 closure commit), so no rebase was needed and no sibling lane had landed
  on row 18's files since.
- **Oracle:** worktree's oracle slot was empty (git-ignored) as expected for a fresh worktree.
  Bootstrapped via `scripts/fetch-pcgen-oracle.sh --dest
  <worktree>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen`
  — `pcgen-oracle: OK 7f818006e371188e5717fd18d74d18a420747fc6`, pin confirmed. Re-ran
  `scripts/verify.sh --only preflight-oracle` explicitly with both env vars exported — PASS
  against the real, non-default slot.

## 1. The honest denominator — classified, not assumed (`§17a`)

Re-derived the brief's own headline figure fresh (not trusted): `pool_group_closure_census_
across_all_six_pools_both_resolvers` printed 94/197 resolved (Sorcerer 31/53, Bloodrager 6/12,
Cleric 35/72, Shaman 12/14, Warpriest 8/37, Cavalier 2/9) — confirmed exactly as handed. **103
groups had never been classified as a whole.**

Wrote a diagnostic (`cycle18_classify_every_unresolved_group_across_all_six_pools`, printed via
`--nocapture`, then **removed before commit**, same methodology cycle 14 used for its own
36-group finding). For every one of the 103 unresolved groups, walked its real member records
(the row's own live `ClassFeatureRecordTokens` table, not a fresh Python re-derivation) and
classified the GROUP by whether any member carries (a) a real `BONUS:VAR`/`DEFINE` token, (b) a
`%N`-substituted `DESC:` formula with an empty `bonus_vars`, or (c) neither:

```
not_numeric=46 refused_bonus_var=39 refused_desc_formula=9   (BEFORE this cycle's own fixes: 46/48/9)
```

**Three counts, as demanded:**

1. **46 groups — not numeric-magnitude work at all** (`§7`/`§16` reclassification, report the
   count, never claimed as units closed). Includes cycle 14's own already-reported 36 (29
   Warpriest + 7 Cavalier) plus 10 more this cycle's own wider pass found: 3 Sorcerer (Anarchic,
   Groveborn, Primal Bloodline), 6 Cleric (Core, Execution, Fate, Politics, Secrets, Seduction
   Domain — plus, after this cycle's own fixes below, the newly-visible bare "Cleric Domain"
   header record itself), 1 Shaman (Shaman Wandering Spirit). Every one of these groups' member
   records carries only `KEY`/`CATEGORY`/`TYPE`/`DESC` prose with no `BONUS`/`DEFINE` and no `%N`
   reference — the SAME canonical zero-magnitude shape `description_completion.rs`'s own module
   doc names (Deflect Arrows). Confirmed directly against `scripts/census_class_feature_pool_
   population.py`'s own `NUMERIC_MAGNITUDE_KEYS` gate — these groups were never counted in that
   script's own `residual numeric-magnitude needing compute` population either.
2. **39 groups (was 48 before this cycle's own fix) — a real `BONUS:VAR` chain this engine
   refuses.** Named below by shape, 10 closed this cycle.
3. **9 groups — a real `%N` desc formula the description-formula resolver alone cannot reach.**
   Named below; not forced this cycle (a genuinely different, smaller gap in a sibling resolver).

## 2. Closed: two independent, oracle-consistent engine gaps in Cleric Domain (largest shape)

Read every one of the 23 Cleric Domain bucket-2 refusals by hand (`Anger`, `Aquatic`, `Arctic`,
`Conversion`, `Eagle`, `Fervor`, `Forbidden Rites`, `Frog`, `Illumination`, `Imprisonment`,
`Jungle`, `Justice`, `Monkey`, `Mountain`, `Order`, `Persistence`, `Plains`, `Scalykind`,
`Serpent`, `Swamp`, `Truth`, `Vengeance`, `Zeal`) against the real corpus, not by pattern-matching
Bloodrager's precedent blindly.

**Gap 1 — a `domain`-kind header record this codebase never read.** `pool_header_record_by_
normalized_suffix` already merges FOUR real per-domain header shapes (bare `"<X> Domain"`
`class_feature` key, `"Cleric Domain ~ <X>"`, the bare plural `"Domains"` base, `"<class> ~
<registered_name>"`). A FIFTH shape exists and was never read: a `domain`-kind corpus record
(`data/corpus/<book>/domain/*.json`, e.g. `data/corpus/ultimate_magic/domain/cave.json`), keyed
by the domain's own BARE name with no `" Domain"` suffix (`"Cave"`, not `"Cave Domain"`).
Confirmed live by direct inspection: this record type ALREADY carries the real
`BONUS:VAR|Domain<X>LVL|DomainLVL`, `BONUS:VAR|Domain<X>DC|10+(Domain<X>LVL/2)+WIS`,
`BONUS:VAR|Domain<X>Times|DomainPowerTimes` chain every one of that domain's `class_feature`
member records needs — it simply lived in a directory no existing table ever walked.

Verified this convention is universal (not a one-domain coincidence) before implementing:
`grep -rhE 'Domain[A-Za-z]+LVL.DomainLVL' data/corpus --include="*.json"` returns 299 hits across
every domain in the corpus, every single one either `DomainLVL` (bare) or `DomainLVL-2|TYPE=Domain`
(the subdomain variant) — no third form exists anywhere.

Added `class_feature_grant_consumer::domain_kind_bonus_vars_any_record()` (mirrors `class_
record_bonus_vars`'s own shape one dir level up — walks `<book>/domain/*.json`, merges
`BONUS:VAR` targets across books, `.or_insert` per target name, never overwriting). Wired into
`pool_header_record_by_normalized_suffix` as a new clause, scoped to `registered_name ==
Some("Domain")` and a `pool_group` ending in `" Domain"` (so no other pool family can collide
with this corpus shape).

**Gap 2 — a `description: null` gate dropping real magnitude-bearing member records.**
`class_feature_record_tokens_pre_gate_safe` (the table `resolve_pool_member_sole_magnitude`
reads) required `data.description` to be present and real, unconditionally — but a real,
invisible, purely-mechanical sub-ability (`VISIBLE:NO`) can carry `description: null` while still
carrying a real `BONUS:VAR` chain. Confirmed live: `Jungle Domain ~ Trap Sense`
(`BONUS:VAR|TrapSenseBonus|DomainJungleLVL/3`, `description: null`) was refused purely because
this table's own `description.as_str()?` gate dropped the record before its `bonus_vars` was ever
read — the EXACT shape `class_feature_bonus_vars_any_record`'s own doc comment already names and
already admits for HEADER-record lookups (`AlchemistDiscoveryLVL|AlchemistLVL`, `description:
null`), never before applied to the MEMBER-record table.

Widened: `description: null` now admitted as an empty `raw_description` (never fabricated text) —
a record with a real but BAD description (`.CLEAR`/`.CLEARALL`/a PI-redaction marker) is still
refused exactly as before; `is_real_description_value` only ever ran on a `Some` description, so
this widening changes nothing about that PI-safety gate (`§15`). Verified safe for every existing
consumer before landing: `resolved_description_for`/`resolved_description_for_formula_only_desc_
argument` both already refuse cleanly (`render_pcgen_desc_with_values("", ...).text.is_empty()`,
`desc_token_arguments("").is_empty()`) on an empty `raw_description` — no caller anywhere gains a
new, fabricated rendering; only `resolve_pool_member_sole_magnitude`'s `BONUS:VAR`-only path
(which never reads `raw_description` at all) gains real, previously-invisible records.

**Net effect, re-derived (`§17a`):** Cleric Domain `bonus_vars`-only moved 34/72 → 44/73;
combined moved 35/72 → 45/73. The 73rd group is a previously wholly-invisible group this same
widening revealed (a group whose only members all carried `description: null` was invisible to
`real_groups_owned_by`'s own tally before this cycle) — a real population correction, not a
resolver artefact; it lands in bucket 1 (not-numeric-work) per this cycle's own classification.
Newly-closed groups (verified by diffing the before/after refusal lists, not assumed): Aquatic,
Arctic, Eagle, Frog, Monkey, Plains, Serpent, Swamp (8 via gap 1 alone), plus 2 more from gap 2
(one of which, "Cleric Domain" itself, moved into bucket 1 rather than resolving). Every other
pool's own figure — Sorcerer 31/53, Bloodrager 5/12 (6/12 combined), Shaman 11/14 (12/14
combined), Warpriest 0/37 (8/37 combined), Cavalier 1/9 (2/9 combined) — UNCHANGED, re-verified.

**Both fixes mutation-proved RED then reverted:**
- Gap 1: temporarily gated the new merge clause behind `if false && ...`. Re-ran
  `pool_group_closure_census_across_all_six_pools_both_resolvers` — FAILED as expected (Cleric
  Domain reverted to 35/73, missing the domain-kind header's own +9). Restored, re-verified GREEN.
- Gap 2: temporarily reverted `description: null` to `continue` (drop the record). Re-ran the
  same census AND `the_live_scale_of_this_waves_widening_is_measured_and_pinned` — BOTH FAILED as
  expected (Cleric Domain reverted to 42/72; the corpus-wide pinned scale test reverted to its old
  36/8 `no_record_at_all`/`chain_unresolvable` split). Restored, re-verified GREEN.

**Correction filed** (`scripts/retro.py correction`, id `1787588445878-t9-onboarding-693efc`):
the corpus-wide `the_live_scale_of_this_waves_widening_is_measured_and_pinned` test's own pin
(`no_record_at_all=36, chain_unresolvable=8`) mis-stated 35 of those 36 as "no record at all" —
they DO have a real `class_feature` corpus record; only their `description` was `null`. Corrected
to `(1, 43)` with a full explanatory comment; `newly_resolved`/`already_admitted`/`class_excluded_
otherwise_resolvable` are all UNCHANGED (diffed the full `newly_resolved_examples` list).

## 3. Not closed this cycle — named by shape, not by "blocked"

**39 bonus-var refusals remain** (down from 48):
- **Sorcerer (18) + Bloodrager (6):** every remaining member chains to a per-bloodline/per-record
  identifier (`Sorcerer_<X>_BloodlinePowerNLVL`, `Sorcerer_Stormborn_BloodlineProgressionLVL`,
  ...) bound ONLY on an UNRELATED cross-class or cross-bloodline record the plain character does
  not hold (confirmed by sampling `Aerial Bloodline ~ Windcaller`: chains to
  `Sorcerer_Stormborn_BloodlinePower9LVL`, a DIFFERENT bloodline's own header, via a genuine
  PCGen "Wildblooded" archetype cross-reference). This is cycle 17's own Bloodrager finding
  (`PlayerCharacter.getVariable` sums character-wide with no class scoping, but the sum is only
  ever nonzero for a variable the character's OWN held sources actually bonus) generalizing to
  every remaining Sorcerer/Bloodrager member this cycle sampled — a genuine data/cross-reference
  gap per `§27b` point 5, not a resolver defect. Not re-verified exhaustively per-record this
  cycle (that would be its own full cycle); named here for the next one.
- **Cleric (14):** every remaining Domain (`Anger`, `Conversion`, `Fervor`, `Forbidden Rites`,
  `Illumination`, `Imprisonment`, `Justice`, `Mountain`, `Order`, `Persistence`, `Scalykind`,
  `Truth`, `Vengeance`, `Zeal`) genuinely has NO header record anywhere in the ingested corpus —
  neither the `class_feature` shapes nor the new `domain`-kind shape this cycle added (confirmed:
  `Anger`/`Conversion`/etc. have no `data/corpus/*/domain/*.json` file at all). This is the exact
  same "hard data gap" shape cycle 17 proved for Bloodrager, now confirmed for these 14 Domains
  too (per-name check run this cycle, not assumed). `Mountain` and `Jungle` DO now have a header
  (gap 1's fix reaches them) but their OWN members still refuse for the SEPARATE reasons named
  below — Mountain stays in this bucket because `Thin Air`'s own record carries TWO independent
  terminal targets (`ThinAirRounds`, `ThinAirRange`) on one record, correctly refused rather than
  guessed (the module's own documented "genuinely novel, two independent magnitudes" case).
- **Shaman (1):** `Shaman Spirit` is the bare tracker header record itself (`"Shaman ~ Spirit"`),
  matched by `real_groups_owned_by`'s naming-shape filter as if it were a real spirit choice — it
  is not a player-selectable group at all, a pre-existing naming-shape false-positive this cycle
  did not touch (named for a future cycle, not this one's scope to fix).

**9 desc-formula refusals remain, unchanged this cycle:** `resolved_description_for_formula_only_
desc_argument` (`pcgen_desc.rs`'s own consumer) evaluates a member's `%N` arguments against ONLY
ability modifiers + the owning class's own level — it never consults a pool's header chain at
all. Confirmed live: `Mountain Domain ~ Foothold`'s `%1` argument is the bare identifier
`DomainMountainTimes`, now resolvable via gap 1's new header merge through the OTHER resolver, but
the desc-formula resolver has no header-merge step of its own to find it. A real, separate,
smaller engine gap (wiring a header-chain merge into the desc-formula resolver) — named here for
a future cycle, not forced this one (would touch `pcgen_desc.rs`'s own consumer contract, a
larger, more careful change than this cycle's remaining scope warranted).

## 4. Full re-census, honest figures re-run (`§17a`)

```bash
cargo test --locked --lib -- \
  rules_core::pilot_compute::generic_pool_group_selection_wiring_tests::pool_group_closure_census_across_all_six_pools_both_resolvers \
  --nocapture
```
```
Sorcerer Bloodline: bonus_vars=31/53, combined(bonus_vars OR desc_formula)=31/53
Bloodrager Bloodline: bonus_vars=5/12, combined(bonus_vars OR desc_formula)=6/12
Cleric Domain: bonus_vars=44/73, combined(bonus_vars OR desc_formula)=45/73
Shaman Spirit: bonus_vars=11/14, combined(bonus_vars OR desc_formula)=12/14
Warpriest Blessing: bonus_vars=0/37, combined(bonus_vars OR desc_formula)=8/37
Cavalier Order: bonus_vars=1/9, combined(bonus_vars OR desc_formula)=2/9

test ... ok
```

## 5. Tests, full re-run

```bash
cargo test --locked --lib -- rules_core::pilot_compute::
```
```
test result: ok. 974 passed; 0 failed; 0 ignored; 0 measured; 1843 filtered out
```
(up from cycle 17's 973 — net +1: the removed diagnostic test is not counted; the real delta is
the fixture/scale-test rewiring, no new permanent test added this cycle beyond the two pinned
assertion updates.)

```bash
cargo test --locked --lib -- rules_core::derived_evaluator_fixture_check::
```
```
test result: ok. 121 passed; 0 failed; 0 ignored; 0 measured; 2696 filtered out
```
(unchanged — no fixture-checked evaluated value touched this cycle)

```bash
cargo test --locked --lib -- hunter
```
```
test result: ok. 45 passed; 0 failed; 0 ignored; 0 measured; 2772 filtered out
```
(unchanged from cycle 17)

## 6. Not attempted / not applicable this cycle

- The 39 bonus-var and 9 desc-formula refusals named in §3 — real, named, remaining work, not
  forced this cycle.
- Rows 11/15 left as found (`in-progress`/`complete`), untouched. `apps/desktop`'s row 19/20
  lanes not touched. `data/corpus/**` untouched throughout (`git status --porcelain --
  data/corpus` — 0 changes).
- `oracle_dispatch_widening_safety_tests::a_mystery_pick_alone_grounds_no_tier_one_revelation` —
  untouched, still green (`cargo test --locked --lib -- oracle_dispatch_widening_safety_tests` —
  48 passed, unchanged).

## 7. Audits

- **Identifier audit** (`grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'`, scoped to
  `git diff --unified=0` of `src/rules_core/pilot_compute/mod.rs`,
  `src/rules_core/pilot_compute/class_feature_grant_consumer.rs`, and the kanban diff, `SD-32`
  self-references excluded): `OK_NO_BUNDLE_TAGS` — 0 hits.
- **Wired-integration audit** (`grep -inE "STUB|MOCK|placeholder|not.?yet.?implemented|todo|
  fixme|hack"`, same code scope): `OK_NO_TOKENS` — 0 hits.
- **PI audit**: `pi_scrub.normalized_term_hits(...)` (imported via `scripts/pi_scrub.py`, never
  copied), scoped to `git diff --unified=0` of the code diff AND the kanban diff separately →
  `[]` (0 hits) each. `data/corpus/**` untouched throughout.

## 8. Kanban

`docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 18 only (cycle field 17 →
18, Notes prepended). Verified structurally after editing (backtick-aware parser): 21 distinct
`^| N |` rows (22 including the header row), 0 duplicate ids, row 18 parses to 9 backtick-aware
raw pipe-split fields (7 real columns) before and after. Rows 11 (`in-progress`) / 15
(`complete`) confirmed untouched (`git diff --stat` shows exactly 1 line changed in the whole
file, the row-18 line). Status stays `in-progress` — real, named remaining scope exists (39 + 9
refusals across the six pools, both named by shape above).

## 9. `df -h /`

```bash
df -h /
```
```
Filesystem      Size  Used Avail Use% Mounted on
/dev/sda1       968G  515G  454G  54% /
```
