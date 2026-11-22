# Cycle receipt — sd32-declared-pi-audit-verdict

RETRO_ACTOR: sd32-declared-pi-audit-verdict
Branch: tranche/12 (worktree `worktree-wf_154d77f8-51f-2`)
Territory: `scripts/` (the `declared-pi-audit` stage and the binary/library it calls), this receipt.

## Task

`scripts/verify.sh --only declared-pi-audit` had never reached a PASS/FAIL
verdict at the widened corpus population (`beginner_box` ingested,
`advanced_race_guide` `feat_gap` restored). Two prior attempts: a 300s wrapped
`timeout` killed the first; a second unwrapped attempt ran the underlying
`declared_pi_shipping_audit` binary at 99.9% CPU for 6+ more minutes with no
output before being abandoned. Instructed to run it to completion, diagnose
*why* it could not finish rather than reporting "slow," and fix through the
guarded generator path only if a real PI defect turned up.

## Root cause — found, not assumed

`declared_pi_shipping_audit.rs`'s `declared_at()` (CHECK A: shipped-record vs.
cited-row cross-check) re-read and re-split the SAME cited PCGen `.lst` source
file from disk on **every** citing record, with **no caching at all**. The
current corpus cites 677 distinct `.lst` files a total of 51,208 times (one
citation per corpus record carrying a `lst_token` source). One file alone,
`acg_abilities_class.lst` (3.17 MB), is cited by 2,687 different records.
Measured directly (`sum(file_size * citation_count)` over every
`(path, count)` pair actually present in `data/corpus/**/*.json`):
**36.7 GB** of redundant file reads + line-splitting for only 72 MB of unique
`.lst` bytes on disk. That — not CHECK B or CHECK C — was the process
observed pegged at 99.9% CPU with no output: single-threaded repeated
`fs::read_to_string` + `str::lines()` re-scans of the same multi-megabyte
files, thousands of times over, is exactly the "no output for minutes"
signature reported.

Confirmed by instrumented phase timing (temporary `eprintln!`s between CHECK
A/B/C, removed before commit): before any fix, CHECK A alone had not finished
after 6+ minutes. After caching, the same corpus's CHECK A completed in
**3.2s** (release build).

A second, smaller inefficiency was found and fixed in the same pass:
`pi_screening::normalized_term_hits` (CHECK C's own per-string blacklist
scan) re-canonicalized the **entire haystack string** once per blacklist term
— 61 times — even though canonicalization only depends on two boolean fold
flags, and across all 61 terms only 2 (the already-documented Jarn/Galt fold
exceptions) diverge from the common case. So there are at most 3 distinct
canonicalized forms of any given haystack, never 61. Memoized per call.

## Fix — both through ordinary source edits, no corpus regen needed

Neither fix touches `data/corpus/**` — this was a stage-tooling performance
defect, not a PI-shipping defect, so no guarded generator run was required.

1. `src/bin/declared_pi_shipping_audit.rs` — `declared_at()` takes a
   `&mut HashMap<PathBuf, Option<Vec<String>>>` cache (new type
   `LstFileCache`), populated lazily; `audit_shipped_records()` owns one
   `HashMap` for its whole run instead of re-reading per record.
2. `src/rules_core/pi_screening.rs` — `normalized_term_hits()` caches each
   distinct `(needs_rn_fold, needs_char_fold)` canonicalized form of
   `free_text` the first time it's needed (small `Vec` of up to 3 entries,
   linear-scanned — cheaper than a `HashMap` at this size), instead of
   recomputing it once per term.

Both changes are pure memoization of functions whose output does not depend
on iteration order or which caller is asking — same terms checked, same
order, same hits, same violations, proven below.

## Behaviour-unchanged proof

- `cargo test --locked --lib rules_core::pi_screening::` — **40/40 green**,
  before and after, including the exact mutation-proof tests for the Jarn/rn
  fold and Galt/char fold exceptions this optimization's cache key is built
  around (`concatenated_scan_jarn_rn_fold_does_not_catch_an_ordinary_jam`,
  `concatenated_scan_galt_char_fold_does_not_catch_an_ordinary_gait`, plus
  their literal-still-catches counterparts).
- `cargo test --locked --bin declared_pi_shipping_audit` — **21/21 green**,
  before and after (unchanged from HEAD's own 21 pre-existing tests; no test
  added or removed this cycle — the fix needed no new coverage since the
  cache is transparent to every existing mutation-proof fixture).
- `cargo test --locked --lib cache_gen::class_feature::` — **53/53 green**
  (the other live caller of `normalized_term_hits`, confirmed unaffected).

## Live corpus run — the actual verdict

