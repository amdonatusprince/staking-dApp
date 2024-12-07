"use client";
import { CONTRACT_NAME, MAX_CONTRACT_EXECUTION_ENERGY } from "@/config";
import {
  AccountAddress,
  ConcordiumGRPCClient,
  ContractAddress,
  ContractName,
  deserializeReceiveReturnValue,
  Energy,
  EntrypointName,
  ModuleReference,
  ReceiveName,
  SchemaVersion,
  serializeUpdateContractParameters,
} from "@concordium/web-sdk";
import React from "react";
import { createContext, useContext, useState } from "react";
import toast from "react-hot-toast";
import { useWallet } from "./WalletProvider";
import { MODULE_REF } from "@/config";

interface UnbondingInfo {
  amount: string;
  unlock_time: string;
}

interface StakerInfo {
  amount: string;
  pending_rewards: string;
  slashed: boolean;
  timestamp: bigint;
  unbonding: UnbondingInfo[];
}

interface Context {
  stakerInfo: StakerInfo | null;
  setStakerInfo: React.Dispatch<React.SetStateAction<StakerInfo | null>>;
  stakeState: any;
  setStakeState: React.Dispatch<React.SetStateAction<any>>;
  loadingUserStakeInfo: boolean;
  setLoadingUserStakeInfo: React.Dispatch<React.SetStateAction<boolean>>;
  loadingProtocolStats: boolean;
  setLoadingProtocolStats: React.Dispatch<React.SetStateAction<boolean>>;
  getStakerInfo: (
    rpc: ConcordiumGRPCClient,
    account: string,
    contract: any
  ) => Promise<void>;
  viewState: (rpc: ConcordiumGRPCClient, contract: any) => Promise<void>;
  getEarnedRewards: (
    rpc: ConcordiumGRPCClient,
    account: string,
    contract: any
  ) => Promise<string>;
  earnedRewards: string;
  setEarnedRewards: React.Dispatch<React.SetStateAction<string>>;
}
const StateContext = createContext<Context>({
  stakerInfo: {
    amount: "",
    pending_rewards: "",
    slashed: false,
    timestamp: BigInt(0),
    unbonding: [],
  },
  setStakerInfo: () => {},
  stakeState: {
    active_stakers: 0,
    stakers_length: 0,
    total_rewards_paid: "0",
    total_staked: "0",
  },
  setStakeState: () => {},
  loadingUserStakeInfo: false,
  setLoadingUserStakeInfo: () => false,
  loadingProtocolStats: false,
  setLoadingProtocolStats: () => false,
  getStakerInfo: async () => {},
  viewState: async () => {},
  getEarnedRewards: async () => "0",
  earnedRewards: "0",
  setEarnedRewards: () => {},
});

