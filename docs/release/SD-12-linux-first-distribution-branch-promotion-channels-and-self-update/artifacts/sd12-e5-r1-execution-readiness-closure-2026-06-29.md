# SD12-E5-R1 Execution Readiness Closure — Provenance, integrity, and update-eligibility gates

## Card outcome
- evidence_class: `blocked`
- readiness_verdict: `blocked-on-missing-provenance-emission-and-trust-gate-surfaces`
- route truth: this card closes as a documentary readiness artifact only; no PR, branch-ready commit, or merge evidence is expected from this card itself
- next board move if accepted:
  - create `SD12-E5-B1 FLOW: Resolve provenance emission and platform trust-gate execution truth`
  - create `SD12-E5-R2 FLOW: Provenance, integrity, and update-eligibility handoff artifact` gated behind that blocker repair

## Live repo truth grounded on 2026-06-29
- `git branch --show-current` reports `feat/sd11-enhancement-request-composer`
- `git rev-parse HEAD` reports `8f3a627655f490551ff23746293cde1622085e97`
- `git branch --list` shows local `develop` and `main`, but no local `uat` branch
- `git branch -r` shows `origin/develop` and `origin/main`, but no remote `origin/uat` branch
- `/home/ubuntu/workspace/repos/codex/.github/workflows/` currently contains exactly one workflow file: `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml`
- that workflow proves only a `develop -> main` pull-request gate plus `develop` branch restoration; it does not emit tester release units, checksums, provenance/build receipts, manifest records, or platform trust-state truth
- `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json` exposes only `npm run typecheck`, `npm run build`, and `npm run tauri:check`; it contains no release, checksum, provenance, signing, or updater scripts
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json` still sets `bundle.active` to `false`, so the accepted trust/update lane cannot honestly claim an already-governed packaging or updater configuration surface
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml` carries only `serde`, `serde_json`, `tauri`, and the repo-local `codex` crate; there is no updater plugin, signing/notarization, checksum, or provenance-oriented dependency surface
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/capabilities/default.json` still grants only `core:default`; there is no dedicated updater or trust-gate permission surface yet
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs` registers exactly two Tauri commands: `load_pilot_shell_snapshot` and `load_ge08_authoring_workbench_snapshot`; there is no command surface for build identity exposure, manifest/provenance retrieval, trust-state evaluation, update check, or update apply
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts` still hard-codes update truth as `state: 'not-yet-supported'` with label `Update checks not yet wired in this slice`, preserving channel/support truth but proving the update-eligibility lane is not executable repo truth yet
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts` keeps updater mechanics explicitly out of scope and must remain a read-only comparison surface for this lane
- local ignored build residue exists under `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/target/`, including package-like outputs such as `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/target/release/bundle/deb/Codex Desktop Shell Scaffold_0.0.0_amd64/control/md5sums`, but `git status --ignored -- apps/desktop/src-tauri/target` reports `!! apps/desktop/src-tauri/target/`; these are ignored local artifacts, not governed publication truth, and cannot satisfy SD-12 checksum/provenance obligations by themselves

## Actual verification run during this closure
The following commands were run successfully on the live repo during this card:

```bash
git -C /home/ubuntu/workspace/repos/codex branch --show-current
git -C /home/ubuntu/workspace/repos/codex rev-parse HEAD
git -C /home/ubuntu/workspace/repos/codex branch --list
git -C /home/ubuntu/workspace/repos/codex branch -r
git -C /home/ubuntu/workspace/repos/codex status --short --ignored -- apps/desktop/src-tauri/target
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run typecheck && npm run build && npm run tauri:check
```

Observed result:
- current working branch is `feat/sd11-enhancement-request-composer`
- current commit is `8f3a627655f490551ff23746293cde1622085e97`
- local and remote branch inventory includes `develop` and `main` but no `uat`
- ignored local build residue is present under `apps/desktop/src-tauri/target/`, so generated bundle outputs must not be mistaken for accepted release truth
- desktop verification still passes:
  - `npm run typecheck` passed
  - `npm run build` passed
  - `npm run tauri:check` passed

