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
- rec:08ce0042 attempt: Replacing the loop in `Workflow.ship` with a comprehension made it sho…
- rec:11ce0b3a decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- rec:c6c78f1e decision: Errors about customer data must not echo the rejected value: they are…

## Code
- sym:3020 Order (tallyho/models.py:64-109)
  class Order:
- sym:3126 Workflow._move (tallyho/workflow.py:29-33)
  def _move(self, order: Order, to: OrderStatus) -> None:
- sym:3128 Workflow.pay (tallyho/workflow.py:41-44)
  def pay(self, order: Order, on: date) -> None:
- sym:3127 Workflow.place (tallyho/workflow.py:35-39)
- sym:3124 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:3123 _ALLOWED (tallyho/workflow.py:14-20)
  _ALLOWED = {
- sym:3130 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:3022 Order.add (tallyho/models.py:76-84)
- sym:3023 Order.remove (tallyho/models.py:86-89)
- sym:2996 ValidationError (tallyho/errors.py:8-17)
- sym:3012 OrderStatus (tallyho/models.py:14-19)
- sym:2995 TallyError (tallyho/errors.py:4-5)

---
tokens: 710/800
omitted 52 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:355c5e41 decision: `Workflow.pay` keeps its docstring as the one place its behaviour is d…
- rec:7d5f00f5 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:5585bee1 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:0d84b78d attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:0fd2e810 attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- … and 46 more
