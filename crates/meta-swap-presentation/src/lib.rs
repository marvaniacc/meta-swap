#![forbid(unsafe_code)]
//! Telegram-facing contracts. Rendering remains outside domain and application layers.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MessageKey {
    SwapStarted,
    SwapPendingVerification,
    SwapAmbiguous,
    SwapFinalized,
    InvalidAction,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CallbackAction {
    pub opaque_id: String,
    pub user_id: i64,
    pub aggregate_version: u64,
}

impl CallbackAction {
    /// # Errors
    ///
    /// Returns [`ActionError::InvalidOpaqueId`] unless the callback is a short opaque token.
    pub fn new(
        opaque_id: impl Into<String>,
        user_id: i64,
        aggregate_version: u64,
    ) -> Result<Self, ActionError> {
        let opaque_id = opaque_id.into();
        if opaque_id.is_empty()
            || opaque_id.len() > 64
            || !opaque_id
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
        {
            return Err(ActionError::InvalidOpaqueId);
        }
        Ok(Self {
            opaque_id,
            user_id,
            aggregate_version,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActionError {
    InvalidOpaqueId,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn callback_data_cannot_contain_business_payload() {
        assert!(CallbackAction::new("approve:1000", 1, 1).is_err());
    }
    #[test]
    fn opaque_callback_data_is_accepted() {
        assert!(CallbackAction::new("a0B_12-z", 1, 1).is_ok());
    }
}
