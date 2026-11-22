---
canonical: true
wave: 30
lane: 4 (sweep residuals)
purpose: >
  Closes or advances the sweeps `todo/sweeps.md` left open, with corpus-wide counts and
  reproducible commands for each. Banks nothing: `docs/work-inventory.json` confirmed
  byte-identical before and after (md5 `d64ddfc677fd1683f5b7638889a25c54`, unchanged
  throughout, including through one live binary run that was reverted before this line was
  written).
---

# VISIBILITY — sweeps residuals (wave 30, lane 4)

## 0. What this lane covered, in ROI order

| Sweep | Verdict this wave | Headline |
|---|---|---|
| S6 residual | **PARTIAL → materially advanced, one real defect confirmed** | 29 of "~30" binaries counted; 12 of 29 individually assessed; 1 genuinely NEW, live-reproduced self-erasure instance found (`gen_advanced_race_guide`, spell+equipment); 1 lead investigated and REFUTED (`gen_cache_beastiary`); the shape's largest-population instance identified but not live-tested (`gen_core_rulebook_cache`, `gen_companion_book`) |
| S2 | **race/monster stand at wave-28's 0; companion now CLOSED at 0; equipment reasoned N/A-by-architecture** | Companion has its own registry + locking test, already proven closed |
| S9 | **class sub-question re-confirmed 0/34 beyond Ninja/Samurai; corpus-wide question still open** | The one shared class-scoped table (`CLASS_WEAPON_PROFICIENCIES`) is exactly 32/34, no further gap there; non-class kinds not reached |
| S7 | **narrowed sub-question CLOSED at 0** | 0 of 494 `assert_eq!(_.len(),N)` sites sit inside a production `ground_*`/`compute_*`/`classify_*`-named function in a content-blind way |
| S8 | **re-derived, holds** | 11,502 non-null anchor reproduces exactly; None-count drifted 979→915, flagged not investigated |

Every number below names the command that produced it. No lane wrote to `data/corpus` or
`docs/work-inventory.json` in the final state — one binary was run live to prove a defect (§1.4),
then reverted with `git checkout --`, confirmed by `git status --short` returning 0 lines.

---

## 1. S6 residual — the Rust binary population

### 1.1 Population, exact command

```
ls src/bin/ | grep -E '^(cache_gen|enrich_|gen_|ingest_)'
```

Returns **29** files (there is no standalone `cache_gen` binary — it is a module,
`src/rules_core/cache_gen/`, invoked by several of the `gen_*` binaries below). The dispatch
brief's "roughly 30" is the same population; 29 is the exact count as of `3627601f1`
(re-counted twice, by `wc -l` on the same command, to catch a miscount before filing — an
earlier draft of this document said 27 and was wrong by 2, caught on its own re-check rather
than downstream).

```
enrich_companion_raw_tokens   enrich_equipment_raw_tokens   enrich_monster_ability_raw_tokens
enrich_monster_raw_tokens     enrich_spell_raw_tokens
gen_book_cache                gen_cache_acg                 gen_cache_apg
gen_cache_beastiary            gen_cache_class_feature       gen_cache_equipment_gap
gen_cache_hand_authored_equipment  gen_cache_spell_lane_dump gen_cache_ultimate_equipment
gen_class_feature_grants      gen_core_rulebook_cache        gen_equipment_gap_tables
gen_feat_gap_tables
ingest_adventurers_guide_spells  ingest_apg_race_traits      ingest_class_spell_levels_arg
ingest_inner_sea_gods_spells  ingest_occult_adventures_spells ingest_pu_classes
ingest_race_traits            ingest_races                  ingest_ultimate_combat_spells
ingest_ultimate_magic_spells  ingest_ultimate_wilderness_spells
```

**Method, stated up front, because it constrains how much confidence each verdict below
carries:** the self-erasure shape (wave 15/S6, D7) is "selects/writes by a status or table
membership that the write step itself can silently drop, either by rebuilding the whole
document from scratch (fixture generators) or by unconditionally overwriting a directory whose
sibling generator, or a downstream enrichment pass, has since added content the writer's own
typed struct cannot represent." For each binary I read the write path for: (a) does it
`remove_dir_all`/rebuild a directory unconditionally, and (b) does anything else — a sibling
generator or an `enrich_*` pass — write into the same directory afterward, in a shape this
writer's own struct would drop. Where the static read left real doubt and a safe, cleanly
revertible test was possible, I ran it. Per the wave's own instruction, I did **not** run
anything I could not cleanly `git checkout --` afterward, and I did not touch `core_rulebook`
live (see §1.5) given its size and centrality.

