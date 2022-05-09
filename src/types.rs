use bitcoin::util::{address::Address, psbt::PartiallySignedTransaction};
use bitcoin::util::bip32::ExtendedPubKey;

use std::ops::Deref;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct HWIExtendedPubKey {
    pub xpub: ExtendedPubKey,
}

#[derive(Deserialize)]
pub struct HWISignature {
    #[serde(deserialize_with = "from_b64")]
    pub signature: Vec<u8>,
}

fn from_b64<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
    let b64_string = String::deserialize(d)?;
    base64::decode(&b64_string)
        .map_err(|_| serde::de::Error::custom("Error while Deserializing Signature"))
}

impl Deref for HWISignature {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.signature
    }
}

#[derive(Deserialize)]
pub struct HWIAddress {
    pub address: Address,
}

#[derive(Deserialize)]
pub struct HWIPartiallySignedTransaction {
    pub psbt: PartiallySignedTransaction,
}

impl Deref for HWIPartiallySignedTransaction {
    type Target = PartiallySignedTransaction;

    fn deref(&self) -> &Self::Target {
        &self.psbt
    }
}

// TODO: use Descriptors
#[derive(Deserialize)]
pub struct HWIDescriptor {
    pub internal: Vec<String>,
    pub receive: Vec<String>,
}

#[derive(Deserialize)]
pub struct HWIKeyPoolElement {
    pub desc: String,
    pub range: Vec<u32>,
    pub timestamp: String,
    pub internal: bool,
    pub keypool: bool,
    pub watchonly: bool,
}

#[derive(Clone)]
pub enum HWIAddressType {
    Pkh,
    ShWpkh,
    Wpkh,
}
