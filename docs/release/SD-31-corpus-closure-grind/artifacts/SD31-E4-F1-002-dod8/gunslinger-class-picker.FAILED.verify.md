# DoD-8 attempt: Gunslinger on-screen (blocked at character creation, not faked)

**Cycle:** SD31-E4-F1-002 (RETRO_ACTOR=sd31-e4-classwire2)
**Time (UTC):** see file mtimes on the PNGs in this directory
**RUN_DESKTOP_AGENT:** sd31-e4-classwire2

## What was attempted

1. `apps/desktop/.claude/skills/run-desktop/driver.sh launch` (RUN_DESKTOP_AGENT unique to this
   cycle) — app came up, window titled "Codex".
2. Hub screenshot (`01-hub.png`) — confirms the app is live and rendering.
3. Clicked "New Character" (958,279) — `02-create-form.png` shows the real Create Character form,
   with a Class `<select>` currently defaulted to "Fighter".
4. Clicked the Class dropdown (943,263) — `03-class-dropdown-open.png`. The dropdown's rendered
   options paint as a solid black box under this Xvfb/WebKitGTK environment (a known rendering
   quirk for native `<select>` popups here, not evidence either way about content).
5. Typed `Gunslinger` while the dropdown was open (`04-typed-gunslinger.png`) — native `<select>`
   elements jump to the first option matching typed text. The visible field (still showing
   "Fighter" above the dropdown) did not change, and after closing the dropdown
   (`05-dropdown-closed.png`) the Class field still reads "Fighter" — i.e. no option beginning
   with "Gunslinger" exists for the type-ahead to jump to.

## Why this is not fabricated evidence

This live-interaction result is exactly what the static source read already predicted
(`OPEN-ISSUES.md` row 95): `apps/desktop/src/characterHub/characterHubModel.ts`'s
`CLASS_OPTIONS` array — the literal, exhaustive list the Class `<select>` is built from
(`CreateCharacterForm.tsx`) — has no Gunslinger entry (`grep -rn gunslinger
apps/desktop/src/` returns zero hits anywhere in the frontend). The type-ahead attempt is the
closer-to-the-metal confirmation: a real user interaction against the real running app, not an
inference from reading code alone.

## What this does NOT test

Reachability of the actual computation is proven a different, real way — 11 new
`build_pilot_headless_receipt`-based tests in `src/rules_core/pilot_compute.rs`
(`gunslinger_tests` module), exercising the production `compute_uc_class_chassis`/
`ground_or_block_gunslinger_class_features`/`archetype_claiming_slot_entry` path end to end. That
is SD31-E4-F1's own named acceptance standard for reachability ("a headless pilot receipt test
through `build_pilot_headless_receipt`, not a unit test on the resolver alone") and it is met. This
file documents only the SEPARATE, honest gap: the desktop UI's character-creation flow cannot
reach Gunslinger yet, so a full on-screen character-sheet render (Decision 7 condition 3's
DoD-8 bar) is blocked until a UI-territory cycle adds Gunslinger (and any other newly-wired
non-CRB/APG/ACG class) to `CLASS_OPTIONS`.

## Files

- `01-hub.png` — app launched, hub renders.
- `02-create-form.png` — Create Character form renders, Class defaults to Fighter.
- `03-class-dropdown-open.png` — dropdown opened (renders black under Xvfb, a harness quirk).
- `04-typed-gunslinger.png` — typed "Gunslinger" into the open native select; no visible change.
- `05-dropdown-closed.png` — dropdown closed, Class field still reads "Fighter" — no Gunslinger
  option exists to select.

Named `.FAILED.` per this program's own convention (`verify-on-screen.sh`'s naming rule) so this
cannot be mistaken for a passing on-screen proof.
