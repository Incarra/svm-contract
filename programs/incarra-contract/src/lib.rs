use anchor_lang::prelude::*;

declare_id!("9cPZ5PjWUmL9g3os5d7xqsy9XSSKP2ekMNiYRNRYyV1");

#[program]
pub mod incarra_agent {
    use super::*;

    /// Creates a personal Incarra agent with Carv ID integration
    pub fn create_incarra_agent(
        ctx: Context<CreateIncarraAgent>,
        agent_name: String,
        personality: String,
        carv_id: String, // Carv ID from Ethereum
        verification_signature: String, // Signature proving ownership of Carv ID
    ) -> Result<()> {
        let incarra = &mut ctx.accounts.incarra_agent;
        let clock = Clock::get()?;

        // Validate Carv ID format (simplified validation)
        if carv_id.is_empty() || carv_id.len() > 42 {
            return err!(ErrorCode::InvalidCarvId);
        }

        incarra.owner = *ctx.accounts.user.key;
        incarra.agent_name = agent_name;
        incarra.personality = personality;
        incarra.created_at = clock.unix_timestamp;
        incarra.last_interaction = clock.unix_timestamp;

        // Initialize Carv ID data
        incarra.carv_id = carv_id.clone();
        incarra.carv_verified = false; // Will be verified separately
        incarra.verification_signature = verification_signature;
        incarra.reputation_score = 0;
        incarra.credentials = Vec::new();
        incarra.achievements = Vec::new();

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
            carv_id: carv_id,
        });

        Ok(())
    }

    /// Verify Carv ID ownership (would integrate with oracle or cross-chain verification)
    pub fn verify_carv_id(
        ctx: Context<UpdateIncarra>,
        verification_proof: String,
    ) -> Result<()> {
        let incarra = &mut ctx.accounts.incarra_agent;
        
        // In production, this would verify against Ethereum using an oracle
        // For now, we'll implement basic verification logic
        if verification_proof.len() < 10 {
            return err!(ErrorCode::InvalidVerificationProof);
        }

        incarra.carv_verified = true;
        incarra.reputation += 50; // Bonus for verified identity

        emit!(CarvIdVerified {
            agent_id: incarra.key(),
            carv_id: incarra.carv_id.clone(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /// Add a credential to the agent's Carv profile
    pub fn add_credential(
        ctx: Context<UpdateIncarra>,
        credential_type: String,
        credential_data: String,
        issuer: String,
    ) -> Result<()> {
        let incarra = &mut ctx.accounts.incarra_agent;

        if !incarra.carv_verified {
            return err!(ErrorCode::CarvIdNotVerified);
        }

        if incarra.credentials.len() >= 10 {
            return err!(ErrorCode::TooManyCredentials);
        }

        let credential = CarvCredential {
            credential_type,
            credential_data,
            issuer,
            issued_at: Clock::get()?.unix_timestamp,
            is_verified: false,
        };

        incarra.credentials.push(credential);
        incarra.reputation_score += 10;

        emit!(CredentialAdded {
            agent_id: incarra.key(),
            credential_type: incarra.credentials.last().unwrap().credential_type.clone(),
            issuer: incarra.credentials.last().unwrap().issuer.clone(),
        });

        Ok(())
    }

    /// Add achievement to agent's profile
    pub fn add_achievement(
        ctx: Context<UpdateIncarra>,
        achievement_name: String,
        achievement_description: String,
        achievement_score: u64,
    ) -> Result<()> {
        let incarra = &mut ctx.accounts.incarra_agent;

        if incarra.achievements.len() >= 20 {
            return err!(ErrorCode::TooManyAchievements);
        }

        let achievement = CarvAchievement {
            name: achievement_name,
            description: achievement_description,
            score: achievement_score,
            earned_at: Clock::get()?.unix_timestamp,
        };

        incarra.achievements.push(achievement);
        incarra.reputation_score += achievement_score;

        emit!(AchievementEarned {
            agent_id: incarra.key(),
            achievement_name: incarra.achievements.last().unwrap().name.clone(),
            score: achievement_score,
        });

        Ok(())
    }

    /// Record interaction with enhanced Carv ID tracking
    pub fn interact_with_incarra(
        ctx: Context<UpdateIncarra>,
        interaction_type: InteractionType,
        experience_gained: u64,
        context_data: String,
    ) -> Result<()> {
        let incarra = &mut ctx.accounts.incarra_agent;
        let clock = Clock::get()?;

        // Update basic stats
        incarra.total_interactions += 1;
        incarra.experience += experience_gained;
        incarra.last_interaction = clock.unix_timestamp;

        // Enhanced reputation based on Carv verification
        let base_reputation = match interaction_type {
            InteractionType::ResearchQuery => 3,
            InteractionType::DataAnalysis => 5,
            InteractionType::Conversation => 1,
            InteractionType::ProblemSolving => 4,
        };

        // Verified users get bonus reputation
        let reputation_gain = if incarra.carv_verified {
            base_reputation + 1
        } else {
            base_reputation
        };

        incarra.reputation += reputation_gain;
        incarra.reputation_score += reputation_gain;

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

    /// Get Carv profile data
    pub fn get_carv_profile(ctx: Context<ReadIncarra>) -> Result<CarvProfile> {
        let incarra = &ctx.accounts.incarra_agent;

        Ok(CarvProfile {
            carv_id: incarra.carv_id.clone(),
            is_verified: incarra.carv_verified,
            reputation_score: incarra.reputation_score,
            credentials_count: incarra.credentials.len() as u64,
            achievements_count: incarra.achievements.len() as u64,
            total_interactions: incarra.total_interactions,
            level: incarra.level,
        })
    }

    // ... (keeping all existing functions: add_knowledge_area, update_personality, get_incarra_context, deactivate_incarra)

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
            incarra.reputation += 2;
            incarra.reputation_score += 2;

            emit!(KnowledgeAreaAdded {
                agent_id: incarra.key(),
                knowledge_area,
                total_areas: incarra.knowledge_areas.len() as u64,
            });
        }

        Ok(())
    }

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
            carv_id: incarra.carv_id.clone(),
            carv_verified: incarra.carv_verified,
            reputation_score: incarra.reputation_score,
        })
    }

    pub fn deactivate_incarra(ctx: Context<UpdateIncarra>) -> Result<()> {
        let incarra = &mut ctx.accounts.incarra_agent;
        incarra.is_active = false;
        Ok(())
    }
}

