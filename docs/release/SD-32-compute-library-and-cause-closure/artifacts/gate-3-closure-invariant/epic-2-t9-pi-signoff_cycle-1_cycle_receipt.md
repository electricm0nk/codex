# Cycle 1 — Epic 2 (T9 PI review, operator sign-off consolidation) / Card 11 `epic-2-cause-closure`

- **Card ID:** `epic-2-cause-closure`
- **Commit SHA:** (this cycle's commit — see push log)
- **Files touched:** `artifacts/gate-3-closure-invariant/t9-pi-signoff-package.md` (new),
  `docs/retro/events/t9-pi-signoff.jsonl` (new — 1 correction), `progress.md` (this cycle's
  entry), this receipt. **No corpus data changed** (`data/corpus/**` untouched);
  `docs/governance/ogl-pi-blacklist.md` **not amended** (status stays `DRAFT`); `kanban.md` row
  11's status **not touched** (stays `in-progress`).
- **Identifier audit result:** see push log below.
- **Wired-integration audit result:** see push log below.
- **Acceptance criterion:** `decisions.md §18` — consolidate the three per-record review lanes
  (spell; feat+equipment; companion+monster_ability) into a single operator-actionable sign-off
  document. Read-only; does not itself close any T9 unit or change card 11's status.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
  `PCGEN_ORACLE_SHA`), fetched fresh this cycle to the repo-local slot (empty on this fresh
  worktree, self-healed via `scripts/fetch-pcgen-oracle.sh --dest <repo-local pcgen slot>`).
- **Status:** review/evidence-only cycle. Card 11's row stays `in-progress`; T9's onboarding stays
  paused. Nothing in `ogl-pi-blacklist.md` changed.

- **Notes:**

  **Footgun 1 fired on this worktree** — cut from a stray merge with no `docs/`, `data/`,
  `scripts/`. Reset to the pinned SHA (`b4192a712`) and rebased onto `origin/tranche/12` before
  any other work.

  **Verified, did not trust.** Bootstrapped the oracle fresh, rebuilt `v06_work_inventory`, and
  re-ran all five committed scripts myself (`sd32_t9_census.py`, `sd32_t9_pi_exposure_audit.py`,
  `sd32_t9_pi_review_spell.py`, `sd32_t9_pi_review_feat_equipment.py`,
  `sd32_t9_pi_review_companion_monsterability.py`) against a freshly re-derived
  `fresh_inventory.json`. Every lane's headline figures reproduced exactly:
  - spell: 352 reviewed → 0 blocked / 350 clear / 2 still_undecidable; clear-bucket recheck 349,
    0 newly_blocked. **Matches the lane's own report exactly.**
  - companion+monster_ability: 1,630 (443+1,187) reviewed → 0 blocked / 316 clear / 1,314
    still_undecidable; clear-bucket recheck 2,024 rows, 0 newly_blocked. **Matches the lane's own
    report exactly** (verified after correctly restricting the script's combined output to the
    originally-uncertain subset — the script's raw printout mixes clear+uncertain final buckets,
    which reads as a mismatch until filtered).
  - feat+equipment: reproduced the script's mechanical outputs (bucket sizes, 0 newly_blocked
    from the normalized scan, the 5-row `.COPY=` inheritance list) exactly; the memo's further
    manual per-record reads (`Aldori`/`Magaambya` prerequisite citations, `Mantis Blade`) are not
    script-automatable and were cross-read against the memo's own quoted row text, not re-derived
    independently — flagged as the honest limit of this consolidation's verification, not
    silently assumed.

  **One arithmetic correction filed against `t9-pi-review-feat-equipment.md §6`'s own summary
  table** (`docs/retro/events/t9-pi-signoff.jsonl`): its stated equipment `clear` figure
  (`141 − 5 + 4 = 140`) omits subtracting `Mantis Blade`'s move from `clear` to
  `still_undecidable`; correct figure is 139 (`222 total − 82 blocked − 1 still_undecidable =
  139`). Does not change any PI verdict, only bucket-size bookkeeping.

  **Deliverable built:** `t9-pi-signoff-package.md`, covering (in order) the recommendation, the
  clear-bucket re-check result (the null case from normalized scanning, the real 5-unit miss from
  `.COPY=`/`.MOD` inheritance tracing — a different mechanism), the final per-kind and per-book
  disposition table stated against both the audit's original 261/1,107/1,344 (2,712 population)
  and this review's 266/1,988/1,319 (re-derived 3,573 population), the four proposed blacklist
  amendments marked PROPOSED — NOT APPLIED, the 1,319-unit still-undecidable set broken into four
  named reasons each with its operator question, what unblocks on sign-off (1,988 units, 11 fully
  resolved books), and a cross-lane consistency check (no unresolved disagreements found between
  the three lanes).

  **`df -h /` at end of cycle:** see push log.

- **Discovery forwards:** `monster_ability`'s embedded-creature-name problem (954
  still_undecidable units, the single largest remaining gate) and the `bestiary_4` Cthulhu
  ability-vs-spell PI-declaration inconsistency it surfaced are both named in the sign-off
  package §4.1 for the operator's next ruling, not resolved here.
- **Next-cycle plan (named, not attempted this cycle):** none — this cycle's deliverable is the
  terminal artifact for `decisions.md §18`; the next action is the operator's, on the sign-off
  package.
