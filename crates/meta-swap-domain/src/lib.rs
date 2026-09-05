#![forbid(unsafe_code)]
//! Chain-neutral financial types and swap lifecycle invariants.
//!
//! This crate deliberately has no transport, database, wallet, or provider dependencies.

use core::fmt;
use core::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ChainId(String);

impl ChainId {
    /// # Errors
    ///
    /// Returns [`IdentityError::Empty`] when `value` is blank.
    pub fn new(value: impl Into<String>) -> Result<Self, IdentityError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(IdentityError::Empty);
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct AssetId {
    chain_id: ChainId,
    canonical_address: String,
}

impl AssetId {
    /// # Errors
    ///
    /// Returns [`IdentityError::Empty`] when the canonical address is blank.
    pub fn new(
        chain_id: ChainId,
        canonical_address: impl Into<String>,
    ) -> Result<Self, IdentityError> {
        let canonical_address = canonical_address.into();
        if canonical_address.trim().is_empty() {
            return Err(IdentityError::Empty);
        }
        Ok(Self {
            chain_id,
            canonical_address,
        })
    }

    #[must_use]
    pub fn chain_id(&self) -> &ChainId {
        &self.chain_id
    }

    #[must_use]
    pub fn canonical_address(&self) -> &str {
        &self.canonical_address
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdentityError {
    Empty,
}

impl fmt::Display for IdentityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("identity must not be empty")
    }
}
impl std::error::Error for IdentityError {}

/// A non-negative amount expressed in the atomic units of exactly one asset.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetAmount {
    asset_id: AssetId,
    atomic: u128,
}

impl AssetAmount {
    #[must_use]
    pub const fn new(asset_id: AssetId, atomic: u128) -> Self {
        Self { asset_id, atomic }
    }

    #[must_use]
    pub const fn atomic(&self) -> u128 {
        self.atomic
    }

    #[must_use]
    pub const fn asset_id(&self) -> &AssetId {
        &self.asset_id
    }

    /// # Errors
    ///
    /// Returns an error for different assets or when the result would be negative.
    pub fn checked_sub(&self, other: &Self) -> Result<Self, AmountError> {
        if self.asset_id != other.asset_id {
            return Err(AmountError::AssetMismatch);
        }
        let atomic = self
            .atomic
            .checked_sub(other.atomic)
            .ok_or(AmountError::Underflow)?;
        Ok(Self::new(self.asset_id.clone(), atomic))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AmountError {
    AssetMismatch,
    Underflow,
    InvalidFormat,
}

impl fmt::Display for AmountError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::AssetMismatch => "amounts belong to different assets",
            Self::Underflow => "amount subtraction would become negative",
            Self::InvalidFormat => "amount must be an unsigned integer in atomic units",
        })
    }
}
impl std::error::Error for AmountError {}

impl FromStr for AtomicAmount {
    type Err = AmountError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(AmountError::InvalidFormat);
        }
        value
            .parse::<u128>()
            .map(Self)
            .map_err(|_| AmountError::InvalidFormat)
    }
}

/// Parsing-only wrapper that ensures decimal/floating-point values cannot enter financial logic.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct AtomicAmount(u128);
impl AtomicAmount {
    #[must_use]
    pub const fn get(self) -> u128 {
        self.0
    }
}

/// A versioned basis-points rate used for fees and referral shares.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BasisPoints(u16);

impl BasisPoints {
    pub const MAX: u16 = 10_000;

    /// # Errors
    ///
    /// Returns [`RateError::OutOfRange`] when the rate exceeds 100%.
    pub const fn new(value: u16) -> Result<Self, RateError> {
        if value > Self::MAX {
            return Err(RateError::OutOfRange);
        }
        Ok(Self(value))
    }

    #[must_use]
    pub const fn get(self) -> u16 {
        self.0
    }

    /// Calculates a fee with floor rounding, which is the only rounding rule in this primitive.
    ///
    /// # Errors
    ///
    /// Returns [`RateError::Overflow`] if multiplying the atomic amount would overflow.
    pub fn floor_of(self, amount: u128) -> Result<u128, RateError> {
        amount
            .checked_mul(u128::from(self.0))
            .map(|product| product / u128::from(Self::MAX))
            .ok_or(RateError::Overflow)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RateError {
    OutOfRange,
    Overflow,
}

impl fmt::Display for RateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::OutOfRange => "basis points must not exceed 10,000",
            Self::Overflow => "basis points calculation overflowed",
        })
    }
}
impl std::error::Error for RateError {}

