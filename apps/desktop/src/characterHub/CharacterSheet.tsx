import { useEffect, useState, type CSSProperties, type ReactNode } from 'react';
import type { CharacterHubListRowSurface } from './buildCharacterHubListSurface';
import {
  loadSavedCharacterDetail,
  type ExplanationDto,
  type LoadSavedCharacterResponse,
  type SpellSelectionDto,
  type WeaponDamageDto,
} from '../boundary/loadSavedCharacterDetail';
import { previewLevelUp as previewLevelUpGrants } from '../boundary/previewLevelUp';
import { buildClassFeatureSurface } from './classFeaturesModel';
import { buildWeaponsTabSurface, ABSENT as ABSENT_FACET } from './weaponsTabModel';
import { buildSpellsPerDaySurface } from './spellsPerDayModel';
import type {
  AbilityScoresDto,
  CorpusDerivedDto,
  EncumbranceDto,
  EquipmentEffectsDto,
  ResolvedEquipmentDto,
} from '../boundary/loadCreateCharacter';
import { buildAcBySourceRows, describeEncumbrance, effectiveMaxDexCap } from './encumbranceTabModel';
import { levelUpCharacter } from '../boundary/levelUpCharacter';
import { purchaseEquipment } from '../boundary/purchaseEquipment';
import { attachEquipmentModifier } from '../boundary/attachEquipmentModifier';
import { addSpellSelection } from '../boundary/addSpellSelection';
import { recordAndPrepareSpellSelection } from '../boundary/recordAndPrepareSpellSelection';
import { addFeatSelection } from '../boundary/addFeatSelection';
import { listEquipment } from '../boundary/listEquipment';
import { listSpells } from '../boundary/listSpells';
import { listFeats } from '../boundary/listFeats';
import { cloneCharacter } from '../boundary/cloneCharacter';
import { recomputeCharacter, type RecomputedCharacterSnapshotDto } from '../boundary/recomputeCharacter';
import { buildRecomputeCharacterRequest } from './characterHubRuntime';
import type { RuleSetId } from './LandingScreen';
import { blockedMessageFromDiagnostics, toCharacterMutationRefresh } from './characterSheetRefresh';
import { resolveSpellRouting } from './spellRoutingModel';
import { mapEquipmentCatalogEntries, mapFeatCatalogEntries, mapSpellCatalogEntries } from './itemPickerFilter';
import { describeFeatTarget, mergeChosenFeatTarget, resolveSelectedFeatEntries } from './featsTabModel';
import {
  describeSpellAcquisition,
  describeSpellSchoolAndLevel,
  resolveSelectedSpellEntries,
  spellSourceClassIds,
} from './spellsTabModel';
import { buildPetsTabView } from './petsTabModel';
import type { PilotSnapshotDto } from '../boundary/loadCreateCharacter';
import type { SpellCatalogEntryDto } from '../boundary/loadSpellCatalog';
import { loadClassSpellLevels, type ClassSpellLevelsDto } from '../boundary/loadClassSpellLevels';
import { ItemPickerModal, type ItemPickerEntry } from './ItemPickerModal';
import { listWeaponTargets } from '../boundary/listWeaponTargets';
import type { ChosenFeatTargetsDto } from '../boundary/loadSavedCharacterDetail';
import {
  featTargetPickerTitle,
  skillTargetOptions,
  spellSchoolTargetOptions,
  weaponTargetOptions,
} from './featTargetOptions';
import {
  buildLevelEntries,
  buildNextEntries,
  casterLevel,
  classWeaponProficiency,
  formatHeldClasses,
  levelGrantsFeat,
  maxHitPoints,
  parseHeldClasses,
  previewLevelUp,
  totalCharacterLevel,
  totalSkillPoints,
  type HeldClass,
  type LevelEntry,
  type WeaponProficiency,
} from './characterProgression';
import { AGE_OPTIONS, ALIGNMENT_OPTIONS, deriveRaceTraits } from './characterHubModel';
import { PortraitUpload } from './PortraitUpload';
import { LevelUpDialog } from './LevelUpDialog';
import { SkillAllocationDialog } from './SkillAllocationDialog';
import { DEFAULT_SKILL_ALLOCATION, SKILLS, isClassSkill, skillIdFor, skillModifier, skillRankCost, totalSkillPointsAvailable } from './skillsModel';
import { setSkillAllocations } from '../boundary/setSkillAllocations';
import { loadCharacterBio, updateCharacterBio } from '../boundary/characterBio';
import { adjustCharacterMoney, gpToCopper, loadCharacterMoney, type CharacterMoneyDto } from '../boundary/characterMoney';
import { adjustCharacterHp, loadCharacterDurability, type CharacterDurabilityDto } from '../boundary/characterDurability';

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
const EQUIPMODS_CATEGORY = 'Equipmods';

/** Matches `characterHubModel.ts`'s `CLASS_OPTIONS` id for Wizard. */
const WIZARD_CLASS_ID = 'class:wizard';

export interface ItemPickerConfig {
  title: string;
  searchPlaceholder: string;
  loadEntries: () => Promise<ItemPickerEntry[]>;
  onSelect: (entry: ItemPickerEntry) => void;
}

/**
 * Pure dispatch table backing the Add Weapon / Add Armor / Add Spell /
 * Add Feat / Attach Modifier onClick affordances (criterion 7.4): which
 * title to show, which real corpus query to run (`listEquipment` narrowed
 * to `ArmsArmor` or `Equipmods`, the unfiltered `listSpells`, or the
 * unfiltered `listFeats`), and which real mutation handler
 * (`addEquipmentSelection`-backed, `addSpellSelection`-backed,
 * `addFeatSelection`-backed, or `attachEquipmentModifier`-backed) the
 * user's pick gets routed to. Extracted from the render body so it is
 * unit-testable without a DOM — this repo has no jsdom/testing-library —
 * per the same split already used for `itemPickerFilter.ts` and
 * `characterSheetRefresh.ts`.
 */
export function buildItemPickerConfig(
  kind: 'weapon' | 'armor' | 'spell' | 'feat' | 'featTarget' | 'modifier' | null,
  deps: {
    loadEquipment: (category: string) => Promise<ItemPickerEntry[]>;
    loadSpells: () => Promise<ItemPickerEntry[]>;
    loadFeats: () => Promise<ItemPickerEntry[]>;
    loadFeatTargets: () => Promise<ItemPickerEntry[]>;
    onSelectEquipment: (entry: ItemPickerEntry) => void;
    onSelectSpell: (entry: ItemPickerEntry) => void;
    onSelectFeat: (entry: ItemPickerEntry) => void;
    onSelectFeatTarget: (entry: ItemPickerEntry) => void;
    onSelectModifier: (entry: ItemPickerEntry) => void;
  }
): ItemPickerConfig | null {
  if (kind === 'weapon' || kind === 'armor') {
    return {
      title: kind === 'weapon' ? 'Add Weapon' : 'Add Armor',
      searchPlaceholder: 'Search arms & armor…',
      loadEntries: () => deps.loadEquipment(WEAPONS_AND_ARMOR_CATEGORY),
      onSelect: deps.onSelectEquipment,
    };
  }
  if (kind === 'spell') {
    return {
      title: 'Add Spell',
      searchPlaceholder: 'Search spells…',
      loadEntries: deps.loadSpells,
      onSelect: deps.onSelectSpell,
    };
  }
  if (kind === 'feat') {
    return {
      title: 'Add Feat',
      searchPlaceholder: 'Search feats…',
      loadEntries: deps.loadFeats,
      onSelect: deps.onSelectFeat,
    };
  }
  // Second step of the two-step chooser-feat flow: the feat is picked
  // first, then the thing it names. The title is overridden at the call
  // site with the feat's own name, so the user sees what they are choosing
  // for rather than a bare "Choose a target".
  if (kind === 'featTarget') {
    return {
      title: 'Choose a target',
      searchPlaceholder: 'Search targets…',
      loadEntries: deps.loadFeatTargets,
      onSelect: deps.onSelectFeatTarget,
    };
  }
  if (kind === 'modifier') {
    return {
      title: 'Attach Modifier',
      searchPlaceholder: 'Search equipment modifiers…',
      loadEntries: () => deps.loadEquipment(EQUIPMODS_CATEGORY),
      onSelect: deps.onSelectModifier,
    };
  }
  return null;
}

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

// 'Details' and 'Bio' are deliberately absent: that content already renders
// unconditionally in the right-column `DetailsPanel` below, regardless of
// which tab is active, so a duplicate tab selector for it would only ever
// show the generic "coming soon" placeholder next to content that's already
// on screen.
const TABS = ['Weapons', 'Defense', 'Gear', 'Spells', 'Pets', 'Feats', 'Actions', 'Overrides'] as const;
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
 * and rendered read-only. Edits persist to the real `bio.json` sidecar via
 * `onBioBlur` (fired on field blur, not per keystroke, to avoid a Tauri
 * round trip on every character typed) — see `handleBioBlur` in
 * `CharacterSheet`.
 */
function DetailsPanel(props: {
  vision: string;
  size: string;
  bio: BioFields;
  onBioChange: (patch: Partial<BioFields>) => void;
  onBioBlur: () => void;
}) {
  const { bio, onBioChange, onBioBlur } = props;
  return (
    <div style={{ ...panel, marginBottom: '1rem', padding: '0.75rem 1rem' }}>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.66rem', letterSpacing: '0.06em', margin: '0 0 0.6rem', textTransform: 'uppercase' }}>
        Character Details
      </p>
      <div style={{ display: 'grid', gap: '0.75rem 1.25rem', gridTemplateColumns: 'repeat(auto-fit, minmax(120px, 1fr))' }}>
        <BioField label="Alignment">
          <select style={bioFieldInputStyle} value={bio.alignment} onChange={(event) => onBioChange({ alignment: event.target.value })} onBlur={onBioBlur}>
            <option value="">—</option>
            {ALIGNMENT_OPTIONS.map((option) => (
              <option key={option} value={option}>
                {option}
              </option>
            ))}
          </select>
        </BioField>
        <BioField label="Deity">
          <input style={bioFieldInputStyle} value={bio.deity} onChange={(event) => onBioChange({ deity: event.target.value })} onBlur={onBioBlur} />
        </BioField>
        <BioField label="Sex">
          <select style={bioFieldInputStyle} value={bio.sex} onChange={(event) => onBioChange({ sex: event.target.value })} onBlur={onBioBlur}>
            <option value="">—</option>
            <option value="male">Male</option>
            <option value="female">Female</option>
          </select>
        </BioField>
        <BioField label="Age">
          <select style={bioFieldInputStyle} value={bio.age} onChange={(event) => onBioChange({ age: event.target.value })} onBlur={onBioBlur}>
            <option value="">—</option>
            {AGE_OPTIONS.map((option) => (
              <option key={option} value={option}>
                {option}
              </option>
            ))}
          </select>
        </BioField>
        <BioField label="Height">
          <input style={bioFieldInputStyle} value={bio.height} onChange={(event) => onBioChange({ height: event.target.value })} onBlur={onBioBlur} />
        </BioField>
        <BioField label="Weight">
          <input style={bioFieldInputStyle} value={bio.weight} onChange={(event) => onBioChange({ weight: event.target.value })} onBlur={onBioBlur} />
        </BioField>
        <BioField label="Hair">
          <input style={bioFieldInputStyle} value={bio.hair} onChange={(event) => onBioChange({ hair: event.target.value })} onBlur={onBioBlur} />
        </BioField>
        <BioField label="Eyes">
          <input style={bioFieldInputStyle} value={bio.eyes} onChange={(event) => onBioChange({ eyes: event.target.value })} onBlur={onBioBlur} />
        </BioField>
        <CalculatedBioField label="Vision" value={props.vision} />
        <CalculatedBioField label="Size" value={props.size} />
      </div>
      <p style={{ color: 'var(--color-text-faint)', fontSize: '0.7rem', margin: '0.6rem 0 0' }}>
        Vision and Size are calculated from race and aren't editable — a race this build has no profile for
        reads "Unknown" rather than a guessed value. The other fields save automatically when you leave the
        field.
      </p>
    </div>
  );
}

