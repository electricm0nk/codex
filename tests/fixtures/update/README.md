# Test fixtures — SD-16 update metadata contracts

This directory holds the byte-faithful fixture set consumed by BOTH the
Python release-tooling lane (via `python -m jsonschema -i`) and the
TypeScript shell-parser lane (via `parseChannelIndex.ts` /
`parseUpdateManifest.ts`). The single source-of-truth for the fixture
contract is the F3b slice of
`programs/codex/requirements/SD-16-feedback-loop-and-self-update-hardening/`
(F1 closure, F2 execution handoff).

## Files and AV mapping

| Fixture                                          | Direction   | AV id    | Notes |
| ------------------------------------------------ | ----------- | -------- | ----- |
| `alpha.json`                                     | positive    | AV-SCH-1 | alpha channel-index, valid; signature:null |
| `beta.json`                                      | positive    | AV-SCH-1 | beta channel-index, valid; signature:null |
| `stable.json`                                    | positive    | AV-SCH-1 | stable channel-index, valid; signature:null |
| `alpha.full-manifest.json`                       | NEGATIVE    | AV-SCH-4 | smuggles manifest fields onto channel-index; rejected by `additionalProperties: false` |
| `channel-index.bad-tag.json`                     | NEGATIVE    | AV-SCH-7 | channel=alpha, tag=beta/...; rejected by allOf cross-field rule (tag prefix != channel) |
| `update-manifest.json`                           | positive    | AV-SCH-2 | valid update-manifest with all canonical fields |
| `release-manifest.bad-path.json`                 | NEGATIVE    | AV-SCH-6 | release_notes_path outside `programs/codex/requirements/[^/]+/release-notes\.md`; rejected by pattern |
| `update-manifest.missing-signature-allowed.json` | positive    | AV-SCH-3 | valid update-manifest with `signature:null`; exercises AV-SCH-3's null-allowed contract |

## Duplication discipline

Every fixture here is the byte-faithful copy of the canonical fixture
that ships to both lanes. If you edit any fixture, BOTH the Python
release-tooling tests (F3a's `tests/release/test_update_metadata_schemas.py`)
and the TS parser tests (F3b's `apps/desktop/src/sd16/update/*test.ts`)
must be re-run; the AJV and jsonschema verdicts MUST match. Divergence
is a schema-drift bug caught by the F4 merge receipt's dual-validator
verification command.
