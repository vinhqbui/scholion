## Task
# relay-coupon-remove · relay-coupon-remove · open
Goal: Shops need to withdraw coupons. Add `CouponBook.remove(code)`: afterwards `get` no longer finds that coupon, and removing a code the book does not know does nothing.

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:b4c99152 attempt: Wrapping `CouponBook.add` in a bare try/except with a retry loop hid t…
- rec:f2a6e45c attempt: Replacing the loop in `CouponBook` with a comprehension made it shorte…
- rec:580407a1 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:d3b9c020 decision: `CouponBook.add` keeps its docstring as the one place its behaviour is…
- rec:4ffecf63 decision: `CouponBook` keeps its docstring as the one place its behaviour is des…
- rec:ae6767b4 decision: New optional parameters of `CouponBook.add` are keyword-only (after *)…

## Code
- sym:3378 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:3381 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:3380 CouponBook.add (tallyho/pricing/coupons.py:46-47)
  def add(self, coupon: Coupon) -> None:
- sym:3379 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
- sym:3382 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:3457 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:3375 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:3393 quote_order (tallyho/pricing/quote.py:30-53)
- sym:3377 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:3477 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)

---
tokens: 635/800
omitted 5 lower-ranked items (expand an id for detail):
- rec:e44d7026 note: Remember `CouponBook.discounts_for` is not thread-safe; each worker ne…
- sym:3326 load_config (tallyho/config.py:23-47)
- sym:3337 EventBus.publish (tallyho/events.py:19-22)
- sym:3395 TaxTable.rate (tallyho/pricing/tax.py:19-23)
- sym:3404 product_from_record (tallyho/repository.py:19-21)
