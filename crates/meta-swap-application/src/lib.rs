#![forbid(unsafe_code)]
//! Use-case ports and orchestration types. Implementations belong in infrastructure.

use meta_swap_domain::{SwapIntent, SwapState};

/// Append-only audit information that is written with immutable intent creation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntentAuditEvent {
    pub event_id: String,
    pub actor_id: String,
    pub correlation_id: String,
}

/// A durable notification/integration event emitted after immutable intent creation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntentOutboxEvent {
    pub event_id: String,
    pub correlation_id: String,
    pub delivery_key: String,
}

/// One all-or-nothing request for immutable intent persistence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PersistIntentRequest {
    pub swap_id: String,
    pub expected_swap_version: u64,
    pub idempotency_key: String,
    pub intent: SwapIntent,
    pub audit_event: IntentAuditEvent,
    pub outbox_event: IntentOutboxEvent,
}

/// The durable result of one accepted intent-persistence request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PersistedIntent {
    pub intent_id: String,
    pub swap_version: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntentPersistenceError {
    NotFound,
    VersionConflict,
    DuplicateIdempotencyKey,
    DuplicateIntentId,
    StorageFailure,
}

/// Port for a single PostgreSQL transaction that creates an immutable intent, audit event, and
/// outbox event. Implementations must never persist a subset of the request.
pub trait IntentRepository {
    /// # Errors
    ///
    /// Returns a durable idempotency, uniqueness, concurrency, or storage error.
    async fn persist_intent(
        &mut self,
        request: PersistIntentRequest,
    ) -> Result<PersistedIntent, IntentPersistenceError>;
}

/// The trusted application component requesting a swap state transition.
///
/// This is selected by the use case that handles an authenticated event; it must never be
/// populated directly from Telegram callback data or another untrusted transport payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransitionAuthority {
    User,
    QuoteProvider,
    IntentBuilder,
    WalletObserver,
    ChainObserver,
    Verifier,
    Finalizer,
    ExpiryWorker,
}

impl TransitionAuthority {
    #[must_use]
    pub const fn permits(self, current: SwapState, next: SwapState) -> bool {
        matches!(
            (self, current, next),
            (
                Self::User,
                SwapState::Draft,
                SwapState::AwaitingPair | SwapState::Cancelled
            ) | (
                Self::User,
                SwapState::AwaitingPair,
                SwapState::AwaitingAmount | SwapState::Cancelled
            ) | (
                Self::User,
                SwapState::AwaitingAmount,
                SwapState::Quoting | SwapState::Cancelled
            ) | (
                Self::User,
                SwapState::QuoteAvailable,
                SwapState::AwaitingConfirmation | SwapState::Cancelled
            ) | (
                Self::User,
                SwapState::AwaitingConfirmation,
                SwapState::BuildingIntent | SwapState::Cancelled
            ) | (
                Self::QuoteProvider,
                SwapState::Quoting,
                SwapState::QuoteAvailable | SwapState::Ambiguous
            ) | (
                Self::IntentBuilder,
                SwapState::BuildingIntent,
                SwapState::AwaitingWalletApproval
                    | SwapState::QuoteAvailable
                    | SwapState::Cancelled
            ) | (
                Self::WalletObserver,
                SwapState::AwaitingWalletApproval,
                SwapState::WalletResponseReceived
                    | SwapState::QuoteAvailable
                    | SwapState::Ambiguous
            ) | (
                Self::ChainObserver,
                SwapState::AwaitingWalletApproval,
                SwapState::ChainCandidateObserved | SwapState::Ambiguous
            ) | (
                Self::ChainObserver,
                SwapState::WalletResponseReceived,
                SwapState::ChainCandidateObserved
                    | SwapState::FinancialVerificationPending
                    | SwapState::Ambiguous
            ) | (
                Self::Verifier,
                SwapState::ChainCandidateObserved,
                SwapState::FinancialVerificationPending | SwapState::Ambiguous
            ) | (
                Self::Verifier,
                SwapState::FinancialVerificationPending,
                SwapState::SwapSucceeded | SwapState::SwapFailedOnchain | SwapState::Ambiguous
            ) | (
                Self::Verifier,
                SwapState::Ambiguous,
                SwapState::FinancialVerificationPending
            ) | (
                Self::Finalizer,
                SwapState::SwapSucceeded,
                SwapState::Finalized
            ) | (
                Self::Finalizer,
                SwapState::SwapFailedOnchain,
                SwapState::FinalizedFailed
            ) | (Self::ExpiryWorker, _, SwapState::Cancelled)
        )
    }
}