/// A canonical approval request captured before a wallet is asked to sign.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionIntent {
    swap_id: String,
    sequence: u32,
    input: AssetAmount,
    minimum_output: AssetAmount,
    policy_version: String,
    payload_hash: String,
}

impl TransactionIntent {
    /// # Errors
    ///
    /// Returns [`IntentError::Invalid`] if mandatory immutable fields are blank or the asset
    /// identities do not share the same chain.
    pub fn new(
        swap_id: impl Into<String>,
        sequence: u32,
        input: AssetAmount,
        minimum_output: AssetAmount,
        policy_version: impl Into<String>,
        payload_hash: impl Into<String>,
    ) -> Result<Self, IntentError> {
        let swap_id = swap_id.into();
        let policy_version = policy_version.into();
        let payload_hash = payload_hash.into();
        if swap_id.is_empty()
            || policy_version.is_empty()
            || payload_hash.is_empty()
            || input.asset_id().chain_id() != minimum_output.asset_id().chain_id()
        {
            return Err(IntentError::Invalid);
        }
        Ok(Self {
            swap_id,
            sequence,
            input,
            minimum_output,
            policy_version,
            payload_hash,
        })
    }

    #[must_use]
    pub fn payload_hash(&self) -> &str {
        &self.payload_hash
    }

    #[must_use]
    pub fn minimum_output(&self) -> &AssetAmount {
        &self.minimum_output
    }

    #[must_use]
    pub fn swap_id(&self) -> &str {
        &self.swap_id
    }

    #[must_use]
    pub const fn sequence(&self) -> u32 {
        self.sequence
    }

    #[must_use]
    pub fn input(&self) -> &AssetAmount {
        &self.input
    }

    #[must_use]
    pub fn policy_version(&self) -> &str {
        &self.policy_version
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntentError {
    Invalid,
}
impl fmt::Display for IntentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid immutable transaction intent")
    }
}
impl std::error::Error for IntentError {}

/// Finalized chain evidence whose values have been independently checked against an intent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedExecution {
    intent_payload_hash: String,
    output: AssetAmount,
    realized_fee_atomic: u128,
    chain_evidence_id: String,
}

impl VerifiedExecution {
    /// # Errors
    ///
    /// Returns [`VerificationError`] when final chain evidence does not satisfy the immutable
    /// intent. A wallet response or provider response cannot construct this value.
    pub fn verify(
        intent: &TransactionIntent,
        intent_payload_hash: impl Into<String>,
        output: AssetAmount,
        realized_fee_atomic: u128,
        chain_evidence_id: impl Into<String>,
    ) -> Result<Self, VerificationError> {
        let intent_payload_hash = intent_payload_hash.into();
        let chain_evidence_id = chain_evidence_id.into();
        if intent.payload_hash() != intent_payload_hash || chain_evidence_id.is_empty() {
            return Err(VerificationError::EvidenceMismatch);
        }
        if output.asset_id() != intent.minimum_output().asset_id()
            || output.atomic() < intent.minimum_output().atomic()
        {
            return Err(VerificationError::MinimumOutputNotMet);
        }
        Ok(Self {
            intent_payload_hash,
            output,
            realized_fee_atomic,
            chain_evidence_id,
        })
    }

    #[must_use]
    pub const fn realized_fee_atomic(&self) -> u128 {
        self.realized_fee_atomic
    }

    #[must_use]
    pub fn chain_evidence_id(&self) -> &str {
        &self.chain_evidence_id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VerificationError {
    EvidenceMismatch,
    MinimumOutputNotMet,
}
impl fmt::Display for VerificationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("chain evidence does not verify the immutable intent")
    }
}
impl std::error::Error for VerificationError {}

/// Immutable, server-validated referral attribution. It is never taken from callback data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferralRelationship {
    referrer_user_id: String,
    referred_user_id: String,
}

impl ReferralRelationship {
    /// # Errors
    ///
    /// Returns [`ReferralError::InvalidRelationship`] for blank or self-referral identities.
    pub fn new(
        inviter: impl Into<String>,
        newcomer: impl Into<String>,
    ) -> Result<Self, ReferralError> {
        let sponsor_id = inviter.into();
        let invitee_id = newcomer.into();
        if sponsor_id.is_empty() || invitee_id.is_empty() || sponsor_id == invitee_id {
            return Err(ReferralError::InvalidRelationship);
        }
        Ok(Self {
            referrer_user_id: sponsor_id,
            referred_user_id: invitee_id,
        })
    }

