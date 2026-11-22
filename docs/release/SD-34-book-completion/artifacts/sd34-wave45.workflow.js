export const meta = {
  name: 'sd34-wave45',
  description: 'Wave 45: registered-prestige-class magnitude formulas from sub-mechanism-5 remainder',
  phases: [{ title: 'Build' }],
}

const buildPrompt = [
  'You are working in /home/ubuntu/workspace/repos/codex on branch tranche/14 (a PF1e rules-engine repo, SD-34 bundle, docs/release/SD-34-book-completion/). This is a shared checkout -- set CARGO_TARGET_DIR=/tmp/claude-1000/-home-ubuntu-workspace-repos-codex-docs-release-SD-33-computed-value-verification/f34e35e3-6405-4a87-b612-555220a48600/scratchpad/cargo-target-wave45 for every cargo command you run (export it once at the start of your session).',
  '',
  'REQUIRED READING FIRST: docs/release/SD-34-book-completion/decisions.md section 22 in full (the WAVE 43 UPDATE and WAVE 44 UPDATE at the end are the most important -- they scope this wave) and docs/release/SD-34-book-completion/progress.md\'s two most recent entries ("Wave 44 wave-end gate" and "Wave 44") for house style and process lessons.',
  '',
  'CRITICAL LESSON FROM WAVE 44 (apply throughout): wave 43\'s own audit made a "real owner" claim for 4 classifier-collision units, and 3 of those 4 turned out to be WRONG when wave 44 actually read the corpus records. Do NOT trust any prior wave\'s prose characterization of a unit\'s owner, formula shape, or "already wired" status -- always read the actual corpus record (`data/corpus/<book>/**/*.json`, cross-referenced with the real `.lst` source file cited in `docs/work-inventory.json`) yourself before writing any code. This is not optional caution, it is this bundle\'s own proven failure mode, twice now.',
  '',
  'SCOPE: sub-mechanism-5 (`class_feature_of_unmodelled_corpus_class`, evidence code emitted at `src/bin/v06_work_inventory.rs`\'s `classify()` function -- search for the literal string to find the current line number, it has shifted since wave 44\'s own edits). Wave 44 fixed the real bug in `scripts/census_prestige_class_entry_requirements.py` that was blocking 191 units, and closed 16 units of its own. decisions.md names the remaining population as still containing units that are (a) now-registered prestige classes needing only a per-feature magnitude formula (the wave-42/43/44 pattern: `chassis_supported(...) || prestige_class_entry_gate::is_registered(...)` already grants the fact at the right level via `class_feature_grant_consumer::push_generic_class_feature_grant_records`, only the magnitude is missing), or (b) still genuinely unaddressed for other reasons.',
  '',
  'YOUR JOB, in order:',
  '',
  '1. Re-derive the CURRENT sub-mechanism-5 population fresh -- do not trust any wave\'s prior count (634, 699, or any other number named in decisions.md) since the registry fixture and classifier have both changed since those were written. Query `docs/work-inventory.json` directly for units carrying that evidence code today.',
  '2. Cross-reference every one of those units against the NOW-74-entry `tests/fixtures/rules_core/prestige-class-entry-requirements.json` registry fixture. Identify which units belong to a class that IS registered there (meaning the generic grant consumer already fires and only a magnitude formula is missing) versus units whose class is NOT registered (out of scope for this wave -- name them, don\'t attempt them).',
  '3. For the registered-and-magnitude-missing set: read each one\'s real corpus record directly (never trust a summary), find the literal formula token (BONUS:/DEFINE:/etc.), and check for an existing precedented compute function of the same shape elsewhere in `src/rules_core/pilot_compute/mod.rs` (this bundle has now built dozens of these -- companion progressions, order/discipline pools, favored-enemy-style choices, `bonus_pp_level`-style manifester-point totals, sneak-attack-dice-by-level, save-DC "10 + level-factor + ability modifier" idioms, etc.). Pick a batch you can close CORRECTLY and VERIFIABLY this cycle -- there is no fixed target count; better to close 15 units you have personally verified against the real corpus than to rush 40 and get some wrong. Use your own judgment on how large a batch is safely verifiable in one cycle, informed by wave 43\'s (12 units, ~97 min) and wave 44\'s (16 units, longer due to the 2 extra investigation pieces) own pace.',
  '4. Skip (name, do not attempt) anything that needs genuinely new subsystem modeling with no precedent, or where the corpus record itself is ambiguous/contradicts its own DESC prose in a way you cannot resolve by reading the raw token directly (this bundle\'s own "authoritative token over DESC prose" ruling, precedent: `warpriest_channel_energy_dc`, and this wave\'s own wave-44 Shadow Illusion/Shadow Jump precedent -- when DESC and the literal token disagree, the literal token wins, transcribe it, and note the discrepancy in your receipt).',
  '5. Write a pure-formula test for every new compute function (direct formula tests covering edge cases, PLUS a real-pipeline reachability test proving the classifier/dispatch chain actually resolves each record end-to-end -- follow `wave43_prestige_class_new_compute_tests` or `wave44`\'s own test modules for the established pattern in this bundle).',
  '',
  '=== VERIFICATION (mandatory, not optional) ===',
  '',
  '1. Run `cargo test --locked --lib -j 6` AND the FULL `cargo test --locked --no-fail-fast -j 6` integration suite. Both must pass clean. If you make ANY code change after your first full-suite run (including a fix for something the first run caught), you MUST re-run the full suite a SECOND time end-to-end before claiming done -- do not just re-verify via a lib-only pass. This exact shortcut is what left a gap in wave 44\'s own cycle that the orchestrator had to close separately.',
  '2. If your changes touch anything F1-shaped (a bare-literal magnitude, no per-level/ability/pool expression) or otherwise change the corpus-wide formula-family population, re-derive the pinned census test in `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs` via `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus` and update the pin following its own established doc-comment convention BEFORE the guarded regen, not after.',
  '3. Guarded regen: run `cargo run --locked --bin v06_work_inventory` (it will refuse until `CORPUS_LITERAL_SWEEP_REPORT` and `DERIVED_FIXTURE_CHECK_REPORT` prerequisites are generated fresh -- generate both first).',
  '4. Before/after: `python3 scripts/completion_atlas.py --check` on both snapshots, plus a direct Python id->status join over the specific units you targeted, confirming exactly the units you intended moved and nothing else did (zero collateral movement). Report the real DONE/D/B/V bucket deltas.',
  '5. Run `python3 scripts/denominator_gate.py --check` directly before finishing -- if you write any percentage in your receipt or progress.md entry, it MUST carry its denominator on the same line (`decisions.md`\'s own discipline, and the exact thing the orchestrator\'s own prior-wave prose failed this same check on).',
  '6. Update `scripts/verify-baselines.env` if lib/full test counts moved (raise only, following the file\'s own dated-comment-block convention -- current baseline is `BASELINE_ROOT_LIB_TESTS=3090` / `BASELINE_ROOT_FULL_TESTS=8495`).',
  '7. Write a cycle receipt at `docs/release/SD-34-book-completion/artifacts/bucket-d-mining/wave45_registered_prestige_magnitude_formulas_cycle_receipt.md` (follow the format of the wave 43/44 receipts in the same directory).',
  '8. Prepend a cycle entry to `docs/release/SD-34-book-completion/progress.md` (house style -- see wave 44\'s own entry for the pattern) and update `kanban.md` if this bundle uses one.',
  '9. Add a dated update to `docs/release/SD-34-book-completion/decisions.md` §22 (append at the end, "WAVE 45 UPDATE, <date>: <summary>") documenting the fresh sub-mechanism-5 population re-derivation, what you closed, what you skipped and why, and what is genuinely left for a future wave.',
  '10. Commit your work on `tranche/14` in as few commits as make sense (a feat commit for the code+tests, a docs commit for any receipt-SHA fill-in, following this bundle\'s own established two-commit pattern from waves 41-44). Do NOT push -- the orchestrator runs the final isolated verify.sh gate and handles the merge.',
  '',
  'Report back: exact commit SHA(s), the fresh sub-mechanism-5 population count you derived (with its own source/method cited), units closed (id list with before/after bucket), confirmation the FULL integration suite passed (paste actual pass/fail counts), whether the F1/shape_ledger pin needed updating and by how much, and what remains named for a future wave.',
].join('\n')

const build = await agent(buildPrompt, {
  label: 'build-wave45-registered-prestige-magnitude',
  phase: 'Build',
  model: 'claude-sonnet-5',
})

return { buildResult: build }
