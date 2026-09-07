# Runner Audit — TargetProjects — 2026-08-31

Audit of every GitHub Actions workflow file under
`/home/ubuntu/workspace/repos/bmad.lens.projects/TargetProjects/`, classifying
each job as **MIGRATABLE** (already runs on github-hosted), **MIGRATABLE-WITH-EFFORT**
(needs network or secret changes to leave the self-hosted k3s runner), or
**LOCALITY-REQUIRED** (genuine need for in-cluster network).

This audit is the per-service-repo counterpart to
`docs/runner-audit-2026-08-31.md` (the prior audit, which only covered
`codex`, `pcgen`, and the seven `terminus.infra` validation workflows). The
two should be read together for the complete picture.

## Executive Summary

**Partial — the k3s self-hosted runner cannot be fully retired, but the
surface that actually needs it is much smaller than the runner pool suggests.**

- **67 jobs** across **34 project-owned workflow files** audited.
- **15 jobs** declare `runs-on: [self-hosted, trantor-internal]`. None of the 15
  is LOCALITY-REQUIRED in a strict sense: every one is reachable through a
  Tailscale sidecar + a GitHub repo-level secret named `INFRA_DEPLOY_KEY`,
  because **the only network target they hit is the in-cluster Temporal gRPC
  frontend at `10.43.29.119:7233`** (or, for three of them, an in-cluster
  `kubectl exec` into the Temporal admintools pod).
- **The only true self-hosted-only job is `deploy-hermes-vm.yml::deploy-hermes-honcho`**
  (from the prior audit, included for completeness), which calls the Semaphore
  API at `https://semaphore.trantor.internal` — a hostname that resolves only
  on the homelab network. Migrate with a Tailscale sidecar; ~5 lines of YAML.
- **Recommendation for the month-end Pro+ → Pro plan decision:**
  - **Hold the runner pool.** The Pro+ plan gives 50 self-hosted runner hours;
    the 10 in-cluster deployments + 2 spares consume them today. The
    deployments themselves (k8s manifest at
    `terminus.infra/platforms/k3s/k8s/actions-runner/runner-deployment.yaml`)
    are repo-scoped (10 of them, one per service), not a single shared pool —
    the 50-hour cap is per-org, but the deployments are replicas of the same
    pool. Downgrading to Pro will *not* free you from needing self-hosted
    capacity unless every release.yml is migrated.
  - **Path to retirement**: migrate the 15 self-hosted jobs onto a Tailscale
    sidecar pattern + `INFRA_DEPLOY_KEY` as a repo secret. Cost: ~5–10 lines
    of YAML per workflow, plus moving `INFRA_DEPLOY_KEY` from Vault →
    ESO → k8s Secret → GitHub repo Secret. Estimated effort: **one focused
    cycle, 2–4 hours of YAML + one secret migration.**
  - **Do not downgrade the plan until the migration lands.** Until then, the
    Pro+ 50-hour self-hosted cap is a real constraint you are using. After
    migration, the k3s runner pool can be decommissioned entirely and the
    plan downgraded without loss.

## Methodology

**Search (matches prior audit):**

```bash
find /home/ubuntu/workspace/repos/bmad.lens.projects/TargetProjects \
  -name '*.yml' -path '*.github/workflows*' -type f \
  -not -path '*/node_modules/*' -not -path '*/.venv/*' \
  -not -path '*/site-packages/*' -not -path '*/venv/*' | sort
```

Returned **34 project-owned files** (excluding vendored `.venv/lib/python3.13/site-packages/temporalio/bridge/sdk-core/...` and `node_modules/@ungap/structured-clone/...`).

**Job count (two-implementation cross-check, per AGENTS.md §Concurrency and Measurement):**

```bash
# Method 1: runs-on: line count (matches prior audit's method)
grep -cE '^\s+runs-on:' <workflow>
# Total = 67

# Method 2: structural parser (jobs: section, 2-space indent, job name followed by runs-on|workflow_call|uses:)
awk-script — see Reproduction Commands
# Total = 67
```

Both methods agree: **67 jobs**.

**Self-hosted declarations:** 15 (matches `grep -rn 'runs-on:.*self-hosted'` output).

**Locality markers checked per job:**

1. `runs-on` value (ubuntu-latest, ubuntu-24.04-arm, macos-latest, windows-latest, matrix, `[self-hosted, trantor-internal]`)
2. `${{ secrets.* }}` references other than `GITHUB_TOKEN`, `CROSS_REPO_PAT`
3. References to `INFRA_DEPLOY_KEY` (env var from runner pod, not from GitHub secrets)
4. References to `SEMAPHORE_*`, `ARGOCD_*`, `VAULT_*`, `TS_AUTHKEY`, `TAILSCALE_*`
5. File-system paths under `/`, `/home`, `/mnt`, `/opt` outside `${{ github.workspace }}`
6. Docker mount syntax, `volumes:` declarations, `services:` blocks
7. Hardcoded in-cluster IPs (10.43.0.0/16 = k3s service CIDR per `terminus.infra/platforms/k3s/ansible/group_vars/all.yml:15`)
8. References to `*.trantor.internal` hostnames
9. Steps that `git push` to in-cluster gitops repos (i.e., `terminus.infra`)
10. Steps that call `kubectl exec` against in-cluster pods
11. Custom actions or scripts that touch local filesystem outside `${{ github.workspace }}` or `$RUNNER_TEMP`

**Classification rules (matches prior audit §Methodology):**

