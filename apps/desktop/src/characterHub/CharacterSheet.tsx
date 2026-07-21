import { useState, type CSSProperties, type ReactNode } from 'react';
import type { CharacterHubListRowSurface } from './buildCharacterHubListSurface';
import type { LoadSavedCharacterResponse } from '../boundary/loadSavedCharacterDetail';
import type { AbilityScoresDto, CorpusDerivedDto } from '../boundary/loadCreateCharacter';
import { levelUpCharacter } from '../boundary/levelUpCharacter';
import { addEquipmentSelection } from '../boundary/addEquipmentSelection';
import { addSpellSelection } from '../boundary/addSpellSelection';
import { listEquipment } from '../boundary/listEquipment';
import { listSpells } from '../boundary/listSpells';
import { toCharacterMutationRefresh } from './characterSheetRefresh';
import { mapEquipmentCatalogEntries, mapSpellCatalogEntries } from './itemPickerFilter';
import { ItemPickerModal, type ItemPickerEntry } from './ItemPickerModal';
import {
  buildLevelEntries,
  buildNextEntries,
  casterLevel,
  classWeaponProficiency,
  formatHeldClasses,
  maxHitPoints,
  parseHeldClasses,
  totalCharacterLevel,
  totalSkillPoints,
  type HeldClass,
  type LevelEntry,
  type WeaponProficiency,
} from './characterProgression';
import { AGE_OPTIONS, ALIGNMENT_OPTIONS, RACE_OPTIONS } from './characterHubModel';
import { PortraitUpload } from './PortraitUpload';
import { LevelUpDialog } from './LevelUpDialog';
import { SkillAllocationDialog } from './SkillAllocationDialog';
import { DEFAULT_SKILL_ALLOCATION, SKILLS, isClassSkill, skillModifier, skillRankCost, totalSkillPointsAvailable } from './skillsModel';

/**
 * Pathfinder 1e character sheet, patterned after Pathbuilder 2e's three-column
 * layout: a left level-progression rail, a center stat column, and a right
 * tabbed content area. Real values (name, level, ability modifiers, AC, saves)
 * come from the loaded snapshot; slots we do not yet compute (feats, per-skill
 * training, weapons) render as "Not Selected" scaffolding.
 *
 * The sheet is game-system specific — this is the Pathfinder 1e variant; other
 * rule sets will supply their own sheet later.
 */

// ---------- shared bits ----------

function fmt(value: number): string {
  return value >= 0 ? `+${value}` : `${value}`;
}


const ZERO_ABILITIES: AbilityScoresDto = {
  strength: 0,
  dexterity: 0,
  constitution: 0,
  intelligence: 0,
  wisdom: 0,
  charisma: 0,
};

const panel: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 10,
};

/** Shared style for the tab-content "Add …" affordances (Add Weapon / Add Armor / Add Spell / Print). */
const addItemButtonStyle: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 8,
  color: 'var(--color-text)',
  cursor: 'pointer',
  padding: '0.5rem 1.5rem',
};

/**
 * `EquipmentCategory` has no separate weapon-vs-armor variant (see
 * `equipment_catalog.rs`'s `EquipmentCategory` enum) — "ArmsArmor" is
 * one combined category covering both. Judgment call: both the Add Weapon
 * and Add Armor pickers narrow to this same category server-side and let
 * the user disambiguate by name in the search box, rather than blocking on
 * a category taxonomy the corpus doesn't expose yet.
 */
const WEAPONS_AND_ARMOR_CATEGORY = 'ArmsArmor';

function NavCard(props: { label: string; value: string }) {
  return (
    <div style={{ ...panel, padding: '0.55rem 0.75rem' }}>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.68rem', letterSpacing: '0.04em', margin: 0, textTransform: 'uppercase' }}>
        {props.label}
      </p>
      <p style={{ color: 'var(--color-text)', fontWeight: 700, margin: '0.1rem 0 0' }}>{props.value}</p>
    </div>
  );
}

function LevelBenefitCard(props: { benefit: LevelEntry; skillPoints: number; variant: 'current' | 'next' }) {
  const { benefit, variant } = props;
  return (
    <div
      style={{
        ...panel,
        backgroundColor: variant === 'next' ? 'var(--color-surface-2)' : 'var(--color-surface)',
        borderStyle: variant === 'next' ? 'dashed' : 'solid',
        padding: '0.55rem 0.7rem',
      }}
    >
      <p style={{ color: 'var(--color-accent)', fontSize: '0.78rem', fontWeight: 800, letterSpacing: '0.02em', margin: 0 }}>
        Level {benefit.classLevel} {benefit.classLabel}
      </p>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', margin: '0.15rem 0 0.25rem' }}>
        Skill points: {props.skillPoints}
      </p>
      <ul style={{ margin: 0, paddingLeft: '1.1rem' }}>
        {benefit.features.map((feature) => (
          <li key={feature} style={{ color: 'var(--color-text-secondary)', fontSize: '0.8rem', marginBottom: '0.1rem' }}>
            {feature}
          </li>
        ))}
      </ul>
    </div>
  );
}

// ---------- center column ----------

const ABILITY_COLUMNS: ReadonlyArray<{ key: keyof AbilityScoresDto; label: string }> = [
  { key: 'strength', label: 'STR' },
  { key: 'dexterity', label: 'DEX' },
  { key: 'constitution', label: 'CON' },
  { key: 'intelligence', label: 'INT' },
  { key: 'wisdom', label: 'WIS' },
  { key: 'charisma', label: 'CHA' },
];

