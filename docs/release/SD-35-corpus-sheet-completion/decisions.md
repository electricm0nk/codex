---
canonical: true
owner: god-emporer
bundle_id: SD-35
date: 2026-09-07
---

# SD-35 Decisions

Bundle-specific ADRs. Numbered in the order they were ruled. Every ruling names the command or
artifact that enforces it, or is marked `UNENFORCED` — and an `UNENFORCED` marking is a tracked
defect (`risks-and-open-questions.md §10`).

---

## §1 — The sheet rule: a unit is done when the sheet shows what the player would write

**Operator ruling, 2026-09-07, verbatim:**

> *"This isn't a video game, it's a character sheet generator. Many rules just need to get printed
> out, they don't need to be used in a computation in the middle of a computer based encounter
> simulation. Just write the rule out so we can print it on paper."*

**Operator correction the same day, when the first draft of this rule drew the line at
"text vs compute":**

> *"the example you have does need to be computed. It should calculate the value to print out.
> [...] We should print out a final number. Not a number, plus another number, plus another
> number. Do the math. But sometimes a number is that very addition. A sword that does d8+2
> damage - you just write that down. You don't need to write a dice rolling engine."*

**Decision.** A content unit is **DONE** when the character sheet shows, for that unit, exactly
what the player would write on paper:

1. **A final number** — every term that resolves for *this* character is added into one value.
   `10 + spell level + Charisma modifier` prints as `DC 15`, never as three terms.
2. **A dice expression** — dice stay dice. `1d8+2` is the final form. No dice engine.
3. **The rule's words** — a term the character does not settle yet (an unmade choice, a
   situational condition) stays as words, printed from the record's own description tokens.

The three forms are not a judgment call per unit. They are what the token closure yields when
run through one generic converter **at ingest time** (`technical-design.md §1`): map every PCGen
term to our own expression form, pass dice literals through, and emit words for what is left.
The live evaluator then does the arithmetic against the character (`technical-design.md §2`). **Per-unit judgment calls are themselves the delay this bundle exists to
end**, so the rule is mechanical or it is not the rule.

**What this replaces.** SD-34's DONE bar (`grounded`) required a magnitude *observed reaching a
consumer through a real pipeline run* — a fixture, hand-derived guards, and a "reachable
consumer" proof per mechanism. Under that bar SD-34 wave 51 built a whole engine module to
compute a save DC, then refused to close 48 Monk unarmed-damage units because no character in
the engine can be size Large. Computing the DC was right. The harness around it, and the
refusal, were the waste. SD-31 Decision 7's text-complete rule only fired when
`magnitude_token_count == 0`, so every unit with a bonus token became an engine job. That is
the mechanism behind the ~1 unit per hour the operator rejected.

**What this does not change.** Values that feed sheet totals (AC, attack, saves, skills, HP,
initiative, CMB/CMD, spell DCs, speed) are still computed, still through the existing
interpreter, still oracle-checkable. `derived_evaluator_fixture_check` (SD-31 Decision 20's one
binding condition) still runs on every interpreted value. The sheet rule adds a terminal state
for everything the totals do not consume; it does not weaken the totals.

**Enforced by:** `AT-35-E2-001` (the converter), `AT-35-E2-002` (the live evaluator and the
on-screen proof, once per kind — SD-31 Decision 7 condition 3 satisfied generically rather than
per unit), `AT-35-E2-003` (the `sheet-complete` status and its classifier rung).

---

## §2 — Batch floor: a cycle is one mechanism, corpus-wide, never fewer than 500 units

**Operator ruling, 2026-09-07:**

> *"We need to tackle large swaths of items before we stop to do a build and test. Doing a build
> takes a good half hour and it was doing multiple tests per item. That's like 90 minutes, mostly
> build time, to do a single item."*
> *"this is ok in theory until you start deciding to only put 2 items in a wave. That can't happen
> again. You have to think bigger."*

**Decision.**

- **A cycle's scope is a whole mechanism across all 37 books.** Never one book, never one
  class, never "the cheapest next item". SD-34 waves 43–48 scoped cycles to 2–45 units each and
  paid a full build per cycle; waves 49–51 scoped to 129, 343 and 217 units with one build each.
