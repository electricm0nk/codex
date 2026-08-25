# Cycle t12-class-feature-pool-population, cycle 3 — Gate 3 (closure invariant) / row 18 (`epic-8-pool-shaped-class-features`)

- **Card ID:** row 18 (`epic-8-pool-shaped-class-features`), governed by `decisions.md §27b`
  (*"EVERYTHING"*). Scope this cycle, per dispatch: **Deliverable 1 is a corrected number, not
  more records** — audit `ALREADY_MODELED_ELSEWHERE` against what the engine genuinely already
  grounds, re-derive every registered pool's real group name against the corpus, then close groups
  with the resolver cycle 2 built.
- **Base:** verified against `PIN=51b541a3e746796df676db8f0c4e7b2cf35225e3` before starting
  (`git merge-base --is-ancestor "$PIN" HEAD` -> `BASE_OK`); `git rebase origin/tranche/12` was a
  no-op (HEAD already equalled `origin/tranche/12` at the pin — footgun 1 did NOT fire this cycle).
- **Oracle:** fresh worktree, empty git-ignored slot as expected. Bootstrapped via
  `scripts/fetch-pcgen-oracle.sh --dest <repo-local artifacts/corpus/operator-supplied/pcgen>`,
  confirmed present: `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`,
  `scripts/verify.sh --only preflight-oracle` -> `PASS`.
