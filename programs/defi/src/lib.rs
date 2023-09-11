use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    native_token::LAMPORTS_PER_SOL, program::invoke, system_instruction,
};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use num_traits::pow;
use pyth_sdk_solana::load_price_feed_from_account_info;
use spl_associated_token_account::get_associated_token_address;
use std::mem::size_of;
declare_id!("FXJBsBUjrjCPJxuFMtu1nAYsPSzjnWS1GCD3Ynzpe6Q7");
pub const sol_usd: &str = "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix";
#[program]
pub mod defi {
    pub const receipt_token: &str = "DAYN3qFmf2q3PpGHVWKiXi7tthgahXfjDkzJno9tF2ox";

    // use std::arch::x86_64::_bextr2_u32;

    use super::*;
    pub fn create_lp_token_bag(
        ctx: Context<CreateLpTokenBag>,
        lp_mint_address: Pubkey,
    ) -> Result<()> {
        Ok(())
    }
    pub fn intialize_pool_profile(
        ctx: Context<IntializePoolProfile>,
        lp_mint_address: Pubkey,
        program_lp_bag_bump: u8,
    ) -> Result<()> {
        let pool: &mut Account<'_, PoolProfile> = &mut ctx.accounts.pool_profile;
        let mint_account = &mut ctx.accounts.program_lp_token_bag;
        if mint_account.mint == Pubkey::default() {
            return err!(ErrorCode::MintNotFound);
        }
        if mint_account.mint != lp_mint_address {
            return err!(ErrorCode::MintMisMatch);
        }

        msg!("Mint is {}", mint_account.mint);

        pool.total_staked_amount = 0.0;
        pool.total_borrowed_amount = 0.0;
        pool.mint = lp_mint_address;
        Ok(())
    }
    pub fn intialize_user_mint_profile(
        ctx: Context<IntializeUserMintProfile>,
        lp_mint_address: Pubkey,
        program_lp_bag_bump: u8,
    ) -> Result<()> {
        let userMintProfile = &mut ctx.accounts.user_mint_profile;

        msg!("Pool owner is {}", ctx.accounts.user.key());

        userMintProfile.total_supplied_amount = 0.0;
        userMintProfile.total_borrowed_amount = 0.0;
        userMintProfile.mint = lp_mint_address;
        userMintProfile.owner = ctx.accounts.user.key();
        Ok(())
    }
    pub fn intialize_user_profile(_ctx: Context<IntializeUserProfile>) -> Result<()> {
        let user_profile = &mut _ctx.accounts.user_profile;
        user_profile.total_supplied_accounts = 0;
        user_profile.total_supplied_amount = 0.0;
        user_profile.total_borrowed_amount = 0.0;

        Ok(())
    }
    pub fn supply(
        _ctx: Context<Supply_>,
        mint_address: Pubkey,
        program_bag_bump: u8,
        supply_count: String,
        user_profile_bump: u8,
        user_mint_profile_bump: u8,
        amount: f64,
    ) -> Result<()> {
        let user = &mut _ctx.accounts.user;
        let programAccount = &mut _ctx.accounts.program_mint_token_bag;
        let userProfile = &mut _ctx.accounts.user_profile;
        let userMintProfile = &mut _ctx.accounts.user_mint_profile;
        let supply = &mut _ctx.accounts.supply;
        // let userAccount=&mut _ctx.accounts.user_mint_token_acc;
        let decimals = _ctx.accounts.mint.decimals;
        // let receipt_mint = _ctx.accounts.receipt_mint.key();

        //  let user_receipt_bag = get_associated_token_address(&user.key(), &receipt_mint);
        //msg!("The user recepit token bag {} ", user_receipt_bag);
        // let user_mint_bag = get_associated_token_address(&user.key(), &mint_address);
        // msg!("The user mint token bag  is {} ", user_mint_bag);

        supply.owner = _ctx.accounts.user.key();
        supply.index = supply_count.parse().unwrap();
        supply.mint = mint_address;
        let amount_decimals: f64 = amount * pow(10.0, decimals as usize);
        // 2. Ask SPL Token Program to transfer Fino from the user.
        let cpi_ctx: CpiContext<token::Transfer> = CpiContext::new(
            _ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: _ctx.accounts.user_mint_token_acc.to_account_info(),
                authority: _ctx.accounts.user.to_account_info(),
                to: _ctx.accounts.program_mint_token_bag.to_account_info(),
            },
        );
        msg!("Mint decimals are{}", decimals as usize);
        msg!("Amount in decimal is {}", amount_decimals);