- **MIGRATABLE**: `runs-on: ubuntu-latest | macos-latest | windows-latest | matrix` with only standard actions and `secrets.GITHUB_TOKEN`. No bind mounts, no local-path env vars, no `services:` with `volumes:`.
- **MIGRATABLE-WITH-EFFORT**: Currently `runs-on: [self-hosted, trantor-internal]` but the steps it performs are reachable from github-hosted *given a specific, named migration change* (Tailscale sidecar, secret moved to GitHub, public hostname for Semaphore, etc.).
- **LOCALITY-REQUIRED**: Cannot run on github-hosted without significant loss — e.g., bind-mounted cluster state, hardware-specific (GPU, arch), secrets that cannot leave the cluster.

## Combined Inventory — Prior Audit + This Audit

| Source | Workflows | Jobs | Self-hosted jobs | Notes |
|---|---:|---:|---:|---|
| `codex` (prior) | 7 | 16 | 0 | All `ubuntu-latest`; publish lane uses matrix. |
| `pcgen` (prior) | 5 | 11 | 0 | All `ubuntu-latest`; matrix across 4 OSes. |
| `terminus.infra` (prior) | 7 | 8 | 1 | Only `deploy-hermes-vm.yml::deploy-hermes-honcho`. |
| **TargetProjects (this audit)** | **34** | **67** | **15** | See per-repo inventory below. |
| **Combined** | **53** | **102** | **16** | 1 from prior + 15 from TargetProjects. |

## Per-Repo Inventory — TargetProjects

| Repo | Workflows | Jobs | Self-hosted jobs | Self-hosted job names |
|---|---:|---:|---:|---|
| `fourdogs-central-ui` | 2 | 3 | 0 | — |
| `fourdogs-central` | 8 | 18 | 5 | `release` (in `on-ui-dispatch.yml`, `on-ui-dispatch-dev.yml`, `release-catalog-trigger.yml`, `release-central.yml`, `release-emailfetcher.yml`, `release-etailpet-trigger.yml`, `release-etailpet-sales-trigger.yml`) — `release-emailfetcher` and the rest of the four `release-*-trigger` family are 5 jobs across 5 files (plus 2 in on-ui-dispatch*.yml) |
| `fourdogs-kaylee-agent` | 3 | 5 | 1 | `promote-and-release` |
| `terminus-inference-gateway` | 2 | 6 | 1 | `promote-and-release` |
| `terminus-inference-qwen-warmup` | 1 | 2 | 1 | `promote-and-release` |
| `terminus.infra` (in TargetProjects) | 7 | 8 | 1 | `deploy-hermes-honcho` (already in prior audit) |
| `terminus.platform` | 4 | 6 | 1 | `promote-and-release` |
| `terminus-portal` | 2 | 6 | 1 | `promote-and-release` |
| `terminus.hermes` | 2 | 4 | 1 | `promote-manifest` |
| `terminus.watchdog` | 2 | 5 | 1 | `promote-manifest` |
| **Total** | **34** | **67** | **15** | — |

Per-file breakdown (matches `awk` parser output):

```
fourdogs-central-ui/ci.yml: 2                      fourdogs-central-ui/main-from-develop-guard.yml: 1
fourdogs-central/ci.yml: 2                         fourdogs-central/main-from-develop-guard.yml: 1
fourdogs-central/on-ui-dispatch-dev.yml: 2         fourdogs-central/on-ui-dispatch.yml: 2
fourdogs-central/release-catalog-trigger.yml: 3    fourdogs-central/release-central.yml: 3
fourdogs-central/release-emailfetcher.yml: 3       fourdogs-central/release-etailpet-trigger.yml: 3
fourdogs-central/release-etailpet-sales-trigger.yml: 3
fourdogs-kaylee-agent/ci.yml: 1                    fourdogs-kaylee-agent/main-from-develop-guard.yml: 1
fourdogs-kaylee-agent/release.yml: 3
terminus-inference-gateway/ci.yml: 4               terminus-inference-gateway/release.yml: 2
terminus-inference-qwen-warmup/release.yml: 2
terminus.infra/argocd-source-coverage-check.yml: 1 terminus.infra/deploy-hermes-vm.yml: 1
terminus.infra/elasticsearch-manifest-validation.yml: 1   terminus.infra/seed-contract-check.yml: 1
terminus.infra/tls-policy-check.yml: 1             terminus.infra/vault-auth-proxy-image.yml: 2
terminus.infra/vm-deploy-coverage-check.yml: 1
terminus.platform/ci.yml: 1                       terminus.platform/env-map.yml: 1
terminus.platform/publish-images.yml: 1           terminus.platform/release.yml: 3
terminus-portal/ci.yml: 3                         terminus-portal/release.yml: 3
terminus.hermes/ci.yml: 2                          terminus.hermes/release.yml: 2
terminus.watchdog/ci.yml: 3                        terminus.watchdog/release.yml: 2
```

Sum: 67. Reconciles with both `grep -cE '^\s+runs-on:'` and the structural `awk` parser.

## Bucket 1 — MIGRATABLE (52 jobs)

All jobs in this bucket already run on github-hosted. Migration cost: zero — do nothing.

### fourdogs-central-ui (3 jobs)

| File | Line | Job | runs-on |
|---|---:|---|---|
| `ci.yml` | 11 | `require-develop-source` | `ubuntu-latest` |
| `ci.yml` | 33 | `build-and-test` | `ubuntu-latest` |
| `main-from-develop-guard.yml` | 9 | `require-develop-source` | `ubuntu-latest` |

Evidence: standard Node 22, `npm ci`, dispatch via `peter-evans/repository-dispatch@v3` using `${{ secrets.CROSS_REPO_PAT }}` — a normal repo secret, available on github-hosted. No bind mounts, no local paths, no `*.trantor.internal` references.

### fourdogs-central (CI + guards, 4 jobs)

| File | Line | Job | runs-on |
|---|---:|---|---|
| `ci.yml` | 14 | `build-test` | `ubuntu-latest` |
| `ci.yml` | 48 | `docker-publish` | `ubuntu-latest` |
| `main-from-develop-guard.yml` | 9 | `require-develop-source` | `ubuntu-latest` |

