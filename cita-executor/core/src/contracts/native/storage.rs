// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

use self::bincode::internal::serialize_into;
use self::bincode::Infinite;
use super::factory::{Contract, Factory};
use bincode;
use cita_types::{Address, H256, U256};
use contracts::tools::method as method_tools;
use evm;
use evm::fake_tests::FakeExt;
use std::io::Write;
use std::str::FromStr;
use types::reserved_addresses;

use byteorder::BigEndian;
use evm::action_params::ActionParams;
use evm::ext::Ext;
use evm::return_data::{GasLeft, ReturnData};
use evm::storage::*;

#[derive(Clone)]
pub struct SimpleStorage {
    uint_value: Scalar,
    string_value: Scalar,
    array_value: Array,
    map_value: Map,
    output: Vec<u8>,
}

impl Contract for SimpleStorage {
    fn exec(&mut self, params: &ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error> {
        if let Some(ref data) = params.data {
            method_tools::extract_to_u32(&data[..]).and_then(|signature| match signature {
                0 => self.init(params, ext),
                0xaa91543e => self.uint_set(params, ext),
                0x832b4580 => self.uint_get(params, ext),
                0xc9615770 => self.string_set(params, ext),
                0xe3135d14 => self.string_get(params, ext),
                0x118b229c => self.array_set(params, ext),
                0x180a4bbf => self.array_get(params, ext),
                0xaaf27175 => self.map_set(params, ext),
                0xc567dff6 => self.map_get(params, ext),
                _ => Err(evm::Error::OutOfGas),
            })
        } else {
            Err(evm::Error::OutOfGas)
        }
    }
    fn create(&self) -> Box<Contract> {
        Box::new(SimpleStorage::default())
    }
}

impl Default for SimpleStorage {
    fn default() -> Self {
        SimpleStorage {
            output: Vec::new(),
            uint_value: Scalar::new(H256::from(0)),
            string_value: Scalar::new(H256::from(1)),
            array_value: Array::new(H256::from(2)),
            map_value: Map::new(H256::from(3)),
        }
    }
}

impl SimpleStorage {
    fn init(&mut self, _params: &ActionParams, _ext: &mut Ext) -> Result<GasLeft, evm::Error> {
        Ok(GasLeft::Known(U256::from(100)))
    }
    // 1) uint
    fn uint_set(&mut self, params: &ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error> {
        let value = U256::from(
            params
                .data
                .to_owned()
                .expect("invalid data")
                .get(4..36)
                .expect("no enough data"),
        );
        self.uint_value.set(ext, value)?;
        Ok(GasLeft::Known(U256::from(100)))
    }
    fn uint_get(&mut self, _params: &ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error> {
        self.output.resize(32, 0);
        self.uint_value
            .get(ext)?
            .to_big_endian(self.output.as_mut_slice());
        Ok(GasLeft::NeedsReturn {
            gas_left: U256::from(100),
            data: ReturnData::new(self.output.clone(), 0, self.output.len()),
            apply_state: true,
        })
    }

    // 2) string
    fn string_set(&mut self, params: &ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error> {
        let data = params.data.to_owned().expect("invalid data");
        let index = U256::from(data.get(4..36).expect("no enough data")).low_u64() as usize + 4;
        let length =
            U256::from(data.get(index..(index + 32)).expect("no enough data")).low_u64() as usize;
        let index = index + 32;
        let value = String::from_utf8(Vec::from(
            data.get(index..index + length).expect("no enough data"),
        ))
        .unwrap();

        self.string_value.set_bytes(ext, &value)?;
        Ok(GasLeft::Known(U256::from(100)))
    }
    fn string_get(&mut self, _params: &ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error> {
        self.output.resize(0, 0);
        let str = self.string_value.get_bytes::<String>(ext)?;
        for i in U256::from(32).0.iter().rev() {
            serialize_into::<_, _, _, BigEndian>(&mut self.output, &i, Infinite)
                .expect("failed to serialize u64");
        }
        for i in U256::from(str.len()).0.iter().rev() {
            serialize_into::<_, _, _, BigEndian>(&mut self.output, &i, Infinite)
                .expect("failed to serialize u64");
        }

        for i in str.bytes() {
            serialize_into::<_, _, _, BigEndian>(&mut self.output, &i, Infinite)
                .expect("failed to serialize ");
        }
        self.output
            .write(&vec![0u8; 32 - str.len() % 32])
            .expect("failed to write [u8]");
        Ok(GasLeft::NeedsReturn {
            gas_left: U256::from(100),
            data: ReturnData::new(self.output.clone(), 0, self.output.len()),
            apply_state: true,
        })
    }

    // 3) array
    fn array_set(&mut self, params: &ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error> {
        let data = params.data.to_owned().expect("invalid data");
        let mut pilot = 4;
        let index = U256::from(data.get(pilot..pilot + 32).expect("no enough data")).low_u64();
        pilot += 32;
        let value = U256::from(data.get(pilot..pilot + 32).expect("no enough data"));
        self.array_value.set(ext, index, &value)?;
        Ok(GasLeft::Known(U256::from(100)))
    }
    fn array_get(&mut self, params: &ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error> {
        let data = params.data.to_owned().expect("invalid data");
        let index = U256::from(data.get(4..4 + 32).expect("no enough data")).low_u64();
        for i in self.array_value.get(ext, index)?.0.iter().rev() {
            serialize_into::<_, _, _, BigEndian>(&mut self.output, &i, Infinite)
                .expect("failed to serialize u64");
        }
        Ok(GasLeft::NeedsReturn {
            gas_left: U256::from(100),
            data: ReturnData::new(self.output.clone(), 0, self.output.len()),
            apply_state: true,
        })
    }

    // 4) map
    fn map_set(&mut self, params: &ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error> {
        let data = params.data.to_owned().expect("invalid data");
        let mut pilot = 4;
        let key = U256::from(data.get(pilot..pilot + 32).expect("no enough data"));
        pilot += 32;
        let value = U256::from(data.get(pilot..pilot + 32).expect("no enough data"));
        self.map_value.set(ext, &key, value)?;
        Ok(GasLeft::Known(U256::from(100)))
    }
    fn map_get(&mut self, params: &ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error> {
        let data = params.data.to_owned().expect("invalid data");
        let key = U256::from(data.get(4..4 + 32).expect("no enough data"));
        for i in self.map_value.get(ext, &key)?.0.iter().rev() {
            serialize_into::<_, _, _, BigEndian>(&mut self.output, &i, Infinite)
                .expect("failed to serialize u64");
        }
        Ok(GasLeft::NeedsReturn {
            gas_left: U256::from(100),
            data: ReturnData::new(self.output.clone(), 0, self.output.len()),
            apply_state: true,
        })
    }
}

#[test]
fn test_native_contract() {
    let factory = Factory::default();
    let mut ext = FakeExt::new();
    let native_addr = Address::from_str(reserved_addresses::NATIVE_SIMPLE_STORAGE).unwrap();
    let value = U256::from(0x1234);
    {
        let mut params = ActionParams::default();
        let mut input = Vec::new();
        let index = 0xaa91543eu32;
        serialize_into::<_, _, _, BigEndian>(&mut input, &index, Infinite)
            .expect("failed to serialize u32");
        for i in value.0.iter().rev() {
            serialize_into::<_, _, _, BigEndian>(&mut input, &i, Infinite)
                .expect("failed to serialize u64");
        }
        params.data = Some(input);
        let mut contract = factory.new_contract(native_addr).unwrap();
        let output = contract.exec(&params, &mut ext).unwrap();
        println!("===={:?}", output);
    }
    {
        let mut params = ActionParams::default();
        let mut input = Vec::new();
        let index = 0x832b4580u32;
        serialize_into::<_, _, _, BigEndian>(&mut input, &index, Infinite)
            .expect("failed to serialize u32");
        params.data = Some(input);

        let mut contract = factory.new_contract(native_addr).unwrap();
        match contract.exec(&params, &mut ext) {
            Ok(GasLeft::NeedsReturn {
                gas_left: _,
                data: return_data,
                apply_state: true,
            }) => {
                let real = U256::from(&*return_data);
                assert!(real == value);
            }
            _ => assert!(false, "no output data"),
        };
    }
}