- **Hard floor: 500 units per cycle.** A mechanism with fewer than 500 units left is bundled
  with others until the cycle reaches 500, or the cycle takes everything that is left in the
  corpus.
- **The floor is a script, not a sentence.** `scripts/cycle_scope_gate.py` (AT-35-E1-001) reads
  the scoped population from `docs/work-inventory.json` and exits non-zero when it is under 500
  and is not the whole remaining non-DONE population. Every dispatch prompt runs it first; a
  cycle that cannot pass it does not start.

**Why a script.** SD-34's wave 50 and 51 dispatch prompts already carried "BATCH BIG" and
"VERIFY ONCE" as numbered rules. Prose rules decay; `workflow-instruction-template.md §12`'s
first standing lesson is that a recurring incident gets a nonzero exit code, not another
sentence. The memory that produced the same rule (`batch-big-not-small-per-dispatch`) existed
before wave 43 and did not stop waves 43–48.

**Sizing context.** The live remainder is 23,315 units of 49,438; the fable review's live
census found 22 mechanisms covering 90% of 22,369 units in the 35 non-vehicle books, so the
typical mechanism carries on the order of a thousand units. 500 is a floor, not a target.

**Exemption, stated here so it is a decision and not a habit.** Cycles that close zero units
*by design* skip the scope gate and write `SCOPE_GATE: EXEMPT (<reason>)` in the receipt: the
gate-building cycles (Epic 1 rows 1, 2, 5, 6), epic wrap-up fix cycles (`§10` step 0), and
Epic 6's exit cycles. SD-34 `workflow-instruction.md §12` row 6 ("measurement waves that bank
zero units are legitimate deliverables") is the precedent. **Nothing is exempt from the residue
check (`§11`).** Any other cycle under 500 that is not the whole remainder does not start.

**Enforced by:** `scripts/cycle_scope_gate.py --min 500` in `workflow-instruction.md §6` step 1;
`§7`'s receipt carries its literal output or the `EXEMPT` line.

---

## §3 — Verification cadence: one build per cycle, the full gate once per epic

**Decision.** Two checks, two cadences:

| Check | What it is | When |
|---|---|---|
| Cycle check | `cargo test --locked --no-run` + the scoped suites + `cargo test --locked --no-fail-fast` when the cycle touched compute or classification + the fast python gates (`completion_atlas.py --check`, `denominator_gate.py`, the three citation `--check`s, `pi-sweep`) | **once per cycle**, after the last figure-moving commit |
| Full gate | `scripts/verify.sh` (all stages — 40 at authoring, 42 after AT-35-E1-002) | **once per epic** (`§10`), and once before the PR (`§11`) |

**Why.** SD-34 ran the full gate as a separate cycle after every wave (~101 minutes recorded at
`SD-34/progress.md:3625`) on top of the cycle's own full-suite run, so a 5-unit wave paid two or
three long runs. The full gate did catch two real regressions (waves 42, 44) — which is why the
per-cycle `--no-fail-fast` run stays for compute-touching cycles and why the fast python gates
run every cycle. What moves to epic cadence is the desktop crate, frontend, clippy, and
dashboard stages.

**Risk accepted.** A desktop or dashboard break surfaces at epic end instead of wave end. That
is at most one fix cycle per epic.

**Amendment, operator 2026-09-07 — the worker split.** Asked whether the run should move to the
cloud for speed, the operator chose the hybrid: cycle work stays **local** (this box compiled the
whole workspace cold in 2 min 45 s at the cut; its cache is warm; its progress is visible), and
the two long **read-only** jobs — the epic-end 40-stage gate (~100 min) and the oracle-harness
runs — go to an **isolated worker that pushes nothing** (a worktree agent here, or a cloud
session when the box is loaded). The gate **overlaps the next epic's first cycle** and must be
green before that epic's second cycle; a red gate costs at most one cycle of rework. The cloud
was rejected as the default because every cloud session starts cold (clone, corpus, build) and
cannot be watched live (`cloud-loop-orchestration-lessons`); two writers on one branch is the
collision this split is designed to prevent. Enforced by `workflow-instruction.md §2`'s worker
split, `§2.4`'s `pendingWrap`, `§10` step 0.

