export const RPC_BASE_URL =
  // Allows override via vite env or Settings pane
  import.meta.env.VITE_RPC_URL ?? "http://127.0.0.1:8080";