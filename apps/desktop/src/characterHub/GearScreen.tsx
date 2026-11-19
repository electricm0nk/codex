import { useEffect, useRef, useState, type CSSProperties, type DragEvent, type HTMLAttributes, type KeyboardEvent } from 'react';
import { discardEquipmentCopy } from '../boundary/discardEquipmentCopy';
import { activateEquipmentSet, createEquipmentSet, deleteEquipmentSet, renameEquipmentSet } from '../boundary/equipmentSets';
import type { CopyKeyDto, LocationDto, PlacedEquipmentCopyDto } from '../boundary/listHeldEquipment';
import { moveEquipmentCopy } from '../boundary/moveEquipmentCopy';
import { renameEquipmentCopy } from '../boundary/renameEquipmentCopy';
import { CustomizeCopyDialog, renameRequest, type CustomizeTarget } from './CustomizeCopyDialog';
import type { EquipmentSetsSummaryDto } from '../boundary/loadSavedCharacterDetail';
import { OverrideTag } from './OverrideTag';
import {
  containerTables,
  copyId,
  DRAG_FORMAT,
  dropTarget,
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
  'Drop keeps the item and marks it Not Carried; Sell returns money under the pricing above; Discard removes it for nothing.';

const panel: CSSProperties = { border: '1px solid var(--color-border)', borderRadius: 6, minWidth: 0, padding: '0.5rem 0.75rem' };
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
  onMove: (row: GearRow, location: LocationDto, quantity?: number) => void;
  onDrop: (row: GearRow, quantity?: number) => void;
  onSell: (row: GearRow, quantity?: number, negotiatedGp?: number) => void;
  onDiscard: (row: GearRow, quantity?: number) => void;
  onSets: GearSetActions;
  /** Customize's Save: names `copy` `text` (blank restores the record's name). */
  onCustomizeName: (copy: CopyKeyDto, text: string) => void;
  /** Customize's Attach modifier: the Equipmods picker for this copy. */
  onCustomizeModifier: (target: CustomizeTarget) => void;
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

/** The inline "how many?" (and Sell's Negotiate, Discard's confirm) under a row. */
function PendingForm(props: { pending: Pending; busy: boolean; onConfirm: (quantity?: number, negotiatedGp?: number) => void; onCancel: () => void }) {
  const held = props.pending.row.held;
  const prompt = quantityPrompt(held);
  const [count, setCount] = useState(String(held.quantity));
  const [negotiated, setNegotiated] = useState('');
  const [error, setError] = useState<string | null>(null);
  const verb = { move: `Move to ${props.pending.targetLabel ?? ''}`.trim(), drop: 'Drop', sell: 'Sell', discard: 'Discard' }[props.pending.kind];
  const confirm = () => {
    const quantity = prompt ? parseQuantity(count, held) : undefined;
    if (typeof quantity === 'string') {
      setError(quantity);
      return;
    }
    const gp = negotiated.trim() === '' ? undefined : Number(negotiated);
    if (gp !== undefined && !(Number.isFinite(gp) && gp >= 0)) {
      setError('Enter the agreed total in gp, or leave it blank for the pricing above.');
      return;
    }
    props.onConfirm(quantity, gp);
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
      {props.pending.kind === 'sell' ? (
        <label style={{ fontSize: '0.75rem' }}>
          Negotiate (gp, blank = the pricing above){' '}
          <input type="number" min={0} step="any" value={negotiated} onChange={(event) => setNegotiated(event.target.value)} style={input} />
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

/** The gear screen's two panels: every row with its buttons on the left, where it is on the right. */
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
    pending && pending.at === at ? <PendingForm key={`${pending.kind}-${at}`} pending={pending} busy={props.busy} onConfirm={confirm} onCancel={() => setPending(null)} /> : null;
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
  const dropZone = (targetId: string, at: string): HTMLAttributes<HTMLDivElement> => ({
    onDragOver: (event: DragEvent<HTMLDivElement>) => {
      if (dropTarget({ getData: (format) => (format === DRAG_FORMAT ? dragging.current : '') }, targetId, model)) {
        event.preventDefault();
        event.stopPropagation();
        event.dataTransfer.dropEffect = 'move';
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

  const treeItem = (item: GearTreeItem, depth: number) => {
    const at = `tree:${copyId(item.copy)}`;
    return (
      <div key={at}>
        <div {...dragSource(item.row)} {...(item.row.isContainer ? dropZone(`inside-${copyId(item.copy)}`, at) : {})} style={{ ...rowStyle, paddingLeft: `${depth * 1}rem` }}>
          <span>
            {item.row.label}
            {tag(item.row)}
            {item.capacityLabel ? <span style={{ ...quiet, marginLeft: '0.5rem' }}>{item.capacityLabel}</span> : null}
            {item.overrideTag ? <OverrideTag flag="ignoreContainerCapacity" /> : null}
          </span>
          <MoveToMenu model={model} row={item.row} busy={props.busy} onChoose={(location, label) => run({ kind: 'move', row: item.row, location, targetLabel: label }, at)} />
        </div>
        {formAt(at)}
        {item.children.map((child) => treeItem(child, depth + 1))}
      </div>
    );
  };

  // One held row with its buttons: the top-level list and every container table render the same row.
  const gearRow = (row: GearRow, at: string) => (
    <div {...dragSource(row)} {...(row.isContainer ? dropZone(`inside-${row.id}`, at) : {})} style={rowStyle}>
      <span>
        {row.label}
        {tag(row)}
        <span style={{ ...quiet, marginLeft: '0.5rem' }}>
          {`${row.weightLabel} · ${row.locationLabel}`}
          {row.capacityLabel ? ` · ${row.capacityLabel}` : ''}
        </span>
        {(row.held.properties ?? []).map((property) => (
          <span key={property} data-testid="gear-property" style={{ ...quiet, display: 'block' }}>
            {property}
          </span>
        ))}
      </span>
      <span style={{ display: 'flex', flexShrink: 0, gap: '0.3rem' }}>
        {customize(row, row.held.copies[0], row.label, `gear-row-customize-${row.id}`)}
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
  const tables = containerTables(model);

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
      <div style={{ display: 'grid', gap: '0.75rem', gridTemplateColumns: 'repeat(auto-fit, minmax(300px, 1fr))' }}>
        <div style={panel}>
          <label style={{ fontSize: '0.8rem' }}>
            Show{' '}
            <select data-testid="gear-filter" value={filter} onChange={(event) => setFilter(event.target.value as GearFilter)}>
              <option value="all">All</option>
              <option value="equipped">Equipped</option>
              <option value="notEquipped">Not equipped</option>
            </select>
          </label>
          {model.rows.length === 0 ? <p style={quiet}>Nothing held yet.</p> : null}
          {filterGearRows(model.rows, filter)
            .filter((row) => row.held.location.kind !== 'inside')
            .map((row) => {
              const at = `row:${row.id}`;
              return (
                <div key={at} data-testid={`gear-row-${row.id}`}>
                  {gearRow(row, at)}
                  {formAt(at)}
                </div>
              );
            })}
          {tables.length > 0 ? (
            <div style={{ display: 'grid', gap: '0.75rem', gridTemplateColumns: 'repeat(auto-fit, minmax(300px, 1fr))', marginTop: '0.5rem' }}>
              {tables.map((table) => (
                <table key={copyId(table.copy)} data-testid={`gear-container-${copyId(table.copy)}`} style={{ ...panel, borderCollapse: 'collapse', width: '100%' }}>
                  <caption style={{ fontSize: '0.8rem', fontWeight: 700, textAlign: 'left' }}>
                    {table.loadLabel ? `${table.title} · ${table.loadLabel}` : table.title}{' '}
                    {customize(table.row, table.copy, table.title, `gear-container-customize-${copyId(table.copy)}`)}
                  </caption>
                  <tbody>
                    {table.rows.length === 0 ? (
                      <tr>
                        <td style={quiet}>Empty</td>
                      </tr>
                    ) : (
                      table.rows.map((row) => {
                        const at = `row:${row.id}`;
                        return (
                          <tr key={at} data-testid={`gear-row-${row.id}`}>
                            <td>
                              {gearRow(row, at)}
                              {formAt(at)}
                            </td>
                          </tr>
                        );
                      })
                    )}
                  </tbody>
                </table>
              ))}
            </div>
          ) : null}
          <p style={{ ...quiet, marginTop: '0.5rem' }}>{DROP_SELL_DISCARD_CAPTION}</p>
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
        <div style={panel}>
          {model.warnings.map((warning) => (
            <p key={copyId(warning.copy)} role="alert" data-testid={`gear-over-${copyId(warning.copy)}`} style={danger}>
              {warning.text}
            </p>
          ))}
          {model.tree.map((section) => (
            <div key={section.id} {...dropZone(section.id, `section:${section.id}`)}>
              <div data-testid={`gear-location-${section.id}`} style={{ fontSize: '0.8rem', fontWeight: 700, marginTop: '0.4rem' }}>
                {section.label}
                {section.places ? <span style={{ ...quiet, fontWeight: 400, marginLeft: '0.4rem' }}>{section.places}</span> : null}
              </div>
              {formAt(`section:${section.id}`)}
              {section.items.map((item) => treeItem(item, 1))}
            </div>
          ))}
        </div>
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
  /** Sells pieces of a stack, at the pricing above or the negotiated total; resolves like `onChange`. */
  onSell: (held: PlacedEquipmentCopyDto, quantity?: number, negotiatedGp?: number) => Promise<string | null>;
  /** Customize's Attach modifier: opens the Equipmods picker for this copy. */
  onPickModifier: (target: CustomizeTarget) => void;
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
  if (copies === null) {
    return <p style={quiet}>Loading what this character holds.</p>;
  }
  return (
    <GearPanels
      model={gearScreenModel(copies, props.context)}
      armorClass={props.armorClass}
      carriedLbs={props.carriedLbs}
      sets={props.sets}
      busy={busy}
      refusal={refusal}
      onMove={(row, location, quantity) => write((savedAt) => moveEquipmentCopy({ characterId, copy: row.held.copies[0], location, quantity, savedAt }))}
      onDrop={(row, quantity) => write((savedAt) => moveEquipmentCopy({ characterId, copy: row.held.copies[0], location: { kind: 'notCarried' }, quantity, savedAt }))}
      onSell={(row, quantity, negotiatedGp) => act(props.onSell(row.held, quantity, negotiatedGp))}
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
