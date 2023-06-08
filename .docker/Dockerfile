# Stage 1: Debug
FROM rust:1.68 as debug

WORKDIR /usr/src/app

# Needed for VS Code Remote Development
RUN apt-get update && \
    apt-get install -y gcc libc6-dev git curl wget bash ca-certificates openssh-client build-essential

RUN cargo install cargo-watch

COPY . .

RUN cargo build

CMD ["cargo", "watch", "-x", "run"]

# Stage 2: Build
FROM rust:1.68-alpine as builder
WORKDIR /usr/src/app

# Install Essential Build Tools
RUN apk add build-base musl-dev
# Download the target for this system. This will be the Rust standard library.
RUN rustup target add x86_64-unknown-linux-musl

COPY . .

# Build for release.
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Stage 3: Release
FROM alpine as release
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/actix-books-app /actix-books-app

CMD ["/actix-books-app"]

