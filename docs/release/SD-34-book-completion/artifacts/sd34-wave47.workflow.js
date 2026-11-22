export const meta = {
  name: 'sd34-wave47',
  description: 'Wave 47: registered-prestige-class magnitude formulas from sub-mechanism-5 remainder (546 units, excluding cross-class-manifester-level group)',
  phases: [{ title: 'Build' }],
}

const buildPrompt = [
  'You are working in /home/ubuntu/workspace/repos/codex on branch tranche/14 (a PF1e rules-engine repo, SD-34 bundle, docs/release/SD-34-book-completion/). This is a shared checkout -- set CARGO_TARGET_DIR=/tmp/claude-1000/-home-ubuntu-workspace-repos-codex-docs-release-SD-33-computed-value-verification/f34e35e3-6405-4a87-b612-555220a48600/scratchpad/cargo-target-wave47 for every cargo command you run (export it once at the start of your session).',
  '',
  'REQUIRED READING FIRST: docs/release/SD-34-book-completion/decisions.md section 22 in full (the WAVE 44/45/46 UPDATEs at the end are the most important -- they scope this wave) and docs/release/SD-34-book-completion/progress.md\'s two most recent entries ("Wave 46 wave-end gate" and "Wave 46") for house style and process lessons.',
  '',
  'CRITICAL LESSONS FROM THIS BUNDLE\'S OWN HISTORY (apply throughout, not optional caution):',
  '- Wave 43\'s own audit made 4 classifier-collision "real owner" claims; 3 of the 4 turned out WRONG when wave 44 actually read the corpus records. NEVER trust any prior wave\'s prose characterization of a unit\'s owner, formula shape, or "already wired" status without reading the actual corpus record yourself first.',
  '- Wave 44\'s own cycle ran its full integration suite once, hit a failure, fixed it, but never re-ran the full suite end-to-end afterward -- the orchestrator had to close that gap separately. Waves 45 and 46 avoided this by re-running the FULL suite again (wave 46 even ran it to completion TWICE identically after a mid-cycle fix). Do the same: if you make ANY code change after your first full-suite pass, re-run the FULL suite a SECOND time end-to-end, not just a lib-only recheck.',
  '- Wave 45 and 46 both caught their own miscounts via their own tests\' first assertion failing for the right reason (RED before GREEN) -- write your own tests to actually verify counts/populations against real files, don\'t just assert what you expect.',
  '- Wave 46\'s own audit found that at least 9 Ultimate Psionics prestige classes (Sighted Seeker, Thrallherd, Psion Uncarnate, Cerebremancer, Metamind, Elocater, Psicrystal Imprinter, Soul Archer, Metaforge) share the IDENTICAL cross-class-manifester-level shape already excluded for Phrenic Slayer. **Skip ALL of these classes this wave** -- they are one harder subsystem question, deliberately out of scope here, not individually re-litigated.',
  '',
  'SCOPE: sub-mechanism-5 (`class_feature_of_unmodelled_corpus_class`, evidence code emitted at `src/bin/v06_work_inventory.rs`\'s `classify()` function -- search for the literal string to find the current line number, it shifts every wave). Wave 46 re-derived the population fresh at 654, closed 20 across 7 classes, leaving 634 (546 registered across 55 classes / 88 not-registered). This wave targets the remaining ~546 registered units, EXCLUDING Phrenic Slayer\'s own remainder and the 9+ sibling Ultimate Psionics classes named above (all cross-class-manifester-level shaped, one subsystem question, out of scope this wave).',
  '',
  'YOUR JOB, in order:',
  '',
  '1. Re-derive the CURRENT sub-mechanism-5 population fresh -- do not trust wave 46\'s own 654/546/88 numbers at face value; the registry fixture and corpus can shift between waves. Query `docs/work-inventory.json` directly for units carrying that evidence code today, cross-reference against `tests/fixtures/rules_core/prestige-class-entry-requirements.json` (currently 74 entries).',
  '2. Group the registered-and-magnitude-missing units by their owning prestige class, EXCLUDING Phrenic Slayer and the 9+ named cross-class-manifester-level classes above (verify each excluded class really does carry that shape by a quick direct read before skipping it wholesale -- don\'t just trust the name match; if any of the 9 named classes turns out NOT to actually be cross-class-manifester-level shaped on closer inspection, that is fair game to close this wave, but name the correction explicitly if you find one).',
  '3. Pick 2-5 prestige classes whose remaining units look cheapest/most precedented (same formula shapes already built repeatedly this bundle: favored-enemy-style choices, companion/order/discipline progressions, `bonus_pp_level`-style manifester-point totals, sneak-attack-dice-by-level, save-DC "10 + level-factor + ability modifier" idioms, etc.). Divine Scion (45 units, the largest remaining class) is a candidate if its own features look tractable on inspection -- read a sample of its actual records before committing to it as a target, since a large unfamiliar class could also turn out to need new subsystem modeling. Close as many units as you can safely, correctly verify this cycle -- no fixed target count, prioritize correctness over volume.',
  '4. For EVERY unit: read its real corpus record directly (never trust a summary or a prior wave\'s characterization), find the literal formula token (BONUS:/DEFINE:/etc.), and check for an existing precedented compute function of the same shape in `src/rules_core/pilot_compute/mod.rs` before writing new code.',
  '5. Skip (name, do not attempt) anything needing genuinely new subsystem modeling with no precedent, or where the corpus record contradicts its own DESC prose in a way you cannot resolve by reading the raw token directly (authoritative-token-over-DESC-prose ruling, precedent: `warpriest_channel_energy_dc`).',
  '6. Write a pure-formula test for every new compute function (direct formula tests covering edge cases, PLUS a real-pipeline reachability test proving the classifier/dispatch chain resolves each record end-to-end -- follow wave 43-46\'s own test modules for the established pattern).',
  '',
  '=== VERIFICATION (mandatory) ===',
  '',
  '1. Run `cargo test --locked --lib -j 6` AND the FULL `cargo test --locked --no-fail-fast -j 6` integration suite. Both must pass clean. If you make ANY code change after your first full-suite run, re-run the full suite a SECOND time end-to-end before claiming done.',
  '2. If your changes touch anything F1-shaped (a bare-literal magnitude, no per-level/ability/pool expression) or otherwise change the corpus-wide formula-family population, re-derive the pinned census test in `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs` via `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus` and update the pin following its own established doc-comment convention BEFORE the guarded regen.',
  '3. Guarded regen: run `cargo run --locked --bin v06_work_inventory` (generate `CORPUS_LITERAL_SWEEP_REPORT` and `DERIVED_FIXTURE_CHECK_REPORT` prerequisites first).',
  '4. Before/after: `python3 scripts/completion_atlas.py --check` on both snapshots, plus a direct Python id->status join over the specific units you targeted, confirming exactly the units you intended moved and nothing else did (zero collateral movement). Report the real DONE/D/B/V bucket deltas.',
  '5. Run `python3 scripts/denominator_gate.py --check` directly before finishing -- any percentage in your receipt or progress.md entry MUST carry its denominator on the same line.',
  '6. Update `scripts/verify-baselines.env` if lib/full test counts moved (raise only, following the file\'s own dated-comment-block convention -- current baseline is `BASELINE_ROOT_LIB_TESTS=3121` / `BASELINE_ROOT_FULL_TESTS=8545`).',
  '7. Write a cycle receipt at `docs/release/SD-34-book-completion/artifacts/bucket-d-mining/wave47_registered_prestige_magnitude_formulas_cycle_receipt.md` (follow the format of the wave 43-46 receipts in the same directory).',
  '8. Prepend a cycle entry to `docs/release/SD-34-book-completion/progress.md` (house style) and update `kanban.md` if this bundle uses one.',
  '9. Add a dated update to `docs/release/SD-34-book-completion/decisions.md` §22 (append at the end, "WAVE 47 UPDATE, <date>: <summary>") documenting the fresh sub-mechanism-5 population re-derivation, which classes you closed, what you skipped and why, and what is genuinely left for a future wave.',
  '10. Commit your work on `tranche/14` in as few commits as make sense (a feat commit for the code+tests, a docs commit for any receipt-SHA fill-in, following this bundle\'s own established two-commit pattern). Do NOT push -- the orchestrator runs the final isolated verify.sh gate and handles the merge.',
  '',
  'NOTE ON SHARED CHECKOUT: another concurrent actor may be running verify.sh/reclaim.sh on this same shared checkout during your cycle -- if you see unexpected changes to files like docs/retro/events/*.jsonl that you did not touch, leave them uncommitted.',
  '',
  'Report back: exact commit SHA(s), the fresh sub-mechanism-5 population count you derived, which prestige classes you closed and how many units each, units closed (id list with before/after bucket), confirmation the FULL integration suite passed (paste actual pass/fail counts, confirm it was re-run a second time if you made post-first-run fixes), whether the F1/shape_ledger pin needed updating, and what remains named for a future wave.',
].join('\n')

const build = await agent(buildPrompt, {
  label: 'build-wave47-registered-prestige-magnitude',
  phase: 'Build',
  model: 'claude-sonnet-5',
})

return { buildResult: build }
