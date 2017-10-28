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

// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct VerifyTxReq {
    // message fields
    pub valid_until_block: u64,
    pub hash: ::std::vec::Vec<u8>,
    pub signature: ::std::vec::Vec<u8>,
    pub crypto: super::blockchain::Crypto,
    pub tx_hash: ::std::vec::Vec<u8>,
    pub signer: ::std::vec::Vec<u8>,
    pub nonce: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VerifyTxReq {}

impl VerifyTxReq {
    pub fn new() -> VerifyTxReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VerifyTxReq {
        static mut instance: ::protobuf::lazy::Lazy<VerifyTxReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VerifyTxReq,
        };
        unsafe {
            instance.get(VerifyTxReq::new)
        }
    }

    // uint64 valid_until_block = 1;

    pub fn clear_valid_until_block(&mut self) {
        self.valid_until_block = 0;
    }

    // Param is passed by value, moved
    pub fn set_valid_until_block(&mut self, v: u64) {
        self.valid_until_block = v;
    }

    pub fn get_valid_until_block(&self) -> u64 {
        self.valid_until_block
    }

    fn get_valid_until_block_for_reflect(&self) -> &u64 {
        &self.valid_until_block
    }

    fn mut_valid_until_block_for_reflect(&mut self) -> &mut u64 {
        &mut self.valid_until_block
    }

    // bytes hash = 2;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.hash, ::std::vec::Vec::new())
    }

    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }

    fn get_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // bytes signature = 3;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.signature
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.signature, ::std::vec::Vec::new())
    }

    pub fn get_signature(&self) -> &[u8] {
        &self.signature
    }

    fn get_signature_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.signature
    }

    // .Crypto crypto = 4;

    pub fn clear_crypto(&mut self) {
        self.crypto = super::blockchain::Crypto::SECP;
    }

    // Param is passed by value, moved
    pub fn set_crypto(&mut self, v: super::blockchain::Crypto) {
        self.crypto = v;
    }

    pub fn get_crypto(&self) -> super::blockchain::Crypto {
        self.crypto
    }

    fn get_crypto_for_reflect(&self) -> &super::blockchain::Crypto {
        &self.crypto
    }

    fn mut_crypto_for_reflect(&mut self) -> &mut super::blockchain::Crypto {
        &mut self.crypto
    }

    // bytes tx_hash = 5;

    pub fn clear_tx_hash(&mut self) {
        self.tx_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_tx_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.tx_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tx_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tx_hash
    }

    // Take field
    pub fn take_tx_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.tx_hash, ::std::vec::Vec::new())
    }

    pub fn get_tx_hash(&self) -> &[u8] {
        &self.tx_hash
    }

    fn get_tx_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.tx_hash
    }

    fn mut_tx_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tx_hash
    }

    // bytes signer = 6;

    pub fn clear_signer(&mut self) {
        self.signer.clear();
    }

    // Param is passed by value, moved
    pub fn set_signer(&mut self, v: ::std::vec::Vec<u8>) {
        self.signer = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signer(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.signer
    }

    // Take field
    pub fn take_signer(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.signer, ::std::vec::Vec::new())
    }

    pub fn get_signer(&self) -> &[u8] {
        &self.signer
    }

    fn get_signer_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.signer
    }

    fn mut_signer_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.signer
    }

    // string nonce = 7;

    pub fn clear_nonce(&mut self) {
        self.nonce.clear();
    }

    // Param is passed by value, moved
    pub fn set_nonce(&mut self, v: ::std::string::String) {
        self.nonce = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nonce(&mut self) -> &mut ::std::string::String {
        &mut self.nonce
    }

    // Take field
    pub fn take_nonce(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.nonce, ::std::string::String::new())
    }

    pub fn get_nonce(&self) -> &str {
        &self.nonce
    }

    fn get_nonce_for_reflect(&self) -> &::std::string::String {
        &self.nonce
    }

    fn mut_nonce_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.nonce
    }
}

