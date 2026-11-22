# SD-21 release closure checklist: build version bump

This is the canonical, four-step process for stamping a new build version
across the repo before cutting or closing a release. Follow it in order;
each step depends on the previous one having landed cleanly.

The version is a `<major>.<tranche>.<build>` triple, e.g. `0.4.94`:

- **`<major>`** — increments only on a first main-publish (a promotion of the
  release out of dash/tranche status onto `main`). Stays `0` until then.
- **`<tranche>`** — increments per-tranche-promotion (moving to a new
  tranche integration branch, e.g. `tranche/4` → `tranche/5`). Dash releases
  off the same tranche (e.g. `tranche/4-1`) keep the same `<tranche>` value.
- **`<build>`** — increments per-CI-build (one per `GITHUB_RUN_NUMBER`; the
  next monotonic value after the last committed build on the integration
  branch). Never resets within a tranche.

## Step 1 — bump the version in the three repo files + the workflow stamp

Update the `"version"` field to the new triple in all three of:

- `apps/desktop/package.json`
- `apps/desktop/src-tauri/tauri.conf.json`
- `apps/desktop/src-tauri/Cargo.toml` (the top-level `version = "..."` key)

All three must carry the identical triple — they are read independently by
different tooling (npm, Tauri, Cargo) and must never drift from each other.

Then update `.github/workflows/publish-tester-release.yml`'s "Stamp build
version" step, which currently reads:

```
VERSION="0.4.${GITHUB_RUN_NUMBER}"
```

Bump the `<major>.<tranche>` prefix here too whenever it changes — this is
the publish-time stamp that overwrites `package.json` and `tauri.conf.json`
at CI publish time, so if this line is left on an old prefix, every
published/tester build will keep showing the old numbering scheme
regardless of what the repo files say. `GITHUB_RUN_NUMBER` is already the
correct monotonic, never-resets counter for the `<build>` position — do not
touch that part.

## Step 2 — build-label format check

The tester workbench surface's human-facing build label
(`apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`,
`formatSd11WorkbenchBuildLabel`) renders as `Codex <version>` (e.g.
`Codex 0.4.94`). Confirm:

- `BUILD_PREFIX` is `'Codex'` and the template is
  `` `${BUILD_PREFIX} ${buildVersion}` `` (a space, not `@`).
- The label is presentation-only — no consumer parses it as structured
  data, so a cosmetic format change here never needs a parser update
  elsewhere. Test fixtures that hard-code a sample build label (in
  `apps/desktop/src/testSupport/makeSurface.ts` and any `*.test.ts` file
  that constructs its own sample surface/status/release-truth object rather
  than going through `makeSurface`) should be refreshed to the current
  triple so example values in tests do not look stale, but this is cosmetic
  housekeeping, not a correctness requirement.

## Step 3 — `cargo check` to refresh `Cargo.lock`

Run:

```
cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml
```

`Cargo.lock` pins the workspace member's own version (`codex-desktop`); the
`cargo check` bump to `Cargo.toml` alone does not update `Cargo.lock`
automatically, so this step must run — and its diff must be included in the
same commit — or a `--locked` build will fail against a stale lockfile.

## Step 4 — commit

Commit the version-file changes, the workflow stamp change, the
build-label changes (if any), and the refreshed `Cargo.lock` together, with
a message of the shape:

```
feat(sd21): bump version to <major>.<tranche>.<build>
```

for example:

```
feat(sd21): bump version to 0.4.94
```
