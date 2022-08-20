FROM rust:1.62-slim as builder

# Run cargo build once with just our Cargo.toml to cache dependencies.
RUN USER=root cargo new --bin chat-client
WORKDIR ./chat-client
## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl
COPY ./Cargo.toml ./Cargo.lock .
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN rm src/*.rs

# Now copy our source code and build it for real.
COPY ./src ./src
## Touch main.rs to prevent cached release build
RUN touch ./src/main.rs
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

COPY --from=builder /chat-client/target/x86_64-unknown-linux-musl/release/chat-client ${APP}/chat-client

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./chat-client"]
