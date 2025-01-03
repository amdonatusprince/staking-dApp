//! SPDX-License-Identifier: MIT

use concordium_std::*; // Import Concordium standard library.
use concordium_cis2::*; // Import Concordium CIS-2 library.

/// The initial value of APR
const INITIAL_APR: u64 = 139;

/// The default denominator of APR
const APR_DENOMINATOR: u128 = 1_000_000_00;

/// The ID of the EUROe token
const TOKEN_ID_EUROE: ContractTokenId = TokenIdUnit();

/// List of entrypoints supported by the `permit` function (CIS3)
const SUPPORTS_PERMIT_ENTRYPOINTS: [EntrypointName; 2] = [
    EntrypointName::new_unchecked("unstake"),
    EntrypointName::new_unchecked("claimRewards"),
];

/// Upgrade parameters
#[derive(Serialize, SchemaType)]
pub struct UpgradeParams {
    /// The new module reference.
    pub module: ModuleReference,

    /// Optional entrypoint to call in the new module after upgrade.
    pub migrate: Option<(OwnedEntrypointName, OwnedParameter)>,
}

/// InitContract parameters
#[derive(Serialize, SchemaType)]
pub struct InitContractParams {
    /// The admin role of concordium liquid staking smart contract.
    pub admin: AccountAddress,

    /// Address of the CIS-2 EUROe token contract.
    pub token_address: ContractAddress,

    /// Unbonding period in seconds
    pub unbonding_period: u64,

    /// Slashing rate in basis points (1% = 100)
    pub slashing_rate: u64,
}

/// Unstake parameters
#[derive(Serialize, SchemaType)]
pub struct UnstakeParams {
    /// The EUROe token amount to unstake
    pub amount: TokenAmountU64,
}

/// Withdraw parameters
#[derive(Serialize, SchemaType)]
pub struct WithdrawEuroEParams {
    /// The address of withdrawable
    withdraw_address: AccountAddress,

    /// The amount to withdraw
    amount: TokenAmountU64,
}

/// Set paused parameters
#[derive(Serialize, SchemaType, Clone)]
#[repr(transparent)]
pub struct SetPausedParams {
    /// Paused state for stopping relevant contract operations.
    pub paused: bool,
}

/// UpdateApr parameters
#[derive(Serialize, SchemaType, Clone)]
pub struct UpdateAprParams {
    /// The new apr value.
    new_apr: u64,
}

/// Part of the parameter type for the contract function `permit`.
/// Specifies the message that is signed.
#[derive(SchemaType, Serialize)]
pub struct PermitMessage {
    /// The contract_address that the signature is intended for.
    pub contract_address: ContractAddress,

    /// A nonce to prevent replay attacks.
    pub nonce: u64,

    /// A timestamp to make signatures expire.
    pub timestamp: Timestamp,

    /// The entry_point that the signature is intended for.
    pub entry_point: OwnedEntrypointName,

    /// The serialized payload.
    #[concordium(size_length = 2)]
    pub payload: Vec<u8>,
}

/// The parameter type for the contract function `permit`.
/// Takes a signature, the signer, and the message that was signed.
#[derive(Serialize, SchemaType)]
pub struct PermitParam {
    /// Signature/s. The CIS3 standard supports multi-sig accounts.
    pub signature: AccountSignatures,

    /// Account that created the above signature.
    pub signer: AccountAddress,

    /// Message that was signed.
    pub message: PermitMessage,
}

#[derive(Serialize)]
pub struct PermitParamPartial {
    /// Signature/s. The CIS3 standard supports multi-sig accounts.
    pub signature: AccountSignatures,

    /// Account that created the above signature.
    pub signer: AccountAddress,
}

/// The parameter type for the contract function `supportsPermit`.
#[derive(Debug, Serialize, SchemaType)]
pub struct SupportsPermitQueryParams {
    /// The list of supportPermit queries.
    #[concordium(size_length = 2)]
    pub queries: Vec<OwnedEntrypointName>,
}

/// View results
#[derive(Serialize, SchemaType)]
pub struct ViewResult {
    /// Paused state for stopping relevant contract operations.
    pub paused: bool,

    /// The admin role of concordium liquid staking smart contract.
    pub admin: AccountAddress,

    /// Total amount of staked tokens.
    pub total_staked: u64,

    /// The Apr.
    pub apr: u64,

    /// Address of the EUROe token contract.
    pub token_address: ContractAddress,

    /// The total number of participants
    pub total_participants: u64,

    /// Track total rewards paid to users
    pub total_rewards_paid: u64,

    /// Track available rewards
    pub rewards_pool: u64,
}

/// Information about a stake.
#[derive(Debug, Serialize, SchemaType, Clone, PartialEq, Eq)]
pub struct StakeInfo {
    /// The staked amount of user.
    pub amount: u64,

    /// Timestamp when the stake was made.
    pub timestamp: u64,

    /// Unbonding information
    pub unbonding: Vec<UnbondingInfo>,

    /// Whether the stake is slashed
    pub slashed: bool,

    /// Pending rewards
    pub pending_rewards: u64,
}

/// Unbonding information
#[derive(Debug, Serialize, SchemaType, Clone, PartialEq, Eq)]
pub struct UnbondingInfo {
    /// Amount to unbond
    pub amount: TokenAmountU64,