        token::transfer(cpi_ctx, amount_decimals as u64)?;
        supply.amount = amount;
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;
        supply.start_time = current_timestamp;
        userMintProfile.total_supplied_amount = userMintProfile.total_supplied_amount + amount;
        userMintProfile.total_supplied_accounts = userMintProfile.total_supplied_accounts + 1;
        userProfile.total_supplied_amount = userProfile.total_supplied_amount + amount;
        userProfile.total_supplied_accounts = userProfile.total_supplied_accounts + 1;

        // minting Recpite Tokens
        //   let seeds = &[receipt_mint.as_ref(), &[receipt_bump]];
        // let signer = [&seeds[..]];

        // let cpi_ctx_reward = CpiContext::new_with_signer(
        //     _ctx.accounts.token_program.to_account_info(),
        //     token::MintTo {
        //         mint: _ctx.accounts.receipt_mint.to_account_info(),
        //         to: _ctx.accounts.user_receipt_bag.to_account_info(),
        //         authority: _ctx.accounts.receipt_mint_authority.to_account_info(),
        //     },
        //     &signer,
        // );
        // //Todo Handle case for diffrent decimals
        // token::mint_to(cpi_ctx_reward, (amount_decimals) as u64)?;
        Ok(())
    }

    pub fn borrow(
        _ctx: Context<Borrow_>,
        borrow_mint_: Pubkey,
        borrow_mint_bump: u8,
        user_profile_bump: u8,
        user_mint_profile_bump: u8,
        borrow_count: String,
        amount: f64,
    ) -> Result<()> {
        let userProfile: &mut Account<'_, UserProfile> = &mut _ctx.accounts.user_profile;
        let user_mint_profile = &mut _ctx.accounts.user_mint_profile;
        let user_borrow_profile = &mut _ctx.accounts.borrow;

        let decimals = _ctx.accounts.borrow_mint.decimals;
        let amount_decimals: f64 = amount * pow(10.0, decimals as usize);
        let supplied_amount_in_decimals: f64 =
            userProfile.total_supplied_amount * pow(10.0, 9 as usize);
        let borrowed_amount_history =
            userProfile.total_borrowed_amount * pow(10.0, decimals as usize);
        let mint_acc = &mut _ctx.accounts.program_borrow_mint_bag;

        if mint_acc.amount < amount_decimals as u64 {
            msg!("Error {}", ErrorCode::LowLiquidity);
            return err!(ErrorCode::LowLiquidity);
        }
        if supplied_amount_in_decimals < amount_decimals {
            msg!("Error{}", ErrorCode::AmountExceedSupply);
            return err!(ErrorCode::AmountExceedSupply);
        }
        if supplied_amount_in_decimals * 0.75 < amount_decimals {
            msg!("Error{}", ErrorCode::SupplyNotEnough);
            return err!(ErrorCode::SupplyNotEnough);
        }
        if (borrowed_amount_history + amount_decimals) > supplied_amount_in_decimals * 0.75 {
            msg!("Error{}", ErrorCode::RepayLoanOrIncreaseSupply);
            return err!(ErrorCode::RepayLoanOrIncreaseSupply);
        }
        let borrow_mint = _ctx.accounts.borrow_mint.key();
        let seeds = &[borrow_mint.as_ref(), &[borrow_mint_bump]];
        let signer = [&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            _ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: _ctx.accounts.program_borrow_mint_bag.to_account_info(),
                to: _ctx.accounts.user_borrow_bag.to_account_info(),
                authority: _ctx.accounts.program_borrow_mint_bag.to_account_info(),
            },
            &signer,
        );
        userProfile.total_borrowed_amount = userProfile.total_borrowed_amount + amount;
        user_mint_profile.total_borrowed_accounts=user_mint_profile.total_borrowed_accounts+1;
        user_mint_profile.total_borrowed_amount = user_mint_profile.total_borrowed_amount + amount;
        user_borrow_profile.index = borrow_count.parse().unwrap();
        user_borrow_profile.borrow_amount = amount;
        user_borrow_profile.borrow_mint = borrow_mint;
        user_borrow_profile.owner=_ctx.accounts.user.key();
        user_borrow_profile.active=true;
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;
        user_borrow_profile.time_stamp = current_timestamp;
        let result = token::transfer(cpi_ctx, amount_decimals as u64);
        if let Err(err) = result {
            msg!("Token transfer failed: {:?}", err);
            return Err(err);
        }
        msg!("Amount Approved and transffered !");

        Ok(())
    }
    pub fn repay(
        _ctx: Context<Repay>,
        borrow_mint_: Pubkey,
        borrow_bump: u8,
        borrow_mint_bump: u8,
        user_profile_bump: u8,
        user_mint_profile_bump: u8,
        borrow_count: String,
    ) -> Result<()> {
        let borrow = &mut _ctx.accounts.borrow;
        if borrow.active==false
        {
            return err!(ErrorCode::AlreadyPaid);
        }
        let user_profile = &mut _ctx.accounts.user_profile;
        let user_mint_profile = &mut _ctx.accounts.user_mint_profile;
        let decimals = _ctx.accounts.borrow_mint.decimals;
        let amount_in_decimals = borrow.borrow_amount * pow(10.0, decimals as usize);

        let borrow_mint = _ctx.accounts.borrow_mint.key();
        let seeds = &[borrow_mint.as_ref(), &[borrow_mint_bump]];
        let signer = [&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            _ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: _ctx.accounts.user_borrow_bag.to_account_info(),
                to: _ctx.accounts.program_borrow_mint_bag.to_account_info(),
                authority: _ctx.accounts.user.to_account_info(),
            },
            &signer,
        );
        let result = token::transfer(cpi_ctx, amount_in_decimals as u64);
        if let Err(err) = result {
            msg!("Token transfer failed: {:?}", err);
            return Err(err);
        }
        borrow.active = false;
        user_mint_profile.total_borrowed_amount =
            user_mint_profile.total_borrowed_amount - borrow.borrow_amount;
        user_profile.total_borrowed_amount =
            user_profile.total_borrowed_amount - borrow.borrow_amount;

        Ok(())
    }

    pub fn repay_once(
        _ctx: Context<RepayOnce>,
        borrow_mint_: Pubkey,
        borrow_mint_bump: u8,
        user_profile_bump: u8,
        user_mint_profile_bump: u8,
        amount:f64,
    ) -> Result<()> {
        
        // if borrow.active==false
        // {
        //     return err!(ErrorCode::AlreadyPaid);
        // }
        let user_profile = &mut _ctx.accounts.user_profile;
        let user_mint_profile = &mut _ctx.accounts.user_mint_profile;
        let decimals = _ctx.accounts.borrow_mint.decimals;
        let amount_in_decimals = amount * pow(10.0, decimals as usize);
        if user_profile.total_borrowed_amount< amount || user_mint_profile.total_borrowed_amount< amount
        {
            return err!(ErrorCode::AmountGreater);
        }
        let borrow_mint = _ctx.accounts.borrow_mint.key();
        let seeds = &[borrow_mint.as_ref(), &[borrow_mint_bump]];
        let signer = [&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            _ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: _ctx.accounts.user_borrow_bag.to_account_info(),
                to: _ctx.accounts.program_borrow_mint_bag.to_account_info(),
                authority: _ctx.accounts.user.to_account_info(),
            },
            &signer,
        );
        let result = token::transfer(cpi_ctx, amount_in_decimals as u64);
        if let Err(err) = result {
            msg!("Token transfer failed: {:?}", err);
            return Err(err);
        }
        
        user_mint_profile.total_borrowed_amount =
            user_mint_profile.total_borrowed_amount - amount;
        user_profile.total_borrowed_amount =
            user_profile.total_borrowed_amount - amount;
        msg!("Token transfered sucessfully ! ");
        msg!("User Profile updated sucessfully  ! ");

        Ok(())
    }

    pub fn withdraw(
        _ctx: Context<Withdraw>,
        mint_address: Pubkey,
        user_profile_bump: u8,
        user_mint_profile_bump: u8,
        amount: f64,
        program_mint_bump: u8,
    ) -> Result<()> {
        // let p = load_price_feed_from_account_info(&_ctx.accounts.sol_usd_account).unwrap();
        // let (current_price) = p.get_price_unchecked();
        // let price: f64 = (u64::try_from(current_price.price).unwrap() as f64)
        //     / 10f64.powf(f64::try_from(-current_price.expo).unwrap());

        //  msg!("The price of sol is {} ", price);
        // ......at
        let user_balance = _ctx.accounts.user_profile.total_supplied_amount;
        let user_supplied_balance=_ctx.accounts.user_mint_profile.total_supplied_amount;
        let decimals = _ctx.accounts.mint.decimals;
        let user_total_borrowed_amount = _ctx.accounts.user_profile.total_borrowed_amount;
        let user_mint_total_borrowed_amount = _ctx.accounts.user_mint_profile.total_borrowed_amount;
        let user_balance_avialable=user_balance-user_total_borrowed_amount/0.75;
        let user_mint_profile: &mut Account<'_, UserMintProfile> = &mut _ctx.accounts.user_mint_profile;
        let user_profile = &mut _ctx.accounts.user_profile;
//

        if amount > user_balance || amount > user_supplied_balance {
            msg!("Error{}", ErrorCode::AmountExceedSupply);
            return err!(ErrorCode::AmountExceedSupply);
        }
        if amount > user_balance_avialable{
            msg!("Error{}", ErrorCode::SupplyLocked);
            return err!(ErrorCode::SupplyLocked);
        }
       
        // transfer code here
        let mint_address = _ctx.accounts.mint.key();
        let seeds: &[&[u8]; 2] = &[mint_address.as_ref(), &[program_mint_bump]];
        let signer = [&seeds[..]];
        user_profile.total_supplied_amount = user_profile.total_supplied_amount - amount;
        user_mint_profile.total_supplied_amount = user_mint_profile.total_supplied_amount - amount;
        let cpi_ctx = CpiContext::new_with_signer(
            _ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: _ctx.accounts.program_mint_token_bag.to_account_info(),
                authority: _ctx.accounts.program_mint_token_bag.to_account_info(),
                to: _ctx.accounts.user_mint_token_acc.to_account_info(),
            },
            &signer,
        );
        let amount_in_decimals = amount * pow(10.0, decimals as usize);
        let result = token::transfer(cpi_ctx, amount_in_decimals as u64);
        if let Err(err) = result {
            msg!("Token transfer failed: {:?}", err);
            return Err(err);
        }
        Ok(())
    }
}

