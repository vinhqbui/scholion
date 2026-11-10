## Task
# relay-order-ids · relay-order-ids · open
Goal: Add `Repository.next_order_id()`, which returns the id to give a new order.

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:906e485d attempt: Tried calling `Repository.order` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/repository.py:Repository.order
  A sleep between the calls did not help either. It needs a real invalidation path, not a retry.
- rec:2861256b decision: New optional parameters of `Repository.save_order` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/repository.py:Repository.save_order
- rec:639fef24 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:327612e3 decision: New optional parameters of `Repository.product` are keyword-only (afte…
- rec:98747c84 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:4ba73e21 decision: `CouponBook.add` keeps its docstring as the one place its behaviour is…
- rec:619a5798 intent: Earlier work on this code

## Code
- sym:2585 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:2584 Repository.save_order (tallyho/repository.py:54-55)
  def save_order(self, o: Order) -> None:
- sym:2587 Repository.orders (tallyho/repository.py:72-74)
  def orders(self, status: OrderStatus | None = None) -> list[Order]:
- sym:2586 Repository._order_from_record (tallyho/repository.py:61-70)
  def _order_from_record(self, r: dict) -> Order:
- sym:2577 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:2588 Page (tallyho/store/base.py:10-14)
  class Page:
- sym:2524 Order.add (tallyho/models.py:76-84)
- sym:2560 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:2522 Order (tallyho/models.py:64-109)
  class Order:
- sym:2528 Order.to_dict (tallyho/models.py:98-109)
- sym:2551 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:2563 quote_order (tallyho/pricing/quote.py:30-53)
- sym:2590 RecordStore.put (tallyho/store/base.py:24-25)
- sym:2636 order (tests/conftest.py:22-26)
- sym:2652 test_list_page_cursor (tests/test_store.py:20-25)

---
tokens: 688/800
