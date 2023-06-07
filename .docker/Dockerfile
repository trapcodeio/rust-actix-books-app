# Stage 1: Debug
FROM rust:1.68-alpine as debug

WORKDIR /usr/src/app

# Needed for VS Code Remote Development
RUN apk add musl libgcc libstdc++ git curl wget bash ca-certificates gcompat openssh-client
# Install musl-tools
RUN apk add musl-dev
# Download the target for this system. This will be the Rust standard library.
RUN rustup target add x86_64-unknown-linux-musl

COPY . .

RUN cargo build --target=x86_64-unknown-linux-musl
CMD ["./target/x86_64-unknown-linux-musl/debug/actix-books-app"]

# Stage 2: Build
FROM rust:1.68-alpine as builder
WORKDIR /usr/src/app

# Needed for VS Code Remote Development
RUN apk add musl-dev
# Download the target for this system. This will be the Rust standard library.
RUN rustup target add x86_64-unknown-linux-musl

COPY . .

# Build for release.
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Stage 3: Release
FROM alpine as release
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/actix-books-app /actix-books-app

CMD ["/actix-books-app"]

