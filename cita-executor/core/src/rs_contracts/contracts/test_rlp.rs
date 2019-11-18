use super::utils::{encode_to_vec, extract_to_u32};
use cita_types::{Address, H256, U256};
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
        // let mut p = Bar { x: BTreeMap::new() };
        // p.x.insert(1, "2.0".to_string());
        // p.x.insert(2, "4.0".to_string());
        // p.x.insert(3, "6.0".to_string());
        // let serialized = serde_json::to_string(&p).unwrap();
        // println!("the serialized string is {}", serialized);

        // let bar: Bar = serde_json::from_str(&serialized).unwrap();
        // println!("the origin structure is {:?}", bar);

        // assert_eq!(
        //     H256::from(0).0,
        //     H256::from(Address::from("0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523")).0
        // );
        // let v = vec![248, 81, 164, 64];
        // // let v = vec![96, 149, 34, 116];
        // let h = hex::encode(v.clone());
        // let res = extract_to_u32(&v).unwrap();
        // let r_h = hex::encode(res.to_vec());
        // assert_eq!(res.to_string(), r_h);

        // let v2 = encode_to_vec(b"admin()");
        // assert_eq!(v2, v);
        let v = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0,0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 132, 128];
        // let a = U256::from_big_endian(&v);
        // assert_eq!(a, U256::from(2000000));

        let b = H256::from(U256::from(2000000));
        let v1 = b.to_vec();
        // b.to_little_endian(&mut v1);
        assert_eq!(v1, v);


    }
}
