# stage 1: planner
FROM rust:1.88-bookworm AS planner
ARG BUILD_JOBS=1
WORKDIR /app
RUN cargo install cargo-chef --jobs $BUILD_JOBS
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# stage 2: cacher
FROM rust:1.88-bookworm AS cacher
ARG BUILD_JOBS=1
WORKDIR /app
RUN cargo install cargo-chef --jobs $BUILD_JOBS
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --jobs $BUILD_JOBS

# stage 3: builder
FROM rust:1.88-bookworm AS builder
ARG BUILD_JOBS=1
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --jobs $BUILD_JOBS

# stage 4: final build
FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/nordle /usr/local/bin/nordle

ENV NORDLE_HOST=0.0.0.0
ENV NORDLE_PORT=8080

EXPOSE 8080

ENTRYPOINT ["nordle"]
