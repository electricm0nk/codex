/**
 * Pure model behind the two-column "Manage" dialogs: options on the left, selected on the right,
 * `<` `>` to move an item across, a Qualified filter on the options, and (for spells and the like)
 * a third column of innate abilities the player cannot change. Used for racial traits, character
 * traits, feats and spells.
 */

export interface TransferItem {
  id: string;
  label: string;
  description?: string;
  /** False when the character may not take it (prerequisites unmet). It is still listed unless the Qualified filter is on. */
  qualified: boolean;
  unqualifiedReason?: string;
  /** The limit group it counts against (a spell level, say); see `groupLimits`. */
  group?: string;
  /** Granted by the build (a racial spell-like ability, say): shown in its own column, never moved. */
  innate?: boolean;
}

/** The left column: everything not yet chosen and not innate; with `qualifiedOnly`, only what the character may take. */
export function availableItems(items: TransferItem[], selected: string[], qualifiedOnly: boolean): TransferItem[] {
  return items.filter((item) => !item.innate && !selected.includes(item.id) && (!qualifiedOnly || item.qualified));
}

/** The right column, in the order the player chose them. Ids that are not options are ignored. */
export function selectedItems(items: TransferItem[], selected: string[]): TransferItem[] {
  return selected.flatMap((id) => {
    const item = items.find((candidate) => candidate.id === id);
    return item ? [item] : [];
  });
}

export function innateItems(items: TransferItem[]): TransferItem[] {
  return items.filter((item) => item.innate === true);
}

/** How many more can be chosen; `null` when the list has no cap. Never negative. */
export function remainingSelections(limit: number | null, selected: string[]): number | null {
  return limit === null ? null : Math.max(0, limit - selected.length);
}

/** Whether `id` can move to the selected column, and if not, why. */
export function canMoveRight(
  items: TransferItem[],
  selected: string[],
  id: string,
  limit: number | null,
  groupLimits?: Record<string, number>
): { ok: boolean; reason?: string } {
  const item = items.find((candidate) => candidate.id === id);
  if (item === undefined) {
    return { ok: false, reason: 'That is not one of the options.' };
  }
  if (item.innate) {
    return { ok: false, reason: 'This comes with the character and cannot be changed.' };
  }
  if (selected.includes(id)) {
    return { ok: false, reason: 'Already selected.' };
  }
  // No room outranks "not qualified": with nothing left to pick, qualification is moot.
  if (limit !== null && selected.length >= limit) {
    return { ok: false, reason: 'No selections remaining.' };
  }
  if (groupLimits !== undefined && item.group !== undefined) {
    const quota = groupLimits[item.group] ?? 0;
    const taken = selected.filter((chosen) => items.find((candidate) => candidate.id === chosen)?.group === item.group).length;
    if (taken >= quota) {
      return { ok: false, reason: `No selections remaining for ${item.group}.` };
    }
  }
  if (!item.qualified) {
    return { ok: false, reason: item.unqualifiedReason ?? 'The character does not qualify.' };
  }
  return { ok: true };
}

/** The selection after moving `id` right; unchanged when it cannot move. */
export function moveRight(
  items: TransferItem[],
  selected: string[],
  id: string,
  limit: number | null,
  groupLimits?: Record<string, number>
): string[] {
  return canMoveRight(items, selected, id, limit, groupLimits).ok ? [...selected, id] : selected;
}

/** For each limited group, how many more may be chosen, in the order the limits are declared. */
export function remainingByGroup(
  items: TransferItem[],
  selected: string[],
  groupLimits: Record<string, number>
): Array<{ key: string; limit: number; remaining: number }> {
  return Object.entries(groupLimits).map(([key, limit]) => {
    const taken = selected.filter((id) => items.find((candidate) => candidate.id === id)?.group === key).length;
    return { key, limit, remaining: Math.max(0, limit - taken) };
  });
}

/** The selection after moving `id` back to the options. */
export function moveLeft(selected: string[], id: string): string[] {
  return selected.includes(id) ? selected.filter((candidate) => candidate !== id) : selected;
}

/** What the description pane shows for the clicked item (on either side). */
export function describeFocus(
  items: TransferItem[],
  focusedId: string | null
): { title: string; body: string; note?: string } | null {
  const item = focusedId === null ? undefined : items.find((candidate) => candidate.id === focusedId);
  if (item === undefined) {
    return null;
  }
  const description = item.description?.trim();
  return {
    title: item.label,
    body: description ? description : 'No description available.',
    ...(item.qualified ? {} : { note: `Not qualified: ${item.unqualifiedReason ?? 'prerequisites are not met.'}` }),
  };
}
