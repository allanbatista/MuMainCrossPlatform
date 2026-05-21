use thiserror::Error;

pub const INITIAL_GAME_SHOP_TRANSACTION_ID: u64 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameShopTransactionKind {
    #[default]
    Purchase,
    Gift,
    StorageDelete,
    StorageConsume,
}

impl GameShopTransactionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Purchase => "purchase",
            Self::Gift => "gift",
            Self::StorageDelete => "storage-delete",
            Self::StorageConsume => "storage-consume",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameShopTransactionOutcome {
    Success,
    Failure,
    InsufficientFunds,
}

impl GameShopTransactionOutcome {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Failure => "failure",
            Self::InsufficientFunds => "insufficient-funds",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameShopTransaction {
    pub id: u64,
    pub kind: GameShopTransactionKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameShopTransactionCompletion {
    pub transaction: GameShopTransaction,
    pub outcome: GameShopTransactionOutcome,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum GameShopTransactionError {
    #[error("game shop transaction is already pending")]
    AlreadyPending,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameShopTransactionManager {
    next_transaction_id: u64,
    active_transaction: Option<GameShopTransaction>,
    last_completion: Option<GameShopTransactionCompletion>,
}

impl Default for GameShopTransactionManager {
    fn default() -> Self {
        Self {
            next_transaction_id: INITIAL_GAME_SHOP_TRANSACTION_ID,
            active_transaction: None,
            last_completion: None,
        }
    }
}

impl GameShopTransactionManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn is_pending(&self) -> bool {
        self.active_transaction.is_some()
    }

    pub fn active_transaction(&self) -> Option<GameShopTransaction> {
        self.active_transaction
    }

    pub fn last_completion(&self) -> Option<GameShopTransactionCompletion> {
        self.last_completion
    }

    pub fn begin_purchase(&mut self) -> Result<GameShopTransaction, GameShopTransactionError> {
        self.begin(GameShopTransactionKind::Purchase)
    }

    pub fn begin_gift(&mut self) -> Result<GameShopTransaction, GameShopTransactionError> {
        self.begin(GameShopTransactionKind::Gift)
    }

    pub fn begin_storage_delete(
        &mut self,
    ) -> Result<GameShopTransaction, GameShopTransactionError> {
        self.begin(GameShopTransactionKind::StorageDelete)
    }

    pub fn begin_storage_consume(
        &mut self,
    ) -> Result<GameShopTransaction, GameShopTransactionError> {
        self.begin(GameShopTransactionKind::StorageConsume)
    }

    pub fn begin(
        &mut self,
        kind: GameShopTransactionKind,
    ) -> Result<GameShopTransaction, GameShopTransactionError> {
        if self.active_transaction.is_some() {
            return Err(GameShopTransactionError::AlreadyPending);
        }

        let transaction = GameShopTransaction {
            id: self.next_transaction_id,
            kind,
        };
        self.next_transaction_id = self.next_transaction_id.saturating_add(1);
        self.active_transaction = Some(transaction);
        Ok(transaction)
    }

    pub fn cancel_active(&mut self) -> Option<GameShopTransaction> {
        self.active_transaction.take()
    }

    pub fn complete_active(
        &mut self,
        outcome: GameShopTransactionOutcome,
    ) -> Option<GameShopTransactionCompletion> {
        let transaction = self.active_transaction.take()?;
        let completion = GameShopTransactionCompletion {
            transaction,
            outcome,
        };
        self.last_completion = Some(completion);
        Some(completion)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GameShopTransactionError, GameShopTransactionKind, GameShopTransactionManager,
        GameShopTransactionOutcome,
    };

    #[test]
    fn begins_a_single_active_transaction_at_a_time() {
        let mut manager = GameShopTransactionManager::new();

        let transaction = manager.begin_purchase().unwrap();
        assert_eq!(transaction.id, 1);
        assert_eq!(transaction.kind, GameShopTransactionKind::Purchase);
        assert!(manager.is_pending());

        assert!(matches!(
            manager.begin_gift(),
            Err(GameShopTransactionError::AlreadyPending)
        ));
        assert_eq!(manager.cancel_active(), Some(transaction));
        assert!(!manager.is_pending());
        assert!(manager.last_completion().is_none());
    }

    #[test]
    fn completion_is_idempotent_for_all_outcomes() {
        let mut manager = GameShopTransactionManager::new();

        let first = manager.begin_purchase().unwrap();
        assert_eq!(
            manager
                .complete_active(GameShopTransactionOutcome::Success)
                .unwrap()
                .transaction,
            first
        );
        assert_eq!(
            manager.last_completion().unwrap().outcome,
            GameShopTransactionOutcome::Success
        );
        assert!(manager
            .complete_active(GameShopTransactionOutcome::Success)
            .is_none());

        let second = manager.begin_storage_delete().unwrap();
        assert_eq!(second.id, 2);
        assert_eq!(
            manager
                .complete_active(GameShopTransactionOutcome::Failure)
                .unwrap()
                .outcome,
            GameShopTransactionOutcome::Failure
        );
        assert!(manager
            .complete_active(GameShopTransactionOutcome::Failure)
            .is_none());

        let third = manager.begin_storage_consume().unwrap();
        assert_eq!(third.id, 3);
        assert_eq!(
            manager
                .complete_active(GameShopTransactionOutcome::InsufficientFunds)
                .unwrap()
                .outcome,
            GameShopTransactionOutcome::InsufficientFunds
        );
        assert!(manager
            .complete_active(GameShopTransactionOutcome::InsufficientFunds)
            .is_none());
        assert!(!manager.is_pending());
    }
}
