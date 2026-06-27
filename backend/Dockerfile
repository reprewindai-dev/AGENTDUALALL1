# Multi-stage production build recipe for High-Performance Rust service
# Leverages caching mechanics for ultra-fast compilation turnarounds

# Step 1: Planning / Cargo Chef preparation
FROM clux/muslrust:1.78.0-stable AS planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Step 2: Dependencies caching
FROM clux/muslrust:1.78.0-stable AS cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

# Step 3: Source compilation
FROM clux/muslrust:1.78.0-stable AS builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --target x86_64-unknown-linux-musl --bin agent_duel_backend

# Step 4: Scrap runtime construction 
FROM alpine:3.19
WORKDIR /app

# Ensure standard certificates are available for external Base.org API lookups
RUN apk --no-cache add ca-certificates tzdata

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/agent_duel_backend ./agent_duel_backend
COPY --from=builder /app/migrations ./migrations

ENV PORT=3000
ENV NODE_ENV=production
EXPOSE 3000

CMD ["./agent_duel_backend"]
