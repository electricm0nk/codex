# Cycle — SD-34 wave 35, Lane B — `Human ~ Tribalistic Languages` TEMPLATE: grant reader

**Status: complete, with an honest correction of scope.** Dispatched to close wave 33 lane
B's own named 2-unit remainder (`Next-cycle plan` item 4: "`Human ~ Tribalistic Languages`
needs a `TEMPLATE:`-reading mechanism (new); `Suli ~ Trusted Mediator` and `Rougarou`'s
selector have no project-side remedy at all"), and the dispatch brief additionally named
`Human ~ Tribalistic` as this unit's "sibling" needing the same closure. **Re-derived before
writing any code:** `Human ~ Tribalistic` (`isr_abilities_race.lst:210`) was already
`status: grounded` in `docs/work-inventory.json` before this cycle started — no defect, no
work needed on it. The real, still-open population this cycle owns is exactly **one** unit:
`Human ~ Tribalistic Languages` (`isr_abilities_race.lst:216`). Logged as a `correction`
(`docs/retro/events/sd34-wave35-laneb.jsonl`, `1788405507082-sd34-wave35-laneb-3579c5`).

## What the record's own TEMPLATE: token encodes — read directly, verified against the corpus

`data/corpus/inner_sea_races/race_trait/human/human_tribalistic_languages.json`'s own
`TEMPLATE` raw token: `Bonus Language ~ Common|Bonus Language ~ Giant|Bonus Language ~
Goblin|Bonus Language ~ Halfling` — matches the pinned upstream `.lst` byte-for-byte
(`isr_abilities_race.lst:216`, read directly this cycle, not only its corpus JSON
transcription). Each named entry is itself a real, already-ingested `Kind::Template` corpus
record (`data/corpus/core_rulebook/template/bonus_language_{common,giant,goblin,halfling}.json`),
and each one's ENTIRE body is one mechanical token: `LANGBONUS:<Lang>`
(e.g. `bonus_language_common.json`'s own `raw_tokens` are exactly `[VISIBLE:NO,
LANGBONUS:Common]`). So the real PF1 mechanic this row encodes is exactly what its own `DESC:`
states: Tribalistic humans' bonus-language *pool* is restricted to these four languages
(Common/Giant/Goblin/Halfling) instead of the standard Human's open "any spoken language"
pool — a real, quoted, non-fabricated grant, not a guess.

## Existing TEMPLATE:-reading precedent, checked before building a new one

`grep -rn "TEMPLATE:" src/rules_core/` found exactly one existing reader:
`race_resolver.rs`'s `declared_size`/`size_from_size_template`, which reads a
`TEMPLATE:SIZE_<code>` literal suffix off a trait's own raw tokens (a *different* shape —
the code IS the size, not a reference to another corpus record). No existing idiom reads a
`TEMPLATE:` value as a *name* pointing at another ingested corpus record, so this cycle adds
one, following the same "transcription, not interpretation" doctrine `declared_size`'s own
doc comment states (`decisions.md §24`).

## Why this stays `engine-does-not-hold` — the real, verified reason, not assumed

Read the pinned upstream `.lst` line directly this cycle (not only the corpus JSON):

```
216:Languages   KEY:Human ~ Tribalistic Languages   CATEGORY:Special Ability
  TYPE:RacialTraits.Human Racial Trait.SpecialQuality.Racial Language   VISIBLE:DISPLAY
  DESC:Tribalistic humans only start with their ethnic language; if they have high
  Intelligence scores, they can select their bonus languages from among Common, Giant,
  Goblin, and Halfling.
  TEMPLATE:Bonus Language ~ Common|Bonus Language ~ Giant|Bonus Language ~ Goblin|
  Bonus Language ~ Halfling   SOURCEPAGE:p.214
