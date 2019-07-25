use evm::cita_types::Address;
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::{Read, Write};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Test(pub BTreeMap<String, Vm>);

impl IntoIterator for Test {
    type Item = <BTreeMap<String, Vm> as IntoIterator>::Item;
    type IntoIter = <BTreeMap<String, Vm> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Test {
    pub fn load<R>(reader: R) -> Result<Self, serde_json::Error>
    where
        R: Read,
    {
        serde_json::from_reader(reader)
    }

    pub fn store<W>(&self, wr: W) -> Result<(), serde_json::Error>
    where
        W: Write,
    {
        serde_json::to_writer_pretty(wr, self)
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Vm {
    #[serde(rename = "callcreates")]
    pub call_creates: Option<Vec<Callcreates>>,

    #[serde(rename = "env")]
    pub env: Env,

    #[serde(rename = "exec")]
    pub exec: Exec,

    #[serde(rename = "gas")]
    pub gas: Option<String>,

    #[serde(rename = "logs")]
    pub logs: Option<String>,

    #[serde(rename = "out")]
    pub out: Option<String>,

    #[serde(rename = "post")]
    pub post: Option<State>,

    #[serde(rename = "pre")]
    pub pre: Option<State>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Callcreates {}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
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
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Exec {
    #[serde(rename = "address")]
    pub address: Address,

    #[serde(rename = "caller")]
    pub caller: Address,

    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "data")]
    pub data: String,

    #[serde(rename = "gas")]
    pub gas: String,

    #[serde(rename = "gasPrice")]
    pub gas_price: String,

    #[serde(rename = "origin")]
    pub origin: Address,

    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Account {
    pub balance: String,
    pub code: String,
    pub nonce: String,
    pub storage: BTreeMap<String, String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct State(pub BTreeMap<Address, Account>);

impl IntoIterator for State {
    type Item = <BTreeMap<Address, Account> as IntoIterator>::Item;
    type IntoIter = <BTreeMap<Address, Account> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::{Address, Test};
    use std::fs;

    #[test]
    fn test_json_tests_parse() {
        let f = fs::File::open("../jsondata/VMTests/vmArithmeticTest/add0.json").unwrap();
        let t = Test::load(f).unwrap();
        assert!(t.0.contains_key("add0"));
        let v = &t.0["add0"];

        assert_eq!(
            v.env.current_coinbase,
            Address::from("0x2adc25665018aa1fe0e6bc666dac8fc2697ff9ba")
        );
        assert_eq!(v.env.current_difficulty, String::from("0x0100"));
        assert_eq!(v.env.current_gas_limit, String::from("0x0f4240"));
        assert_eq!(v.env.current_number, String::from("0x00"));
        assert_eq!(v.env.current_timestamp, String::from("0x01"));

        assert_eq!(
            v.exec.address,
            Address::from("0x0f572e5295c57f15886f9b263e2f6d2d6c7b5ec6")
        );
        assert_eq!(
            v.exec.caller,
            Address::from("0xcd1722f2947def4cf144679da39c4c32bdc35681")
        );
        assert_eq!(v.exec.data, String::from("0x"));
        assert_eq!(v.exec.gas, String::from("0x0186a0"));
        assert_eq!(v.exec.gas_price, String::from("0x5af3107a4000"));
        assert_eq!(
            v.exec.origin,
            Address::from("0xcd1722f2947def4cf144679da39c4c32bdc35681")
        );
        assert_eq!(v.exec.value, String::from("0x0de0b6b3a7640000"));

        assert_eq!(v.gas, Some(String::from("0x013874")));
        assert_eq!(
            v.logs,
            Some(String::from(
                "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"
            ))
        );
        assert_eq!(v.out, Some(String::from("0x")));

        if let Some(data) = &v.post {
            let post_account =
                &data.0[&Address::from("0x0f572e5295c57f15886f9b263e2f6d2d6c7b5ec6")];
            assert_eq!(post_account.balance, String::from("0x0de0b6b3a7640000"));
            assert_eq!(
                post_account.storage[&String::from("0x00")],
                String::from("0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe")
            )
        }

        if let Some(data) = &v.pre {
            let pre_account = &data.0[&Address::from("0x0f572e5295c57f15886f9b263e2f6d2d6c7b5ec6")];
            assert_eq!(pre_account.balance, String::from("0x0de0b6b3a7640000"));
        }
    }
}