/**
 * The columns are exactly the facets `damage_total.rs` grounds per weapon —
 * base dice, the Strength contribution, weapon enhancement (damage and
 * attack kept apart, because they are separate corpus values), critical,
 * and wield category.
 *
 * The previous header set (`Attack / Damage / Type / Range`) promised two
 * things the engine does not compute: a single summed damage number, and
 * per-weapon type/range. It sat above a hardcoded "No weapons added yet."
 * with no row-rendering path at all.
 *
 * **No summed damage column, deliberately.** No summed damage-roll formula
 * exists anywhere in the engine, and the wield multiplier needed to build
 * one honestly is unknown — `contract.rs`'s `PilotReceipt::weapon_damage`
 * boundary note owns that decision and it stands. Each facet is its own
 * column; the player adds them at the table, where they know their own
 * grip.
 */
const WEAPON_COLUMNS = ['Weapon', 'Base Dice', 'STR', 'Enh. Dmg', 'Enh. Atk', 'Critical', 'Wield'] as const;

const WEAPON_GRID_COLUMNS = '2fr repeat(6, 1fr)';

function WeaponsTab(props: {
  proficiency: WeaponProficiency;
  weaponDamage: readonly WeaponDamageDto[];
  corpusDerived: CorpusDerivedDto | null;
  onAddWeapon: () => void;
}) {
  const categories: ReadonlyArray<{ label: string; proficient: boolean }> = [
    { label: 'Simple', proficient: props.proficiency.simple },
    { label: 'Martial', proficient: props.proficiency.martial },
    { label: 'Exotic', proficient: props.proficiency.exotic },
  ];
  const surface = buildWeaponsTabSurface(props.weaponDamage, props.corpusDerived);
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
        <button type="button" onClick={() => window.print()} style={addItemButtonStyle}>
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
          gridTemplateColumns: WEAPON_GRID_COLUMNS,
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

      {surface.isEmpty ? (
        <p style={{ color: 'var(--color-text-faint)', margin: '1.25rem 0 0', textAlign: 'center' }}>
          No weapons equipped.
        </p>
      ) : (
        surface.rows.map((row) => (
          <div key={row.itemId} style={{ borderBottom: '1px solid var(--color-border)', padding: '0.45rem 0' }}>
            <div
              style={{
                display: 'grid',
                fontSize: '0.85rem',
                gap: '0.5rem',
                gridTemplateColumns: WEAPON_GRID_COLUMNS,
              }}
            >
              <span style={{ color: 'var(--color-text)', fontWeight: 700 }}>{row.name}</span>
              <span style={{ color: 'var(--color-text)' }}>{row.baseDice}</span>
              <span style={{ color: 'var(--color-text)' }}>{row.strDamage}</span>
              <span style={{ color: 'var(--color-text)' }}>{row.enhancementDamage}</span>
              <span style={{ color: 'var(--color-text)' }}>{row.enhancementAttack}</span>
              <span style={{ color: 'var(--color-text)' }}>{row.critical}</span>
              <span style={{ color: 'var(--color-text)' }}>{row.wield}</span>
            </div>
            {row.featEffects.length > 0 ? (
              <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.72rem', margin: '0.25rem 0 0' }}>
                Feat damage: {row.featEffects.join(', ')}
              </p>
            ) : null}
          </div>
        ))
      )}

      {/*
        The one thing this table deliberately does not show. Stating it is
        not an apology for a missing feature — it tells the player which
        arithmetic is theirs, so nobody reads the columns as an incomplete
        total.
      */}
      <p style={{ color: 'var(--color-text-faint)', fontSize: '0.7rem', margin: '0.9rem 0 0', textAlign: 'center' }}>
        Facets are shown separately, not summed: the damage total depends on how you wield the weapon
        this round, which the engine cannot know. {ABSENT_FACET} means the weapon's corpus record carries
        no value for that facet.
      </p>
    </div>
  );
}

/**
 * Honest "not shown" signal for `CorpusDerivedDto.unresolvedSpellIds` /
 * `unresolvedEquipmentItemIds` — real, disk-persisted selections that fall
 * outside the desktop app's tiny bundled demo corpus (`corpus_fixtures.rs`,
 * ~4 records total) and so can't be resolved to a display name, school, or
 * grounded status the way the rest of this tab renders. Shows only the raw
 * ids, which is all that's cleanly available — no fabricated detail about
 * what they are beyond that. This is exactly the mechanism behind the
 * "0 of 3" / "1 of 3" spell-count inconsistency found in the frontend's own
 * live smoke test — not a Wizard/race bug.
 */
function UnresolvedNotice(props: { ids: string[]; kind: 'spell' | 'item' }) {
  const noun = props.ids.length === 1 ? props.kind : `${props.kind}s`;
  return (
    <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', margin: '0 0 1rem', textAlign: 'center' }}>
      {props.ids.length} {noun} not shown (outside demo corpus): {props.ids.join(', ')}
    </p>
  );
}

/**
 * The character's own spells, each resolved to its real name, school, level
 * and effect text.
 *
 * This tab used to render *only* `corpusDerived.schoolCoverage`. That
 * section is real, but `load_saved_character` builds it via
 * `compute_pilot_with_corpus(&..., corpus_fixture_bundle())`, and
 * `corpus_fixtures.rs`'s `SPELL_FIXTURES` is two files
 * (`spell_abjuration.txt`, `spell_illusion.txt`) — so every other spell a
 * character actually held resolved against nothing, landed in
 * `unresolvedSpellIds`, and reached the player as a bare internal id. The
 * full 652-record catalog, with level and effect text, was in
 * `SPELL_LIST` the whole time and already served by the same `listSpells`
 * command the Add Spell picker calls (see `spellsTabModel.ts` for the full
 * trace). Same defect the Feats tab already fixed: correct in the engine,
 * invisible where it mattered.
 *
 * Per the operator's standing ruling, a spell's real description is a
 * legitimate deliverable in its own right. This tab still computes no slots,
 * DCs or prepared/known posture, and says so rather than implying otherwise.
 *
 * **The level shown is now the level for the row's own source class.** The
 * catalog record's `level` is the minimum across every class on the record,
 * so this tab used to show a Wizard "Level 1" for Hideous Laughter
 * (`CLASSES:Bard=1|Sorcerer,Wizard=2`) — 67 of the 580 Wizard-list spells
 * read wrong that way, always low. Two loads feed the rows: `listSpells`
 * for the record, and `loadClassSpellLevels` for the per-class level of
 * each class this character actually learned spells from. Because every
 * selection persists its own `sourceClassId`, a multiclass sheet needs no
 * arbitration — each row answers for its own class. See `spellsTabModel.ts`.
 */
