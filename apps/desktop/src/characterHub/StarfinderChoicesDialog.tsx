import { useEffect, useRef, useState, type CSSProperties } from 'react';
import { appendToCharacter } from '../boundary/appendToCharacter';
import {
  listStarfinderEquipmentOptions,
  previewStarfinderChoices,
  saveStarfinderChoices,
  type SfChoicesDto,
  type SfChoicesPreviewDto,
  type SfEquipmentOptionDto,
} from '../boundary/starfinderChoices';
import type { DiagnosticDto } from '../boundary/loadCreateCharacter';
import { StarfinderChoicesPanel } from './StarfinderChoicesPanel';
import { addGear, blockingChoiceProblems, choicesTotalsLine } from './starfinderChoicesModel';

/** The Starfinder rule-system id `append_to_character` routes to `StarfinderAdapter`. */
const STARFINDER_RULE_SYSTEM_ID = 'starfinder-1e';

/**
 * The Starfinder sheet's "Feats, spells and gear" dialog (SD-37 E6.5a). It opens on the saved
 * character's own choices (`preview_starfinder_choices` with none sent) and re-asks the engine on
 * every edit. An item is added through the Starfinder adapter's own append
 * (`append_to_character`, saved at once, refused when it puts the loadout over the character's
 * credits); every other edit is saved by "Save choices" (`save_starfinder_choices`), which
 * recomputes the character and refuses, saving nothing, what the rules do not allow.
 */
export function StarfinderChoicesDialog(props: {
  characterId: string;
  onClose: () => void;
  /** Called with the engine's totals line once the choices are saved. */
  onSaved: (totalsLine: string) => void;
}) {
  const [choices, setChoices] = useState<SfChoicesDto | null>(null);
  const [preview, setPreview] = useState<SfChoicesPreviewDto | null>(null);
  const [equipment, setEquipment] = useState<SfEquipmentOptionDto[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [refused, setRefused] = useState<DiagnosticDto[]>([]);
  const [saving, setSaving] = useState(false);
  const round = useRef(0);

  useEffect(() => {
    listStarfinderEquipmentOptions()
      .then(setEquipment)
      .catch((cause: unknown) => setError(cause instanceof Error ? cause.message : String(cause)));
  }, []);

  const requestKey = JSON.stringify(choices);
  useEffect(() => {
    const mine = ++round.current;
    previewStarfinderChoices({ characterId: props.characterId, savedAt: '', choices })
      .then((next) => {
        if (mine !== round.current) {
          return;
        }
        setPreview(next);
        setError(null);
        if (choices === null) {
          setChoices(next.chosen);
        }
      })
      .catch((cause: unknown) => {
        if (mine === round.current) {
          setError(cause instanceof Error ? cause.message : String(cause));
        }
      });
  }, [requestKey]);

  async function appendItem(itemId: string, equipped: boolean) {
    if (choices === null) {
      return;
    }
    setRefused([]);
    try {
      const result = await appendToCharacter({
        characterId: props.characterId,
        itemsToAppend: [{ itemId, activeState: equipped ? 'EquippedActive' : 'SelectedInactive' }],
        savedAt: new Date().toISOString(),
        ruleSystemId: STARFINDER_RULE_SYSTEM_ID,
      });
      if (result.success) {
        setChoices(addGear(choices, itemId, equipped));
      } else {
        setRefused([{ id: 'append_to_character', message: result.error ?? 'the item was not added', claimBlocking: true }]);
      }
    } catch (cause: unknown) {
      setError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  async function save() {
    if (choices === null) {
      return;
    }
    setSaving(true);
    setRefused([]);
    try {
      const result = await saveStarfinderChoices({ characterId: props.characterId, savedAt: new Date().toISOString(), choices });
      if (result.kind === 'Saved') {
        props.onSaved(choicesTotalsLine(result.explanations));
      } else {
        setRefused(result.diagnostics);
      }
    } catch (cause: unknown) {
      setError(cause instanceof Error ? cause.message : String(cause));
    } finally {
      setSaving(false);
    }
  }

  const blocking = preview ? blockingChoiceProblems(preview) : [];
  const optional = preview?.problems.filter((problem) => !problem.claimBlocking) ?? [];

  return (
    <section role="dialog" aria-label="Feats, spells and gear" data-testid="sf-choices-dialog" style={DIALOG_STYLE}>
      <h3 style={{ margin: '0 0 0.75rem' }}>Feats, spells and gear</h3>
      {preview && choices ? (
        <StarfinderChoicesPanel preview={preview} choices={choices} equipment={equipment} onChange={setChoices} onAddGear={(id, equipped) => void appendItem(id, equipped)} />
      ) : (
        <p style={{ color: 'var(--color-text-muted)' }}>Reading the character…</p>
      )}
      {optional.length > 0 ? (
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.78rem', margin: '0.2rem 0' }}>
          Not chosen yet (optional): {optional.map((problem) => problem.message).join('; ')}
        </p>
      ) : null}
      {[...blocking, ...refused].length > 0 ? (
        <ul style={{ color: 'var(--color-warn)', fontSize: '0.85rem', margin: '0 0 0.75rem', paddingLeft: '1.1rem' }}>
          {[...blocking, ...refused].map((problem) => (
            <li key={`${problem.id}:${problem.message}`}>{problem.message}</li>
          ))}
        </ul>
      ) : null}
      {error ? <p style={{ color: 'var(--color-error)' }}>{error}</p> : null}
      <div style={{ display: 'flex', gap: '0.5rem' }}>
        <button
          type="button"
          disabled={saving || preview === null || choices === null || blocking.length > 0}
          onClick={() => void save()}
          style={{ backgroundColor: 'var(--color-accent)', border: 'none', borderRadius: 8, color: 'var(--color-on-accent)', cursor: 'pointer', padding: '0.5rem 1rem' }}
        >
          {saving ? 'Saving…' : 'Save choices'}
        </button>
        <button type="button" onClick={props.onClose} style={CLOSE_STYLE}>
          Close feats, spells and gear
        </button>
      </div>
    </section>
  );
}

const DIALOG_STYLE: CSSProperties = { border: '1px solid var(--color-accent)', borderRadius: 10, margin: '0 0 1rem', padding: '1rem' };
const CLOSE_STYLE: CSSProperties = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  color: 'var(--color-text)',
  cursor: 'pointer',
  padding: '0.5rem 1rem',
};
