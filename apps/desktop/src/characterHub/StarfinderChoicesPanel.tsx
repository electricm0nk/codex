import { useState, type CSSProperties } from 'react';
import type { SfChoicesDto, SfChoicesPreviewDto, SfEquipmentOptionDto } from '../boundary/starfinderChoices';
import { picksForSlot } from './starfinderCreationModel';
import { SlotPicker } from './StarfinderCreateForm';
import {
  addFeat,
  addGear,
  addModifier,
  addSpell,
  choicesTotalsLine,
  equipmentOptionLabel,
  featOptionLabel,
  removeFeatAt,
  removeGearAt,
  removeModifierAt,
  removeSpell,
  setFeatPick,
  setGearEquipped,
} from './starfinderChoicesModel';

/**
 * A Starfinder character's feats, spells known and gear (SD-37 E6.5a), the one panel the
 * creation form, the level-up dialog and the sheet's "Feats, spells and gear" dialog share.
 * Every option list, prerequisite verdict, spells-known total and sheet total is the engine's
 * preview (`preview`); the panel only edits the player's choices (`choices`, reported through
 * `onChange`). `onAddGear`, when given, adds an item through the caller's own save path (the
 * sheet appends it through the Starfinder adapter); otherwise the item joins the choices.
 */

const SECTION_STYLE: CSSProperties = { border: '1px solid var(--color-border)', borderRadius: 8, margin: '0 0 0.75rem', padding: '0.6rem 0.9rem' };
const MUTED_STYLE: CSSProperties = { color: 'var(--color-text-muted)', fontSize: '0.78rem', margin: '0.2rem 0' };
const SELECT_STYLE: CSSProperties = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 8,
  color: 'var(--color-text)',
  maxWidth: '100%',
  padding: '0.4rem 0.6rem',
};
const SMALL_BUTTON_STYLE: CSSProperties = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  color: 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '0.78rem',
  padding: '0.15rem 0.45rem',
};
const ROW_STYLE: CSSProperties = { alignItems: 'center', display: 'flex', flexWrap: 'wrap', gap: '0.4rem', margin: '0.2rem 0' };