#[account]
pub struct Supply {
    index: u64,
    owner: Pubkey,
    start_time: i64,
    mint: Pubkey,
    amount: f64,
}
#[account]
pub struct Borrow {
    index: u64,
    owner: Pubkey,
    time_stamp: i64,
    borrow_mint: Pubkey,
    active: bool,
    borrow_amount: f64,
}
#[account]
pub struct UserMintProfile {
    total_supplied_amount: f64,
    total_borrowed_amount: f64,
    total_borrowed_accounts: u64,
    total_supplied_accounts: u8,
    owner: Pubkey,
    mint: Pubkey,
}
#[account]
pub struct UserProfile {
    total_supplied_amount: f64,
    total_borrowed_amount: f64,
    total_supplied_accounts: u8,
    owner: Pubkey,
}
#[account]
pub struct PoolProfile {
    total_staked_amount: f64,
    total_borrowed_amount: f64,
    mint: Pubkey,
}

#[derive(Accounts)]
#[instruction(lp_mint_address:Pubkey)]
pub struct CreateLpTokenBag<'info> {
    // 1. PDA (so pubkey) for the soon-to-be created fino token bag for our program.
    #[account(
        init,
        payer = payer,
        // We use the token mint as a seed for the mapping -> think "HashMap[seeds+bump] = pda"
        seeds = [ lp_mint_address.as_ref() ],
        bump,
        // Token Program wants to know what kind of token this token bag is for
        token::mint = lp_mint,
        // It's a PDA so the authority is itself!
        token::authority = program_lp_token_bag,
    )]
    pub program_lp_token_bag: Account<'info, TokenAccount>,

    // 2. The mint Fino  because it's needed from above ⬆️ token::mint = ...
    #[account(
        address = lp_mint_address,
    )]
    pub lp_mint: Account<'info, Mint>,

    // 3. The rent payer
    #[account(mut)]
    pub payer: Signer<'info>,

    // 4. Needed from Anchor for the creation of an Associated Token Account
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(lp_mint_address:Pubkey,program_lp_bag_bump: u8)]
pub struct IntializePoolProfile<'info> {
    #[account(
        mut,
        seeds = [lp_mint_address.as_ref()],
        bump = program_lp_bag_bump,
    )]
    pub program_lp_token_bag: Account<'info, TokenAccount>,
    #[account(
        init,
        // State account seed uses the string "state" and the users' key. 
        // Note that we can only have 1 active transaction
        seeds = [b"defiPool".as_ref(),lp_mint_address.as_ref()],
        bump,
        payer = user,
        space = 200 + 16
    )]
    pub pool_profile: Account<'info, PoolProfile>,
    pub token_program: Program<'info, Token>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(lp_mint_address:Pubkey,program_lp_bag_bump: u8)]