impl ::protobuf::Message for VerifyTxReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.valid_until_block = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.hash)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.signature)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.crypto = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.tx_hash)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.signer)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.nonce)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.valid_until_block != 0 {
            my_size += ::protobuf::rt::value_size(1, self.valid_until_block, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.hash);
        }
        if !self.signature.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.signature);
        }
        if self.crypto != super::blockchain::Crypto::SECP {
            my_size += ::protobuf::rt::enum_size(4, self.crypto);
        }
        if !self.tx_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.tx_hash);
        }
        if !self.signer.is_empty() {
            my_size += ::protobuf::rt::bytes_size(6, &self.signer);
        }
        if !self.nonce.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.nonce);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.valid_until_block != 0 {
            os.write_uint64(1, self.valid_until_block)?;
        }
        if !self.hash.is_empty() {
            os.write_bytes(2, &self.hash)?;
        }
        if !self.signature.is_empty() {
            os.write_bytes(3, &self.signature)?;
        }
        if self.crypto != super::blockchain::Crypto::SECP {
            os.write_enum(4, self.crypto.value())?;
        }
        if !self.tx_hash.is_empty() {
            os.write_bytes(5, &self.tx_hash)?;
        }
        if !self.signer.is_empty() {
            os.write_bytes(6, &self.signer)?;
        }
        if !self.nonce.is_empty() {
            os.write_string(7, &self.nonce)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for VerifyTxReq {
    fn new() -> VerifyTxReq {
        VerifyTxReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<VerifyTxReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "valid_until_block",
                    VerifyTxReq::get_valid_until_block_for_reflect,
                    VerifyTxReq::mut_valid_until_block_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    VerifyTxReq::get_hash_for_reflect,
                    VerifyTxReq::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    VerifyTxReq::get_signature_for_reflect,
                    VerifyTxReq::mut_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::blockchain::Crypto>>(
                    "crypto",
                    VerifyTxReq::get_crypto_for_reflect,
                    VerifyTxReq::mut_crypto_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tx_hash",
                    VerifyTxReq::get_tx_hash_for_reflect,
                    VerifyTxReq::mut_tx_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signer",
                    VerifyTxReq::get_signer_for_reflect,
                    VerifyTxReq::mut_signer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "nonce",
                    VerifyTxReq::get_nonce_for_reflect,
                    VerifyTxReq::mut_nonce_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VerifyTxReq>(
                    "VerifyTxReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VerifyTxReq {
    fn clear(&mut self) {
        self.clear_valid_until_block();
        self.clear_hash();
        self.clear_signature();
        self.clear_crypto();
        self.clear_tx_hash();
        self.clear_signer();
        self.clear_nonce();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VerifyTxReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VerifyTxReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VerifyTxResp {
    // message fields
    pub tx_hash: ::std::vec::Vec<u8>,
    pub ret: Ret,
    pub signer: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VerifyTxResp {}

impl VerifyTxResp {
    pub fn new() -> VerifyTxResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VerifyTxResp {
        static mut instance: ::protobuf::lazy::Lazy<VerifyTxResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VerifyTxResp,
        };
        unsafe {
            instance.get(VerifyTxResp::new)
        }
    }

    // bytes tx_hash = 1;

    pub fn clear_tx_hash(&mut self) {
        self.tx_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_tx_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.tx_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tx_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tx_hash
    }

    // Take field
    pub fn take_tx_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.tx_hash, ::std::vec::Vec::new())
    }

    pub fn get_tx_hash(&self) -> &[u8] {
        &self.tx_hash
    }

    fn get_tx_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.tx_hash
    }

    fn mut_tx_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tx_hash
    }

    // .Ret ret = 2;

    pub fn clear_ret(&mut self) {
        self.ret = Ret::Ok;
    }

    // Param is passed by value, moved
    pub fn set_ret(&mut self, v: Ret) {
        self.ret = v;
    }

    pub fn get_ret(&self) -> Ret {
        self.ret
    }

    fn get_ret_for_reflect(&self) -> &Ret {
        &self.ret
    }

    fn mut_ret_for_reflect(&mut self) -> &mut Ret {
        &mut self.ret
    }

    // bytes signer = 3;

    pub fn clear_signer(&mut self) {
        self.signer.clear();
    }

    // Param is passed by value, moved
    pub fn set_signer(&mut self, v: ::std::vec::Vec<u8>) {
        self.signer = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signer(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.signer
    }

    // Take field
    pub fn take_signer(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.signer, ::std::vec::Vec::new())
    }

    pub fn get_signer(&self) -> &[u8] {
        &self.signer
    }

    fn get_signer_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.signer
    }

    fn mut_signer_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.signer
    }
}

