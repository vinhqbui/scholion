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
- sym:4919 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:4929 MemoryStore (tallyho/store/memory.py:12-46)
  class MemoryStore(RecordStore):
- sym:4913 RecordStore (tallyho/store/base.py:17-41)
- sym:4936 count_records (tallyho/store/scan.py:23-24)
- sym:4935 iter_records (tallyho/store/scan.py:10-20)
- sym:4925 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:4918 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:4815 _repo (tallyho/cli.py:15-17)
- sym:4974 store (tests/test_store.py:8-9)
- sym:4818 main (tallyho/cli.py:31-42)
- sym:4819 Config (tallyho/config.py:15-20)
  class Config:
- sym:4901 Repository (tallyho/repository.py:32-74)

---
tokens: 706/800
omitted 114 lower-ranked items (expand an id for detail):
- rec:eba63f01 note: `MemoryStore` is on the hot path of the nightly batch job; profile bef…
- rec:81123815 note: Benchmarks for `JsonFileStore` are in a separate, manual job; the unit…
- rec:c139dbde attempt: Adding type: ignore comments around `_repo` to quiet the checker hid a…
- rec:27356826 attempt: Wrapping `RecordStore` in a bare try/except with a retry loop hid the…
- rec:59c7f352 attempt: Caching what `RecordStore` returns in a module-level dict broke tests…
- rec:0357884b decision: Log lines from the `memory` module go through the logging module, neve…
- … and 108 more