function SpellsTab(props: {
  spellsSelected: SpellSelectionDto[];
  corpusDerived: CorpusDerivedDto | undefined;
  /** Drives the spells-per-day block — see `spellsPerDayModel.ts`. */
  explanations: readonly ExplanationDto[];
  onAddSpell: () => void;
}) {
  const [catalog, setCatalog] = useState<SpellCatalogEntryDto[] | null>(null);
  useEffect(() => {
    let cancelled = false;
    listSpells({ nameContains: null, school: null })
      .then((response) => {
        if (!cancelled) {
          setCatalog(response.entries);
        }
      })
      .catch(() => {
        // Falls back to the raw-id rendering below (via an empty catalog, so
        // every spell resolves to `resolved: false`) rather than an alarming
        // error — the raw ids are still real, honest data.
        if (!cancelled) {
          setCatalog([]);
        }
      });
    return () => {
      cancelled = true;
    };
  }, []);

  // Only the classes this character actually learned spells from, so a
  // multiclass sheet pulls each of its lists and nothing else. Joined into
  // a string so the effect re-runs when the set changes, not on every
  // render of an equal array.
  const sourceClassIds = spellSourceClassIds(props.spellsSelected);
  const sourceClassKey = sourceClassIds.join('|');
  const [classSpellLevels, setClassSpellLevels] = useState<ClassSpellLevelsDto[]>([]);
  useEffect(() => {
    let cancelled = false;
    if (sourceClassIds.length === 0) {
      setClassSpellLevels([]);
      return;
    }
    loadClassSpellLevels(sourceClassIds)
      .then((response) => {
        if (!cancelled) {
          setClassSpellLevels(response.classes);
        }
      })
      .catch(() => {
        // An empty list leaves every row `class-list-unknown`, which reads
        // as a labelled "lowest class level" rather than claiming a
        // per-class level this load never delivered.
        if (!cancelled) {
          setClassSpellLevels([]);
        }
      });
    return () => {
      cancelled = true;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [sourceClassKey]);

  const rows = resolveSelectedSpellEntries(props.spellsSelected, catalog ?? [], classSpellLevels);
  const schools = props.corpusDerived?.schoolCoverage ?? [];
  // Real `class_spell.*.<total|base>_<spells|extracts>_per_day.*` records,
  // for whichever casters this build actually grounds — replacing a
  // hardcoded Wizard-only, levels-1-to-9 table that used to live in
  // `characterProgression.ts`.
  const spellsPerDay = buildSpellsPerDaySurface(props.explanations);

  return (
    <div>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', margin: '0 0 0.4rem', textAlign: 'center' }}>
        Spells this character knows, from the real spell catalog. DCs and prepared/known posture are
        not computed.
      </p>

      {spellsPerDay.isEmpty ? null : (
        <div style={{ borderBottom: '1px solid var(--color-border)', margin: '0 0 1rem', paddingBottom: '0.75rem' }}>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', fontWeight: 700, letterSpacing: '0.04em', margin: '0 0 0.4rem', textTransform: 'uppercase' }}>
            Spells per day
          </p>
          {spellsPerDay.rows.map((row) => (
            <div key={`${row.classToken}-${row.spellLevel}`} style={{ padding: '0.3rem 0' }}>
              <span style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', marginRight: '0.5rem', textTransform: 'capitalize' }}>
                {row.classToken}
              </span>
              <span style={{ color: 'var(--color-text)', fontSize: '0.85rem' }}>
                {row.spellLevel === 0 ? 'Cantrips' : `Level ${row.spellLevel}`}
              </span>
              <span style={{ color: 'var(--color-accent)', fontSize: '0.85rem', fontWeight: 800, marginLeft: '0.5rem' }}>
                {row.count}
              </span>
              {/*
                A base count is not a total: the engine grounded the class
                table's own column but not the casting stat's bonus spells.
                Saying so is cheaper than showing a number that quietly
                means something else.
              */}
              {row.basis === 'base' ? (
                <span style={{ color: 'var(--color-text-faint)', fontSize: '0.68rem', marginLeft: '0.4rem' }}>
                  (class table only — bonus spells not included)
                </span>
              ) : null}
              {/* The engine's own derivation, verbatim. */}
              <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.68rem', margin: '0.1rem 0 0' }}>
                {row.detail}
              </p>
            </div>
          ))}
        </div>
      )}
      {/* Each row's level now comes from the per-class spell list for that
          row's own `sourceClassId`, via `list_class_spell_levels` — so a
          Wizard reads Hideous Laughter (`CLASSES:Bard=1|Sorcerer,Wizard=2`)
          as "Wizard level 2", not the record's minimum-across-classes 1
          this tab used to show every class. Where no per-class list exists
          for a source class (Magus, Summoner, Oracle — real casters whose
          lists are not ingested), the row falls back to the record's own
          number under an explicit "Lowest class level" label rather than
          attributing it to a class. See `spellsTabModel.ts`. */}
      <p style={{ color: 'var(--color-text-faint)', fontSize: '0.68rem', margin: '0 0 1rem', textAlign: 'center' }}>
        Each level is the level for that spell&rsquo;s own source class. Rows reading
        &ldquo;Lowest class level&rdquo; are classes with no spell list ingested yet &mdash; that
        number is the spell&rsquo;s lowest level across all classes, not this one&rsquo;s.
      </p>
      <div style={{ display: 'flex', gap: '0.6rem', justifyContent: 'center', marginBottom: '1.25rem' }}>
        <button type="button" onClick={props.onAddSpell} style={addItemButtonStyle}>
          Add Spell
        </button>
      </div>
      {rows.length === 0 ? (
        <p style={{ color: 'var(--color-text-faint)', margin: 0, textAlign: 'center' }}>No spells selected yet.</p>
      ) : (
        rows.map((row, index) => (
          <div key={`${row.raw}-${index}`} style={{ borderBottom: '1px solid var(--color-border)', padding: '0.5rem 0' }}>
            <span style={{ fontWeight: 700 }}>{row.name}</span>
            {describeSpellSchoolAndLevel(row) === null ? null : (
              <span style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', marginLeft: '0.5rem' }}>
                {describeSpellSchoolAndLevel(row)}
              </span>
            )}
            <p style={{ color: 'var(--color-text-faint)', fontSize: '0.72rem', margin: '0.15rem 0 0' }}>
              {describeSpellAcquisition(row)}
            </p>
            {row.effectText === null ? null : (
              <p style={{ color: 'var(--color-text-muted)', fontSize: '0.78rem', margin: '0.25rem 0 0' }}>
                {row.effectText}
              </p>
            )}
          </div>
        ))
      )}
      {schools.length === 0 ? null : (
        <div style={{ marginTop: '1.5rem' }}>
          <p style={{ color: 'var(--color-text-faint)', fontSize: '0.7rem', margin: '0 0 0.4rem', textAlign: 'center' }}>
            Engine corpus-resolution receipt — which selected spells the rules engine resolved
            against its bundled demo corpus. Separate from the catalog lookup above.
          </p>
          {schools.map((school) => (
            <div key={school.school} style={{ borderTop: '1px solid var(--color-border)', padding: '0.4rem 0' }}>
              <span style={{ fontWeight: 700, fontSize: '0.78rem' }}>{school.school}</span>
              <span style={{ color: 'var(--color-text-muted)', fontSize: '0.78rem', marginLeft: '0.5rem' }}>
                {school.spells.join(', ')}
              </span>
              {school.grounded ? (
                <span style={{ color: 'var(--color-accent)', fontSize: '0.7rem', marginLeft: '0.5rem' }}>✓ grounded</span>
              ) : null}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

/**
 * View-and-spend money panel, wired to the real `load_character_money` /
 * `adjust_character_money` commands. The gp input converts to the wire's
 * canonical copper delta via `gpToCopper` (positive to add funds, negative
 * to spend) — the backend rejects a spend that would go negative rather
 * than silently allowing it, surfaced here as a real error message.
 */
function MoneyPanel(props: {
  money: CharacterMoneyDto;
  busy: boolean;
  error: string | null;
  onAdjust: (gpAmount: number) => void;
}) {
  const [gpInput, setGpInput] = useState('');
  const parsedGp = Number(gpInput);
  const validAmount = gpInput.trim() !== '' && Number.isFinite(parsedGp) && parsedGp > 0;

  return (
    <div style={{ ...panel, marginBottom: '1rem', padding: '0.75rem 1rem' }}>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.66rem', letterSpacing: '0.06em', margin: '0 0 0.6rem', textTransform: 'uppercase' }}>
        Money
      </p>
      <div style={{ display: 'flex', gap: '0.5rem', marginBottom: '0.6rem' }}>
        <StatTile label="PP" value={props.money.platinum} />
        <StatTile label="GP" value={props.money.gold} />
        <StatTile label="SP" value={props.money.silver} />
        <StatTile label="CP" value={props.money.copper} />
      </div>
      <div style={{ alignItems: 'center', display: 'flex', gap: '0.5rem' }}>
        <input
          type="number"
          min="0"
          step="0.01"
          placeholder="gp amount"
          value={gpInput}
          onChange={(event) => setGpInput(event.target.value)}
          style={{ ...bioFieldInputStyle, width: 110 }}
        />
        <button
          type="button"
          disabled={!validAmount || props.busy}
          onClick={() => {
            props.onAdjust(parsedGp);
            setGpInput('');
          }}
          style={{ ...addItemButtonStyle, cursor: validAmount && !props.busy ? 'pointer' : 'not-allowed', opacity: validAmount && !props.busy ? 1 : 0.5, padding: '0.4rem 0.9rem' }}
        >
          Add
        </button>
        <button
          type="button"
          disabled={!validAmount || props.busy}
          onClick={() => {
            props.onAdjust(-parsedGp);
            setGpInput('');
          }}
          style={{ ...addItemButtonStyle, cursor: validAmount && !props.busy ? 'pointer' : 'not-allowed', opacity: validAmount && !props.busy ? 1 : 0.5, padding: '0.4rem 0.9rem' }}
        >
          Spend
        </button>
      </div>
      {props.error ? (
        <p role="alert" style={{ color: 'var(--color-danger, #c0392b)', fontSize: '0.72rem', margin: '0.5rem 0 0' }}>
          {props.error}
        </p>
      ) : null}
    </div>
  );
}

/**
 * Real HP/durability tracking, wired to `load_character_durability` /
 * `adjust_character_hp` (v0.6 alpha swarm, risks-and-open-questions.md
 * item 4). `durability` is `null` both while the initial load is in
 * flight and — indistinguishably, on purpose — when this build isn't
 * durability-supported (only single-class Fighter/Wizard/Rogue); either
 * way, the honest "not available" line is correct and nothing is
 * fabricated in its place. `status` is a pre-computed label from the real
 * PF1 injury/death thresholds — rendered directly, no client-side
 * re-derivation.
 */
function DurabilityPanel(props: {
  durability: CharacterDurabilityDto | null;
  busy: boolean;
  error: string | null;
  onAdjust: (deltaHp: number) => void;
}) {
  const [amountInput, setAmountInput] = useState('');
  const parsedAmount = Number(amountInput);
  const validAmount = amountInput.trim() !== '' && Number.isFinite(parsedAmount) && parsedAmount > 0;

  if (!props.durability) {
    return (
      <p style={{ color: 'var(--color-text-faint)', margin: '0 0 1rem', textAlign: 'center' }}>
        HP tracking isn't available for this build yet — only single-class Fighter, Wizard, or Rogue.
      </p>
    );
  }

  const { currentHp, maxHp, nonlethalDamage, status } = props.durability;
  return (
    <div style={{ ...panel, marginBottom: '1rem', padding: '0.75rem 1rem' }}>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.66rem', letterSpacing: '0.06em', margin: '0 0 0.6rem', textTransform: 'uppercase' }}>
        Hit Points
      </p>
      <div style={{ display: 'flex', gap: '0.5rem', marginBottom: '0.6rem' }}>
        <StatTile label="Current" value={currentHp} />
        <StatTile label="Max" value={maxHp} />
        <StatTile label="Nonlethal" value={nonlethalDamage} />
        <StatTile label="Status" value={status} emphasize={status !== 'Normal'} />
      </div>
      <div style={{ alignItems: 'center', display: 'flex', gap: '0.5rem' }}>
        <input
          type="number"
          min="0"
          step="1"
          placeholder="amount"
          value={amountInput}
          onChange={(event) => setAmountInput(event.target.value)}
          style={{ ...bioFieldInputStyle, width: 110 }}
        />
        <button
          type="button"
          disabled={!validAmount || props.busy}
          onClick={() => {
            props.onAdjust(-parsedAmount);
            setAmountInput('');
          }}
          style={{ ...addItemButtonStyle, cursor: validAmount && !props.busy ? 'pointer' : 'not-allowed', opacity: validAmount && !props.busy ? 1 : 0.5, padding: '0.4rem 0.9rem' }}
        >
          Damage
        </button>
        <button
          type="button"
          disabled={!validAmount || props.busy}
          onClick={() => {
            props.onAdjust(parsedAmount);
            setAmountInput('');
          }}
          style={{ ...addItemButtonStyle, cursor: validAmount && !props.busy ? 'pointer' : 'not-allowed', opacity: validAmount && !props.busy ? 1 : 0.5, padding: '0.4rem 0.9rem' }}
        >
          Heal
        </button>
      </div>
      {props.error ? (
        <p role="alert" style={{ color: 'var(--color-danger, #c0392b)', fontSize: '0.72rem', margin: '0.5rem 0 0' }}>
          {props.error}
        </p>
      ) : null}
    </div>
  );
}

/**
 * Real, corpus-resolved equipment-effect totals (v0.6 alpha swarm item 1,
 * shape (c)) — not claim-gated, reflects whatever gear is actually
 * equipped regardless of whether the build reaches `Computed`.
 * `armorClassDelta`/`armorCheckPenaltyTotal` are always real (a real `0`
 * when nothing equipped grants either, not "absent"). `maxDexCap`/
 * `spellFailureChance`/`attackBonusDelta` only render when genuinely
 * present — most builds today only produce non-trivial values from the
 * fixed starting loadout, and a missing tile here is the honest state for
 * everything else, not a bug.
 */
function EquipmentEffectsPanel(props: { effects: EquipmentEffectsDto | undefined }) {
  if (!props.effects) {
    return null;
  }
  const { armorClassDelta, armorCheckPenaltyTotal, maxDexCap, spellFailureChance, attackBonusDelta } = props.effects;
  return (
    <div style={{ ...panel, marginBottom: '1rem', padding: '0.75rem 1rem' }}>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.66rem', letterSpacing: '0.06em', margin: '0 0 0.6rem', textTransform: 'uppercase' }}>
        Equipment Effects
      </p>
      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem' }}>
        <StatTile label="AC Bonus" value={fmt(armorClassDelta)} />
        <StatTile label="Armor Check Penalty" value={fmt(armorCheckPenaltyTotal)} />
        {maxDexCap !== undefined ? <StatTile label="Max Dex" value={maxDexCap} /> : null}
        {spellFailureChance !== undefined ? <StatTile label="Spell Failure" value={`${spellFailureChance}%`} /> : null}
        {attackBonusDelta !== undefined ? <StatTile label="Attack Bonus" value={fmt(attackBonusDelta)} /> : null}
      </div>
    </div>
  );
}

/**
 * Real, bounded: HP/durability tracking and equipment effects (above) are
 * real for whatever this build actually has equipped. The only other
 * Defense stat with a backend computation to show is the flat Damage
 * Reduction magnitude (`PilotSnapshotDto.damageReduction`,
 * `character_hub.rs`) — currently only ever grounded for Barbarian, which
 * isn't a chassis-supported class through this UI yet, so `undefined` here
 * is the expected, honest state for every character reachable today, not a
 * bug. AC breakdown *by source* (which item contributed what) and save
 * modifiers by source have no equivalent backend computation, so that part
 * of the tab stays an honest placeholder rather than a fabricated layout
 * for uncomputed data. Note: the "Recompute" menu action doesn't currently
 * refresh `damageReduction` (`RecomputedCharacterSnapshotDto` doesn't carry
 * it), so it always reflects the originally loaded snapshot — a real,
 * narrow, pre-existing gap, not something this change papers over.
 */
function DefenseTab(props: {
  damageReduction: number | undefined;
  equipmentEffects: EquipmentEffectsDto | undefined;
  encumbrance: EncumbranceDto | undefined;
  durability: CharacterDurabilityDto | null;
  durabilityBusy: boolean;
  durabilityError: string | null;
  onAdjustHp: (deltaHp: number) => void;
}) {
  return (
    <div>
      <DurabilityPanel
        durability={props.durability}
        busy={props.durabilityBusy}
        error={props.durabilityError}
        onAdjust={props.onAdjustHp}
      />
      <EquipmentEffectsPanel effects={props.equipmentEffects} />
      <AcBySourcePanel effects={props.equipmentEffects} encumbrance={props.encumbrance} />
      {props.damageReduction !== undefined ? (
        <p style={{ margin: '0 0 1rem', textAlign: 'center' }}>
          <span style={{ fontWeight: 700 }}>Damage Reduction:</span> {props.damageReduction}/—
        </p>
      ) : null}
      <p style={{ color: 'var(--color-text-faint)', margin: 0, textAlign: 'center' }}>
        Save modifiers by source — coming soon.
      </p>
    </div>
  );
}

/**
 * AC breakdown by source: which equipped item contributed what.
 *
 * This replaced a "coming soon" placeholder, but the data behind it is not
 * new — `equipment_effects::compute_equipment_effects` has always populated
 * `EquipmentEffects.per_item` with each item's real corpus-derived armor
 * bonus, max Dex, check penalty and spell failure. Only the aggregate
 * totals crossed the IPC boundary, so the per-source detail the engine
 * already knew was invisible. Exposing `EquipmentEffectsDto.perItem` is
 * what made this renderable.
 *
 * The load row is a genuinely separate source: an encumbered character
 * takes a max-Dex cap and check penalty from the *weight carried*, not from
 * any worn item. It only appears when the current load actually imposes
 * one.
 */
function AcBySourcePanel(props: {
  effects: EquipmentEffectsDto | undefined;
  encumbrance: EncumbranceDto | undefined;
}) {
  const rows = buildAcBySourceRows(props.effects?.perItem ?? []);
  const loadMaxDex = props.encumbrance?.loadMaxDexCap;
  const loadCheckPenalty = props.encumbrance?.loadArmorCheckPenalty ?? 0;
  const loadContributes = loadMaxDex !== undefined || loadCheckPenalty !== 0;
  if (rows.length === 0 && !loadContributes) {
    return null;
  }
  const effectiveMaxDex = effectiveMaxDexCap(props.effects?.maxDexCap, loadMaxDex);
  return (
    <div style={{ ...panel, marginBottom: '1rem', padding: '0.75rem 1rem' }}>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.66rem', letterSpacing: '0.06em', margin: '0 0 0.6rem', textTransform: 'uppercase' }}>
        AC by Source
      </p>
      {rows.map((row) => (
        <div
          key={row.itemId}
          style={{ alignItems: 'baseline', borderBottom: '1px solid var(--color-border)', display: 'flex', gap: '0.5rem', justifyContent: 'space-between', padding: '0.3rem 0' }}
        >
          <span style={{ fontWeight: 700 }}>{row.label}</span>
          <span style={{ color: 'var(--color-text-secondary)', fontSize: '0.74rem' }}>
            {[
              `AC ${fmt(row.armorClassBonus)}`,
              row.maxDex !== undefined ? `Max Dex +${row.maxDex}` : null,
              row.armorCheckPenalty !== undefined && row.armorCheckPenalty !== 0
                ? `Check ${row.armorCheckPenalty}`
                : null,
              row.spellFailure !== undefined ? `Spell Fail ${row.spellFailure}%` : null,
            ]
              .filter(Boolean)
              .join(' · ')}
          </span>
        </div>
      ))}
      {loadContributes ? (
        <div style={{ alignItems: 'baseline', borderBottom: '1px solid var(--color-border)', display: 'flex', gap: '0.5rem', justifyContent: 'space-between', padding: '0.3rem 0' }}>
          <span style={{ fontWeight: 700 }}>Encumbrance</span>
          <span style={{ color: 'var(--color-text-secondary)', fontSize: '0.74rem' }}>
            {[
              loadMaxDex !== undefined ? `Max Dex +${loadMaxDex}` : null,
              loadCheckPenalty !== 0 ? `Check ${loadCheckPenalty}` : null,
            ]
              .filter(Boolean)
              .join(' · ')}
          </span>
        </div>
      ) : null}
      <div style={{ display: 'flex', gap: '0.5rem', justifyContent: 'space-between', paddingTop: '0.45rem' }}>
        <span style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem' }}>
          Total AC bonus from equipment
        </span>
        <span style={{ fontWeight: 800 }}>{fmt(props.effects?.armorClassDelta ?? 0)}</span>
      </div>
      {effectiveMaxDex !== undefined ? (
        <p style={{ color: 'var(--color-text-faint)', fontSize: '0.7rem', margin: '0.4rem 0 0' }}>
          Effective max Dex bonus to AC: +{effectiveMaxDex} — the tighter of worn armor and current
          load, which never sum.
        </p>
      ) : null}
    </div>
  );
}

/**
 * Real carried weight against PF1's Strength-derived carrying capacity,
 * with the current load tier and the penalties that tier imposes.
 *
 * Every number here is computed by `rules_core::encumbrance` from the
 * corpus's own `WT:`/`COST:` tokens and the real PCGen Pathfinder
 * `load.lst` capacity table — none of it is derived in the frontend. The
 * engine has computed all of it since the v0.6 alpha swarm; it simply had
 * no path across the IPC boundary until `CorpusDerivedDto.encumbrance`
 * existed, so none of it was ever visible to a player.
 */
function EncumbrancePanel(props: { encumbrance: EncumbranceDto | undefined }) {
  if (!props.encumbrance) {
    return null;
  }
  const described = describeEncumbrance(props.encumbrance);
  const barColor = described.overCapacity
    ? 'var(--color-danger, #c0392b)'
    : described.penalties.length > 0
      ? 'var(--color-warning, #d68910)'
      : 'var(--color-accent)';
  return (
    <div style={{ ...panel, marginBottom: '1rem', padding: '0.75rem 1rem' }}>
      <div style={{ alignItems: 'baseline', display: 'flex', justifyContent: 'space-between', marginBottom: '0.6rem' }}>
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.66rem', letterSpacing: '0.06em', margin: 0, textTransform: 'uppercase' }}>
          Carried Weight
        </p>
        <span style={{ color: barColor, fontSize: '0.72rem', fontWeight: 800 }}>{described.levelLabel}</span>
      </div>
      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginBottom: '0.6rem' }}>
        <StatTile label="Total Weight" value={described.totalWeightLabel} emphasize />
        <StatTile label="Light / Med / Heavy" value={described.capacityLabel} />
        <StatTile label="Remaining" value={`${described.remainingLbs} lb`} />
        <StatTile label="Gear Value" value={described.totalCostLabel} />
      </div>
      {/* Proportion of the heavy maximum currently carried. */}
      <div
        aria-hidden
        style={{ backgroundColor: 'var(--color-surface-2)', borderRadius: '999px', height: '0.35rem', overflow: 'hidden' }}
      >
        <div style={{ backgroundColor: barColor, height: '100%', width: `${described.fractionOfCapacity * 100}%` }} />
      </div>
      {described.penalties.length > 0 ? (
        <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.72rem', margin: '0.6rem 0 0' }}>
          <span style={{ fontWeight: 700 }}>This load imposes:</span>{' '}
          {described.penalties.map((penalty) => `${penalty.label} ${penalty.value}`).join(' · ')}
        </p>
      ) : null}
      {described.overCapacity ? (
        <p role="alert" style={{ color: 'var(--color-danger, #c0392b)', fontSize: '0.72rem', margin: '0.6rem 0 0' }}>
          Carrying more than this character's heavy maximum.
        </p>
      ) : null}
      {described.unresolvedCount > 0 ? (
        <p style={{ color: 'var(--color-text-faint)', fontSize: '0.7rem', margin: '0.5rem 0 0' }}>
          {described.unresolvedCount} carried item(s) have no corpus weight and are excluded from this total.
        </p>
      ) : null}
    </div>
  );
}

/**
 * Equipped-item reachability, sourced from `compute_pilot_with_corpus` via
 * the real IPC boundary — not mock data. Per-item weight and price are the
 * records' own real corpus `WT:`/`COST:` tokens; an item with no price
 * shown is a genuine corpus absence (an unpriced base template, or a
 * formula-priced modifier), not a lookup failure.
 */
function GearTab(props: {
  corpusDerived: CorpusDerivedDto | undefined;
  onAddArmor: () => void;
  onAttachModifier: (item: ResolvedEquipmentDto) => void;
  money: CharacterMoneyDto;
  moneyBusy: boolean;
  moneyError: string | null;
  onAdjustMoney: (gpAmount: number) => void;
}) {
  const items = props.corpusDerived?.equippedItems ?? [];
  const unresolved = props.corpusDerived?.unresolvedEquipmentItemIds ?? [];
  const encumbrance = props.corpusDerived?.encumbrance;
  // Per-item weight/price, keyed by the same `itemId` the equipped-items
  // list is keyed on, so each row can show what it actually contributes.
  const carriedById = new Map((encumbrance?.perItem ?? []).map((entry) => [entry.itemId, entry]));
  return (
    <div>
      <MoneyPanel money={props.money} busy={props.moneyBusy} error={props.moneyError} onAdjust={props.onAdjustMoney} />
      <EncumbrancePanel encumbrance={encumbrance} />
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', margin: '0 0 1rem', textAlign: 'center' }}>
        Corpus-derived equipment — each item resolves against the real PF1 corpus, and its weight
        and price are that record's own corpus values.
      </p>
      <div style={{ display: 'flex', gap: '0.6rem', justifyContent: 'center', marginBottom: '1.25rem' }}>
        <button type="button" onClick={props.onAddArmor} style={addItemButtonStyle}>
          Add Armor
        </button>
      </div>
      {unresolved.length > 0 ? <UnresolvedNotice ids={unresolved} kind="item" /> : null}
      {items.length === 0 ? (
        <p style={{ color: 'var(--color-text-faint)', margin: 0, textAlign: 'center' }}>
          No corpus-reachable equipment selected yet.
        </p>
      ) : (
        items.map((item) => (
          <div key={item.itemId} style={{ borderBottom: '1px solid var(--color-border)', padding: '0.5rem 0' }}>
            <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between' }}>
              <div>
                <span style={{ fontWeight: 700 }}>{item.equipmentRecordName}</span>
                <span style={{ color: 'var(--color-text-muted)', fontSize: '0.7rem', marginLeft: '0.5rem' }}>
                  ({item.itemId})
                </span>
                {item.grounded ? (
                  <span style={{ color: 'var(--color-accent)', fontSize: '0.7rem', marginLeft: '0.5rem' }}>✓ grounded</span>
                ) : null}
                {/* Real corpus WT:/COST: values. A row with no price is a
                    genuine corpus absence, so nothing is rendered for it
                    rather than a fabricated 0 gp. */}
                {carriedById.has(item.itemId) ? (
                  <span style={{ color: 'var(--color-text-secondary)', fontSize: '0.7rem', marginLeft: '0.5rem' }}>
                    {carriedById.get(item.itemId)!.weightLbs} lb
                    {carriedById.get(item.itemId)!.costGp !== undefined
                      ? ` · ${carriedById.get(item.itemId)!.costGp} gp`
                      : ''}
                  </span>
                ) : null}
              </div>
              <button
                type="button"
                onClick={() => props.onAttachModifier(item)}
                style={{ ...addItemButtonStyle, fontSize: '0.7rem', padding: '0.3rem 0.6rem' }}
              >
                Attach Modifier
              </button>
            </div>
            {item.appliedModifiers.length > 0 ? (
              <ul style={{ margin: '0.35rem 0 0', paddingLeft: '1.25rem' }}>
                {item.appliedModifiers.map((modifier) => (
                  <li key={modifier.itemId} style={{ color: 'var(--color-text-secondary)', fontSize: '0.78rem' }}>
                    {modifier.equipmentRecordName}
                    {modifier.grounded ? (
                      <span style={{ color: 'var(--color-accent)', fontSize: '0.7rem', marginLeft: '0.4rem' }}>✓ grounded</span>
                    ) : null}
                  </li>
                ))}
              </ul>
            ) : null}
          </div>
        ))
      )}
    </div>
  );
}

/**
 * "Class Features & Special Abilities" — the engine's own
 * `class_feature.*` / `class_chassis.*` records for this exact build:
 * every magnitude it computed, each under the corpus-cited derivation it
 * wrote.
 *
 * This tab used to render a hand-authored client-side table of bare labels
 * (`'Bravery +1'`, `'Bonus combat feat'`) covering two classes, with no
 * magnitudes and no provenance — a second, uncited source of rules truth
 * sitting on top of 411 records the engine already computed, tested and
 * cited on every load and then dropped at the IPC boundary. The table is
 * deleted; these rows are the engine's.
 *
 * `notComputed` is rendered as its own section, without numbers: those
 * records are the engine saying a facet is not grounded, and their `value`
 * is a filler zero. Showing them as "0" would flatten `Blocked` into a
 * magnitude, which is exactly what this sheet must not do.
 *
 * The universal per-level benefits (a feat at every odd character level,
 * an ability score increase every 4th) stay alongside, because they are
 * PF1 general rules keyed to character level, not entries from any class
 * table.
 */
function ActionsTab(props: {
  levelEntries: LevelEntry[];
  explanations: readonly ExplanationDto[];
  heldClasses: HeldClass[];
}) {
  const surface = buildClassFeatureSurface(props.explanations, props.heldClasses);
  const generalBenefits = props.levelEntries.flatMap((entry) =>
    entry.features.map((feature) => ({ characterLevel: entry.characterLevel, feature }))
  );

  if (surface.features.length === 0 && surface.notComputed.length === 0 && generalBenefits.length === 0) {
    return (
      <p style={{ color: 'var(--color-text-faint)', margin: 0, textAlign: 'center' }}>
        No class features granted yet.
      </p>
    );
  }

  return (
    <div>
      {surface.features.length > 0 ? (
        <>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', margin: '0 0 1rem', textAlign: 'center' }}>
            Every class feature the rules engine computed for this build, with its own derivation.
          </p>
          {surface.features.map((row) => (
            <div key={row.id} style={{ borderBottom: '1px solid var(--color-border)', padding: '0.5rem 0' }}>
              <div style={{ alignItems: 'baseline', display: 'flex', gap: '0.6rem' }}>
                {row.classToken ? (
                  <span style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', minWidth: 80, textTransform: 'capitalize' }}>
                    {row.classToken}
                  </span>
                ) : (
                  <span style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', minWidth: 80 }}>Chassis</span>
                )}
                <span style={{ color: 'var(--color-text)', fontSize: '0.85rem', fontWeight: 700 }}>{row.label}</span>
                <span style={{ color: 'var(--color-accent)', fontSize: '0.85rem', fontWeight: 800 }}>{row.value}</span>
              </div>
              {/*
                The engine's own corpus-cited derivation, verbatim. Never
                paraphrased here: it is the rules citation, and for records
                whose magnitude alone is incomplete (a sneak-attack die
                *count* of 6 means 6d6) it is the only place the full
                expression appears.
              */}
              <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.72rem', margin: '0.2rem 0 0 calc(80px + 0.6rem)' }}>
                {row.detail}
              </p>
            </div>
          ))}
        </>
      ) : (
        <p style={{ color: 'var(--color-text-faint)', fontSize: '0.8rem', margin: '0 0 1rem', textAlign: 'center' }}>
          The rules engine grounded no class-feature records for this build.
        </p>
      )}

      {surface.notComputed.length > 0 ? (
        <div style={{ marginTop: '1.25rem' }}>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', fontWeight: 700, letterSpacing: '0.04em', margin: '0 0 0.4rem', textTransform: 'uppercase' }}>
            Not computed
          </p>
          {surface.notComputed.map((notice) => (
            <div key={notice.id} style={{ borderBottom: '1px solid var(--color-border)', padding: '0.4rem 0' }}>
              <span style={{ color: 'var(--color-text)', fontSize: '0.82rem', fontWeight: 700 }}>{notice.label}</span>
              <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.72rem', margin: '0.15rem 0 0' }}>
                {notice.detail}
              </p>
            </div>
          ))}
        </div>
      ) : null}

      {generalBenefits.length > 0 ? (
        <div style={{ marginTop: '1.25rem' }}>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', fontWeight: 700, letterSpacing: '0.04em', margin: '0 0 0.4rem', textTransform: 'uppercase' }}>
            Universal level benefits
          </p>
          {generalBenefits.map((row, index) => (
            <div
              key={`${row.characterLevel}-${row.feature}-${index}`}
              style={{ alignItems: 'baseline', borderBottom: '1px solid var(--color-border)', display: 'flex', gap: '0.6rem', padding: '0.35rem 0' }}
            >
              <span style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', minWidth: 80 }}>
                Level {row.characterLevel}
              </span>
              <span style={{ color: 'var(--color-text)', fontSize: '0.82rem' }}>{row.feature}</span>
            </div>
          ))}
        </div>
      ) : null}
    </div>
  );
}

/**
 * Add Feat picker + the character's full persisted feat list, sourced from
 * `load_saved_character`'s `selectedFeats` field (backend commit `1509124`)
 * — real, complete, not just feats added this session.
 *
 * Each entry in `selectedFeats` is a raw internal selection string, not
 * display text — and not even a single consistent shape: it may be the
 * catalog's own human-readable `key` (e.g. `"Deflect Arrows"`, what the
 * "Add Feat" picker itself pushes) or the rules engine's lowercase
 * `feat:snake_case` selection token (e.g. `"feat:deflect_arrows"`, what
 * character creation seeds and `pilot_compute.rs`'s gates match against —
 * see `featsTabModel.ts`'s doc comment for the full trace). This tab loads
 * the same real `listFeats` catalog the picker loads (unfiltered, once per
 * mount) and resolves each selected feat to its catalog name + description
 * via `resolveSelectedFeatEntries`. This is not cosmetic: some feats (e.g.
 * Deflect Arrows — see `pilot_compute.rs` around line 24487) carry no
 * numeric magnitude anywhere in the rules engine at all; the description
 * text rendered here is their complete, correct mechanical representation.
 * A selected feat that resolves to no catalog entry (a non-CRB feat, since
 * today's catalog is CRB-only — see `feat_catalog.rs`'s own doc comment —
 * or any other genuine mismatch) falls back to the raw string rather than
 * being hidden or shown blank.
 */
function FeatsTab(props: {
  selectedFeats: string[];
  chosenFeatTargets: ChosenFeatTargetsDto[];
  onAddFeat: () => void;
}) {
  const [catalog, setCatalog] = useState<ItemPickerEntry[] | null>(null);
  useEffect(() => {
    let cancelled = false;
    listFeats({ nameContains: null, category: null })
      .then((response) => {
        if (!cancelled) {
          setCatalog(mapFeatCatalogEntries(response.entries));
        }
      })
      .catch(() => {
        // Falls back to the raw-string rendering below (via an empty
        // catalog, so every feat resolves to `entry: null`) rather than
        // an alarming error — the raw strings are still real, honest data.
        if (!cancelled) {
          setCatalog([]);
        }
      });
    return () => {
      cancelled = true;
    };
  }, []);

  const resolvedFeats = resolveSelectedFeatEntries(
    props.selectedFeats,
    catalog ?? [],
    props.chosenFeatTargets
  );

  return (
    <div>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', margin: '0 0 1rem', textAlign: 'center' }}>
        Add feats from the real CRB feat catalog.
      </p>
      <div style={{ display: 'flex', gap: '0.6rem', justifyContent: 'center', marginBottom: '1.25rem' }}>
        <button type="button" onClick={props.onAddFeat} style={addItemButtonStyle}>
          Add Feat
        </button>
      </div>
      {resolvedFeats.length === 0 ? (
        <p style={{ color: 'var(--color-text-faint)', margin: 0, textAlign: 'center' }}>
          No feats selected yet.
        </p>
      ) : (
        resolvedFeats.map((row, index) => (
          <div key={`${row.raw}-${index}`} style={{ borderBottom: '1px solid var(--color-border)', padding: '0.5rem 0' }}>
            <span style={{ fontWeight: 700 }}>{row.entry ? row.entry.name : row.raw}</span>
            {describeFeatTarget(row) === null ? null : (
              <p
                style={{
                  color: row.targets.length === 0 ? 'var(--color-text-faint)' : 'var(--color-text)',
                  fontSize: '0.78rem',
                  fontStyle: row.targets.length === 0 ? 'italic' : 'normal',
                  margin: '0.25rem 0 0',
                }}
              >
                {describeFeatTarget(row)}
              </p>
            )}
            {row.entry ? (
              <p style={{ color: 'var(--color-text-muted)', fontSize: '0.78rem', margin: '0.25rem 0 0' }}>
                {row.entry.detail}
              </p>
            ) : null}
          </div>
        ))
      )}
    </div>
  );
}

/**
 * The character's animal companion or mount, rendered from the real
 * computed stat block.
 *
 * This replaced the sheet's generic `"{tab} — coming soon."` placeholder,
 * but nothing about the data is new: `pilot_compute.rs`'s
 * `ground_wolf_companion_stat_block` / `ground_horse_companion_stat_block`
 * have grounded Hit Dice, base attack bonus, all three base saves, hit
 * points, armor class, the natural-armor and Strength advances and the
 * natural attack across all twenty master levels — for Druid, Hunter and
 * the Cavalier's Mount. The values simply had no field to travel in:
 * `PilotSnapshotDto` carried no companion, the same way
 * `EquipmentEffects.per_item` was fully populated and uncarried while the
 * AC-by-source panel sat as a placeholder over it.
 *
 * Nothing here is fabricated. The tab renders exactly the statistics the
 * engine emitted, each beside the engine's own corpus-cited derivation.
 * The columns the grounding deliberately left unbuilt — bonus tricks, the
 * companion's own skills and feats, the player-chosen stat increase at
 * master levels 4/9/14/20, the size advance, and the named abilities
 * Evasion / Devotion / Multiattack — are shown as the engine's own honest
 * `advancement_absent` note, never as invented values
 * (`docs/governance/no-stub-mvp-doctrine.md`).
 */
function PetsTab(props: { snapshot: PilotSnapshotDto | null }) {
  const view = buildPetsTabView(props.snapshot);

  if (view.kind !== 'Companion') {
    return (
      <p style={{ color: 'var(--color-text-faint)', margin: 0, textAlign: 'center' }}>
        {view.message}
      </p>
    );
  }

  return (
    <div>
      <div style={{ marginBottom: '1rem', textAlign: 'center' }}>
        <div style={{ fontSize: '1.05rem', fontWeight: 700 }}>{view.heading}</div>
        <div style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem' }}>{view.subheading}</div>
      </div>

      {view.stats.map((stat) => (
        <div
          key={stat.label}
          style={{ borderBottom: '1px solid var(--color-border)', padding: '0.5rem 0' }}
        >
          <div style={{ display: 'flex', gap: '0.6rem', justifyContent: 'space-between' }}>
            <span style={{ fontSize: '0.85rem' }}>{stat.label}</span>
            <span style={{ fontWeight: 700 }}>{stat.rendered}</span>
          </div>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', margin: '0.25rem 0 0' }}>
            {stat.detail}
          </p>
        </div>
      ))}

      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', margin: '1rem 0 0' }}>
        {view.summaryDetail}
      </p>

      {view.notes.map((note) => (
        <p key={note} style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', margin: '0.5rem 0 0' }}>
          {note}
        </p>
      ))}

      {view.advancementNote === null ? null : (
        <p style={{ color: 'var(--color-text-faint)', fontSize: '0.72rem', margin: '1rem 0 0' }}>
          <span style={{ fontWeight: 700 }}>Not modelled: </span>
          {view.advancementNote}
        </p>
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
  /**
   * The panel's active rule-system adapter (SD-25 Criterion 3.5) — the
   * landing screen's rule-set picker (`LandingScreen.tsx`), threaded down
   * through `CharacterHubPage.tsx`. Every mutation call site this sheet
   * routes through the 3.4 adapter-dispatch seam resolves this via
   * `characterHubRuntime.ts`'s `resolveRuleSystemId` rather than hardcoding
   * `"pf1"` — see the "Recompute" menu action below.
   */
  ruleSet: RuleSetId;
  /** Top-menu "Open": returns to the Load Character screen so the operator can pick a different saved character without losing this one's on-disk state. */
  onOpen: () => void;
  /** Top-menu "Clone": called after a successful clone so the parent can refresh its saved-character list; the sheet stays open on the original (un-cloned) character. */
  onCloned: () => void;
}) {
  const [tab, setTab] = useState<Tab>('Weapons');
  const [menuOpen, setMenuOpen] = useState(false);
  const [leftCollapsed, setLeftCollapsed] = useState(false);
  const [levelUpOpen, setLevelUpOpen] = useState(false);
  // Covers every saved-character mutation this sheet can trigger (level-up,
  // add-equipment, add-spell) — one error slot, not three near-duplicates,
  // since only one mutation can be in flight from this sheet at a time.
  const [mutationError, setMutationError] = useState<string | null>(null);
  const [itemPickerOpen, setItemPickerOpen] = useState<
    'weapon' | 'armor' | 'spell' | 'feat' | 'featTarget' | 'modifier' | null
  >(null);
  // Set between the two steps of adding a chooser feat: the feat has been
  // picked, and the picker has reopened for the thing it names.
  const [pendingFeatTarget, setPendingFeatTarget] = useState<{
    featKey: string;
    featName: string;
    targetKind: string;
  } | null>(null);
  // Set only while a level-up that grants a feat is waiting on the user to
  // pick one via the reused feat `ItemPickerModal` (see `handleLevelUpAccept`/
  // `handleLevelUpFeatPick`) — null the rest of the time, including for a
  // plain Feats-tab "Add Feat" pick, which stays routed to `handleAddFeat`.
  const [pendingFeatLevelUp, setPendingFeatLevelUp] = useState<{ classId: string; newClassLevel: number } | null>(null);
  // Set only while the Gear tab's "Attach Modifier" flow is waiting on the
  // user to pick a modifier via the reused Equipmods `ItemPickerModal` —
  // identifies which already-equipped selection the pick attaches to (the
  // button lives on that specific row, so there is no separate "which
  // weapon" step). items-1-and-27-scoping.md sub-task 6.
  const [pendingModifierAttachment, setPendingModifierAttachment] = useState<ResolvedEquipmentDto | null>(null);
  const [bio, setBio] = useState<BioFields>({ ...BLANK_BIO_FIELDS });
  // Loads the real persisted bio (or the all-empty default for a character
  // that has never saved one) whenever the sheet opens on a different
  // character. A load failure just leaves the blank default up rather than
  // breaking the sheet — bio is pure flavor text, not load-bearing.
  useEffect(() => {
    let cancelled = false;
    loadCharacterBio(props.row.characterId)
      .then((loaded) => {
        if (!cancelled) {
          setBio(loaded);
        }
      })
      .catch(() => {
        if (!cancelled) {
          setBio({ ...BLANK_BIO_FIELDS });
        }
      });
    return () => {
      cancelled = true;
    };
  }, [props.row.characterId]);

  const [money, setMoney] = useState<CharacterMoneyDto>({ totalCopper: 0, platinum: 0, gold: 0, silver: 0, copper: 0 });
  const [moneyBusy, setMoneyBusy] = useState(false);
  const [moneyError, setMoneyError] = useState<string | null>(null);
  // Loads the real persisted balance (or zero for a character that has
  // never saved one) whenever the sheet opens on a different character.
  useEffect(() => {
    let cancelled = false;
    loadCharacterMoney(props.row.characterId)
      .then((loaded) => {
        if (!cancelled) {
          setMoney(loaded);
        }
      })
      .catch(() => {
        if (!cancelled) {
          setMoney({ totalCopper: 0, platinum: 0, gold: 0, silver: 0, copper: 0 });
        }
      });
    return () => {
      cancelled = true;
    };
  }, [props.row.characterId]);
  /**
   * The engine's computed records for this build: every
   * `ComputationExplanation` it emitted, and the per-weapon damage
   * breakdown. Both arrive on `load_saved_character` and on nothing else,
   * so they are held here and re-read after mutations
   * (`refreshEngineRecords`) rather than threaded through the mutation
   * responses, which do not carry them.
   *
   * Seeded from `props.detail` so the browser preview (no Tauri runtime)
   * still renders its sample records.
   */
  const [engineRecords, setEngineRecords] = useState<{
    explanations: ExplanationDto[];
    weaponDamage: WeaponDamageDto[];
  }>({
    explanations: props.detail?.explanations ?? [],
    weaponDamage: props.detail?.weaponDamage ?? [],
  });
  useEffect(() => {
    let cancelled = false;
    setEngineRecords({
      explanations: props.detail?.explanations ?? [],
      weaponDamage: props.detail?.weaponDamage ?? [],
    });
    loadSavedCharacterDetail({ characterId: props.row.characterId })
      .then((loaded) => {
        if (!cancelled) {
          setEngineRecords({ explanations: loaded.explanations, weaponDamage: loaded.weaponDamage });
        }
      })
      .catch(() => {
        // Keeps whatever `props.detail` seeded — the browser preview path.
      });
    return () => {
      cancelled = true;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [props.row.characterId]);

  const [durability, setDurability] = useState<CharacterDurabilityDto | null>(null);
  const [durabilityBusy, setDurabilityBusy] = useState(false);
  const [durabilityError, setDurabilityError] = useState<string | null>(null);
  // Loads the real persisted HP/durability whenever the sheet opens on a
  // different character. Unlike bio/money, a rejection here is a real,
  // structural "this build isn't durability-supported" outcome (only
  // single-class Fighter/Wizard/Rogue), not an expected empty-first-load
  // state — `durability` staying `null` is exactly how the Defense tab
  // renders that as an honest "not available for this build" line rather
  // than a fabricated HP value or an alarming error.
  useEffect(() => {
    let cancelled = false;
    setDurability(null);
    loadCharacterDurability(props.row.characterId)
      .then((loaded) => {
        if (!cancelled) {
          setDurability(loaded);
        }
      })
      .catch(() => {
        if (!cancelled) {
          setDurability(null);
        }
      });
    return () => {
      cancelled = true;
    };
  }, [props.row.characterId]);
  const [skillAllocation, setSkillAllocation] = useState<Record<string, number>>({ ...DEFAULT_SKILL_ALLOCATION });
  const [skillDialogOpen, setSkillDialogOpen] = useState(false);
  // Freshly recomputed derived stats from the "Recompute" menu action —
  // null until the operator explicitly triggers a recompute, so display
  // stays byte-for-byte the pre-existing `snapshot`-derived values until
  // then (PF1 behavior unchanged, per `cycles/3_5.md`'s GREEN text).
  const [recomputed, setRecomputed] = useState<RecomputedCharacterSnapshotDto | null>(null);
  const [recomputing, setRecomputing] = useState(false);
  const [cloning, setCloning] = useState(false);
  const [statusMessage, setStatusMessage] = useState<string | null>(null);

  /**
   * Persists the level-up via `level_up_character`'s extended v0.6 request
   * shape (class-level increment plus `additionalChoices`). Records the new
   * class level's hit-die roll as `hp:average` — deterministic and
   * consistent with `maxHitPoints`'s own average-based math above, rather
   * than fabricating a dice-roll UI for a choice the compute engine doesn't
   * yet consume differently either way (backend's own test: "nothing reads
   * these choice_set_ids as a gate"). Skill points stay on the existing,
   * already-wired "Manage skill allocation" dialog rather than duplicating
   * that UI here — `skillAllocations` is deliberately omitted so a level-up
   * never overwrites an allocation the player set separately.
   *
   * v0.6 alpha swarm, item 23: a feat-gaining level (the universal odd-level
   * feat, Fighter's bonus combat feat, Wizard's periodic bonus feat — see
   * `levelGrantsFeat`) no longer levels up immediately. It instead defers to
   * `pendingFeatLevelUp` and opens the same feat `ItemPickerModal` the
   * Feats-tab "Add Feat" affordance already uses (real `list_feats` catalog,
   * not a second bespoke picker) — `handleLevelUpFeatPick` below persists
   * both the level-up and the real feat grant once the user picks one. A
   * level that grants no feat still goes straight through, unchanged.
   */
  async function handleLevelUpAccept(classId: string) {
    setMutationError(null);
    const preview = previewLevelUp(heldClasses, classId);
    // The feat gate now reads the engine's real grants for this exact
    // transition alongside the universal odd-level feat, instead of a
    // hand-authored class-feature table. A failed preview is treated as
    // "no engine-reported grants" rather than blocking the level-up: the
    // universal rule still applies and is decided locally.
    let engineGrantNames: string[] = [];
    try {
      const plan = await previewLevelUpGrants({ characterId: props.row.characterId, classId });
      engineGrantNames = plan.automaticFeatures
        .map((grant) => grant.name)
        .concat(plan.pickFromLists.filter((list) => list.category === 'Feat').map(() => 'feat'));
    } catch {
      engineGrantNames = [];
    }
    if (levelGrantsFeat(preview.features, engineGrantNames)) {
      setPendingFeatLevelUp({ classId, newClassLevel: preview.classLevel });
      setItemPickerOpen('feat');
      return;
    }
    try {
      const outcome = await levelUpCharacter({
        characterId: props.row.characterId,
        classId,
        additionalChoices: [
          { choiceSetId: `choice:level_${preview.classLevel}_hit_points`, selectionId: 'hp:average' },
        ],
        savedAt: new Date().toISOString(),
      });
      const refresh = toCharacterMutationRefresh(
        outcome,
        props.detail?.selectedFeats ?? [],
        props.detail?.spellsSelected ?? []
      );
      if (refresh.kind === 'blocked') {
        setMutationError(refresh.message);
        return;
      }
      props.onDetailRefreshed(refresh.detail);
      // A level-up moves nearly every class-feature magnitude, and the
      // level-up response carries none of the engine's records.
      await refreshEngineRecords();
    } catch (cause: unknown) {
      setMutationError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  /**
   * Completes a feat-gaining level-up once the user picks a feat from the
   * reused picker (see `handleLevelUpAccept`). Two real, sequential
   * mutations, not one fabricated atomic call — no backend command grants a
   * feat as part of `level_up_character` itself. The level-up's own
   * `additionalChoices` only ever lands in the inert `chosen.selected_choices`
   * provenance bag (nothing reads it as a gate or a grant) — and unlike the
   * hit-die record's fixed `hp:average` selection id, a real feat catalog
   * key (e.g. `"Cleave"`) has no colon segments, so it fails
   * `local_store.rs`'s "at least two colon-segments to round-trip through
   * the fixture grammar" persistence check (confirmed live: the backend
   * rejected the level-up outright with exactly that message). So the feat
   * pick is deliberately NOT also recorded via `additionalChoices` — only
   * the hit-die choice is, same as the no-feat path — and the real grant
   * comes solely from calling the exact same `addFeatSelection` the
   * Feats-tab picker uses, into `chosen.selected_feats`. Refreshes after the
   * level-up succeeds, before the feat call even starts, so the sheet is
   * never stale relative to the already-persisted level increment if the
   * follow-on feat grant fails — that failure is surfaced as its own honest
   * error rather than silently dropped or rolled back (there is nothing to
   * roll back; the level-up already happened).
   */
  async function handleLevelUpFeatPick(entry: ItemPickerEntry) {
    const pending = pendingFeatLevelUp;
    setPendingFeatLevelUp(null);
    if (!pending) {
      return;
    }
    setMutationError(null);
    try {
      const outcome = await levelUpCharacter({
        characterId: props.row.characterId,
        classId: pending.classId,
        additionalChoices: [
          { choiceSetId: `choice:level_${pending.newClassLevel}_hit_points`, selectionId: 'hp:average' },
        ],
        savedAt: new Date().toISOString(),
      });
      const refresh = toCharacterMutationRefresh(
        outcome,
        props.detail?.selectedFeats ?? [],
        props.detail?.spellsSelected ?? []
      );
      if (refresh.kind === 'blocked') {
        setMutationError(refresh.message);
        return;
      }
      props.onDetailRefreshed(refresh.detail);

      const featOutcome = await addFeatSelection({
        characterId: props.row.characterId,
        featId: entry.key,
        savedAt: new Date().toISOString(),
      });
      const featRefresh = toCharacterMutationRefresh(
        featOutcome,
        [...(props.detail?.selectedFeats ?? []), entry.key],
        refresh.detail.spellsSelected
      );
      if (featRefresh.kind === 'blocked') {
        setMutationError(`Leveled up, but the picked feat could not be added: ${featRefresh.message}`);
        return;
      }
      props.onDetailRefreshed(featRefresh.detail);
      // Same as handleAddFeat — the feat grant can change maxHp, and
      // neither this call's response nor the level-up's own carries a
      // durability field. Nor the engine's own records.
      await refreshDurability();
      await refreshEngineRecords();
    } catch (cause: unknown) {
      setMutationError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  /**
   * Routes through `purchase_equipment` (v0.6 alpha swarm, risks-and-
   * open-questions.md item 9), not the plain `addEquipmentSelection` —
   * the Add Weapon/Add Armor pickers only ever offer real catalog items
   * with a real gold cost, so every pick here should atomically charge
   * that cost against the persisted money balance rather than being a
   * free grant. A `Blocked` purchase (insufficient funds, or an item with
   * no known `cost_gp`) leaves both the equipment and the money balance
   * untouched, same honest-Blocked invariant as every other mutation.
   */
  async function handleAddEquipment(entry: ItemPickerEntry) {
    setMutationError(null);
    try {
      // The user picking an item from the catalog to add to their loadout
      // is, by construction, actively equipping it — `EquippedActive` is
      // the only choice that matches that action without asking the user
      // to make an extra decision the picker's scope doesn't cover.
      const outcome = await purchaseEquipment({
        characterId: props.row.characterId,
        itemId: entry.key,
        activeState: 'EquippedActive',
        savedAt: new Date().toISOString(),
      });
      if (outcome.kind === 'Blocked') {
        setMutationError(blockedMessageFromDiagnostics(outcome.diagnostics));
        return;
      }
      props.onDetailRefreshed({
        summary: outcome.summary,
        snapshot: outcome.snapshot,
        diagnostics: [],
        corpusDerived: outcome.corpusDerived,
        selectedFeats: props.detail?.selectedFeats ?? [],
        spellsSelected: props.detail?.spellsSelected ?? [],
        chosenFeatTargets: props.detail?.chosenFeatTargets ?? [],
        // `purchase_equipment` returns no engine records; the Weapons tab
        // reads them from `engineRecords`, re-read immediately below.
        explanations: [],
        weaponDamage: [],
      });
      setMoney(outcome.money);
      // Buying a weapon is exactly what makes a new Weapons row appear.
      await refreshEngineRecords();
    } catch (cause: unknown) {
      setMutationError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  /** Opens the reused Equipmods `ItemPickerModal`, remembering which equipped selection the pick will attach to. */
  function handleAttachModifier(item: ResolvedEquipmentDto) {
    setPendingModifierAttachment(item);
    setItemPickerOpen('modifier');
  }

  /**
   * `attach_equipment_modifier` mirrors `purchase_equipment`'s atomic
   * resolve-cost/charge sequencing, so the refresh shape here matches
   * `handleAddEquipment`'s exactly — the one difference is `itemId` names
   * the *target* selection to attach to (from `pendingModifierAttachment`),
   * not a new top-level selection.
   */
  async function handleModifierPicked(entry: ItemPickerEntry) {
    const target = pendingModifierAttachment;
    setPendingModifierAttachment(null);
    if (!target) {
      return;
    }
    setMutationError(null);
    try {
      const outcome = await attachEquipmentModifier({
        characterId: props.row.characterId,
        itemId: target.itemId,
        modifierItemId: entry.key,
        savedAt: new Date().toISOString(),
      });
      if (outcome.kind === 'Blocked') {
        setMutationError(blockedMessageFromDiagnostics(outcome.diagnostics));
        return;
      }
      props.onDetailRefreshed({
        summary: outcome.summary,
        snapshot: outcome.snapshot,
        diagnostics: [],
        corpusDerived: outcome.corpusDerived,
        selectedFeats: props.detail?.selectedFeats ?? [],
        spellsSelected: props.detail?.spellsSelected ?? [],
        chosenFeatTargets: props.detail?.chosenFeatTargets ?? [],
        // See `handleAddEquipment` — no engine records on this response.
        explanations: [],
        weaponDamage: [],
      });
      setMoney(outcome.money);
      // Attaching a +1 enhancement changes that weapon's Enh. columns.
      await refreshEngineRecords();
    } catch (cause: unknown) {
      setMutationError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  async function handleAddSpell(entry: ItemPickerEntry) {
    setMutationError(null);
    // `add_spell_selection`/`record_and_prepare_spell_selection` require a
    // `sourceClassId`; the picker's scope is "search, filter, pick a spell"
    // (no class chooser) — `resolveSpellRouting` decides both which held
    // class to attribute the pick to (preferring Wizard over
    // `heldClasses[0]`, so a Fighter/Wizard multiclass build doesn't
    // misattribute the pick to Fighter) and which command that resolves to.
    const existingSpells = props.detail?.spellsSelected ?? [];
    const routing = resolveSpellRouting(heldClasses, existingSpells, WIZARD_CLASS_ID);
    if (!routing) {
      setMutationError('This character has no class to learn the spell from yet.');
      return;
    }
    const { primaryClassId, useAtomicBootstrap } = routing;
    try {
      // Wizard's `unmet_wizard_spellbook_conditions` requires a non-empty
      // Known set AND a non-empty Prepared set simultaneously — no sequence
      // of single-mode `add_spell_selection` calls can ever satisfy that
      // from zero (each call is independently gated on reaching `Computed`
      // before persisting, and neither mode alone gets there), so only the
      // genuine bootstrap — this character's first-ever Wizard spell —
      // needs the atomic record-and-prepare command. Once that first spell
      // exists, both sets are already non-empty, so a plain
      // `add_spell_selection` (Known) is enough to keep reaching `Computed`
      // for every spell after (risks-and-open-questions.md item 9a) — the
      // gate only checks the sets are non-empty overall, not that this
      // specific spell is in both, and the existing SpellsTab doesn't model
      // Known-vs-Prepared posture for any class either, so this isn't a
      // visible behavior change from always taking the atomic path.
      const outcome = useAtomicBootstrap
        ? await recordAndPrepareSpellSelection({
            characterId: props.row.characterId,
            spellId: entry.key,
            sourceClassId: primaryClassId,
            savedAt: new Date().toISOString(),
          })
        : await addSpellSelection({
            characterId: props.row.characterId,
            spellId: entry.key,
            sourceClassId: primaryClassId,
            // "Known" is the closest default to "the character now has
            // access to this spell" without picking a prepared-caster's
            // daily list — out of scope for a search-and-select picker.
            acquisitionMode: 'Known',
            savedAt: new Date().toISOString(),
          });
      // Mirrors exactly what the mutation itself appended — never
      // fabricated. The bootstrap path appends both a Known and a Prepared
      // entry for the same spell in one call; every other path appends one.
      const newSpells: SpellSelectionDto[] = useAtomicBootstrap
        ? [
            { spellId: entry.key, sourceClassId: primaryClassId, acquisitionMode: 'Known' },
            { spellId: entry.key, sourceClassId: primaryClassId, acquisitionMode: 'Prepared' },
          ]
        : [{ spellId: entry.key, sourceClassId: primaryClassId, acquisitionMode: 'Known' }];
      const refresh = toCharacterMutationRefresh(outcome, props.detail?.selectedFeats ?? [], [
        ...existingSpells,
        ...newSpells,
      ]);
      if (refresh.kind === 'blocked') {
        setMutationError(refresh.message);
        return;
      }
      props.onDetailRefreshed(refresh.detail);
      // A spell add changes the caster's own `class_spell.*` records.
      await refreshEngineRecords();
    } catch (cause: unknown) {
      setMutationError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  function handleAddFeat(entry: ItemPickerEntry) {
    setMutationError(null);
    // A chooser feat is meaningless without its target -- Weapon Focus with
    // no weapon grounds nothing -- so route to the second picker step
    // instead of silently saving a feat that computes nothing.
    if (entry.chooserTargetKind) {
      setPendingFeatTarget({
        featKey: entry.key,
        featName: entry.name,
        targetKind: entry.chooserTargetKind,
      });
      setItemPickerOpen('featTarget');
      return;
    }
    void commitFeatSelection(entry.key, null, null);
  }

  /** Second step: the target was picked, so save feat + target together. */
  function handleFeatTargetPicked(entry: ItemPickerEntry) {
    const pending = pendingFeatTarget;
    if (!pending) {
      return;
    }
    setPendingFeatTarget(null);
    void commitFeatSelection(pending.featKey, entry.key, pending.targetKind);
  }

  async function commitFeatSelection(
    featId: string,
    target: string | null,
    targetKind: string | null
  ) {
    setMutationError(null);
    try {
      const outcome = await addFeatSelection({
        characterId: props.row.characterId,
        featId,
        target,
        savedAt: new Date().toISOString(),
      });
      // The feat was just appended to chosen.selected_feats by this exact
      // mutation, so appending it here mirrors the real backend change —
      // not fabricated, the same append `add_feat_selection` itself made.
      const refresh = toCharacterMutationRefresh(
        outcome,
        [...(props.detail?.selectedFeats ?? []), featId],
        props.detail?.spellsSelected ?? [],
        // Mirrors exactly what the backend just recorded: the target is
        // appended to this feat's existing entry, or a new entry is added.
        // Nothing is invented -- a null target adds no target.
        mergeChosenFeatTarget(
          props.detail?.chosenFeatTargets ?? [],
          featId,
          target,
          targetKind
        )
      );
      if (refresh.kind === 'blocked') {
        setMutationError(refresh.message);
        return;
      }
      props.onDetailRefreshed(refresh.detail);
      // A feat can change maxHp (e.g. Toughness) but add_feat_selection's
      // response carries no durability field — refresh it explicitly so
      // the Defense tab doesn't show a stale pre-feat value.
      await refreshDurability();
      // A feat can also change a weapon's damage (Weapon Specialization)
      // and gate class-feature records.
      await refreshEngineRecords();
    } catch (cause: unknown) {
      setMutationError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  /**
   * SD-25 Criterion 3.5 register A3: the real UI affordance wired to one of
   * `append_to_character` / `recompute_character` / `re_save_character`
   * (matching SD-24 Criterion 7.4's Add-Weapon/Add-Armor/Add-Spell
   * precedent — a menu action that calls a real boundary wrapper's
   * `invoke()`, not refactored-but-unused plumbing). Routes through the
   * panel's active adapter via `buildRecomputeCharacterRequest`/
   * `resolveRuleSystemId` rather than hardcoding `"pf1"` — the seam
   * criterion 3.5's own RED/GREEN targets. `recompute_character` never
   * mutates the on-disk character, so this is safe to call as often as the
   * operator likes; it simply pulls fresh derived stats straight from the
   * real compute engine instead of whatever `snapshot` was loaded with.
   */
  async function handleRecompute() {
    setMutationError(null);
    setStatusMessage(null);
    setRecomputing(true);
    try {
      const request = buildRecomputeCharacterRequest(props.row.characterId, props.ruleSet);
      const response = await recomputeCharacter(request);
      if (!response.success || !response.character) {
        setMutationError(response.error ?? 'Recompute did not return a usable result.');
        return;
      }
      setRecomputed(response.character);
      setStatusMessage('Recomputed derived stats from the current on-disk build.');
    } catch (cause: unknown) {
      setMutationError(cause instanceof Error ? cause.message : String(cause));
    } finally {
      setRecomputing(false);
    }
  }

  /** Top-menu "Clone": duplicates the currently open character under a new id, mirroring `LoadCharacterScreen.tsx`'s own `handleClone`. */
  async function handleClone() {
    setMutationError(null);
    setStatusMessage(null);
    setCloning(true);
    try {
      const outcome = await cloneCharacter({
        characterId: props.row.characterId,
        newCharacterId: crypto.randomUUID(),
        newDisplayLabel: `${props.row.displayLabel} (Copy)`,
        savedAt: new Date().toISOString(),
      });
      if (outcome.kind === 'Saved') {
        setStatusMessage(`Cloned as "${outcome.summary.displayLabel}".`);
        props.onCloned();
      } else {
        setMutationError('Clone failed: the copy no longer computes cleanly, so nothing was saved.');
      }
    } catch (cause: unknown) {
      setMutationError(cause instanceof Error ? cause.message : 'Could not clone the character.');
    } finally {
      setCloning(false);
    }
  }

  function updateBio(patch: Partial<BioFields>) {
    setBio((prev) => ({ ...prev, ...patch }));
  }

  /**
   * Persists the current bio field set via `update_character_bio` on field
   * blur (not per keystroke — bio has 8 free-text/select fields and firing
   * a Tauri round trip on every character typed would be wasteful). Pure
   * passthrough persistence with no rules-engine involvement, so a failure
   * here is a real I/O problem, not a Blocked/diagnostics outcome — surfaced
   * through the same `mutationError` banner as every other mutation.
   */
  async function handleBioBlur() {
    try {
      await updateCharacterBio(props.row.characterId, bio);
    } catch (cause: unknown) {
      setMutationError(cause instanceof Error ? cause.message : 'Could not save the bio fields.');
    }
  }

  /**
   * Applies a gp-denominated add/spend to the persisted money balance via
   * `adjust_character_money`. `gpAmount` is positive to add funds, negative
   * to spend — converted to the wire's canonical copper delta via
   * `gpToCopper`. A spend that would drive the balance negative comes back
   * as a real error from the backend, surfaced here rather than assumed to
   * always succeed (there is no Blocked/diagnostics concept for this pure
   * passthrough command — just success or a real error, same as bio).
   */
  async function handleAdjustMoney(gpAmount: number) {
    setMoneyError(null);
    setMoneyBusy(true);
    try {
      const updated = await adjustCharacterMoney(props.row.characterId, gpToCopper(gpAmount));
      setMoney(updated);
    } catch (cause: unknown) {
      setMoneyError(cause instanceof Error ? cause.message : 'Could not update the money balance.');
    } finally {
      setMoneyBusy(false);
    }
  }

  /**
   * Re-fetches HP/durability after a mutation that can change `maxHp` (a
   * feat grant, e.g. Toughness's +3 HP) but whose own response carries no
   * durability field — unlike `adjustCharacterHp`'s. Without this, the
   * Defense tab would keep showing the pre-feat `maxHp` until the sheet
   * was closed and reopened (QA found this verifying item 17 — same
   * render-staleness shape as the `corpus_derived` bug and the Load-list
   * staleness item 26 already fixed). Failure falls back to `null`, the
   * same honest "not available" treatment the initial per-character load
   * uses — never a stale or fabricated value.
   */
  async function refreshDurability() {
    try {
      const loaded = await loadCharacterDurability(props.row.characterId);
      setDurability(loaded);
    } catch {
      setDurability(null);
    }
  }

  /**
   * Re-reads the engine's computed records (class-feature explanations and
   * the per-weapon damage breakdown) after a mutation.
   *
   * The mutation commands return a `CreateCharacterResponse`, which carries
   * neither — only `load_saved_character` does. Carrying the pre-mutation
   * records forward instead would be worse than re-reading: a level-up
   * moves nearly every class-feature magnitude, so the sheet would show
   * last level's numbers as if they were current. Same post-mutation
   * re-read shape `refreshDurability` above already uses, for the same
   * render-staleness reason.
   *
   * A failure leaves whatever is already on screen rather than blanking
   * the section — the browser preview (no Tauri runtime) takes this path
   * on mount and keeps its seeded sample records.
   */
  async function refreshEngineRecords() {
    try {
      const loaded = await loadSavedCharacterDetail({ characterId: props.row.characterId });
      setEngineRecords({ explanations: loaded.explanations, weaponDamage: loaded.weaponDamage });
    } catch {
      // Intentionally keeps the current records.
    }
  }

  /**
   * `deltaHp`/`deltaNonlethal` are already the exact wire deltas (positive
   * to heal / take nonlethal damage is the caller's choice, see
   * `adjustCharacterHp`'s own doc comment) — one atomic round trip, no
   * read-then-write coordination needed on this side.
   */
  async function handleAdjustHp(deltaHp: number, deltaNonlethal: number) {
    setDurabilityError(null);
    setDurabilityBusy(true);
    try {
      const updated = await adjustCharacterHp(props.row.characterId, deltaHp, deltaNonlethal);
      setDurability(updated);
    } catch (cause: unknown) {
      setDurabilityError(cause instanceof Error ? cause.message : 'Could not update HP.');
    } finally {
      setDurabilityBusy(false);
    }
  }

  /**
   * Persists the dialog's full draft allocation via `set_skill_allocations`
   * (a wholesale replace, not a delta — see `setSkillAllocations`'s doc
   * comment). On `Blocked` the on-disk character (and this panel's
   * `skillAllocation` state) is left exactly as it was — the compute
   * engine's `Computed` path only accepts one exact hardcoded posture
   * today (Climb/Intimidate/Swim at rank 1, chain shirt equipped; see
   * `pilot_compute.rs`), so most allocations legitimately come back
   * blocked with real diagnostics rather than silently applying.
   */
  async function handleSkillAllocationAccept(draft: Record<string, number>) {
    setMutationError(null);
    try {
      const outcome = await setSkillAllocations({
        characterId: props.row.characterId,
        skillAllocations: Object.entries(draft)
          .filter(([, ranks]) => ranks > 0)
          .map(([skillName, ranks]) => ({ skillId: skillIdFor(skillName), ranks })),
        savedAt: new Date().toISOString(),
      });
      const refresh = toCharacterMutationRefresh(
        outcome,
        props.detail?.selectedFeats ?? [],
        props.detail?.spellsSelected ?? []
      );
      if (refresh.kind === 'blocked') {
        setMutationError(refresh.message);
        return;
      }
      setSkillAllocation(draft);
      props.onDetailRefreshed(refresh.detail);
      await refreshEngineRecords();
    } catch (cause: unknown) {
      setMutationError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  const snapshot = props.detail?.snapshot ?? null;
  const abilities = snapshot?.abilityModifiers ?? ZERO_ABILITIES;
  // `recomputed` (set by the "Recompute" menu action) takes precedence over
  // the originally loaded `snapshot` when present — it is always at least
  // as fresh, since it comes from the same real compute engine reading
  // whatever is currently on disk. Ability modifiers/skills aren't part of
  // `recompute_character`'s response surface, so those keep reading from
  // `snapshot` regardless.
  const ac = recomputed?.baselineArmorClass ?? snapshot?.baselineArmorClass ?? 10;
  const saves = recomputed?.totalSaves ?? snapshot?.totalSaves ?? { fortitude: 0, reflex: 0, will: 0 };
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

  // A race this build carries no profile for reports `Unknown` for both,
  // rather than the old `?? 'Medium'` / `?? 'Normal'` defaults — the panel
  // captions these as calculated from race, so a guess reads as a rules
  // fact. See `deriveRaceTraits`.
  const { size, vision } = deriveRaceTraits(props.detail?.summary.raceId);
  const baseAttackBonus = recomputed?.baseAttackBonus ?? snapshot?.baseAttackBonus ?? 0;
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

  // SD-25 Criterion 3.5 register A4: `Open`/`Save`/`Clone` were bare
  // `() => {}` no-op handlers — a no-stub-doctrine violation on a
  // user-facing affordance. `Open` and `Clone` are wired to real behavior
  // below. `Save` is replaced with `Recompute`: every mutation this sheet
  // can trigger (level-up, add-equipment, add-spell, clone) already
  // persists immediately on selection — there is no session-local "unsaved
  // edit" state for an explicit Save to commit (the Bio fields are the one
  // exception, and they have no persisted schema slot to save into yet; see
  // `DetailsPanel`'s own doc comment). Labeling a real action "Save" when it
  // does not persist anything would itself be dishonest UI, so this cycle
  // wires the menu to the real capability that IS available from the panel
  // today — `recompute_character` (register A3) — rather than fabricating a
  // Save that has nothing new to write.
  const menuItems: ReadonlyArray<{ label: string; onSelect: () => void; dividerBefore?: boolean }> = [
    { label: 'Open', onSelect: props.onOpen },
    { label: recomputing ? 'Recomputing…' : 'Recompute', onSelect: () => void handleRecompute() },
    { label: cloning ? 'Cloning…' : 'Clone', onSelect: () => void handleClone() },
    { label: 'Print', onSelect: () => window.print() },
  ];

  // One generic `ItemPickerModal` backs all four "Add …" affordances — see
  // `buildItemPickerConfig`'s doc comment for the dispatch shape (title /
  // corpus query / mutate-handler per `itemPickerOpen` kind). When a
  // feat-gaining level-up is waiting on a pick (`pendingFeatLevelUp`), the
  // same 'feat' picker instance routes to `handleLevelUpFeatPick` instead of
  // the plain `handleAddFeat` — same catalog and UI, different mutation.
  const itemPickerConfig = buildItemPickerConfig(itemPickerOpen, {
    loadEquipment: (category) =>
      listEquipment({ nameContains: null, category }).then((response) => mapEquipmentCatalogEntries(response.entries)),
    loadSpells: () => listSpells({ nameContains: null, school: null }).then((response) => mapSpellCatalogEntries(response.entries)),
    loadFeats: () => listFeats({ nameContains: null, category: null }).then((response) => mapFeatCatalogEntries(response.entries)),
    loadFeatTargets: () => {
      const kind = pendingFeatTarget?.targetKind ?? null;
      if (kind === 'Weapon') {
        return listWeaponTargets().then(weaponTargetOptions);
      }
      if (kind === 'Skill') {
        return Promise.resolve(skillTargetOptions());
      }
      if (kind === 'SpellSchool') {
        return Promise.resolve(spellSchoolTargetOptions());
      }
      return Promise.resolve([]);
    },
    onSelectEquipment: handleAddEquipment,
    onSelectSpell: handleAddSpell,
    onSelectFeat: pendingFeatLevelUp ? (entry) => void handleLevelUpFeatPick(entry) : handleAddFeat,
    onSelectFeatTarget: handleFeatTargetPicked,
    onSelectModifier: (entry) => void handleModifierPicked(entry),
  });
  const itemPickerTitle = pendingFeatTarget
    ? featTargetPickerTitle(pendingFeatTarget.featName, pendingFeatTarget.targetKind)
    : pendingFeatLevelUp
    ? `Pick a feat — level ${pendingFeatLevelUp.newClassLevel}`
    : pendingModifierAttachment
      ? `Attach Modifier — ${pendingModifierAttachment.equipmentRecordName}`
      : itemPickerConfig?.title ?? '';

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

              {statusMessage ? (
                <p
                  role="status"
                  style={{
                    backgroundColor: 'var(--color-surface-2)',
                    border: '1px solid var(--color-border)',
                    borderRadius: 6,
                    color: 'var(--color-text-secondary)',
                    fontSize: '0.75rem',
                    margin: '0 0 0.5rem',
                    padding: '0.4rem 0.55rem',
                  }}
                >
                  {statusMessage}
                </p>
              ) : null}

              <LevelUpDialog
                open={levelUpOpen}
                onClose={() => setLevelUpOpen(false)}
                characterId={props.row.characterId}
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
                <WeaponsTab
                  proficiency={weaponProficiency}
                  weaponDamage={engineRecords.weaponDamage}
                  corpusDerived={props.detail?.corpusDerived ?? null}
                  onAddWeapon={() => setItemPickerOpen('weapon')}
                />
              ) : tab === 'Defense' ? (
                <DefenseTab
                  damageReduction={snapshot?.damageReduction}
                  equipmentEffects={props.detail?.corpusDerived?.equipmentEffects}
                  encumbrance={props.detail?.corpusDerived?.encumbrance}
                  durability={durability}
                  durabilityBusy={durabilityBusy}
                  durabilityError={durabilityError}
                  onAdjustHp={(deltaHp) => void handleAdjustHp(deltaHp, 0)}
                />
              ) : tab === 'Spells' ? (
                <SpellsTab
                  spellsSelected={props.detail?.spellsSelected ?? []}
                  corpusDerived={props.detail?.corpusDerived}
                  explanations={engineRecords.explanations}
                  onAddSpell={() => setItemPickerOpen('spell')}
                />
              ) : tab === 'Gear' ? (
                <GearTab
                  corpusDerived={props.detail?.corpusDerived}
                  onAddArmor={() => setItemPickerOpen('armor')}
                  onAttachModifier={handleAttachModifier}
                  money={money}
                  moneyBusy={moneyBusy}
                  moneyError={moneyError}
                  onAdjustMoney={(gpAmount) => void handleAdjustMoney(gpAmount)}
                />
              ) : tab === 'Feats' ? (
                <FeatsTab
            selectedFeats={props.detail?.selectedFeats ?? []}
            chosenFeatTargets={props.detail?.chosenFeatTargets ?? []}
            onAddFeat={() => setItemPickerOpen('feat')}
          />
              ) : tab === 'Pets' ? (
                <PetsTab snapshot={snapshot} />
              ) : tab === 'Actions' ? (
                <ActionsTab
                  levelEntries={currentBenefits}
                  explanations={engineRecords.explanations}
                  heldClasses={heldClasses}
                />
              ) : (
                <p style={{ color: 'var(--color-text-faint)', margin: 0, textAlign: 'center' }}>{tab} — coming soon.</p>
              )}
            </div>
          </div>
        </div>

        {/* RIGHT: character details, then skills beneath */}
        <div style={{ flex: '0 0 300px', minWidth: 0 }}>
          <DetailsPanel vision={vision} size={size} bio={bio} onBioChange={updateBio} onBioBlur={() => void handleBioBlur()} />
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
        onAccept={(draft) => void handleSkillAllocationAccept(draft)}
      />

      <ItemPickerModal
        open={itemPickerConfig !== null}
        title={itemPickerTitle}
        searchPlaceholder={itemPickerConfig?.searchPlaceholder ?? ''}
        loadEntries={itemPickerConfig?.loadEntries ?? (() => Promise.resolve([]))}
        onClose={() => {
          setItemPickerOpen(null);
          setPendingFeatLevelUp(null);
          setPendingModifierAttachment(null);
        }}
        onSelect={(entry) => itemPickerConfig?.onSelect(entry)}
      />
    </div>
  );
}
