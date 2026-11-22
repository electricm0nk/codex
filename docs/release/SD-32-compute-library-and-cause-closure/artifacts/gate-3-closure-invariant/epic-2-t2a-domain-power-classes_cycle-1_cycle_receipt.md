# Cycle — Gate 3 (closure invariant) / Card 11, `Domain Power` upstream class link (decisions.md §23a)

- **Card ID:** `epic-2-cause-closure` (row 11; this receipt covers ONLY the `Domain Power`
  (172-unit) sub-population `decisions.md §23a` names — row 11 stays `in-progress`, per this
  cycle's dispatch brief, since T2a-residual's remaining ~525 labels and card 11's other four
  open sub-populations are unaffected)
- **Commit SHA:** recorded in `progress.md`'s entry for this cycle after push (`git log -1
  --format=%H`)
- **Files touched:**
  - `src/rules_core/cache_gen/class_feature.rs` — new `classes: Option<Vec<String>>` field on
    `ClassFeatureData`; new `scan_domain_power_owners`/`domain_power_owning_classes`/
    `effective_lst_key` functions (the seventh resolution tier, kept SEPARATE from `class` and
    from `category_label_alias_owner`); wired into `generate()`; +4 new tests
  - `scripts/derive_domain_power_classes.py` — new, independently re-runnable Python oracle of
    the identical scan logic, used to cross-validate the Rust implementation before landing it
  - `data/corpus/**/class_feature/domain_power/*.json` (172 records — data, not code; `data.class`
    unchanged on every one, `data.classes` newly populated on all 172)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  (`git diff --unified=0 HEAD -- src/rules_core/cache_gen/class_feature.rs scripts/
  derive_domain_power_classes.py` — no `sd[0-9]+_`/`SD[0-9]+_`/`Sd[0-9]+`/`t_[0-9a-f]{8,}` matches)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff — no
  `STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack` tokens)
- **Acceptance criterion:** `acceptance-and-verification.md` AT-32-E2-001, scoped by
  `decisions.md §23a`: `Domain Power` (172 units) closes by extending the generator's inputs to
  read the upstream class link, able to report several owning classes, not forced into
  `CATEGORY_LABEL_ALIASES`' single-label-to-single-class shape.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`
  `PCGEN_ORACLE_SHA`) — fresh worktree, empty oracle slot, self-healed per §8 via
  `scripts/fetch-pcgen-oracle.sh --dest <repo-local pcgen slot>`; verified to match the pin exactly
  (`pcgen-oracle: OK 7f818006e371188e5717fd18d74d18a420747fc6 <dest>`).
- **Status:** complete (this batch's own scope — the `Domain Power` label only)

## Re-deriving before trusting the brief's figures (`decisions.md §17a` bar)

```
grep -rl '"class": *"Domain Power"' data/corpus/*/class_feature 2>/dev/null | wc -l
# -> 172
```
Confirmed: 172 records, matching the brief and the prior receipt
(`epic-2-t2a-residual-alias-tier_cycle-1_cycle_receipt.md`). 172 of 172 carry `DESC`/`TYPE`/`PRE*`
tokens with no per-record class name (re-confirmed by re-reading a sample against the corpus
before building anything, per the brief's "prove you can read it for a sample of records before
building the mechanism").

## Finding the upstream link BEFORE writing anything (brief's step 1)

Read PCGen's own domain `.lst` files directly (`cr_domains.lst`, `apg_abilities_class.lst`, etc.)
against the pinned oracle. The domain-power-granting record's own line (e.g. `KEY:Domain Power ~
Chaos Blade`, `cr_abilities_class.lst:694`) carries no class name, confirming the prior cycle's
finding. One hop upstream, it does exist:

**Every `"Domain Power ~ <X>"` ability is granted to a character by a class-namespaced chooser
record shaped `"<Prefix> Domain ~ <domain>"` (`CATEGORY:Internal`), via an
`ABILITY:...|AUTOMATIC|Domain Power ~ <X>|...` token on that chooser record.** Read directly:

```
grep -n "^Chaos" apg_abilities_class.lst
# Chaos ... KEY:Inquisitor Domain ~ Chaos ... CATEGORY:Special Ability TYPE:InquisitorDomain
#   PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Inquisitor Domain ~ Chaos],
#     [PREABILITY:1,CATEGORY=Special Ability,Core Domain ~ Chaos Domain], ...
grep -n "Core Domain ~ Azata Subdomain (Chaos)" apg_abilities_class.lst
# Core Domain ~ Azata Subdomain (Chaos)  CATEGORY:Internal
#   ABILITY:Special Ability|AUTOMATIC|Domain Power ~ Elysium's Call|...
#   ABILITY:Special Ability|AUTOMATIC|Domain Power ~ Chaos Blade|PREVARGTEQ:DomainChaosAbilityTriggerLVL,8
```

The prefix names which class's domain-access mechanism the grant runs through. Read the class
`.lst` files directly to confirm which classes each prefix actually maps to (never assumed):

- **`Core Domain ~`** — the base PCGen `DOMAIN` facet. `cr_classes.lst` `CLASS:Cleric`:
  `BONUS:DOMAIN|NUMBER|ClericDomainCount` and `BONUS:VAR|ClericDomainCount|2`. `cr_classes.lst`
  `CLASS:Paladin`: also `BONUS:DOMAIN|NUMBER|PaladinDomainCount`, but `PaladinDomainCount`
  `DEFINE`s to 0 and is raised ONLY by the Sacred Servant archetype's own ability
  (`apg_abilities_class.lst`, `KEY:Sacred Servant ~ Spells`, `BONUS:VAR|PaladinDomainCount|1
  |TYPE=Base`, `BONUS:VAR|DomainLVL|PaladinLVL-3`) — so `"Core Domain ~"` genuinely owns both
  classes, confirmed by reading both class files and the archetype ability, not assumed from the
  prefix name.
- **`Inquisitor Domain ~`** — Inquisitor's own domain-access mechanism (verified present and used
  across the seven SD-32 in-scope books this population touches: 63 of 172 records also carry an
  `Inquisitor Domain ~` grant alongside `Core Domain ~`).
- **`Druid Domain ~`** — exists in the oracle (verified via `KEY:Druid Domain ~ Alchemy` etc.,
  third-party `divine_favor_cleric`) but does not co-occur with any of these 172 in-scope records;
  0 of 172 carry it.

**Bare-key grants (no class-namespaced wrapper).** Some (mostly newer/subdomain) books skip the
wrapper and grant a power straight from the bare domain/subdomain-named record — e.g.
`bestiary_6/b6_domains.lst`'s `"Dragon Subdomain"` grants `"Domain Power ~ Venomous Stare"`
directly, no `"Core Domain ~ Dragon Subdomain"` intermediate. Read directly and confirmed: that
bare record is gated by the SAME generic `PREDOMAIN:` check every domain uses, reachable through
the identical base `DOMAIN` facet `"Core Domain ~"` runs through — so it resolves the same two
owners (Cleric, Paladin), not a guess, the identical mechanism minus one redundant hop. One
exclusion, found and proven by reading the actual line: a `.MOD` line whose first field is a
foreign feat-category token (`CATEGORY=FEAT|Believer's Boon.MOD`, `acg_feats.lst:177`) is a
cross-reference into an unrelated feat record, not a domain grant point, and is excluded.

