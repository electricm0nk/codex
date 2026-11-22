# Cycle t9-race-trait-remediation-mode — Gate 3 (closure invariant) / Card 11 (T9)

- **Card ID:** 11 (`epic-2-cause-closure`, T9 sub-lane `t9-race-trait-remediation-mode`, follow-on
  to `t9-generic-ingest-remediation-mode_cycle-1_cycle_receipt.md`, commit `067ae9cfe2`)
- **Commit SHA:** (this cycle's commit — see push output)
- **Files touched:**
  - `scripts/ingest_race_trait_generic.py` — added `--remediate` mode:
    `find_owned_race_trait_files` (ownership scoped to the ABSENCE of the
    `codex_generated_name` key — see "Ownership scoping" below) and
    `remediate` (re-derives every self-owned `race_trait_generic` record
    from its own pinned-oracle citation and re-applies the current
    redaction pipeline in place, including a raw_tokens-wide
    `blacklist_term_hit_including_concatenated` scan this script's ordinary
    writer never had). `main()` branches to `--remediate` before the
    `--ledger` requirement, mirroring `ingest_generic_kind.py`'s shape.
  - `scripts/tests/test_race_trait_remediation.py` (new) — ownership
    predicate soundness test (corpus-wide, all owned files verified to
    lack `codex_generated_name`), a scoped `--dry-run` sweep over every
    owned file OUTSIDE `bestiary_4` (territory), and two mutation-proof
    tests: one in-memory-only (mirrors `test_generic_ingest_remediation.py`
    exactly), one that runs `remediate` itself end-to-end against a TEMP
    COPY of a real record dirtied with a reintroduced leak (real corpus
    file never opened for write).
  - `data/corpus/**` — **untouched.** `git status --porcelain data/corpus`
    is empty (0 M / 0 A / 0 D) both before and after this cycle.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to my own
  working-tree diff, `git diff --unified=0 HEAD -- scripts/
  ingest_race_trait_generic.py scripts/tests/test_race_trait_remediation.py`,
  filtered to `+`/`-` content lines only — the raw `--unified=0` output
  still carries git's own hunk-header function-context line, which is not
  an added/removed line and was excluded before the grep).
- **Wired-integration audit result:** `OK_NO_TOKENS` on the same scoped diff.
- **Acceptance criterion:** dispatch brief — close the same
  `no_record`-ledger-gated structural defect in
  `scripts/ingest_race_trait_generic.py` that
  `t9-generic-ingest-remediation-mode` closed in `ingest_generic_kind.py`;
  find a sound ownership predicate for this script's own population (which,
  unlike the sibling script's, carries no `codex_generated_name` key at
  all); prove the path with a mutation test, not an invented leak; verify
  `no_record` unmoved.
