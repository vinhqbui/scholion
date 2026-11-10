## Task
# relay-quote-split · relay-quote-split · open
Goal: Customers want to split a bill evenly between several payers. Add `Quote.split(parts)`, returning what each of the `parts` payers pays, as a list of Money. Raise ValidationError when parts is less than 1.

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
- rec:6e98c192 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:4cbab36d attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:b8031c2e attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…

## Code
- sym:2841 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:2871 Money.allocate (tallyho/money.py:63-68)
- sym:2842 Ledger.post (tallyho/ledger.py:28-35)
- sym:2888 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:2830 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:2854 Order (tallyho/models.py:64-109)
- sym:2863 Money (tallyho/money.py:22-83)
- sym:2833 CurrencyMismatch (tallyho/errors.py:20-21)
- sym:2847 Product (tallyho/models.py:23-35)
- sym:2851 OrderLine (tallyho/models.py:51-60)
- sym:2887 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:2893 Quote (tallyho/pricing/quote.py:16-27)

---
tokens: 709/800
omitted 52 lower-ranked items (expand an id for detail):
- rec:4aec710b attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:f14bba5d decision: Errors about customer data must not echo the rejected value: they are…
- rec:f304554e decision: Tests for `TallyError` build their inputs with small factory helpers i…
- rec:6a88a647 decision: New optional parameters of `parse_quantity` are keyword-only (after *)…
- rec:b1ec796e decision: `_exponent` keeps its docstring as the one place its behaviour is desc…
- rec:f31f5d73 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- … and 46 more