```

Zero `FACT:`, `PREFACT:`, `PREABILITY:` or `ABILITY:<category>|AUTOMATIC|<key>` tokens of any
kind. The alternate that logically owns it, `Human ~ Tribalistic` (`:210`), fires only
`FACT:Human_ReplaceLanguages|true`, which *suppresses* the standard `Human ~ Languages` row
(`suppressed_by_flag: Human_ReplaceLanguages` in its own corpus JSON) — it does **not** name
`Human ~ Tribalistic Languages` anywhere. This is a genuine upstream PCGen data omission,
independently confirmed three ways before this cycle started: `race_resolver.rs`'s own
`no_corpus_trait_is_left_without_a_readable_gate` test comment, `reach_gate.rs`'s dated
`OPEN_FINDINGS` entry (`"inner_sea_races"`/`"race_traits"`), and this cycle's own direct read
of the pinned `.lst` byte content. `race_resolver.rs`'s own `TraitRole::FlagGranted` doc
comment states its contract precisely: *either* a positive `PREFACT:...=True` gate on the
replacement row *or* a direct `ABILITY:<category>|AUTOMATIC|<key>` token on the granting row.
`Human ~ Tribalistic Languages` matches neither. Inventing a THIRD classification path (a
name/flag pattern match with no real upstream token backing it) would fabricate a game
mechanic the source data does not license — exactly what `AGENTS.md`'s "no fabricated
magnitude" rule and item 7 ("a proof is only as wide as the cases it covers") forbid. So the
record correctly stays `TraitRole::Unclassified` / `engine-does-not-hold`: **nothing upstream
ever fires it**, regardless of what it would grant if it did.

What changes is the evidence string's own honesty. The blanket
`race_trait_record_loaded_but_never_applies` a record with genuinely NO resolvable content
also carries is silent on the fact that THIS record's own content is real, quoted, and
verified. `race_trait_template_bonus_language_grant_verified_but_has_no_upstream_activation_gate`
says exactly that instead — matching wave 33 lane B's own precedent for the 27-unit
"resolves real grants, no consumer wired" shape, one level more precise: here there is no
consumer to wire at all, only a real grant with no gate.

## Files touched this cycle

- `src/rules_core/race_resolver.rs` — new `declared_template_bonus_languages(&[RawToken]) ->
  Vec<String>` (transcription of a `TEMPLATE:Bonus Language ~ <Lang>|...` chain), 4 unit
  tests (2 real-corpus-grounded proof cases, 2 negative controls).
- `src/bin/v06_work_inventory.rs` — new `RaceTraitProbe::template_bonus_language_grant`
  field, populated in `probe_race_trait_corpus`'s existing wave-33-lane-B population loop;
  new `EngineFacts::race_trait_template_bonus_language_grant` accessor; new `classify()`
  branch in the `Kind::RaceTrait` Unclassified fallback, giving the one matching record
  (`isr_abilities_race.lst:216`) the precise evidence string above. 3 new tests: the proof
  case, a transcription-accuracy regression, and a negative control (`Suli ~ Trusted
  Mediator`, the OTHER inner_sea_races Unclassified residue this same remainder item named,
  which carries no `TEMPLATE:` token at all and must stay untouched).
- `scripts/completion_atlas.py` — this cycle's own insertions (an import, an accessor, a
  probe field, its population loop, and the new `classify()` branch, all landing ABOVE
  every citation's own construction site) shifted all 9 non-`V` bucket citations
  (`--check`'s own `citation_failures=9` before the fix, listing exactly DONE/A/B/C/D/M/U/X/Z
  — `V` sits below this cycle's edits too but its own 13562 pin happened to still resolve
  correctly). Re-derived all 9 fresh: `grep -n` for each marker's own unique
  construction-site literal against the current file, each new line's content read back and
  confirmed before writing it here (never assumed from arithmetic alone, though the +44 /
  +77 cumulative-shift pattern the fresh greps produced is internally consistent and
  recorded per-citation).
- `docs/work-inventory.json`, `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/
  completion-atlas.json` — regenerated via the guarded path (below).
- This receipt, `progress.md`.

## Identifier / wired-integration audits

`git diff --unified=0 2fb15ced6be68c86bd135b39082c331c92b26bea -- src/rules_core/
race_resolver.rs src/bin/v06_work_inventory.rs | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|
t_[0-9a-f]{8,})'` -> `OK_NO_BUNDLE_TAGS`.
`git diff --unified=0 2fb15ced6be68c86bd135b39082c331c92b26bea -- src/rules_core/
race_resolver.rs src/bin/v06_work_inventory.rs | grep -nE '\b(STUB|MOCK|placeholder|not yet
implemented|todo|fixme|hack)\b'` -> `OK_NO_TOKENS`.

