# Axum-sqlx Template

A template for creating REST API's using axum & sqlx.

This template primarily uses:

- [axum](https://crates.io/crates/axum)
- [sqlx](https://crates.io/crates/sqlx)
- [postgres](https://www.postgresql.org/)

Requirements:

- [sqlx-cli](https://crates.io/crates/sqlx-cli)
- [direnv](https://github.com/direnv/direnv) (optional)

# What to do first

1. Change package name in [Cargo.toml](./Cargo.toml)
2. Update import names in [src/main.rs](./src/main.rs)

# How to use

1. Make sure you set DATABASE_URL in your environment variables (I personally use direnv with .env)

2. Create database

```
sqlx database create
```

3. Create a migration

> Note: you can omit the "-r", it's for creating revertable migrations (up.sql with down.sql). Check their readme for more info.

```
sqlx migrate add -r init
```

4. Run the app

```
cargo run
```
