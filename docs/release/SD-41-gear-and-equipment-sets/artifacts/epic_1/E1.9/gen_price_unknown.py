"""Writes data/price_overrides/price-unknown.json from the Market census E1.9's acceptance command
writes (artifacts/epic_1/E1.8/market-prices.json): one { key, reason } per Market key still without a
positive price after the converter fix. E1.11 removes each key it prices and adds `tried` to the rest.
Run from the repo root after the census: python3 -I <this file>"""
import json
census = json.load(open("docs/release/SD-41-gear-and-equipment-sets/artifacts/epic_1/E1.8/market-prices.json"))
REASON = {
    "cost_literal_zero": "PCGen states COST:0 and no eqmod; awaiting the d20pfsrd lookup (E1.11)",
    "cost_offset_by_eqmod": "PCGen's literal COST plus its eqmods sums to 0; awaiting the d20pfsrd lookup (E1.11)",
    "price_in_eqmods": "an eqmod cost the converter does not evaluate (SIZE formula, an unread ITEMCOST prerequisite) or sums to 0; awaiting the d20pfsrd lookup (E1.11)",
    "composite_resolved_to_base": "the .COPY= base row is not in the pinned oracle's equipment files, or the composite sums to 0; awaiting the d20pfsrd lookup (E1.11)",
    "no_cost_token": "the record states no price in PCGen; awaiting the d20pfsrd lookup (E1.11)",
    "no_raw_tokens": "the record states no price in PCGen; awaiting the d20pfsrd lookup (E1.11)",
    "table_disagrees_with_corpus": "the Market's table row is unpriced while another book's record of this key is priced; awaiting the d20pfsrd lookup (E1.11)",
    "retirement_directive_row": "a PCGen removal directive, not an item (E2.8 hides it from the Market)",
}
rows = {}
for kind in ("none", "zero", "negative"):
    for r in census[kind]:
        rows.setdefault(r["key"], {"key": r["key"], "reason": REASON[r["cause"]]})
out = sorted(rows.values(), key=lambda r: r["key"])
with open("data/price_overrides/price-unknown.json", "w") as f:
    json.dump(out, f, indent=2, ensure_ascii=False)
    f.write("\n")
print(len(out))
