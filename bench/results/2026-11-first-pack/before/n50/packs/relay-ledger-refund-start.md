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
- rec:70411550 decision: `Ledger.record_sale` keeps its docstring as the one place its behaviour is described; the README only links to it.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:743a53ba decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:36e3f442 decision: Constants that `Ledger.record_sale` depends on live at the top of its module in UPPER_CASE, not as magic numbers inline.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:3337cb80 attempt: Moving the imports inside `Money.zero` to speed up start-up saved unde…

## Code
- sym:2013 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:2016 OrderStatus (tallyho/models.py:14-19)
- sym:2011 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:2132 Workflow.pay (tallyho/workflow.py:41-44)
- sym:2012 Ledger.post (tallyho/ledger.py:28-35)
- sym:2065 quote_order (tallyho/pricing/quote.py:30-53)
- sym:2063 Quote (tallyho/pricing/quote.py:16-27)
- sym:2024 Order (tallyho/models.py:64-109)
- sym:2014 Ledger.balances (tallyho/ledger.py:46-50)
- sym:2010 Entry (tallyho/ledger.py:15-20)
- sym:2004 NotFound (tallyho/errors.py:24-25)
- sym:2128 Workflow (tallyho/workflow.py:23-50)

---
tokens: 713/800
omitted 50 lower-ranked items (expand an id for detail):
- rec:43596dbf note: Benchmarks for `Ledger.record_sale` are in a separate, manual job; the…
- rec:3d68f208 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:98858efa attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:6bc0c404 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:12d9c8a3 decision: `Workflow.pay` keeps its docstring as the one place its behaviour is d…
- rec:b9d975f5 decision: The `ledger` module keeps its docstring as the one place its behaviour…
- … and 44 more
