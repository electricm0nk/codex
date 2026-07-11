import { useEffect, type ReactNode } from 'react';

/**
 * Centered, Obsidian-style settings dialog: a left options rail plus a
 * scrolling detail pane. Tabs are data-driven so new option groups can be
 * added by extending `SETTINGS_TABS` and the `panels` map the caller passes.
 */

export type SettingsTab = 'appearance' | 'developer';

export const SETTINGS_TABS: ReadonlyArray<{ id: SettingsTab; label: string }> = [
  { id: 'appearance', label: 'Appearance' },
  { id: 'developer', label: 'Developer' },
];

export function SettingsModal(props: {
  open: boolean;
  activeTab: SettingsTab;
  onTabChange: (tab: SettingsTab) => void;
  onClose: () => void;
  panels: Record<SettingsTab, ReactNode>;
}) {
  const { open, onClose } = props;

  useEffect(() => {
    if (!open) {
      return;
    }
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        onClose();
      }
    };
    window.addEventListener('keydown', onKeyDown);
    return () => window.removeEventListener('keydown', onKeyDown);
  }, [open, onClose]);

  if (!open) {
    return null;
  }

  const activeTabLabel = SETTINGS_TABS.find((tab) => tab.id === props.activeTab)?.label ?? '';

  return (
    <div
      role="presentation"
      onClick={onClose}
      style={{
        alignItems: 'center',
        backgroundColor: 'rgba(0, 0, 0, 0.55)',
        display: 'flex',
        inset: 0,
        justifyContent: 'center',
        padding: '2rem',
        position: 'fixed',
        zIndex: 1000,
      }}
    >
      <div
        role="dialog"
        aria-modal="true"
        aria-label="Settings"
        onClick={(event) => event.stopPropagation()}
        style={{
          backgroundColor: 'var(--color-surface)',
          border: '1px solid var(--color-border)',
          borderRadius: 12,
          boxShadow: '0 24px 60px rgba(0, 0, 0, 0.5)',
          display: 'flex',
          height: 'min(620px, 85vh)',
          overflow: 'hidden',
          width: 'min(900px, 92vw)',
        }}
      >
        {/* Options rail */}
        <nav
          style={{
            backgroundColor: 'var(--color-page)',
            borderRight: '1px solid var(--color-border)',
            display: 'flex',
            flexDirection: 'column',
            gap: '0.15rem',
            minWidth: 200,
            padding: '1.25rem 0.75rem',
          }}
        >
          <p
            style={{
              color: 'var(--color-text-muted)',
              fontSize: '0.72rem',
              fontWeight: 700,
              letterSpacing: '0.08em',
              margin: '0 0 0.5rem 0.75rem',
              textTransform: 'uppercase',
            }}
          >
            Options
          </p>
          {SETTINGS_TABS.map((tab) => {
            const active = tab.id === props.activeTab;
            return (
              <button
                key={tab.id}
                type="button"
                onClick={() => props.onTabChange(tab.id)}
                style={{
                  backgroundColor: active ? 'var(--color-accent)' : 'transparent',
                  border: 'none',
                  borderRadius: 8,
                  color: active ? 'var(--color-on-accent)' : 'var(--color-text-secondary)',
                  cursor: 'pointer',
                  fontSize: '0.9rem',
                  fontWeight: active ? 700 : 500,
                  padding: '0.5rem 0.75rem',
                  textAlign: 'left',
                }}
              >
                {tab.label}
              </button>
            );
          })}
        </nav>

        {/* Detail pane */}
        <section style={{ display: 'flex', flexDirection: 'column', flex: 1, minWidth: 0 }}>
          <header
            style={{
              alignItems: 'center',
              borderBottom: '1px solid var(--color-border)',
              display: 'flex',
              justifyContent: 'space-between',
              padding: '1rem 1.5rem',
            }}
          >
            <h2 style={{ fontSize: '1.1rem', margin: 0 }}>{activeTabLabel}</h2>
            <button
              type="button"
              aria-label="Close settings"
              onClick={onClose}
              style={{
                background: 'none',
                border: 'none',
                color: 'var(--color-text-muted)',
                cursor: 'pointer',
                fontSize: '1.4rem',
                lineHeight: 1,
                padding: '0.15rem 0.35rem',
              }}
            >
              ×
            </button>
          </header>
          <div style={{ flex: 1, overflowY: 'auto', padding: '1.5rem' }}>{props.panels[props.activeTab]}</div>
        </section>
      </div>
    </div>
  );
}
