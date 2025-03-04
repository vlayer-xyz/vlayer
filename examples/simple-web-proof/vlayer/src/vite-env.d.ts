/// <reference types="vite/client" />
import { ClientAuthMode } from "./hooks/useAddress.ts";

interface ImportMetaEnv {
  readonly VITE_CLIENT_AUTH_MODE: ClientAuthMode;
}
