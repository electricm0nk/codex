# Acceptance and Verification

## Acceptance Criteria
- [ ] the SD-11 control bundle exists with `README.md`, `technical-requirements.md`, `technical-design.md`, `acceptance-and-verification.md`, `risks-and-open-questions.md`, and `epic-breakdown.md`
- [ ] the packet defines the first tester workbench as a bounded surface over real Codex truth rather than a mock façade or abstract UI wish-list
- [ ] the packet defines separate GitHub bug-report and enhancement-request contracts with structured evidence requirements
- [ ] the packet defines an evidence-capture matrix that distinguishes auto-captured fields, user-supplied fields, optional evidence, and redaction rules
- [ ] the packet defines an honest update/channel/support mapping that preserves live `develop -> main` as operator truth, treats any future `beta` stage as reserved until backed, and still refuses raw branch names as primary tester UX
- [ ] the packet remains planning-only and does not counterfeit code authority

## Artifact Completeness Gate
- [ ] every `status: required` artifact in **Expected Output Artifacts** has an exact destination path
- [ ] every `status: required` artifact has a concrete completion rule
- [ ] final verification proves each required artifact exists or was updated as required
- [ ] any intentionally deferred artifact is called out explicitly in `README.md` and `risks-and-open-questions.md`

## Verification Requirements

### Planning-surface truth check
- confirm the packet reflects the live repo truth that `repos/codex/apps/desktop/src/App.tsx` is currently a bounded GE-08 workbench and that `loadPilotShellSnapshot.ts` remains a placeholder seam
- confirm Linux-first / macOS-second / Windows-third-class posture remains explicit in the packet
- confirm GitHub is the only feedback system of record named by SD-11

### Minimal Change Rule
- make only documentary/control-plane changes required to create and register the SD-11 source STC
- do not change repo implementation code in `repos/codex/**`
- do not unblock later tranche domains while performing SD-11 documentary work

### Final Verification
- verify the new packet directory exists under `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/`
- verify all required control-bundle files exist
- verify all required same-epic documentary artifacts exist
- verify `programs/codex/requirements/README.md` now indexes SD-11 as a live source STC rather than a merely planned one
- verify the SD-11 strategic spec-domain no longer claims that no source STC exists

## Evidence Expectations
- report actual file paths written and actual control-plane files patched
- if any referenced upstream truth cannot be grounded from the live repo or current accepted docs, say so explicitly
- do not substitute intention for proof

## Exit Conditions
The source STC may be treated as execution-story-ready when the packet and same-epic documentary artifacts exist, the control plane is updated, and `epic-breakdown.md` is concrete enough to mint bounded same-domain stories without guesswork. A later coding handoff remains forbidden until a future bounded slice grounds exact repo paths, verification commands, and write scope.
