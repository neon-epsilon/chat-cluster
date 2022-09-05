FROM rust:1.62-slim as builder

# Run cargo build once with our Cargo.tomls but without our source code to
# cache dependencies.
RUN mkdir rust-workspace
WORKDIR rust-workspace
COPY ./Cargo.toml ./Cargo.lock .
## Install target platform (Cross-Compilation). Needed for Alpine.
RUN rustup target add x86_64-unknown-linux-musl
## Initialize the crates.
RUN cargo new --lib common
COPY ./common/Cargo.toml ./common
RUN cargo new --bin replication-log
COPY ./replication-log/Cargo.toml ./replication-log
RUN cargo new --bin chat-client
COPY ./chat-client/Cargo.toml ./chat-client
## Build dependencies.
RUN cargo build --release --target x86_64-unknown-linux-musl

# Now copy our source code and build it for real.
RUN rm ./common/src/*.rs
COPY ./common/src ./common/src
RUN rm ./chat-client/src/*.rs
COPY ./chat-client/src ./chat-client/src
RUN rm ./replication-log/src/*.rs
COPY ./replication-log/src ./replication-log/src
## Touch main.rs to prevent cached release build
RUN touch ./chat-client/src/main.rs

RUN cargo build --release --target x86_64-unknown-linux-musl


# The actual image containing the app
FROM alpine:3.16 AS runtime
ARG APP=/usr/src/app

RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

COPY --from=builder /rust-workspace/target/x86_64-unknown-linux-musl/release/chat-client ${APP}/chat-client

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./chat-client"]
