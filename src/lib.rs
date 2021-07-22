use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata,
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde_json::json;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{
    env, near_bindgen, PanicOnDefault, Promise, Balance, Gas,
};

near_sdk::setup_alloc!();


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self{}
    }

    pub fn deploy_status_message(&self, account_id: String, amount: U128) {
        Promise::new(account_id)
            .create_account()
            .transfer(amount.0)
            .add_full_access_key(env::signer_account_pk())
            .deploy_contract(
                include_bytes!("../res/nft.wasm").to_vec(),
            );
    }
}