    /// Unlock time in seconds
    pub unlock_time: u64,
}

/// State of the contract.
#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
struct State<S = StateApi> {
    /// Paused state for stopping relevant contract operations.
    paused: bool,

    /// The admin role of concordium liquid staking smart contract.
    admin: AccountAddress,

    /// The total amount of staked tokens.
    total_staked: TokenAmountU64,

    /// The annual percentage rate.
    apr: u64,

    /// Mapping of staker addresses to their stake info.
    stakes: StateMap<AccountAddress, StakeInfo, S>,

    /// Address of the EUROe token contract.
    token_address: ContractAddress,

    /// The total number of participants
    total_participants: u64,

    /// A registry to link an account to its next nonce.
    nonces_registry: StateMap<AccountAddress, u64, S>,

    /// Unbonding period in seconds
    unbonding_period: u64,

    /// Slashing rate in basis points (1% = 100)
    slashing_rate: u64,

    /// Track available rewards
    rewards_pool: TokenAmountU64,

    /// Track total rewards paid to users
    total_rewards_paid: TokenAmountU64,
}

/// Implementation of state
impl State {
    /// Get user stake info
    #[allow(dead_code)]
    pub fn get_user_stake(
        &self,
        user: &AccountAddress
    ) -> (TokenAmountU64, u64) {
        self.stakes.get(user).map_or_else(
            || (TokenAmountU64(0), 0),
            |s| (TokenAmountU64(s.amount), s.timestamp)
        )
    }

    /// Get currrent nonce of a user
    pub fn get_user_nonce(&self, user: &AccountAddress) -> u64 {
        self.nonces_registry.get(user).map_or_else(
            || 0,
            |n| n.clone()
        )
    }
}

/// The concordium liquid staking smart contract errors.
#[derive(Debug, PartialEq, Eq, Clone, Reject, Serialize, SchemaType)]
pub enum Error {
    /// Failed Parsing The Parameter.
    #[from(ParseError)]
    ParseParams, // -1

    /// Prevent Unauthorized Access
    UnAuthorized, // -2

    /// Invalid Stake Amount
    InvalidStakeAmount, // -3

    /// No Stake Found
    NoStakeFound, // -4

    /// OnlyAccount
    OnlyAccount, // -5

    /// Only Admin Access
    OnlyAdmin, // -6

    /// Raised when the invocation of the cis2 token contract fails.
    InvokeContractError, //-7

    /// Raised when the parsing of the result from the cis2 token contract
    /// fails.
    ParseResult, //-8

    /// Raised when the response of the cis2 token contract is invalid.
    InvalidResponse, //-9

    /// Failed logging: Log is full.
    LogFull, // -10

    /// Failed logging: Log is malformed.
    LogMalformed, // -11

    /// Upgrade failed because the new module does not exist.
    FailedUpgradeMissingModule, // -12

    /// Upgrade failed because the new module does not contain a contract with a
    /// matching name.
    FailedUpgradeMissingContract, // -13

    /// Upgrade failed because the smart contract version of the module is not
    /// supported.
    FailedUpgradeUnsupportedModuleVersion, // -14

    // Contract is paused.
    ContractPaused, // -15

    /// Insufficient funds
    InsufficientFunds, // -16

    /// Raised when someone else than the cis2 token contract invokes the `stake`
    /// entry point.
    NotTokenContract, //-17

    /// Failed to verify signature because signer account does not exist on
    /// chain.
    MissingAccount, // -18

    /// Failed to verify signature because data was malformed.
    MalformedData, // -19

    /// Failed signature verification: Invalid signature.
    WrongSignature, // -20

    /// Failed signature verification: A different nonce is expected.
    NonceMismatch, // -21

    /// Failed signature verification: Signature was intended for a different
    /// contract.
    WrongContract, // -22

    /// Failed signature verification: Signature was intended for a different
    /// entry_point.
    WrongEntryPoint, // -23

    /// Failed signature verification: Signature is expired.
    Expired, // -24

    /// Invalid unstake amount
    InvalidUnstakeAmount,

    /// Unbonding period not met
    UnbondingPeriodNotMet,

    /// Already slashed
    AlreadySlashed,

    /// Insufficient rewards pool
    InsufficientRewardsPool,

    /// No rewards available to claim
    NoRewardsAvailable,
}

/// Mapping the logging errors to Error.
impl From<LogError> for Error {
    fn from(le: LogError) -> Self {
        match le {
            LogError::Full => Self::LogFull,
            LogError::Malformed => Self::LogMalformed,
        }
    }
}

/// Mapping Cis2ClientError<Error> to Error.
impl From<Cis2ClientError<Error>> for Error {
    fn from(e: Cis2ClientError<Error>) -> Self {
        match e {
            Cis2ClientError::InvokeContractError(_) =>
                Self::InvokeContractError,
            Cis2ClientError::ParseResult => Self::ParseResult,
            Cis2ClientError::InvalidResponse => Self::InvalidResponse,
        }
    }
}

/// Mapping UpgradeError to Error
impl From<UpgradeError> for Error {
    #[inline(always)]
    fn from(ue: UpgradeError) -> Self {
        match ue {
            UpgradeError::MissingModule => Self::FailedUpgradeMissingModule,
            UpgradeError::MissingContract => Self::FailedUpgradeMissingContract,
            UpgradeError::UnsupportedModuleVersion =>
                Self::FailedUpgradeUnsupportedModuleVersion,
        }
    }
}

