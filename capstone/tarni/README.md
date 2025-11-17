<div align="center">

# **TARNI**

### Decentralized, transparent, and secure tournament platform on Solana

[![Solana](https://img.shields.io/badge/Solana-Devnet-purple?style=flat-square)](https://solana.com/)
[![Anchor](https://img.shields.io/badge/Anchor-v0.32.1-blue?style=flat-square)](https://www.anchor-lang.com/)
[![Tests](https://img.shields.io/badge/tests-24%2F24%20passing-brightgreen?style=flat-square)](#testing)
[![Platform](https://img.shields.io/badge/platform-Solana-brightgreen?style=flat-square)](https://solana.com/)
[![TypeScript](https://img.shields.io/badge/TypeScript-Ready-blue?style=flat-square)](https://www.typescriptlang.org/)
[![⭐ GitHub stars](https://img.shields.io/github/stars/solana-turbin3/Q4_25_Builder_Paras?style=social)](https://github.com/solana-turbin3/Q4_25_Builder_Paras/stargazers)
[![🍴 GitHub forks](https://img.shields.io/github/forks/solana-turbin3/Q4_25_Builder_Paras?style=social)](https://github.com/solana-turbin3/Q4_25_Builder_Paras/network)
[![🐛 GitHub issues](https://img.shields.io/github/issues/solana-turbin3/Q4_25_Builder_Paras)](https://github.com/solana-turbin3/Q4_25_Builder_Paras/issues)
[![💾 GitHub code size](https://img.shields.io/github/languages/code-size/solana-turbin3/Q4_25_Builder_Paras)](https://github.com/solana-turbin3/Q4_25_Builder_Paras)

🏆

</div>

---

## 🎯 **What is Tarni?**

Tarni revolutionizes competitive gaming by bringing **transparent, automated tournament management** to Solana. Create tournaments, register players, and distribute prizes automatically through smart contracts—no intermediaries, no disputes, just pure competitive gaming.

Real-time state management ensures fairness, and all tournament operations are **100% transparent on-chain**.

> **🎮 Gaming Focus:** Tarni specializes in **decentralized tournament management** with automated prize distribution, transparent player registration, and immutable tournament records. Built for competitive gaming communities that value fairness and transparency.

### 🌟 **Key Features**

- ⚡ **Instant Tournament Creation**: Launch tournaments with customizable entry fees and prize pools
- 🏛️ **Transparent Operations**: All tournament data and transactions visible on-chain
- 💰 **Automated Prize Distribution**: Smart contract handles prize payouts automatically
- 🔒 **Secure \& Auditable**: All tournament logic and prize distribution on-chain
- 🎫 **Player Registration**: Simple and secure tournament entry system

> Create tournaments, compete fairly, get paid instantly. No intermediaries, no disputes.

<div align="center">


### Build with us


Ready for transparent competitive gaming? You're exactly who we built this for.


<a href="https://github.com/solana-turbin3/Q4_25_Builder_Paras">
  <img src="https://img.shields.io/badge/⭐%20Star%20this%20repo-Join%20our%20gaming%20community!-yellow?style=for-the-badge&logo=github" alt="Star this repo" />
</a>


</div>

---

## ✨ **Features**

### ⚡ **Tournament Management**

- **Create Tournaments**: Configure entry fees, participant limits, prize structures, and rules
- **Multiple Game Types**: Support for Single Match for mvp
- **Flexible Prize Distribution**: Winner-takes-all or top-three split configurations
- **Tournament Lifecycle**: Open → Locked → In Progress → Complete states


### 🎫 **Participant System**

- **Secure Registration**: Entry fee payments automatically held in escrow
- **Check-in System**: Time-window based player verification before matches
- **Game Account Integration**: Link external gaming accounts to blockchain identity
- **Automated Prize Calculation**: Fair distribution based on tournament results


### 💰 **Escrow \& Payments**

- **System-Owned Escrow**: Secure SOL custody using Program Derived Addresses (PDAs)
- **Automated Transfers**: Direct prize payments to winners' wallets
- **Refund Mechanism**: Automatic refunds for cancelled tournaments
- **Gas-Efficient**: Optimized for minimal transaction costs


### 🔐 **Security \& Validation**

- **Role-Based Access**: Organizer, backend authority, and participant permissions
- **Comprehensive Error Handling**: 20+ custom error types for robust validation
- **State Management**: Strict tournament and match state transitions
- **Duplicate Prevention**: Protection against double registrations and payments

---

## 🚀 **Deployment**

### **Devnet (Live)**

- **Program ID**: `6ZDRzAxyRYS5GsZKEm4BWjEty3NaBuvRg8GvzDKck27c`
- **Network**: Solana Devnet
- **Explorer**: [View on Solana Explorer](https://explorer.solana.com/address/6ZDRzAxyRYS5GsZKEm4BWjEty3NaBuvRg8GvzDKck27c?cluster=devnet)

The Tarni tournament platform is **live on Solana Devnet** and ready for testing! All 24 tests pass with comprehensive coverage of tournament creation, player registration, prize distribution, and edge cases.

---

## 🦾 **Technology Stack**

### ⛓️ **Blockchain**

- **Solana**: High-performance blockchain for instant transactions
- **Anchor Framework**: Secure program development framework
- **SPL Token**: Native Solana token standards for escrow management


### 🛠️ **Development Tools**

- **Rust**: Systems programming language for smart contracts
- **TypeScript**: Type-safe testing and interaction scripts
- **Anchor CLI**: Build and deploy Solana programs
- **Solana CLI**: Blockchain interaction and wallet management


### 🧪 **Testing Framework**

- **Mocha/Chai**: Comprehensive test suite with 24 test cases
- **Anchor Test**: Integration testing with local validator
- **Edge Case Coverage**: Tournament lifecycle and error handling

---

## 📋 **Prerequisites**

### **Required Tools**

- **Node.js** 18+ and **npm/yarn**
- **Rust** 1.75+ and **Cargo**
- **Solana CLI** 1.18+ installed
- **Anchor** framework 0.32.1+ installed
- **Solana wallet** (recommended: local filesystem wallet)


### **Installation Links**

- [Rust](https://rustup.rs/) - Rust programming language
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) - Solana command line tools
- [Anchor CLI](https://www.anchor-lang.com/docs/installation) - Anchor framework
- [Node.js](https://nodejs.org/) - JavaScript runtime

---

## 🏁 **Quick Start**

### **1. Clone \& Install**

```bash
# Clone the repository
git clone [https://github.com/solana-turbin3/Q4_25_Builder_Paras.git](https://github.com/solana-turbin3/Q4_25_Builder_Paras.git)
cd Q4_25_Builder_Paras/capstone/tarni


# Install dependencies
npm install
```


### **2. Build \& Test**

```bash
# Build the program
anchor build


# Run comprehensive test suite
anchor test


# Expected: ✅ 24/24 tests passing
```


### **3. Deploy to Devnet**

```bash
# Configure Solana CLI for devnet
solana config set --url devnet


# Airdrop SOL for testing
solana airdrop 2


# Deploy the program
anchor deploy
```


### **4. Interact with the Program**

```bash
# Run the test suite to see all interactions
anchor test

# Or test specific functionality
anchor test --grep "Tournament Creation"
```





## 🧪 Testing

The project includes a comprehensive test suite with 24 test cases covering:

### Test Categories

- **Tournament Creation Edge Cases** (6 tests)
    - Entry fee validation
    - Participant limits
    - Prize split validation
    - Time constraints
- **Registration \& Management** (9 tests)
    - Player registration flow
    - Tournament updates and locking
    - Authorization checks
- **Match Execution** (2 tests)
    - Check-in system
    - Match state management
- **Financial Operations** (6 tests)
    - Tournament cancellation and refunds
    - Prize distribution
    - Escrow management
- **Complete Integration** (1 test)
    - End-to-end tournament lifecycle


### Run Tests

```bash
# All tests
anchor test


# Specific test pattern
anchor test --grep "Tournament Creation"


# Skip validator startup (if already running)
anchor test --skip-local-validator
```


## 📖 Program Architecture

### Core Instructions

#### Tournament Management

- `create_tournament` - Initialize new tournament with configuration
- `update_tournament` - Modify tournament parameters (before registration)
- `lock_registration` - Close registration and prepare for launch
- `launch_tournament` - Start tournament and create match
- `cancel_tournament` - Cancel tournament and enable refunds


#### Participant Operations

- `register_participant` - Join tournament with entry fee payment
- `checkin` - Verify attendance before match starts
- `claim_prize` - Withdraw winnings after tournament completion
- `claim_refund` - Get refund from cancelled tournament


#### Match Management

- `start_match` - Begin match execution (backend authority)
- `submit_results` - Upload match results and placements
- `distribute_prizes` - Calculate and assign prize amounts


### Account Structure

```rust
// Main tournament state
Tournament {
    tournament_id: u64,
    organizer: Pubkey,
    backend: Pubkey,
    entry_fee: u64,
    max_participants: u8,
    current_participants: u8,
    prize_split: PrizeSplit,
    state: TournamentState,
    escrow: Pubkey,
    // ... additional fields
}


// Player registration record
Participant {
    tournament: Pubkey,
    player: Pubkey,
    game_account: Pubkey,
    entry_paid: u64,
    prize_amount: u64,
    checked_in: bool,
    claimed: bool,
    // ... additional fields
}
```


### PDA (Program Derived Address) Seeds

```rust
// Tournament PDA
["tournament", tournament_id.to_le_bytes()]


// Escrow PDA  
["escrow", tournament.key()]


// Participant PDA
["participant", tournament.key(), player.key()]


// Match PDA
["match", tournament.key()]


// Result PDA
["result", match.key()]
```


## 🎮 Usage Examples

### Create Tournament

```typescript
await program.methods
  .createTournament(
    tournamentId,           // Unique tournament ID
    { singleMatch: {} },    // Game type
    entryFee,              // Entry fee in lamports
    maxParticipants,       // Max players (2-64)
    { winnerTakesAll: {} }, // Prize distribution
    rules,                 // Tournament rules string
    startsAt              // Start timestamp
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
```


### Register Player

```typescript
await program.methods
  .registerParticipant(gameAccountPubkey)
  .accounts({
    tournament: tournamentPda,
    participant: participantPda,
    player: player.publicKey,
    escrow: escrowPda,
    systemProgram: SystemProgram.programId,
  })
  .signers([player])
  .rpc();
```


### Claim Prize

```typescript
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
```


## 🔧 Configuration

### Anchor.toml

```toml
[programs.devnet]
tarni = "6ZDRzAxyRYS5GsZKEm4BWjEty3NaBuvRg8GvzDKck27c"


[provider]
cluster = "devnet"
wallet = "/path/to/your/wallet.json"
```


### Environment Variables

```bash
# Optional: Set custom RPC endpoint
export ANCHOR_PROVIDER_URL="https://api.devnet.solana.com"


# Optional: Set wallet path
export ANCHOR_WALLET="/path/to/wallet.json"
```


## 🚨 Error Handling

The program includes comprehensive error types:

```rust
pub enum TarniError {
    #[msg("Entry fee is too low")]
    EntryFeeTooLow,
    
    #[msg("Tournament is full")]
    TournamentFull,
    
    #[msg("Invalid tournament state")]
    InvalidTournamentState,
    
    #[msg("Not enough participants")]
    NotEnoughParticipants,
    
    #[msg("Player has already checked in")]
    AlreadyCheckedIn,
    
    #[msg("Check-in window is not open")]
    CheckinNotOpen,
    
    // ... 15+ additional error types
}
```


## 🛡️ Security Considerations

### Access Control

- **Organizer-only**: Tournament creation, updates, locking, launching
- **Backend Authority**: Match management, result submission
- **Player-only**: Registration, check-in, prize claiming
- **System Validation**: All state transitions and financial operations


### Financial Security

- **Escrow System**: Funds held in program-controlled PDAs
- **Atomic Operations**: All transfers are atomic and reversible on failure
- **Validation**: Entry fees, prize calculations, and refunds are validated
- **No Reentrancy**: Secure against common DeFi attack vectors


### State Management

- **Immutable States**: Critical tournament data cannot be modified after registration
- **Strict Transitions**: State changes follow predetermined flow
- **Deadline Enforcement**: Time-based constraints prevent invalid operations


## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Run tests (`anchor test`)
4. Commit changes (`git commit -am 'Add amazing feature'`)
5. Push to branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

### Development Setup

```bash
# Install dependencies
yarn install

# Build program
anchor build

# Run comprehensive test suite
anchor test

# Deploy to localnet for testing
solana-test-validator --reset &
anchor deploy --provider.cluster localnet
```


## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Turbin3 Program**: Built during the Turbin3 Solana development program
- **Anchor Framework**: Leveraging Anchor for secure Solana program development
- **Solana Foundation**: For providing the high-performance blockchain infrastructure


## 📞 Support

- **Issues**: [GitHub Issues](link-to-issues)
- **Documentation**: [Anchor Docs](https://www.anchor-lang.com/)
- **Solana Docs**: [Solana Developer Docs](https://docs.solana.com/)

---

**Built with ❤️ on Solana** | **Powering the Future of Decentralized Gaming** 🎮⚡"

Here's a cleaned-up, professional README that removes the cringe and makes it actually useful:

***

```markdown
# Tarni

**Decentralized tournament platform for competitive gaming on Solana**

[![Solana](https://img.shields.io/badge/Solana-Devnet-purple?style=flat-square)](https://solana.com/)
[![Anchor](https://img.shields.io/badge/Anchor-v0.32.1-blue?style=flat-square)](https://www.anchor-lang.com/)
[![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)](LICENSE)

A smart contract platform for running transparent, automated esports tournaments on Solana. Players register and pay entry fees on-chain, compete off-chain, and winners claim prizes automatically from escrow.

**Program ID (Devnet):** `6ZDRzAxyRYS5GsZKEm4BWjEty3NaBuvRg8GvzDKck27c`

---

## Features

- **On-chain escrow** - Entry fees locked in program-controlled PDAs
- **Automated prize distribution** - Winners claim directly from escrow
- **Flexible prize splits** - Winner-takes-all or top 3/5 distribution
- **Check-in system** - Time-windowed player verification
- **Cancellation refunds** - Automatic refunds if tournament cancelled
- **Match ID privacy** - Commitment scheme prevents sniping

---

## Architecture

### State Accounts

**Tournament** - Configuration and state tracking
```

pub struct Tournament {
pub tournament_id: u64,
pub organizer: Pubkey,
pub backend: Pubkey,           // Oracle authority
pub entry_fee: u64,
pub max_participants: u8,
pub current_participants: u8,
pub prize_split: PrizeSplit,
pub state: TournamentState,    // Open → Locked → InProgress → Complete
pub escrow: Pubkey,
pub prize_pool: u64,
}

```

**Participant** - Player registration and prize tracking
```

pub struct Participant {
pub player: Pubkey,
pub tournament: Pubkey,
pub game_account: Pubkey,      // External game ID
pub entry_paid: u64,
pub checked_in: bool,
pub prize_amount: u64,
pub claimed: bool,
}

```

**Match** - Match state and check-in tracking
```

pub struct Match {
pub tournament: Pubkey,
pub match_id: u64,
pub match_id_hash: [u8; 32],   // Commitment
pub state: MatchState,          // Pending → CheckedIn → InProgress → Complete
pub checked_in: u8,
}

```

### PDA Seeds
```rust
["tournament", tournament_id.to_le_bytes()]
["escrow", tournament_pubkey.as_ref()]
["participant", tournament_pubkey.as_ref(), player_pubkey.as_ref()]
["match", tournament_pubkey.as_ref()]
["result", match_pubkey.as_ref()]
```

---

## Instructions

### Tournament Lifecycle
- `create_tournament` - Organizer creates tournament with config
- `update_tournament` - Modify params (only before first registration)
- `lock_registration` - Close registration and open check-in window
- `launch_tournament` - Create match account with commitment hash
- `cancel_tournament` - Cancel and enable refunds (before match starts)

### Player Flow
- `register_participant` - Join tournament and pay entry fee to escrow
- `checkin` - Check in during window before match
- `claim_prize` - Withdraw winnings after results submitted
- `claim_refund` - Get refund if tournament cancelled

### Match Management (Backend Oracle)
- `start_match` - Record match start and reveal match ID
- `submit_results` - Upload IPFS proof and player placements
- `distribute_prizes` - Calculate individual prize amounts

### Access Control
| Instruction | Caller | Constraints |
|-------------|--------|-------------|
| `create_tournament` | Organizer | Anyone |
| `update_tournament` | Organizer | State = Open, no registrations |
| `lock_registration` | Organizer | Min 2 participants |
| `register_participant` | Player | State = Open, tournament not full |
| `start_match` | Backend | After check-in deadline |
| `submit_results` | Backend | Match in progress |

---

## Setup

### Prerequisites
- Rust 1.75+
- Solana CLI 1.18+
- Anchor 0.32.1+
- Node.js 18+

### Installation
```bash
git clone https://github.com/solana-turbin3/Q4_25_Builder_Paras.git
cd Q4_25_Builder_Paras/capstone/tarni
yarn install
```

### Build
```

anchor build

```

### Test
```

anchor test

# 24 test cases covering:

# - Tournament creation edge cases

# - Registration and state management

# - Check-in and match flow

# - Prize distribution

# - Cancellation and refunds

---

## Usage

### Create Tournament
```typescript
await program.methods
  .createTournament(
    new anchor.BN(Date.now()),         // tournament_id
    { singleMatch: {} },               // game_type
    new anchor.BN(0.05 * LAMPORTS_PER_SOL), // entry_fee (minimum 0.05 SOL)
    3,                                 // max_participants
    { winnerTakesAll: {} },           // prize_split
    "Tournament rules",               // rules
    new anchor.BN(Math.floor(Date.now() / 1000) + 120) // starts_at
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
```

### Register Player
```typescript
await program.methods
  .registerParticipant(gameAccountPubkey)
  .accounts({
    tournament: tournamentPda,
    participant: participantPda,
    player: player.publicKey,
    escrow: escrowPda,
    systemProgram: SystemProgram.programId,
  })
  .signers([player])
  .rpc();
```

### Claim Prize
```typescript
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
```

---

## Security

### Escrow
- Funds held in system-owned PDA (not program account)
- Withdrawals only via signed instructions
- Validates amounts match participant records

### State Machine
- Strict state transitions enforced
- Tournament params frozen after first registration
- Check-in window time-locked

### Match ID Privacy
- Organizer commits hash during launch
- Actual match ID revealed only after match starts
- Prevents room code sniping

### Error Handling
25+ custom error types covering:
- Invalid state transitions
- Insufficient funds  
- Timing violations
- Authorization failures
- Math overflow/underflow
- Tournament registration issues
- Check-in and match state errors

---

## Configuration

### Anchor.toml
```

[programs.devnet]
tarni = "6ZDRzAxyRYS5GsZKEm4BWjEty3NaBuvRg8GvzDKck27c"

[provider]
cluster = "devnet"
wallet = "/home/psh/paras/keys/turbin3_wallet.json"

```

### Constants
```rust
pub const MIN_ENTRY_FEE: u64 = 50_000_000;      // 0.05 SOL
pub const MIN_PARTICIPANTS: u8 = 2;
pub const MAX_PARTICIPANTS: u8 = 64;
pub const CHECKIN_WINDOW_SECONDS: i64 = 6 * 60 * 60; // 6 hours
```

---

## Project Structure
```
tarni/
├── programs/tarni/src/
│   ├── lib.rs                    # Program entrypoint
│   ├── constants.rs              # Program constants
│   ├── error.rs                  # Custom error types
│   ├── events.rs                 # Event definitions
│   ├── state/                    # Account definitions
│   │   ├── tournament.rs
│   │   ├── participant.rs
│   │   ├── match_.rs
│   │   ├── result.rs
│   │   └── escrow.rs
│   ├── instructions/             # Instruction handlers
│   │   ├── tournament/
│   │   ├── participant/
│   │   ├── match_/
│   │   └── distribution/
│   └── utils/                    # Helper functions
├── tests/
│   └── tarni.ts                  # Integration tests (24 test cases)
├── target/
│   ├── idl/
│   │   └── tarni.json           # Generated IDL
│   └── types/
│       └── tarni.ts             # Generated TypeScript types
├── package.json                  # Dependencies and scripts
└── Anchor.toml                   # Anchor configuration
```

---
