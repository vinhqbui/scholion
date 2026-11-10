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
- rec:e65f4bfb decision: New optional parameters of `Repository.product` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/repository.py:Repository.product
- rec:23ffd089 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug" -> "mug-1-blue-mug". Names alone collide (two SKUs called "Blue mug"), so the SKU always leads.
  anchors: tallyho/models.py:Product
- rec:9c5717dd decision: `CouponBook.add` keeps its docstring as the one place its behaviour is…
- rec:749d75dd decision: `Ledger.record_sale` keeps its docstring as the one place its behaviou…
- rec:4f46c3b8 decision: New optional parameters of `JsonFileStore._load` are keyword-only (aft…
- rec:4710b112 decision: New optional parameters of the `discounts` module are keyword-only (af…
- rec:803e0e06 decision: `JsonFileStore._save` keeps its docstring as the one place its behavio…

## Code
- sym:2747 Repository.products (tallyho/repository.py:44-46)
  def products(self, category: str | None = None) -> list[Product]:
- sym:2743 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:2662 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:2779 slugify (tallyho/text.py:8-19)
- sym:2681 Product (tallyho/models.py:23-35)
  class Product:
- sym:2685 OrderLine (tallyho/models.py:51-60)
- sym:2745 Repository.save_product (tallyho/repository.py:38-39)
- sym:2746 Repository.product (tallyho/repository.py:41-42)
- sym:2688 Order (tallyho/models.py:64-109)
- sym:2740 product_from_record (tallyho/repository.py:19-21)
- sym:2661 Config (tallyho/config.py:15-20)
- sym:2658 cmd_products (tallyho/cli.py:20-23)
- sym:2660 main (tallyho/cli.py:31-42)
- sym:2669 ConfigError (tallyho/errors.py:28-29)
- sym:2689 Order.currency (tallyho/models.py:73-74)
- sym:2819 test_slugify (tests/test_text.py:4-5)
- sym:2800 book (tests/conftest.py:12-13)

---
tokens: 689/800
