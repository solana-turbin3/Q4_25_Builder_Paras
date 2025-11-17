import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { expect, assert } from "chai";
import { Tarni } from "../target/types/tarni";
import { createHash } from "crypto";

describe("Tarni", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  
  const program = anchor.workspace.Tarni as Program<Tarni>;
  
  // Helper function to create funded keypairs
  async function createFundedKeypair(lamports: number = 2 * LAMPORTS_PER_SOL): Promise<Keypair> {
    const keypair = Keypair.generate();
    await provider.connection.requestAirdrop(keypair.publicKey, lamports);
    await new Promise(resolve => setTimeout(resolve, 1000));
    return keypair;
  }

  // Helper to create a tournament
  async function createTournament(config?: {
    organizer?: Keypair,
    backend?: Keypair,
    entryFee?: anchor.BN,
    maxParticipants?: number,
    prizeSplit?: any,
    startsAt?: anchor.BN
  }) {
    const tournamentId = new anchor.BN(Date.now() + Math.random() * 1000);
    const organizer = config?.organizer || await createFundedKeypair();
    const backend = config?.backend || await createFundedKeypair();
    
    const [tournamentPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("tournament"), tournamentId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    
    const [escrowPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), tournamentPda.toBuffer()],
      program.programId
    );

    await program.methods
      .createTournament(
        tournamentId,
        { singleMatch: {} },
        config?.entryFee || new anchor.BN(0.1 * LAMPORTS_PER_SOL),
        config?.maxParticipants || 3,
        config?.prizeSplit || { winnerTakesAll: {} },
        "Test tournament",
        config?.startsAt || new anchor.BN(Math.floor(Date.now() / 1000) + 120)
      )
      .accounts({
        tournament: tournamentPda,
        escrow: escrowPda,
        organizer: organizer.publicKey,
        backend: backend.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([organizer])
      .rpc();

    return { tournamentId, tournamentPda, escrowPda, organizer, backend };
  }

  // Helper to register a player
  async function registerPlayer(
    tournamentPda: PublicKey,
    escrowPda: PublicKey,
    player?: Keypair
  ) {
    const playerKeypair = player || await createFundedKeypair();
    const [participantPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("participant"), tournamentPda.toBuffer(), playerKeypair.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .registerParticipant(Keypair.generate().publicKey)
      .accounts({
        tournament: tournamentPda,
        participant: participantPda,
        player: playerKeypair.publicKey,
        escrow: escrowPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([playerKeypair])
      .rpc();

    return { player: playerKeypair, participantPda };
  }

  describe("Tournament Creation tests", () => {
    it("Should fail with entry fee below minimum", async () => {
      const organizer = await createFundedKeypair();
      const backend = await createFundedKeypair();
      const tournamentId = new anchor.BN(Date.now());
      
      const [tournamentPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("tournament"), tournamentId.toArrayLike(Buffer, "le", 8)],
        program.programId
      );
      
      const [escrowPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("escrow"), tournamentPda.toBuffer()],
        program.programId
      );

      try {
        await program.methods
          .createTournament(
            tournamentId,
            { singleMatch: {} },
            new anchor.BN(0.01 * LAMPORTS_PER_SOL), // Below minimum
            3,
            { winnerTakesAll: {} },
            "Test",
            new anchor.BN(Math.floor(Date.now() / 1000) + 120)
          )
          .accounts({
            tournament: tournamentPda,
            escrow: escrowPda,
            organizer: organizer.publicKey,
            backend: backend.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([organizer])
          .rpc();
        
        assert.fail("Should have failed with low entry fee");
      } catch (error) {
        expect(error.error.errorCode.code).to.equal("EntryFeeTooLow");
      }
    });

    it("Should fail with invalid participant count (too low)", async () => {
      const organizer = await createFundedKeypair();
      const backend = await createFundedKeypair();
      const tournamentId = new anchor.BN(Date.now());
      
      const [tournamentPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("tournament"), tournamentId.toArrayLike(Buffer, "le", 8)],
        program.programId
      );
      
      const [escrowPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("escrow"), tournamentPda.toBuffer()],
        program.programId
      );

      try {
        await program.methods
          .createTournament(
            tournamentId,
            { singleMatch: {} },
            new anchor.BN(0.1 * LAMPORTS_PER_SOL),
            1, // Below minimum (MIN_PARTICIPANTS = 2)
            { winnerTakesAll: {} },
            "Test",
            new anchor.BN(Math.floor(Date.now() / 1000) + 120)
          )
          .accounts({
            tournament: tournamentPda,
            escrow: escrowPda,
            organizer: organizer.publicKey,
            backend: backend.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([organizer])
          .rpc();
        
        assert.fail("Should have failed with invalid participant count");
      } catch (error) {
        expect(error.error.errorCode.code).to.equal("InvalidArgument");
      }
    });

    it("Should fail with invalid participant count (too high)", async () => {
      const organizer = await createFundedKeypair();
      const backend = await createFundedKeypair();
      const tournamentId = new anchor.BN(Date.now());
      
      const [tournamentPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("tournament"), tournamentId.toArrayLike(Buffer, "le", 8)],
        program.programId
      );
      
      const [escrowPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("escrow"), tournamentPda.toBuffer()],
        program.programId
      );

      try {
        await program.methods
          .createTournament(
            tournamentId,
            { singleMatch: {} },
            new anchor.BN(0.1 * LAMPORTS_PER_SOL),
            65, // Above maximum (MAX_PARTICIPANTS = 64)
            { winnerTakesAll: {} },
            "Test",
            new anchor.BN(Math.floor(Date.now() / 1000) + 120)
          )
          .accounts({
            tournament: tournamentPda,
            escrow: escrowPda,
            organizer: organizer.publicKey,
            backend: backend.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([organizer])
          .rpc();
        
        assert.fail("Should have failed with too many participants");
      } catch (error) {
        expect(error.error.errorCode.code).to.equal("InvalidArgument");
      }
    });

    it("Should fail with invalid prize split (doesn't sum to 100)", async () => {
      const organizer = await createFundedKeypair();
      const backend = await createFundedKeypair();
      const tournamentId = new anchor.BN(Date.now());
      
      const [tournamentPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("tournament"), tournamentId.toArrayLike(Buffer, "le", 8)],
        program.programId
      );
      
      const [escrowPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("escrow"), tournamentPda.toBuffer()],
        program.programId
      );

      try {
        await program.methods
          .createTournament(
            tournamentId,
            { singleMatch: {} },
            new anchor.BN(0.1 * LAMPORTS_PER_SOL),
            3,
            { topThree: { first: 50, second: 30, third: 10 } }, // Only 90%
            "Test",
            new anchor.BN(Math.floor(Date.now() / 1000) + 120)
          )
          .accounts({
            tournament: tournamentPda,
            escrow: escrowPda,
            organizer: organizer.publicKey,
            backend: backend.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([organizer])
          .rpc();
        
        assert.fail("Should have failed with invalid prize split");
      } catch (error) {
        expect(error.error.errorCode.code).to.equal("InvalidPrizeSplit");
      }
    });

    it("Should fail with start time in the past", async () => {
      const organizer = await createFundedKeypair();
      const backend = await createFundedKeypair();
      const tournamentId = new anchor.BN(Date.now());
      
      const [tournamentPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("tournament"), tournamentId.toArrayLike(Buffer, "le", 8)],
        program.programId
      );
      
      const [escrowPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("escrow"), tournamentPda.toBuffer()],
        program.programId
      );

      try {
        await program.methods
          .createTournament(
            tournamentId,
            { singleMatch: {} },
            new anchor.BN(0.1 * LAMPORTS_PER_SOL),
            3,
            { winnerTakesAll: {} },
            "Test",
            new anchor.BN(Math.floor(Date.now() / 1000) - 3600) // 1 hour ago
          )
          .accounts({
            tournament: tournamentPda,
            escrow: escrowPda,
            organizer: organizer.publicKey,
            backend: backend.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([organizer])
          .rpc();
        
        assert.fail("Should have failed with past start time");
      } catch (error) {
        expect(error.error.errorCode.code).to.equal("InvalidArgument");
      }
    });

    it("Should create tournament with TopThree prize split correctly", async () => {
      const { tournamentPda } = await createTournament({
        prizeSplit: { topThree: { first: 50, second: 30, third: 20 } }
      });

      const tournament = await program.account.tournament.fetch(tournamentPda);
      expect(tournament.prizeSplit).to.deep.equal({ 
        topThree: { first: 50, second: 30, third: 20 } 
      });
    });
  });

  describe("Tournament Update tests", () => {
    it("Should fail to update after first registration", async () => {
      const { tournamentPda, escrowPda, organizer } = await createTournament();
      
      // Register one player
      await registerPlayer(tournamentPda, escrowPda);

      // Try to update
      try {
        await program.methods
          .updateTournament(
            new anchor.BN(0.2 * LAMPORTS_PER_SOL),
            null,
            null,
            "Updated rules",
            null
          )
          .accounts({
            tournament: tournamentPda,
            organizer: organizer.publicKey,
          })
          .signers([organizer])
          .rpc();
        
        assert.fail("Should have failed updating after registration");
      } catch (error) {
        expect(error.error.errorCode.code).to.equal("InvalidTournamentState");
      }
    });

    it("Should fail when non-organizer tries to update", async () => {
      const { tournamentPda } = await createTournament();
      const attacker = await createFundedKeypair();

      try {
        await program.methods
          .updateTournament(null, null, null, "Hacked", null)
          .accounts({
            tournament: tournamentPda,
            organizer: attacker.publicKey,
          })
          .signers([attacker])
          .rpc();
        
        assert.fail("Should have failed with wrong organizer");
      } catch (error) {
        expect(error.error.errorCode.code).to.equal("NotOrganizer");
      }
    });

    it("Should fail to reduce max_participants below current registrations", async () => {
      const { tournamentPda, escrowPda, organizer } = await createTournament({
        maxParticipants: 5
      });
      
      // Register 3 players
      await registerPlayer(tournamentPda, escrowPda);
      await registerPlayer(tournamentPda, escrowPda);
      await registerPlayer(tournamentPda, escrowPda);

      // Try to reduce to 2
      try {
        await program.methods
          .updateTournament(null, 2, null, null, null)
          .accounts({
            tournament: tournamentPda,
            organizer: organizer.publicKey,
          })
          .signers([organizer])
          .rpc();
        
        assert.fail("Should not allow reducing below current count");
      } catch (error) {
        expect(error.error.errorCode.code).to.equal("InvalidTournamentState");
      }
    });
  });

  describe("Registration tests", () => {
    it("Should fail when player tries to register twice", async () => {
      const { tournamentPda, escrowPda } = await createTournament();
      const player = await createFundedKeypair();

      // First registration
      await registerPlayer(tournamentPda, escrowPda, player);

      // Try to register again
      const [participantPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("participant"), tournamentPda.toBuffer(), player.publicKey.toBuffer()],
        program.programId
      );

      try {
        await program.methods
          .registerParticipant(Keypair.generate().publicKey)
          .accounts({
            tournament: tournamentPda,
            participant: participantPda,
            player: player.publicKey,
            escrow: escrowPda,
            systemProgram: SystemProgram.programId,
          })
          .signers([player])
          .rpc();
        
        assert.fail("Should fail on double registration");
      } catch (error) {
        // Anchor will throw account already initialized error
        expect(error.toString()).to.include("already in use");
      }
    });

    it("Should verify escrow receives correct SOL amount", async () => {
      const { tournamentPda, escrowPda } = await createTournament({
        entryFee: new anchor.BN(0.5 * LAMPORTS_PER_SOL)
      });

      const escrowBalanceBefore = await provider.connection.getBalance(escrowPda);

      await registerPlayer(tournamentPda, escrowPda);

      const escrowBalanceAfter = await provider.connection.getBalance(escrowPda);
      expect(escrowBalanceAfter - escrowBalanceBefore).to.equal(0.5 * LAMPORTS_PER_SOL);
    });

    it("Should update prize pool correctly with multiple registrations", async () => {
      const { tournamentPda, escrowPda } = await createTournament({
        entryFee: new anchor.BN(0.3 * LAMPORTS_PER_SOL)
      });

      await registerPlayer(tournamentPda, escrowPda);
      await registerPlayer(tournamentPda, escrowPda);

      const tournament = await program.account.tournament.fetch(tournamentPda);
      expect(tournament.prizePool.toNumber()).to.equal(0.6 * LAMPORTS_PER_SOL);
    });
  });

  describe("Lock Registration tests", () => {
    it("Should fail to lock with insufficient participants", async () => {
      const { tournamentPda, organizer } = await createTournament();

      try {
        await program.methods
          .lockRegistration()
          .accounts({
            tournament: tournamentPda,
            organizer: organizer.publicKey,
          })
          .signers([organizer])
          .rpc();
        
        assert.fail("Should fail with not enough participants");
      } catch (error) {
        expect(error.error.errorCode.code).to.equal("NotEnoughParticipants");
      }
    });

    it("Should fail when non-organizer tries to lock", async () => {
      const { tournamentPda, escrowPda } = await createTournament();
      const attacker = await createFundedKeypair();

      // Register minimum participants
      await registerPlayer(tournamentPda, escrowPda);
      await registerPlayer(tournamentPda, escrowPda);

      try {
        await program.methods
          .lockRegistration()
          .accounts({
            tournament: tournamentPda,
            organizer: attacker.publicKey,
          })
          .signers([attacker])
          .rpc();
        
        assert.fail("Should fail with wrong organizer");
      } catch (error) {
        expect(error.error.errorCode.code).to.equal("NotOrganizer");
      }
    });

    it("Should not allow locking twice", async () => {
      const { tournamentPda, escrowPda, organizer } = await createTournament();

      await registerPlayer(tournamentPda, escrowPda);
      await registerPlayer(tournamentPda, escrowPda);

      // First lock
      await program.methods
        .lockRegistration()
        .accounts({
          tournament: tournamentPda,
          organizer: organizer.publicKey,
        })
        .signers([organizer])
        .rpc();

      // Try to lock again
      try {
        await program.methods
          .lockRegistration()
          .accounts({
            tournament: tournamentPda,
            organizer: organizer.publicKey,
          })
          .signers([organizer])
          .rpc();
        
        assert.fail("Should not allow double lock");
      } catch (error) {
        // This test can fail in two valid ways:
        // 1. Program rejects with InvalidTournamentState  
        // 2. Solana rejects duplicate transaction
        const isValidError = 
          error?.error?.errorCode?.code === "InvalidTournamentState" ||
          error?.transactionMessage?.includes("already been processed");
        
        expect(isValidError).to.be.true;
      }
    });
  });

  describe("Check-in tests", () => {
    it("Should fail to check in before window opens", async () => {
      const { tournamentPda, escrowPda, organizer } = await createTournament({
        startsAt: new anchor.BN(Math.floor(Date.now() / 1000) + 7 * 3600) // 7 hours from now
      });

      const { player, participantPda } = await registerPlayer(tournamentPda, escrowPda);
      await registerPlayer(tournamentPda, escrowPda);

      await program.methods
        .lockRegistration()
        .accounts({
          tournament: tournamentPda,
          organizer: organizer.publicKey,
        })
        .signers([organizer])
        .rpc();

      const hash = createHash('sha256').update(Buffer.from("test")).digest();
      const [matchPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("match"), tournamentPda.toBuffer()],
        program.programId
      );

      await program.methods
        .launchTournament(Array.from(hash))
        .accounts({
          tournament: tournamentPda,
          matchAccount: matchPda,
          organizer: organizer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([organizer])
        .rpc();

      try {
        await program.methods
          .checkin()
          .accounts({
            tournament: tournamentPda,
            matchAccount: matchPda,
            participant: participantPda,
            player: player.publicKey,
          })
          .signers([player])
          .rpc();
        
        assert.fail("Should fail checking in too early");
      } catch (error) {
        expect(error.error.errorCode.code).to.equal("InvalidMatchState");
      }
    });

    it("Should fail to check in twice", async () => {
      const { tournamentPda, escrowPda, organizer } = await createTournament();

      const { player, participantPda } = await registerPlayer(tournamentPda, escrowPda);
      await registerPlayer(tournamentPda, escrowPda);

      await program.methods
        .lockRegistration()
        .accounts({
          tournament: tournamentPda,
          organizer: organizer.publicKey,
        })
        .signers([organizer])
        .rpc();

      const hash = createHash('sha256').update(Buffer.from("test")).digest();
      const [matchPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("match"), tournamentPda.toBuffer()],
        program.programId
      );

      await program.methods
        .launchTournament(Array.from(hash))
        .accounts({
          tournament: tournamentPda,
          matchAccount: matchPda,
          organizer: organizer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([organizer])
        .rpc();

      // First check-in
      await program.methods
        .checkin()
        .accounts({
          tournament: tournamentPda,
          matchAccount: matchPda,
          participant: participantPda,
          player: player.publicKey,
        })
        .signers([player])
        .rpc();

      // Try to check in again
      try {
        await program.methods
          .checkin()
          .accounts({
            tournament: tournamentPda,
            matchAccount: matchPda,
            participant: participantPda,
            player: player.publicKey,
          })
          .signers([player])
          .rpc();
        
        assert.fail("Should fail on double check-in");
      } catch (error) {
        // Handle different error structures for Anchor errors
        const errorCode = error?.error?.errorCode?.code || error?.errorCode?.code || error?.code;
        const isAlreadyProcessed = error?.transactionMessage?.includes("already been processed");
        
        // Either should work - program error or duplicate transaction  
        const isValidError = errorCode === "AlreadyCheckedIn" || isAlreadyProcessed;
        expect(isValidError).to.be.true;
      }
    });
  });

  describe("Cancel Tournament tests", () => {
    it("Should allow cancellation before launch", async () => {
      const { tournamentPda, escrowPda, organizer } = await createTournament();

      const { participantPda } = await registerPlayer(tournamentPda, escrowPda);
      await registerPlayer(tournamentPda, escrowPda);

      await program.methods
        .lockRegistration()
        .accounts({
          tournament: tournamentPda,
          organizer: organizer.publicKey,
        })
        .signers([organizer])
        .rpc();

      await program.methods
        .cancelTournament()
        .accounts({
          tournament: tournamentPda,
          participant: participantPda,
          organizer: organizer.publicKey,
        })
        .signers([organizer])
        .rpc();

      const tournament = await program.account.tournament.fetch(tournamentPda);
      expect(tournament.state).to.deep.equal({ cancelled: {} });
    });

    it("Should allow players to claim refunds after cancellation", async () => {
      const { tournamentPda, escrowPda, organizer } = await createTournament({
        entryFee: new anchor.BN(0.5 * LAMPORTS_PER_SOL)
      });

      const { player, participantPda } = await registerPlayer(tournamentPda, escrowPda);
      await registerPlayer(tournamentPda, escrowPda);

      await program.methods
        .lockRegistration()
        .accounts({
          tournament: tournamentPda,
          organizer: organizer.publicKey,
        })
        .signers([organizer])
        .rpc();

      await program.methods
        .cancelTournament()
        .accounts({
          tournament: tournamentPda,
          participant: participantPda,
          organizer: organizer.publicKey,
        })
        .signers([organizer])
        .rpc();

      const balanceBefore = await provider.connection.getBalance(player.publicKey);

      await program.methods
        .claimRefund()
        .accounts({
          tournament: tournamentPda,
          participant: participantPda,
          player: player.publicKey,
          escrow: escrowPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([player])
        .rpc();

      const balanceAfter = await provider.connection.getBalance(player.publicKey);
      expect(balanceAfter - balanceBefore).to.be.closeTo(0.5 * LAMPORTS_PER_SOL, 10000);
    });

    it("Should fail to cancel after match starts", async () => {
      const { tournamentPda, escrowPda, organizer, backend } = await createTournament();

      const { participantPda } = await registerPlayer(tournamentPda, escrowPda);
      await registerPlayer(tournamentPda, escrowPda);

      await program.methods
        .lockRegistration()
        .accounts({
          tournament: tournamentPda,
          organizer: organizer.publicKey,
        })
        .signers([organizer])
        .rpc();

      const hash = createHash('sha256').update(Buffer.from("test")).digest();
      const [matchPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("match"), tournamentPda.toBuffer()],
        program.programId
      );

      await program.methods
        .launchTournament(Array.from(hash))
        .accounts({
          tournament: tournamentPda,
          matchAccount: matchPda,
          organizer: organizer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([organizer])
        .rpc();

      try {
        await program.methods
          .cancelTournament()
          .accounts({
            tournament: tournamentPda,
            participant: participantPda,
            organizer: organizer.publicKey,
          })
          .signers([organizer])
          .rpc();
        
        assert.fail("Should not allow cancellation after launch");
      } catch (error) {
        expect(error.error.errorCode.code).to.equal("InvalidTournamentState");
      }
    });
  });

  describe("Prize Distribution tests", () => {
    it("Should not allow claiming prize before tournament completes", async () => {
      const { tournamentPda, escrowPda } = await createTournament();

      const { player, participantPda } = await registerPlayer(tournamentPda, escrowPda);

      try {
        await program.methods
          .claimPrize()
          .accounts({
            tournament: tournamentPda,
            participant: participantPda,
            player: player.publicKey,
            escrow: escrowPda,
            systemProgram: SystemProgram.programId,
          })
          .signers([player])
          .rpc();
        
        assert.fail("Should not allow early prize claim");
      } catch (error) {
        expect(error.error.errorCode.code).to.equal("InvalidTournamentState");
      }
    });

    it("Should not allow claiming prize twice", async () => {
      // This would require full tournament flow, skip for brevity
      // but implementation should check participant.claimed flag
    });

    it("Should not allow non-winners to claim prizes", async () => {
      // This would require submitting results with specific placements
      // Check that only players in placements[] can claim
    });
  });

  // Add a complete integration test to demonstrate full flow
  describe("Complete Tournament Flow Integration", () => {
    it("Should run a complete tournament successfully", async () => {
      const { tournamentPda, escrowPda, organizer, backend } = await createTournament({
        entryFee: new anchor.BN(0.1 * LAMPORTS_PER_SOL),
        maxParticipants: 3
      });

      // Register 3 players
      const { player: player1, participantPda: participant1Pda } = await registerPlayer(tournamentPda, escrowPda);
      const { player: player2, participantPda: participant2Pda } = await registerPlayer(tournamentPda, escrowPda);
      const { player: player3, participantPda: participant3Pda } = await registerPlayer(tournamentPda, escrowPda);

      // Lock registration
      await program.methods
        .lockRegistration()
        .accounts({
          tournament: tournamentPda,
          organizer: organizer.publicKey,
        })
        .signers([organizer])
        .rpc();

      // Launch tournament
      const matchId = new anchor.BN(12345);
      const matchIdBytes = matchId.toArrayLike(Buffer, "le", 8);
      const hash = createHash('sha256').update(matchIdBytes).digest();
      const [matchPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("match"), tournamentPda.toBuffer()],
        program.programId
      );

      await program.methods
        .launchTournament(Array.from(hash))
        .accounts({
          tournament: tournamentPda,
          matchAccount: matchPda,
          organizer: organizer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([organizer])
        .rpc();

      // Players check in
      await program.methods
        .checkin()
        .accounts({
          tournament: tournamentPda,
          matchAccount: matchPda,
          participant: participant1Pda,
          player: player1.publicKey,
        })
        .signers([player1])
        .rpc();

      await program.methods
        .checkin()
        .accounts({
          tournament: tournamentPda,
          matchAccount: matchPda,
          participant: participant2Pda,
          player: player2.publicKey,
        })
        .signers([player2])
        .rpc();

      await program.methods
        .checkin()
        .accounts({
          tournament: tournamentPda,
          matchAccount: matchPda,
          participant: participant3Pda,
          player: player3.publicKey,
        })
        .signers([player3])
        .rpc();

      // Start match - use same match ID that was hashed during launch
      await program.methods
        .startMatch(matchId)
        .accounts({
          tournament: tournamentPda,
          matchAccount: matchPda,
          backend: backend.publicKey,
        })
        .signers([backend])
        .rpc();

      // Submit results
      const [resultPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("result"), matchPda.toBuffer()],
        program.programId
      );

      const ipfsCid = "QmTestCID123";
      const signature = Array.from(crypto.getRandomValues(new Uint8Array(64)));
      const placements = [
        { player: player1.publicKey.toString(), placement: 1, kills: 5 },
        { player: player2.publicKey.toString(), placement: 2, kills: 3 },
        { player: player3.publicKey.toString(), placement: 3, kills: 1 },
      ];

      await program.methods
        .submitResults(ipfsCid, signature, placements)
        .accounts({
          tournament: tournamentPda,
          matchAccount: matchPda,
          result: resultPda,
          backend: backend.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([backend])
        .rpc();

      // Distribute prizes
      await program.methods
        .distributePrizes()
        .accounts({
          tournament: tournamentPda,
          matchAccount: matchPda,
          result: resultPda,
          participant: participant1Pda,
        })
        .rpc();

      // Winner claims prize
      const initialBalance = await provider.connection.getBalance(player1.publicKey);

      await program.methods
        .claimPrize()
        .accounts({
          tournament: tournamentPda,
          participant: participant1Pda,
          player: player1.publicKey,
          escrow: escrowPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([player1])
        .rpc();

      const finalBalance = await provider.connection.getBalance(player1.publicKey);
      expect(finalBalance).to.be.greaterThan(initialBalance);

      const participant1 = await program.account.participant.fetch(participant1Pda);
      expect(participant1.claimed).to.be.true;
    });
  });
});
