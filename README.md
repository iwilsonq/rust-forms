# Rust Quiz Server

## psql

http://postgresguide.com/utilities/psql.html

## Shelling into the DB

```sh
psql postgres://ianwilson@localhost/diesel_demo
```

If psql cannot connect to the server, then make sure it is running:

```sh
# if install via homebrew on Mac
pg_ctl -D /usr/local/var/postgres start
```

## Setup and Migration

Steps for setting up tables:

```sh
diesel setup

diesel migration generate create_posts

# Creating migrations/20160815133237_create_posts/up.sql
# Creating migrations/20160815133237_create_posts/down.sql
```

This creates a `migrations` directory with a folder labeled by the current date. Inside are two files, `up.sql` and `down.sql`.
