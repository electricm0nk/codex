import type { AdventureLogEntry, DateOrRange } from '../boundary/campaignFolder/adventureLog';

/** The Adventure Log tab's pure display rules (SD-42 `decisions.md §15.2`); Rust reads and sorts. */

export function sessionLabel(session: DateOrRange | null): string {
  if (!session) return '—';
  return session.end ? `${session.start} – ${session.end}` : session.start;
}

/** The title "+" proposes: one past the number of entries. */
export function nextSessionTitle(entries: readonly AdventureLogEntry[]): string {
  return `Session ${entries.length + 1}`;
}

/** `YYYY-MM-DD` of `date` in the user's own time zone (the session is a real, local play date). */
export function localIsoDate(date: Date): string {
  const pad = (n: number) => String(n).padStart(2, '0');
  return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}`;
}

/** SD-o: an entry whose frontmatter Codex could not read is listed by filename with this mark. */
export function statusMark(entry: AdventureLogEntry): string | null {
  return entry.frontmatterOk ? null : 'frontmatter not read';
}
