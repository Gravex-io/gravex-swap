#![allow(unexpected_cfgs)]

/*

on-chain and dev-net
------------------------------------------------------------------------------------------------------------------
GRVTp.myhCiGf2WWF4Gxy3onNfjabqTJ42azm6p8ArokN.json  // on-chain ProgramID   id() 
GDEVp.s2GqJRh88meHWaopWaU6V7f5SgSzN1mZqvJGNMt.json  // devnet   ProgramID   id()
GRVAp.Uwdqcmfb11bzmbo3Q7c5d8Xu7Ftt6RJoRFJgWjy.json  // on-chain ProgramUpdateID ( authortity /  owner )

GRVTa.zQEkM8kMLY8TyHvRstnxbVwixmCHMCuEJd16qyv.json  // on-chain Admin ID    crate::admin::id()
GDEVa.gqMW1LTka5RryB1wtGavSxhUWWErpBaua1QDL5Y.json  // dev-net  Admin ID    crate::admin::id()

GRVTf.7tih2mVFdUwupAfFSbqGsK1YV3N7AQKNmN3AuJe.json  // on-chain FeeID       crate::create_pool_fee_reveiver::id()
GDEVf.Va57vyPiBEqd7YwNuF9bW15VYCapsTQPa4s8AUn.json  // dev-net  FeeID       crate::create_pool_fee_reveiver::id()

GRV Coin Mint  
GRVCtMV5r2YEejEAvShQeWcd9faSDBd2TrvKituMC2Pq.json   // on-chain Token       GRV Token 
GRVAm.UBWjHGmWBuYmr2t7r2Y8CN9ZTW6H2cPhtM3ARyz.json  // on-chain Factory     GRV Mint authority  

Meme Coin Mint

spl-token create-token [FLAGS] [OPTIONS] [TOKEN_KEYFILE]
    FLAGS
    --enable-metadata Enables metadata in the mint. The mint authority must initialize the metadata
    --metadata-address <ADDRESS> Specify address that stores token metadata.
    --decimals <DECIMALS> Number of base 10 digits to the right of the decimal place [default: 9]
    --fee-payer <FEE_PAYER_KEYPAIR> Fee payer for the transaction
    --mint-authority <ADDRESS> Specify the mint authority address. Defaults to the client keypair address.
    --program-id <PROGRAM_ID> Program ID of the token program [default: spl-token]

spl-token initialize-metadata [FLAGS] [OPTIONS] <TOKEN_MINT_ADDRESS> <TOKEN_NAME> <TOKEN_SYMBOL> <TOKEN_URI> 
    --mint-authority <KEYPAIR>      Specify the mint authority keypair. This may be a keypair file or the ASK
    --program-id <PROGRAM_ID>       SPL Token program id
    --update-authority <ADDRESS>    Specify the update authority address. Defaults to the client keypair address.

close-mint --mint-authority <MINT_AUTHORITY_PUBKEY> --target <TOKEN_PUBKEY>

solana --version 
solana-cli 1.17.22 (src:dbf06e25; feat:3580551090, client:SolanaLabs)

solana-install init 1.17.22

spl-token --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb create-token --enable-close --immutable

------------------------------------------------------------------------------------------------------------------


*/

pub mod curve;
pub mod error;
pub mod instructions;
pub mod states;
pub mod utils;

use crate::curve::fees::FEE_RATE_DENOMINATOR_VALUE;
use anchor_lang::prelude::*;
use instructions::*;

#[cfg(not(feature = "no-entrypoint"))]
solana_security_txt::security_txt! {
    name: "gravex-swap",
    project_url: "https://gravex.io",
    contacts: "dev@gravex.io",
    policy: "https://immunefi.com/bounty/gravex",
    source_code: "https://github.com/Gravex-io/gravex-swap",
    preferred_languages: "en",
    auditors: "https://github.com/raydium-io/raydium-docs/blob/master/audit/MadShield%20Q1%202024/raydium-cp-swap-v-1.0.0.pdf"
}

#[cfg(feature = "devnet")]
declare_id!("GDEVps2GqJRh88meHWaopWaU6V7f5SgSzN1mZqvJGNMt");
#[cfg(not(feature = "devnet"))]
declare_id!("GRVTpmyhCiGf2WWF4Gxy3onNfjabqTJ42azm6p8ArokN");

