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
- rec:f7c14307 attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:4d9fa9b8 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:1f2cd74d decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:3e2bdd81 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…

## Code
- sym:530 Order (tallyho/models.py:64-109)
  class Order:
- sym:571 quote_order (tallyho/pricing/quote.py:30-53)
- sym:570 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:569 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:529 OrderLine.subtotal (tallyho/models.py:59-60)
  def subtotal(self) -> Money:
- sym:523 Product (tallyho/models.py:23-35)
- sym:519 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:533 Order.remove (tallyho/models.py:86-89)
- sym:532 Order.add (tallyho/models.py:76-84)
- sym:500 cmd_products (tallyho/cli.py:20-23)
- sym:527 OrderLine (tallyho/models.py:51-60)
- sym:524 Product.__post_init__ (tallyho/models.py:30-35)

---
tokens: 704/800
omitted 35 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- sym:528 OrderLine.__post_init__ (tallyho/models.py:55-56)
- sym:585 Repository (tallyho/repository.py:32-74)
- sym:531 Order.currency (tallyho/models.py:73-74)
- sym:534 Order.subtotal (tallyho/models.py:92-93)
- sym:539 Money (tallyho/money.py:22-83)
- sym:552 total (tallyho/money.py:86-91)
- sym:556 CouponBook (tallyho/pricing/coupons.py:40-63)
- … and 27 more
