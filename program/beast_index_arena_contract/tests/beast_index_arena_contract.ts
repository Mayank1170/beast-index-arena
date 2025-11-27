import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BeastIndexArenaContract } from "../target/types/beast_index_arena_contract";

describe("Phase 1: Basic Battle System (2 Creatures)", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BeastIndexArenaContract as Program<BeastIndexArenaContract>;

  let battleState: anchor.web3.PublicKey;
  const battleId = new anchor.BN(1);

  it("Step 1.1: Initializes a battle with 2 creatures", async () => {
    [battleState] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("battle"), battleId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    console.log("\n📍 Battle Account:", battleState.toBase58());

    const tx = await program.methods
      .initializeBattle(battleId, 100, 50, 20)
      .accounts({
        battleState: battleState,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("✅ Transaction:", tx);

    const battle = await program.account.battleState.fetch(battleState);

    console.log("\n📊 Battle State:");
    console.log("  Battle ID:", battle.battleId.toNumber());
    console.log("  Creature A HP:", battle.creatureAHp);
    console.log("  Creature B HP:", battle.creatureBHp);
    console.log("  Is Battle Over:", battle.isBattleOver);

    if (battle.battleId.toNumber() !== 1) throw new Error("❌ Battle ID wrong");
    if (battle.creatureAHp !== 100) throw new Error("❌ Creature A HP wrong");
    if (battle.creatureBHp !== 100) throw new Error("❌ Creature B HP wrong");
    if (battle.isBattleOver !== false) throw new Error("❌ Battle should not be over");

    console.log("✅ Step 1.1 Complete!\n");
  });

  it("Step 1.2: Executes a turn and reduces HP", async () => {
    console.log("\n⚔️  Executing Turn 1...\n");

    let battleBefore = await program.account.battleState.fetch(battleState);
    console.log("Before Turn:");
    console.log("  Creature A HP:", battleBefore.creatureAHp);
    console.log("  Creature B HP:", battleBefore.creatureBHp);

    await program.methods
      .executeTurn()
      .accounts({
        battleState: battleState,
        executor: provider.wallet.publicKey,
      })
      .rpc();

    let battleAfter = await program.account.battleState.fetch(battleState);
    console.log("\nAfter Turn:");
    console.log("  Creature A HP:", battleAfter.creatureAHp);
    console.log("  Creature B HP:", battleAfter.creatureBHp);

    if (battleAfter.creatureAHp >= battleBefore.creatureAHp) {
      throw new Error("❌ Creature A should have taken damage");
    }
    if (battleAfter.creatureBHp >= battleBefore.creatureBHp) {
      throw new Error("❌ Creature B should have taken damage");
    }

    console.log("✅ Step 1.2 Complete!\n");
  });

  it("Step 1.3: Battle continues until someone wins", async () => {
    console.log("\n🎮 Running Battle Until End...\n");

    let turnCount = 0;
    const maxTurns = 20;

    while (turnCount < maxTurns) {
      let battle = await program.account.battleState.fetch(battleState);

      if (battle.isBattleOver) {
        console.log(`\n🏆 Battle ended after ${battle.currentTurn.toNumber()} turns!`);

        if (battle.winner === null) {
          console.log("   Result: Draw");
        } else if (battle.winner === 0) {
          console.log("   Winner: Creature A");
        } else {
          console.log("   Winner: Creature B");
        }

        break;
      }

      console.log(`Turn ${turnCount + 1}: A=${battle.creatureAHp} HP, B=${battle.creatureBHp} HP`);

      await program.methods
        .executeTurn()
        .accounts({
          battleState: battleState,
          executor: provider.wallet.publicKey,
        })
        .rpc();

      turnCount++;
    }

    const finalBattle = await program.account.battleState.fetch(battleState);

    if (!finalBattle.isBattleOver) {
      throw new Error("❌ Battle should have ended");
    }

    console.log("✅ Step 1.3 Complete!\n");
  });

  it("Should reject turn execution on finished battle", async () => {
    console.log("\n🚫 Testing error handling...\n");

    try {
      await program.methods
        .executeTurn()
        .accounts({
          battleState: battleState,
          executor: provider.wallet.publicKey,
        })
        .rpc();

      throw new Error("❌ Should have thrown an error");

    } catch (error) {
      if (error.message.includes("Battle is already over")) {
        console.log("✅ Correctly rejected: Battle is already over");
      } else {
        throw error;
      }
    }

    console.log("✅ Error handling works!\n");
  });
});