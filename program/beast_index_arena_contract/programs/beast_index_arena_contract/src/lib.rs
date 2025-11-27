use anchor_lang::prelude::*;

declare_id!("28VkZmQABZWqq3gmossB41hYF9846gG2TWMyk4u6jTd4");

#[program]
pub mod beast_index_arena_contract {
    use super::*;

    pub fn initialize_battle(
        ctx: Context<InitializeBattle>,
        battle_id: u64,
        hp: u16,
        atk: u16,
        def: u16,
    ) -> Result<()> {
        let battle = &mut ctx.accounts.battle_state;
        battle.battle_id = battle_id;
        battle.authourity = ctx.accounts.authourity.key();

        battle.creature_a_hp = hp;
        battle.creature_a_atk = atk;
        battle.creature_a_def = def;
        battle.creature_a_max_hp = hp;

        battle.creature_b_hp = hp;
        battle.creature_b_atk = atk;
        battle.creature_b_def = def;
        battle.creature_b_max_hp = hp;

        battle.is_battle_over = false;
        battle.winner = None;
        battle.current_turn = 0;
        battle.bump = ctx.bumps.battle_state;

        Ok(())
    }

    pub fn execute_turn(ctx: Context<ExecuteTurn>) -> Result<()> {
        let battle = &mut ctx.accounts.battle_state;

        require!(!battle.is_battle_over, GameError::BattleAlreadyOver);

        let damage_a_to_b = battle
            .creature_a_atk
            .saturating_sub(battle.creature_b_def)
            .max(1);

        let damage_b_to_a = battle
            .creature_b_atk
            .saturating_sub(battle.creature_a_def)
            .max(1);

        battle.creature_b_hp = battle.creature_b_hp.saturating_sub(damage_a_to_b);
        msg!(
            "Creature A attacks B for {} damage! B HP: {}",
            damage_a_to_b,
            battle.creature_b_hp
        );

        if battle.creature_b_hp > 0 {
            battle.creature_a_hp = battle.creature_a_hp.saturating_sub(damage_b_to_a);
            msg!(
                "Creature B attacks A for {} damage! A HP: {}",
                damage_b_to_a,
                battle.creature_a_hp
            );
        } else {
            msg!("Creature B died before counter-attacking!");
        }

        if battle.creature_b_hp == 0 {
            battle.is_battle_over = true;
            battle.winner = Some(0);
            msg!("Creature A WINS!");
        } else if battle.creature_a_hp == 0 {
            battle.is_battle_over = true;
            battle.winner = Some(1);
            msg!("Creature B WINS!");
        }

        battle.current_turn += 1;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(battle_id: u64)]
pub struct InitializeBattle<'info> {
    #[account(
        init,
        payer= authourity,
        space= BattleState::LEN,
        seeds= [b"battle", battle_id.to_le_bytes().as_ref()],
        bump
    )]
    pub battle_state: Account<'info, BattleState>,

    #[account(mut)]
    pub authourity: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteTurn<'info> {
    #[account(
        mut,
        seeds = [b"battle", battle_state.battle_id.to_le_bytes().as_ref()],
        bump  = battle_state.bump,
    )]
    pub battle_state: Account<'info, BattleState>,
    pub executer: Signer<'info>,
}

#[account]
pub struct BattleState {
    pub battle_id: u64,
    pub authourity: Pubkey,

    pub creature_a_hp: u16,
    pub creature_a_max_hp: u16,
    pub creature_a_atk: u16,
    pub creature_a_def: u16,

    pub creature_b_hp: u16,
    pub creature_b_max_hp: u16,
    pub creature_b_atk: u16,
    pub creature_b_def: u16,

    pub is_battle_over: bool,
    pub winner: Option<u8>,
    pub current_turn: u64,

    pub bump: u8,
}

impl BattleState {
    pub const LEN: usize = 8 + 8 + 32 + 2 + 2 + 2 + 2 + 2 + 2 + 2 + 2 + 1 + 1 + 1 + 8 + 1 + 100;
}

#[error_code]
pub enum GameError {
    #[msg("Battle is already over")]
    BattleAlreadyOver,
}
