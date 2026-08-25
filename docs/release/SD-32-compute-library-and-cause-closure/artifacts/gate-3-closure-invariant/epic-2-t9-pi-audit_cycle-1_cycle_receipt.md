# Cycle 1 — Epic 2 (T9 PI-exposure audit lane) / Card 11 `epic-2-cause-closure`

- **Card ID:** `epic-2-cause-closure`
- **Commit SHA:** (this cycle's commit — see push log)
- **Files touched:** `scripts/sd32_t9_pi_exposure_audit.py` (new), `artifacts/gate-3-closure-invariant/t9-pi-exposure-audit.md` (new), `docs/retro/events/t9-pi-audit.jsonl` (new — 1 correction), `kanban.md` (card 11 note prepended, status left `in-progress`), `progress.md` (this cycle's entry). **No corpus data changed** (`data/corpus/**` untouched); `docs/governance/ogl-pi-blacklist.md` **not amended** (status stays `DRAFT`).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  (`BASE_BRANCH=$(git merge-base HEAD origin/develop); git diff --unified=0
  "${BASE_BRANCH}...HEAD" -- scripts docs/release ':!**/__tests__/**' ':!**/*.test.*' | grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → no matches).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff, no
  `STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack` tokens).
- **Acceptance criterion:** `decisions.md §15` — "A dedicated audit lane sweeps all 2,712 T9
  units against `ogl-pi-blacklist.md` and reports the real blocked count, per kind and per book,
  naming the records." Read-only; does not itself close any T9 unit or change card 11's status.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
  `PCGEN_ORACLE_SHA`), fetched fresh this cycle to the repo-local slot (empty on this fresh
  worktree, self-healed via `scripts/fetch-pcgen-oracle.sh --dest <repo-local pcgen slot>`; `OK`
  after fetch).
- **Status:** measurement/evidence-only cycle. Card 11's row stays `in-progress`; T9's onboarding
  stays paused per `decisions.md §15` ("T9's onboarding work is paused pending this audit"). This
  cycle does not lift that pause — only the operator's ruling on the memo's §9 question can.

- **Notes:**

  **Step 1 — re-derived the population.**
  ```
  cargo build --locked --release --bin v06_work_inventory
  PCGEN_CORPUS_ROOT=<repo-local pcgen slot>/data ./target/release/v06_work_inventory --stdout-only > fresh_inventory.json
  python3 scripts/sd32_t9_census.py fresh_inventory.json
  ```
  38,391 total units (matches `decisions.md §12c`'s inventory denominator). T9-filtered: **2,712**
  (spell 732, companion 726, feat 487, monster_ability 517, equipment 222, monster 28) — identical
  to `decisions.md §13` and `card11-t9-census-census.md`. No correction filed for the population.

  **Step 2 — built and ran the classification script over the full population.**
  ```
  python3 scripts/sd32_t9_pi_exposure_audit.py fresh_inventory.json \
      --corpus-root <repo-local pcgen slot>/data --json-out t9_pi_classified.json
  ```
  Method: resolve each unit's `(source_file, source_line)` to a real oracle file (basename index
  over the whole corpus tree, all 2,712 resolved unambiguously — `resolve_note == "ok"` for every
  unit), read the whole raw tab-separated row, classify blocked (declared `NAMEISPI:YES`/
  `DESCISPI:YES`, or a `PI_BLACKLIST_TERMS` substring hit — byte-identical 57-term copy of
  `src/rules_core/pi_screening.rs`'s list) / uncertain (a `DESC:`/`BENEFIT:`/`SPECIALS:`/`SA:`
  free-text tag present, not blocked — `ogl-pi-blacklist.md §2.3`'s own "requires per-record
  judgment" category, widened past its four named structs to every T9 kind) / clear (no
  declaration, no term hit, no free-text tag — blanket OGL, `§2.2`).

  **Results:** blocked 261 (9.6%), clear 1,107 (40.8%), uncertain 1,344 (49.6%). Full per-kind and
  per-book tables, named example records (up to 8 per kind), and the two fully-clear books
  (`occult_adventures` 330/330, `bestiary_5` 2/2) are in
  `artifacts/gate-3-closure-invariant/t9-pi-exposure-audit.md` §3/§4/§6.

  **Validation against the existing sample:** the `monster` kind (28 units, the population the
  T9 lane's own cycle-1 receipt already forensically checked) reproduces exactly — 21
  blocked / 7 clear / 0 uncertain, matching `card11-t9-census-census.md §5`'s 21-PI-excluded /
  6-structural-non-defect / 1-genuine-gap split.

  **Step 3 — one correction filed**, against `decisions.md §15`'s own prose ("the 96% rate
  observed in the monster kind" vs. its own cited 21/28 = 75.0%): `docs/retro/events/t9-pi-audit.jsonl`,
  `RETRO_ACTOR=t9-pi-audit python3 scripts/retro.py correction --subject "decisions.md §15" ...
  --verified-by "python3 -c 'print(21/28*100)' -> 75.0"`. Not corrected in `decisions.md` itself —
  locked operator-pinned text, outside this audit's write scope.

  **Step 4 — blacklist gaps recorded as proposals, none applied.** `ogl-pi-blacklist.md` was not
  written to. Three gaps named in the memo §8: (1) `companion`/`monster_ability` have no §2.3
  field-classification entry (802/1,344 of `uncertain` — 59.7% — come from these two kinds); (2)
  no OCR/typo-normalization pass on the term list (a real incident already recorded in the
  blacklist's own §4 Inner Sea Gods entry); (3) `.MOD` reference-row PI inheritance unaddressed by
  a per-line-only method.

  **Why zero units are banked, and why that is the correct disposition for this lane:**
  `decisions.md §15` scoped this lane to evidence, explicitly: "The audit is read-only... Its
  deliverable is evidence, not a decision." Closing any unit — even a `clear` one — was never in
  this cycle's Definition of Done; the operator's own sign-off on the memo's §9 question is the
  next act, not this cycle's.

- **Discovery forwards:** none requiring a new card — scoped audit against the existing T9 line of
  card 11, per the dispatch brief.
- **Next-cycle plan (named, not attempted this cycle):**
  1. **Operator ruling** on the memo's §9 question — sign off the blacklist as drafted (unblocks
     only the 1,107 `clear` units) or direct a per-record review pass on the 1,344 `uncertain`
     units first.
  2. **`occult_adventures` and `bestiary_5`** (332 units combined) can be dispatched to a book
     onboarding cycle the moment the operator signs off the blacklist as a mechanism — no
     per-record risk remains in either book under this audit's findings.
  3. **A per-record review pass on the `uncertain` bucket**, starting with the highest-value books
     named in the memo §4 (`bestiary_4`'s `monster_ability` alone carries 123 uncertain units;
     `ultimate_wilderness`'s `companion` carries 221), if the operator chooses option (b) in §9.
