import { useEffect, useRef, useState } from 'react';
import { deleteCharacterPortrait, loadCharacterPortrait, saveCharacterPortrait } from '../boundary/characterPortrait';
import { ACCEPTED_PORTRAIT_TYPES, resizeAndCropPortrait } from './portraitImageProcessing';

/** Upload box for a character's portrait — a silhouette placeholder until one is set. Fills the width of its container and stays square. */
export function PortraitUpload(props: { characterId: string }) {
  const [portraitUrl, setPortraitUrl] = useState<string | null>(null);
  const [hovered, setHovered] = useState(false);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    let cancelled = false;
    setPortraitUrl(null);
    setError(null);
    loadCharacterPortrait(props.characterId)
      .then((url) => {
        if (!cancelled) {
          setPortraitUrl(url);
        }
      })
      .catch(() => {
        // Preview mode (no Tauri runtime) resolves to null already; any
        // real failure here just leaves the silhouette placeholder up.
      });
    return () => {
      cancelled = true;
    };
  }, [props.characterId]);

  async function onFileSelected(event: { target: { files: FileList | null; value: string } }) {
    const file = event.target.files?.[0];
    event.target.value = '';
    if (!file) {
      return;
    }

    setBusy(true);
    setError(null);
    const previousUrl = portraitUrl;
    try {
      const resized = await resizeAndCropPortrait(file);
      setPortraitUrl(resized);
      await saveCharacterPortrait(props.characterId, resized);
    } catch (cause: unknown) {
      setPortraitUrl(previousUrl);
      setError(cause instanceof Error ? cause.message : 'Could not upload that image.');
    } finally {
      setBusy(false);
    }
  }

  async function onRemove(event: React.MouseEvent) {
    event.stopPropagation();
    setBusy(true);
    setError(null);
    try {
      await deleteCharacterPortrait(props.characterId);
      setPortraitUrl(null);
    } catch (cause: unknown) {
      setError(cause instanceof Error ? cause.message : 'Could not remove the portrait.');
    } finally {
      setBusy(false);
    }
  }

  return (
    <div>
      <button
        type="button"
        onClick={() => inputRef.current?.click()}
        onMouseEnter={() => setHovered(true)}
        onMouseLeave={() => setHovered(false)}
        disabled={busy}
        title={portraitUrl ? 'Change portrait' : 'Upload a portrait'}
        style={{
          aspectRatio: '1 / 1',
          backgroundColor: 'var(--color-surface-2)',
          border: '1px solid var(--color-border)',
          borderRadius: 10,
          cursor: busy ? 'wait' : 'pointer',
          display: 'block',
          overflow: 'hidden',
          padding: 0,
          position: 'relative',
          width: '100%',
        }}
      >
        {portraitUrl ? (
          <img
            src={portraitUrl}
            alt="Character portrait"
            style={{ display: 'block', height: '100%', objectFit: 'cover', width: '100%' }}
          />
        ) : (
          <SilhouetteIcon />
        )}

        {portraitUrl && hovered && !busy ? (
          <span
            role="button"
            aria-label="Remove portrait"
            onClick={onRemove}
            style={{
              alignItems: 'center',
              backgroundColor: 'rgba(0, 0, 0, 0.6)',
              borderRadius: '50%',
              color: '#fff',
              cursor: 'pointer',
              display: 'flex',
              fontSize: '0.8rem',
              fontWeight: 700,
              height: 20,
              justifyContent: 'center',
              position: 'absolute',
              right: 4,
              top: 4,
              width: 20,
            }}
          >
            ×
          </span>
        ) : null}

        {busy ? (
          <span
            style={{
              alignItems: 'center',
              backgroundColor: 'rgba(0, 0, 0, 0.45)',
              color: '#fff',
              display: 'flex',
              fontSize: '0.65rem',
              inset: 0,
              justifyContent: 'center',
              position: 'absolute',
            }}
          >
            …
          </span>
        ) : null}
      </button>

      <input
        ref={inputRef}
        type="file"
        accept={ACCEPTED_PORTRAIT_TYPES.join(',')}
        onChange={onFileSelected}
        style={{ display: 'none' }}
      />

      {error ? (
        <p style={{ color: 'var(--color-error)', fontSize: '0.7rem', margin: '0.3rem 0 0' }}>{error}</p>
      ) : null}
    </div>
  );
}

function SilhouetteIcon() {
  return (
    <svg
      viewBox="0 0 24 24"
      aria-hidden
      style={{ color: 'var(--color-text-faint)', height: '60%', width: '60%' }}
    >
      <circle cx="12" cy="8" r="4" fill="currentColor" />
      <path d="M4 20c0-4.4 3.6-8 8-8s8 3.6 8 8" fill="currentColor" />
    </svg>
  );
}
