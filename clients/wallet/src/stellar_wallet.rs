use reqwest::Client;
use std::{fmt::Formatter, sync::Arc};

use substrate_stellar_sdk::{
	compound_types::LimitedString,
	network::{Network, PUBLIC_NETWORK, TEST_NETWORK},
	types::{Preconditions, SequenceNumber},
	Asset, Memo, Operation, PublicKey, SecretKey, StroopAmount, Transaction, TransactionEnvelope,
};
use tokio::sync::{oneshot, Mutex};

use crate::{
	cache::WalletStateStorage,
	error::Error,
	horizon::{Balance, HorizonClient, TransactionResponse},
	types::StellarPublicKeyRaw,
};

use crate::{
	error::CacheErrorKind,
	horizon::{TransactionsResponseIter, DEFAULT_PAGE_SIZE},
	types::PagingToken,
};
use primitives::{derive_shortened_request_id, TransactionEnvelopeExt};

#[derive(Clone)]
pub struct StellarWallet {
	secret_key: SecretKey,
	is_public_network: bool,
	/// Used to make sure that only one transaction is submitted at a time,
	/// so that the transaction is not rejected due to an outdated sequence number.
	/// Releasing the lock ensures the sequence number of the account
	/// has been increased on the network.
	transaction_submission_lock: Arc<Mutex<()>>,
	/// Used for caching Stellar transactions before they get submitted.
	/// Also used for caching the latest cursor to page through Stellar transactions in horizon
	cache: WalletStateStorage,

	/// maximum retry attempts for submitting a transaction before switching to a fallback url
	max_retry_attempts_before_fallback: u8,

	/// the waiting time (in seconds) for retrying.
	max_backoff_delay: u16,
}

impl StellarWallet {
	/// if the user doesn't define the maximum number of retry attempts for 500 internal server
	/// error, this will be the default.
	const DEFAULT_MAX_RETRY_ATTEMPTS_BEFORE_FALLBACK: u8 = 3;

	const DEFAULT_MAX_BACKOFF_DELAY_IN_SECS: u16 = 600;

	/// Returns a TransactionResponse after submitting transaction envelope to Stellar,
	/// Else an Error.
	async fn submit_transaction(
		&self,
		envelope: TransactionEnvelope,
		horizon_client: Client,
	) -> Result<TransactionResponse, Error> {
		let sequence = &envelope.sequence_number().ok_or(Error::cache_error_with_env(
			CacheErrorKind::UnknownSequenceNumber,
			envelope.clone(),
		))?;

		let submission_result = horizon_client
			.submit_transaction(
				envelope.clone(),
				self.is_public_network,
				self.max_retry_attempts_before_fallback,
				self.max_backoff_delay,
			)
			.await;

		let _ = self.cache.remove_tx_envelope(*sequence);

		submission_result
	}
}

#[doc(hidden)]
/// Creates and returns a Transaction
fn create_transaction(
	destination_address: PublicKey,
	asset: Asset,
	stroop_amount: i64,
	request_id: [u8; 32],
	stroop_fee_per_operation: u32,
	public_key: PublicKey,
	next_sequence_number: SequenceNumber,
) -> Result<Transaction, Error> {
	let memo_text = Memo::MemoText(
		LimitedString::new(derive_shortened_request_id(&request_id))
			.map_err(|_| Error::BuildTransactionError("Invalid hash".to_string()))?,
	);

	let mut transaction = Transaction::new(
		public_key.clone(),
		next_sequence_number,
		Some(stroop_fee_per_operation),
		Preconditions::PrecondNone,
		Some(memo_text),
	)
	.map_err(|_e| Error::BuildTransactionError("Creating new transaction failed".to_string()))?;

	let amount = StroopAmount(stroop_amount);
	transaction
		.append_operation(
			Operation::new_payment(destination_address, asset, amount)
				.map_err(|_e| {
					Error::BuildTransactionError("Creation of payment operation failed".to_string())
				})?
				.set_source_account(public_key)
				.map_err(|_e| {
					Error::BuildTransactionError("Setting source account failed".to_string())
				})?,
		)
		.map_err(|_e| {
			Error::BuildTransactionError("Appending payment operation failed".to_string())
		})?;

	Ok(transaction)
}

impl StellarWallet {
	pub fn from_secret_encoded(secret_key: &str, is_public_network: bool) -> Result<Self, Error> {
		Self::from_secret_encoded_with_cache(secret_key, is_public_network, "./".to_string())
	}

