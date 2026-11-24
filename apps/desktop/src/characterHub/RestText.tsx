import { useEffect, useState, type CSSProperties } from 'react';
import { loadRestText, type RestTextDto } from '../boundary/loadRestText';

const headingStyle: CSSProperties = { color: 'var(--color-text-muted)', fontSize: '0.62rem', letterSpacing: '0.03em', margin: '1rem 0 0.3rem', textTransform: 'uppercase' };
const textStyle: CSSProperties = { fontSize: '0.85rem', lineHeight: 1.5, margin: '0 0 0.4rem' };

/** The rest and recovery text, read-only: the same rows, override or gap sentence the print block shows. */
export function RestTextView(props: { rest: RestTextDto }) {
  const { rest } = props;
  return (
    <section aria-label="Rest and recovery">
      <p style={headingStyle}>Rest and recovery</p>
      {rest.overrideText ? (
        <p style={{ ...textStyle, whiteSpace: 'pre-wrap' }}>{rest.overrideText}</p>
      ) : rest.rows.length > 0 ? (
        rest.rows.map((row) => (
          <p key={row.id} style={textStyle}>
            <strong>{row.rule}.</strong> {row.text}{' '}
            <span style={{ color: 'var(--color-text-muted)' }}>({row.book} {row.page})</span>
          </p>
        ))
      ) : (
        <p style={textStyle}>{rest.gap ?? 'No rest and recovery text for this character.'}</p>
      )}
    </section>
  );
}

/** Loads the character's rest text and shows it; a failed read says so rather than hiding the section. */
export function RestTextPanel(props: { characterId: string; overrideText?: string }) {
  const [rest, setRest] = useState<RestTextDto | null>(null);
  const [error, setError] = useState<string | null>(null);
  useEffect(() => {
    let live = true;
    setRest(null);
    setError(null);
    loadRestText(props.characterId).then(
      (dto) => live && setRest(dto),
      (cause: unknown) => live && setError(cause instanceof Error ? cause.message : 'Could not load the rest text.'),
    );
    return () => {
      live = false;
    };
  }, [props.characterId, props.overrideText]);
  if (error) {
    return <p role="alert" style={textStyle}>{error}</p>;
  }
  return rest ? <RestTextView rest={rest} /> : null;
}
