use anchor_lang::prelude::*;
use anchor_lang::system_program;

pub use crate::states::{CollectionPool, Offer, Vault};

#[derive(Accounts)]
pub struct OfferLoan<'info> {
    #[account(
        init,
        seeds=[
            b"offer",
            collection_pool.key().as_ref(),
            lender.key().as_ref(),
            &collection_pool.total_offers.to_le_bytes(),
        ],
        bump,
        payer=lender,
        space=Offer::LEN
    )]
    pub offer_loan: Box<Account<'info, Offer>>,

    #[account(
        init,
        seeds=[
            b"vault",
            collection_pool.key().as_ref(),
            lender.key().as_ref(),
            &collection_pool.total_offers.to_le_bytes(),
        ],
        bump,
        payer = lender,
        space = Vault::LEN
    )]
    pub vault_account: Account<'info, Vault>,

    #[account(mut)]
    pub collection_pool: Box<Account<'info, CollectionPool>>,

    #[account(mut)]
    pub lender: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> OfferLoan<'info> {
    fn transfer_to_vault_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, system_program::Transfer<'info>> {
        let cpi_accounts = system_program::Transfer {
            from: self.lender.to_account_info().clone(),
            to: self.vault_account.to_account_info().clone(),
        };

        CpiContext::new(self.system_program.to_account_info(), cpi_accounts)
    }
}

pub fn handler(ctx: Context<OfferLoan>, offer_amount: u64) -> Result<()> {
    let offer_account = &mut ctx.accounts.offer_loan;
    let collection = &mut ctx.accounts.collection_pool;
    let vault = &mut ctx.accounts.vault_account;

    msg!("Collection Pool Key: {:?}", collection.key());
    msg!("Lender Key: {:?}", ctx.accounts.lender.key());
    msg!("Total Offers: {:?}", collection.total_offers);

    offer_account.collection = collection.key();
    offer_account.offer_lamport_amount = offer_amount;
    offer_account.repay_lamport_amount = offer_amount + offer_amount * 10 / 100;
    offer_account.lender = ctx.accounts.lender.key();
    offer_account.bump = ctx.bumps.offer_loan;

    collection.total_offers += 1;

    vault.offer = offer_account.key();
    vault.bump = ctx.bumps.vault_account;

    system_program::transfer(ctx.accounts.transfer_to_vault_context(), offer_amount)?;

    Ok(())
}

// CreatePool struct and handler remain unchanged
#[derive(Accounts)]
#[instruction(collection_id: Pubkey, pool_name: String)]
pub struct CreatePool<'info> {
    #[account(
        init,
        seeds=[
            b"collection-pool",
            collection_id.key().as_ref(),
            pool_name.as_bytes(),
            authority.key().as_ref()
        ],
        bump,
        payer=authority,
        space=CollectionPool::LEN
    )]
    pub collection_pool: Box<Account<'info, CollectionPool>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_pool_handler(ctx: Context<CreatePool>, collection_id: Pubkey,pool_name: String, duration: i64) -> Result<()> {
    let collection = &mut ctx.accounts.collection_pool;

    collection.collection_id = collection_id;
    collection.pool_owner = ctx.accounts.authority.key();
    collection.duration = duration;
    collection.total_offers = 0;
    collection.pool_name = pool_name;
    collection.bump = ctx.bumps.collection_pool;

    Ok(())
}