## RED -> GREEN

- RED: the new `classify()` branch neutralized in place (`if false && let Some(langs) = ...`,
  brace structure preserved so the file still parses), proof test re-run:
  `tribalistic_languages_gets_a_precise_template_grant_evidence_not_the_blanket_string`
  FAILED on `assert_ne!` — both sides read `"race_trait_record_loaded_but_never_applies"` —
  the intended reason (the record reads exactly as indistinguishable from one with no
  resolvable content, which is the defect this cycle fixes).
- GREEN, branch restored:
  `cargo test --locked --bin v06_work_inventory race_trait_grounding_tests:: -j 6` ->
  **41 passed; 0 failed** (38 pre-existing + 3 new).
  `cargo test --locked --lib rules_core::race_resolver::declared_template_bonus_languages -j
  6` -> **4 passed; 0 failed**.
  `cargo test --locked --lib no_corpus_trait_is_left_without_a_readable_gate -j 6` -> **1
  passed** — confirms `TraitRole::Unclassified` is UNCHANGED for this record (and every
  other Unclassified record); only the evidence string moved.
- `cargo clippy --locked --bin v06_work_inventory -j 6 -- -D warnings` and
  `cargo clippy --locked --lib -j 6 -- -D warnings` -> both clean, 0 warnings.

## Scope-safety check: is this the ONLY Unclassified record this branch can touch?

`grep -rl '"key": "TEMPLATE"'` across every `data/corpus/*/race_trait/**/*.json`, filtered to
`Bonus Language ~` values, found **~49** race_trait records carrying this shape (every
race's standard `<Race> ~ Languages` default plus every alternate-trait `~ Languages`
replacement: `Feral ~ Languages`, `Scion of Humanity ~ Languages`, `Deep Jungle Halfling ~
Languages`, the four Geneiekin "Mostly Human ~ <Kin> ~ Languages" rows, etc.). All of them
except `Human ~ Tribalistic Languages` are already `Default`/`Alternate`/`FlagGranted` —
confirmed by cross-checking against `no_corpus_trait_is_left_without_a_readable_gate`'s own
EXHAUSTIVE, pinned Unclassified-key list, which names `Human ~ Tribalistic Languages` as the
only "~ Languages"-shaped entry in it. `probe_race_trait_corpus`'s `reachable` set is built
from a per-record `role != Unclassified` check over EVERY loaded record of a race (not a
"resolved with no alternates" simulation), so every one of those ~48 other rows already hits
`race_trait_engine_book(unit) = Some(...)` and returns from an EARLIER branch in `classify()`
— my new branch, placed after that check, structurally cannot reach any of them.

## Guarded regeneration

```
cargo run --locked --bin corpus_literal_sweep -- --json-out <scratch>/corpus_literal_sweep_report.json --quiet
-> corpus-literal-sweep: CLEAN
cargo run --locked --bin derived_evaluator_fixture_check -- --json-out <scratch>/derived_evaluator_fixture_check_report.json --quiet
-> (no findings)
CORPUS_LITERAL_SWEEP_REPORT=<scratch>/corpus_literal_sweep_report.json \
DERIVED_FIXTURE_CHECK_REPORT=<scratch>/derived_evaluator_fixture_check_report.json \
cargo run --locked --bin v06_work_inventory
-> docs/work-inventory.json regenerated, 49438 units
```

`python3 -c "import json; print(len(json.load(open('docs/work-inventory.json'))['units']))"`
-> **49438** — unchanged (no corpus records added or removed this cycle, only a probe field
and a `classify()` evidence-string branch).

## Movement (four buckets, this cycle)

- **Closure (bucket -> DONE):** 0.
- **Reclassification:** 0 — the one unit stays in bucket D (`engine-does-not-hold`).
- **Reachability:** 0 — nothing upstream fires this record; it did not become newly
  reachable, and cannot without an upstream PCGen data fix.
