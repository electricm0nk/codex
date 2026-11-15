import type { BodySlotDto, CopyKeyDto, LocationDto, PlacedEquipmentCopyDto } from '../boundary/listHeldEquipment';
import type { ContainerLoadDto } from '../boundary/loadSavedCharacterDetail';

/**
 * The gear screen's pure model (SD-41 `decisions.md §13, §14, §27, §28, §30, §32`): left rows, the
 * filter, the right tree, each row's Move to targets, the over-capacity warnings. No number is
 * computed here: weights come from the engine's `encumbrance.perItem`, loads from `containerLoads`.
 */

/** Slots as a list of what is worn runs: the hands, then down the body; what takes no slot is last. */
export const WORN_SLOTS: ReadonlyArray<{ slot: BodySlotDto; label: string }> = [
  { slot: 'wield_main', label: 'Main hand' },
  { slot: 'wield_off', label: 'Off hand' },
  { slot: 'wield_two_handed', label: 'Both hands' },
  { slot: 'shield', label: 'Shield' },
  { slot: 'armor', label: 'Armor' },
  { slot: 'head', label: 'Head' },
  { slot: 'headband', label: 'Headband' },
  { slot: 'eyes', label: 'Eyes' },
  { slot: 'shoulders', label: 'Shoulders' },
  { slot: 'neck', label: 'Neck' },
  { slot: 'chest', label: 'Chest' },
  { slot: 'body', label: 'Body' },
  { slot: 'belt', label: 'Belt' },
  { slot: 'wrist', label: 'Wrist' },
  { slot: 'hands', label: 'Hands' },
  { slot: 'ring', label: 'Ring' },
  { slot: 'feet', label: 'Feet' },
  { slot: 'none', label: 'No slot' },
];

export function wornLabel(slot: BodySlotDto): string {
  return WORN_SLOTS.find((row) => row.slot === slot)?.label ?? slot;
}

/** Places per slot (`data/equipment_types.json` `slots[].capacity`): the Ring holds two. */
function placesOf(slot: BodySlotDto): number {
  return slot === 'ring' ? 2 : 1;
}

export type GearFilter = 'all' | 'equipped' | 'notEquipped';

export interface GearRow {
  /** `<itemId>-<bonus>-<ordinal>` of the row's first copy. */
  id: string;
  held: PlacedEquipmentCopyDto;
  label: string;
  weightLabel: string;
  locationLabel: string;
  isContainer: boolean;
  /** "x of y lb" for a one-copy container; null otherwise. */
  capacityLabel: string | null;
}

export interface GearTreeItem {
  row: GearRow;
  /** The copy this node stands for: a container stack is one node per copy. */
  copy: CopyKeyDto;
  capacityLabel: string | null;
  /** The container is over its pounds but the character ignores container capacity limits. */
  overrideTag: boolean;
  children: GearTreeItem[];
}

export interface GearTreeSection {
  /** `slot-<slot>`, `equipped`, `carried`, `notCarried`: the `gear-location-` testid suffix. */
  id: string;
  label: string;
  location: LocationDto;
  /** "used of places" on a slot; null elsewhere. */
  places: string | null;
  items: GearTreeItem[];
}

export interface GearWarning {
  copy: CopyKeyDto;
  text: string;
}

export interface GearModel {
  rows: GearRow[];
  tree: GearTreeSection[];
  warnings: GearWarning[];
  /** The override is on: the screen tags it beside the containers. */
  capacityOverride: boolean;
}

export interface GearContext {
  perItem: ReadonlyArray<{ itemId: string; weightLbs: number }>;
  containerLoads: ReadonlyArray<ContainerLoadDto>;
  ignoreContainerCapacity?: boolean;
}

export interface MoveTarget {
  id: string;
  label: string;
  location: LocationDto;
}

export function copyId(copy: CopyKeyDto): string {
  return `${copy.itemId}-${copy.bonus}-${copy.ordinal}`;
}

function sameCopy(a: CopyKeyDto, b: CopyKeyDto): boolean {
  return a.itemId === b.itemId && a.bonus === b.bonus && a.ordinal === b.ordinal;
}

/** A figure as the engine gave it, without invented precision. */
function lb(value: number): string {
  return Number.isInteger(value) ? String(value) : String(Number(value.toFixed(2)));
}

/** `"<name> ×<n>"` in pieces; a bundle's `(20)` is dropped once bought (`decisions.md §30`). */
export function stackLabel(held: PlacedEquipmentCopyDto): string {
  const name = held.baseQuantity > 1 ? held.name.replace(/\s*\(\d+\)\s*$/, '') : held.name;
  return held.quantity > 1 ? `${name} ×${held.quantity}` : name;
}

