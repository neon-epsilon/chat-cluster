# Create an image that has only our Cargo.toml files without our actual source
# code; we will use this to cache a builder image that already has the
# dependencies from our Cargo.tomls compiled.
FROM rust:1.62-slim as dependencies

RUN cargo new template_binary_crate
WORKDIR rust-workspace
COPY ./Cargo.toml ./Cargo.lock .
COPY ./crates/ ./crates/
COPY ./binaries/ ./binaries/
RUN \
  find . -type f -not \( -name "Cargo.toml" -or -name "Cargo.lock" \) -delete && \
  find ./crates/ -maxdepth 1 -mindepth 1 -type d -exec touch {}/src/lib.rs \; && \
  find ./binaries/ -maxdepth 1 -mindepth 1 -type d -exec cp ../template_binary_crate/src/main.rs {}/src/main.rs \;


FROM rust:1.62-slim as builder

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
