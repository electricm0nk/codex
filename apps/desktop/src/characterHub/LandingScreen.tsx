import type { CSSProperties } from 'react';
import type { CampaignManagerAccessGate } from '../campaign/campaignManagerAccessGate';

/**
 * Rule-set landing screen (Pathbuilder-style): a rule-set selector followed by
 * "New character" / "Load character" banners whose artwork changes per rule
 * set. Only Pathfinder 1e is currently selectable; the other rule sets are
 * present as unselectable stubs until their rules land.
 */

export type RuleSetId = 'pathfinder-1e' | 'solarus-arcanum' | 'traveller' | 'cyberpunk' | 'starfinder-1e' | 'world-of-darkness';

export interface RuleSet {
  id: RuleSetId;
  name: string;
  tagline: string;
  available: boolean;
  /** Overrides the auto-generated (first-letter-per-word) badge initials when that would read poorly, e.g. "World of Darkness" -> "WoD" instead of "WO". */
  abbr?: string;
  /** Banner artwork per action. Gradients today; drop in illustration URLs later. */
  art: { create: string; load: string };
}

// Amber/Wasp-toned gradient placeholders standing in for per-rule-set artwork.
// Exported so other screens (e.g. campaign creation's rule-set picker) share
// one catalogue instead of maintaining their own copy of the list.
export const RULE_SETS: readonly RuleSet[] = [
  {
    id: 'pathfinder-1e',
    name: 'Pathfinder 1e',
    tagline: 'Fantasy d20',
    available: true,
    art: {
      create: 'linear-gradient(115deg, #3a2a08 0%, #6b4e10 45%, #f8c537 130%)',
      load: 'linear-gradient(115deg, #241a05 0%, #4a3608 50%, #c88a1e 135%)',
    },
  },
  {
    id: 'solarus-arcanum',
    name: 'Solarus Arcanum',
    tagline: 'Arcane sci-fantasy',
    available: false,
    art: {
      create: 'linear-gradient(115deg, #241634 0%, #4a2e6b 60%, #a06bd4 130%)',
      load: 'linear-gradient(115deg, #180f24 0%, #382050 60%, #7c4ea0 130%)',
    },
  },
  {
    id: 'traveller',
    name: 'Traveller MGT 2e',
    tagline: 'Hard sci-fi',
    available: false,
    art: {
      create: 'linear-gradient(115deg, #06202a 0%, #0e4a5c 60%, #2aa6c8 130%)',
      load: 'linear-gradient(115deg, #04141a 0%, #0a3642 60%, #1e7c96 130%)',
    },
  },
  {
    id: 'cyberpunk',
    name: 'Cyberpunk Red',
    tagline: 'Neon dystopia',
    available: false,
    art: {
      create: 'linear-gradient(115deg, #2a0820 0%, #6b1050 55%, #f838a0 130%)',
      load: 'linear-gradient(115deg, #1a0514 0%, #500a38 55%, #c81e7c 130%)',
    },
  },
  {
    id: 'starfinder-1e',
    name: 'Starfinder 1e',
    tagline: 'Sci-fi fantasy',
    available: false,
    art: {
      create: 'linear-gradient(115deg, #062a1a 0%, #0e5c3a 55%, #2ad68a 130%)',
      load: 'linear-gradient(115deg, #041a10 0%, #0a4228 55%, #1ea868 135%)',
    },
  },
  {
    id: 'world-of-darkness',
    name: 'World of Darkness',
    tagline: 'Gothic horror',
    available: false,
    abbr: 'WoD',
    art: {
      create: 'linear-gradient(115deg, #240808 0%, #5c1414 55%, #c8283a 130%)',
      load: 'linear-gradient(115deg, #180505 0%, #420e0e 55%, #96202c 135%)',
    },
  },
];

