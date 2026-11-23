import { useEffect, useRef, useState, type CSSProperties, type DragEvent, type HTMLAttributes, type KeyboardEvent, type MouseEvent, type ReactNode } from 'react';
import type { HeldEquipmentDetailDto } from '../boundary/describeHeldEquipment';
import { discardEquipmentCopy } from '../boundary/discardEquipmentCopy';
import { activateEquipmentSet, createEquipmentSet, deleteEquipmentSet, renameEquipmentSet } from '../boundary/equipmentSets';
import type { CopyKeyDto, LocationDto, PlacedEquipmentCopyDto } from '../boundary/listHeldEquipment';
import { moveEquipmentCopy } from '../boundary/moveEquipmentCopy';
import { renameEquipmentCopy } from '../boundary/renameEquipmentCopy';
import { CustomizeCopyDialog, renameRequest, type CustomizeTarget } from './CustomizeCopyDialog';
import { GearDetail } from './GearDetail';
import { listMarket } from '../boundary/listMarket';
import { MARKET_CATEGORIES, rowOfHeldCopy } from './MarketDialog';
import { mapMarketTableRows } from './marketModel';
import { SellDialog } from './SellDialog';
import { sellBundleGp } from './sellDialogModel';
import type { EquipmentSetsSummaryDto } from '../boundary/loadSavedCharacterDetail';
import { OverrideTag } from './OverrideTag';
import {
  copyId,
  doubleClickTarget,
  DRAG_FORMAT,
  dropTarget,
  ROW_CONTROLS,
  rowActivates,
  insideZoneId,
  filterGearRows,
  gearScreenModel,
  menuKeyStep,
  moveTargets,
  parseQuantity,
  quantityPrompt,
  type GearContext,
  type GearFilter,
  type GearModel,
  type GearRow,
  type GearTreeItem,
} from './gearScreenModel';

const DROP_SELL_DISCARD_CAPTION =
  'Drop keeps the item and marks it Not Carried; Sell opens the sell window (Market Rate, Build or Negotiate); Discard removes it for nothing.';

const panel: CSSProperties = { border: '1px solid var(--color-border)', borderRadius: 6, minWidth: 0, padding: '0.5rem 0.75rem' };
/** A container's light box (today's panel border, tighter); nested boxes sit inside the parent's. */
const box: CSSProperties = { ...panel, margin: '0.25rem 0', padding: '0.1rem 0.5rem' };
const rowStyle: CSSProperties = { alignItems: 'center', borderBottom: '1px solid var(--color-border)', display: 'flex', flexWrap: 'wrap', gap: '0.4rem', justifyContent: 'space-between', padding: '0.25rem 0' };
const quiet: CSSProperties = { color: 'var(--color-text-faint)', fontSize: '0.75rem' };
const danger: CSSProperties = { color: 'var(--color-danger, #c0392b)', fontSize: '0.8rem', margin: '0 0 0.25rem' };
const button: CSSProperties = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 4,
  color: 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '0.7rem',
  padding: '0.2rem 0.55rem',
};
const input: CSSProperties = { fontSize: '0.75rem', padding: '0.15rem 0.3rem', width: '5rem' };

type PendingKind = 'move' | 'drop' | 'sell' | 'discard';
interface Pending {
  kind: PendingKind;
  row: GearRow;
  location?: LocationDto;
  targetLabel?: string;
}

export interface GearSetActions {
  activate: (name: string) => void;
  create: (name: string, copyActive: boolean) => void;
  rename: (to: string) => void;
  remove: () => void;
}

export interface GearPanelsProps {
  model: GearModel;
  /** The sheet's Armor Class, as the Defense tab shows it. */
  armorClass: number | null;
  /** `encumbrance.totalCarriedWeightLbs`, the engine's. */
  carriedLbs: number | null;
  sets?: EquipmentSetsSummaryDto;
  busy: boolean;
  /** The backend's refusal of the last change, word for word. */
  refusal: string | null;
  /** `swapOut` (a double-click on a full slot): that row goes to Carried first, in the same write. */
  onMove: (row: GearRow, location: LocationDto, quantity?: number, swapOut?: { row: GearRow; quantity?: number }) => void;
  onDrop: (row: GearRow, quantity?: number) => void;
  onSell: (row: GearRow, quantity?: number, negotiatedGp?: number) => void;
  /** The sell window's bundle price for a row (`sellBundleGp`); `null` when none is listed. */
  loadSellPrice: (row: GearRow) => Promise<number | null>;
  onDiscard: (row: GearRow, quantity?: number) => void;
  onSets: GearSetActions;
  /** Customize's Save: names `copy` `text` (blank restores the record's name). */
  onCustomizeName: (copy: CopyKeyDto, text: string) => void;
  /** Customize's Attach modifier: the Equipmods picker for this copy. */
  onCustomizeModifier: (target: CustomizeTarget) => void;
  /** The selected row's id (`aria-selected`), whose detail panel shows under the tables. */
  selectedId?: string | null;
  /** A click on a row, outside its buttons, selects it. */
  onSelect?: (row: GearRow) => void;
  /** The selected row's detail panel (SD-42 `decisions.md §27.4`). */
  detail?: ReactNode;
}

