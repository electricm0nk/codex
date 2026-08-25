# Cycle t9-onboarding (DISCOVERED-queue drain, 2) — `progress.md` `## DISCOVERED`

- **Card ID:** `epic-2-cause-closure` (row 11; docs-only reconciliation, no code/corpus write). Rows
  11 and 15 left `in-progress`, per the dispatch brief's own constraint.
- **Actor:** `t9-onboarding`
- **Base:** `bd6e0b696856e08890d518d81f1f2c9a61ed32f2` (`PIN`, == `origin/tranche/12` HEAD at cycle
  start). Rebased mid-cycle onto `origin/tranche/12` after 5 sibling commits landed
  (`3f8ddca7fd`, `6d7fd2e081`, `5c0178a397`, `6eecd50e58`, `239f9d925b`) — no conflicts.
- **Files touched:** `docs/release/SD-32-compute-library-and-cause-closure/progress.md`,
  `docs/release/SD-32-compute-library-and-cause-closure/acceptance-and-verification.md`,
  `docs/retro/events/t9-onboarding.jsonl` (1 new correction event, plus one self-correction to
  redact a PI term written into the same log by this cycle's own first draft — caught by this
  cycle's own pre-push PI grep, fixed before push, never landed on `origin`).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff — working-tree changes plus this
  cycle's one committed docs commit, `e0e2a75df4`, scoped per §6 step 2's own instruction to check
  the cycle's own diff, not the tens-of-thousands-of-line `BASE_BRANCH...HEAD` form).
- **Wired-integration audit result:** `OK_NO_TOKENS` — the literal string `todo` matched twice, both
  inside the pre-existing directory-path reference `docs/release/SD-31-corpus-closure-grind/todo/`,
  not the forbidden-token pattern; reviewed by hand, not a violation.
- **PI audit of own diff (`decisions.md §15`/`§24b`-2):** grepped every added line against the full
  60-term blacklist (`src/rules_core/pi_screening.rs::PI_BLACKLIST_TERMS`) before commit. One
  self-inflicted leak found and fixed pre-push: the first draft of the retro correction event
  (below) named two `decisions.md §19a`-3d term-list-addition terms directly; rewritten to cite
  `ogl-pi-blacklist.md §2.3c` instead of reproducing them, matching the convention the file's own
  pre-existing entries already use. Two case-insensitive substring hits on the term `Nex` were
  checked by hand and are both instances of the ordinary English word "next" (the exact false-
  positive shape `decisions.md §19a`-3b names) — not the term.
- **Acceptance criterion:** drain `progress.md`'s `## DISCOVERED` queue per the dispatch brief:
  for each open entry, CLOSE (cited), CLOSE (do the work), ESCALATE (rewritten, self-contained), or
  KEEP (figures re-derived).
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (oracle bootstrapped fresh into this
  worktree's empty repo-local slot via `scripts/fetch-pcgen-oracle.sh`, confirmed PASS via
  `scripts/verify.sh --only preflight-oracle`).
- **`no_record` before/after (this cycle made no corpus/code writes):**
  `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json`:
  - At this cycle's own base (`bd6e0b6968`): **982**.
  - After rebasing onto 5 sibling commits landed mid-cycle (`3f8ddca7fd` spell, `6d7fd2e081`
    monster_ability, `5c0178a397` PI leaks, `239f9d925b` spell/equipment wave 2): **788**.
  - The 194-unit drop is entirely sibling-lane work; this cycle's own diff is docs-only and moved
    `no_record` by **0**.

## §17a — the brief's own "9 top-level entries" figure was wrong; re-derived to 10

Re-ran the same `awk` the prior reconciliation cycle (`t9-onboarding-blocker-reconciliation`) used
to find its "9": `awk '/^## DISCOVERED/{f=1} f && /^## / && !/^## DISCOVERED/{exit} f && /^- 20/{c++}
END{print c}' progress.md` → **10**, not 9, both before and after this cycle's rebase (no entry was
added or removed by the sibling commits that landed mid-cycle — all 5 touched other sections of the
file). The prior cycle's own count was off by one; not re-litigated further, corrected here per
`decisions.md §17a` ("re-derive every figure you are handed"). Still well under the
`workflow-instruction.md §8` >10 non-self-healable ceiling either way.

