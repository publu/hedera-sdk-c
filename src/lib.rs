#![feature(try_from, type_ascription)]

#[macro_use]
mod macros;

mod alloc;
mod array;
mod claim;
mod client;
mod crypto;
mod duration;
mod errors;
mod id;
mod timestamp;
mod transaction_id;
mod transaction_record;

mod contract_info;

mod query_crypto_get_account_balance;

mod query_contract_get_bytecode;
mod query_contract_get_info;
// mod query_contract_get_records;

mod query_transaction_get_receipt;
mod query_transaction_get_record;

mod transaction;

mod transaction_admin_delete;
mod transaction_admin_recover;

mod transaction_contract_call;
mod transaction_contract_create;
mod transaction_contract_update;

mod transaction_crypto_add_claim;
mod transaction_crypto_create;
mod transaction_crypto_delete;
mod transaction_crypto_transfer;
mod transaction_crypto_update;

mod transaction_file_append;
mod transaction_file_create;
mod transaction_file_delete;
mod transaction_file_update;
