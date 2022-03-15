# Leveraging the pre-built Docker images with 
# cargo-chef and the Rust toolchain
FROM lukemathwalker/cargo-chef:latest-rust-1.59 AS chef
WORKDIR app
RUN cargo install sccache
ENV RUSTC_WRAPPER=sccache

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS cook 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

FROM cook as dev
COPY . .

FROM cook as builder
# Build application
COPY . .
RUN cargo build --release 

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/release/chicken-bot /usr/local/bin
ENTRYPOINT ["/usr/local/bin/chicken-bot"]