- **Files touched:**
  - `scripts/census_class_feature_pool_population.py` — added a per-record predicate
    (`WITCH_HEX_FAMILY_GROUPS`, `witch_hex_alias_target`) alongside the existing whole-group
    `ALREADY_MODELED_ELSEWHERE` set, and reports the correction as its own line in both text and
    JSON output.
  - `src/rules_core/pilot_compute/class_slayer.rs` — one new call site
    (`push_generic_pool_choice_magnitude` for Slayer Talent), purely additive alongside the
    existing hand-modelled Foil Scrutiny closure.
  - `src/rules_core/pilot_compute/mod.rs` — 5 new tests (`opponent_conditioned_tier_zero_tests`);
    one doc-comment correction (5,981 -> ~5,927, citing this cycle's re-derivation) on
    `resolve_pool_member_sole_magnitude`'s own module-level doc block. No existing function's
    behaviour changed.
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 18 only (cycle field
    1 -> 2, Notes appended), verified still 9-pipe-delimited (10 fields) and the file's own line
    count unchanged (87) before and after.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — scoped to this cycle's own touched files
  (`git diff --unified=0 -- scripts/census_class_feature_pool_population.py
  src/rules_core/pilot_compute/mod.rs src/rules_core/pilot_compute/class_slayer.rs
  docs/release/SD-32-compute-library-and-cause-closure/kanban.md`), `grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'` -> 0 hits.
- **Wired-integration audit result:** `OK_NO_TOKENS` — same diff scope, `grep -inE
  "STUB|MOCK|placeholder|not.?yet.?implemented|todo|fixme|hack"` -> the only matches are the
  pre-existing kanban prose discussing stub *policy* ("No stubs;..."), not an actual stub in new
  code; this cycle's own new Rust/Python is a real evaluator wired to a real chassis call site with
  real tests, not a scaffold.
- **PI audit:** `python3 -c "import pi_scrub; pi_scrub.normalized_term_hits(...)"` against this
  cycle's full diff text -> `[]` (0 hits). No corpus record name, blacklist term, or PI item name
  appears in any receipt, test name, test constant, kanban row, or this file (`§24b`-2).
  `data/corpus/**` untouched throughout (`git status --porcelain -- data/corpus` — 0 changes).
- **Acceptance criterion (this cycle's stated scope):** correct the census's over-count per the
  Witch Hex finding, re-derive real group names for the registered pools before assuming coverage,
  then close groups at scale using cycle 2's resolver — reported as `§16` instrument correction
  separately from closure.
- **Status:** in-progress — genuine, multi-cycle epic, unchanged from cycle 1/2's own framing.
  1 more group closed (partially — see §3 below); the corrected number is this cycle's primary,
  load-bearing deliverable.

---

## 1. The census correction (`§16`, Deliverable 1)

**Finding, re-verified live:** cycle 2 found Witch Hex's 51-of-53 base hexes already ground through
a pre-existing shared function (`witch_hex_save_dc`, unconditionally pushed once per Witch,
grounding `10 + WitchHexStat + WitchHexAbilityLVL/2`) never registered in the census's
`ALREADY_MODELED_ELSEWHERE` set. This cycle re-derived the exact scope of that gap, precisely,
across all three Witch-Hex-family census groups (`Witch Hex`, `Witch Major Hex`, `Witch Grand
Hex` — the census splits what the doc comment calls "the 53 base hexes" across these three real
corpus group names):

```bash
python3 -c "
import json,glob
groups={'Witch Hex','Witch Major Hex','Witch Grand Hex'}
covered=0; not_covered=[]
for f in glob.glob('data/corpus/*/class_feature/**/*.json', recursive=True):
    d=json.load(open(f))
    k=d.get('data',{}).get('key','')
    if not isinstance(k,str) or ' ~ ' not in k: continue
    g=k.split(' ~ ',1)[0]
    if g not in groups: continue
    toks=d['data'].get('raw_tokens') or []
    numeric = any(t.get('key') in ('BONUS','DEFINE') for t in toks if isinstance(t,dict))
    if not numeric: continue
    has_alias = any(t.get('key')=='BONUS' and isinstance(t.get('value'),str)
                     and t['value'].startswith('VAR|WitchHexDC_')
                     and t['value'].split('|')[2]=='WitchHexDC' for t in toks)
    if has_alias: covered += 1
    else: not_covered.append(k)
print('covered', covered, 'not_covered', not_covered)
"
```
Result: **54 of 58** numeric-magnitude records across the three groups carry the
`BONUS:VAR|WitchHexDC_<Name>|WitchHexDC` alias — the exact per-hex facet of the one shared DC
formula `witch_hex_save_dc` already grounds unconditionally. This includes Cauldron and Flight,
whose OWN extra bonuses (Craft (Alchemy), Swim) are ALSO already separately hand-grounded in
`ground_or_block_witch_class_features` (confirmed by direct read of that function). The remaining
**4** (`Bouda's Eye`, `Enemy Ground`, `Mud Witch`, `No Place Like Home`) carry their own distinct
`DEFINE`-based magnitude with no such alias and are correctly left in the residual.

**Why a per-record predicate, not a whole-group exclusion:** the existing `ALREADY_MODELED_ELSEWHERE`
mechanism excludes an entire group. Applying it to `Witch Hex`/`Witch Major Hex`/`Witch Grand Hex`
wholesale would have wrongly dropped those 4 genuinely-unclosed records from the residual — the
opposite failure from the one being fixed. `scripts/census_class_feature_pool_population.py` now
carries a `WITCH_HEX_FAMILY_GROUPS` set plus a `witch_hex_alias_target(raw_tokens)` predicate,
checked per-record before a Witch-Hex-family record is added to the residual.

**Corrected residual, re-run live:**

```bash
python3 scripts/census_class_feature_pool_population.py
```
```
class_feature pool-shaped population census (decisions.md §17/§17a)
  files scanned                                  18076
  malformed JSON                                      0
  distinct ' ~ '-group-qualified names             1913
  total group-qualified records                   16350
  catalog-servable text-only (no engine token)     7423
  any engine-effect token (ABILITY/CSKILL/SELECT/AUTO/SAB/BONUS/DEFINE/ADD/SPELLS/DR/SR)
    records                                        8927
  numeric magnitude (BONUS/DEFINE var math, or %N-substituted DESC)
    records                                        6306
  already modeled elsewhere (groups: Domain Power, Inquisitor Domain, Rage Power, Rogue Talent)
    records                                         596
    of which numeric-magnitude                      325
  Witch Hex family alias-covered (witch_hex_save_dc, T12 cycle-3 fix)
    groups: Witch Grand Hex, Witch Hex, Witch Major Hex
    numeric-magnitude records covered                54
  RESIDUAL numeric-magnitude needing compute       5927
```

**Corrected residual: 5,981 -> 5,927** (54 fewer, exactly matching the 54 alias-covered records
found above; cycle 2's own 3 closures are ALSO subtracted already, since they were closed against
the live corpus and the census's `numeric_magnitude` total is measured, not decremented by hand).

## 2. Group-name re-derivation (`§17a` — validate before trusting the registry)

Cycle 2 found ONE mismatch (`CLASS_FEATURE_POOLS`'s `"Spirit"` matching zero corpus records). This
cycle checked the pattern against every other pool named in the dispatch's residual list and found
it generalizes far beyond Shaman — reported here as load-bearing discovery-forward, per the
dispatch's own instruction, rather than left for a future cycle to re-find:

| Registered name (`v06_work_inventory.rs::CLASS_FEATURE_POOLS`) | Real corpus group(s) | Records |
|---|---|---:|
| `Spirit` (Shaman) | **`Shaman Spirit Hex`** (the granted abilities) — `Shaman Spirit`/`Shaman Wandering Spirit` are 12-record per-spirit HEADER groups, a different thing | 59 |
| `Mystery` (Oracle) | **No such group.** ~24 separate per-mystery groups instead (`Ancestor Mystery`, `Apocalypse Mystery`, `Battle Mystery`, ... 11 records each) | 0 exact |
| `Curse` (Oracle) | **No such group.** `Curse Subdomain` (1), `Curse of Brittle Bones` (1), `Dual-Cursed`(2)/`Dual-Cursed Oracle`(13) exist but are not "Curse" | 0 exact |
| `Revelation` (Oracle) | **No such group** at all — revelations live inside each per-mystery group above | 0 exact |
| `Order` (Cavalier) | **`Cavalier Order`** (7 records) exists but is a ZERO-magnitude ABILITY-only dispatcher layer; the real granted-ability groups are ~18 separate per-order groups (`Order of the Lion`, `Order of the Beast`, `Order of the Paw`, ...) | 7 (0 numeric) + ~18 groups |
| `Investigator Talent` | Real prefix matches only **2** records (`Rapid Reload`, `Extra Grit`); the real 118-record pool is named just `Investigator` | 2 exact / 118 real |
| `Blessing` (Warpriest) | **No exact `Warpriest Blessing` group.** Real groups: `Warpriest` (18), `Warpriest Bonus Feat` (432) — neither is "Blessing" | 0 exact |
| `Evolution` (Summoner) | **No exact `Summoner Evolution` group.** Real groups: `Summoner` (19), `Unchained Summoner` (17), `Spirit Summoner` (3), `Master Summoner` (2) | 0 exact |
| `Arcane School` / `Focused Arcane School` (Wizard) | `Focused Arcane School` matches (17); plain `Arcane School` does not exist as a group | 17 / 0 |
| `Domain` (Cleric) | **No exact `Domain` group.** Dozens of per-domain groups (`Air Domain`, `Anger Domain`, ... mostly 2-3 records each) plus `Cleric Domain` (22), `Druid Domain` (110), `Sorcerer Domain` (22), `Core Domain` (110), `Domain Power` (172, already excluded) | 0 exact |
| `Grand Discovery` / `Advanced Talents` | Neither matches any real group prefix | 0 exact |
| `Slayer Talent` | **Exact match** — the real corpus prefix genuinely is `Slayer Talent` | 46 (confirmed, wired below) |
| `Hunter Animal Focus` | **Exact match** | 21 (not wired this cycle — see §4) |

**Consequence for future cycles:** most of the ~55-group residual list from cycle 2's next-cycle
plan needs its real per-book/per-subtype group name re-derived — often several real groups per one
registered entry — before `push_generic_pool_choice_magnitude` can even be called for it. This is a
materially bigger fan-out than "wire the other 12 registered pools" implied. `Cavalier Order`
specifically needs a TWO-LEVEL walk (the zero-magnitude `Cavalier Order ~ *` dispatcher names each
real per-order group by text in its own `ABILITY` tokens, e.g. `Order of the Beast ~ Edicts`) that
the current `resolve_pool_selection_corpus_key`/`push_generic_pool_choice_magnitude` pair does not
support without a selection-to-order lookup this cycle did not build (scoped out, named here rather
than silently deferred).

## 3. Group closed at scale with cycle 2's resolver (`§17`, not a second mechanism)

**Slayer Talent** (46 real corpus records, exact-prefix match confirmed above) — wired additively in
`class_slayer.rs::ground_or_block_slayer_class_features`, alongside the existing hand-modelled Foil
Scrutiny closure, using the SAME `push_generic_pool_choice_magnitude` cycle 2 built. No second
resolver written.

**A second, narrower instance of the group-name-mismatch class found live while wiring this:** most
of Slayer Talent's level/DC-scaled members (`Assassinate`, `Slowing Strike`, `Hard to Fool`, ...)
chain through a class-specific level variable `SlayerTalentLVL`, bound only by the real corpus
header record `Slayer ~ Slayer Talents` — **plural**. This call site's `pool_group` argument,
`"Slayer Talent"` (singular), is the CORRECT member-key prefix (`"Slayer Talent ~ <name>"` is the
real member shape) but does NOT match the header suffix `resolve_pool_member_sole_magnitude`
constructs (`format!("{} ~ {pool_group}", record.class)` = `"Slayer ~ Slayer Talent"`, singular —
never matches `"Slayer ~ Slayer Talents"`). The header merge silently finds nothing, so
`SlayerTalentLVL` is unbound in these formulas.

**This does NOT fabricate a wrong value.** `PcgenFormulaEvaluator::evaluate` errors on an unbound
identifier (not a silent 0), so `resolve_pcgen_var_chain`'s fixed-point loop never inserts a value
for any target whose formula needs `SlayerTalentLVL`, and `resolve_pool_member_sole_magnitude`
correctly returns `None` for the whole record. Proven live, not assumed:

```bash
cargo test --locked --lib -- rules_core::pilot_compute::opponent_conditioned_tier_zero_tests::slayer_generic_resolver_refuses_rather_than_fabricates_a_missing_class_level_binding
```
```
test result: ok. 1 passed
```
Asserts `class_feature.acg.slayer.talent.generic.assassinate.slayerassassinatedc` is `None`, not a
DC computed as if `SlayerTalentLVL` were 0 (which would read `10+(0/2)+INT = 10+INT`, a real,
wrong, silently-plausible number this proof rules out).

**What DOES close:** flat/constant-magnitude Slayer Talent members needing no class-level binding
at all. `Slayer Talent ~ Deadly Range Output` (`BONUS:VAR|DeadlyRangeDistance|30`, a self-contained
constant) now reaches `compute_pilot_base_chassis` -> `compute_class_chassis`:

```bash
cargo test --locked --lib -- rules_core::pilot_compute::opponent_conditioned_tier_zero_tests::slayer_deadly_range_output_talent_resolves_generically_as_a_flat_constant
```
```
test result: ok. 1 passed
```

Fixing the header-suffix mismatch (so the level/DC-scaled majority of Slayer Talent also closes) is
named as real, scoped follow-on work for a future cycle, not attempted here — `push_generic_pool_
choice_magnitude`'s signature has no separate header-suffix parameter, and changing that signature
touches its only other call site (Alchemist Discovery), which was out of this cycle's stated scope.

## 4. Tests, RED->GREEN (`§1a`)

5 new tests, `rules_core::pilot_compute::opponent_conditioned_tier_zero_tests`:

```bash
cargo test --locked --lib -- rules_core::pilot_compute::opponent_conditioned_tier_zero_tests::slayer
```
```
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured
```

- `slayer_deadly_range_output_talent_resolves_generically_as_a_flat_constant` — the real closure.
- `slayer_generic_resolver_refuses_rather_than_fabricates_a_missing_class_level_binding` — the
  safety proof for §3's finding.
- `an_invented_slayer_talent_selection_never_grounds_a_generic_magnitude` — mirrors the Rage Power
  precedent.
- `slayer_generic_talent_resolver_stays_silent_below_the_talent_grant_level` — mirrors Alchemist
  Discovery's own level-gate proof (talents start at 2nd level).

**Mutation altitude 1 (chassis call site):** temporarily wrapped the `push_generic_pool_choice_
magnitude(...)` call in `class_slayer.rs` in `if false { ... }` -> re-ran
`slayer_deadly_range_output_talent_resolves_generically_as_a_flat_constant` ->
```
thread '...' panicked: assertion `left == right` failed
  left: None
 right: Some(30)
```
RED confirmed. Reverted (`if false { ... }` removed, call site restored verbatim); re-ran the 20
scoped Slayer/dispatch tests -> green again.

**Mutation altitude 2 (library logic):** the safety property this cycle relies on (refuse on an
unbound identifier rather than default to 0) is the SAME `PcgenFormulaEvaluator`/`resolve_pcgen_var_
chain` machinery cycle 2 already mutation-proved (its own multi-terminal-refusal mutation, reverted,
receipt §4) — this cycle did not modify that logic, so it is not re-proven here; citing cycle 2's
proof rather than duplicating it.

**Regression check**, the two touched files' full existing suites plus the shared pool-catalog/
grant-consumer suites:
```bash
cargo test --locked --lib -- rules_core::pilot_compute::opponent_conditioned_tier_zero_tests::slayer \
  rules_core::pilot_compute::slayer_dispatch_widening_safety_tests \
  rules_core::pilot_compute::spellcasting_shaped_class_closure_tests \
  class_feature_pool_catalog class_feature_grant_consumer
```
```
test result: ok. 80 passed; 0 failed; 0 ignored; 0 measured; 2662 filtered out
```
No pre-existing test's behaviour changed.

## 5. Sweep (`§3`)

```bash
grep -rn "5,981\|5981\b" docs/release/SD-32-compute-library-and-cause-closure/*.md tests/ src/ scripts/ apps/ 2>/dev/null
```
Only cycle 2's OWN prior kanban entry and receipt still cite 5,981 (historical record of what cycle
2 reported at the time — left unchanged, correct as a record of cycle 2's own derivation); this
cycle's new kanban addendum and this receipt both cite the corrected 5,927. No test or source file
pins 5,981 as an assertion.

```bash
grep -rn "resolve_pool_member_sole_magnitude\|push_generic_pool_choice_magnitude\|WITCH_HEX_FAMILY_GROUPS\|witch_hex_alias_target" tests/ src/ scripts/ apps/ 2>/dev/null | grep -v "pilot_compute/mod.rs\|pilot_compute/class_slayer.rs\|census_class_feature_pool_population.py"
```
No hit outside the three files this cycle touched.

## 6. Scope discipline

Did not attempt: fixing the header-suffix mismatch found in §3 (real, scoped follow-on, named
above); wiring Hunter Animal Focus (exact-name match confirmed, but its existing hand-modelled
function is activation-gated with an enforced per-day budget and a hard claim-blocking posture for
every non-Bull focus — extending it safely needs care this cycle's remaining scope did not allow,
named rather than risked); Cavalier Order's two-level walk (§2); Oracle's ~24 per-mystery groups and
Cleric's dozens of per-domain groups (§2 — real work, but re-deriving `push_generic_pool_choice_
magnitude`'s call convention for a one-registered-entry-to-many-real-groups shape is itself a
sub-task a future cycle should scope, not something to rush here); rows 11/15 (left `in-progress`,
untouched); the `cache_gen::class_feature::generate` `[not implemented]`-marker root cause (cycle
1's own discovery-forward, still unaddressed, out of this cycle's file territory); `apps/desktop`'s
row 19 lane, `bestiary_4/monster_ability`'s `DESC-PI-SHIPPED` lane — both sibling territory, not
touched. `data/corpus/**` untouched throughout.

`df -h /`: reported in the dispatch's final report.
