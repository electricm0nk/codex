import { assert } from '../testSupport/asserts';
import { COMMUNITY_THEME_BRIDGE_CSS } from './communityTheme';

// Brief 1 F1 (tranche-17-ui): Obsidian themes mark note headings with generated text -- the built-in
// Ayu Mirage theme's obsidian.css has `h2:before { content: "H2 "; }` -- which put "H2" before every
// app heading ("H2 Create a character"). The bridge stylesheet, injected with every community theme,
// must switch those markers off for all six heading levels while a theme is active.
function verifiesTheBridgeSwitchesOffThemeHeadingMarkers() {
  const css = COMMUNITY_THEME_BRIDGE_CSS.replace(/\s+/g, ' ');
  for (let level = 1; level <= 6; level += 1) {
    for (const mode of ['theme-dark', 'theme-light']) {
      assert(css.includes(`body.${mode} h${level}::before`), `h${level}::before is covered under body.${mode}`);
    }
  }
  assert(/::before[^{]*\{ content: none !important; \}/.test(css), 'and its content is removed, outranking the theme');
}

verifiesTheBridgeSwitchesOffThemeHeadingMarkers();
console.log('communityTheme.test.ts: all assertions passed');
