---
title: GE08-E5-R2 Desktop Authoring Boundary and Native Verification Closure
artifact_type: route-closure
stc_id: STC-CODEX-GE-08
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE08-E5-R2 — Desktop authoring boundary and native verification closure
workflow_route: readiness-closure
readiness: codex-ready
handoff_created: true
handoff_path: ./ge08-e5-f1-execution-handoff-2026-06-27.md
review_date: 2026-06-27
owner: Todd Hintzmann
scope: program
code_authority: false
related_artifacts:
  - ./ge08-e5-r1-execution-readiness-closure-2026-06-27.md
  - ./rules-studio-surface-definition.md
  - ./validation-and-preview-workflow-requirements.md
  - ./ge08-e4-f1-execution-handoff-2026-06-27.md
  - ../../GE-07-desktop-shell-and-modern-ux/artifacts/ui-command-boundary-requirements.md
---

# GE08-E5-R2 Desktop Authoring Boundary and Native Verification Closure

## Verdict
GE08-E5 is now ready to mint a bounded GE08-E5-F1 code-authorizing handoff.

The decisive change is not that the desktop shell suddenly became complete. It did not. The decisive change is that the two missing truths from R1 are now grounded:

1. the first truthful GE08 desktop request/response contract can now be named directly against the live GE08-E4 headless substrate on `origin/ge08-e4-f1-preview-and-explanation-bridge@9c883dc1ee3e8a7de35ecea9cf84bd6c9611cb1f`
2. the designated implementation host can now execute native Tauri verification far enough to expose repo-local failures instead of dying on missing host prerequisites

That is enough to stop blocking on readiness and start a bounded coding lane.

## What changed from R1
R1 was blocked for two honest reasons:
- the desktop seam still exposed only `load_pilot_shell_snapshot` placeholder truth
- this host could not verify any Tauri-side change because native Linux prerequisites were absent

This pass repaired the second truth surface and grounded the first.

### Native verification truth on this host
The host now has the native prerequisites needed for Tauri verification:
- `pkg-config 1.8.1`
- `glib-2.0 2.80.0`
- `gtk+-3.0 3.24.41`
- `webkit2gtk-4.1 2.52.3`

Observed command results after repair:

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop
npm run typecheck     # pass
npm run build         # pass
npm run tauri:check   # reaches real Tauri compile; now fails on missing repo asset, not host setup
```

Current remaining native failure:
- `apps/desktop/src-tauri/icons/icon.png` is missing, so `tauri::generate_context!()` aborts during `cargo check`

This is no longer an environment blocker. It is a bounded repo-local implementation defect that can be addressed inside GE08-E5-F1.

## Live route truth
| Gate | Status | Evidence |
|---|---|---|
| GE08 headless authoring substrate exists on a real stacked branch | pass | `origin/ge08-e4-f1-preview-and-explanation-bridge@9c883dc1ee3e8a7de35ecea9cf84bd6c9611cb1f` exposes `src/homebrew_authoring/mod.rs`, `package_manifest.rs`, `package_store.rs`, and `preview_bridge.rs`. |
| Existing desktop shell still exposes placeholder pilot-snapshot truth | pass | `apps/desktop/src/boundary/loadPilotShellSnapshot.ts` and `apps/desktop/src-tauri/src/main.rs` still speak only `load_pilot_shell_snapshot` placeholder data. |
| First truthful GE08 desktop contract can now be named without guessing | pass | The stacked branch already defines the source package, validation state, diagnostics, preview envelope, selected-slot resolution, provenance refs, explanation refs, and package-store lifecycle edges the shell must consume. |
| Native Tauri verification is repairable on the designated host | pass | After installing Linux Tauri prerequisites, `npm run tauri:check` now reaches repo code and fails only on missing `icons/icon.png`. |
| Exact GE08-E5-F1 write scope and verification commands are now honest | pass | The remaining work is bounded to the desktop boundary, Tauri command adapter, path dependency / asset fix, and shell rendering. |

## The first truthful GE08 desktop contract
The lesser models would stuff GE08 payloads into the existing `PilotShellSnapshot` name and call it progress. That is counterfeit continuity. The first truthful route is a new authoring-workbench contract over the existing headless substrate.

### Command name
`load_ge08_authoring_workbench_snapshot`

### Request
The first proof slice must let the shell identify exactly which deterministic package bundle it is loading and which authored record it should focus.

```ts
type Ge08AuthoringWorkbenchRequest = {
  packageRoot: string;
  activeRecordRef?: string | null;
};
```

Required request rules:
- `packageRoot` is repo-root-relative for this lane (example: `tests/fixtures/ge08/guard-stance-package`)
- `activeRecordRef` is an authored stable id when the workbench should focus a specific record (`feat.homebrew.guard_stance`, `effect.homebrew.guard_stance.ac_bonus`, or `prerequisite.homebrew.guard_stance.dex13`)
- the command must refuse unknown package roots or fake fallback packages instead of inventing shell truth

### Response
The response must be a direct desktop-consumer projection over the existing GE08 package store and preview bridge, not a second semantic engine.

```ts
type Ge08AuthoringWorkbenchSnapshot = {
  packageRoot: string;
  packageState: 'draft' | 'valid' | 'invalid' | 'deferred';
  packageManifest: {
    packageId: string;
    packageTitle: string;
    packageVersion: string;
    dependsOn: string[];
    supportedObjectKinds: string[];
  };
  activeRecordRef: string | null;
  authoredRecords: {
    feat: {
      stableId: string;
      displayName: string;
      objectKind: string;
      substitutesFor: string;
    } | null;
    effect: {
      stableId: string;
      owningFeatId: string;
      targetFamily: string;
      modifierType: string;
      modifierValue: number;
      stackingPosture: string;
    } | null;
    prerequisite: {
      stableId: string;
      owningFeatId: string;
      predicate: string;
    } | null;
  };
  preview: {
    caseId: string;
    previewStatus: 'success' | 'blocked' | 'unsupported';
    selectedSlotResolution: {
      slot: string;
      removed: string;
      added: string;
      resolvedFeatId: string;
    };
    baselineArmorClass:
      | { kind: 'computed'; value: number }
      | { kind: 'blocked'; reason: string };
    diagnostics: Array<{
      class: string;
      severity: 'Error' | 'Warning';
      message: string;
      subjectRef: string;
      claimBlocking: boolean;
    }>;
    provenanceRefs: Array<{
      stableId: string;
      sourcePackageId: string;
      authoredPath: string;
    }>;
    explanationRefs: Array<{
      nodeKind: string;
      refId: string;
      detail: string;
    }>;
    oracleDimensionStatus: Array<{
      dimension: string;
      status: string;
    }>;
    blockedClaims: string[];
  };
  lifecycleGateState: {
    saveAllowed: true;
    previewAllowed: boolean;
    exportAllowed: boolean;
    diffMode: 'deferred';
  };
  dataSource: 'ge08-headless-preview-bridge' | 'tauri-unavailable';
  note: string;
};
```

### Why this contract is honest
It is grounded directly in live stacked-branch structures:
- `PackageManifest` -> `packageState`, manifest identity/version/dependencies
- `SourcePackage` -> authored feat/effect/prerequisite inventory
- `PreviewEnvelope` -> `previewStatus`, `selectedSlotResolution`, diagnostics, provenance refs, explanation refs, oracle-dimension status, blocked claims
- `PackageStore::export` and package validation state -> lifecycle gate state

It does not ask the desktop shell to invent rules semantics, preview outputs, provenance, or blocked-path posture.

## Exact repo surface now authorized by truth
The first bounded GE08-E5-F1 lane can be named honestly now.

Required write surface:
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/boundary/**`
- `apps/desktop/src-tauri/src/**`
- `apps/desktop/src-tauri/Cargo.toml`
- `apps/desktop/src-tauri/Cargo.lock`
- `apps/desktop/src-tauri/tauri.conf.json`
- `apps/desktop/src-tauri/icons/**`

