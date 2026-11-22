# Cycle sweep-remainder-and-chassis-red — Gate 3 closure invariant / two follow-ups left behind by declared-pi-shipping-65-followups

- **Card ID:** none (precondition/instrument fix, same shape as the two prior
  `corpus-literal-sweep-*` unblock cycles — not itself an Epic card; touches no kanban row).
- **Actor:** `t9-onboarding`
- **Base:** `a73bd33d34f62d132d275e9814758e2230095398` (pinned `PIN`, == `origin/tranche/12` tip at
  dispatch — no intervening commits).
- **Commit SHA:** see push receipt (this cycle's own commit).
- **Files touched:**
  - `src/rules_core/corpus_literal_sweep.rs` (fourth `§24` exemption + doc comment + 2 new tests)
  - `src/rules_core/rules_tables/monster_chassis.rs` (stale digest-ratchet pin retargeted + round-10
    doc-comment entry; no other line changed)
  - `docs/retro/events/t9-onboarding.jsonl` (append-only; one auto-logged `verification` event from
    the required footgun-4/oracle-bootstrap preflight check at dispatch start)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to this cycle's own diff: `git diff --
  src/rules_core/corpus_literal_sweep.rs src/rules_core/rules_tables/monster_chassis.rs`)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope)
- **Acceptance criterion:** dispatch brief items 1 and 2 — reproduce and decide
  `corpus_literal_sweep`'s remaining 15 findings / 6 records (sweep bug vs. genuinely inconsistent
  records), and establish the true state of
  `monster_chassis::tests::widening_the_facet_vocabulary_does_not_reclassify_any_existing_record`
  (RED, unconfirmed by the orchestrator).
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`;
  oracle slot was empty in this fresh worktree, bootstrapped via `scripts/fetch-pcgen-oracle.sh
  --dest <repo-local slot>`, confirmed via `/proc/<pid>/environ` before trusting any output).
- **Status:** complete
- **Notes:** see full narrative below.
- **Discovery forwards:** none.
- **Next-cycle plan:** none needed for either item; both are closed at the root, not carved out.

## 0. Wrong-base check (footgun 1)

