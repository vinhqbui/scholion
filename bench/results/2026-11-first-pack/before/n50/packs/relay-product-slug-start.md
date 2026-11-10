## Task
# relay-product-slug · relay-product-slug · open
Goal: The web shop needs URL slugs for products. Add a read-only `Product.slug` property.
Workspace: /root/bench-work/ws/bfp74a3-relay-product-slug-av-1/relay-product-slug
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Code
- sym:2662 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:2661 Config (tallyho/config.py:15-20)
  class Config:
- sym:2685 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:2688 Order (tallyho/models.py:64-109)
  class Order:
- sym:2681 Product (tallyho/models.py:23-35)
  class Product:
- sym:2743 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:2747 Repository.products (tallyho/repository.py:44-46)
  def products(self, category: str | None = None) -> list[Product]:
- sym:2682 Product.__post_init__ (tallyho/models.py:30-35)
  def __post_init__(self) -> None:
- sym:2745 Repository.save_product (tallyho/repository.py:38-39)
  def save_product(self, p: Product) -> None:
- sym:2658 cmd_products (tallyho/cli.py:20-23)
  def cmd_products(repo: Repository, args: argparse.Namespace) -> int:
- sym:2746 Repository.product (tallyho/repository.py:41-42)
  def product(self, sku: str) -> Product:
- sym:2660 main (tallyho/cli.py:31-42)
  def main(argv: list[str] | None = None) -> int:
- sym:2669 ConfigError (tallyho/errors.py:28-29)
  class ConfigError(TallyError):
- sym:2689 Order.currency (tallyho/models.py:73-74)
  def currency(self) -> str:
- sym:2717 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:2731 TaxTable.rate (tallyho/pricing/tax.py:19-23)
  def rate(self, region: str) -> Decimal:

---
tokens: 582/800
