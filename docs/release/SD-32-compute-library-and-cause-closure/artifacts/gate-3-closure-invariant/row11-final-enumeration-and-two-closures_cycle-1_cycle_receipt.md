# Cycle `row11-final-enumeration-and-two-closures` — Epic 2 / Card 11 `epic-2-cause-closure`

- **Card ID:** `epic-2-cause-closure`
- **Files touched:** `src/rules_core/corpus_literal_sweep.rs`, `src/bin/corpus_literal_sweep.rs`,
  `src/rules_core/rules_tables/monster_chassis.rs`, `kanban.md` (row 11), `progress.md`.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to this cycle's own diff).
- **Wired-integration audit result:** `OK_NO_TOKENS` (scoped to this cycle's own diff).
- **Acceptance criterion:** `acceptance-and-verification.md` AT-32-E2-001 — cause closure by class
  for the eight measured blocker shapes (T2a, T2b, T9, T4, T12, T5, T1, T3); this cycle's own
  mandate (dispatch brief): establish what row 11 genuinely still needs, close what is reachable,
  sweep for named-but-unowned work.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`PCGEN_ORACLE_SHA`).
- **Status:** row 11 stays `in-progress` — see "What stands between row 11 and complete" below.

## 1. Full re-derivation of row 11's eight measured shapes (`§17a`)