function capacityLabel(load: ContainerLoadDto | undefined): string | null {
  if (!load) {
    return null;
  }
  switch (load.capacity.kind) {
    case 'pounds':
      return `${lb(load.contentsLbs)} of ${lb(load.capacity.lbs)} lb`;
    case 'unlimited':
      return `${lb(load.contentsLbs)} lb, no limit`;
    case 'unstated':
      return `${lb(load.contentsLbs)} lb`;
  }
}

/**
 * Each row's weight from the engine's per-line rows. `perItem` names only the item, in saved order
 * over the lines the engine weighs (not Not Carried, not unresolved); a row is matched when every
 * weighed copy of its item shares one bonus, so saved order is the ordinal order. Else "—".
 */
function weightLabels(held: PlacedEquipmentCopyDto[], perItem: GearContext['perItem']): Map<PlacedEquipmentCopyDto, string> {
  const labels = new Map<PlacedEquipmentCopyDto, string>();
  for (const itemId of new Set(held.map((row) => row.itemId))) {
    const weighed = held.filter((row) => row.itemId === itemId && row.activeState !== 'Absent');
    const entries = perItem.filter((entry) => entry.itemId === itemId);
    const copies = weighed.flatMap((row) => row.copies.map((copy) => ({ row, copy }))).sort((a, b) => a.copy.ordinal - b.copy.ordinal);
    const oneBonus = new Set(weighed.map((row) => row.bonus)).size <= 1;
    const byRow = new Map<PlacedEquipmentCopyDto, number[]>();
    if (oneBonus && copies.length === entries.length) {
      copies.forEach(({ row }, index) => byRow.set(row, [...(byRow.get(row) ?? []), entries[index].weightLbs]));
    }
    for (const row of weighed) {
      const weights = byRow.get(row) ?? [];
      const one = weights.length > 0 && weights.every((weight) => weight === weights[0]);
      if (one && weights.length === 1) {
        labels.set(row, `${lb(weights[0])} lb`);
      } else if (one && row.quantity === weights.length) {
        labels.set(row, `${lb(weights[0])} lb each`);
      }
    }
  }
  return labels;
}

export function gearScreenModel(held: PlacedEquipmentCopyDto[], context: GearContext): GearModel {
  const loadOf = (copy: CopyKeyDto) => context.containerLoads.find((load) => sameCopy(load.copy, copy));
  const weights = weightLabels(held, context.perItem);
  const containerName = (copy: CopyKeyDto) => {
    const owner = held.find((row) => row.copies.some((candidate) => sameCopy(candidate, copy)));
    return owner ? stackLabel({ ...owner, quantity: 1 }) : copy.itemId;
  };
  const locationLabel = (location: LocationDto): string => {
    switch (location.kind) {
      case 'slot':
        return wornLabel(location.slot);
      case 'equipped':
        return 'Equipped';
      case 'carried':
        return 'Carried';
      case 'notCarried':
        return 'Not Carried';
      case 'inside':
        return `In ${containerName(location.container)}`;
    }
  };
  const rows: GearRow[] = held.map((row) => ({
    id: copyId(row.copies[0]),
    held: row,
    label: stackLabel(row),
    weightLabel: weights.get(row) ?? '—',
    locationLabel: locationLabel(row.location),
    isContainer: row.copies.some((copy) => loadOf(copy) !== undefined),
    capacityLabel: row.copies.length === 1 ? capacityLabel(loadOf(row.copies[0])) : null,
  }));

  const ignore = context.ignoreContainerCapacity === true;
  const placed = new Set<GearRow>();
  const nodesOf = (row: GearRow, seen: Set<string>): GearTreeItem[] => {
    placed.add(row);
    const keys = row.isContainer ? row.held.copies : [row.held.copies[0]];
    return keys.map((copy) => {
      const load = loadOf(copy);
      const inside = seen.has(copyId(copy))
        ? []
        : rows.filter((candidate) => candidate.held.location.kind === 'inside' && sameCopy(candidate.held.location.container, copy) && !placed.has(candidate));
      const deeper = new Set([...seen, copyId(copy)]);
      return {
        row,
        copy,
        capacityLabel: capacityLabel(load),
        overrideTag: ignore && load?.over === true,
        children: inside.flatMap((child) => nodesOf(child, deeper)),
      };
    });
  };
  const at = (test: (location: LocationDto) => boolean) => rows.filter((row) => test(row.held.location)).flatMap((row) => nodesOf(row, new Set()));

  const tree: GearTreeSection[] = WORN_SLOTS.filter((slot) => slot.slot !== 'none').map(({ slot, label }) => {
    const items = at((location) => location.kind === 'slot' && location.slot === slot);
    const used = copiesIn(items);
    return { id: `slot-${slot}`, label, location: { kind: 'slot', slot }, places: `${used} of ${placesOf(slot)}`, items };
  });
  tree.push({ id: 'equipped', label: 'Equipped', location: { kind: 'equipped' }, places: null, items: at((location) => location.kind === 'equipped') });
  const carried: GearTreeSection = { id: 'carried', label: 'Carried', location: { kind: 'carried' }, places: null, items: at((location) => location.kind === 'carried') };
  const notCarried: GearTreeSection = { id: 'notCarried', label: 'Not Carried', location: { kind: 'notCarried' }, places: null, items: at((location) => location.kind === 'notCarried') };
  // A row inside a copy no other row holds (or a cycle the move refused too late) still shows.
  carried.items.push(...rows.filter((row) => !placed.has(row)).flatMap((row) => nodesOf(row, new Set())));
  tree.push(carried, notCarried);

  const warnings = ignore
    ? []
    : context.containerLoads
        .filter((load) => load.over && load.capacity.kind === 'pounds')
        .map((load) => ({
          copy: load.copy,
          text: `${containerName(load.copy)} holds ${lb(load.contentsLbs)} lb, over its ${lb(load.capacity.kind === 'pounds' ? load.capacity.lbs : 0)} lb.`,
        }));

  return { rows, tree, warnings, capacityOverride: ignore };
}

