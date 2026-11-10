# First packs: is the task's own lesson in them?

Noise notes: 500. Tasks: 10. No agent ran.

| task | records seeded | pack at task start | hook pack | records in hook pack | omitted | pre-read (file: lesson, records) |
|---|---|---|---|---|---|---|
| relay-coupon-remove | 510 | absent | handle | 6 | 5 | tallyho/pricing/coupons.py: no, 7 |
| relay-customer-region | 510 | absent | handle | 4 | 168 | tallyho/models.py: no, 7; tallyho/validate.py: no, 8 |
| relay-ledger-refund | 510 | omitted #2 | handle | 4 | 203 | tallyho/ledger.py: no, 7 |
| relay-line-taxes | 510 | handle | summary | 4 | 162 | tallyho/pricing/quote.py: no, 7 |
| relay-money-plain | 510 | absent | handle | 6 | 6 | tallyho/money.py: no, 8 |
| relay-order-ids | 510 | absent | absent | 7 | 0 | tallyho/repository.py: no, 7 |
| relay-product-slug | 510 | absent | handle | 7 | 0 | tallyho/models.py: no, 7 |
| relay-quote-split | 510 | absent | omitted #1 | 4 | 171 | tallyho/pricing/quote.py: no, 7 |
| relay-refund-event | 510 | absent | absent | 4 | 202 | tallyho/workflow.py: no, 8; tallyho/models.py: no, 7 |
| relay-store-count | 510 | absent | omitted #3 | 4 | 125 | tallyho/store/base.py: no, 7; tallyho/store/memory.py: no, 7; tallyho/store/jsonfile.py: no, 7 |

Lesson shown in the pack at task start: 1 of 10. In the hook pack: 6 of 10. In the hook pack or a pre-read answer: 6 of 10.
