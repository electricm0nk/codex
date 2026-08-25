# Cycle 1 — Gate 3 / Epic 2 — formula interpreter wired into race-trait ingestion

- **Card ID:** `epic-2-cause-closure` (row 11) — closes the "needs the formula
  interpreter `decisions.md §24` forbids" blocking reason on the ARG lane's
  escalation; row 11 itself is left `in-progress` per this cycle's brief
  (other T2b/T9/T12/T2a/T4-L9 sub-populations named in `decisions.md §13` are
  unaffected and remain open).
- **Actor:** `interpreter-race-trait-wiring`
- **Base:** worktree started on a stray `site-publish` merge commit
  (`275581bf0`, footgun 1, fired again — no `docs/`, `data/`, `scripts/`) —
  `git reset --hard 07c88775d7f9fcacffef6d825807a81fed89d8d4`, re-verified,
  then `git fetch origin tranche/12` (already at that tip: `git rebase
  origin/tranche/12` was a no-op, "Current branch … is up to date").
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env` `PCGEN_ORACLE_SHA`), fetched fresh via
  `scripts/fetch-pcgen-oracle.sh --dest <repo-local slot>` after
  `scripts/verify.sh --only preflight-oracle` FAILed on this fresh worktree
  (empty git-ignored oracle slot) — matches the pin exactly after fetch.
- **Environment note:** the sandbox refused writes under
  `/home/ubuntu/.cache/...` ("too complex to verify that it stays inside the
  worktree") — a path-outside-worktree refusal, not a git-operation issue.
  Used an in-worktree `CARGO_TARGET_DIR=.cargo-target-scratch` per the
  brief's documented fallback.

## 0. Re-derived the real blocked population before building anything

The dispatch brief cited "29 units" (ARG's Samsaran `Mystic Past Life` trait +
28 per-class CHOOSE-target primitives). `artifacts/gate-3-closure-invariant/
card11-t2b-remeasure.md §6` (read per the brief, committed 2026-08-23) had
already re-derived that this specific 29-unit claim is **stale**: Decision
20 overturned the formula-interpreter ban on 2026-08-21, so the 29 units'
real remaining blocker is `Samsaran` not being in `ingest_race_traits.rs`'s
`IN_SCOPE_RACES` roster (a new-race-onboarding scope question), not formula
evaluation — Samsaran is not even ingested yet, so `ingest_race_traits.rs`
never reaches its `Mystic Past Life` row at all regardless of interpreter
wiring.

That memo explicitly did not itself wire the interpreter (§9, "did not wire
the formula interpreter into race-trait ingestion"), so the population
**actually reachable and closable by this cycle** — race-trait rows already
in scope whose `DESC:`/`BONUS:VAR` resolution fails *only* because no formula
evaluator was wired, not because of a missing chassis, an external
(cross-record/character-state) variable, or a scope ruling — needed its own
fresh derivation, corpus-wide over every book `ingest_race_traits.rs` and
`ingest_races.rs` actually declare:

```
python3 .cargo-target-scratch/scan.py   # ingest_race_traits.rs's 8 declared source files
python3 .cargo-target-scratch/scan2.py  # ingest_races.rs's 34 in-scope race directories
```
(both scripts scan `$PCGEN_CORPUS_ROOT` for `DESC:` argument tails matching
`<identifier>[+-]<integer>` — the one same-row-formula shape either binary's
pre-wiring code could not resolve — and for non-literal `BONUS:VAR` amounts)

Result: **exactly 1 unit** — `advanced_race_guide/arg_abilities_race.lst:227`
(`Halfling ~ Adaptable Luck`), whose real `%2` `DESC:` argument is the
expression `Halfling_AdaptableLuck_Bonus-1` (base 2 via same-row
`DEFINE:…|0` + `BONUS:VAR:…|2`, so the correct value is 1). Every other
non-literal `BONUS:VAR`/`DESC:` shape found in-scope (172 `BONUS:VAR`
instances in `ingest_races.rs`'s 34 races, ~80 more in `ingest_race_traits`'s
4 books) names a variable the row **never itself defines** — `CHA`, `TL`,
`CHASCORE`, `WIS`, a class-feature-owned pool variable — which is a missing
*binding*, not a missing *evaluator*; wiring in `PcgenFormulaEvaluator`
correctly still refuses these (verified below, §3), because they are
character-state/cross-record dependencies real ingestion-time data cannot
supply, formula interpreter or not.

**Retro correction logged** (the dispatch brief's 29-unit figure vs. the
real in-scope-and-closable population of 1):
```
scripts/retro.py correction --subject "dispatch brief (this cycle's own handoff)" \
  --claimed "29 units blocked on the formula interpreter" \
  --actual "1 unit (Halfling ~ Adaptable Luck's %2 DESC arg) is blocked purely on the interpreter; \
the 29-unit ARG figure's real remaining blocker is Samsaran not being in IN_SCOPE_RACES (a scope \
ruling, per card11-t2b-remeasure.md §6, not the interpreter)" \
  --verified-by "python3 .cargo-target-scratch/scan.py; python3 .cargo-target-scratch/scan2.py"
