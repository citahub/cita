extern crate evm;

use self::evm::cita_types::Address;
use serde_derive::Deserialize;
use serde_json::Error;
use std::collections::BTreeMap;
use std::io::Read;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Env {
    #[serde(rename = "currentCoinbase")]
    pub current_coinbase: Address,

    #[serde(rename = "currentDifficulty")]
    pub current_difficulty: String,

    #[serde(rename = "currentGasLimit")]
    pub current_gas_limit: String,

    #[serde(rename = "currentNumber")]
    pub current_number: String,

    #[serde(rename = "currentTimestamp")]
    pub current_timestamp: String,

    #[serde(rename = "previousHash")]
    pub previous_hash: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Transaction {
    #[serde(rename = "data")]
    pub data: Vec<String>,

    #[serde(rename = "gasLimit")]
    pub gas_limit: Vec<String>,

    #[serde(rename = "gasPrice")]
    pub gas_price: String,

    #[serde(rename = "nonce")]
    pub nonce: String,

    #[serde(rename = "secretKey")]
    pub secret_key: String,

    #[serde(rename = "to")]
    pub to: String,

    #[serde(rename = "value")]
    pub value: Vec<String>,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Account {
    pub balance: String,
    pub code: String,
    pub nonce: String,
    pub storage: BTreeMap<String, String>,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct State(pub BTreeMap<Address, Account>);

impl IntoIterator for State {
    type Item = <BTreeMap<Address, Account> as IntoIterator>::Item;
    type IntoIter = <BTreeMap<Address, Account> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct PostData {
    #[serde(rename = "hash")]
    pub hash: String,

    #[serde(rename = "indexes")]
    pub indexes: BTreeMap<String, usize>,

    #[serde(rename = "logs")]
    pub logs: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Post {
    #[serde(rename = "Byzantium")]
    pub byzantium: Option<Vec<PostData>>,

    #[serde(rename = "Constantinople")]
    pub constantinople: Option<Vec<PostData>>,

    #[serde(rename = "EIP150")]
    pub eip150: Option<Vec<PostData>>,

    #[serde(rename = "EIP158")]
    pub eip158: Option<Vec<PostData>>,

    #[serde(rename = "Frontier")]
    pub frontier: Option<Vec<PostData>>,

    #[serde(rename = "Homestead")]
    pub homestead: Option<Vec<PostData>>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Vm {
    #[serde(rename = "env")]
    pub env: Env,

    #[serde(rename = "transaction")]
    pub transaction: Transaction,

    #[serde(rename = "post")]
    pub post: Option<Post>,

    #[serde(rename = "pre")]
    pub pre: Option<State>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Test(BTreeMap<String, Vm>);

impl IntoIterator for Test {
    type Item = <BTreeMap<String, Vm> as IntoIterator>::Item;
    type IntoIter = <BTreeMap<String, Vm> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Test {
    pub fn load<R>(reader: R) -> Result<Self, Error>
    where
        R: Read,
    {
        serde_json::from_reader(reader)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn test_json_tests_parse() {
        let f = fs::File::open("./tests/jsondata/GeneralStateTests/stArgsZeroOneBalance/addmodNonConst.json").unwrap();
        let t = Test::load(f).unwrap();
        assert!(t.0.contains_key("addmodNonConst"));
        let v = &t.0["addmodNonConst"];
        assert_eq!(v.env.current_coinbase, Address::from("0x2adc25665018aa1fe0e6bc666dac8fc2697ff9ba"));
    }
}