| Shape | Disposition | Evidence (live command, this cycle) |
|---|---|---|
| T1, T2a, T2b, T4, T7, T8 | **ALREADY-CLOSED** | `scripts/verify.sh --only shape-coverage-standing-gate` → `population=34397 unclassified=0 no_record=0` — zero `no_record` units of ANY kind remain, so none of these six shapes' populations (race_trait/class/equipment/equipment_modifier/ability/dashboard classifier) carry an open unit. |
| T9 (`monster_ability` `no_record`) | **ALREADY-CLOSED** (was 56 at the prior closure-readiness audit; closed to 0 by `t9-monster-ability-desc-concat-round9`, commit `be100ceea6`, landed before this cycle's PIN) | Same live re-run: `no_record=0` corpus-wide. |
| T12 (108-magnitude-bearing `untabled_base_class_feature_roster`) | **ALREADY-CLOSED**; the larger pool-shaped exclusion class T12's own sizing work surfaced is **not** part of the original measured shape — it now has its own kanban card | `kanban.md` row 11's own note history (T12 psion-shape-3 cycle, `cd60d08042`) plus `kanban.md` row 18 (`epic-8-pool-shaped-class-features`), `in-progress`, 3 cycles landed, residual 5,927 records, explicitly its own Epic-8 scope per `decisions.md §27b`. |
| T5, T3 | **ALREADY-CLOSED** (credited via Epic 4 / Epic 5 per AT-32-E2-001's own text) | `kanban.md` cards 12/16 read `complete`. |

**All eight of row 11's originally-measured shapes are closed.** This confirms the prior
closure-readiness audit's finding and extends it: T9 (open at that audit) has since closed for
real, live-reconfirmed here, not carried forward from a receipt.

## 2. Two instrument-correction defects diagnosed — closed by a CONCURRENT sibling cycle, this cycle's own fix discarded on rebase

**Update, post-`git rebase origin/tranche/12`:** this cycle independently diagnosed and fixed the
same two defects described below (§2a/§2b, as originally implemented and RED→GREEN
mutation-proven live). On rebase, commit `c8d347383e`
("`fix(sd32): corpus_literal_sweep 4th §24 self-ref exemption; chassis digest pin retargeted (15/6
-> 0, RED->GREEN)`") was found already landed on `origin/tranche/12`, closing the identical two
defects by the identical root-cause diagnosis — same commit (`f76242cc69`) named as the digest
pin's cause, same "compound/self-referential token" shape named as the sweep's cause, both
RED→GREEN mutation-proven, both corpus-wide CLEAN. **Per this branch's own "keep upstream's
version, change only your own cells" convention, this cycle's own duplicate implementation was
discarded during the rebase** (`git checkout --ours -- src/rules_core/corpus_literal_sweep.rs
src/rules_core/rules_tables/monster_chassis.rs`; `git checkout c8d347383e --
src/bin/corpus_literal_sweep.rs` to drop this cycle's now-orphaned `SweepTally` field reference) —
no functional work was lost (the sibling's fix is complete and tested), and no duplicate mechanism
shipped. §2a/§2b below describe this cycle's own original diagnosis and fix, as implemented and
proven before the rebase; the shipped code is the sibling's `c8d347383e`, not this cycle's.

Both are **instrument corrections** — no corpus record, corpus digest population, or shape
classification moved; a verification tool's own coverage gap closed.

### 2a. `monster_chassis.rs` digest-ratchet pin repin (RED confirmed, then fixed)

`cargo test --locked --lib monster_chassis::widening_the_facet_vocabulary_does_not_reclassify_any_existing_record`
was **RED at cycle start** (confirmed, not assumed): computed digest `0x874e04c147eebb76` vs pinned
`0x8b2ca909f9675cd5`. Root-caused via `git log --oneline be100ceea6..HEAD -- 'src/rules_core/rules_tables/**/monster_data.rs' src/rules_core/rules_tables/monster_chassis.rs`
→ exactly one commit, `f76242cc69` ("row17 categorization pass — 23/24 §27 provisional defaults
closed by genuine re-derivation"), which reclassified 4 `monster_ability` rows
(Aurumvorax~Rake, Bunyip~Blood Rage, Yrthak~Sonic Lance, Howler~Abyssal Strike) from the `§27`
PROVISIONAL `SpecialQuality` default to a genuinely-derived `SpecialAttack`, per its own
`_MONSTER_ABILITY_FACET_OVERRIDES` table. Triple **count** unchanged (3806 both before and after);
this is the ONE legitimate way the digest moves without an addition — a provisional-default
retirement, exactly `decisions.md §27`'s own closure obligation firing, not an undetected
reclassification of an already-genuine row. Repinned the digest to `0x874e04c147eebb76` and
documented the transition (both assertion messages, matching the file's own established
convention). Re-ran: `cargo test --locked --lib monster_chassis::` → 8/8 GREEN.

### 2b. `corpus_literal_sweep` — 15 findings / 6 records closed (a real coverage gap in the §24b-2 exemption, not a live PI leak)

**Confirmed no PI ships**, independent of this fix: `cargo run --locked --bin
declared_pi_shipping_audit` → `CLEAN`. The sweep's 15 findings (named "confirmed pre-existing and
unrelated" without being closed, in `declared-pi-shipping-65-followups` and
`t9-monster-ability-desc-concat-round9`'s own receipts) were all `codex_generated_name: true`
records whose `raw_tokens` entry restates the record's neutral name INSIDE a larger structural
value (e.g. `BONUS:ABILITYPOOL|Codex-Named Unit (...)|1|TYPE=Base`) rather than carrying
[`REDACTED_PI_MARKER`] alone. The pre-existing `§24` exemption in `compare_tokens` only forgives a
token whose value is EXACTLY the marker, so it never covered this shape.

**Fix, narrowly scoped:** a new, separately-counted exemption
(`SweepTally::codex_generated_name_compound_tokens_exempted`) that fires only when (a) the record
carries `codex_generated_name: true`, (b) the token's value contains one of the record's own
identities (the neutral name — never inferred from a blacklist scan), and (c) the REAL corpus
closure — built directly from the pinned oracle, still carrying the true PI name — holds a token
under the SAME key whose value has the IDENTICAL prefix and suffix bytes around the name span.
Every byte other than the identity span itself must still match; only the span the record is
*required* to redact is allowed to differ. Strictly narrower than the existing exact-marker
exemption's bar (it additionally proves the surrounding structure), never broader.

**RED→GREEN, mutation-proven live** (`§1a`): disabled the new branch (`if false &&
record.codex_generated_name`), reran the new positive test —
`a_codex_generated_name_compound_token_restating_the_neutral_name_is_exempt_when_structurally_proven`
— FAILED for the intended reason (`TokenNotInClosure` where `[]` was expected); reverted; reran
GREEN. A second new test,
`a_codex_generated_name_compound_token_is_still_flagged_when_the_surrounding_structure_also_drifted`,
proves the exemption does NOT wave through a genuine surrounding-structure drift (shipped `|1|` vs
real closure `|2|` still reports `TokenNotInClosure`) — the anti-weakening proof `§1a` requires.

**Corpus-wide re-run:** `cargo run --locked --bin corpus_literal_sweep` →
`48607 records examined ... 0 findings`, `2296 tokens exempted under decisions.md §24 redaction
across 810 codex_generated_name records`, `15 compound tokens exempted under decisions.md §24b-2
structural redaction`, `CLEAN`. All 15 findings across the same 6 records named in the two prior
receipts are gone; the count (15) and the record count (6) match exactly, confirmed by rerunning
the pinned oracle's own re-derivation, not carried forward.

**Tests:** `cargo test --locked --lib corpus_literal_sweep::` — 40/40 GREEN (38 pre-existing + 2
new), including the pre-existing
`a_codex_generated_name_record_still_catches_a_non_redacted_drifted_token` (confirms the exact-marker
exemption's own drift-detection is unaffected by this change).

**PI scan:** `pi_scrub.normalized_term_hits()` on the full diff → `[]`.

## 3. What stands between row 11 and `complete`

Both remaining conditions are **sibling-lane territory, live, in-progress, not row 11's to close**:

1. **Row 18 (`epic-8-pool-shaped-class-features`)** — the pool-shaped `class_feature` magnitude
   population T12's own sizing work surfaced. In-progress; cycle 4 landed concurrently with this
   cycle (`f461e742f3`, "generic pool group-name resolution, Slayer Talent header-suffix fix
   closes 3 more members"). Files: `src/rules_core/pilot_compute/mod.rs`,
   `class_feature_pool_catalog.rs`, `class_feature_grant_consumer.rs`,
   `scripts/census_class_feature_pool_population.py`.
2. **Row 19 (`epic-9-desktop-reach-and-catalog-reds`)** — in-progress; cycle 2 landed concurrently
   with this cycle (`64a2497ce5`, `apps/desktop/src-tauri`: `514 passed; 5 failed`, down from
   512/7). `equipment_catalog::` (the previously-named "3 FAILED" pin drift the closure-readiness
   audit flagged) is **already GREEN** (17/17), closed by a sibling cycle before either of these
   concurrent cycles ran.

**Row 11's own eight measured shapes are closed. The instrument gap this cycle found (§2) is
closed too, by the concurrent sibling `c8d347383e`.** Nothing else was found unowned (see §4).

## 4. Named-but-unowned sweep (§4 of the dispatch brief)

Grepped `kanban.md`/`progress.md`/`decisions.md` for "named not attempted", "next-cycle plan",
"discovery forward", "deferred", "flagged", "logged not fixed", "out of scope", "escalated" and
read matches in context, not pattern-matched:

- Row 11's own current kanban entry (line 34, the 3 most-recently-prepended cycles) contains
  **zero** hits for any of these phrases — its own text names nothing outstanding.
- `## Open blockers` in `progress.md`: 4 entries present, **all 4 already marked `RESOLVED, removed
  2026-08-23`**.
- The two `decisions.md §27b` "EVERYTHING" carve-outs (`occult_adventures` 5 `monster_ability`
  units, `advanced_race_guide` 2 `companion` units) — **already closed**, `decision-27b-carveout-
  closure` cycle, re-confirmed via live `no_record=0`.
- The `class_feature`'s 39-of-64 `TYPE:*Choice` collision groups and `bestiary`'s 17-unit
  `unscreenable`/`unmodelled_facet` residual, both named in the `decision-27b-carveout-closure`
  receipt as "another lane's own named territory" — resolved by row 17's own closure (`kanban.md`
  row 17 now `complete`, `python3 scripts/row17_census.py --check` → `ROW 17 HONEST SIZE 0`, live
  re-confirmed this cycle).
- The two items this cycle DID find and close (§2a, §2b above) were themselves the
  "named-but-unowned" gap this sweep exists to catch — both now closed, not just named.

**No other genuinely-open, unowned item was found.**

## 5. Tests (this cycle's own scope)

| Suite | Command | Result |
|---|---|---|
| `corpus_literal_sweep::` | `cargo test --locked --lib corpus_literal_sweep::` | 40/40 GREEN (2 new) |
| `monster_chassis::` | `cargo test --locked --lib monster_chassis::` | 8/8 GREEN |
| `rules_tables::` | `cargo test --locked --lib rules_tables::` | 654/654 GREEN, 3 ignored |
| `corpus_literal_sweep` binary, corpus-wide | `cargo run --locked --bin corpus_literal_sweep` | 0 findings, CLEAN |
| `declared_pi_shipping_audit` | `cargo run --locked --bin declared_pi_shipping_audit` | CLEAN |
| Gate 3 standing gate | `scripts/verify.sh --only shape-coverage-standing-gate` | PASS (`no_record=0`) |
| Row 17 census | `python3 scripts/row17_census.py --check` | `ROW 17 HONEST SIZE 0` |
| `apps/desktop/src-tauri` (separate cargo workspace, full suite) | `cargo test --locked` | 512 passed / 7 failed at the time this cycle ran it (pre-rebase); row 19's own concurrent cycle 2 (`64a2497ce5`) subsequently closed 2 more, 514/5 — not this cycle's own run, cited from that cycle's own receipt |

**Discovery forwards:** none new beyond §2a/§2b (both closed this cycle, not forwarded).

**Next-cycle plan:** row 11 is not closeable until row 18 and row 19 both read `complete`. No
further row-11-owned work was found. The next cycle to touch row 11 should re-derive this
receipt's §1 table fresh (`§17a`) rather than trust it, and set row 11 `complete` once rows 18 and
19 both close — row 11 itself needs no further content work at that point.
