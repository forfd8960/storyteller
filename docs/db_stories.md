# Story Table

* `sqlx migrate add add_story_table`
* `sqlx migrate run`
* `psql -U db_manager -d story_collection`


```sh
storyteller git:(main) sqlx migrate add add_story_table
Creating migrations/20240925004725_add_story_table.sql
➜  storyteller git:(main) ✗ sqlx migrate run
Applied 20240925004725/migrate add story table (46.831208ms)
➜  storyteller git:(main) ✗ psql -U db_manager -d story_collection
psql (14.12 (Homebrew))
Type "help" for help.

story_collection=> \dt
               List of relations
 Schema |       Name       | Type  |   Owner
--------+------------------+-------+------------
 public | _sqlx_migrations | table | db_manager
 public | stories          | table | db_manager
 public | users            | table | db_manager
(3 rows)

story_collection=> \d stories
                                       Table "public.stories"
   Column   |           Type           | Collation | Nullable |               Default
------------+--------------------------+-----------+----------+-------------------------------------
 id         | bigint                   |           | not null | nextval('stories_id_seq'::regclass)
 title      | character varying(500)   |           | not null |
 author_id  | integer                  |           | not null |
 content    | text                     |           | not null | ''::text
 created_at | timestamp with time zone |           | not null | CURRENT_TIMESTAMP
 updated_at | timestamp with time zone |           | not null | CURRENT_TIMESTAMP
Indexes:
    "stories_pkey" PRIMARY KEY, btree (id)
    "idx_stories_author_id" btree (author_id)
    "idx_stories_author_id_created_at" btree (author_id, created_at)
    "idx_stories_author_id_updated_at" btree (author_id, updated_at)
```

## Change author_id to `BIGINT`

```sh
➜  storyteller git:(main) ✗ sqlx migrate add change_stories_author_id_type
Creating migrations/20240925010637_change_stories_author_id_type.sql

➜  storyteller git:(main) ✗ sqlx migrate run
Applied 20240925010637/migrate change stories author id type (31.789833ms)
➜  storyteller git:(main) ✗ psql -U db_manager -d story_collection
psql (14.12 (Homebrew))
Type "help" for help.

story_collection=> \dt
               List of relations
 Schema |       Name       | Type  |   Owner
--------+------------------+-------+------------
 public | _sqlx_migrations | table | db_manager
 public | stories          | table | db_manager
 public | users            | table | db_manager
(3 rows)

story_collection=> \d stories
                                       Table "public.stories"
   Column   |           Type           | Collation | Nullable |               Default
------------+--------------------------+-----------+----------+-------------------------------------
 id         | bigint                   |           | not null | nextval('stories_id_seq'::regclass)
 title      | character varying(500)   |           | not null |
 author_id  | bigint                   |           | not null |
 content    | text                     |           | not null | ''::text
 created_at | timestamp with time zone |           | not null | CURRENT_TIMESTAMP
 updated_at | timestamp with time zone |           | not null | CURRENT_TIMESTAMP
Indexes:
    "stories_pkey" PRIMARY KEY, btree (id)
    "idx_stories_author_id" btree (author_id)
    "idx_stories_author_id_created_at" btree (author_id, created_at)
    "idx_stories_author_id_updated_at" btree (author_id, updated_at)

```