### 1.2 The 5 `enrich_*` binaries — SAFE BY CONSTRUCTION, all 5, confirmed

All five (`enrich_companion_raw_tokens`, `enrich_equipment_raw_tokens`,
`enrich_monster_ability_raw_tokens`, `enrich_monster_raw_tokens`, `enrich_spell_raw_tokens`)
share one architecture, read directly from `enrich_equipment_raw_tokens.rs::enrich_one`:

1. `fs::read_dir` walks the **existing on-disk files** — there is no hardcoded record table to
   rebuild from, so there is nothing to silently omit.
2. Each file is parsed as a generic `serde_json::Value`, not a typed struct — a field this
   binary doesn't know about cannot be dropped by (de)serialization.
3. An explicit idempotency guard (`Outcome::AlreadyEnriched`) skips any record that already
   carries `raw_tokens`/`raw_bonus_chains` — a second run cannot re-derive-and-lose anything,
   it does nothing.
4. `fs::write` targets the SAME path it read, one file at a time. The only `remove_dir_all`
   calls in any of the 5 files are inside `#[cfg(test)]` sandbox teardown, never in `main()`.

This is the architecturally safe shape wave 28's S5 census already validated for PI-screening
purposes; the same structural facts make it independently immune to S6. **Closed for all 5, no
further action.**

### 1.3 `gen_book_cache` — the one binary with 4 different generator functions, checked individually

```
grep -n "^fn gen_" src/bin/gen_book_cache.rs
```

```
616:fn gen_pathfinder_unchained()
805:fn gen_advanced_race_guide()
1162:fn gen_monster_book(spec: &MonsterBookSpec)
1549:fn gen_companion_book(spec: &CompanionBookSpec)
```

| Function | Kinds written | `remove_dir_all` scope | Exists-guard? | Verdict |
|---|---|---|---|---|
| `gen_pathfinder_unchained` | feat, equipment | `for sub in ["feat","equipment"]`, scoped to this book only | n/a — disjoint from `ingest_pu_classes.rs`'s `class`/`class_feature` scope in the same book (verified: no other binary writes `pathfinder_unchained/feat` or `/equipment`) | **SAFE** |
| `gen_monster_book` | monster, monster_ability | whole subdir, but a per-key check gates deletion | **YES** — `if out_path.exists() { kept += 1; continue; }` at lines 1320 and 1392, landed as `SD31-E6-F9-005` specifically to fix this exact shape (comment at line ~1206 names the prior bug: 724 already-enriched monster/monster_ability records silently reverted to the un-enriched base shape) | **FIXED, already** |
| `gen_advanced_race_guide` | **spell, equipment, feat** | `for sub in ["spell","equipment","feat"]` unconditional, whole subdir, no per-record check | **NO** | **VULNERABLE — live-reproduced below** |
| `gen_companion_book` | companion | whole subdir, unconditional | **NO** (`grep -n "out_path.exists()" src/bin/gen_book_cache.rs` returns exactly 2 hits, both inside `gen_monster_book`; zero inside `gen_companion_book`) | **VULNERABLE — not live-tested (see §1.5)** |

**The fix for `monster`/`monster_ability` (`gen_monster_book`) exists and was never extended to
the other two functions in the same file that write kinds carrying the identical downstream-
enrichment dependency.** This is the residual: the same file, the same class of bug, fixed once
and not generalized to its own siblings.

### 1.4 Live reproduction — `gen_advanced_race_guide`, confirmed, then reverted

```
git status --short              # 0 lines, confirmed clean before touching anything
export CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/w30-lane4
cargo build --locked --bin gen_book_cache
cargo run --locked --bin gen_book_cache -- advanced_race_guide
```

Output: `spells written: 93/93, equipment written: 200/200, feats written: 187/187`.

```
git status --short data/corpus/advanced_race_guide/ | awk '{print $1}' | sort | uniq -c
#   15 D
#  481 M
```

