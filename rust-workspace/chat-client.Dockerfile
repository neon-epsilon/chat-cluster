FROM rust:1.62-slim as builder

# Run cargo build once with our Cargo.tomls but without our source code to
# cache dependencies.
WORKDIR rust-workspace
## Install target platform (Cross-Compilation) (needed for Alpine) and
## initialize the crates.
RUN \
  rustup target add x86_64-unknown-linux-musl && \
  cargo new --lib crates/common && \
  cargo new --bin binaries/replication-log && \
  cargo new --bin binaries/chat-client
COPY ./Cargo.toml ./Cargo.lock .
COPY ./crates/common/Cargo.toml ./crates/common/
COPY ./binaries/replication-log/Cargo.toml ./binaries/replication-log/
COPY ./binaries/chat-client/Cargo.toml ./binaries/chat-client/
## Build dependencies.
RUN cargo build --release --target x86_64-unknown-linux-musl

# Now copy our source code and build it for real.
RUN rm ./crates/common/src/*.rs ./binaries/chat-client/src/*.rs ./binaries/replication-log/src/*.rs
COPY ./crates/common/src/ ./crates/common/src/
COPY ./binaries/chat-client/src/ ./binaries/chat-client/src/
COPY ./binaries/replication-log/src/ ./binaries/replication-log/src/

## Touch main.rs to prevent cached release build
RUN \
  touch ./crates/common/src/lib.rs && \
  touch ./binaries/replication-log/src/main.rs && \
  touch ./binaries/chat-client/src/main.rs

RUN cargo build -p chat-client --release --target x86_64-unknown-linux-musl


# The actual image containing the app
FROM alpine:3.16 AS runtime
ARG APP=/usr/src/app

RUN \
  apk update && \
  apk add --no-cache ca-certificates tzdata && \
  rm -rf /var/cache/apk/*

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
