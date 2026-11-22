# Cycle AT-33-E6-001 (attempt 12) — epic-6-closure / final-acceptance scan, post-fold-fix re-run

- **Commit SHA:** recorded on landing (see `progress.md` entry `sd33-fold-fix-rescan`).
- **Scanned HEAD:** `c0f5e9091e81be39dda82eff4b26061fa82557cf` (`origin/tranche/13`), read in a
  clean `git worktree add --detach` at `.claude/worktrees/sd33-r12-scan` (`git status --porcelain`
  empty at creation and at every checkpoint).
- **Files touched:** this receipt, `progress.md`, `kanban.md` (row 19 + the row-count header),
  `release-notes.md`, `forward-scope-register.md` (§D1.1),
  `docs/retro/sd33-computed-value-verification-retrospective.md`, retro events.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS — `git diff --unified=0 f53b8e32da...HEAD -- src
  apps scripts ':!**/__tests__/**' ':!**/*.test.*' | grep -cE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → **0**
- **Wired-integration audit result:** OK_NO_TOKENS — the same diff's added lines match the token
  pattern **7** times, all seven the English word "placeholder" inside a comment describing real
  upstream data (PCGen's own `p.xx` page placeholder, an `OUTPUTNAME` placeholder, `unmeasurable`'s
  marker). **0** are stub markers; each was read.
- **Acceptance criterion (verbatim):** every criterion `AT-33-E1-001`..`AT-33-E5-003` is `complete`,
  every kanban card is `complete`, `## Open blockers` holds no active entry, and the bundle's own
  gates are green.

## Gate result: **PASS**

Attempt 11 halted on exactly one blocking shortfall — an **ordering** bug, not a data problem: a
live F1 count assertion pinned at 6,260 against a live 6,257, because `cef0ca1b39` regenerated
`docs/work-inventory.json` *after* the lib suite that had validated the previous pin. `c0f5e9091e`
(`fold-fix-repin`) closed it. This scan re-derived the closure independently and found nothing
short.

---

## CHECK 1 — the blocking item is closed, and closed the right way

```
$ python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus
  family rollup:  F1  6257
$ cargo test --locked --lib ; echo LIB_EXIT=$?
  test result: ok. 2845 passed; 0 failed; 14 ignored; 0 measured; 0 filtered out; finished in 57.40s
  LIB_EXIT=0
  test rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::\
       f1_population_matches_the_current_true_formula_bearing_count_not_the_stale_sd32_census ... ok
```

**2,845 of 2,845** lib tests pass, **0 failed**. Attempt 11 measured 2,844 of 2,845 with this exact
test red.

**The pinned constant equals the value this scan derived itself, by a different implementation.**
The test computes F1 from `run_corpus_wide_scan` (Rust); `scripts/shape_ledger.py` computes it from
`docs/work-inventory.json` + `data/corpus` (Python). Two independent implementations, both
**6,257** — the "two independent implementations agreeing" bar (`AGENTS.md`, Concurrency and
Measurement).

**The diff was read, not trusted** (`git show c0f5e9091e -- src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs`):

| Weakening shape | Present? | Evidence |
|---|---|---|
| assertion deleted | no | `assert_eq!(f1.population, 6257, …)` still stands, 1 line changed |
| loosened to a range / `>=` | no | still `assert_eq!` on an exact literal |
| `#[ignore]` added | no | `grep -c '#\[ignore' src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs` → **0**; `git diff f652db7ac7..HEAD` on that file adds **0** `#[ignore]` and removes **0** `assert` lines |
| population narrowed | no | the sibling `a_subset_run_trips_the_population_mismatch_check` (the guard against exactly that) is green in the same run |