- **Spell: raw_tokens dropped, all 93 records.** Spot-checked
  `advanced_race_guide/spell/agonizing_rebuke.json`: `grep -c raw_tokens` was `1` before the
  run, `0` after; `git diff --stat` on that one file: `88 lines → 11 lines (77 deletions)`.
  Every `advanced_race_guide` spell record carried `raw_tokens` before this run
  (`git ls-files data/corpus/advanced_race_guide/spell/ | wc -l` = 93, all 93 enriched — same
  method as §1.6's corpus-wide count) — this is a 100%-of-population hit, not a sample.
- **Equipment: 15 records permanently deleted, not renamed.** `git status --short` showed 15
  `D` entries under `equipment/equipmods/` and **zero** untracked (`??`) files anywhere under
  `advanced_race_guide/`; the on-disk equipment file count after the run was exactly 200
  (matching the generator's own "200/200" report), against 215 committed
  (`git ls-files data/corpus/advanced_race_guide/equipment/ | wc -l` = 215). One spot-checked
  casualty, `equipment/equipmods/cruel.json`, carried real shipped content (a "Cruel" weapon
  special ability: description, `raw_tokens`, `raw_bonus_chains`) — not a stub or placeholder.
  **Root cause, confirmed by cross-reference**: `gen_equipment_gap_tables.rs` (line 258) and
  `cache_gen::equipment_gap` (`"ARG" => Some(("advanced_race_guide", ...))`) both also write
  into `advanced_race_guide/equipment/`; `gen_advanced_race_guide`'s unconditional
  `remove_dir_all` wipes their output too, and its own hardcoded `equipment_tables()` (200
  entries) does not reproduce it. **This is the exact cross-generator mutual-destruction shape
  `ingest_races.rs`/`ingest_race_traits.rs` were deliberately hardened against
  (`SD-31-E6-F4-002`/`-003`, see §1.5) — landed there, never landed here.**

Reverted immediately:

```
git checkout -- data/corpus/advanced_race_guide/
git status --short          # 0 lines
md5sum docs/work-inventory.json   # d64ddfc677fd1683f5b7638889a25c54, unchanged
```

**This is a genuinely new finding, not a restatement of D1.** D1 (defects.md) names
`advanced_players_guide`/`core_rulebook` equipment citation-reversion; `advanced_race_guide` is
a fourth book, and the confirmed loss here is broader than citation-narrowing — a whole record
kind (`spell`) never covered by D1 at all, plus outright content deletion (not merely a
citation regressing to a wider one). Filed as `defects.md` D9 (§4 below).

### 1.5 Checked by code read, not live-tested — named explicitly, per the wave's own caution

- **`gen_companion_book`** (§1.3): identical unguarded shape to `gen_advanced_race_guide`,
  confirmed by the same `out_path.exists()` grep returning zero hits inside it. **927 of 927
  companion records corpus-wide** (100%, `git ls-files data/corpus/*/companion/ | wc -l` = 927,
  cross-checked against the raw_tokens-carrying count in §1.6) sit behind this function across
  **16** `CompanionBookSpec` entries (`grep -c 'CompanionBookSpec {' src/bin/gen_book_cache.rs`
  minus the struct definition itself = 16). Not live-tested: companion is the single largest
  at-risk population found this wave and I judged reproducing on the real corpus and reverting,
  a second time, added confirmation but not new information the code read didn't already give
  with high confidence — the exists-guard's absence is unambiguous by direct grep, not
  inference from behavior.
- **`gen_core_rulebook_cache`** (separate binary, already named in D1 for its equipment-citation
  aspect): `grep -n "out_path.exists()" src/bin/gen_core_rulebook_cache.rs` returns **zero**
  hits — the whole-book unconditional wipe (`fs::remove_dir_all` over every subdirectory,
  line 387) has no per-record guard at all, the same shape as `gen_advanced_race_guide`, for
  the largest single book in the corpus. `core_rulebook/spell`: **664 of 664 (100%)** records
  carry `raw_tokens` (`git ls-files data/corpus/core_rulebook/spell/ | wc -l` = 664, cross-
  checked as in §1.6). **Not live-tested — deliberately.** `core_rulebook` is the single
  largest, most central book in the corpus (2,993 equipment + 664 spell + more); the
  `advanced_race_guide` reproduction already proves the shape holds for this exact code
  pattern (`gen_book_cache.rs`'s two unguarded functions and `gen_core_rulebook_cache.rs`'s
  main loop are structurally identical: unconditional whole-subdir wipe, no per-record check),
  and running a second, larger, live-then-revert cycle against the corpus's most central book
  to confirm what the code already shows unambiguously was judged the wrong risk/information
  trade this wave. **Recommend a future cycle extend the `SD31-E6-F9-005` exists-guard to
  `gen_core_rulebook_cache.rs` and `gen_companion_book` directly, rather than re-proving it live
  a second and third time.**

### 1.6 Corpus-wide exposure ceiling — how much content depends on a downstream pass surviving a future regen

```
python3 -c "
import json, glob
for kind in ['spell','equipment','companion','monster','monster_ability']:
    files = glob.glob(f'data/corpus/*/{kind}/**/*.json', recursive=True)
    tot = len(files); n = 0
    for f in files:
        d = json.load(open(f))
        data = d.get('data', {})
        if isinstance(data, dict) and (data.get('raw_tokens') is not None or data.get('raw_bonus_chains') is not None):
            n += 1
    print(kind, n, '/', tot)
"
```

```
spell            1999 / 2011
equipment        7095 / 7284
companion         927 / 927
monster          1242 / 1242     (FIXED — gen_monster_book's exists-guard)
monster_ability  1920 / 1920     (FIXED — gen_monster_book's exists-guard)
```

**Read this as a ceiling, not a per-run blast radius.** No single generator rerun touches the
whole corpus — `gen_advanced_race_guide`'s one confirmed run touched exactly its own 93+200+187
records, not all 1,999 spell records. The number that matters per-generator is the book(s) it
owns; `companion` is the one kind where a single function's scope (`gen_companion_book`, called
once per book but sharing the one unguarded code path) can reach the *entire* kind's population
across all 16 books over repeated invocations. `equipment`'s citation-narrowing sub-risk is
already tracked (D1); its raw_tokens sub-risk is covered process-side by the mandatory
`enrich_equipment_raw_tokens` rerun (`docs/governance/book-ingestion-playbook.md` item 9,
confirmed present — see §1.7). `spell` has **no equivalent documented mandatory step anywhere**
(`grep -n "enrich_spell\|enrich_companion\|enrich_monster" docs/governance/book-ingestion-playbook.md`
returns nothing) — this is the actual gap: the mitigation that exists for equipment was never
generalized to the three other `enrich_*` kinds, even though the same generator-divergence
mechanism playbook item 9 describes for equipment applies identically to all four.

### 1.7 A lead investigated and REFUTED — `gen_cache_beastiary` / `Rag Armor (Dark Creeper)`

Initial static read of `cache_gen::beastiary1::equipment_source()` found it hardcodes
`Source::WebSecondSource` for `"Rag Armor (Dark Creeper)"` unconditionally, which looked like a
third instance of D1's exact shape. **Checked live, non-destructively, and refuted:**

```
cargo run --locked --bin repair_lst_provenance -- --check
```

```
beastiary: 0 record(s) narrowed to an lst_token citation, 3 already cited (0 stale wiring_class
refreshed), 1 refused, of 4 read
  REFUSED  .../beastiary/equipment/rag_armor_dark_creeper.json: no corpus row matches this
           record's identity
```

`repair_lst_provenance` itself permanently refuses this exact record — it cannot be narrowed
(there is no real corpus armor row backing a monster's special-quality item), so
`gen_cache_beastiary`'s hardcoded citation is not reverting anything; it is the correct,
terminal state, and the two tools agree. **Also confirms `advanced_players_guide` and
`core_rulebook` currently have ZERO pending narrows** (372 and 2,993 "already cited", 0
"narrowed" — meaning D1's "412 narrowed citations" describes content already sitting in the
corpus today, at risk from a *future* rerun of those two generators, not a currently-pending
repair). This is the correct outcome for a residual sweep: a plausible-looking lead, checked
with a safe dry-run instead of assumed, and dropped when the evidence didn't support it.

### 1.8 Checked and confirmed SAFE — the `ingest_*` race/class family (already hardened)

`ingest_races.rs`, `ingest_race_traits.rs`, `ingest_apg_race_traits.rs`, `ingest_pu_classes.rs`
all call `remove_dir_all`. Read each site:

- `ingest_races.rs` (line 1253): whole-directory clear, but **explicitly scoped to the 5 books
  it exclusively owns** (`core_rulebook`, `beastiary`, `bestiary_2`, `bestiary_5`,
  `bestiary_6`) — `advanced_race_guide` is deliberately excluded from this list because
  `ingest_race_traits.rs` also writes there, and the code's own comment names the exact hazard
  and why it is avoided.
- `ingest_race_traits.rs`: clears **by content field** (`is_racial_default: false`), not by
  directory, specifically because `advanced_race_guide` became a shared-write target with
  `ingest_races.rs` (`SD-31-E6-F4-002`/`-003`, both documented in-line with the exact
  mutual-destruction scenario they prevent).
- `ingest_apg_race_traits.rs` (line 493): `advanced_players_guide/race_trait` —
  cross-referenced (`grep -rln 'advanced_players_guide/race_trait' src/bin/`), this binary is
  the sole writer. Safe.
- `ingest_pu_classes.rs` (line 901): `pathfinder_unchained/{class,class_feature}` —
  cross-referenced against every other binary touching `pathfinder_unchained`
  (`gen_book_cache`'s `gen_pathfinder_unchained` writes only `feat`/`equipment`, disjoint).
  Safe.

**These four are the best-hardened files in the whole population** — each carries an in-line
doc comment recording the exact hazard this sweep is asking about and the specific incident
that motivated the fix. This is prior remediation, re-confirmed, not new territory.

### 1.9 Not reached this wave — named plainly, 17 of 29

**Not reached, 10 `gen_*`:** `gen_cache_acg`, `gen_cache_apg` (beyond its already-known D1
citation aspect), `gen_cache_class_feature`, `gen_cache_equipment_gap`,
`gen_cache_hand_authored_equipment`, `gen_cache_spell_lane_dump`, `gen_cache_ultimate_equipment`,
`gen_class_feature_grants`, `gen_equipment_gap_tables`, `gen_feat_gap_tables`.

**Not reached, 7 `ingest_*`:** `ingest_adventurers_guide_spells`, `ingest_class_spell_levels_arg`,
`ingest_inner_sea_gods_spells`, `ingest_occult_adventures_spells`, `ingest_ultimate_combat_spells`,
`ingest_ultimate_magic_spells`, `ingest_ultimate_wilderness_spells`.

All 17 were confirmed to write to `data/corpus` (§1.1's population command) but their
write-vs-merge pattern and cross-generator directory overlap were **not** individually traced
this wave. Given the `gen_book_cache`/`gen_core_rulebook_cache` results (2 of 6 checked
generator functions vulnerable, both in the same file class as the ones already fixed), I would
not assume these 17 are clean without checking — but I also would not assume any one of them
repeats the defect without checking; this is genuinely unknown, stated as such rather than
extrapolated either direction.

**S6 residual status: PARTIAL. Population 29, counted twice by the same command to catch a
first-draft miscount (§1.1).** 10 of 29 confirmed safe or refuted this wave (5 `enrich_*` +
`ingest_races`/`ingest_race_traits`/`ingest_apg_race_traits`/`ingest_pu_classes` (4, already
hardened by a prior cycle) + `gen_cache_beastiary` (1, lead investigated and refuted)); 2 of 29
confirmed-or-high-confidence to contain a vulnerable, unguarded write path (`gen_book_cache` —
2 of its 4 internal generator functions, `gen_advanced_race_guide` live-reproduced and
`gen_companion_book` by code read — and `gen_core_rulebook_cache`, D1-adjacent, newly
generalized to the `spell` kind, by code read); 17 of 29 not reached (immediately above).
10 + 2 + 17 = 29 ✓.

---

## 2. S2 — generalizing the Monk case beyond classes

Wave 28 checked the 3 kinds carrying an `IdEnum`+table split (class/race/monster) and found 0
new gaps, then found the shape recurs in a non-enum form (`SPELL_BOOK_B6`, D8). This wave
checked the two kinds the dispatch brief specifically named as unchecked: **companion** and
**equipment**.

**Companion — CLOSED at 0, by construction, not by fresh audit.** `companion_chassis::
COMPANION_BOOKS` is the table; every consumer (`gen_book_cache.rs`'s `gen_companion_book`,
`companion_catalog.rs`, `v06_work_inventory.rs`, `corpus_ingest_diagnostic.rs`,
`derived_evaluator_fixture_check.rs`) reads it directly, and `companion_catalog.rs` already
carries a locking test (`grep -n "COMPANION_BOOKS without reaching the catalog fails here"
apps/desktop/src-tauri/src/companion_catalog.rs`) asserting every table entry reaches the
catalog. The Monk shape (table complete, dispatch string→id mapping silently absent) cannot
recur here undetected — the test that would catch it already exists and already passes.

**Equipment — the enum-mediated shape does not apply; the general "table vs. consumer" shape
recurs, but it is the SAME instance §1 already found, not a new one.** Equipment has no
`EquipmentId`-style enum+table+dispatch layer analogous to `ClassId`/`RaceId` (checked:
`grep -rn "^pub enum" src/rules_core/rules_tables/*.rs` naming an equipment id enum returns
nothing) — a corpus record IS the artifact once a generator writes it, there is no intermediate
runtime dispatch table an id could go missing from the way `table_class_id` was missing a Monk
row. The place equipment content genuinely goes missing is the cross-generator
`remove_dir_all` collision §1.4 found — that is S6-shaped (a write clobbers a sibling writer's
output), not S2-shaped (a table exists but nothing reads it). Cross-referenced here rather than
double-counted.

**S2 status: race/monster remain at wave-28's 0 (not re-derived this wave); companion CLOSED
at 0; equipment reasoned N/A for the enum-mediated shape, its real risk already filed under
S6/D9.**

---

## 3. S9 — one row from working, corpus-wide

The Ninja/Samurai shape: `CLASS_WEAPON_PROFICIENCIES` is missing exactly their 2 rows.

```
grep -c "ClassWeaponProficiency {" src/rules_core/rules_tables/crb/weapon_tables.rs
```
→ **32** (of the 34 dispatched classes) — re-confirms wave 28's class-scoped 0/34-beyond-
Ninja/Samurai finding exactly: 34 − 32 = 2, and those 2 are the known pair.

Searched for other `ClassId`-keyed lookup tables the same shared architecture might expose:

```
grep -rn "\.class_id == class_id\|\.iter().find(|.*class_id" src/rules_core/ apps/desktop/src-tauri/src/
```

10 hits. Of the class-lookup ones, `CLASS_META` (`crb/class_tables.rs`) is **intentionally**
CRB-scoped (12 entries — hit die/BAB/saves for the 11-12 CRB base classes only, not a
34-class shared table; comparing it against 34 would be the wrong denominator, not a gap).
No other shared, all-34-classes lookup table with a narrower row count than 34 was found.

**S9 status: the class-scoped sub-question stays answered at 0/34 beyond Ninja/Samurai — this
wave re-derived the anchor number rather than trusting it, and it holds. The corpus-wide
question (does this shape recur in companion/monster/race/spell/equipment tables, not just
class-keyed ones) was NOT reached this wave** — auditing every keyed lookup table across 5 more
kinds for a narrow-row-count gap is a genuinely large search that this lane's remaining budget
did not cover. Said plainly rather than extrapolated from the one population checked.

---

## 4. S7 — narrowed to production `ground_*`/`compute_*`/`classify_*` functions

```
grep -rn "assert_eq!(.*\.len()" --include=*.rs src apps/desktop/src-tauri/src | wc -l
```
→ **494** raw sites (differs from THE-BOX's cited 311 — a different literal pattern or scope;
494 is what this exact command returns against `3627601f1` and is reproducible).

Wrote a scanner (kept at `/tmp/.../scratchpad/s7_scan2.py`, reproducible, not committed) that
tracks, per `.rs` file, the nearest preceding `fn` declaration and whether it was immediately
preceded by `#[test]`:

```
total assert_eq!(...len()...) sites: 494
  inside a #[test] fn: 487
  in production (non-test) fns: 7
matched in ground_/compute_/classify_-named production functions: 1
```

The one match, `pilot_compute::compute_pu_class_chassis`
(`debug_assert_eq!(input.chosen.class_levels.len(), 1)`), read in context: it is a documented
single-class-only invariant ("this assertion says so out loud rather than the fact being
inferable only from the absence of a call") guarding a precondition, not a doneness/content gate
counting granted abilities without checking which ones — **not** the wave-16 companion shape.
The other 6 production sites are either test-harness helper functions (`foo`, inside
`formula_reproduction_harness.rs`'s own test-support code) or genuine but benign
completeness invariants (`assert_eq!(written, rows.len(), ...)` in `ingest_race_traits.rs`/
`ingest_apg_race_traits.rs`, checking that ingestion dropped nothing — a S6-adjacent protective
assertion, not an S7-shaped defect).

**S7 narrowed sub-question: CLOSED at 0.** Caveat stated plainly: this only catches the
`assert_eq!(_.len(),_)` proxy specifically, and only when the enclosing function's own name
carries the prefix; a gate shaped as `if x.len() >= N` with no `assert_eq!`, or one living in a
differently-named helper a `ground_*`/`compute_*`/`classify_*` function calls, would not be
caught by this method. The broader S7 population (which of the 494 sites, test or production,
represent a genuine "count without identity" risk by some other shape) remains open.

---

## 5. S8 — reproduction check

```
python3 -c "
import json, glob
total = 0; none_class = 0
for f in glob.glob('data/corpus/*/class_feature/**/*.json', recursive=True):
    d = json.load(open(f))
    data = d.get('data', {})
    if isinstance(data, dict) and 'class' in data:
        total += 1
        if data.get('class') is None:
            none_class += 1
print(total, none_class, total - none_class)
"
```
→ `12417 915 11502`

Wave 28's anchor (**11,502 non-null `data.class` records**, the denominator its 71.4% figure is
built on) **reproduces exactly.** The `None` sub-count drifted: wave 28 recorded 979, this wave
finds 915 (−64). Not investigated further this wave — plausibly wave 29's `adventurers_guide`
book onboarding (which added new `class_feature` records) or an unrelated reclassification;
flagged, not explained. **S8 status unchanged: MEASURED, not closed** (the 71.4%
non-dispatched-class breakdown itself was not re-run this wave — only its anchor population was
re-derived and holds).

---

## 6. New todo entries filed

### `defects.md` D9 (new)

**Defect:** `gen_book_cache.rs`'s `gen_advanced_race_guide()` and `gen_companion_book()`, and
the separate binary `gen_core_rulebook_cache.rs`, unconditionally `remove_dir_all` + regenerate
their owned subdirectories on every run, with no per-record exists-guard — the exact shape
`gen_monster_book` in the SAME FILE was fixed for (`SD31-E6-F9-005`) but the fix was never
extended to its own siblings. **Live-reproduced for `gen_advanced_race_guide`**: one real run
against the committed corpus wiped `raw_tokens` from all 93 `advanced_race_guide` spell records
(a kind, `spell`, D1 never named) and permanently deleted 15 real, populated equipment records
belonging to `gen_equipment_gap_tables`/`gen_cache_equipment_gap` (cross-generator collision,
not merely citation-narrowing) — reverted cleanly, `git status` and `docs/work-inventory.json`
hash confirmed unchanged. `gen_companion_book` (927 of 927 companion records, 100% of the kind,
across 16 books) and `gen_core_rulebook_cache` (664 of 664 core_rulebook spell records) are
confirmed vulnerable by the identical code-read method (zero `out_path.exists()` guards) but not
live-tested, given their size. **Blast radius: 93 confirmed spell + 15 confirmed equipment
(live); up to 927 companion + 664 core_rulebook spell (code-read, high confidence, not yet
proven live) — a genuine extension of D1, not a duplicate of it.** **Fix:** extend the
`SD31-E6-F9-005` `out_path.exists()`-then-skip pattern from `gen_monster_book` to
`gen_advanced_race_guide`, `gen_companion_book`, and `gen_core_rulebook_cache`'s three affected
subdirectories.

### `sweeps.md` S6 residual — update in place (§7 below shows the diff)

### `levers.md` — not filed; this is a defect (known-wrong, fixable directly), not a lever
(nothing here unblocks other units; it protects already-shipped ones from a future regen).

---

## 7. `sweeps.md` S6 row — replacement text

See the companion edit to `todo/sweeps.md` (S6 row) and `todo/defects.md` (new D9 row), both
applied in the same commit as this file.
