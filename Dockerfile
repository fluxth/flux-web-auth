# Phase 1: Get version from git describe
FROM alpine:3.15.3 as version
RUN apk add --no-cache git

WORKDIR /version

COPY .git ./.git
RUN git describe | tr -d '\n' > version.txt

# Phase 2: Build static assets
FROM node:16.13 AS webpack-build
WORKDIR /usr/app/src

COPY webpack ./webpack
RUN cd webpack && \
    yarn install --frozen-lockfile && \
    NODE_ENV=production yarn build

# Phase 3: Build server binary
FROM ekidd/rust-musl-builder:stable AS rust-build
WORKDIR /usr/app/src

RUN rustup target add x86_64-unknown-linux-musl
USER root

COPY Cargo.toml Cargo.lock ./
RUN mkdir .cargo && \
    cargo vendor > .cargo/config

COPY src ./src
COPY migrations ./migrations
COPY --from=version /version/version.txt ./src/
RUN cargo install --target x86_64-unknown-linux-musl --path . --root /usr/local/cargo && \
    strip /usr/local/cargo/bin/flux-web-auth

# Phase 4: Runtime
FROM scratch AS runtime
WORKDIR /usr/app/run

COPY --from=webpack-build /usr/app/src/static ./static
COPY --from=rust-build /usr/local/cargo/bin/flux-web-auth .
COPY templates ./templates
COPY Rocket.toml .

EXPOSE 8000
CMD ["./flux-web-auth"]
