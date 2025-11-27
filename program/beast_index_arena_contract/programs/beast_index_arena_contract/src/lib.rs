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

        battle.creature_hp = [hp, hp, hp, hp];
        battle.creature_atk = [atk, atk, atk, atk];
        battle.creature_def = [def, def, def, def];
        battle.creature_max_hp = [hp, hp, hp, hp];
        battle.is_alive = [true, true, true, true];

        battle.is_battle_over = false;
        battle.winner = None;
        battle.current_turn = 0;
        battle.bump = ctx.bumps.battle_state;

        Ok(())
    }

    pub fn execute_turn(ctx: Context<ExecuteTurn>) -> Result<()> {
        let battle = &mut ctx.accounts.battle_state;

        require!(!battle.is_battle_over, GameError::BattleAlreadyOver);

        let alive_count = battle.is_alive.iter().filter(|&&x| x).count();

        for attacker_idx in 0..4 {
            if !battle.is_alive[attacker_idx] {
                continue;
            }
            let target_idx = (attacker_idx + 1) % 4;
            let mut actual_target_idx = target_idx;
            let mut searched = 0;
            while !battle.is_alive[actual_target_idx] && searched < 4 {
                actual_target_idx = (actual_target_idx + 1) % 4;
                searched += 1;
            }
            if !battle.is_alive[actual_target_idx] || actual_target_idx == attacker_idx {
                continue;
            }

            let damage = battle.creature_atk[attacker_idx]
                .saturating_sub(battle.creature_def[actual_target_idx])
                .max(1);

            battle.creature_hp[actual_target_idx] =
                battle.creature_hp[actual_target_idx].saturating_sub(damage);

            msg!(
                "Creature {} attacks Creature {} for {} damage! HP: {}",
                attacker_idx,
                actual_target_idx,
                damage,
                battle.creature_hp[actual_target_idx]
            );

            if (battle.creature_hp[actual_target_idx] == 0) {
                battle.is_alive[actual_target_idx] = false;
                msg!("Creature {} died!", actual_target_idx);
            }
        }
        let alive_creatures: Vec<usize> = battle
            .is_alive
            .iter()
            .enumerate()
            .filter(|(_, &alive)| alive)
            .map(|(idx, _)| idx)
            .collect();

        if alive_creatures.len() == 1 {
            battle.is_battle_over = true;
            battle.winner = Some(alive_creatures[0] as u8);
            msg!("🏆 Creature {} WINS!", alive_creatures[0]);
        } else if alive_creatures.len() == 0 {
            battle.is_battle_over = true;
            battle.winner = None;
            msg!("⚖️  All creatures died! It's a draw!");
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

    pub creature_hp: [u16; 4],
    pub creature_max_hp: [u16; 4],
    pub creature_atk: [u16; 4],
    pub creature_def: [u16; 4],
    pub is_alive: [bool; 4],

    pub is_battle_over: bool,
    pub winner: Option<u8>,
    pub current_turn: u64,

    pub bump: u8,
}

impl BattleState {
    pub const LEN: usize =
        8 + 32 + (2 * 4) + (2 * 4) + (2 * 4) + (2 * 4) + 1 * 4 + 1 + 1 + 1 + 8 + 1 + 100;
}

#[error_code]
pub enum GameError {
    #[msg("Battle is already over")]
    BattleAlreadyOver,
}
