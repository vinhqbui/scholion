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
- rec:e7d91138 decision: Errors about customer data must not echo the rejected value: they are…
- rec:31bfff37 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:aab95bbe decision: Event payloads carry money as integer minor units plus the currency co…
- rec:03e98f60 decision: New optional parameters of `Repository.product` are keyword-only (afte…

## Code
- sym:1958 check_region (tallyho/validate.py:24-26)
- sym:1956 check_sku (tallyho/validate.py:14-16)
- sym:1852 Product.__post_init__ (tallyho/models.py:30-35)
- sym:1960 parse_quantity (tallyho/validate.py:34-42)
- sym:1957 check_email (tallyho/validate.py:19-21)
- sym:1959 require_positive (tallyho/validate.py:29-31)
- sym:1834 ValidationError (tallyho/errors.py:8-17)
- sym:1851 Product (tallyho/models.py:23-35)
- sym:1858 Order (tallyho/models.py:64-109)
- sym:1955 _REGION (tallyho/validate.py:11-11)
- sym:1853 Customer (tallyho/models.py:39-47)
- sym:1931 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:1860 Order.add (tallyho/models.py:76-84)

---
tokens: 712/800
omitted 54 lower-ranked items (expand an id for detail):
- rec:760cebb7 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:f99390fb attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:2b73bc24 attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- rec:8b42ff23 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:3deea274 note: The `scan` module reads files as UTF-8 explicitly; the platform defaul…
- rec:694a897b note: The `cli` module reads files as UTF-8 explicitly; the platform default…
- … and 48 more
