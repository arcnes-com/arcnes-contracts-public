use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_sdk::{env, serde_json};
use near_sdk::serde::Serialize;

use crate::royalty::Royalty;

#[must_use]
#[derive(Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SetRoyalty {
    pub previous_royalty: Royalty,
    pub new_royalty: Royalty,
}

impl SetRoyalty {
    /// Logs the event to the host.
    pub fn emit(self) {
        NFTExtensionsEventKind::SetRoyalty(&self).emit()
    }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SetTokenMetadata {
    pub token_id: String,
    pub previous_token_metadata: Option<TokenMetadata>,
    pub new_token_metadata: TokenMetadata,
}

impl SetTokenMetadata {
    /// Logs the event to the host.
    pub fn emit(self) {
        NFTExtensionsEventKind::SetTokenMetadata(&self).emit()
    }
}

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
enum NFTExtensionsEventKind<'a> {
    SetRoyalty(&'a SetRoyalty),
    SetTokenMetadata(&'a SetTokenMetadata),
}

impl<'a> NFTExtensionsEventKind<'a> {
    fn to_json_string(&self) -> String {
        // Events cannot fail to serialize so fine to panic on error
        #[allow(clippy::redundant_closure)]
        serde_json::to_string(self).ok().unwrap_or_else(|| env::abort())
    }

    fn to_json_event_string(&self) -> String {
        format!("EVENT_JSON:{}", self.to_json_string())
    }

    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub(crate) fn emit(self) {
        env::log_str(&self.to_json_event_string());
    }
}