/// The trusted application component requesting a swap state transition.
///
/// This is selected by the use case that handles an authenticated event; it must never be
/// populated directly from Telegram callback data or another untrusted transport payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransitionAuthority {
    User,
    QuoteProvider,
    IntentBuilder,
    WalletObserver,
    ChainObserver,
    Verifier,
    Finalizer,
    ExpiryWorker,
}

impl TransitionAuthority {
    #[must_use]
    pub const fn permits(self, current: SwapState, next: SwapState) -> bool {
        matches!(
            (self, current, next),
            (
                Self::User,
                SwapState::Draft,
                SwapState::AwaitingPair | SwapState::Cancelled
            ) | (
                Self::User,
                SwapState::AwaitingPair,
                SwapState::AwaitingAmount | SwapState::Cancelled
            ) | (
                Self::User,
                SwapState::AwaitingAmount,
                SwapState::Quoting | SwapState::Cancelled
            ) | (
                Self::User,
                SwapState::QuoteAvailable,
                SwapState::AwaitingConfirmation | SwapState::Cancelled
            ) | (
                Self::User,
                SwapState::AwaitingConfirmation,
                SwapState::BuildingIntent | SwapState::Cancelled
            ) | (
                Self::QuoteProvider,
                SwapState::Quoting,
                SwapState::QuoteAvailable | SwapState::Ambiguous
            ) | (
                Self::IntentBuilder,
                SwapState::BuildingIntent,
                SwapState::AwaitingWalletApproval
                    | SwapState::QuoteAvailable
                    | SwapState::Cancelled
            ) | (
                Self::WalletObserver,
                SwapState::AwaitingWalletApproval,
                SwapState::WalletResponseReceived
                    | SwapState::QuoteAvailable
                    | SwapState::Ambiguous
            ) | (
                Self::ChainObserver,
                SwapState::AwaitingWalletApproval,
                SwapState::ChainCandidateObserved | SwapState::Ambiguous
            ) | (
                Self::ChainObserver,
                SwapState::WalletResponseReceived,
                SwapState::ChainCandidateObserved
                    | SwapState::FinancialVerificationPending
                    | SwapState::Ambiguous
            ) | (
                Self::Verifier,
                SwapState::ChainCandidateObserved,
                SwapState::FinancialVerificationPending | SwapState::Ambiguous
            ) | (
                Self::Verifier,
                SwapState::FinancialVerificationPending,
                SwapState::SwapSucceeded | SwapState::SwapFailedOnchain | SwapState::Ambiguous
            ) | (
                Self::Verifier,
                SwapState::Ambiguous,
                SwapState::FinancialVerificationPending
            ) | (
                Self::Finalizer,
                SwapState::SwapSucceeded,
                SwapState::Finalized
            ) | (
                Self::Finalizer,
                SwapState::SwapFailedOnchain,
                SwapState::FinalizedFailed
            ) | (Self::ExpiryWorker, _, SwapState::Cancelled)
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapRecord {
    pub id: String,
    pub state: SwapState,
    pub version: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepositoryError {
    NotFound,
    VersionConflict,
    DuplicateIdempotencyKey,
}

pub trait SwapRepository {
    /// # Errors
    ///
    /// Returns [`RepositoryError::NotFound`] when the aggregate does not exist.
    fn load(&self, swap_id: &str) -> Result<SwapRecord, RepositoryError>;

    /// # Errors
    ///
    /// Returns a not-found, optimistic-concurrency, or duplicate-idempotency error.
    fn compare_and_set(
        &mut self,
        record: SwapRecord,
        expected_version: u64,
        idempotency_key: &str,
    ) -> Result<SwapRecord, RepositoryError>;
}

/// Advances one persisted aggregate revision. The infrastructure implementation must perform
/// the state update, audit event, and outbox write in the same `PostgreSQL` transaction.
///
/// # Errors
///
/// Returns an error for missing aggregates, stale revisions, duplicate delivery keys, and
/// forbidden state transitions.
pub fn advance_swap(
    repository: &mut impl SwapRepository,
    swap_id: &str,
    expected_version: u64,
    authority: TransitionAuthority,
    next_state: SwapState,
    idempotency_key: &str,
) -> Result<SwapRecord, AdvanceError> {
    let current = repository.load(swap_id).map_err(AdvanceError::Repository)?;
    if current.version != expected_version {
        return Err(AdvanceError::Repository(RepositoryError::VersionConflict));
    }
    if !authority.permits(current.state, next_state) {
        return Err(AdvanceError::UnauthorizedTransition {
            authority,
            from: current.state,
            to: next_state,
        });
    }
    let state = current
        .state
        .transition_to(next_state)
        .map_err(AdvanceError::Transition)?;
    repository
        .compare_and_set(
            SwapRecord {
                id: current.id,
                state,
                version: current.version + 1,
            },
            expected_version,
            idempotency_key,
        )
        .map_err(AdvanceError::Repository)
}

#[derive(Debug)]
pub enum AdvanceError {
    Repository(RepositoryError),
    UnauthorizedTransition {
        authority: TransitionAuthority,
        from: SwapState,
        to: SwapState,
    },
    Transition(meta_swap_domain::TransitionError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};
    struct MemoryRepository {
        rows: HashMap<String, SwapRecord>,
        keys: HashSet<String>,
    }
    impl SwapRepository for MemoryRepository {
        fn load(&self, id: &str) -> Result<SwapRecord, RepositoryError> {
            self.rows.get(id).cloned().ok_or(RepositoryError::NotFound)
        }
        fn compare_and_set(
            &mut self,
            record: SwapRecord,
            version: u64,
            key: &str,
        ) -> Result<SwapRecord, RepositoryError> {
            if !self.keys.insert(key.into()) {
                return Err(RepositoryError::DuplicateIdempotencyKey);
            }
            let current = self.rows.get(&record.id).ok_or(RepositoryError::NotFound)?;
            if current.version != version {
                return Err(RepositoryError::VersionConflict);
            }
            self.rows.insert(record.id.clone(), record.clone());
            Ok(record)
        }
    }
    #[test]
    fn duplicate_delivery_cannot_advance_twice() {
        let mut repository = MemoryRepository {
            rows: HashMap::from([(
                String::from("swap-1"),
                SwapRecord {
                    id: String::from("swap-1"),
                    state: SwapState::Draft,
                    version: 0,
                },
            )]),
            keys: HashSet::new(),
        };
        assert!(
            advance_swap(
                &mut repository,
                "swap-1",
                0,
                TransitionAuthority::User,
                SwapState::AwaitingPair,
                "update-99"
            )
            .is_ok()
        );
        assert!(matches!(
            advance_swap(
                &mut repository,
                "swap-1",
                0,
                TransitionAuthority::User,
                SwapState::AwaitingPair,
                "update-99"
            ),
            Err(AdvanceError::Repository(RepositoryError::VersionConflict))
        ));
    }
    #[test]
    fn only_the_verifier_can_record_a_verified_outcome() {
        assert!(!TransitionAuthority::User.permits(
            SwapState::FinancialVerificationPending,
            SwapState::SwapSucceeded
        ));
        assert!(TransitionAuthority::Verifier.permits(
            SwapState::FinancialVerificationPending,
            SwapState::SwapSucceeded
        ));
        assert!(TransitionAuthority::Verifier.permits(
            SwapState::FinancialVerificationPending,
            SwapState::SwapFailedOnchain
        ));
    }
    #[test]
    fn unauthorized_call_cannot_persist_a_verified_outcome() {
        let mut repository = MemoryRepository {
            rows: HashMap::from([(
                String::from("swap-1"),
                SwapRecord {
                    id: String::from("swap-1"),
                    state: SwapState::FinancialVerificationPending,
                    version: 8,
                },
            )]),
            keys: HashSet::new(),
        };
        assert!(matches!(
            advance_swap(
                &mut repository,
                "swap-1",
                8,
                TransitionAuthority::User,
                SwapState::SwapSucceeded,
                "callback-1"
            ),
            Err(AdvanceError::UnauthorizedTransition {
                authority: TransitionAuthority::User,
                from: SwapState::FinancialVerificationPending,
                to: SwapState::SwapSucceeded,
            })
        ));
        assert_eq!(repository.rows["swap-1"].version, 8);
        assert!(repository.keys.is_empty());
    }
    #[test]
    fn finalization_requires_the_finalizer() {
        assert!(
            !TransitionAuthority::Verifier.permits(SwapState::SwapSucceeded, SwapState::Finalized)
        );
        assert!(
            TransitionAuthority::Finalizer.permits(SwapState::SwapSucceeded, SwapState::Finalized)
        );
    }
}
