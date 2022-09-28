use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{assert_one_yocto, env};

use crate::*;
use crate::events::SetTokenMetadata;

pub trait NonFungibleTokenMetadata {
    /// Set specific token metadata, this function can be locked
    fn set_token_metadata(&mut self, token_id: TokenId, token_metadata: TokenMetadata);
}

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

#[near_bindgen]
impl NonFungibleTokenMetadata for Contract {
    #[payable]
    fn set_token_metadata(&mut self, token_id: TokenId, token_metadata: TokenMetadata) {
        assert_one_yocto();

        // Only collection owner can call this function
        assert_eq!(env::predecessor_account_id(), self.tokens.owner_id, "Unauthorized");

        // Check if function is not locked
        assert!(!self.is_locked, "Locked function");

        let token_metadata_by_id =
            self.tokens.token_metadata_by_id.as_mut().unwrap_or_else(|| {
                env::panic_str("NFT does not support Metadata");
            });
        let previous_token_metadata = token_metadata_by_id.get(&token_id);
        token_metadata_by_id.insert(&token_id, &token_metadata);

        SetTokenMetadata { token_id, previous_token_metadata, new_token_metadata: token_metadata }.emit();
    }
}