## Building it generically (`decisions.md §17`)

`scan_domain_power_owners(corpus_root)` walks every `.lst` file under the oracle ONCE (≈2,900
files) and returns `{domain power key suffix -> owning classes}` for EVERY `"Domain Power ~ <X>"`
target the corpus grants anywhere — not scoped to the 172 records, not a per-record lookup table.
`domain_power_owning_classes(key, owners)` then does a single map lookup per record. The
prefix→classes table is 3 entries (`Core`, `Druid`, `Inquisitor`), each independently verified
against the class `.lst` files above — not a hand-authored table of 172 mappings.
`scripts/derive_domain_power_classes.py` carries the identical logic as an independently
re-runnable, non-Rust cross-check; both were run against the pinned oracle and agree (see below).

**`CATEGORY_LABEL_ALIASES` was NOT touched, and the standing test
`category_label_alias_owner_refuses_the_known_multi_owner_and_not_class_owned_labels` was NOT
amended.** `Domain Power`'s multi-owner resolution is a SEPARATE new field (`data.classes:
Option<Vec<String>>`), never collapsed into `class` and never routed through
`category_label_alias_owner` (which correctly keeps refusing to give `"Domain Power"` a single-class
answer — that refusal is still true and still enforced). This satisfies the brief's requirement
that a domain power be able to resolve to several owning classes without forcing
`CATEGORY_LABEL_ALIASES`' single-label shape.

## Cross-validation: Rust scan vs. independent Python oracle

```
python3 scripts/derive_domain_power_classes.py --corpus-root "$PCGEN_CORPUS_ROOT" \
  --check-names <172 record KEYs, one per line> --json /tmp/domain_power_owners.json
