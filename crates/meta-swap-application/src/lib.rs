#![forbid(unsafe_code)]
//! Use-case ports and orchestration types. Implementations belong in infrastructure.

use meta_swap_domain::SwapState;

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
    next_state: SwapState,
    idempotency_key: &str,
) -> Result<SwapRecord, AdvanceError> {
    let current = repository.load(swap_id).map_err(AdvanceError::Repository)?;
    if current.version != expected_version {
        return Err(AdvanceError::Repository(RepositoryError::VersionConflict));
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
                SwapState::AwaitingPair,
                "update-99"
            ),
            Err(AdvanceError::Repository(RepositoryError::VersionConflict))
        ));
    }
}
