import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary reporting which build of the Rust backend is
 * actually running: its crate version and the git commit it was compiled
 * from. Successfully reaching this command is itself proof the Tauri IPC
 * bridge to the Rust backend is alive, independent of any feature check.
 */

export interface BackendHealthSnapshot {
  version: string;
  gitCommit: string;
}

export async function loadBackendHealth(): Promise<BackendHealthSnapshot> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for the backend health check');
  }

  try {
    return await invoke<BackendHealthSnapshot>('load_backend_health');
  } catch (cause: unknown) {
    throw new Error(`Failed to load backend health: ${formatError(cause)}`);
  }
}
