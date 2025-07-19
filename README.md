# 🤖 Incarra Agent - Solana Smart Contract

[![Solana](https://img.shields.io/badge/Solana-9945FF?style=for-the-badge&logo=solana&logoColor=white)](https://solana.com)
[![Anchor](https://img.shields.io/badge/Anchor-FF6B35?style=for-the-badge&logo=anchor&logoColor=white)](https://anchor-lang.com)
[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://rust-lang.org)

> **Personal AI agents that evolve on the blockchain**

Incarra Agent is a revolutionary Solana smart contract that creates personalized AI agents with persistent memory, evolving personalities, and gamified progression. Each agent grows smarter through interactions, building knowledge areas and reputation over time.

## 🌟 Features

### 🧠 **Intelligent Agent System**
- **Personalized AI Agents**: Create unique agents with custom names and personalities
- **Persistent Memory**: All interactions and learning stored permanently on-chain
- **Evolutionary Personalities**: Agents can update their personality traits over time

### 📈 **Gamified Progression**
- **Experience & Levels**: Agents gain XP and level up through interactions
- **Reputation System**: Build credibility through quality interactions
- **Knowledge Areas**: Expand expertise in up to 20 different domains
- **Activity Tracking**: Monitor research projects, conversations, and data analysis

### 🔄 **Rich Interaction Types**
- **Research Queries**: Academic and professional research assistance
- **Data Analysis**: Upload and analyze datasets with your agent
- **Conversations**: Natural dialogue and relationship building
- **Problem Solving**: Complex challenge resolution

### 🎯 **Enterprise-Ready**
- **Scalable Architecture**: Efficient account structure and minimal storage
- **Event-Driven**: Comprehensive logging for analytics and monitoring
- **Security First**: Proper ownership validation and access controls

## 🚀 Quick Start

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.16.0/install)"

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

### Installation

```bash
# Clone the repository
git clone <your-repo-url>
cd incarra-agent

# Install dependencies
yarn install

# Build the program
anchor build

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

## 📋 Program Interface

### Core Instructions

#### 1. Create Agent
```rust
pub fn create_incarra_agent(
    ctx: Context<CreateIncarraAgent>,
    agent_name: String,        // Max 50 characters
    personality: String,       // Max 200 characters
) -> Result<()>
```

#### 2. Interact with Agent
```rust
pub fn interact_with_incarra(
    ctx: Context<UpdateIncarra>,
    interaction_type: InteractionType,
    experience_gained: u64,
    context_data: String,      // JSON metadata
) -> Result<()>
```

#### 3. Add Knowledge Area
```rust
pub fn add_knowledge_area(
    ctx: Context<UpdateIncarra>,
    knowledge_area: String,    // Max 30 characters
) -> Result<()>
```

#### 4. Update Personality
```rust
pub fn update_personality(
    ctx: Context<UpdateIncarra>,
    new_personality: String,   // Max 200 characters
) -> Result<()>
```

### Interaction Types

```rust
pub enum InteractionType {
    ResearchQuery,    // +3 reputation, tracks research projects
    DataAnalysis,     // +5 reputation, tracks data sources
    Conversation,     // +1 reputation, tracks AI conversations
    ProblemSolving,   // +4 reputation, tracks research projects
}
```

## 🏗️ Architecture

### Account Structure

```rust
pub struct IncarraAgent {
    // Identity
    pub owner: Pubkey,                // Agent owner
    pub agent_name: String,           // Display name
    pub personality: String,          // Character description
    pub created_at: i64,              // Creation timestamp
    pub last_interaction: i64,        // Last activity
    
    // Progression
    pub level: u64,                   // Current level (XP/100 + 1)
    pub experience: u64,              // Total experience points
    pub reputation: u64,              // Cumulative reputation score
    pub total_interactions: u64,      // Interaction counter
    
    // Capabilities
    pub research_projects: u64,       // Research queries completed
    pub data_sources_connected: u64,  // Data analysis sessions
    pub ai_conversations: u64,        // Conversation interactions
    pub knowledge_areas: Vec<String>, // Up to 20 expertise areas
    
    // State
    pub is_active: bool,              // Agent availability
}
```

### Events System

The contract emits comprehensive events for frontend integration and analytics:

```rust
// Agent lifecycle events
IncarraAgentCreated { agent_id, owner, agent_name }
IncarraLevelUp { agent_id, old_level, new_level, total_experience }

// Interaction events
IncarraInteraction { agent_id, interaction_type, experience_gained, new_reputation, timestamp }
KnowledgeAreaAdded { agent_id, knowledge_area, total_areas }
```

## 💻 Frontend Integration

### JavaScript/TypeScript Example

```javascript
import { Program, AnchorProvider, web3 } from '@coral-xyz/anchor';
import { Connection, PublicKey, SystemProgram } from '@solana/web3.js';

// Initialize connection
const connection = new Connection("https://rpc.testnet.carv.io/rpc");
const programId = new PublicKey("9cPZ5PjWUmL9g3os5d7xqsy9XSSKP2ekMNiYRNRYyV1");

// Create an agent
async function createAgent(wallet, agentName, personality) {
  const [agentPda] = await PublicKey.findProgramAddress(
    [Buffer.from("incarra_agent"), wallet.publicKey.toBuffer()],
    programId
  );

  const tx = await program.methods
    .createIncarraAgent(agentName, personality)
    .accounts({
      incarraAgent: agentPda,
      user: wallet.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .rpc();

  return { signature: tx, agentAddress: agentPda };
}

// Interact with agent
async function interactWithAgent(wallet, interactionType, experience, contextData) {
  const [agentPda] = await PublicKey.findProgramAddress(
    [Buffer.from("incarra_agent"), wallet.publicKey.toBuffer()],
    programId
  );

  const tx = await program.methods
    .interactWithIncarra(interactionType, experience, contextData)
    .accounts({
      incarraAgent: agentPda,
      owner: wallet.publicKey,
    })
    .rpc();

  return tx;
}
```

### React Hook Example

```javascript
import { useConnection, useWallet } from '@solana/wallet-adapter-react';

function useIncarraAgent() {
  const { connection } = useConnection();
  const wallet = useWallet();

  const createAgent = async (name, personality) => {
    // Implementation here
  };

  const getAgent = async () => {
    const [agentPda] = await PublicKey.findProgramAddress(
      [Buffer.from("incarra_agent"), wallet.publicKey.toBuffer()],
      programId
    );
    
    const agent = await program.account.incarraAgent.fetch(agentPda);
    return agent;
  };

  return { createAgent, getAgent };
}
```

## 🔧 Configuration

### Anchor.toml
```toml
[programs.localnet]
incarra_agent_project = "9cPZ5PjWUmL9g3os5d7xqsy9XSSKP2ekMNiYRNRYyV1"

[programs.devnet]
incarra_agent_project = "9cPZ5PjWUmL9g3os5d7xqsy9XSSKP2ekMNiYRNRYyV1"

[programs.testnet]
incarra_agent_project = "9cPZ5PjWUmL9g3os5d7xqsy9XSSKP2ekMNiYRNRYyV1"

[provider]
cluster = "https://rpc.testnet.carv.io/rpc"  # CARV SVM Testnet
```

## 🧪 Testing

```bash
# Run all tests
anchor test

# Run specific test
anchor test --skip-local-validator -- --grep "should create agent"
```

### Test Examples

```javascript
describe("Incarra Agent", () => {
  it("Creates an agent successfully", async () => {
    await program.methods
      .createIncarraAgent("TestBot", "Helpful AI assistant")
      .accounts({
        incarraAgent: agentKeypair.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([agentKeypair])
      .rpc();
  });

  it("Levels up after sufficient experience", async () => {
    // Interact multiple times to gain 100+ experience
    // Check that level increases
  });
});
```

## 🌐 Network Information

| Network | RPC Endpoint | Program ID |
|---------|-------------|------------|
| **CARV SVM Testnet** | `https://rpc.testnet.carv.io/rpc` | `9cPZ5PjWUmL9g3os5d7xqsy9XSSKP2ekMNiYRNRYyV1` |
| **Devnet** | `https://api.devnet.solana.com` | `9cPZ5PjWUmL9g3os5d7xqsy9XSSKP2ekMNiYRNRYyV1` |
| **Localnet** | `http://localhost:8899` | `9cPZ5PjWUmL9g3os5d7xqsy9XSSKP2ekMNiYRNRYyV1` |

## 🔍 Verification Status

- **SolScan**: [View on CARV Explorer](https://rpc.testnet.carv.io/rpc)
- **Verification**: Not required for testnet/hackathon usage
- **Security Audit**: Pending (recommended for mainnet)

## 📊 Use Cases

### 🎓 **Education**
- Personal tutoring assistants that learn your learning style
- Research companions for academic projects
- Knowledge tracking across subjects

### 💼 **Professional**
- AI consultants specialized in your industry
- Project management assistants with context memory
- Professional development coaches

### 🎮 **Gaming & Social**
- Companion NPCs with persistent personalities
- Social AI friends that grow with interactions
- Gamified learning and skill development

### 🔬 **Research & Development**
- Specialized research assistants for specific domains
- Data analysis companions with growing expertise
- Collaborative AI for complex problem-solving

## 🛣️ Roadmap

### Phase 1 (Current) ✅
- [x] Core agent creation and management
- [x] Experience and reputation system
- [x] Knowledge area expansion
- [x] Basic interaction types

### Phase 2 (Next)
- [ ] Agent collaboration and communication
- [ ] Advanced personality evolution algorithms
- [ ] Integration with external AI services
- [ ] Mobile-friendly wallet support

### Phase 3 (Future)
- [ ] Agent marketplace and trading
- [ ] Cross-chain compatibility
- [ ] Advanced analytics dashboard
- [ ] Enterprise API and SDKs

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Fork and clone the repo
git clone https://github.com/yourusername/incarra-agent.git

# Create a feature branch
git checkout -b feature/amazing-feature

# Make your changes and test
anchor build && anchor test

# Submit a PR
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Discord**: [Join our community](https://discord.gg/incarra)
- **Documentation**: [Full API docs](https://docs.incarra.com)
- **Issues**: [GitHub Issues](https://github.com/yourusername/incarra-agent/issues)
- **Email**: support@incarra.com

## 🏆 Acknowledgments

- Built with [Anchor Framework](https://anchor-lang.com)
- Deployed on [Solana](https://solana.com) blockchain
- Tested on [CARV SVM Testnet](https://carv.io)

---

<div align="center">
  <strong>🚀 Start building the future of AI agents today!</strong><br>
  <em>Made with ❤️ by the Incarra team</em>
</div>