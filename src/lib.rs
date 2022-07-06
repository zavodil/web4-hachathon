mod web4;
mod utils;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, env};
use near_sdk::collections::{UnorderedMap};
use crate::utils::unordered_map_pagination;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub applications: UnorderedMap<AccountId, Option<ApplicationData>>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ApplicationData {
    pub description: String,
    pub github_url: String,
    pub contact_data: String,
    pub contract_id: String,
    pub youtube_url: Option<String>
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Applications
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            applications: UnorderedMap::new(StorageKey::Applications)
        }
    }

    pub fn register (&mut self, application: Option<ApplicationData>) {
        assert!(env::block_timestamp() <= 1657864800000000000, "ERR_TOO_LATE");
        self.applications.insert(&env::predecessor_account_id(), &application);
    }

     pub fn get_applications (&self) -> Vec<(AccountId, Option<ApplicationData>)> {
         unordered_map_pagination(&self.applications, None, None)
    }
}