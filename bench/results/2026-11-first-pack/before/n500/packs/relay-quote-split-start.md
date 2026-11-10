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
- rec:460ed814 decision: Errors about customer data must not echo the rejected value: they are…

## Code
- sym:4531 Money.allocate (tallyho/money.py:63-68)
- sym:4548 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:4490 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:4523 Money (tallyho/money.py:22-83)
  class Money:
- sym:4501 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:4547 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:4553 Quote (tallyho/pricing/quote.py:16-27)
- sym:4554 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:4489 TallyError (tallyho/errors.py:4-5)
- sym:4502 Ledger.post (tallyho/ledger.py:28-35)
- sym:4508 Product.__post_init__ (tallyho/models.py:30-35)

---
tokens: 710/800
omitted 165 lower-ranked items (expand an id for detail):
- rec:94926a8a attempt: Adding type: ignore comments around `check_email` to quiet the checker…
- rec:2cdf251d attempt: Wrapping `Money.allocate` in a bare try/except with a retry loop hid t…
- rec:88401dda attempt: Tried calling `check_email` twice to force a refresh; the second call…
- rec:ccc57e87 attempt: Tried calling `JsonFileStore._path` twice to force a refresh; the seco…
- rec:68251fa1 attempt: Tried calling `Coupon.discount` twice to force a refresh; the second c…
- rec:ba877cff attempt: Wrapping the `errors` module in a bare try/except with a retry loop hi…
- … and 159 more
