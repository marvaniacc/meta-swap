#![forbid(unsafe_code)]
//! Chain-neutral financial types and swap lifecycle invariants.
//!
//! This crate deliberately has no transport, database, wallet, or provider dependencies.

use core::fmt;
use core::str::FromStr;

/// The transaction model used by a chain adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ChainFamily {
    Ton,
    Evm,
    Solana,
    Bitcoin,
}

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

/// A chain network identifier supplied by configuration, never inferred from a wallet response.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct NetworkId(String);

impl NetworkId {
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

/// Stable chain context used to scope assets, intents, and verification evidence.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ChainContext {
    chain_id: ChainId,
    family: ChainFamily,
    network_id: NetworkId,
}

impl ChainContext {
    #[must_use]
    pub const fn new(chain_id: ChainId, family: ChainFamily, network_id: NetworkId) -> Self {
        Self {
            chain_id,
            family,
            network_id,
        }
    }

    #[must_use]
    pub const fn chain_id(&self) -> &ChainId {
        &self.chain_id
    }

    #[must_use]
    pub const fn family(&self) -> ChainFamily {
        self.family
    }

    #[must_use]
    pub const fn network_id(&self) -> &NetworkId {
        &self.network_id
    }
}

/// Whether an observed execution has reached the adapter-defined finality threshold.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FinalityLevel {
    Observed,
    Finalized,
}

impl FinalityLevel {
    #[must_use]
    pub const fn is_finalized(self) -> bool {
        matches!(self, Self::Finalized)
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

/// A versioned policy snapshot that is bound to an immutable business intent.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct PolicyVersion(String);

impl PolicyVersion {
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

/// Immutable business terms that must be matched by later transaction compilation and verification.
///
/// This contains no wallet signing material or adapter-specific payload. Those remain outside the
/// domain boundary and must be correlated to this intent by a stable identifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapIntent {
    id: String,
    chain_context: ChainContext,
    input: AssetAmount,
    output_asset: AssetId,
    minimum_received: AssetAmount,
    policy_version: PolicyVersion,
}

impl SwapIntent {
    /// # Errors
    ///
    /// Returns an error for a blank identifier, a cross-chain asset, mismatched output minimum,
    /// identical input/output assets, or a zero input amount.
    pub fn new(
        id: impl Into<String>,
        chain_context: ChainContext,
        input: AssetAmount,
        output_asset: AssetId,
        minimum_received: AssetAmount,
        policy_version: PolicyVersion,
    ) -> Result<Self, IntentError> {
        let id = id.into();
        if id.trim().is_empty() {
            return Err(IntentError::EmptyId);
        }
        if input.atomic() == 0 {
            return Err(IntentError::ZeroInputAmount);
        }
        if input.asset_id().chain_id() != chain_context.chain_id()
            || output_asset.chain_id() != chain_context.chain_id()
        {
            return Err(IntentError::ChainMismatch);
        }
        if minimum_received.asset_id() != &output_asset {
            return Err(IntentError::MinimumOutputAssetMismatch);
        }
        if input.asset_id() == &output_asset {
            return Err(IntentError::IdenticalAssets);
        }
        Ok(Self {
            id,
            chain_context,
            input,
            output_asset,
            minimum_received,
            policy_version,
        })
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub const fn chain_context(&self) -> &ChainContext {
        &self.chain_context
    }

    #[must_use]
    pub const fn input(&self) -> &AssetAmount {
        &self.input
    }

    #[must_use]
    pub const fn output_asset(&self) -> &AssetId {
        &self.output_asset
    }

    #[must_use]
    pub const fn minimum_received(&self) -> &AssetAmount {
        &self.minimum_received
    }

    #[must_use]
    pub const fn policy_version(&self) -> &PolicyVersion {
        &self.policy_version
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntentError {
    EmptyId,
    ZeroInputAmount,
    ChainMismatch,
    MinimumOutputAssetMismatch,
    IdenticalAssets,
}

impl fmt::Display for IntentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::EmptyId => "intent identifier must not be empty",
            Self::ZeroInputAmount => "intent input amount must be greater than zero",
            Self::ChainMismatch => "intent assets must belong to its chain context",
            Self::MinimumOutputAssetMismatch => {
                "minimum output must use the requested output asset"
            }
            Self::IdenticalAssets => "intent input and output assets must differ",
        })
    }
}

impl std::error::Error for IntentError {}

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
    fn ton_context() -> ChainContext {
        ChainContext::new(
            ChainId::new("ton-mainnet").unwrap(),
            ChainFamily::Ton,
            NetworkId::new("mainnet").unwrap(),
        )
    }
    #[test]
    fn chain_context_keeps_family_and_network_explicit() {
        let context = ChainContext::new(
            ChainId::new("ton").unwrap(),
            ChainFamily::Ton,
            NetworkId::new("mainnet").unwrap(),
        );
        assert_eq!(context.chain_id().as_str(), "ton");
        assert_eq!(context.family(), ChainFamily::Ton);
        assert_eq!(context.network_id().as_str(), "mainnet");
        assert!(FinalityLevel::Finalized.is_finalized());
        assert!(!FinalityLevel::Observed.is_finalized());
    }
    #[test]
    fn network_identity_cannot_be_blank() {
        assert_eq!(NetworkId::new(" "), Err(IdentityError::Empty));
    }
    #[test]
    fn intent_binds_assets_and_policy_to_one_chain_context() {
        let input_asset = ton();
        let output_asset =
            AssetId::new(ChainId::new("ton-mainnet").unwrap(), "jetton:output").unwrap();
        let intent = SwapIntent::new(
            "intent-1",
            ton_context(),
            AssetAmount::new(input_asset, 1_000),
            output_asset.clone(),
            AssetAmount::new(output_asset, 900),
            PolicyVersion::new("policy-2026-09-06").unwrap(),
        )
        .unwrap();
        assert_eq!(intent.id(), "intent-1");
        assert_eq!(intent.minimum_received().atomic(), 900);
        assert_eq!(intent.policy_version().as_str(), "policy-2026-09-06");
    }
    #[test]
    fn intent_rejects_a_minimum_output_for_another_asset() {
        let output_asset =
            AssetId::new(ChainId::new("ton-mainnet").unwrap(), "jetton:output").unwrap();
        let other_asset =
            AssetId::new(ChainId::new("ton-mainnet").unwrap(), "jetton:other").unwrap();
        assert_eq!(
            SwapIntent::new(
                "intent-1",
                ton_context(),
                AssetAmount::new(ton(), 1_000),
                output_asset,
                AssetAmount::new(other_asset, 900),
                PolicyVersion::new("policy-1").unwrap(),
            ),
            Err(IntentError::MinimumOutputAssetMismatch)
        );
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
}