interface State {
  children: React.ReactNode;
}
const StateProvider = ({ children }: State) => {
  const [stakerInfo, setStakerInfo] = useState<StakerInfo | null>(null);
  const [loadingUserStakeInfo, setLoadingUserStakeInfo] = useState(false);
  const [loadingProtocolStats, setLoadingProtocolStats] = useState(false);
  const [stakeState, setStakeState] = useState<any>({
    active_stakers: 0,
    stakers_length: 0,
    total_rewards_paid: "0",
    total_staked: "0",
  });
  const [earnedRewards, setEarnedRewards] = useState<string>("0");

  const { rpc, connection, account } = useWallet();

  const getStakerInfo = async (
    rpc: ConcordiumGRPCClient,
    account: string,
    contract: any
  ) => {
    const receiveName = "getStakeInfo";

    try {
      setLoadingUserStakeInfo(true);
      if (contract) {
        const contract_schema = await rpc?.getEmbeddedSchema(
          ModuleReference.fromHexString(MODULE_REF)
        );

        const serializedParameter = serializeUpdateContractParameters(
          ContractName.fromString(CONTRACT_NAME),
          EntrypointName.fromString(receiveName),
          account,
          contract_schema,
          SchemaVersion.V1
        );

        const result = await rpc?.invokeContract({
          contract: ContractAddress?.create(contract.index, 0),
          method: ReceiveName.create(
            contract?.name,
            EntrypointName?.fromString(receiveName)
          ),
          energy: Energy.create(MAX_CONTRACT_EXECUTION_ENERGY),
          invoker: AccountAddress?.fromJSON(account),
          parameter: serializedParameter,
        });

        if (!result?.returnValue) {
          throw new Error("No return value from contract");
        }

        const buffer = Buffer.from(result.returnValue?.buffer as Uint8Array);
        const values = await deserializeReceiveReturnValue(
          buffer,
          contract_schema,
          ContractName?.fromString(CONTRACT_NAME),
          EntrypointName?.fromString(receiveName),
          SchemaVersion?.V1
        );

        const transformedStakerInfo: StakerInfo = {
          amount: values.amount.toString(),
          pending_rewards: values.pending_rewards.toString(),
          slashed: values.slashed,
          timestamp: BigInt(values.timestamp),
          unbonding: values.unbonding.map((u: any) => ({
            amount: u.amount.toString(),
            unlock_time: u.unlock_time.toString()
          }))
        };

        setStakerInfo(transformedStakerInfo);
        setLoadingUserStakeInfo(false);
        toast.success("User Stake Information fetched successfully");
      }
    } catch (err) {
      console.error("Error fetching user stake information:", err);
      setLoadingUserStakeInfo(false);
      toast.error("Error fetching user stake information");
    }
  };

  const viewState = async (rpc: ConcordiumGRPCClient, contract: any) => {
    const receiveName = "view";

    try {
      setLoadingProtocolStats(true);
      if (contract) {
        const contract_schema = await rpc?.getEmbeddedSchema(
          ModuleReference.fromHexString(MODULE_REF)
        );

        const result = await rpc?.invokeContract({
          contract: contract && ContractAddress?.create(contract?.index, 0),
          method: contract && ReceiveName?.create(
            contract?.name,
            EntrypointName?.fromString(receiveName)
          ),
          energy: Energy.create(MAX_CONTRACT_EXECUTION_ENERGY),
        });

        if (!result?.returnValue) {
          throw new Error("No return value from contract");
        }
        const buffer = Buffer.from(result.returnValue.buffer);
        const name = ContractName?.fromString(CONTRACT_NAME);
        const entry_point = EntrypointName?.fromString(receiveName);

        const values = await deserializeReceiveReturnValue(
          buffer,
          contract_schema,
          name,
          entry_point,
          SchemaVersion?.V1
        );

        const transformedState = {
          active_stakers: values.total_participants,
          total_rewards_paid: values.total_rewards_paid.toString(),
          total_staked: values.total_staked.toString(),
        };

        // console.log("Contract state:", transformedState);
        setStakeState(transformedState);
        setLoadingProtocolStats(false);
        toast.success("Protocol statistics fetched successfully");
      }
    } catch (err) {
      console.error("Error fetching protocol stats:", err);
      setLoadingProtocolStats(false);
      toast.error("Error fetching protocol statistics");
    }
  };

  const getEarnedRewards = async (
    rpc: ConcordiumGRPCClient,
    account: string,
    contract: any
  ): Promise<string> => {
    try {
      if (!contract) {
        console.log("No contract provided");
        return "0";
      }
      const contract_schema = await rpc?.getEmbeddedSchema(
        ModuleReference.fromHexString(MODULE_REF)
      );
      const parameter = serializeUpdateContractParameters(
        ContractName.fromString(CONTRACT_NAME),
        EntrypointName.fromString("getEarnedRewards"),
        account,
        contract_schema,
        SchemaVersion.V1
      );

      const result = await rpc.invokeContract({
        contract: ContractAddress.create(contract.index, 0),
        method: ReceiveName.create(contract.name, EntrypointName.fromString("getEarnedRewards")),
        parameter: parameter,
        energy: Energy.create(BigInt(30000)),
      });

      const buffer = Buffer.from(result.returnValue?.buffer as Uint8Array);
      const values = await deserializeReceiveReturnValue(
        buffer,
        contract_schema,
        ContractName.fromString(CONTRACT_NAME),
        EntrypointName.fromString("getEarnedRewards"),
        SchemaVersion.V1
      );

      console.log(values)
      const reward = values?.toString() || "0";
      setEarnedRewards(reward);
      return reward;
    } catch (err) {
      console.error("Error fetching earned rewards:", err);
      setEarnedRewards("0");
      return "0";
    }
  };

  return (
    <StateContext.Provider
      value={{
        stakerInfo,
        setStakerInfo,
        loadingUserStakeInfo,
        setLoadingUserStakeInfo,
        loadingProtocolStats,
        setLoadingProtocolStats,
        getStakerInfo,
        viewState,
        stakeState,
        setStakeState,
        getEarnedRewards,
        earnedRewards,
        setEarnedRewards,
      }}
    >
      {children}
    </StateContext.Provider>
  );
};

export default StateProvider;

export const useStateProvider = () => {
  return useContext(StateContext);
};