/// Mapping of errors related to contract invocations to Error.
impl<T> From<CallContractError<T>> for Error {
    fn from(_cce: CallContractError<T>) -> Self {
        Self::InvokeContractError
    }
}

/// Mapping account signature error to CustomContractError
impl From<CheckAccountSignatureError> for Error {
    fn from(e: CheckAccountSignatureError) -> Self {
        match e {
            CheckAccountSignatureError::MissingAccount => Self::MissingAccount,
            CheckAccountSignatureError::MalformedData => Self::MalformedData,
        }
    }
}

/// Enum for different event types in the contract.
#[derive(Debug, Serial, Deserial, PartialEq, Eq, SchemaType)]
#[concordium(repr(u8))]
pub enum Event {
    /// Event for when tokens are staked.
    Staked(StakeEvent),

    /// Event for when tokens are unstaked.
    Unstaked(UnstakeEvent),

    /// Event for when rewards are claimed.
    Claimed(ClaimEvent),

    /// Event for when APR is updated.
    AprUpdated(UpdateAprEvent),
    /// Cis3 event.
    /// The event tracks the nonce used by the signer of the `PermitMessage`
    /// whenever the `permit` function is invoked.
    #[concordium(tag = 250)]
    Nonce(NonceEvent),
}

/// Event structure for staking.
#[derive(Debug, Serialize, SchemaType, PartialEq, Eq)]
pub struct StakeEvent {
    /// Address of the user who staked.
    user: AccountAddress,

    /// Amount of tokens staked.
    stake_amount: TokenAmountU64,

    /// Timestamp when the stake was made.
    staked_timestamp: u64,
}

/// Event structure for unstaking.
#[derive(Debug, Serialize, SchemaType, PartialEq, Eq)]
pub struct UnstakeEvent {
    /// Address of the user who unstaked.
    user: AccountAddress,

    /// Amount of tokens unstaked.
    unstaked_amount: TokenAmountU64,

    /// Timestamp when the unstake was made.
    unix_timestamp: u64,

    /// Rewards earned by the user.
    rewards_earned: TokenAmountU64,
}

/// Event structure for claiming rewards.
#[derive(Debug, Serialize, SchemaType, PartialEq, Eq)]
pub struct ClaimEvent {
    /// Address of the user who claimed rewards.
    user: AccountAddress,

    /// Amount of rewards claimed.
    rewards_claimed: TokenAmountU64,

    /// Timestamp when the claim was made.
    claim_timestamp: u64,
}

/// Event structure for updating APR.
#[derive(Debug, Serialize, SchemaType, PartialEq, Eq)]
pub struct UpdateAprEvent {
    /// New APR value.
    new_apr: u64,

    /// Timestamp when the APR was updated.
    update_timestamp: u64,
}

/// The NonceEvent is logged when the `permit` function is invoked. The event
/// tracks the nonce used by the signer of the `PermitMessage`.
#[derive(Debug, Serialize, SchemaType, PartialEq, Eq)]
pub struct NonceEvent {
    /// The nonce that was used in the `PermitMessage`.
    pub nonce: u64,
    /// Account that signed the `PermitMessage`.
    pub account: AccountAddress,
}

/// Contract token ID type. It has to be the `ContractTokenId` from the cis2
/// token contract.
pub type ContractTokenId = TokenIdUnit;

/// ContractResult type.
pub type ContractResult<A> = Result<A, Error>;

/// Initialization function for the contract.
#[init(contract = "concordium_staking", parameter = "InitContractParams")]
fn contract_init(
    ctx: &InitContext,
    state_builder: &mut StateBuilder
) -> InitResult<State> {
    let params: InitContractParams = ctx.parameter_cursor().get()?;
    let state = State {
        paused: false,
        admin: params.admin,
        total_staked: TokenAmountU64(0),
        total_participants: 0,
        apr: INITIAL_APR,
        stakes: state_builder.new_map(),
        token_address: params.token_address,
        nonces_registry: state_builder.new_map(),
        unbonding_period: params.unbonding_period,
        slashing_rate: params.slashing_rate,
        rewards_pool: TokenAmountU64(0),
        total_rewards_paid: TokenAmountU64(0),
    };

    Ok(state)
}

/// Receive cis-2 token
#[receive(
    contract = "concordium_staking",
    name = "onReceivingCIS2",
    error = "Error"
)]
fn contract_on_cis2_received<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>
) -> ContractResult<()> {
    Ok(())
}

