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
- rec:4aec710b attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:6a88a647 decision: New optional parameters of `parse_quantity` are keyword-only (after *)…
- rec:f304554e decision: Tests for `TallyError` build their inputs with small factory helpers i…
- rec:80370454 decision: Event payloads carry money as integer minor units plus the currency co…

## Code
- sym:2871 Money.allocate (tallyho/money.py:63-68)
- sym:2863 Money (tallyho/money.py:22-83)
  class Money:
- sym:2893 Quote (tallyho/pricing/quote.py:16-27)
- sym:2894 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:2888 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:2830 ValidationError (tallyho/errors.py:8-17)
- sym:2854 Order (tallyho/models.py:64-109)
- sym:2866 Money._check (tallyho/money.py:39-43)
- sym:2841 Ledger (tallyho/ledger.py:24-53)
- sym:2887 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:2868 Money.__sub__ (tallyho/money.py:49-51)

---
tokens: 713/800
omitted 56 lower-ranked items (expand an id for detail):
- rec:b95d3331 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:4ffc393c attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:bcbae76a attempt: Wrapping `Discount` in a bare try/except with a retry loop hid the rea…
- rec:6e98c192 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:4cbab36d attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:b8031c2e attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- … and 50 more
