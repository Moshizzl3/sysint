# public.transactions

## Description

## Columns

| Name | Type | Default | Nullable | Children | Parents | Comment |
| ---- | ---- | ------- | -------- | -------- | ------- | ------- |
| id | integer | nextval('transactions_id_seq'::regclass) | false |  |  |  |
| account_id | integer |  | false |  | [public.accounts](public.accounts.md) |  |
| amount | integer |  | false |  |  |  |

## Constraints

| Name | Type | Definition |
| ---- | ---- | ---------- |
| fk_account_id | FOREIGN KEY | FOREIGN KEY (account_id) REFERENCES accounts(id) |
| transactions_pkey | PRIMARY KEY | PRIMARY KEY (id) |

## Indexes

| Name | Definition |
| ---- | ---------- |
| transactions_pkey | CREATE UNIQUE INDEX transactions_pkey ON public.transactions USING btree (id) |

## Relations

![er](public.transactions.svg)

---

> Generated by [tbls](https://github.com/k1LoW/tbls)
