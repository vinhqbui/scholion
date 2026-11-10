## Task
# relay-refund-event · relay-refund-event · open

## Map
- tallyho/errors.py
- tallyho/models.py
- tests/conftest.py
- tallyho/ledger.py
- tallyho/money.py
- tallyho/pricing/coupons.py
- tallyho/pricing/discounts.py
- tallyho/pricing/quote.py
- tallyho/report/csv_export.py
- tallyho/repository.py
- tallyho/store/jsonfile.py
- tallyho/store/memory.py
- tallyho/validate.py
- tallyho/workflow.py
- tests/test_models.py
- tests/test_pricing.py
- tallyho/cli.py

## Decisions & dead ends
- rec:33957bd1 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:ade3e2ca attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:ad22683e decision: Errors about customer data must not echo the rejected value: they are…

## Code
- sym:1360 Order (tallyho/models.py:64-109)
  class Order:
- sym:1336 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:1352 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:1466 Workflow._move (tallyho/workflow.py:29-33)
  def _move(self, order: Order, to: OrderStatus) -> None:
- sym:1468 Workflow.pay (tallyho/workflow.py:41-44)
  def pay(self, order: Order, on: date) -> None:
- sym:1467 Workflow.place (tallyho/workflow.py:35-39)
- sym:1354 Product.__post_init__ (tallyho/models.py:30-35)
- sym:1363 Order.remove (tallyho/models.py:86-89)
- sym:1464 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:1463 _ALLOWED (tallyho/workflow.py:14-20)
- sym:1340 NotFound (tallyho/errors.py:24-25)
- sym:1353 Product (tallyho/models.py:23-35)
- sym:1357 OrderLine (tallyho/models.py:51-60)

---
tokens: 726/800
omitted 39 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:ba5405f7 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- sym:1358 OrderLine.__post_init__ (tallyho/models.py:55-56)
- sym:1335 TallyError (tallyho/errors.py:4-5)
- sym:1348 Ledger.post (tallyho/ledger.py:28-35)
- sym:1362 Order.add (tallyho/models.py:76-84)
- sym:1368 _exponent (tallyho/money.py:14-18)
- sym:1377 Money.allocate (tallyho/money.py:63-68)
- … and 31 more
