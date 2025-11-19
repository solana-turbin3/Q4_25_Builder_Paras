<div align="center">

# 🏆 TARNI

### Decentralized Tournament Platform on Solana

*Smart Contract-Based Gaming Tournaments with Automated Prize Distribution*

[![Solana](https://img.shields.io/badge/Solana-Devnet-purple)](https://explorer.solana.com/address/6ZDRzAxyRYS5GsZKEm4BWjEty3NaBuvRg8GvzDKck27c?cluster=devnet)
[![Anchor](https://img.shields.io/badge/Anchor-v0.32.1-blue)](https://www.anchor-lang.com/)
[![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/Tests-Passing-green)](./tests/)

**Program ID:** `6ZDRzAxyRYS5GsZKEm4BWjEty3NaBuvRg8GvzDKck27c`

[Architecture](#-system-architecture) • [How It Works](#-how-it-works) • [Quick Start](#-quick-start) • [API Reference](#-program-instructions)

</div>

---

## 🎯 Overview

Tarni is a decentralized tournament management system built on Solana using the Anchor framework. It enables trustless, transparent gaming tournaments with automated prize distribution through smart contracts.

**Core Features:**

- Trustless tournament creation and management
- Automated escrow system for entry fees and prizes  
- Real-time match state management with check-in system
- Flexible prize distribution (Winner-takes-all, Top 3, Top 5)
- IPFS integration for verifiable match results
- Complete on-chain audit trail

**Live on Devnet:** [View on Explorer](https://explorer.solana.com/address/6ZDRzAxyRYS5GsZKEm4BWjEty3NaBuvRg8GvzDKck27c?cluster=devnet)

---

## 🏗️ **System Architecture**

### **High-Level Flow**

```mermaid
graph TB
    A[Tournament Creation] --> B[Registration Phase]
    B --> C[Lock Registration]
    C --> D[Launch Tournament]
    D --> E[Match Execution]
    E --> F[Submit Results]
    F --> G[Distribute Prizes]
    G --> H[Players Claim Prizes]
    
    B -.->|Cancel| I[Refund Players]
    C -.->|Cancel| I
    
    style A fill:#9945FF
    style H fill:#14F195
    style I fill:#FF6B6B
```

### **Account Architecture**

```mermaid
graph LR
    A[Tournament PDA] --> B[Escrow PDA]
    A --> C[Match PDA]
    C --> D[Result PDA]
    A --> E[Participant PDA 1]
    A --> F[Participant PDA 2]
    A --> G[Participant PDA N]
    
    E -.->|Entry Fee| B
    F -.->|Entry Fee| B
    G -.->|Entry Fee| B
    
    B -.->|Prize Claim| E
    B -.->|Prize Claim| F
    B -.->|Prize Claim| G
    
    style A fill:#9945FF
    style B fill:#14F195
    style C fill:#FF6B6B
```

### PDA Derivation

```rust
// Tournament: ["tournament", tournament_id.to_le_bytes()]
// Escrow: ["escrow", tournament.key()]  
// Participant: ["participant", tournament.key(), player.key()]
// Match: ["match", tournament.key()]
// Result: ["result", match.key()]
```

***

## 🔄 **How It Works**

### 1. Tournament Lifecycle

```mermaid
stateDiagram-v2
    [*] --> Open: create_tournament()
    Open --> Locked: lock_registration()
    Locked --> InProgress: launch_tournament()
    InProgress --> Complete: submit_results()
    Complete --> Distributed: distribute_prizes()
    
    Open --> Cancelled: cancel_tournament()
    Locked --> Cancelled: cancel_tournament()
    
    Distributed --> [*]
    Cancelled --> [*]
```

**State Transitions:**

- **Open**: Organizer can update tournament parameters, players can register
- **Locked**: No new registrations, awaiting tournament launch
- **InProgress**: Match active, check-ins completed
- **Complete**: Results submitted, awaiting prize distribution
- **Distributed**: Prizes calculated and distributed, players claim winnings
- **Cancelled**: Tournament cancelled, refunds available for all participants

### **2. Match Flow**

```mermaid
sequenceDiagram
    participant O as Organizer
    participant P as Players
    participant T as Tournament PDA
    participant E as Escrow PDA
    participant B as Backend Authority
    participant M as Match PDA
    
    O->>T: create_tournament()
    P->>T: register_participant()
    P->>E: Transfer entry_fee
    O->>T: lock_registration()
    O->>T: launch_tournament()
    T->>M: Create match
    P->>M: checkin()
    B->>M: start_match()
    Note over B,M: Off-chain game execution
    B->>M: submit_results()
    P->>E: claim_prize()
    E->>P: Transfer winnings
```

### **3. Prize Distribution Models**

```mermaid
graph TD
    A[Total Prize Pool] --> B{Prize Split Type}
    
    B -->|WinnerTakesAll| C[1st Place: 100%]
    
    B -->|TopThree| D[1st: 50%]
    D --> E[2nd: 30%]
    E --> F[3rd: 20%]
    
    B -->|TopFive| G[1st: 40%]
    G --> H[2nd: 25%]
    H --> I[3rd: 20%]
    I --> J[4th: 10%]
    J --> K[5th: 5%]
    
    style A fill:#9945FF
    style C fill:#14F195
    style D fill:#14F195
    style G fill:#14F195
```

***

## 💾 Core Account Structures

### Tournament Account

```rust
pub struct Tournament {
    pub tournament_id: u64,           // Unique identifier
    pub organizer: Pubkey,            // Authority for tournament ops
    pub backend: Pubkey,              // Authority for match ops
    pub game_type: GameType,          // Currently: SingleMatch
    pub entry_fee: u64,               // SOL lamports
    pub max_participants: u8,         // Max players allowed
    pub current_participants: u8,     // Current registered count
    pub prize_split: PrizeSplit,      // Distribution model
    pub rules: String,                // Tournament rules (max 200 chars)
    pub state: TournamentState,       // Current lifecycle state
    pub created_at: i64,              // Creation timestamp
    pub starts_at: i64,               // Tournament start time
    pub checkin_opens: i64,           // Check-in window start
    pub checkin_closes: i64,          // Check-in deadline
    pub started_at: i64,              // Actual start timestamp
    pub escrow: Pubkey,               // Escrow PDA holding funds
    pub prize_pool: u64,              // Total prize amount
    pub escrow_bump: u8,              // Escrow PDA bump
    pub bump: u8,                     // Tournament PDA bump
}
```

### Participant Account

```rust
pub struct Participant {
    pub player: Pubkey,               // Player wallet
    pub tournament: Pubkey,           // Parent tournament
    pub game_account: Pubkey,         // Linked game account
    pub registered_at: i64,           // Registration timestamp
    pub entry_paid: u64,              // Amount contributed
    pub checked_in: bool,             // Pre-match verification
    pub checkin_time: i64,            // Check-in timestamp
    pub disqualified: bool,           // DQ status
    pub dq_reason: String,            // DQ reason (max 32 chars)
    pub prize_amount: u64,            // Calculated winnings
    pub claimed: bool,                // Prize claimed status
    pub refunded: bool,               // Refund status
    pub refund_amount: u64,           // Refund amount
    pub bump: u8,                     // Participant PDA bump
}
```

### Match Account

```rust
pub struct Match {
    pub tournament: Pubkey,           // Parent tournament
    pub match_id_hash: [u8; 32],      // External match ID hash
    pub match_id: u64,                // External match ID
    pub participants: Vec<Pubkey>,    // Registered participants (max 64)
    pub state: MatchState,            // Pending/CheckedIn/InProgress/Complete
    pub created_at: i64,              // Creation timestamp
    pub starts_at: i64,               // Match start time
    pub cin_deadline: i64,            // Check-in deadline
    pub participant_limit: u8,        // Max participants
    pub checked_in: u8,               // Current check-in count
    pub bump: u8,                     // Match PDA bump
}
```

### Result Account

```rust
pub struct Result {
    pub match_: Pubkey,               // Parent match
    pub tournament: Pubkey,           // Parent tournament
    pub ipfs_cid: String,             // IPFS hash for match data (max 120 chars)
    pub submitted_at: i64,            // Submission timestamp
    pub signature: [u8; 64],          // Cryptographic signature
    pub placements: Vec<PlayerResult>, // Player rankings (max 64)
    pub verified: bool,               // Verification status
    pub distributed: bool,            // Prize distribution status
    pub bump: u8,                     // Result PDA bump
}
```

***

## 🔧 Program Instructions

### Tournament Management

| Instruction | Authority | Parameters | Description |
|------------|-----------|------------|-------------|
| `create_tournament` | Organizer | `tournament_id`, `game_type`, `entry_fee`, `max_participants`, `prize_split`, `rules`, `starts_at` | Initialize tournament with configuration |
| `update_tournament` | Organizer | Optional: `entry_fee`, `max_participants`, `prize_split`, `rules`, `starts_at` | Modify parameters (only when state is Open) |
| `lock_registration` | Organizer | None | Close registration, prepare for launch |
| `launch_tournament` | Organizer | `match_id_hash` | Create match account and transition to InProgress |
| `cancel_tournament` | Organizer | None | Cancel tournament and enable refunds |

### Player Operations

| Instruction | Authority | Parameters | Description |
|------------|-----------|------------|-------------|
| `register_participant` | Player | `game_account` | Join tournament and pay entry fee |
| `checkin` | Player | None | Verify attendance within check-in window |
| `claim_prize` | Player | None | Withdraw winnings from escrow |
| `claim_refund` | Player | None | Get refund from cancelled tournaments |

### Match & Result Operations

| Instruction | Authority | Parameters | Description |
|------------|-----------|------------|-------------|
| `start_match` | Backend | `match_id` | Begin match execution after check-in |
| `submit_results` | Backend | `ipfs_cid`, `signature`, `placements` | Upload match results with IPFS verification |
| `auto_disqualify` | Backend | None | Disqualify players who missed check-in |

***
