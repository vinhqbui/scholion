## Task
# relay-line-taxes · relay-line-taxes · open

## Map
- tallyho/pricing/quote.py
- tallyho/models.py
- tallyho/money.py
- tallyho/pricing/coupons.py
- tallyho/pricing/discounts.py
- tallyho/pricing/tax.py
- tallyho/report/csv_export.py
- tallyho/repository.py
- tallyho/workflow.py
- tests/conftest.py
- tests/test_pricing.py
- tallyho/cli.py
- tallyho/config.py
- tallyho/ledger.py

## Decisions & dead ends
- rec:6b707ca5 attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:b1bf2865 attempt: Caching what `quote_order` returns in a module-level dict broke tests…
- rec:c1844479 decision: New optional parameters of `quote_order` are keyword-only (after *), s…
- rec:5c7f99df decision: `quote_order` keeps its docstring as the one place its behaviour is de…

## Code
- sym:3891 quote_order (tallyho/pricing/quote.py:30-53)
- sym:3890 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:3889 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:3850 Order (tallyho/models.py:64-109)
  class Order:
- sym:3852 Order.add (tallyho/models.py:76-84)
- sym:3894 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:3853 Order.remove (tallyho/models.py:86-89)
- sym:3843 Product (tallyho/models.py:23-35)
- sym:3905 Repository (tallyho/repository.py:32-74)
- sym:3974 test_quote_taxes_only_taxable (tests/test_pricing.py:20-24)
- sym:3849 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:3851 Order.currency (tallyho/models.py:73-74)

---
tokens: 710/800
omitted 162 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:6969349d decision: Type hints on `quote_order` use the built-in generics (list[str], dict…
- rec:2dbba02e note: Remember `quote_order` is not thread-safe; each worker needs its own i…
- rec:94667fff note: Deprecation warnings from `quote_order` are turned into errors in CI (…
- rec:2bc41c24 note: Benchmarks for `quote_order` are in a separate, manual job; the unit t…
- rec:0506e748 attempt: Caching what `TaxTable.tax` returns in a module-level dict broke tests…
- … and 156 more
