import {
  Abi,
  Address,
  ContractFunctionArgs,
  ContractFunctionName,
  createTestClient,
  http,
  HttpTransport,
  publicActions,
  walletActions
} from "viem";
import {generatePrivateKey, privateKeyToAccount} from 'viem/accounts'
import {foundry, mainnet, sepolia} from "viem/chains";
import {type ContractSpec} from "./prover";
import {readContract} from "viem/actions";

const rpcUrls: Map<number, HttpTransport> = new Map([[mainnet.id, http()], [sepolia.id, http("http://127.0.0.1:8546")]]);

export function client(chainId: number = sepolia.id) {
  let transport = rpcUrls.get(chainId);
  if (transport == undefined) {
    throw Error(`No url for chainId ${chainId}`);
  }

  return createTestClient({
    chain: foundry,
    mode: 'anvil',
    transport
  }).extend(publicActions).extend(walletActions)
}

export async function deployContract(contractSpec: ContractSpec, args: any[] = [], chainId: number = sepolia.id): Promise<Address> {
  const ethClient = client(chainId);
  const [deployer] = await ethClient.getAddresses();

  const txHash = await ethClient.deployContract({
    abi: contractSpec.abi,
    bytecode: contractSpec.bytecode.object,
    account: deployer,
    args
  });

  const receipt = await ethClient.waitForTransactionReceipt({hash: txHash})

  let adddress = receipt.contractAddress;

  if (adddress === undefined || adddress === null) {
    throw new Error(`Contract deployment failed with status: ${receipt.status}`);
  }

  return adddress;
}


export async function call<T extends Abi, F extends ContractFunctionName<T, 'pure' | 'view'>>(abi: T, address: Address, functionName: F, args?: ContractFunctionArgs<T, 'pure' | 'view', F>, chainId: number = sepolia.id) {
  const ethClient = client(chainId);
  const [account] = await ethClient.getAddresses();

  return readContract(ethClient, {
    abi,
    address,
    functionName,
    args,
    account
  });
}

export async function send<
  T extends Abi,
  F extends ContractFunctionName<T, 'payable' | 'nonpayable'>,
>(address: Address, abi: T, functionName: F, args: ContractFunctionArgs<T, 'payable' | 'nonpayable', F>, chainId: number = sepolia.id) {
  const ethClient = client(chainId);
  const [account] = await ethClient.getAddresses();

  const txHash = await ethClient.writeContract({
    abi,
    address,
    functionName,
    args,
    account,
  } as any);

  return ethClient.waitForTransactionReceipt({hash: txHash});
}

export function encodeCalldata(response, blockNo, block) {

  const {
    result: {
      result: {
        evm_call_result,
        function_selector,
        prover_contract_address,
        seal
      }
    }
  } = response;

  let blockNoEnc = blockNo.toString(16);
  let blockNoEncPadded = "0".repeat(64 - blockNoEnc.length) + blockNoEnc;
  let prover_address = "0".repeat(12 * 2) + prover_contract_address;
  let function_selector_padded = function_selector + "0".repeat(56);

  let seal_without_prefix = seal.replaceAll("0x", "");

  let evm_call_result1 = "0x1111111111111111111111111111111111111111111111111111111111111111";


  let length = "0".repeat(62) + "a0";
  let commitment = prover_address + function_selector_padded + blockNoEncPadded + block.hash + evm_call_result1;
  let commitment_without_prefixes = commitment.replaceAll("0x", "");
  let selector = "73138a30";
  let calldata = "0x" + selector + length + seal_without_prefix + commitment_without_prefixes;

  return calldata;

}

export const getTestAccount = () => privateKeyToAccount(generatePrivateKey());
