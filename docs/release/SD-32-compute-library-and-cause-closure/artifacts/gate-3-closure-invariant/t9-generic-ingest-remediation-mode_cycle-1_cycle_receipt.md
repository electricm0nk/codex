# Cycle t9-generic-ingest-remediation-mode — Gate 3 (closure invariant) / Card 11 (T9)

- **Card ID:** 11 (`epic-2-cause-closure`, T9 sub-lane `t9-generic-ingest-remediation-mode`)
- **Commit SHA:** (this cycle's commit — see push output)
- **Files touched:**
  - `scripts/ingest_generic_kind.py` — added `--remediate` mode:
    `find_owned_generic_files` (ownership scoped to the `codex_generated_name`
    key being present — the structural marker only this script stamps),
    `load_inventory_coordinate_index` (reconstructs a renamed record's
    original name/key from its own `rename.coordinate`, used ONLY to build
    the redaction needle set, never written into the record), and
    `remediate` (re-derives from the pinned-oracle citation the record
    already carries and re-applies the current scrub pipeline in place).
    `main()` branches to `--remediate` before the `--ledger` requirement, so
    remediation never needs (and never consults) `no_record` status.
  - `scripts/tests/test_generic_ingest_remediation.py` (new) — corpus-content
    regression test over the 10 remediated records, plus a mutation-proof
    test that reintroduces a leak into an in-memory copy and asserts the
    check goes RED.
  - `data/corpus/adventurers_guide/feat_generic/*.json` (18 files) — 7
    real blacklist-term leaks (the named defect — three of the four §19a/
    §19c per-book-override terms hit here, by index/coordinate not spelled
    out) plus 11 already-renamed (`codex_named_unit_*`) records whose
    raw_tokens still carried an un-redacted `DESC`/other-token copy — all
    re-derived and rewritten in place by `--remediate`.
  - `data/corpus/inner_sea_bestiary/monster_generic/{chemnosit,volnagur}.json`
    — the 2 named `monster_generic` leaks, fixed the same way.
  - `data/corpus/inner_sea_races/race_trait_generic/codex_named_unit_race_trait_inner_sea_races_isr_abilities_race_lst_67.json`
    — same defect shape, found by this cycle's own re-derivation of
    `declared_pi_shipping_audit`, not named in the dispatch brief's original
    9; fixed by the same `--remediate` run (its own directory is a shared
    one with `scripts/ingest_race_trait_generic.py`'s 47 other, differently-
    schemaed records — none of those 47 were touched; see "Ownership
    scoping" below).
  - `data/corpus/{inner_sea_gods,inner_sea_magic}/ability/*.json` (10 files)
    — fixed by re-running `scripts/ingest_ability.py` (unchanged; already
    `no_record`-ungated and self-remediating by design — see "What did NOT
    need a code change" below).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to my own working-
  tree diff of `scripts/ingest_generic_kind.py` +
  `scripts/tests/test_generic_ingest_remediation.py`, via `git diff HEAD --
  <paths>` — the `BASE_BRANCH...HEAD` form flags the WHOLE file as new
  because it postdates `origin/develop`'s merge-base, which is not a
  per-cycle signal per `workflow-instruction.md §6`).
- **Wired-integration audit result:** `OK_NO_TOKENS` on the same scoped diff.
- **Acceptance criterion:** dispatch brief — (1) close the structural gap
  (`ingest_generic_kind.py`'s writer is `no_record`-ledger-gated and cannot
  re-touch an already-shipped record); (2) remediate the confirmed leaks
  through the guarded path; (3) a mutation-proved test; (4) report the audit
  clean or name exactly what remains.
- **Corpus SHA:** PCGen oracle pin `7f818006e371188e5717fd18d74d18a420747fc6`
  (bootstrapped fresh this cycle — worktree's oracle slot was empty).
- **Status:** complete
- **Notes:**

  ## Re-derivation per `§17a` — the brief's two figures had both moved

  **Figure 1 (blacklist term-hit audit).** `python3 scripts/pi_key_rawtokens_audit.py`
  fresh: `scanned=26553 confirmed_records=10` (up from the brief's stated 9 —
  the same 9 named leaks plus the already-documented `bard_s_escape.json`
  false positive still counted in `confirmed`, not the 4-record count the
  original report used). `by_kind`: `feat_generic` 7, `monster_generic` 2,
  `spell` 1 — matching the brief's 9 real + 1 false-positive exactly once
  the false positive is separated out.

  **Figure 2 (`declared_pi_shipping_audit`).** The brief's "82 pre-existing
  `DESC-PI-SHIPPED-IN-RAW-TOKENS` violations in `ability`/`feat_generic`/
  `race_trait_generic`" does **not** hold as stated. Re-run fresh:
  `82` total violations split into **two distinct violation shapes**, not
  one:
  - `DESC-PI-SHIPPED` (a metadata-consistency check: `data.description` is
    the redaction marker but `license`/`pi_field` were never updated to
    say so) — **65**, all in `data/corpus/bestiary_4/monster_ability/**` —
    a **sibling-owned kind** (`monster_ability`), untouched per the
    dispatch brief's own scope boundary, named here for that lane.
  - `DESC-PI-SHIPPED-IN-RAW-TOKENS` (the shape actually named in the
    brief — `data.description` correctly redacted but `data.raw_tokens`'
    own `DESC` copy still carries the real prose) — **17**, not 82,
    entirely within my scope: `feat_generic` 6 (`adventurers_guide`),
    `ability` 10 (`inner_sea_gods` 1, `inner_sea_magic` 9),
    `race_trait_generic` 1 (`inner_sea_races`).

  Both figures are re-derived, commanded, and named by population per
  `§12c` — the brief's headline "82" was approximately right as a raw total
  but wrong about which kind/violation-shape carried it; do not quote "82
  in ability/feat_generic/race_trait_generic" as this cycle's own finding.

  ## The structural gap, closed

  `ingest_generic_kind.py`'s ordinary writer (`load_no_record_ids` ->
  `load_units`) is gated on `join_status == "no_record"`. Once a unit is
  ingested it leaves that set permanently, so the ordinary writer can never
  re-touch a record it already shipped — the exact wall the
  `pi-key-rawtokens-followup` cycle named and could not clear for its 9
  confirmed leaks.

  `--remediate` bypasses the ledger entirely: it walks the SELF-OWNED
  records already on disk (`find_owned_generic_files`), re-reads each
  one's own pinned-oracle citation (`source.path` + `source.line` — the
  same coordinate the ORIGINAL ingest used, never re-resolved by name), and
  re-runs the CURRENT `declared_pi`/`blacklist_term_hit_including_concatenated`/
  `scrub_name_pi_tokens` pipeline over it, byte-for-byte the same logic the
  ordinary writer uses for a fresh record. A record is rewritten only if its
  content (everything except `ingested_at`) actually changed. It can also
  newly rename a record whose name was previously clean but now hits an
  since-added blacklist term (the `newly_renamed` path) — not exercised by
  any of this cycle's 10 real fixes, but implemented and covered by the
  existing `ScrubNamePiTokensTests`/`SlugifyTests` unit coverage
  (`scripts/tests/test_ingest_generic_kind.py`, unchanged, still green)
  since it reuses the identical `neutral_name`/`slugify` calls the fresh-
  ingest path already uses.

  ## Ownership scoping — never a blanket rewrite

  `find_owned_generic_files` only returns a file that carries the
  `codex_generated_name` key. `data/corpus/inner_sea_races/race_trait_generic/`
  holds 48 files; only 1 carries that key (the one this cycle fixed) — the
  other 47 were written by the SEPARATE, older `scripts/ingest_race_trait_generic.py`
  (a genuinely different schema — no `codex_generated_name`/`rename` fields
  at all), sharing the same physical directory. **This is one of the two
  "dormant shared-directory pairs" the dispatch brief names as a standing
  hazard.** Verified live: `git diff --stat data/corpus/inner_sea_races/`
  after this cycle's `--remediate` run shows exactly 1 file changed, not 48.

  ## What did NOT need a code change

  `scripts/ingest_ability.py::load_units` iterates every `kind == "ability"`
  unit in `docs/work-inventory.json` unconditionally — it was never
  `no_record`-gated, unlike `ingest_generic_kind.py`/
  `ingest_race_trait_generic.py`. `records_equal_ignoring_timestamp` already
  makes every run self-remediating: re-running it re-derives every ability
  record from the oracle and only rewrites a file whose content actually
  changed. Simply re-running it (`python3 scripts/ingest_ability.py`)
  closed all 10 of its `DESC-PI-SHIPPED-IN-RAW-TOKENS` instances —
  `"changed": 10, "unchanged": 4814"` in its own report. No remediation
  mode needed there; named so a future cycle does not duplicate this
  discovery.

  `scripts/ingest_race_trait_generic.py` (the OTHER writer sharing
  `race_trait_generic/`) is **also** `no_record`-ledger-gated, the SAME
  structural defect class as `ingest_generic_kind.py` pre-fix — but it
  currently has **zero** confirmed leaks of its own (all leaks found this
  cycle were in `ingest_generic_kind.py`-owned records). Not modified this
  cycle (`§17a`: land what is proven necessary, name what is not); flagged
  as a standing risk for the next cycle that finds a leak inside its own
  47-record population, since today it would hit the identical wall.

  ## Verification

  ```
  python3 scripts/pi_key_rawtokens_audit.py            # confirmed_records: 10 -> 1 (false positive only)
  cargo run --locked --bin declared_pi_shipping_audit   # DESC-PI-SHIPPED-IN-RAW-TOKENS: 17 -> 0 (mine); 65 DESC-PI-SHIPPED remain, all monster_ability (sibling-owned, untouched)
  python3 -m unittest scripts.tests.test_generic_ingest_remediation -v          # 3/3 pass
  python3 -m unittest scripts.tests.test_ingest_generic_kind -v                 # 13/13 pass, unchanged
  python3 -m unittest scripts.tests.test_ingest_race_trait_generic -v           # 8/8 pass, unchanged
  python3 -m unittest scripts.tests.test_pi_key_rawtokens_defect1_regen -v      # 4/4 pass, unchanged
  python3 -m unittest scripts.tests.test_declared_pi_shipping_defect2_regen -v  # 2/2 pass, unchanged (needs CARGO_TARGET_DIR)
  ```

  **RED → GREEN, live against real content, not just the new test's own
  `assertRaises`:** `git show HEAD:data/corpus/adventurers_guide/feat_generic/duelist_of_the_roaring_falls.json`
  copied over the fixed file reproduces the exact pre-fix leak
  (`PREABILITY`/`BENEFIT` carrying the real, unredacted text) —
  `RemediatedRecordsNoLongerLeakTest` goes RED against it (an `AssertionError`
  naming the same term this record's own blacklist hit already names, not
  repeated here), confirming the check can fail. Restored the fixed file;
  re-ran green.

  ## `no_record` — unaffected

  All 31 changed files were REWRITES of already-existing paths (`git status
  --porcelain data/corpus` shows 31 `M`, 0 `A`, 0 `D`). Every record kept its
  `(book, source_file, source_line)` coordinate — `shape_ledger.py`'s join
  key, not the filename — so no unit moved kind, was created, or was
  deleted. **`no_record` is unmoved by this cycle.**

  ## Regeneration safety

  `data/corpus/**` only, via `ingest_generic_kind.py --remediate` (in-place
  rewrite, same file paths) and `ingest_ability.py` (its own existing
  `records_equal_ignoring_timestamp`-gated writer, unchanged code). No
  `--allow-stamp-loss` flag exists on either. `CORPUS_LITERAL_SWEEP_REPORT`/
  `DERIVED_FIXTURE_CHECK_REPORT` are consumed only by `v06_work_inventory`,
  never run this cycle. Diffed `git status --porcelain data/corpus`
  before/after: 0 → 31 `M`, 0 `A`, 0 `D` — every changed file's content
  inspected via `git diff`, not assumed.

  ## Own-diff PI scrub

  Grepped every added line in `scripts/ingest_generic_kind.py` and
  `scripts/tests/test_generic_ingest_remediation.py` (the working-tree diff,
  `+`-added lines only) against `normalized_term_hit` before committing:
  `CLEAN`. The new test's mutation case builds its reintroduced-leak string
  via runtime concatenation specifically so no blacklist term is ever a
  contiguous literal substring anywhere in this test file's own source.
  This receipt, `progress.md`'s new entry, and `kanban.md`'s prepended note
  were grepped the same way before this commit — found and fixed 3 hits
  in this receipt's own first draft (blacklist terms spelled out in prose
  describing which terms hit which records), rewritten to reference them
  by index/coordinate instead; re-grepped after the fix: `CLEAN`.

  ## Gate 3's budget constants — untouched.