export function StarfinderChoicesPanel(props: {
  preview: SfChoicesPreviewDto;
  choices: SfChoicesDto;
  equipment: SfEquipmentOptionDto[];
  onChange: (choices: SfChoicesDto) => void;
  onAddGear?: (itemId: string, equipped: boolean) => void;
}) {
  const { preview, choices, onChange } = props;
  const [itemId, setItemId] = useState('');
  const totals = choicesTotalsLine(preview.totals);

  function add(equipped: boolean) {
    if (itemId === '') {
      return;
    }
    if (props.onAddGear) {
      props.onAddGear(itemId, equipped);
    } else {
      onChange(addGear(choices, itemId, equipped));
    }
    setItemId('');
  }

  return (
    <div data-testid="sf-choices">
      <div style={SECTION_STYLE}>
        <p style={{ fontWeight: 700, margin: '0 0 0.3rem' }}>Feats</p>
        {preview.featRules.map((line) => (
          <p key={line} style={MUTED_STYLE}>
            {line}
          </p>
        ))}
        {choices.feats.map((featId, index) => {
          const label = preview.featOptions.find((o) => o.id === featId)?.label ?? preview.feats.find((o) => o.id === featId)?.label ?? featId;
          return (
            <div key={`${featId}:${index}`} style={ROW_STYLE}>
              <span style={{ fontSize: '0.85rem' }}>{label}</span>
              <button type="button" style={SMALL_BUTTON_STYLE} onClick={() => onChange(removeFeatAt(choices, index))}>
                Remove {label}
              </button>
            </div>
          );
        })}
        {preview.featSlots.map((slot) => (
          <SlotPicker
            key={slot.slotId}
            slot={slot}
            chosen={picksForSlot(choices.featPicks, slot.slotId)}
            onChange={(position, optionId) => onChange(setFeatPick(choices, slot, position, optionId))}
          />
        ))}
        <select aria-label="Add a feat" style={SELECT_STYLE} value="" onChange={(event) => onChange(addFeat(choices, event.target.value))}>
          <option value="">— add a feat —</option>
          {preview.featOptions.map((option) => (
            <option key={option.id} value={option.id}>
              {featOptionLabel(option)}
            </option>
          ))}
        </select>
      </div>

      {preview.spellLevels.length > 0 ? (
        <div style={SECTION_STYLE}>
          <p style={{ fontWeight: 700, margin: '0 0 0.3rem' }}>Spells known</p>
          {preview.spellLevels.map((level) => {
            const name = `${level.classLabel} level ${level.level}`;
            return (
              <div key={`${level.classId}:${level.level}`} style={{ margin: '0 0 0.5rem' }}>
                <p style={{ fontSize: '0.85rem', fontWeight: 600, margin: '0.2rem 0' }}>
                  {name} spells: {level.chosen.length} of {level.known} known
                </p>
                <p style={MUTED_STYLE}>{level.knownTerms.join('; ')}</p>
                {level.chosen.map((spell) => (
                  <div key={spell.id} style={ROW_STYLE}>
                    <span style={{ fontSize: '0.85rem' }}>{spell.label}</span>
                    <button type="button" style={SMALL_BUTTON_STYLE} onClick={() => onChange(removeSpell(choices, level.classId, spell.id))}>
                      Remove {spell.label}
                    </button>
                  </div>
                ))}
                <select
                  aria-label={`Add a ${name} spell`}
                  style={SELECT_STYLE}
                  value=""
                  onChange={(event) => onChange(addSpell(choices, level.classId, event.target.value))}
                >
                  <option value="">— add a {name} spell —</option>
                  {level.options.map((option) => (
                    <option key={option.id} value={option.id}>
                      {option.label}
                    </option>
                  ))}
                </select>
              </div>
            );
          })}
        </div>
      ) : null}

      <div style={SECTION_STYLE}>
        <p style={{ fontWeight: 700, margin: '0 0 0.3rem' }}>Gear</p>
        {preview.gear.map((line) => (
          <div key={`${line.itemId}:${line.position}`} style={{ margin: '0 0 0.4rem' }}>
            <div style={ROW_STYLE}>
              <span style={{ fontSize: '0.85rem', fontWeight: 600 }}>
                {line.label} — {line.equipped ? 'equipped' : 'carried'}
              </span>
              <button type="button" style={SMALL_BUTTON_STYLE} onClick={() => onChange(setGearEquipped(choices, line.position, !line.equipped))}>
                {line.equipped ? `Carry ${line.label}` : `Equip ${line.label}`}
              </button>
              <button type="button" style={SMALL_BUTTON_STYLE} onClick={() => onChange(removeGearAt(choices, line.position))}>
                Remove {line.label}
              </button>
            </div>
            {line.modifiers.map((modifier, index) => (
              <div key={`${modifier.id}:${index}`} style={{ ...ROW_STYLE, paddingLeft: '1rem' }}>
                <span style={{ fontSize: '0.8rem' }}>{modifier.label}</span>
                <button type="button" style={SMALL_BUTTON_STYLE} onClick={() => onChange(removeModifierAt(choices, line.position, index))}>
                  Remove {modifier.label} from {line.label}
                </button>
              </div>
            ))}
            {line.modifierOptions.length > 0 ? (
              <select
                aria-label={`Add an upgrade to ${line.label}`}
                style={{ ...SELECT_STYLE, marginLeft: '1rem' }}
                value=""
                onChange={(event) => onChange(addModifier(choices, line.position, event.target.value))}
              >
                <option value="">— add an upgrade, fusion or material to {line.label} —</option>
                {line.modifierOptions.map((option) => (
                  <option key={option.id} value={option.id}>
                    {option.label}
                  </option>
                ))}
              </select>
            ) : null}
          </div>
        ))}
        <div style={ROW_STYLE}>
          <select aria-label="Choose equipment" style={SELECT_STYLE} value={itemId} onChange={(event) => setItemId(event.target.value)}>
            <option value="">— choose equipment —</option>
            {props.equipment.map((option) => (
              <option key={option.id} value={option.id}>
                {equipmentOptionLabel(option)}
              </option>
            ))}
          </select>
          <button type="button" style={SMALL_BUTTON_STYLE} disabled={itemId === ''} onClick={() => add(true)}>
            Add equipped
          </button>
          <button type="button" style={SMALL_BUTTON_STYLE} disabled={itemId === ''} onClick={() => add(false)}>
            Add carried
          </button>
        </div>
      </div>

      {totals === '' ? null : (
        <p data-testid="sf-choices-totals" style={{ fontWeight: 700, margin: '0 0 0.5rem' }}>
          {totals}
        </p>
      )}
    </div>
  );
}
