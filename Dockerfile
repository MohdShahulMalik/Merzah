# syntax=docker/dockerfile:1

# Builds the Merzah Leptos SSR app for Hostless (or any Docker host).
# Runtime: Debian slim, non-root not used (Hostless runs the container as-is;
# the app binds 0.0.0.0:$PORT).

# ---------- CSS stage ----------
FROM oven/bun:1-debian AS css
WORKDIR /app
COPY package.json bun.lock postcss.config.js ./
COPY style ./style
RUN bun install --frozen-lockfile \
    && bun run build:css

# ---------- Rust build stage ----------
FROM rust:1-bookworm AS build
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        curl \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown

# Prebuilt cargo-leptos; compiling it from source would take 10+ minutes.
ARG CARGO_LEPTOS_VERSION=0.3.11
RUN curl --proto '=https' --tlsv1.2 -LsSf \
        "https://github.com/leptos-rs/cargo-leptos/releases/download/v${CARGO_LEPTOS_VERSION}/cargo-leptos-installer.sh" \
        | sh

WORKDIR /app
# Cargo.lock is gitignored in this repo today; the "Cargo.lock*" glob keeps the
# build working either way. See .gitignore note in DEPLOYMENT.md.
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
COPY assets ./assets
COPY --from=css /app/style/output ./style/output

RUN cargo leptos build --release

# ---------- Runtime stage ----------
FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=build /app/target/release/merzah /app/merzah
COPY --from=build /app/target/site /app/site

ENV LEPTOS_SITE_ROOT=/app/site \
    LEPTOS_ENV=PROD

# Hostless sets PORT; default to 8080 for local docker runs.
EXPOSE 8080
CMD ["sh", "-c", "LEPTOS_SITE_ADDR=0.0.0.0:${PORT:-8080} ./merzah"]
