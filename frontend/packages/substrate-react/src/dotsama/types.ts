import { ApiPromise } from "@polkadot/api";
import { DEFI_CONFIG } from "./config";

export type TokenId = typeof DEFI_CONFIG.tokenIds[number];
export type ParachainId = typeof DEFI_CONFIG.parachainIds[number];
export type RelayChainId = typeof DEFI_CONFIG.relayChainIds[number];
export type AccountType = "secp256k1" | "*25519";

export type SubstrateChainApiStatus = "initializing" | "failed" | "connected";

export type DotSamaExtensionStatus =
  | "initializing"
  | "connecting"
  | "connected"
  | "no_extension"
  | "error";

export interface ConnectedAccount {
  address: string;
  name: string;
}

export interface SubstrateChainApi {
  parachainApi: ApiPromise | undefined;
  apiStatus: SubstrateChainApiStatus;
  prefix: number;
  accounts: ConnectedAccount[];
}

export interface ParachainApi extends SubstrateChainApi {
  chainId: ParachainId;
}

export interface RelaychainApi extends SubstrateChainApi {
  chainId: RelayChainId;
}

export interface DotSamaContext {
  parachainProviders: { [chainId in ParachainId]: ParachainApi };
  relaychainProviders: { [chainId in RelayChainId]: RelaychainApi };
  extensionStatus: DotSamaExtensionStatus;
  activate?: (setDefaultAccount?: boolean) => Promise<any[] | undefined>;
  deactivate?: () => Promise<void>;
  selectedAccount: number;
  setSelectedAccount?: (account: number) => void;
}

export interface SubstrateNetwork {
  name: string;
  wsUrl: string;
  tokenId: TokenId;
  prefix: number;
  accountType: AccountType;
  subscanUrl: string;
  decimals: number;
  color?: string;
  symbol: string;
  logo: string;
}

export interface ParachainNetwork extends SubstrateNetwork {
  parachainId: number;
  relayChain: RelayChainId;
}

export interface RelaychainNetwork extends SubstrateNetwork {
  networkId: RelayChainId;
}