/// Verify an ed25519 signature and allow the unstake, claimRewards.
#[receive(
    contract = "concordium_staking",
    name = "permit",
    parameter = "PermitParam",
    error = "Error",
    crypto_primitives,
    mutable,
    enable_logger
)]
fn contract_permit(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    _logger: &mut Logger,
    crypto_primitives: &impl HasCryptoPrimitives
) -> ContractResult<()> {
    // Check if the contract is paused.
    ensure!(!host.state().paused, Error::ContractPaused);

    // Parse the parameter.
    let param: PermitParam = ctx.parameter_cursor().get()?;

    // Update the nonce.
    let mut entry = host
        .state_mut()
        .nonces_registry.entry(param.signer)
        .or_insert_with(|| 0);

    // Get the current nonce.
    let nonce = *entry;

    // Bump nonce.
    *entry += 1;
    drop(entry);

    let message = param.message;

    ensure_eq!(message.nonce, nonce, Error::NonceMismatch); // Check the nonce to prevent replay attacks.

    ensure_eq!(
        message.contract_address,
        ctx.self_address(),
        Error::WrongContract
    ); // Check that the signature was intended for this contract.

    ensure!(message.timestamp > ctx.metadata().slot_time(), Error::Expired); // Check signature is not expired.

    let message_hash = contract_view_message_hash(
        ctx,
        host,
        crypto_primitives
    )?;

    let valid_signature = host.check_account_signature(
        param.signer,
        &param.signature,
        &message_hash
    )?; // Check signature.

    ensure!(valid_signature, Error::WrongSignature);

    if
        message.entry_point.as_entrypoint_name() ==
        EntrypointName::new_unchecked("unstake")
    {
        let payload: UnstakeParams = from_bytes(&message.payload)?;
        unstake_helper(ctx, host, _logger, param.signer, payload.amount)?;
    } else if
        // claim
        message.entry_point.as_entrypoint_name() ==
        EntrypointName::new_unchecked("claimRewards")
    {
        claim_rewards_helper(ctx, host, _logger, param.signer)?;
    } else {
        // no entrypoint
        bail!(Error::WrongEntryPoint);
    }

    // Log the nonce event.
    _logger.log(
        &Event::Nonce(NonceEvent {
            account: param.signer,
            nonce,
        })
    )?;

    Ok(())
}

/// Function to stake tokens.
#[receive(
    contract = "concordium_staking",
    name = "stake",
    parameter = "OnReceivingCis2DataParams<ContractTokenId, TokenAmountU64,AdditionalData>",
    error = "Error",
    mutable,
    enable_logger
)]
fn contract_stake(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger
) -> ContractResult<()> {
    let state = host.state_mut();
    // Check if sender is the token contract
    if !ctx.sender().matches_contract(&state.token_address) {
        bail!(Error::NotTokenContract);
    }

    let params: OnReceivingCis2DataParams<
        ContractTokenId,
        TokenAmountU64,
        AdditionalData
    > = ctx.parameter_cursor().get()?;

    ensure!(params.token_id == TOKEN_ID_EUROE, Error::InvalidResponse);

    let sender_address = only_account(&params.from)?;
    let unix_timestamp = get_current_timestamp(ctx);
    let amount = params.amount;

    ensure!(!state.paused, Error::ContractPaused);
    ensure!(amount.gt(&TokenAmountU64(0)), Error::InvalidStakeAmount);

    // Get or create stake info
    let is_new_staker = state.stakes.get(&sender_address).is_none();
    let mut sender_stake = state.stakes
        .entry(sender_address)
        .or_insert_with(|| StakeInfo {
            amount: 0,
            timestamp: unix_timestamp,
            unbonding: Vec::new(),
            slashed: false,
            pending_rewards: 0,
        });

    // Calculate pending rewards before updating stake
    if sender_stake.amount > 0 {
        let new_rewards = calculate_reward(
            sender_stake.amount,
            sender_stake.timestamp,
            unix_timestamp,
            state.apr
        );
        sender_stake.pending_rewards = sender_stake.pending_rewards.saturating_add(new_rewards);
    }

    // Update stake amount and timestamp
    sender_stake.amount = sender_stake.amount.saturating_add(amount.0);
    sender_stake.timestamp = unix_timestamp;

    // Update total staked and participants
    state.total_staked = TokenAmountU64(state.total_staked.0.saturating_add(amount.0));
    if is_new_staker {
        state.total_participants = state.total_participants.saturating_add(1);
    }

    logger.log(&Event::Staked(StakeEvent {
        user: sender_address,
        stake_amount: amount,
        staked_timestamp: unix_timestamp,
    }))?;

    Ok(())
}

/// Function to unstake tokens.
#[receive(
    contract = "concordium_staking",
    name = "unstake",
    parameter = "UnstakeParams",
    error = "Error",
    mutable,
    enable_logger
)]
fn contract_unstake(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    _logger: &mut Logger
) -> ContractResult<()> {
    let param: UnstakeParams = ctx.parameter_cursor().get()?;
    let sender_address = only_account(&ctx.sender())?;
    
    let state = host.state_mut();
    ensure!(!state.paused, Error::ContractPaused);

    let mut sender_stake = state.stakes
        .entry(sender_address)
        .occupied_or(Error::NoStakeFound)?;

    ensure!(!sender_stake.slashed, Error::AlreadySlashed);
    ensure!(sender_stake.amount >= param.amount.0, Error::InvalidUnstakeAmount);

    let current_time = get_current_timestamp(ctx);
    let unlock_time = current_time + state.unbonding_period;

    // Add to unbonding list
    sender_stake.unbonding.push(UnbondingInfo {
        amount: param.amount,
        unlock_time,
    });

    // Update stake amount
    sender_stake.amount -= param.amount.0;
    state.total_staked -= param.amount;

    _logger.log(&Event::Unstaked(UnstakeEvent {
        user: sender_address,
        unstaked_amount: param.amount,
        unix_timestamp: current_time,
        rewards_earned: TokenAmountU64(0), // Rewards claimed separately
    }))?;

    Ok(())
}

