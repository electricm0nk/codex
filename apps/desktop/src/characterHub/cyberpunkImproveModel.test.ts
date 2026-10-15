import type { CyberpunkImprovePreview, CyberpunkImproveSkillOption } from '../boundary/cprImprove';
import {
  EMPTY_CYBERPUNK_IMPROVE_PLAN,
  addPurchase,
  canSpend,
  improvePlanFromIpText,
  improveTotals,
  nextTarget,
  purchaseCost,
  purchaseLabel,
  removePurchase,
  skillPurchaseKey,
  switchOptionLabel,
} from './cyberpunkImproveModel';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * SD-38 E5.1: the improve dialog's model. It builds the plan the engine prices (IP entered,
 * purchases one Level/Rank at a time) and reads every number it shows from the engine's preview
 * rows (`cpr.improve.purchase.<i>.cost`, `cpr.improve.ip.available|cost|remaining`, each option's
 * `next` row) -- it adds, subtracts and prices nothing itself. Prices, skipped-Level and overspend
 * refusals are the engine's (`cpr_improve.rs` tests).
 */

const handgun: CyberpunkImproveSkillOption = {
  key: 'Handgun',
  name: 'Handgun',
  category: 'Ranged Weapon Skills',
  x2: false,
  specialtyRequired: false,
  level: 6,
  next: { id: 'cpr.improve.skill.handgun.next', value: 140, detail: '140 IP' },
};
const language: CyberpunkImproveSkillOption = {
  key: 'Language',
  name: 'Language',
  category: 'Education Skills',
  x2: false,
  specialtyRequired: true,
  level: 0,
  next: { id: 'cpr.improve.skill.language.next', value: 20, detail: '20 IP' },
};

// 1. IP entered: digits only; empty is 0; anything else is refused with a reason and keeps the plan.
let plan = EMPTY_CYBERPUNK_IMPROVE_PLAN;
assertEqual(improvePlanFromIpText(plan, '200').plan.ipAwarded, 200, '200 entered');
assertEqual(improvePlanFromIpText(plan, '').plan.ipAwarded, 0, 'empty is 0');
assertEqual(improvePlanFromIpText(plan, '-5').error, 'IP entered must be a whole number of 0 or more', 'negative refused');
assertEqual(improvePlanFromIpText(plan, '1.5').plan, plan, 'a refused entry keeps the plan');
plan = improvePlanFromIpText(plan, '200').plan;

// 2. Keys: a skill that needs a specialty is bought as `<name> (<specialty>)`; none named -> null.
assertEqual(skillPurchaseKey(handgun, ''), 'Handgun', 'a plain skill is its own key');
assertEqual(skillPurchaseKey(language, ' Spanish '), 'Language (Spanish)', 'specialty joined');
assertEqual(skillPurchaseKey(language, '  '), null, 'a specialty skill with no specialty has no key');

// 3. The next target is one Level above the option's engine Level, or above the plan's last purchase of that key.
assertEqual(nextTarget(plan, { kind: 'skill', key: 'Handgun' }, handgun.level), 7, 'Handgun 6 -> 7');
plan = addPurchase(plan, { kind: 'skill', key: 'Handgun', to: 7 });
assertEqual(nextTarget(plan, { kind: 'skill', key: 'Handgun' }, handgun.level), 8, 'after 7 is planned, 8');
assertEqual(nextTarget(plan, { kind: 'roleRank' }, 4), 5, 'Rank 4 -> 5');
plan = addPurchase(plan, { kind: 'roleRank', to: 5 });
assertEqual(plan.purchases.length, 2, 'two purchases');
assertEqual(purchaseLabel(plan.purchases[0]), 'Handgun to Level 7', 'skill label');
assertEqual(purchaseLabel(plan.purchases[1]), 'Role Ability to Rank 5', 'rank label');

