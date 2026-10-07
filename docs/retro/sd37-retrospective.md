---
canonical: true
owner: sd37-orchestrator
purpose: SD-37 (Starfinder 1e) retrospective, grounded in the retro log and the package's own receipts, written at E7.4.
date: 2026-10-07
bundle: SD-37
branch: tranche/17
---

# SD-37 retrospective

Starfinder 1e went from "not loadable" to a system the real app builds, levels, prints and totals from
converted PCGen data, with the Pathfinder render unchanged. Every number below carries the command that
produced it. Where a number comes from `retro.py summary`, the command is

```
python3 scripts/retro.py summary --since 2026-10-02 --json
```

and its output at the time of writing is kept in
`docs/release/SD-37-starfinder-1e/artifacts/epic_7/E7.4_logs/retro-summary.json` (generated
`2026-10-07T18:01:14Z`; the log keeps growing, so a re-run later will read higher).

**State at writing.** 51 of 57 `kanban.md` rows `complete`; the six that are not are E7.4 (this card) to
E7.9, the closure chain
(`awk -F'|' '$2 ~ /^ (C|E)[0-9]/ && $5 ~ /^ complete *$/' kanban.md | awk 'END{print NR}'` → 51, run in
the package directory before this card's own row moved). The bundle ran 2026-10-02 to 2026-10-07 on
`tranche/17`: 115 commits past the cut
(`git log --oneline 20bf84a3b2..HEAD | awk 'END{print NR}'` → 115; `git_join.commits` in the summary agrees).

```
EVENTS  160   (the summary's own total; denominator = events since 2026-10-02)
   69  verification      37  correction     23  rework
   15  incident          10  deferral        3  note
    2  resolution         1  near_miss

incidents                      15, of which 4 silent, 47 recorded minutes lost
verification runs              69, 19 failed (fail_rate 0.2754)
corrections caught before use   3 of 37
deferrals                      10 filed, 2 resolution events
```

The figures come from the summary's `events.by_type`, `incidents`, `verification` and
`corrections.caught_before` keys. "37 corrections" is every event of type `correction` in the window; it
is not a count of distinct mistakes (see lesson 3).

---

## What the data says, before any interpretation

### 1. Our own documents were the most corrected artifacts again

37 corrections. The subjects are mostly written artifacts, not code: package authoring (CUI rows,
`decisions.md` sections, the E7.3 scan command, DEF-1's revisit command), `progress.md` Summary rows
(4 subjects), receipts (E3.4, E4.3, E4.6, E4a.1 x2, E5.4 x2), and `kanban.md` row notes.
By corrector, from `corrections.by_corrector`: the two C0 cards 8, the six merge checks 6, the E4a cards
7. The merge checks were the corrector of 6 of 37; **only 3 of 37 were caught before use**
(`corrections.caught_before` = `{implementation: 2, release: 1}`). That ratio is the point: most wrong statements were found by a later card re-deriving, not by the
author.

The worst instance for cost was in the package itself. The E7.3 scan command as authored printed nothing
over 54 open cards (C0.2 measured it). It could never fail. C0.2 rewrote it and added planted faults, and
E7.3 re-ran with two planted faults that print. That is the correct shape for a gate and it was found only
because an Opus review looked at the gate's ability to fail, not its output.

The last correction of the bundle: E7.3 found the forward-scope register carrying "8 open SD-34 P1s" when
the count was 0 (6 fixed by SD-36 Epic E, 2 mooted by file retirement). A number quoted across two
bundles had been wrong since it was copied.

### 2. Generated artifacts had drifted from their generators, silently (E4a.4a)

Three corrections from one card. When E4a.4 removed `src/rules_core/rules_tables`, six codegen tools
still wrote into it. Porting them and re-running against the pinned oracle showed the shipped tables were
**not** the generators' output:

- `transcribe_monster_tables.py`: SD-35 cycle 11 hand-edited 3 generated tables (`%CHOICE` x5, `%LIST`
  x15, `TYPE=Base` x2, ` DESC:&nl; ` x4); a re-run reverted them.
- `transcribe_companion_tables.py`: SD-35 cycles 9 and 13 had converted guard strings to typed
  conditions by hand; the transcriber still emitted strings.
- `ingest_spells`: a re-run added 411 `.COPY=` rows to 7 books that SD-32 never shipped and reverted one
  SD-34 hand edit (soft hyphens in `monster_codex` Spellsteal).

The claim "the generator reproduces the shipped table" had been true by assumption for years. It was
caught because a card was forced to run all six (42 invocations) and diff. Fixed: the four hand-edit
classes are folded into the generators; `git status --porcelain data/rules_tables` is empty after a
re-run and `rules_tables_package --check` passes 281 tables
(`E4a.4a_cycle_receipt.md`). The 411 `.COPY=` rows are **not** shipped and that decision is open
(deferral, below).

### 3. A proof is only as wide as its seeds, and the seeds were two

The Pathfinder no-regression proof for the whole bundle is the render hash of two characters (Aldric
Fighter 3 `1d830682…a569`, Elowen Wizard 5 `8d1a711c…00f2`) plus the structural diff (49,450 PF records
unmoved). It held at E1.4, E1.MC, E3.5, E4.MC, E6.MC, E4a.4a and E7.2. It also under-reaches, and the
bundle found that three times:

- E1.MC planted a drop of `core_rulebook` from the registry's PF entry (flips both hashes) and wrapper-only
  drops (race corpus, class family) which do **not** flip. A near-miss event records it: "harness
  sensitivity is 2 seeds' reach, not the registries".
- E4a.MC planted a Fighter L3 `will_save` 1 → 7 in the package: the golden test fails, **Aldric does not
  move**, because PF Fighter base saves come from a second source. Wizard rows do feed Elowen.
- E4a.MC found that across the whole E4a epic 14 of 15 PF catalog dumps are byte-identical;
  `list_monster_catalog` changes 2 lines (the Crocodile `groundingNote` citations losing `.lst`, the
  §19-mandated strip), and E4a.4's own "15 dumps identical" had covered only its own commits. The hash
  pair proves the render of 2 builds, not the catalog.

Standing rule 7 of `AGENTS.md` applies in the form: the pair is a floor for "PF did not move", and each
card that claimed it stated what it does not cover. The next bundle should not read "pair equal" as
"PF unchanged" for any character other than those two.

### 4. Scope fences stopped the run twice, then a standing rule fixed it

`workflow-instruction.md §3` fenced each epic's files. Run 1 stopped at E5.3 after about 61 hours
(`blocked-escalated`; the figure is from `sd37-launch-state.md`, which the orchestrator wrote; I did not
re-derive it) because a numeric feed needed `pilot_compute/sf_loadout.rs`, an E4 file. Run 2 stopped at E5.4
for `sheet_rule.rs`, an E2 file (master-variable resolution). Both requests were the exact kind
`AGENTS.md` Blocker Discipline asks for and both cost a run. `decisions.md §20` and then **§21** (standing
rule: a completed epic's files are open to a later serial card under its own criterion, with that epic's
gate re-run) cleared them and no later card stopped on a fence. The fence was right at authoring (parallel
lanes) and wrong after E2–E5 had completed and the chain was serial. The package had no rule for the
change of regime.

### 5. The workflow script did not read the board

Two incident keys fired twice or more:

```
redispatch-of-complete-card       2   (E0.4, E1.4; a third event is keyed redispatch-of-completed-card: E0.2)
dispatch-before-discovered-dependency  2   (E4a.MC attempts 1 and 2, dispatched while E4a.4a was "ready")
```

Mechanism, both: the script held a static step list. After a resume, call order differed (E0 and E1
lanes interleave), the cache missed, and three complete cards were re-dispatched (each declined without
rewriting, one agent turn each; the launch-state note puts it at about 465 k tokens, not re-derived). And
E4a.4 *discovered* a card (E4a.4a) that the step list did not know, so E4a.MC ran twice against an unmet
dependency. The receipt of the second firing says it plainly: "a warning in a receipt does not stop it".
`R-W` (a key firing more than twice gets a non-zero-exit check) was not triggered by the count, but the
mechanism was the missing control. Run 5 changed the script to run any `discovered` card next, before the
epic's MC; E6.5a and E4a.4a were both closed that way. Not yet built: a step that reads `Depends on` from
`kanban.md` and refuses to dispatch a card whose dependency is not `complete`.

### 6. Verification stages that cards did not run

19 of 69 verification runs failed (`verification.failed_runs` / `runs`). By failing stage
(`verification.by_failing_stage`): clippy 9, rules-schema-check 5, preflight-oracle 2, sf-sheet-rules-check
2, tauri-resources-tracked 1; 9 + 5 + 2 + 2 + 1 = 19, so each failed run is attributed once. Clippy is 9 of
19. The incident `clippy-stage-not-in-card-verify` and the E4.MC finding (root suite red on
`45d30bf85b`: `sd24_wired_integration_audit` check 4 flagged 3 quoted `"Would …"` literals in E4.6 test
code; E4.6 never ran the root suite) are the same shape: a card ran its targeted tests and a merge check
found the stage it skipped. `R-G` (widest build scope) says the right thing and the cards that ran at
widest scope had the failure, but the stages differ by card type and no mechanism ran the cheap ones
everywhere. The same applies to `sd24_wired_integration_audit` red since E6.3 on a fixture's "Magic Hack"
corpus words (incident, found at E6.5, a gap of two cards).

---

## What worked

**The merge checks.** Every epic's `*.MC` card found a defect its epic's cards had missed, and fixed or
routed it. A selection, each in its receipt: E4.MC (root suite red from E4.6's test literals); E4a.MC (a
third attempt re-ran every E4a acceptance command with a Python twin, 42 generator invocations, mutation
per generator, 6 of 6 undone by their own re-run); E6.MC (a hand-kept `SCORES_PER_INCREASE = 4` in the
level-up dialog, R2, fixed test-first; real-app ui-smoke 99 of 103 green, 0 red, 6 of 6 seeds open,
real store 15,379 entries sha256 equal before and after, 45 of 45 plants red); E5.MC. `R3` (merge checks
stay on Opus) held: `awk -F'|' '$2 ~ /MC/ && $4 !~ /opus/' kanban.md` is the rule's check.

**Oracle parity as an explained list, not a green light.** E7.1 ran real PCGen on 14 characters (the 4
seeds, the 10 player classes at level 1 with Mechanic 1 = the drone build) plus the drone in party mode.
Gate `sf_oracle_parity`: **782 oracle fields compared, 36 differ, 36 explained, 0 unexplained, 0 stale**
(`sf_parity_check.py` agrees). The 36 are 25 oracle departures from the SRD (bulk 13, armour-check skills
6, 0-level per day 5, drone fly 1) plus 11 drone totals the engine prints as terms. Where the oracle and
the SRD disagree, the SRD hand values won and the departure is named. The gate also kept a not-in-oracle
list of 1,413 rows so that "782 compared" cannot be read as "782 of everything".

**A hypothesis tested rather than assumed.** `decisions.md §8` guessed HP and Stamina formulas. E3.3 ran
6 real PCGen builds and compared with the SRD: seeds HP 25/34/29/20, Stamina 24/25/35/21, SRD = PCGen =
mapping. "Resolve has no direct row" was wrong and was corrected. The package carried it as a hypothesis
until a card was assigned to settle it.

**The sum-the-piles control for the denominator.** E0.3: 8,582 units of 12,718 in-scope F-6 rows, 229
excluded named, Python and awk agree, 6 planted faults exit 1; E3.5 later `_report.json` records the same
8,582 with 0 refused and 34 degraded (each named). The denominator held across two epics.

**Mutations that must fail.** M1–M4 FAIL after every converter-touching card; 45 of 45 plants red at E6.MC,
12 of 12 at E5.4. Four converter defects were found by the wide books
(E3.5) and fixed RED-first. A `FAIL`-count line in a receipt is cheap, and it meant the plants were
re-run each time instead of cited.

**Discovery as a first-class path.** E6.5 found that feats, spells and gear were not pickable in the real
SF app (E6.5a: 190 of 190 sheet totals, 24 of 24 loadout hand values, ui-smoke SF rows 7/7); E4a.4 found
six generators writing into a removed directory (E4a.4a). Both were added as kanban rows with
commands that could fail, the row-count pin was raised each time (57 rows now:
`awk -F'|' '$2 ~ /^ (C|E)[0-9]/ { n++ } END { print n }' kanban.md` → 57), and the script runs them
before the epic's merge check.

**Honest partials.** E4a.MC was `partial` at attempt 1, `declined` at attempt 2 with `owned_by E4a.4a`
(correctly, since the dependency was unmet), and `complete` at attempt 3. E5.4 attempt 1 recorded two
"Does not cover" claims later corrected (Flight System, drone HP); attempt 2 sourced both (SRD drone table
plus a PCGen party run of `sf_mechanic_drone.pcg`). Nothing was closed over an open item.

---

## What did not work

**Resume.** `resumeFromRunId` replays only an unchanged call-order prefix. The parallel E0/E1 lanes
interleaved differently, the cache missed, and three complete cards were re-dispatched. The memory note
`sd37-launch-state` already says: do not resume a script with parallel lanes, set `START` and launch a new
run. Five workflow runs (`wf_2bee0ad4-5fc`, `wf_061eb317-457`, `wf_4ce3fd1a-f99`, `wf_12033d99-996`,
`wf_72574aac-2c8`) is the cost of that lesson and of the two fence stops (lesson 4).

**Provenance strings and textual rewrites (E4a.2).** The first re-point pass rewrote `rules_tables::` to
`rules_catalog::` in 253 importers, string literals included. 41 literals in 16 files are provenance text
whose bytes the PF render hash pins; the root run showed one failed suite (`sd20_feat_prereqs_parity`). It
was redone as `E4a.2_split_provenance_strings.py` (the literals keep their bytes). The acceptance command
counted the literal, the render counted the bytes, and nothing tied them. Later E4a.4 had the same
mechanism on doc comments that `schemars` publishes as schema descriptions (18 description lines).

**Tooling that does not know what it did not see.** E4a.1's collection scanner missed 2 of 211
collection functions because `rustfmt` splits a signature as `fn class_skills(\n) ->`; E4a.1's bundle-path
proof showed the packaged app would panic on its first table read on a tester machine (no checkout, no
env); E4a.3's ".lst elsewhere" count was 1,098 at authoring and 1,117 at E4a.3 start. Each is a count or a
proof whose reach nobody asked about until the next card.

**Out-of-scope findings that stayed findings.** Examples recorded in `progress.md ## DISCOVERED` and not
fixed: a wall-clock bound in `sd17_b5_equipment.rs` (2 s, read 4.46 s at load average 18); one failing test
of 40 in `tests.test_transcribe_monster_tables` that is not a `verify.sh` stage; `pcgen_residue_gate.py`
line 227's stale "burn down" comment. These are small, but the second is exactly "a gate nobody runs".

---

## Open items carried out of the bundle (named, with the command that shows each)

None of these is an open card; every card but E7.4–E7.9 is `complete`. They are decisions and deferrals,
listed so the operator sees them without reading 57 receipts.

1. **FSR-C4 is met: an operator decision, not a gate.** 31 Starfinder `(kind, slug)` pairs appear in 2
   books, 1 with a differing value (`equipment:needler_rifle`), of 8,551 distinct pairs (8,582 units).
   Recorded in `forward-scope-register.md §3.1`. Command and both implementations:
   `artifacts/epic_7/E7.3_cycle_receipt.md` Figures. Not traced: callers of `find` /
   `find_in_every_kind` on an SF package other than the two named.
2. **Deferrals the log still shows open: 8 of 10 filed** (`deferrals.open`). The log under-records closure:
   five of the eight name a revisit card that `kanban.md` shows `complete` (race speed → E5.1, SF
   level-up → E6.5, SF attack bonuses and per-weapon rows → E7.1, with `sf_attack.rs` landing 78 values
   RED → = oracle; scalar consts and label functions → E4a.2/E4a.4). **I did not verify each closure
   or write `resolution` events for them** (outside this card's criterion); only 2 resolution events
   exist, and the two the log counts resolved are `sd37-e4a-2`'s and `sd37-e4a-4`'s. The three that are
   genuinely open as decisions:
   - Drone totals (EAC/KAC, total saves, initiative, attack, skill totals) are printed as terms, not
     computed (E5.4 scope, accepted by E5.MC; 10 drone fields + Stamina are explained in E7.1). Needs an
     operator ruling on whether drone totals are sheet totals under `decisions.md §5`.
   - `sf_defense::held` leaves `CharacterFacts.base_attack` at 0, so a printed line over BAB reads 0
     (Deadly Aim on SF-Soldier-3 prints "additional 0 (minimum 1)" where min(1, BAB/2) = 1). Recorded by
     E6.5a and E7.1.
   - Whether the 411 `.COPY=` spell rows `ingest_spells` resolves for 7 books should ship (388 are
     `mod_only` in `docs/work-inventory.json`, OA 369 and UM 19; 23 are absent from it: AG 7, HA 4, ISM
     1, ISWG 2, UW 9). A PF content and census change, outside Starfinder.
3. **Starship (E8) is a planned deferral, DEF-1.** Its revisit command ran at E7.3: no stdout, exit 0,
   pinned oracle `7f818006e3…`. It only checks the pinned oracle; a newer PCGen with starship data would
   not show.
4. **SF licence rows have `operator_sign_off: false`** (`progress.md` Decisions, E0.2 SD-a row). The
   closing PR does not change them. The operator signs.
5. **CI evidence is one run.** No CI runs on `tranche/17` pushes (SD-n); the only CI is the closing PR's
   `pr-tests`, awaited by E7.9. Windows and macOS packaged builds, and an app launched on a tester machine
   (versus the bundle-path proof in E4a.1), are not proven by any card here.
6. **What the Starfinder proof does not contain.** The oracle parity set is 14 characters at level 1 (and
   the seeds at 3 and 5): classes above level 1 other than the seeds, multiclass Starfinder builds, and
   the 10 classes' level-up beyond the seeds were not run against PCGen. The 782 compared fields are the
   oracle's printed fields; the 1,413-row not-in-oracle list is test-kept in
   `scripts/oracle_harness/sf_parity/not_in_oracle.tsv`.

---

## Changes for the next bundle

| Change | Mechanism or home |
|---|---|
| A workflow step reads `Depends on` from `kanban.md` and refuses a card whose dependency is not `complete`; discovered cards are read from the board, not a static list (lesson 5; two incident firings) | workflow script; `workflow-instruction.md §2.4` |
| Never resume a script with parallel lanes; set `START` and launch a new run (lesson in `sd37-launch-state`) | script prefix; `decisions.md §12.4` |
| A fence regime has an exit: say at authoring when a completed epic's files open to later serial cards, with its gate re-run (the §21 rule, written in advance) | `workflow-instruction.md §3` |
| Every generator that writes a shipped, generated artifact has a re-run-and-diff check in `verify.sh`; a hand edit to a generated file fails it (lesson 2) | `scripts/verify.sh` stage; extends `rules_tables_package --check` |
| Clippy and the wired-integration audit (`sd24_wired_integration_audit`) run in every code card's verify, not only in merge checks (lesson 6; 9 of 19 failed verifications were clippy) | card verify block in the dispatch prefix |
| A textual rewrite of source that touches string literals must also run the render-hash and fixture suites before the sweep is "done" (E4a.2, E4a.4) | E4a.2's `split_provenance_strings` precedent |
| Write `resolution` events when a deferral's revisit card completes, so `deferrals.open` means open | card closing procedure; `scripts/retro.py` |
| Widen the PF no-regression set beyond Aldric/Elowen, or state per card that the pair is a floor (lesson 3) | `E1.4`'s harness |
| Every figure copied from an earlier bundle's doc is re-derived before use (FSR-C1 "8 open P1s" was 0) | `AGENTS.md` rule 9 |

---

## Figures table (re-derive before treating this as final)

| Figure | Predicate / denominator | Command |
|---|---|---|
| 57 rows, 51 complete at writing | `kanban.md` lines with `$2 ~ /^ (C\|E)[0-9]/` | `awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/ { n++ } END { print n }' kanban.md`, and the same with `&& $5 ~ /^ complete *$/` (unescape `\|`) |
| 115 commits | `20bf84a3b2..HEAD` on `tranche/17` | `git log --oneline 20bf84a3b2..HEAD \| awk 'END{print NR}'` |
| 160 events, 37 corrections, 15 incidents, 23 reworks, 10 deferrals | all events since 2026-10-02 | `python3 scripts/retro.py summary --since 2026-10-02 --json` (`events.by_type`) |
| 19 of 69 verification runs failed; clippy 9 | `verification` key | same command |
| 782 / 36 / 36 / 0 / 0 | E7.1 parity gate | `artifacts/epic_7/E7.1_cycle_receipt.md` |
| 8,582 units of 12,718 | E0.3 / E3.5 | `artifacts/epic_0/E0.3_cycle_receipt.md`, `artifacts/epic_3/E3.5_cycle_receipt.md` |
| 53 of 53 `verify.sh` stages | E7.2 on `2443fb2ac4` | `bash scripts/verify.sh --show-actuals -j 8` (log `artifacts/epic_7/E7.2_logs/`) |
| PF 49,450 unmoved; hash pair | structural diff; 2 seeds | `structural_diff.py` per `workflow-instruction.md §6` step 5; E1.4 harness |

The counts of events and runs move every time a card emits one. They were read at
`2026-10-07T18:01:14Z`. The totals for deferrals open (8), and the claim that four of them are closed by
their revisit cards, are the most perishable lines here.