pub mod admin {
    use anchor_lang::prelude::declare_id;
    #[cfg(feature = "devnet")]
    declare_id!("GDEVagqMW1LTka5RryB1wtGavSxhUWWErpBaua1QDL5Y");
    #[cfg(not(feature = "devnet"))]
    declare_id!("GRVTazQEkM8kMLY8TyHvRstnxbVwixmCHMCuEJd16qyv");
}

pub mod create_pool_fee_reveiver {
    use anchor_lang::prelude::declare_id;
    #[cfg(feature = "devnet")]
    declare_id!("GDEVfVa57vyPiBEqd7YwNuF9bW15VYCapsTQPa4s8AUn.json");
    #[cfg(not(feature = "devnet"))]
    declare_id!("GRVTf7tih2mVFdUwupAfFSbqGsK1YV3N7AQKNmN3AuJe");
}
/*
#[cfg(feature = "devnet")]
declare_id!("CPMDWBwJDtYax9qW7AyRuVC19Cc4L4Vcy4n2BHAbHkCW");
#[cfg(not(feature = "devnet"))]
declare_id!("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C");

pub mod admin {
    use anchor_lang::prelude::declare_id;
    #[cfg(feature = "devnet")]
    declare_id!("adMCyoCgfkg7bQiJ9aBJ59H3BXLY3r5LNLfPpQfMzBe");
    #[cfg(not(feature = "devnet"))]
    declare_id!("GThUX1Atko4tqhN2NaiTazWSeFWMuiUvfFnyJyUghFMJ");
}

pub mod create_pool_fee_reveiver {
    use anchor_lang::prelude::declare_id;
    #[cfg(feature = "devnet")]
    declare_id!("G11FKBRaAkHAKuLCgLM6K6NUc9rTjPAznRCjZifrTQe2");
    #[cfg(not(feature = "devnet"))]
    declare_id!("DNXgeM9EiiaAbaWvwjHj9fQQLAX5ZsfHyvmYUNRAdNC8");
}
*/

pub const AUTH_SEED: &str = "vault_and_lp_mint_auth_seed";

#[program]
pub mod raydium_cp_swap {
    use super::*;

    // The configuation of AMM protocol, include trade fee and protocol fee
    /// # Arguments
    ///
    /// * `ctx`- The accounts needed by instruction.
    /// * `index` - The index of amm config, there may be multiple config.
    /// * `trade_fee_rate` - Trade fee rate, can be changed.
    /// * `protocol_fee_rate` - The rate of protocol fee within tarde fee.
    /// * `fund_fee_rate` - The rate of fund fee within tarde fee.
    ///
    pub fn create_amm_config(
        ctx: Context<CreateAmmConfig>,
        index: u16,
        trade_fee_rate: u64,
        protocol_fee_rate: u64,
        fund_fee_rate: u64,
        create_pool_fee: u64,
    ) -> Result<()> {
        assert!(trade_fee_rate < FEE_RATE_DENOMINATOR_VALUE);
        assert!(protocol_fee_rate <= FEE_RATE_DENOMINATOR_VALUE);
        assert!(fund_fee_rate <= FEE_RATE_DENOMINATOR_VALUE);
        assert!(fund_fee_rate + protocol_fee_rate <= FEE_RATE_DENOMINATOR_VALUE);
        instructions::create_amm_config(
            ctx,
            index,
            trade_fee_rate,
            protocol_fee_rate,
            fund_fee_rate,
            create_pool_fee,
        )
    }

    /// Updates the owner of the amm config
    /// Must be called by the current owner or admin
    ///
    /// # Arguments
    ///
    /// * `ctx`- The context of accounts
    /// * `trade_fee_rate`- The new trade fee rate of amm config, be set when `param` is 0
    /// * `protocol_fee_rate`- The new protocol fee rate of amm config, be set when `param` is 1
    /// * `fund_fee_rate`- The new fund fee rate of amm config, be set when `param` is 2
    /// * `new_owner`- The config's new owner, be set when `param` is 3
    /// * `new_fund_owner`- The config's new fund owner, be set when `param` is 4
    /// * `param`- The vaule can be 0 | 1 | 2 | 3 | 4, otherwise will report a error
    ///
    pub fn update_amm_config(ctx: Context<UpdateAmmConfig>, param: u8, value: u64) -> Result<()> {
        instructions::update_amm_config(ctx, param, value)
    }

    /// Update pool status for given vaule
    ///
    /// # Arguments
    ///
    /// * `ctx`- The context of accounts
    /// * `status` - The vaule of status
    ///
    pub fn update_pool_status(ctx: Context<UpdatePoolStatus>, status: u8) -> Result<()> {
        instructions::update_pool_status(ctx, status)
    }

