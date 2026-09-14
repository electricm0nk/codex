# Cycle 3 — Epic 7 Closure epilogue / AT-35-E7-000-POPULATION-CENSUS

**THE ONE-SENTENCE ANSWER.** The census is **closed**: all **2,912** never-reached `data/corpus`
records now carry an independently re-verified disposition — the **2,657** cycle 2 had explicitly
*not* re-verified were re-tested one at a time against their bucket's own predicate (**2,650**
confirmed, **7** failing a prose-quantity clause this cycle *added*, **0** of them absent), and the
**10** cycle 2 called a *ceiling* were re-run under three progressively weaker text tests plus a
name search across all 49,438 inventory units and **0 of 10 were rescued** — so **10 is exact, not
an upper bound**, and the corpus-wide figure **48,854 of 48,864 = 99.9795%** is confirmed from a
population eleven times larger than the one that first produced it.

- **Commit SHA:** `<filled at commit>`. Cycle start `82ffbb4ed24f1e3671bb63b4b74ef426373d7eae`.
- **Scope gate:** `SCOPE_GATE: EXEMPT (measurement cycle — it moves no unit and writes no rule; its deliverable is a census and a verdict)`
  (`decisions.md §2` floor exemption — the same ground cycles 1 and 2 stood on, for the same reason:
  this cycle closes ZERO units **by design** and its own criterion forbids it to move any. The
  remainder it is scoped to is furthermore **not expressible as `cycle_scope_gate.py` flags at all**
  — the gate's population is `docs/work-inventory.json`, and these records are by definition the
  ones *not in it*. Run for the record at cycle start, the gate reads
  `scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER`, which is the inventory
  being wholly DONE, not a statement about this cycle's scope.)
- **Files touched:**
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/population_census_final.py` (new — the method)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/population-census-final.json` (new — the closed census)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/population-census-final-detail.json` (new — per-record verdict for all 2,657)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/AT-35-E7-000-POPULATION-CENSUS_cycle3_receipt.md` (this file)
  - `docs/release/SD-35-corpus-sheet-completion/progress.md`, `docs/release/SD-35-corpus-sheet-completion/kanban.md`
  - `docs/retro/events/at-35-e7-000-population-census.jsonl`
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` — **not this
    cycle's content change**: `completion_atlas.py --check` restamps its own `derived_at`. Folded
    rather than left dirty (memory's `clean-tree-means-unfiltered-status`), exactly as cycle 2 did.
  - **Nothing else.** `data/corpus`, `docs/work-inventory.json`, `data/sheet_rules/` and `src/` are
    byte-identical to the cycle-start SHA.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS. Run directly over this cycle's three new files
  (they are untracked at audit time, so a `git diff` cannot see them — the §6 step-2 regexes were
  applied to the files themselves). The wider `${BASE_BRANCH}...HEAD` diff reports only the
  pre-existing non-stub hits cycles 1 and 2 already enumerated — two lines of published Pathfinder
  prose containing an audit trigger verb, and three `docs/work-inventory.json` lines this bundle
  **removed** — plus the receipts that enumerate them. None is this cycle's and none is a stub.
- **Acceptance criterion:** `AT-35-E7-000-POPULATION-CENSUS` is **not** a section in
  `epic-breakdown.md` — re-verified this cycle with
  `grep -rn 'AT-35-E7-000' docs/release/SD-35-corpus-sheet-completion/epic-breakdown.md` (no output;
  the id appears only in `kanban.md` and `progress.md`). Its dispatch text is therefore the
  acceptance bar, and the criterion line is *"how many corpus records never reach the inventory —
  measure, do not fix."* Cycle 3's dispatch scopes it to the cycle-2 remainder
  `record_absent_from_inventory_population=10`.
- **Receipt rows (mechanical):**
  `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=0`
- **PCGen residue:** `live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS`
  (also `shipped_data_files=0 shipped_data_hits=0 shipped_scanned=11`,
  `identifier_files=0 identifier_hits=0`). Identical to the cycle-start reading and to cycles 1
  and 2 — this cycle writes no Rust and no shipped data.
- **Oracle parity:** N/A — no `Number` mapping added, no live path touched. The pinned tree was read
  **read-only**, to re-derive the root cause of the 10 independently of cycle 2's claim.
- **Movement, four buckets:**
  - closure: **none** (id-set empty)
  - relabel: **none**
  - reachability: **none.** No record moved. Every figure here describes the tree as it stood at
    cycle start and as it stands at HEAD.
  - instrument-correction: **this whole cycle**, and specifically it corrects the *strength of the
    evidence* behind two existing figures rather than the figures themselves. `10` and
    `48,854 of 48,864` do not move; what moves is that they are now derived from an exhaustively
    re-verified population instead of a partly asserted one.
- **Refused tokens:** `record_absent_from_inventory_population=10`
  (`pathfinder_unchained/feat=9` + `mythic_adventures/spell=1` = **10**). Unchanged from cycle 2 and
  now proven exact rather than bounded. Emitted as a `deferral` retro event.
- **Discoveries:** two, each emitted as a `correction` retro event —
  1. cycle 2's own caveat that the 10 are a **ceiling** is wrong: they are **exact**. 0 of 10 are
     rescued by any weaker test, and the best token-overlap score any of the 10 reaches against the
     whole reached population is a Jaccard of **0.3636** (`champion_of_tranquility`) — a ratio of
     shared tokens to the union of both records' tokens, not a percentage of a population.
  2. cycle 1's `duplicate_ingest` predicate had **no prose-quantity clause**, so it could in
     principle have credited a duplicate to a twin that lost the rule text. Adding the clause fails
     **7 of 612** — all 7 still reached, exposing a *different* shape (one `.lst` row ingested as
     several prose variants of one rule) rather than a lost record.

  Neither is a `token-coverage.json` refusal shape and neither is a new atlas remaining-step
  category, so neither instrument needs re-deriving. Both `--check`s were run anyway and are
  recorded below.

## Figures + their re-derive commands

Every figure re-derives from

```
python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/population_census_final.py
```

(runtime **14 s**, measured: `real 0m14.169s`) unless another command is named beside it.

### GAP 1 — the 2,657 cycle 2 did not re-verify

Cycle 2's `what_this_proof_does_not_cover` item 1, verbatim: *"It does not re-verify the 2,657
records cycle 1 placed in the three not-a-record buckets."* Those records were dispositioned by the
bucket's defining predicate at classification time and then trusted. At **2,657** they are the
census's largest unverified population — **ten times** the 255 cycle 2 spent a whole cycle on — and
one of them carrying printable prose that reaches nothing would mean the answer is not 10.

All 2,657 were re-tested, one at a time, against the bucket's own predicate:

| Bucket | Predicate re-tested | n | CONFIRMED | FAILED |
| --- | --- | --- | ---: | ---: |
| `generated_artifact_no_completeness` | no `completeness`, no `source` object, no `data` object, kind is `_settled` | 47 | **47** | 0 |
| `chassis_only_mod_row_no_rule_text` | `completeness == "chassis_only"` **and zero chars of published prose** — a chassis row carrying prose would be a real record under the wrong label | 1,998 | **1,998** | 0 |
| `duplicate_ingest_of_a_row_already_reached` | another corpus record sits at the **same** `(source_file, source_line)` (book label dropped), that twin is itself reached, **and the twin carries at least as much published prose** | 612 | **605** | **7** |
| | | **2,657** | **2,650** | **7** |

**Every record that failed then went through cycle 2's full content-key ladder** — it is no longer
dispositioned by assertion. All 7 resolve `reached_via_reattributed_same_row`:
**`gap_1_new_absent = 0`.**

**The 7, named** (this is the clause cycle 1's predicate did not have, so they are new information,
not a defect cycle 1 made):

| record | reached twin carries | reaches |
| --- | --- | --- |
| `inner_sea_races/spell/elemental_mastery-3.json` | 224 < **766** chars | `inner_sea_races:spell:elemental_mastery` |
| `inner_sea_races/spell/elemental_mastery-5.json` | 224 < 246 | same |
| `inner_sea_races/spell/elemental_mastery.json` | 224 < 237 | same |
| `inner_sea_races/spell/elemental_mastery-4.json` | 224 < 235 | same |
| `bestiary_4/spell/summon_nature_s_ally_v_oceanid.json` | 106 < 116 | `bestiary_4:spell:summon_nature_s_ally_v_oceanid` |
| `advanced_class_guide/class_feature/warpriest/blessings_favored_class.json` | 0 < 199 | `advanced_class_guide:class_feature:warpriest_favored_class_blessings` |
| `pathfinder_unchained/class_feature/summoner/unchained_summoner.json` | 0 < 172 | `pathfinder_unchained:class_feature:summoner_unchained_class` |

The mechanism is **one `.lst` row ingested as several prose variants of one rule**. All five
`elemental_mastery*.json` records cite `isr_spells.lst:18` and all five carry the KEY
`Elemental Mastery`; their descriptions are the per-element paragraphs of one spell (*"…a +20 ft.
insight bonus to the speed of all your movement modes…"* / *"…a burrow speed of 15 ft.…"* / *"You
can choose any one of the four elements…"*). One unit, `inner_sea_races:spell:elemental_mastery`
(`sheet-complete`), holds the row.

```
python3 -c "import json;[print(f,json.load(open(f'data/corpus/inner_sea_races/spell/{f}.json'))['source']['line']) for f in ('elemental_mastery','elemental_mastery-2','elemental_mastery-3')]"
```

**This is not an absence**, and it is deliberately not counted as one: the rule reaches a rendered
sheet line. What the 7 raise is a narrower question — whether every prose *variant* of a
multi-variant row prints, or only the one the unit resolved to — which is a
**rendering-completeness** question about a **reached** unit, not a reachability question, and so
sits outside a criterion whose words are *"how many corpus records never reach the inventory"*. It
is recorded in the artifact's `what_this_proof_does_not_cover` so it is a named item and not a
silent one.

### GAP 3 — the 10 are exact, not a ceiling

Cycle 2's item 3, verbatim: *"Prose identity is normalised for whitespace and case ONLY. … The 10
are therefore a **ceiling**, not a floor."* Each of the 10 was re-run under three progressively
weaker tests, plus a name search:

1. **strict identity** — cycle 2's test, whitespace + case. Unchanged, for reference.
2. **punctuation-normalised identity** — `[^a-z0-9]+` collapsed. **Digits survive this**, so `1d6`
   still compares different from `1d8` and `+2` from `+3`; only punctuation and spacing are
   laundered. This is the test cycle 2 named as the one it had not run.
3. **containment** — is this record's *whole* paragraph a substring of some reached record's
   paragraph? Applied only at ≥ 24 chars, so a five-word benefit line cannot be swallowed by an
   unrelated long paragraph.
4. **name** — does **any** of the 49,438 inventory units carry this record's KEY or name?

And the **best token-overlap (Jaccard) score against the whole reached population** is reported for
every one, so the distance to the nearest twin is a number rather than an assertion.

| record | row | chars | t1 | t2 | t3 | name | best Jaccard | nearest reached record |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| `champion_of_anarchy` | `pu_feats.lst:6` | 33 | 0 | 0 | 0 | 0 | 0.2174 | `inner_sea_combat/.../code_of_conduct.json` |
| `champion_of_balance` | `pu_feats.lst:7` | 55 | 0 | 0 | 0 | 0 | 0.3333 | `advanced_race_guide/feat/combat/gloom_strike.json` |
| `champion_of_destruction` | `pu_feats.lst:8` | 57 | 0 | 0 | 0 | 0 | 0.2632 | `core_rulebook/companion/come.json` |
| `champion_of_freedom` | `pu_feats.lst:9` | 50 | 0 | 0 | 0 | 0 | 0.2667 | `advanced_class_guide/.../greater_trip.json` |
| `champion_of_grace` | `pu_feats.lst:10` | 50 | 0 | 0 | 0 | 0 | 0.2941 | `mythic_adventures/.../witch_knife.json` |
| `champion_of_malevolence` | `pu_feats.lst:11` | 58 | 0 | 0 | 0 | 0 | 0.1765 | `core_rulebook/.../scroll_of_true_seeing_arcane.json` |
| `champion_of_righteousness` | `pu_feats.lst:12` | 89 | 0 | 0 | 0 | 0 | 0.2368 | `core_rulebook/class_feature/monk/diamond_soul.json` |
| `champion_of_tranquility` | `pu_feats.lst:13` | 41 | 0 | 0 | 0 | 0 | **0.3636** | `ultimate_psionics/feat/enlarged_collective.json` |
| `champion_of_tyranny` | `pu_feats.lst:14` | 49 | 0 | 0 | 0 | 0 | 0.2857 | `ultimate_psionics/.../mind_knight_path.json` |
| `elemental_body_iiimod` | `ma_spells.lst:98` | 649 | 0 | 0 | 0 | 0 | 0.2900 | `mythic_adventures/ability/elemental_bond.json` |

**`rescued_by_a_weaker_test = 0`. `still_absent_under_every_test = 10`.** The highest score in the
column is 0.3636 — the nearest reached record shares barely a third of `champion_of_tranquility`'s
vocabulary, which is what two unrelated short rules written in the same house style look like, not a
twin. **The ceiling equals the floor.**

Independently corroborated without the script:

```
grep -c 'Champion of Anarchy'  docs/work-inventory.json   # -> 0
grep -c 'Elemental Body IIIMOD' docs/work-inventory.json  # -> 0
grep -c 'Combat Stamina'        docs/work-inventory.json  # -> 1   (control: a pu_feats.lst row that IS held)
```

### GAP 2 — the prose-reach figure, given its predicate

Cycle 2's item 2 warned that `reached_via_identical_prose_record` proves the **words** reach a sheet
line, not that the reached record is filed under the name a player would look up. Quantified:
**4 of 4** prose-reached records carry a name that differs from their twin's.

| record | its name | prints under |
| --- | --- | --- |
| `apg/spell/threefold_aspect_young_adult` | Threefold Aspect (Young Adult) | Threefold Aspect |
| `apg/spell/threefold_aspect_adulthood` | Threefold Aspect (Adulthood) | Threefold Aspect |
| `apg/spell/threefold_aspect_elderly` | Threefold Aspect (Elderly) | Threefold Aspect |
| `apg/spell/wall_of_thorms` | Wall of Thorms | Wall of Thorns |

Three are sub-forms of one spell printing under the base rule's name; the fourth is a **misspelling**
in the PCGen source (`Thorms`) whose 1,851 characters are the Core Rulebook *Wall of Thorns*
paragraph. In all four the words print. This is a lookup-ergonomics observation, and under the sheet
rule the unit is DONE — stated as a number so no reader has to infer it from a caveat.

### Root cause of the 10 — re-derived from the pinned tree, not quoted

Cycle 2 named one predicate, `src/bin/v06_work_inventory.rs:3172-3173` (`has_classifying_token`):

```rust
Kind::Feat  => has_token(fields, "TYPE:"),
Kind::Spell => has_token(fields, "SCHOOL:") || has_token(fields, "CLASSES:"),
```

A row failing its kind's test is never enumerated and lands in the `missing_classifying_token` trap.
Re-derived here from the pinned checkout rather than carried forward as a quote:

| Evidence | Command | Result |
| --- | --- | --- |
| `pu_feats.lst` rows 6–14 carry **no** `TYPE:` | `awk -F'\t' 'NR>=6&&NR<=14' "$PU" \| grep -c 'TYPE:'` | `0` |
| the 8 `pu_feats.lst` rows the inventory **does** hold all carry `TYPE:` | `awk 'NR==18\|\|NR==19\|\|NR==20\|\|NR==25\|\|NR==26\|\|NR==27\|\|NR==29\|\|NR==32' "$PU" \| grep -c 'TYPE:'` | `8` |
| `pu_feats.lst:6` is the row | `awk -F'\t' 'NR==6{print $1}' "$PU"` | `Champion of Anarchy` |
| `ma_spells.lst:98` carries neither `SCHOOL:` nor `CLASSES:` | `awk 'NR==98' "$MA" \| grep -cE 'SCHOOL:\|CLASSES:'` | `0` |
| `ma_spells.lst:98` is the row | `awk -F'\t' 'NR==98{print $1}' "$MA"` | `Elemental Body IIIMOD` |

(`PU=$PCGEN_CORPUS_ROOT/pathfinder/paizo/roleplaying_game/pathfinder_unchained/pu_feats.lst`,
`MA=$PCGEN_CORPUS_ROOT/pathfinder/paizo/roleplaying_game/mythic_adventures/ma_spells.lst`.)

Under the sheet rule all 10 would render as words — nine short feat benefits and one 649-character
mythic augmentation paragraph. **This is not a carve-out and not an impossibility**; it is a
one-predicate enumeration gap with a named fix.

### The closed census — every figure reconciling

| Statement | Value | Derivation |
| --- | --- | ---: |
| the atlas's figure | `DONE 49,438 of 49,438` = 100% **of the inventory** | `python3 scripts/completion_atlas.py --check` → `population=49438 buckets=10 unclassified=0 overlap=0`, `DONE: 49438` |
| `data/corpus` JSON files | 51,521 | cycle 1 |
| never reached by any inventory unit | **2,912** | cycle 1 |
| ⤷ not-a-record buckets | **2,657** | 1,998 + 612 + 47; **all re-verified this cycle** |
| ⤷ `completeness: full` with prose | **255** | dispositioned by cycle 2 |
| real corpus rules records | **48,864** | cycle 1 denominator, carried forward |
| …reached by a DONE inventory unit | **48,854** | 48,609 + (255 − 10) |
| **corpus-wide completion** | **48,854 of 48,864 = 99.9795%** | confirmed, not moved |
| records reaching no sheet line | **10** | exact under four tests |

```
2,657 + 255 = 2,912   (python3 -c over population-census-unreached.json)
193 + 48 + 4 + 10 = 255   (cycle 2's dispositions, re-summed this cycle)
2,650 + 7 = 2,657   and   gap_1_new_absent = 0
```

All three reconcile exactly. Every inventory unit is DONE at HEAD, so "reached" and "reached by a
DONE unit" are the same set here — stated rather than assumed.

## What this proof does **not** cover

`AGENTS.md §7` makes the omission the load-bearing part. Recorded in the artifact as
`what_this_proof_does_not_cover` and repeated here:

1. **It does not fix the 10.** The criterion is *measure, do not fix*, `src/` is outside its write
   scope, and the fix moves the atlas denominator off 49,438 and every bundle figure quoting it, so
   it needs an operator ruling as well as write scope.
2. **The `_settled` aggregates are verified to be aggregates, not verified to agree with the records
   they aggregate.** That is settle-pass fidelity, a different question; no figure here rests on it.
3. **Token overlap is reported as a DISTANCE, never as a reach.** No record is dispositioned
   `reached` by a near-match in this cycle — only exact identity, containment, the same `.lst` row,
   a `.COPY=` twin, or a name an inventory unit carries can do that.
4. **It does not establish that every prose *variant* of a multi-variant `.lst` row prints** — the 7
   above. Reachability, which this criterion measures, is settled for them; rendering completeness
   is not asked.
5. **The denominator 48,864 is cycle 1's and is carried forward unchanged.** This cycle re-verifies
   the *disposition* of the never-reached population, not the enumeration of `data/corpus` itself.

## Build scope verified

**No build was run this cycle, and none is load-bearing for its claim.** The cycle changes no Rust,
no `data/corpus` record, no `data/sheet_rules` file and no inventory unit —
`closed=0 rust_lines_changed=0`, and `git diff --stat 82ffbb4ed2..HEAD -- src data docs/work-inventory.json`
is empty. Per `decisions.md §3` there is nothing for a build to prove. Cycle 1 ran
`cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0` at `83dfcbcfa1` over the identical `src/`;
that result stands unchanged at this HEAD by construction.

The instruments that **do** bear on the claim were all run at HEAD:

- `python3 scripts/completion_atlas.py --check` → `population=49438 buckets=10 unclassified=0 overlap=0`, `DONE: 49438`, `done_evidence_violations=0`, `missing_clearing_mechanisms=0`, `stale_derived_at=False`, `citation_failures=0` — **PASS**
- `python3 scripts/token_coverage.py --check` → `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=233 shapes=1 verdict=PASS`
- `python3 scripts/pcgen_residue_gate.py --check` → `live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS` (run at cycle start **and** at HEAD; identical)
- `python3 scripts/shape_engine_boundary.py --check` → `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True`
- `python3 scripts/missing_engine_tables.py --check` → `population=0 kinds=0 citation_failures=0`
- `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → `0`
- `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` → recorded in the commit message for this receipt's own SHA
- `scripts/verify.sh --only pi-sweep` → recorded likewise
- **Skipped, with the reason:** `cargo run --locked --bin corpus_literal_sweep` (runs only when
  corpus records changed — none did); `cargo run --locked --bin sheet_rule_convert` and `-- --check`
  and the oracle comparison (run only when the converter, its mapping table or the corpus moved —
  none did); `cargo clippy` (no Rust target touched); the desktop crate and frontend (`apps/`
  untouched).

## Sweep population

N/A — no corpus record changed, so `corpus_literal_sweep` has nothing to re-examine.

## Oracle pin

`7f818006e371188e5717fd18d74d18a420747fc6` — confirmed in the checkout this cycle with
`git -C "$PCGEN_REPO_DIR" rev-parse HEAD`, matching `scripts/pcgen-oracle-pin.env`. The pinned tree
was read **read-only**, and the five root-cause figures above are derived from it, each with its
command.

## Status

**partial.**

The **measurement** is finished: there is no undispositioned corpus record left, every disposition
is independently re-verified, and all three gaps cycle 2 named are closed. But the criterion's
population is **not zero at HEAD** — **10 corpus records carrying published rules prose reach no
rendered sheet line** — and returning `complete` over them would be a carve-out, which the standing
rulings forbid.

- **Refused-token remainder, named and summing:**
  `record_absent_from_inventory_population=10` (`pathfinder_unchained/feat=9` +
  `mythic_adventures/spell=1` = **10**). Unchanged from cycle 2, and now **exact** rather than an
  upper bound.
- **Not `blocked-escalated`:** nothing on `workflow-instruction.md §8`'s non-self-healable list
  fired. Clean tree, one writer, no gate regressed, `pcgen_live_files=0` did not rise, one refused
  token type (limit 10).
- **This is not a carve-out.** The 10 are ordinary feats and one mythic spell augmentation with
  printed text; each renders as words under the sheet rule. They are absent because one enumeration
  predicate requires a `TYPE:`/`SCHOOL:` token they do not carry.

## Notes

**A fourth measurement cycle would return 10 and stop.** This criterion is `measure, do not fix`;
its census is now exhaustive and its answer stable across three independent methods. The open item
is not measurement, and `workflow-instruction.md §8` says a criterion behind the corpus is re-scoped,
**not ground**. Per `AGENTS.md` Blocker Discipline this receipt is the raised hand, and the ask is
exact: **write scope to `src/bin/v06_work_inventory.rs` and an operator ruling on moving the
`completion_atlas.py` denominator off 49,438.** Absent that ruling, the operator's other available
disposition is to rule the 10 acceptable on the record — but that is the operator's call, not a
measuring cycle's.

## Next-cycle scope

Criterion's measurement at **zero**. Criterion's measurand at **10**, exact.

1. **Not expressible as `cycle_scope_gate.py` flags** — the gate's population is
   `docs/work-inventory.json` and these 10 records are by definition outside it. A successor claims
   the `decisions.md §2` floor exemption or takes the whole 10 as its named remainder.
2. **No further measurement cycle on this criterion is useful.** The next cycle is either the fix
   (needing the two grants named in Notes) or an operator ruling.
3. The fix is **one predicate**: widen `has_classifying_token`
   (`src/bin/v06_work_inventory.rs:3172-3173`) so a `Kind::Feat` row carrying `DESC:`/`BENEFIT:` but
   no `TYPE:`, and a `Kind::Spell` row carrying `DESC:` but neither `SCHOOL:` nor `CLASSES:`, is
   enumerated. Expected effect: atlas denominator 49,438 → 49,448.
4. **`AT-35-E7-001`'s final-acceptance scan and SD-35's closure must quote the corrected pair:**
   `49,438 of 49,438` is 100% **of the inventory**; the corpus-wide figure is **99.9795%**
   (48,854 of 48,864). Cycles 1 and 2 are left intact as history.