/** A titled stat panel. */
function StatBox(props: { title?: string; children: ReactNode }) {
  return (
    <div style={{ ...panel, padding: '0.6rem 0.75rem' }}>
      {props.title ? (
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.66rem', letterSpacing: '0.06em', margin: '0 0 0.45rem', textTransform: 'uppercase' }}>
          {props.title}
        </p>
      ) : null}
      {props.children}
    </div>
  );
}

/** A boxed value with a caption, e.g. AC / Touch / CMB. */
function StatTile(props: { label: string; value: ReactNode; emphasize?: boolean }) {
  return (
    <div style={{ ...panel, backgroundColor: 'var(--color-surface-2)', flex: 1, minWidth: 0, padding: '0.35rem 0.3rem', textAlign: 'center' }}>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.56rem', fontWeight: 700, letterSpacing: '0.03em', margin: 0, textTransform: 'uppercase' }}>
        {props.label}
      </p>
      <p style={{ fontSize: props.emphasize ? '1.3rem' : '1rem', fontWeight: 800, margin: '0.1rem 0 0' }}>{props.value}</p>
    </div>
  );
}

/** Approximate the ability score from its modifier for display (exact for even scores). */
function scoreFromModifier(modifier: number): number {
  return 10 + modifier * 2;
}

function IdentityPanel(props: { name: string; campaign: string }) {
  return (
    <div style={{ ...panel, alignItems: 'center', display: 'flex', gap: '0.75rem', justifyContent: 'space-between', padding: '0.6rem 0.75rem' }}>
      <div style={{ minWidth: 0 }}>
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.6rem', margin: 0 }}>Character Name</p>
        <p style={{ fontWeight: 700, margin: '0.1rem 0 0', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{props.name}</p>
      </div>
      <div style={{ minWidth: 0, textAlign: 'right' }}>
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.6rem', margin: 0 }}>Campaign</p>
        <p style={{ fontWeight: 700, margin: '0.1rem 0 0', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{props.campaign}</p>
      </div>
    </div>
  );
}

function AbilitiesPanel(props: { abilities: AbilityScoresDto }) {
  return (
    <StatBox title="Abilities">
      <div style={{ display: 'grid', gap: '0.25rem', gridTemplateColumns: 'repeat(6, 1fr)', textAlign: 'center' }}>
        {ABILITY_COLUMNS.map((col) => (
          <div key={col.key}>
            <p style={{ color: 'var(--color-text-muted)', fontSize: '0.6rem', fontWeight: 700, margin: 0 }}>{col.label}</p>
            <p style={{ fontSize: '1.05rem', fontWeight: 800, margin: '0.1rem 0 0' }}>{scoreFromModifier(props.abilities[col.key])}</p>
            <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.72rem', margin: 0 }}>{fmt(props.abilities[col.key])}</p>
          </div>
        ))}
      </div>
    </StatBox>
  );
}

function ArmorClassPanel(props: { ac: number; touch: number; flatFooted: number }) {
  return (
    <StatBox title="Armor Class">
      <div style={{ display: 'flex', gap: '0.5rem' }}>
        <StatTile label="AC" value={props.ac} emphasize />
        <StatTile label="Touch" value={props.touch} />
        <StatTile label="Flat-Footed" value={props.flatFooted} />
      </div>
    </StatBox>
  );
}

function SavingThrowsPanel(props: { saves: { fortitude: number; reflex: number; will: number } }) {
  return (
    <StatBox title="Saving Throws">
      <div style={{ display: 'flex', gap: '0.5rem' }}>
        <StatTile label="Fortitude" value={fmt(props.saves.fortitude)} />
        <StatTile label="Reflex" value={fmt(props.saves.reflex)} />
        <StatTile label="Will" value={fmt(props.saves.will)} />
      </div>
    </StatBox>
  );
}

function InitiativeHpPanel(props: { initiative: number; hp: number }) {
  return (
    <StatBox>
      <div style={{ display: 'flex', gap: '0.5rem' }}>
        <StatTile label="Initiative" value={fmt(props.initiative)} />
        <StatTile label="Hit Points" value={`${props.hp} / ${props.hp}`} />
      </div>
    </StatBox>
  );
}

function SpeedPanel() {
  return (
    <StatBox title="Speed">
      <div style={{ display: 'flex', gap: '0.5rem' }}>
        <StatTile label="Land" value="30 ft." />
        <StatTile label="Fly" value="—" />
        <StatTile label="Swim" value="—" />
        <StatTile label="Climb" value="—" />
      </div>
    </StatBox>
  );
}

function AttackPanel(props: { baseAttackBonus: number; cmb: number; cmd: number }) {
  return (
    <StatBox title="Attack">
      <div style={{ display: 'flex', gap: '0.5rem' }}>
        <StatTile label="BAB" value={fmt(props.baseAttackBonus)} />
        <StatTile label="Spell Res." value="—" />
        <StatTile label="CMB" value={fmt(props.cmb)} />
        <StatTile label="CMD" value={props.cmd} />
      </div>
    </StatBox>
  );
}

/** Automatic PF1 languages: every character knows Common; a positive Int modifier grants that many bonus language slots. */
function spokenLanguages(intelligenceModifier: number): string[] {
  const languages = ['Common'];
  if (intelligenceModifier > 0) {
    languages.push(`+${intelligenceModifier} bonus language slot${intelligenceModifier > 1 ? 's' : ''} (not yet selectable)`);
  }
  return languages;
}

function SkillsPanel(props: {
  abilities: AbilityScoresDto;
  heldClasses: HeldClass[];
  isHuman: boolean;
  allocation: Record<string, number>;
  realModifiers?: { climb: number; intimidate: number; swim: number };
  onOpenDialog: () => void;
}) {
  const REAL_MODIFIER_BY_SKILL: Record<string, number | undefined> = {
    Climb: props.realModifiers?.climb,
    Intimidate: props.realModifiers?.intimidate,
    Swim: props.realModifiers?.swim,
  };

  const spent = SKILLS.reduce(
    (sum, skill) => sum + (props.allocation[skill.name] ?? 0) * skillRankCost(isClassSkill(props.heldClasses, skill.name)),
    0
  );
  const remaining = totalSkillPointsAvailable(props.heldClasses, props.abilities.intelligence, props.isHuman) - spent;

  return (
    <StatBox title="Skills">
      <button
        type="button"
        onClick={props.onOpenDialog}
        title="Manage skill allocation"
        style={{
          alignItems: 'center',
          backgroundColor: remaining > 0 ? 'var(--color-surface-2)' : 'transparent',
          border: `1px solid ${remaining > 0 ? 'var(--color-accent)' : 'var(--color-border)'}`,
          borderRadius: 6,
          cursor: 'pointer',
          display: 'flex',
          fontSize: '0.75rem',
          justifyContent: 'space-between',
          marginBottom: '0.5rem',
          padding: '0.35rem 0.55rem',
          width: '100%',
        }}
      >
        <span style={{ color: 'var(--color-text-secondary)' }}>Manage skill allocation</span>
        {remaining > 0 ? (
          <span style={{ color: 'var(--color-accent)', fontWeight: 800 }}>{remaining} unallocated</span>
        ) : (
          <span style={{ color: 'var(--color-text-muted)' }}>fully allocated</span>
        )}
      </button>
      <div style={{ display: 'flex', flexDirection: 'column', gap: '0.15rem' }}>
        {SKILLS.map((skill) => {
          const classSkill = isClassSkill(props.heldClasses, skill.name);
          const ranks = props.allocation[skill.name] ?? 0;
          const abilityMod = props.abilities[skill.ability];
          const real = REAL_MODIFIER_BY_SKILL[skill.name];
          const total = real ?? skillModifier(abilityMod, ranks, classSkill);
          return (
            <div key={skill.name} style={{ alignItems: 'center', display: 'flex', fontSize: '0.85rem', gap: '0.4rem' }}>
              <span style={{ color: 'var(--color-text-secondary)', width: 34 }}>{fmt(total)}</span>
              <span style={{ color: classSkill ? 'var(--color-text)' : 'var(--color-text-secondary)' }}>{skill.name}</span>
              {ranks > 0 ? <span style={{ color: 'var(--color-text-muted)', fontSize: '0.7rem' }}>({ranks})</span> : null}
            </div>
          );
        })}
      </div>
      <hr style={{ border: 'none', borderTop: '1px solid var(--color-border)', margin: '0.6rem 0' }} />
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.66rem', letterSpacing: '0.03em', margin: '0 0 0.3rem', textTransform: 'uppercase' }}>
        Languages
      </p>
      <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.8rem', margin: 0 }}>
        {spokenLanguages(props.abilities.intelligence).join(', ')}
      </p>
    </StatBox>
  );
}

// ---------- right column ----------

const TABS = ['Weapons', 'Defense', 'Gear', 'Spells', 'Pets', 'Details', 'Feats', 'Actions', 'Bio', 'Overrides'] as const;
type Tab = (typeof TABS)[number];

export interface BioFields {
  alignment: string;
  deity: string;
  sex: string;
  age: string;
  height: string;
  weight: string;
  hair: string;
  eyes: string;
}

export const BLANK_BIO_FIELDS: BioFields = {
  alignment: '',
  deity: '',
  sex: '',
  age: '',
  height: '',
  weight: '',
  hair: '',
  eyes: '',
};

const bioFieldLabelStyle: CSSProperties = {
  color: 'var(--color-text-muted)',
  fontSize: '0.62rem',
  letterSpacing: '0.03em',
  margin: '0 0 0.2rem',
  textTransform: 'uppercase',
};

const bioFieldInputStyle: CSSProperties = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  boxSizing: 'border-box',
  color: 'var(--color-text)',
  fontSize: '0.85rem',
  fontWeight: 600,
  padding: '0.3rem 0.4rem',
  width: '100%',
};

function BioField(props: { label: string; children: ReactNode }) {
  return (
    <div>
      <p style={bioFieldLabelStyle}>{props.label}</p>
      {props.children}
    </div>
  );
}

/** Calculated, non-editable value shown alongside the editable bio fields. */
function CalculatedBioField(props: { label: string; value: string }) {
  return (
    <div>
      <p style={bioFieldLabelStyle}>{props.label}</p>
      <p style={{ fontWeight: 600, margin: 0 }}>{props.value}</p>
    </div>
  );
}

/**
 * Character bio / physical details panel across the top of the right column.
 * Alignment/Deity/Sex/Age/Height/Weight/Hair/Eyes are the character's own
 * choices, so they're editable here; Vision and Size are derived from race
 * and rendered read-only. Edits are session-local — there is no persisted
 * schema slot for these fields yet (see BLANK_BIO_FIELDS' call site), so
 * they are lost on close/reopen until a future cycle wires storage.
 */
function DetailsPanel(props: { vision: string; size: string; bio: BioFields; onBioChange: (patch: Partial<BioFields>) => void }) {
  const { bio, onBioChange } = props;
  return (
    <div style={{ ...panel, marginBottom: '1rem', padding: '0.75rem 1rem' }}>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.66rem', letterSpacing: '0.06em', margin: '0 0 0.6rem', textTransform: 'uppercase' }}>
        Character Details
      </p>
      <div style={{ display: 'grid', gap: '0.75rem 1.25rem', gridTemplateColumns: 'repeat(auto-fit, minmax(120px, 1fr))' }}>
        <BioField label="Alignment">
          <select style={bioFieldInputStyle} value={bio.alignment} onChange={(event) => onBioChange({ alignment: event.target.value })}>
            <option value="">—</option>
            {ALIGNMENT_OPTIONS.map((option) => (
              <option key={option} value={option}>
                {option}
              </option>
            ))}
          </select>
        </BioField>
        <BioField label="Deity">
          <input style={bioFieldInputStyle} value={bio.deity} onChange={(event) => onBioChange({ deity: event.target.value })} />
        </BioField>
        <BioField label="Sex">
          <select style={bioFieldInputStyle} value={bio.sex} onChange={(event) => onBioChange({ sex: event.target.value })}>
            <option value="">—</option>
            <option value="male">Male</option>
            <option value="female">Female</option>
          </select>
        </BioField>
        <BioField label="Age">
          <select style={bioFieldInputStyle} value={bio.age} onChange={(event) => onBioChange({ age: event.target.value })}>
            <option value="">—</option>
            {AGE_OPTIONS.map((option) => (
              <option key={option} value={option}>
                {option}
              </option>
            ))}
          </select>
        </BioField>
        <BioField label="Height">
          <input style={bioFieldInputStyle} value={bio.height} onChange={(event) => onBioChange({ height: event.target.value })} />
        </BioField>
        <BioField label="Weight">
          <input style={bioFieldInputStyle} value={bio.weight} onChange={(event) => onBioChange({ weight: event.target.value })} />
        </BioField>
        <BioField label="Hair">
          <input style={bioFieldInputStyle} value={bio.hair} onChange={(event) => onBioChange({ hair: event.target.value })} />
        </BioField>
        <BioField label="Eyes">
          <input style={bioFieldInputStyle} value={bio.eyes} onChange={(event) => onBioChange({ eyes: event.target.value })} />
        </BioField>
        <CalculatedBioField label="Vision" value={props.vision} />
        <CalculatedBioField label="Size" value={props.size} />
      </div>
      <p style={{ color: 'var(--color-text-faint)', fontSize: '0.7rem', margin: '0.6rem 0 0' }}>
        Vision and Size are calculated from race and aren't editable. The other fields aren't saved to the
        character file yet — edits here only last for this session.
      </p>
    </div>
  );
}

const WEAPON_COLUMNS = ['Weapon', 'Attack', 'Damage', 'Critical', 'Type', 'Range'] as const;

function WeaponsTab(props: { proficiency: WeaponProficiency; onAddWeapon: () => void }) {
  const categories: ReadonlyArray<{ label: string; proficient: boolean }> = [
    { label: 'Simple', proficient: props.proficiency.simple },
    { label: 'Martial', proficient: props.proficiency.martial },
    { label: 'Exotic', proficient: props.proficiency.exotic },
  ];
  return (
    <div>
      {/* PF1 weapon proficiency categories */}
      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '1.5rem', justifyContent: 'center', marginBottom: '0.5rem' }}>
        {categories.map((category) => (
          <span key={category.label} style={{ alignItems: 'center', display: 'flex', fontSize: '0.9rem', gap: '0.4rem' }}>
            <span aria-hidden style={{ color: category.proficient ? 'var(--color-accent)' : 'var(--color-text-faint)', fontWeight: 800 }}>
              {category.proficient ? '✓' : '✗'}
            </span>
            <span style={{ color: category.proficient ? 'var(--color-text)' : 'var(--color-text-muted)' }}>{category.label} Weapons</span>
          </span>
        ))}
      </div>
      <p style={{ color: 'var(--color-text-faint)', fontSize: '0.72rem', margin: '0 0 1.25rem', textAlign: 'center' }}>
        Proficiency granted by class; exotic weapons require the Exotic Weapon Proficiency feat.
      </p>

      <div style={{ display: 'flex', gap: '0.6rem', justifyContent: 'center', marginBottom: '1.25rem' }}>
        <button type="button" onClick={props.onAddWeapon} style={addItemButtonStyle}>
          Add Weapon
        </button>
        <button type="button" style={addItemButtonStyle}>
          Print
        </button>
      </div>

      {/* PF1 weapon table header */}
      <div
        style={{
          borderBottom: '1px solid var(--color-border)',
          color: 'var(--color-text-muted)',
          display: 'grid',
          fontSize: '0.72rem',
          gridTemplateColumns: '2fr repeat(5, 1fr)',
          gap: '0.5rem',
          letterSpacing: '0.04em',
          paddingBottom: '0.4rem',
          textTransform: 'uppercase',
        }}
      >
        {WEAPON_COLUMNS.map((column) => (
          <span key={column}>{column}</span>
        ))}
      </div>
      <p style={{ color: 'var(--color-text-faint)', margin: '1.25rem 0 0', textAlign: 'center' }}>No weapons added yet.</p>
    </div>
  );
}

/**
 * Spell-school reachability, sourced from `compute_pilot_with_corpus` via
 * the real IPC boundary — not mock data. Resolved against a small bundled
 * corpus-fixture set (see `src-tauri/src/corpus_fixtures.rs`), not the full
 * PCGen corpus, so only schools with a selected, resolvable spell appear
 * here; this is a reachability proof, not a spellbook or slot tracker
 * (spell slots, DCs, and prepared/known posture remain out of scope).
 */
function SpellsTab(props: { corpusDerived: CorpusDerivedDto | undefined; onAddSpell: () => void }) {
  const schools = props.corpusDerived?.schoolCoverage ?? [];
  return (
    <div>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', margin: '0 0 1rem', textAlign: 'center' }}>
        Corpus-derived spell-school reachability — proves each spell resolves against the real
        PF1 corpus; does not compute slots, DCs, or prepared/known posture.
      </p>
      <div style={{ display: 'flex', gap: '0.6rem', justifyContent: 'center', marginBottom: '1.25rem' }}>
        <button type="button" onClick={props.onAddSpell} style={addItemButtonStyle}>
          Add Spell
        </button>
      </div>
      {schools.length === 0 ? (
        <p style={{ color: 'var(--color-text-faint)', margin: 0, textAlign: 'center' }}>
          No corpus-reachable spells selected yet.
        </p>
      ) : (
        schools.map((school) => (
          <div key={school.school} style={{ borderBottom: '1px solid var(--color-border)', padding: '0.5rem 0' }}>
            <span style={{ fontWeight: 700 }}>{school.school}</span>
            <span style={{ color: 'var(--color-text-muted)', marginLeft: '0.5rem' }}>
              {school.spells.join(', ')}
            </span>
            {school.grounded ? (
              <span style={{ color: 'var(--color-accent)', fontSize: '0.7rem', marginLeft: '0.5rem' }}>✓ grounded</span>
            ) : null}
          </div>
        ))
      )}
    </div>
  );
}

/**
 * Equipped-item reachability, sourced from `compute_pilot_with_corpus` via
 * the real IPC boundary — not mock data. Same bundled-fixture scope note
 * as `SpellsTab`: derived stats (armor bonus, attack bonus, etc.) are a
 * documented capability-slice non-goal and are not yet populated.
 */
function GearTab(props: { corpusDerived: CorpusDerivedDto | undefined; onAddArmor: () => void }) {
  const items = props.corpusDerived?.equippedItems ?? [];
  return (
    <div>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', margin: '0 0 1rem', textAlign: 'center' }}>
        Corpus-derived equipment reachability — proves each item resolves against the real PF1
        corpus; derived combat stats are not yet computed.
      </p>
      <div style={{ display: 'flex', gap: '0.6rem', justifyContent: 'center', marginBottom: '1.25rem' }}>
        <button type="button" onClick={props.onAddArmor} style={addItemButtonStyle}>
          Add Armor
        </button>
      </div>
      {items.length === 0 ? (
        <p style={{ color: 'var(--color-text-faint)', margin: 0, textAlign: 'center' }}>
          No corpus-reachable equipment selected yet.
        </p>
      ) : (
        items.map((item) => (
          <div key={item.itemId} style={{ borderBottom: '1px solid var(--color-border)', padding: '0.5rem 0' }}>
            <span style={{ fontWeight: 700 }}>{item.equipmentRecordName}</span>
            <span style={{ color: 'var(--color-text-muted)', fontSize: '0.7rem', marginLeft: '0.5rem' }}>
              ({item.itemId})
            </span>
            {item.grounded ? (
              <span style={{ color: 'var(--color-accent)', fontSize: '0.7rem', marginLeft: '0.5rem' }}>✓ grounded</span>
            ) : null}
          </div>
        ))
      )}
    </div>
  );
}

// ---------- root ----------

export function CharacterSheet(props: {
  row: CharacterHubListRowSurface;
  detail: LoadSavedCharacterResponse | null;
  onClose: () => void;
  /**
   * Called with the fresh detail after a saved-character mutation (level-up,
   * add-equipment, or add-spell) succeeds. `CharacterSheet` is a controlled/
   * presentational component — it does not own a copy of `detail` — so the
   * parent is responsible for updating whatever state it passed in as
   * `detail` (and, since the Level box/class panel/Progression rail derive
   * from `row.classSummary` rather than `detail`, for rebuilding `row` from
   * the refreshed `detail.summary` too).
   */
  onDetailRefreshed: (detail: LoadSavedCharacterResponse) => void;
}) {
  const [tab, setTab] = useState<Tab>('Weapons');
  const [menuOpen, setMenuOpen] = useState(false);
  const [leftCollapsed, setLeftCollapsed] = useState(false);
  const [levelUpOpen, setLevelUpOpen] = useState(false);
  // Covers every saved-character mutation this sheet can trigger (level-up,
  // add-equipment, add-spell) — one error slot, not three near-duplicates,
  // since only one mutation can be in flight from this sheet at a time.
  const [mutationError, setMutationError] = useState<string | null>(null);
  const [itemPickerOpen, setItemPickerOpen] = useState<'weapon' | 'armor' | 'spell' | null>(null);
  const [bio, setBio] = useState<BioFields>({ ...BLANK_BIO_FIELDS });
  const [skillAllocation, setSkillAllocation] = useState<Record<string, number>>({ ...DEFAULT_SKILL_ALLOCATION });
  const [skillDialogOpen, setSkillDialogOpen] = useState(false);

  async function handleLevelUpAccept(classId: string) {
    setMutationError(null);
    try {
      const outcome = await levelUpCharacter({
        characterId: props.row.characterId,
        classId,
        savedAt: new Date().toISOString(),
      });
      const refresh = toCharacterMutationRefresh(outcome);
      if (refresh.kind === 'blocked') {
        setMutationError(refresh.message);
        return;
      }
      props.onDetailRefreshed(refresh.detail);
    } catch (cause: unknown) {
      setMutationError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  async function handleAddEquipment(entry: ItemPickerEntry) {
    setMutationError(null);
    try {
      // The user picking an item from the catalog to add to their loadout
      // is, by construction, actively equipping it — `EquippedActive` is
      // the only choice that matches that action without asking the user
      // to make an extra decision the picker's scope doesn't cover.
      const outcome = await addEquipmentSelection({
        characterId: props.row.characterId,
        itemId: entry.key,
        activeState: 'EquippedActive',
        savedAt: new Date().toISOString(),
      });
      const refresh = toCharacterMutationRefresh(outcome);
      if (refresh.kind === 'blocked') {
        setMutationError(refresh.message);
        return;
      }
      props.onDetailRefreshed(refresh.detail);
    } catch (cause: unknown) {
      setMutationError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  async function handleAddSpell(entry: ItemPickerEntry) {
    setMutationError(null);
    // `add_spell_selection` requires a `sourceClassId`; the picker's scope
    // is "search, filter, pick a spell" (no class chooser), so this defaults
    // to the character's first held class — see `heldClasses` below, parsed
    // from the same `classSummary` the Level box already reads.
    const primaryClassId = heldClasses[0]?.classId;
    if (!primaryClassId) {
      setMutationError('This character has no class to learn the spell from yet.');
      return;
    }
    try {
      const outcome = await addSpellSelection({
        characterId: props.row.characterId,
        spellId: entry.key,
        sourceClassId: primaryClassId,
        // "Known" is the closest default to "the character now has access
        // to this spell" without picking a prepared-caster's daily list —
        // out of scope for a search-and-select picker.
        acquisitionMode: 'Known',
        savedAt: new Date().toISOString(),
      });
      const refresh = toCharacterMutationRefresh(outcome);
      if (refresh.kind === 'blocked') {
        setMutationError(refresh.message);
        return;
      }
      props.onDetailRefreshed(refresh.detail);
    } catch (cause: unknown) {
      setMutationError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  function updateBio(patch: Partial<BioFields>) {
    setBio((prev) => ({ ...prev, ...patch }));
  }

  const snapshot = props.detail?.snapshot ?? null;
  const abilities = snapshot?.abilityModifiers ?? ZERO_ABILITIES;
  const ac = snapshot?.baselineArmorClass ?? 10;
  const saves = snapshot?.totalSaves ?? { fortitude: 0, reflex: 0, will: 0 };
  const dexMod = abilities.dexterity;
  const touch = 10 + dexMod;
  const flatFooted = ac - Math.max(0, dexMod);

  const heldClasses = parseHeldClasses(props.row.classSummary);
  const classLabel = formatHeldClasses(props.row.classSummary); // e.g. "Fighter 3 / Wizard 1"
  const level = totalCharacterLevel(props.row.classSummary);
  const casterLvl = casterLevel(props.row.classSummary);
  const isHuman = props.row.raceLabel.toLowerCase() === 'human';
  const skillPointsFor = (benefit: LevelEntry) => totalSkillPoints(benefit.skillPointsBase, abilities.intelligence, isHuman);

  // Every character level already taken (numbered by character level), then the
  // next level available for each held class.
  const currentBenefits = buildLevelEntries(heldClasses);
  const nextBenefits = buildNextEntries(heldClasses);

  const race = RACE_OPTIONS.find((entry) => entry.id === props.detail?.summary.raceId);
  const size = race?.size ?? 'Medium';
  const vision = race?.vision ?? 'Normal';
  const baseAttackBonus = snapshot?.baseAttackBonus ?? 0;
  const hp = maxHitPoints(heldClasses, abilities.constitution);
  const cmb = baseAttackBonus + abilities.strength;
  const cmd = 10 + baseAttackBonus + abilities.strength + dexMod;

  // Weapon proficiency is the union across all held classes.
  const weaponProficiency = heldClasses.reduce<WeaponProficiency>(
    (accumulated, held) => {
      const classProficiency = classWeaponProficiency(held.classId);
      return {
        simple: accumulated.simple || classProficiency.simple,
        martial: accumulated.martial || classProficiency.martial,
        exotic: accumulated.exotic || classProficiency.exotic,
      };
    },
    { simple: false, martial: false, exotic: false }
  );

  const menuItems: ReadonlyArray<{ label: string; onSelect: () => void; dividerBefore?: boolean }> = [
    { label: 'Open', onSelect: () => {} },
    { label: 'Save', onSelect: () => {} },
    { label: 'Clone', onSelect: () => {} },
    { label: 'Print', onSelect: () => window.print() },
  ];

  // One generic `ItemPickerModal` backs all three "Add …" affordances — the
  // Add Weapon / Add Armor pickers both narrow the equipment catalog to the
  // combined "ArmsArmor" category (see `WEAPONS_AND_ARMOR_CATEGORY`'s doc
  // comment) and the Add Spell picker narrows the spell catalog by nothing
  // server-side, relying on the modal's own search box.
  const itemPickerConfig =
    itemPickerOpen === 'weapon' || itemPickerOpen === 'armor'
      ? {
          title: itemPickerOpen === 'weapon' ? 'Add Weapon' : 'Add Armor',
          searchPlaceholder: 'Search arms & armor…',
          loadEntries: () =>
            listEquipment({ nameContains: null, category: WEAPONS_AND_ARMOR_CATEGORY }).then((response) =>
              mapEquipmentCatalogEntries(response.entries)
            ),
          onSelect: handleAddEquipment,
        }
      : itemPickerOpen === 'spell'
        ? {
            title: 'Add Spell',
            searchPlaceholder: 'Search spells…',
            loadEntries: () => listSpells({ nameContains: null, school: null }).then((response) => mapSpellCatalogEntries(response.entries)),
            onSelect: handleAddSpell,
          }
        : null;

  return (
    <div style={{ marginLeft: 'calc(50% - 50vw)', marginTop: '-3rem', width: '100vw' }}>
      {/* Top menu bar — extra right padding + height keep the fixed gear clear of the accent line */}
      <div
        style={{
          alignItems: 'center',
          borderBottom: '2px solid var(--color-accent)',
          display: 'flex',
          gap: '1.5rem',
          padding: '0.95rem 4rem 0.95rem 1.5rem',
        }}
      >
        {/* Menu dropdown */}
        <div style={{ position: 'relative' }}>
          <button
            type="button"
            onClick={() => setMenuOpen((open) => !open)}
            style={{ background: 'none', border: 'none', color: 'var(--color-text)', cursor: 'pointer', fontSize: '1.1rem', fontWeight: 700, padding: 0 }}
          >
            ☰ Menu
          </button>
          {menuOpen ? (
            <>
              <div onClick={() => setMenuOpen(false)} style={{ inset: 0, position: 'fixed', zIndex: 20 }} />
              <div
                style={{
                  ...panel,
                  backgroundColor: 'var(--color-surface)',
                  boxShadow: '0 8px 24px rgba(0, 0, 0, 0.45)',
                  left: 0,
                  minWidth: 160,
                  overflow: 'hidden',
                  position: 'absolute',
                  top: 'calc(100% + 10px)',
                  zIndex: 21,
                }}
              >
                {menuItems.map((item) => (
                  <div key={item.label}>
                    {item.dividerBefore ? <div style={{ borderTop: '1px solid var(--color-border)', margin: '0.25rem 0' }} /> : null}
                    <button
                      type="button"
                      onClick={() => {
                        setMenuOpen(false);
                        item.onSelect();
                      }}
                      style={{
                        background: 'none',
                        border: 'none',
                        color: 'var(--color-text)',
                        cursor: 'pointer',
                        display: 'block',
                        fontSize: '0.9rem',
                        padding: '0.55rem 1rem',
                        textAlign: 'left',
                        width: '100%',
                      }}
                    >
                      {item.label}
                    </button>
                  </div>
                ))}
              </div>
            </>
          ) : null}
        </div>
        <button
          type="button"
          onClick={props.onClose}
          title="Close character"
          style={{ background: 'none', border: 'none', color: 'var(--color-text-muted)', cursor: 'pointer', fontSize: '1rem' }}
        >
          ✕
        </button>
        <span style={{ fontWeight: 600 }}>
          {props.row.displayLabel} — {classLabel}
        </span>
      </div>

      {/* Body: collapsible progression | stats + weapons | details + skills */}
      <div style={{ display: 'flex', gap: '0.75rem', padding: '1rem 1.5rem' }}>
        {/* LEFT: collapsible level progression, separated by a vertical line */}
        <div
          style={{
            borderRight: '1px solid var(--color-border)',
            flex: leftCollapsed ? '0 0 34px' : '0 0 240px',
            paddingRight: '0.75rem',
          }}
        >
          <div style={{ alignItems: 'center', display: 'flex', justifyContent: leftCollapsed ? 'center' : 'space-between', marginBottom: '0.6rem' }}>
            {!leftCollapsed ? (
              <span style={{ color: 'var(--color-text-muted)', fontSize: '0.7rem', fontWeight: 700, letterSpacing: '0.06em', textTransform: 'uppercase' }}>
                Progression
              </span>
            ) : null}
            <button
              type="button"
              onClick={() => setLeftCollapsed((collapsed) => !collapsed)}
              title={leftCollapsed ? 'Expand progression' : 'Collapse progression'}
              style={{ background: 'none', border: '1px solid var(--color-border)', borderRadius: 6, color: 'var(--color-text-secondary)', cursor: 'pointer', padding: '0.1rem 0.4rem' }}
            >
              {leftCollapsed ? '»' : '«'}
            </button>
          </div>

          {!leftCollapsed ? (
            <>
              <div style={{ marginBottom: '0.75rem' }}>
                <PortraitUpload characterId={props.row.characterId} />
              </div>

              {/* Level / XP */}
              <div style={{ ...panel, display: 'flex', gap: '0.5rem', marginBottom: '0.5rem', padding: '0.4rem' }}>
                <div style={{ ...panel, backgroundColor: 'var(--color-surface-2)', flex: 1, padding: '0.3rem 0.5rem', position: 'relative', textAlign: 'center' }}>
                  <p style={{ color: 'var(--color-text-muted)', fontSize: '0.6rem', margin: 0 }}>Level</p>
                  <p style={{ fontWeight: 800, margin: 0 }}>{level}</p>
                  <button
                    type="button"
                    onClick={() => setLevelUpOpen(true)}
                    title="Level up"
                    aria-label="Level up"
                    style={{
                      alignItems: 'center',
                      background: 'var(--color-accent)',
                      border: 'none',
                      borderRadius: '50%',
                      color: 'var(--color-on-accent)',
                      cursor: 'pointer',
                      display: 'flex',
                      fontSize: '0.7rem',
                      fontWeight: 800,
                      height: 16,
                      justifyContent: 'center',
                      lineHeight: 1,
                      padding: 0,
                      position: 'absolute',
                      right: 4,
                      top: 4,
                      width: 16,
                    }}
                  >
                    +
                  </button>
                </div>
                <div style={{ ...panel, backgroundColor: 'var(--color-surface-2)', flex: 1, padding: '0.3rem 0.5rem', textAlign: 'center' }}>
                  <p style={{ color: 'var(--color-text-muted)', fontSize: '0.6rem', margin: 0 }}>Caster Level</p>
                  <p style={{ fontWeight: 800, margin: 0 }}>{casterLvl > 0 ? casterLvl : '—'}</p>
                </div>
              </div>

              {mutationError ? (
                <p
                  role="alert"
                  style={{
                    backgroundColor: 'var(--color-surface-2)',
                    border: '1px solid var(--color-danger, #c0392b)',
                    borderRadius: 6,
                    color: 'var(--color-danger, #c0392b)',
                    fontSize: '0.75rem',
                    margin: '0 0 0.5rem',
                    padding: '0.4rem 0.55rem',
                  }}
                >
                  {mutationError}
                </p>
              ) : null}

              <LevelUpDialog
                open={levelUpOpen}
                onClose={() => setLevelUpOpen(false)}
                heldClasses={heldClasses}
                intelligenceModifier={abilities.intelligence}
                isHuman={isHuman}
                onAccept={handleLevelUpAccept}
              />

              <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
                <NavCard label="Race" value={props.row.raceLabel} />
                <NavCard label="Class" value={classLabel} />
              </div>

              {/* Current levels — each level taken and what it granted */}
              <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem', marginTop: '1rem' }}>
                {currentBenefits.map((benefit) => (
                  <LevelBenefitCard
                    key={`${benefit.classId}-${benefit.characterLevel}`}
                    benefit={benefit}
                    skillPoints={skillPointsFor(benefit)}
                    variant="current"
                  />
                ))}
              </div>

              {/* Next — the next level available for each held class */}
              <div style={{ alignItems: 'center', display: 'flex', gap: '0.5rem', margin: '1rem 0 0.6rem' }}>
                <span style={{ borderTop: '1px solid var(--color-border)', flex: 1 }} />
                <span style={{ color: 'var(--color-accent)', fontSize: '0.8rem', fontWeight: 800, letterSpacing: '0.08em', textTransform: 'uppercase' }}>
                  Next
                </span>
                <span style={{ borderTop: '1px solid var(--color-border)', flex: 1 }} />
              </div>
              <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
                {nextBenefits.map((benefit) => (
                  <LevelBenefitCard
                    key={`next-${benefit.classId}-${benefit.characterLevel}`}
                    benefit={benefit}
                    skillPoints={skillPointsFor(benefit)}
                    variant="next"
                  />
                ))}
              </div>
            </>
          ) : null}
        </div>

        {/* MIDDLE: stat panels on top, weapons/defense/gear at the bottom */}
        <div style={{ display: 'flex', flex: 1, flexDirection: 'column', gap: '0.6rem', minWidth: 0 }}>
          <IdentityPanel name={props.row.displayLabel} campaign={props.row.campaign ?? '—'} />
          <AbilitiesPanel abilities={abilities} />
          <InitiativeHpPanel initiative={dexMod} hp={hp} />

          <div style={{ display: 'flex', gap: '0.6rem' }}>
            <div style={{ display: 'flex', flex: 1, flexDirection: 'column', gap: '0.6rem', minWidth: 0 }}>
              <ArmorClassPanel ac={ac} touch={touch} flatFooted={flatFooted} />
              <AttackPanel baseAttackBonus={baseAttackBonus} cmb={cmb} cmd={cmd} />
            </div>
            <div style={{ display: 'flex', flex: 1, flexDirection: 'column', gap: '0.6rem', minWidth: 0 }}>
              <SavingThrowsPanel saves={saves} />
              <SpeedPanel />
            </div>
          </div>

          {/* Weapons / Defense / Gear — bottom, spanning the middle */}
          <div style={{ marginTop: '0.4rem' }}>
            <div style={{ borderBottom: '1px solid var(--color-border)', display: 'flex', flexWrap: 'wrap', gap: '1.25rem', marginBottom: '1rem' }}>
              {TABS.map((name) => {
                const active = name === tab;
                return (
                  <button
                    key={name}
                    type="button"
                    onClick={() => setTab(name)}
                    style={{
                      background: 'none',
                      border: 'none',
                      borderBottom: `2px solid ${active ? 'var(--color-accent)' : 'transparent'}`,
                      color: active ? 'var(--color-text)' : 'var(--color-text-muted)',
                      cursor: 'pointer',
                      fontSize: '0.95rem',
                      fontWeight: active ? 700 : 500,
                      padding: '0.3rem 0.1rem 0.6rem',
                    }}
                  >
                    {name}
                  </button>
                );
              })}
            </div>

            <div style={{ ...panel, minHeight: 200, padding: '1.25rem' }}>
              {tab === 'Weapons' ? (
                <WeaponsTab proficiency={weaponProficiency} onAddWeapon={() => setItemPickerOpen('weapon')} />
              ) : tab === 'Spells' ? (
                <SpellsTab corpusDerived={props.detail?.corpusDerived} onAddSpell={() => setItemPickerOpen('spell')} />
              ) : tab === 'Gear' ? (
                <GearTab corpusDerived={props.detail?.corpusDerived} onAddArmor={() => setItemPickerOpen('armor')} />
              ) : (
                <p style={{ color: 'var(--color-text-faint)', margin: 0, textAlign: 'center' }}>{tab} — coming soon.</p>
              )}
            </div>
          </div>
        </div>

        {/* RIGHT: character details, then skills beneath */}
        <div style={{ flex: '0 0 300px', minWidth: 0 }}>
          <DetailsPanel vision={vision} size={size} bio={bio} onBioChange={updateBio} />
          <SkillsPanel
            abilities={abilities}
            heldClasses={heldClasses}
            isHuman={isHuman}
            allocation={skillAllocation}
            realModifiers={snapshot?.selectedSkillModifiers}
            onOpenDialog={() => setSkillDialogOpen(true)}
          />
        </div>
      </div>

      <SkillAllocationDialog
        open={skillDialogOpen}
        onClose={() => setSkillDialogOpen(false)}
        heldClasses={heldClasses}
        characterLevel={level}
        abilities={abilities}
        totalPoints={totalSkillPointsAvailable(heldClasses, abilities.intelligence, isHuman)}
        allocation={skillAllocation}
        onAccept={setSkillAllocation}
      />

      <ItemPickerModal
        open={itemPickerConfig !== null}
        title={itemPickerConfig?.title ?? ''}
        searchPlaceholder={itemPickerConfig?.searchPlaceholder ?? ''}
        loadEntries={itemPickerConfig?.loadEntries ?? (() => Promise.resolve([]))}
        onClose={() => setItemPickerOpen(null)}
        onSelect={(entry) => itemPickerConfig?.onSelect(entry)}
      />
    </div>
  );
}