pub struct IntializeUserMintProfile<'info> {
    pub token_program: Program<'info, Token>,
    #[account(
        mut,
        seeds = [lp_mint_address.as_ref()],
        bump = program_lp_bag_bump,
    )]
    pub program_lp_token_bag: Account<'info, TokenAccount>,
    #[account(
        init,
        // State account seed uses the string "state" and the users' key. 
        // Note that we can only have 1 active transaction
        seeds = [b"userMintProfile".as_ref(),lp_mint_address.as_ref(),user.key().as_ref()],
        bump,
        payer = user,
        space = size_of::<UserMintProfile>() + 16
    )]
    pub user_mint_profile: Account<'info, UserMintProfile>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IntializeUserProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        // State account seed uses the string "state" and the users' key. 
        // Note that we can only have 1 active transaction
        seeds = [b"userProfile".as_ref(),user.key().as_ref()],
        bump,
        payer = user,
        space = size_of::<UserProfile>() + 16
    )]
    pub user_profile: Account<'info, UserProfile>,
    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(mint_address:Pubkey,program_bag_bump: u8,supply_count:String ,user_profile_bump:u8,user_mint_profile_bump:u8)]
pub struct Supply_<'info> {
    // pub user_mint_token_acc_authority: Signer<'info>,
    #[account(
        init,
        seeds = [b"supply".as_ref(),mint_address.as_ref(),user.key.as_ref(),supply_count.as_ref()],
        bump,
        payer = user,
        space = size_of::<Supply>() + 32
    )]
    pub supply: Account<'info, Supply>,

    #[account(
        mut,
        seeds = [mint_address.as_ref()],
        bump = program_bag_bump,
    )]
    pub program_mint_token_bag: Account<'info, TokenAccount>,
    #[account  ( mut,
        seeds = [b"userProfile".as_ref(),user.key().as_ref()],
        bump=user_profile_bump,
    )]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut,
        seeds = [b"userMintProfile".as_ref(),mint_address.as_ref(),user.key().as_ref()],
        bump=user_mint_profile_bump,
    )]
    pub user_mint_profile: Account<'info, UserMintProfile>,

    #[account(
        address = mint_address,
    )]
    pub mint: Account<'info, Mint>,
    //  mint Token
    //  minting
    #[account(mut)]
    pub user_mint_token_acc: Account<'info, TokenAccount>,
    //  #[account(mut)]
    //  pub user_receipt_bag: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
