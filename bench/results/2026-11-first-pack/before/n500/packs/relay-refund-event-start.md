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
- rec:ed6d3034 attempt: Wrapping `ValidationError` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/errors.py:ValidationError
- rec:8291d2f4 attempt: Caching what `Order` returns in a module-level dict broke tests that expect a fresh value per case.
  anchors: tallyho/models.py:Order
- rec:8963ba7f decision: Type hints on `Order` use the built-in generics (list[str], dict[str, int]), not the typing aliases.
  anchors: tallyho/models.py:Order

## Code
- sym:4680 Order (tallyho/models.py:64-109)
  class Order:
- sym:4786 Workflow._move (tallyho/workflow.py:29-33)
- sym:4788 Workflow.pay (tallyho/workflow.py:41-44)
- sym:4787 Workflow.place (tallyho/workflow.py:35-39)
- sym:4683 Order.remove (tallyho/models.py:86-89)
  def remove(self, sku: str) -> None:
- sym:4656 ValidationError (tallyho/errors.py:8-17)
- sym:4784 Workflow (tallyho/workflow.py:23-50)
- sym:4783 _ALLOWED (tallyho/workflow.py:14-20)
- sym:4672 OrderStatus (tallyho/models.py:14-19)
- sym:4655 TallyError (tallyho/errors.py:4-5)
- sym:4668 Ledger.post (tallyho/ledger.py:28-35)
- sym:4674 Product.__post_init__ (tallyho/models.py:30-35)

---
tokens: 708/800
omitted 202 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:dfd5cfed decision: New optional parameters of `ValidationError` are keyword-only (after *…
- rec:830b97b8 note: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:228f1d06 attempt: Adding type: ignore comments around `check_email` to quiet the checker…
- rec:97f1f185 attempt: Adding type: ignore comments around `Repository.save_order` to quiet t…
- rec:5d5e8139 attempt: Replacing the loop in the `models` module with a comprehension made it…
- … and 196 more