    #[must_use]
    pub fn referrer_user_id(&self) -> &str {
        &self.referrer_user_id
    }
    #[must_use]
    pub fn referred_user_id(&self) -> &str {
        &self.referred_user_id
    }
}

/// An append-only reward event derived from a verified realized fee.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferralRewardEvent {
    realized_fee_correlation_id: String,
    relationship: ReferralRelationship,
    amount_atomic: u128,
    share_bps: BasisPoints,
    policy_version: String,
}

impl ReferralRewardEvent {
    /// Creates a pending reward using floor rounding and immutable realized-fee correlation.
    ///
    /// # Errors
    ///
    /// Returns [`ReferralError::InvalidReward`] for blank correlation/policy or zero fee, and
    /// [`ReferralError::Rate`] if the integer calculation overflows.
    pub fn from_verified_fee(
        execution: &VerifiedExecution,
        relationship: ReferralRelationship,
        realized_fee_correlation_id: impl Into<String>,
        share_bps: BasisPoints,
        policy_version: impl Into<String>,
    ) -> Result<Self, ReferralError> {
        let realized_fee_correlation_id = realized_fee_correlation_id.into();
        let policy_version = policy_version.into();
        if realized_fee_correlation_id.is_empty()
            || policy_version.is_empty()
            || execution.realized_fee_atomic() == 0
        {
            return Err(ReferralError::InvalidReward);
        }
        let amount_atomic = share_bps
            .floor_of(execution.realized_fee_atomic())
            .map_err(ReferralError::Rate)?;
        Ok(Self {
            realized_fee_correlation_id,
            relationship,
            amount_atomic,
            share_bps,
            policy_version,
        })
    }

    #[must_use]
    pub const fn amount_atomic(&self) -> u128 {
        self.amount_atomic
    }

    #[must_use]
    pub fn realized_fee_correlation_id(&self) -> &str {
        &self.realized_fee_correlation_id
    }

    #[must_use]
    pub const fn share_bps(&self) -> BasisPoints {
        self.share_bps
    }

    #[must_use]
    pub fn policy_version(&self) -> &str {
        &self.policy_version
    }

    #[must_use]
    pub fn relationship(&self) -> &ReferralRelationship {
        &self.relationship
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferralError {
    InvalidRelationship,
    InvalidReward,
    Rate(RateError),
}
impl fmt::Display for ReferralError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid referral attribution or reward")
    }
}
impl std::error::Error for ReferralError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SwapState {
    Draft,
    AwaitingPair,
    AwaitingAmount,
    Quoting,
    QuoteAvailable,
    AwaitingConfirmation,
    BuildingIntent,
    AwaitingWalletApproval,
    WalletResponseReceived,
    ChainCandidateObserved,
    FinancialVerificationPending,
    SwapSucceeded,
    SwapFailedOnchain,
    Ambiguous,
    Finalized,
    FinalizedFailed,
    Cancelled,
}

impl SwapState {
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Finalized | Self::FinalizedFailed | Self::Cancelled
        )
    }

    /// # Errors
    ///
    /// Returns an error when the state is terminal or the transition is not allowed.
    pub fn transition_to(self, next: Self) -> Result<Self, TransitionError> {
        if self.is_terminal() {
            return Err(TransitionError::Terminal);
        }
        let allowed = matches!(
            (self, next),
            (Self::Draft, Self::AwaitingPair | Self::Cancelled)
                | (Self::AwaitingPair, Self::AwaitingAmount | Self::Cancelled)
                | (Self::AwaitingAmount, Self::Quoting | Self::Cancelled)
                | (
                    Self::Quoting,
                    Self::QuoteAvailable | Self::Ambiguous | Self::Cancelled
                )
                | (
                    Self::QuoteAvailable,
                    Self::AwaitingConfirmation | Self::Cancelled
                )
                | (
                    Self::AwaitingConfirmation,
                    Self::BuildingIntent | Self::Cancelled
                )
                | (
                    Self::BuildingIntent,
                    Self::AwaitingWalletApproval | Self::QuoteAvailable | Self::Cancelled
                )
                | (
                    Self::AwaitingWalletApproval,
                    Self::WalletResponseReceived
                        | Self::ChainCandidateObserved
                        | Self::Ambiguous
                        | Self::QuoteAvailable
                        | Self::Cancelled
                )
                | (
                    Self::WalletResponseReceived,
                    Self::ChainCandidateObserved
                        | Self::FinancialVerificationPending
                        | Self::Ambiguous
                )
                | (
                    Self::ChainCandidateObserved,
                    Self::FinancialVerificationPending | Self::Ambiguous
                )
                | (
                    Self::FinancialVerificationPending,
                    Self::SwapSucceeded | Self::SwapFailedOnchain | Self::Ambiguous
                )
                | (Self::SwapSucceeded, Self::Finalized)
                | (Self::SwapFailedOnchain, Self::FinalizedFailed)
                | (
                    Self::Ambiguous,
                    Self::FinancialVerificationPending | Self::Cancelled
                )
        );
        if allowed {
            Ok(next)
        } else {
            Err(TransitionError::Forbidden {
                from: self,
                to: next,
            })
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransitionError {
    Terminal,
    Forbidden { from: SwapState, to: SwapState },
}
impl fmt::Display for TransitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid swap state transition: {self:?}")
    }
}
impl std::error::Error for TransitionError {}

