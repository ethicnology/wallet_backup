#![allow(unused)]
use std::collections::BTreeMap;

use miniscript::bitcoin::{taproot::Signature, Network, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Backup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub accounts: Vec<Account>,
    pub network: Network,
    /// App proprietary metadata (settings, configuration, etc..)
    #[serde(skip_serializing_if = "serde_json::Map::is_empty")]
    pub proprietary: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub descriptor: miniscript::Descriptor<miniscript::DescriptorPublicKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub keys: BTreeMap<miniscript::DescriptorPublicKey, Key>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<bip329::Labels>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub transactions: Vec<Transaction>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub psbts: Vec<miniscript::bitcoin::Psbt>,
    #[serde(skip_serializing_if = "serde_json::Map::is_empty")]
    pub proprietary: serde_json::Map<String, serde_json::Value>,
}

impl Account {
    pub fn new(descriptor: miniscript::Descriptor<miniscript::DescriptorPublicKey>) -> Self {
        Self {
            name: None,
            descriptor,
            timestamp: None,
            keys: BTreeMap::new(),
            labels: None,
            transactions: Vec::new(),
            psbts: Vec::new(),
            proprietary: serde_json::Map::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Key {
    pub key: miniscript::DescriptorPublicKey,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<KeyRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<KeyType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KeyRole {
    /// Key to be used in normal spending condition
    Main,
    /// Key that will be used for recover in case loss of main key(s)
    Recovery,
    /// Key that wil inherit coins if main user disapear
    Inheritance,
    /// Key that will cosign a spend in order to enforce some policy
    Cosigning,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KeyType {
    /// Main user
    Internal,
    /// Heirs or friends
    External,
    /// Service the user pay for
    ThirdParty,
}
