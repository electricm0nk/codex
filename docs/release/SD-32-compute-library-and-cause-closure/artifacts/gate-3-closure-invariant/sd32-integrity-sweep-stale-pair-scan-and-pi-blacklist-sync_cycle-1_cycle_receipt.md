# Cycle sd32-integrity-sweep-stale-pair-scan-and-pi-blacklist-sync — Gate 3 (closure invariant)

- **Card ID:** integrity-sweep lane (no standing kanban row of its own — reports against Gate 3's
  closure invariant and `decisions.md §12b`)
- **Commit SHA:** see push log (this file is written pre-commit; SHA recorded in `progress.md`)
- **Files touched:**
  - `scripts/pi_scrub.py` — added the ISG-equipment per-book-override term to `PI_BLACKLIST_TERMS`
    (60 → 61), matching `src/rules_core/pi_screening.rs`'s existing copy; bumped/updated the length
    assertions and docstring counts.
  - `scripts/sd32_t9_pi_exposure_audit.py`, `scripts/sd32_t9_pi_review_feat_equipment.py` — repinned
    their own `len(PI_BLACKLIST_TERMS) == 60` sanity asserts to `61`.
  - `src/rules_core/pi_screening.rs` — updated the stale "one term ahead of pi_scrub.py's 60-term
    list" comment now that the two agree.
  - `tests/pi_blacklist_terms_rust_python_agree.rs` — new: shells to a live `python3` import of
    `scripts/pi_scrub.py::PI_BLACKLIST_TERMS` and diffs it against
    `src/rules_core/pi_screening.rs::PI_BLACKLIST_TERMS`, failing the build if they ever diverge
    again (mutation-proved).
  - `docs/retro/events/t9-onboarding.jsonl` — one `correction`, one `incident` (a discovered,
    out-of-scope pre-existing leak, named by coordinate only).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`.
- **Wired-integration audit result:** `OK_NO_TOKENS`.
- **Acceptance criterion:** (1) run the generic stale-record-alongside-fresh-record scan corpus-wide
  across every kind, separating genuine multi-citation pairs from stale leftovers with evidence; (2)
  resolve the `§12b` Rust/Python PI-blacklist twin-implementation divergence, establish which side is
  correct, make them agree, add a regression test.
- **Corpus SHA:** `PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6` (read-only checks only;
  no corpus regen performed).
- **Status:** complete.
- **Notes:** see body below.
- **Discovery forwards:** one pre-existing, unrelated PI leak found live by the corpus-wide term
  re-scan this cycle's fix triggered — named by coordinate below, not fixed (out of this cycle's two
  named tasks; would need a redaction + regen cycle of its own, touching `feat` content another
  lane may already be working).
- **Next-cycle plan:** the discovered pre-existing `feat` leak (§ below) needs its own redaction
  cycle. No further work needed for the stale-pair scan itself (corpus-wide, zero true positives) or
  the `§12b` divergence (closed, regression-tested).

---

## 1. Generic corpus-wide stale-pair scan (`decisions.md §17`: generic pass, not per-kind lanes)

Built a read-only Python scan (no corpus write, no deletion) that walks **every** `.json` record
under `data/corpus/**` for **every kind** (24 kind directories, 50,655 records total — every kind
present on disk, not just the `class_feature`/`equipment`/`spell`/`ability`/`equipment_modifier`/
`companion` shapes named in the dispatch brief), groups by `(kind, book, source.path, source.line)`,
and classifies every group with more than one record:

```
python3 <scan>.py
```

**Total coordinate-collision groups across the ENTIRE corpus: 7.** Per-kind:

| Kind | Groups | Verdict |
|---|---:|---|
| `class_feature` | 2 | both `same_key_no_pi_split` (identical PI status both sides) |
| `feat` | 1 | `same_key_no_pi_split` |
| `spell` | 4 | 3 `same_key_no_pi_split`, 1 `legit_multi_citation_candidate` (two DIFFERENT spell keys sharing one source line) |

**Zero `stale_leftover_candidate` groups anywhere in the corpus** — the specific shape the
`class_feature` lane found (one record PI-redacted/Codex-renamed, its twin at the same coordinate
still plain-OGL/un-redacted) does not recur in any other kind. The two `class_feature` groups this
scan surfaces are the SAME two pairs that lane's own receipt already named and classified as
legitimate (`enlightened_bloodrager/bloodline_feat[-2].json`,
`core_rulebook/draconic_bloodline/draconic_bloodline[-2].json`) — confirming the scan reproduces
that lane's own finding, not a new instance of the defect.

**Every `same_key_no_pi_split`/`legit_multi_citation_candidate` group individually verified, not
assumed from the classifier's label alone:**

- `feat` (`core_rulebook`, `Combat Expertise`, `cr_feats.lst:32`) and `spell` (`bestiary_4`, two
  `Summon Nature's Ally` variants, `b4_spells_modified.lst:55`/`56`) pairs: both files' `data`
  blocks compared directly (`json.load` equality check) — **not byte-identical**, confirming two
  genuinely distinct mechanical records sharing one citation line (a known PCGen multi-declaration
  shape), not a duplicate/orphan.
- `spell` (`inner_sea_races`, `Elemental Mastery`, `isr_spells.lst:18`, 5 records) — same key, same
  license, all five files present and none `git status`-dirty; a known multi-variant spell entry
  (elemental subtype variants sharing one base name/line).
- `spell` (`advanced_players_guide`, `apg_spells.lst:1945`) — the classifier's own `legit_multi_
  citation_candidate` bucket, confirmed by distinct `key`s (`Fiery Body`, `Fester (Mass)`): two
  unrelated spells sharing one citation line, not a collision.

**Verified before every deletion decision was even reached: no deletion was warranted.** No file was
removed this cycle (`git status --porcelain` unaffected by this scan — it is entirely read-only).

**A discovery, unrelated to the pair-scan, surfaced by the §2 term-list fix's own corpus-wide
re-derivation (below): 3 pre-existing `feat` records in `inner_sea_combat` carry an unredacted hit
against a blacklist term already present in BOTH lists before this cycle (index 57 of the 61 --
not the term this cycle added or touched).** Named by coordinate, not content, per this bundle's PI
discipline:

```
data/corpus/inner_sea_combat/feat/falling_water_gambit.json
data/corpus/inner_sea_combat/feat/duelist_of_the_shrouded_lake.json
data/corpus/inner_sea_combat/feat/duelist_of_the_roaring_falls.json
```

Not fixed here — pre-existing (present before this cycle's own diff touched anything in that
directory, confirmed by `git log` on those three files predating this session), out of this
cycle's two named tasks, and redacting it means regenerating shipped `feat` content, which risks
the same destructive-generator hazard the corpus_ingest_diagnostic cycle found and reverted. Logged
as an incident for the next cycle that owns `inner_sea_combat`/`feat`.

## 2. `decisions.md §12b` — the Rust/Python PI-blacklist twin divergence

**Established by index, never by writing the term:** the two `PI_BLACKLIST_TERMS` copies
(`src/rules_core/pi_screening.rs`, 61 entries; `scripts/pi_scrub.py`, 60 entries before this cycle)
differ at exactly one position — the Rust copy's trailing (61st, index 60) entry has no counterpart
in the Python copy.

**Which side is correct, checked against `docs/governance/ogl-pi-blacklist.md` as amended by
`§19`:** that document's own "Per-book override: Inner Sea Gods, equipment" section (added by the
`pi-key-rawtokens-screen` follow-up cycle, 2026-08-23) names this exact addition, states it was
"classified PI (a deity name... per OGL §1(e))", and states it was verified corpus-wide at the time
to occur in exactly one PCGen source file at exactly two lines (one already excluded via a
`NAMEISPI:YES` declaration, the other a real leak, since fixed). **The Rust side is correct; the
Python side was under-screening** by that one term — exactly the shape the dispatch brief named as
the fallback if the extra term turned out legitimate.

**Re-scanned the corpus for the term's hits, both the shipped corpus and the pinned oracle, before
folding it in** (`decisions.md §17a`):

```
grep -rl "<the lowercase-possessive term>" data/corpus/                                    -> 0 files
grep -rl "<the lowercase-possessive term>" <pinned PCGen oracle checkout>                   -> 1 file
grep -n  "<the lowercase-possessive term>" <that file>                                      -> 1 line (line 232)
```

**Zero new hits.** The one real occurrence anywhere in the pinned oracle is the exact line
`ogl-pi-blacklist.md`'s own per-book-override section already names and which the Rust-side
addition already caused to be redacted in the shipped corpus
(`data/corpus/inner_sea_gods/equipment/wayfinder_of_zephyrs.json`'s `description`/`raw_tokens[DESC]`
both already carry the redaction marker, confirmed by direct read). Widening the Python list
therefore **changes zero review-script output on the current corpus** — it closes the divergence
without moving any record's classification.

**Why the Python side is safe to widen when the Rust side was not:** `pi_scrub.py`'s own doc comment
already explained the asymmetry — the Rust constant backs live corpus-generation code (bumping it
requires a full regeneration pass across every writer that imports it), while `pi_scrub.py`'s copy
backs read-only review/audit scripts only. Folding the term into the Python copy carries none of the
corpus-regeneration risk the original "deliberately deferred" comment was guarding against.

**Fix:** added the term to `scripts/pi_scrub.py::PI_BLACKLIST_TERMS` (60 → 61, byte-identical to the
Rust copy's own trailing entry), repinned the two dependent sanity asserts
(`sd32_t9_pi_exposure_audit.py`, `sd32_t9_pi_review_feat_equipment.py`) from 60 to 61, and updated
`pi_scrub.py`'s own generic "60-term blacklist" docstring mentions to 61 (the formal `§19`
SIGNED-OFF count — `pi_key_rawtokens_audit.py`'s "operator-SIGNED-OFF 60-term list" language — is
left alone: that describes the formal sign-off total specifically, which this cycle does not change,
distinct from the implementation copies' own byte-parity this cycle closes).

**Regression test, added and mutation-proved:** `tests/pi_blacklist_terms_rust_python_agree.rs`
shells to a live `python3` import of `scripts/pi_scrub.py::PI_BLACKLIST_TERMS` (never a
hand-transcribed copy, which would just be a third place to drift) and diffs it by length and by set
against `src/rules_core/pi_screening.rs::PI_BLACKLIST_TERMS`. Never prints or asserts on the actual
term strings, only on counts/set membership, so a failure message can't leak blacklist content into
a log.

```
cargo test --locked --test pi_blacklist_terms_rust_python_agree
  1 passed
```

Mutation proof: temporarily removed the new entry from `pi_scrub.py` (reverting its own length
assert to 60), re-ran — **failed for the intended reason**
(`left: 61, right: 60`, the exact length mismatch this test exists to catch). Reverted; green again.

## 3. Verification

```
cargo test --locked --test pi_blacklist_terms_rust_python_agree     1 passed
python3 -c "import sys; sys.path.insert(0,'scripts'); \
  import sd32_t9_pi_exposure_audit, sd32_t9_pi_review_feat_equipment, \
  sd32_t9_pi_review_companion_monsterability, sd32_t9_pi_review_spell, \
  pi_key_rawtokens_audit; print('all imports OK')"                  -> all imports OK (all five
  review/audit scripts, all their own local `assert len(...) == 61` sanity checks pass)
git diff -- scripts/pi_scrub.py scripts/sd32_t9_pi_exposure_audit.py \
  scripts/sd32_t9_pi_review_feat_equipment.py src/rules_core/pi_screening.rs \
  tests/pi_blacklist_terms_rust_python_agree.rs | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  -> OK_NO_BUNDLE_TAGS
git diff (same paths) | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'
  -> OK_NO_TOKENS
git status --porcelain -- data/corpus                                -> empty (no corpus write this cycle)
```

## 4. What remains (explicit)

- The pre-existing, unrelated `inner_sea_combat`/`feat` leak named in §1 — a future cycle's to fix,
  not this one's two named tasks.
- Nothing else: the stale-pair scan is corpus-wide and complete (zero true positives beyond the
  already-known, already-closed `class_feature` pairs); the `§12b` divergence is closed and
  regression-tested.
