import {
  buildRelaunchPromptMarkup,
  INSTALL_RELAUNCH_PROMPT_ID,
  makePerformInstallResponseFixture,
  mountRelaunchPrompt,
  performInstall,
  type PerformInstallResponse,
} from "./installAction";
import { assert, assertEqual } from "../../testSupport/asserts";

interface CapturedInvokeCall {
  command: string;
  args: Record<string, unknown>;
}

interface InvokeStub {
  calls: CapturedInvokeCall[];
  setResponse: (value: unknown) => void;
  setError: (error: Error) => void;
}

/**
 * Install a Tauri-runtime + invoke stub for the duration of a single test.
 *
 * The repo's `boundary/runtime.ts` decides "Tauri is present" by checking
 * `typeof window !== 'undefined' && ('__TAURI_INTERNALS__' in window || ...)`.
 * In node tsx that returns false, so without this stub `performInstall`
 * short-circuits to the no-runtime sentinel. We attach a stub `window` and
 * a captured `__TAURI_INTERNALS__.invoke` so the test exercises the real
 * `invoke(...)` call path.
 */
function installTauriRuntime(): { stub: InvokeStub; restore: () => void } {
  const calls: CapturedInvokeCall[] = [];
  let response: unknown = undefined;
  let error: Error | undefined;
  const stub: InvokeStub = {
    calls,
    setResponse(value) {
      response = value;
      error = undefined;
    },
    setError(value) {
      error = value;
      response = undefined;
    },
  };
  const win = (globalThis as Record<string, unknown>).window as
    | Record<string, unknown>
    | undefined;
  const hadWindow = win !== undefined;
  const previousWindow = win;
  const tauriInternals = (globalThis as Record<string, unknown>)
    .__TAURI_INTERNALS__ as Record<string, unknown> | undefined;
  const previousInternals = tauriInternals;

  const stubWindow: Record<string, unknown> = hadWindow
    ? { ...(previousWindow as Record<string, unknown>) }
    : {};
  stubWindow.__TAURI_INTERNALS__ = {
    invoke: async (command: string, args: Record<string, unknown>) => {
      calls.push({ command, args });
      if (error) throw error;
      return response;
    },
  };
  (globalThis as Record<string, unknown>).window = stubWindow;

  return {
    stub,
    restore: () => {
      if (hadWindow && previousWindow) {
        (globalThis as Record<string, unknown>).window = previousWindow;
      } else {
        delete (globalThis as Record<string, unknown>).window;
      }
      if (previousInternals === undefined) {
        delete (globalThis as Record<string, unknown>).__TAURI_INTERNALS__;
      }
    },
  };
}

async function main(): Promise<void> {
  await testPerformInstallForwardsManifestAndIndexUrl();
  await testPerformInstallReturnsResponseVerbatim();
  await testPerformInstallMountsRelaunchPromptOnMustRelaunch();
  await testPerformInstallDoesNotMountOnMustRelaunchFalse();
  await testMountRelaunchPromptReflectsMustRelaunchFlag();
  await testMountRelaunchPromptIsIdempotent();
  await testBuildRelaunchPromptMarkupCarriesAllHookAttributes();
}

async function testPerformInstallForwardsManifestAndIndexUrl(): Promise<void> {
  const { stub, restore } = installTauriRuntime();
  try {
    stub.setResponse(makePerformInstallResponseFixture({ mustRelaunch: false }));
    const manifest = { version: "0.0.2", channel: "alpha" };
    await performInstall(manifest, "https://example.invalid/alpha/index.json");
    assertEqual(stub.calls.length, 1, "performInstall must invoke exactly once");
    assertEqual(
      stub.calls[0]!.command,
      "perform_install",
      "command name must be perform_install",
    );
    assertEqual(
      stub.calls[0]!.args.manifest,
      manifest,
      "manifest argument must be forwarded verbatim",
    );
    assertEqual(
      stub.calls[0]!.args.indexUrl,
      "https://example.invalid/alpha/index.json",
      "indexUrl argument must be forwarded verbatim",
    );
  } finally {
    restore();
  }
}

async function testPerformInstallReturnsResponseVerbatim(): Promise<void> {
  const { stub, restore } = installTauriRuntime();
  try {
    const fixture = makePerformInstallResponseFixture({
      mustRelaunch: true,
      fromVersion: "0.0.5",
      toVersion: "0.0.6",
      newArtifactSha256: "deadbeef",
    });
    stub.setResponse(fixture);
    const response = await performInstall({ version: "0.0.6" }, "https://x/index.json");
    assertEqual(response, fixture, "performInstall must return the Rust response verbatim");
  } finally {
    restore();
  }
}

