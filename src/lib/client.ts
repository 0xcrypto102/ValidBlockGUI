import { createConnectTransport } from '@connectrpc/connect-web';
import { createPromiseClient } from '@connectrpc/connect';
import { AnchorService, VerifyService } from '../gen/proto/validblock_connect';

const transport = createConnectTransport({
  baseUrl: 'http://127.0.0.1:8080', // gRPC-Web backend port (Tauri sidecar)
  useBinaryFormat: true,
  // interceptors: [
  //   {
  //     async interceptUnary(next, req) {
  //       req.header.set('Authorization', '111');
  //       return next(req);
  //     }
  //   }
  // ]
});

export const anchorClient = createPromiseClient(AnchorService, transport);
export const verifyClient = createPromiseClient(VerifyService, transport);
