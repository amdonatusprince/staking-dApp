import {
  BrowserWalletConnector,
  CONCORDIUM_WALLET_CONNECT_PROJECT_ID,
  persistentConnectorType,
  WalletConnectConnector,
} from "@concordium/react-components";

export const DEFAULT_CONTRACT_INDEX = BigInt(10391);
export const MODULE_REF = "2188a1b4715a1991b2c0d852cc0663df3cef62967e47feee305c1799ecd49671"
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
