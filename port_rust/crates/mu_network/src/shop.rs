use std::net::SocketAddr;
use std::time::Duration;

use mu_protocol::cash_shop::{
    cash_shop_delete_storage_item_request, cash_shop_event_item_list_request,
    cash_shop_item_buy_request, cash_shop_item_gift_request, cash_shop_open_state,
    cash_shop_point_info_request, cash_shop_storage_item_consume_request,
    cash_shop_storage_list_request,
};
use mu_protocol::PacketCodecError;
use thiserror::Error;

use crate::redaction::redact;
use crate::{Client, ClientError};

const COMPONENT_CASH_SHOP: &str = "cash-shop";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CashShopTransactionKind {
    #[default]
    Purchase,
    Gift,
    StorageDelete,
    StorageConsume,
}

impl CashShopTransactionKind {
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
pub enum CashShopTransactionOutcome {
    Success,
    Failure,
    InsufficientFunds,
}

impl CashShopTransactionOutcome {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Failure => "failure",
            Self::InsufficientFunds => "insufficient-funds",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CashShopTransaction {
    pub id: u64,
    pub kind: CashShopTransactionKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CashShopTransactionCompletion {
    pub transaction: CashShopTransaction,
    pub outcome: CashShopTransactionOutcome,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum CashShopTransactionError {
    #[error("cash shop transaction is already pending")]
    AlreadyPending,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CashShopTransactionState {
    next_transaction_id: u64,
    active_transaction: Option<CashShopTransaction>,
    last_completion: Option<CashShopTransactionCompletion>,
}

impl Default for CashShopTransactionState {
    fn default() -> Self {
        Self {
            next_transaction_id: 1,
            active_transaction: None,
            last_completion: None,
        }
    }
}

impl CashShopTransactionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn is_pending(&self) -> bool {
        self.active_transaction.is_some()
    }

    pub fn active_transaction(&self) -> Option<CashShopTransaction> {
        self.active_transaction
    }

    pub fn last_completion(&self) -> Option<CashShopTransactionCompletion> {
        self.last_completion
    }

    pub fn begin_purchase(&mut self) -> Result<CashShopTransaction, CashShopTransactionError> {
        self.begin(CashShopTransactionKind::Purchase)
    }

    pub fn begin_gift(&mut self) -> Result<CashShopTransaction, CashShopTransactionError> {
        self.begin(CashShopTransactionKind::Gift)
    }

    pub fn begin_storage_delete(
        &mut self,
    ) -> Result<CashShopTransaction, CashShopTransactionError> {
        self.begin(CashShopTransactionKind::StorageDelete)
    }

    pub fn begin_storage_consume(
        &mut self,
    ) -> Result<CashShopTransaction, CashShopTransactionError> {
        self.begin(CashShopTransactionKind::StorageConsume)
    }

    pub fn begin(
        &mut self,
        kind: CashShopTransactionKind,
    ) -> Result<CashShopTransaction, CashShopTransactionError> {
        if self.active_transaction.is_some() {
            return Err(CashShopTransactionError::AlreadyPending);
        }

        let transaction = CashShopTransaction {
            id: self.next_transaction_id,
            kind,
        };
        self.next_transaction_id = self.next_transaction_id.saturating_add(1);
        self.active_transaction = Some(transaction);
        Ok(transaction)
    }

    pub fn cancel_active(&mut self) -> Option<CashShopTransaction> {
        self.active_transaction.take()
    }

    pub fn complete_active(
        &mut self,
        outcome: CashShopTransactionOutcome,
    ) -> Option<CashShopTransactionCompletion> {
        let transaction = self.active_transaction.take()?;
        let completion = CashShopTransactionCompletion {
            transaction,
            outcome,
        };
        self.last_completion = Some(completion);
        Some(completion)
    }
}

#[derive(Debug, Error)]
pub enum CashShopClientError {
    #[error(transparent)]
    Transaction(#[from] CashShopTransactionError),
    #[error(transparent)]
    Packet(#[from] PacketCodecError),
    #[error(transparent)]
    Client(#[from] ClientError),
}

#[derive(Debug)]
pub struct CashShopClient {
    client: Client,
    transactions: CashShopTransactionState,
}

impl CashShopClient {
    pub async fn connect(
        endpoint: SocketAddr,
        connect_timeout: Duration,
        read_timeout: Duration,
    ) -> Result<Self, CashShopClientError> {
        let client = Client::connect(endpoint, connect_timeout, read_timeout).await?;
        Ok(Self {
            client,
            transactions: CashShopTransactionState::default(),
        })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn client_mut(&mut self) -> &mut Client {
        &mut self.client
    }

    pub fn transactions(&self) -> &CashShopTransactionState {
        &self.transactions
    }

    pub fn transactions_mut(&mut self) -> &mut CashShopTransactionState {
        &mut self.transactions
    }

    pub fn disconnect(&mut self) {
        self.client.disconnect();
        self.transactions.cancel_active();
    }

    pub async fn reconnect(&mut self) -> Result<(), CashShopClientError> {
        self.client.reconnect().await?;
        self.transactions.cancel_active();
        Ok(())
    }

    pub async fn request_point_info(&mut self) -> Result<(), CashShopClientError> {
        self.client.send(cash_shop_point_info_request()?).await?;
        Ok(())
    }

    pub async fn request_open_state(&mut self, is_closed: bool) -> Result<(), CashShopClientError> {
        self.client.send(cash_shop_open_state(is_closed)?).await?;
        Ok(())
    }

    pub async fn request_storage_list(
        &mut self,
        page_index: u32,
        inventory_type: u8,
    ) -> Result<(), CashShopClientError> {
        self.client
            .send(cash_shop_storage_list_request(page_index, inventory_type)?)
            .await?;
        Ok(())
    }

    pub async fn request_event_item_list(
        &mut self,
        category_index: u32,
    ) -> Result<(), CashShopClientError> {
        self.client
            .send(cash_shop_event_item_list_request(category_index)?)
            .await?;
        Ok(())
    }

    pub async fn submit_purchase(
        &mut self,
        package_main_index: u32,
        category: u32,
        product_main_index: u32,
        item_index: u16,
        coin_index: u32,
        mileage_flag: u8,
    ) -> Result<CashShopTransaction, CashShopClientError> {
        let packet = cash_shop_item_buy_request(
            package_main_index,
            category,
            product_main_index,
            item_index,
            coin_index,
            mileage_flag,
        )?;
        self.submit_transaction(
            CashShopTransactionKind::Purchase,
            packet,
            |transaction_id| {
                tracing::info!(
                    component = COMPONENT_CASH_SHOP,
                    action = "purchase-submit",
                    transaction_id = transaction_id,
                    package_main_index,
                    category,
                    product_main_index,
                    item_index,
                    coin_index,
                    mileage_flag
                );
            },
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn submit_gift(
        &mut self,
        package_main_index: u32,
        category: u32,
        product_main_index: u32,
        item_index: u16,
        coin_index: u32,
        mileage_flag: u8,
        gift_receiver_name: impl AsRef<[u8]>,
        gift_text: impl AsRef<[u8]>,
    ) -> Result<CashShopTransaction, CashShopClientError> {
        let packet = cash_shop_item_gift_request(
            package_main_index,
            category,
            product_main_index,
            item_index,
            coin_index,
            mileage_flag,
            gift_receiver_name.as_ref(),
            gift_text.as_ref(),
        )?;
        let receiver = String::from_utf8_lossy(gift_receiver_name.as_ref()).into_owned();
        let message = String::from_utf8_lossy(gift_text.as_ref()).into_owned();

        self.submit_transaction(CashShopTransactionKind::Gift, packet, |transaction_id| {
            tracing::info!(
                component = COMPONENT_CASH_SHOP,
                action = "gift-submit",
                transaction_id = transaction_id,
                package_main_index,
                category,
                product_main_index,
                item_index,
                coin_index,
                mileage_flag,
                receiver = %redact(&receiver),
                message = %redact(&message)
            );
        })
        .await
    }

    pub async fn submit_storage_delete(
        &mut self,
        base_item_code: u32,
        main_item_code: u32,
        product_type: u8,
    ) -> Result<CashShopTransaction, CashShopClientError> {
        let packet =
            cash_shop_delete_storage_item_request(base_item_code, main_item_code, product_type)?;
        self.submit_transaction(
            CashShopTransactionKind::StorageDelete,
            packet,
            |transaction_id| {
                tracing::info!(
                    component = COMPONENT_CASH_SHOP,
                    action = "storage-delete-submit",
                    transaction_id = transaction_id,
                    base_item_code,
                    main_item_code,
                    product_type
                );
            },
        )
        .await
    }

    pub async fn submit_storage_consume(
        &mut self,
        base_item_code: u32,
        main_item_code: u32,
        item_index: u16,
        product_type: u8,
    ) -> Result<CashShopTransaction, CashShopClientError> {
        let packet = cash_shop_storage_item_consume_request(
            base_item_code,
            main_item_code,
            item_index,
            product_type,
        )?;
        self.submit_transaction(
            CashShopTransactionKind::StorageConsume,
            packet,
            |transaction_id| {
                tracing::info!(
                    component = COMPONENT_CASH_SHOP,
                    action = "storage-consume-submit",
                    transaction_id = transaction_id,
                    base_item_code,
                    main_item_code,
                    item_index,
                    product_type
                );
            },
        )
        .await
    }

    pub fn record_result(
        &mut self,
        outcome: CashShopTransactionOutcome,
    ) -> Option<CashShopTransactionCompletion> {
        let completion = self.transactions.complete_active(outcome)?;
        tracing::info!(
            component = COMPONENT_CASH_SHOP,
            action = "transaction-complete",
            transaction_id = completion.transaction.id,
            kind = completion.transaction.kind.as_str(),
            outcome = completion.outcome.as_str()
        );
        Some(completion)
    }

    async fn submit_transaction(
        &mut self,
        kind: CashShopTransactionKind,
        packet: Vec<u8>,
        log_submit: impl FnOnce(u64),
    ) -> Result<CashShopTransaction, CashShopClientError> {
        let transaction = match self.transactions.begin(kind) {
            Ok(transaction) => transaction,
            Err(error) => {
                tracing::warn!(
                    component = COMPONENT_CASH_SHOP,
                    action = "transaction-rejected",
                    kind = kind.as_str(),
                    error = %error
                );
                return Err(error.into());
            }
        };

        log_submit(transaction.id);

        if let Err(error) = self.client.send(packet).await {
            self.transactions.cancel_active();
            tracing::error!(
                component = COMPONENT_CASH_SHOP,
                action = "transaction-send-failed",
                transaction_id = transaction.id,
                kind = kind.as_str(),
                error = %error
            );
            return Err(error.into());
        }

        Ok(transaction)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CashShopClient, CashShopClientError, CashShopTransactionKind, CashShopTransactionOutcome,
        CashShopTransactionState,
    };
    use crate::fake_server::{ConnectionScript, FakeServer, FakeServerScenario};
    use mu_protocol::cash_shop::{
        cash_shop_item_buy_request, cash_shop_item_gift_request, cash_shop_storage_list_request,
    };
    use std::io::{self, Write};
    use std::net::SocketAddr;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use tracing_subscriber::fmt::MakeWriter;

    struct BufferWriter(Arc<Mutex<Vec<u8>>>);

    impl<'a> MakeWriter<'a> for BufferWriter {
        type Writer = BufferSink;

        fn make_writer(&'a self) -> Self::Writer {
            BufferSink(Arc::clone(&self.0))
        }
    }

    struct BufferSink(Arc<Mutex<Vec<u8>>>);

    impl Write for BufferSink {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0
                .lock()
                .expect("log buffer poisoned")
                .extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn capture_logs(f: impl FnOnce()) -> String {
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let subscriber = tracing_subscriber::fmt()
            .with_ansi(false)
            .without_time()
            .with_target(false)
            .compact()
            .with_writer(BufferWriter(Arc::clone(&buffer)))
            .finish();

        tracing::subscriber::with_default(subscriber, f);

        let output = buffer.lock().expect("log buffer poisoned").clone();
        String::from_utf8(output).expect("log output is utf-8")
    }

    #[test]
    fn transaction_state_is_idempotent_for_success_failure_and_insufficient_funds() {
        let mut state = CashShopTransactionState::new();

        let first = state.begin_purchase().unwrap();
        assert_eq!(first.id, 1);
        assert_eq!(first.kind, CashShopTransactionKind::Purchase);
        assert_eq!(
            state
                .complete_active(CashShopTransactionOutcome::Success)
                .unwrap()
                .outcome,
            CashShopTransactionOutcome::Success
        );
        assert!(state
            .complete_active(CashShopTransactionOutcome::Success)
            .is_none());

        let second = state.begin_gift().unwrap();
        assert_eq!(second.id, 2);
        assert_eq!(
            state
                .complete_active(CashShopTransactionOutcome::Failure)
                .unwrap()
                .outcome,
            CashShopTransactionOutcome::Failure
        );
        assert!(state
            .complete_active(CashShopTransactionOutcome::Failure)
            .is_none());

        let third = state.begin_storage_consume().unwrap();
        assert_eq!(third.id, 3);
        assert_eq!(
            state
                .complete_active(CashShopTransactionOutcome::InsufficientFunds)
                .unwrap()
                .outcome,
            CashShopTransactionOutcome::InsufficientFunds
        );
        assert!(state
            .complete_active(CashShopTransactionOutcome::InsufficientFunds)
            .is_none());
        assert!(!state.is_pending());
    }

    #[tokio::test]
    async fn duplicate_purchase_submission_is_rejected_and_only_one_packet_is_sent() {
        let request = cash_shop_item_buy_request(1, 2, 3, 4, 5, 6).unwrap();
        let server = FakeServer::spawn(
            "127.0.0.1:0".parse::<SocketAddr>().unwrap(),
            FakeServerScenario::single(ConnectionScript::new().expect_packet(request).close()),
        )
        .await
        .unwrap();

        let mut client = CashShopClient::connect(
            server.address(),
            Duration::from_millis(250),
            Duration::from_millis(250),
        )
        .await
        .unwrap();

        let transaction = client.submit_purchase(1, 2, 3, 4, 5, 6).await.unwrap();
        assert_eq!(transaction.id, 1);
        assert_eq!(transaction.kind, CashShopTransactionKind::Purchase);
        assert!(matches!(
            client.submit_purchase(1, 2, 3, 4, 5, 6).await,
            Err(CashShopClientError::Transaction(
                super::CashShopTransactionError::AlreadyPending
            ))
        ));

        assert_eq!(
            client
                .record_result(CashShopTransactionOutcome::Success)
                .unwrap()
                .outcome,
            CashShopTransactionOutcome::Success
        );
        assert!(client
            .record_result(CashShopTransactionOutcome::Success)
            .is_none());

        server.finish().await.unwrap();
    }

    #[test]
    fn gift_submission_logs_redacted_receiver_and_message() {
        let request =
            cash_shop_item_gift_request(1, 2, 3, 4, 5, 6, b"Alice", b"keep this private").unwrap();

        let output = capture_logs(|| {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("runtime");

            runtime.block_on(async {
                let server = FakeServer::spawn(
                    "127.0.0.1:0".parse::<SocketAddr>().unwrap(),
                    FakeServerScenario::single(
                        ConnectionScript::new().expect_packet(request).close(),
                    ),
                )
                .await
                .unwrap();

                let mut client = CashShopClient::connect(
                    server.address(),
                    Duration::from_millis(250),
                    Duration::from_millis(250),
                )
                .await
                .unwrap();

                client
                    .submit_gift(1, 2, 3, 4, 5, 6, b"Alice", b"keep this private")
                    .await
                    .unwrap();
                client.record_result(CashShopTransactionOutcome::Failure);

                server.finish().await.unwrap();
            });
        });

        assert!(output.contains("[redacted]"), "{output}");
        assert!(!output.contains("Alice"), "{output}");
        assert!(!output.contains("keep this private"), "{output}");
    }

    #[tokio::test]
    async fn non_transactional_requests_still_proxy_to_the_client() {
        let storage_list = cash_shop_storage_list_request(1, 0).unwrap();
        let server = FakeServer::spawn(
            "127.0.0.1:0".parse::<SocketAddr>().unwrap(),
            FakeServerScenario::single(ConnectionScript::new().expect_packet(storage_list).close()),
        )
        .await
        .unwrap();

        let mut client = CashShopClient::connect(
            server.address(),
            Duration::from_millis(250),
            Duration::from_millis(250),
        )
        .await
        .unwrap();

        client.request_storage_list(1, 0).await.unwrap();
        server.finish().await.unwrap();
    }
}