**Enforced by:** `workflow-instruction.md §6` step 3 and `§10` step 0.

---

## §4 — No per-unit proof machinery: lines of new code per unit closed is a reported ratio

**Decision.** A cycle reports `rust_lines_changed / units_closed` in its receipt. The number is
computed by `scripts/cycle_scope_gate.py --receipt` from `git diff --stat` and the before/after
inventory diff. A ratio above **3.0** does not fail the cycle, but the epic wrap-up (`§10`) must
name every such cycle and state what the lines bought. The fable review's TOKEN-MODEL found the
slow SD-34 hours were transcription — reading tokens by eye, writing bespoke Rust, hand-writing
per-feature probes — not computation. This ratio is the instrument that sees it recur.

**What counts as per-unit machinery (do not build):** a fixture whose expected value is
hand-derived for one record; a `ground_<x>` function that serves one feature; a "reachable
consumer" probe for one mechanism; a test file named after one wave's one family. **What is
fine:** one converter mapping row per token type; one corpus-fixture gate per *kind* that reads
the live corpus directory and pins every record of that kind at once.

**Enforced by:** `§7`'s receipt row; `§10` step 1's ratio review. Marked **partially enforced**
— the ratio is mechanical, the "what it bought" review is a reading.

---

## §5 — Slug, branch, board, dispatch

- Slug `corpus-sheet-completion`; folder `docs/release/SD-35-corpus-sheet-completion/`.
- Branch `tranche/15`, cut from `develop` after SD-34's closure PR merges.
- Board: local-file `./kanban.md` (Hermes retired 2026-08-01, SD-30 Decision 14a).
- Dispatch: the `Workflow` tool from a live session, `workflow-instruction.md §2`.

---

## §6 — A blocker is cleared or escalated, never deferred

Inherited unchanged from SD-34 `decisions.md §6` and `../../governance/blocker-closure-doctrine.md`.
`## Open blockers` is a request for an operator ruling and pauses the bundle. There is no
"complete or filed under Open blockers" anywhere in this package. **The batch floor does not
license deferral:** a mechanism under 500 units is bundled, not parked.

**Enforced by:** `workflow-instruction.md §8`, `§11` step 1; AT-35-E6-001.

---

## §7 — Relationship to SD-34, which merged without closing

SD-34 was `in-progress` at this package's authoring (wave 51, 2026-09-07 morning). The same
evening the operator merged SD-34's PR #383 to `develop` (`fe5ae6cd4a`, 22:26 UTC), cut
`tranche/15`, and stamped `0.15.0` (`4c6c57eb9f`). **SD-34's closure epilogue never ran**: no
retrospective, 17 of 37 kanban rows not `complete`, no final-acceptance scan. `§12` records the
ruling that folds that debt into SD-35. SD-35's population is *every* non-DONE unit in the corpus
at the cut, all 37 books — including whatever SD-34 left in `core_rulebook` and
`ultimate_campaign` — re-measured at `4c6c57eb9f` (`content-unit-inventory.md §0`).

SD-34's `decisions.md §22` (2026-09-04) told future waves to scope bucket-D work "as real
feature-building work (new `ground_<class>_class_features`-style dispatch functions)". **Under
§1 that instruction does not carry into SD-35.** Bucket D's unmodelled classes get a generic
class-chassis loader from corpus `CLASS` records (AT-35-E5-002), not sixty hand-written
functions.

---

## §8 — Every figure states its denominator in the same construct

Inherited from SD-34 `decisions.md §3`. `scripts/denominator_gate.py`'s default scope is
widened to this package by AT-35-E1-004; until then every command passes the explicit glob
`'docs/release/SD-35-corpus-sheet-completion/*.md'`.

Three denominators are live in this package and every figure names which:

| Population | Count | Where it comes from |
|---|---:|---|
| Corpus, all 37 books | 49,438 | `docs/work-inventory.json` `units` |
| Non-DONE, all 37 books | 23,315 | `completion_atlas.py --check` at HEAD `5f6b18f4e3` |
| Non-DONE, 35 non-vehicle books | 22,369 | fable review B-SYNTH at `b4485ae534` — **older snapshot**, quoted only when citing that review |

