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
- rec:44f579ac decision: Errors about customer data must not echo the rejected value: they are…
- rec:10d7e824 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:15608370 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:be93be26 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…

## Code
- sym:298 check_region (tallyho/validate.py:24-26)
- sym:296 check_sku (tallyho/validate.py:14-16)
- sym:192 Product.__post_init__ (tallyho/models.py:30-35)
- sym:300 parse_quantity (tallyho/validate.py:34-42)
- sym:297 check_email (tallyho/validate.py:19-21)
- sym:299 require_positive (tallyho/validate.py:29-31)
- sym:174 ValidationError (tallyho/errors.py:8-17)
- sym:191 Product (tallyho/models.py:23-35)
- sym:198 Order (tallyho/models.py:64-109)
- sym:295 _REGION (tallyho/validate.py:11-11)
- sym:193 Customer (tallyho/models.py:39-47)
- sym:271 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:178 NotFound (tallyho/errors.py:24-25)

---
tokens: 716/800
omitted 37 lower-ranked items (expand an id for detail):
- rec:10fb5486 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:eb335e9a decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:cb6463f2 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:31a199f1 intent: Earlier work on this code
- sym:293 _SKU (tallyho/validate.py:9-9)
- sym:176 ValidationError.__str__ (tallyho/errors.py:15-17)
- sym:173 TallyError (tallyho/errors.py:4-5)
- sym:186 Ledger.post (tallyho/ledger.py:28-35)
- … and 29 more
