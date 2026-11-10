## Task
# relay-product-slug · relay-product-slug · open
Goal: The web shop needs URL slugs for products. Add a read-only `Product.slug` property.

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:53b71d75 decision: New optional parameters of `Repository.products` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/repository.py:Repository.products
  A positional bool added in the middle of a signature bit us once.
- rec:376189a0 decision: New optional parameters of `Repository.product` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/repository.py:Repository.product
- rec:a2d54724 decision: `Repository.products` keeps its docstring as the one place its behaviour is described; the README only links to it.
  anchors: tallyho/repository.py:Repository.products
- rec:92987495 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:0ec503c2 decision: `product_to_record` keeps its docstring as the one place its behaviour…
- rec:ed93ef78 decision: `CouponBook.add` keeps its docstring as the one place its behaviour is…
- rec:d10b738b decision: New optional parameters of `cmd_products` are keyword-only (after *),…

## Code
- sym:4407 Repository.products (tallyho/repository.py:44-46)
  def products(self, category: str | None = None) -> list[Product]:
- sym:4403 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:4322 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:4439 slugify (tallyho/text.py:8-19)
- sym:4341 Product (tallyho/models.py:23-35)
- sym:4345 OrderLine (tallyho/models.py:51-60)
- sym:4405 Repository.save_product (tallyho/repository.py:38-39)
- sym:4406 Repository.product (tallyho/repository.py:41-42)
- sym:4348 Order (tallyho/models.py:64-109)
- sym:4400 product_from_record (tallyho/repository.py:19-21)
- sym:4321 Config (tallyho/config.py:15-20)
- sym:4318 cmd_products (tallyho/cli.py:20-23)
- sym:4320 main (tallyho/cli.py:31-42)
- sym:4329 ConfigError (tallyho/errors.py:28-29)
- sym:4349 Order.currency (tallyho/models.py:73-74)
- sym:4479 test_slugify (tests/test_text.py:4-5)
- sym:4460 book (tests/conftest.py:12-13)

---
tokens: 688/800
