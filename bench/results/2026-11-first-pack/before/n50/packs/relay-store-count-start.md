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
- rec:e496f6ff attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:2dfbc4da attempt: Wrapping `count_records` in a bare try/except with a retry loop hid th…
- rec:a35487cf attempt: Wrapping `JsonFileStore.delete` in a bare try/except with a retry loop…
- rec:c70da368 attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…

## Code
- sym:3259 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:3269 MemoryStore (tallyho/store/memory.py:12-46)
- sym:3253 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:3155 _repo (tallyho/cli.py:15-17)
- sym:3276 count_records (tallyho/store/scan.py:23-24)
- sym:3159 Config (tallyho/config.py:15-20)
- sym:3275 iter_records (tallyho/store/scan.py:10-20)
- sym:3166 NotFound (tallyho/errors.py:24-25)
- sym:3175 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:3265 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:3186 Order (tallyho/models.py:64-109)
- sym:3156 cmd_products (tallyho/cli.py:20-23)

---
tokens: 715/800
omitted 25 lower-ranked items (expand an id for detail):
- rec:e9f1c455 decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- rec:a9cf1e3a decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:f599c9a4 decision: New optional parameters of `Repository.product` are keyword-only (afte…
- rec:9d8a0dad decision: Log lines from `JsonFileStore._save` go through the logging module, ne…
- rec:4171a766 decision: `JsonFileStore._save` keeps its docstring as the one place its behavio…
- rec:225d4f31 decision: New optional parameters of `JsonFileStore._load` are keyword-only (aft…
- … and 19 more
