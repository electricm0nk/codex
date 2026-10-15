import { readFileSync } from 'node:fs';
import type { LoadSavedCharacterResponse } from './loadSavedCharacterDetail';
import { applyCyberpunkImprovement, previewCyberpunkImprovement, type CyberpunkImprovePreview } from './cprImprove';
import { assertEqual } from '../testSupport/asserts';

/**
 * SD-38 E5.1: the improve dialog previews and spends through `preview_cyberpunk_improvement` and
 * `apply_cyberpunk_improvement` (`apps/desktop/src-tauri/src/cpr_improve.rs`). This proves the wire
 * shape the Rust commands deserialise (`CprImproveRequest`: `{ request: { characterId, ipAwarded,
 * purchases: [{ kind: 'skill', key, to } | { kind: 'roleRank', to } | { kind: 'roleSwitch', role }] } }`), that the preview and the
 * reloaded sheet are handed back as answered, and that a refusal reaches the caller with its reason.
 * The prices, the refusals and the save are proved against the real store in Rust.
 */
async function messageOf(run: () => Promise<unknown>): Promise<string> {
  try {
    await run();
  } catch (cause) {
    return cause instanceof Error ? cause.message : String(cause);
  }
  return '';
}

async function main() {
  const plan = {
    ipAwarded: 200,
    purchases: [
      { kind: 'skill' as const, key: 'Handgun', to: 7 },
      { kind: 'roleRank' as const, to: 5 },
      { kind: 'roleSwitch' as const, role: 'Nomad' },
    ],
  };
  assertEqual(
    await messageOf(() => previewCyberpunkImprovement('cpr-solo-kitted', plan)),
    'Tauri runtime not available for previewing a Cyberpunk RED improvement',
    'no-runtime preview failure is descriptive',
  );
  assertEqual(
    await messageOf(() => applyCyberpunkImprovement('cpr-solo-kitted', plan)),
    'Tauri runtime not available for spending Cyberpunk RED Improvement Points',
    'no-runtime apply failure is descriptive',
  );

  const sheet = JSON.parse(
    readFileSync(new URL('../characterHub/__tests__/cyberpunkSheetFixtures/cpr-solo-kitted.json', import.meta.url), 'utf8'),
  ) as LoadSavedCharacterResponse;
  const preview: CyberpunkImprovePreview = {
    skills: [],
    role: null,
    switch: [],
    cyberware: [],
    therapy: [],
    rows: [{ id: 'cpr.improve.ip.available', value: 200, detail: 'Improvement Points available 200' }],
    refusal: null,
    seed: 5,
  };
  const calls: Array<{ cmd: string; args: unknown }> = [];
  const globals = globalThis as Record<string, unknown>;
  globals.window = {
    __TAURI_INTERNALS__: {
      invoke: async (cmd: string, args: unknown) => {
        calls.push({ cmd, args });
        if (cmd === 'preview_cyberpunk_improvement') {
          return preview;
        }
        if (cmd === 'apply_cyberpunk_improvement') {
          return sheet;
        }
        throw new Error(`unexpected command ${cmd}`);
      },
      transformCallback: () => 0,
    },
  };
  try {
    const wire = JSON.stringify({
      request: {
        characterId: 'cpr-solo-kitted',
        ipAwarded: 200,
        purchases: [
          { kind: 'skill', key: 'Handgun', to: 7 },
          { kind: 'roleRank', to: 5 },
          { kind: 'roleSwitch', role: 'Nomad' },
        ],
      },
    });
    assertEqual(await previewCyberpunkImprovement('cpr-solo-kitted', plan), preview, 'the preview is the answer');
    assertEqual(await applyCyberpunkImprovement('cpr-solo-kitted', plan), sheet, 'the apply answer is the reloaded sheet');
    assertEqual(calls.length, 2, 'two IPC calls');
    assertEqual(calls[0].cmd, 'preview_cyberpunk_improvement', 'the preview command registered in main.rs');
    assertEqual(calls[1].cmd, 'apply_cyberpunk_improvement', 'the apply command registered in main.rs');
    assertEqual(JSON.stringify(calls[0].args), wire, 'the preview request is the shape CprImproveRequest deserialises');
    assertEqual(JSON.stringify(calls[1].args), wire, 'the apply request is the same shape');

    // E5.3: a cyberware install (entered or blank loss) and the pinned seed travel as CprImproveRequest deserialises them.
    const install = {
      ipAwarded: 0,
      seed: 4242,
      purchases: [
        { kind: 'cyberware' as const, name: 'Gills', humanityLoss: 9 },
        { kind: 'cyberware' as const, name: 'Nasal Filters', humanityLoss: null },
      ],
    };
    await previewCyberpunkImprovement('cpr-solo-kitted', install);
    assertEqual(
      JSON.stringify(calls[2].args),
      JSON.stringify({
        request: {
          characterId: 'cpr-solo-kitted',
          ipAwarded: 0,
          purchases: [
            { kind: 'cyberware', name: 'Gills', humanityLoss: 9 },
            { kind: 'cyberware', name: 'Nasal Filters', humanityLoss: null },
          ],
          seed: 4242,
        },
      }),
      'an install and the seed are the shape CprImproveRequest deserialises',
    );

    // E5.4: a therapy (entered or blank regain) travels as CprImproveRequest deserialises it.
    await previewCyberpunkImprovement('cpr-solo-kitted', {
      ipAwarded: 0,
      purchases: [
        { kind: 'therapy' as const, name: 'Standard Humanity Loss', regained: 7 },
        { kind: 'therapy' as const, name: 'Extreme Humanity Loss', regained: null },
      ],
    });
    assertEqual(
      JSON.stringify(calls[3].args),
      JSON.stringify({
        request: {
          characterId: 'cpr-solo-kitted',
          ipAwarded: 0,
          purchases: [
            { kind: 'therapy', name: 'Standard Humanity Loss', regained: 7 },
            { kind: 'therapy', name: 'Extreme Humanity Loss', regained: null },
          ],
        },
      }),
      'a therapy is the shape CprImproveRequest deserialises',
    );

    (globals.window as { __TAURI_INTERNALS__: { invoke: unknown } }).__TAURI_INTERNALS__.invoke = async () => {
      throw 'Overspend: these improvements cost 140 IP, but 90 IP are available';
    };
    assertEqual(
      await messageOf(() => applyCyberpunkImprovement('cpr-solo-kitted', plan)),
      'Failed to spend Cyberpunk RED Improvement Points: Overspend: these improvements cost 140 IP, but 90 IP are available',
      'the engine refusal is passed on',
    );
  } finally {
    delete globals.window;
  }
  console.log('cprImprove: preview/apply request shape (skill, rank, switch, cyberware, therapy), answers and refusal checked');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
