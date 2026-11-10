# First packs: is the task's own lesson in them?

Noise notes: 0. Tasks: 10. No agent ran.

| task | records seeded | pack at task start | hook pack | records in hook pack | omitted | pre-read (file: lesson, records) |
|---|---|---|---|---|---|---|
| relay-coupon-remove | 10 | absent | absent | 0 | 0 | tallyho/pricing/coupons.py: yes, 1 |
| relay-customer-region | 10 | handle | handle | 4 | 37 | tallyho/models.py: no, 1; tallyho/validate.py: yes, 1 |
| relay-ledger-refund | 10 | summary | summary | 4 | 44 | tallyho/ledger.py: no, 0 |
| relay-line-taxes | 10 | summary | summary | 4 | 34 | tallyho/pricing/quote.py: no, 0 |
| relay-money-plain | 10 | absent | handle | 7 | 0 | tallyho/money.py: yes, 2 |
| relay-order-ids | 10 | absent | handle | 7 | 0 | tallyho/repository.py: yes, 1 |
| relay-product-slug | 10 | absent | handle | 5 | 6 | tallyho/models.py: yes, 1 |
| relay-quote-split | 10 | handle | handle | 3 | 42 | tallyho/pricing/quote.py: no, 1 |
| relay-refund-event | 10 | absent | omitted #3 | 4 | 40 | tallyho/workflow.py: yes, 1; tallyho/models.py: no, 1 |
| relay-store-count | 10 | absent | handle | 3 | 19 | tallyho/store/base.py: yes, 1; tallyho/store/memory.py: no, 0; tallyho/store/jsonfile.py: no, 0 |

Lesson shown in the pack at task start: 4 of 10. In the hook pack: 8 of 10. In the hook pack or a pre-read answer: 10 of 10.
