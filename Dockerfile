FROM lukemathwalker/cargo-chef:latest-rust-1.63.0 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y
FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .
ENV SQLX_OFFLINE true
# Build our project
RUN cargo build --release --bin zero2prod
FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]

#####################################################################################################
### Final image
#####################################################################################################
#FROM gcr.io/distroless/cc
#
## Import from builder.
#COPY --from=builder /etc/passwd /etc/passwd
#COPY --from=builder /etc/group /etc/group
#
#WORKDIR /app
#
## Copy our build
#COPY --from=builder /app/target/release/zero2prod ./
#COPY --from=builder /app/configuration ./configuration
#
## Use an unprivileged user.
#USER app:app
#
#ENV APP_ENVIRONMENT production
#
#ENTRYPOINT ["./zero2prod"]