/** Copies in a slot: a count of what is there, one per copy of each row (a container row is one node per copy). */
function copiesIn(items: GearTreeItem[]): number {
  return [...new Set(items.map((item) => item.row))].flatMap((row) => row.held.copies).length;
}

export function filterGearRows(rows: GearRow[], filter: GearFilter): GearRow[] {
  const equipped = (row: GearRow) => row.held.location.kind === 'slot' || row.held.location.kind === 'equipped';
  return filter === 'all' ? rows : rows.filter((row) => equipped(row) === (filter === 'equipped'));
}

/** Every place a row can go: free slots, Equipped, Carried, Not Carried, every container not it or in it. */
export function moveTargets(model: GearModel, row: GearRow): MoveTarget[] {
  const own = row.held.location;
  const inside = new Set(row.held.copies.map(copyId));
  // Copies inside the row, transitively: a move there would be a cycle.
  for (let grew = true; grew; ) {
    grew = false;
    for (const candidate of model.rows) {
      const location = candidate.held.location;
      if (location.kind === 'inside' && inside.has(copyId(location.container))) {
        for (const copy of candidate.held.copies) {
          if (!inside.has(copyId(copy))) {
            inside.add(copyId(copy));
            grew = true;
          }
        }
      }
    }
  }
  const targets: MoveTarget[] = [];
  for (const section of model.tree) {
    const location = section.location;
    const isOwn = location.kind === own.kind && (location.kind !== 'slot' || (own.kind === 'slot' && own.slot === location.slot));
    if (isOwn) {
      continue;
    }
    if (location.kind === 'slot') {
      if (copiesIn(section.items) >= placesOf(location.slot)) {
        continue;
      }
    }
    targets.push({ id: section.id, label: section.label, location });
  }
  for (const candidate of model.rows.filter((other) => other.isContainer)) {
    candidate.held.copies.forEach((copy, index) => {
      const here = own.kind === 'inside' && sameCopy(own.container, copy);
      if (inside.has(copyId(copy)) || here) {
        return;
      }
      const label = candidate.held.copies.length > 1 ? `${stackLabel({ ...candidate.held, quantity: 1 })} #${index + 1}` : stackLabel({ ...candidate.held, quantity: 1 });
      targets.push({ id: `inside-${copyId(copy)}`, label: `Into ${label}`, location: { kind: 'inside', container: copy } });
    });
  }
  return targets;
}

/** "How many?" only on a stack: 1 to its pieces (`decisions.md §28, §30`). */
export function quantityPrompt(held: PlacedEquipmentCopyDto): { min: number; max: number } | null {
  return held.quantity > 1 ? { min: 1, max: held.quantity } : null;
}

export function parseQuantity(text: string, held: PlacedEquipmentCopyDto): number | string {
  const value = Number(text.trim());
  return Number.isInteger(value) && value >= 1 && value <= held.quantity ? value : `Enter a whole number from 1 to ${held.quantity}.`;
}

/** The Move to menu's keys: arrows move (wrapping), Esc closes, anything else is the browser's. */
export function menuKeyStep(index: number, key: string, count: number): number | 'close' | null {
  switch (key) {
    case 'ArrowDown':
      return (index + 1) % count;
    case 'ArrowUp':
      return (index - 1 + count) % count;
    case 'Escape':
      return 'close';
    default:
      return null;
  }
}
