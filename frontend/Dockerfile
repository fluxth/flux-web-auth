# Dockerfile context should be in the project root!
ARG NODE_VERSION=18.2.0

FROM node:${NODE_VERSION} as builder
ARG GRPC_WEB_VERSION=1.3.1

RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

WORKDIR /root
RUN curl -OL "https://github.com/grpc/grpc-web/releases/download/${GRPC_WEB_VERSION}/protoc-gen-grpc-web-${GRPC_WEB_VERSION}-linux-x86_64" && \
    curl -OL "https://github.com/grpc/grpc-web/releases/download/${GRPC_WEB_VERSION}/protoc-gen-grpc-web-${GRPC_WEB_VERSION}-linux-x86_64.sha256"
RUN cat protoc-gen-grpc-web-${GRPC_WEB_VERSION}-linux-x86_64.sha256 | sha256sum --check --status
RUN mv protoc-gen-grpc-web-${GRPC_WEB_VERSION}-linux-x86_64 /usr/local/bin/protoc-gen-grpc-web && \
    chmod +x /usr/local/bin/protoc-gen-grpc-web

WORKDIR /usr/app/frontend

COPY frontend/package.json frontend/yarn.lock frontend/svelte.config.js ./
RUN yarn install --immutable --immutable-cache --check-cache

COPY ./frontend ./
COPY ./scripts ../scripts
COPY ./proto ../proto
RUN mkdir -p proto && \
    yarn build

FROM node:${NODE_VERSION}-alpine as runtime

WORKDIR /usr/app/run
COPY --from=builder /usr/app/frontend/package.json .
COPY --from=builder /usr/app/frontend/build ./build
COPY --from=builder /usr/app/frontend/node_modules ./node_modules

EXPOSE 3000
CMD ["node", "./build/index.js"]