/// Function to claim rewards.
#[receive(
    contract = "concordium_staking",
    name = "claimRewards",
    error = "Error",
    mutable,
    enable_logger
)]
fn contract_claim_rewards(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    _logger: &mut Logger
) -> ContractResult<()> {
    let sender_address = only_account(&ctx.sender())?;
    claim_rewards_helper(ctx, host, _logger, sender_address)
}

/// Function to withdraw EUROe stablecoin
/// Access by contract owner only.
#[receive(
    contract = "concordium_staking",
    name = "withdrawEuroe",
    parameter = "WithdrawEuroEParams",
    error = "Error",
    mutable
)]
fn contract_withdraw_euroe(
    ctx: &ReceiveContext,
    host: &mut Host<State>
) -> ContractResult<()> {
    let params: WithdrawEuroEParams = ctx.parameter_cursor().get()?;
    let sender = ctx.sender();
    ensure!(sender.matches_account(&ctx.owner()), Error::UnAuthorized); // Access by contract owner only.

    transfer_euroe_token(
        host,
        Address::Contract(ctx.self_address()),
        Receiver::Account(params.withdraw_address),
        params.amount,
        true
    )?; // transfer EUROe token

    Ok(()) // Return success
}

/// Function to pause or unpause the concordium liquid staking contract
/// Access by contract owner only.
#[receive(
    contract = "concordium_staking",
    name = "setPaused",
    parameter = "SetPausedParams",
    error = "Error",
    mutable
)]
fn contract_set_paused(
    ctx: &ReceiveContext,
    host: &mut Host<State>
) -> ContractResult<()> {
    let params: SetPausedParams = ctx.parameter_cursor().get()?;
    let sender = ctx.sender();
    ensure!(sender.matches_account(&ctx.owner()), Error::UnAuthorized);

    let state = host.state_mut();
    state.paused = params.paused;
    Ok(()) // Return success
}

/// Function to update the APR.
/// Access by contract owner only.
#[receive(
    contract = "concordium_staking",
    name = "updateApr",
    parameter = "UpdateAprParams",
    error = "Error",
    mutable,
    enable_logger
)]
fn update_apr(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    _logger: &mut Logger
) -> ContractResult<()> {
    let params: UpdateAprParams = ctx.parameter_cursor().get()?; // Get request parameters.
    let sender = ctx.sender(); // Get the sender's address.

    let update_timestamp = get_current_timestamp(ctx); // Get the current timestamp.
    ensure!(sender.matches_account(&ctx.owner()), Error::UnAuthorized); // Ensure only the contract owner can update the APR
    let state = host.state_mut(); // Get the contract state.

    state.apr = params.new_apr; // Update the APR.
    _logger.log(
        &Event::AprUpdated(UpdateAprEvent {
            new_apr: params.new_apr,
            update_timestamp,
        })
    )?; // Log APR update event.

    Ok(()) // Return success
}

/// Upgrade this smart contract instance to a new module and call optionally a
/// migration function after the upgrade.
///
/// It rejects if:
/// - Sender is not the admin of the contract instance.
/// - It fails to parse the parameter.
/// - If the ugrade fails.
/// - If the migration invoke fails.
///
/// This function is marked as `low_level`. This is **necessary** since the
/// high-level mutable functions store the state of the contract at the end of
/// execution. This conflicts with migration since the shape of the state
/// **might** be changed by the migration function. If the state is then written
/// by this function it would overwrite the state stored by the migration
/// function.
#[receive(
    contract = "concordium_staking",
    name = "upgrade",
    parameter = "UpgradeParams",
    error = "Error",
    low_level
)]
fn contract_upgrade(
    ctx: &ReceiveContext,
    host: &mut LowLevelHost
) -> ContractResult<()> {
    let state: State = host.state().read_root()?; // Read the top-level contract state.
    ensure!(ctx.sender().matches_account(&state.admin), Error::OnlyAdmin); // Check that only the admin is authorized to upgrade the smart contract.
    let params: UpgradeParams = ctx.parameter_cursor().get()?; // Parse the parameter.

    host.upgrade(params.module)?; // Trigger the upgrade.
    if let Some((func, parameters)) = params.migrate {
        host.invoke_contract_raw(
            &ctx.self_address(),
            parameters.as_parameter(),
            func.as_entrypoint_name(),
            Amount::zero()
        )?;
    } // Call the migration function if provided.

    Ok(()) // Return success
}

/// Get current nonce of a user
#[receive(
    contract = "concordium_staking",
    name = "getUserNonce",
    parameter = "AccountAddress",
    error = "Error",
    return_value = "u64"
)]
fn contract_get_user_nonce(
    ctx: &ReceiveContext,
    host: &Host<State>
) -> ContractResult<u64> {
    let user: AccountAddress = ctx.parameter_cursor().get()?;
    let state = host.state();
    Ok(state.get_user_nonce(&user))
}

