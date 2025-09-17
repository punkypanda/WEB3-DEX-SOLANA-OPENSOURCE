use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer, MintTo, Token};

declare_id!("DEX1111111111111111111111111111111111111111");

#[program]
pub mod web3_dex {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.bump = *ctx.bumps.get("pool").unwrap();
        pool.token_a = ctx.accounts.token_a.key();
        pool.token_b = ctx.accounts.token_b.key();
        pool.lp_mint = ctx.accounts.lp_mint.key();
        pool.admin = ctx.accounts.admin.key();
        Ok(())
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
        // Transfer token A and B from user to pool vaults
        let cpi_accounts_a = Transfer { from: ctx.accounts.user_token_a.to_account_info(), to: ctx.accounts.vault_a.to_account_info(), authority: ctx.accounts.user.to_account_info() };
        let cpi_accounts_b = Transfer { from: ctx.accounts.user_token_b.to_account_info(), to: ctx.accounts.vault_b.to_account_info(), authority: ctx.accounts.user.to_account_info() };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(CpiContext::new(cpi_program.clone(), cpi_accounts_a), amount_a)?;
        token::transfer(CpiContext::new(cpi_program, cpi_accounts_b), amount_b)?;
        // TODO: mint LP tokens proportional to deposit
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, amount_in: u64) -> Result<()> {
        // Basic swap skeleton: charge 0.3% fee
        let fee = (amount_in as u128 * 3 / 1000) as u64; // 0.3%
        let amount_after_fee = amount_in.checked_sub(fee).ok_or(ErrorCode::Overflow)?;
        // Transfer in token from user to vault
        token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), Transfer { from: ctx.accounts.user_source.to_account_info(), to: ctx.accounts.vault_source.to_account_info(), authority: ctx.accounts.user.to_account_info() }), amount_in)?;
        // TODO: compute output amount using constant product formula and update vault balances
        // Send fee to admin: part to pool (keep in vault) and part to admin account
        Ok(())
    }

    pub fn create_token(ctx: Context<CreateToken>, name: String, symbol: String, decimals: u8) -> Result<()> {
        // This instruction will be used to record metadata on-chain if desired.
        // Actual mint creation and fee transfer are handled off-chain via backend and Anchor client.
        Ok(())
    }

    pub fn lock_liquidity(ctx: Context<LockLiquidity>, lock_period: i64) -> Result<()> {
        let lock = &mut ctx.accounts.lock_info;
        lock.owner = ctx.accounts.owner.key();
        lock.unlock_ts = Clock::get()?.unix_timestamp.checked_add(lock_period).ok_or(ErrorCode::Overflow)?;
        Ok(())
    }
}

#[account]
pub struct Pool {
    pub bump: u8,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub lp_mint: Pubkey,
    pub admin: Pubkey,
}

#[account]
pub struct LockInfo {
    pub owner: Pubkey,
    pub unlock_ts: i64,
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(init, payer = admin, space = 8 + 32*4 + 1, seeds = [b"pool", token_a.key().as_ref(), token_b.key().as_ref()], bump)]
    pub pool: Account<'info, Pool>,
    pub token_a: Account<'info, Mint>,
    pub token_b: Account<'info, Mint>,
    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)] pub pool: Account<'info, Pool>,
    #[account(mut)] pub user: Signer<'info>,
    #[account(mut)] pub user_token_a: Account<'info, TokenAccount>,
    #[account(mut)] pub user_token_b: Account<'info, TokenAccount>,
    #[account(mut)] pub vault_a: Account<'info, TokenAccount>,
    #[account(mut)] pub vault_b: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)] pub pool: Account<'info, Pool>,
    #[account(mut)] pub user: Signer<'info>,
    #[account(mut)] pub user_source: Account<'info, TokenAccount>,
    #[account(mut)] pub user_destination: Account<'info, TokenAccount>,
    #[account(mut)] pub vault_source: Account<'info, TokenAccount>,
    #[account(mut)] pub vault_destination: Account<'info, TokenAccount>,
    #[account(mut)] pub admin_account: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct LockLiquidity<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8)]
    pub lock_info: Account<'info, LockInfo>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Overflow error")]
    Overflow,
}