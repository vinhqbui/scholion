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
- rec:4e152f4b attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:3523fe7d attempt: Replacing the loop in `Order.currency` with a comprehension made it sh…
- rec:a68832ac decision: Event payloads carry money as integer minor units plus the currency co…
- rec:cf4baaba decision: New optional parameters of `Money.format` are keyword-only (after *),…
- rec:b5f12e28 note: `Discount.amount_off` reads files as UTF-8 explicitly; the platform de…
- rec:13406446 note: The `money` module reads files as UTF-8 explicitly; the platform default encoding differs on the Windows build agents.
  anchors: tallyho/money.py

## Code
- sym:4036 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:4038 total (tallyho/money.py:86-91)
- sym:4026 Money.of (tallyho/money.py:29-33)
- sym:4025 Money (tallyho/money.py:22-83)
  class Money:
- sym:4027 Money.zero (tallyho/money.py:36-37)
- sym:4037 Money.__str__ (tallyho/money.py:82-83)
- sym:4040 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:4061 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:4017 Order.currency (tallyho/models.py:73-74)
- sym:4039 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:4054 apply_discounts (tallyho/pricing/discounts.py:46-67)

---
tokens: 656/800
omitted 6 lower-ranked items (expand an id for detail):
- rec:c6dc842a note: `CurrencyMismatch` reads files as UTF-8 explicitly; the platform defau…
- sym:4005 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:4020 Order.subtotal (tallyho/models.py:92-93)
- sym:4024 _exponent (tallyho/money.py:14-18)
- sym:4034 Money.amount (tallyho/money.py:71-72)
- sym:4134 test_of_and_format (tests/test_money.py:9-12)
