# Stage 1: Build
FROM rust:1.68-alpine as builder

# Build argument to determine if we're doing a release or debug build
ARG BUILD_MODE=debug

WORKDIR /usr/src/actix-books-app

# Needed for VS Code Remote Development
RUN apk add musl libgcc libstdc++ git curl wget bash ca-certificates gcompat openssh-client

# Install musl-tools
RUN apk add musl-dev

# Download the target for this system. This will be the Rust standard library.
RUN rustup target add x86_64-unknown-linux-musl

COPY . .

# If build mode is 'debug', build without --release flag. Otherwise, build with --release flag.
RUN if [ "$BUILD_MODE" = "debug" ] ; then cargo build --target=x86_64-unknown-linux-musl ; else cargo build --release --target=x86_64-unknown-linux-musl ; fi

# Stage 2: Setup the runtime environment
FROM alpine

ARG BUILD_MODE=debug

# If build mode is 'release', copy from release folder. Otherwise, copy from debug folder.
COPY --from=builder /usr/src/actix-books-app/target/x86_64-unknown-linux-musl/${BUILD_MODE}/actix-books-app /actix-books-app

CMD ["/actix-books-app"]
