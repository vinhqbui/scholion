## Task
# relay-order-ids · relay-order-ids · open
Goal: Add `Repository.next_order_id()`, which returns the id to give a new order.
Workspace: /root/bench-work/ws/bfp7924-relay-order-ids-av-1/relay-order-ids
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:7cd5cf93 attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:7031fe83 attempt: Wrapping `Repository.save_order` in a bare try/except with a retry loo…
- rec:a279a0e2 attempt: Adding type: ignore comments around `Repository._order_from_record` to…
- rec:c7818191 decision: New optional parameters of `Repository._order_from_record` are keyword…
- rec:c95d9a52 decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- rec:d956d4b6 decision: `Repository.save_order` keeps its docstring as the one place its behav…
- rec:a248b63a note: `Repository.save_order` is on the hot path of the nightly batch job; profile before adding work to it.
  anchors: tallyho/repository.py:Repository.save_order

## Code
- sym:4245 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:4244 Repository.save_order (tallyho/repository.py:54-55)
  def save_order(self, o: Order) -> None:
- sym:4247 Repository.orders (tallyho/repository.py:72-74)
  def orders(self, status: OrderStatus | None = None) -> list[Order]:
- sym:4246 Repository._order_from_record (tallyho/repository.py:61-70)
  def _order_from_record(self, r: dict) -> Order:
- sym:4237 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:4248 Page (tallyho/store/base.py:10-14)
  class Page:
- sym:4184 Order.add (tallyho/models.py:76-84)
- sym:4220 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:4182 Order (tallyho/models.py:64-109)
  class Order:
- sym:4188 Order.to_dict (tallyho/models.py:98-109)
- sym:4211 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:4223 quote_order (tallyho/pricing/quote.py:30-53)
- sym:4250 RecordStore.put (tallyho/store/base.py:24-25)
- sym:4296 order (tests/conftest.py:22-26)
- sym:4312 test_list_page_cursor (tests/test_store.py:20-25)

---
tokens: 682/800
