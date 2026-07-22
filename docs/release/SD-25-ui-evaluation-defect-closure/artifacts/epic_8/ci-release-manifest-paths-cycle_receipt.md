# Cycle ci-release-manifest-paths — Epic 8 Closure Epilogue / CI regression fix
- **Card ID:** n/a (operator-dispatched direct fix, not a hermes kanban cycle)
- **Commit SHA:** a2a1072 (pushed to `tranche/5-3`)
- **Files touched:** `.github/workflows/check-release-manifest.yml`
- **Identifier audit result:** OK_NO_BUNDLE_TAGS — verified no residual `sd11`/`sd15` references remain in the file after the edit (`grep -n "sd11\|sd15"` returns no matches); the two replacement path entries (`apps/desktop/src/testerWorkbench/update/**`, `apps/desktop/src/operatorTriage/**`) match real on-disk directories confirmed present via `ls`.
- **Wired-integration audit result:** OK_NO_TOKENS / OK_NO_NOOP_HANDLERS / OK_NO_MOCK_LEAKS / OK_NO_WOULD_STRINGS — CI path-filter string change only, no handlers/tokens/mocks in scope.
- **Acceptance criterion:** (ad hoc fix, not a numbered epic-breakdown criterion) — `check-release-manifest.yml`'s `on.pull_request.paths` filter must reference the real, current directory names so the release-manifest-binding gate still fires on PRs touching the renamed SD-25 criterion 1.1 directories.
- **Status:** complete
- **Notes:**
  - Root cause: SD-25 criterion 1.1 (identifier-cleanup) renamed `apps/desktop/src/sd11/` → `apps/desktop/src/testerWorkbench/` and `apps/desktop/src/sd15/` → `apps/desktop/src/operatorTriage/`, but `.github/workflows/check-release-manifest.yml` was outside 1.1's file-touch grant and was never updated. Its `paths` glob still referenced the pre-rename directories, so PRs touching the renamed (now-real) directories silently stopped triggering the release-manifest-binding gate — a live CI regression.
  - Verified before editing: `apps/desktop/src/sd11/` and `apps/desktop/src/sd15/` do not exist on disk (confirmed via `ls`); `apps/desktop/src/testerWorkbench/update/` and `apps/desktop/src/operatorTriage/` do exist and contain the migrated code (`deriveWorkbenchUpdateAction.ts`, `updateActionModel.ts`, etc. under `testerWorkbench/update/`; `buildOperatorTriageDraft.ts` under `operatorTriage/`).
  - Deliberately left untouched: `apps/desktop/src/sd16/**` — `sd16/` still legitimately exists on disk (not renamed by 1.1). `apps/desktop/src/sd17/**` — `sd17/` does not exist, but predates SD-25 and is unrelated pre-existing staleness outside this fix's scope; no evidence of what (if anything) it should be renamed to, so left alone rather than guessed at.
  - `git diff` confirmed only the two intended lines changed (verified below); YAML re-parsed successfully post-edit via `python3 -c "import yaml; yaml.safe_load(...)"` → `YAML OK`.
  - Diff:
    ```diff
    -      - 'apps/desktop/src/sd11/update/**'
    -      - 'apps/desktop/src/sd15/**'
    +      - 'apps/desktop/src/testerWorkbench/update/**'
    +      - 'apps/desktop/src/operatorTriage/**'
    ```
- **Discovery forwards:** none new. (Optionally flags `apps/desktop/src/sd17/**` staleness for a future carry-forward register entry if anyone wants to investigate what it should point to, or whether it's dead weight to remove — out of scope here.)
- **Next-cycle plan:** none required; this was a standalone CI-regression fix dispatched directly, not part of an ongoing epic cycle sequence.