	/// creates a wallet based on the secret key,
	/// and can specify the path where the cache will be saved.
	pub fn from_secret_encoded_with_cache(
		secret_key: &str,
		is_public_network: bool,
		cache_path: String,
	) -> Result<Self, Error> {
		let secret_key =
			SecretKey::from_encoding(secret_key).map_err(|_| Error::InvalidSecretKey)?;

		Self::from_secret_key_with_cache(secret_key, is_public_network, cache_path)
	}

	pub fn from_secret_key(secret_key: SecretKey, is_public_network: bool) -> Result<Self, Error> {
		Self::from_secret_key_with_cache(secret_key, is_public_network, "./".to_string())
	}

	pub fn from_secret_key_with_cache(
		secret_key: SecretKey,
		is_public_network: bool,
		cache_path: String,
	) -> Result<Self, Error> {
		let pub_key = secret_key.get_public().to_encoding();
		let pub_key = std::str::from_utf8(&pub_key).map_err(|_| Error::InvalidSecretKey)?;

		let cache = WalletStateStorage::new(cache_path, pub_key, is_public_network);

		Ok(StellarWallet {
			secret_key,
			is_public_network,
			transaction_submission_lock: Arc::new(Mutex::new(())),
			cache,
			max_retry_attempts_before_fallback: Self::DEFAULT_MAX_RETRY_ATTEMPTS_BEFORE_FALLBACK,
			max_backoff_delay: Self::DEFAULT_MAX_BACKOFF_DELAY_IN_SECS,
		})
	}

	pub fn with_max_retry_attempts_before_fallback(mut self, max_retries: u8) -> Self {
		self.max_retry_attempts_before_fallback = max_retries;

		self
	}

	pub fn with_max_backoff_delay(mut self, max_backoff_delay_in_secs: u16) -> Self {
		// a number more than the default max would be too large
		if max_backoff_delay_in_secs < Self::DEFAULT_MAX_BACKOFF_DELAY_IN_SECS {
			self.max_backoff_delay = max_backoff_delay_in_secs;
		}

		self
	}

	pub fn get_public_key_raw(&self) -> StellarPublicKeyRaw {
		self.secret_key.get_public().clone().into_binary()
	}

	pub fn get_public_key(&self) -> PublicKey {
		self.secret_key.get_public().clone()
	}

	pub fn get_secret_key(&self) -> SecretKey {
		self.secret_key.clone()
	}

	pub fn is_public_network(&self) -> bool {
		self.is_public_network
	}

	pub fn get_last_cursor(&self) -> PagingToken {
		self.cache.get_last_cursor()
	}

	pub fn save_cursor(&self, paging_token: PagingToken) -> Result<(), Error> {
		self.cache.save_cursor(paging_token)
	}

	/// Returns an iter for all transactions.
	/// This method is looking BACKWARDS, so the transactions are in DESCENDING order:
	/// starting from the LATEST ones, at the time of the call.
	pub async fn get_all_transactions_iter(
		&self,
	) -> Result<TransactionsResponseIter<Client>, Error> {
		let horizon_client = Client::new();

		let public_key_encoded = self.get_public_key().to_encoding();
		let account_id = std::str::from_utf8(&public_key_encoded).map_err(Error::Utf8Error)?;

		let transactions_response = horizon_client
			.get_transactions(account_id, self.is_public_network, 0, DEFAULT_PAGE_SIZE, false)
			.await?;

		let next_page = transactions_response.next_page();
		let records = transactions_response.records();

		Ok(TransactionsResponseIter { records, next_page, client: horizon_client })
	}

	/// Submits transactions found in the wallet's cache to Stellar.
	/// Returns a list of oneshot receivers to send back the result of resubmission.
	pub async fn resubmit_transactions_from_cache(
		&self,
	) -> Vec<oneshot::Receiver<Result<TransactionResponse, Error>>> {
		let _ = self.transaction_submission_lock.lock().await;
		let horizon_client = Client::new();

		// Iterates over all errors and creates channels that are used to send errors back to the
		// caller of this function.
		let mut error_receivers = vec![];

		let mut collect_errors = |errors: Vec<Error>| {
			for error in errors {
				let (sender, receiver) = oneshot::channel();
				error_receivers.push(receiver);

				if let Err(e) = sender.send(Err(error)) {
					tracing::error!("failed to send error to list: {e:?}");
				}
			}
		};

		let envs = match self.cache.get_tx_envelopes() {
			Ok((envs, errors)) => {
				collect_errors(errors);
				envs
			},
			Err(errors) => {
				collect_errors(errors);
				return error_receivers
			},
		};

		let me = Arc::new(self.clone());
		for env in envs.into_iter() {
			let me_clone = Arc::clone(&me);
			let horizon_clone = horizon_client.clone();

			let (sender, receiver) = oneshot::channel();
			error_receivers.push(receiver);

			tokio::spawn(async move {
				if let Err(e) = sender.send(me_clone.submit_transaction(env, horizon_clone).await) {
					tracing::error!("failed to send message: {e:?}");
				};
			});
		}

		error_receivers
	}

