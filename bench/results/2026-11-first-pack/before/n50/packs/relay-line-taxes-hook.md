## Task
# relay-line-taxes · relay-line-taxes · open

## Map
- tallyho/pricing/quote.py
- tallyho/models.py
- tallyho/money.py
- tallyho/pricing/coupons.py
- tallyho/pricing/discounts.py
- tallyho/pricing/tax.py
- tallyho/report/csv_export.py
- tallyho/repository.py
- tallyho/workflow.py
- tests/conftest.py
- tests/test_pricing.py
- tallyho/cli.py
- tallyho/config.py
- tallyho/ledger.py

## Decisions & dead ends
- rec:c66b79c9 attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:79c6416e attempt: Caching what `Discount.amount_off` returns in a module-level dict brok…
- rec:7edef53d decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:88b28b08 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…

## Code
- sym:2231 quote_order (tallyho/pricing/quote.py:30-53)
- sym:2230 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:2229 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:2190 Order (tallyho/models.py:64-109)
- sym:2192 Order.add (tallyho/models.py:76-84)
- sym:2234 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:2193 Order.remove (tallyho/models.py:86-89)
- sym:2183 Product (tallyho/models.py:23-35)
- sym:2245 Repository (tallyho/repository.py:32-74)
- sym:2314 test_quote_taxes_only_taxable (tests/test_pricing.py:20-24)
- sym:2189 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:2191 Order.currency (tallyho/models.py:73-74)

---
tokens: 709/800
omitted 50 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:84259605 decision: New optional parameters of the `discounts` module are keyword-only (af…
- rec:f7a994fc note: `TaxTable.tax` is on the hot path of the nightly batch job; profile be…
- rec:464f3149 note: Benchmarks for the `tax` module are in a separate, manual job; the uni…
- rec:916c1ae4 attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:c1c8d594 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- … and 44 more
