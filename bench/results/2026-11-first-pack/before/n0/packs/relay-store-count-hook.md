## Task
# relay-store-count · relay-store-count · open
Goal: Add `count(collection)` to the record store: it returns how many records a collection holds (0 for an unknown one). Every backend (MemoryStore and JsonFileStore) must support it.

## Map
- tallyho/store/jsonfile.py
- tallyho/store/memory.py
- tallyho/cli.py
- tallyho/store/base.py
- tests/test_store.py
  L8: def store(request, tmp_path):
  L12: def test_put_get_delete(store):
  L20: def test_list_page_cursor(store):
- tallyho/config.py
- tallyho/repository.py
- tallyho/store/scan.py
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)

## Decisions & dead ends
- rec:117e37cc attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:75b2b4ef decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:90019cf3 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…

## Code
- sym:1593 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:1599 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:1609 MemoryStore (tallyho/store/memory.py:12-46)
- sym:1616 count_records (tallyho/store/scan.py:23-24)
- sym:1605 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:1598 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:1615 iter_records (tallyho/store/scan.py:10-20)
- sym:1604 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- sym:1606 JsonFileStore.get (tallyho/store/jsonfile.py:56-60)
- sym:1611 MemoryStore.put (tallyho/store/memory.py:18-26)
- sym:1595 RecordStore.get (tallyho/store/base.py:28-29)
- sym:1495 _repo (tallyho/cli.py:15-17)

---
tokens: 707/800
omitted 19 lower-ranked items (expand an id for detail):
- rec:4b284b3b attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:63cacfc1 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:7b8a1271 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:7a1ca825 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- sym:1654 store (tests/test_store.py:8-9)
- sym:1498 main (tallyho/cli.py:31-42)
- sym:1499 Config (tallyho/config.py:15-20)
- … and 12 more
