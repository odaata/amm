import { type AmmProgram, loadSvm } from "./helpers";

let program: AmmProgram;
describe("amm", () => {
  beforeAll(() => {
    program = loadSvm().program;
  });

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
