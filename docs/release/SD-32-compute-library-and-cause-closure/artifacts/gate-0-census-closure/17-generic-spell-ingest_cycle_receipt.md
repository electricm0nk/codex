# Cycle 17-generic-spell-ingest — Epic 5 automation / `decisions.md §17` item 2

- **Card ID:** no dedicated kanban row exists for §17 item 2 (added same-day as this
  dispatch); this cycle is filed as supporting infrastructure toward card 11
  (`epic-2-cause-closure`, T9's per-book spell onboarding) and card 15
  (`census-scope-closure`). Both rows left `in-progress` per dispatch instruction — neither
  flips to `complete` from this cycle alone.
- **Commit SHA(s):** to be filled at push time (see progress.md entry for the pushed SHA).
- **Files touched:**
  - Added: `src/bin/ingest_spells.rs` (one config-driven binary, `BOOKS: &[BookInput]`, 9
    entries, replacing all ten books' worth of per-book ingest logic).
  - Deleted: `src/bin/ingest_adventurers_guide_spells.rs`,
    `ingest_inner_sea_gods_spells.rs`, `ingest_inner_sea_setting_spells.rs` (itself already
    config-driven over 3 books), `ingest_occult_adventures_spells.rs`,
    `ingest_ultimate_combat_spells.rs`, `ingest_ultimate_magic_spells.rs`,
    `ingest_ultimate_wilderness_spells.rs` — 3,367 + 510 = 3,877 lines removed net of the new
    file (`ingest_spells.rs` is ~830 lines including tests and doc comment).
  - Regenerated (content-identical `SPELL_LIST` entries, doc-header provenance line only
    differs — see §"Output equivalence" below): `src/rules_core/rules_tables/{adventurers_guide,
    inner_sea_gods,occult_adventures,ultimate_combat,ultimate_magic,ultimate_wilderness,
    inner_sea_faiths,inner_sea_magic,inner_sea_temples}/spell_list.rs`.
  - Added: `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/
    17-pi-screen-drift-diff.py` (reproducible pi_screen/min_level drift diff, reads deleted
    binaries via `git show <ref>:<path>` so it survives the collapse).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion:** `decisions.md §17` item 2 — "Collapse the seven per-book
  spell-ingest binaries into one config-driven pass, and fix the three-way `pi_screen` drift
  to a single screen in a single place while doing it."
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`; `PCGEN_REPO_DIR`/`PCGEN_CORPUS_ROOT` bootstrapped this
  cycle via `scripts/fetch-pcgen-oracle.sh` into a fresh worktree's empty slot — confirmed
  populated via `scripts/verify.sh --only preflight-oracle` before trusting any figure below).
- **Status:** complete for the scope stated (collapse the seven, fix the drift, prove
  equivalence, assess the wider family). No kanban card closes from this alone.

## 1. The `pi_screen` finding — the highest-stakes part of this cycle, done first

Hashed the raw bytes of `pi_screen` across all seven binaries at the last commit that still
has them (`6ae4a364b`): **three distinct byte sequences**, confirming the task brief's claim
at the byte level:

```
53cbef5d (18 lines): adventurers_guide, inner_sea_gods, inner_sea_setting, ultimate_magic, ultimate_wilderness
f5936f96 (18 lines): occult_adventures
5952257a (12 lines): ultimate_combat
```

Diffed again with comments and whitespace normalized away (re-derivable:
`python3 docs/release/.../17-pi-screen-drift-diff.py 6ae4a364b1e42ace9e25df047a2de70bdf4c4948`):
**all three collapse to two normalized hash groups, and the two differ by exactly one
trailing comma** in a struct-literal (`ultimate_combat`'s is formatted on fewer lines by
hand). Same three calls (`declared_product_identity` / `classify_field` /
`classify_optional_field_declared`), same order, same branch conditions, in all seven.

**Finding: there is no live licensing-correctness defect in `pi_screen` itself.** The "three
screens" are raw-text/formatting drift (line-wrap width, one doc-comment word, one missing
trailing comma), not behavioural drift. `occult_adventures`'s and `ultimate_combat`'s
`min_level` DID carry a real, separate defect (below) — but that is a different function.

This does not weaken the collapse's value: three independently-edited byte copies of a
licensing-critical function is a live risk regardless of whether today's three copies happen
to agree, and the T9 PI-exposure audit's own `clear` bucket (`decisions.md §15`,
1,107 units) depended on exactly this kind of cross-book consistency without a mechanism
enforcing it. This cycle reduces it to one copy, checked against the OGL PI blacklist doctrine
(`docs/governance/ogl-pi-blacklist.md`, `SD-27.../decisions.md §17`) and proved by a
mutation-tested unit test (§3 below) that a future divergent per-book screen cannot be
silently reintroduced without a red test.

## 2. The real, separate drift found and fixed: `min_level`

`occult_adventures` and `ultimate_combat`'s `min_level` took only a `CLASSES:` field — no
`DOMAINS:` support, and no `[PRESKILL:...]`/`[PREDEITY:...]` bracket-stripping before
`rsplit_once('=')`, which silently discards a real level if a bracketed sub-condition's own
`=` is grabbed instead. Re-derived against the pinned oracle:

```
grep -c "DOMAINS:" .../occult_adventures/oa_spells.lst   -> 0
grep -c "PRESKILL"  .../occult_adventures/oa_spells.lst   -> 0
grep -c "DOMAINS:" .../ultimate_combat/uc_spells.lst      -> 0
grep -c "PRESKILL"  .../ultimate_combat/uc_spells.lst     -> 0
```

Neither book's corpus exercises the gap today, so the unified (strictly more general)
`min_level` produces byte-identical `SPELL_LIST` output for both — verified, not assumed
(§4). The fix closes the defect before some future OA/UC printing needs `DOMAINS:` or a
bracketed clause.

## 3. Structural + mutation proof that every book gets the same screen

`BookInput` (the config struct driving `ingest_spells.rs`'s 9-entry `BOOKS` table) carries no
field of function-pointer type over `PiOutcome` — `pi_screen` is called unconditionally from
the single `ingest_one_book` loop. `book_input_carries_no_per_book_pi_screen_override_field`
enumerates `BookInput`'s six actual fields so a future PR adding a per-book override fails
that enumeration and forces a human to re-justify it.

**Mutation proof, not just structural argument.** Manually deleted `|| name_blacklisted` from
`pi_screen`'s guard clause (simulating a divergent, under-screening per-book variant):

- Before adding a new test: **every existing test stayed green** — the `NAMEISPI:YES`-based
  tests never exercised the blacklist-only branch, so the mutation was invisible to them. This
  is exactly the "a proof is only as wide as the cases it covers" trap (`AGENTS.md` rule 7).
- Added `pi_screen_drops_a_record_whose_name_is_blacklisted_with_no_declared_pi_token_at_all`
  (a name containing the blacklisted deity term "Iomedae", no `NAMEISPI:`/`DESCISPI:` token at
  all) — **RED** under the mutation (`cargo test --bin ingest_spells
  pi_screen_drops_a_record_whose_name_is_blacklisted` → 1 failed, panic message states the
  expected reason).
- Reverted the mutation — **GREEN**, 19/19 (`cargo test --locked --bin ingest_spells`).

## 4. Output equivalence — proven, not assumed

Backed up all nine books' pre-collapse `spell_list.rs` files, ran `ingest_spells` (all 9
books, one invocation) against the pinned oracle, diffed each book's `SPELL_LIST` entry block
(`pub const SPELL_LIST: &[SpellListEntry] = &[ ... ];`) old vs new:

```
adventurers_guide: ENTRIES IDENTICAL (45 entries)
inner_sea_gods:     ENTRIES IDENTICAL (92 entries)
occult_adventures:  ENTRIES IDENTICAL (144 entries)
ultimate_combat:    ENTRIES IDENTICAL (146 entries)
ultimate_magic:     ENTRIES IDENTICAL (269 entries)
ultimate_wilderness: ENTRIES IDENTICAL (61 entries)
inner_sea_faiths:  ENTRIES IDENTICAL (2 entries)
inner_sea_magic:   ENTRIES IDENTICAL (34 entries)
inner_sea_temples: ENTRIES IDENTICAL (21 entries)
```

**Zero content differences across all nine books, all 814 combined entries.** The only diff in
any file is the module doc-comment header (`//! Generated by ...`), which legitimately changed
because the generator binary changed — this is provenance text, not data. No record was
dropped, added, or PI-relabelled that wasn't already dropped/added/labelled by the pre-collapse
binaries. **No finding to report under the "any pi_screen-driven output change" clause of this
cycle's brief — there is none.**

Re-derive: `python3 <this receipt's sibling compare script logic>` — the exact procedure is
(1) `git show <pre-collapse-SHA>:src/rules_core/rules_tables/<book>/spell_list.rs` to a temp
file per book, (2) run `cargo run --locked --bin ingest_spells` with `PCGEN_CORPUS_ROOT` set to
the pinned oracle, (3) diff each book's `SPELL_LIST` block old vs new.

## 5. What a new book now costs

**A `BookInput` entry: 8 lines** (`id`, `display_name`, `lst_rel`, `out_path`,
`already_ingested`, `dedup_within_book`) — no new function, no new struct, no new binary. If
the new book needs cross-book dedup (rare — only 2 of 9 books need it today), that's one more
`fn already_ingested_<book>() -> BTreeSet<&'static str>` (5-10 lines, following
`already_ingested_oa`/`already_ingested_uc`'s pattern). Nothing else is in the way for a book
whose spells are in `*_spells.lst`-shaped PCGen data — this genuinely reaches the "config
entry" bar the task brief asked about.

## 6. Wider-family assessment (assessed, not collapsed this cycle — per the brief)

`pcgen_data_root()` boilerplate (4-7 lines, near-identical) is duplicated across
`ingest_class_spell_levels_arg.rs`, `ingest_apg_race_traits.rs`, `ingest_pu_classes.rs`,
`ingest_race_traits.rs`, `ingest_races.rs` — but **these five are NOT seven near-duplicate
binaries solving the same problem** the way the spell binaries were; each ingests a
structurally distinct record shape (class spell-level tables / APG race traits / Unchained
classes / race traits / races). The "collapse N per-book copies into one `BOOKS` config
table" pattern used here does not transfer directly — there is nothing to collapse across
these five beyond the shared `pcgen_data_root()` stub, which is a much smaller win than the
spell case (3,877 lines removed here).

**A real, live defect found while checking this:** `ingest_races.rs` reads
`PCGEN_DATA_ROOT`, while all four other ingest binaries in this family (and the new
`ingest_spells.rs`) read `PCGEN_CORPUS_ROOT` — the standard name per
`scripts/fetch-pcgen-oracle.sh`, `AGENTS.md`'s own Concurrency & Measurement section, and this
cycle's own `§2.1` env block. Pointing `PCGEN_CORPUS_ROOT` at the pinned repo-local oracle (as
every dispatch prompt in this bundle instructs) **silently does not redirect
`ingest_races.rs`** — it falls back to `$HOME/workspace/repos/pcgen/data`, the literal path
`AGENTS.md` explicitly forbids hardcoding into new docs/scripts, and which does not exist in a
fresh worktree. **Not fixed here** (out of this cycle's file scope, `src/bin/ingest_*_spells.rs`
only) — reported by name so the orchestrator can route it.

**The real leveraged item in this family — `IN_SCOPE_RACES`, `ingest_race_traits.rs:315`, a
34-race hand-curated allowlist (widened 18→24→30→34 across SD-31 waves)** — is the same
"snowflake treatment" `decisions.md §17` diagnosed for spells, but its fix is a **different**
shape: corpus-driven enumeration replacing a hand-maintained include-list (§17 item 1's
scope — `v06_work_inventory.rs` enumerating every kind the census already finds — not item
2's "merge duplicate binaries" scope, which is this cycle's card). Recommend it be tracked
under item 1's umbrella, not folded into a future "collapse N ingest binaries" card — it is
not a duplication defect, it is a hand-allowlist defect.

`src/rules_core/pi_screening.rs`'s own module doc comment independently corroborates the
shape of this finding: `PI_BLACKLIST_TERMS`/`classify_field`-equivalent logic "was forked
three times, independently" across `gen_book_cache.rs`, `ingest_pu_classes.rs`, and
`ingest_races.rs` before being unified into `pi_screening.rs` — i.e. this exact defect class
(a licensing-critical screen duplicated per-binary) has already recurred once in this family
and was already fixed once for the non-spell binaries. The spell binaries' seven-way
`pi_screen` copy (now collapsed) was the same defect class recurring a second time in a
different corner of the ingest surface.

## 7. Suites run

- `cargo test --locked --bin ingest_spells`: 19/19 (RED→GREEN mutation proof included).
- `cargo test --locked --lib`: 2388 passed, 0 failed, 13 ignored.
- `cargo test --locked --bins` (widest build scope, every bin target): 0 `FAILED` across every
  suite (grepped `test result:` lines, all `ok`).
- `apps/desktop/src-tauri` (separate cargo workspace, tested explicitly per hard rule):
  `cargo build --locked` clean; `cargo test --locked`: 517/517 passed, 0 failed.
- `scripts/verify.sh --only preflight-oracle`: PASS (oracle at pin
  `7f818006e371188e5717fd18d74d18a420747fc6`).

## Discovery forwards

1. `ingest_races.rs`'s `PCGEN_DATA_ROOT`/`PCGEN_CORPUS_ROOT` env-var-name drift (§6 above) —
   a live footgun for any cycle that runs `ingest_races.rs` expecting the standard
   `PCGEN_CORPUS_ROOT` to redirect it to the pinned oracle. Not fixed (out of this cycle's file
   scope).
2. `IN_SCOPE_RACES` (34-race hand allowlist, `ingest_race_traits.rs:315`) is the real
   leveraged item in the race/class ingest family — filed under `decisions.md §17` item 1's
   scope (generic enumeration), not item 2's (this cycle's collapse pattern).

## Next-cycle plan

`decisions.md §17` item 3: "re-run the shape ledger over everything and report what is
genuinely left" — belongs to whichever cycle owns card 15's reconciliation, not this one
(this cycle's scope was item 2 only, plus the item-1-adjacent assessment above).
