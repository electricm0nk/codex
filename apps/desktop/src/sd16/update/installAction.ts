/**
 * SD-16-E7-F3c shell-side install action bridge.
 *
 * The desktop shell mounts a `#install-relaunch-prompt` DOM hook whenever the
 * staged-transaction Tauri command reports `must_relaunch: true`. This module
 * is the single seam between the React UI and the Rust staged transaction.
 *
 * The F2 handoff scoped this surface to F3a (the transaction owner). F3a's
 * PR #61 shipped the Rust side but did not author this TypeScript bridge, so
 * F3c authors it from scratch against the Rust surface F3a already merged.
 * The deviation is recorded in the F3c receipt.
 *
 * F3c extends the wire contract with a relaunch-prompt side-effect: when
 * `must_relaunch` is true, this module mounts the `#install-relaunch-prompt`
 * DOM element with the running-artifact verification seam exposed. The
 * `#install-relaunch-prompt` hook is the canonical DOM contract surface that
 * `restoreOffer.tsx` and F3b's `perform_restore_previous` integration depend
 * on.
 *
 * The Tauri 2.x `invoke` import path is `@tauri-apps/api/core` (not the older
 * `@tauri-apps/api/tauri`); this matches the canonical pattern used by every
 * other loader under `apps/desktop/src/boundary/`.
 */

import { invoke } from "@tauri-apps/api/core";
import { formatError, hasTauriRuntime } from "../../boundary/runtime";

/**
 * Response shape returned by the staged-transaction Tauri command.
 *
 * Mirrors the Rust `RelaunchPrompt` struct in
 * `apps/desktop/src-tauri/src/update/transaction.rs`.
 */
export interface PerformInstallResponse {
  mustRelaunch: boolean;
  newArtifactSha256: string;
  newManagedExecutablePath: string;
  previousBackupPath: string;
  pendingStatePath: string;
  fromVersion: string;
  toVersion: string;
}

/**
 * Outcome of `verify_relaunch_artifact` after the shell relaunches with the
 * staged binary in place. Mirrors the Rust `ReloadVerifyOutcome` enum.
 */
export type ReloadVerifyOutcome =
  | {
      kind: "promoted";
      installedStatePath: string;
      promotedVersion: string;
    }
  | {
      kind: "verification-failed";
      expected: string;
      actual: string;
    }
  | { kind: "no-pending-update" };

/**
 * DOM id the shell mounts on relaunch prompt so the test surface and the
 * restoreOffer UI can find it. The id is part of the E6+F7 contract.
 */
export const INSTALL_RELAUNCH_PROMPT_ID = "install-relaunch-prompt";

/**
 * Sentinel response used when the Tauri runtime is not available (e.g. unit
 * tests in a node tsx host). Returning a `must_relaunch: false` sentinel keeps
 * the UI honest: it surfaces "no install in flight" rather than fabricating
 * a prompt.
 */
export const NO_TAURI_RUNTIME_RESPONSE: PerformInstallResponse = {
  mustRelaunch: false,
  newArtifactSha256: "",
  newManagedExecutablePath: "",
  previousBackupPath: "",
  pendingStatePath: "",
  fromVersion: "",
  toVersion: "",
};

/**
 * Call the staged-transaction Tauri command.
 *
 * F3c is the sole author of this surface today (F3a shipped Rust without the
 * TS bridge). On a successful staged transaction with `must_relaunch: true`,
 * the function mounts the `#install-relaunch-prompt` DOM hook with the prior
 * and target versions so the operator can confirm the relaunch.
 *
 * When the Tauri runtime is absent (vite dev, vitest), the function returns
 * the no-runtime sentinel rather than throwing — the UI must surface a
 * deterministic "not wired" state per AGENTS.md's no-fake-completion rule.
 */
export async function performInstall(
  manifest: unknown,
  indexUrl: string,
): Promise<PerformInstallResponse> {
  if (!hasTauriRuntime()) {
    return NO_TAURI_RUNTIME_RESPONSE;
  }
  try {
    const response = await invoke<PerformInstallResponse>("perform_install", {
      manifest,
      indexUrl,
    });
    if (response.mustRelaunch) {
      mountRelaunchPrompt(response);
    }
    return response;
  } catch (cause: unknown) {
    throw new Error(`perform_install failed: ${formatError(cause)}`);
  }
}

