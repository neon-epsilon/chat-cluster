# Create an image that has only our Cargo.toml files without our actual source
# code; we will use this to cache a builder image that already has the
# dependencies from our Cargo.tomls compiled.
FROM rust:1.66-slim as dependencies

# We need a sample main.rs file.
RUN cargo new template_binary_crate
WORKDIR rust-workspace
COPY ./Cargo.toml ./Cargo.lock .
COPY ./crates/ ./crates/
COPY ./binaries/ ./binaries/
# Only keep Cargo.toml/Cargo.lock files and empty lib.rs/sample main.rs files.
RUN \
  find . -type f -not \( -name "Cargo.toml" -or -name "Cargo.lock" \) -delete && \
  find ./crates/ -maxdepth 1 -mindepth 1 -type d -exec touch {}/src/lib.rs \; && \
  find ./binaries/ -maxdepth 1 -mindepth 1 -type d -exec cp ../template_binary_crate/src/main.rs {}/src/main.rs \;


FROM rust:1.66-slim as builder

WORKDIR rust-workspace
COPY --from=dependencies /rust-workspace/ .
# Install target platform (Cross-Compilation) (needed for Alpine).
RUN rustup target add x86_64-unknown-linux-musl
# Run cargo build once with our Cargo.tomls but without our source code to
# obtain a cached Docker layer with just our dependencies.
RUN cargo build --release --target x86_64-unknown-linux-musl

# Now copy our source code to build it for real.
COPY ./crates/ ./crates/
COPY ./binaries/ ./binaries/
# Touch main.rs/lib.rs to prevent cached release builds.
RUN \
  find ./crates/ -maxdepth 1 -mindepth 1 -type d -exec touch {}/src/lib.rs \; && \
  find ./binaries/ -maxdepth 1 -mindepth 1 -type d -exec touch {}/src/main.rs \;

RUN cargo build --release --target x86_64-unknown-linux-musl


# An image ready to host a web server
FROM alpine:3.16 AS web-server-base

RUN \
  apk update && \
  apk add --no-cache ca-certificates tzdata && \
  rm -rf /var/cache/apk/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER


# The actual images
FROM web-server-base AS chat-server
WORKDIR /usr/src/chat-server

COPY --from=builder /rust-workspace/target/x86_64-unknown-linux-musl/release/chat-server .
RUN chown -R $APP_USER:$APP_USER .

USER $APP_USER
CMD ["./chat-server"]


FROM web-server-base AS replication-log
WORKDIR /usr/src/replication-log

COPY --from=builder /rust-workspace/target/x86_64-unknown-linux-musl/release/replication-log .
RUN chown -R $APP_USER:$APP_USER .

USER $APP_USER
CMD ["./replication-log"]
