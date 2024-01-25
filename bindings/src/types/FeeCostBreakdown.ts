// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Amount } from "./Amount";
import type { FeeSource } from "./FeeSource";

export interface FeeCostBreakdown {
  total_fees_charged: Amount;
  breakdown: Array<[FeeSource, bigint]>;
}
