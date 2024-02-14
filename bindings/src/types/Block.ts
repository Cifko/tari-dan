// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Command } from "./Command";
import type { Epoch } from "./Epoch";
import type { NodeHeight } from "./NodeHeight";
import type { QuorumCertificate } from "./QuorumCertificate";
import type { Shard } from "./Shard";

export interface Block {
  id: string;
  network: string;
  parent: string;
  justify: QuorumCertificate;
  height: NodeHeight;
  epoch: Epoch;
  proposed_by: string;
  total_leader_fee: number;
  merkle_root: string;
  commands: Array<Command>;
  is_dummy: boolean;
  is_processed: boolean;
  is_committed: boolean;
  foreign_indexes: Record<Shard, bigint>;
  stored_at: Array<number> | null;
  signature: { public_nonce: string; signature: string } | null;
}
