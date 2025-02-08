# Axum-sqlx Template

A template for creating REST API's using axum & sqlx.

Built with:

- [axum](https://crates.io/crates/axum)
- [sqlx](https://crates.io/crates/sqlx)
- [postgres](https://www.postgresql.org/)

## Requirements:

### Development

- [sqlx-cli](https://crates.io/crates/sqlx-cli)
- [direnv](https://github.com/direnv/direnv) (optional)
- [postgres](https://www.postgresql.org/)

### Deployment

> This template uses Docker for deployment so you don't need to install
> anything other than below listed packages on the production system.

- [docker](https://docker.com)
- [docker-compose](https://docs.docker.com/compose/)

# How to use the template

## For public repositories

You can click "Use this template" or just fork and start using it.

## For private repositories

You can clone this repository and start using it.

# Configuration

Check [src/config.rs](./src/config.rs) for necessary environemnt variables.

# Rename your project

1. Change package name in [Cargo.toml](./Cargo.toml)
2. Update import names in [src/main.rs](./src/main.rs) to reflect the new package name from above
3. Change container names in [docker-compose.yml](./docker-compose.yml)

# Running the application locally

1. Make sure you set DATABASE_URL in your environment variables.

   > I recommend using direnv + .env

2. Create database

   ```
   sqlx database create
   ```

3. Create a migration

   > Note: You just need to do this step so it creates a "migrations" directory,
   > you don't necessarily need to add anything in the migration files yet.

   ```
   sqlx migrate add -r init
   ```

4. Run the app

   ```
   cargo run
   ```

# Deploy the app for production

1. Update the environment file:

   Copy the .env.sample file to .env and then modify the contents for production

2. Run the app:

   ```
   docker-compose up -d
   ```

## Adding TLS Encryption (HTTPS)

To add HTTPS Encryption to your server, it is recommended to use a reverse
proxy like caddy/nginx instead of handling the HTTPS traffic directly.
