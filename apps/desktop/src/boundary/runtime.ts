/**
 * Shared Tauri-boundary runtime helpers.
 *
 * Every boundary loader (and the SD-11 submit flows) previously carried its
 * own identical copy of these; they live here so the runtime detection and
 * error formatting cannot drift between surfaces.
 */

export function hasTauriRuntime(): boolean {
  return typeof window !== 'undefined' && ('__TAURI_INTERNALS__' in window || '__TAURI__' in window);
}

export function formatError(cause: unknown): string {
  return cause instanceof Error ? cause.message : String(cause);
}
