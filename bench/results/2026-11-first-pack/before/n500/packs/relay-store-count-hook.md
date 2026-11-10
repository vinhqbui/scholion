## Task
# relay-store-count · relay-store-count · open

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
- rec:80999336 attempt: Tried calling `JsonFileStore` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/store/jsonfile.py:JsonFileStore
- rec:dc5ff473 attempt: Replacing the loop in `JsonFileStore` with a comprehension made it sho…
- rec:c43712ea attempt: Adding type: ignore comments around `JsonFileStore` to quiet the checker hid a real Optional bug two callers up.
  anchors: tallyho/store/jsonfile.py:JsonFileStore
- rec:a8921763 decision: `JsonFileStore` keeps its docstring as the one place its behaviour is…

## Code
- sym:4913 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:4919 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:4929 MemoryStore (tallyho/store/memory.py:12-46)
- sym:4936 count_records (tallyho/store/scan.py:23-24)
- sym:4925 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:4918 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:4935 iter_records (tallyho/store/scan.py:10-20)
- sym:4924 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- sym:4926 JsonFileStore.get (tallyho/store/jsonfile.py:56-60)
- sym:4931 MemoryStore.put (tallyho/store/memory.py:18-26)
- sym:4915 RecordStore.get (tallyho/store/base.py:28-29)

---
tokens: 703/800
omitted 125 lower-ranked items (expand an id for detail):
- rec:eba63f01 note: `MemoryStore` is on the hot path of the nightly batch job; profile bef…
- rec:81123815 note: Benchmarks for `JsonFileStore` are in a separate, manual job; the unit…
- rec:f30e2246 attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:c5f49be3 decision: Constants that `RecordStore.put_many` depends on live at the top of it…
- rec:ebc18cc0 decision: New optional parameters of `RecordStore.put_many` are keyword-only (af…
- rec:8366d58c note: Remember `count_records` is not thread-safe; each worker needs its own…
- … and 119 more