function RuleSetChip(props: { ruleSet: RuleSet; selected: boolean; onSelect: () => void }) {
  const { ruleSet, selected } = props;
  const disabled = !ruleSet.available;

  const base: CSSProperties = {
    alignItems: 'center',
    border: `1px solid ${selected ? 'var(--color-accent)' : 'var(--color-border)'}`,
    borderRadius: 999,
    cursor: disabled ? 'not-allowed' : 'pointer',
    display: 'flex',
    gap: '0.6rem',
    opacity: disabled ? 0.45 : 1,
    padding: '0.5rem 1rem 0.5rem 0.5rem',
    textAlign: 'left',
  };
  const themed: CSSProperties = selected
    ? { backgroundColor: 'var(--color-accent)', color: 'var(--color-on-accent)' }
    : { backgroundColor: 'var(--color-surface-2)', color: 'var(--color-text-secondary)' };

  const initials = (
    ruleSet.abbr ??
    ruleSet.name
      .split(' ')
      .map((word) => word[0])
      .join('')
      .slice(0, 2)
  ).toUpperCase();

  return (
    <button
      type="button"
      onClick={disabled ? undefined : props.onSelect}
      disabled={disabled}
      title={disabled ? `${ruleSet.name} — coming soon` : ruleSet.name}
      style={{ ...base, ...themed }}
    >
      <span
        aria-hidden
        style={{
          alignItems: 'center',
          backgroundColor: selected ? 'var(--color-on-accent)' : 'var(--color-surface)',
          borderRadius: '50%',
          color: selected ? 'var(--color-accent)' : 'var(--color-text-muted)',
          display: 'flex',
          flexShrink: 0,
          fontSize: '0.75rem',
          fontWeight: 800,
          height: 34,
          justifyContent: 'center',
          width: 34,
        }}
      >
        {initials}
      </span>
      <span style={{ display: 'flex', flexDirection: 'column', lineHeight: 1.15 }}>
        <span style={{ fontSize: '0.95rem', fontWeight: 800, letterSpacing: '0.02em' }}>{ruleSet.name}</span>
        <span style={{ fontSize: '0.72rem', opacity: 0.85 }}>{disabled ? 'Coming soon' : ruleSet.tagline}</span>
      </span>
    </button>
  );
}

function ActionBanner(props: { title: string; art: string; onClick: () => void; disabled?: boolean; disabledHint?: string }) {
  const disabled = Boolean(props.disabled);
  return (
    <button
      type="button"
      className="landing-banner"
      onClick={disabled ? undefined : props.onClick}
      disabled={disabled}
      title={disabled ? props.disabledHint : undefined}
      style={{
        alignItems: 'flex-end',
        backgroundImage: props.art,
        backgroundPosition: 'center',
        backgroundSize: 'cover',
        border: `2px solid ${disabled ? 'var(--color-border)' : 'var(--color-accent)'}`,
        borderRadius: 14,
        cursor: disabled ? 'not-allowed' : 'pointer',
        display: 'flex',
        filter: disabled ? 'grayscale(0.8)' : 'none',
        justifyContent: 'flex-end',
        minHeight: 160,
        opacity: disabled ? 0.55 : 1,
        overflow: 'hidden',
        padding: '1.1rem 1.4rem',
        position: 'relative',
        width: '100%',
      }}
    >
      {/* Tints the fixed per-rule-set art with the active theme's accent color,
          so switching themes visibly shifts the banner while keeping each
          rule set's distinct gradient identity underneath. */}
      <span
        aria-hidden
        style={{
          backgroundColor: 'var(--color-accent)',
          inset: 0,
          mixBlendMode: 'color',
          opacity: disabled ? 0 : 0.4,
          position: 'absolute',
        }}
      />
      {/* Legibility scrim behind the label. */}
      <span
        aria-hidden
        style={{
          background: 'linear-gradient(90deg, rgba(0,0,0,0) 30%, rgba(0,0,0,0.55) 100%)',
          inset: 0,
          position: 'absolute',
        }}
      />
      <span
        style={{
          color: '#ffffff',
          fontSize: 'clamp(1.5rem, 3.2vw, 2.1rem)',
          fontWeight: 800,
          lineHeight: 1.05,
          position: 'relative',
          textAlign: 'right',
          textShadow: '0 2px 8px rgba(0, 0, 0, 0.6)',
          textTransform: 'uppercase',
          whiteSpace: 'pre-line',
        }}
      >
        {props.title}
      </span>
      {disabled && props.disabledHint ? (
        <span
          style={{
            backgroundColor: 'rgba(0, 0, 0, 0.55)',
            borderRadius: 999,
            bottom: '1.1rem',
            color: '#ffffff',
            fontSize: '0.72rem',
            fontWeight: 700,
            left: '1.4rem',
            padding: '0.3rem 0.7rem',
            position: 'absolute',
            textTransform: 'none',
          }}
        >
          {props.disabledHint}
        </span>
      ) : null}
    </button>
  );
}

const CAMPAIGN_MANAGER_ART = 'linear-gradient(115deg, #0a1428 0%, #1c3466 55%, #4a7ad6 130%)';
const DM_TOOLKIT_ART = 'linear-gradient(115deg, #240818 0%, #5c1040 55%, #c8288a 130%)';