#[instruction( borrow_mint_: Pubkey,
    borrow_bump:u8,
    borrow_mint_bump: u8,
    user_profile_bump: u8,
    user_mint_profile_bump: u8,
    borrow_count: String)]
pub struct Repay<'info> {
    //todo
    #[account(mut,seeds=[b"borrow".as_ref(),borrow_mint_.as_ref(),user.key.as_ref(),borrow_count.as_ref()],bump=borrow_bump)]
    pub borrow: Account<'info, Borrow>,
    #[account(mut , seeds=[borrow_mint.key().as_ref()],bump=borrow_mint_bump)]
    pub program_borrow_mint_bag: Account<'info, TokenAccount>,
    #[account  ( mut,
        seeds = [b"userProfile".as_ref(),user.key().as_ref()],
        bump=user_profile_bump,
    )]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut,
        seeds = [b"userMintProfile".as_ref(),borrow_mint_.as_ref(),user.key().as_ref()],
        bump=user_mint_profile_bump,
    )]
    pub user_mint_profile: Account<'info, UserMintProfile>,
    #[account(mut)]
    pub user_borrow_bag: Account<'info, TokenAccount>,
    #[account(address=borrow_mint_)]
    pub borrow_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction( borrow_mint_: Pubkey,
    borrow_mint_bump: u8,
    user_profile_bump: u8,
    user_mint_profile_bump: u8)]
