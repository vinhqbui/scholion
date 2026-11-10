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
- rec:1a7be868 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:553bd864 attempt: Wrapping `Discount` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/pricing/discounts.py:Discount
- rec:4baa5583 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:cfe4a0d9 attempt: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…
- rec:bec7c860 decision: `CouponBook.add` keeps its docstring as the one place its behaviour is…
- rec:eea7673d decision: New optional parameters of the `discounts` module are keyword-only (af…
- rec:48e05fea note: `CouponBook.__init__` is on the hot path of the nightly batch job; pro…

## Code
- sym:1718 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:1721 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:1720 CouponBook.add (tallyho/pricing/coupons.py:46-47)
  def add(self, coupon: Coupon) -> None:
- sym:1719 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
  def __init__(self, coupons: list[Coupon] | None = None) -> None:
- sym:1722 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:1797 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:1715 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:1733 quote_order (tallyho/pricing/quote.py:30-53)
- sym:1717 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:1817 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)
- sym:1666 load_config (tallyho/config.py:23-47)
- sym:1677 EventBus.publish (tallyho/events.py:19-22)
- sym:1735 TaxTable.rate (tallyho/pricing/tax.py:19-23)
- sym:1744 product_from_record (tallyho/repository.py:19-21)

---
tokens: 684/800
