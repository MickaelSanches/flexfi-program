use anchor_lang::prelude::*;
use solana_program::blake3::hash as blake3_hash;

declare_id!("GZYx7tr7vmLp92WgCfyaPmP68zm15RdSiCt31D9fUDoV");

#[program]
pub mod flexfi {
    use super::*;

    // 1. Création d'un wallet FlexFi avec mot de passe
    pub fn create_wallet(ctx: Context<CreateWallet>, seed_phrase: String, password: String) -> Result<()> {
        let wallet = &mut ctx.accounts.user_wallet;
        wallet.owner = *ctx.accounts.owner.key;
        wallet.seed_phrase = seed_phrase;

        // Hasher le mot de passe avant de le stocker
        let password_hash = blake3_hash(password.as_bytes());
        wallet.password_hash = password_hash.as_ref().to_vec();

        Ok(())
    }

    // 2. Demander un paiement en plusieurs fois
    pub fn request_installment_payment(
        ctx: Context<RequestInstallmentPayment>,
        amount: u64,
        installments: u8,
    ) -> Result<()> {
        let payment = &mut ctx.accounts.installment_payment;
        payment.owner = *ctx.accounts.owner.key;
        payment.amount = amount;
        payment.installments = installments;
        payment.remaining_installments = installments;
        payment.total_paid = 0;
        Ok(())
    }

    // 3. Enregistrer une transaction dans l'historique
    pub fn log_transaction(ctx: Context<LogTransaction>, description: String, amount: u64) -> Result<()> {
        let transaction = &mut ctx.accounts.transaction;
        transaction.owner = *ctx.accounts.owner.key;
        transaction.description = description;
        transaction.amount = amount;
        transaction.timestamp = Clock::get()?.unix_timestamp;
        Ok(())
    }

    // 4. Créer une notification
    pub fn create_notification(ctx: Context<CreateNotification>, message: String) -> Result<()> {
        let notification = &mut ctx.accounts.notification;
        notification.owner = *ctx.accounts.owner.key;
        notification.message = message;
        notification.timestamp = Clock::get()?.unix_timestamp;
        Ok(())
    }

    // 5. Mise à jour des informations de compte (sans toucher à la seed phrase)
    pub fn update_account(ctx: Context<UpdateAccount>, new_password: String) -> Result<()> {
        let wallet = &mut ctx.accounts.user_wallet;

        // Hasher le nouveau mot de passe
        let new_password_hash = blake3_hash(new_password.as_bytes());
        wallet.password_hash = new_password_hash.as_ref().to_vec();

        Ok(())
    }

    // 6. Consultation sécurisée de la seed phrase
    pub fn view_seed_phrase(ctx: Context<ViewSeedPhrase>, password: String) -> Result<String> {
        let wallet = &ctx.accounts.user_wallet;

        // Hasher le mot de passe fourni pour comparaison
        let password_hash = blake3_hash(password.as_bytes());

        if password_hash.as_ref() == wallet.password_hash {
            Ok(wallet.seed_phrase.clone())
        } else {
            Err(ErrorCode::InvalidPassword.into())
        }
    }

    // 7. Vérification du mot de passe pour la connexion
    pub fn verify_password(ctx: Context<VerifyPassword>, password: String) -> Result<()> {
        let wallet = &ctx.accounts.user_wallet;

        // Hasher le mot de passe fourni pour comparaison
        let password_hash = blake3_hash(password.as_bytes());

        if password_hash.as_ref() == wallet.password_hash {
            Ok(())
        } else {
            Err(ErrorCode::InvalidPassword.into())
        }
    }
}

// Structure pour représenter un wallet utilisateur
#[account]
pub struct UserWallet {
    pub owner: Pubkey,
    pub seed_phrase: String,
    pub password_hash: Vec<u8>,
}

// Structure pour représenter un paiement échelonné
#[account]
pub struct InstallmentPayment {
    pub owner: Pubkey,
    pub amount: u64,
    pub installments: u8,
    pub remaining_installments: u8,
    pub total_paid: u64,
}

// Structure pour enregistrer une transaction dans l'historique
#[account]
pub struct Transaction {
    pub owner: Pubkey,
    pub description: String,
    pub amount: u64,
    pub timestamp: i64,
}

// Structure pour gérer les notifications
#[account]
pub struct Notification {
    pub owner: Pubkey,
    pub message: String,
    pub timestamp: i64,
}

// Instructions d'initialisation pour les différents comptes
#[derive(Accounts)]
pub struct CreateWallet<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 64 + 4 + 32)] // Ajusté
    pub user_wallet: Account<'info, UserWallet>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RequestInstallmentPayment<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 1 + 1 + 8)]
    pub installment_payment: Account<'info, InstallmentPayment>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LogTransaction<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 64 + 8)] // Ajusté
    pub transaction: Account<'info, Transaction>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateNotification<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 64 + 8)] // Ajusté
    pub notification: Account<'info, Notification>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct UpdateAccount<'info> {
    #[account(mut, has_one = owner)]
    pub user_wallet: Account<'info, UserWallet>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ViewSeedPhrase<'info> {
    #[account(mut, has_one = owner)]
    pub user_wallet: Account<'info, UserWallet>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerifyPassword<'info> {
    #[account(mut, has_one = owner)]
    pub user_wallet: Account<'info, UserWallet>,
    pub owner: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Seed Phrase.")]
    InvalidSeedPhrase,
    #[msg("Invalid Password.")]
    InvalidPassword,
}
