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
- rec:64da4aa2 attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:0d84b78d attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:08ce0042 attempt: Replacing the loop in `Workflow.ship` with a comprehension made it sho…
- rec:c6c78f1e decision: Errors about customer data must not echo the rejected value: they are…

## Code
- sym:3020 Order (tallyho/models.py:64-109)
  class Order:
- sym:2996 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:3012 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:3126 Workflow._move (tallyho/workflow.py:29-33)
  def _move(self, order: Order, to: OrderStatus) -> None:
- sym:3128 Workflow.pay (tallyho/workflow.py:41-44)
- sym:3127 Workflow.place (tallyho/workflow.py:35-39)
- sym:3014 Product.__post_init__ (tallyho/models.py:30-35)
  def __post_init__(self) -> None:
- sym:3023 Order.remove (tallyho/models.py:86-89)
- sym:3124 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:3123 _ALLOWED (tallyho/workflow.py:14-20)
- sym:3000 NotFound (tallyho/errors.py:24-25)
- sym:3013 Product (tallyho/models.py:23-35)

---
tokens: 719/800
omitted 54 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:0fd2e810 attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:1f4f8003 attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- rec:a10d08b8 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:f45258a7 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:11ce0b3a decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- … and 48 more