pub struct RepayOnce<'info> {
    //todo
   
    #[account(mut , seeds=[borrow_mint.key().as_ref()],bump=borrow_mint_bump)]
    pub program_borrow_mint_bag: Account<'info, TokenAccount>,
    #[account  ( mut,
        seeds = [b"userProfile".as_ref(),user.key().as_ref()],
        bump=user_profile_bump,
    )]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut,
        seeds = [b"userMintProfile".as_ref(),borrow_mint_.as_ref(),user.key().as_ref()],
        bump=user_mint_profile_bump,
    )]
    pub user_mint_profile: Account<'info, UserMintProfile>,
    #[account(mut)]
    pub user_borrow_bag: Account<'info, TokenAccount>,
    #[account(address=borrow_mint_)]
    pub borrow_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
#[instruction(borrow_mint_:Pubkey,borrow_mint_bump:u8,user_profile_bump:u8,user_mint_profile_bump:u8,borrow_count:String,)]
pub struct Borrow_<'info> {
    // Steps
    // Check supplied amount
    // no minting
    #[account(
        mut,
        seeds = [borrow_mint_.key().as_ref()],
        bump=borrow_mint_bump,
    )]
    pub program_borrow_mint_bag: Account<'info, TokenAccount>,
    #[account  ( mut,
        seeds = [b"userProfile".as_ref(),user.key().as_ref()],
        bump=user_profile_bump,
    )]
    pub user_profile: Account<'info, UserProfile>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub user_borrow_bag: Account<'info, TokenAccount>,
    #[account(mut,
        seeds = [b"userMintProfile".as_ref(),borrow_mint_.as_ref(),user.key().as_ref()],
        bump=user_mint_profile_bump,
    )]
    pub user_mint_profile: Account<'info, UserMintProfile>,
    #[account(
        init,
        seeds = [b"borrow".as_ref(),borrow_mint_.as_ref(),user.key.as_ref(),borrow_count.as_ref()],
        bump,
        payer = user,
        space = size_of::<Borrow>() + 16
    )]
    pub borrow: Account<'info, Borrow>,
    #[account(
        address = borrow_mint_,
    )]
    pub borrow_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
#[instruction(mint_address:Pubkey,user_profile_bump:u8,user_mint_profile_bump:u8)]
pub struct Withdraw<'info> {
    // #[account(mut,seeds=[b"supply".as_ref(),mint_address.as_ref(),user.key().as_ref(),supply_count.as_ref()
    // ],bump=supply_bump)]
    // pub supply: Account<'info, Supply>,
    #[account(mut,seeds=[b"userProfile".as_ref(),user.key().as_ref()],bump=user_profile_bump)]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut, seeds=[b"userMintProfile".as_ref(),mint_address.as_ref(),user.key().as_ref()],bump=user_mint_profile_bump)]
    pub user_mint_profile: Account<'info, UserMintProfile>,
    #[account(address= mint_address)]
    pub mint: Account<'info, Mint>,
    // #[account(
    //     mut,
    //     address = receipt_token.parse::<Pubkey>().unwrap(),
    // )]
    // pub recpit_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    #[account(
        mut,
        seeds = [mint.key().as_ref()],
        bump ,
    )]
    pub program_mint_token_bag: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_mint_token_acc: Account<'info, TokenAccount>,
    // #[account(mut)]
    // pub user_receipt_token_acc: Account<'info, TokenAccount>,
    /// CHECK: todo later
    sol_usd_account: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[error_code]
pub enum ErrorCode {
    #[msg("Insufficent Fino Balance")]
    InsufficentBalance,
    #[msg("Insufficent Reward Balance")]
    InsufficentRewardBalance,
    #[msg("Time Locked")]
    TimeLocked,
    #[msg("Mint mismatch")]
    MintMisMatch,
    #[msg("Mint Not Found.Create mint Bag! ")]
    MintNotFound,
    #[msg("Amount exceed supplied amount")]
    AmountExceedSupply,
    #[msg("Amount must be 75% of the total supplied amount")]
    SupplyNotEnough,
    #[msg("Repay loan or increase supply Amount")]
    RepayLoanOrIncreaseSupply,
    #[msg("Low Liquidity")]
    LowLiquidity,
    #[msg("Supply Locked Repay Borrowed Loan")]
    SupplyLocked,
    #[msg("Low Supply balance")]
    LowSupplyBalance,
    #[msg("Invlaid Amount")]
    InvalidAmount,
    #[msg("Low program mint liquidtity")]
    LowProgramLiquidtity,
    #[msg("AlreadyPaid")]
    AlreadyPaid,
    #[msg("Amount greater then borrowed amount ")]
    AmountGreater,
}
