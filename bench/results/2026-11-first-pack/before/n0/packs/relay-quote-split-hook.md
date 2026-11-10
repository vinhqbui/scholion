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
- rec:4c6d1985 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:ca794c23 decision: Errors about customer data must not echo the rejected value: they are…
- rec:23adcf84 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…

## Code
- sym:1211 Money.allocate (tallyho/money.py:63-68)
- sym:1203 Money (tallyho/money.py:22-83)
- sym:1233 Quote (tallyho/pricing/quote.py:16-27)
- sym:1234 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:1228 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:1170 ValidationError (tallyho/errors.py:8-17)
- sym:1194 Order (tallyho/models.py:64-109)
- sym:1206 Money._check (tallyho/money.py:39-43)
- sym:1181 Ledger (tallyho/ledger.py:24-53)
- sym:1227 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:1208 Money.__sub__ (tallyho/money.py:49-51)
- sym:1207 Money.__add__ (tallyho/money.py:45-47)

---
tokens: 718/800
omitted 42 lower-ranked items (expand an id for detail):
- rec:f2b88ebf attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:92381957 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:637e294e decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:97fcb48e decision: Event payloads carry money as integer minor units plus the currency co…
- rec:c272cb99 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- sym:1169 TallyError (tallyho/errors.py:4-5)
- sym:1182 Ledger.post (tallyho/ledger.py:28-35)
- … and 35 more