    /// Collect the protocol fee accrued to the pool
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of accounts
    /// * `amount_0_requested` - The maximum amount of token_0 to send, can be 0 to collect fees in only token_1
    /// * `amount_1_requested` - The maximum amount of token_1 to send, can be 0 to collect fees in only token_0
    ///
    pub fn collect_protocol_fee(
        ctx: Context<CollectProtocolFee>,
        amount_0_requested: u64,
        amount_1_requested: u64,
    ) -> Result<()> {
        instructions::collect_protocol_fee(ctx, amount_0_requested, amount_1_requested)
    }

    /// Collect the fund fee accrued to the pool
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of accounts
    /// * `amount_0_requested` - The maximum amount of token_0 to send, can be 0 to collect fees in only token_1
    /// * `amount_1_requested` - The maximum amount of token_1 to send, can be 0 to collect fees in only token_0
    ///
    pub fn collect_fund_fee(
        ctx: Context<CollectFundFee>,
        amount_0_requested: u64,
        amount_1_requested: u64,
    ) -> Result<()> {
        instructions::collect_fund_fee(ctx, amount_0_requested, amount_1_requested)
    }

    /// Creates a pool for the given token pair and the initial price
    ///
    /// # Arguments
    ///
    /// * `ctx`- The context of accounts
    /// * `init_amount_0` - the initial amount_0 to deposit
    /// * `init_amount_1` - the initial amount_1 to deposit
    /// * `open_time` - the timestamp allowed for swap
    ///
    pub fn initialize(
        ctx: Context<Initialize>,
        init_amount_0: u64,
        init_amount_1: u64,
        open_time: u64,
    ) -> Result<()> {
        instructions::initialize(ctx, init_amount_0, init_amount_1, open_time)
    }

    /// Creates a pool for the given token pair and the initial price
    ///
    /// # Arguments
    ///
    /// * `ctx`- The context of accounts
    /// * `lp_token_amount` - Pool token amount to transfer. token_a and token_b amount are set by the current exchange rate and size of the pool
    /// * `maximum_token_0_amount` -  Maximum token 0 amount to deposit, prevents excessive slippage
    /// * `maximum_token_1_amount` - Maximum token 1 amount to deposit, prevents excessive slippage
    ///
    pub fn deposit(
        ctx: Context<Deposit>,
        lp_token_amount: u64,
        maximum_token_0_amount: u64,
        maximum_token_1_amount: u64,
    ) -> Result<()> {
        instructions::deposit(
            ctx,
            lp_token_amount,
            maximum_token_0_amount,
            maximum_token_1_amount,
        )
    }

    /// Withdraw lp for token0 ande token1
    ///
    /// # Arguments
    ///
    /// * `ctx`- The context of accounts
    /// * `lp_token_amount` - Amount of pool tokens to burn. User receives an output of token a and b based on the percentage of the pool tokens that are returned.
    /// * `minimum_token_0_amount` -  Minimum amount of token 0 to receive, prevents excessive slippage
    /// * `minimum_token_1_amount` -  Minimum amount of token 1 to receive, prevents excessive slippage
    ///
    pub fn withdraw(
        ctx: Context<Withdraw>,
        lp_token_amount: u64,
        minimum_token_0_amount: u64,
        minimum_token_1_amount: u64,
    ) -> Result<()> {
        instructions::withdraw(
            ctx,
            lp_token_amount,
            minimum_token_0_amount,
            minimum_token_1_amount,
        )
    }

    /// Swap the tokens in the pool base input amount
    ///
    /// # Arguments
    ///
    /// * `ctx`- The context of accounts
    /// * `amount_in` -  input amount to transfer, output to DESTINATION is based on the exchange rate
    /// * `minimum_amount_out` -  Minimum amount of output token, prevents excessive slippage
    ///
    pub fn swap_base_input(
        ctx: Context<Swap>,
        amount_in: u64,
        minimum_amount_out: u64,
    ) -> Result<()> {
        instructions::swap_base_input(ctx, amount_in, minimum_amount_out)
    }

    /// Swap the tokens in the pool base output amount
    ///
    /// # Arguments
    ///
    /// * `ctx`- The context of accounts
    /// * `max_amount_in` -  input amount prevents excessive slippage
    /// * `amount_out` -  amount of output token
    ///
    pub fn swap_base_output(ctx: Context<Swap>, max_amount_in: u64, amount_out: u64) -> Result<()> {
        instructions::swap_base_output(ctx, max_amount_in, amount_out)
    }
}
