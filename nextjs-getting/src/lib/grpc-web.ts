import { Channel, Client, ClientFactory, createChannel, createClientFactory, Metadata } from "nice-grpc-web";
import { UserDefinition } from "@/pb/getting/v1/user";

const clientFactory: ClientFactory = createClientFactory().use((call, options) => {
  const token = "L1AhTRgFMiTkQMuGf8PnY6yHAmaV72ESQsEzo0cVWmiodIEx";
  return call.next(call.request, {
    ...options,
    metadata: Metadata(options.metadata).set("Authorization", `Bearer ${token}`),
  });
});

export const channel: Channel = createChannel(""); // 设置为 "" 字符串将使用从浏览器中获取的 URL 地址。
export const userClient: Client<UserDefinition> = clientFactory.create(UserDefinition, channel);
