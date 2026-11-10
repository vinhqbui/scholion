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
- rec:cddabbca decision: Errors about customer data must not echo the rejected value: they are…

## Code
- sym:3618 check_region (tallyho/validate.py:24-26)
- sym:3616 check_sku (tallyho/validate.py:14-16)
- sym:3512 Product.__post_init__ (tallyho/models.py:30-35)
- sym:3620 parse_quantity (tallyho/validate.py:34-42)
- sym:3617 check_email (tallyho/validate.py:19-21)
- sym:3619 require_positive (tallyho/validate.py:29-31)
- sym:3494 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:3511 Product (tallyho/models.py:23-35)
- sym:3518 Order (tallyho/models.py:64-109)
- sym:3615 _REGION (tallyho/validate.py:11-11)
- sym:3513 Customer (tallyho/models.py:39-47)
- sym:3591 JsonFileStore (tallyho/store/jsonfile.py:14-72)

---
tokens: 703/800
omitted 168 lower-ranked items (expand an id for detail):
- rec:8e2c801a decision: Tests for `check_region` build their inputs with small factory helpers…
- rec:605f9044 note: The `validate` module reads files as UTF-8 explicitly; the platform de…
- rec:112ebd60 note: Deprecation warnings from `Customer` are turned into errors in CI (fil…
- rec:a564ef12 note: `ValidationError.__str__` reads files as UTF-8 explicitly; the platfor…
- rec:fc8d1239 note: Deprecation warnings from `customer_from_record` are turned into error…
- rec:696eecab attempt: Adding type: ignore comments around `check_email` to quiet the checker…
- … and 162 more
