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

    // Transfert en une fois avec frais de 1 %
    pub fn transfer_sol(ctx: Context<TransferSol>, amount: u64) -> Result<()> {
        let balance = ctx.accounts.owner.lamports();
        if balance < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        // Calcul des frais (1 % du montant)
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

    // Transfert avec BNPL (paiement en plusieurs fois) et frais spécifiques
    pub fn bnpl_transfer(
        ctx: Context<BnplTransfer>, 
        amount: u64, 
        num_payments: u8  // Nombre de paiements (6, 12...)
    ) -> Result<()> {
        let balance = ctx.accounts.owner.lamports();
        if balance < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        // Calcul des frais pour le Shopper (12 % du montant total)
        let shopper_fee = (amount * 12) / 100;
        
        // Calcul des frais pour le Commerçant (2 % du montant total)
        let merchant_fee = (amount * 2) / 100;
        
        // Diviser le montant en paiements égaux pour le BNPL
        let payment_amount = (amount + shopper_fee) / num_payments as u64;

        // Faire les paiements échelonnés pour chaque mois
        for _ in 0..num_payments {
            let ix = anchor_lang::solana_program::system_instruction::transfer(
                &ctx.accounts.owner.key(),
                &ctx.accounts.recipient.key(),
                payment_amount,
            );
            anchor_lang::solana_program::program::invoke(
                &ix,
                &[
                    ctx.accounts.owner.to_account_info(),
                    ctx.accounts.recipient.to_account_info(),
                ],
            )?;
        }

        // Transfert des frais du Shopper à FlexFi
        let shopper_fee_ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.owner.key(),
            &ctx.accounts.flexfi_wallet.key(),
            shopper_fee,
        );
        anchor_lang::solana_program::program::invoke(
            &shopper_fee_ix,
            &[
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.flexfi_wallet.to_account_info(),
            ],
        )?;

        // Transfert des frais du Commerçant à FlexFi
        let merchant_fee_ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.owner.key(),
            &ctx.accounts.flexfi_wallet.key(),
            merchant_fee,
        );
        anchor_lang::solana_program::program::invoke(
            &merchant_fee_ix,
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
    #[account(mut)]
    pub owner: Signer<'info>,

    // Initialisation d’un wallet utilisateur avec un PDA
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
    pub flexfi_wallet: SystemAccount<'info>, // Wallet pour les frais FlexFi
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BnplTransfer<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    #[account(mut)]
    pub flexfi_wallet: SystemAccount<'info>,  // Wallet pour les frais FlexFi
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
