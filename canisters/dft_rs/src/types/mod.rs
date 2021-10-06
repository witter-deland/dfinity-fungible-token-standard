mod account_identifier;
mod approve_result;
mod burn_result;
mod call_data;
mod fee;
mod key_value_pair;
pub mod message;
mod metadata;
mod token_holder;
mod token_info;
mod token_payload;
mod transfer_result;
mod tx_record;

use candid::Principal;
use ic_cdk::export::candid::Nat;
use std::collections::HashMap;
use std::string::String;

pub type TransactionId = String;
pub type ExtendData = HashMap<String, String>;
pub type Balances = HashMap<TokenHolder, Nat>;
pub type StorageCanisterIds = HashMap<Nat, Principal>;
pub type Txs = Vec<TxRecord>;
pub type Allowances = HashMap<TokenHolder, HashMap<TokenHolder, Nat>>;
pub use account_identifier::AccountIdentifier;
pub use account_identifier::Subaccount;
pub use account_identifier::SUB_ACCOUNT_ZERO;
pub use call_data::CallData;
pub use fee::Fee;
pub use key_value_pair::KeyValuePair;
pub use metadata::MetaData;
pub use token_holder::TokenHolder;
pub type TransferFrom = TokenHolder;
pub type TokenReceiver = TokenHolder;
pub use approve_result::ApproveResponse;
pub use approve_result::ApproveResult;
pub use burn_result::BurnResponse;
pub use burn_result::BurnResult;
pub use token_info::TokenInfo;
pub use token_payload::TokenPayload;
pub use transfer_result::TransferResponse;
pub use transfer_result::TransferResult;
pub use tx_record::TxRecord;

#[test]
fn test_nat_size() {
    let nat_size = std::mem::size_of::<Nat>();
    assert_eq!(24, nat_size, "nat_size is not {}", 24);
}
