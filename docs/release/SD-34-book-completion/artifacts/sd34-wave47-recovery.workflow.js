export const meta = {
  name: 'sd34-wave47-recovery',
  description: 'Wave 47 recovery: finish and verify uncommitted Divine Scion work left by a stalled prior agent run',
  phases: [{ title: 'Build' }],
}

const buildPrompt = [
  'You are working in /home/ubuntu/workspace/repos/codex on branch tranche/14 (a PF1e rules-engine repo, SD-34 bundle, docs/release/SD-34-book-completion/). This is a shared checkout -- set CARGO_TARGET_DIR=/tmp/claude-1000/-home-ubuntu-workspace-repos-codex-docs-release-SD-33-computed-value-verification/f34e35e3-6405-4a87-b612-555220a48600/scratchpad/cargo-target-wave47-recovery for every cargo command you run (export it once at the start of your session).',
  '',
  'CONTEXT: A prior "wave 47" build agent (script sd34-wave47.workflow.js) was dispatched with a scope of closing registered-prestige-class magnitude formulas from the sub-mechanism-5 remainder (docs/release/SD-34-book-completion/decisions.md section 22, WAVE 43-46 UPDATEs). It ran for ~48 minutes and made real, substantial progress on Divine Scion (a 45-unit prestige class from inner_sea_magic) -- it added `probe_divine_scion_wiring` and a `divine_scion_wired` field to `src/bin/v06_work_inventory.rs`, and a large `ground_divine_scion_class_features` implementation plus a full test module to `src/rules_core/pilot_compute/mod.rs` (Domain Specialization pool size, Divine Wrath, Deific Defense, Weapon and Armor Proficiency, four Opposition Alignment DR records, and 35 per-domain Domain Specialization sub-records with per-domain uses-per-day tables). Then it stalled -- its final message was a garbled non-report ("I\'ll stop issuing further commands now and wait for the monitor notifications to arrive"), and it never committed, never wrote a receipt, never touched progress.md/decisions.md, and never ran final verification. NONE of this uncommitted work should be assumed correct just because it looks complete -- verify it from scratch yourself, the same distrust this bundle applies to every prior wave\'s own self-report.',
  '',
  'REQUIRED READING FIRST: `git diff -- src/bin/v06_work_inventory.rs src/rules_core/pilot_compute/mod.rs` to see the FULL uncommitted diff yourself (do not just trust this prompt\'s summary of it). Also read docs/release/SD-34-book-completion/decisions.md section 22 in full (WAVE 44/45/46 UPDATEs especially) and progress.md\'s two most recent entries ("Wave 46 wave-end gate" and "Wave 46") for house style.',
  '',
  'CRITICAL LESSONS FROM THIS BUNDLE\'S OWN HISTORY (apply throughout):',
  '- Wave 43\'s own audit made 4 classifier-collision "real owner" claims; 3 of 4 turned out WRONG when wave 44 actually read the corpus records. NEVER trust any prior wave\'s (or prior agent\'s) prose characterization of a unit\'s owner, formula shape, or "already wired" status without reading the actual corpus record yourself.',
  '- Always re-run the FULL `cargo test --locked --no-fail-fast -j 6` integration suite a SECOND time end-to-end if you make ANY code change after your first full-suite pass.',
  '- Waves 45/46 caught their own miscounts via their own tests\' first assertion failing for the right reason (RED before GREEN).',
  '- Wave 46 found 9+ Ultimate Psionics prestige classes (Sighted Seeker, Thrallherd, Psion Uncarnate, Cerebremancer, Metamind, Elocater, Psicrystal Imprinter, Soul Archer, Metaforge) share the cross-class-manifester-level shape excluded for Phrenic Slayer -- these remain OUT OF SCOPE, do not touch them. Phrenic Slayer\'s own remainder is also out of scope.',
  '',
  'YOUR JOB, in order:',
  '',
  '1. Read the full uncommitted diff. Assess whether the Divine Scion work is genuinely complete and correct: does `probe_divine_scion_wiring` actually get called/wired into the classify() dispatch chain (not just defined)? Does `ground_divine_scion_class_features` actually get dispatched from wherever class features are resolved for a real character (not just defined as a dead function)? Read the real corpus records for Divine Scion directly (data/corpus, search inner_sea_magic + divine_scion) and check every formula token (BONUS:/DEFINE:/etc.) the diff claims to model actually matches -- do not trust the code comments\' own claims about "43 members" or specific bonus values without checking at least a representative sample (Domain Specialization pool size, Divine Wrath, Deific Defense, one Opposition Alignment DR, one per-domain caster_level/uses_per_day pair) against the raw corpus token yourself.',
  '2. Fix anything wrong or incomplete you find. If the work is fundamentally sound, finish it: make sure it is actually wired end-to-end, not just sitting as unreferenced code.',
  '3. If any part of the uncommitted diff turns out to be broken beyond a quick fix, or claims something not supported by the real corpus records, revert JUST that part (do not blanket-discard good work) and document the correction honestly.',
  '4. Write a pure-formula test for every new compute function if not already present (direct formula tests covering edge cases, PLUS a real-pipeline reachability test proving the classifier/dispatch chain resolves each record end-to-end) -- the existing diff already has substantial tests, verify they are real and correctly written, not just present.',
  '',
  '=== VERIFICATION (mandatory) ===',
  '',
  '1. Run `cargo test --locked --lib -j 6` AND the FULL `cargo test --locked --no-fail-fast -j 6` integration suite. Both must pass clean. If you make ANY code change after your first full-suite run, re-run the full suite a SECOND time end-to-end before claiming done.',
  '2. If your changes touch anything F1-shaped (a bare-literal magnitude, no per-level/ability/pool expression) or otherwise change the corpus-wide formula-family population, re-derive the pinned census test in `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs` via `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus` and update the pin following its own established doc-comment convention BEFORE the guarded regen.',
  '3. Guarded regen: run `cargo run --locked --bin v06_work_inventory` (generate `CORPUS_LITERAL_SWEEP_REPORT` and `DERIVED_FIXTURE_CHECK_REPORT` prerequisites first).',
  '4. Before/after: `python3 scripts/completion_atlas.py --check` on both snapshots, plus a direct Python id->status join over the specific Divine Scion units you targeted, confirming exactly the units you intended moved and nothing else did (zero collateral movement). Report the real DONE/D/B/V bucket deltas against the wave 46 gate baseline (DONE 25419, D 2441, V 345).',
  '5. Run `python3 scripts/denominator_gate.py --check` directly before finishing -- any percentage in your receipt or progress.md entry MUST carry its denominator on the same line.',
  '6. Update `scripts/verify-baselines.env` if lib/full test counts moved (raise only, following the file\'s own dated-comment-block convention -- current baseline is `BASELINE_ROOT_LIB_TESTS=3121` / `BASELINE_ROOT_FULL_TESTS=8545`).',
  '7. Write a cycle receipt at `docs/release/SD-34-book-completion/artifacts/bucket-d-mining/wave47_registered_prestige_magnitude_formulas_cycle_receipt.md` (follow the format of the wave 43-46 receipts in the same directory) -- be honest that this wave recovered from a stalled prior run, name what you found wrong/fixed if anything.',
  '8. Prepend a cycle entry to `docs/release/SD-34-book-completion/progress.md` (house style) and update `kanban.md` if this bundle uses one.',
  '9. Add a dated update to `docs/release/SD-34-book-completion/decisions.md` §22 (append at the end, "WAVE 47 UPDATE, <date>: <summary>") documenting the fresh sub-mechanism-5 population re-derivation, what you closed (Divine Scion units, honest count), what remains named for a future wave, and the recovery context (prior agent stalled without committing, you verified and completed its work / found and fixed X).',
  '10. Commit your work on `tranche/14` in as few commits as make sense (a feat commit for the code+tests, a docs commit for any receipt-SHA fill-in). Do NOT push -- the orchestrator runs the final isolated verify.sh gate and handles the merge.',
  '',
  'NOTE ON SHARED CHECKOUT: another concurrent actor may be running verify.sh/reclaim.sh on this same shared checkout during your cycle -- if you see unexpected changes to files like docs/retro/events/*.jsonl or docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json that you did not intentionally regenerate yourself, do not blindly commit them -- check whether they are yours (from your own guarded regen) or stray, and only commit your own.',
  '',
  'Report back: exact commit SHA(s), the fresh sub-mechanism-5 population count you derived, units closed (id list with before/after bucket), confirmation the FULL integration suite passed (paste actual pass/fail counts, confirm it was re-run a second time if you made post-first-run fixes), whether the F1/shape_ledger pin needed updating, what (if anything) was wrong in the recovered work and how you fixed it, and what remains named for a future wave.',
].join('\n')

const build = await agent(buildPrompt, {
  label: 'build-wave47-recovery',
  phase: 'Build',
  model: 'claude-sonnet-5',
})

return { buildResult: build }
