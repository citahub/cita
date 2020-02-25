// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::crypto::{Secp256k1Signature, HASH_BYTES_LEN, SIGNATURE_BYTES_LEN};
use base64::decode;
use chrono::{DateTime, Utc};
use cita_types::{Address, H256};
use tiny_keccak::{Hasher, Keccak};

const LICENSE_DATA_ELEMENT: usize = 6;

#[derive(Debug, PartialEq)]
pub enum LicenseType {
    Oem = 1,
    Chain,
    Node,
}

impl LicenseType {
    pub fn from_u32(num: u32) -> Result<LicenseType, String> {
        match num {
            1 => Ok(LicenseType::Oem),
            2 => Ok(LicenseType::Chain),
            3 => Ok(LicenseType::Node),
            _ => Err("Error license type".to_owned()),
        }
    }
}

#[derive(Debug)]
pub struct LicenseInfo {
    pub lic_version: u32,
    pub cita_version: String,
    pub lic_type: LicenseType,
    pub lic_value: String,
    pub begin_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub issuer: Address,
    pub finger_print: H256,
}

// License info format:
// base64( [License version | CITA version | Type | Value | Begin time | End time | Fingerprint | sign] )
impl LicenseInfo {
    pub fn from(data: String) -> Result<Self, String> {
        // Decode from base64
        let license_bytes =
            decode(&data).map_err(|e| format!("Decode license file error: {:?}", e))?;

        // Get license data, not include fingerprint and sign
        let len = HASH_BYTES_LEN + SIGNATURE_BYTES_LEN;
        let license_data_len = if let Some(data_len) = license_bytes.len().checked_sub(len) {
            data_len
        } else {
            return Err("License data len less than required!".to_owned());
        };
        let mut license_data: Vec<u8> = Vec::new();
        license_data.extend_from_slice(&license_bytes[..license_data_len]);
        // Get license data hash
        let mut lic_data_hasher = Keccak::v256();
        lic_data_hasher.update(&license_data);
        let mut lic_data_hash = [0u8; HASH_BYTES_LEN];
        lic_data_hasher.finalize(&mut lic_data_hash);
        // Get license info split by ' '.
        let license_data = String::from_utf8(license_data)
            .map_err(|e| format!("Get license data  error: {:?}", e))?;
        let data_vec: Vec<&str> = license_data.split(' ').collect();
        if data_vec.len() != LICENSE_DATA_ELEMENT {
            return Err("License file has been destroyed!".to_owned());
        }

        // Get finger print
        let mut finger_print: [u8; HASH_BYTES_LEN] = Default::default();
        finger_print
            .copy_from_slice(&license_bytes[license_data_len..license_data_len + HASH_BYTES_LEN]);
        // Check data integrity
        if lic_data_hash.ne(&finger_print) {
            return Err("License file has been destroyed!".to_owned());
        }
        // Get sign
        let mut sign = [4u8; SIGNATURE_BYTES_LEN];
        sign.copy_from_slice(
            &license_bytes[license_data_len + HASH_BYTES_LEN
                ..license_data_len + HASH_BYTES_LEN + SIGNATURE_BYTES_LEN],
        );

        // Recover public key from finger print and sign
        let sign = Secp256k1Signature(sign);
        let pub_key = sign
            .recover(&H256(finger_print))
            .map_err(|e| format!("Recover sign error: {:?}", e))?;
        if !sign
            .verify_public(&pub_key, &H256(finger_print))
            .map_err(|e| format!("{:?}", e))?
        {
            return Err("Incorrect sign".to_owned());
        }

        // Get address from public key
        let mut hasher = Keccak::v256();
        hasher.update(&pub_key);
        let mut output = [0u8; HASH_BYTES_LEN];
        hasher.finalize(&mut output);
        let addr = Address::from(H256(output));

        Ok(LicenseInfo {
            lic_version: data_vec[0]
                .parse::<u32>()
                .map_err(|e| format!("Parse license version error {}", e))?,
            cita_version: data_vec[1].to_owned(),
            lic_type: LicenseType::from_u32(
                data_vec[2]
                    .parse::<u32>()
                    .map_err(|e| format!("Parse license type error {}", e))?,
            )?,
            lic_value: data_vec[3].to_owned(),
            begin_time: DateTime::parse_from_rfc3339(data_vec[4].to_string().as_ref())
                .map_err(|e| format!("Parse begin time error {}", e))?
                .with_timezone(&Utc),
            end_time: DateTime::parse_from_rfc3339(data_vec[5].to_string().as_ref())
                .map_err(|e| format!("Parse begin time error {}", e))?
                .with_timezone(&Utc),
            issuer: addr,
            finger_print: H256(finger_print),
        })
    }
}