---

## §9 — Lessons carried in from SD-34's run, each with its enforcing command

| # | Lesson | Source | Enforced by |
|---|---|---|---|
| L1 | The finish line shapes the work; a simulation-shaped bar builds simulation | wave 51 receipt §3, §5 | §1 + AT-35-E2-003 |
| L2 | Batch size decays to small without a floor | waves 43–48 vs 49–51 | §2 + `cycle_scope_gate.py` |
| L3 | Line-number citation pins are a self-inflicted tax on every edit | wave 51 receipt §6; nearly every SD-34 receipt | AT-35-E1-002 (content-anchored pins) |
| L4 | Two `--check` instruments drifted because nobody ran them | wave 51 receipt §6 | AT-35-E1-002 wires them into `verify.sh` |
| L5 | A new status value breaks every consumer that raises on unknown status | fable review §7 (14 of 40 stages red after `oracle-agree`) | AT-35-E2-003's consumer sweep with grep counts before/after |
| L6 | 543 test binaries, two templated families of ~80k lines, is why a build is 30 minutes | fable review R10 | AT-35-E1-003 with build time measured before/after |
| L7 | A dispatch script's return value is not a closure claim | SD-34 `decisions.md §12` L3 | AT-35-E6-001 re-derives every `complete` from the repo |
| L8 | Never carry your own number forward — re-derive it | SD-34 `decisions.md §12` L2 | `verify.sh --only figure-provenance` |
| L9 | Sum the piles, always | SD-34 `decisions.md §12` L4 | `completion_atlas.py --check` `unclassified=0 overlap=0`; `token-coverage.json`'s own sum check |
| L10 | Only a move **into** DONE is closure; a bucket-to-bucket move is a relabel | memory `bucket-diff-to-separate-closure-from-relabel` | `§7`'s four-buckets row, derived by id-set diff |
| L11 | Parallel cargo fan-out crashes the VM | memory `proxmox-host-stops-vm-on-guest-oom` | `workflow-instruction.md §3`: at most 3 concurrent lanes, `-j 6` per lane |
| L12 | Shared instrument files are not fenced by a bucket or kind boundary | memory `territory-must-fence-shared-instrument-files` | `§3`: any two lanes both touching `v06_work_inventory.rs` or `completion_atlas.py` run sequentially |
| L13 | A run-time interpreter permitted "for now" stays forever unless a gate counts it | SD-31 Decision 20 → 78 live files by 2026-09-07 | §11 + `pcgen_residue_gate.py`, monotonic, zero at closure |

---

## §11 — No PCGen in live code: the converter and the test oracle are the only readers

**Operator ruling, 2026-09-07, verbatim:**

> *"you are using the pcgen tokens to build the logic, right? I want to make sure you aren't
> building a pcgen engine into the middle of our live code. Using it to convert and test is
> fine. But when we are done, there should be nothing left of pcgen."*
> *"it's fine to use pc-gen for testing the rewrite to ensure the rewrite is solid. it's not fine
> to include 1 line of java in our live code."*

