use anchor_lang::prelude::*;
use solana_program::blake3::hash as blake3_hash;

declare_id!("GZYx7tr7vmLp92WgCfyaPmP68zm15RdSiCt31D9fUDoV");

#[program]
pub mod flexfi {
    use super::*;

    pub fn create_wallet(ctx: Context<CreateWallet>, seed_phrase: String, password: String) -> Result<()> {
        let wallet = &mut ctx.accounts.user_wallet;
        wallet.owner = *ctx.accounts.owner.key;
        let wallet_id = blake3_hash(wallet.owner.as_ref());
        wallet.wallet_id = wallet_id.as_ref().to_vec();
        wallet.seed_phrase = seed_phrase;
        let password_hash = blake3_hash(password.as_bytes());
        wallet.password_hash = password_hash.as_ref().to_vec();
        wallet.balance_usd = 0;
        wallet.balance_eur = 0;
        wallet.balance_sol = 0;
        wallet.balance_btc = 0;
        wallet.balance_usdc = 0;
        wallet.balance_eurcv = 0;
        Ok(())
    }

    pub fn get_wallet_balance(ctx: Context<GetWalletBalance>, asset: String) -> Result<u64> {
        let wallet = &ctx.accounts.user_wallet;
        let balance = match asset.as_str() {
            "USD" => wallet.balance_usd,
            "EUR" => wallet.balance_eur,
            "SOL" => wallet.balance_sol,
            "BTC" => wallet.balance_btc,
            "USDC" => wallet.balance_usdc,
            "EURCV" => wallet.balance_eurcv,
            _ => return Err(ErrorCode::InvalidAsset.into()),
        };
        Ok(balance)
    }

    pub fn direct_payment(ctx: Context<DirectPayment>, amount: u64, asset: String) -> Result<()> {
        let wallet = &mut ctx.accounts.user_wallet;

        let balance = match asset.as_str() {
            "USD" => &mut wallet.balance_usd,
            "EUR" => &mut wallet.balance_eur,
            "SOL" => &mut wallet.balance_sol,
            "BTC" => &mut wallet.balance_btc,
            "USDC" => &mut wallet.balance_usdc,
            "EURCV" => &mut wallet.balance_eurcv,
            _ => return Err(ErrorCode::InvalidAsset.into()),
        };

        if *balance < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        *balance -= amount;

        let recipient_wallet = &mut ctx.accounts.recipient_wallet;

        let recipient_balance = match asset.as_str() {
            "USD" => &mut recipient_wallet.balance_usd,
            "EUR" => &mut recipient_wallet.balance_eur,
            "SOL" => &mut recipient_wallet.balance_sol,
            "BTC" => &mut recipient_wallet.balance_btc,
            "USDC" => &mut recipient_wallet.balance_usdc,
            "EURCV" => &mut recipient_wallet.balance_eurcv,
            _ => return Err(ErrorCode::InvalidAsset.into()),
        };

        *recipient_balance += amount;

        Ok(())
    }

    pub fn payment_via_buyer_id(ctx: Context<PaymentViaBuyerId>, buyer_id: Vec<u8>, amount: u64, asset: String) -> Result<()> {
        let wallet = &mut ctx.accounts.user_wallet;

        let balance = match asset.as_str() {
            "USD" => &mut wallet.balance_usd,
            "EUR" => &mut wallet.balance_eur,
            "SOL" => &mut wallet.balance_sol,
            "BTC" => &mut wallet.balance_btc,
            "USDC" => &mut wallet.balance_usdc,
            "EURCV" => &mut wallet.balance_eurcv,
            _ => return Err(ErrorCode::InvalidAsset.into()),
        };

        if *balance < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        *balance -= amount;

        let recipient_wallet = &mut ctx.accounts.recipient_wallet;

        let recipient_balance = match asset.as_str() {
            "USD" => &mut recipient_wallet.balance_usd,
            "EUR" => &mut recipient_wallet.balance_eur,
            "SOL" => &mut recipient_wallet.balance_sol,
            "BTC" => &mut recipient_wallet.balance_btc,
            "USDC" => &mut recipient_wallet.balance_usdc,
            "EURCV" => &mut recipient_wallet.balance_eurcv,
            _ => return Err(ErrorCode::InvalidAsset.into()),
        };

        *recipient_balance += amount;

        Ok(())
    }