impl ::protobuf::Message for VerifyTxResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.tx_hash)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.ret = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.signer)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.tx_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.tx_hash);
        }
        if self.ret != Ret::Ok {
            my_size += ::protobuf::rt::enum_size(2, self.ret);
        }
        if !self.signer.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.signer);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.tx_hash.is_empty() {
            os.write_bytes(1, &self.tx_hash)?;
        }
        if self.ret != Ret::Ok {
            os.write_enum(2, self.ret.value())?;
        }
        if !self.signer.is_empty() {
            os.write_bytes(3, &self.signer)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for VerifyTxResp {
    fn new() -> VerifyTxResp {
        VerifyTxResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<VerifyTxResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tx_hash",
                    VerifyTxResp::get_tx_hash_for_reflect,
                    VerifyTxResp::mut_tx_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Ret>>(
                    "ret",
                    VerifyTxResp::get_ret_for_reflect,
                    VerifyTxResp::mut_ret_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signer",
                    VerifyTxResp::get_signer_for_reflect,
                    VerifyTxResp::mut_signer_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VerifyTxResp>(
                    "VerifyTxResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VerifyTxResp {
    fn clear(&mut self) {
        self.clear_tx_hash();
        self.clear_ret();
        self.clear_signer();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VerifyTxResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VerifyTxResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VerifyBlockReq {
    // message fields
    pub id: u64,
    pub reqs: ::protobuf::RepeatedField<VerifyTxReq>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VerifyBlockReq {}

impl VerifyBlockReq {
    pub fn new() -> VerifyBlockReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VerifyBlockReq {
        static mut instance: ::protobuf::lazy::Lazy<VerifyBlockReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VerifyBlockReq,
        };
        unsafe {
            instance.get(VerifyBlockReq::new)
        }
    }

    // uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u64 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.id
    }

    // repeated .VerifyTxReq reqs = 2;

    pub fn clear_reqs(&mut self) {
        self.reqs.clear();
    }

    // Param is passed by value, moved
    pub fn set_reqs(&mut self, v: ::protobuf::RepeatedField<VerifyTxReq>) {
        self.reqs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_reqs(&mut self) -> &mut ::protobuf::RepeatedField<VerifyTxReq> {
        &mut self.reqs
    }

    // Take field
    pub fn take_reqs(&mut self) -> ::protobuf::RepeatedField<VerifyTxReq> {
        ::std::mem::replace(&mut self.reqs, ::protobuf::RepeatedField::new())
    }

    pub fn get_reqs(&self) -> &[VerifyTxReq] {
        &self.reqs
    }

    fn get_reqs_for_reflect(&self) -> &::protobuf::RepeatedField<VerifyTxReq> {
        &self.reqs
    }

    fn mut_reqs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<VerifyTxReq> {
        &mut self.reqs
    }
}

impl ::protobuf::Message for VerifyBlockReq {
    fn is_initialized(&self) -> bool {
        for v in &self.reqs {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.reqs)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.reqs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_uint64(1, self.id)?;
        }
        for v in &self.reqs {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for VerifyBlockReq {
    fn new() -> VerifyBlockReq {
        VerifyBlockReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<VerifyBlockReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    VerifyBlockReq::get_id_for_reflect,
                    VerifyBlockReq::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VerifyTxReq>>(
                    "reqs",
                    VerifyBlockReq::get_reqs_for_reflect,
                    VerifyBlockReq::mut_reqs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VerifyBlockReq>(
                    "VerifyBlockReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VerifyBlockReq {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_reqs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VerifyBlockReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VerifyBlockReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VerifyBlockResp {
    // message fields
    pub id: u64,
    pub ret: Ret,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VerifyBlockResp {}

impl VerifyBlockResp {
    pub fn new() -> VerifyBlockResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VerifyBlockResp {
        static mut instance: ::protobuf::lazy::Lazy<VerifyBlockResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VerifyBlockResp,
        };
        unsafe {
            instance.get(VerifyBlockResp::new)
        }
    }

    // uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u64 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.id
    }

    // .Ret ret = 2;

    pub fn clear_ret(&mut self) {
        self.ret = Ret::Ok;
    }

    // Param is passed by value, moved
    pub fn set_ret(&mut self, v: Ret) {
        self.ret = v;
    }

    pub fn get_ret(&self) -> Ret {
        self.ret
    }

    fn get_ret_for_reflect(&self) -> &Ret {
        &self.ret
    }

    fn mut_ret_for_reflect(&mut self) -> &mut Ret {
        &mut self.ret
    }
}

impl ::protobuf::Message for VerifyBlockResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.ret = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.ret != Ret::Ok {
            my_size += ::protobuf::rt::enum_size(2, self.ret);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_uint64(1, self.id)?;
        }
        if self.ret != Ret::Ok {
            os.write_enum(2, self.ret.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for VerifyBlockResp {
    fn new() -> VerifyBlockResp {
        VerifyBlockResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<VerifyBlockResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    VerifyBlockResp::get_id_for_reflect,
                    VerifyBlockResp::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Ret>>(
                    "ret",
                    VerifyBlockResp::get_ret_for_reflect,
                    VerifyBlockResp::mut_ret_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VerifyBlockResp>(
                    "VerifyBlockResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VerifyBlockResp {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_ret();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VerifyBlockResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VerifyBlockResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockTxHashes {
    // message fields
    pub height: u64,
    pub tx_hashes: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    pub block_gas_limit: u64,
    pub account_gas_limit: ::protobuf::SingularPtrField<super::blockchain::AccountGasLimit>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockTxHashes {}

impl BlockTxHashes {
    pub fn new() -> BlockTxHashes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockTxHashes {
        static mut instance: ::protobuf::lazy::Lazy<BlockTxHashes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockTxHashes,
        };
        unsafe {
            instance.get(BlockTxHashes::new)
        }
    }

    // uint64 height = 1;

    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u64) {
        self.height = v;
    }

    pub fn get_height(&self) -> u64 {
        self.height
    }

    fn get_height_for_reflect(&self) -> &u64 {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut u64 {
        &mut self.height
    }

    // repeated bytes tx_hashes = 2;

    pub fn clear_tx_hashes(&mut self) {
        self.tx_hashes.clear();
    }

    // Param is passed by value, moved
    pub fn set_tx_hashes(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.tx_hashes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tx_hashes(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.tx_hashes
    }

    // Take field
    pub fn take_tx_hashes(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.tx_hashes, ::protobuf::RepeatedField::new())
    }

    pub fn get_tx_hashes(&self) -> &[::std::vec::Vec<u8>] {
        &self.tx_hashes
    }

    fn get_tx_hashes_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.tx_hashes
    }

    fn mut_tx_hashes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.tx_hashes
    }

    // uint64 block_gas_limit = 3;

    pub fn clear_block_gas_limit(&mut self) {
        self.block_gas_limit = 0;
    }

    // Param is passed by value, moved
    pub fn set_block_gas_limit(&mut self, v: u64) {
        self.block_gas_limit = v;
    }

    pub fn get_block_gas_limit(&self) -> u64 {
        self.block_gas_limit
    }

    fn get_block_gas_limit_for_reflect(&self) -> &u64 {
        &self.block_gas_limit
    }

    fn mut_block_gas_limit_for_reflect(&mut self) -> &mut u64 {
        &mut self.block_gas_limit
    }

    // .AccountGasLimit account_gas_limit = 4;

    pub fn clear_account_gas_limit(&mut self) {
        self.account_gas_limit.clear();
    }

    pub fn has_account_gas_limit(&self) -> bool {
        self.account_gas_limit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_gas_limit(&mut self, v: super::blockchain::AccountGasLimit) {
        self.account_gas_limit = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_account_gas_limit(&mut self) -> &mut super::blockchain::AccountGasLimit {
        if self.account_gas_limit.is_none() {
            self.account_gas_limit.set_default();
        }
        self.account_gas_limit.as_mut().unwrap()
    }

    // Take field
    pub fn take_account_gas_limit(&mut self) -> super::blockchain::AccountGasLimit {
        self.account_gas_limit.take().unwrap_or_else(|| super::blockchain::AccountGasLimit::new())
    }

    pub fn get_account_gas_limit(&self) -> &super::blockchain::AccountGasLimit {
        self.account_gas_limit.as_ref().unwrap_or_else(|| super::blockchain::AccountGasLimit::default_instance())
    }

    fn get_account_gas_limit_for_reflect(&self) -> &::protobuf::SingularPtrField<super::blockchain::AccountGasLimit> {
        &self.account_gas_limit
    }

    fn mut_account_gas_limit_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::blockchain::AccountGasLimit> {
        &mut self.account_gas_limit
    }
}

impl ::protobuf::Message for BlockTxHashes {
    fn is_initialized(&self) -> bool {
        for v in &self.account_gas_limit {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.height = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.tx_hashes)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.block_gas_limit = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.account_gas_limit)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(1, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.tx_hashes {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        if self.block_gas_limit != 0 {
            my_size += ::protobuf::rt::value_size(3, self.block_gas_limit, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.account_gas_limit.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.height != 0 {
            os.write_uint64(1, self.height)?;
        }
        for v in &self.tx_hashes {
            os.write_bytes(2, &v)?;
        };
        if self.block_gas_limit != 0 {
            os.write_uint64(3, self.block_gas_limit)?;
        }
        if let Some(ref v) = self.account_gas_limit.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BlockTxHashes {
    fn new() -> BlockTxHashes {
        BlockTxHashes::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockTxHashes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    BlockTxHashes::get_height_for_reflect,
                    BlockTxHashes::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tx_hashes",
                    BlockTxHashes::get_tx_hashes_for_reflect,
                    BlockTxHashes::mut_tx_hashes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "block_gas_limit",
                    BlockTxHashes::get_block_gas_limit_for_reflect,
                    BlockTxHashes::mut_block_gas_limit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockchain::AccountGasLimit>>(
                    "account_gas_limit",
                    BlockTxHashes::get_account_gas_limit_for_reflect,
                    BlockTxHashes::mut_account_gas_limit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockTxHashes>(
                    "BlockTxHashes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockTxHashes {
    fn clear(&mut self) {
        self.clear_height();
        self.clear_tx_hashes();
        self.clear_block_gas_limit();
        self.clear_account_gas_limit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockTxHashes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockTxHashes {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockTxHashesReq {
    // message fields
    pub height: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockTxHashesReq {}

impl BlockTxHashesReq {
    pub fn new() -> BlockTxHashesReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockTxHashesReq {
        static mut instance: ::protobuf::lazy::Lazy<BlockTxHashesReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockTxHashesReq,
        };
        unsafe {
            instance.get(BlockTxHashesReq::new)
        }
    }

    // uint64 height = 1;

    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u64) {
        self.height = v;
    }

    pub fn get_height(&self) -> u64 {
        self.height
    }

    fn get_height_for_reflect(&self) -> &u64 {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut u64 {
        &mut self.height
    }
}

impl ::protobuf::Message for BlockTxHashesReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.height = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(1, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.height != 0 {
            os.write_uint64(1, self.height)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BlockTxHashesReq {
    fn new() -> BlockTxHashesReq {
        BlockTxHashesReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockTxHashesReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    BlockTxHashesReq::get_height_for_reflect,
                    BlockTxHashesReq::mut_height_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockTxHashesReq>(
                    "BlockTxHashesReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockTxHashesReq {
    fn clear(&mut self) {
        self.clear_height();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockTxHashesReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockTxHashesReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Ret {
    Ok = 0,
    InvalidNonce = 1,
    Dup = 2,
    InvalidUntilBlock = 3,
    BadSig = 4,
    NotReady = 5,
    Busy = 6,
}

impl ::protobuf::ProtobufEnum for Ret {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Ret> {
        match value {
            0 => ::std::option::Option::Some(Ret::Ok),
            1 => ::std::option::Option::Some(Ret::InvalidNonce),
            2 => ::std::option::Option::Some(Ret::Dup),
            3 => ::std::option::Option::Some(Ret::InvalidUntilBlock),
            4 => ::std::option::Option::Some(Ret::BadSig),
            5 => ::std::option::Option::Some(Ret::NotReady),
            6 => ::std::option::Option::Some(Ret::Busy),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Ret] = &[
            Ret::Ok,
            Ret::InvalidNonce,
            Ret::Dup,
            Ret::InvalidUntilBlock,
            Ret::BadSig,
            Ret::NotReady,
            Ret::Busy,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Ret>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Ret", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Ret {
}

impl ::std::default::Default for Ret {
    fn default() -> Self {
        Ret::Ok
    }
}

impl ::protobuf::reflect::ProtobufValue for Ret {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nauth.proto\x1a\x10blockchain.proto\"\xd3\x01\n\x0bVerifyTxReq\x12*\n\
    \x11valid_until_block\x18\x01\x20\x01(\x04R\x0fvalidUntilBlock\x12\x12\n\
    \x04hash\x18\x02\x20\x01(\x0cR\x04hash\x12\x1c\n\tsignature\x18\x03\x20\
    \x01(\x0cR\tsignature\x12\x1f\n\x06crypto\x18\x04\x20\x01(\x0e2\x07.Cryp\
    toR\x06crypto\x12\x17\n\x07tx_hash\x18\x05\x20\x01(\x0cR\x06txHash\x12\
    \x16\n\x06signer\x18\x06\x20\x01(\x0cR\x06signer\x12\x14\n\x05nonce\x18\
    \x07\x20\x01(\tR\x05nonce\"W\n\x0cVerifyTxResp\x12\x17\n\x07tx_hash\x18\
    \x01\x20\x01(\x0cR\x06txHash\x12\x16\n\x03ret\x18\x02\x20\x01(\x0e2\x04.\
    RetR\x03ret\x12\x16\n\x06signer\x18\x03\x20\x01(\x0cR\x06signer\"B\n\x0e\
    VerifyBlockReq\x12\x0e\n\x02id\x18\x01\x20\x01(\x04R\x02id\x12\x20\n\x04\
    reqs\x18\x02\x20\x03(\x0b2\x0c.VerifyTxReqR\x04reqs\"9\n\x0fVerifyBlockR\
    esp\x12\x0e\n\x02id\x18\x01\x20\x01(\x04R\x02id\x12\x16\n\x03ret\x18\x02\
    \x20\x01(\x0e2\x04.RetR\x03ret\"\xaa\x01\n\rBlockTxHashes\x12\x16\n\x06h\
    eight\x18\x01\x20\x01(\x04R\x06height\x12\x1b\n\ttx_hashes\x18\x02\x20\
    \x03(\x0cR\x08txHashes\x12&\n\x0fblock_gas_limit\x18\x03\x20\x01(\x04R\r\
    blockGasLimit\x12<\n\x11account_gas_limit\x18\x04\x20\x01(\x0b2\x10.Acco\
    untGasLimitR\x0faccountGasLimit\"*\n\x10BlockTxHashesReq\x12\x16\n\x06he\
    ight\x18\x01\x20\x01(\x04R\x06height*c\n\x03Ret\x12\x06\n\x02Ok\x10\0\
    \x12\x10\n\x0cInvalidNonce\x10\x01\x12\x07\n\x03Dup\x10\x02\x12\x15\n\
    \x11InvalidUntilBlock\x10\x03\x12\n\n\x06BadSig\x10\x04\x12\x0c\n\x08Not\
    Ready\x10\x05\x12\x08\n\x04Busy\x10\x06J\xd6\x0e\n\x06\x12\x04\0\01\x01\
    \n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\x07\x19\n\n\
    \n\x02\x05\0\x12\x04\x04\0\x0c\x01\n\n\n\x03\x05\0\x01\x12\x03\x04\x05\
    \x08\n\x0b\n\x04\x05\0\x02\0\x12\x03\x05\x04\x0b\n\x0c\n\x05\x05\0\x02\0\
    \x01\x12\x03\x05\x04\x06\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x05\t\n\n\
    \x0b\n\x04\x05\0\x02\x01\x12\x03\x06\x04\x15\n\x0c\n\x05\x05\0\x02\x01\
    \x01\x12\x03\x06\x04\x10\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x06\x13\
    \x14\n\x0b\n\x04\x05\0\x02\x02\x12\x03\x07\x04\x0c\n\x0c\n\x05\x05\0\x02\
    \x02\x01\x12\x03\x07\x04\x07\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x07\n\
    \x0b\n\x0b\n\x04\x05\0\x02\x03\x12\x03\x08\x04\x1a\n\x0c\n\x05\x05\0\x02\
    \x03\x01\x12\x03\x08\x04\x15\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03\x08\
    \x18\x19\n\x0b\n\x04\x05\0\x02\x04\x12\x03\t\x04\x0f\n\x0c\n\x05\x05\0\
    \x02\x04\x01\x12\x03\t\x04\n\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\t\r\
    \x0e\n\x0b\n\x04\x05\0\x02\x05\x12\x03\n\x04\x11\n\x0c\n\x05\x05\0\x02\
    \x05\x01\x12\x03\n\x04\x0c\n\x0c\n\x05\x05\0\x02\x05\x02\x12\x03\n\x0f\
    \x10\n\x0b\n\x04\x05\0\x02\x06\x12\x03\x0b\x04\r\n\x0c\n\x05\x05\0\x02\
    \x06\x01\x12\x03\x0b\x04\x08\n\x0c\n\x05\x05\0\x02\x06\x02\x12\x03\x0b\
    \x0b\x0c\n\n\n\x02\x04\0\x12\x04\x0e\0\x16\x01\n\n\n\x03\x04\0\x01\x12\
    \x03\x0e\x08\x13\n\x0b\n\x04\x04\0\x02\0\x12\x03\x0f\x04!\n\r\n\x05\x04\
    \0\x02\0\x04\x12\x04\x0f\x04\x0e\x15\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\
    \x0f\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0f\x0b\x1c\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\x0f\x1f\x20\n\x0b\n\x04\x04\0\x02\x01\x12\x03\
    \x10\x04\x13\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x10\x04\x0f!\n\x0c\n\
    \x05\x04\0\x02\x01\x05\x12\x03\x10\x04\t\n\x0c\n\x05\x04\0\x02\x01\x01\
    \x12\x03\x10\n\x0e\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x10\x11\x12\n\
    \x0b\n\x04\x04\0\x02\x02\x12\x03\x11\x04\x18\n\r\n\x05\x04\0\x02\x02\x04\
    \x12\x04\x11\x04\x10\x13\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x11\x04\t\
    \n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x11\n\x13\n\x0c\n\x05\x04\0\x02\
    \x02\x03\x12\x03\x11\x16\x17\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x12\x04\
    \x16\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\x12\x04\x11\x18\n\x0c\n\x05\x04\
    \0\x02\x03\x06\x12\x03\x12\x04\n\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\
    \x12\x0b\x11\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x12\x14\x15\n\x0b\n\
    \x04\x04\0\x02\x04\x12\x03\x13\x04\x16\n\r\n\x05\x04\0\x02\x04\x04\x12\
    \x04\x13\x04\x12\x16\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\x13\x04\t\n\
    \x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x13\n\x11\n\x0c\n\x05\x04\0\x02\x04\
    \x03\x12\x03\x13\x14\x15\n-\n\x04\x04\0\x02\x05\x12\x03\x14\x04\x15\"\
    \x20public\x20key\x20only\x20set\x20in\x20BlockReq\n\n\r\n\x05\x04\0\x02\
    \x05\x04\x12\x04\x14\x04\x13\x16\n\x0c\n\x05\x04\0\x02\x05\x05\x12\x03\
    \x14\x04\t\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03\x14\n\x10\n\x0c\n\x05\
    \x04\0\x02\x05\x03\x12\x03\x14\x13\x14\n\x0b\n\x04\x04\0\x02\x06\x12\x03\
    \x15\x04\x15\n\r\n\x05\x04\0\x02\x06\x04\x12\x04\x15\x04\x14\x15\n\x0c\n\
    \x05\x04\0\x02\x06\x05\x12\x03\x15\x04\n\n\x0c\n\x05\x04\0\x02\x06\x01\
    \x12\x03\x15\x0b\x10\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03\x15\x13\x14\n\
    \n\n\x02\x04\x01\x12\x04\x18\0\x1c\x01\n\n\n\x03\x04\x01\x01\x12\x03\x18\
    \x08\x14\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x19\x04\x16\n\r\n\x05\x04\x01\
    \x02\0\x04\x12\x04\x19\x04\x18\x16\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\
    \x19\x04\t\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x19\n\x11\n\x0c\n\x05\
    \x04\x01\x02\0\x03\x12\x03\x19\x14\x15\n\x0b\n\x04\x04\x01\x02\x01\x12\
    \x03\x1a\x04\x10\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x1a\x04\x19\x16\n\
    \x0c\n\x05\x04\x01\x02\x01\x06\x12\x03\x1a\x04\x07\n\x0c\n\x05\x04\x01\
    \x02\x01\x01\x12\x03\x1a\x08\x0b\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\
    \x1a\x0e\x0f\n+\n\x04\x04\x01\x02\x02\x12\x03\x1b\x04\x15\"\x1epublic\
    \x20key\x20only\x20set\x20in\x20TxResp\n\n\r\n\x05\x04\x01\x02\x02\x04\
    \x12\x04\x1b\x04\x1a\x10\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\x1b\x04\
    \t\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x1b\n\x10\n\x0c\n\x05\x04\x01\
    \x02\x02\x03\x12\x03\x1b\x13\x14\n\n\n\x02\x04\x02\x12\x04\x1e\0!\x01\n\
    \n\n\x03\x04\x02\x01\x12\x03\x1e\x08\x16\n\x0b\n\x04\x04\x02\x02\0\x12\
    \x03\x1f\x04\x12\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x1f\x04\x1e\x18\n\
    \x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x1f\x04\n\n\x0c\n\x05\x04\x02\x02\0\
    \x01\x12\x03\x1f\x0b\r\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x1f\x10\x11\
    \n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x20\x04\"\n\x0c\n\x05\x04\x02\x02\
    \x01\x04\x12\x03\x20\x04\x0c\n\x0c\n\x05\x04\x02\x02\x01\x06\x12\x03\x20\
    \r\x18\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x20\x19\x1d\n\x0c\n\x05\
    \x04\x02\x02\x01\x03\x12\x03\x20\x20!\n\n\n\x02\x04\x03\x12\x04#\0&\x01\
    \n\n\n\x03\x04\x03\x01\x12\x03#\x08\x17\n\x0b\n\x04\x04\x03\x02\0\x12\
    \x03$\x04\x12\n\r\n\x05\x04\x03\x02\0\x04\x12\x04$\x04#\x19\n\x0c\n\x05\
    \x04\x03\x02\0\x05\x12\x03$\x04\n\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03$\
    \x0b\r\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03$\x10\x11\n\x0b\n\x04\x04\
    \x03\x02\x01\x12\x03%\x04\x10\n\r\n\x05\x04\x03\x02\x01\x04\x12\x04%\x04\
    $\x12\n\x0c\n\x05\x04\x03\x02\x01\x06\x12\x03%\x04\x07\n\x0c\n\x05\x04\
    \x03\x02\x01\x01\x12\x03%\x08\x0b\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\
    \x03%\x0e\x0f\n\n\n\x02\x04\x04\x12\x04(\0-\x01\n\n\n\x03\x04\x04\x01\
    \x12\x03(\x08\x15\n\x0b\n\x04\x04\x04\x02\0\x12\x03)\x04\x16\n\r\n\x05\
    \x04\x04\x02\0\x04\x12\x04)\x04(\x17\n\x0c\n\x05\x04\x04\x02\0\x05\x12\
    \x03)\x04\n\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03)\x0b\x11\n\x0c\n\x05\
    \x04\x04\x02\0\x03\x12\x03)\x14\x15\n\x0b\n\x04\x04\x04\x02\x01\x12\x03*\
    \x04!\n\x0c\n\x05\x04\x04\x02\x01\x04\x12\x03*\x04\x0c\n\x0c\n\x05\x04\
    \x04\x02\x01\x05\x12\x03*\r\x12\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03*\
    \x13\x1c\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03*\x1f\x20\n\x0b\n\x04\
    \x04\x04\x02\x02\x12\x03+\x04\x1f\n\r\n\x05\x04\x04\x02\x02\x04\x12\x04+\
    \x04*!\n\x0c\n\x05\x04\x04\x02\x02\x05\x12\x03+\x04\n\n\x0c\n\x05\x04\
    \x04\x02\x02\x01\x12\x03+\x0b\x1a\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\
    \x03+\x1d\x1e\n\x0b\n\x04\x04\x04\x02\x03\x12\x03,\x04*\n\r\n\x05\x04\
    \x04\x02\x03\x04\x12\x04,\x04+\x1f\n\x0c\n\x05\x04\x04\x02\x03\x06\x12\
    \x03,\x04\x13\n\x0c\n\x05\x04\x04\x02\x03\x01\x12\x03,\x14%\n\x0c\n\x05\
    \x04\x04\x02\x03\x03\x12\x03,()\n\n\n\x02\x04\x05\x12\x04/\01\x01\n\n\n\
    \x03\x04\x05\x01\x12\x03/\x08\x18\n\x0b\n\x04\x04\x05\x02\0\x12\x030\x04\
    \x16\n\r\n\x05\x04\x05\x02\0\x04\x12\x040\x04/\x1a\n\x0c\n\x05\x04\x05\
    \x02\0\x05\x12\x030\x04\n\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x030\x0b\x11\
    \n\x0c\n\x05\x04\x05\x02\0\x03\x12\x030\x14\x15b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
