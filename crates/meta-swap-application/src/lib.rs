#![forbid(unsafe_code)]
//! Use-case orchestration and persistence ports. Concrete adapters belong in infrastructure.

use meta_swap_domain::SwapState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapRecord {
    id: String,
    state: SwapState,
    version: u64,
}

impl SwapRecord {
    /// # Errors
    ///
    /// Returns [`RepositoryError::InvalidRecord`] for a blank aggregate identifier.
    pub fn new(
        id: impl Into<String>,
        state: SwapState,
        version: u64,
    ) -> Result<Self, RepositoryError> {
        let id = id.into();
        if id.is_empty() {
            return Err(RepositoryError::InvalidRecord);
        }
        Ok(Self { id, state, version })
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }
    #[must_use]
    pub const fn state(&self) -> SwapState {
        self.state
    }
    #[must_use]
    pub const fn version(&self) -> u64 {
        self.version
    }
}

/// Immutable records that the database adapter must append with the state revision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransitionCommit {
    pub record: SwapRecord,
    pub expected_version: u64,
    pub idempotency_key: String,
    pub audit_event: AuditEvent,
    pub outbox_event: OutboxEvent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditEvent {
    pub aggregate_id: String,
    pub event_type: &'static str,
    pub correlation_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutboxEvent {
    pub aggregate_id: String,
    pub event_type: &'static str,
    pub delivery_key: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepositoryError {
    NotFound,
    VersionConflict,
    DuplicateIdempotencyKey,
    InvalidRecord,
}

/// A production implementation must issue the state compare-and-set, audit append, and outbox
/// insert in one `PostgreSQL` transaction. It must never update a terminal outcome in place.
pub trait SwapRepository {
    /// # Errors
    ///
    /// Returns [`RepositoryError::NotFound`] when the aggregate does not exist.
    fn load(&self, swap_id: &str) -> Result<SwapRecord, RepositoryError>;

    /// # Errors
    ///
    /// Returns a durable idempotency or optimistic-concurrency failure without partially
    /// writing any record in the commit.
    fn persist_transition(
        &mut self,
        commit: TransitionCommit,
    ) -> Result<SwapRecord, RepositoryError>;
}

/// Builds one atomically persisted revision and its audit/outbox side effects.
///
/// # Errors
///
/// Returns an error for missing aggregates, stale revisions, duplicate delivery keys, and
/// forbidden state transitions.
pub fn advance_swap(
    repository: &mut impl SwapRepository,
    swap_id: &str,
    expected_version: u64,
    next_state: SwapState,
    idempotency_key: impl Into<String>,
) -> Result<SwapRecord, AdvanceError> {
    let idempotency_key = idempotency_key.into();
    if idempotency_key.is_empty() {
        return Err(AdvanceError::Repository(RepositoryError::InvalidRecord));
    }
    let current = repository.load(swap_id).map_err(AdvanceError::Repository)?;
    if current.version() != expected_version {
        return Err(AdvanceError::Repository(RepositoryError::VersionConflict));
    }
    let state = current
        .state()
        .transition_to(next_state)
        .map_err(AdvanceError::Transition)?;
    let record = SwapRecord::new(current.id().to_owned(), state, current.version() + 1)
        .map_err(AdvanceError::Repository)?;
    let aggregate_id = record.id().to_owned();
    let commit = TransitionCommit {
        expected_version,
        idempotency_key: idempotency_key.clone(),
        audit_event: AuditEvent {
            aggregate_id: aggregate_id.clone(),
            event_type: "swap_state_transition",
            correlation_id: idempotency_key.clone(),
        },
        outbox_event: OutboxEvent {
            aggregate_id,
            event_type: "swap_state_changed",
            delivery_key: format!("swap-state:{}:{idempotency_key}", record.id()),
        },
        record,
    };
    repository
        .persist_transition(commit)
        .map_err(AdvanceError::Repository)
}

#[derive(Debug)]
pub enum AdvanceError {
    Repository(RepositoryError),
    Transition(meta_swap_domain::TransitionError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};
    struct MemoryRepository {
        rows: HashMap<String, SwapRecord>,
        keys: HashSet<String>,
        commits: Vec<TransitionCommit>,
    }
    impl SwapRepository for MemoryRepository {
        fn load(&self, id: &str) -> Result<SwapRecord, RepositoryError> {
            self.rows.get(id).cloned().ok_or(RepositoryError::NotFound)
        }
        fn persist_transition(
            &mut self,
            commit: TransitionCommit,
        ) -> Result<SwapRecord, RepositoryError> {
            if self.keys.contains(&commit.idempotency_key) {
                return Err(RepositoryError::DuplicateIdempotencyKey);
            }
            let current = self
                .rows
                .get(commit.record.id())
                .ok_or(RepositoryError::NotFound)?;
            if current.version() != commit.expected_version {
                return Err(RepositoryError::VersionConflict);
            }
            if commit.audit_event.aggregate_id != commit.record.id()
                || commit.outbox_event.aggregate_id != commit.record.id()
            {
                return Err(RepositoryError::InvalidRecord);
            }
            self.keys.insert(commit.idempotency_key.clone());
            self.rows
                .insert(commit.record.id().to_owned(), commit.record.clone());
            self.commits.push(commit.clone());
            Ok(commit.record)
        }
    }
    fn repository() -> MemoryRepository {
        MemoryRepository {
            rows: HashMap::from([(
                String::from("swap-1"),
                SwapRecord::new("swap-1", SwapState::Draft, 0).unwrap(),
            )]),
            keys: HashSet::new(),
            commits: Vec::new(),
        }
    }
    #[test]
    fn transition_has_atomic_audit_and_outbox_intent() {
        let mut repository = repository();
        let record = advance_swap(
            &mut repository,
            "swap-1",
            0,
            SwapState::AwaitingPair,
            "update-99",
        )
        .unwrap();
        assert_eq!(record.version(), 1);
        assert_eq!(repository.commits.len(), 1);
        assert_eq!(
            repository.commits[0].audit_event.correlation_id,
            "update-99"
        );
        assert_eq!(
            repository.commits[0].outbox_event.delivery_key,
            "swap-state:swap-1:update-99"
        );
    }
    #[test]
    fn duplicate_delivery_cannot_advance_twice_after_restart() {
        let mut repository = repository();
        advance_swap(
            &mut repository,
            "swap-1",
            0,
            SwapState::AwaitingPair,
            "update-99",
        )
        .unwrap();
        let persisted_rows = repository.rows.clone();
        let persisted_keys = repository.keys.clone();
        let mut restarted = MemoryRepository {
            rows: persisted_rows,
            keys: persisted_keys,
            commits: Vec::new(),
        };
        assert!(matches!(
            advance_swap(
                &mut restarted,
                "swap-1",
                1,
                SwapState::AwaitingAmount,
                "update-99"
            ),
            Err(AdvanceError::Repository(
                RepositoryError::DuplicateIdempotencyKey
            ))
        ));
        assert_eq!(
            restarted.load("swap-1").unwrap().state(),
            SwapState::AwaitingPair
        );
    }
}
