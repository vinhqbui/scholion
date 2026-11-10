## Task
# relay-customer-region · relay-customer-region · open

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
- rec:d9fda090 attempt: Wrapping `ValidationError` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/errors.py:ValidationError
- rec:24c99112 decision: New optional parameters of `ValidationError` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/errors.py:ValidationError
- rec:25e24231 note: Benchmarks for `ValidationError` are in a separate, manual job; the unit tests must not depend on timing.
  anchors: tallyho/errors.py:ValidationError
- rec:696eecab attempt: Adding type: ignore comments around `check_email` to quiet the checker…

## Code
- sym:3618 check_region (tallyho/validate.py:24-26)
  def check_region(region: str) -> None:
- sym:3620 parse_quantity (tallyho/validate.py:34-42)
- sym:3616 check_sku (tallyho/validate.py:14-16)
- sym:3617 check_email (tallyho/validate.py:19-21)
- sym:3619 require_positive (tallyho/validate.py:29-31)
- sym:3512 Product.__post_init__ (tallyho/models.py:30-35)
- sym:3494 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:3518 Order (tallyho/models.py:64-109)
  class Order:
- sym:3591 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:3493 TallyError (tallyho/errors.py:4-5)
- sym:3506 Ledger.post (tallyho/ledger.py:28-35)

---
tokens: 717/800
omitted 161 lower-ranked items (expand an id for detail):
- rec:a0ddcb5e attempt: Wrapping `Money.allocate` in a bare try/except with a retry loop hid t…
- rec:e4e8278d attempt: Tried calling `check_email` twice to force a refresh; the second call…
- rec:ad2fca70 attempt: Tried calling `JsonFileStore._path` twice to force a refresh; the seco…
- rec:ca4ef4a3 attempt: Tried calling `Coupon.discount` twice to force a refresh; the second c…
- rec:c156b3fa attempt: Wrapping the `errors` module in a bare try/except with a retry loop hi…
- rec:78b62efd attempt: Adding type: ignore comments around `Money.allocate` to quiet the chec…
- … and 155 more