/** A "Move to" button and its keyboard-complete `role="menu"` (`decisions.md §14`). */
function MoveToMenu(props: { model: GearModel; row: GearRow; busy: boolean; onChoose: (location: LocationDto, label: string) => void }) {
  const [open, setOpen] = useState(false);
  const menu = useRef<HTMLDivElement>(null);
  const opener = useRef<HTMLButtonElement>(null);
  const targets = open ? moveTargets(props.model, props.row) : [];
  const items = () => Array.from(menu.current?.querySelectorAll<HTMLButtonElement>('[role="menuitem"]') ?? []);
  useEffect(() => {
    if (open) {
      items()[0]?.focus();
    }
  }, [open]);
  const close = () => {
    setOpen(false);
    opener.current?.focus();
  };
  const onKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    const list = items();
    const step = menuKeyStep(Math.max(0, list.indexOf(document.activeElement as HTMLButtonElement)), event.key, list.length);
    if (step === 'close') {
      event.preventDefault();
      close();
    } else if (step !== null) {
      event.preventDefault();
      list[step]?.focus();
    }
  };
  return (
    <span style={{ position: 'relative' }}>
      <button
        ref={opener}
        type="button"
        data-testid="gear-move-to"
        aria-haspopup="menu"
        aria-expanded={open}
        aria-label={`Move ${props.row.label} to`}
        disabled={props.busy}
        onClick={() => setOpen(!open)}
        style={button}
      >
        Move to
      </button>
      {open ? (
        <div
          ref={menu}
          role="menu"
          aria-label={`Move ${props.row.label} to`}
          onKeyDown={onKeyDown}
          style={{
            backgroundColor: 'var(--color-surface)',
            border: '1px solid var(--color-border)',
            borderRadius: 4,
            display: 'flex',
            flexDirection: 'column',
            maxHeight: 280,
            overflowY: 'auto',
            position: 'absolute',
            right: 0,
            zIndex: 10,
          }}
        >
          {targets.map((target) => (
            <button
              key={target.id}
              type="button"
              role="menuitem"
              data-testid={`gear-move-target-${target.id}`}
              onClick={() => {
                setOpen(false);
                props.onChoose(target.location, target.label);
              }}
              style={{ ...button, border: 'none', borderRadius: 0, textAlign: 'left', whiteSpace: 'nowrap' }}
            >
              {target.label}
            </button>
          ))}
        </div>
      ) : null}
    </span>
  );
}

/** The inline "how many?" (and Discard's confirm) under a row; Sell has its own window. */
function PendingForm(props: { pending: Pending; busy: boolean; onConfirm: (quantity?: number) => void; onCancel: () => void }) {
  const held = props.pending.row.held;
  const prompt = quantityPrompt(held);
  const [count, setCount] = useState(String(held.quantity));
  const [error, setError] = useState<string | null>(null);
  const verb = { move: `Move to ${props.pending.targetLabel ?? ''}`.trim(), drop: 'Drop', sell: 'Sell', discard: 'Discard' }[props.pending.kind];
  const confirm = () => {
    const quantity = prompt ? parseQuantity(count, held) : undefined;
    if (typeof quantity === 'string') {
      setError(quantity);
      return;
    }
    props.onConfirm(quantity);
  };
  return (
    <div role="group" aria-label={`${verb} ${props.pending.row.label}`} style={{ display: 'flex', flexWrap: 'wrap', gap: '0.4rem', padding: '0.25rem 0 0.4rem 1rem' }}>
      {props.pending.kind === 'discard' ? <span style={{ fontSize: '0.75rem' }}>{`Discard ${props.pending.row.label} for nothing?`}</span> : null}
      {prompt ? (
        <label style={{ fontSize: '0.75rem' }}>
          How many?{' '}
          <input type="number" min={prompt.min} max={prompt.max} step={1} value={count} onChange={(event) => setCount(event.target.value)} style={input} />
          {` of ${prompt.max}`}
        </label>
      ) : null}
      <button type="button" disabled={props.busy} onClick={confirm} style={button}>
        {verb}
      </button>
      <button type="button" onClick={props.onCancel} style={button}>
        Cancel
      </button>
      {error ? <span role="alert" style={danger}>{error}</span> : null}
    </div>
  );
}

