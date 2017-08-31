use super::Address;
use std::marker;

pub trait Sign
where
    Self: marker::Sized,
{
    type PrivKey;
    type PubKey;
    type Message;
    type Error;

    fn sign(privkey: &Self::PrivKey, message: &Self::Message) -> Result<Self, Self::Error>;
    fn recover(&self, message: &Self::Message) -> Result<Self::PubKey, Self::Error>;
    fn verify_public(&self, pubkey: &Self::PubKey, message: &Self::Message) -> Result<bool, Self::Error>;
    fn verify_address(&self, address: &Address, message: &Self::Message) -> Result<bool, Self::Error>;
}

pub trait CreateKey
where
    Self: marker::Sized,
{
    type PrivKey;
    type PubKey;
    type Error;

    fn from_privkey(privkey: Self::PrivKey) -> Result<Self, Self::Error>;
    fn gen_keypair() -> Self;
    fn privkey(&self) -> &Self::PrivKey;
    fn pubkey(&self) -> &Self::PubKey;
    fn address(&self) -> Address;
}
