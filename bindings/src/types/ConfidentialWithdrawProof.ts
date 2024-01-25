// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { BalanceProofSignature } from "./BalanceProofSignature";
import type { ConfidentialOutputProof } from "./ConfidentialOutputProof";
import type { PedersonCommitmentBytes } from "./PedersonCommitmentBytes";

export interface ConfidentialWithdrawProof {
  inputs: Array<PedersonCommitmentBytes>;
  output_proof: ConfidentialOutputProof;
  balance_proof: BalanceProofSignature;
}