Dispatch-start `HEAD` was **not** a descendant of `PIN` (`git merge-base --is-ancestor` exit 1 — the
worktree started on a stale lineage, `1bb523773d` / PR #374's merge). `git reset --hard "$PIN"`,
re-verified `ANCESTOR_OK`, `git rebase origin/tranche/12` — a no-op, since `origin/tranche/12`'s tip
**is** the pin commit (no intervening pushes). Oracle slot confirmed empty (git-ignored, fresh
worktree), bootstrapped via `scripts/fetch-pcgen-oracle.sh --dest <repo-local slot>`
(never the forbidden `~/workspace/repos/pcgen` default), `PCGEN_CORPUS_ROOT` confirmed via
`/proc/<pid>/environ` pointing at the repo-local slot before the first real audit run.

## 1. Item 2 first — `widening_the_facet_vocabulary_does_not_reclassify_any_existing_record`: real RED, then real GREEN, root cause identified

Reproduced with a full, real compile (`cargo test --locked --lib
rules_core::rules_tables::monster_chassis`, `CARGO_TARGET_DIR`-isolated, `CARGO_INCREMENTAL=0`):
**RED**, 7 passed / 1 failed. The count assertion (`triples.len() == 3806`) **passed** — the failure
is on the digest assertion alone, printed live:

```
left:  9749735471485729654   (0x874e04c147eebb76)
right: 10028576330551352533  (0x8b2ca909f9675cd5 -- the pin currently in the file)
```

**Root cause, established before touching the pin (`§17a`).** A digest change with an UNCHANGED
triple count means at least one already-shipped `(book, key)` pair's facet moved — exactly the shape
this test exists to catch when it is an accident. `git log --oneline -1 -- src/rules_core/
rules_tables/monster_chassis.rs` names the commit that last set the pin
(`be100ceea6`); `git log --oneline -3 -- src/rules_core/rules_tables/*/monster_data.rs` shows one
commit landed AFTER it and BEFORE this cycle's dispatch: `f76242cc69` ("row17 categorization pass —
23/24 §27 provisional defaults closed by genuine re-derivation, 1 escalated"), already merged to
`origin/tranche/12`. `git show --stat f76242cc69 -- 'src/rules_core/rules_tables/*/monster_data.rs'`:
5 files, 4 insertions / 62 deletions (comment-block removal only, plus 4 one-line facet-field
diffs). `git show f76242cc69 -- src/rules_core/rules_tables/bestiary_2/monster_data.rs`: confirms the
4 real content changes are exactly `facet: MonsterAbilityFacet::SpecialQuality` →
`MonsterAbilityFacet::SpecialAttack` for `Aurumvorax ~ Rake` and `Bunyip ~ Blood Rage` (the other two,
`Yrthak ~ Sonic Lance` and `Howler ~ Abyssal Strike`, are the same shape in `bestiary_2`'s own diff,
named identically in `f76242cc69`'s own commit message).

**Disposition: this is NOT the defect the test guards against.** `f76242cc69` is row 17's own
TDD'd, mutation-proved, per-record oracle-re-derived closure cycle (`decisions.md §27`/`§27a`/`§27b`),
applied through the newly-sanctioned `_MONSTER_ABILITY_FACET_OVERRIDES` mechanism in
`transcribe_monster_tables.py` — not a careless widening of `parse_type`'s facet vocabulary. It is the
first round where the triple COUNT does not move at all (3806 → 3806): 4 already-shipped triples were
deliberately reclassified, corroborated per-record against the pinned oracle, by a cycle that ran and
merged before this one, leaving this file's ratchet pin the only thing not yet caught up.

**Fix:** retargeted the pin to the live-failure's own printed digest (`0x874e_04c1_47ee_bb76`,
converted from the failing assertion's own `left` value via `python3 -c "print(hex(...))"` — never
guessed) and appended a "round 10" entry to the assertion's own doc-comment explaining why this round
is qualitatively different from rounds 1–9 (a genuine, evidence-backed reclassification, not an
addition) and naming the commit and the 4 records by coordinate.

**GREEN, re-verified with a second full real compile+run** (not assumed from the diagnosis alone):
`cargo test --locked --lib rules_core::rules_tables::monster_chassis -- --test-threads=4` → **8/8
pass**, including `widening_the_facet_vocabulary_does_not_reclassify_any_existing_record`.

No mutate-and-revert was needed on this test beyond what already happened: the RED state itself was
the live proof the guard fires on a real digest mismatch; the GREEN state after retargeting is the
proof it accepts the correct value. (The test's own historical rounds 1–9, each independently
mutation-tested at the time, remain intact — only the terminal comparison value changed.)

## 2. Item 1 — `corpus_literal_sweep`'s 15 findings / 6 records: sweep bug, not bad records

Reproduced with a real oracle run (`cargo run --locked --release --bin corpus_literal_sweep --
--json-out ...`, release build, pinned oracle): **15 findings across 6 records**, exactly as briefed
— `adventurers_guide/{class_feature/fighter, class_feature/shaman, feat_generic}`,
`inner_sea_gods/trait_generic`, `inner_sea_magic/class_feature/bard`,
`inner_sea_world_guide/feat_generic`. Every finding is `MISMATCH ...: token not byte-present in
corpus token closure`, and every flagged token's VALUE contains the string
`"Codex-Named Unit (<coordinate>)"` — the record's own `§24` neutral name — embedded inside an
otherwise ordinary, mechanically real token: `BONUS:ABILITYPOOL|Codex-Named Unit (...)|1|TYPE=Base`,
`PREMULT:1,[PREABILITY:1,CATEGORY=...,Codex-Named Unit (...)],...`, `KEY:Codex-Named Unit (...)`,
`ABILITY:...|Codex-Named Unit (...) ~ Costume Proficiency|...` (a child ability namespaced under this
record's own renamed parent key).

**Per-record read (`§17a`, never by class alone):** read the full JSON of
`data/corpus/adventurers_guide/class_feature/fighter/codex_named_unit_class_feature_adventurers_guide_
ag_abilities_class_lst_846.json`. `codex_generated_name: true`, `pi_field:
"description,name,raw_tokens"`, `rename.reason: "name_pi_blocked"`. `KEY`/`TYPE`/`DESC` already ship
the bare `[redacted PI]` marker (and were already exempt under the existing exact-match `§24`
exemption); `BONUS` ships `ABILITYPOOL|<neutral name>|1|TYPE=Base` — the record's OWN neutral name
substituted for the original, not the bare marker. This is the correct, intended shape: `BONUS:
ABILITYPOOL`'s value is semantically "the name of this ability's own pool" (a real PCGen idiom — a
class feature commonly names its own ability pool after itself); redacting it to the bare marker
would ship a value the compute engine cannot read as a pool name at all. `decisions.md §24b`-2
requires the PI original appear nowhere that ships — it does not require the redaction mechanism to
be the bare marker specifically, and other §24-family generators (the `PREMULT`/`ABILITY`
self-references in the other 5 records) show the same pattern.

**Disposition: (b) — the sweep's exemption logic did not recognise this legitimate shape.** The
existing third `§24` exemption in `compare_tokens` only recognised a token whose value is EXACTLY the
marker sentinel. It had no branch for a token whose value CONTAINS the record's own renamed identity
as a substring inside an otherwise-real token. **The records are correct; the sweep was too narrow —
same finding shape as both prior `corpus_literal_sweep` unblock cycles (Galt/gait, Andoran, Cayden
Cailean, hidden_wand), which is why this was checked first rather than assumed** (`§17a`'s standing
instruction not to assume either direction on this instrument).

**Fix (`src/rules_core/corpus_literal_sweep.rs::compare_tokens`):** added a fourth, sibling `§24`
exemption: on a `codex_generated_name: true` record, a token whose value contains ANY of the record's
own `identities` (`data.key`/`data.name`/`source.record_key` — all equal to the neutral name on a
`§24` record) as a substring is exempt, counted the same way the existing exemptions are
(`SweepTally::codex_generated_name_tokens_exempted`/`_records_exempted` — the divergence stays
visible, never silent, `§22`). Narrow by construction: gated on `codex_generated_name` (never
inferred from filename), and only fires when the token's own value carries the record's own renamed
identity string.

**Proven narrow, not just proven green (`§1a` — never widen an exemption to silence a finding that is
telling the truth):**
- New test `a_self_referential_token_containing_the_records_own_neutral_name_is_exempt` reproduces
  the live shape (KEY/BONUS/PREMULT/ABILITY all self-referencing) and proves all four are exempt,
  one (COST) is not.
- New test `a_codex_generated_name_record_still_catches_a_non_self_referential_drifted_token`
  proves a `codex_generated_name: true` record with an UNRELATED drifted token (one that neither
  reads the marker nor contains the record's own identity anywhere) is **still reported** — this is
  the mutation-style negative the brief requires: a genuinely inconsistent `§24` record cannot smuggle
  a real defect through by merely being renamed.
- `cargo test --locked --lib rules_core::corpus_literal_sweep`: **40/40 pass** (38 pre-existing + 2
  new).

**Re-derivation with the real oracle, before vs. after (`§17a`, no guessing):**
- Before (this cycle's own reproduction): `48607 records examined of 51389 read, ... 15 findings`,
  `2296 tokens exempted ... across 807 codex_generated_name records`.
- After: `48607 records examined of 51389 read, ... 0 findings` — **CLEAN**. `2311 tokens exempted
  ... across 810 codex_generated_name records` (+15 tokens, +3 records — exactly the 15 findings
  closed; 3 of the 6 affected records already had at least one token exempted via the pre-existing
  exact-marker exemption and were already counted, the other 3 are newly counted).
- **No `data/corpus/**` file was touched by this fix** — the records were already correct; only the
  sweep's own logic changed. `git status --porcelain` confirms zero corpus-file changes for this
  cycle.
- `declared_pi_shipping_audit` re-run (release, pinned oracle): **CLEAN — no shipped record
  contradicts its own corpus row's PI declaration** (unaffected by this cycle, as expected — this
  cycle touches neither `data/corpus/**` nor that binary's own source).

## 3. Universal requirements

- **PI grep on this cycle's own diff:** `git diff -- src/rules_core/corpus_literal_sweep.rs
  src/rules_core/rules_tables/monster_chassis.rs` grepped for every live `ogl-pi-blacklist.md` term
  (Irori, Cayden/Cailean, Aldori, Magaambya(n), Jarn, Torag, Sarenrae, Golarion, ...): **zero hits**.
  This cycle's new test fixtures use only synthetic placeholder strings
  (`"Codex-Named Unit (class_feature_x_1)"`, `"The Real PI Name"`, `"Costume Proficiency"`), matching
  the file's own existing convention.
- **No record-count or shape-population change.** This cycle wrote zero `data/corpus/**` bytes; the
  full corpus status distribution is unchanged (verified: `corpus_literal_sweep`'s own
  `records_examined`/`records_read` totals are identical before and after — 48607/51389 both runs).
  `no_record`, `docs/work-inventory.json`, and the Gate 3 standing-gate budget are therefore untouched
  and were not re-derived (nothing in this cycle's scope could move them).
- **Row 17 / Row 17a marker discipline:** this cycle applied no `§27` provisional default and stamped
  no `shape_provisional_marker.py` marker — it only re-verified that row 17's own already-merged,
  already-mutation-proved facet corrections (`f76242cc69`) are correctly reflected in this file's
  independent digest ratchet.

## 4. Territory

No file overlap with row 17 (`pilot_compute/mod.rs` untouched), row 18
(`pilot_compute/mod.rs`/`class_feature_pool_catalog.rs`/`class_feature_grant_consumer.rs`/
`scripts/census_class_feature_pool_population.py` — none touched), or row 19
(`apps/desktop/src-tauri` — none touched; see §5 below for the read-only sweep of that workspace).
`kanban.md` untouched (this cycle mints no card, per the two prior `corpus-literal-sweep-*` unblock
cycles' own precedent — a precondition/instrument fix, not an Epic card).

## 5. Item 3 — both-workspace sweep

See the separate, scoped test runs recorded in this cycle's push receipt / final report: root
`cargo test --locked --lib rules_core::` (scoped, not the full unscoped suite) and
`apps/desktop/src-tauri`'s own scoped `cargo test --locked --bin codex-desktop` run, reported by test
name with provenance rather than re-run to exhaustion here.