/**
 * Mount (or update) the `#install-relaunch-prompt` DOM hook with the response.
 *
 * Idempotent: if a prior prompt element already exists at the canonical id,
 * it is replaced rather than duplicated. Tests assert on the id and the
 * `data-from-version` / `data-to-version` attributes via `buildRelaunchPromptMarkup`.
 */
export function mountRelaunchPrompt(response: PerformInstallResponse): void {
  if (typeof document === "undefined") {
    // Non-browser test host (e.g. node tsx): skip DOM mutation. Tests assert
    // against `buildRelaunchPromptMarkup` directly to keep the surface pure.
    return;
  }
  const existing = document.getElementById(INSTALL_RELAUNCH_PROMPT_ID);
  if (existing) {
    existing.remove();
  }
  const hook = document.createElement("div");
  hook.id = INSTALL_RELAUNCH_PROMPT_ID;
  hook.setAttribute("data-from-version", response.fromVersion);
  hook.setAttribute("data-to-version", response.toVersion);
  hook.setAttribute("data-pending-state-path", response.pendingStatePath);
  hook.setAttribute("data-managed-executable-path", response.newManagedExecutablePath);
  hook.setAttribute("data-new-artifact-sha256", response.newArtifactSha256);
  hook.setAttribute("data-previous-backup-path", response.previousBackupPath);
  hook.setAttribute("data-must-relaunch", "true");
  hook.textContent = `Relaunch required to finish update ${response.fromVersion} → ${response.toVersion}`;
  document.body.appendChild(hook);
}

/**
 * Pure renderer used by the test surface — produces the markup string that
 * the DOM hook would carry without touching the real DOM. Keeping it pure
 * makes the wire contract testable in node tsx without a jsdom dependency.
 *
 * The renderer reflects the response's `mustRelaunch` flag. The DOM hook is
 * only mounted when `mustRelaunch` is true; for `mustRelaunch: false` the
 * markup is still useful as a representation of "no relaunch is required" so
 * tests can assert on the wire contract end-to-end.
 */
export function buildRelaunchPromptMarkup(response: PerformInstallResponse): string {
  const mustRelaunchAttr = response.mustRelaunch ? "true" : "false";
  const attrs = [
    `id="${INSTALL_RELAUNCH_PROMPT_ID}"`,
    `data-from-version="${escapeAttr(response.fromVersion)}"`,
    `data-to-version="${escapeAttr(response.toVersion)}"`,
    `data-pending-state-path="${escapeAttr(response.pendingStatePath)}"`,
    `data-managed-executable-path="${escapeAttr(response.newManagedExecutablePath)}"`,
    `data-new-artifact-sha256="${escapeAttr(response.newArtifactSha256)}"`,
    `data-previous-backup-path="${escapeAttr(response.previousBackupPath)}"`,
    `data-must-relaunch="${mustRelaunchAttr}"`,
  ].join(" ");
  const text = response.mustRelaunch
    ? `Relaunch required to finish update ${escapeText(response.fromVersion)} → ${escapeText(response.toVersion)}`
    : `Install completed without relaunch`;
  return `<div ${attrs}>${text}</div>`;
}

/**
 * Build a deterministic `perform_install` mock response for tests. Mirrors
 * the Rust `RelaunchPrompt` payload shape that F3a ships.
 */
export function makePerformInstallResponseFixture(
  overrides: Partial<PerformInstallResponse> = {},
): PerformInstallResponse {
  return {
    mustRelaunch: true,
    newArtifactSha256: "abc123sha",
    newManagedExecutablePath: "/opt/codex/Codex-Desktop-Shell-Scaffold.AppImage",
    previousBackupPath: "/home/ubuntu/.config/codex-desktop-shell-scaffold/update/backups/Codex-Desktop-Shell-Scaffold.previous.AppImage",
    pendingStatePath: "/home/ubuntu/.config/codex-desktop-shell-scaffold/update/pending-update.json",
    fromVersion: "0.0.0",
    toVersion: "0.0.1",
    ...overrides,
  };
}

function escapeAttr(value: string): string {
  return value.replace(/&/g, "&amp;").replace(/"/g, "&quot;").replace(/</g, "&lt;");
}

function escapeText(value: string): string {
  return value.replace(/&/g, "&amp;").replace(/</g, "&lt;");
}