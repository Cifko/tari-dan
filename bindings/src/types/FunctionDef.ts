// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ArgDef } from "./ArgDef";
import type { Type } from "./Type";

export interface FunctionDef {
  name: string;
  arguments: Array<ArgDef>;
  output: Type;
  is_mut: boolean;
}