```
Event: `docs/retro/events/interpreter-race-trait-wiring.jsonl`.

## 1. What was wired

**No second evaluator.** Reused `formula_interpreter::PcgenFormulaEvaluator`
(`src/rules_core/pilot_compute/formula_interpreter.rs`, unmodified) via one
new thin binding module:

- `src/rules_core/pilot_compute/race_trait_formula_binding.rs` (new) —
  `pub fn resolve_same_row_formula(name, vars: &BTreeMap<String, Option<i64>>)
  -> Option<i64>`: direct-key lookup, then bare-literal parse, then (only for
  a `name` the interpreter's own `recognises_shape` accepts) evaluation via
  `PcgenFormulaEvaluator` against every currently-`Some`-resolved same-row
  variable — never a `None` one, and never a name absent from the caller's
  table, so an external/cross-record reference still refuses exactly as
  before. Registered `pub mod race_trait_formula_binding;` in
  `src/rules_core/pilot_compute/mod.rs`.
- `src/bin/ingest_race_traits.rs` — `same_row_vars`'s `BONUS:VAR` amount,
  `eval_prevar_gate`'s operand closure, and `substitute_placeholders`'s `%N`
  argument resolution all now call `resolve_same_row_formula` in place of
  their old "literal-or-bare-name-only" logic. Module doc comments citing the
  overturned `decisions.md §24` ban updated to name Decision 20 and this
  module.
- `src/bin/ingest_races.rs` — the identical three call sites (this binary
  ships its own copy of the same row-shaped logic, per its own module doc
  explaining why a `.lst`-row-shaped copy is not the "fourth private copy"
  `pcgen_desc.rs`'s doc warns against) updated the same way.
- `src/rules_core/pcgen_desc.rs` — doc-only: the module doc's `%N`-drop bullet
  cited `decisions.md §24` as still forbidding a general interpreter; updated
  to name Decision 20's overturn and this cycle's ingest-time fix, without
  touching `resolve_desc_argument`'s own narrower runtime-display-value logic
  (out of this cycle's scope — see §5).

## 2. Fixture discipline (`decisions.md §3`)

`race_trait_formula_binding.rs`'s own test module hand-transcribes the
`Halfling ~ Adaptable Luck` row's real bytes (never read by the code under
test) and asserts the interpreted result against them:

```rust
// DEFINE:Halfling_AdaptableLuck_Bonus|0 + BONUS:VAR|Halfling_AdaptableLuck_Bonus|2 -> base 2
// DESC arg "Halfling_AdaptableLuck_Bonus-1" -> 2 - 1 = 1 (hand-transcribed, not read by the evaluator)
resolve_same_row_formula("Halfling_AdaptableLuck_Bonus-1", &v) == Some(1)
```
Plus fixtures proving the refusal boundary holds (unresolved same-row var,
external/never-defined var, unrecognised token shape) and a multi-operator
case (`min(floor((TL+1)/2),5)`, `10+(TL/2)+CON`) proving this is genuinely
the shared interpreter's full grammar, not a hand-rolled subtraction-only
subset. `ingest_race_traits.rs`'s own `adaptable_luck_resolves_what_the_row_
states_and_drops_only_what_it_does_not` test carries the same hand-transcribed
expected value end-to-end through the row parser.

## 3. RED → GREEN, preserved

**RED** (before the production fix; row parser test, `cargo test --locked
--bin ingest_race_traits adaptable_luck`):
```
left:  "...if they choose to do so afterward, they only gain a +1 bonus...."
right: "...if they choose to do so afterward, they only gain a bonus...."
assertion `left == right` failed
```
(`left` is the code's actual output at the moment this test was run — the
race_trait_formula_binding wiring was already in place in `same_row_vars`/
`substitute_placeholders`; `right` is the still-stale test expectation. This
is the intended-reason RED: the fix already changed production output, the
test hadn't been updated yet.)

**GREEN** after updating the test's expected description and
`unresolved_desc_args` assertion to the corrected values (§1 above) —
`cargo test --locked --bin ingest_race_traits` → 16/16 passed.

A second, independent RED→GREEN surfaced from a sibling consumer
(`apps/desktop/src-tauri/src/race_trait_picker.rs`'s
`every_menu_row_has_a_rendered_description_and_none_leaks_pcgen_syntax`,
which compares the corpus's *stored* `data.description` against a live
re-render and pins the list of records where they still differ):
```
left:  ["Oversized Goblin", "Nagaji ~ Hypnotic Gaze", "Suli ~ Energy Strike", "Undine ~ Nereid Fascination"]
right: ["Oversized Goblin", "Halfling ~ Adaptable Luck", "Nagaji ~ Hypnotic Gaze", "Suli ~ Energy Strike", "Undine ~ Nereid Fascination"]
```
`right` (the old hardcoded expectation) still listed `Halfling ~ Adaptable
Luck` as a record whose stored description diverges from its live
re-render; after this cycle's corpus regeneration (§4) the stored value now
matches the live re-render (both compute "+1"), so the record correctly
dropped out of the "differs" list. Updated the assertion and its doc comment
accordingly. Full suite: `cargo test --locked --manifest-path
apps/desktop/src-tauri/Cargo.toml` → 518/518 passed (both before-my-edit RED
and after-my-edit GREEN observed directly, not inferred).

## 4. Corpus regeneration — guarded generator path only

```
cargo run --locked --bin ingest_race_traits -- advanced_race_guide
```
`git diff --stat -- data/corpus/advanced_race_guide/race_trait` → **307
files changed, 308 insertions(+), 308 deletions(-)**: 306 of those are a
1-line `ingested_at` timestamp bump only (the binary stamps a fresh
timestamp on every run of a book, whether or not any record's content
moved — confirmed by content diff, not assumed), and exactly **one** file
carries a real content change:

```
git diff -- data/corpus/advanced_race_guide/race_trait/halfling/halfling_adaptable_luck.json
```
→ `description` field: `"...they only gain a bonus..."` → `"...they only
gain a +1 bonus..."`. No hand-editing; the regeneration ran the real ingest
binary against the real pinned oracle. `ingest_races.rs` was NOT re-run for
its 34 races — §0's scan found zero DESC-arg-formula shapes in that binary's
scope, so re-running it would only churn 34 races' worth of `ingested_at`
timestamps for zero content change; skipped per `decisions.md §1a` (no
gaming a count with a no-op regen).

