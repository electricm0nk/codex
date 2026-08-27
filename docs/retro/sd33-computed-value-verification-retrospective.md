---
canonical: true
owner: closure-epilogue
purpose: SD-33 retrospective, grounded in `python3 scripts/retro.py summary --since 2026-08-24 --json`
  rather than recollection.
date: 2026-08-26
board: kanban rows 1-21 `complete` at first closure (`AT-33-E6-001` attempt 10 PASS, `AT-33-E6-002`,
  `AT-33-E6-003`); the operator's 2026-08-26 fold ruling reopened Epic 6 with rows 22-24
  (`fold-skinwalker`, `fold-undine`, `fold-inventory`) and re-ran row 19 (attempt 11 FAIL, attempt
  12 PASS). All **24 of 24** rows `complete` as of this update — see §6 below.
---

# SD-33 retrospective — computed-value verification

Ten dispatch waves, 2026-08-24 through 2026-08-25, closing on `AT-33-E6-001` attempt 10's PASS at
`origin/tranche/13` = `4a84b567e0`. Every number below is re-derivable — `python3 scripts/retro.py
summary --since 2026-08-24 --json` for the retro-log figures, or the cited command for anything
else. `deferrals.open` **is trustworthy in this window**: `grep -n 'len(open_deferrals)'
scripts/retro.py` → line 772, `"open": len(open_deferrals)`, not `deferrals[-limit:]` — the SD-32
defect (`decisions.md §2` item 1) is fixed and stayed fixed through this whole bundle. Re-derived
live this cycle: `deferrals.open = 3` (down from attempt 9's 8 — see "Ledger hygiene" below).

## Why this bundle existed

SD-33 answered one question: **is a computed value that looks right actually right?** SD-32 closed
8,330 units to `done` against fixtures the program wrote itself, never against an external oracle.
A wrong number that matches a wrong fixture looks identical, in every dashboard, to a right one.
SD-33 built the oracle harness and ran the whole blessed population through it. It found real wrong
answers, and it fixed them.

**That is the headline. The process that produced it is the second story, not the first.**

---

## 1. The defects — what SD-33 actually caught, each with its mechanism and its commit

Every one of these was a computed value the program had already marked `done`, wrong in a way that
compiled clean and passed every prior test.

1. **The spell-DC fixture defect — 103 units, correct by accident.** `fixture-generate-spell-batch.py`'s
   `.pcg` template pinned `STAT:WIS|SCORE:10` for every casting class. It was silently right for
   Intelligence/Charisma-cast classes (their save DC never reads the WIS score) and silently wrong
   for every Wisdom-caster fixture — the harness was comparing against its own miscalibrated input,
   not the real spell-save-DC formula. Fixed: `dded72f0b4` (`STAT:WIS|SCORE:10` → `18`, matching the
   probe's own pinned ability array; all 103 disagreements traced to this one template line and
   re-derived clean).
2. **The armor compute defect — 22 units.** `compute_arms_armor_effect`/`compute_var_effect` never
   resolved a base item's `EQMOD:`-referenced modifier record's own separate `BONUS:` chain — the
   armor engine simply did not know a referenced modifier could carry its own bonus. New
   `equipment_effects::eqmod_referenced_records` resolver plus one `TYPE=Circumstance` exclusion fix.
   Commit `abc72f75ec`, 70/70 `equipment_effects` tests green (4 new).
3. **The AC measurement method — 4 units, a wrong ruler, not a wrong engine.** The harness's
   whole-character `AC.TOTAL` diff conflated an item's own AC bonus with a second-order `MAXDEX`
   cap loss or a co-located Dex-enhancement chain riding the same record. The *engine* was right;
   the *measurement* wasn't isolating the dimension it claimed to measure. Fixed by replacing the
   whole-character diff with an absolute per-type isolator (`AC.Armor`), re-running the full
   already-judged 66-unit population at 0 disagree. Commit `a68fbeea3d`.
4. **`compute_equipmods_effect` multi-chain summing — 1 unit (`heavy_hammer`), a real engine
   defect.** A record with two separately-scoped `BONUS:` chains only had the first one summed.
   `WeaponEnhancementBonus` split into independent `tohit_bonus`/`damage_bonus` fields; corpus-wide
   scan confirmed `heavy_hammer` was the only affected record. Commit `2f1d52f22d`.
5. **`equipment_id_resolve` OUTPUTNAME-divergent identity fix.** A templated-variant record's
   engine identity resolution failed because its `OUTPUTNAME` diverged from its base key — the
   resolver was matching the wrong axis. RED→GREEN engine fix landed in the skill-combat wave-5
   lane (`AT-33-E5-last67-skill-combat`).
6. **The campaign-KEY-vs-display-name harness defect.** The `ultimate_psionics` oracle-comparison
   harness was loading campaigns by display name where the pinned oracle indexes by KEY —
   comparisons against that book silently ran against the wrong campaign context. Fixed in the same
   skill-combat wave-5 lane.
7. **The corpus-extraction gap that dropped `.MOD`-attached EQMOD references — 139 records / 9
   books, plus `rending_claw_blades`'s own two-fold engine gap.** `fbc945f198` fixed the extraction
   pipeline itself (a record's second `.MOD`-attached `EQMOD:` token was never folded into
   `raw_tokens` at all). That regeneration then exposed two further `src/rules_core/` defects on
   `rending_claw_blades`: `eqmod_referenced_records` read only the record's *first* `EQMOD:` token
   via `.find()`, and `compute_equipment_effects`'s weapon path never folded `EQMOD:`-referenced
   modifier records' weapon-enhancement chains at all (the AC dimension already had this pattern;
   the weapon dimension didn't). Both closed via a new
   `equipmods::apply_eqmod_weapon_enhancement_bonus`, combining by MAX not sum (same-`TYPE=Enhancement`
   stacking, live-oracle-confirmed `MAGICHIT=+1`, not `+2`). Commit `7d439876b7`.
8. **Two previously-unwired resolvers landed RED→GREEN.** `EQMWEAPON|DAMAGESIZE` and
   `EQM|WEIGHTDIV` — neither had ever been handled anywhere in `src/rules_core/` before the
   wave-6 `eqm-modifier-final` lane. A genuinely new modifier-application mechanism (EQMOD baked
   into a homebrew LST item at load time, distinct from the `.pcg`-time `CUSTOMIZATION:` block wave
   5 had already proven broken for).
9. **Epic 4's reclassification turned the lib suite red on an unmapped doneness pair — a DoD
   defect the bundle closed rather than handed on.** `AT-33-E4-002` (commit `00ca087775`) widened
   doneness classification and left 3 of 2,836 lib tests red on an unmapped `(ambiguous,
   unmeasurable)` verdict pair, 11 of 49,438 units. Closed inside this bundle
   (`AT-33-E6-001-suite-green_cycle_receipt.md`), not handed to a successor — a fail-closed raise
   still fires for any genuinely-unmapped pair; only the 11 known ones were mapped.
10. **A struct rename in Epic 5's own commit hid 543 of 543 integration targets behind a compile
    error.** `2f1d52f22d` (`AT-33-E5-finalize-wave5`) split `WeaponEnhancementBonus`'s `affects`/
    `bonus` fields into `tohit_bonus`/`damage_bonus` and two test files outside the commit's own
    diff kept the old field names — `tests/sd20_equipment_equipmods.rs` and (found only because the
    *full* workspace build ran, not a name-grep, since the second site bound the struct through a
    local variable named `enhancement`) `tests/sd20_tabletop_readiness_integration.rs`. **0 of 543**
    integration targets executed until this was fixed. Both rewrites strengthened the assertions
    (two typed fields instead of one string) rather than weakening them.
11. **The two disagreeing `.MOD`-chain token derivations the corpus sweep caught once enrichment
    made the records measurable.** `corpus_literal_sweep.rs`'s own `.MOD`-identity closure-builder,
    not `enrich_equipment_raw_tokens.rs`, was wrong on two shapes: a whole-book unsorted
    `read_dir` walk resolved a `.COPY=` base to the wrong same-named record two files away
    (`hellscourge` → the profession-feat decoy `Scorpion Whip` in `ue_profs_weapon.lst` instead of
    the real base three lines later in the *same* file as the `.COPY=` row), and a PI-redaction
    re-screen exclusion (`token.key != "DESC"`) suppressed a legitimate redaction the corpus record
    had correctly applied. Both confirmed by hand-derivation against the pinned oracle `.lst` bytes,
    independent of either program's own claim. `data/corpus/**` untouched — 0 of 6 files in the fix
    commit under that path; 105 findings → 0.
12. **The gate's own scan SCOPE was itself a defect, found by the scanner and closed by an
    instrument lane.** Attempt 9's shortfall was not a corpus bug — it was `corpus_literal_sweep`
    the *tool* being wrong about 10 of its own 137 newly-regenerated records. The final-acceptance
    scan does not fix code; it found the shortfall and a dedicated instrument-fix lane (not the
    scan itself) closed it, verified against all four illegitimate routes to the same zero
    (population narrowed, `raw_tokens` reverted, an exclusion added, `data/corpus/**` hand-edited —
    none of the four happened).

## 2. The throughput arc

Examined-of-8,330, by wave:

```
32 -> 6,940 -> 7,939 -> 8,255 -> 8,263 -> 8,291 -> 8,330 of 8,330 examined
```

Disagreements surfaced, by wave: **26 -> 4 -> 1 -> 0.**

Every jump exposed new defects, never fewer: wave 1's 32 units found nothing (too small a sample);
6,940 surfaced 103 (the WIS-fixture defect, item 1 above); 7,939 held; 8,255 surfaced 26 (22 fixed
same wave — item 2 — 4 escalated); 8,263→8,291 fixed those 4 via the measurement-method rewrite
(item 3) while the *newly-reachable* 28 units surfaced 2 more (`heavy_hammer`, item 4, and
`rending_claw_blades`, item 7); 8,330 closed the population and confirmed 0 disagree. **Coverage
growth surfaces defects late** — every disagreement in this bundle appeared in a shape the previous
wave had not yet reached, never in a shape already judged.

**Ten dispatch waves, nine correct halts, one bundle.** `AT-33-E6-001`'s final-acceptance scan
failed nine times before it passed. Every failure was correct: attempts 1-5 found real unrowed
population or unresolved disagreements; attempt 6 found a lib-suite regression from Epic 4's own
commit; attempts 7-8 found a broken workspace build (0 of 543 integration targets executing) and
then, once that was fixed, two further real test-file gaps the broken build had been hiding;
attempt 9 found the corpus-sweep instrument bug (item 11/12 above) filed as a blocker rather than
cleared. **Nine scans failed and each failure was the system working** — the lanes wrote honest
short rows rather than false greens, with two exceptions, **both caught mechanically**:

- **Wave 2's equipment lane** (`AT-33-E5-002`) examined 103 of its own 494-unit population and its
  receipt states the remainder plainly (`"Remaining 391 ... stay named per-shape"`) — the near-miss
  was upstream of that receipt, in an earlier draft of the row's own status text claiming coverage
  the row-count did not support. **Caught by row-counting**: the scan's per-file
  `len(results)`-vs-population check (Check 4/Check 1 pattern in every attempt-N receipt) makes a
  short population mechanically visible regardless of what the prose around it claims.
- **Wave 7's mis-attribution.** A failing suite was reported as pre-existing/environmental without
  re-deriving it against the cut SHA. **Caught by re-deriving from `git`**: attempt 8/9/10's Check 2
  (`git log f652db7ac7..HEAD -- <target>` for every failing target, summed to 0) is exactly the
  mechanism that later confirmed **31 of 31** genuinely-inherited targets *and* would have caught a
  false claim the same way.

---

## 3. Lessons, each with its enforcing mechanism

Per `decisions.md §4`: a lesson without a mechanism is a quote.

- **a. Measure per-unit cost before a population-scoped run.** *Mechanism:* a required
  dispatch-brief field — measured cost, population, projected wall time — filled before the run
  starts.
- **b. A remainder named per-MECHANISM is closable; "the rest" is not.** *Mechanism:* a required
  per-shape enumeration of anything unexamined (rows 17/18's own "23 weapon-shape + 9
  skill-combat-shape + 7 eqm-shape" tables are this lesson in practice, not prose about it).
- **c. A lane's status must be a mechanical function of its row count.** *Mechanism:* the scan
  counts rows and derives the unexamined SET, both directions (`ROWED_NOT_BLESSED` as well as
  unblessed-not-rowed), every attempt.
- **d. Coverage growth surfaces defects late.** Disagreements appeared only in newly-covered
  shapes (§2 above); a bundle stopping at 95% would have shipped every one of items 2, 4, and 7.
- **e. A method carried past its limit is this bundle's recurring failure shape.** *Mechanism:*
  change the method and RE-RUN EVERYTHING IT ALREADY JUDGED; the scan verifies that re-run. The
  AC-measurement fix (item 3) re-ran the full already-judged 66-unit population, not just the 4
  disagreeing ones, and the corpus-sweep fix (item 11) re-ran all 48,634 examined records, not just
  the 10 known-bad ones.
- **f. A blocker whose fix lives in another subsystem is still a fix.** *Mechanism:*
  `blocker-closure-doctrine.md`'s two dispositions, enforced by the scan reading `## Open
  blockers`. This bundle filed three blockers over its run and cleared all three by decomposing
  them: the corpus-extraction gap (item 7), the build break (item 10), and the corpus-sweep
  instrument bug (item 11/12) — none deferred, none handed on.
- **g. A count change compiles clean and leaves other assertions red.** *Mechanism:* a required
  count sweep across `tests`/`src`/`apps`/`scripts`. Item 10 is this lesson's own worked example:
  the struct rename compiled, and a *second* stale test file survived a targeted name-grep and was
  only found by the full `--no-run` build across the whole workspace.
- **h. Verify at the widest build scope the repo has.** *Mechanism:* `cargo test --no-run` plus the
  full workspace run in the scan, and `apps/desktop/src-tauri` tested explicitly as the separate
  workspace it is (548 of 548, every attempt, unchanged since the cut).
- **i. A lane's attribution of a failure is a claim, not evidence.** *Mechanism:* the scan
  re-derives attribution from `git` against the cut SHA. It caught wave 7's false "pre-existing"
  claim and, on the same mechanism, confirmed a true one covering 31 suites / 49 tests at attempts
  8, 9, and 10 alike — 0 of 31 ever carried a commit since `f652db7ac7`.
- **j. A vacuous pass is not a pass.** The corpus sweep was green on records whose `raw_tokens`
  were `[]` — its population was "tokens the record claims", and a record claiming nothing cannot
  mismatch. Enrichment made them measurable and the disagreement appeared instantly (item 11).
  *Mechanism:* state the population of every gate, and treat an empty population as unmeasured
  rather than passing.

**The gate's own scan SCOPE was a defect, found by the scanner and closed by an instrument
lane** — recorded here per the dispatch instruction, and expanded as item 12 above: the scan does
not fix code, it finds the shortfall; a separate instrument-fix lane did the reconciliation and the
scan re-verified it on the next attempt.

### `workflow-instruction.md §12` rows 3 and 8 — closed

Both were marked `UNENFORCED` at launch.

- **Row 3 — "Dispatch first, report second."** Closed by making it a receipt field: every
  attempt-N final-acceptance receipt in `epic-6-closure/` carries an explicit `Next-cycle plan`
  line naming the very next dispatch (attempt 9's plan named the reconciliation cycle that became
  item 11; attempt 10's names `AT-33-E6-002` then `AT-33-E6-003`) — a mechanical trace from one
  cycle's end to the next cycle's start, not a stated intention.
- **Row 8 — "Carve-out sweeps grep code, not only prose."** Closed with a live grep, re-run this
  cycle: `grep -rnE "EXCLUDED|EXCLUDE_|SKIP_BOOKS|ALLOWLIST|SKIPLIST|IGNORE_BOOKS|beginner_box"
  src/bin/corpus_literal_sweep.rs src/rules_core/corpus_literal_sweep.rs scripts/box_ledger.py
  scripts/denominator_gate.py src/bin/v06_work_inventory.rs` → **0 hits in either sweep file**; the
  one inventory-side hit (`out_of_scope = ["core_essentials"]`) is verbatim-inherited from the
  `tranche/13` cut (`git show f652db7ac7:src/bin/v06_work_inventory.rs | grep -c
  '\["core_essentials"\]'` → `1`), confirmed by attempt 10 and re-confirmed here. This grep is now
  standing practice inside `AT-33-E6-001`'s own scan procedure (route (c) of the four illegitimate
  routes checked every attempt) rather than a one-off; the `verify.sh` stage promised in
  `decisions.md §6`'s corollary is filed forward — see `forward-scope-register.md` — because
  wiring a new stage is net-new tooling scope, not a re-run of the existing grep.

---

## 4. Ledger hygiene

Attempt 10 closed four stale `deferral` events with `resolution` events — each one had named scope
that *was* in the Definition of Done when written and was since completed to a commit, but whose
ledger row was never closed:

| Deferral | Named at | Closed by |
|---|---|---|
| Live oracle verification of 67 units | row 17, wave 4 | wave 6: literal rows 6,589 of 6,589 |
| 13 unexamined weapon-shape units | row 17, wave 5 | wave 6: same |
| 10 unexamined shape-B/C/D units | row 17, wave 5 | wave 6: same |
| Corpus-extraction `.MOD`-EQMOD fix + regen | row 18, wave 5 | wave 6: `fbc945f198`, 137 records |

`deferrals.open` moved **8 → 4** (attempt 9 → attempt 10) on those four resolutions, then **4 → 3**
this cycle as `sd33-r8-build-green`'s corpus-sweep deferral resolved against item 11/12's fix
(`sd33-r9-corpus-sweep.jsonl`'s own `resolution` event, confirmed above). The **3 still open**
(2 `sd33-e4-unknown` capability deferrals, 1 `sd33-r6-skillcombat` engine-surface deferral) — **0 of
3 defer live DoD scope**, each carries a revisit condition, and none is this bundle's to clear;
they are registered in `forward-scope-register.md`.

---

## 5. Inherited debt — registered, not vanished

**29 of 599** test suites, carrying **46 of 8,034** executed tests, were proven — not assumed —
pre-existing at the `tranche/13` cut. Re-derived independently by attempts 8, 9, 10, 11 and 12
(five cycles, five separate worktrees, the same result each time): every failing target's
`git log f652db7ac7..HEAD -- <target>` is empty, and the failing set's shape (same targets, same
per-target pass/fail counts, same order) is byte-identical to a clean run at `f652db7ac7`. **0 of
29 carry a single commit since the cut.**

**The figure was 31 of 599 / 49 of 8,026 through attempt 10 and is corrected here**, not
reclassified: the operator's 2026-08-26 fold ruling **fixed** two of the inherited 31 outright
(`src/bin/ingest_races.rs` and `tests/sd27_alternate_racial_trait_reachability.rs`, both green at
attempt 12), and the executed denominator grew by the fold's own 8 new cases. Re-derive:
`cargo test --locked --no-fail-fast`, attributing every `test result:` line back to its own
`Running` line (`AT-33-E6-001-attempt12_cycle_receipt.md`). This bundle verified their inheritance by execution
rather than accepting a predecessor's claim of it — see `forward-scope-register.md` for the full
proof commands and counts, entered there this same cycle per the dispatch instruction.

---

## What actually closed the bundle

Nine consecutive scan failures, each a real and different shortfall, none deferred and none argued
around. (A tenth followed: the operator's 2026-08-26 fold ruling re-opened Epic 6, attempt 11
caught a real regression the fold had introduced — a live F1 count assertion left stale at 6,260
against a live 6,257 by a `docs/work-inventory.json` regen that landed after the lib suite ran —
and attempt 12 passed once `fold-fix-repin` closed it. **Ten halts is now the count** — attempts
1-9 plus attempt 11 — and every one of the ten was a correct refusal.) The tenth attempt passed because the ninth attempt's one shortfall — a tool bug in the
verification instrument itself — was genuinely fixed and re-verified against every route that could
have produced the same "0 findings" dishonestly. No card in this bundle ever closed by narrowing a
population, weakening an assertion, or filing a blocker in place of clearing it. Blocker Discipline
(`AGENTS.md`) held for real, ten times: **clear it, or raise your hand and wait** — this bundle only
ever took the first branch.

## 6. The fold — recovered SD-31 work, and its own lessons

The bundle closed once, at attempt 10 (`1bfb80d7b7`), with PR #377 open and not yet merged. Before
the PR merged, a sweep of stale local branches — the same kind of sweep `AT-33-E6-002` ran to
prune worktrees — found two branches holding real, unmerged content generated during SD-31 and
lost when a wave-11 API error dropped the session that produced it: **45 hand-copied Skinwalker
`race_trait` records** and **103 Undine fixture-file string occurrences (3 real fixture entries)**.
The operator ruled: fold the genuinely-unique content into SD-33 rather than let it ride into
SD-34 as an unexplained gap or force a from-scratch re-derivation of work that already existed.
Neither branch was merged on trust — each was regenerated through its guarded generator path and
independently hand-traced against the pinned oracle before landing (`fold-skinwalker` produced
**65** real records from the branch's 45, all 75 on-disk records token-traced clean;
`fold-undine`'s 3 entries were re-verified against HEAD's own corpus, byte-for-byte). Full account:
`../release/SD-33-computed-value-verification/release-notes.md` "Recovered work" and
`fold-skinwalker_cycle_receipt.md`/`fold-undine_cycle_receipt.md`.

The same sweep found three other branches and ruled them **out** — recorded in
`forward-scope-register.md §E1` so SD-34 does not re-litigate them.

Folding after closure was not free: reopening Epic 6 cost two more final-acceptance-scan attempts
(11 FAIL, 12 PASS) — bringing the bundle's total correct halts to **ten**: attempts 1 through 9,
plus attempt 11. Attempt 11's shortfall was the fold's own, not inherited (3 commits since the cut
touched the file the failing assertion lived in, against 0 for every genuinely-inherited target),
and it is this reopen's own worked example of three lessons, each with what makes it fail:

- **A stale branch's file count is not its value.** The 1,612-file grant branch
  (`worktree-wf_a45ece26-3fc-1`) looked like the biggest recovery available in the sweep and was
  superseded — its `class` field held feature-group names where HEAD's curated records hold real
  class names, and it lacked `granted_via_archetype`, a field the live consumer treats as
  authoritative and defaults to `true` when absent, so folding it would have silently mis-marked
  every record. The 45-record Skinwalker branch looked like the smallest, most marginal find in
  the same sweep and was the real one. **What tells them apart is never the file count** — it is
  reading the branch's own record schema against HEAD's current schema, and checking whether the
  live consumer requires a field the branch's records don't carry. A count comparison alone would
  have picked the wrong branch to trust.
- **Run the suite after the last write that can move it.** `fold-skinwalker` re-pinned F1's
  population correctly (6,278 → 6,260) against the `docs/work-inventory.json` committed at that
  moment, and its own receipt reported `cargo test --locked --lib` green — a true measurement of
  the tree it actually landed. The *next* commit, `fold-inventory`, then regenerated that same
  inventory file for an unrelated reason and moved F1 again (6,260 → 6,257) without re-running the
  lib suite afterward, leaving a stale `assert_eq!` red at HEAD. The receipt that reported "0
  failed" was correct and still produced a red tree one commit later, because a later write moved
  the number the assertion depended on. The mechanism is not "run the suite" — every cycle already
  does that — it is **run it after the last commit in the chain that can move the figure being
  asserted**, which for an inventory-regeneration step is not necessarily the step's own commit.
- **A gate's examined-population must grow when records are added, or its pass is unproven.**
  `corpus_literal_sweep` going 48,634 → 48,699 examined — exactly **+65**, the fold's own new
  record count, independently confirmed by a live mutation probe that caught a planted defect on
  one of the folded records — is what proved the new records were genuinely inside the swept
  population rather than the sweep passing vacuously over a corpus it never actually walked over
  them. A "0 findings, unchanged" result with an unchanged examined-count would have been
  indistinguishable from the sweep silently skipping every new file.

## Cross-references

- `../release/SD-33-computed-value-verification/decisions.md` — §1-§7, the mechanisms this
  retrospective's lessons cite.
- `../release/SD-33-computed-value-verification/workflow-instruction.md §12` — the standing-lessons
  table this retrospective closes rows 3 and 8 of.
- `../release/SD-33-computed-value-verification/forward-scope-register.md` — the inherited-debt
  entry, §E1's three ruled-out fold branches, and the 3 open deferrals.
- `../release/SD-33-computed-value-verification/artifacts/epic-6-closure/AT-33-E6-001-attempt10_cycle_receipt.md` —
  the scan that first closed the bundle, pre-fold.
- `../release/SD-33-computed-value-verification/artifacts/epic-6-closure/AT-33-E6-001-attempt11_cycle_receipt.md`,
  `AT-33-E6-001-attempt12_cycle_receipt.md`, `fold-skinwalker_cycle_receipt.md`,
  `fold-undine_cycle_receipt.md`, `fold-inventory_cycle_receipt.md`, `fold-fix-repin_cycle_receipt.md` —
  the fold reopen this §6 is written against.
- `sd32-compute-library-and-cause-closure-retrospective.md` — the direct predecessor, source of
  `decisions.md §1/§2/§4/§6/§7`.
