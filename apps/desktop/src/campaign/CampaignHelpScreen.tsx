import { useEffect, useState, type CSSProperties } from 'react';
import { listCampaignTree } from '../boundary/campaignFolder/campaignFiles';
import type { CampaignTreeEntry } from '../boundary/campaignFolder/types';
import { openExternal } from '../boundary/openExternal';
import { CAMPAIGN_HELP_SECTIONS, OBSIDIAN_URL, helpTreeLines } from './campaignHelpModel';

/**
 * Campaign help (SD-42 `decisions.md §13.3`): how a campaign folder is shared, the folder tree
 * (live from `list_campaign_tree` when a campaign is open, else the new-campaign layout), the
 * Obsidian link, "everything is shared" and how Characters/ works. A dialog over whichever
 * campaign screen opened it, so every screen reaches it through `CampaignHelpButton`.
 */

const linkButton: CSSProperties = {
  background: 'none',
  border: '1px solid var(--color-border)',
  borderRadius: 8,
  color: 'var(--color-accent)',
  cursor: 'pointer',
  padding: '0.4rem 0.9rem',
};

type Tree = { kind: 'template' } | { kind: 'loading' } | { kind: 'live'; entries: CampaignTreeEntry[] } | { kind: 'failed'; message: string };

export function CampaignHelpScreen(props: { campaignDir: string | null; onClose: () => void }) {
  const [tree, setTree] = useState<Tree>(props.campaignDir ? { kind: 'loading' } : { kind: 'template' });
  const [linkError, setLinkError] = useState<string | null>(null);

  useEffect(() => {
    if (!props.campaignDir) {
      setTree({ kind: 'template' });
      return;
    }
    let live = true;
    setTree({ kind: 'loading' });
    listCampaignTree(props.campaignDir)
      .then((entries) => live && setTree({ kind: 'live', entries }))
      .catch((cause: unknown) => live && setTree({ kind: 'failed', message: cause instanceof Error ? cause.message : String(cause) }));
    return () => {
      live = false;
    };
  }, [props.campaignDir]);

  function openObsidian() {
    setLinkError(null);
    openExternal(OBSIDIAN_URL).catch((cause: unknown) => setLinkError(cause instanceof Error ? cause.message : String(cause)));
  }

  return (
    <div
      role="presentation"
      onClick={props.onClose}
      style={{ alignItems: 'center', backgroundColor: 'rgba(0, 0, 0, 0.55)', display: 'flex', inset: 0, justifyContent: 'center', padding: '2rem', position: 'fixed', zIndex: 1000 }}
    >
      <div
        role="dialog"
        aria-modal="true"
        aria-label="Campaign help"
        data-testid="campaign-help"
        onClick={(event) => event.stopPropagation()}
        style={{
          backgroundColor: 'var(--color-surface)',
          border: '1px solid var(--color-border)',
          borderRadius: 12,
          boxShadow: '0 24px 60px rgba(0, 0, 0, 0.5)',
          maxHeight: '90vh',
          maxWidth: 760,
          overflowY: 'auto',
          padding: '1.5rem 1.75rem',
          width: '96vw',
        }}
      >
        <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between', marginBottom: '0.5rem' }}>
          <h2 style={{ margin: 0 }}>Campaign help</h2>
          <button type="button" onClick={props.onClose} style={linkButton}>
            Close
          </button>
        </div>
        {CAMPAIGN_HELP_SECTIONS.map((section) => (
          <section key={section.id} data-testid={`campaign-help-${section.id}`} style={{ marginTop: '1.1rem' }}>
            <h3 style={{ margin: '0 0 0.4rem' }}>{section.title}</h3>
            {section.paragraphs.map((paragraph) => (
              <p key={paragraph} style={{ color: 'var(--color-text-secondary)', lineHeight: 1.55, margin: '0 0 0.45rem' }}>
                {paragraph}
              </p>
            ))}
            {section.id === 'structure' ? (
              tree.kind === 'failed' ? (
                <p role="alert" style={{ color: 'var(--color-error)', margin: 0 }}>{tree.message}</p>
              ) : (
                <pre
                  data-testid="campaign-help-tree"
                  style={{ backgroundColor: 'var(--color-surface-2)', border: '1px solid var(--color-border)', borderRadius: 8, fontSize: '0.8rem', margin: 0, overflowX: 'auto', padding: '0.7rem 0.9rem' }}
                >
                  {tree.kind === 'loading'
                    ? 'Reading the campaign folder…'
                    : helpTreeLines(tree.kind === 'live' ? tree.entries : null).join('\n')}
                </pre>
              )
            ) : null}
            {section.id === 'editors' ? (
              <div>
                <button type="button" data-testid="campaign-help-obsidian" onClick={openObsidian} style={linkButton}>
                  Open {OBSIDIAN_URL}
                </button>
                {linkError ? <p role="alert" style={{ color: 'var(--color-error)', margin: '0.4rem 0 0' }}>{linkError}</p> : null}
              </div>
            ) : null}
          </section>
        ))}
      </div>
    </div>
  );
}

/** The "Help" button every campaign screen's header carries; it opens the help dialog over that screen. */
export function CampaignHelpButton(props: { campaignDir: string | null; style?: CSSProperties }) {
  const [open, setOpen] = useState(false);
  return (
    <>
      <button type="button" data-testid="campaign-help-button" onClick={() => setOpen(true)} style={props.style ?? linkButton}>
        Help
      </button>
      {open ? <CampaignHelpScreen campaignDir={props.campaignDir} onClose={() => setOpen(false)} /> : null}
    </>
  );
}
