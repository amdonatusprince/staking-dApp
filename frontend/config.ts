import {
  BrowserWalletConnector,
  CONCORDIUM_WALLET_CONNECT_PROJECT_ID,
  persistentConnectorType,
  WalletConnectConnector,
} from "@concordium/react-components";

export const DEFAULT_CONTRACT_INDEX = BigInt(10404);
export const MODULE_REF = "347cd8d67b9accf8a21edd1827295e3e0544ada05419461d2063e61cb6b8a901"
// export const DEFAULT_CONTRACT_INDEX = BigInt(10398);
// export const MODULE_REF = "8cb9abab787be7407d4f7b74f74a80525c4f9a942af3524fee143ff4d8a18f02"

export const MAX_CONTRACT_EXECUTION_ENERGY = BigInt(30000);

export const CONTRACT_NAME = "concordium_staking";
export const CONTRACT_SUB_INDEX = BigInt(0);
export const MICRO_CCD = 1000000;
export const MULTIPLIER = 1000_000;
export const APR_DENOMINATOR = 1_000_000;
export const DAY_IN_SECONDS = 86400;


const WALLET_CONNECT_OPTS = {
  projectId: CONCORDIUM_WALLET_CONNECT_PROJECT_ID,
  metadata: {
    name: "Staking Dapp",
    description: "Stake your $EUROe and get daily reward incentive",
    url: "#",
    icons: ["https://walletconnect.com/walletconnect-logo.png"],
  },
};

export const BROWSER_WALLET = persistentConnectorType(
  BrowserWalletConnector.create
);
export const WALLET_CONNECT = persistentConnectorType(
  WalletConnectConnector.create.bind(this, WALLET_CONNECT_OPTS)
);