    pub fn payment_via_qr(ctx: Context<PaymentViaQR>, qr_data: String, amount: u64, asset: String) -> Result<()> {
        let wallet = &mut ctx.accounts.user_wallet;

        let balance = match asset.as_str() {
            "USD" => &mut wallet.balance_usd,
            "EUR" => &mut wallet.balance_eur,
            "SOL" => &mut wallet.balance_sol,
            "BTC" => &mut wallet.balance_btc,
            "USDC" => &mut wallet.balance_usdc,
            "EURCV" => &mut wallet.balance_eurcv,
            _ => return Err(ErrorCode::InvalidAsset.into()),
        };

        if *balance < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        *balance -= amount;

        let recipient_wallet = &mut ctx.accounts.recipient_wallet;

        let recipient_balance = match asset.as_str() {
            "USD" => &mut recipient_wallet.balance_usd,
            "EUR" => &mut recipient_wallet.balance_eur,
            "SOL" => &mut recipient_wallet.balance_sol,
            "BTC" => &mut recipient_wallet.balance_btc,
            "USDC" => &mut recipient_wallet.balance_usdc,
            "EURCV" => &mut recipient_wallet.balance_eurcv,
            _ => return Err(ErrorCode::InvalidAsset.into()),
        };

        *recipient_balance += amount;

        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, from_asset: String, to_asset: String, amount: u64) -> Result<u64> {
        let oracle = &ctx.accounts.oracle;
        let conversion_rate = oracle.get_conversion_rate(&from_asset, &to_asset)?;
        let swap_amount = amount * conversion_rate;

        let wallet = &mut ctx.accounts.user_wallet;

        match from_asset.as_str() {
            "USD" => wallet.balance_usd -= amount,
            "EUR" => wallet.balance_eur -= amount,
            "SOL" => wallet.balance_sol -= amount,
            "BTC" => wallet.balance_btc -= amount,
            "USDC" => wallet.balance_usdc -= amount,
            "EURCV" => wallet.balance_eurcv -= amount,
            _ => return Err(ErrorCode::InvalidAsset.into()),
        };

        match to_asset.as_str() {
            "USD" => wallet.balance_usd += swap_amount,
            "EUR" => wallet.balance_eur += swap_amount,
            "SOL" => wallet.balance_sol += swap_amount,
            "BTC" => wallet.balance_btc += swap_amount,
            "USDC" => wallet.balance_usdc += swap_amount,
            "EURCV" => wallet.balance_eurcv += swap_amount,
            _ => return Err(ErrorCode::InvalidAsset.into()),
        };

        Ok(swap_amount)
    }

