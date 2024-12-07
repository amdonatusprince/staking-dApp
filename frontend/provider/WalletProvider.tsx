/* eslint-disable react/prop-types */
// import React from 'react'

import {
  useConnect,
  useConnection,
  useGrpcClient,
  WalletConnection,
  WalletConnectionProps,
} from "@concordium/react-components";
import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";
import { BROWSER_WALLET, DEFAULT_CONTRACT_INDEX } from "../config";
// import { initContract } from "../utils";
import {
  ConcordiumGRPCClient,
  ContractAddress,
  ContractName,
  InitName,
  InstanceInfo,
  InstanceInfoCommon,
} from "@concordium/web-sdk";
import { comma } from "postcss/lib/list";

interface WalletContextType {
  walletProps: WalletConnectionProps | null;
  rpc: ConcordiumGRPCClient | undefined; // You can type this appropriately
  connection: WalletConnection | undefined; // Add correct type
  connect: (() => void) | undefined;
  isConnecting: boolean;
  account: any; // Add correct type
  moduleSchemaBase64Embedded: string; // Add correct type
  contract: any;
}

const WalletContext = createContext<WalletContextType>({
  walletProps: null,
  rpc: undefined,
  connection: undefined,
  connect: () => {},
  isConnecting: false,
  account: null,
  moduleSchemaBase64Embedded: "",
  contract: null,
});

interface WalletProviders {
  walletProps: WalletConnectionProps;
  children: ReactNode;
}

const WalletProvider = ({ children, walletProps }: WalletProviders) => {
  const [contract, setContract] = useState<any>();
  const [schema, setSchema] = useState<ArrayBufferLike | null>(null);

  const {
    setActiveConnectorType,
    activeConnector,
    connectedAccounts,
    genesisHashes,
    network,
  } = walletProps;

  const rpc = useGrpcClient(network);

  useEffect(() => {
    setActiveConnectorType(BROWSER_WALLET);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const { connection, setConnection, account } = useConnection(
    connectedAccounts,
    genesisHashes
  );

  const { connect, isConnecting } = useConnect(activeConnector, setConnection);

  async function initContract(
    rpc: ConcordiumGRPCClient | undefined,
    index: bigint
  ) {
    console.debug(`Refreshing info for contract ${index.toString()}`);
    const info = await rpc?.getInstanceInfo(ContractAddress.create(index, 0));
    if (!info) {
      throw new Error(`contract ${index} not found`);
    }

    const { version, name, owner, amount, methods, sourceModule } = info;
    const prefix = "init_";
    if (!InitName.toString(name).startsWith(prefix)) {
      throw new Error(`name "${name}" doesn't start with "init_"`);
    }
    return {
      version,
      index,
      name: ContractName.fromInitName(name),
      amount,
      owner,
      methods,
      sourceModule,
    };

    // return info;
  }

  useEffect(() => {
    const getContract = async () => {
      const contractValue =
        rpc && (await initContract(rpc, DEFAULT_CONTRACT_INDEX));
      setContract(contractValue);
    };
    getContract();
  }, [rpc]);

  useEffect(() => {
    const getSchema = async () => {
      if (rpc && contract) {
        try {
          // Get the embedded schema from the contract's source module
          const schema = await rpc.getEmbeddedSchema(contract?.sourceModule);
          setSchema(schema);
        } catch (error) {
          console.error("Error initializing contract:", error);
        }
      }
    };

    getSchema();
  }, [rpc, contract]);

  const moduleSchemaBase64Embedded = btoa(
    new Uint8Array(schema as ArrayBufferLike).reduce(
      (data, byte) => data + String.fromCharCode(byte),
      ""
    )
  );

  return (
    <WalletContext.Provider
      value={{
        walletProps,
        rpc,
        connection,
        connect,
        isConnecting,
        account,
        moduleSchemaBase64Embedded,
        contract,
      }}
    >
      {children}
    </WalletContext.Provider>
  );
};

export default WalletProvider;

export const useWallet = () => {
  return useContext(WalletContext);
};