function SetPicker(props: { sets: EquipmentSetsSummaryDto; busy: boolean; actions: GearSetActions }) {
  const [form, setForm] = useState<null | 'new' | 'rename' | 'delete'>(null);
  const [name, setName] = useState('');
  const [copyActive, setCopyActive] = useState(true);
  const open = (next: 'new' | 'rename' | 'delete') => {
    setForm(next);
    setName(next === 'rename' ? props.sets.active : '');
  };
  return (
    <div style={{ alignItems: 'center', display: 'flex', flexWrap: 'wrap', gap: '0.4rem', marginBottom: '0.5rem' }}>
      <label style={{ fontSize: '0.8rem' }}>
        Equipment set{' '}
        <select data-testid="gear-set-select" value={props.sets.active} disabled={props.busy} onChange={(event) => props.actions.activate(event.target.value)}>
          {props.sets.names.map((setName) => (
            <option key={setName} value={setName}>
              {setName}
            </option>
          ))}
        </select>
      </label>
      <button type="button" data-testid="gear-set-new" disabled={props.busy} onClick={() => open('new')} style={button}>
        New
      </button>
      <button type="button" data-testid="gear-set-rename" disabled={props.busy} onClick={() => open('rename')} style={button}>
        Rename
      </button>
      <button type="button" data-testid="gear-set-delete" disabled={props.busy || props.sets.names.length <= 1} onClick={() => open('delete')} style={button}>
        Delete
      </button>
      {form === 'new' || form === 'rename' ? (
        <span style={{ alignItems: 'center', display: 'flex', flexWrap: 'wrap', gap: '0.4rem' }}>
          <input aria-label="Set name" data-testid="gear-set-name" value={name} maxLength={40} onChange={(event) => setName(event.target.value)} style={{ ...input, width: '10rem' }} />
          {form === 'new' ? (
            <label style={{ fontSize: '0.75rem' }}>
              <input type="checkbox" checked={copyActive} onChange={(event) => setCopyActive(event.target.checked)} />
              {` Copy of ${props.sets.active}`}
            </label>
          ) : null}
          <button
            type="button"
            data-testid="gear-set-save"
            disabled={props.busy}
            onClick={() => {
              setForm(null);
              if (form === 'new') {
                props.actions.create(name, copyActive);
              } else {
                props.actions.rename(name);
              }
            }}
            style={button}
          >
            {form === 'new' ? 'Create' : 'Rename'}
          </button>
          <button type="button" onClick={() => setForm(null)} style={button}>
            Cancel
          </button>
        </span>
      ) : null}
      {form === 'delete' ? (
        <span style={{ alignItems: 'center', display: 'flex', gap: '0.4rem' }}>
          <span style={{ fontSize: '0.75rem' }}>{`Delete the set ${props.sets.active}?`}</span>
          <button
            type="button"
            data-testid="gear-set-delete-confirm"
            disabled={props.busy}
            onClick={() => {
              setForm(null);
              props.actions.remove();
            }}
            style={button}
          >
            Delete
          </button>
          <button type="button" onClick={() => setForm(null)} style={button}>
            Cancel
          </button>
        </span>
      ) : null}
    </div>
  );
}