#[cfg(test)]
mod tests {
    use super::*;
    fn ton() -> AssetId {
        AssetId::new(ChainId::new("ton-mainnet").unwrap(), "native").unwrap()
    }
    #[test]
    fn atomic_parser_rejects_decimal_values() {
        assert!("1.5".parse::<AtomicAmount>().is_err());
    }
    #[test]
    fn amount_cannot_cross_assets() {
        let other = AssetId::new(ChainId::new("ton-mainnet").unwrap(), "jetton:a").unwrap();
        assert_eq!(
            AssetAmount::new(ton(), 2).checked_sub(&AssetAmount::new(other, 1)),
            Err(AmountError::AssetMismatch)
        );
    }
    #[test]
    fn wallet_response_is_not_a_final_state() {
        assert!(!SwapState::WalletResponseReceived.is_terminal());
        assert!(
            SwapState::WalletResponseReceived
                .transition_to(SwapState::Finalized)
                .is_err()
        );
    }
    #[test]
    fn verification_is_required_for_finalization() {
        assert_eq!(
            SwapState::FinancialVerificationPending.transition_to(SwapState::SwapSucceeded),
            Ok(SwapState::SwapSucceeded)
        );
        assert_eq!(
            SwapState::SwapSucceeded.transition_to(SwapState::Finalized),
            Ok(SwapState::Finalized)
        );
    }
    #[test]
    fn fee_rounds_down_using_atomic_integers() {
        assert_eq!(BasisPoints::new(25).unwrap().floor_of(999).unwrap(), 2);
    }
    #[test]
    fn verified_execution_requires_matching_intent_and_minimum_output() {
        let input = AssetAmount::new(ton(), 100);
        let output_asset =
            AssetId::new(ChainId::new("ton-mainnet").unwrap(), "jetton:out").unwrap();
        let intent = TransactionIntent::new(
            "swap-1",
            1,
            input,
            AssetAmount::new(output_asset.clone(), 99),
            "policy-1",
            "hash-1",
        )
        .unwrap();
        assert!(
            VerifiedExecution::verify(
                &intent,
                "hash-1",
                AssetAmount::new(output_asset, 100),
                1,
                "chain-tx-1"
            )
            .is_ok()
        );
        assert!(
            VerifiedExecution::verify(
                &intent,
                "other",
                AssetAmount::new(ton(), 100),
                1,
                "chain-tx-1"
            )
            .is_err()
        );
    }
    #[test]
    fn referral_reward_is_derived_only_from_a_verified_realized_fee() {
        let output_asset =
            AssetId::new(ChainId::new("ton-mainnet").unwrap(), "jetton:out").unwrap();
        let intent = TransactionIntent::new(
            "swap-1",
            1,
            AssetAmount::new(ton(), 100),
            AssetAmount::new(output_asset.clone(), 90),
            "policy-1",
            "hash-1",
        )
        .unwrap();
        let execution = VerifiedExecution::verify(
            &intent,
            "hash-1",
            AssetAmount::new(output_asset, 99),
            25,
            "chain-tx-1",
        )
        .unwrap();
        let relationship = ReferralRelationship::new("referrer", "referred").unwrap();
        let reward = ReferralRewardEvent::from_verified_fee(
            &execution,
            relationship,
            "fee-1",
            BasisPoints::new(2_000).unwrap(),
            "policy-1",
        )
        .unwrap();
        assert_eq!(reward.amount_atomic(), 5);
        assert!(ReferralRelationship::new("same", "same").is_err());
    }
}