Explicit non-goals remain:
- no edits under `src/homebrew_authoring/**`
- no edits under `src/rules_core/**`
- no changes under `programs/codex/**` except documentary artifacts already produced by Hermes
- no broadened editor/plugin/runtime work
- no fake product state outside Tauri runtime

## Success-path and blocked-path verification contract
A later GE08-E5-F1 implementation is only honest if it proves both:

1. success path
   - load the valid guard-stance package
   - surface `packageState: valid`
   - surface `previewStatus: success`
   - surface selected-slot resolution for the Human bonus feat substitution
   - surface provenance refs and explanation refs
   - keep export allowed only when the package remains valid

2. blocked path
   - load a malformed or widened guard-stance variant
   - surface `previewStatus: blocked` or `unsupported`
   - preserve diagnostics and blocked claims
   - preserve explanation/provenance visibility instead of blanking the shell
   - refuse export when the package is invalid/deferred/unsupported

## Why readiness is no longer blocked
The route is no longer blocked by missing truth. It is blocked only by implementation work.

That distinction matters.

Before this pass, any GE08-E5-F1 code packet would have had to invent both the desktop contract and the native verification environment. After this pass:
- the desktop contract is explicitly named
- the host can run native Tauri verification meaningfully
- the remaining failures are exact repo defects inside a bounded file surface

That is the threshold for code authority.

## Exact next move
Mint the bounded stage-specific handoff:

`./ge08-e5-f1-execution-handoff-2026-06-27.md`

The next executable successor is a Claude-launchable GE08-E5-F1 desktop workbench proof lane on a stacked worktree rooted at `origin/ge08-e4-f1-preview-and-explanation-bridge@9c883dc1ee3e8a7de35ecea9cf84bd6c9611cb1f`.

## Completion rule
This closure is complete because it did the one honest job remaining between GE08 headless proof and a real desktop lane:
- named the first truthful GE08 desktop request/response contract over the live headless substrate
- repaired the designated host's native verification prerequisites until Tauri reached repo-local code truth
- converted the remaining native failure from environment folklore into a bounded file-level implementation issue
- established an exact write scope and verification surface for GE08-E5-F1
- minted the code-authorizing handoff instead of hiding behind another fake readiness block
