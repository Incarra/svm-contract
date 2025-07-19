use anchor_lang::prelude::*;

declare_id!("9cPZ5PjWUmL9g3os5d7xqsy9XSSKP2ekMNiYRNRYyV1");

#[program]
pub mod incarra_agent {
    use super::*;

    /// Creates a personal Incarra agent for the user
    pub fn create_incarra_agent(
        ctx: Context<CreateIncarraAgent>,
        agent_name: String,
        personality: String,
    ) -> Result<()> {
        let incarra = &mut ctx.accounts.incarra_agent;
        let clock = Clock::get()?;

        incarra.owner = *ctx.accounts.user.key;
        incarra.agent_name = agent_name;
        incarra.personality = personality;
        incarra.created_at = clock.unix_timestamp;
        incarra.last_interaction = clock.unix_timestamp;

        // Initialize user context
        incarra.level = 1;
        incarra.experience = 0;
        incarra.reputation = 0;
        incarra.total_interactions = 0;

        // Initialize capabilities
        incarra.research_projects = 0;
        incarra.data_sources_connected = 0;
        incarra.ai_conversations = 0;
        incarra.knowledge_areas = Vec::new();

        incarra.is_active = true;

        emit!(IncarraAgentCreated {
            agent_id: incarra.key(),
            owner: incarra.owner,
            agent_name: incarra.agent_name.clone(),
        });

        Ok(())
    }

