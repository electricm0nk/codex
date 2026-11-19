import { useState, type CSSProperties } from 'react';
import type { CopyKeyDto, PlacedEquipmentCopyDto } from '../boundary/listHeldEquipment';
import type { RenameEquipmentCopyRequest } from '../boundary/renameEquipmentCopy';

/**
 * The gear screen's Customize dialog (SD-42 `decisions.md §27.3`, operator REN-1 + CUS-1): one held
 * copy's name and its modifiers in one place. Save renames the copy (`rename_equipment_copy`, a
 * blank name restores the record's); Attach modifier opens the Equipmods picker for this copy.
 */

/** The copy the Equipmods picker attaches to, and how the screen names it. */
export interface CustomizeTarget {
  copy: CopyKeyDto;
  title: string;
}

/** What Save sends: the copy and the name trimmed; blank clears it, so the record's name shows. */
export function renameRequest(characterId: string, copy: CopyKeyDto, text: string): RenameEquipmentCopyRequest {
  return { characterId, copy, name: text.trim() };
}

const button: CSSProperties = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 4,
  color: 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '0.75rem',
  padding: '0.25rem 0.6rem',
};
const quiet: CSSProperties = { color: 'var(--color-text-faint)', fontSize: '0.75rem' };

export interface CustomizeCopyPanelProps {
  title: string;
  held: PlacedEquipmentCopyDto;
  /** The name field's text. */
  name: string;
  busy: boolean;
  onNameChange: (text: string) => void;
  onSave: () => void;
  onAttach: () => void;
  onClose: () => void;
}

/** The dialog's markup and handlers, without the name field's state. */
export function CustomizeCopyPanel(props: CustomizeCopyPanelProps) {
  const modifiers = props.held.appliedModifiers ?? [];
  return (
    <div
      role="presentation"
      onClick={props.onClose}
      style={{ alignItems: 'center', backgroundColor: 'rgba(0,0,0,0.6)', display: 'flex', inset: 0, justifyContent: 'center', position: 'fixed', zIndex: 60 }}
    >
      <div
        role="dialog"
        aria-modal="true"
        aria-label={`Customize ${props.title}`}
        data-testid="gear-customize-dialog"
        onClick={(event) => event.stopPropagation()}
        style={{
          backgroundColor: 'var(--color-surface)',
          border: '1px solid var(--color-border)',
          borderRadius: 6,
          color: 'var(--color-text)',
          maxWidth: '90vw',
          padding: '1rem',
          width: '360px',
        }}
      >
        <h2 style={{ fontSize: '1rem', margin: '0 0 0.75rem' }}>{`Customize ${props.title}`}</h2>
        <label style={{ display: 'block', fontSize: '0.8rem' }}>
          Name{' '}
          <input
            data-testid="gear-customize-name"
            value={props.name}
            placeholder={props.held.baseName}
            maxLength={80}
            onChange={(event) => props.onNameChange(event.target.value)}
            style={{ fontSize: '0.8rem', padding: '0.2rem 0.35rem', width: '14rem' }}
          />
        </label>
        <p style={{ ...quiet, margin: '0.25rem 0 0.5rem' }}>{`Leave blank for ${props.held.baseName}.`}</p>
        <button type="button" data-testid="gear-customize-save" disabled={props.busy} onClick={props.onSave} style={button}>
          Save name
        </button>
        <h3 style={{ fontSize: '0.85rem', margin: '1rem 0 0.25rem' }}>Modifiers</h3>
        {modifiers.length === 0 ? (
          <p style={{ ...quiet, margin: 0 }}>None attached.</p>
        ) : (
          <ul data-testid="gear-customize-modifiers" style={{ margin: 0, paddingLeft: '1.25rem' }}>
            {modifiers.map((modifier, index) => (
              <li key={`${modifier}-${index}`} style={{ fontSize: '0.8rem' }}>
                {modifier}
              </li>
            ))}
          </ul>
        )}
        <div style={{ display: 'flex', gap: '0.4rem', justifyContent: 'space-between', marginTop: '0.75rem' }}>
          <button type="button" data-testid="gear-customize-attach" disabled={props.busy} onClick={props.onAttach} style={button}>
            Attach modifier
          </button>
          <button type="button" data-testid="gear-customize-close" onClick={props.onClose} style={button}>
            Close
          </button>
        </div>
      </div>
    </div>
  );
}

export interface CustomizeCopyDialogProps {
  held: PlacedEquipmentCopyDto;
  target: CustomizeTarget;
  busy: boolean;
  /** Saves `text` as the copy's name; the screen sends it through {@link renameRequest}. */
  onRename: (copy: CopyKeyDto, text: string) => void;
  onAttach: (target: CustomizeTarget) => void;
  onClose: () => void;
}

export function CustomizeCopyDialog(props: CustomizeCopyDialogProps) {
  const [name, setName] = useState(props.held.name);
  return (
    <CustomizeCopyPanel
      title={props.target.title}
      held={props.held}
      name={name}
      busy={props.busy}
      onNameChange={setName}
      onSave={() => props.onRename(props.target.copy, name)}
      onAttach={() => props.onAttach(props.target)}
      onClose={props.onClose}
    />
  );
}
