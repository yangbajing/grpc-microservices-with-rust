import { createChannel, createClient } from "nice-grpc";
import { AuthClient, AuthDefinition } from "@/pb/getting/v1/auth";

export const channel = createChannel("localhost:9999");

export const authClient: AuthClient = createClient(AuthDefinition, channel);