    /// Record interaction with Incarra (chat, research query, etc.)
    pub fn interact_with_incarra(
        ctx: Context<UpdateIncarra>,
        interaction_type: InteractionType,
        experience_gained: u64,
        context_data: String, // JSON string with interaction context
    ) -> Result<()> {
        let incarra = &mut ctx.accounts.incarra_agent;
        let clock = Clock::get()?;

        // Update basic stats
        incarra.total_interactions += 1;
        incarra.experience += experience_gained;
        incarra.last_interaction = clock.unix_timestamp;

        // Update reputation based on interaction type
        let reputation_gain = match interaction_type {
            InteractionType::ResearchQuery => 3,
            InteractionType::DataAnalysis => 5,
            InteractionType::Conversation => 1,
            InteractionType::ProblemSolving => 4,
        };
        incarra.reputation += reputation_gain;

        // Update specific counters
        match interaction_type {
            InteractionType::ResearchQuery => {
                incarra.research_projects += 1;
            }
            InteractionType::DataAnalysis => {
                incarra.data_sources_connected += 1;
            }
            InteractionType::Conversation => {
                incarra.ai_conversations += 1;
            }
            InteractionType::ProblemSolving => {
                incarra.research_projects += 1;
            }
        }

        // Level up check (every 100 experience)
        let new_level = (incarra.experience / 100) + 1;
        if new_level > incarra.level {
            incarra.level = new_level;

            emit!(IncarraLevelUp {
                agent_id: incarra.key(),
                old_level: incarra.level - 1,
                new_level: incarra.level,
                total_experience: incarra.experience,
            });
        }

        emit!(IncarraInteraction {
            agent_id: incarra.key(),
            interaction_type,
            experience_gained,
            new_reputation: incarra.reputation,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    /// Add knowledge area to Incarra (e.g., "Machine Learning", "DeFi")
    pub fn add_knowledge_area(
        ctx: Context<UpdateIncarra>,
        knowledge_area: String,
    ) -> Result<()> {
        let incarra = &mut ctx.accounts.incarra_agent;

        if knowledge_area.len() > 30 {
            return err!(ErrorCode::KnowledgeAreaTooLong);
        }

        if incarra.knowledge_areas.len() >= 20 {
            return err!(ErrorCode::TooManyKnowledgeAreas);
        }

        if !incarra.knowledge_areas.contains(&knowledge_area) {
            incarra.knowledge_areas.push(knowledge_area.clone());
            incarra.reputation += 2; // Bonus for expanding knowledge

            emit!(KnowledgeAreaAdded {
                agent_id: incarra.key(),
                knowledge_area,
                total_areas: incarra.knowledge_areas.len() as u64,
            });
        }

        Ok(())
    }

    /// Update Incarra's personality (agent can evolve)
    pub fn update_personality(
        ctx: Context<UpdateIncarra>,
        new_personality: String,
    ) -> Result<()> {
        let incarra = &mut ctx.accounts.incarra_agent;

        if new_personality.len() > 200 {
            return err!(ErrorCode::PersonalityTooLong);
        }

        incarra.personality = new_personality;
        Ok(())
    }

    /// Get Incarra's current context (for AI system to use)
    pub fn get_incarra_context(ctx: Context<ReadIncarra>) -> Result<IncarraContext> {
        let incarra = &ctx.accounts.incarra_agent;

        Ok(IncarraContext {
            owner: incarra.owner,
            agent_name: incarra.agent_name.clone(),
            personality: incarra.personality.clone(),
            level: incarra.level,
            experience: incarra.experience,
            reputation: incarra.reputation,
            knowledge_areas: incarra.knowledge_areas.clone(),
            total_interactions: incarra.total_interactions,
            research_projects: incarra.research_projects,
            ai_conversations: incarra.ai_conversations,
        })
    }

    /// Deactivate Incarra (pause agent)
    pub fn deactivate_incarra(ctx: Context<UpdateIncarra>) -> Result<()> {
        let incarra = &mut ctx.accounts.incarra_agent;
        incarra.is_active = false;
        Ok(())
    }
}

// ========== Account Structure ==========

#[account]
pub struct IncarraAgent {
    // Core Identity
    pub owner: Pubkey,                // 32 bytes
    pub agent_name: String,           // 4 + 50 bytes
    pub personality: String,          // 4 + 200 bytes
    pub created_at: i64,              // 8 bytes
    pub last_interaction: i64,        // 8 bytes

    // Agent Stats
    pub level: u64,                   // 8 bytes
    pub experience: u64,              // 8 bytes
    pub reputation: u64,              // 8 bytes
    pub total_interactions: u64,      // 8 bytes

    // Agent Capabilities
    pub research_projects: u64,       // 8 bytes
    pub data_sources_connected: u64,  // 8 bytes
    pub ai_conversations: u64,        // 8 bytes
    pub knowledge_areas: Vec<String>, // 4 + (4 + 30) * 20 = 684 bytes

    // State
    pub is_active: bool,              // 1 byte
}

// Context object for AI system to read
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct IncarraContext {
    pub owner: Pubkey,
    pub agent_name: String,
    pub personality: String,
    pub level: u64,
    pub experience: u64,
    pub reputation: u64,
    pub knowledge_areas: Vec<String>,
    pub total_interactions: u64,
    pub research_projects: u64,
    pub ai_conversations: u64,
}

// ========== Enums ==========

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum InteractionType {
    ResearchQuery,    // User asks for research help
    DataAnalysis,     // User uploads data for analysis
    Conversation,     // General chat with AI
    ProblemSolving,   // User asks for problem solving help
}

// ========== Events ==========

#[event]
pub struct IncarraAgentCreated {
    pub agent_id: Pubkey,
    pub owner: Pubkey,
    pub agent_name: String,
}

#[event]
pub struct IncarraInteraction {
    pub agent_id: Pubkey,
    pub interaction_type: InteractionType,
    pub experience_gained: u64,
    pub new_reputation: u64,
    pub timestamp: i64,
}

#[event]
pub struct IncarraLevelUp {
    pub agent_id: Pubkey,
    pub old_level: u64,
    pub new_level: u64,
    pub total_experience: u64,
}

#[event]
pub struct KnowledgeAreaAdded {
    pub agent_id: Pubkey,
    pub knowledge_area: String,
    pub total_areas: u64,
}

// ========== Account Validation ==========

#[derive(Accounts)]
pub struct CreateIncarraAgent<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 54 + 204 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 684 + 1 + 50, // Extra padding
        seeds = [b"incarra_agent", user.key().as_ref()],
        bump
    )]
    pub incarra_agent: Account<'info, IncarraAgent>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateIncarra<'info> {
    #[account(
        mut,
        has_one = owner,
        seeds = [b"incarra_agent", owner.key().as_ref()],
        bump
    )]
    pub incarra_agent: Account<'info, IncarraAgent>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ReadIncarra<'info> {
    #[account(
        seeds = [b"incarra_agent", incarra_agent.owner.as_ref()],
        bump
    )]
    pub incarra_agent: Account<'info, IncarraAgent>,
}

// ========== Errors ==========

#[error_code]
pub enum ErrorCode {
    #[msg("Agent name is too long (max 50 characters).")]
    AgentNameTooLong,
    #[msg("Personality description is too long (max 200 characters).")]
    PersonalityTooLong,
    #[msg("Knowledge area name is too long (max 30 characters).")]
    KnowledgeAreaTooLong,
    #[msg("Too many knowledge areas (max 20).")]
    TooManyKnowledgeAreas,
    #[msg("Agent is currently inactive.")]
    AgentInactive,
}