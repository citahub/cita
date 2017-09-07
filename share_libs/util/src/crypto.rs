// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

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