**Decision.** PCGen `.lst` tokens, PCGen formula strings, PCGen variable names, and the code
that parses or evaluates them exist in exactly two places: **the converter** (ingest-time
tooling that writes our own `SheetRule` records — `technical-design.md §1`) and **the test
oracle** (`scripts/oracle_harness/`, comparing our numbers against PCGen's). **The live side —
`src/rules_core/`, `src/saved_character/`, `src/campaign/`, `src/homebrew_authoring/`,
`apps/desktop/` — contains none of it at closure.** The boundary by path is
`technical-design.md §0`.

**What this corrects in this package's first draft.** The first draft put the renderer in
`src/rules_core/` reading the token closure at run time — a PCGen interpreter in live code. It
also inherited, without saying so, that the live code already does this today: a coarse grep at
authoring finds **78 files** on the live side reading `raw_tokens`, calling
`PcgenFormulaEvaluator`, or calling `render_pcgen_desc` (`content-unit-inventory.md §6`). SD-31
Decision 20 permitted the run-time interpreter to get a build in front of users; nothing since
ruled it back out. This ruling does. The operator chose option 1 of three — retire the existing
live readers inside SD-35, not register them for a successor — because leaving them means two
engines side by side and the rule is not met at closure.

**How it is met.** Epic 2 builds the converter and our evaluator. Epics 3–5 move every unit
onto converted records. **Epic 6 removes the old path** — the formula evaluator, the token
closure reader, the generators, the prose renderer, and the desktop catalogs' `raw_tokens`
reads — with the oracle harness proving parity before and after. `scripts/pcgen_residue_gate.py`
(AT-35-E1-005) counts the live-side surface from cycle 1, **fails any cycle that increases it**,
and fails on anything above zero from AT-35-E6-004 onward.

**What is KEPT — operator clarification, 2026-09-07, verbatim:**

> *"when we finish pathfinder1e - we do have other game systems to convert. Do[n't] start
> deleting our conversion engines and test oracles as soon as we finish pf1e, because right
> after that we are going to start on starfinder."*

The exit removes PCGen from the **live** side only. The tool side is a reusable asset and is
**not** deleted, trimmed, or "cleaned up" by any SD-35 cycle: the converter
(`src/bin/sheet_rule_convert.rs`, `src/pcgen_import/**` including the relocated parser,
`bonus_stack_reader`, `pre_tokens`, `wiring_class`, `cache_gen`), the `src/bin` generators, the
oracle harness (`scripts/oracle_harness/**`, `src/oracle_validation/**`), the oracle pin and
fetch script, and the pinned PCGen checkout slot. Starfinder is the same `.lst` format with a
different `.pcc` include structure (`forward-scope-register.md` C2.1) and is the next system.
**A cycle that deletes converter or oracle code is a defect**, not a tidiness win — AT-35-E6-004
and AT-35-E7-001 check that the converter still builds and the harness still runs at closure.

**Enforced by:** `scripts/pcgen_residue_gate.py --check` every cycle; `--closure` at AT-35-E6-004
and AT-35-E7-001; `pcgen_live_files` in every receipt; the parity artifacts in
`artifacts/epic-6-pcgen-exit/`; the independent grep in `acceptance-and-verification.md §3a`;
the tool-side liveness check in `acceptance-and-verification.md §3a`.

---

## §12 — Operator ruling, 2026-09-07: SD-34's unrun closure is folded into SD-35 Epic 1

**What happened.** SD-34's PR #383 was merged and `tranche/15` cut before SD-34 ran its own
closure epilogue (`SD-34/workflow-instruction.md §11`). At the cut: no
`docs/retro/sd34-book-completion-retrospective.md`; **17 of 37** `kanban.md` rows not
`complete` — rows 13, 14, 15, 17, 20 (Epic 3/4 book-to-zero criteria), 26, 27 (the closure
epilogue itself), and 28–37 (the salvage, trait-slice, and bucket-D-mining cycle rows); no
final-acceptance scan. `tranche/14` is deleted on origin. The launch-readiness audit of this
package surfaced it.

**Operator ruling (three options presented; the first chosen):** fold it into SD-35 Epic 1 as
one housekeeping cycle — **AT-35-E1-006**. That cycle:

1. writes `docs/retro/sd34-book-completion-retrospective.md` from
   `python3 scripts/retro.py summary --since 2026-08-27 --json`, in `sd31-retrospective.md`'s
   shape, and cites it from **both** `../SD-34-book-completion/references/README.md` and this
   package's `references/README.md`;
2. maps each of SD-34's 17 open rows to the SD-35 criterion whose population now owns its
   units (rows 13/14/15/17/20 → AT-35-E3-001..E4-001 by bucket; rows 28–37 → the same by
   bucket; rows 26/27 → AT-35-E7-001..003), with the unit counts summing against
   `completion_atlas.py --book core_rulebook` and `--book ultimate_campaign` at the cut;
3. records in SD-34's `progress.md` that the bundle closed **by operator merge on 2026-09-07
   with its epilogue folded into SD-35**, so no reader mistakes the 17 rows for live work.

**What this is not.** Not a waiver: the retrospective gets written and SD-34's lessons reach
`§9` before Epic 2 dispatches. Not a laundering: the 17 rows' units are in SD-35's Definition of
Done by construction (they are non-DONE units in the corpus), not in a register.

**Enforced by:** AT-35-E1-006's evidence (file exists; both citations grep ≥ 1; the row map
sums); `workflow-instruction.md §1` item 4; `§12` row 37.

