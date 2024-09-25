use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("GZYx7tr7vmLp92WgCfyaPmP68zm15RdSiCt31D9fUDoV");

#[program]
pub mod flexfi {
    use super::*;

    // Création d'un wallet avec un PDA
    pub fn create_wallet(ctx: Context<CreateWallet>, bump: u8) -> Result<()> {
        let wallet = &mut ctx.accounts.user_wallet;

        // Assigner le propriétaire du wallet
        wallet.owner = *ctx.accounts.owner.key;
        Ok(())
    }

    // Transfert réel de SOL d'un compte à un autre
    pub fn transfer_sol(ctx: Context<TransferSol>, amount: u64, fee_bump: u8) -> Result<()> {
        let balance = ctx.accounts.owner.lamports();
        if balance < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }
    
        // Calcul des frais (1% du montant)
        let fee_amount = amount / 100;
    
        // Transfert du montant - frais
        let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.owner.key(),
            &ctx.accounts.recipient.key(),
            amount - fee_amount,
        );
        anchor_lang::solana_program::program::invoke(
            &transfer_ix,
            &[
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.recipient.to_account_info(),
            ],
        )?;
    
        // Transfert des frais à FlexFi
        let fee_ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.owner.key(),
            &ctx.accounts.flexfi_wallet.key(),
            fee_amount,
        );
        anchor_lang::solana_program::program::invoke(
            &fee_ix,
            &[
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.flexfi_wallet.to_account_info(),
            ],
        )?;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateWallet<'info> {
    // Le compte payer est responsable des frais de transaction
    #[account(mut)]
    pub owner: Signer<'info>,

    // Le nouveau compte utilisateur, initialisé avec un PDA
    #[account(init, seeds = [b"user_wallet", owner.key().as_ref()], bump, payer = owner, space = 8 + 32)]
    pub user_wallet: Account<'info, UserWallet>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferSol<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    #[account(mut)]
    pub flexfi_wallet: SystemAccount<'info>, // Wallet pour les frais
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserWallet {
    pub owner: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient Funds.")]
    InsufficientFunds,
}
