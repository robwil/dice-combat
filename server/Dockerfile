# Dockerfile from https://github.com/LukeMathWalker/cargo-chef

FROM rust as planner
WORKDIR /app
# We only pay the installation cost once, 
# it will be cached from the second build onwards
RUN cargo install cargo-chef 
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR /app
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release --bin dice-combat

FROM rust as runtime
WORKDIR /app
ENV PORT 9000
EXPOSE 9000
COPY --from=builder /app/target/release/dice-combat /usr/local/bin
ENTRYPOINT ["/usr/local/bin/dice-combat"]
