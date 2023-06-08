# Stage 1: Debug
FROM rust:1.68 as debug

WORKDIR /usr/src/app

# Install Essential Build Tools
RUN apt-get update && apt-get install -y build-essential

# Needed for VS Code Remote Development
RUN apt-get install -y musl-tools libgcc-$(dpkg --print-architecture)1 libstdc++6 git curl wget bash ca-certificates openssh-client

RUN cargo install cargo-watch

COPY . .

# Build to cache dependenciess
RUN cargo build

# Run the project and watch only `src` for changes.
CMD ["cargo", "watch", "-x", "run", "-w", "src"]

# Stage 2: Build
FROM rust:1.68 as builder
WORKDIR /usr/src/app

# Install Essential Build Tools
RUN apt-get update && apt-get install -y build-essential

COPY . .

# Build for release.
RUN cargo build --release

# Stage 3: Release
FROM alpine as release
# Add necessary runtime dependencies
RUN apk add --no-cache libgcc libstdc++
COPY --from=builder /usr/src/app/target/release/actix-books-app /actix-books-app

CMD ["/actix-books-app"]
