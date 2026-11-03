import { useEffect, useState } from 'react';
import type { BodySlotDto, PlacedEquipmentCopyDto } from '../boundary/listHeldEquipment';

type ChangeableState = 'EquippedActive' | 'SelectedInactive';

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

/** Worn rows by slot, two in one slot in saved order, and carried rows in saved order. */
export function equipmentPanelLists(copies: PlacedEquipmentCopyDto[]): { worn: PlacedEquipmentCopyDto[]; carried: PlacedEquipmentCopyDto[] } {
  const order = (copy: PlacedEquipmentCopyDto) => WORN_SLOTS.findIndex((row) => row.slot === copy.slot);
  return {
    worn: copies.filter((copy) => copy.activeState === 'EquippedActive').sort((a, b) => order(a) - order(b)),
    carried: copies.filter((copy) => copy.activeState === 'SelectedInactive'),
  };
}

const heading = { fontSize: '0.8rem', margin: '0.6rem 0 0.25rem' };
const row = { alignItems: 'center', borderBottom: '1px solid var(--color-border)', display: 'flex', justifyContent: 'space-between', padding: '0.25rem 0' };
const actionButton = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 4,
  color: 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '0.7rem',
  padding: '0.2rem 0.55rem',
};
const quiet = { color: 'var(--color-text-faint)', fontSize: '0.8rem', margin: 0 };

export interface EquipmentPanelViewProps {
  /** Null until the first read returns. */
  copies: PlacedEquipmentCopyDto[] | null;
  loadError: string | null;
  /** The backend's refusal of the last change, word for word. */
  refusal: string | null;
  busy: boolean;
  onSetState: (copy: PlacedEquipmentCopyDto, activeState: ChangeableState) => void;
}

export function EquipmentPanelView(props: EquipmentPanelViewProps) {
  const lists = equipmentPanelLists(props.copies ?? []);
  const action = (copy: PlacedEquipmentCopyDto, word: 'Stow' | 'Equip', activeState: ChangeableState) => (
    <button
      type="button"
      disabled={props.busy}
      onClick={() => props.onSetState(copy, activeState)}
      style={actionButton}
    >
      {`${word} ${copy.name}`}
    </button>
  );
  return (
    <section aria-label="Equipment" className="no-print" style={{ marginBottom: '1.25rem' }}>
      <h3 style={{ fontSize: '0.95rem', margin: '0 0 0.25rem' }}>Equipment</h3>
      {props.refusal ? (
        <p role="alert" style={{ color: 'var(--color-danger, #c0392b)', fontSize: '0.8rem', margin: '0 0 0.25rem' }}>
          {props.refusal}
        </p>
      ) : null}
      {props.loadError ? <p role="alert" style={{ color: 'var(--color-danger, #c0392b)', fontSize: '0.8rem', margin: 0 }}>{props.loadError}</p> : null}
      {!props.loadError && props.copies === null ? <p style={quiet}>Loading what this character holds.</p> : null}
      {props.copies?.length === 0 ? <p style={quiet}>Nothing held yet.</p> : null}
      {props.copies && props.copies.length > 0 ? (
        <>
          <h4 style={heading}>Worn</h4>
          {lists.worn.length === 0 ? <p style={quiet}>Nothing worn.</p> : null}
          {lists.worn.map((copy, index) => (
            <div key={`${copy.itemId}::${copy.bonus}::${index}`} style={row}>
              <span>{`${wornLabel(copy.slot)}: ${copy.name}`}</span>
              {action(copy, 'Stow', 'SelectedInactive')}
            </div>
          ))}
          <h4 style={heading}>Carried</h4>
          {lists.carried.length === 0 ? <p style={quiet}>Nothing carried.</p> : null}
          {lists.carried.map((copy, index) => (
            <div key={`${copy.itemId}::${copy.bonus}::${index}`} style={row}>
              <span>{copy.name}</span>
              {action(copy, 'Equip', 'EquippedActive')}
            </div>
          ))}
        </>
      ) : null}
    </section>
  );
}

/**
 * The Gear tab's Equipment control: every held copy, worn by slot or carried, off
 * `list_held_equipment`. Re-reads when `refreshKey` changes, which every saved change to the
 * character does. A refusal stays on screen until the next change is asked for.
 */
export function EquipmentPanel(props: {
  load: () => Promise<PlacedEquipmentCopyDto[]>;
  refreshKey: unknown;
  onSetState: (copy: PlacedEquipmentCopyDto, activeState: ChangeableState) => Promise<string | null>;
}) {
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

  return (
    <EquipmentPanelView
      copies={copies}
      loadError={loadError}
      refusal={refusal}
      busy={busy}
      onSetState={(copy, activeState) => {
        setBusy(true);
        setRefusal(null);
        void props
          .onSetState(copy, activeState)
          .then(setRefusal)
          .finally(() => setBusy(false));
      }}
    />
  );
}
