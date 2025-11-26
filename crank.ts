import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BeastIndexArena } from "../target/types/beast_index_arena";

// CONFIGURATION
const CRANK_INTERVAL_MS = 30 * 1000; [cite_start]// Run turn every 30 seconds [cite: 23]

async function main() {
  // 1. Setup Provider (Use local wallet or env secret)
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.BeastIndexArena as Program<BeastIndexArena>;
  
  // NOTE: In production, fetch the actual Game Account Public Key from a config file
  // For now, we assume a known Game Account for the demo
  // const gameAccountPubkey = new anchor.web3.PublicKey("..."); 

  console.log("⚔️ Starting Beast Index Arena Crank...");
  console.log(`⏱️ Interval: ${CRANK_INTERVAL_MS}ms`);

  // 2. Infinite Loop
  setInterval(async () => {
    try {
      console.log("🤖 Crank: Executing Turn...");

      // Call the Smart Contract function 'execute_turn'
      const tx = await program.methods
        .executeTurn()
        .accounts({
          // gameAccount: gameAccountPubkey, // Uncomment when connected to real account
        })
        .rpc();

      console.log(`✅ Turn Executed! Tx: ${tx}`);
      
    } catch (err) {
      console.error("❌ Crank Failed:", err);
    }
  }, CRANK_INTERVAL_MS);
}

main().then(() => {
  console.log("Crank script initialized.");
});