## 5. What remains genuinely blocked, and on what

**Not on the formula interpreter.** After wiring, `cargo run --bin
ingest_race_traits -- advanced_race_guide`'s own run report still lists
6 unresolved `DESC:` args (down from 7 pre-wiring — `Halfling_
AdaptableLuck_Bonus-1` is the one that dropped out): `Undine ~ Nereid
Fascination` (×2), `Nagaji ~ Hypnotic Gaze` (×2), `Suli ~ Energy Strike`,
`Wayang ~ Dissolution's Child`. Every one of these names a variable
(`TL`, `Nagaji_RacialCastingMod`, `Suli_ElementalAssault_Duration`, …) this
row itself never `DEFINE`s — cross-record or live-character-state
dependencies (total level, another feature's own casting-modifier variable)
that no same-row formula evaluator, wired or not, can supply from ingested
data alone. This is the same finding `card11-t2b-remeasure.md §4`'s per-book
table already named for these four books; unchanged by this cycle.

**Samsaran's 28-unit `Mystic Past Life` shape (the brief's original "29
units", minus the 1 real Halfling unit this cycle actually closed) is
blocked on Samsaran not being in `IN_SCOPE_RACES`** — a race-onboarding
scope question `card11-t2b-remeasure.md §6`/§7 item 4 already escalated,
unrelated to the interpreter (confirmed: `ingest_race_traits.rs`'s
`IN_SCOPE_RACES` still excludes Samsaran, and the row is never reached by
this binary regardless of what evaluator is wired behind it). Not re-filed
against the interpreter ban — that ruling no longer exists (Decision 20) —
and not treated as closed by this cycle, since it is a different, real,
still-open blocker (scope, not evaluation capability).

## 6. Test runs (all in-turn, foreground/monitored)

```
cargo test --locked --bin ingest_race_traits          -> 16 passed, 0 failed
cargo test --locked --bin ingest_races                -> 44 passed, 0 failed
cargo test --locked --lib rules_core::pilot_compute::  -> 862 passed, 0 failed (incl. new module's 7 fixture tests)
cargo test --locked --lib                              -> 2397 passed, 0 failed, 13 ignored
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml -> 518 passed, 0 failed
scripts/verify.sh --only reach                         -> PASS (31 passed)
scripts/verify.sh --only preflight-oracle               -> PASS (oracle at pin 7f818006e371188e5717fd18d74d18a420747fc6)
```

## 7. Dual audit (§6 step 2/4, over this cycle's own diff)

```
git diff --unified=0 07c88775d7f9fcacffef6d825807a81fed89d8d4 -- src tests data docs/release \
    ":!**/__tests__/**" ":!**/*.test.*" \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
-> OK_NO_BUNDLE_TAGS

git diff --unified=0 07c88775d7f9fcacffef6d825807a81fed89d8d4 -- src tests data docs/release \
    ":!**/__tests__/**" ":!**/*.test.*" \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
-> OK_NO_TOKENS
```
Note: the workflow-instruction.md §6 step 2 canonical form uses
`BASE_BRANCH=$(git merge-base HEAD origin/develop)` (`1bb523773d`), which
diffs the *whole* `tranche/12` branch against `develop`, not just this
cycle's own commits; that wider diff carries 168 pre-existing `sd18_*`/
`sd20_*`/etc. matches from earlier, already-landed cycles' own commentary
text (test filenames, doc prose) — none of them introduced by this cycle,
confirmed by the narrower diff above scoped to this cycle's actual starting
point (`07c88775d`, this session's own base). Both audits pass at this
cycle's own scope.

## 8. Files touched

- `src/rules_core/pilot_compute/race_trait_formula_binding.rs` (new, 7 tests)
- `src/rules_core/pilot_compute/mod.rs` (module registration)
- `src/bin/ingest_race_traits.rs` (wiring + doc updates + 1 test updated)
- `src/bin/ingest_races.rs` (wiring + doc updates)
- `src/rules_core/pcgen_desc.rs` (doc-only correction)
- `apps/desktop/src-tauri/src/race_trait_picker.rs` (1 test's expected list
  updated to reflect the closed divergence)
- `data/corpus/advanced_race_guide/race_trait/**` (307 files regenerated via
  `cargo run --bin ingest_race_traits -- advanced_race_guide`; 1 real content
  change, 306 timestamp-only)

## 9. Status

**complete** for this cycle's scope: the formula interpreter is wired into
both race-trait ingest binaries' same-row `DESC:`/`BONUS:VAR` resolution, the
one in-scope unit genuinely blocked on it is closed (fixture-checked,
RED→GREEN preserved in two independent consumers), and every other
previously-unresolved DESC-arg case in-scope is confirmed (not merely
asserted) to be blocked on a different, real cause — an external variable
binding or a race-scope ruling — not the interpreter. Kanban row 11 left
`in-progress` per the brief (its other four sub-populations are untouched by
this cycle).

## Disk usage

```
df -h /
```
→ `/dev/sda1  968G  367G  602G  38% /` — no pressure, no cleanup needed.