	fn create_envelope(
		&self,
		destination_address: PublicKey,
		asset: Asset,
		stroop_amount: i64,
		request_id: [u8; 32],
		stroop_fee_per_operation: u32,
		next_sequence_number: SequenceNumber,
	) -> Result<TransactionEnvelope, Error> {
		let transaction = create_transaction(
			destination_address,
			asset,
			stroop_amount,
			request_id,
			stroop_fee_per_operation,
			self.get_public_key(),
			next_sequence_number,
		)?;

		let mut envelope = transaction.into_transaction_envelope();
		let network: &Network =
			if self.is_public_network { &PUBLIC_NETWORK } else { &TEST_NETWORK };

		envelope
			.sign(network, vec![&self.get_secret_key()])
			.map_err(|_e| Error::SignEnvelopeError)?;

		Ok(envelope)
	}

	pub async fn send_payment_to_address(
		&mut self,
		destination_address: PublicKey,
		asset: Asset,
		stroop_amount: i64,
		request_id: [u8; 32],
		stroop_fee_per_operation: u32,
	) -> Result<TransactionResponse, Error> {
		let _ = self.transaction_submission_lock.lock().await;
		let horizon_client = reqwest::Client::new();

		let public_key_encoded = self.get_public_key().to_encoding();
		let account_id_string =
			std::str::from_utf8(&public_key_encoded).map_err(Error::Utf8Error)?;
		let account = horizon_client.get_account(account_id_string, self.is_public_network).await?;
		let next_sequence_number = account.sequence + 1;

		tracing::info!(
			"Next sequence number: {} for account: {:?}",
			next_sequence_number,
			account.account_id
		);

		let envelope = self.create_envelope(
			destination_address,
			asset,
			stroop_amount,
			request_id,
			stroop_fee_per_operation,
			next_sequence_number,
		)?;

		let _ = self.cache.save_tx_envelope(envelope.clone())?;
		self.submit_transaction(envelope, horizon_client).await
	}

	///return balances for public key
	pub async fn get_balance(&self) -> Result<Vec<Balance>, Error> {
		let horizon_client = reqwest::Client::new();
		let public_key_encoded = self.get_public_key().to_encoding();
		let account_id_string =
			std::str::from_utf8(&public_key_encoded).map_err(Error::Utf8Error)?;
		let account = horizon_client.get_account(account_id_string, self.is_public_network).await?;
		Ok(account.balances)
	}
}

impl std::fmt::Debug for StellarWallet {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let public_key_encoded = self.get_public_key().to_encoding();
		let account_id_string =
			std::str::from_utf8(&public_key_encoded).map_err(|_e| std::fmt::Error)?;
		write!(
			f,
			"StellarWallet [public key: {}, public network: {}]",
			account_id_string, self.is_public_network
		)
	}
}

#[cfg(feature = "testing-utils")]
impl Drop for StellarWallet {
	fn drop(&mut self) {
		self.cache.remove_dir();
	}
}

#[cfg(test)]
mod test {
	use crate::error::Error;
	use primitives::TransactionEnvelopeExt;
	use serial_test::serial;
	use std::sync::Arc;
	use substrate_stellar_sdk::{PublicKey, TransactionEnvelope, XdrCodec};
	use tokio::sync::RwLock;

	use crate::StellarWallet;

	const STELLAR_VAULT_SECRET_KEY: &str =
		"SCV7RZN5XYYMMVSWYCR4XUMB76FFMKKKNHP63UTZQKVM4STWSCIRLWFJ";
	const IS_PUBLIC_NETWORK: bool = false;

	fn wallet(storage: &str) -> Arc<RwLock<StellarWallet>> {
		Arc::new(RwLock::new(
			StellarWallet::from_secret_encoded_with_cache(
				&STELLAR_VAULT_SECRET_KEY.to_string(),
				IS_PUBLIC_NETWORK,
				storage.to_string(),
			)
			.unwrap(),
		))
	}

