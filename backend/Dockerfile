# Dockerfile context should be in the project root!
ARG RUST_VERSION=1.61.0

FROM rust:${RUST_VERSION} as builder

RUN apt-get update && apt-get install -y cmake && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/app/
COPY backend ./backend
COPY proto ./proto

WORKDIR /usr/app/backend
RUN cargo build --release --locked --target x86_64-unknown-linux-musl
RUN strip "./target/x86_64-unknown-linux-musl/release/flux-web-auth-backend"

FROM scratch as runtime

COPY --from=builder /usr/app/backend/target/x86_64-unknown-linux-musl/release/flux-web-auth-backend /

EXPOSE 9090
CMD ["/flux-web-auth-backend"]