- **Corpus SHA:** PCGen oracle pin `7f818006e371188e5717fd18d74d18a420747fc6`
  (bootstrapped fresh this cycle at the repo-local artifacts slot — the
  worktree's oracle slot was empty; never referenced `~/workspace/repos/pcgen`).
- **Status:** complete
- **Notes:**

  ## Re-derivation per `§17a` — the brief's "47" was scoped, not corpus-wide

  The dispatch brief's "47-record population" is the sibling receipt's own
  figure for ONE directory, `data/corpus/inner_sea_races/race_trait_generic/`
  (48 files there, 47 lacking `codex_generated_name`) — confirmed exactly:
  `python3 scripts/ingest_race_trait_generic.py --remediate --book
  inner_sea_races --dry-run` → `"scanned": 47, "changed": 0`.

  The SCRIPT's real, full, corpus-wide population is much larger:
  `python3 scripts/pi_key_rawtokens_audit.py --kind race_trait_generic`
  scans **1,884** files across every `race_trait_generic/` directory
  corpus-wide; **6** carry `codex_generated_name` (all
  `ingest_generic_kind.py`'s own `--kind race_trait` output — see below);
  the other **1,878** are this script's own. **`confirmed_records=0`**
  corpus-wide — zero leaks confirmed, matching the brief's "this is
  preventive" framing, now re-derived rather than assumed.

  A `--remediate --book <book> --dry-run` sweep over every one of those 26
  `race_trait_generic/`-bearing books EXCEPT `bestiary_4` (forbidden
  territory this cycle) scanned **1,763** owned files, **0** changed,
  **0** unresolved, **0** `name_pi_newly_detected` — independently confirms
  the audit tool's finding via the remediation pipeline itself, not just
  the read-only scanner. (`1,878 - 115 [bestiary_4] = 1,763`, matches.)

  ## The structural gap, closed the same way

  `ingest_race_trait_generic.py`'s ordinary writer (`load_no_record_ids` ->
  `load_units`) is gated on `join_status == "no_record"`, identically to
  `ingest_generic_kind.py` before its own fix. `--remediate` bypasses the
  ledger entirely, walks the SELF-OWNED records already on disk, re-reads
  each one's own pinned-oracle citation (`source.path` + `source.line`),
  and re-runs the current `declared_pi`/`normalized_term_hit` checks PLUS a
  new raw_tokens-wide `blacklist_term_hit_including_concatenated` scan
  (imported from `scripts/pi_scrub.py`, never re-defined — `decisions.md
  §17`) that this script's ORDINARY writer never had (it only ever scanned
  `name`/`key` and the row's free-text `DESC`, not every individual
  `raw_tokens` value — the exact gap `ingest_generic_kind.py`'s own
  `--remediate` closed for its kinds, in the same window, same file). A
  record is rewritten only if content (everything but `ingested_at`)
  changed.

  ## Ownership scoping — the hard part, done the OPPOSITE way from the sibling

  `ingest_generic_kind.py`'s remediate scopes ownership by the PRESENCE of
  `codex_generated_name` (a key that script stamps on every record). This
  script's own ordinary writer stamps NO such key on anything it writes —
  it has no rename mechanism at all; a name-PI unit is skipped outright,
  never ingested. So the correct predicate here is the reverse: ABSENCE of
  `codex_generated_name` identifies THIS script's own records specifically
  because the ONLY other writer sharing `race_trait_generic/` directories
  (`ingest_generic_kind.py`, invoked with `--kind race_trait` at least
  once, for the ONE unit this script itself skipped as name-PI at
  `inner_sea_races/race_trait_generic/codex_named_unit_race_trait_
  inner_sea_races_isr_abilities_race_lst_67.json`) always stamps that key.

  **Verified sound, not merely assumed** (`decisions.md §17a`): every one
  of the 1,878 files identified as "owned" by the absence check was loaded
  and its top-level, `data`, and `source` key sets compared field-by-field
  against this script's own exact write schema (`population`,
  `completeness`, `ingested_at`, `data{key,name,description,raw_tokens}`,
  `source{kind,path,sha256,line,record_key}`, `wiring_class`,
  `wiring_class_signals`, `license`, `pi_field`, `pi_marker`) — **zero
  mismatches**. No third writer's records are silently included.
  `OwnershipPredicateSoundTest` (new test file) re-runs this check live.

  Same shared-directory shape the sibling receipt named: `data/corpus/
  inner_sea_races/race_trait_generic/` holds 48 files, 47 owned by this
  script, 1 by the sibling — verified with `--book inner_sea_races
  --dry-run` scanning exactly 47, not 48.

  ## No invented leak — the brief's own constraint honoured

  Zero leaks were confirmed anywhere in this population before this cycle
  and none were fabricated to demonstrate the path. The proof is entirely
  mutation-based:
  1. `test_check_goes_red_when_a_leak_is_reintroduced` — mirrors
     `test_generic_ingest_remediation.py`'s own shape: reintroduce a
     blacklisted term into an IN-MEMORY copy of a real, currently-clean
     owned record (`inner_sea_races/race_trait_generic/
     android_repairing_nanites.json`), assert the shared assertion helper
     goes RED, confirm the on-disk record is untouched.
  2. `test_remediate_itself_rewrites_a_genuinely_dirtied_in_memory_record` —
     exercises `remediate` end-to-end (not just the assertion helper): a
     TEMP COPY of the same real record is dirtied with the same
     reintroduced leak, `find_owned_race_trait_files` is monkeypatched to
     return only that temp path, `remediate(dry_run=False)` runs for real
     against it, and the temp file on disk is confirmed rewritten clean
     (`report["changed"] == 1`, `_assert_record_carries_no_blacklist_hit`
     passes on the reloaded temp file). The real corpus file is never
     opened for write; reloaded and compared byte-identical to the
     original at the end of the test. Live output:
     `{"scanned": 1, "changed": 1, "unchanged": 0, "unresolved": [],
     "changed_paths": [".../android_repairing_nanites.json"],
     "name_pi_newly_detected": []}` (temp path elided).

  Both tests pass; 5/5 in `test_race_trait_remediation.py`.

  ## `no_record` — unaffected (0 M / 0 A / 0 D, not just 0 A / 0 D)

  `git status --porcelain data/corpus` is EMPTY before and after this
  cycle — no corpus file was written by any live (non-dry-run,
  non-monkeypatched) invocation, because zero confirmed leaks exist in the
  reachable (non-`bestiary_4`) population. `no_record` cannot have moved:
  no `(book, source_file, source_line)` coordinate was touched at all.

  ## Territory — `bestiary_4` never invoked live

  `find_owned_race_trait_files` correctly identifies 115 owned files under
  `bestiary_4/race_trait_generic/` (verified, `--book bestiary_4` in a
  read-only test) — the predicate is not blind there. No non-dry-run
  `--remediate` call in this cycle used `--book bestiary_4` or omitted
  `--book` entirely; every live demonstration was scoped per-book,
  explicitly excluding `bestiary_4`, per the dispatch brief's territory
  rule (a `monster_ability` lane is live on that book).

  ## No rename mechanism — a real, named limit, not silently papered over

  Unlike `ingest_generic_kind.py`, this script's writer has no `§24`
  neutral-name rename path — a name-PI unit is skipped at ingest, never
  written. If `remediate` finds a previously-shipped, currently-clean
  record whose name/key NOW hits the blacklist (a term added since the
  original ingest — not observed this cycle, `name_pi_newly_detected` was
  empty on every run), it does not invent an unapproved rename scheme for
  it and does not delete the record (deletion would move `no_record`).
  It is reported by coordinate under `name_pi_newly_detected` for an
  operator ruling instead — `decisions.md §15`'s stop-on-that-record
  discipline applied to a hypothetical this cycle never actually hit.

  ## Verification

  ```
  python3 -m unittest scripts.tests.test_ingest_race_trait_generic -v      # 8/8 pass, unchanged
  python3 -m unittest scripts.tests.test_race_trait_remediation -v         # 5/5 pass (new)
  python3 -m unittest scripts.tests.test_generic_ingest_remediation -v     # 3/3 pass, unchanged
  python3 -m unittest scripts.tests.test_ingest_generic_kind -v            # 13/13 pass, unchanged
  python3 scripts/pi_key_rawtokens_audit.py --kind race_trait_generic      # confirmed_records: 0 (scanned 1884)
  python3 scripts/ingest_race_trait_generic.py --remediate --book <B> --dry-run   # 0 changed, all 25 non-bestiary_4 books
  git status --porcelain data/corpus                                       # empty
  ```

  ## Own-diff PI scrub

  `git diff --unified=0 HEAD -- scripts/ingest_race_trait_generic.py
  scripts/tests/test_race_trait_remediation.py`, filtered to `+`/`-`
  content lines, scanned with `normalized_term_hit` OR
  `blacklist_term_hit_including_concatenated` over 452 added lines: **2**
  hits, both the SAME line (`tok["value"] = "Al" + "dori Dueling
  Disciple"`, the deliberate split-string mutation idiom this file copies
  verbatim from `test_generic_ingest_remediation.py`'s own precedent).
  `normalized_term_hit` ALONE returns `None` on that exact line — the hit
  only fires from `blacklist_term_hit_including_concatenated`'s
  alphanumeric-normalized (all-non-alnum-stripped) check, which is designed
  to scan DATA VALUES, not Python source text; stripping the quotes/`+`/
  spaces from the source line collapses the two adjacent string literals
  into one contiguous "Aldori..." run that the identical, already-merged
  sibling test line also collapses into. Confirmed a known false positive
  from applying a data-scrub tool to source code, not a real leak — the
  actual PI term is never a contiguous literal substring anywhere in
  either file's own source, which is the property the split-string idiom
  exists to guarantee. Re-grepped after this receipt/progress.md/kanban.md
  were written: same result, same explanation, `CLEAN` by the applicable
  standard.

  ## Gate 3's budget constants — untouched.
- **Discovery forwards:**
  - `bestiary_4/race_trait_generic`'s 115 self-owned records are correctly
    identifiable by this cycle's predicate but were never scanned live
    (dry-run or real) this cycle — territory. A future cycle with write
    scope there can run `--remediate --book bestiary_4 --dry-run` first.
  - The `name_pi_newly_detected` path in `remediate` is implemented and
    covered structurally (never triggers false-negative silently — it is
    the ONLY branch that skips a record without redacting it), but has no
    live positive example yet; if the blacklist ever grows to catch a
    previously-clean shipped `race_trait_generic` name, that report field
    is where it will surface, for an operator ruling on how this
    rename-less script should handle it (extend it with a `§24`-style
    scheme, or route those units through `ingest_generic_kind.py --kind
    race_trait` instead, mirroring how the one existing `codex_generated_name`
    file in this shared directory came to exist).
- **Next-cycle plan:** none opened by this cycle; the structural gap this
  card named is now closed for both `ingest_generic_kind.py` and
  `ingest_race_trait_generic.py`, the two writers `race_trait_generic/`
  is shared between.