	#[test]
	fn test_add_backoff_delay() {
		let wallet = StellarWallet::from_secret_encoded_with_cache(
			&STELLAR_VAULT_SECRET_KEY.to_string(),
			IS_PUBLIC_NETWORK,
			"resources/test_add_backoff_delay".to_owned(),
		)
		.unwrap();

		assert_eq!(wallet.max_backoff_delay, StellarWallet::DEFAULT_MAX_BACKOFF_DELAY_IN_SECS);

		// too big backoff delay
		let expected_max_backoff_delay = 800;
		let new_wallet = wallet.with_max_backoff_delay(expected_max_backoff_delay);
		assert_ne!(new_wallet.max_backoff_delay, expected_max_backoff_delay);

		let expected_max_backoff_delay = 300;
		let new_wallet = new_wallet.with_max_backoff_delay(expected_max_backoff_delay);
		assert_eq!(new_wallet.max_backoff_delay, expected_max_backoff_delay);

		new_wallet.cache.remove_dir();
	}

	#[test]
	fn test_add_retry_attempt() {
		let wallet = StellarWallet::from_secret_encoded_with_cache(
			&STELLAR_VAULT_SECRET_KEY.to_string(),
			IS_PUBLIC_NETWORK,
			"resources/test_add_retry_attempt".to_owned(),
		)
		.unwrap();

		assert_eq!(
			wallet.max_retry_attempts_before_fallback,
			StellarWallet::DEFAULT_MAX_RETRY_ATTEMPTS_BEFORE_FALLBACK
		);

		let expected_max_retries = 5;
		let new_wallet = wallet.with_max_retry_attempts_before_fallback(expected_max_retries);
		assert_eq!(new_wallet.max_retry_attempts_before_fallback, expected_max_retries);

		new_wallet.cache.remove_dir();
	}

	#[tokio::test]
	#[serial]
	async fn test_locking_submission() {
		let wallet = wallet("resources/test_locking_submission").clone();
		let wallet_clone = wallet.clone();

		let first_job = tokio::spawn(async move {
			let destination = PublicKey::from_encoding(
				"GCENYNAX2UCY5RFUKA7AYEXKDIFITPRAB7UYSISCHVBTIAKPU2YO57OA",
			)
			.unwrap();
			let asset = substrate_stellar_sdk::Asset::native();
			let amount = 100;
			let request_id = [0u8; 32];

			let response = wallet_clone
				.write()
				.await
				.send_payment_to_address(destination, asset, amount, request_id, 100)
				.await
				.expect("it should return a success");

			assert!(!response.hash.to_vec().is_empty());
			assert!(response.ledger() > 0);
		});

		let wallet_clone2 = wallet.clone();
		let second_job = tokio::spawn(async move {
			let destination = PublicKey::from_encoding(
				"GCENYNAX2UCY5RFUKA7AYEXKDIFITPRAB7UYSISCHVBTIAKPU2YO57OA",
			)
			.unwrap();
			let asset = substrate_stellar_sdk::Asset::native();
			let amount = 50;
			let request_id = [1u8; 32];

			let result = wallet_clone2
				.write()
				.await
				.send_payment_to_address(destination, asset, amount, request_id, 100)
				.await;

			let transaction_response = result.expect("should return a transaction response");
			assert!(!transaction_response.hash.to_vec().is_empty());
			assert!(transaction_response.ledger() > 0);
		});

		let _ = tokio::join!(first_job, second_job);

		wallet.read().await.cache.remove_dir();
	}

	#[tokio::test]
	#[serial]
	async fn sending_payment_works() {
		let wallet = wallet("resources/sending_payment_works");
		let destination =
			PublicKey::from_encoding("GCENYNAX2UCY5RFUKA7AYEXKDIFITPRAB7UYSISCHVBTIAKPU2YO57OA")
				.unwrap();
		let asset = substrate_stellar_sdk::Asset::native();
		let amount = 100;
		let request_id = [0u8; 32];

		let result = wallet
			.write()
			.await
			.send_payment_to_address(destination, asset, amount, request_id, 100)
			.await;

		assert!(result.is_ok());
		let transaction_response = result.unwrap();
		assert!(!transaction_response.hash.to_vec().is_empty());
		assert!(transaction_response.ledger() > 0);
		wallet.read().await.cache.remove_dir();
	}

