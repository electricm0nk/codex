import { useEffect, useState, type CSSProperties } from 'react';
import { loadCyberpunkCatalog } from '../../boundary/loadCyberpunkCatalog';
import { computeCyberpunkDeck, type CyberpunkDeckRequest, type CyberpunkDeckResponse } from '../../boundary/cprDeck';
import {
  INTERFACE_RANKS,
  buildDeckOptions,
  deckRequest,
  describeDeckLoad,
  initialSelection,
  withoutIndex,
  type DeckOption,
  type DeckOptions,
  type DeckSelection,
} from './cyberpunkDeckModel';

/**
 * SD-38 E6.3 -- the Netrunning tools screen: pick a Cyberdeck, Programs and Hardware, and read
 * the slots used and available, the NET Actions for an Interface rank, and the engine's refusal
 * when the loadout is over capacity. Every number is a row `compute_cyberpunk_deck` returns.
 */

const panel: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 10,
  padding: '0.75rem 1rem',
};

const smallButton: CSSProperties = {
  background: 'none',
  border: '1px solid var(--color-border)',
  borderRadius: 8,
  cursor: 'pointer',
  padding: '0.25rem 0.75rem',
};

function slotText(option: DeckOption): string {
  return option.slots === null ? '' : ` (${option.slots} slot${option.slots === 1 ? '' : 's'})`;
}

function OptionPicker(props: {
  label: string;
  options: readonly DeckOption[];
  chosen: readonly string[];
  onAdd: (name: string) => void;
  onRemove: (index: number) => void;
}) {
  const [pick, setPick] = useState('');
  return (
    <div data-cpr-deck-picker={props.label} style={{ ...panel, display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
      <strong>{props.label}</strong>
      <div style={{ display: 'flex', gap: '0.5rem' }}>
        <select aria-label={`${props.label} to add`} value={pick} onChange={(event) => setPick(event.target.value)}>
          <option value="">Choose…</option>
          {props.options.map((option) => (
            <option key={option.name} value={option.name}>
              {`${option.name}${slotText(option)}${option.detail ? ` — ${option.detail}` : ''}`}
            </option>
          ))}
        </select>
        <button
          type="button"
          disabled={pick === ''}
          onClick={() => {
            props.onAdd(pick);
          }}
          style={smallButton}
        >
          Add
        </button>
      </div>
      <ul style={{ margin: 0, paddingLeft: '1.1rem' }}>
        {props.chosen.map((name, index) => (
          <li key={`${name}:${index}`} data-cpr-deck-item={`${props.label}:${name}`}>
            {name}{' '}
            <button type="button" onClick={() => props.onRemove(index)} style={smallButton} aria-label={`Remove ${name}`}>
              Remove
            </button>
          </li>
        ))}
      </ul>
    </div>
  );
}

/** The calculator body for one selection and the command's answer (pure: the screen below loads it). */
export function CyberpunkDeckView(props: {
  options: DeckOptions;
  selection: DeckSelection;
  onChange: (next: DeckSelection) => void;
  response: CyberpunkDeckResponse | null;
  refusal: string | null;
}) {
  const { options, selection, response } = props;
  const change = (patch: Partial<DeckSelection>) => props.onChange({ ...selection, ...patch });
  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '0.75rem' }}>
      <label style={{ display: 'flex', flexDirection: 'column', gap: '0.25rem', fontSize: '0.85rem' }}>
        Cyberdeck
        <select value={selection.deck} onChange={(event) => change({ deck: event.target.value })}>
          {options.decks.map((deck) => (
            <option key={deck.name} value={deck.name}>{`${deck.name}${slotText(deck)}`}</option>
          ))}
        </select>
      </label>
      <label style={{ display: 'flex', flexDirection: 'column', gap: '0.25rem', fontSize: '0.85rem' }}>
        Interface rank
        <select value={selection.interfaceRank} onChange={(event) => change({ interfaceRank: Number(event.target.value) })}>
          {INTERFACE_RANKS.map((rank) => (
            <option key={rank} value={rank}>{rank}</option>
          ))}
        </select>
      </label>
      <label>
        <input
          type="checkbox"
          checked={selection.extras.bodyweightSuit}
          onChange={(event) => change({ extras: { ...selection.extras, bodyweightSuit: event.target.checked } })}
        />{' '}
        Wearing a Bodyweight Suit
      </label>
      <label>
        <input
          type="checkbox"
          checked={selection.extras.cyberarmDeck}
          onChange={(event) => change({ extras: { ...selection.extras, cyberarmDeck: event.target.checked } })}
        />{' '}
        Cyberdeck Cyberarm option
      </label>
      <OptionPicker
        label="Programs"
        options={options.programs}
        chosen={selection.programs}
        onAdd={(name) => change({ programs: [...selection.programs, name] })}
        onRemove={(index) => change({ programs: withoutIndex(selection.programs, index) })}
      />
      <OptionPicker
        label="Hardware"
        options={options.hardware}
        chosen={selection.hardware}
        onAdd={(name) => change({ hardware: [...selection.hardware, name] })}
        onRemove={(index) => change({ hardware: withoutIndex(selection.hardware, index) })}
      />
      {props.refusal !== null ? (
        <p role="alert" data-cpr-deck-refusal style={{ ...panel, color: 'var(--color-danger, #c0392b)', margin: 0 }}>
          {props.refusal}
        </p>
      ) : response !== null ? (
        <div data-cpr-deck-result style={panel}>
          <p style={{ fontWeight: 600, margin: 0 }}>
            {`Slots used ${response.slotsUsed} of ${response.slotsAvailable}`}
          </p>
          {response.netActions !== null ? (
            <p data-cpr-deck-net-actions style={{ margin: '0.25rem 0 0' }}>
              {`NET Actions per Turn: ${response.netActions.value}`}
            </p>
          ) : null}
          <ul style={{ margin: '0.5rem 0 0', paddingLeft: '1.1rem' }}>
            {describeDeckLoad(response).map((row) => (
              <li key={row.id} data-cpr-deck-row={row.id} style={{ fontSize: '0.85rem' }}>
                {row.detail}
              </li>
            ))}
          </ul>
        </div>
      ) : null}
    </div>
  );
}

