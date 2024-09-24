# DB - Postgres

## Create DB

```sh
createdb story_collection --template=template0
```

## Sqlx migrate

* `sqlx migrate add init_table`

create table with sql

```sql
-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id bigserial PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(100) NOT NULL,
    password_hash VARCHAR(100) NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_users_email ON users(email);
```

## .env

create `.env`

```toml
DATABASE_URL="postgresql://db_manager@localhost:5432/story_collection"
```

## Run Migration

```sh
storyteller (main) [1]> sqlx migrate run
Applied 20240924010111/migrate init table (34.038042ms)
```

## Check DB

```sh
storyteller (main) [2]> psql -U db_manager -d story_collection
psql (14.12 (Homebrew))
Type "help" for help.

story_collection=> \dt
               List of relations
 Schema |       Name       | Type  |   Owner    
--------+------------------+-------+------------
 public | _sqlx_migrations | table | db_manager
 public | users            | table | db_manager
(2 rows)

story_collection=> select * from users
story_collection-> \d users
                                        Table "public.users"
    Column     |           Type           | Collation | Nullable |              Default              
---------------+--------------------------+-----------+----------+-----------------------------------
 id            | bigint                   |           | not null | nextval('users_id_seq'::regclass)
 username      | character varying(255)   |           | not null | 
 email         | character varying(100)   |           | not null | 
 password_hash | character varying(100)   |           | not null | 
 created_at    | timestamp with time zone |           |          | CURRENT_TIMESTAMP
Indexes:
    "users_pkey" PRIMARY KEY, btree (id)
    "idx_users_email" UNIQUE, btree (email)

story_collection-> 
```
