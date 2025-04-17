import { Buffer } from "buffer";
import { Address } from '@stellar/stellar-sdk';
import {
  AssembledTransaction,
  Client as ContractClient,
  ClientOptions as ContractClientOptions,
  MethodOptions,
  Result,
  Spec as ContractSpec,
} from '@stellar/stellar-sdk/contract';
import type {
  u32,
  i32,
  u64,
  i64,
  u128,
  i128,
  u256,
  i256,
  Option,
  Typepoint,
  Duration,
} from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk'
export * as contract from '@stellar/stellar-sdk/contract'
export * as rpc from '@stellar/stellar-sdk/rpc'

if (typeof window !== 'undefined') {
  //@ts-ignore Buffer exists
  window.Buffer = window.Buffer || Buffer;
}


export const networks = {
  testnet: {
    networkPassphrase: "Test SDF Network ; September 2015",
    contractId: "CBSMARIEUIPI2FI7UITQ2FGXDZ4WGN7KMIQKYSZQFRWZ5N5BBYNCA7X5",
  }
} as const

export const Errors = {
  1: {message:"AlreadyClaimed"},

  2: {message:"InvalidProof"}
}

export interface Client {
  /**
   * Construct and simulate a claim transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  claim: ({index, receiver, amount, proof}: {index: u32, receiver: string, amount: i128, proof: Array<Buffer>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

}
export class Client extends ContractClient {
  static async deploy<T = Client>(
        /** Constructor/Initialization Args for the contract's `__constructor` method */
        {root_hash, token, funding_amount, funding_source}: {root_hash: Buffer, token: string, funding_amount: i128, funding_source: string},
    /** Options for initalizing a Client as well as for calling a method, with extras specific to deploying. */
    options: MethodOptions &
      Omit<ContractClientOptions, "contractId"> & {
        /** The hash of the Wasm blob, which must already be installed on-chain. */
        wasmHash: Buffer | string;
        /** Salt used to generate the contract's ID. Passed through to {@link Operation.createCustomContract}. Default: random. */
        salt?: Buffer | Uint8Array;
        /** The format used to decode `wasmHash`, if it's provided as a string. */
        format?: "hex" | "base64";
      }
  ): Promise<AssembledTransaction<T>> {
    return ContractClient.deploy({root_hash, token, funding_amount, funding_source}, options)
  }
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAABAAAAAAAAAAAAAAABUVycm9yAAAAAAAAAgAAAAAAAAAOQWxyZWFkeUNsYWltZWQAAAAAAAEAAAAAAAAADEludmFsaWRQcm9vZgAAAAI=",
        "AAAAAAAAAAAAAAANX19jb25zdHJ1Y3RvcgAAAAAAAAQAAAAAAAAACXJvb3RfaGFzaAAAAAAAA+4AAAAgAAAAAAAAAAV0b2tlbgAAAAAAABMAAAAAAAAADmZ1bmRpbmdfYW1vdW50AAAAAAALAAAAAAAAAA5mdW5kaW5nX3NvdXJjZQAAAAAAEwAAAAA=",
        "AAAAAAAAAAAAAAAFY2xhaW0AAAAAAAAEAAAAAAAAAAVpbmRleAAAAAAAAAQAAAAAAAAACHJlY2VpdmVyAAAAEwAAAAAAAAAGYW1vdW50AAAAAAALAAAAAAAAAAVwcm9vZgAAAAAAA+oAAAPuAAAAIAAAAAEAAAPpAAAD7QAAAAAAAAAD" ]),
      options
    )
  }
  public readonly fromJSON = {
    claim: this.txFromJSON<Result<void>>
  }
}