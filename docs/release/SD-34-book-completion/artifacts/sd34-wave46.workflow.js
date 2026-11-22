export const meta = {
  name: 'sd34-wave46',
  description: 'Wave 46: registered-prestige-class magnitude formulas from sub-mechanism-5 remainder (566 units)',
  phases: [{ title: 'Build' }],
}

const buildPrompt = [
  'You are working in /home/ubuntu/workspace/repos/codex on branch tranche/14 (a PF1e rules-engine repo, SD-34 bundle, docs/release/SD-34-book-completion/). This is a shared checkout -- set CARGO_TARGET_DIR=/tmp/claude-1000/-home-ubuntu-workspace-repos-codex-docs-release-SD-33-computed-value-verification/f34e35e3-6405-4a87-b612-555220a48600/scratchpad/cargo-target-wave46 for every cargo command you run (export it once at the start of your session).',
  '',
  'REQUIRED READING FIRST: docs/release/SD-34-book-completion/decisions.md section 22 in full (the WAVE 43/44/45 UPDATEs at the end are the most important -- they scope this wave) and docs/release/SD-34-book-completion/progress.md\'s two most recent entries ("Wave 45 wave-end gate" and "Wave 45") for house style and process lessons.',
  '',
  'CRITICAL LESSONS FROM THIS BUNDLE\'S OWN HISTORY (apply throughout, not optional caution):',
  '- Wave 43\'s own audit made 4 classifier-collision "real owner" claims; 3 of the 4 turned out WRONG when wave 44 actually read the corpus records. NEVER trust any prior wave\'s prose characterization of a unit\'s owner, formula shape, or "already wired" status without reading the actual corpus record yourself first.',
  '- Wave 44\'s own cycle ran its full integration suite once, hit a failure, fixed it, but never re-ran the full suite end-to-end afterward -- the orchestrator had to close that gap separately. Wave 45 avoided this by re-running the FULL suite a second time itself. Do the same: if you make ANY code change after your first full-suite pass (including a fix for something that pass caught), re-run the FULL suite a second time end-to-end, not just a lib-only recheck.',
  '- Wave 45 caught its own miscount (31 vs 30 creature-type records) via its own test\'s first assertion failing for the right reason (RED before GREEN) -- write your own tests to actually verify counts/populations against real files, don\'t just assert what you expect.',
  '',
  'SCOPE: sub-mechanism-5 (`class_feature_of_unmodelled_corpus_class`, evidence code emitted at `src/bin/v06_work_inventory.rs`\'s `classify()` function -- search for the literal string to find the current line number, it shifts every wave). Wave 45 re-derived the population fresh at 686 total, split 598 registered against the 74-entry prestige-class registry / 88 not-registered (named by slug in decisions.md -- do not attempt those, they are out of scope: psychic_detective 18, eidolon 16, animal 17, phantom 9, plant 9, undead 8, dragon 8, gifted_blade 3). Wave 45 closed 32 of the 598 registered units (Phrenic Slayer\'s Favored Enemy). This wave targets the remaining ~566 registered units, spread across dozens of OTHER prestige classes.',
  '',
  'YOUR JOB, in order:',
  '',
  '1. Re-derive the CURRENT sub-mechanism-5 population fresh -- do not trust wave 45\'s own 686/598/88 numbers at face value; the registry fixture and corpus can shift between waves. Query `docs/work-inventory.json` directly for units carrying that evidence code today, cross-reference against `tests/fixtures/rules_core/prestige-class-entry-requirements.json` (currently 74 entries).',
  '2. Group the registered-and-magnitude-missing units by their owning prestige class (NOT Phrenic Slayer -- that class\'s Favored Enemy is done; its own remaining 11 units, per decisions.md, key off cross-class prime-stat variables and are a separate, harder subsystem question -- skip those too unless you find a clean precedented path, and if you do attempt any of them, verify extremely carefully since this is exactly the shape wave 45 itself flagged as NOT simple).',
  '3. Pick 2-4 prestige classes whose remaining units look cheapest/most precedented (same formula shapes already built repeatedly this bundle: favored-enemy-style choices, companion/order/discipline progressions, `bonus_pp_level`-style manifester-point totals, sneak-attack-dice-by-level, save-DC "10 + level-factor + ability modifier" idioms, etc.) and close as many of their units as you can safely, correctly verify this cycle. No fixed target count -- prioritize correctness over volume, but this population has now proven cheap and precedented across 4 consecutive waves (42-45), so a reasonably large batch (comparable to or larger than wave 45\'s own 32) is a realistic goal if the classes you pick are genuinely simple.',
  '4. For EVERY unit: read its real corpus record directly (never trust a summary or a prior wave\'s characterization), find the literal formula token (BONUS:/DEFINE:/etc.), and check for an existing precedented compute function of the same shape in `src/rules_core/pilot_compute/mod.rs` before writing new code.',
  '5. Skip (name, do not attempt) anything needing genuinely new subsystem modeling with no precedent, or where the corpus record contradicts its own DESC prose in a way you cannot resolve by reading the raw token directly (authoritative-token-over-DESC-prose ruling, precedent: `warpriest_channel_energy_dc`, Shadow Illusion/Shadow Jump from wave 44).',
  '6. Write a pure-formula test for every new compute function (direct formula tests covering edge cases, PLUS a real-pipeline reachability test proving the classifier/dispatch chain resolves each record end-to-end -- follow `wave43_prestige_class_new_compute_tests`/wave 44/wave 45\'s own test modules for the established pattern).',
  '',
  '=== VERIFICATION (mandatory) ===',
  '',
  '1. Run `cargo test --locked --lib -j 6` AND the FULL `cargo test --locked --no-fail-fast -j 6` integration suite. Both must pass clean. If you make ANY code change after your first full-suite run, re-run the full suite a SECOND time end-to-end before claiming done.',
  '2. If your changes touch anything F1-shaped (a bare-literal magnitude, no per-level/ability/pool expression) or otherwise change the corpus-wide formula-family population, re-derive the pinned census test in `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs` via `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus` and update the pin following its own established doc-comment convention BEFORE the guarded regen.',
  '3. Guarded regen: run `cargo run --locked --bin v06_work_inventory` (generate `CORPUS_LITERAL_SWEEP_REPORT` and `DERIVED_FIXTURE_CHECK_REPORT` prerequisites first).',
  '4. Before/after: `python3 scripts/completion_atlas.py --check` on both snapshots, plus a direct Python id->status join over the specific units you targeted, confirming exactly the units you intended moved and nothing else did (zero collateral movement). Report the real DONE/D/B/V bucket deltas.',
  '5. Run `python3 scripts/denominator_gate.py --check` directly before finishing -- any percentage in your receipt or progress.md entry MUST carry its denominator on the same line.',
  '6. Update `scripts/verify-baselines.env` if lib/full test counts moved (raise only, following the file\'s own dated-comment-block convention -- current baseline is `BASELINE_ROOT_LIB_TESTS=3095` / `BASELINE_ROOT_FULL_TESTS=8504`).',
  '7. Write a cycle receipt at `docs/release/SD-34-book-completion/artifacts/bucket-d-mining/wave46_registered_prestige_magnitude_formulas_cycle_receipt.md` (follow the format of the wave 43/44/45 receipts in the same directory).',
  '8. Prepend a cycle entry to `docs/release/SD-34-book-completion/progress.md` (house style) and update `kanban.md` if this bundle uses one.',
  '9. Add a dated update to `docs/release/SD-34-book-completion/decisions.md` §22 (append at the end, "WAVE 46 UPDATE, <date>: <summary>") documenting the fresh sub-mechanism-5 population re-derivation, which classes you closed, what you skipped and why, and what is genuinely left for a future wave.',
  '10. Commit your work on `tranche/14` in as few commits as make sense (a feat commit for the code+tests, a docs commit for any receipt-SHA fill-in, following this bundle\'s own established two-commit pattern). Do NOT push -- the orchestrator runs the final isolated verify.sh gate and handles the merge.',
  '',
  'NOTE ON SHARED CHECKOUT: another concurrent actor may be running verify.sh/reclaim.sh on this same shared checkout during your cycle (this happened during wave 45) -- if you see unexpected changes to files like docs/retro/events/*.jsonl that you did not touch, leave them uncommitted (this bundle\'s own commits never touch those files).',
  '',
  'Report back: exact commit SHA(s), the fresh sub-mechanism-5 population count you derived, which prestige classes you closed and how many units each, units closed (id list with before/after bucket), confirmation the FULL integration suite passed TWICE if you made post-first-run fixes (paste actual pass/fail counts), whether the F1/shape_ledger pin needed updating, and what remains named for a future wave.',
].join('\n')

const build = await agent(buildPrompt, {
  label: 'build-wave46-registered-prestige-magnitude',
  phase: 'Build',
  model: 'claude-sonnet-5',
})

return { buildResult: build }