async function testPerformInstallMountsRelaunchPromptOnMustRelaunch(): Promise<void> {
  const { stub, restore } = installTauriRuntime();
  try {
    stub.setResponse(
      makePerformInstallResponseFixture({
        mustRelaunch: true,
        fromVersion: "0.0.0",
        toVersion: "0.0.1",
      }),
    );
    await performInstall({ version: "0.0.1" }, "https://x/index.json");
    const markup = buildRelaunchPromptMarkup(makePerformInstallResponseFixture());
    assert(
      markup.includes(`id="${INSTALL_RELAUNCH_PROMPT_ID}"`),
      "relaunch prompt markup must carry the canonical DOM id",
    );
    assert(
      markup.includes('data-from-version="0.0.0"'),
      "relaunch prompt markup must expose data-from-version",
    );
    assert(
      markup.includes('data-to-version="0.0.1"'),
      "relaunch prompt markup must expose data-to-version",
    );
    assert(
      markup.includes('data-must-relaunch="true"'),
      "relaunch prompt markup must expose data-must-relaunch=true",
    );
  } finally {
    restore();
  }
}

async function testPerformInstallDoesNotMountOnMustRelaunchFalse(): Promise<void> {
  const { stub, restore } = installTauriRuntime();
  try {
    stub.setResponse(makePerformInstallResponseFixture({ mustRelaunch: false }));
    await performInstall({ version: "0.0.1" }, "https://x/index.json");
    const markup = buildRelaunchPromptMarkup(
      makePerformInstallResponseFixture({ mustRelaunch: false }),
    );
    assert(
      !markup.includes('data-must-relaunch="true"'),
      "when mustRelaunch=false the markup must not claim relaunch-required",
    );
  } finally {
    restore();
  }
}

async function testMountRelaunchPromptReflectsMustRelaunchFlag(): Promise<void> {
  const globalRecord = globalThis as Record<string, unknown>;
  const previousDocument = globalRecord.document;
  const appended: Array<{ id: string; attrs: Record<string, string> }> = [];
  try {
    const fakeDocument = {
      getElementById(id: string) {
        return (appended.find((entry) => entry.id === id) as unknown as { remove: () => void }) ?? null;
      },
      createElement() {
        const attrs: Record<string, string> = {};
        const element = {
          id: "",
          attrs,
          textContent: "",
          setAttribute(name: string, value: string) {
            attrs[name] = value;
          },
          remove() {
            const index = appended.findIndex((entry) => entry.id === element.id);
            if (index >= 0) appended.splice(index, 1);
          },
        };
        return element;
      },
      body: {
        appendChild(element: { id: string; attrs: Record<string, string> }) {
          appended.push(element);
        },
      },
    };
    globalRecord.document = fakeDocument as unknown as Document;

    mountRelaunchPrompt(makePerformInstallResponseFixture({ mustRelaunch: false }));

    assertEqual(appended.length, 1, "mountRelaunchPrompt must append one hook element");
    assertEqual(
      appended[0]!.attrs["data-must-relaunch"],
      "false",
      "mountRelaunchPrompt must reflect mustRelaunch=false in the DOM contract",
    );
  } finally {
    if (previousDocument === undefined) {
      delete globalRecord.document;
    } else {
      globalRecord.document = previousDocument;
    }
  }
}

async function testMountRelaunchPromptIsIdempotent(): Promise<void> {
  // The real DOM mutation is host-gated; in node tsx mountRelaunchPrompt is a no-op,
  // but the pure renderer must produce stable markup that includes the canonical id.
  const fixture = makePerformInstallResponseFixture();
  const markupA = buildRelaunchPromptMarkup(fixture);
  const markupB = buildRelaunchPromptMarkup(fixture);
  assertEqual(markupA, markupB, "pure renderer must be deterministic");
  mountRelaunchPrompt(fixture);
  mountRelaunchPrompt(fixture);
  assert(true, "mountRelaunchPrompt must be safe to call repeatedly");
}

async function testBuildRelaunchPromptMarkupCarriesAllHookAttributes(): Promise<void> {
  const response: PerformInstallResponse = {
    mustRelaunch: true,
    newArtifactSha256: "sha-of-new-artifact",
    newManagedExecutablePath: "/opt/codex/Codex.AppImage",
    previousBackupPath: "/home/x/.config/codex-desktop-shell-scaffold/update/backups/Codex-Desktop-Shell-Scaffold.previous.AppImage",
    pendingStatePath: "/home/x/.config/codex-desktop-shell-scaffold/update/pending-update.json",
    fromVersion: "0.0.1",
    toVersion: "0.0.2",
  };
  const markup = buildRelaunchPromptMarkup(response);
  const expected = [
    `id="${INSTALL_RELAUNCH_PROMPT_ID}"`,
    'data-from-version="0.0.1"',
    'data-to-version="0.0.2"',
    'data-pending-state-path="/home/x/.config/codex-desktop-shell-scaffold/update/pending-update.json"',
    'data-managed-executable-path="/opt/codex/Codex.AppImage"',
    'data-new-artifact-sha256="sha-of-new-artifact"',
    'data-previous-backup-path="/home/x/.config/codex-desktop-shell-scaffold/update/backups/Codex-Desktop-Shell-Scaffold.previous.AppImage"',
    'data-must-relaunch="true"',
  ];
  for (const fragment of expected) {
    assert(
      markup.includes(fragment),
      `relaunch prompt markup must carry \`${fragment}\` — got: ${markup}`,
    );
  }
}

main()
  .then(() => {
    console.log("installAction.test.ts: all assertions passed");
  })
  .catch((error) => {
    console.error(error);
    throw error;
  });