# Cycle 008 — Gate 2 — Engines / Criterion AT-32-G2-004 (F10 binding layer, corpus-wide run)

- **Card ID:** `gate-2-corpus-wide-runs` (kanban.md #8) — engine cycle: `gate-2-engines-f10-binding`
  (card 7)
- **Commit SHA:** (recorded after commit — see push step below; confirmed in `progress.md`'s append)
- **Files touched:**
  - `src/rules_core/pilot_compute/bonus_stack_reader.rs` — added
    `resolve_all_producer_chains_corpus_wide` and `CorpusWideOutcome`/`CorpusWideReport`, the
    population-discovery + full-sweep entry point this card's CLI wraps.
  - `src/bin/bonus_stack_reader.rs` (new) — the `--bin bonus_stack_reader` target card 7's own
    receipt named as open scope: `--corpus-wide --output <path>` (the corpus-wide run) and
    `--fixture-check --input <path> --expected-from <path>` (this card's own fixture-check).
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-2-engines/bonus_stack_reader.corpus-wide.json`
    (new) — the actual corpus-wide run's output.
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-2-engines/bonus_stack_reader.expected.json`
    (new) — the hand-transcribed fixture this card's own fixture-check runs against.
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-2-engines/008_cycle_receipt.md`
    (this file).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  ```
  git diff --unified=0 1bb523773d32705d1b7387fd4c494861523f55ba...HEAD \
    -- src/rules_core/pilot_compute/bonus_stack_reader.rs src/bin/bonus_stack_reader.rs \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
  # also checked directly (untracked, so no git diff): the two new artifacts/gate-2-engines/*.json
  grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' \
    docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-2-engines/bonus_stack_reader.{corpus-wide,expected}.json \
    || echo 'OK_NO_BUNDLE_TAGS'
  ```
  (`BASE_BRANCH` scoped to exactly the two `.rs` files this cycle touched — the wider
  `artifacts/gate-2-engines/` path also picks up card 7's already-committed `007_cycle_receipt.md`,
  which quotes this very grep pattern as documentation and self-matches; scoping to the real diff
  avoids that known false positive rather than silently waving it through.)
- **Wired-integration audit result:** `OK_NO_TOKENS`
  ```
  git diff --unified=0 1bb523773d32705d1b7387fd4c494861523f55ba...HEAD \
    -- src/rules_core/pilot_compute/bonus_stack_reader.rs src/bin/bonus_stack_reader.rs \
    | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
  grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' \
    docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-2-engines/bonus_stack_reader.expected.json \
    || echo 'OK_NO_TOKENS'
  ```
- **Acceptance criterion (verbatim, `acceptance-and-verification.md` AT-32-G2-004):**
  > No engine is "complete" until it has been run corpus-wide once. The corpus-wide run is itself
  > a cycle, with its own receipt, and its own fixture-check, against the closed Gate 1 census.
  > A cycle that runs an engine against a subset and declares the engine done is out of protocol —
  > the subset is not the population the engine claims to handle.

## Scope note — this card's engine, not the sibling

This cycle is the corpus-wide run for the F10 binding-layer engine
(`gate-2-engines-f10-binding`, card 7's generalised `bonus_stack_reader.rs`) only. The
`formula_interpreter` (F1-F9) sibling engine's own card-8 cycle is a separate, disjoint cycle per
`workflow-instruction.md §2.4`'s `pipeline([card(6), card(7)], ..., (_, c) => agent(card(8),
{engineFrom: c}))` — this receipt does not claim or touch that engine.

## What this cycle built

Card 7's own receipt (`007_cycle_receipt.md`, "Next-cycle plan") explicitly left two things open
for this card: (1) a `--bin bonus_stack_reader`-shaped CLI entry point, and (2) the corpus-wide
run itself, fixture-checked, per AT-32-G2-004. Both land in this cycle.

1. **`resolve_all_producer_chains_corpus_wide`** (library, `bonus_stack_reader.rs`) — the
   population-discovery step card 7's `resolve_producer_chain_corpus_wide` did not have: scans
   every record in a caller-supplied population for every distinct `BONUS:VAR` target variable
   (not just ones the caller already knows to ask about), then resolves EACH one's producer chain
   against the SAME full population via card 7's own (unmodified) `resolve_producer_chain_corpus_wide`.
   Returns a `CorpusWideReport { population, outcomes: BTreeMap<String, CorpusWideOutcome> }`
   where `CorpusWideOutcome` is `Resolved(ProducerChain)` or `Refused(String)` — a refusal is
   recorded, never dropped, so the report's own `population` count cannot silently undercount.
2. **`src/bin/bonus_stack_reader.rs`** (new CLI) —
   - `--corpus-wide --output <path>`: walks `data/corpus/**/*.json` (excluding `LICENSE.json`,
     mirroring `corpus_literal_sweep.rs`'s own walker — same exclusion, same sorted-deterministic
     traversal), reads each record's `data.raw_tokens`, and runs the full population through
     `resolve_all_producer_chains_corpus_wide`. Fails closed (exit 2) on a missing corpus root, an
     unreadable-with-zero-population run, or zero distinct target variables found — an empty
     population asserts nothing, `corpus_literal_sweep`'s own posture, deliberately mirrored.
   - `--fixture-check --input <path> --expected-from <path>`: this card's own fixture-check
     mechanism (AT-32-G2-004's "its own fixture-check"). For each variable named in
     `--expected-from`, compares the corpus-wide run's outcome for it: `resolved` entries compare
     `status`/`base`/`addends` fields exactly (structural facts transcribable directly off corpus
     bytes, independent of the engine); `refused` entries compare `status` plus a caller-named
     `reason_contains` substring (a refusal's exact prose is engine-generated, not an
     independently-transcribable corpus fact — pinning it byte-for-byte would assert
     engine-internals-as-written rather than the real "this variable is not safely resolvable, and
     here is which real corpus PRE-tag caused it" fact). Exit 1 on any mismatch, exit 2 on an
     empty expected-fixture (same fail-closed posture).

### Deviation from `acceptance-and-verification.md`'s literal command block, and why

`acceptance-and-verification.md`'s Gate 2 block shows `derived_evaluator_fixture_check --input
... --expected-from ...`. That binary's own module doc (`derived_evaluator_fixture_check.rs`)
scopes it specifically to `tests/fixtures/rules_core/derived-evaluator-fixtures.json`'s `derived`
wiring-class units — it takes no `--input`/`--expected-from` flags at all, and the acceptance
doc's own text flags that whole block as "the contract, not a runnable command" pending Gate 2's
own deliverables. F10's producer-chain shape (a structural `{base, addends}` resolution, not a
single evaluated `derived` value) does not fit that binary's fixture format. This cycle
implements the fixture-check as a mode of the `bonus_stack_reader` binary itself instead, against
an F10-specific `expected.json` this cycle also authors — the honest minimal implementation of
AT-32-G2-004's "its own fixture-check", not a forced fit into a CLI scoped to a different unit
kind. Flagged explicitly rather than silently reinterpreted.

## RED → GREEN evidence

**RED (symbols did not exist at the pinned base):**
```
git show HEAD:src/rules_core/pilot_compute/bonus_stack_reader.rs \
  | grep -c "resolve_all_producer_chains_corpus_wide\|CorpusWideReport"
# -> 0
ls src/bin/bonus_stack_reader.rs   # -> No such file or directory (before this cycle)
```

**GREEN — library:**
```
CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-gate-2-corpus-run \
  cargo test --locked --lib rules_core::pilot_compute::bonus_stack_reader
# test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 2346 filtered out
```
21 tests (18 pre-existing from waves 26 and card 7, unchanged and still green — no regression —
plus 3 new tests for `resolve_all_producer_chains_corpus_wide`). Widest relevant scope also run
clean:
```
CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-gate-2-corpus-run \
  cargo test --locked --lib rules_core::pilot_compute::
# test result: ok. 833 passed; 0 failed; 0 ignored; 0 measured; 1534 filtered out
```
(833 vs card 7's own receipt's 830 — the 3 new tests, no regressions.)

**The load-bearing new-code proof** (`resolve_all_producer_chains_corpus_wide_finds_and_classifies_every_distinct_target_var`):
reads three REAL corpus records (`alchemist/bomb.json`, `master_chymist/bomb_thrower.json`,
`witch_hex/ward.json`'s own `BONUS:VAR` tokens) and asserts the sweep finds and correctly
classifies all three of `AlchemistBombLVL` (resolves), `WitchWardBonus` (resolves, no `DEFINE`),
and `WitchHexDC_Ward` (refuses — real `PREABILITY` gate) in ONE call, with none dropped and none
misclassified by another's outcome.

**Mutation proof** (`resolve_all_producer_chains_corpus_wide_uses_every_scanned_record_not_just_the_first`):
a sweep over only `alchemist/bomb.json` resolves `AlchemistBombLVL` with 1 addend, not the
multi-record total — proving the sweep genuinely feeds every scanned record into resolution, not
just `records[0]`.

**Fail-closed proof** (`resolve_all_producer_chains_corpus_wide_over_no_records_reports_zero_population`):
an empty record set reports `population: 0` and an empty `outcomes` map — the CLI's own
`--corpus-wide` mode turns that into a hard exit-2 failure (see below), never a silent "0 done,
0 to check, report success".

**GREEN — the real corpus-wide run itself:**
```
CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-gate-2-corpus-run \
  cargo run --locked --bin bonus_stack_reader -- --corpus-wide \
  --output artifacts/gate-2-engines/bonus_stack_reader.corpus-wide.json
# bonus-stack-reader: 26932 record(s) scanned, 0 unreadable, 4736 distinct F10 target
# variable(s) found (3519 resolved, 1217 refused) -> .../bonus_stack_reader.corpus-wide.json
```
26,932 is every `data/corpus/**/*.json` record (excluding `LICENSE.json`) at this cycle's HEAD —
the whole shipped population, not a subset (AT-32-G2-004's own bar). 4,736 distinct `BONUS:VAR`
target variables were found; 3,519 (74.3%) resolve cleanly (this reader's understood shape:
`DEFINE` base + `PREVARGTEQ`-or-ungated `BONUS:VAR` addends, agreeing across every record that
mentions the variable); 1,217 (25.7%) refuse (an unrecognised PRE-tag kind on some addend, or a
cross-record disagreeing `DEFINE`) — refused is a correct, disclosed outcome for a variable this
reader has genuinely not verified how to resolve, not a defect. This is a related but DIFFERENT
denominator from card 7's own cited "77.2% (893 of 1,156)" figure
(`MEASURE-TWICE.md §3.1`/`epic-breakdown.md` Epic 1 F2) — that figure counts distinct CUSTOM
IDENTIFIERS by a different classification pass; this run counts distinct `BONUS:VAR` TARGET
NAMES found by this binary's own literal token scan. Both are real, both are re-derivable by
their own stated commands; this receipt does not claim they are the same measurement and neither
number supersedes the other.

**GREEN — the fixture-check (AT-32-G2-004's "its own fixture-check"):**
```
CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-gate-2-corpus-run \
  cargo run --locked --bin bonus_stack_reader -- --fixture-check \
  --input artifacts/gate-2-engines/bonus_stack_reader.corpus-wide.json \
  --expected-from artifacts/gate-2-engines/bonus_stack_reader.expected.json
# bonus-stack-reader: fixture-check OK — 3 variable(s) matched their expected outcome exactly
```
`bonus_stack_reader.expected.json` transcribes, by hand, directly off the named corpus files'
`data.raw_tokens` (never regenerated from this binary's own output — each entry's `_sources`
field cites the exact file(s) and token line(s) read): `AlchemistBombLVL` (base `"0"`, 3 addends —
see "Discovery" below), `WitchWardBonus` (base `"0"`, 3 addends, 2 gated), `WitchHexDC_Ward`
(refused, `PREABILITY` gate). All three matched exactly.

**Fixture-check mutation proof (RED for the check mechanism itself):** mutating one expected
value (`WitchWardBonus`'s `base` from `"0"` to `"99"`) and re-running:
```
cargo run --locked --bin bonus_stack_reader -- --fixture-check \
  --input artifacts/gate-2-engines/bonus_stack_reader.corpus-wide.json \
  --expected-from /tmp/bad_expected.json
# bonus-stack-reader: FAIL WitchWardBonus: expected status/base/addends ... "base":"99" ...,
# got ... "base":"0" ...
# bonus-stack-reader: fixture-check found 1 mismatch(es) of 3 checked
echo $?   # -> 1
```
Proves the fixture-check genuinely compares values rather than always reporting success.

## Corpus SHA

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`) —
this cycle's own figures (26,932 records, 4,736 variables, 3,519/1,217 split) are re-derived from
`data/corpus/` (the already-ingested, shipped corpus), not a live oracle read; the oracle itself
was not queried by this cycle's own commands (per `2.1`'s preflight, it was already present at
the pin from a prior cycle in this worktree — `scripts/verify.sh --only preflight-oracle` was not
re-run since no command in this cycle touches `$PCGEN_CORPUS_ROOT`).

## Status

**complete**

## Discovery forwards

**One real finding, logged as a retro correction, not a new card** (it corrects a fact stated in
card 7's own receipt, does not change this bundle's scope): card 7's `007_cycle_receipt.md` names
exactly two real producers of `AlchemistBombLVL` (`alchemist/bomb.json`,
`master_chymist/bomb_thrower.json`). This cycle's corpus-wide sweep found a THIRD, real,
independently-verified producer: `data/corpus/inner_sea_magic/class_feature/crypt_breaker/alkahest_bombs.json`
carries its own `DEFINE:AlchemistBombLVL|0` (agreeing with the other two — no refusal) and
`BONUS:VAR|AlchemistBombLVL|AlchemistLVL`. This is exactly the shape the corpus-wide run exists to
catch (a producer no single-record or two-record proof could see) and is now the load-bearing
fixture case for `AlchemistBombLVL` in `bonus_stack_reader.expected.json`.
```
scripts/retro.py correction --subject card-7-007_cycle_receipt \
  --claimed "AlchemistBombLVL has 2 known producers" \
  --actual "AlchemistBombLVL has a 3rd real producer: inner_sea_magic/.../alkahest_bombs.json" \
  --verified-by "grep -rl 'AlchemistBombLVL|AlchemistLVL' data/corpus/"
```

## Next-cycle plan

Gate 2 for the F10 engine is now closed (AT-32-G2-001 via card 7, AT-32-G2-004 via this card).
AT-32-G2-002/003 (per-value fixture clearance narrative, proof-width statement) are satisfied by
this card's own fixture-check plus card 7's receipt's disclosed proof-width section — no further
cycle is owed on this engine unless the sibling `formula_interpreter` (F1-F9) engine's own card 6
→ card 8 chain surfaces a shared-surface conflict (none expected; disjoint files per
`technical-design.md`'s File-disjointness table). Gate 2 overall stays open until the sibling
chain's own card 8 cycle lands its receipt. Gate 3 (`gate-3-closure-invariant`, card 9) is gated
on Gate 2 as a whole, not per-engine.
