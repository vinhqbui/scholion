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
- rec:6dc3f31a attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:ea7b3a02 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:b38add37 attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:f9a5a612 decision: Errors about customer data must not echo the rejected value: they are…
- rec:65c2a8f7 decision: Event payloads carry money as integer minor units plus the currency co…

## Code
- sym:1087 Repository.products (tallyho/repository.py:44-46)
  def products(self, category: str | None = None) -> list[Product]:
- sym:1083 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:1002 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:1119 slugify (tallyho/text.py:8-19)
  def slugify(text: str) -> str:
- sym:1021 Product (tallyho/models.py:23-35)
  class Product:
- sym:1025 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:1085 Repository.save_product (tallyho/repository.py:38-39)
- sym:1086 Repository.product (tallyho/repository.py:41-42)
- sym:1028 Order (tallyho/models.py:64-109)
- sym:1080 product_from_record (tallyho/repository.py:19-21)
- sym:1001 Config (tallyho/config.py:15-20)
- sym:998 cmd_products (tallyho/cli.py:20-23)
- sym:1000 main (tallyho/cli.py:31-42)

---
tokens: 658/800
omitted 6 lower-ranked items (expand an id for detail):
- rec:e5004f1b decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:f252cfe0 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- sym:1009 ConfigError (tallyho/errors.py:28-29)
- sym:1029 Order.currency (tallyho/models.py:73-74)
- sym:1159 test_slugify (tests/test_text.py:4-5)
- sym:1140 book (tests/conftest.py:12-13)
