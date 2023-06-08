# Stage 1: Debug
FROM rust:1.68-alpine as debug

WORKDIR /usr/src/app

# Needed for VS Code Remote Development
RUN apk add musl libgcc libstdc++ git curl wget bash ca-certificates gcompat openssh-client
# Install Essential Build Tools
RUN apk add build-base musl-dev
# Download the target for this system. This will be the Rust standard library.
RUN rustup target add x86_64-unknown-linux-musl

RUN cargo install cargo-watch

COPY . .

# Build to cache dependencies
RUN RUN cargo build --target x86_64-unknown-linux-musl

# Run the project and watch only `src` for changes.
CMD ["cargo", "watch", "-x", "run", "-w", "src"]

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

