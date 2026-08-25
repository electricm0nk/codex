# Cycle pi-key-rawtokens-screen — Gate 3 (closure invariant) / Card 11 (T9)

- **Card ID:** 11 (`epic-2-cause-closure`, T9 sub-lane `pi-key-rawtokens-screen`)
- **Commit SHA:** (this cycle's commit — see push output)
- **Files touched:**
  - `scripts/ingest_ability.py` — new `scrub_blacklist_pi_tokens` (blacklist-only scan applied
    to every raw_token value, not only `DESC`, for records whose bare name is not PI) and
    `records_equal_ignoring_timestamp` (idempotent regen — a re-run only writes a file whose
    content genuinely changed).
  - `scripts/pi_key_rawtokens_audit.py` (new) — generic corpus-wide `data.key`/`data.raw_tokens`
    PI screen, every kind.
  - `src/rules_core/pi_screening.rs` — `PI_BLACKLIST_TERMS` 57 → 60 (added `Aldori`,
    `Magaambya`, `Magaambyan`, per the operator-signed-off `decisions.md §19a` amendment 3d,
    already in the Python-side copy but not yet ported to the Rust production copy).
  - `scripts/tests/test_ingest_ability_raw_tokens_pi_screen.py` (new)
  - `scripts/tests/test_pi_key_rawtokens_audit.py` (new)
  - `data/corpus/inner_sea_gods/ability/adept.json` — the 1 of 2 named confirmed leaks, fixed
    through the guarded generator path (`scripts/ingest_ability.py`, not hand-edited).
  - `data/corpus/inner_sea_magic/ability/diplomatic_student.json` — the 2nd named confirmed
    leak, same path.
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-3-closure-invariant/pi-key-rawtokens-corpus-report.md`
    (new) — job 2/3 deliverable, full corpus-wide report.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` on all new/modified files listed above,
  except one expected match: `from sd32_t9_pi_review_feat_equipment import ...` in the two new
  test files and `pi_key_rawtokens_audit.py` — a real, already-shipped shared module's name
  (identical import already exists, unmodified, in `scripts/ingest_ability.py` line 93), not a
  new bundle-tag leak.
- **Wired-integration audit result:** `OK_NO_TOKENS` on all new/modified files.
- **Acceptance criterion:** dispatch brief (three jobs): (1) fix the 2 operator-confirmed
  `ability` leaks (`decisions.md §19`'s 60-term list against `data.key`/`data.raw_tokens` with a
  clean `name`); (2) close the screening gap generically, corpus-wide, every kind, report real
  confirmed count per kind/book; (3) report the unratified-vocabulary candidate population for
  an operator ruling — do not act on it.
- **Corpus SHA:** PCGen oracle pin `7f818006e371188e5717fd18d74d18a420747fc6` (fetched fresh this
  cycle — worktree's oracle slot was empty; bootstrapped via `scripts/fetch-pcgen-oracle.sh`).
- **Status:** complete
- **Notes:**
  - **Job 1 — the 2 confirmed leaks, fixed.** Re-derived first (`§17a`): dry-run
    `scripts/ingest_ability.py --dry-run` reproduces `population: 4824, name_pi_renamed: 576`
    exactly, matching the brief. Real (non-dry-run) regen changed exactly 2 files
    (`"changed": 2, "unchanged": 4822"` in the generator's own report) — the fix is byte-scoped
    to the two named records, confirmed by `git status --porcelain data/corpus`. Both now pass
    `corpus_literal_sweep` (0 mismatches for either file) because `pi_screening.rs`'s
    `PI_BLACKLIST_TERMS` bump to 60 makes the sweep's own PI-redaction exemption
    (`classify_field` in `corpus_literal_sweep.rs::compare_tokens`) recognize the new
    redactions — this bump was load-bearing, not cosmetic.
  - **Job 2 — generic screen built, run corpus-wide.** `scripts/pi_key_rawtokens_audit.py`
    walks all `data/corpus/<book>/<kind>/*.json` (24,051 records scanned across 15 kinds this
    run). **`§17a` self-correction, recorded live**: the first version wrongly counted 37
    confirmed records because it treated `data.name == "[redacted PI]"` as "clean" — 26/30
    sampled were records an EARLIER screen had already correctly redacted, not new leaks. Fixed
    (`name_already_flagged`); corrected run: **4** additional confirmed leaks beyond the 2 fixed
    ones (`domain` 1, `equipment` 1, `language` 1, `spell` 1 — one each in `core_rulebook`,
    `inner_sea_gods`, `inner_sea_temples`, `advanced_players_guide`). Logged as
    `scripts/retro.py correction` `1787493549497-t9-onboarding-01846b` and superseding
    `deferral` `1787493585450-t9-onboarding-bcf0ca` (the original, wrong-figure deferral
    `1787493371990-t9-onboarding-d41331` stays in the log per its append-only contract, corrected
    by the correction event). **Named, not remediated this cycle** — each of the 4 kinds has its
    own generator this cycle did not inspect; a safe per-kind fix needs its own guarded-path
    cycle. Full table in the committed report.
  - **`declared-pi-audit` (28 violations, `language`/`template`, `NAME-PI-SHIPPED` shape) —
    confirmed pre-existing and unrelated.** Different defect (a declared-PI record's own NAME
    shipped unredacted, vs. this cycle's clean-name-hiding-a-field-leak shape). This cycle's
    diff touches none of those 28 files and no code path `declared_pi_shipping_audit` reads.
  - **Job 3 — unratified-vocabulary candidates reported, not acted on.** 23,062 of 24,051
    scanned records show a capitalized non-blacklisted token via a heuristic scan. Spot-checked
    the top terms: dominated by ordinary mechanical vocabulary (`Base`, `Weapon`, `Melee`,
    `Magic`, ...), not proper nouns — reported honestly as too noisy to be directly actionable.
    `ogl-pi-blacklist.md` untouched (stays `SIGNED-OFF` at exactly the 60 terms `§19` approved).
    The exact question for the operator is in the committed report's own section, not
    paraphrased here.
  - **Regeneration safety.** Diffed `git status --porcelain data/corpus` before/after: 0 → 2
    files. No `--allow-stamp-loss` used (this generator has no such flag).
    `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` are consumed only by
    `v06_work_inventory` (`docs/work-inventory.json`), which this cycle never regenerated — only
    `scripts/ingest_ability.py` ran, writing `data/corpus/**` only. `no_record` per kind is
    unaffected by this cycle (no unit moved kind or population; both fixed records already had a
    corpus record before and after — only their `raw_tokens`/`pi_field` changed).
  - **Own-diff PI scrub.** Grepped every new/modified file (source + this receipt + the report)
    against all 60 blacklist terms before pushing; found and fixed 3 uses of "Golarion" as a
    generic descriptor in my own docstrings/report prose (replaced with "published
    campaign-setting"), per this cycle's own instruction not to carry a real PI term in
    authored text. `pi_screening.rs`'s `PI_BLACKLIST_TERMS` array itself legitimately contains
    the real terms — it is the canonical blacklist source, not illustrative example text.
  - **Gate 3's budget constants untouched.**
- **Discovery forwards:**
  - `scripts/retro.py deferral` `1787493585450-t9-onboarding-bcf0ca` — 4 confirmed leaks in
    `domain`/`equipment`/`language`/`spell`, named, not fixed this cycle.
  - `scripts/retro.py deferral` `1787493382983-t9-onboarding-9161f5` — unratified-vocabulary
    candidate population reported for an operator ruling.
  - `scripts/retro.py correction` `1787493549497-t9-onboarding-01846b` — self-correction of the
    first (wrong) 37-record confirmed-leak figure down to the real 4.
- **Next-cycle plan:** a follow-up lane picks up the 4 named confirmed leaks (each needs its own
  kind's generator inspected and a `scrub_blacklist_pi_tokens`-shaped fix), and separately an
  operator ruling on the unratified-vocabulary question stated in the committed report before any
  candidate-detector investment.
