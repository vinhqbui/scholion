## Task
# relay-quote-split · relay-quote-split · open

## Map
- tallyho/errors.py
- tallyho/ledger.py
- tallyho/models.py
- tallyho/money.py
- tallyho/pricing/coupons.py
- tallyho/pricing/discounts.py
- tallyho/store/jsonfile.py
- tallyho/store/memory.py
- tallyho/validate.py
- tallyho/workflow.py
- tests/test_models.py
- tallyho/events.py
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:c104d1be attempt: Wrapping `ValidationError` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/errors.py:ValidationError
- rec:8317d323 decision: New optional parameters of `ValidationError` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/errors.py:ValidationError
- rec:6453f7f0 note: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:e19d2679 decision: Tests for `Money.format` build their inputs with small factory helpers…

## Code
- sym:4531 Money.allocate (tallyho/money.py:63-68)
- sym:4523 Money (tallyho/money.py:22-83)
  class Money:
- sym:4553 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:4554 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:4548 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:4490 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:4514 Order (tallyho/models.py:64-109)
- sym:4526 Money._check (tallyho/money.py:39-43)
- sym:4501 Ledger (tallyho/ledger.py:24-53)
- sym:4547 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:4528 Money.__sub__ (tallyho/money.py:49-51)

---
tokens: 714/800
omitted 171 lower-ranked items (expand an id for detail):
- rec:ef53efe5 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:2cdf251d attempt: Wrapping `Money.allocate` in a bare try/except with a retry loop hid t…
- rec:b8dcc950 attempt: Wrapping `Order.currency` in a bare try/except with a retry loop hid t…
- rec:359fd74c attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:4a88f587 decision: Tests for `Money._check` build their inputs with small factory helpers…
- rec:94926a8a attempt: Adding type: ignore comments around `check_email` to quiet the checker…
- … and 165 more