// 3b. A Role switch (E5.2, p. 143): the new Role starts at Rank 1, so a Rank purchase after it asks for Rank 2 of the new Role.
let switched = addPurchase(addPurchase(EMPTY_CYBERPUNK_IMPROVE_PLAN, { kind: 'roleSwitch', role: 'Nomad' }), { kind: 'roleRank', to: 2 });
assertEqual(nextTarget(addPurchase(EMPTY_CYBERPUNK_IMPROVE_PLAN, { kind: 'roleSwitch', role: 'Nomad' }), { kind: 'roleRank' }, 4), 2, 'after a switch, Rank 2');
assertEqual(nextTarget(switched, { kind: 'roleRank' }, 4), 3, 'after the switch and Rank 2, Rank 3');
assertEqual(purchaseLabel(switched.purchases[0]), 'Switch Role to Nomad (Rank 1)', 'switch label');
assertEqual(purchaseLabel(switched.purchases[1]), 'Role Ability to Rank 2', 'the rank after the switch');
const switchOptions = [
  { role: 'Nomad', cost: { id: 'cpr.improve.switch.nomad.cost', value: 60, detail: '60 IP: Rank 1 in the Nomad Role (A-1)' }, refusal: null },
  { role: 'Fixer', cost: null, refusal: 'cannot change Roles: the latest Role, Solo, is at Rank 3; it must reach Rank 4 first (p. 143)' },
];
assertEqual(switchOptionLabel(switchOptions[0]), 'Nomad — Rank 1 for 60 IP', 'a priced switch option shows the engine price');
assertEqual(switchOptionLabel(switchOptions[1]), 'Fixer — locked (p. 143)', 'a locked option is marked; the engine refusal says why');

// 4. Every figure the dialog shows is an engine row, found by id.
const preview: CyberpunkImprovePreview = {
  skills: [handgun, language],
  role: { name: 'Solo', rank: 4, next: { id: 'cpr.improve.role.solo.next', value: 300, detail: '300 IP' } },
  switch: switchOptions,
  rows: [
    { id: 'cpr.improve.purchase.0.cost', value: 1140, detail: 'Handgun' },
    { id: 'cpr.improve.purchase.1.cost', value: 1300, detail: 'Solo' },
    { id: 'cpr.improve.ip.available', value: 1200, detail: 'available' },
    { id: 'cpr.improve.ip.cost', value: 1440, detail: 'cost' },
    { id: 'cpr.improve.ip.remaining', value: 1060, detail: 'remaining' },
  ],
  refusal: null,
};
assertEqual(purchaseCost(preview, 0)?.value, 1140, 'purchase 0 cost is its row');
assertEqual(purchaseCost(preview, 1)?.value, 1300, 'purchase 1 cost is its row');
assertEqual(purchaseCost(preview, 2), null, 'no row, no cost');
const totals = improveTotals(preview);
assertEqual(totals.available?.value, 1200, 'available is its row');
assertEqual(totals.cost?.value, 1440, 'cost is its row (not 1140 + 1300)');
assertEqual(totals.remaining?.value, 1060, 'remaining is its row (not 1200 - 1440)');

// 5. Spend is offered only for a priced, unrefused plan with something in it.
assert(canSpend(plan, preview), 'a priced plan can be spent');
assert(!canSpend(plan, { ...preview, refusal: 'Overspend' }), 'a refused plan cannot');
assert(!canSpend(plan, null), 'an unpriced plan cannot');
assert(!canSpend(EMPTY_CYBERPUNK_IMPROVE_PLAN, preview), 'an empty plan cannot');

// 6. Removing a purchase drops it and keeps the order of the rest.
const removed = removePurchase(plan, 0);
assertEqual(removed.purchases.length, 1, 'one left');
assertEqual(purchaseLabel(removed.purchases[0]), 'Role Ability to Rank 5', 'the rank purchase stays');

console.log('cyberpunkImproveModel: IP entry, keys, targets, Role switch, engine-row figures and spend gate checked');
