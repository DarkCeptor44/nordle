# stage 1: chef
FROM lukemathwalker/cargo-chef:latest-rust-1.88.0 AS chef
WORKDIR /app

# stage 2: planner
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# stage 3: builder
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

# stage 4: final build
FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/nordle /usr/local/bin/nordle

ENV NORDLE_HOST=0.0.0.0
ENV NORDLE_PORT=8080

EXPOSE 8080

ENTRYPOINT ["nordle"]
