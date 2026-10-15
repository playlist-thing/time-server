# syntax=docker/dockerfile:1

FROM rust:1.99-alpine3.23 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked

FROM alpine:3.23 AS runtime

RUN apk add --no-cache curl \
    && addgroup -g 10001 appuser \
    && adduser -D -u 10001 -G appuser appuser

WORKDIR /app
COPY --from=builder /app/target/release/time-server /usr/local/bin/time-server

USER appuser

ENV BIND_ADDR=0.0.0.0:3000
EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -fsS http://localhost:3000/api/health || exit 1

ENTRYPOINT ["time-server"]