## The 10 entries, dispositioned

Numbered by their line position in the section, oldest-dated first as the file orders them.

1. **T2b classifier: `core_rulebook`/`advanced_players_guide` 51-unit plumbing-row misclassification
   (`epic-2-t2b-w1-c/1`).** **KEEP.** Two later classifier-fix cycles landed since this was filed
   (`t2b-refine-kind-fix`, `epic-2-t2b-cluster4-classfeature-fix`) and neither closes it — the
   second explicitly, by its own regression test, keeps this exact row shape (a bare-class-name-KEY
   Favored-Class-Bonus row) untouched by design. Confirmed still open by reading both landed
   cycles' own text, not by re-running the original lane's custom classification script (out of a
   docs-reconciliation cycle's reach). Count not re-sized (51, unverified fresh this cycle).
2. **"Adopted Race" spans 4 books, 1 mechanism (`epic-2-t2b-w1-c/1`).** **SUPERSEDED**, cited in
   place — the document's own later `t2b-adoptive-parentage/1` correction (entry 8) already
   established this is two shapes, not one. Marked so a future reader doesn't treat it as live scope
   independent of entry 9's escalation.
3. **`bestiary_3` 819-unit classifier confirmation (`t2b-w1-d/1`).** **KEEP**, figure corrected.
   `t2b-refine-kind-fix` landed and moved `819 -> 194` (625 reclassified `race_trait ->
   monster_ability`, proven by coordinate join). Re-derived this cycle: `race_trait`'s corpus-wide
   `no_record` is 0, so the 194 residual is reachability scope, not a `no_record` gap — still real,
   not re-sized further.
4. **Ten-kind list omits `class_feature` (`gate-0-census-closure` Cycle 2).** Already marked
   `RESOLVED` inline (`decisions.md §12`) by a prior cycle. No action; correctly closed.
5. **158-book / 38,372-unit figures carry no derivation command (`gate-0-census-closure` Cycle 2).**
   **CLOSED — did the work this cycle.** Added reproducible derivation commands to
   `acceptance-and-verification.md` AT-32-G0-001/AT-32-G0-002 (commit `e0e2a75df4`) rather than
   overwriting the frozen launch-time numbers — re-ran both commands fresh: `discovered_book_dirs`
   186 (`scripts/census_independent.py`, confirms the prior reconciliation cycle's own 186 figure
   unchanged), `jq '.totals.units' docs/work-inventory.json` **49,490** (grown substantially since
   Gate 0's launch via card 15's `no_record` campaign — both numbers now stated with commands and
   dates, per `decisions.md §12c`).
6. **`gen_book_cache.rs` self-erasure (`boundary-branch-review` Cycle 1).** Already marked
   `RESOLVED` inline, same day, by `epic-5-protective-sweep` commit `3b470c56f`. No action;
   correctly closed.
7. **SD-31 `todo/` doc corrections not landed (`boundary-branch-review` Cycle 1).** **KEEP**,
   re-checked. `todo/levers.md` L3 is still `NOT STARTED — now sized`, not `DEAD`; `todo/defects.md`'s
   highest ID is now `D14` (a new finding would be `D15`). The source worktree
   (`worktree-wf_c1156061-e3f-5`) is confirmed gone — absent from `git worktree list` and from
   `git log --all --oneline` — so the exact proposed correction text is unrecoverable; noted so the
   next cycle reconstructs from current state rather than assumes a cherry-pick is possible.
8. **Correction: Adoptive Parentage is 2 shapes, not 1 (`t2b-adoptive-parentage/1`).** This entry
   *is* the correction that supersedes entry 2 above; it is itself already-landed documentation, not
   an action item. No change.
9. **14-unit `kind: trait` content-surface scope (`t2b-adoptive-parentage/1`).** **ESCALATED**,
   rewritten per the dispatch brief's outcome-3 format. Re-derived fresh rather than trusted: `find
   data/corpus -mindepth 2 -maxdepth 2 -type d -name trait` returns 0 hits corpus-wide — still no
   `kind: trait` surface exists, count confirmed still 14 (`bestiary_2` 7 / `bestiary_3` 5 /
   `bestiary_5` 1 / `bestiary_6` 1). Rewritten paragraph states today's count, the exact question
   (does this scope belong in SD-32's DoD or a successor bundle), and the cost of each answer
   (yes: a real multi-cycle new-kind epic before closure; no: requires an explicit operator ruling
   narrowing card 11's scope, not a cycle-authored forward-scope-register entry, per `decisions.md
   §10` item 2).
10. **PI leaks in `data.key`/`data.raw_tokens`, 4 outstanding (`pi-key-rawtokens-screen`
    follow-up).** **KEEP, figures substantially corrected.** A sibling lane (`5c0178a397`, landed on
    `origin/tranche/12` before this cycle's rebase) fixed 3 of the original 4
    (`domain`/`equipment`/`language`) at the cause and determined the 4th (`spell`) is a confirmed
    false positive (an OCR-fold collision with an ordinary English word in genuine OGL prose, not a
    leak). That same commit's own corpus-wide re-scan independently found the same 9 additional
    leaks this reconciliation cycle found by re-running the audit tool itself
    (`python3 scripts/pi_key_rawtokens_audit.py`, `confirmed_records=10`, 1 of which is the
    confirmed-false-positive `spell` record): 7 `feat_generic` in one book, 2 `monster_generic` in
    another — both kinds landed by sibling generic-ingest lanes after the original audit ran, both
    made detectable by `decisions.md §19a`-3d's two new blacklist terms (not reproduced here per
    `§24b`-2; see `ogl-pi-blacklist.md §2.3c`). Neither commit remediates the 9 — the generic-ingest
    writer is `no_record`-ledger-gated and cannot re-touch already-shipped records, so this needs a
    small new remediation-only path. Retro correction logged:
    `docs/retro/events/t9-onboarding.jsonl` id `1787499981214-t9-onboarding-252784`. Real open count
    is now **9** (`feat_generic`/`monster_generic`), not 4 (`domain`/`equipment`/`language`/`spell`).

## What was NOT done (explicitly out of this cycle's scope)

- No code or corpus write. No cargo build. No card 11/15 status change.
- No scope moved into `forward-scope-register.md` — entry 9's escalation stays an escalation, not a
  disposition, per `blocker-closure-doctrine.md`.
- Did not attempt to remediate the 9 real PI leaks named in entry 10 — that is corpus-write work
  requiring a new generator-side mechanism, out of a docs-reconciliation cycle's granted reach, and
  risks collision with the sibling lane that is still actively working this exact shape.
- Did not attempt to re-derive entry 1's 51-unit count fresh (would require the original lane's
  custom row-classification script) or entry 3's 194-unit residual beyond what the landed fix
  cycle's own receipt already states.

## Suites / build

Docs-only change; no Rust/Python source touched. `python3 scripts/shape_ledger.py` and `python3
scripts/pi_key_rawtokens_audit.py` were run read-only for verification only (no write); `python3
scripts/census_independent.py` was run read-only with `--output` pointed at a scratch path, deleted
after reading, not committed.

## Discovery forwards

None new. Entry 9's escalation and entry 10's corrected 9-leak figure are re-derivations of
pre-existing entries, not new discoveries.

## Next-cycle plan

- A card-11-scoped cycle can pick up entry 1 (core_rulebook/APG 51-unit plumbing-row
  misclassification) as a dedicated classifier cycle.
- A T9/PI-scoped cycle can pick up entry 10's 9 confirmed `feat_generic`/`monster_generic` leaks —
  needs a small remediation-only write path since the generic-ingest writer is ledger-gated.
- Entry 9 (14-unit `kind: trait` scope) needs an operator ruling before any further cycle acts on
  it — relay the rewritten paragraph verbatim.
- No other action required by this reconciliation cycle.