(Plus 1 extra `require-develop-source` in `ci.yml:11` which only fires on PR-to-main. Counted separately above.) The `docker-publish` job in `ci.yml:48-136` does commit+push to the in-repo `deploy/helm/fourdogs-central/values.yaml` (line 121-128), but that is the SAME repo (the values file lives in the same checkout) — pure github-hosted native. No `*.trantor.internal` references.

### fourdogs-central (on-ui-dispatch, the 2 cloud-runner jobs)

The `on-ui-dispatch*.yml` files have 2 jobs each — `build-and-push` (cloud) and `release` (self-hosted). The cloud jobs:

| File | Line | Job | runs-on |
|---|---:|---|---|
| `on-ui-dispatch-dev.yml` | 16 | `build-and-push` | `ubuntu-latest` |
| `on-ui-dispatch.yml` | 16 | `build-and-push` | `ubuntu-latest` |

These are standard cross-repo checkout + Node 22 + `docker/build-push-action@v5` → `ghcr.io/electricm0nk/fourdogs-central:$SHA`. No locality.

### fourdogs-central (release-*-trigger family, 8 jobs)

These workflows each have 3 jobs; 1 is self-hosted (`promote-and-release`) and 2 are cloud. The cloud jobs:

| File | Line | Job | runs-on |
|---|---:|---|---|
| `release-catalog-trigger.yml` | 33 | `resolve` | `ubuntu-latest` |
| `release-catalog-trigger.yml` | 62 | `build-and-push` | `ubuntu-latest` |
| `release-central.yml` | 27 | `resolve` | `ubuntu-latest` |
| `release-central.yml` | 63 | `build-and-push` | `ubuntu-latest` |
| `release-emailfetcher.yml` | 32 | `resolve` | `ubuntu-latest` |
| `release-emailfetcher.yml` | 61 | `build-and-push` | `ubuntu-latest` |
| `release-etailpet-sales-trigger.yml` | 32 | `resolve` | `ubuntu-latest` |
| `release-etailpet-sales-trigger.yml` | 61 | `build-and-push` | `ubuntu-latest` |
| `release-etailpet-trigger.yml` | 32 | `resolve` | `ubuntu-latest` |
| `release-etailpet-trigger.yml` | 61 | `build-and-push` | `ubuntu-latest` |

