# Phase 1: Build static assets
FROM node:16.13 AS webpack-build
WORKDIR /usr/app/src

COPY webpack ./webpack
RUN cd webpack && \
    yarn install --frozen-lockfile && \
    NODE_ENV=production yarn build

# Phase 2: Build server binary
FROM ekidd/rust-musl-builder:stable AS rust-build
WORKDIR /usr/app/src

RUN rustup target add x86_64-unknown-linux-musl

COPY . .

USER root
RUN cargo install --target x86_64-unknown-linux-musl --path . --root /usr/local/cargo && \
    strip /usr/local/cargo/bin/flux-web-auth

# Phase 3: Runtime
FROM scratch AS runtime
WORKDIR /usr/app/run

COPY templates ./templates
COPY --from=webpack-build /usr/app/src/static ./static
COPY --from=rust-build /usr/local/cargo/bin/flux-web-auth .

EXPOSE 8000
CMD ["./flux-web-auth"]
