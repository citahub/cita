use cita_types::{Address, H256};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
struct Bar {
    x: BTreeMap<u64, String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rlp() {
        let mut p = Bar { x: BTreeMap::new() };
        p.x.insert(1, "2.0".to_string());
        p.x.insert(2, "4.0".to_string());
        p.x.insert(3, "6.0".to_string());
        let serialized = serde_json::to_string(&p).unwrap();
        println!("the serialized string is {}", serialized);

        let bar: Bar = serde_json::from_str(&serialized).unwrap();
        println!("the origin structure is {:?}", bar);

        assert_eq!(
            H256::from(0).0,
            H256::from(Address::from("0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523")).0
        );
    }
}