/// Helper function that can be invoked at the front-end to serialize the
/// `PermitMessage` before signing it in the wallet.
#[receive(
    contract = "concordium_staking",
    name = "serializationHelper",
    parameter = "PermitMessage"
)]
fn contract_serialization_helper(
    _ctx: &ReceiveContext,
    _host: &Host<State>
) -> ContractResult<()> {
    Ok(())
}

/// Calculates the message hash
/// The contract can only be called by any account
/// Returns message hash
///
/// It rejects if:
/// - It fails to parse the parameter
#[receive(
    contract = "concordium_staking",
    name = "viewMessageHash",
    parameter = "PermitParam",
    return_value = "[u8;32]",
    crypto_primitives
)]
fn contract_view_message_hash<S: HasStateApi>(
    ctx: &ReceiveContext,
    _host: &impl HasHost<State<S>, StateApiType = S>,
    crypto_primitives: &impl HasCryptoPrimitives
) -> ContractResult<[u8; 32]> {
    // Parse the parameter.
    let mut cursor = ctx.parameter_cursor();
    // The input parameter is `PermitParam` but we only read the initial part of it
    // with `PermitParamPartial`. I.e. we read the `signature` and the
    // `signer`, but not the `message` here.
    let param: PermitParamPartial = cursor.get()?;

    // The input parameter is `PermitParam` but we have only read the initial part
    // of it with `PermitParamPartial` so far. We read in the `message` now.
    // `(cursor.size() - cursor.cursor_position()` is the length of the message in
    // bytes.
    let mut message_bytes =
        vec![0; (cursor.size() - cursor.cursor_position()) as usize];

    cursor.read_exact(&mut message_bytes)?;

    // The message signed in the Concordium browser wallet is prepended with the
    // `account` address and 8 zero bytes. Accounts in the Concordium browser wallet
    // can either sign a regular transaction (in that case the prepend is
    // `account` address and the nonce of the account which is by design >= 1)
    // or sign a message (in that case the prepend is `account` address and 8 zero
    // bytes). Hence, the 8 zero bytes ensure that the user does not accidentally
    // sign a transaction. The account nonce is of type u64 (8 bytes).
    let mut msg_prepend = vec![0; 32 + 8];

    // Prepend the `account` address of the signer.
    msg_prepend[0..32].copy_from_slice(param.signer.as_ref());

    // Prepend 8 zero bytes.
    msg_prepend[32..40].copy_from_slice(&[0u8; 8]);

    // Calculate the message hash.
    let message_hash = crypto_primitives.hash_sha2_256(
        &[&msg_prepend[0..40], &message_bytes].concat()
    ).0;

    Ok(message_hash)
}

/// Get the entrypoints supported by the `permit` function given a
/// list of entrypoints.
///
/// It rejects if:
/// - It fails to parse the parameter.
#[receive(
    contract = "concordium_staking",
    name = "supportsPermit",
    parameter = "SupportsPermitQueryParams",
    return_value = "SupportsQueryResponse",
    error = "Error"
)]
fn contract_supports_permit<S: HasStateApi>(
    ctx: &ReceiveContext,
    _host: &impl HasHost<State<S>, StateApiType = S>
) -> ContractResult<SupportsQueryResponse> {
    // Parse the parameter.
    let params: SupportsPermitQueryParams = ctx.parameter_cursor().get()?;

    // Build the response.
    let mut response = Vec::with_capacity(params.queries.len());
    for entrypoint in params.queries {
        if
            SUPPORTS_PERMIT_ENTRYPOINTS.contains(
                &entrypoint.as_entrypoint_name()
            )
        {
            response.push(SupportResult::Support);
        } else {
            response.push(SupportResult::NoSupport);
        }
    }
    let result = SupportsQueryResponse::from(response);
    Ok(result)
}

/// View function to get contract state
#[receive(
    contract = "concordium_staking",
    name = "view",
    return_value = "ViewResult"
)]
fn contract_view(
    _ctx: &ReceiveContext,
    host: &Host<State>
) -> ContractResult<ViewResult> {
    let state = host.state();
    
    Ok(ViewResult {
        paused: state.paused,
        admin: state.admin,
        total_staked: state.total_staked.0,
        apr: state.apr,
        token_address: state.token_address,
        total_participants: state.total_participants,
        total_rewards_paid: state.total_rewards_paid.0,
        rewards_pool: state.rewards_pool.0,
    })
}

/// Function to retrieve specific user stake
#[receive(
    contract = "concordium_staking",
    name = "getStakeInfo",
    parameter = "AccountAddress",
    return_value = "StakeInfo",
    error = "Error"
)]
fn contract_get_stake_info(
    ctx: &ReceiveContext,
    host: &Host<State>
) -> ContractResult<StakeInfo> {
    let user: AccountAddress = ctx.parameter_cursor().get()?;
    let state = host.state();
    
    // Return default StakeInfo if no stake exists
    let stake_info = state.stakes.get(&user).map(|s| {
        let current_time = get_current_timestamp(ctx);
        
        // Calculate new rewards since last update
        let additional_rewards = calculate_reward(
            s.amount,
            s.timestamp,
            current_time,
            state.apr
        );

        // Add new rewards to existing pending rewards
        let total_pending_rewards = s.pending_rewards.saturating_add(additional_rewards);

        StakeInfo {
            amount: s.amount,
            timestamp: s.timestamp,
            unbonding: s.unbonding.clone(),
            slashed: s.slashed,
            pending_rewards: total_pending_rewards,  // Use total rewards including new calculations
        }
    }).unwrap_or(StakeInfo {
        amount: 0,
        timestamp: get_current_timestamp(ctx),
        unbonding: Vec::new(),
        slashed: false,
        pending_rewards: 0,
    });
    
    Ok(stake_info)
}