This is the decisive fact. The documentary contract requires official build identity, checksums, provenance/build receipts, manifest linkage, and platform trust gates before automatic update can ever be claimed. The live repo still exposes none of the governed emission or trust-decision surfaces that would make those claims executable truth.

## Exact required reads for the blocker-repair lane
The blocker-repair successor should require reading exactly these surfaces:
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/epic-breakdown.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/provenance-integrity-and-update-eligibility.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/self-update-transport-and-manifest-contract.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e2-r1-execution-readiness-closure-2026-06-29.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e3-r1-execution-readiness-closure-2026-06-29.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md`
- `/home/ubuntu/workspace/repos/codex/AGENTS.md`
- `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/capabilities/default.json`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`

## Exact candidate repo paths and allowed write scope once the blocker is repaired
This lane is narrower than `repos/codex/**`. If the missing provenance/trust execution truth is repaired, the later handoff should authorize only these exact candidate write surfaces unless a new readiness pass widens scope explicitly.

### Candidate provenance/checksum emission surfaces
1. `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml`
   - candidate future authoritative release-unit surface for emitting the bounded Linux-first tester build, checksums, provenance/build receipt, and manifest reference
   - not live today; must not be invented as already-real repo truth
   - remains blocked on the upstream E2 publication-truth repair and E3 manifest-boundary repair because provenance emission cannot outrun the publication and manifest topology that defines it

2. `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml`
   - writable only if the upstream publication-truth repair explicitly decides that promotion-governance wiring must change to align provenance emission with branch truth
   - not a grant to rewrite unrelated branch policy, repository automation, or merge doctrine

3. `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json`
   - candidate verification-script surface if the later handoff needs explicit named commands for build identity, checksum, provenance, or package/trust verification
   - not a grant for broad toolchain churn or unrelated package-manager cleanup

### Candidate platform trust-gate surfaces
4. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json`
   - candidate config surface if the accepted execution path uses Tauri-native bundling or updater configuration as part of Linux trust-gate truth
   - must not be widened into fake macOS or Windows automatic-update claims

5. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml`
   - candidate dependency surface only if a dedicated updater/trust reader or build-identity/provenance exposure path truly requires a Rust dependency
   - should stay read-only unless the later handoff names the exact dependency reason

6. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/capabilities/default.json`
   - candidate permission surface if a dedicated manifest/update or trust-state command requires explicit capability changes
   - must remain minimal and bounded; no blanket privilege widening

7. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`
   - candidate runtime surface if the accepted path requires a read-only command for build identity, eligibility, manifest/provenance retrieval, or trust-state exposure
   - should not become a hidden publication surface or UI-policy authoring surface