export function CyberpunkDeckScreen(props: {
  /** The calculator command; the screen passes the real one, a test passes the engine's rows. */
  compute?: (request: CyberpunkDeckRequest) => Promise<CyberpunkDeckResponse>;
}) {
  const compute = props.compute ?? computeCyberpunkDeck;
  const [options, setOptions] = useState<DeckOptions | null>(null);
  const [selection, setSelection] = useState<DeckSelection | null>(null);
  const [response, setResponse] = useState<CyberpunkDeckResponse | null>(null);
  const [refusal, setRefusal] = useState<string | null>(null);
  const [loadError, setLoadError] = useState<string | null>(null);

  useEffect(() => {
    let live = true;
    Promise.all([loadCyberpunkCatalog('cyberdeck'), loadCyberpunkCatalog('program'), loadCyberpunkCatalog('net_hardware')])
      .then(([decks, programs, hardware]) => {
        if (!live) return;
        const built = buildDeckOptions(decks.records, programs.records, hardware.records);
        setOptions(built);
        setSelection(initialSelection(built));
      })
      .catch((cause: unknown) => {
        if (live) setLoadError(cause instanceof Error ? cause.message : String(cause));
      });
    return () => {
      live = false;
    };
  }, []);

  useEffect(() => {
    if (selection === null || selection.deck === '') return;
    let live = true;
    compute(deckRequest(selection))
      .then((answer) => {
        if (!live) return;
        setResponse(answer);
        setRefusal(null);
      })
      .catch((cause: unknown) => {
        if (!live) return;
        setResponse(null);
        setRefusal(cause instanceof Error ? cause.message : String(cause));
      });
    return () => {
      live = false;
    };
  }, [selection, compute]);

  if (loadError !== null) {
    return (
      <p role="alert" style={{ color: 'var(--color-danger, #c0392b)', margin: 0 }}>
        {loadError}
      </p>
    );
  }
  if (options === null || selection === null) {
    return <p style={{ color: 'var(--color-text-muted)', margin: 0 }}>Loading…</p>;
  }
  return (
    <CyberpunkDeckView options={options} selection={selection} onChange={setSelection} response={response} refusal={refusal} />
  );
}
