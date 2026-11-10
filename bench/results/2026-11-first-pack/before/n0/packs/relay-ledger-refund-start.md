## Task
# relay-ledger-refund · relay-ledger-refund · open

## Map
- tallyho/ledger.py
- tallyho/pricing/quote.py
- tallyho/models.py
- tallyho/money.py
- tallyho/pricing/tax.py
- tallyho/workflow.py
- tallyho/config.py
- tallyho/errors.py
- tallyho/pricing/coupons.py
- tallyho/pricing/discounts.py
- tallyho/report/csv_export.py
- tallyho/repository.py
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)

## Decisions & dead ends
- rec:a3bac6fb decision: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:4d501dd2 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:8b0f7f30 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…

## Code
- sym:353 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:356 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:351 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:472 Workflow.pay (tallyho/workflow.py:41-44)
- sym:352 Ledger.post (tallyho/ledger.py:28-35)
- sym:405 quote_order (tallyho/pricing/quote.py:30-53)
- sym:403 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:364 Order (tallyho/models.py:64-109)
  class Order:
- sym:354 Ledger.balances (tallyho/ledger.py:46-50)
- sym:350 Entry (tallyho/ledger.py:15-20)
  class Entry:
- sym:344 NotFound (tallyho/errors.py:24-25)
- sym:468 Workflow (tallyho/workflow.py:23-50)
- sym:467 _ALLOWED (tallyho/workflow.py:14-20)

---
tokens: 716/800
omitted 36 lower-ranked items (expand an id for detail):
- rec:135a1205 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:c2e5487d attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- sym:365 Order.currency (tallyho/models.py:73-74)
- sym:373 Money (tallyho/money.py:22-83)
- sym:375 Money.zero (tallyho/money.py:36-37)
- sym:386 total (tallyho/money.py:86-91)
- sym:408 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:338 load_config (tallyho/config.py:23-47)
- … and 28 more