- **Discovery forwards:**
  - `scripts/ingest_race_trait_generic.py` carries the SAME structural
    defect class (`no_record`-ledger-gated writer, cannot re-touch a shipped
    record) as `ingest_generic_kind.py` had before this cycle, for its own
    47-record, non-`codex_generated_name` population. Zero confirmed leaks
    found in that population this cycle, so not fixed — named so the next
    cycle that finds one there does not have to re-discover the wall.
  - The 65 `DESC-PI-SHIPPED` (metadata-consistency, not raw-tokens) violations
    in `bestiary_4/monster_ability/**` are real, confirmed, sibling-owned,
    and out of this cycle's scope — named for that lane, not fixed here.
  - `spell/bard_s_escape.json`'s `normalized_term_hit` OCR-fold
    (`rn`→`m`) false positive is unchanged from the prior cycle's finding —
    re-confirmed still present, still not a real leak, still not touched
    (the fold itself is `decisions.md §19a`'s own approved, operator-signed
    scheme; an operator ruling on narrowing it is still pending, per the
    prior receipt).
- **Next-cycle plan:** `bestiary_4/monster_ability`'s 65 `DESC-PI-SHIPPED`
  violations (sibling lane); `ingest_race_trait_generic.py`'s own
  no_record-gating defect, if that lane's population ever leaks; an
  operator ruling on the `normalized_term_hit` OCR-fold false-positive risk
  (still outstanding, not this cycle's to resolve).