/** The gear screen: one column under the equipment set, a section per location, each row with its buttons on its line (SD-43 E1.2). */
export function GearPanels(props: GearPanelsProps) {
  const [filter, setFilter] = useState<GearFilter>('all');
  const [pending, setPending] = useState<(Pending & { at: string }) | null>(null);
  const [customizing, setCustomizing] = useState<(CustomizeTarget & { row: GearRow }) | null>(null);
  const model = props.model;
  const customize = (row: GearRow, copy: CopyKeyDto, title: string, testId: string) => (
    <button type="button" data-testid={testId} aria-label={`Customize ${title}`} disabled={props.busy} onClick={() => setCustomizing({ row, copy, title })} style={button}>
      Customize
    </button>
  );

  const run = (next: Pending, at: string) => {
    const needsForm = next.kind === 'sell' || next.kind === 'discard' || quantityPrompt(next.row.held) !== null;
    if (needsForm) {
      setPending({ ...next, at });
    } else if (next.kind === 'move' && next.location) {
      props.onMove(next.row, next.location);
    } else if (next.kind === 'drop') {
      props.onDrop(next.row);
    }
  };
  const confirm = (quantity?: number, negotiatedGp?: number) => {
    if (!pending) {
      return;
    }
    setPending(null);
    switch (pending.kind) {
      case 'move':
        props.onMove(pending.row, pending.location!, quantity);
        break;
      case 'drop':
        props.onDrop(pending.row, quantity);
        break;
      case 'sell':
        props.onSell(pending.row, quantity, negotiatedGp);
        break;
      case 'discard':
        props.onDiscard(pending.row, quantity);
        break;
    }
  };
  const formAt = (at: string) =>
    pending && pending.kind !== 'sell' && pending.at === at ? <PendingForm key={`${pending.kind}-${at}`} pending={pending} busy={props.busy} onConfirm={confirm} onCancel={() => setPending(null)} /> : null;
  // Drag (`decisions.md §14`, SD-g): HTML5 only. `dragover` cannot read the drag data, so the dragged
  // row's id is kept here too; the drop itself reads the data.
  const dragging = useRef('');
  const dragSource = (row: GearRow): HTMLAttributes<HTMLDivElement> => ({
    draggable: !props.busy,
    onDragStart: (event: DragEvent<HTMLDivElement>) => {
      event.stopPropagation();
      event.dataTransfer.setData(DRAG_FORMAT, row.id);
      event.dataTransfer.effectAllowed = 'move';
      dragging.current = row.id;
    },
    onDragEnd: () => {
      dragging.current = '';
    },
  });
  // A container box (`swallow`) swallows every drop over it, so a refused or same-container drop never falls through to an outer box or section.
  const dropZone = (targetId: string, at: string, swallow = false): HTMLAttributes<HTMLDivElement> => ({
    onDragOver: (event: DragEvent<HTMLDivElement>) => {
      if (dropTarget({ getData: (format) => (format === DRAG_FORMAT ? dragging.current : '') }, targetId, model)) {
        event.preventDefault();
        event.stopPropagation();
        event.dataTransfer.dropEffect = 'move';
      } else if (swallow) {
        event.stopPropagation();
      }
    },
    onDrop: (event: DragEvent<HTMLDivElement>) => {
      event.preventDefault();
      event.stopPropagation();
      dragging.current = '';
      const location = dropTarget(event.dataTransfer, targetId, model);
      const row = model.rows.find((candidate) => candidate.id === event.dataTransfer.getData(DRAG_FORMAT));
      if (location && row) {
        // The same call the Move to menu makes: a stack asks "how many?" first.
        run({ kind: 'move', row, location, targetLabel: moveTargets(model, row).find((target) => target.id === targetId)?.label }, at);
      }
    },
  });
  const tag = (row: GearRow) => (row.held.override ? <OverrideTag flag={row.held.override} /> : null);
  // A row is selected by a click on it, not on one of its buttons, menus or forms.
  // Double-click or Enter on a row (SD-43 E1.4): the Move to write, to the slot or to Carried; a slot-less row says why.
  const [note, setNote] = useState<{ at: string; text: string } | null>(null);
  const activate = (row: GearRow, at: string) => (event: MouseEvent<HTMLElement> | KeyboardEvent<HTMLElement>) => {
    if (props.busy || !rowActivates(event)) {
      return;
    }
    event.preventDefault();
    const target = doubleClickTarget(model, row);
    if ('reason' in target) {
      setNote({ at, text: target.reason });
      return;
    }
    setNote(null);
    props.onMove(row, target.location, target.quantity, target.swapOut);
  };
  const selectable = (row: GearRow, at: string) => ({
    'aria-selected': props.selectedId === row.id,
    tabIndex: 0,
    onClick: (event: MouseEvent<HTMLElement>) => {
      if (!(event.target as HTMLElement).closest(ROW_CONTROLS)) {
        props.onSelect?.(row);
      }
    },
    onDoubleClick: activate(row, at),
    onKeyDown: activate(row, at),
  });

  // One held row with its buttons on its line (operator 5-a); a container's capacity and Customize read the copy it stands for.
  const gearRow = (item: GearTreeItem, at: string) => {
    const row = item.row;
    return (
      <div {...dragSource(row)} style={rowStyle}>
        <span>
          {row.label}
          {tag(row)}
          <span style={{ ...quiet, marginLeft: '0.5rem' }}>
            {`${row.weightLabel} · ${row.locationLabel}`}
            {item.capacityLabel ? ` · ${item.capacityLabel}` : ''}
          </span>
          {item.overrideTag ? <OverrideTag flag="ignoreContainerCapacity" /> : null}
          {(row.held.properties ?? []).map((property) => (
            <span key={property} data-testid="gear-property" style={{ ...quiet, display: 'block' }}>
              {property}
            </span>
          ))}
        </span>
        <span style={{ display: 'flex', flexShrink: 0, gap: '0.3rem' }}>
          {customize(row, item.copy, row.label, `gear-row-customize-${row.id}`)}
          <MoveToMenu model={model} row={row} busy={props.busy} onChoose={(location, label) => run({ kind: 'move', row, location, targetLabel: label }, at)} />
          {row.held.location.kind !== 'notCarried' ? (
            <button type="button" data-testid="gear-drop" aria-label={`Drop ${row.label}`} disabled={props.busy} onClick={() => run({ kind: 'drop', row }, at)} style={button}>
              Drop
            </button>
          ) : null}
          {row.held.itemId.startsWith('custom:') ? null : (
            <button type="button" data-testid="gear-sell" aria-label={`Sell ${row.label}`} disabled={props.busy} onClick={() => run({ kind: 'sell', row }, at)} style={button}>
              Sell
            </button>
          )}
          <button type="button" data-testid="gear-discard" aria-label={`Discard ${row.label}`} disabled={props.busy} onClick={() => run({ kind: 'discard', row }, at)} style={button}>
            Discard
          </button>
        </span>
      </div>
    );
  };
  // A loose row, or a container's light box holding its own row and, indented, its contents (operator 5-b).
  const gearItem = (item: GearTreeItem): ReactNode => {
    const at = `row:${item.row.id}`;
    const line = (
      <div data-testid={`gear-row-${item.row.id}`} {...selectable(item.row, at)}>
        {gearRow(item, at)}
        {note && note.at === at ? (
          <p role="status" data-testid="gear-row-note" style={{ ...quiet, margin: '0 0 0.25rem 1rem' }}>
            {note.text}
          </p>
        ) : null}
        {formAt(at)}
      </div>
    );
    if (!item.row.isContainer) {
      return <div key={at}>{line}</div>;
    }
    return (
      <div key={at} data-testid={`gear-container-${copyId(item.copy)}`} {...dropZone(insideZoneId(item.copy), at, true)} style={box}>
        {line}
        <div style={{ paddingLeft: '1rem' }}>{item.children.length === 0 ? <p style={{ ...quiet, margin: '0.2rem 0' }}>Empty</p> : item.children.map(gearItem)}</div>
      </div>
    );
  };
  // The filter narrows each section's top-level rows; a container shows all it holds.
  const shown = new Set(filterGearRows(model.rows, filter));
  const sections = model.tree
    .map((section) => ({ ...section, items: section.items.filter((item) => shown.has(item.row)) }))
    .filter((section) => filter === 'all' || section.items.length > 0);

  return (
    <section aria-label="Gear" className="no-print" style={{ marginBottom: '1.25rem' }}>
      <h3 style={{ fontSize: '0.95rem', margin: '0 0 0.25rem' }}>Gear</h3>
      <p style={{ fontSize: '0.85rem', margin: '0 0 0.5rem' }}>
        {props.armorClass !== null ? `AC ${props.armorClass}` : null}
        {props.armorClass !== null && props.carriedLbs !== null ? ' · ' : null}
        {props.carriedLbs !== null ? `${Number(props.carriedLbs.toFixed(2))} lb carried` : null}
      </p>
      {props.sets ? <SetPicker sets={props.sets} busy={props.busy} actions={props.onSets} /> : null}
      {props.refusal ? (
        <p role="alert" style={danger}>
          {props.refusal}
        </p>
      ) : null}
      <div style={panel}>
        {model.warnings.map((warning) => (
          <p key={copyId(warning.copy)} role="alert" data-testid={`gear-over-${copyId(warning.copy)}`} style={danger}>
            {warning.text}
          </p>
        ))}
        <label style={{ fontSize: '0.8rem' }}>
          Show{' '}
          <select data-testid="gear-filter" value={filter} onChange={(event) => setFilter(event.target.value as GearFilter)}>
            <option value="all">All</option>
            <option value="equipped">Equipped</option>
            <option value="notEquipped">Not equipped</option>
          </select>
        </label>
        {model.rows.length === 0 ? <p style={quiet}>Nothing held yet.</p> : null}
        {sections.map((section) => (
          <div key={section.id} {...dropZone(section.id, `section:${section.id}`)}>
            <div data-testid={`gear-location-${section.id}`} style={{ fontSize: '0.8rem', fontWeight: 700, marginTop: '0.4rem' }}>
              {section.label}
              {section.places ? <span style={{ ...quiet, fontWeight: 400, marginLeft: '0.4rem' }}>{section.places}</span> : null}
              {section.location.kind === 'notCarried' ? <span style={{ ...quiet, fontWeight: 400, marginLeft: '0.4rem' }}>Not counted in carried weight</span> : null}
            </div>
            {formAt(`section:${section.id}`)}
            {section.items.map(gearItem)}
          </div>
        ))}
        <p style={{ ...quiet, marginTop: '0.5rem' }}>{DROP_SELL_DISCARD_CAPTION}</p>
        {props.detail}
        {pending?.kind === 'sell' ? (
          <SellDialog
            key={`sell-${pending.row.id}`}
            label={pending.row.label}
            held={pending.row.held}
            busy={props.busy}
            loadBundleGp={() => props.loadSellPrice(pending.row)}
            onConfirm={confirm}
            onCancel={() => setPending(null)}
          />
        ) : null}
        {customizing ? (
          <CustomizeCopyDialog
            key={copyId(customizing.copy)}
            held={customizing.row.held}
            target={customizing}
            busy={props.busy}
            onRename={(copy, text) => {
              setCustomizing(null);
              props.onCustomizeName(copy, text);
            }}
            onAttach={(target) => {
              setCustomizing(null);
              props.onCustomizeModifier({ copy: target.copy, title: target.title });
            }}
            onClose={() => setCustomizing(null)}
          />
        ) : null}
      </div>
    </section>
  );
}