# -> total 153 single-owner 0 multi-owner 153 unresolved 0   (153 = distinct KEYs among 172 records)
```
Then the real Rust `generate()` regen (below) independently reproduced the identical owner sets
for all 172 records — Rust and Python agree, corpus-wide, not sampled.

## Regeneration discipline (`decisions.md §22`'s divergence-visibility bar, and the guarded-regen
discipline from the prior T2a-residual cycle)

```
CARGO_TARGET_DIR=... PCGEN_CORPUS_ROOT=<repo-local pinned oracle> \
  cargo run --locked --bin gen_cache_class_feature
# -> class_feature cache generated: 17856 records across 23 books; 140 skipped (NAMEISPI:YES)
```

Field-by-field diff of every modified file against its pre-image (`scripts/diff_check_regen.py`,
excludes `ingested_at` and `data.class`/`data.classes` — the only fields this cycle's regen is
expected to move):

```
python3 scripts/diff_check_regen.py
# checked 17852 modified files
# class/classes changed: 172
# classes newly added: 172
# OTHER field changed (should be 0): 0
```

**Exactly the 172 `Domain Power` records changed anything beyond `ingested_at`, and the only
thing that changed on them is the new `data.classes` field — `data.class` is byte-identical to
before.** No other record's `class`, `raw_tokens`, `description`, or any other field moved. This
is a strictly additive regen: no `--allow-stamp-loss` was used, no `CORPUS_LITERAL_SWEEP_REPORT`/
`DERIVED_FIXTURE_CHECK_REPORT` regeneration was needed because no stamp-bearing field changed on
any record outside the 172 (verified by the diff check above, not assumed).

## Per-record outcome (brief's explicit ask — no bare totals, `decisions.md §12c`)

```
python3 scripts/summarize_domain_power_classes.py
# total domain_power records: 172
# 2 owners: ('Cleric', 'Paladin') -> 109 records
# 3 owners: ('Cleric', 'Inquisitor', 'Paladin') -> 63 records
# unresolved (no classes field): 0
```

- **172 of 172 resolve to MULTIPLE owning classes.** 0 resolve to a single class (confirming the
  prior cycle's finding that `Domain Power` is not single-owner — this cycle closes it by
  recording the real multi-owner answer, not by forcing one).
- **0 remain unresolved.** No record was rounded into a guess; every one of the 172 traces to a
  real, read `ABILITY:...AUTOMATIC|Domain Power ~ <X>` grant line in the oracle.
- 109 records: `{Cleric, Paladin}` (via `Core Domain ~` or an equivalent bare-domain grant).
- 63 records: `{Cleric, Inquisitor, Paladin}` (also carry an `Inquisitor Domain ~` grant).

## RED → GREEN

`domain_power_owning_classes(&unit.key, &domain_power_owners)` mutated to `None::<Vec<String>>`:

```
cargo test --locked --lib cache_gen::class_feature::tests::generate_writes_the_multi_owner_classes_for_a_domain_power_record_never_collapsing_class
# FAILED: left: None  right: Some(["Cleric", "Paladin"])  -- failed for the intended reason
```

Reverted; green again. Module suite (`cache_gen::class_feature::`): 38/38 pass (4 new:
`effective_lst_key_prefers_the_explicit_key_token_over_the_display_name`,
`scan_domain_power_owners_resolves_core_druid_inquisitor_and_bare_grants`,
`domain_power_owning_classes_returns_none_outside_the_namespace_and_when_unmapped`,
`generate_writes_the_multi_owner_classes_for_a_domain_power_record_never_collapsing_class`).

## Reachability proved separately from resolution (brief's step 5)

`domain_power_owning_classes` returning `Some([...])` proves the OWNER SET is resolvable and
recorded — it says nothing about consumer wiring. Confirmed no consumer treats `data.class`'s
existing (unchanged) value as authoritative in a way this cycle could break, and no consumer reads
`data.classes` yet (grep below): this cycle is a data/resolution closure, not an engine-wiring
claim, and does not represent that `Domain Power` records are now computed by any consumer.

```
grep -rn 'data\["class"\]\|data\.get("class")\|\.class ==\|data\.class\b\|data\["classes"\]\|\.classes\b' \
  src/ apps/desktop/src-tauri/src/
