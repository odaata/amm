import { Program } from "@coral-xyz/anchor";
import { fromWorkspace, LiteSVMProvider } from "anchor-litesvm";

import AMM_IDL from "../target/idl/amm.json";
import { Amm } from "../target/types/amm";

export type AmmProgram = Program<Amm>;

export const loadSvm = () => {
  const svm = fromWorkspace(".");
  const provider = new LiteSVMProvider(svm);
  const program = new Program<Amm>(AMM_IDL, provider);
  return { program, provider, svm };
};
