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
- rec:2dfbc4da attempt: Wrapping `count_records` in a bare try/except with a retry loop hid th…
- rec:300b248b attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:8b87de18 attempt: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…
- rec:c70da368 attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…

## Code
- sym:3253 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:3259 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:3269 MemoryStore (tallyho/store/memory.py:12-46)
- sym:3276 count_records (tallyho/store/scan.py:23-24)
- sym:3265 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:3258 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:3275 iter_records (tallyho/store/scan.py:10-20)
- sym:3264 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- sym:3266 JsonFileStore.get (tallyho/store/jsonfile.py:56-60)
- sym:3271 MemoryStore.put (tallyho/store/memory.py:18-26)
- sym:3314 store (tests/test_store.py:8-9)

---
tokens: 708/800
omitted 27 lower-ranked items (expand an id for detail):
- rec:86443bfa attempt: Wrapping `RecordStore.list_page` in a bare try/except with a retry loo…
- rec:225d4f31 decision: New optional parameters of `JsonFileStore._load` are keyword-only (aft…
- rec:dfc2c7df note: The repr of objects around `JsonFileStore._path` shows up in logs, so…
- rec:e496f6ff attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:a35487cf attempt: Wrapping `JsonFileStore.delete` in a bare try/except with a retry loop…
- rec:e9f1c455 decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- … and 21 more