    pub fn update_conversion_rate(ctx: Context<UpdateConversionRate>, from_asset: String, to_asset: String, rate: u64) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle;
        oracle.update_rate(from_asset, to_asset, rate);
        Ok(())
    }

    pub fn view_seed_phrase(ctx: Context<ViewSeedPhrase>, password: String) -> Result<String> {
        let wallet = &ctx.accounts.user_wallet;
        let password_hash = blake3_hash(password.as_bytes());

        if password_hash.as_ref() == wallet.password_hash {
            Ok(wallet.seed_phrase.clone())
        } else {
            Err(ErrorCode::InvalidPassword.into())
        }
    }

    pub fn verify_password(ctx: Context<VerifyPassword>, password: String) -> Result<()> {
        let wallet = &ctx.accounts.user_wallet;
        let password_hash = blake3_hash(password.as_bytes());

        if password_hash.as_ref() == wallet.password_hash {
            Ok(())
        } else {
            Err(ErrorCode::InvalidPassword.into())
        }
    }

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

    pub fn log_transaction(ctx: Context<LogTransaction>, description: String, amount: u64) -> Result<()> {
        let transaction = &mut ctx.accounts.transaction;
        transaction.owner = *ctx.accounts.owner.key;
        transaction.description = description;
        transaction.amount = amount;
        transaction.timestamp = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn create_notification(ctx: Context<CreateNotification>, message: String) -> Result<()> {
        let notification = &mut ctx.accounts.notification;
        notification.owner = *ctx.accounts.owner.key;
        notification.message = message;
        notification.timestamp = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn update_account(ctx: Context<UpdateAccount>, new_password: String) -> Result<()> {
        let wallet = &mut ctx.accounts.user_wallet;
        let new_password_hash = blake3_hash(new_password.as_bytes());
        wallet.password_hash = new_password_hash.as_ref().to_vec();
        Ok(())
    }
}

#[account]
pub struct UserWallet {
    pub owner: Pubkey,
    pub wallet_id: Vec<u8>,
    pub seed_phrase: String,
    pub password_hash: Vec<u8>,
    pub balance_usd: u64,
    pub balance_eur: u64,
    pub balance_sol: u64,
    pub balance_btc: u64,
    pub balance_usdc: u64,
    pub balance_eurcv: u64,
}

#[account]
pub struct InstallmentPayment {
    pub owner: Pubkey,
    pub amount: u64,
    pub installments: u8,
    pub remaining_installments: u8,
    pub total_paid: u64,
}

#[account]
pub struct Transaction {
    pub owner: Pubkey,
    pub description: String,
    pub amount: u64,
    pub timestamp: i64,
}

#[account]
pub struct Notification {
    pub owner: Pubkey,
    pub message: String,
    pub timestamp: i64,
}

#[account]
pub struct Oracle {
    pub usd_to_btc_rate: u64,
    pub btc_to_usd_rate: u64,
    pub usd_to_eur_rate: u64,
    pub eur_to_usd_rate: u64,
    pub usd_to_sol_rate: u64,
    pub sol_to_usd_rate: u64,
    pub usd_to_usdc_rate: u64,
    pub usdc_to_usd_rate: u64,
    pub usd_to_eurcv_rate: u64,
    pub eurcv_to_usd_rate: u64,
    pub eur_to_btc_rate: u64,
    pub btc_to_eur_rate: u64,
    pub eur_to_sol_rate: u64,
    pub sol_to_eur_rate: u64,
    pub eur_to_usdc_rate: u64,
    pub usdc_to_eur_rate: u64,
    pub eur_to_eurcv_rate: u64,
    pub eurcv_to_eur_rate: u64,
}

impl Oracle {
    pub fn get_conversion_rate(&self, from_asset: &str, to_asset: &str) -> Result<u64> {
        match (from_asset, to_asset) {
            ("USD", "BTC") => Ok(self.usd_to_btc_rate),
            ("BTC", "USD") => Ok(self.btc_to_usd_rate),
            ("USD", "EUR") => Ok(self.usd_to_eur_rate),
            ("EUR", "USD") => Ok(self.eur_to_usd_rate),
            ("USD", "SOL") => Ok(self.usd_to_sol_rate),
            ("SOL", "USD") => Ok(self.sol_to_usd_rate),
            ("USD", "USDC") => Ok(self.usd_to_usdc_rate),
            ("USDC", "USD") => Ok(self.usdc_to_usd_rate),
            ("USD", "EURCV") => Ok(self.usd_to_eurcv_rate),
            ("EURCV", "USD") => Ok(self.eurcv_to_usd_rate),
            ("EUR", "BTC") => Ok(self.eur_to_btc_rate),
            ("BTC", "EUR") => Ok(self.btc_to_eur_rate),
            ("EUR", "SOL") => Ok(self.eur_to_sol_rate),
            ("SOL", "EUR") => Ok(self.sol_to_eur_rate),
            ("EUR", "USDC") => Ok(self.eur_to_usdc_rate),
            ("USDC", "EUR") => Ok(self.usdc_to_eur_rate),
            ("EUR", "EURCV") => Ok(self.eur_to_eurcv_rate),
            ("EURCV", "EUR") => Ok(self.eurcv_to_eur_rate),
            _ => Err(ErrorCode::InvalidConversion.into()),
        }
    }

    pub fn update_rate(&mut self, from_asset: String, to_asset: String, rate: u64) {
        match (from_asset.as_str(), to_asset.as_str()) {
            ("USD", "BTC") => self.usd_to_btc_rate = rate,
            ("BTC", "USD") => self.btc_to_usd_rate = rate,
            ("USD", "EUR") => self.usd_to_eur_rate = rate,
            ("EUR", "USD") => self.eur_to_usd_rate = rate,
            ("USD", "SOL") => self.usd_to_sol_rate = rate,
            ("SOL", "USD") => self.sol_to_usd_rate = rate,
            ("USD", "USDC") => self.usd_to_usdc_rate = rate,
            ("USDC", "USD") => self.usdc_to_usd_rate = rate,
            ("USD", "EURCV") => self.usd_to_eurcv_rate = rate,
            ("EURCV", "USD") => self.eurcv_to_usd_rate = rate,
            ("EUR", "BTC") => self.eur_to_btc_rate = rate,
            ("BTC", "EUR") => self.btc_to_eur_rate = rate,
            ("EUR", "SOL") => self.eur_to_sol_rate = rate,
            ("SOL", "EUR") => self.sol_to_eur_rate = rate,
            ("EUR", "USDC") => self.eur_to_usdc_rate = rate,
            ("USDC", "EUR") => self.usdc_to_eur_rate = rate,
            ("EUR", "EURCV") => self.eur_to_eurcv_rate = rate,
            ("EURCV", "EUR") => self.eurcv_to_eur_rate = rate,
            _ => (),
        }
    }
}

#[derive(Accounts)]
pub struct CreateWallet<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 32 + 64 + 4 + 32 + 6*8)]
    pub user_wallet: Account<'info, UserWallet>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetWalletBalance<'info> {
    #[account(mut, has_one = owner)]
    pub user_wallet: Account<'info, UserWallet>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct DirectPayment<'info> {
    #[account(mut, has_one = owner)]
    pub user_wallet: Account<'info, UserWallet>,
    #[account(mut)]
    pub recipient_wallet: Account<'info, UserWallet>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct PaymentViaBuyerId<'info> {
    #[account(mut, has_one = owner)]
    pub user_wallet: Account<'info, UserWallet>,
    #[account(mut)]
    pub recipient_wallet: Account<'info, UserWallet>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct PaymentViaQR<'info> {
    #[account(mut, has_one = owner)]
    pub user_wallet: Account<'info, UserWallet>,
    #[account(mut)]
    pub recipient_wallet: Account<'info, UserWallet>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut, has_one = owner)]
    pub user_wallet: Account<'info, UserWallet>,
    pub oracle: Account<'info, Oracle>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateConversionRate<'info> {
    #[account(mut)]
    pub oracle: Account<'info, Oracle>,
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
    #[account(init, payer = owner, space = 8 + 32 + 4 + 64 + 8)]
    pub transaction: Account<'info, Transaction>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateNotification<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 64 + 8)]
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

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Seed Phrase.")]
    InvalidSeedPhrase,
    #[msg("Invalid Password.")]
    InvalidPassword,
    #[msg("Insufficient Funds.")]
    InsufficientFunds,
    #[msg("Invalid Conversion.")]
    InvalidConversion,
    #[msg("Invalid Asset.")]
    InvalidAsset,
}