export function LandingScreen(props: {
  selectedRuleSet: RuleSetId;
  onSelectRuleSet: (id: RuleSetId) => void;
  onCreate: () => void;
  onLoad: () => void;
  onBrowseEquipment: () => void;
  onBrowseSpells: () => void;
  onBrowseClasses: () => void;
  onBrowseRaces: () => void;
  onBrowseMonsters: () => void;
  onBrowseCompanions: () => void;
  onBrowseIntelligentItems: () => void;
  onCorpusIngestDiagnostic: () => void;
  onCampaignManager: () => void;
  campaignManagerGate: CampaignManagerAccessGate;
  onDmToolkit: () => void;
}) {
  const active = RULE_SETS.find((rs) => rs.id === props.selectedRuleSet) ?? RULE_SETS[0];

  return (
    <section style={{ display: 'flex', flexDirection: 'column', gap: '1.25rem', marginTop: '1rem' }}>
      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.75rem' }}>
        {RULE_SETS.map((ruleSet) => (
          <RuleSetChip
            key={ruleSet.id}
            ruleSet={ruleSet}
            selected={ruleSet.id === props.selectedRuleSet}
            onSelect={() => props.onSelectRuleSet(ruleSet.id)}
          />
        ))}
      </div>

      <ActionBanner title={'New\nCharacter'} art={active.art.create} onClick={props.onCreate} />
      <ActionBanner title={'Load\nCharacter'} art={active.art.load} onClick={props.onLoad} />
      <ActionBanner
        title={'Campaign\nManager'}
        art={CAMPAIGN_MANAGER_ART}
        onClick={props.onCampaignManager}
        disabled={!props.campaignManagerGate.enabled}
        disabledHint={props.campaignManagerGate.disabledHint}
      />
      <ActionBanner title={'DM\nToolkit'} art={DM_TOOLKIT_ART} onClick={props.onDmToolkit} />

      <div style={{ alignSelf: 'center', display: 'flex', flexWrap: 'wrap', gap: '1.25rem', justifyContent: 'center' }}>
        <button
          type="button"
          onClick={props.onBrowseEquipment}
          style={{
            background: 'none',
            border: 'none',
            color: 'var(--color-text-muted)',
            cursor: 'pointer',
            fontSize: '0.85rem',
            textDecoration: 'underline',
          }}
        >
          Browse Equipment Catalog
        </button>
        <button
          type="button"
          onClick={props.onBrowseSpells}
          style={{
            background: 'none',
            border: 'none',
            color: 'var(--color-text-muted)',
            cursor: 'pointer',
            fontSize: '0.85rem',
            textDecoration: 'underline',
          }}
        >
          Browse Spell Catalog
        </button>
        <button
          type="button"
          onClick={props.onBrowseClasses}
          style={{
            background: 'none',
            border: 'none',
            color: 'var(--color-text-muted)',
            cursor: 'pointer',
            fontSize: '0.85rem',
            textDecoration: 'underline',
          }}
        >
          Browse Class Progression
        </button>
        <button
          type="button"
          onClick={props.onBrowseRaces}
          style={{
            background: 'none',
            border: 'none',
            color: 'var(--color-text-muted)',
            cursor: 'pointer',
            fontSize: '0.85rem',
            textDecoration: 'underline',
          }}
        >
          Browse Race Traits
        </button>
        <button
          type="button"
          onClick={props.onBrowseMonsters}
          style={{
            background: 'none',
            border: 'none',
            color: 'var(--color-text-muted)',
            cursor: 'pointer',
            fontSize: '0.85rem',
            textDecoration: 'underline',
          }}
        >
          Browse Monster Catalog
        </button>
        <button
          type="button"
          onClick={props.onBrowseCompanions}
          style={{
            background: 'none',
            border: 'none',
            color: 'var(--color-text-muted)',
            cursor: 'pointer',
            fontSize: '0.85rem',
            textDecoration: 'underline',
          }}
        >
          Browse Companion Catalog
        </button>
        <button
          type="button"
          onClick={props.onBrowseIntelligentItems}
          style={{
            background: 'none',
            border: 'none',
            color: 'var(--color-text-muted)',
            cursor: 'pointer',
            fontSize: '0.85rem',
            textDecoration: 'underline',
          }}
        >
          Browse Intelligent Item Components
        </button>
        <button
          type="button"
          onClick={props.onCorpusIngestDiagnostic}
          style={{
            background: 'none',
            border: 'none',
            color: 'var(--color-text-muted)',
            cursor: 'pointer',
            fontSize: '0.85rem',
            textDecoration: 'underline',
          }}
        >
          Corpus Ingest Diagnostic
        </button>
      </div>
    </section>
  );
}
