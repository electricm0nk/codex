import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Anonymous reports through the codex-web API (`decisions.md §23`). The request is made by the
 * `send_report` Tauri command in `report_api.rs`, which holds the build-stamped key; this boundary
 * never sees the key, the endpoint or the 8 KB count — it renders the outcome Rust returns.
 */

export type ReportKind = 'bug' | 'enhancement';

export interface ReportRequest {
  kind: ReportKind;
  title: string;
  body: string;
}

export type ReportOutcome =
  | { outcome: 'sent'; message: string }
  | { outcome: 'unavailable'; reason: string }
  | { outcome: 'invalid'; reason: string }
  | { outcome: 'rejected'; status: number; reason: string }
  | { outcome: 'no-network'; reason: string }
  | { outcome: 'timed-out'; reason: string };

export interface ReportAvailability {
  available: boolean;
  reason: string | null;
}

export async function loadReportAvailability(): Promise<ReportAvailability> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for checking anonymous reports');
  }
  try {
    return await invoke<ReportAvailability>('report_api_availability');
  } catch (cause: unknown) {
    throw new Error(`Failed to check anonymous reports: ${formatError(cause)}`);
  }
}

export async function sendReport(request: ReportRequest): Promise<ReportOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for sending a report');
  }
  try {
    return await invoke<ReportOutcome>('send_report', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to send the report: ${formatError(cause)}`);
  }
}
