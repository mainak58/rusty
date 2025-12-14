# 1. Builder image
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# 2. Runtime image
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/backend .
ENV PORT=8000
EXPOSE 8000
CMD ["./backend"]
