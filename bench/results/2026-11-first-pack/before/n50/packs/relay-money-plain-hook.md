## Task
# relay-money-plain · relay-money-plain · open
Goal: Report tables want amounts without the currency code. Add `Money.format_plain()`, returning the amount as text without the currency code, for example "12.34" for 12.34 USD.

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:8b29e7df attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:e52a66bf attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:ba9b6653 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:6bdb0802 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:176d9741 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:72f773a2 decision: New optional parameters of `parse_quantity` are keyword-only (after *)…
- rec:1fafdb62 decision: Constants that `Ledger.record_sale` depends on live at the top of its…

## Code
- sym:2376 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:2378 total (tallyho/money.py:86-91)
  def total(amounts, currency: str = "USD") -> Money:
- sym:2366 Money.of (tallyho/money.py:29-33)
  def of(cls, amount: str | int | Decimal, currency: str = "USD") -> Money:
- sym:2365 Money (tallyho/money.py:22-83)
  class Money:
- sym:2367 Money.zero (tallyho/money.py:36-37)
  def zero(cls, currency: str = "USD") -> Money:
- sym:2377 Money.__str__ (tallyho/money.py:82-83)
- sym:2380 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:2401 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:2357 Order.currency (tallyho/models.py:73-74)
- sym:2379 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:2394 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:2345 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:2360 Order.subtotal (tallyho/models.py:92-93)
- sym:2364 _exponent (tallyho/money.py:14-18)
- sym:2374 Money.amount (tallyho/money.py:71-72)
- sym:2474 test_of_and_format (tests/test_money.py:9-12)

---
tokens: 685/800