	#[tokio::test]
	#[serial]
	async fn sending_correct_payment_after_incorrect_payment_works() {
		let wallet =
			wallet("resources/sending_correct_payment_after_incorrect_payment_works").clone();
		let mut wallet = wallet.write().await;

		// let's cleanup, just to make sure.
		wallet.cache.remove_all_tx_envelopes();

		let destination =
			PublicKey::from_encoding("GCENYNAX2UCY5RFUKA7AYEXKDIFITPRAB7UYSISCHVBTIAKPU2YO57OA")
				.unwrap();
		let asset = substrate_stellar_sdk::Asset::native();
		let amount = 1000;
		let request_id = [0u8; 32];
		let correct_amount_that_should_not_fail = 100;
		let incorrect_amount_that_should_fail = 0;

		let response = wallet
			.send_payment_to_address(
				destination.clone(),
				asset.clone(),
				amount,
				request_id,
				correct_amount_that_should_not_fail,
			)
			.await;

		assert!(response.is_ok());

		let err_insufficient_fee = wallet
			.send_payment_to_address(
				destination.clone(),
				asset.clone(),
				amount,
				request_id,
				incorrect_amount_that_should_fail,
			)
			.await;

		assert!(err_insufficient_fee.is_err());
		match err_insufficient_fee.unwrap_err() {
			Error::HorizonSubmissionError { title: _, status: _, reason, envelope_xdr: _ } => {
				assert_eq!(reason, "tx_insufficient_fee");
			},
			_ => assert!(false),
		}

		let tx_response = wallet
			.send_payment_to_address(
				destination.clone(),
				asset.clone(),
				amount,
				request_id,
				correct_amount_that_should_not_fail,
			)
			.await;

		assert!(tx_response.is_ok());

		wallet.cache.remove_dir();
	}

	#[tokio::test]
	#[serial]
	async fn resubmit_transactions_works() {
		let wallet = wallet("resources/resubmit_transactions_works").clone();
		let mut wallet = wallet.write().await;

		// let's send a successful transaction first
		let destination =
			PublicKey::from_encoding("GCENYNAX2UCY5RFUKA7AYEXKDIFITPRAB7UYSISCHVBTIAKPU2YO57OA")
				.unwrap();
		let asset = substrate_stellar_sdk::Asset::native();
		let amount = 1001;
		let stroop_fee = 100;
		let request_id = [0u8; 32];

		let response = wallet
			.send_payment_to_address(
				destination.clone(),
				asset.clone(),
				amount,
				request_id,
				stroop_fee,
			)
			.await
			.expect("should be ok");

		// get the sequence number of the previous one.
		let env =
			TransactionEnvelope::from_base64_xdr(response.envelope_xdr).expect("should convert ok");
		let seq_number = env.sequence_number().expect("should return sequence number");

		// creating a `tx_bad_seq` envelope.
		let request_id = [1u8; 32];
		let bad_envelope = wallet
			.create_envelope(
				destination.clone(),
				asset.clone(),
				amount,
				request_id,
				stroop_fee,
				seq_number,
			)
			.expect("should return an envelope");

		// let's save this in storage
		let _ = wallet.cache.save_tx_envelope(bad_envelope.clone()).expect("should save.");

		// create a successful transaction
		let request_id = [2u8; 32];
		let good_envelope = wallet
			.create_envelope(destination, asset, amount, request_id, stroop_fee, seq_number + 1)
			.expect("should return an envelope");

		// let's save this in storage
		let _ = wallet.cache.save_tx_envelope(good_envelope.clone()).expect("should save");

		// let's resubmit these 2 transactions
		let receivers = wallet.resubmit_transactions_from_cache().await;
		assert_eq!(receivers.len(), 2);

		// a count on how many txs passed, and how many failed.
		let mut passed_count = 0;
		let mut failed_count = 0;

		for receiver in receivers {
			match &receiver.await {
				Ok(Ok(env)) => {
					assert_eq!(env.envelope_xdr, good_envelope.to_base64_xdr());
					passed_count += 1;
				},
				Ok(Err(Error::HorizonSubmissionError {
					title: _,
					status: _,
					reason,
					envelope_xdr: _,
				})) => {
					assert_eq!(reason, "tx_bad_seq");
					failed_count += 1;
				},
				other => {
					panic!("other result was received: {other:?}")
				},
			}
		}

		// 1 should pass, and 1 should fail.
		assert_eq!(passed_count, 1);
		assert_eq!(failed_count, 1);

		wallet.cache.remove_dir();
	}
}
