## Task
# relay-order-ids · relay-order-ids · open
Goal: Add `Repository.next_order_id()`, which returns the id to give a new order.
Workspace: /root/bench-work/ws/bfp703c-relay-order-ids-av-1/relay-order-ids
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:7a227ad9 attempt: CouponBook stores codes as code.upper() and get() looks them up with code.strip().upper(). Indexing _by_code with the code as typed misses ' save10 ' or 'save10'.
  anchors: tallyho/pricing/coupons.py:CouponBook
- rec:484d7bc2 attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:f85f2a75 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:de0620e5 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:7a178db2 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:e973d125 decision: Errors about customer data must not echo the rejected value: they are…
- rec:fa0f6eee intent: Earlier work on this code

## Code
- sym:925 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:924 Repository.save_order (tallyho/repository.py:54-55)
  def save_order(self, o: Order) -> None:
- sym:927 Repository.orders (tallyho/repository.py:72-74)
  def orders(self, status: OrderStatus | None = None) -> list[Order]:
- sym:926 Repository._order_from_record (tallyho/repository.py:61-70)
  def _order_from_record(self, r: dict) -> Order:
- sym:917 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:928 Page (tallyho/store/base.py:10-14)
  class Page:
- sym:864 Order.add (tallyho/models.py:76-84)
- sym:900 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:862 Order (tallyho/models.py:64-109)
  class Order:
- sym:868 Order.to_dict (tallyho/models.py:98-109)
- sym:891 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:903 quote_order (tallyho/pricing/quote.py:30-53)
- sym:930 RecordStore.put (tallyho/store/base.py:24-25)
- sym:976 order (tests/conftest.py:22-26)
- sym:992 test_list_page_cursor (tests/test_store.py:20-25)

---
tokens: 684/800