- **Instrument-correction:** 1 evidence string replaced (a blanket "never applies" with a
  precise, corpus-verified "TEMPLATE-borne grant with no upstream gate"), plus 9
  `completion_atlas.py` citation lines re-derived after this cycle's own insertions shifted
  them, plus 1 dispatch-brief scope correction logged to retro (the named "sibling" was
  already done).

## Figures (every number, its command, its denominator)

- Population this cycle owns: **1** — `Human ~ Tribalistic Languages`
  (`isr_abilities_race.lst:216`); denominator: wave 33 lane B's own named 2-unit remainder,
  corrected to 1 (the other named unit, `Human ~ Tribalistic`, was already `grounded`).
- `docs/work-inventory.json` total population: **49438** —
  `python3 -c "import json; print(len(json.load(open('docs/work-inventory.json'))['units']))"`,
  denominator: whole corpus, unchanged from pre-cycle.
- `completion_atlas.py --check`: `population=49438 buckets=10 unclassified=0 overlap=0`,
  `citation_failures=0` (was 9 before this cycle's own re-derivation) —
  `python3 scripts/completion_atlas.py --check`, denominator: whole corpus.
- Bucket D count: **2891**, unchanged before and after this cycle's regen (D=2891 both
  before this cycle's code change and after, confirmed by re-running `--check` against the
  freshly-regenerated `docs/work-inventory.json`, `generated_at: 2026-09-03T03:29:15Z`) —
  this cycle moves no unit between buckets, only an evidence string within D.

## Sweep population

N/A — this cycle never wrote `data/corpus/**` (no corpus records added, removed, or
regenerated; only `src/rules_core/race_resolver.rs`, `src/bin/v06_work_inventory.rs`,
`scripts/completion_atlas.py` and the two regenerated instrument JSONs). `corpus_literal_
sweep` was still run as part of the guarded regen path (`records_examined: 48706`, `CLEAN`,
0 findings) — its count reflects prior waves' own corpus growth, not this cycle's, since
this cycle added nothing to the population it examines.

## Retro events this cycle

- `incident` (`docs/retro/events/sd34-wave35-laneb.jsonl`,
  `1788405470604-sd34-wave35-laneb-25042f`, `recurrence_key: wrong-base-worktree`): this
  worktree's own branch was cut at `ea2b3396f2` (409 commits behind the real bundle state),
  and `origin/tranche/14` was itself stale relative to a further-ahead local `tranche/14` ref
  holding wave 33+34's unpushed work. The identical shape `wave34_laneB`'s own receipt names
  ("A note on the worktree this cycle started from") — the same class `AGENTS.md` item 8
  already tracks at 27+ prior occurrences, now recurring a further time. Resolved via
  `git rebase tranche/14` (the local, ahead-of-origin ref) rather than `origin/tranche/14`.
- `correction` (`1788405507082-sd34-wave35-laneb-3579c5`): the dispatch brief's own claim
  that `Human ~ Tribalistic` needed closing alongside `Human ~ Tribalistic Languages` was
  wrong — it was already `grounded` before this cycle. See "Status" above.

## Oracle pin

Not consulted this cycle — no figure here came from the pinned PCGen corpus checkout; every
quoted token is read directly from this repo's own `data/corpus/` and the pinned upstream
`.lst` path already checked into the repo's oracle-fetch slot.

## Next-cycle plan

1. **0 remaining** for this exact shape: `Human ~ Tribalistic Languages` is now honestly
   evidenced and cannot move further without an upstream PCGen data fix (the row has no gate
   to read). Name it, alongside `Suli ~ Trusted Mediator` and `Rougarou`'s selector, as
   **permanently blocked pending upstream** — not a to-do — matching wave 33 lane B's own
   disposition for the other two.
2. Wave 33 lane B's next-cycle plan item 2 (20 Skinwalker `Change Shape` components, a
   TYPE-pool option picker) and item 3 (Human Ethnicity placeholder picker, Oversized Goblin
   ability-pool variant) remain the only real, buildable remaining `race_trait` bucket-D work
   this lane's own reconnaissance named — both still require new mechanisms out of this
   cycle's scope.
