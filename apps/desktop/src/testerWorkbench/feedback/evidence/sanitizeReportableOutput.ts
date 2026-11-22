export const INTERNAL_CONTEXT_REMOVED_MARKER = '_[internal context block removed from reportable output]_';

const MEMORY_CONTEXT_BLOCK = /<memory-context\b[^>]*>[\s\S]*?<\/memory-context>/gi;
const MEMORY_CONTEXT_TAIL = /<memory-context\b[^>]*>[\s\S]*$/gi;
const RECALLED_MEMORY_SYSTEM_NOTE_TAIL = /\n?\[System note: The following is recalled memory context[\s\S]*$/gi;

/**
 * Defensive last-mile sanitizer for any tester-facing, copyable, or reportable
 * output. The app must preserve ordinary tester evidence while stripping
 * Hermes/Honcho recalled-memory context blocks that can be appended outside the
 * product boundary by an agent or transcript surface.
 */
export function sanitizeReportableOutput(value: string): string {
  let sanitized = value;

  sanitized = sanitized.replace(MEMORY_CONTEXT_BLOCK, replacementWithLineBreaks);
  sanitized = sanitized.replace(MEMORY_CONTEXT_TAIL, replacementWithLineBreaks);
  sanitized = sanitized.replace(RECALLED_MEMORY_SYSTEM_NOTE_TAIL, (match) => {
    const startsWithNewline = match.startsWith('\n');
    return `${startsWithNewline ? '\n' : ''}${INTERNAL_CONTEXT_REMOVED_MARKER}`;
  });

  return sanitized
    .replace(new RegExp(`${escapeRegExp(INTERNAL_CONTEXT_REMOVED_MARKER)}(?:\\s*${escapeRegExp(INTERNAL_CONTEXT_REMOVED_MARKER)})+`, 'g'), INTERNAL_CONTEXT_REMOVED_MARKER)
    .replace(/[ \t]+\n/g, '\n')
    .trim();
}

function replacementWithLineBreaks(match: string): string {
  const prefix = match.startsWith('\n') ? '\n' : '';
  const suffix = match.endsWith('\n') ? '\n' : '';
  return `${prefix}${INTERNAL_CONTEXT_REMOVED_MARKER}${suffix}`;
}

function escapeRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}
