import type { RestoreStarterCharactersResponse } from '../boundary/restoreStarterCharacters';

/**
 * The status line the Load screen's "Restore starter characters" button shows
 * after `restore_starter_characters` returns. A seed that failed is reported
 * with its real error -- never folded into "already here".
 */
export function restoreStarterCharactersMessage(response: RestoreStarterCharactersResponse): string {
  const parts: string[] = [];
  if (response.restored.length > 0) {
    parts.push(`Restored: ${response.restored.join(', ')}`);
  }
  if (response.errors.length > 0) {
    parts.push(`Could not restore: ${response.errors.join('; ')}`);
  }
  return parts.length > 0 ? parts.join('. ') : 'All starter characters are already here.';
}

/**
 * The Load screen's notice for starter characters the latest seeding pass
 * could not create (`list_starter_seed_errors`); `null` when there are none.
 */
export function starterSeedErrorsMessage(errors: string[]): string | null {
  return errors.length > 0 ? `A starter character could not be created: ${errors.join('; ')}` : null;
}