### Read-only comparison surfaces for this lane
These files should be read for truth preservation but are not part of the first expected write scope:
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/target/**` as ignored local output only; never as an authority surface

### Explicit scope boundary
If the later handoff discovers that provenance emission or trust-gate enforcement cannot be implemented inside the surfaces above, it must stop and route back through a new readiness closure before touching broader surfaces such as:
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/**` beyond the exact files named above
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/**` beyond the exact files named above
- `/home/ubuntu/workspace/repos/codex/src/**`
- feedback-intake or issue-payload wiring that belongs to SD-11 / SD12-E6
- rollback/withdrawal execution surfaces that belong to SD12-E4
- broad signing/notarization infrastructure for macOS or Windows that would widen platform scope beyond this Linux-first tranche

## Exact non-goals for the later provenance/trust lane
The next handoff must state these non-goals plainly:
- no treating ignored local bundle residue in `apps/desktop/src-tauri/target/**` as official tester release truth
- no silent invention of a `uat` branch, publication workflow, or manifest producer that does not exist in live repo truth
- no broadening of SD-11 tester-facing status/UI wording; E5 does not own the presentation contract
- no raw `develop`, `uat`, or `main` branch names as the primary tester-facing update UX
- no automatic-update claim for Linux, macOS, or Windows until release-unit truth, manifest linkage, checksum/provenance publication, and recovery posture all exist explicitly
- no macOS signing/notarization or Windows trust work unless a later exact handoff explicitly widens the tranche to those platform thresholds
- no rollback, downgrade, withdrawal, or manifest-consumer implementation beyond what is minimally required to preserve accepted provenance/update-eligibility truth
- no PR, merge, or branch-ready evidence from this documentary readiness card itself

## Exact verification surfaces for the blocker-repair lane and later handoff
Required branch/workflow verification surfaces:

```bash
git -C /home/ubuntu/workspace/repos/codex branch --list
git -C /home/ubuntu/workspace/repos/codex branch -r
```

Required build-residue truth check:

```bash
git -C /home/ubuntu/workspace/repos/codex status --short --ignored -- apps/desktop/src-tauri/target
```

Required desktop verification commands that must remain available for the eventual handoff:

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run typecheck
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run build
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run tauri:check
```

Required file-surfaces to inspect directly:
- `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml` once it exists
- `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/capabilities/default.json`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`

Verification interpretation:
- the git branch inventory proves whether the accepted `develop -> uat -> main` publication backing surface is real instead of documentary wishfulness
- the workflow surface proves whether official release units, checksums, provenance/build receipts, and manifest references can be emitted from governed repo truth instead of local residue
- the ignored-target check prevents accidental counterfeit completion by mistaking local generated package files for publication truth
- the Tauri files prove whether a bounded Linux trust-gate surface exists instead of a generic shell scaffold being mistaken for an updater/runtime contract
- the desktop verification commands protect the existing shell proof surface from collateral drift while provenance and trust work is introduced

## Readiness verdict
This lane is not yet ready for a handoff artifact.

Why it is blocked:
- the accepted SD-12 contract requires official build identity, checksums, provenance/build receipts, manifest linkage, and trust thresholds before any automatic-update claim can be honest
- the live repo still has no publication workflow or other governed surface that emits those companions
- the live desktop/Tauri runtime still exposes no dedicated trust-gate or build-identity command surface, no updater-specific capability surface, and no accepted package/update configuration path beyond a scaffold with `bundle.active: false`
- the upstream E2 lane is already blocked on missing `uat` publication truth and the upstream E3 lane is already blocked on missing manifest producer/consumer truth, so any E5 handoff authored now would inherit counterfeit release topology
- local ignored package-like files under `src-tauri/target/` are residue, not authority; counting them as provenance or integrity fulfillment would be false completion

Why the closure still matters:
- it narrows the future write surface to one future release workflow plus a very small Tauri/package boundary instead of letting provenance or trust work sprawl across the repo
- it separates governed publication truth from ignored local build residue, which is exactly where a weaker process would counterfeit progress
- it preserves SD-11 status/UI authority and keeps E5 focused on build identity, integrity materials, and trust thresholds rather than collapsing several lanes into one

## Successor truth
The real blocker is missing governed provenance-emission and trust-gate execution truth. The next truthful board move is therefore a blocker-repair pair:
- `SD12-E5-B1 FLOW: Resolve provenance emission and platform trust-gate execution truth`
  - unblock condition: the repo exposes an accepted official surface for release-unit-linked build identity, checksum/provenance publication, and Linux trust-gate truth, or the documentary contract is revised explicitly so SD-12 no longer claims this executable lane yet
  - prerequisite truth: the upstream E2 and E3 blocker repairs must land first, because provenance/update-eligibility cannot outrun the publication and manifest topology that defines it
- `SD12-E5-R2 FLOW: Provenance, integrity, and update-eligibility handoff artifact`
  - this card should remain gated behind `SD12-E5-B1` and should only author the stage-specific handoff after the provenance/trust boundary is truthful