**The doc comment carries the new figure**, not the old one: `+42 −9` lines of doc comment added
above the test naming `6,260 -> 6,257 by cef0ca1b39 … an ORDERING bug, not a further content
change`, the three units that left F1's `not_done_population()` gate by id, the re-derive command,
and the rule the re-pin exists to make mechanical ("run the suite after the last write that can
move it, not before"). The assertion **message** carries `6,257` and names the superseded 6,260,
6,278, 6,308 and 6,032 as superseded.

**`docs/work-inventory.json` was not regenerated again:**

```
$ git log --oneline f652db7ac7..HEAD -- docs/work-inventory.json
cef0ca1b39 fix(sd33): fold-inventory -- regenerate docs/work-inventory.json and re-green every gate
00ca087775 fix(sd33): AT-33-E4-002 -- 4,224 unknown units reclassified to zero
```

The most recent write is `cef0ca1b39`, which is an **ancestor** of the re-pin `c0f5e9091e`. The
ordering hazard is closed, not restarted: **0** inventory writes land after the re-pin.

## CHECK 2 — nothing the fold established has moved

```
$ cargo run --locked --bin corpus_literal_sweep ; echo SWEEP_EXIT=$?
corpus-literal-sweep: 48699 records examined of 51473 read, 413288 tokens compared (9 synthesized),
                      51460 digests checked, 0 findings
corpus-literal-sweep: CLEAN
SWEEP_EXIT=0
```

**48,699 examined, 0 findings** — identical to attempt 11, so the +65 folded records are still
inside the examined population. The examined count did **not** drop.

Epic 5, derived as a set from the artifacts themselves rather than read from a receipt:

```
$ <id-keyed Counter + set algebra over the three results files>
fixture   rows=1741 dups=0 {'agree':396,'unverifiable':1345} reasonless=0 disagree=0
literal   rows=6589 dups=0 {'agree':415,'unverifiable':6174} reasonless=0 disagree=0
combined  rows=8330 dups=0 {'agree':811,'unverifiable':7519} reasonless=0 disagree=0
pop(fixture|literal)=8330   combined-rows-set=8330
pop-minus-combined=0        combined-minus-pop=0        fixture&literal overlap=0
```

Rows **1,741 / 6,589 / 8,330**; the unexamined set is **empty in both directions**; **0 of 8,330**
disagree; **0 reasonless `unverifiable` of 7,519**; **0** duplicate `unit_id`s.

```
$ python3 scripts/box_ledger.py --check --oracle-results .../AT-33-E5-003.combined-oracle-results.json
uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False
BOX_EXIT=0
$ jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json   -> 0
$ jq '.units|length' docs/work-inventory.json                                  -> 49438
```

Work-inventory `unknown` **0 of 49,438**.

```
$ ls data/corpus/bestiary_5/race_trait/skinwalker/*.json | wc -l   -> 75
$ <field tally over all 75>
license:   OGL 67,  PI-REDACTED 8
pi_field:  null 67, "description" 8      pi_marker: null 67, "redacted" 8
records with empty data.raw_tokens: 0 of 75
$ grep -c 'DESCISPI:YES' <pinned>/…/skinwalker_abilities_race_subrace.lst   -> 8
```

**75 of 75** Skinwalker records present with `license`/`pi_field` intact; the 8 redactions
cross-check against the oracle's own 8 `DESCISPI:YES` rows.

```
$ cargo test --locked --no-run ; echo NO_RUN_EXIT=$?              -> 0
$ cargo test --locked --no-run 2>&1 | grep -c "Executable tests/" -> 543
$ ls tests/*.rs | wc -l                                           -> 543
$ cd apps/desktop/src-tauri && cargo test --locked   # its own CARGO_TARGET_DIR
  test result: ok. 548 passed; 0 failed; 0 ignored; finished in 94.94s   DESKTOP_EXIT=0
```

**543 of 543** integration targets build; desktop **548 of 548**.

## CHECK 3 — the inherited failing set did not grow. **29 / 46, re-derived.**

```
$ cargo test --locked --no-fail-fast ; echo NFF_EXIT=$?
NFF_EXIT=101
$ <attribute every "test result:" line back to its own "Running" line>
599 "Running" lines + 1 "Doc-tests" line = 600 result lines
total: 7,988 passed, 46 failed = 8,034 of 8,034 executed tests
FAILING SUITES: 29, carrying 46 failures
```

**29 of 599** suites fail, carrying **46 of 8,034** executed tests — exactly the 30/47 attempt 11
measured, minus the one SD-33-owned failure it named. `src/lib.rs` is **green** (2,845/0) and
`src/bin/ingest_races.rs` is **green**; neither appears in the failing set.

Every one of the 29 is proven inherited against the cut SHA with `git`, never asserted:

```
$ for f in <each of the 29 failing target paths>; do
      git log --oneline f652db7ac7..HEAD -- "$f" | wc -l; done | awk '{s+=$1} END {print s}'
0
```

**0 of 29** carry a single commit since the `tranche/13` cut. The 29 are exactly
`forward-scope-register.md §D1.1`'s registered 31 **minus** the two the fold fixed
(`src/bin/ingest_races.rs`, `tests/sd27_alternate_racial_trait_reachability.rs`) — set membership
checked target by target, not by count. **0 failures outside the inherited set**, so nothing here
is SD-33's.

Full failing set, with per-target pass/fail:
`duergar_invisibility_sla_reaches_a_player_via_monster_codex` 6/1 ·
`formula_interpreter_family_fixture_check` 4/1 · `no_foreign_home_paths` 2/1 ·
`sd13_sorcerer_level{1,9,10}` 18/1, 11/1, 11/1 ·
`sd18_cleric_level{11,12,13,14}_widening` 8/2 each · `sd18_cleric_level{15,16,17}_widening` 8/1 each ·
`sd18_cleric_level{18,19,20}_widening` 7/2 each ·
`sd24_identifier_discipline_audit` 0/1 · `sd24_wired_integration_audit` 3/2 ·
`sd26_cache_acg` 7/2 · `sd26_cache_apg` 5/2 · `sd26_identifier_discipline_audit` 0/1 ·
`sd27_ability_automatic_granted_race_traits` 4/2 · `sd27_advanced_race_guide_cache_shape` 6/2 ·
`sd27_book_license_record_counts` 4/2 ·
`sd27_equipment_modifier_price_matches_corpus_cost_token` 1/2 ·
`sd27_known_spells_must_be_on_the_class_spell_list` 5/1 ·
`sd30_declared_product_identity_in_shipped_class_features` 2/1 ·
`sd31_class_feature_corpus_key_uniqueness` 1/1 · `v06_corpus_trap_report` 21/4.

## CHECK 4 — the fold's own quality holds

### Skinwalker → pinned oracle. **75 of 75 traced, not 2.**

Pin confirmed first: `scripts/pcgen-oracle-pin.env` names
`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`;
`git -C $PCGEN_REPO_DIR rev-parse HEAD` → identical. `sha256sum` of
`…/skinwalker_abilities_race_subrace.lst` →
`a5b5c3f65d28e5a01b935ffc2a177752184be170e340fdddc14cdbf3ba6c3b27`, matching every record's cited
`source.sha256`.

The dispatch asks for 2 spot-checks; a full trace costs the same and covers more, so **all 75** were
tab-split against their own cited line and compared token-for-token
(`data.raw_tokens` + `data.raw_bonus_chains` vs the line's fields 2..n):

```
records traced OK to pinned oracle line: 67 of 75
mismatches: 8   -- all 8 are exactly the 8 PI-REDACTED records, and in each the ONLY
                   difference is DESC:<real text>  ->  DESC:[redacted PI]
```

**67 of 75** are byte-identical; the other **8 of 8** differ in exactly one token, the redaction
that `pi_field:"description"` declares. **0 of 75** carry a token the oracle line does not, and
**0 of 75** drop one. Nothing was synthesized.

Two hand-traced in full, chosen by this scan (different records from attempt 11's):

- **`weretiger_kin_change_shape`**, cited line **165**. Line carries a display name plus **8**
  tokens (`KEY`, `CATEGORY`, `TYPE`, `PREABILITY`, `DESC`, `ABILITY`, `SOURCEPAGE`, `FACT`); the
  record carries **8 of 8** `raw_tokens`, byte-for-byte, in order, and nothing else. **Verified.**
- **`wererat_kin_spell_like_ability`**, cited line **118**. Line carries a display name plus **10**
  tokens; the record carries the **8** non-`BONUS` tokens verbatim and both `BONUS:VAR` chains in
  `raw_bonus_chains` (`RacialSLA_SpeakWithAnimalsRodentsOnly_Times|3|TYPE=Base`,
  `…_DCMod|WIS-CHA`) — **10 of 10** accounted for. **Verified.**

### Undine fixtures execute, and are not a mirror

```
$ <len(race_trait_formula_entries), sum(len(expected_at_sample_points))>  -> 3, 30
   undine_acid_breath / undine_nereid_fascination / undine_ooze_breath, 3 formulas × 10 points each
$ cargo test --locked --lib | grep race_trait_formula_bar_check_tests
   run_race_trait_formula_bar_check_clears_every_committed_fixture ... ok
   a_mutated_evaluator_is_caught_by_the_race_trait_formula_gate ... ok
   a_transcription_regression_in_the_shipped_table_is_caught ... ok
```

**3 of 3** entries, **30 sample points**, **90 scalar assertions**, all executed in this scan's own
`--lib` run. `run_race_trait_formula_bar_check` refuses a fixture that pins no sample point
(`fixture.expected_at.is_empty()` → recorded as a failure, not counted as cleared), so the gate
cannot pass vacuously.

**Fixture discipline, checked at the source rather than from the receipt:**
`scripts/derive_race_trait_formula_fixtures.py` imports only `argparse, hashlib, json, math, os,
re, sys, pathlib` — **no engine module, no Rust table, no file under `data/corpus/`** (the artifact
the engine evaluates). The expected values are computed by its own per-formula Python functions and
then checked against `PcgenFormulaEvaluator` (Rust). The bar check additionally asserts the
**shipped** `UNDINE_RACE_TRAIT_FORMULAS` text equals the fixture's oracle-derived formula text, so
a transcription drift in the shipped table fails rather than silently agreeing. **Not a mirror.**

### Stale prose, re-derived rather than accepted

```
$ find data/corpus -type d -name race_trait | while read d; do find "$d" -name '*.json'; done | wc -l
910
```

`src/bin/v06_work_inventory.rs:4308`'s "every one of the **910** currently-ingested `race_trait`
records" is now correct (was 831). Swept for the other candidates as **live** figures:

```
$ grep -rn '\b<n>\b' --include=*.rs --include=*.py --include=*.ts --include=*.tsx src tests apps scripts
  831   -> 6 hits, every one inside a `<old> -> <new>` provenance chain ("824 -> 831", "831 -> 910")
  6260  -> 1 hit, EquipmentTableEntry cost_gp 6260.0 for "Enchanted Eelskin" (unrelated)
  6278 / 6308 / 48634 / 6032 -> 0 hits
```

**0 stale live assertions remain.**

**`race_resolver.rs:3105`'s "370" is confirmed NOT stale**, and this scan seconds `fold-fix-repin`'s
correction of attempt 11's own RISK 2 finding. The sentence reads "…SD-33 Epic 6's 45 folded
Skinwalker heritage records … every one resolves with no inert flag, **same as the other 370**" —
415 − 45 = **370** is exactly the count of the *others*, so rewriting it to 415 would make the
sentence self-referentially wrong. The two live assertions on that number are correctly at **415**
(`race_resolver.rs:3382`, and the running-total comment at `:2680`), and both are green in this
scan's `--lib` run. **Attempt 11's RISK 2 was wrong on this item; the correction stands.**

### Gates re-proven to still fail, then returned to baseline

**Denominator gate** — planted inside its own scanned scope:

```
$ <append "PROBE: coverage is 100% complete." to fold-fix-repin_cycle_receipt.md>
VIOLATION .../fold-fix-repin_cycle_receipt.md:237: PROBE: coverage is 100% complete.
files_checked=68  violations=1
$ git checkout -- <that file>
files_checked=68  violations=0    git status --porcelain -> (empty)
```

**0 violations of 68 files** at baseline (67 at `fold-fix-repin`, which ran before its own receipt
was committed; +1 is that receipt).

**`box_ledger.py` disagreement detection** — planted on a **scratch copy**, never on the shipped
artifact:

```
$ <flip results[0].verdict to "disagree" into a scratch file>
uncovered=0 overlap=0 population=49438 oracle_disagreement=1 …
ORACLE_DISAGREEMENT: ultimate_equipment:equipment:belt_of_mighty_hurling_greater
$ rm <scratch>   git status --porcelain -> (empty)
```

**`corpus_literal_sweep` detection, re-proven live on a folded record.** Probe hygiene per the
dispatch: **no `cp -al`**. The mutation was written **in place inside this scan's own detached
worktree** — a real, separate checkout, so no inode is shared with the primary tree — and restored
with `git checkout --`:

```
$ <append {"key":"SOURCEPAGE","value":"p.888888"} to wereshark_kin_change_shape.json's raw_tokens>
$ git -C <primary tree> status --porcelain -- data/corpus     -> (empty)   # no hardlink bleed
corpus-literal-sweep: 48699 records examined …, 413289 tokens compared …, 1 findings
corpus-literal-sweep: MISMATCH data/corpus/bestiary_5/race_trait/skinwalker/wereshark_kin_change_shape.json:
                      token not byte-present in corpus token closure: SOURCEPAGE:p.888888
$ git checkout -- <that file>
corpus-literal-sweep: 48699 records examined …, 413288 tokens compared …, 0 findings   CLEAN
$ git status --porcelain (scan worktree) -> (empty)
$ git status --porcelain -- data/corpus (primary tree) -> (empty)
```

Tokens compared moved 413,288 → **413,289** (the one planted token) and back. **Both trees verified
clean after.** No residue.

## The rest of the scan

**Criterion coverage.** `grep -oE 'AT-33-E[0-9]-[0-9]{3}' epic-breakdown.md | sort -u` and the same
over `kanban.md` return the **identical 21 ids**. No criterion exists that the board does not carry.

**`kanban.md` — 24 of 24 rows `complete`** after this cycle flips row 19. Rows 1–18 (every
`AT-33-E1-001`..`AT-33-E5-003`) and 20–24 were already `complete`; row 19 was `in-progress`, set
there by `fold-fix-repin` because that lane closed the shortfall without re-running the scan. No
row is `not-started`, `blocked-escalated`, or `returned-to-backlog`. The header's own "**21 rows**"
is corrected to **24 rows** in this cycle — a stale figure the denominator gate does not check.

**`## Open blockers` — 0 active entries.**

```
$ grep -n "^## Open blockers" progress.md                -> 302
$ awk 'NR>302 && /^## /{print NR; exit}' progress.md     -> 470   (## Cycles)
$ sed -n '302,470p' progress.md | grep -n "^###\|^<details>\|^</details>"
  50:<details>  53:### corpus_literal_sweep mismatch on 10 weapon records …  83:</details>
  98:<details>  101:### rending_claw_blades … EQMOD-resolution gap …         167:</details>
```

Both `###` entries lie **inside** `<details>` historical blocks and the section's own text records
both as CLEARED. **0 active of 2 historical.** No fold or fix lane filed a new one.

**Open deferrals — 3 open, 0 of 3 defer live DoD scope.**
`python3 scripts/retro.py summary --since 2026-08-24 --json` → `deferrals.open = 3`, the same three
attempts 10 and 11 left standing (`REGISTERED_POOL_GROUPS` widening;
`pf1e_dashboard_producer.py` `unmeasurable` recognition; the COMBAT non-AC aggregation surface).
Each carries a revisit condition (**3 of 3**), and each is a capability deferral registered in
`forward-scope-register.md`, not scope that was in this bundle's Definition of Done.

**Retrospective written and cited** (`AT-33-E6-002`):
`docs/retro/sd33-computed-value-verification-retrospective.md` exists and is cited from
`references/README.md §1`.

**`scripts/verify.sh` not run end-to-end.** Its `site-dashboard-check` stage hangs on the
producer's unbounded `v06_work_inventory` call — root-caused and registered at
`forward-scope-register.md §D1.2` by attempt 10, unchanged. This verdict does not rest on it: every
load-bearing stage was run directly and is reported above (`corpus-sweep`, `denominator-gate`,
`root-lib`, `desktop`), plus `--no-run` and the full `--no-fail-fast` workspace run.

---

## Movement, four buckets

- **Closure 1.** `AT-33-E6-001` is satisfied at `c0f5e9091e`; kanban row 19 → `complete`, the
  bundle's 24th and last card.
- **Reclassification 0.**
- **Reachability 0** (a scan; it wired nothing).
- **Instrument-correction 3**, all figure corrections in shipped closure prose, each re-derived
  here rather than copied:
  1. `release-notes.md`, `forward-scope-register.md §D1.1` and the retrospective's §5 carried the
     inherited-debt figure **31 of 599 suites / 49 of 8,026 tests**, true at attempt 10 and stale
     since the fold. Corrected to **29 of 599 / 46 of 8,034**, with the reason named: the fold
     *fixed* two inherited targets outright and the executed denominator grew by 8. This is a
     shrink of inherited debt, **not** a reclassification of it (`workflow-instruction.md §12`
     row 10 / `decisions.md §2`).
  2. `forward-scope-register.md §D1.1`'s explicit 31-target list dropped the two now-green targets;
     the remaining 29 were checked against the failing set member by member, not by count.
  3. The retrospective's "nine correct halts" narrative is amended for the fold re-open: **ten
     halts**, attempts 1–9 plus attempt 11, every one a correct refusal.
- **Correction to a prior receipt (not a bucket, recorded for the reader):** attempt 11's RISK 2
  reported `race_resolver.rs:3105`'s "370" as a stale prose figure. It is not stale. This scan
  re-derived it independently and agrees with `fold-fix-repin`, which had already refused to
  "fix" it.

## Status: complete

- **Notes:** Twelve scans have run on this bundle. Ten halted it and every halt was correct; two
  passed. Attempt 10's PASS was correct for the tree it scanned, and the operator's fold ruling
  then changed the tree and carried one real regression with it — a stale count assertion produced
  purely by run order, which attempt 11 caught and `fold-fix-repin` closed by re-pinning to a
  freshly derived value with the mechanism written into the test's own doc comment. This scan
  confirms the re-pin is a re-pin and not a weakening (assertion intact, exact-equality, no
  `#[ignore]`, guard test green), that no further inventory write landed after it, and that
  everything the fold established still holds. The one place this scan went beyond attempt 11 is
  the Skinwalker trace: all **75 of 75** records were traced to the pinned oracle rather than 2,
  and the only deviations are the 8 declared PI redactions. Three stale figures in shipped closure
  prose were found and corrected with their re-derive commands.
- **Next-cycle plan:** none owed by this criterion. `release-notes.md` and PR #377 carry the
  corrected inherited-debt figure; the bundle is closed at 24 of 24 cards.
