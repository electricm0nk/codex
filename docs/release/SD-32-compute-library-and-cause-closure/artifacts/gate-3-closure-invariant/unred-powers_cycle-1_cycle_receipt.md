# Cycle unred-powers-1 — un-red `origin/tranche/12` / `tests/v06_work_inventory.rs:1064`

- **Card ID:** none (dispatched directly against a red build at `origin/tranche/12` tip; not a
  numbered kanban row — same shape as the sibling `unred-branch` cycle this one follows).
- **Commit SHA:** (recorded after push, see §6 below)
- **Files touched:** `tests/v06_work_inventory.rs`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion:** the branch must build and test GREEN at tip; no test may be deleted or
  loosened to pass (`decisions.md §1a`); a stale deferral gets re-pinned to the new true state, per
  `docs/governance/deferral-revisit-doctrine.md`.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`
  `PCGEN_ORACLE_SHA`)
- **Status:** complete
- **Notes:** see root-cause analysis below. This is the third stale-deferral assertion fixed this
  bundle (`unred-branch` / `fd6339ce4`, `t2b-refine-kind-fix`, this cycle).
- **Discovery forwards:** none requiring a new card.
- **Next-cycle plan:** none for this specific assertion. Swept for other stale assertions touching
  the five new kinds (`Template`/`Deity`/`Power`/`Domain`/`Language`) landed by `8e98424eb`; found
  none (see §Sweep below).

## Root cause

`origin/tranche/12` tip (`8046a9bfc`) was red:

```
cargo test --locked --test v06_work_inventory --no-fail-fast
FAILED: ultimate_psionics_appears_in_the_inventory_with_real_per_kind_status
  tests/v06_work_inventory.rs:1064
  "up_powers.lst must land in files_not_enumerated -- mapping it to Spell is
   deliberately deferred to Epic 9."