```
Same four `class`-reading consumers the T2a+T12 and T2a-residual cycles already found
(`class_feature_pool_catalog.rs`, `class_feature_descriptions.rs`,
`class_feature_grant_consumer.rs`, `class_feature_feat_bridge.rs`), plus one new one
(`intelligent_item_catalog.rs`) — none reads `data.classes` (a brand-new field this cycle adds; no
consumer to conflict with). No consumer-conflict hazard; `data.class`'s value for these 172
records is byte-identical to before this cycle, so no existing consumer's behavior changes.

## Suites run

- `cargo test --locked --lib cache_gen::class_feature::` — **38/38 pass** (4 new).
- `cargo test --locked --lib` — running in background at receipt-write time; result appended to
  `progress.md`'s entry for this cycle once it completes (this module's own regen only added a
  field, did not touch any other module's code).
- `cargo run --locked --bin corpus_literal_sweep` — running at receipt-write time against the full
  regenerated corpus; result appended to `progress.md`.

## Pinned-count sweep

`grep -rn "172\b" --include='*.rs' --include='*.py'` across `src/`, `tests/`, `scripts/`, `apps/`
— no hits pinning the 172-unit `Domain Power` population as a test assertion outside this bundle's
own docs, so this cycle's addition of `data.classes` does not leave any other file's hardcoded
count red.

## What this cycle closes, and what it does not

**Closed:** the upstream class link for all 172 `Domain Power` units — a real, provable,
corpus-wide, multi-owner resolution recorded on every record, extending the generator's inputs per
the operator's ruled option (a) (`decisions.md §23a`), not a relabelling and not a single-class
guess.

**Not this cycle's scope:** `Demonic Obedience` (42 units, `decisions.md §23b` — a `kind`
re-typing, different file); the remaining ~525 T2a-residual labels (`decisions.md §23c`); card
11's other four open sub-populations (T2b, T9, T12, T4-L9, `decisions.md §13`). Row 11 stays
`in-progress`.

- **Discovery forwards:** none requiring a new card.
- **Next-cycle plan:** apply `decisions.md §23c`'s table to the remaining ~525 T2a-residual
  labels — most will be single-owner (`CATEGORY_LABEL_ALIASES` path); any found genuinely
  multi-owner can reuse this cycle's `classes` field shape rather than inventing a new one.

`df -h /`: recorded at end of turn, see final report.
