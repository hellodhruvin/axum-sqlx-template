FROM rust:1.84 AS build

WORKDIR /app
COPY . /app
RUN echo '[[bin]]\nname = "server"\npath = "src/main.rs"' >> Cargo.toml
RUN cargo build --release --bin server

FROM debian:12.9-slim

# ARG APP_PORT

RUN apt-get update && apt-get install -y libssl-dev libpq-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=build /app/target/release/server /app/

EXPOSE $APP_PORT

# You can use some environment variable set here during runtime like --port. Check src/config.rs for more.
# CMD ["./server", "--port", "$APP_PORT"]

CMD exec ./server --port $APP_PORT
