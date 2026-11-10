## Task
# relay-customer-region · relay-customer-region · open
Goal: Customer regions are never checked, so typos such as "us-ca" end up in the store. Validate the region when a Customer is created and raise ValidationError (with field "region") for an invalid one.

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
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)

## Decisions & dead ends
- rec:760cebb7 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:f99390fb attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:2b73bc24 attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- rec:8b42ff23 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…

## Code
- sym:1852 Product.__post_init__ (tallyho/models.py:30-35)
- sym:1834 ValidationError (tallyho/errors.py:8-17)
- sym:1958 check_region (tallyho/validate.py:24-26)
- sym:1960 parse_quantity (tallyho/validate.py:34-42)
- sym:1956 check_sku (tallyho/validate.py:14-16)
- sym:1957 check_email (tallyho/validate.py:19-21)
- sym:1959 require_positive (tallyho/validate.py:29-31)
- sym:1851 Product (tallyho/models.py:23-35)
  class Product:
- sym:1838 NotFound (tallyho/errors.py:24-25)
- sym:1858 Order (tallyho/models.py:64-109)
  class Order:
- sym:1853 Customer (tallyho/models.py:39-47)
- sym:1836 ValidationError.__str__ (tallyho/errors.py:15-17)

---
tokens: 708/800
omitted 49 lower-ranked items (expand an id for detail):
- rec:e7d91138 decision: Errors about customer data must not echo the rejected value: they are…
- rec:910979d5 decision: Tests for `TallyError` build their inputs with small factory helpers i…
- rec:5193ca67 decision: New optional parameters of `parse_quantity` are keyword-only (after *)…
- rec:bc9c9e3f decision: `_exponent` keeps its docstring as the one place its behaviour is desc…
- rec:3b7edfeb attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:d43e939d attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- … and 43 more