/// Function to get earned rewards.
#[receive(
    contract = "concordium_staking",
    name = "getEarnedRewards",
    parameter = "AccountAddress",
    return_value = "u64",
    error = "Error"
)]
fn get_earned_rewards(
    ctx: &ReceiveContext,
    host: &Host<State>
) -> ContractResult<u64> {
    let user: AccountAddress = ctx.parameter_cursor().get()?;
    let unix_timestamp = get_current_timestamp(ctx);
    let state = host.state();

    // Return 0 if no stake exists or if stake is slashed
    let earned_rewards = state.stakes.get(&user).map_or(0, |stake_info| {
        if stake_info.slashed {
            0
        } else {
            calculate_reward(
                stake_info.amount,
                stake_info.timestamp,
                unix_timestamp,
                state.apr
            )
        }
    });

    Ok(earned_rewards)
}

//  ## HELPER FUNCTIONS ##

fn unstake_helper(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    _logger: &mut Logger,
    sender_address: AccountAddress,
    amount: TokenAmountU64
) -> ContractResult<()> {
    let unix_timestamp = get_current_timestamp(ctx);

    let earned_rewards = {
        let state = host.state_mut();  // Get mutable state
        ensure!(!state.paused, Error::ContractPaused);
    
        let sender_stake = state.stakes.get(&sender_address).ok_or(Error::NoStakeFound)?;
        let staked_amount = sender_stake.amount;
        ensure!(staked_amount >= amount.0, Error::InvalidUnstakeAmount);
    
        let earned_rewards = TokenAmountU64(
            calculate_reward(
                amount.0,
                sender_stake.timestamp,
                unix_timestamp,
                state.apr
            ).into()
        );
    
        // Remove entry if fully unstaking
        if amount.eq(&TokenAmountU64(staked_amount)) {
            state.stakes.remove(&sender_address);
            state.total_participants -= 1;
        } else {
            // Otherwise just update the amount
            let _ = state.stakes.insert(sender_address, StakeInfo {
                amount: staked_amount - amount.0,
                timestamp: sender_stake.timestamp,
                unbonding: sender_stake.unbonding.clone(),
                slashed: sender_stake.slashed,
                pending_rewards: sender_stake.pending_rewards,
            });
        }
    
        state.total_staked -= amount;
        earned_rewards
    }; // state borrow ends here

    transfer_euroe_token(
        host,
        Address::Contract(ctx.self_address()),
        Receiver::Account(sender_address),
        amount + earned_rewards,
        true
    )?;

    _logger.log(
        &Event::Unstaked(UnstakeEvent {
            user: sender_address,
            unstaked_amount: amount,
            unix_timestamp,
            rewards_earned: earned_rewards.into(),
        })
    )?;

    Ok(())
}

fn claim_rewards_helper(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
    sender_address: AccountAddress
) -> ContractResult<()> {
    // Calculate rewards and update state
    let earned_rewards = {
        let state = host.state_mut();
        ensure!(!state.paused, Error::ContractPaused);

        let mut sender_stake = state.stakes
            .entry(sender_address)
            .occupied_or(Error::NoStakeFound)?;

        ensure!(!sender_stake.slashed, Error::AlreadySlashed);

        // Calculate new rewards
        let current_time = get_current_timestamp(ctx);
        let new_rewards = calculate_reward(
            sender_stake.amount,
            sender_stake.timestamp,
            current_time,
            state.apr
        );

        // Get total rewards (pending + new)
        let total_rewards = TokenAmountU64(sender_stake.pending_rewards.saturating_add(new_rewards));
        ensure!(total_rewards.0 > 0, Error::NoRewardsAvailable);
        ensure!(state.rewards_pool.0 >= total_rewards.0, Error::InsufficientRewardsPool);

        // Reset pending rewards and update timestamp
        sender_stake.pending_rewards = 0;
        sender_stake.timestamp = current_time;
        
        // Update contract state
        state.rewards_pool.0 = state.rewards_pool.0.saturating_sub(total_rewards.0);
        state.total_rewards_paid.0 = state.total_rewards_paid.0.saturating_add(total_rewards.0);
        
        total_rewards
    };

    // Transfer rewards to user
    if earned_rewards.0 > 0 {
        transfer_euroe_token(
            host,
            Address::Contract(ctx.self_address()),
            Receiver::Account(sender_address),
            earned_rewards,
            true
        )?;
    }

    logger.log(&Event::Claimed(ClaimEvent {
        user: sender_address,
        rewards_claimed: earned_rewards,
        claim_timestamp: get_current_timestamp(ctx),
    }))?;

    Ok(())
}

/// Validation function to check only account
fn only_account(sender: &Address) -> ContractResult<AccountAddress> {
    match sender {
        Address::Contract(_) => bail!(Error::OnlyAccount),
        Address::Account(account_address) => Ok(*account_address),
    }
}

