#![allow(unused)]
use std::collections::BTreeMap;

use miniscript::bitcoin::{taproot::Signature, Transaction};

pub struct Backup {
    name: Option<String>,
    accounts: Vec<Account>,
    /// App proprietary metadata (settings, configuration, etc..)
    proprietary: serde_json::Map<String, serde_json::Value>,
}

pub struct Account {
    name: Option<String>,
    descriptor: miniscript::Descriptor<miniscript::DescriptorPublicKey>,
    timestamp: Option<u64>,
    keys: BTreeMap<miniscript::DescriptorPublicKey, Key>,
    labels: bip329::Labels,
    transactions: Vec<Transaction>,
    psbts: Vec<miniscript::bitcoin::Psbt>,
    proprietary: serde_json::Map<String, serde_json::Value>,
}

pub struct Key {
    key: miniscript::DescriptorPublicKey,
    alias: Option<String>,
    role: Option<KeyRole>,
    key_type: Option<KeyType>,
}

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

pub enum KeyType {
    /// Main user
    Internal,
    /// Heirs or friends
    External,
    /// Service the user pay for
    ThirdParty,
}
