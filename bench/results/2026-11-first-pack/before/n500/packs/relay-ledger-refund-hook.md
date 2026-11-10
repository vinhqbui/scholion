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
- rec:19ce5c3b attempt: Tried calling `Ledger.record_sale` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:fd759568 attempt: Caching what `Ledger.record_sale` returns in a module-level dict broke tests that expect a fresh value per case.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:3cfa3d77 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:f28bd332 attempt: Moving the imports inside `Ledger.record_sale` to speed up start-up sa…

## Code
- sym:3673 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:3792 Workflow.pay (tallyho/workflow.py:41-44)
- sym:3671 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:3725 quote_order (tallyho/pricing/quote.py:30-53)
- sym:3723 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:3676 OrderStatus (tallyho/models.py:14-19)
- sym:3788 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:3753 RecordStore.get (tallyho/store/base.py:28-29)
- sym:3748 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:3664 NotFound (tallyho/errors.py:24-25)
  class NotFound(TallyError):
- sym:3787 _ALLOWED (tallyho/workflow.py:14-20)

---
tokens: 722/800
omitted 203 lower-ranked items (expand an id for detail):
- rec:a6fe04cf decision: New optional parameters of `Ledger.record_sale` are keyword-only (afte…
- rec:a38c04c3 decision: Constants that `Ledger.record_sale` depends on live at the top of its…
- rec:00f03ae4 decision: `Ledger.record_sale` keeps its docstring as the one place its behaviou…
- rec:8f8968c0 note: Benchmarks for `Ledger.record_sale` are in a separate, manual job; the…
- rec:27532e8e attempt: Wrapping `Quote` in a bare try/except with a retry loop hid the real e…
- rec:f99fdd1e decision: `Quote` keeps its docstring as the one place its behaviour is describe…
- … and 197 more
