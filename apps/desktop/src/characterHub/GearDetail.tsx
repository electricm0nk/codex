import type { CSSProperties } from 'react';
import type { ContainerStatsDto, HeldEquipmentDetailDto } from '../boundary/describeHeldEquipment';
import type { WeaponDamageDto } from '../boundary/loadSavedCharacterDetail';
import { hasDescription } from '../equipmentCatalog/EquipmentCatalogScreen';
import { wornLabel } from './gearScreenModel';

/**
 * The gear detail panel (SD-42 `decisions.md §27.4`, operator DET-1): everything about the selected
 * held copy, in two blocks — the item's record and this copy. Every value is the engine's, from
 * `describe_held_equipment`; the panel formats and never derives (the twin rule). A value the record
 * does not state reads `—`.
 */

export const ABSENT = '—';

const block: CSSProperties = { margin: '0.25rem 0 0.5rem' };
const term: CSSProperties = { color: 'var(--color-text-faint)', fontSize: '0.75rem' };
const value: CSSProperties = { fontSize: '0.8rem', margin: '0 0 0.2rem' };

/** `v` as written, or `—` when absent or blank. */
function shown(v: string | number | null | undefined): string {
  return v === null || v === undefined || v === '' ? ABSENT : String(v);
}

/** A bonus with its sign: `+4`, `-2`, `0`. */
function signed(v: number | null | undefined): string {
  return v === null || v === undefined ? ABSENT : v > 0 ? `+${v}` : String(v);
}

function unit(v: number | null | undefined, suffix: string): string {
  return v === null || v === undefined ? ABSENT : `${v} ${suffix}`;
}

function list(items: string[] | undefined): string {
  return items && items.length > 0 ? items.join(', ') : ABSENT;
}

function damage(weapon: WeaponDamageDto): string {
  return weapon.baseDice ? `${weapon.baseDice.count}d${weapon.baseDice.dieSize}` : shown(weapon.damageOverride);
}

function critical(weapon: WeaponDamageDto): string {
  const range = weapon.criticalThreatRange;
  const threat = range ? (range[0] === range[1] ? String(range[1]) : `${range[0]}–${range[1]}`) : null;
  const multiplier = weapon.criticalMultiplier !== null ? `×${weapon.criticalMultiplier}` : null;
  return threat && multiplier ? `${threat}/${multiplier}` : shown(threat ?? multiplier);
}

function capacity(container: ContainerStatsDto): string {
  switch (container.capacity.kind) {
    case 'pounds':
      return `${container.capacity.lbs} lb`;
    case 'unlimited':
      return 'No limit';
    case 'unstated':
      return ABSENT;
  }
}

function contentsWeigh(pct: number): string {
  return pct === 0 ? 'Contents weigh nothing' : pct === 100 ? 'Contents weigh in full' : `Contents weigh ${pct}% of their weight`;
}

function Field(props: { label: string; text: string; testId: string }) {
  return (
    <>
      <dt style={term}>{props.label}</dt>
      <dd data-testid={props.testId} style={value}>
        {props.text}
      </dd>
    </>
  );
}

export interface GearDetailProps {
  detail: HeldEquipmentDetailDto;
  /** Where the copy is, as the gear screen's row says it. */
  locationLabel: string;
}

export function GearDetail(props: GearDetailProps) {
  const { entry, weapon, armor, container, copy } = props.detail;
  const named = copy.name !== copy.baseName;
  return (
    <section data-testid="gear-detail" aria-label={`About ${copy.name}`} style={{ borderTop: '1px solid var(--color-border)', marginTop: '0.5rem', paddingTop: '0.4rem' }}>
      <h4 style={{ fontSize: '0.85rem', margin: '0 0 0.25rem' }}>Item</h4>
      <dl style={block}>
        <Field label="Name" text={entry.name} testId="gear-detail-name" />
        <Field label="Type" text={entry.types.length > 0 ? entry.types.join(', ') : shown(entry.category)} testId="gear-detail-type" />
        <Field label="Weight" text={unit(entry.weightLbs, 'lb')} testId="gear-detail-weight" />
        <Field label="Cost" text={unit(entry.costGp, 'gp')} testId="gear-detail-cost" />
        <Field label="Body slot" text={wornLabel(copy.slot)} testId="gear-detail-slot" />
        <Field label="Damage" text={weapon ? damage(weapon) : ABSENT} testId="gear-detail-damage" />
        <Field label="Critical" text={weapon ? critical(weapon) : ABSENT} testId="gear-detail-critical" />
        <Field label="Range" text={unit(weapon?.rangeIncrement, 'ft.')} testId="gear-detail-range" />
        <Field label="Wield" text={shown(weapon?.wieldCategory)} testId="gear-detail-wield" />
        <Field label="Weapon properties" text={list(weapon?.properties)} testId="gear-detail-weapon-properties" />
        <Field label="AC bonus" text={signed(armor?.armorClassBonus)} testId="gear-detail-ac" />
        <Field label="Max Dex" text={signed(armor?.maxDex)} testId="gear-detail-max-dex" />
        <Field label="Armor check penalty" text={signed(armor?.armorCheckPenalty)} testId="gear-detail-acp" />
        <Field label="Arcane spell failure" text={armor?.spellFailure === undefined ? ABSENT : `${armor.spellFailure}%`} testId="gear-detail-asf" />
        <Field label="Capacity" text={container ? capacity(container) : ABSENT} testId="gear-detail-capacity" />
        <Field label="Contents" text={container ? contentsWeigh(container.contentsWeightPct) : ABSENT} testId="gear-detail-contents" />
        <Field label="Description" text={hasDescription(entry.description) ? entry.description : ABSENT} testId="gear-detail-description" />
        <Field label="Source" text={shown(entry.book)} testId="gear-detail-book" />
      </dl>
      <h4 style={{ fontSize: '0.85rem', margin: '0 0 0.25rem' }}>This copy</h4>
      <dl style={block}>
        <Field label="Name" text={named ? `${copy.name} (${copy.baseName})` : copy.name} testId="gear-detail-copy-name" />
        <Field label="Size" text={shown(copy.size)} testId="gear-detail-size" />
        <Field label="Material and abilities" text={list(copy.abilities)} testId="gear-detail-abilities" />
        <Field label="Modifiers" text={list(copy.appliedModifiers)} testId="gear-detail-modifiers" />
        <Field label="Damage override" text={shown(copy.damageOverride)} testId="gear-detail-damage-override" />
        <Field label="Weight override" text={unit(copy.weightOverrideLbs, 'lb')} testId="gear-detail-weight-override" />
        <Field label="Cost override" text={unit(copy.costOverrideCopper, 'cp')} testId="gear-detail-cost-override" />
        <Field label="Properties" text={list(copy.properties)} testId="gear-detail-properties" />
        <Field label="Quantity" text={String(copy.quantity)} testId="gear-detail-quantity" />
        <Field label="Location" text={props.locationLabel} testId="gear-detail-location" />
      </dl>
    </section>
  );
}