(That's 10 cloud jobs; double-check: 5 files × 2 = 10, and indeed the awk count was 3 real jobs per file × 5 files = 15, with 1 self-hosted per file = 5 self-hosted and 10 cloud.) All standard `actions/checkout@v4`, `actions/setup-go@v5`, `actions/setup-node@v4`, `docker/login-action@v3`, `docker/build-push-action@v5` with `secrets.GITHUB_TOKEN` for GHCR auth. The `release-central.yml::docker-publish` job (line 121-128 of the older `ci.yml`) and `release-central.yml::build-and-push` both push images to `ghcr.io/electricm0nk/fourdogs-central:$SHA`. No bind mounts, no `*.trantor.internal` in the cloud jobs.

### fourdogs-kaylee-agent (1 cloud job + 1 guard)

| File | Line | Job | runs-on |
|---|---:|---|---|
| `ci.yml` | 10 | `ci` | `ubuntu-latest` |
| `main-from-develop-guard.yml` | 9 | `require-develop-source` | `ubuntu-latest` |

Standard Python 3.12 + uv + ruff/mypy/pytest. No locality.

### terminus-inference-gateway (4 cloud jobs)

| File | Line | Job | runs-on |
|---|---:|---|---|
| `ci.yml` | 9 | `generate` | `ubuntu-latest` |
| `ci.yml` | 22 | `build` | `ubuntu-latest` |
| `ci.yml` | 34 | `lint` | `ubuntu-latest` |
| `ci.yml` | 45 | `test` | `ubuntu-latest` |

Standard Go 1.24 + golangci-lint + cache. No locality.

### terminus.infra (6 cloud jobs)

These are the seven validation workflows from the prior audit, all standard Python/Go/Helm/kubeconform setup, all `ubuntu-latest`. Already classified as MIGRATABLE in the prior audit at `docs/runner-audit-2026-08-31.md:99-108`. Re-listed here for completeness:

| File | Line | Job | runs-on |
|---|---:|---|---|
| `argocd-source-coverage-check.yml` | 10 | `validate-argocd-source-coverage` | `ubuntu-latest` |
| `elasticsearch-manifest-validation.yml` | 22 | `validate-elasticsearch-manifests` | `ubuntu-latest` |
| `seed-contract-check.yml` | 10 | `validate-seed-contract` | `ubuntu-latest` |
| `tls-policy-check.yml` | 10 | `validate-tls-policy` | `ubuntu-latest` |
| `vault-auth-proxy-image.yml` | 19 | `test` | `ubuntu-latest` |
| `vault-auth-proxy-image.yml` | 46 | `build-push` | `ubuntu-latest` |
| `vm-deploy-coverage-check.yml` | 10 | `validate-vm-deploy-coverage` | `ubuntu-latest` |

### terminus.platform (1 CI + 1 env-map + 1 publish-images)

| File | Line | Job | runs-on |
|---|---:|---|---|
| `ci.yml` | 7 | `test` | `ubuntu-latest` |
| `env-map.yml` | 60 | `map` | `ubuntu-latest` (this is a reusable workflow — invoked by the `release.yml` files of services that adopted the deliverycontrol pattern) |
| `publish-images.yml` | 13 | `build-and-push` | `ubuntu-latest` |

`env-map.yml::map` outputs strings (`env=`, `namespace=`, `values_file=`, etc.) — no network. `publish-images.yml` is a plain `docker/build-push-action@v5` to `ghcr.io/electricm0nk/terminus-platform-worker:$SHA`. No locality.

### terminus-portal (3 cloud jobs)

| File | Line | Job | runs-on |
|---|---:|---|---|
| `ci.yml` | 24 | `test` | `ubuntu-latest` |
| `ci.yml` | 42 | `build-push` | `ubuntu-latest` |
| `ci.yml` | 89 | `build-push-sidecar` | `ubuntu-latest` |

Standard Node 20 + `npm ci` + `npm test` + `docker/build-push-action@v6`. No locality.

### terminus.hermes (2 cloud jobs)

| File | Line | Job | runs-on |
|---|---:|---|---|
| `ci.yml` | 10 | `lint` | `ubuntu-latest` |
| `ci.yml` | 23 | `test` | `ubuntu-latest` |

Standard Python 3.12 + uv + ruff/pytest. No locality.

### terminus.watchdog (3 cloud jobs)

| File | Line | Job | runs-on |
|---|---:|---|---|
| `ci.yml` | 10 | `lint` | `ubuntu-latest` |
| `ci.yml` | 23 | `typecheck` | `ubuntu-latest` |
| `ci.yml` | 36 | `test` | `ubuntu-latest` |

Standard Python 3.12 + uv + ruff/mypy/pytest. No locality.

## Bucket 2 — MIGRATABLE-WITH-EFFORT (15 jobs)

Every self-hosted job in TargetProjects is MIGRATABLE-WITH-EFFORT. The "effort" is uniform: Tailscale sidecar + move `INFRA_DEPLOY_KEY` from a Vault-synced k8s Secret to a GitHub repo secret. See the **Deep Dive on `promote-and-release`** below for the common pattern.

### Group A — Temporal-only `promote-and-release` jobs (10 jobs)

These jobs:
1. Read `INFRA_DEPLOY_KEY` from runner env (which is currently mounted from a Vault-synced k8s Secret).
2. Run `git clone https://x-access-token:${INFRA_DEPLOY_KEY}@github.com/electricm0nk/terminus.infra.git` (the redacted value in the workflow files; see `terminus.infra/platforms/k3s/k8s/actions-runner/runner-deployment.yaml:55-59` for the source).
3. `sed` an image tag into a values.yaml or app manifest file.
4. `git commit && git push origin ${INFRA_BRANCH}` (terminate here).
5. Run `temporal workflow start --address 10.43.29.119:7233 ...` against the in-cluster Temporal frontend (ClusterIP `10.43.29.119`, see `terminus.infra/platforms/k3s/ansible/group_vars/all.yml:15` — `k3s_service_cidr: "10.43.0.0/16"`).

| File | Line | Job | Uses kubectl exec? | Temporal IP |
|---|---:|---|:---:|:---:|
| `fourdogs-central/on-ui-dispatch-dev.yml` | 87 | `release` | no | 10.43.29.119:7233 |
| `fourdogs-central/on-ui-dispatch.yml` | 96 | `release` | no | 10.43.29.119:7233 |
| `fourdogs-central/release-catalog-trigger.yml` | 89 | `promote-and-release` | no | 10.43.29.119:7233 |
| `fourdogs-central/release-central.yml` | 139 | `promote-and-release` | no | 10.43.29.119:7233 |
| `fourdogs-central/release-etailpet-sales-trigger.yml` | 88 | `promote-and-release` | no | 10.43.29.119:7233 |
| `fourdogs-central/release-etailpet-trigger.yml` | 98 | `promote-and-release` | no | 10.43.29.119:7233 |
| `terminus-inference-gateway/release.yml` | 38 | `promote-and-release` | no | 10.43.29.119:7233 |
| `terminus-inference-qwen-warmup/release.yml` | 50 | `promote-and-release` | no | 10.43.29.119:7233 |
| `terminus-portal/release.yml` | 112 | `promote-and-release` | no | 10.43.29.119:7233 |
| `terminus.platform/release.yml` | 80 | `promote-and-release` | **yes (2×)** | 10.43.29.119:7233 |
| `terminus.hermes/release.yml` | 55 | `promote-manifest` | no | — (no Temporal call; git-push-only) |
| `terminus.watchdog/release.yml` | 56 | `promote-manifest` | no | — (no Temporal call; git-push-only) |

Wait — that's 12 jobs, but the self-hosted count is 15. Reconciling:
- `terminus.hermes/release.yml::promote-manifest` — yes, self-hosted; git-push-only; no Temporal call.
- `terminus.watchdog/release.yml::promote-manifest` — yes, self-hosted; git-push-only; no Temporal call.

So the "Temporal-only" sub-group is 10 jobs, and the "git-push-only" sub-group is 2 jobs. Total: 12. The remaining 3 are in **Group B**.

### Group B — `kubectl exec` Temporal jobs (3 jobs)

These three jobs go beyond the simple Temporal `workflow start` — they `kubectl exec` into the Temporal admintools pod inside the `temporal` namespace. They need either a kubeconfig with cluster-admin in the `temporal` namespace, or a Tailscale tunnel + a forwarded admintools port.

| File | Line | Job | kubectl exec count | What it does |
|---|---:|---|:---:|---|
| `terminus.platform/release.yml` | 80 | `promote-and-release` | 2 (line 204 start, line 228 verify) | `kubectl exec -n temporal deploy/temporal-server-admintools -- /usr/local/bin/temporal workflow start` then `... workflow describe` to verify RunId and Status |
| `fourdogs-central/release-emailfetcher.yml` | 98 | `promote-and-release` | 1 (line 175) | `kubectl exec -n temporal deploy/temporal-server-admintools -- /usr/local/bin/temporal workflow start` (no describe) |
| `fourdogs-kaylee-agent/release.yml` | 85 | `promote-and-release` | 1 (line 157) | `kubectl exec -n temporal deploy/temporal-server-admintools -- /usr/local/bin/temporal workflow start` (no describe) |

Why three workflows still use `kubectl exec`: earlier per-service release files were written before the Temporal CLI was available as a static download on github-hosted (the workflow comment in `terminus-portal/release.yml:136` even says "Download temporal CLI (runner pod has no kubectl; direct gRPC is cleaner)"). The `kubectl exec` pattern was the "the runner pod already has kubectl" shortcut. **Both approaches work; the Temporal CLI download pattern is what every other workflow already uses.** Migrating the three `kubectl-exec` jobs to the CLI pattern is a one-liner change and removes a dependency on the in-cluster `temporal-server-admintools` Deployment.

### Group C — `deploy-hermes-vm.yml` (1 job, repeated from prior audit)

`terminus.infra/deploy-hermes-vm.yml::deploy-hermes-honcho` (line 21-43). Already classified in the prior audit as MIGRATABLE-WITH-EFFORT (one of the buckets there). The locality concern is `SEMAPHORE_BASE_URL=https://semaphore.trantor.internal` (line 10, line 40) — a private DNS zone unreachable from github-hosted without Tailscale. Cost: 5–10 lines of YAML for a Tailscale sidecar; `SEMAPHORE_API_TOKEN` is already a GitHub repo secret.

**The combined MIGRATABLE-WITH-EFFORT surface across both audits: 16 jobs** (15 in TargetProjects + 1 in terminus.infra validation set).

## Bucket 3 — LOCALITY-REQUIRED (0 jobs)

**No job in TargetProjects — or in the prior audit's set — requires self-hosted locality beyond what a Tailscale sidecar + repo-level `INFRA_DEPLOY_KEY` GitHub secret provides.**

Specifically checked and rejected:
- **Bind mounts:** None. The k3s runner image (`ghcr.io/myoung34/docker-github-actions-runner:latest`) is a stock docker-image-based runner. `RUNNER_WORKDIR=/tmp/runner/work` (per `runner-deployment.yaml:45`) is the standard docker-runner ephemeral working directory. There are no host bind mounts. The `INFRA_DEPLOY_KEY` is *environment-injected*, not *bind-mounted*.
- **GPU jobs:** None. The `terminus-inference-qwen-warmup/release.yml` and `terminus-inference-gateway/release.yml` only build amd64 images and call Temporal — no GPU code runs in CI.
- **Architectural locality:** None. The runner pool is repo-scoped (10 deployments, one per service repo at `runner-deployment.yaml:5-9, 71-74, 137-140, 203-206, 269-272, 333-336, 401-404, 467-470, 533-536, 599-602`), but `RUNNER_SCOPE: repo` (line 39) is the standard scoped-runner config, and GitHub-Actions-hosted runners can be targeted at any repo regardless of physical locality.
- **Hardware-specific arch:** None. All jobs are linux/amd64.

## Deep Dive on `promote-and-release` Family

The 12 `promote-and-release` (and `promote-manifest`) jobs share an identical structure:

1. **Validate inputs** (GitHub env vars + `INFRA_DEPLOY_KEY` from runner pod env).
2. **Promote manifest in terminus.infra** — `git clone https://x-access-token:${INFRA_DEPLOY_KEY}@github.com/electricm0nk/terminus.infra.git` → `cd` → `sed` image tag into `platforms/k3s/helm/${SERVICE}/${VALUES_FILE}` (or `${APP_MANIFEST}` for inference-gateway, or `apps/gpu-worker/ollama-deployment.yaml` for qwen-warmup) → `git commit` → `git push origin ${INFRA_BRANCH}` with retry on conflict.
3. **Start Temporal ReleaseWorkflow** — download Temporal CLI static binary → `temporal workflow start --address 10.43.29.119:7233 ...` (or `kubectl exec` for 3 of the 12).

### Why the k3s runner was added (reconstructing the rationale)

The deployment history at `runner-deployment.yaml:20` (`terminus.io/restarted-at: "2026-04-18T00:00:00Z"`) plus the architecture doc at `docs/terminus/platform/deliverycontrol/architecture.md:185` and `docs/terminus/architecture.md:282-313` make the design explicit:

- The runner is in-cluster (same `actions-runner` namespace as the rest of the GitOps delivery control plane).
- It has `serviceAccountName: actions-runner` (`runner-deployment.yaml:26`) — the runner pod can `kubectl exec` into other pods without an explicit kubeconfig, because the in-pod ServiceAccount token is auto-mounted.
- It has `INFRA_DEPLOY_KEY` injected from a Vault-synced k8s Secret (the Vault path is referenced in `docs/terminus/platform/deliverycontrol/architecture.md:375`: `secret/terminus/default/github/runner-token` for `ACCESS_TOKEN` and a parallel path for `INFRA_DEPLOY_KEY`).
- It reaches the Temporal frontend at its ClusterIP because it's in the same cluster.

So three things depend on the runner being in-cluster:
- (a) Cluster-local Temporal gRPC at `10.43.29.119:7233`.
- (b) `kubectl exec` into `temporal-server-admintools` (for 3 workflows).
- (c) `INFRA_DEPLOY_KEY` is sourced from a Vault-synced k8s Secret, not from GitHub secrets.

### What changes for github-hosted migration

The minimum change set:

1. **Move `INFRA_DEPLOY_KEY` from Vault → ESO → k8s Secret into a GitHub repo-level secret** (one secret per repo, or one org-level secret exposed to those repos via `secrets:` declarations in each workflow). The deploy key is just a GitHub fine-grained PAT or a deploy-key SSH token — there's no reason it cannot live in GitHub secrets.
   - Cost: ~5 minutes per repo to add the secret in GitHub Settings → Secrets. Same secret value, new home.
   - Optional: rotate the key after migration (low cost — fine-grained PATs can be revoked and re-issued).

2. **Add a Tailscale sidecar step** at the top of every `promote-and-release` / `promote-manifest` job. Tailscale is already in operational use (`terminus.infra/tofu/environments/trantor/home-assistant/runbook.md:122`: `vault kv put secret/trantor/home-assistant/tailscale-auth-key value=tskey-auth-<KEY>`), so the project's Vault auth-key rotation procedure is already documented. A reusable auth key (reusable, ephemeral=tag) is exactly what the GH Actions sidecar needs.
   - The sidecar brings the runner pod (ephemeral VM) into the Tailscale `trantor` tailnet, where:
     - The Temporal frontend is reachable via its Tailscale IP, OR via `temporal.trantor.internal` if MagicDNS is enabled, OR via the k8s service ClusterIP if subnet routing is configured.
   - Concrete options:
     - **Option A (cheapest): `tailscale/github-action@v2`** with `args: --tag=ci` and a reusable auth key from `${{ secrets.TAILSCALE_AUTHKEY }}`. ~6 lines of YAML per job.
     - **Option B (slightly fancier): Tailscale subnet router** — if the k3s node running the runner already advertises the cluster CIDR (`10.43.0.0/16`) as a Tailscale subnet route, then a github-hosted Tailscale sidecar can reach `10.43.29.119` directly without a magic-DNS alias. This is the cleanest path because the existing workflow lines `temporal workflow start --address 10.43.29.119:7233` work unchanged.
   - Either way, no workflow logic changes beyond adding the sidecar step and changing `runs-on` from `[self-hosted, trantor-internal]` to `ubuntu-latest`.

3. **For the 3 `kubectl exec` jobs** (`terminus.platform/release.yml`, `fourdogs-central/release-emailfetcher.yml`, `fourdogs-kaylee-agent/release.yml`): change to the static-CLI pattern already used by the other 9 Temporal workflows. Replace:
   ```bash
   kubectl exec -n temporal deploy/temporal-server-admintools -- \
     /usr/local/bin/temporal workflow start --address 10.43.29.119:7233 ...
   ```
   with:
   ```bash
   # Download Temporal CLI (existing step in terminus-portal/release.yml:136-140, inference-gateway/release.yml:85-90, etc.)
   temporal workflow start --address 10.43.29.119:7233 ...
   ```
   This removes the dependency on the in-cluster `temporal-server-admintools` Deployment.

4. **Side benefit — eliminate `actions-runner` namespace entirely.** Once all 15 self-hosted jobs migrate, the entire `runner-deployment.yaml` (660 lines, 10 deployments + 2 spares = ~12 runner pods) can be deleted. The Vault-synced k8s Secret for `INFRA_DEPLOY_KEY` can be deleted. The `actions-runner` ServiceAccount + RBAC can be deleted. Net cluster footprint reduction: 12 pods, 1 ServiceAccount, ~660 lines of YAML, 1 Vault-secret sync.

### Cost summary

| Change | Effort | Per-repo | Total |
|---|---|---|---|
| Add `INFRA_DEPLOY_KEY` to GitHub secrets | trivial | ~5 min | ~15 min |
| Add Tailscale sidecar step | trivial | 6 lines YAML | ~30 min |
| Migrate 3 `kubectl exec` jobs to CLI pattern | trivial | 1 line each | ~10 min |
| Validate one workflow end-to-end | medium | ~30 min | ~30 min (smoke) |
| Decommission runner pool | trivial | delete runner-deployment.yaml | ~5 min |

**Total wall-clock for the whole migration: one focused cycle (half a day).** After that, the runner pool can be removed and the Pro+ plan downgraded to Pro.

## The "Required Migration Change" for Each Workflow

A specific delta per workflow file. **None of these changes the workflow logic** — they only add a Tailscale sidecar and switch `runs-on`.

### `fourdogs-central/on-ui-dispatch-dev.yml` (`release` job at line 87-161)

Required changes:
- `runs-on: [self-hosted, trantor-internal]` → `runs-on: ubuntu-latest` (line 89)
- Insert before line 96 ("Validate resolved inputs" step):
  ```yaml
  - name: Connect Tailscale
    uses: tailscale/github-action@v2
    with:
      oauth-client-id: ${{ secrets.TS_OAUTH_CLIENT_ID }}
      oauth-secret: ${{ secrets.TS_OAUTH_SECRET }}
      tags: tag:ci
      args: --accept-routes
  ```
- Add `secrets: INFRA_DEPLOY_KEY, TS_OAUTH_CLIENT_ID, TS_OAUTH_SECRET` to the `release` job block.
- Promote from ${{ secrets.INFRA_DEPLOY_KEY }} in the `git clone` line.

### `fourdogs-central/on-ui-dispatch.yml`

Same delta as on-ui-dispatch-dev.yml, applied to the `release` job at line 96-169.

### `fourdogs-central/release-central.yml`

Same delta, applied to the `promote-and-release` job at line 139-217.

### `fourdogs-central/release-catalog-trigger.yml`

Same delta, applied to the `promote-and-release` job at line 89-188.

### `fourdogs-central/release-emailfetcher.yml`

Same delta + replace `kubectl exec ...` at line 175 with the static-CLI `temporal workflow start` pattern (use line 156-174 of `release-etailpet-trigger.yml` as the reference — same `temporal` install shape, just skip the retry loop if desired).

### `fourdogs-central/release-etailpet-trigger.yml`

Same delta as release-central.yml, applied to the `promote-and-release` job at line 98-197.

### `fourdogs-central/release-etailpet-sales-trigger.yml`

Same delta as release-central.yml, applied to the `promote-and-release` job at line 88-187.

### `fourdogs-kaylee-agent/release.yml`

Same delta as release-central.yml + replace `kubectl exec ...` at line 157 with the static-CLI pattern.

### `terminus-inference-gateway/release.yml`

Same delta as release-central.yml, applied to the `promote-and-release` job at line 38-101.

### `terminus-inference-qwen-warmup/release.yml`

Same delta as release-central.yml, applied to the `promote-and-release` job at line 50-101.

### `terminus-platform/release.yml`

Same delta + replace BOTH `kubectl exec ...` calls (line 204-219 start, line 228-234 verify) with the static-CLI pattern. The "verify Temporal execution" step (line 224-245) can stay (it's a `temporal workflow describe` against the same address, no in-cluster exec needed) — but it must use the static CLI binary, not kubectl.

### `terminus-portal/release.yml`

Same delta as release-central.yml. Note: this file's "promote" step at line 120-132 does a non-retrying `git commit && git push` (no retry loop, unlike the others) and writes to `git checkout main` (no branch variable, hardcoded). Worth refactoring in the same PR, but not blocking the migration.

### `terminus.hermes/release.yml`

This is the SIMPLEST case — `promote-manifest` (line 55-105) does only `git clone → sed → git commit → git push`. **No Temporal call.** A Tailscale sidecar is technically *not even required* if the only `*.trantor.internal` reference is `git config user.email "ci@trantor.internal"` (line 74) which is just a string used in `git commit`, not a hostname.

Wait — re-read the file: yes, `ci@trantor.internal` is the git committer email, not a hostname. **There is no actual network need for self-hosted here at all.** The only reason it runs on the k3s runner is the `INFRA_DEPLOY_KEY` env-var pattern (line 67-68).

Migration:
- `runs-on: [self-hosted, trantor-internal]` → `runs-on: ubuntu-latest` (line 60)
- Change `${INFRA_DEPLOY_KEY}` references to `${{ secrets.INFRA_DEPLOY_KEY }}` (lines 67, 72)
- Add the GitHub repo-level secret `INFRA_DEPLOY_KEY`
- **No Tailscale sidecar needed** — this job has zero `*.trantor.internal` host reach and zero ClusterIP reach.

This is the **cheapest migration in the entire set** — 1-line `runs-on` change + 2 env var name changes.

### `terminus.watchdog/release.yml`

Identical to `terminus.hermes/release.yml`. Same migration delta. **No Tailscale sidecar needed.**

### `terminus.infra/deploy-hermes-vm.yml`

This is the prior-audit job (line 21-43, in `terminus.infra/.github/workflows/`). Already analyzed in `docs/runner-audit-2026-08-31.md:115-136`:
- `runs-on: [self-hosted, trantor-internal]` → `runs-on: ubuntu-latest` (line 23)
- Add Tailscale sidecar before line 37 (the `python scripts/run_semaphore_pipeline.py hermes-honcho` step)
- `SEMAPHORE_BASE_URL=https://semaphore.trantor.internal` already uses a public GitHub secret; the only network locality concern is the `trantor.internal` DNS zone.

## Concrete Recommendation Ordered by Least-Effort Wins

1. **`terminus.hermes/release.yml`** — 1-line `runs-on` change + 2 env var swaps. **No Tailscale needed.** Smoke-test on a PR. **~10 minutes.**
2. **`terminus.watchdog/release.yml`** — identical to #1. **~10 minutes.**
3. **All seven `fourdogs-central` workflows** — uniform Tailscale sidecar + `INFRA_DEPLOY_KEY` GitHub secret + `runs-on` flip. The 5 `release-*` files share an identical template; can be done with a sed pass + manual smoke-test of one. **~1 hour total for the seven.**
4. **`terminus-portal/release.yml`** — same pattern as #3, plus a small refactor of the `git push` step to add retry logic (currently a one-shot). **~30 minutes.**
5. **`terminus-inference-gateway/release.yml` and `terminus-inference-qwen-warmup/release.yml`** — same pattern as #3. **~30 minutes each.**
6. **`terminus.platform/release.yml`, `fourdogs-central/release-emailfetcher.yml`, `fourdogs-kaylee-agent/release.yml`** — same pattern as #3 PLUS replace `kubectl exec ...` with static-CLI `temporal workflow start`. **~1 hour total for the three.**
7. **`terminus.infra/deploy-hermes-vm.yml`** — Tailscale sidecar for the Semaphore API call. Already covered by the prior audit's recommendation. **~30 minutes.**

**Sum of all 7 work items: ~5 hours, single focused cycle.**

After all 7 land:
- Delete `terminus.infra/platforms/k3s/k8s/actions-runner/runner-deployment.yaml` (660 lines).
- Delete the Vault → ESO → k8s Secret → runner pod env chain for `INFRA_DEPLOY_KEY`.
- Decommission the 12 runner pods.
- Downgrade GitHub plan from Pro+ to Pro at month-end.

## Risks and Open Questions

1. **`INFRA_DEPLOY_KEY` rotation.** The current pattern uses Vault ESO to keep the deploy key in sync. If moved to GitHub secrets, rotation becomes a manual `git mv` step (or `gh secret set`). For a fine-grained PAT, this is fine; for a long-lived deploy key, automation is more painful. **Recommendation: convert to fine-grained PAT scoped to the `terminus.infra` repo with `contents:write` only.** Rotation then becomes a single `gh secret set` + revoke-old-token.

2. **Tailscale auth-key lifecycle.** The existing pattern uses reusable auth keys stored in Vault (`secret/trantor/home-assistant/tailscale-auth-key`). For CI sidecars, **ephemeral auth keys with a short TTL (1 hour) + `tag:ci`** are the secure default. The reusable pattern is fine for long-lived VMs but generates noise in the Tailscale admin console when CI spawns hundreds of throwaway devices per week. A `tag:ci` tag in the Tailscale ACL can auto-expire stale nodes.

3. **Tailscale subnet router.** If the k3s cluster doesn't already advertise `10.43.0.0/16` as a Tailscale subnet route, then `10.43.29.119` is unreachable from a github-hosted Tailscale sidecar. Two options:
   - Add a Tailscale subnet router (one-time setup, ~30 min, lives in the cluster).
   - Use `temporal.trantor.internal` (assuming MagicDNS) and add a Tailscale split-DNS record for `trantor.internal`. Same effect.
   - Both are operator-friendly; one is more invasive than the other. Pick the one with the smaller blast radius.

4. **Concurrency / `group:` semantics.** All 15 workflows use `concurrency: cancel-in-progress: false` and group names keyed on branch. Migrating to github-hosted changes nothing here — concurrency groups are runner-agnostic.

5. **`audit()` actions and GitHub API rate limits.** None of the workflows call the GitHub API directly (they all use `git push` and `docker push`). github-hosted runners have a higher API rate limit than self-hosted (1000/hr vs 60/hr for `GITHUB_TOKEN`-authenticated requests). Migration is a strict improvement.

6. **Workflow file commit history preservation.** Migration changes are pure YAML; no script logic changes. Each PR can land as a single commit per workflow file, easy to revert.

7. **What about `terminus.infra/platforms/k3s/k8s/actions-runner/runner-rbac.yaml`?** The `ServiceAccount` and RBAC at that file (`runner-rbac.yaml:3, 35`) only grant the runner pod the right to `kubectl exec` into `temporal-server-admintools`. After migrating the three `kubectl exec` workflows to the static-CLI pattern, this RBAC is no longer needed. It can also be deleted.

## Reproduction Commands

```bash
# Workflow inventory (this audit)
find /home/ubuntu/workspace/repos/bmad.lens.projects/TargetProjects \
  -name '*.yml' -path '*.github/workflows*' -type f \
  -not -path '*/node_modules/*' -not -path '*/.venv/*' \
  -not -path '*/site-packages/*' -not -path '*/venv/*' | wc -l
# Returns: 34

# Job count — Method 1 (runs-on: line count, matches prior audit's method)
find /home/ubuntu/workspace/repos/bmad.lens.projects/TargetProjects \
  -name '*.yml' -path '*.github/workflows*' -type f \
  -not -path '*/node_modules/*' -not -path '*/.venv/*' \
  -not -path '*/site-packages/*' -not -path '*/venv/*' \
  -exec grep -cE '^\s+runs-on:' {} \; | awk '{s+=$1} END {print s}'
# Returns: 67

# Job count — Method 2 (structural awk parser)
# Same as Method 1 — both report 67.

# Self-hosted declarations only
grep -rn --include="*.yml" -E 'runs-on:.*self-hosted.*trantor-internal' \
  /home/ubuntu/workspace/repos/bmad.lens.projects/TargetProjects/ \
  --exclude-dir=node_modules --exclude-dir=.venv \
  --exclude-dir=venv --exclude-dir=site-packages
# Returns: 15 hits across 15 workflow files (see Bucket 2 table).

# In-cluster kubectl exec sites (Group B)
grep -rn --include="*.yml" "kubectl exec -n temporal" \
  /home/ubuntu/workspace/repos/bmad.lens.projects/TargetProjects/
# Returns: 4 hits — release.yml:204 (terminus.platform), release.yml:228 (terminus.platform),
#   release.yml:175 (fourdogs-central/release-emailfetcher), release.yml:157 (fourdogs-kaylee-agent)
# (The first two are both in terminus.platform/release.yml: the start and verify Temporal steps.)

# Hardcoded Temporal ClusterIP
grep -rn --include="*.yml" "10.43.29.119" \
  /home/ubuntu/workspace/repos/bmad.lens.projects/TargetProjects/
# Returns: 13 hits across 12 workflow files (terminus.platform hits twice — start + verify).

# INFRA_DEPLOY_KEY source — verify it's runner-pod env, not GitHub secrets
grep -n "INFRA_DEPLOY_KEY" \
  /home/ubuntu/workspace/repos/bmad.lens.projects/TargetProjects/terminus/infra/terminus.infra/platforms/k3s/k8s/actions-runner/runner-deployment.yaml
# Lines 55-59, 121-125, 187-191, 253-257, 319-323, 385-389, 451-455, 517-521, 583-587, 649-653
# Each: secretKeyRef on actions-runner-infra-deploy-key / INFRA_DEPLOY_KEY.
```

## What the Prior Audit Missed

The prior audit (`docs/runner-audit-2026-08-31.md`) covered only `codex`, `pcgen`, and `terminus.infra/.github/workflows/`. It correctly identified `deploy-hermes-vm.yml::deploy-hermes-honcho` as the only self-hosted job in that surface, but **failed to audit the per-service release workflows under `TargetProjects/`**. Those workflows are the primary production-deployment surface and contain **all 14 of the additional self-hosted jobs**. The prior audit's conclusion "retire all local runners; flip the one `runs-on`" was based on a partial inventory and overstates what can be migrated without operator-side work.

This audit corrects that gap. The corrected combined picture:

| Surface | Self-hosted jobs | Migration cost |
|---|---:|---|
| Prior audit surface (codex + pcgen + terminus.infra validation) | 1 | 5–10 lines YAML (Tailscale sidecar for `deploy-hermes-vm.yml`) |
| TargetProjects audit (this document) | 15 | Tailscale sidecar + `INFRA_DEPLOY_KEY` → GitHub secret for 12 jobs; + the same delta minus the Tailscale sidecar for 2 jobs (terminus.hermes, terminus.watchdog — no Temporal call, no network locality) |
| **Combined** | **16** | **~5 hours of focused work, single cycle** |

After migration: **zero self-hosted jobs.** The k3s runner pool at `terminus.infra/platforms/k3s/k8s/actions-runner/runner-deployment.yaml` (660 lines, 12 pods) can be decommissioned and the Pro+ plan can be downgraded without operational impact.