Two full runs against the pinned oracle
(`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, bootstrapped
fresh into this worktree's own repo-local
`docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/
operator-supplied/pcgen/` slot — never the forbidden
`~/workspace/repos/pcgen`):

- **Release build**, direct binary invocation: completed in **78.3s**
  (CHECK A 3.2s, CHECK B 0.5s, CHECK C 74.6s).
- **Debug build**, via `scripts/verify.sh --only declared-pi-audit` itself
  (`cargo run --locked --quiet`, matching the stage exactly): completed in
  **6m55.9s** (`real` from `time`). Slower than release, as expected for an
  unoptimized build doing ~1M string scans, but — the actual point — it now
  **completes**, where before this fix it did not complete inside 6+ minutes
  and was killed both times it was tried.

Both runs report the identical verdict:

```
declared-pi-audit: CLEAN — no shipped record contradicts its own corpus row's PI declaration
```

**Denominator:** `find data/corpus -name '*.json' ! -name LICENSE.json | wc -l`
→ **51,408** — every shipped corpus record this audit's `find_json_files`
walk reaches, CHECK A and CHECK C both. (Note this is a different count from
the "34,416" `shape_ledger.py` population figure named in the dispatch brief
— that is a kind-filtered/classified ledger population, not the raw
corpus-JSON-file count this binary walks directly. Both are legitimate
metrics measuring different things; this receipt's 51,408 is what
`declared_pi_shipping_audit` itself actually scanned.)

**0 violations** across all three checks (NAME-PI-SHIPPED, DESC-PI-SHIPPED,
DESC-PI-SHIPPED-IN-RAW-TOKENS, LICENSE-CLAIM-UNVERIFIED,
BLACKLIST-TERM-SHIPPED) — the widened population (`beginner_box` + restored
`advanced_race_guide feat_gap`) introduced no new PI exposure, and the two
previously-closed defect classes (the 28 `NAME-PI-SHIPPED` violations, the 65
`DESC-PI-SHIPPED` in `bestiary_4/monster_ability` — both root-caused and
closed in earlier `t9-onboarding` cycles per `progress.md`) remain closed.

## No blacklist change, no corpus edit

`docs/governance/ogl-pi-blacklist.md` untouched — still the signed-off
60-term list per operator ruling `decisions.md §28` (2026-08-24). No term
added, no term removed. `data/corpus/**` untouched — `git status --porcelain
-- data/corpus` empty for this cycle's own diff. The Jarn/rn-fold and
Galt/char-fold coordinate-scoped exemptions in
`declared_pi_shipping_audit.rs`'s `KNOWN_OCR_FOLD_FALSE_POSITIVES` were not
touched, widened, or removed.

## Identifier / wired-integration audit (own diff, scoped to my two files)

```
git diff --unified=0 -- src/bin/declared_pi_shipping_audit.rs src/rules_core/pi_screening.rs \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'   -> OK_NO_BUNDLE_TAGS
git diff --unified=0 -- src/bin/declared_pi_shipping_audit.rs src/rules_core/pi_screening.rs \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'   -> OK_NO_TOKENS
```

(The wider `BASE_BRANCH...HEAD` form over these two files' full multi-cycle
history does surface one incidental match — a pre-existing doc-comment line,
from an earlier cycle, correctly naming the real script
`scripts/sd32_t9_corpus_wide_pi_rescan.py`, which does exist in this repo.
Not introduced by this cycle; confirmed by the scoped working-tree diff
above, which is clean.)

## Forward-scope note, not acted on this cycle

`scripts/verify.sh`'s `declared-pi-audit` stage runs `cargo run --locked
--quiet` (debug build), same as every other `cargo run`-based stage in that
script. The release build of the identical binary/logic is ~5x faster
(78s vs. 6m56s) purely from compiler optimization on the string-heavy CHECK
C scan. Flipping just this one stage to `--release` would cut its wall time
substantially, but that changes `verify.sh`'s build-profile convention for
one stage out of several that share the same pattern — a deliberate
consistency decision, not a bugfix, so left named here rather than done
unasked.

## `## Open blockers` in `progress.md`

The `declared-pi-audit did not complete at the widened population` entry
(filed 2026-08-24) is marked RESOLVED below, with this receipt cited as
evidence.

## Status

Complete. `declared-pi-audit` verdict: **CLEAN** at 51,408 records, corpus
SHA `7f818006e371188e5717fd18d74d18a420747fc6`. Named owner for the
"desktop cargo suite regression" sibling blocker (filed the same day) is
unchanged by this cycle — out of territory (`apps/desktop/src-tauri/**`,
lane G), not touched.