---

## §13 — `box_ledger.py` is retired as a standing gate; the Completion Atlas is the partition

**Finding, launch-readiness audit 2026-09-07.** `python3 scripts/box_ledger.py --check` exits 1
at the `tranche/15` cut: `uncovered=27502 of 49438`, with eight warnings that
`docs/release/SD-33-computed-value-verification/THE-BOX.md` "needs re-deriving". THE-BOX.md is
SD-33's frozen closure artifact and pins SD-33's status vocabulary — `not-ingested` at 26,002,
`grounded` at 3,415, `literal-verified` at 6,589 — against a live inventory where
`not-ingested` is **0** (SD-34's AT-34-E1-005 renamed it to `engine-does-not-hold` on
2026-08-26) and `grounded` is 5,222. The count did not drop because coverage was lost; it
dropped because the instrument reads names the inventory no longer emits
(`instrument-correction-is-not-closure`). The ledger is not a `scripts/verify.sh` stage and has
no re-derive mode, so nothing ran it after the rename. SD-34 listed it as a standing gate
"green at every cycle"; it was red for the whole of SD-34's Epics 2–5.

**Decision.** `box_ledger.py --check` is **not** an SD-35 standing gate. The partition of record
is `scripts/completion_atlas.py --check` — fail-closed on six conditions, content-cited, green at
the cut (`unclassified=0 overlap=0 citation_failures=0`). THE-BOX.md stays as SD-33 history and
is not re-derived by this bundle. The one thing the ledger checked that the atlas does not —
`oracle_disagreement` against SD-33's `oracle-results.json` — is covered by AT-35-E4-002's
harness run and the parity artifacts (`§11`).

**Why not fix it instead.** A second partition that must be hand-kept in sync with the first is
the drift SD-34 `decisions.md §12` L1 ("a field's name is not its meaning") warns about; the
atlas already re-derives the same population every cycle with citations that fail closed.

**Enforced by:** its removal from `acceptance-and-verification.md §2`, `technical-requirements.md §2`,
and `workflow-instruction.md §6` step 3; the finding recorded in `§1` item 9.

---

## §14 — Operator ruling, 2026-09-07: Fable on every lane until it runs dry, then Opus; orchestrator on Opus

**Operator, verbatim:** *"i would really prefer just to throw fable at everything until it runs
dry, and then switch to opus. We can run the orchestrator as opus to manage this."* Context: about
36 hours to the quota reset with a large surplus.

**Decision.** For this bundle the global model tiering (Sonnet as the execution default) is
overridden: every dispatched cycle, wrap-up, oracle, and scan lane runs on **`fable`** until the
Fable quota is exhausted, then on **`opus`**; housekeeping (release notes, version confirmation)
stays on Haiku. The **orchestrating session runs on Opus**. The mechanism is
`workflow-instruction.md §2.4`'s `LANE_MODEL` (from `args.laneModel`): a lane that returns `null`
on a terminal API error halts the script naming its criterion; the orchestrator rebuilds the
criteria lists from `kanban.md` and relaunches with `args.laneModel = 'opus'`. Nothing is resumed
blind — the relaunch re-reads the board.

**Also spent on Fable before launch, same ruling:** the converter's token-type → `Expr` /
`Applies` / prose mapping table (`artifacts/epic-2-sheet-rule/token-mapping/`), derived from the
corpus and judged adversarially, so AT-35-E2-001 transcribes a reviewed table instead of
inventing one mid-cycle (`risks-and-open-questions.md §2` R1).

**Enforced by:** `LANE_MODEL` in every `agent()` call; `§12` row 39.

---

## §15 — Schema v2 landed from the token-mapping synthesis; three rulings PENDING