export interface GearScreenProps {
  characterId: string;
  load: () => Promise<PlacedEquipmentCopyDto[]>;
  /** A new value re-reads the held list: any saved change arrives as a new detail. */
  refreshKey: unknown;
  context: GearContext;
  armorClass: number | null;
  carriedLbs: number | null;
  sets?: EquipmentSetsSummaryDto;
  /**
   * Runs one write (a move, a discard, a set change) with its `savedAt`, then re-reads the sheet;
   * resolves to the refusal, word for word, or null when it saved.
   */
  onChange: (write: (savedAt: string) => Promise<unknown>) => Promise<string | null>;
  /** Sells pieces of a stack for the sell window's gp total (none when no price is listed); resolves like `onChange`. */
  onSell: (held: PlacedEquipmentCopyDto, quantity?: number, negotiatedGp?: number) => Promise<string | null>;
  /** Customize's Attach modifier: opens the Equipmods picker for this copy. */
  onPickModifier: (target: CustomizeTarget) => void;
  /** Reads the detail panel's data for one held copy (`describe_held_equipment`). */
  describe: (copy: CopyKeyDto) => Promise<HeldEquipmentDetailDto>;
}

/**
 * The Gear tab's gear screen (SD-41 `decisions.md §13`): replaces the Equipment panel. Reads the held
 * stacks off `list_held_equipment` whenever `refreshKey` changes; every button is a real write.
 */
