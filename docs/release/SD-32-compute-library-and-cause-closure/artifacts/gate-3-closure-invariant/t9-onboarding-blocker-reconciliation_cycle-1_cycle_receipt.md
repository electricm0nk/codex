# Cycle t9-onboarding (blocker reconciliation) — `progress.md` `## Open blockers` / `## DISCOVERED` reconciliation

- **Card ID:** `epic-2-cause-closure` (row 11; docs-only reconciliation, no code/corpus write).
- **Actor:** `t9-onboarding`
- **Base:** `54a5d94ef6158b13fa818271b068b71b2100ebbd` (`PIN`, == `origin/tranche/12` HEAD at cycle
  start — no rebase needed).
- **Files touched:** `docs/release/SD-32-compute-library-and-cause-closure/progress.md` only.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff).
- **Wired-integration audit result:** `OK_NO_TOKENS` (own diff).
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (oracle bootstrapped fresh into this
  worktree's empty repo-local slot via `scripts/fetch-pcgen-oracle.sh`, per §2.1).
- **`no_record` before/after (this cycle made no corpus/code changes):**
  `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json` →
  `no_record 982 / 35328` both times, unmoved.

## Scope

Reconcile `progress.md`'s `## Open blockers` (4 entries, all card 11, all filed 2026-08-22/23) and
`## DISCOVERED` (9 top-level entries, not 0 — the orchestrator's own "0 bullet items" count was
stale/wrong, established fresh this cycle by `awk`/`grep` over the live file) against current
reality, per `decisions.md §10`/`blocker-closure-doctrine.md`.

## `## Open blockers` — all 4 entries dispositioned, all RESOLVED, all removed with a citation note

1. **"remaining blocker shapes" (filed 2026-08-22, closure-epilogue).** RESOLVED — superseded by
   `decisions.md §10` (rejected the filing's own forward-scope-deferral premise; the entry's own
   addendum already recorded this) and `§13` (operator ruling: all five sub-populations close by
   doing the work). Removed, one-paragraph note left citing `§10`/`§13` and the current bundle-wide
   `no_record` figure (20,889 → 982).
2. **"reopened, ruling needed on four shapes" (reclosure-epilogue cycle 2, 2026-08-22).** RESOLVED
   — answered directly by `decisions.md §13`. Removed, note citing `§13` and pointing at entries 3/4
   below for how T2b specifically closed.
3. **T2b `inner_sea_races` 45-unit residual (lane `epic-2-t2b-w1b`, 2026-08-23).** RESOLVED — NOT
   by the chassis-wiring path this filing asked the operator to sequence, but by the
   generic-verbatim-ingest mechanism `decisions.md §17`/`§20` authorized afterward:
   `scripts/ingest_race_trait_generic.py` (commit `75ea0c9109`, 1,883 → 5) then
   `scripts/ingest_generic_kind.py` (commit `eba2fd7f04`, 6 → 0). Re-derived this cycle:
   `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json` → `race_trait`
   `no_record` **0**, corpus-wide (was 2,001 total `race_trait` rows, `Counter({'no_formula_tokens':
   1102, 'matched': 899})`, zero `no_record`). The chassis/heritage-selector work this filing named
   (Dhampir/Changeling/Skinwalker, 15 chassis-less races) is still genuinely unbuilt — that is
   reachability scope, not a Gate 1 `no_record` blocker, and the removal note says so explicitly so
   it isn't misread as "the chassis got built."
4. **T2b `bestiary_5` fully out of ingest-tool-extension scope (lane `epic-2-t2b-w1b`,
   2026-08-23).** RESOLVED — same two commits, same re-derivation, same caveat (8-race chassis
   batch + Skinwalker heritage-selector + cross-book `Adopted Race` selector remain unbuilt;
   reachability, not `no_record`).

**Verification method (`§17a`):** did not trust the brief's "race_trait is now at ZERO" claim —
ran `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json` myself against the
freshly-bootstrapped pinned oracle, got the by-kind `no_record` breakdown
(`spell` 285, `monster_ability` 267, `companion` 217, `equipment` 170, `equipment_modifier` 43,
total 982 — no `race_trait` in the list at all), and separately confirmed `race_trait`'s full
row population (2,001) has zero `no_record` rows. Then traced the exact two commits that did it
(`git log --oneline --all | grep -i race`, `git show --stat` on each, cross-checked against
`git log HEAD` to confirm both are ancestors of HEAD) and read both cycles' own receipts
(`epic-2-race-trait-generic-ingest_cycle-1_cycle_receipt.md`,
`t9-t2-race-monster-class-racetrait-no-record-closure_cycle-1_cycle_receipt.md`) to confirm they
explicitly name `inner_sea_races` and `bestiary_5` as part of the population closed, not merely
consistent by kind-total arithmetic.

## `## DISCOVERED` — real state established: 9 entries, not 0

The orchestrator's brief reported the orchestrator's own count found 0 bullet items. That count was
wrong or mis-scoped — `## DISCOVERED` exists at `progress.md:2113` (post-edit line number) with
**9** top-level `- 2026-...` bullet entries beneath it (verified with
`awk '/^## DISCOVERED/{f=1} f && /^## / && !/^## DISCOVERED/{print; exit}'` to find the section's
own end, then counting `^- 20` bullets inside that span). 9 < 10, so the queue is **not** at the
`workflow-instruction.md §8` non-self-healable ceiling — establishing that fact, not closing every
entry, was this cycle's job here.

Of the 9:

- **1 was already marked `RESOLVED` inline by the document itself** (the `gen_book_cache.rs`
  self-erasure finding, `boundary-branch-review` cycle) — no action needed, already correct.
- **1 closed this cycle** (ten-kind-list-omits-`class_feature` finding, `gate-0-census-closure`
  Cycle 2): **RESOLVED, `decisions.md §12`** (operator ruling added cards 14/15 to close exactly
  this population; card 15's `§20` campaign has since ingested most of the named buckets). Note
  appended in place, original bullet retained (historical record) per the same convention used for
  the Open-blockers entries.
- **1 partially addressed this cycle, left open** (the ability `data.key`/`data.raw_tokens` PI-leak
  discovery, 503-candidate population, `decisions.md §24 / ability-pi-rename` cycle): the proposed
  target (a dedicated generic audit lane) **landed** — `pi-key-rawtokens-screen` (commit
  `95348a92e`) built `scripts/pi_key_rawtokens_audit.py` and ran it corpus-wide, and its own `§17a`
  self-correction found the 503 figure over-counted already-redacted records; the real confirmed
  count is 6 (2 fixed by that cycle, 4 more found and named, not yet remediated). Per
  `decisions.md §15`'s standing rule ("never transcribe, never silently skip" on a suspected PI
  record), this is reported as **partially resolved, still real open PI remediation work**, not
  closed — the 4 outstanding leaks are named by kind (`domain`/`equipment`/`language`/`spell`) with
  their retro-deferral id in the note.
- **6 left untouched, genuinely still open**, all real per-object/mechanism work with named
  proposed targets not yet fully executed (the T2b `refine_kind` classifier's core_rulebook/APG
  Favored-Class-Bonus false-positive shape — though `decisions.md §16` already treats the
  underlying finding as settled/finding-of-record, with item 1 of its 3-step plan landed via
  `t2b-refine-kind-fix`; the Adopted-Race/Adoptive-Parentage selector scope, itself later corrected
  by a same-queue entry to be two shapes not one, with 7 of 21 units closed and 14 needing a new
  `kind: trait` content surface — an explicit open escalation already stated in its own entry; and
  the SD-31-scope doc corrections in `worktree-wf_c1156061-e3f-5`). None of these six is a stale
  filing masquerading as done — each still names real, unclosed work.

## What was NOT done (explicitly out of this cycle's scope)

- No code or corpus write. No cargo build. No card 11/15 status change (both remain `in-progress`
  per the dispatch brief's own constraint).
- No scope moved into `forward-scope-register.md` — every resolution above is either "the work
  landed" (cited by commit+command) or "an operator ruling already answered it" (cited by
  decisions.md section), never a scope transfer.
- Did not attempt to fully resolve the 6 still-open `## DISCOVERED` entries — reported their real
  state accurately, per the brief's own instruction to establish, not necessarily close, the real
  state of that queue.

## Suites / build

Docs-only change; no Rust/Python source touched. No test suite re-run required or attempted (would
be out of scope and non-informative for a `.md`-only diff). `scripts/shape_ledger.py` was run
read-only against the committed `docs/work-inventory.json` and the bootstrapped oracle, twice
(before/after), for verification only — no write.

## Discovery forwards

None new. The 6 still-open `## DISCOVERED` entries and the 4 outstanding PI leaks are pre-existing,
already named in the file itself (not re-filed here).

## Next-cycle plan

- A T9/T2b-scoped cycle can pick up the 4 outstanding `domain`/`equipment`/`language`/`spell` PI-key
  leaks named in the `pi-key-rawtokens-screen` follow-up note.
- A card-11-scoped cycle can pick up the Adopted-Race/Adoptive-Parentage 14-unit `kind: trait`
  escalation (needs an operator ruling per that entry's own text — SD-32 scope vs. successor
  bundle, per `AGENTS.md` Blocker Discipline disposition 2).
- No other action required by this reconciliation cycle.
