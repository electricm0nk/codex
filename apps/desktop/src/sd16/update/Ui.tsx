import { useCallback, useEffect, useState } from 'react';
import { ChannelSelector } from './ChannelSelector';
import { CheckPanel } from './CheckPanel';
import { InstallControl } from './InstallControl';
import { InstalledPanel } from './installedPanel';
import { LastCheckPanel } from './lastCheckPanel';
import { PendingRollbackPanel } from './pendingRollbackPanel';
import { RestoreOffer } from './restoreOffer';
import {
  buildUnwiredUpdateDeps,
  UI_BUTTON_BASE_STYLE,
  UI_BUTTON_DISABLED_STYLE,
  UI_CONTAINER_STYLE,
  type UpdateChannelLabel,
  type UpdateControllerDeps,
} from './updateModel';

export const UPDATE_UI_ID = 'update-ui';

export interface RestoreOfferProps {
  priorVersion: string;
  restoreAvailable: boolean;
  onRestore: () => Promise<void>;
}

export interface UpdateUiProps {
  /**
   * Optional initial deps. When omitted, the UI mounts with the
   * deterministic unwired controller (no F3a/F3b fetcher wired yet).
   * The runtime can later swap `deps` to a wired controller; the UI
   * remounts with the new state but never fabricates eligibility.
   */
  initialDeps?: UpdateControllerDeps;
  /**
   * Present only when `verify_relaunch_artifact` (real, run at mount by the
   * caller) reported a verification mismatch. `RestoreOffer` itself is
   * pure/no-side-effects by design, so the actionable "Restore" control and
   * its `perform_restore_previous` call live here at the orchestration layer.
   */
  restoreOffer?: RestoreOfferProps;
}

/**
 * SD-16-E6-F3c page-level entry. Wires the five sub-components together
 * and owns the transient UI-only state (which channel is selected,
 * whether Check / Install are in progress).
 *
 * It does not reach into F3a or F3b; everything passes through the
 * controller interface declared in `updateModel`. When F3a and F3b land,
 * `initialDeps` is supplied with a wired controller and release-notes
 * payload; nothing in this file has to change.
 */
export function UpdateUi({ initialDeps, restoreOffer }: UpdateUiProps) {
  const [deps, setDeps] = useState<UpdateControllerDeps>(
    () => initialDeps ?? buildUnwiredUpdateDeps(),
  );
  useEffect(() => {
    if (initialDeps) {
      setDeps(initialDeps);
    }
  }, [initialDeps]);
  const [checkInProgress, setCheckInProgress] = useState(false);
  const [installInProgress, setInstallInProgress] = useState(false);
  const [restoreInProgress, setRestoreInProgress] = useState(false);
  const handleRestore = useCallback(async () => {
    if (!restoreOffer) {
      return;
    }
    setRestoreInProgress(true);
    try {
      await restoreOffer.onRestore();
    } finally {
      setRestoreInProgress(false);
    }
  }, [restoreOffer]);

  // Re-derive lastCheck.selectedChannel from the deps so the visible
  // selector reflects the most recent authoritative value, including any
  // state mutation F3a performs during runCheck().
  const [selectedChannel, setSelectedChannel] = useState<UpdateChannelLabel>(
    () => (initialDeps ? initialDeps.lastCheck.selectedChannel : 'alpha'),
  );

  useEffect(() => {
    setSelectedChannel(deps.lastCheck.selectedChannel);
  }, [deps.lastCheck.selectedChannel]);

  const handleCheck = useCallback(
    async (channel: UpdateChannelLabel) => {
      setCheckInProgress(true);
      setInstallInProgress(false);
      try {
        await deps.controller.runCheck(channel);
        // The wired controller (F3a/F3b) mutates `lastCheck` and
        // `releaseNotes` in place; the UI re-reads them on next render
        // because they are props of `UpdateControllerDeps`. The
        // unwired controller is a no-op; the UI surfaces its
        // "not wired" reason unchanged.
      } finally {
        setCheckInProgress(false);
        // Re-emit deps so React re-renders against whatever the wired
        // controller mutated. The unwired controller leaves deps as-is;
        // the re-emission is a cheap shape-stable reference swap.
        setDeps((prev) => ({ ...prev }));
      }
    },
    [deps],
  );

  const handleInstall = useCallback(() => {
    setInstallInProgress(true);
    // The actual install transaction lives in SD-16-E7 (Tauri backend);
    // F3c only owns the gate. When E7 lands, the wired `controller` will
    // either expose an `install` method or hand the UI off to a Tauri
    // command. F3c never invokes a Tauri command directly.
    setInstallInProgress(false);
  }, []);

  const handleChannelChange = useCallback(
    (channel: UpdateChannelLabel) => {
      setSelectedChannel(channel);
      setDeps((prev) => ({
        ...prev,
        lastCheck: { ...prev.lastCheck, selectedChannel: channel },
      }));
    },
    [],
  );

  return (
    <main
      id={UPDATE_UI_ID}
      data-testid={UPDATE_UI_ID}
      style={UI_CONTAINER_STYLE}
    >
      <h1
        style={{
          fontSize: '20px',
          fontWeight: 600,
          margin: '0 0 12px 0',
        }}
      >
        Codex Desktop Updates
      </h1>
      <ChannelSelector
        selected={selectedChannel}
        onChange={handleChannelChange}
        disabled={checkInProgress || installInProgress}
      />
      <CheckPanel
        deps={deps}
        checkInProgress={checkInProgress}
        onCheck={handleCheck}
      />
      <InstallControl
        deps={deps}
        installInProgress={installInProgress}
        onInstall={handleInstall}
      />
      <InstalledPanel deps={deps} />
      <LastCheckPanel deps={deps} />
      <PendingRollbackPanel deps={deps} />
      {restoreOffer ? (
        <>
          <RestoreOffer
            priorVersion={restoreOffer.priorVersion}
            restoreAvailable={restoreOffer.restoreAvailable}
          />
          {restoreOffer.restoreAvailable ? (
            <button
              id="restore-previous-button"
              data-testid="restore-previous-button"
              type="button"
              disabled={restoreInProgress}
              style={restoreInProgress ? UI_BUTTON_DISABLED_STYLE : UI_BUTTON_BASE_STYLE}
              onClick={() => {
                void handleRestore();
              }}
            >
              {restoreInProgress ? 'Restoring…' : 'Restore previous version'}
            </button>
          ) : null}
        </>
      ) : null}
    </main>
  );
}