export function GearScreen(props: GearScreenProps) {
  const [copies, setCopies] = useState<PlacedEquipmentCopyDto[] | null>(null);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [refusal, setRefusal] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [detail, setDetail] = useState<{ id: string; dto: HeldEquipmentDetailDto } | { id: string; error: string } | null>(null);
  const model = copies === null ? null : gearScreenModel(copies, props.context);
  // A sold, discarded or regrouped row is no longer there to describe.
  const selected = model?.rows.find((row) => row.id === selectedId) ?? null;

  useEffect(() => {
    if (!selected) {
      setDetail(null);
      return;
    }
    let current = true;
    const id = selected.id;
    props
      .describe(selected.held.copies[0])
      .then((dto) => current && setDetail({ id, dto }))
      .catch((cause: unknown) => current && setDetail({ id, error: cause instanceof Error ? cause.message : String(cause) }));
    return () => {
      current = false;
    };
    // `describe` is a new closure each render; the row and the held list are what change it.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [selected?.id, copies]);

  useEffect(() => {
    let current = true;
    props
      .load()
      .then((loaded) => {
        if (current) {
          setCopies(loaded);
          setLoadError(null);
        }
      })
      .catch((cause: unknown) => {
        if (current) {
          setLoadError(cause instanceof Error ? cause.message : String(cause));
        }
      });
    return () => {
      current = false;
    };
    // `load` is a new closure each render; `refreshKey` is what says the character changed.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [props.refreshKey]);

  const act = (result: Promise<string | null>) => {
    setBusy(true);
    setRefusal(null);
    void result.then(setRefusal).finally(() => setBusy(false));
  };
  const write = (call: (savedAt: string) => Promise<unknown>) => act(props.onChange(call));
  const characterId = props.characterId;
  const active = props.sets?.active ?? '';

  if (loadError) {
    return (
      <p role="alert" style={danger}>
        {loadError}
      </p>
    );
  }
  if (model === null) {
    return <p style={quiet}>Loading what this character holds.</p>;
  }
  const shown = selected && detail?.id === selected.id ? detail : null;
  return (
    <GearPanels
      model={model}
      selectedId={selected?.id ?? null}
      onSelect={(row) => setSelectedId(row.id)}
      detail={
        shown === null ? null : 'dto' in shown ? (
          <GearDetail detail={shown.dto} locationLabel={selected!.locationLabel} />
        ) : (
          <p role="alert" style={danger}>
            {shown.error}
          </p>
        )
      }
      armorClass={props.armorClass}
      carriedLbs={props.carriedLbs}
      sets={props.sets}
      busy={busy}
      refusal={refusal}
      onMove={(row, location, quantity, swapOut) =>
        write(async (savedAt) => {
          if (swapOut) {
            const out = await moveEquipmentCopy({ characterId, copy: swapOut.row.held.copies[0], location: { kind: 'carried' }, quantity: swapOut.quantity, savedAt });
            if (out.kind === 'Blocked') {
              return out;
            }
          }
          return moveEquipmentCopy({ characterId, copy: row.held.copies[0], location, quantity, savedAt });
        })
      }
      onDrop={(row, quantity) => write((savedAt) => moveEquipmentCopy({ characterId, copy: row.held.copies[0], location: { kind: 'notCarried' }, quantity, savedAt }))}
      onSell={(row, quantity, negotiatedGp) => act(props.onSell(row.held, quantity, negotiatedGp))}
      loadSellPrice={async ({ held }) => {
        // A `+N` copy is priced at the Market's offer, as `copy_price_gp` prices it; a plain one at its record.
        if (held.bonus > 0 && held.costOverrideCopper === undefined) {
          const lists = await Promise.all(MARKET_CATEGORIES.map(({ value }) => listMarket({ category: value, nameContains: held.baseName })));
          return sellBundleGp(held, null, rowOfHeldCopy(lists.flatMap((list) => mapMarketTableRows(list.entries)), held));
        }
        return sellBundleGp(held, (await props.describe(held.copies[0])).entry.costGp, null);
      }}
      onDiscard={(row, quantity) => write((savedAt) => discardEquipmentCopy({ characterId, copy: row.held.copies[0], quantity, savedAt }))}
      onCustomizeName={(copy, text) => write(() => renameEquipmentCopy(renameRequest(characterId, copy, text)))}
      onCustomizeModifier={props.onPickModifier}
      onSets={{
        activate: (name) => write((savedAt) => activateEquipmentSet({ characterId, name, savedAt })),
        create: (name, copyActive) => write(() => createEquipmentSet({ characterId, name, copyActive })),
        rename: (to) => write(() => renameEquipmentSet({ characterId, from: active, to })),
        remove: () => write((savedAt) => deleteEquipmentSet({ characterId, name: active, savedAt })),
      }}
    />
  );
}
