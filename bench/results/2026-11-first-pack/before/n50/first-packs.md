# First packs: is the task's own lesson in them?

Noise notes: 50. Tasks: 10. No agent ran.

| task | records seeded | pack at task start | hook pack | records in hook pack | omitted | pre-read (file: lesson, records) |
|---|---|---|---|---|---|---|
| relay-coupon-remove | 60 | absent | handle | 7 | 0 | tallyho/pricing/coupons.py: yes, 3 |
| relay-customer-region | 60 | omitted #1 | handle | 4 | 54 | tallyho/models.py: no, 2; tallyho/validate.py: yes, 2 |
| relay-ledger-refund | 60 | handle | summary | 4 | 58 | tallyho/ledger.py: no, 6 |
| relay-line-taxes | 60 | summary | summary | 4 | 50 | tallyho/pricing/quote.py: no, 0 |
| relay-money-plain | 60 | absent | handle | 7 | 0 | tallyho/money.py: yes, 6 |
| relay-order-ids | 60 | absent | handle | 7 | 0 | tallyho/repository.py: yes, 2 |
| relay-product-slug | 60 | absent | summary | 7 | 0 | tallyho/models.py: no, 1 |
| relay-quote-split | 60 | omitted #1 | handle | 4 | 56 | tallyho/pricing/quote.py: no, 1 |
| relay-refund-event | 60 | absent | omitted #4 | 4 | 52 | tallyho/workflow.py: yes, 5; tallyho/models.py: no, 2 |
| relay-store-count | 60 | absent | handle | 4 | 27 | tallyho/store/base.py: yes, 4; tallyho/store/memory.py: no, 1; tallyho/store/jsonfile.py: no, 6 |

Lesson shown in the pack at task start: 2 of 10. In the hook pack: 9 of 10. In the hook pack or a pre-read answer: 10 of 10.
