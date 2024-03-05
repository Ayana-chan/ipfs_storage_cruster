FROM rust:latest as builder
ARG APP_NAME
WORKDIR /usr/src/${APP_NAME}
COPY . .
RUN cd ./crates/${APP_NAME} && \
    cargo build --release

FROM debian:buster-slim
ARG APP_NAME
COPY --from=builder /usr/src/${APP_NAME}/target/release/${APP_NAME} /usr/local/bin/${APP_NAME}
CMD ["sh", "-c", "/usr/local/bin/$APP_NAME"]
