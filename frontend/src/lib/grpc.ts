import { browser, dev } from "$app/env";
import type { RpcTransport } from "@protobuf-ts/runtime-rpc";

export async function transport(): Promise<RpcTransport> {
  if (browser) {
    const { GrpcWebFetchTransport } = await import("@protobuf-ts/grpcweb-transport");
    return new GrpcWebFetchTransport({ baseUrl: "/rpc/", format: "binary" });
  } else {
    const { GrpcTransport } = await import("@protobuf-ts/grpc-transport");
    const { ChannelCredentials } = await import("@grpc/grpc-js");
    return new GrpcTransport({
      host: dev ? "localhost:9090" : "backend:9090",
      channelCredentials: ChannelCredentials.createInsecure(),
    });
  }
}
