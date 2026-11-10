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
- rec:743a53ba decision: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:36e3f442 decision: Constants that `Ledger.record_sale` depends on live at the top of its…
- rec:70411550 decision: `Ledger.record_sale` keeps its docstring as the one place its behaviou…
- rec:3337cb80 attempt: Moving the imports inside `Money.zero` to speed up start-up saved unde…

## Code
- sym:2013 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:2132 Workflow.pay (tallyho/workflow.py:41-44)
- sym:2011 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:2065 quote_order (tallyho/pricing/quote.py:30-53)
- sym:2063 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:2016 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:2128 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:2093 RecordStore.get (tallyho/store/base.py:28-29)
- sym:2088 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:2004 NotFound (tallyho/errors.py:24-25)
  class NotFound(TallyError):
- sym:2127 _ALLOWED (tallyho/workflow.py:14-20)

---
tokens: 720/800
omitted 58 lower-ranked items (expand an id for detail):
- rec:43596dbf note: Benchmarks for `Ledger.record_sale` are in a separate, manual job; the…
- rec:de48ad3d attempt: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…
- rec:b2ecf64e attempt: Wrapping `RecordStore.list_page` in a bare try/except with a retry loo…
- rec:ba52fced attempt: Wrapping `count_records` in a bare try/except with a retry loop hid th…
- rec:3d68f208 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:98858efa attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- … and 52 more