**What happened (2026-09-08, pre-launch Fable spend under `§14`).** Four Fable lanes mapped the
corpus's PCGen token vocabulary onto our schema (bonus, formula, prereq, prose); four Fable
judges tried to refute every row against the corpus and the pinned PCGen Java; one synthesis
lane reconciled them. Results in `artifacts/epic-2-sheet-rule/token-mapping/`:
`mapping-table.v1.json` (249 rows: 128 kept, 49 kept-annotated, 49 rewritten, 8 dropped, 23
added; 9 REFUSE rows by shape; **23,312 of 23,315** non-DONE units reachable, 3 unjoined spells
uncovered), `SYNTHESIS.md` (12 cross-family conflicts, each resolved to one rule with its
citation), `blockers.md` (10 work items with owners, 3 rulings). **`technical-design.md §1–§2`
are now schema v2** — `Expr` gained `Var` (with a typed, converter-emitted contribution table
folded over HELD rules exactly as PCGen's `BonusManager` folds), `Floor`, `Ceil`, `BaseSize`,
`SizeMod`, `BaseSave`, `SkillRanks`, `SkillTotal`, `HeldCount`, `ChallengeRating`, `Speed`,
`HighestSpellLevel`, `MasterLevel`, `MasterVar`; division is exact with ONE truncation at the
sheet boundary (per-step floor diverged on 255 formulas / 452 units); `prose` is a slot
template filled at evaluate time; `Applies` is two-valued plus `Situational`. AT-35-E2-001
**transcribes the table**; a row invented mid-cycle is a defect.

**Three findings that changed the plan, not just the schema:** (1) the `_pfs/` overlay leak is
826 records, not 215, and 6,686 cross-book `.MOD` rows never attach under today's per-book
index — the converter's closure is corpus-wide and matches on KEY (`blockers.md` B9, ruling 3);
(2) the "no class feature has a holder" blocker was **refuted** — Paizo base-class grants are
already in the closure; only prestige/hybrid/3rd-party level lines are unpersisted, a tool-side
read (B5); (3) PCGen's oracle export has no skill, speed, DR, DC, or spells-per-day totals, so
AT-35-E2-005's parity needs new export tokens first (B1, owner AT-35-E2-005).

**Rulings requested — PENDING the operator (`blockers.md` has the full options):**

| # | Question | Recommendation |
|---|---|---|
| R1 | `VISIBLE:DISPLAY` rows (2,319 instances of 7,505): follow PCGen and hide them from the printed sheet, or print everything but `NO`? | **(a) hide** — a DISPLAY row is bookkeeping the player never writes; the feature still prints once where the book puts it |
| R2 | The PI term-hit bucket (~900 records): omit the redacted field, stamp `provenance.pi`, print the licensed remainder — or refuse forever? | **(a) omit-and-stamp** — one outcome for one fact; refusing forever is a carve-out no mechanism can close |
| R3 | Converter closure scope: corpus-wide (a later book's `.MOD` changes an earlier book's rule, provenance cites it) or per book? | **(a) corpus-wide** — it is what the loader already does and what PCGen does with all campaigns on |

Until ruled, AT-35-E2-001 builds to the recommendations and flags each in its receipt; a
different ruling is a re-conversion, not a redesign.

**Enforced by:** AT-35-E2-001's evidence (transcription of `mapping-table.v1.json`; the B9
closure fix landed before the first conversion); AT-35-E2-005 (B1 export tokens); this entry
re-read at Epic 2's dispatch.

---

## §10 — Build version

SD-35's first concrete build value is `0.15.0`, stamped in `apps/desktop/package.json` and
`apps/desktop/src-tauri/tauri.conf.json` at the `tranche/15` cut. The tranche digit moves on a
new `tranche/N` cut, never on a bundle's own closure (SD-34 `decisions.md §11`). Root
`Cargo.toml` stays at `0.1.0` and is not the version source of truth.

**Resolved 2026-09-07:** the cut landed as `4c6c57eb9f` ("feat(sd35): version bump 0.15.0 for
tranche/15") on `fe5ae6cd4a` (SD-34's PR #383 merge); both version files read `0.15.0`;
`git ls-remote --heads origin tranche/15` resolves. No deferral remains.
