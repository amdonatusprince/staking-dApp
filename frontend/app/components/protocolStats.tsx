import { MICRO_CCD } from "@/config";
import { useStateProvider } from "@/provider/StateProvider";
import { useWallet } from "@/provider/WalletProvider";
import { ConcordiumGRPCClient } from "@concordium/web-sdk";
import React, { useEffect } from "react";

const Card = ({ className = "", children }: any) => (
  <div className={`bg-white shadow-md rounded-lg ${className}`}>{children}</div>
);

const ProtocolStats = () => {
  const { contract, rpc } = useWallet();
  const { loadingProtocolStats, viewState, stakeState } = useStateProvider();

  useEffect(() => {
    viewState(rpc as ConcordiumGRPCClient, contract);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [contract, rpc]);

  return (
    <Card className="w-full p-8">
      <h2 className="text-2xl font-semibold text-gray-700 mb-6 flex items-center">
        <svg
          className="w-6 h-6 mr-2 text-blue-500"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
          />
        </svg>
        Protocol Statistics
      </h2>

      <div className="grid md:grid-cols-2 gap-8">
        {/* Total Value Locked */}
        <div className="bg-gradient-to-br from-blue-50 to-indigo-50 rounded-lg p-6 border border-blue-100">
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center">
              <div className="p-2 bg-blue-100 rounded-lg mr-3">
                <svg
                  className="w-5 h-5 text-blue-600"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
                  />
                </svg>
              </div>
              <h3 className="text-gray-600 font-medium">Total Value Locked</h3>
            </div>
            <span className="text-xs font-semibold px-2 py-1 bg-blue-100 text-blue-800 rounded-full">
              Live
            </span>
          </div>

          {loadingProtocolStats ? (
            <div className="mt-4 animate-pulse">
              <div className="h-8 bg-gray-200 rounded w-[130px]"></div>
            </div>
          ) : (
            <div className="flex items-baseline">
              <span className="text-3xl font-bold text-gray-800">
                {Number(stakeState?.total_staked) / MICRO_CCD}
              </span>
              <span className="ml-2 text-lg font-semibold text-gray-600">
                EUROe
              </span>
            </div>
          )}

          <div className="mt-2 flex items-center text-sm text-gray-500">
            <span className="flex items-center text-green-500">
              <svg
                className="w-4 h-4 mr-1"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"
                />
              </svg>
              +12.5%
            </span>
            <span className="ml-2">from last month</span>
          </div>
        </div>

        {/* Total Rewards Paid */}
        <div className="bg-gradient-to-br from-purple-50 to-pink-50 rounded-lg p-6 border border-purple-100">
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center">
              <div className="p-2 bg-purple-100 rounded-lg mr-3">
                <svg
                  className="w-5 h-5 text-purple-600"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                  />
                </svg>
              </div>
              <h3 className="text-gray-600 font-medium">Total Rewards Paid</h3>
            </div>
            <span className="text-xs font-semibold px-2 py-1 bg-purple-100 text-purple-800 rounded-full">
              Updated
            </span>
          </div>

          {loadingProtocolStats ? (
            <div className="mt-4 animate-pulse">
              <div className="h-8 bg-gray-200 rounded w-[130px]"></div>
            </div>
          ) : (
            <div className="flex items-baseline">
              <span className="text-3xl font-bold text-gray-800">
                {Number(stakeState.total_rewards_paid) / MICRO_CCD || "0"}
              </span>
              <span className="ml-2 text-lg font-semibold text-gray-600">
                EUROe
              </span>
            </div>
          )}
          <div className="mt-2 flex items-center text-sm text-gray-500">
            <span className="font-medium">
              {Number(stakeState.active_stakers) || "0"}
            </span>
            <span className="ml-1">active stakers earning rewards</span>
          </div>
        </div>
      </div>
    </Card>
  );
};

export default ProtocolStats;
