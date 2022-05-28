# Dockerfile context should be in the project root!
ARG RUST_VERSION=1.61.0
ARG OUT_BINARY_NAME=flux-web-auth-backend

FROM rust:${RUST_VERSION} as builder

RUN apt-get update && apt-get install -y cmake && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/app/
COPY backend ./backend
COPY proto ./proto

WORKDIR /usr/app/backend
RUN cargo build --release

ENV OUT_BINARY_NAME=$OUT_BINARY_NAME
RUN strip "./target/release/${OUT_BINARY_NAME}"

FROM rust:${RUST_VERSION} as runtime

COPY --from=builder /usr/app/backend/target/release/${OUT_BINARY_NAME} /usr/local/bin

ENV OUT_BINARY_NAME=$OUT_BINARY_NAME
CMD ["/usr/local/bin/${OUT_BINARY_NAME}"]