// ========== Enhanced Account Structure ==========

#[account]
pub struct IncarraAgent {
    // Core Identity
    pub owner: Pubkey,                // 32 bytes
    pub agent_name: String,           // 4 + 50 bytes
    pub personality: String,          // 4 + 200 bytes
    pub created_at: i64,              // 8 bytes
    pub last_interaction: i64,        // 8 bytes

    // Carv ID Integration
    pub carv_id: String,              // 4 + 42 bytes (Ethereum address format)
    pub carv_verified: bool,          // 1 byte
    pub verification_signature: String, // 4 + 130 bytes (signature)
    pub reputation_score: u64,        // 8 bytes
    pub credentials: Vec<CarvCredential>, // 4 + (100 * 10) = 1004 bytes
    pub achievements: Vec<CarvAchievement>, // 4 + (80 * 20) = 1604 bytes

    // Agent Stats (existing)
    pub level: u64,                   // 8 bytes
    pub experience: u64,              // 8 bytes
    pub reputation: u64,              // 8 bytes
    pub total_interactions: u64,      // 8 bytes

    // Agent Capabilities (existing)
    pub research_projects: u64,       // 8 bytes
    pub data_sources_connected: u64,  // 8 bytes
    pub ai_conversations: u64,        // 8 bytes
    pub knowledge_areas: Vec<String>, // 4 + (4 + 30) * 20 = 684 bytes

    // State
    pub is_active: bool,              // 1 byte
}

// Carv ID specific structures
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CarvCredential {
    pub credential_type: String,      // e.g., "Education", "Skill", "Experience"
    pub credential_data: String,      // JSON or encoded credential data
    pub issuer: String,               // Who issued this credential
    pub issued_at: i64,
    pub is_verified: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CarvAchievement {
    pub name: String,
    pub description: String,
    pub score: u64,
    pub earned_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CarvProfile {
    pub carv_id: String,
    pub is_verified: bool,
    pub reputation_score: u64,
    pub credentials_count: u64,
    pub achievements_count: u64,
    pub total_interactions: u64,
    pub level: u64,
}

// Enhanced context with Carv data
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
    
    // Carv ID fields
    pub carv_id: String,
    pub carv_verified: bool,
    pub reputation_score: u64,
}

// ========== Enums (unchanged) ==========

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum InteractionType {
    ResearchQuery,
    DataAnalysis,
    Conversation,
    ProblemSolving,
}

// ========== Enhanced Events ==========

#[event]
pub struct IncarraAgentCreated {
    pub agent_id: Pubkey,
    pub owner: Pubkey,
    pub agent_name: String,
    pub carv_id: String,
}

#[event]
pub struct CarvIdVerified {
    pub agent_id: Pubkey,
    pub carv_id: String,
    pub timestamp: i64,
}

#[event]
pub struct CredentialAdded {
    pub agent_id: Pubkey,
    pub credential_type: String,
    pub issuer: String,
}

#[event]
pub struct AchievementEarned {
    pub agent_id: Pubkey,
    pub achievement_name: String,
    pub score: u64,
}

// Existing events
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
        space = 8 + 32 + 54 + 204 + 8 + 8 + 46 + 1 + 134 + 8 + 1004 + 1604 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 684 + 1 + 200, // Enhanced space calculation
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

// ========== Enhanced Errors ==========

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
    
    // Carv ID specific errors
    #[msg("Invalid Carv ID format.")]
    InvalidCarvId,
    #[msg("Carv ID is not verified.")]
    CarvIdNotVerified,
    #[msg("Invalid verification proof.")]
    InvalidVerificationProof,
    #[msg("Too many credentials (max 10).")]
    TooManyCredentials,
    #[msg("Too many achievements (max 20).")]
    TooManyAchievements,
}