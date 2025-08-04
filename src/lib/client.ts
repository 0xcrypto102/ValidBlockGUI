import { createConnectTransport } from "@connectrpc/connect-web";
import { createPromiseClient } from "@connectrpc/connect";
import { AnchorService, VerifyService } from "../gen/proto/validblock_connect";
import { RPC_BASE_URL } from "./config";

const transport = createConnectTransport({ baseUrl: RPC_BASE_URL });

export const anchorClient = createPromiseClient(AnchorService, transport);
export const verifyClient = createPromiseClient(VerifyService, transport);
