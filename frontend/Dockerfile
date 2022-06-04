# Dockerfile context should be in the project root!
ARG NODE_VERSION=18.2.0

FROM node:${NODE_VERSION} as builder
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/app/frontend
COPY frontend/package.json frontend/yarn.lock frontend/svelte.config.js ./
RUN yarn install --immutable --immutable-cache --check-cache

COPY ./frontend ./
COPY ./proto ../proto
RUN yarn build

FROM node:${NODE_VERSION}-alpine as runtime

WORKDIR /usr/app/run
COPY --from=builder /usr/app/frontend/package.json .
COPY --from=builder /usr/app/frontend/node_modules ./node_modules
COPY --from=builder /usr/app/frontend/build ./build

EXPOSE 3000
CMD ["node", "./build/index.js"]
