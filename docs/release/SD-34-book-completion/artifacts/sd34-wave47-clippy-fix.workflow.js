export const meta = {
  name: 'sd34-wave47-clippy-fix',
  description: 'Fix 3 clippy collapsible_if warnings in wave 47 Divine Scion code, re-verify full suite',
  phases: [{ title: 'Build' }],
}

const buildPrompt = [
  'You are working in /home/ubuntu/workspace/repos/codex on branch tranche/14 (a PF1e rules-engine repo, SD-34 bundle). This is a shared checkout -- set CARGO_TARGET_DIR=/tmp/claude-1000/-home-ubuntu-workspace-repos-codex-docs-release-SD-33-computed-value-verification/f34e35e3-6405-4a87-b612-555220a48600/scratchpad/cargo-target-wave47-clippy-fix for every cargo command you run (export it once at the start of your session).',
  '',
  'CONTEXT: wave 47 (commits 24666d0667 and fa356e6db7, already on tranche/14) added Divine Scion choice-gating code to src/rules_core/pilot_compute/mod.rs. This bundle\'s own verify.sh gate enforces a ZERO clippy warning ceiling, and a full isolated verify.sh run just found 3 new `collapsible_if` warnings introduced by that wave, all in the same function:',
  '',
  '1. mod.rs:36255 -- a nested `if let Some(dr) = divine_scion_opposition_alignment_dr(level) { if let Some(selection) = choice_selection(...) { ... } }` that clippy wants collapsed into a single `if let ... && let ...` chain.',
  '2. mod.rs:36313 -- `if level >= 3 { if let Some(selection) = choice_selection(input, DIVINE_SCION_DOMAIN_SPECIALIZATION_CHOICE_ID) { ... } }` similarly collapsible.',
  '3. mod.rs:36314 -- a nested `if let Some(selection) = choice_selection(...) { if let Some((slug, display, uses_per_day)) = DIVINE_SCION_DOMAIN_SPECIALIZATION_USES_PER_DAY... { ... } }` also collapsible.',
  '',
  'YOUR JOB:',
  '',
  '1. Run `cargo clippy --locked --tests -j 6 2>&1 | grep -A 20 "collapsible_if"` first to see the exact current warnings yourself (line numbers may have shifted slightly from the report above -- do not assume they are still exact).',
  '2. Fix ALL THREE by collapsing the nested `if let` into `if let ... && let ...` chains (Rust\'s let-chains, already used elsewhere in this codebase per clippy\'s own suggestion) -- this is a pure control-flow simplification with IDENTICAL runtime behavior, not a logic change. Read enough surrounding context to make sure the collapse is correct (matching braces, no accidentally-dropped statements) -- do not use `cargo clippy --fix` blindly without reviewing the diff it produces, since auto-fix can occasionally reformat more than intended.',
  '3. Run `cargo clippy --locked --tests -j 6` again and confirm ZERO warnings for the root crate (this is the whole point -- verify.sh enforces a ceiling of 0).',
  '4. Run `cargo test --locked --lib -j 6` AND the FULL `cargo test --locked --no-fail-fast -j 6` integration suite. Both must pass clean -- this is a pure refactor so nothing should change, but prove it: the lib suite should show the same 3134 passed as before this fix, and the full suite should show 8562 passed across 589 suites (wave 47\'s own established count) or explain any delta honestly.',
  '5. Also run `cargo clippy --locked --tests -j 6` for `apps/desktop/src-tauri` (the desktop crate) just to confirm it is still at its own 0-warning ceiling too (should be unaffected, but confirm).',
  '6. Commit this as ONE small commit on `tranche/14` (e.g. "fix(sd34): wave 47 -- collapse 3 clippy-flagged nested if-lets in Divine Scion code, no behavior change"). Do NOT push -- the orchestrator handles the final gate and merge.',
  '',
  'Report back: exact commit SHA, confirmation clippy shows 0 warnings on both crates, confirmation both the lib and full integration suites pass with their expected counts.',
].join('\n')

const build = await agent(buildPrompt, {
  label: 'fix-wave47-clippy',
  phase: 'Build',
  model: 'claude-sonnet-5',
})

return { buildResult: build }
