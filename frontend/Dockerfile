# Dockerfile context should be in the project root!
ARG NODE_VERSION=18.2.0

FROM node:${NODE_VERSION} as builder

WORKDIR /usr/app/build

COPY frontend/package.json frontend/yarn.lock ./
RUN yarn install --immutable --immutable-cache --check-cache

COPY ./frontend ./
RUN yarn build

FROM node:${NODE_VERSION}-alpine as runtime

WORKDIR /usr/app/run
COPY --from=builder /usr/app/build/ ./

EXPOSE 3000
CMD ["node", "./build/index.js"]
