import { MAX_CONTRACT_EXECUTION_ENERGY, MICRO_CCD } from "@/config";
import { useWallet } from "@/provider/WalletProvider";
import { useStateProvider } from "@/provider/StateProvider";
import { moduleSchemaFromBase64 } from "@concordium/react-components";
import {
  AccountTransactionType,
  CcdAmount,
  ContractAddress,
  Energy,
  ModuleReference,
  EntrypointName,
  ReceiveName,
} from "@concordium/web-sdk";
import React, { useState } from "react";
import toast from "react-hot-toast";
import { BeatLoader } from "react-spinners";
import { MODULE_REF } from "@/config";

const FundContract = () => {
  const [fundAmount, setFundAmount] = useState("");
  const [loadingFundContract, setLoadingFundContract] = useState(false);

  const { contract, account, connection, rpc } = useWallet();
  const { viewState } = useStateProvider();

  const fundContract = async (amount: number) => {
    try {
      setLoadingFundContract(true);
      if (!account || !connection || !rpc || !contract) {
        throw new Error("Wallet not connected");
      }

      const contract_schema = await rpc?.getEmbeddedSchema(
        ModuleReference.fromHexString(MODULE_REF)
      );

      const parameter = (amount * MICRO_CCD).toString();

      const transaction = await connection.signAndSendTransaction(
        account,
        AccountTransactionType.Update,
        {
          address: ContractAddress.create(contract.index),
          receiveName: ReceiveName.create(
            contract.name,
            EntrypointName.fromString("fundRewards")
          ),
          amount: CcdAmount.zero(),
          maxContractExecutionEnergy: Energy.create(MAX_CONTRACT_EXECUTION_ENERGY),
        },
        {
          parameters: parameter,
          schema: moduleSchemaFromBase64(btoa(
            new Uint8Array(contract_schema).reduce(
              (data, byte) => data + String.fromCharCode(byte),
              ""
            )
          ))
        }
      );

      toast.success(`Successfully funded ${amount} EUROe to the rewards pool`);
      setFundAmount("");

      // Update state after transaction
      setTimeout(async () => {
        await viewState(rpc, contract);
      }, 10000);

      setLoadingFundContract(false);
      return transaction;
    } catch (error: any) {
      if (error.message?.includes("OnlyAdmin")) {
        toast.error("Only admin can fund rewards");
      } else if (error.message?.includes("InsufficientFunds")) {
        toast.error("Insufficient EUROe balance");
      } else {
        toast.error("Error funding rewards pool");
        console.error(error);
      }
      setLoadingFundContract(false);
    }
  };

  // Only show to admin
  if (!account) {
    return null;
  }
  if (account !== contract?.owner?.address) {
    return null;
  }

  return (
    <div className="mb-10 bg-white shadow-md rounded-lg p-6 text-gray-700">
      <label htmlFor="amount" className="block text-gray-700 font-bold mb-2">
        Fund Rewards Pool (EUROe)
      </label>
      <div className="space-y-2">
        <input
          id="amount"
          type="number"
          value={fundAmount}
          onChange={(e) => setFundAmount(e.target.value)}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 mb-5"
          placeholder="Enter amount"
          required
        />
      </div>
      <button
        type="button"
        className="w-full bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded"
        onClick={() => {
          if (!account) {
            toast.error("Please connect a wallet to fund rewards");
            return;
          }
          if (Number(fundAmount) > 0) {
            fundContract(Number(fundAmount));
          } else {
            toast.error("Amount must be greater than zero");
          }
        }}
      >
        {loadingFundContract ? <BeatLoader color="#fff" /> : "Fund Rewards Pool"}
      </button>
    </div>
  );
};

export default FundContract;