test result: 15 passed; 1 failed
```

Commit `8e98424eb` ("generic enumeration — 5 new kinds landed via data table, `decisions.md §17`")
added `SIMPLE_FILENAME_KINDS`, a data-driven table in `file_kind()` (`src/bin/v06_work_inventory.rs`)
that resolves `*_powers.lst` basenames to a new `Kind::Power`. `up_powers.lst` (Dreamscarred Press
*Ultimate Psionics*, the only book with this file) now enumerates as 421 real `Kind::Power` units
instead of staying in `files_not_enumerated`. `git diff 8e98424eb..HEAD -- src/bin/v06_work_inventory.rs`
is empty — no cycle between `8e98424eb` and this one's own dispatch touched that file, so the
enumeration behavior this test now exercises is exactly what `8e98424eb` landed. Provenance
independently confirmed (not just re-quoted): re-ran the exact assertion at this cycle's own base
(`8046a9bfc`, current tip of `origin/tranche/12`) and reproduced the identical failure before
touching anything.

### Disposition — (a): the deferral is genuinely lifted, but not the way the old comment implied

The old assertion's failure message said *"mapping it to **Spell** is deliberately deferred to
Epic 9"* — Spell, specifically, not "any kind at all". Checked what actually happened:

1. **`15-card-15-other-kinds-memo.md` §3** (the design memo `8e98424eb`'s own receipt cites) analyzed
   `up_powers.lst` directly: 421 units, structurally spell-shaped (`SCHOOL`/`CLASSES`/`CASTTIME`/
   `RANGE`/`TARGETAREA`/`DURATION`/`SAVEINFO`/`SPELLRES`/`DESC` fields match `spell`'s own field
   shape) but filed under a PCGen naming convention (`up_powers.lst`, not `*_spells.lst`) that
   `file_kind()`'s `spell` branch never matches. The memo's own ruling: *"Recommend a new kind
   `power` (parallel to, not folded into, `spell`) — the two are file-distinct in every in-scope
   book's own PCGen data, and folding them would require a cross-kind merge this measurement lane
   is not scoped to perform."*
2. Confirmed independently against `docs/work-inventory.json` on disk (not just re-quoting the
   memo): `ultimate_psionics`'s `spell` kind count is unaffected —
   `jq '[.units[] | select(.book=="ultimate_psionics" and .kind=="spell")] | length' docs/work-inventory.json`
   → `0`, unchanged from before `8e98424eb`. `up_powers.lst`'s 421 rows never became `spell` rows;
   they became `power` rows, all `not-ingested` status
   (`jq '.units[] | select(.book=="ultimate_psionics" and .kind=="power") | .status' docs/work-inventory.json | sort -u`
   → `"not-ingested"` × 421).

So Epic 9's deferral — *"mapping it to Spell"* — is **still true and still unclaimed**: `up_powers.lst`
was never mapped into the `spell` kind's ingest pipeline, and no engine logic treats its rows as
spells. What changed is that the file stopped being *unenumerated entirely* and started being
enumerated as its own first-class kind. The old assertion conflated "not mapped to Spell" with "not
enumerated at all" — those were the same fact when the test was written (no kind existed for it
yet) and stopped being the same fact the moment `Kind::Power` landed. This is exactly
`docs/governance/deferral-revisit-doctrine.md` firing: a deferral condition (no generic-enumeration
mechanism existed for filename-only kinds) that has since been met (`8e98424eb`'s
`SIMPLE_FILENAME_KINDS`), encoded in a test nobody re-read until this cycle.

**No data-table defect** — disposition (b) does not apply. `Kind::Power` claiming `up_powers.lst`
is correct per the memo's field-shape analysis and the file-naming distinctness check; it is not an
accidental sweep-up by an over-broad substring match (`_powers` doesn't collide with any other
in-scope book's basenames — confirmed by the pinned-oracle grep `8e98424eb`'s own commit message
cites, and this cycle found no counter-evidence).

## Fix

Rewrote the pinned assertion (did **not** delete or loosen it) to pin the new, real state:

1. Inverted the `files_not_enumerated` check: `up_powers.lst` must **not** appear there any more
   (was: must appear).
2. Added a new assertion that `ultimate_psionics`'s `power` kind exists with exactly 421 units
   (the memo's own count) — catches a regression where the kind disappears or the count drifts.
3. Added a per-unit status pin (after `up_units` is bound, alongside the existing `feat_statuses`
   check): all 421 `power` units must be `not-ingested` and nothing else — catches both "power
   silently starts being partially graded" (would mean an undocumented ingest landed, contradicting
   Epic 9's still-standing deferral) and "power regresses to blanket not-started" (would mean
   `RuleSetId::Upsi` fell out of `COMPILED_RULE_SETS`, already guarded by the pre-existing
   `assert_ne!(status, "not-started")` loop but reinforced here at the specific-value level).

Comments at the assertion site explain the Epic-9-deferred-Spell-mapping vs. now-enumerated-as-Power
distinction so a future reader doesn't need to re-derive it from this receipt.

## RED → GREEN, proven by mutation (not merely re-run)

Confirmed the new assertions actually catch regression, not just happen to pass:

1. **Baseline (pre-fix) confirmed RED** for the original reason:
   ```
   cargo test --locked --test v06_work_inventory ultimate_psionics_appears_in_the_inventory_with_real_per_kind_status
   FAILED: up_powers.lst must land in files_not_enumerated -- mapping it to Spell is deliberately deferred to Epic 9.
   ```
2. **Fix applied → GREEN:**
   ```
   test ultimate_psionics_appears_in_the_inventory_with_real_per_kind_status ... ok
   ```
3. **Mutation 1** — temporarily re-appended `up_powers.lst` to `docs/work-inventory.json`'s
   `files_not_enumerated` for `ultimate_psionics` (simulating the pre-`8e98424eb` regression) →
   **RED**, correct assertion fired:
   ```
   panicked at tests/v06_work_inventory.rs:1076:5:
   up_powers.lst must NOT land in files_not_enumerated any more -- decisions.md §17 / commit
   8e98424eb enumerates it as Kind::Power via SIMPLE_FILENAME_KINDS. Saw files_not_enumerated: [...,
   "up_powers.lst"]
   ```
   Restored the file byte-for-byte from a pre-mutation copy; `git status --porcelain -- docs/work-inventory.json` confirmed clean before re-running.
4. **Mutation 2** — temporarily set `power` kind's `units` to 420 (simulating a count regression) →
   **RED**, the new count-pin assertion fired. Restored the same way, confirmed clean again.
5. **Final state re-confirmed GREEN** after both mutations were reverted:
   ```
   test ultimate_psionics_appears_in_the_inventory_with_real_per_kind_status ... ok
   ```

`docs/work-inventory.json` carries **zero net diff** from this cycle — only `tests/v06_work_inventory.rs`
changed.

## Sweep for sibling staleness (dispatch brief's explicit ask)

Grepped for pinned counts and deferral comments touching the five new kinds
(`Template`/`Deity`/`Power`/`Domain`/`Language`) across `tests/*.rs`:

```
grep -rn "files_not_enumerated|deliberately deferred|not_enumerated\b" tests/*.rs
grep -n "_templates|_deities|_domains|_powers\.lst|_languages\.lst|Kind::Template|Kind::Deity|Kind::Power|Kind::Domain|Kind::Language" -r tests/*.rs
grep -rn "38391|38,391|38540|38,540|41987|41,987" tests/*.rs   # pinned pre-8e98424eb totals
```

Findings:
- The only `files_not_enumerated`/"deliberately deferred" hit touching the new kinds was the one
  fixed here. `tests/sd13_ranger_third_favored_enemy.rs`'s "deliberately deferred" comment is
  unrelated (favored-enemy mechanics, not enumeration).
- Every other `Kind::Template`/`Deity`/`Domain`/`Language`/`_templates`/`_deities` hit is in the
  **unrelated** `sd17_*` legacy LST-metadata-parser test suite (`MetadataKind`, a different enum in
  a different module, predating `v06_work_inventory.rs`'s `Kind`) or in `sd27_*` tests checking race
  subtype grants from `*_templates.lst` files directly (already correct — they read the corpus, not
  the inventory's `Kind` classification, so `8e98424eb` cannot have staled them).
- No test file pins the pre-`8e98424eb` total (`38,540`/`38,391`) or post-`8e98424eb` total
  (`41,987`) as a hardcoded assertion — those totals live only in receipts/docs, not in `assert_eq!`
  bodies, so `count-change-needs-sweep-not-just-build` does not apply here.

**No other stale assertion found from this specific commit.**

## Verification

```
$ cargo test --locked --test v06_work_inventory --no-fail-fast
test result: ok. 16 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out

$ cargo test --locked --lib
test result: ok. 2390 passed; 0 failed; 13 ignored; 0 measured; 0 filtered out

$ cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml
(see progress.md / this cycle's final report for the recorded result — run in background per
§2.5, polled to completion inside this turn, not left waiting across turns)

$ scripts/verify.sh --only reach
(see progress.md / this cycle's final report)
```

Oracle was bootstrapped fresh in this worktree (`scripts/verify.sh --only preflight-oracle` FAILed
on a clean checkout — the slot is git-ignored — `scripts/fetch-pcgen-oracle.sh --dest <repo-local
pcgen slot>` fetched it, confirmed at pin `7f818006e371188e5717fd18d74d18a420747fc6` before any
figure in this receipt was trusted).

## Other red found (not fixed, out of scope, named per dispatch brief)

`site-dashboard-check` known-failing from unrelated dashboard-JSON staleness (declared in dispatch
brief; not re-derived here to avoid duplicating an already-recorded finding).

## Retro logging

`scripts/retro.py correction` logged (`docs/retro/events/unred-powers.jsonl`, id
`1787464765770-unred-powers-51a340`): subject the test assertion itself, claimed
`files_not_enumerated` must contain `up_powers.lst` (Spell-mapping deferred to Epic 9), actual it is
now enumerated as `Kind::Power` (421 units, not-ingested) via `8e98424eb`'s `SIMPLE_FILENAME_KINDS`
— Epic 9's actual deferral (mapping to *Spell* specifically) was never claimed and remains
undisturbed. Verified by the cargo test RED→GREEN cycle and `git show 8e98424eb`.
