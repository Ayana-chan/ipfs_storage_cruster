# cargo with source replacement
# Comment the `RUN` if you are not in China.
FROM rust:1.75.0-bookworm as source-replaced-cargo

RUN mkdir -p /usr/local/cargo/registry \
    && echo '[source.crates-io]' > /usr/local/cargo/config.toml \
    && echo 'replace-with = "rsproxy"' >> /usr/local/cargo/config.toml \
    && echo '[source.rsproxy]' >> /usr/local/cargo/config.toml \
    && echo 'registry = "https://rsproxy.cn/crates.io-index"' >> /usr/local/cargo/config.toml

# cargo-chef
FROM source-replaced-cargo as chef

RUN cargo install cargo-chef

# Computes the recipe file
FROM chef as planner

WORKDIR /app
COPY . .

RUN cargo chef prepare --recipe-path recipe.json

# Build app
FROM chef as builder

WORKDIR /app

COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer.
# No source code here, so as long as the dependency remains unchanged, it will not be rebuilt.
RUN cargo chef cook --release --recipe-path recipe.json

ARG APP_NAME

COPY . .

# Build
RUN cd ./crates/${APP_NAME} \
    && cargo build --release

# Run app
FROM debian:bookworm-slim

EXPOSE 3000 4000

ARG APP_NAME
COPY --from=builder /app/target/release/${APP_NAME} /usr/local/bin/${APP_NAME}

ENV APP_NAME_ENV=${APP_NAME}
CMD ["sh", "-c", "/usr/local/bin/$APP_NAME_ENV"]