/// Function to derive current block timestamp
fn get_current_timestamp(ctx: &ReceiveContext) -> u64 {
    ctx.metadata().block_time().millis / 1000
}

/// Function to calculate rewards.
fn calculate_reward(
    staked_amount: u64,
    last_timestamp: u64,
    current_timestamp: u64,
    apr: u64
) -> u64 {
    if staked_amount == 0 {
        return 0;
    }

    let time_staked = current_timestamp.saturating_sub(last_timestamp);
    
    // Use u128 for intermediate calculations to prevent overflow
    let staked_amount_u128 = staked_amount as u128;
    
    // Calculate reward: (staked_amount * apr * time_staked) / (365 * 24 * 60 * 60 * 10000)
    // The 10000 divisor is because APR is in basis points (1% = 100)
    staked_amount_u128
        .saturating_mul(apr as u128)
        .saturating_mul(time_staked as u128)
        .saturating_div(365 * 24 * 60 * 60 * 10000)
        .try_into()
        .unwrap_or(0)
}

/// Function to transfer EUROe stablecoin.
fn transfer_euroe_token(
    host: &mut Host<State>,
    from: Address,
    to: Receiver,
    amount: TokenAmountU64,
    before_transfer_check: bool
) -> ContractResult<()> {
    let state = host.state();
    let client = Cis2Client::new(state.token_address);

    if before_transfer_check {
        let contract_balance = client.balance_of::<
            State,
            ContractTokenId,
            TokenAmountU64,
            Error
        >(host, TOKEN_ID_EUROE, from)?;
        ensure!(contract_balance.gt(&amount), Error::InsufficientFunds);
    }

    client.transfer::<State, ContractTokenId, TokenAmountU64, Error>(
        host,
        Transfer {
            amount,
            from,
            to,
            token_id: TOKEN_ID_EUROE,
            data: AdditionalData::empty(),
        }
    )?;

    Ok(())
}

/// New function to fund rewards pool
#[receive(
    contract = "concordium_staking",
    name = "fundRewards",
    parameter = "TokenAmountU64",
    error = "Error",
    mutable
)]
fn contract_fund_rewards(
    ctx: &ReceiveContext,
    host: &mut Host<State>
) -> ContractResult<()> {
    // Get admin address first
    let admin = host.state().admin;
    ensure!(ctx.sender().matches_account(&admin), Error::OnlyAdmin);
    
    let amount: TokenAmountU64 = ctx.parameter_cursor().get()?;
    
    // Transfer EUROe from admin to contract
    transfer_euroe_token(
        host,
        Address::Account(admin),
        Receiver::Contract(
            ctx.self_address(),
            OwnedEntrypointName::new_unchecked("onReceivingCIS2".to_string())
        ),
        amount,
        true
    )?;
    
    // Update rewards pool after transfer
    host.state_mut().rewards_pool += amount;
    
    Ok(())
}

/// New function to complete unstaking after unbonding period
#[receive(
    contract = "concordium_staking",
    name = "completeUnstake",
    error = "Error",
    mutable,
    enable_logger
)]
fn contract_complete_unstake(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    _logger: &mut Logger
) -> ContractResult<()> {
    let sender_address = only_account(&ctx.sender())?;
    let current_time = get_current_timestamp(ctx);
    
    let state = host.state_mut();
    let mut stake_info = state.stakes
        .entry(sender_address)
        .occupied_or(Error::NoStakeFound)?;

    ensure!(!stake_info.slashed, Error::AlreadySlashed);

    let mut total_amount = TokenAmountU64(0);
    let mut remaining_unbonding = Vec::new();

    // Process unbonding entries
    for unbonding in stake_info.unbonding.iter() {
        if current_time >= unbonding.unlock_time {
            total_amount += unbonding.amount;
        } else {
            remaining_unbonding.push(unbonding.clone());
        }
    }

    ensure!(total_amount.0 > 0, Error::UnbondingPeriodNotMet);

    // Update unbonding list
    stake_info.unbonding = remaining_unbonding;

    // If slashed, apply slashing
    if stake_info.slashed {
        let slash_amount = (total_amount.0 * state.slashing_rate) / 10000;
        total_amount = TokenAmountU64(total_amount.0 - slash_amount);
    }

    // Drop the state borrow before calling transfer_euroe_token
    drop(stake_info);  // Drop any state borrows first

    transfer_euroe_token(
        host,
        Address::Contract(ctx.self_address()),
        Receiver::Account(sender_address),
        total_amount,
        true
    )?;

    Ok(())
}

/// New function to slash a staker
#[receive(
    contract = "concordium_staking",
    name = "slash",
    parameter = "AccountAddress",
    error = "Error",
    mutable
)]
fn contract_slash(
    ctx: &ReceiveContext,
    host: &mut Host<State>
) -> ContractResult<()> {
    let state = host.state_mut();
    ensure!(ctx.sender().matches_account(&state.admin), Error::OnlyAdmin);
    
    let staker: AccountAddress = ctx.parameter_cursor().get()?;
    let mut stake_info = state.stakes
        .entry(staker)
        .occupied_or(Error::NoStakeFound)?;

    ensure!(!stake_info.slashed, Error::AlreadySlashed);

    // Mark as slashed
    stake_info.slashed = true;

    Ok(())
}