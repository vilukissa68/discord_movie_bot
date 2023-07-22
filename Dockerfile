FROM rust:latest AS builder
RUN update-ca-certificates

WORKDIR /app
COPY . .
RUN cargo build --release


FROM debian:bullseye-slim

WORKDIR /app
COPY --from=builder ./app/target/release/discord_movie_bot ./
