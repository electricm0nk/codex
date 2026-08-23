# Cycle 1 — Epic 2 (T9 per-record PI review, `spell` kind lane) / Card 11 `epic-2-cause-closure`

- **Card ID:** `epic-2-cause-closure`
- **Commit SHA:** (this cycle's commit — see push log)
- **Files touched:** `scripts/sd32_t9_pi_review_spell.py` (new),
  `artifacts/gate-3-closure-invariant/t9-pi-review-spell.md` (new),
  `docs/retro/events/spell.jsonl` (new — 1 correction), `progress.md` (this cycle's entry). **No
  corpus data changed** (`data/corpus/**` untouched); `docs/governance/ogl-pi-blacklist.md` **not
  amended** (status stays `DRAFT`); `kanban.md` row 11's status **not touched** (stays
  `in-progress`).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  (`BASE_BRANCH=$(git merge-base HEAD origin/develop); git diff --unified=0
  "${BASE_BRANCH}...HEAD" -- scripts docs/release ':!**/__tests__/**' ':!**/*.test.*' | grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → no matches).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff, no
  `STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack` tokens).
- **Acceptance criterion:** `decisions.md §18` — a per-record review of `spell`'s share of the
  1,344 uncertain T9 units, resolving what the drafted blacklist cannot, before any further
  sign-off. Read-only; does not itself close any T9 unit or change card 11's status.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
  `PCGEN_ORACLE_SHA`), fetched fresh this cycle to the repo-local slot (empty on this fresh
  worktree, self-healed via `scripts/fetch-pcgen-oracle.sh --dest <repo-local pcgen slot>`; `OK`
  after fetch).
- **Status:** review/evidence-only cycle, per `decisions.md §18` ("The review **proposes**; it
  does not amend the blacklist and does not transcribe anything"). Card 11's row stays
  `in-progress`; T9's onboarding stays paused.

- **Notes:**

  **Footgun 1 fired on this worktree** — cut from a stray `site-publish` merge with no `docs/`,
  `data/`, `scripts/`. Reset to the pinned SHA and rebased onto `origin/tranche/12` before any
  other work, per the dispatch brief's own instruction.

  **Step 1 — re-derived the population.** `spell` kind: total=732, blocked=31, clear=349,
  uncertain=352 — byte-identical to `t9-pi-exposure-audit.md §3`'s spell row. **No correction filed
  against the audit's own spell figures.** One correction filed against the audit's **T9-wide
  total**: it no longer re-derives (2,712 → 3,573 on this cycle's pin), entirely from
  `monster_ability` (517 → 1,378), a later unrelated commit outside this lane's kind and scope.
  `docs/retro/events/spell.jsonl`, `--verified-by "python3 scripts/sd32_t9_census.py
  fresh_inventory.json"`.

  **Step 2 — per-record review of all 352 `spell` `uncertain` units.** Extracted each record's
  `DESC:` free text, hand-triaged every capitalized word not matching ~180 ordinary
  D&D/Pathfinder mechanical-vocabulary terms (established by reading every flagged word's context
  across the full 352 in an iterative pass, not guessed up front) or a roman numeral. Result:
  **350 clear, 2 still_undecidable (`inner_sea_races:Bleaching Resistance`,
  `monster_codex:Gift of the Deep` — both leaning PI on this reviewer's read, neither matching an
  existing blacklist rule), 0 blocked.** Full reasoning and both records read in full:
  `t9-pi-review-spell.md §2`.

  **Step 3 — clear-bucket recheck, normalized scan.** Ran a case-folded, word-boundary-matched,
  bounded-OCR-confusion (`l`/`1`→`i`, `rn`→`m`) scan over all 349 `clear` and all 352 `uncertain`
  `spell` rows. **`newly_blocked = 0`, `newly_uncertain = 0`.** Two false-positive traps hit and
  fixed while building the scan, documented so a future cycle doesn't rediscover them: (a) naive
  case-folding alone reopens a hole the case-sensitive original scan closed (`Nex` collides with
  ordinary `next` once folded — fixed with word-boundary matching); (b) `|` must not be folded
  into the OCR-confusion set — it is PCGen's own field delimiter, and folding it produced a false
  NEGATIVE on the recorded `Cayden CaiLean` incident itself. Verified post-fix the scan still
  catches both recorded incidents (`Cayden CaiLean`, `lrori`). Full detail: `t9-pi-review-spell.md
  §3`.

  **Step 4 — proposed `§2.3` addition for `spell`** (not applied): the existing
  `SpellCacheData.description` entry's guidance is right in shape but under-specified on what a
  reviewer should look for beyond the term list — named two concrete shapes found this cycle
  (named setting phenomena; named creature-variant labels inside bracketed spell options).
  `t9-pi-review-spell.md §4`.

  **Step 5 — `.MOD`/`.COPY` question, spell kind.** 0 of 732 spell units are `.MOD`/`.COPY`-shaped
  — no cross-reference-inheritance rule needed for this kind. `t9-pi-review-spell.md §5`.

  **Step 6 — spot-check table, 10 records** (2 still_undecidable, 7 clear, 1 already-blocked for
  contrast), `t9-pi-review-spell.md §6`.

  **`df -h /` at end of cycle:** 665G available, 32% used (968G filesystem).

- **Discovery forwards:** the T9-wide population drift (`monster_ability` 517→1,378, filed as a
  correction this cycle) affects whichever lane owns `monster_ability`'s own uncertain-bucket
  re-derivation — its denominator has moved since the audit's base and needs its own re-check, not
  assumed unchanged the way `spell`'s was confirmed to be.
- **Next-cycle plan (named, not attempted this cycle):** the two `still_undecidable` spell records
  and this lane's proposed `§2.3` addition feed the operator's next ruling on
  `docs/governance/ogl-pi-blacklist.md`; the remaining kinds' per-record reviews (`companion`,
  `feat`, `monster_ability`, `equipment`) are separate lanes' scope, not this one's.
