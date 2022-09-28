use near_sdk::assert_one_yocto;
use near_sdk::serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct IsLocked {
    pub is_locked: bool,
}

pub trait NonFungibleTokenLock {
    /// Locks NFT contract,
    /// it means all functions which modifies NFT contract metadata will be locked.
    fn nft_lock(&mut self);

    /// Returns information if NFT contract is locked
    fn is_locked(&self) -> IsLocked;
}

#[near_bindgen]
impl NonFungibleTokenLock for Contract {
    #[payable]
    fn nft_lock(&mut self) {
        assert_one_yocto();
        assert_eq!(env::predecessor_account_id(), self.tokens.owner_id, "Unauthorized");
        self.is_locked = true;
    }

    fn is_locked(&self) -> IsLocked {
        IsLocked { is_locked: self.is_locked }
    }
}
