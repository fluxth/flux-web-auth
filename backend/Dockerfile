# Dockerfile context should be in the project root!
FROM ghcr.io/fluxth/rust-musl-builder:1.62.0 AS rust-build

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /home/rust/app/
COPY proto ./proto

WORKDIR /home/rust/app/backend
RUN sudo chown -R rust:rust /home/rust/app

COPY backend/build.rs .
COPY backend/Cargo.toml .
COPY backend/Cargo.lock .
RUN echo 'fn main() {}' > dummy.rs
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release --locked --target x86_64-unknown-linux-musl

COPY backend/build.rs .
COPY backend .
RUN cargo build --release --locked --target x86_64-unknown-linux-musl
RUN strip "./target/x86_64-unknown-linux-musl/release/flux-web-auth-backend"

FROM scratch as runtime

COPY --from=rust-build /home/rust/app/backend/target/x86_64-unknown-linux-musl/release/flux-web-auth-backend /

EXPOSE 9090
CMD ["/flux-web-auth-backend"]
