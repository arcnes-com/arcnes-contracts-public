use std::fmt;

use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_sdk::{env, serde_json};
use near_sdk::serde::{Deserialize, Serialize};
use crate::{CONTRACT_STANDARD, CONTRACT_VERSION};

use crate::royalty::Royalty;

#[must_use]
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct SetRoyalty {
    pub previous_royalty: Royalty,
    pub new_royalty: Royalty,
}

impl SetRoyalty {
    /// Logs the event to the host.
    pub fn emit(self) {
        new_log(NFTExtensionsEventKind::SetRoyalty(self)).emit()
    }
}

#[must_use]
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct SetTokenMetadata {
    pub token_id: String,
    pub previous_token_metadata: Option<TokenMetadata>,
    pub new_token_metadata: TokenMetadata,
}

impl SetTokenMetadata {
    /// Logs the event to the host.
    pub fn emit(self) {
        new_log(NFTExtensionsEventKind::SetTokenMetadata(self)).emit()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub enum NFTExtensionsEventKind {
    SetRoyalty(SetRoyalty),
    SetTokenMetadata(SetTokenMetadata),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLog {
    pub standard: String,
    pub version: String,

    // `flatten` to not have "event": {<EventLogVariant>} in the JSON, just have the contents of {<EventLogVariant>}.
    #[serde(flatten)]
    pub event: NFTExtensionsEventKind,
}

impl EventLog {
    pub(crate) fn emit(self) {
        env::log_str(&self.to_string());
    }
}

impl fmt::Display for EventLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "EVENT_JSON:{}",
            &serde_json::to_string(self).map_err(|_| fmt::Error)?
        ))
    }
}

fn new_log(event_variant: NFTExtensionsEventKind) -> EventLog {
    EventLog {
        standard: CONTRACT_STANDARD.to_string(),
        version: CONTRACT_VERSION.to_string(),
        event: event_variant,
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use std::collections::HashMap;
    use near_sdk::{AccountId, test_utils};

    use super::*;

    #[test]
    fn set_royalty_event() {
        let expected = r#"EVENT_JSON:{"standard":"arcnes_nft","version":"1.0.0","event":"set_royalty","data":{"previous_royalty":{},"new_royalty":{"test.near":200}}}"#;
        let event = SetRoyalty {
            previous_royalty: Default::default(),
            new_royalty: HashMap::from([(AccountId::new_unchecked("test.near".to_string()), 200)])
        };

        event.emit();
        assert_eq!(test_utils::get_logs()[0], expected);
    }

    #[test]
    fn set_token_metadata_event() {
        let expected = r#"EVENT_JSON:{"standard":"arcnes_nft","version":"1.0.0","event":"set_token_metadata","data":{"token_id":"1","previous_token_metadata":null,"new_token_metadata":{"title":null,"description":null,"media":null,"media_hash":null,"copies":null,"issued_at":null,"expires_at":null,"starts_at":null,"updated_at":null,"extra":null,"reference":null,"reference_hash":null}}}"#;
        let event = SetTokenMetadata {
            token_id: "1".to_string(),
            previous_token_metadata: None,
            new_token_metadata: TokenMetadata {
                title: None,
                description: None,
                media: None,
                media_hash: None,
                copies: None,
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                extra: None,
                reference: None,
                reference_hash: None
            }
        };

        event.emit();
        assert_eq!(test_utils::get_logs()[0], expected);
    }
}
