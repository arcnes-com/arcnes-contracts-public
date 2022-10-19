use std::collections::HashMap;

use near_contract_standards::non_fungible_token::refund_approved_account_ids;
use near_sdk::{assert_one_yocto, Balance};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};

use crate::*;
use crate::events::SetRoyalty;

// Royalty percentages
pub type Royalty = HashMap<AccountId, u32>;

pub fn assert_valid_royalty(royalty: &Royalty) {
    // Check if max number of royalties is not exceeded
    assert!(royalty.len() < 7, "Cannot add more than 6 royalty amounts");

    // Check if royalties doesn't exceed 100%
    let amounts: Vec<u32> = royalty.values().cloned().collect();
    let sum: u32 = amounts.iter().sum();
    assert!(sum < 10_000u32, "Cannot set 100% or more for royalties");
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonRoyalty {
    pub royalty: Royalty,
}

pub trait NonFungibleTokenRoyalty {
    /// Calculates the payout for a token given the passed in balance. This is a view method
    fn nft_payout(&self, token_id: TokenId, balance: U128, max_len_payout: u32) -> Payout;

    /// Transfers the token to the receiver ID and returns the payout object that should be payed given the passed in balance.
    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: Option<String>,
        balance: U128,
        max_len_payout: u32,
    ) -> Payout;

    /// Returns the token royalty
    fn nft_royalty(&self) -> JsonRoyalty;

    /// Sets new token royalty values. This function can be locked.
    fn set_nft_royalty(&mut self, royalty: Royalty);
}

#[near_bindgen]
impl NonFungibleTokenRoyalty for Contract {
    fn nft_payout(&self, token_id: TokenId, balance: U128, max_len_payout: u32) -> Payout {
        //get the token object
        let owner_id = self.tokens.owner_by_id.get(&token_id).expect("No token");
        let royalty = self.royalty.get().unwrap_or_default();

        //keep track of the total perpetual royalties
        let mut total_perpetual = 0;
        //get the u128 version of the passed in balance (which was U128 before)
        let balance_u128 = u128::from(balance);
        //keep track of the payout object to send back
        let mut payout_object = Payout {
            payout: HashMap::new()
        };

        //make sure we're not paying out to too many people (GAS limits this)
        assert!(royalty.len() as u32 <= max_len_payout, "Market cannot payout to that many receivers");

        //go through each key and value in the royalty object
        for (k, v) in royalty.iter() {
            //get the key
            let key = k.clone();
            //only insert into the payout if the key isn't the token owner (we add their payout at the end)
            if key != owner_id {
                //
                payout_object.payout.insert(key, royalty_to_payout(*v, balance_u128));
                total_perpetual += *v;
            }
        }

        // payout to previous owner who gets 100% - total perpetual royalties
        payout_object.payout.insert(owner_id, royalty_to_payout(10000 - total_perpetual, balance_u128));

        //return the payout object
        payout_object
    }

    #[payable]
    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: Option<String>,
        balance: U128,
        max_len_payout: u32,
    ) -> Payout {
        //assert that the user attached 1 yocto NEAR for security reasons
        assert_one_yocto();
        //get the sender ID
        let sender_id = env::predecessor_account_id();
        //transfer the token to the passed in receiver and get the previous token object back
        let (previous_token_owner_id, previous_token_approvals) = self.tokens.internal_transfer(
            &sender_id,
            &receiver_id,
            &token_id,
            Some(approval_id),
            memo,
        );

        //refund the previous token owner for the storage used up by the previous approved account IDs
        refund_approved_account_ids(
            previous_token_owner_id.clone(),
            &previous_token_approvals.unwrap_or_default(),
        );

        //get the owner of the token
        let owner_id = previous_token_owner_id;
        //keep track of the total perpetual royalties
        let mut total_perpetual = 0;
        //get the u128 version of the passed in balance (which was U128 before)
        let balance_u128 = u128::from(balance);
        //keep track of the payout object to send back
        let mut payout_object = Payout {
            payout: HashMap::new()
        };
        //get the royalty object from token
        let royalty = self.royalty.get().unwrap_or_default();

        //make sure we're not paying out to too many people (GAS limits this)
        assert!(royalty.len() as u32 <= max_len_payout, "Market cannot payout to that many receivers");

        //go through each key and value in the royalty object
        for (k, v) in royalty.iter() {
            //get the key
            let key = k.clone();
            //only insert into the payout if the key isn't the token owner (we add their payout at the end)
            if key != owner_id {
                payout_object.payout.insert(key, royalty_to_payout(*v, balance_u128));
                total_perpetual += *v;
            }
        }

        // payout to previous owner who gets 100% - total perpetual royalties
        payout_object.payout.insert(owner_id, royalty_to_payout(10000 - total_perpetual, balance_u128));

        //return the payout object
        payout_object
    }

    fn nft_royalty(&self) -> JsonRoyalty {
        JsonRoyalty { royalty: self.royalty.get().unwrap_or_default() }
    }

    #[payable]
    fn set_nft_royalty(&mut self, royalty: Royalty) {
        assert_one_yocto();

        // Only collection owner can call this function
        assert_eq!(env::predecessor_account_id(), self.tokens.owner_id, "Unauthorized");

        // Check if function is not locked
        assert!(!self.is_locked, "Locked function");

        assert_valid_royalty(&royalty);

        let previous_royalty = self.royalty.get().unwrap_or_default();
        self.royalty.set(&royalty);

        SetRoyalty { previous_royalty, new_royalty: royalty }.emit();
    }
}

//convert the royalty percentage and amount to pay into a payout (U128)
pub(crate) fn royalty_to_payout(royalty_percentage: u32, amount_to_pay: Balance) -> U128 {
    U128(royalty_percentage as u128 * amount_to_pay / 10_000u128)
}
