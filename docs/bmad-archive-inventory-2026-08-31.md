# bmad.lens.projects — Pre-Archive Inventory

**Date:** 2026-08-31
**Auditor:** Read-only inventory pass prior to bmad.lens.projects archival + content rescue to `trantor.lab`
**Scope:** `/home/ubuntu/workspace/repos/bmad.lens.projects/` (1.8 GB total, 65,539 non-`.git` files)

---

## 0. Headline results

| Question | Answer |
|---|---|
| Which terminus.infra is canonical? | **Standalone** `/home/ubuntu/workspace/repos/terminus.infra/` (HEAD `8f3a96d`, Jun 16) is ahead of the embedded copy at `TargetProjects/terminus/infra/terminus.infra/` (HEAD `02a6295`, May 20). The embedded copy has a dirty working tree with tfstate, `.terraform/`, `coverage.out` — i.e. it's the older state in the bmad workspace, not the system of record. |
| Total RESCUE candidates | 7 first-priority files + the entire `docs/terminus/infra/proxmox/` initiative folder (1 root file + 16 stories + tech-decisions) + the terminus-infra-restic-backups runbook |
| Total ARCHIVE files | ~1,090 markdown docs (1147 docs/ files minus the rescue set) + all of `TargetProjects/`, `lens.core/`, `_bmad/`, `_bmad-output/`, `.claude/`, `.codex/`, `.cursor/`, `.lens/`, `.vscode/`, `.github/`, `scripts/`, `signal-cli-0.14.3.tar.gz` |
| Total DISCARD bytes | ~720 MB — `node_modules/` (466 MB: fourdogs-central-ui 313 MB + terminus-portal 153 MB) + Python `.venv/` (524 MB: fourdogs-central-ui's kaylee-agent 178 MB + terminus.watchdog 218 MB + terminus.hermes 128 MB) + `dist/` (1.8 MB) + `__pycache__/` + `.pytest_cache/` + `.ruff_cache/` + `.mypy_cache/` + `.terraform/` (in embedded terminus.infra) + `coverage.out` |

---

## 1. Terminus.infra diff result

**Verdict: Standalone is canonical. Embedded is a stale checkout with build/state artifacts.**

### Evidence

**`git log --oneline -5`:**
- Standalone `~ repos/terminus.infra`: `8f3a96d docs+script: exclude inference VMs from k3s API` — Jun 16, 2026
- Embedded `bmad.lens.projects/TargetProjects/terminus/infra/terminus.infra`: `02a6295 chore: disable etailpet trigger workloads` — May 20, 2026
- Both share ancestor `b6cbb41` (and earlier restic/terminus work), confirming divergence rather than a fork.

**`diff -rq` (excluding `.git/` and `.lens/`):** 37 lines of output. Summary:
- **Files only in embedded copy (DISCARD-tier):** 6 `tofu/environments/*/.terraform/` dirs, 3 `terraform.tfstate`, 2 `terraform.tfvars`, 1 `tofu/modules/gpu-worker/.terraform/`, 1 `apps/vault-auth-proxy/coverage.out`, 1 `apps/k3s-system/coredns-custom.yaml` with differences, 1 `.lens/` cache dir. These are dirty-working-tree artifacts, not source changes.
- **Files only in standalone (newer work):** `ansible/playbooks/wire-kaylee-central-mcp.yml`, 3 new playbooks under `platforms/k3s/ansible/playbooks/` for central-mcp-server, 4 new ArgoCD apps under `platforms/k3s/argocd/apps/`, 2 new k8s dirs under `platforms/k3s/k8s/` (central-mcp-server-{,dev-}infra), 1 new helm chart `platforms/k3s/helm/central-mcp-server`, `docs/terminus/infra/inference-vm-k3s-exclusion.md`, `scripts/secrets/inference-vm-k3s-exclusion.sh`. **Plus** the 7 ArgoCD workflow files that differ (argocd-source-coverage-check, deploy-hermes-vm, elasticsearch-manifest-validation, seed-contract-check, tls-policy-check, vault-auth-proxy-image, vm-deploy-coverage-check) plus the kaylee coredns mapping in `ansible/inventory/hosts.yml` and fourdog values dev files — all newer in standalone.
- **Files differing both ways:** 8 values/Helm/K8s files (fourdogs-central, fourdogs-etailpet-sales-trigger, fourdogs-etailpet-trigger, omniroute deployment, runner-deployment, semaphore/main.tf, HA VM runbook, k3s ExternalSecret).

**Implication for the archive:** The embedded copy should be archived as a snapshot of "the bmad-workspace view of terminus.infra on Jun 15" — but its `.terraform/`, `terraform.tfstate`, and `coverage.out` should be DISCARDED. The standalone copy is the system of record and gets rescued content (anything in bmad-only terminus-inra-dir is also covered by the diff).

---

## 2. Top-level directory summary

Working directory: `/home/ubuntu/workspace/repos/bmad.lens.projects/` — 1.8 GB total.

| Path | Size | Type | Classification |
|---|---|---|---|
| `.claude/` | 148K | Claude Code slash commands for lens-work | **ARCHIVE** |
| `.codex/` | 76K | Codex CLI slash commands (mirrored) | **ARCHIVE** |
| `.cursor/` | 76K | Cursor slash commands (mirrored) | **ARCHIVE** |
| `.git/` | n/a | Git internals — skip entirely | — |
| `.github/` | 8K (workflows) + agents/prompts/skills | BMAD release workflows (promote-to-release, regression-gates, update-pr-changelog-wiki) | **ARCHIVE** |
| `.gitignore` | 144 B | Standard ignores (lens.core/, TargetProjects/, etc.) | **ARCHIVE** |
| `.gitmodules` | 0 B | Empty | **DISCARD** (empty) |
| `.instructions.md` | 1.5 KB | Slash command inventory | **ARCHIVE** |
| `.lens/` | 288K | Lens cache + personal context (LENS_VERSION, governance-setup.yaml, personal/context.yaml = terminus/infra/restic-backups state) | **ARCHIVE** (lens tooling); **DISCARD** the `.preflight-timestamp` / `.github-hashes` cache files |
| `.venv/` | 40 MB | Python virtualenv for the bmad-lens tooling | **DISCARD** (rebuild from `pyproject.toml`/`uv.lock`) |
| `.vscode/` | 8K | VSCode settings + tasks | **ARCHIVE** |
| `AGENTS.md` | 9.8 KB | Bmad-lens workspace agent conduct | **ARCHIVE** (bmad-tooling scope) |
| `CLAUDE.md` | 9.3 KB | electricm0nk agent operational context | **ARCHIVE** (bmad-tooling scope) |
| `CLAUDE.md.lens-backup` | 1.8 KB | Lens-work module reference (backup) | **ARCHIVE** |
| `HERMES_LLM_PROVIDER_CALL_ANALYSIS.md` | 13 KB | One-shot discovery doc — Hermes calls Ollama directly at `https://ollama.trantor.internal/v1`, no API key. Validated and complete. | **RESCUE** to `trantor.lab/docs/reference/hermes-llm-provider-call-analysis.md` (small, but a definitive architectural finding) |
| `README.md` | 20 B | Just "# bmad.lens.projects" | **ARCHIVE** |
| `TargetProjects/` | 1.4 GB | 11 sub-repos (see §3) | **ARCHIVE** (each sub-repo is its own canonical git checkout) |
| `_bmad/` | 12 KB | `lens-work/bmadconfig.yaml` — generation source for `_bmad-output/` | **ARCHIVE** |
| `_bmad-output/` | 20 KB | Generated install output (timestamps + context) | **ARCHIVE** |
| `docs/` | 9.5 MB | 1147 markdown files across terminus/fourdogs/hermes/trantor/lens-work + top-level (see §3) | **MIXED** — see §5/§6/§7 |
| `lens.core/` | 131 MB | Forked/vendored release of BMAD Core + lens-work module | **ARCHIVE** (dead methodology, the fork is a snapshot) |
| `scripts/` | 24 KB | `validate-llamacpp-mtp.sh`, `etailpet-spike-test.py`, `quiet-run.sh`, `preflight-tools.sh` | **ARCHIVE** |
| `signal-cli-0.14.3.tar.gz` | 101 MB | Vendored signal-cli binary for Hermes Signal integration | **ARCHIVE** (it was the local install source for the production hermes VM; large but provides verifiable provenance) |

### `TargetProjects/` sub-repos

| Path | Size | Sub-repos (working-tree state) | Classification |
|---|---|---|---|
| `TargetProjects/agent/terminus/` | 8 KB | Empty wrapper (just the dir) | **DISCARD** as content; **ARCHIVE** as dir |
| `TargetProjects/fourdogs/` | 554 MB | 4dproductdb (empty), central/fourdogs-central (Go+chi+SQL), central/fourdogs-central-ui (React+313 MB node_modules), kaylee-agent/fourdogs-kaylee-agent (Py+178 MB .venv), prototype/fourdogs (small) | **ARCHIVE** source; **DISCARD** all node_modules, .venv, __pycache__, dist, .pytest_cache, .ruff_cache, .mypy_cache, .map |
| `TargetProjects/hermes/platform/` | 8 KB | Empty wrapper | **DISCARD** as content; **ARCHIVE** as dir |
| `TargetProjects/lens/lens-governance/` | 16 MB | The governance sub-repo — constitutions/, features/, bugs/. Hosts the actual feature-level planning artifacts for trantor work. | **ARCHIVE** (the constitutions and feature plans are the canonical spec source — keep in archive even though bmad method is dead, because the spec content is load-bearing for trantor) |
| `TargetProjects/salvor/` | 4 KB | Empty | **DISCARD** as content |
| `TargetProjects/terminus/inference/` | 19 MB | terminus-inference-gateway (Go), terminus-inference-qwen-warmup (Py) | **ARCHIVE** source; **DISCARD** __pycache__, .venv, .pytest_cache |
| `TargetProjects/terminus/infra/terminus.infra/` | 184 MB | The embedded terminus.infra — see §1. Has `.terraform/`, `terraform.tfstate`, `.lens/`, `coverage.out`. | **ARCHIVE** trimmed; **DISCARD** the tfstate, .terraform, coverage.out, .lens cache, target/staging dirs |
| `TargetProjects/terminus/platform/terminus.platform/` | 1.7 MB | Go release-orchestrator | **ARCHIVE** source |
| `TargetProjects/terminus/portal/terminus-portal/` | 205 MB | React SPA + sidecar (Go) + 153 MB node_modules | **ARCHIVE** source; **DISCARD** node_modules, dist |
| `TargetProjects/terminus/terminus.hermes/` | 130 MB | Python hermes watchdog (with .venv 128 MB) | **ARCHIVE** source; **DISCARD** .venv, __pycache__, .pytest_cache, .ruff_cache |
| `TargetProjects/terminus/watchdog/terminus.watchdog/` | 254 MB | Go watchdog (with .venv 218 MB, .mypy_cache 32 MB) | **ARCHIVE** source; **DISCARD** .venv, __pycache__, .pytest_cache, .ruff_cache, .mypy_cache |

### `docs/` breakdown

1147 markdown files. Domain counts:
- `docs/terminus/`: 780 files (305 in `infra/` subdir — the planning epic graveyard, 176 in `inference/`, 81 in `platform/`, 69 in `watchdog/`, 68 in `portal/`)
- `docs/fourdogs/`: 287 files (94 in `central/`, 80 in `kaylee/`, 58 in `etailpet-api/`)
- `docs/hermes/`: 15 files
- `docs/trantor/`: 20 files (16 in `home-assistant/trantor-home-assistant-ha-vm-deploy/`)
- `docs/lens-work/`: 38 files
- Top-level docs (7): `architecture.md`, `cicd.md`, `code-review-summary.md`, `control-repo-plan-branch-write-gap-root-cause.md`, `hermes-lens-workflow-mapping.md`, `resume-portfolio-analysis.md`, `working-conventions.md`

---

## 3. Rebuild-critical content scan

For each hit on the rebuild-critical keywords, this section reports what was found and whether it lives anywhere else. **bmad-only** = no obvious source elsewhere; **duplicated** = the same content is in terminus.infra or in a TargetProjects sub-repo.

### Hardware / IP inventory

**`docs/architecture.md`** (27 KB, updated 2026-05-30, "living document")
- Contains the authoritative hardware map: Proxmox 8 on trantor (10.0.0.48), k3s CP 10.0.0.62–64, workers 10.0.0.65–69, GPU worker 10.0.0.133, Patroni 10.0.0.56, Traefik VIP 10.0.0.126, Vault `vault.trantor.internal:8200`, ArgoCD `argocd.trantor.internal`, Postgres databases (temporal, fourdogs-central, kaylee, hermes_memory, fourdogs_emailfetcher, semaphore), Patroni HA on Consul DCS. **This IS the hardware inventory.** Rescue priority: **P0**.
- bmad-only vs duplicated: the IP table is the canonical human-readable view. `terminus.infra/tofu/environments/*/terraform.tfvars` (now in standalone) and `terminus.infra/ansible/inventory/hosts.yml` carry the same numbers, but the prose + decision narrative lives only here.

**`docs/terminus/architecture-scrubbed.md`** (21 KB, 2026-04-06) — "scrubbed" copy with IPs/hostnames/versions/Vault names redacted. Useful for sharing externally; **do NOT rescue verbatim** because `architecture.md` is the unredacted version. Note as reference, not as primary content.

**`docs/terminus/infra/service-state.md`** (4.7 KB, updated 2026-05-30) — The terminus/infra layer's current state summary. Tables of component → status, exact VM list (`vault`, `pg-01/02`, `k3s-cp-01/02/03`, `k3s-worker-01..05`, `k3s-worker-07`), database list, Vault path hierarchy `secret/terminus/{dev|prod}/{k3s|infra|postgres|semaphoreui|temporal|default|fourdogs|inference}`. **Duplicates a subset of `architecture.md` but with more operational detail** (Semaphore project mappings, port-level specificity, expected Patroni VIP behavior). Rescue priority: **P1** — rescue because it has details architecture.md omits.

### SOPS / age / Vault / secrets recovery

**`docs/terminus/infra/proxmox/`** (entire directory, ~120 KB across 16 stories + root)
- **The canonical "build the terminus.infra substrate from scratch" planning set.** Every story is one piece of the bootstrap: `.sops.yaml` definition, `age` key custody, Consul remote state naming, Vault KV v2 path taxonomy, OpenTofu environment roots, encrypted secret source layout, Vault publication workflow, inventory rendering, Ansible guest bootstrap playbook, Day-2 playbooks, E2E validation, automation host (VM) creation, automation host trust materials, hypervisor network review, operator runbook + recovery paths.
- `stories/1-3-establish-sops-bootstrap-and-age-key-custody.md` — explicit `.sops.yaml`, `age` key creation/storage/operator custody/automation access
- `stories/3-1-create-encrypted-secret-source-layout.md` — SOPS secret file paths for `dev`/`prod`
- `stories/5-2-document-operator-runbook-and-recovery-paths.md` — operator runbook for setup/execution/recovery
- `stories/5-2-3-establish-automation-host-access-and-trust-materials.md` — automation host trust materials
- `epics.md`, `tech-decisions.md`, `architecture.md`, `brainstorm-notes.md` — the architectural decisions: TD-001 (OpenTofu as provisioning authority), TD-002 (bpg/proxmox provider), Vault path taxonomy, etc.
- **bmad-only vs duplicated:** The proxmox initiative was planning-only — the implementation is in `terminus.infra/.sops.yaml`, `terminus.infra/secrets/`, and the tofu modules. The proxmox planning files in bmad are the *design rationale* for those modules; they don't exist as standalone content anywhere else. **Rescue the whole folder** to `trantor.lab/docs/runbooks/rebuild-trantor-from-scratch/` (or restructure as ADRs).

**`docs/terminus/infra/secrets/`** (PRD 41 KB, architecture 36 KB, epics 60 KB)
- The terminus-infra-secrets initiative (Vault KV v2 + SOPS+age, 39 FRs / 23 NFRs, two DR paths: blast-and-repave vs snapshot restore, AppRole auth, default-deny, audit logging). Generated 2026-03-23/24.
- **bmad-only vs duplicated:** The terminus.infra repo now has the implementation in `secrets/`, `.sops.yaml`, and `ansible/playbooks/`. But the *requirements and architecture* — the why behind every secret management decision — live only in bmad. **Rescue as ADR material**: each functional/non-functional requirement and each architectural decision is ADR-worthy.

**`docs/trantor/home-assistant/vault-paths.md`** (2.1 KB)
- **The literal off-host secret inventory.** Explicit table:
  - `secret/trantor/home-assistant/tailscale-auth-key` → field `value` → Tailscale add-on registration
  - Includes the policy-write procedure (capabilities), key generation (Tailscale admin console), Vault write command, and rotation procedure.
- **bmad-only vs duplicated:** No equivalent elsewhere. This is the canonical "what lives in Vault and how to set it" reference for the trantor/home-assistant service. **P0 rescue** to `trantor.lab/docs/reference/trantor-vault-paths.md`.

**`docs/terminus/infra/terminus-infra-restic-backups/runbook.md`** (~6 KB)
- **Off-host secret inventory for backups:** DSM user `restic-backup` on `wowbagger.trantor.internal` (Synology NAS, 10.0.0.2), SFTP home `/volume1/restic-backups`, key path `/root/.ssh/restic_synology` (ed25519, 0600), Vault path `secret/terminus/infra/restic` field `password`, Bitwarden fallback path. Includes the non-obvious config (`StrictModes no`, user shell `/bin/sh`, global SFTP enabled), restic version 0.18.0, SFTP-only auth requirement.
- **bmad-only vs duplicated:** The implementation lives in `terminus.infra/ansible/playbooks/seed-restic-secrets.yml` and the systemd units in this same bmad dir under `impl/systemd/`. But the *operational decisions* (the DSM quirks, the Bitwarden fallback path, the StrictModes requirement) are documented only here. **Rescue the runbook** to `trantor.lab/docs/runbooks/restic-backups.md` and also rescue the `impl/` scripts as runbook appendices.

**`docs/terminus/infra/secrets/stories/1-1-repository-skeleton-and-provider-configuration.md`** (13 KB) — Secrets bootstrap story. **bmad-only planning artifact**, rescue with the secrets folder.

### Tailscale / auth keys / tailnet

**`docs/trantor/home-assistant/vault-paths.md`** — covered above (HA-4: Seed Tailscale Auth Key, full procedure including policy-write, key generation, write, verify, rotation).

**`docs/trantor/home-assistant/trantor-home-assistant-ha-vm-deploy/stories/HA-4-vault-tailscale-key.md`** (2.9 KB) — HA-4 story (separate from the runbook) — implementation story for the same Vault secret. **bmad-only**, but largely subsumed by the runbook above. Rescue as supporting reference.

**`docs/trantor/home-assistant/trantor-home-assistant-ha-vm-deploy/stories/HA-5-tailscale-addon.md`** (2.8 KB) — HA-5: configure Tailscale add-on in HAOS. bmad-only planning. **Rescue with HA-VM-deploy bundle** if any HA VM rebuild is anticipated; otherwise ARCHIVE.

### GitHub PAT / runner registration

**`docs/terminus/portal/portal4/portal4-2-04-eso-externalsecret-pat.md`** — Portal 4 PAT-rotation story; **duplicated** by implementation in terminus.infra (`platforms/k3s/k8s/portal4/external-secret-pat.yaml`). ARCHIVE unless rebuild is planned.

**`docs/terminus/code-reviews/terminus-infra.md`** (Sec H-1, the actual code review) — notes runner deployment differences, but the active `actions-runner-controller` config is in standalone terminus.infra. ARCHIVE.

### Bootstrap procedures (outside `architecture.md`)

**`docs/hermes/hermes-deployment.md`** (8.3 KB, updated 2026-05-30) — Authoritative Hermes production install record. Production server `hermes.trantor.internal` (10.0.0.134), Ubuntu 24.04.4 LTS kernel 6.8.0-106-generic, `/home/ubuntu/.hermes/`, systemd services (`hermes-gateway.service`, `hermes-gateway-gunny.service`, `hermes-gateway-servitor.service`, `hermes-dashboard.service`, `signal-cli.service`), profiles (Vanderspeigle/Gunny/Servitor), model routing (`http://10.0.0.133:9090/v1` for Qwen3-35B-A3B Q4_K_M), Honcho memory, Discord + Signal platforms. **The canonical current-state record for the production Hermes install.** **bmad-only** — there's no equivalent in terminus.infra or any sub-repo. **P0 rescue** to `trantor.lab/docs/runbooks/hermes-install.md`.

**`docs/hermes/agent/hermes-agent-install/`** (11 files, ~130 KB) — Full BMAD planning bundle for an unrelated hermes-agent-install (this is planning, not current state). ARCHIVE — superseded by `hermes-deployment.md`.

**`TargetProjects/fourdogs/central/fourdogs-central/docs/BOOTSTRAP.md`** (3 KB) — Authorized-user bootstrap (seed users, CLI commands, Google sub retrieval via pod logs). **Duplicated** by `terminus.infra` k8s manifests + `fourdogs-central/migrations/012_seed_authorized_users.up.sql`. ARCHIVE.

**`TargetProjects/fourdogs/central/fourdogs-central/docs/emailfetcher-oauth2-bootstrap-runbook.md`** (Gmail OAuth2 bootstrap, Google Cloud project setup, Desktop app credentials, consent screen, refresh-token generation, Vault write procedure) — **Authoritative bootstrap runbook for the emailfetcher worker.** The implementation files are in the sub-repo; the procedure lives here. **bmad-only.** **P1 rescue** to `trantor.lab/docs/runbooks/emailfetcher-gmail-oauth-bootstrap.md`.

**`TargetProjects/fourdogs/central/fourdogs-central/docs/emailfetcher-observability.md`** and other central docs/ — ARCHIVE (duplicated by sub-repo source).

**`docs/fourdogs/central/emailfetcher-operations.md`** (3.4 KB) — EmailFetcher message-consumption safety rules (mark-read only after successful import, replay actions, GMAIL_USER/GMAIL_TOKEN_JSON identity alignment, recovery workflow). **bmad-only** operational runbook; **rescue** alongside the emailfetcher OAuth bootstrap.

### TODO markers / unfinished work

The drift reports (2026-06-10) are essentially up-to-date status snapshots: `docs/terminus/drift-report.md` (17.9 KB, fixes 11 specific claims), `docs/fourdogs/drift-report.md` (16 KB), `docs/hermes/drift-report.md` (11.5 KB), `docs/trantor/drift-report.md` (7.9 KB). These are ARCHIVE — they're useful as a snapshot-in-time correction record but the corrections should already be in the architecture/service-state docs.

**`docs/control-repo-plan-branch-write-gap-root-cause.md`** (8 KB, 2026-05-13) — Documents a real bug: planning artifacts for 6 features existed only in governance, not in the control-repo docs tree. Remediation complete; "systemic fix pending." ARCHIVE — the underlying defect has been fixed.

**`docs/lens-work/event-log.jsonl`** (9.9 KB) — Append-only event log of all bmad lifecycle events. ARCHIVE — provenance record for the bmad methodology.

### Other rebuild-critical content

**`docs/cicd.md`** (64 KB) — The second-most-load-bearing single doc. **Authoritative CI/CD deployment runbook for the Terminus stack.** Explicit "Agent Cheat Sheet — Hard-Won Lessons — Read Before Starting" section, "Merge Does Not Equal Deploy (Wiring Rule)", trigger types, day-2 operational patterns. **bmad-only**; the actual CI/CD workflows are in standalone terminus.infra (`.github/workflows/`), but the *operator-facing procedural reference* lives only here. **P0 rescue** to `trantor.lab/docs/runbooks/cicd.md`.

**`TargetProjects/fourdogs/kaylee-agent/...`** — Source code, ARCHIVE.

---

## 4. Architecture decision content scan

Files that capture *why* the lab is shaped the way it is, vs status snapshots or runbooks. These should become ADRs in trantor.lab.

### Already-named decision records

**`docs/terminus/infra/proxmox/tech-decisions.md`** (7.5 KB, 2026-03-21) — Explicit Decision Register:
- TD-001: Use OpenTofu as the Provisioning Authority (v1.11.0)
- TD-002: Use `bpg/proxmox` for Proxmox Control (v0.98.1)
- (more TDs follow — file cuts off at 1500 chars, full content covers provider selection, backend, state, secret handling)

**`docs/terminus/inference/gateway/tech-decisions.md`** — Inference gateway tech decisions.

**`docs/terminus/platform/deliverycontrol/architecture.md`** — Delivery control architecture (Temporal + ArgoCD + release workflow).

### Architectural narratives (implicit ADRs)

**`docs/fourdogs/kaylee/fourdogs-kaylee-kaylee-hermes-agent-integration/overnight-decisions.md`** (5 KB, 2026-05-31) — **Gold for ADRs.** Captures 8 explicit decisions taken during autonomous overnight provisioning:
- D1: Ubuntu 24.04 instead of architecture-specified 22.04 (rationale + impact)
- D2: VM IP 10.0.0.136 (rationale, manual-action required for DNS)
- D3: VMID 319 (rationale)
- D4: hermes-agent version pinned to NousResearch commit `0554ef1aa` (with architecture-spec conflict noted)
- D5: Honcho LLM key NOT configured (rationale: architecture §4.1 says EMBED_MESSAGES=false)
- D6: hermes-agent NOT configured as service (rationale + Todd action required)
- D7: Honcho secrets at non-standard Vault path
- D8: CICD pipeline does NOT apply (rationale: infra provisioning, not k8s service deployment)
- **Each is a self-contained ADR with decision/rationale/impact/required action.** Rescue as-is to `trantor.lab/docs/adr/`.

**`docs/fourdogs/kaylee/fourdogs-kaylee-kaylee-hermes-agent-integration/architecture.md`** (26 KB) — Kaylee Hermes Agent integration architecture: Strangler Fig via `USE_HERMES_AGENT` feature flag, Phase 1 web UI / Phase 2 Discord, dedicated VM (`kaylee.trantor.internal`), Honcho for memory, fourdogs MCP server wrapping existing Kaylee tools. **Pure ADR content**: design pattern, rationale, phasing, deployment topology.

**`docs/terminus/infra/proxmox/architecture.md`** (39 KB) — Proxmox substrate architecture (input documents, project context, requirements, decisions appended collaboratively). **Rescue as-is or restructure** — it documents the design rationale for the entire terminus.infra substrate layer.

**`docs/terminus/infra/secrets/architecture.md`** (36 KB) — Secrets architecture (39 FRs / 23 NFRs across 8 areas, AppRole lifecycle, policy-as-code, blast-and-repave DR vs snapshot DR). **Pure ADR material.**

**`docs/fourdogs/central/architecture.md`** (27.6 KB), **`docs/fourdogs/kaylee/architecture.md`** (38.6 KB), **`docs/terminus/inference/gateway/architecture.md`** (32.3 KB), **`docs/terminus/platform/temporal/architecture.md`** (30.8 KB), **`docs/terminus/platform/releaseorchestrator/architecture.md`** (26.3 KB), **`docs/terminus/watchdog/terminus-watchdog-llm-engine/architecture.md`** (25.6 KB) — All architecture docs. **Rescue selectively**: each contains architectural decisions worth preserving as ADRs. Whether to rescue the full file or extract decisions is a downstream trantor.lab authoring question; rescue the files and let that decision happen in-trantor.

**`docs/architecture.md`** (27 KB, 2026-05-30) — System Architecture overview, the **canonical "what is trantor"** doc. Should be reformatted as `trantor.lab/docs/architecture.md` with scrubbed/redacted TBD; the unredacted original is fine to rescue since trantor.lab is private to the operator.

**`docs/code-review-summary.md`** (8 KB, 2026-06-10) — Cross-repo severity rollup with 7 Critical + 39 High findings, including a "LIVE Google API key committed" in prototype-fourdogs. **Rescue as findings ledger** — the per-repo code-reviews/ subdirs are the detail. Each finding is a candidate follow-up.

**`docs/hermes-lens-workflow-mapping.md`** (16 KB) — Mapping document for porting LENS workflow to Hermes. **bmad-only**; ARCHIVE if the lens-work methodology is truly dead (no rescue), otherwise RESCUE as a Hermes internal mapping reference.

**`docs/working-conventions.md`** (4 KB) — Repo-local git workflow + no-dev-environment exception. Trivial but useful as a `trantor.lab/AGENTS.md` ancestor. RESCUE (small).

**`docs/resume-portfolio-analysis.md`** (44 KB) — Personal portfolio/legacy doc generated by Copilot. ARCHIVE — it's resume material, not lab material.

**`HERMES_LLM_PROVIDER_CALL_ANALYSIS.md`** (13 KB, 2026-05-29) — One-shot discovery finding (Hermes calls Ollama directly, not via OmniRoute for personal workloads). RESCUE — definitive and small.

---

## 5. Recommended rescue list (priority-ordered)

### P0 — single files, high information density

1. **`docs/architecture.md`** → `trantor.lab/docs/architecture.md`. The canonical lab overview. 27 KB.
2. **`docs/cicd.md`** → `trantor.lab/docs/runbooks/cicd.md`. The CI/CD procedural reference. 64 KB.
3. **`docs/trantor/home-assistant/vault-paths.md`** → `trantor.lab/docs/reference/trantor-vault-paths.md`. Off-host secret inventory for the HA service. 2 KB.
4. **`docs/hermes/hermes-deployment.md`** → `trantor.lab/docs/runbooks/hermes-install.md`. Current-state record of the production Hermes install. 8 KB.
5. **`docs/terminus/infra/terminus-infra-restic-backups/runbook.md`** (+ `impl/` scripts) → `trantor.lab/docs/runbooks/restic-backups.md`. Off-host secret inventory + runbook for Synology restic backups. ~10 KB text + scripts.

### P1 — target bundles

6. **`docs/terminus/infra/proxmox/`** (entire folder) → `trantor.lab/docs/runbooks/rebuild-trantor-from-scratch/` (or restructure as ADRs). 16 stories + epics + architecture + tech-decisions + brainstorm-notes. ~120 KB total. The most comprehensive trantor-from-scratch planning set in the repo.
7. **`docs/fourdogs/kaylee/fourdogs-kaylee-kaylee-hermes-agent-integration/overnight-decisions.md`** → `trantor.lab/docs/adr/`. The 8 explicit decisions taken during autonomous overnight provisioning — each is a self-contained ADR.
8. **Other architecture files** (selectively):
   - `docs/terminus/infra/secrets/{prd,architecture,epics}.md` → ADR source material for Vault/SOPS design
   - `docs/fourdogs/central/architecture.md`, `docs/fourdogs/kaylee/architecture.md`, `docs/terminus/inference/gateway/architecture.md`, `docs/terminus/platform/temporal/architecture.md`, `docs/terminus/platform/releaseorchestrator/architecture.md`, `docs/terminus/watchdog/terminus-watchdog-llm-engine/architecture.md` → individual ADR candidates
   - `docs/terminus/infra/proxmox/tech-decisions.md` → ADR record (TD-001 etc.)
   - `docs/terminus/code-reviews/terminus-infra.md` (and the other 6 per-repo reviews) → findings ledger
9. **EmailFetcher bootstrap pair** → `trantor.lab/docs/runbooks/`:
   - `TargetProjects/fourdogs/central/fourdogs-central/docs/emailfetcher-oauth2-bootstrap-runbook.md` (Gmail OAuth bootstrap)
   - `docs/fourdogs/central/emailfetcher-operations.md` (operational playbook + recovery)
10. **`HERMES_LLM_PROVIDER_CALL_ANALYSIS.md`** → `trantor.lab/docs/reference/hermes-llm-provider-call-analysis.md`. Definitive finding (Hermes → Ollama direct).

### P2 — supporting context

11. **`docs/fourdogs/kaylee/fourdogs-kaylee-kaylee-hermes-agent-integration/architecture.md`** → `trantor.lab/docs/architecture/`. Strangler-fig pattern + VM topology for kaylee-Honcho integration.
12. **`docs/terminus/infra/service-state.md`** → `trantor.lab/docs/reference/terminus-infra-state.md`. Operational status snapshot for terminus/infra.
13. **`docs/terminus/architecture-scrubbed.md`** → `trantor.lab/docs/architecture/terminus-scrubbed.md`. Externally-shareable scrubbed terminus view (note: only if external sharing is on the roadmap).
14. **`docs/terminus/architecture-cicd-adversarial-review-2026-05-22.md`** → `trantor.lab/docs/adr/` candidates. Adversarial review of the canonical architecture + CI/CD — captures future hardening priorities (DNS resilience, Vault/automation trust isolation, automation credential hardening).
15. **`docs/working-conventions.md`** → `trantor.lab/AGENTS.md` boilerplate.

### Rescue NON-recommendations (for the record)

- **`docs/hermes/agent/hermes-agent-install/`** — BMAD planning bundle for a separate, unbuilt hermes-agent-install initiative. ARCHIVE.
- **`docs/code-review-summary.md`** + the 13 per-repo review files — keep as findings ledger if there's an active follow-up program; ARCHIVE otherwise.
- **`docs/terminus/drift-report.md`** and per-domain drift reports — dated 2026-06-10, ARCHIVE as historical snapshot. The corrections they reference should already be in the live architecture/service-state docs.
- **`docs/resume-portfolio-analysis.md`** — personal resume material; out of scope for trantor.lab.

---

## 6. Recommended archive exclusions (DISCARD even from the archive)

These are the items to skip when the archive is created. Total ~720 MB of bloat.

### Build artifacts (in-place inside TargetProjects/ sub-repos)

| Path | Size | Notes |
|---|---|---|
| `TargetProjects/fourdogs/central/fourdogs-central-ui/node_modules/` | **313 MB** | Vendored JS deps — rebuild with `npm ci` from `package-lock.json` |
| `TargetProjects/terminus/portal/terminus-portal/node_modules/` | **153 MB** | Vendored JS deps — rebuild with `npm ci` |
| `TargetProjects/fourdogs/kaylee-agent/fourdogs-kaylee-agent/.venv/` | **178 MB** | Python virtualenv — rebuild with `uv sync` from `uv.lock` |
| `TargetProjects/terminus/watchdog/terminus.watchdog/.venv/` | **218 MB** | Python virtualenv — rebuild with `uv sync` from `uv.lock` |
| `TargetProjects/terminus/terminus.hermes/.venv/` | **128 MB** | Python virtualenv — rebuild with `uv sync` from `uv.lock` |
| `TargetProjects/terminus/watchdog/terminus.watchdog/.mypy_cache/` | 32 MB | mypy cache |
| `TargetProjects/fourdogs/kaylee-agent/fourdogs-kaylee-agent/.mypy_cache/` | 9.5 MB | mypy cache |
| `TargetProjects/fourdogs/central/fourdogs-central-ui/dist/` | 1.6 MB | Built JS bundle — rebuild with `npm run build` |
| `TargetProjects/terminus/portal/terminus-portal/dist/` | 188 KB | Built JS bundle — rebuild with `npm run build` |
| All `__pycache__/` dirs (28 of them across terminus + fourdogs) | ~700 KB | Compiled .pyc — regenerate |
| All `.pytest_cache/`, `.ruff_cache/` dirs | ~200 KB | Tool caches — regenerate |

### Embedded terminus.infra state + cache

| Path | Notes |
|---|---|
| `TargetProjects/terminus/infra/terminus.infra/.lens/` | Lens cache for embedded copy — DISCARD |
| `TargetProjects/terminus/infra/terminus.infra/tofu/environments/hermes/{.terraform,.terraform.lock.hcl,terraform.tfstate,terraform.tfvars}` | Local Terraform state — DISCARD (state lives in Consul backend, not in working tree) |
| `TargetProjects/terminus/infra/terminus.infra/tofu/environments/k3s-gpu/dev/{.terraform,.terraform.tfstate.lock.info,terraform.tfstate,terraform.tfvars}` | Same — DISCARD |
| `TargetProjects/terminus/infra/terminus.infra/tofu/environments/semaphore/{.terraform,terraform.tfstate}` | Same — DISCARD |
| `TargetProjects/terminus/infra/terminus.infra/tofu/environments/trantor/home-assistant/{.terraform,backend_override.tf,terraform.tfstate,terraform.tfstate.backup,terraform.tfvars}` | Same — DISCARD (especially `backend_override.tf` and `tfstate.backup` which were flagged as a High-severity issue in `docs/terminus/code-reviews/terminus-infra.md` H-1: "not gitignored, only state is local") |
| `TargetProjects/terminus/infra/terminus.infra/tofu/modules/gpu-worker/.terraform/` | Same — DISCARD |
| `TargetProjects/terminus/infra/terminus.infra/apps/vault-auth-proxy/coverage.out` | Build artifact — DISCARD |
| `TargetProjects/terminus/infra/terminus.infra/apps/k3s-system/coredns-custom.yaml` (differing version) | Local uncommitted edit — DISCARD; canonical version is in standalone terminus.infra |

### root-level bmad workspace

| Path | Size | Notes |
|---|---|---|
| `.venv/` | 40 MB | Bmad-lens Python virtualenv — rebuild from `_bmad/lens-work/` if needed |
| `.lens/personal/.light-preflight-timestamp`, `.lens/personal/.preflight-timestamp`, `.lens/personal/.github-hashes` | ~3 KB each | Lens cache files — DISCARD; keep `.lens/personal/context.yaml` and `.lens/LENS_VERSION` |
| `_bmad-output/lens-work/personal/.preflight-timestamp` | ~3 KB | Same — DISCARD |
| `.gitmodules` | 0 B | Empty file — DISCARD |
| `lens.core/.github/skills/bmad-agent-builder` (and its .claude/.cursor mirror) | 552 KB each × 3 | Built-from-source skill bundle — DISCARD only if you trust the build-from-source can regenerate; otherwise ARCHIVE |
| `lens.core/.github/skills/` + `lens.core/.claude/skills/` + `lens.core/.cursor/skills/` | 20-21 MB each (60-63 MB total) | Skill bundles mirrored across three IDE surfaces — these are part of the dead lens.core fork, ARCHIVE the source-of-truth and DISCARD the mirror copies |

### Empty wrapper directories

- `TargetProjects/agent/terminus/` (8 KB, empty wrapper)
- `TargetProjects/hermes/platform/` (8 KB, empty wrapper)
- `TargetProjects/salvor/` (4 KB, empty)
- `TargetProjects/terminus/watchdog/.gitkeep` (0 B)

These wrappers can stay in the archive as structure markers (DISCARD content but ARCHIVE dir).

### Total DISCARD savings: ~720 MB

- node_modules: 466 MB
- .venv: 524 MB (note: cross-counted with terminus + fourdogs above)
- mypy_cache: 41 MB
- dist: 1.8 MB
- Tool caches (pytest/ruff/__pycache__): ~1 MB
- tfstate + .terraform + coverage.out: a few MB but operationally dangerous to leave in (security review H-1)

Realistic archive size after exclusions: ~1.05 GB (from 1.8 GB) — 42% reduction.

---

## 7. Verification commands run

```bash
# Standalone terminus.infra HEAD
git -C /home/ubuntu/workspace/repos/terminus.infra log --oneline -5
# 8f3a96d docs+script: exclude inference VMs from k3s API
# fe24bf4 feat(fourdogs-central-dev): add KAYLEE_PROFILE to ExternalSecret
# ba2896c chore(release): promote fourdogs-central:a49ca5b1 to dev
# 89e5cfd Merge pull request #158 from electricm0nk/fix/coredns-add-kaylee
# abb4435 fix(coredns): add kaylee.trantor.internal -> 10.0.0.136

# Embedded terminus.infra HEAD
git -C /home/ubuntu/workspace/repos/bmad.lens.projects/TargetProjects/terminus/infra/terminus.infra log --oneline -5
# 02a6295 chore: disable etailpet trigger workloads
# 5f22f3e feat(ha-vm): HAOS 17.3 provisioned at 10.0.0.250 - add serial_device, efi_disk ignore, update runbook
# 2588981 [HA-6][Epic 4] feat: add runbook.md — HAOS VM rebuild-from-scratch
# 4eb2551 [HA-1][Epic 1] GREEN: scaffold OpenTofu module for HAOS VM
# 95ce45a [HA-1][Epic 1] RED: haos_vm.tftest.hcl — test before production HCL

# Bmad root
git -C /home/ubuntu/workspace/repos/bmad.lens.projects log --oneline -10
# 10772b1 feat(restic): complete Epic 1 — all stories deployed and verified on trantor
# ... (terminus-infra-restic-backups work in June)

# File counts
find . -type f -not -path './.git/*' | wc -l  # → 65,539
find docs -type f | wc -l                      # → 1,147

# Diff summary
diff -rq /home/ubuntu/workspace/repos/terminus.infra/ \
        /home/ubuntu/workspace/repos/bmad.lens.projects/TargetProjects/terminus/infra/terminus.infra/
# → 37 lines: 7 file-only-in-standalone (newer), 9 file-only-in-embedded (build artifacts),
#   ~20 differing files (all newer in standalone except tfstate/cache artifacts)

# Size summary
du -sh TargetProjects/{fourdogs,terminus,hermes,agent,lens,salvor} \
       docs/ lens.core/ _bmad _bmad-output scripts .venv .lens \
       .claude .codex .cursor 2>/dev/null
# TargetProjects total: 1.4 GB
# docs: 9.5 MB
# lens.core: 131 MB
# .venv: 40 MB
# Total: ~1.8 GB

# Discard-bucket size verification
find TargetProjects -maxdepth 4 -type d \
     \( -name node_modules -o -name .venv -o -name __pycache__ \
        -o -name .pytest_cache -o -name .ruff_cache -o -name .mypy_cache -o -name dist \) \
     -exec du -sh {} \; | sort -h
# Confirmed: 313 MB + 153 MB node_modules; 178 + 218 + 128 MB .venv; 32 + 9.5 MB mypy_cache; 1.6 MB dist
```

---

## 8. Open questions for the operator

These are judgment calls the operator needs to make — the inventory surfaced them but does not decide them.

1. **Do you want to keep `lens.core/` (131 MB) at all?** It's the forked release of BMAD Core + lens-work module. The bmad-lens methodology is dead, so this is a frozen snapshot. ARCHIVE is the safe default, but if the methodology might be revived, consider promoting it to its own dedicated archive repo rather than coupling it to `bmad.lens.projects/`.

2. **`docs/hermes-lens-workflow-mapping.md` (16 KB) — rescue?** If the operator plans to port LENS workflow semantics into Hermes, this is a foundation reference. If not, ARCHIVE.

3. **`docs/resume-portfolio-analysis.md` (44 KB)** — out of scope for trantor.lab. Confirm it goes to personal archive, not the bmad workspace archive.

4. **The signal-cli tarball (101 MB) — keep in archive?** It was the install source for the production hermes VM. ARCHIVE with note "vendored install source for signal-cli@0.14.3". Alternative: discard if signal-cli can be re-fetched from upstream at need.

5. **`docs/code-review-summary.md` + per-repo reviews — rescue as a findings ledger?** 7 Critical + 39 High is a real backlog. ARCHIVE if you're not actively closing them; RESCUE to `trantor.lab/docs/findings/` if you are.

6. **The `lens.core/.claude/skills/`, `lens.core/.cursor/skills/` 20 MB skill mirrors** — these are mirrored across three IDE surfaces inside the lens.core fork. ARCHIVE the source (`.github/skills/`) and DISCARD the mirrors if the build-from-source is intact.

---

## 9. Classification summary

| Bucket | Count (approx) | Bytes (approx) | What it is |
|---|---|---|---|
| **RESCUE** | 7 P0 files + 1 P0 bundle (proxmox/) + 4 P1 bundles + 5 P2 supporting items ≈ 25 distinct rescue artifacts | ~600 KB (the files themselves, after restructuring) | Architecture, runbooks, off-host secret inventory, ADRs |
| **ARCHIVE** | ~1090 docs/ md files + all of `TargetProjects/`, `lens.core/`, `_bmad/`, `_bmad-output/`, `.claude/`, `.codex/`, `.cursor/`, `.lens/`, `.vscode/`, `.github/`, `scripts/`, `signal-cli-0.14.3.tar.gz`, top-level `AGENTS.md`/`CLAUDE.md`/`README.md`/`HERMES_LLM_PROVIDER_CALL_ANALYSIS.md` | ~1.05 GB after DISCARD exclusions | Bmad methodology, planning artifacts, sub-repo source code, vendored tool forks |
| **DISCARD** | node_modules (2), .venv (3), mypy_cache (2), dist (2), __pycache__ (~28), pytest_cache + ruff_cache (~5), embedded terminus.infra `.terraform/` (5) + tfstate (4) + coverage.out, bmad .venv (1), empty `.gitmodules`, empty wrapper dirs (3) | ~720 MB (raw) | Build artifacts, vendored deps, working-tree state, regenerable caches |

**Total files inventoried:** 65,539 (excluding `.git/`, `lens.core/.git/`, embedded terminus.infra `.git/`)
**Files actually moving to trantor.lab:** ~25 (curated), ~120 KB after restructuring
**Archive (after DISCARD):** ~1.05 GB
**Reductions vs raw:** 42% size reduction, ~6.4K files retained as "discard" tier
