import { Button } from "react-daisyui";
import { useAccount, useConnect } from "wagmi";
import { Chain } from "viem";

export function AddressArea() {
  const { address, chain } = useAccount();
  const { isConnected } = useAccount();
  const { connect, connectors } = useConnect();

  if (!isConnected) {
    return (
      <Button onClick={() => connect({ connector: connectors[0] })}>
        Connect
      </Button>
    );
  }
  return (
    <div className="flex flex-col gap-6">
      <div className="flex flex-col gap-2">
        <div className="flex items-center justify-center gap-4">
          <span className="text-xl font-bold text-gray-500">Address:</span>
          <span className="text-xl font-bold text-gray-500">
            {address ? formatAddress(address) : "Not connected"}
          </span>
        </div>
        <span className="text-lg font-bold text-gray-500">
          {chain ? chain.name : "Not connected"}
          {chain ? (
            isVlayerSupportedChain(chain) ? (
              <span className="text-green-500"> (supported)</span>
            ) : (
              <span className="text-red-500"> (not supported)</span>
            )
          ) : (
            ""
          )}
        </span>
      </div>
    </div>
  );
}

function formatAddress(address: `0x${string}`) {
  return `${address.slice(0, 6)}...${address.slice(-4)}`;
}

function isVlayerSupportedChain(chain: Chain) {
  console.log(
    "isVlayerSupportedChain",
    import.meta.env.VITE_CHAIN_NAME,
    chain.name
  );
  return import.meta.env.VITE_CHAIN_NAME === chain.name.toLowerCase();
}
