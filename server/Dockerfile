FROM rust:1.62 as builder

# Run cargo build once with just our Cargo.toml to cache dependencies.
RUN USER=root cargo new --bin server
WORKDIR ./server
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

# Now copy our source code and build it for real.
COPY . ./
RUN rm ./target/release/deps/server*
RUN cargo build --release


# The actual image containing the app
FROM debian:buster-slim as server
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /server/target/release/server ${APP}/server

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./server"]
