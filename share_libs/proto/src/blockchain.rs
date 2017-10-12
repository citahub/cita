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
pub struct Proof {
    // message fields
    pub content: ::std::vec::Vec<u8>,
    pub field_type: ProofType,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Proof {}

impl Proof {
    pub fn new() -> Proof {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Proof {
        static mut instance: ::protobuf::lazy::Lazy<Proof> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Proof,
        };
        unsafe {
            instance.get(Proof::new)
        }
    }

    // bytes content = 1;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::vec::Vec<u8>) {
        self.content = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.content, ::std::vec::Vec::new())
    }

    pub fn get_content(&self) -> &[u8] {
        &self.content
    }

    fn get_content_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.content
    }

    // .ProofType type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = ProofType::AuthorityRound;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ProofType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> ProofType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &ProofType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ProofType {
        &mut self.field_type
    }
}

impl ::protobuf::Message for Proof {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.content)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
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
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.content);
        }
        if self.field_type != ProofType::AuthorityRound {
            my_size += ::protobuf::rt::enum_size(2, self.field_type);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.content.is_empty() {
            os.write_bytes(1, &self.content)?;
        }
        if self.field_type != ProofType::AuthorityRound {
            os.write_enum(2, self.field_type.value())?;
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

impl ::protobuf::MessageStatic for Proof {
    fn new() -> Proof {
        Proof::new()
    }

    fn descriptor_static(_: ::std::option::Option<Proof>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "content",
                    Proof::get_content_for_reflect,
                    Proof::mut_content_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ProofType>>(
                    "type",
                    Proof::get_field_type_for_reflect,
                    Proof::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Proof>(
                    "Proof",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Proof {
    fn clear(&mut self) {
        self.clear_content();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Proof {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Proof {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockHeader {
    // message fields
    pub prevhash: ::std::vec::Vec<u8>,
    pub timestamp: u64,
    pub height: u64,
    pub state_root: ::std::vec::Vec<u8>,
    pub transactions_root: ::std::vec::Vec<u8>,
    pub receipts_root: ::std::vec::Vec<u8>,
    pub gas_used: u64,
    pub gas_limit: u64,
    pub proof: ::protobuf::SingularPtrField<Proof>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockHeader {}

impl BlockHeader {
    pub fn new() -> BlockHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockHeader {
        static mut instance: ::protobuf::lazy::Lazy<BlockHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockHeader,
        };
        unsafe {
            instance.get(BlockHeader::new)
        }
    }

    // bytes prevhash = 1;

    pub fn clear_prevhash(&mut self) {
        self.prevhash.clear();
    }

    // Param is passed by value, moved
    pub fn set_prevhash(&mut self, v: ::std::vec::Vec<u8>) {
        self.prevhash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prevhash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.prevhash
    }

    // Take field
    pub fn take_prevhash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.prevhash, ::std::vec::Vec::new())
    }

    pub fn get_prevhash(&self) -> &[u8] {
        &self.prevhash
    }

    fn get_prevhash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.prevhash
    }

    fn mut_prevhash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.prevhash
    }

    // uint64 timestamp = 2;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = v;
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    fn get_timestamp_for_reflect(&self) -> &u64 {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut u64 {
        &mut self.timestamp
    }

    // uint64 height = 3;

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

    // bytes state_root = 4;

    pub fn clear_state_root(&mut self) {
        self.state_root.clear();
    }

    // Param is passed by value, moved
    pub fn set_state_root(&mut self, v: ::std::vec::Vec<u8>) {
        self.state_root = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_state_root(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.state_root
    }

    // Take field
    pub fn take_state_root(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.state_root, ::std::vec::Vec::new())
    }

    pub fn get_state_root(&self) -> &[u8] {
        &self.state_root
    }

    fn get_state_root_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.state_root
    }

    fn mut_state_root_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.state_root
    }

    // bytes transactions_root = 5;

    pub fn clear_transactions_root(&mut self) {
        self.transactions_root.clear();
    }

    // Param is passed by value, moved
    pub fn set_transactions_root(&mut self, v: ::std::vec::Vec<u8>) {
        self.transactions_root = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transactions_root(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.transactions_root
    }

    // Take field
    pub fn take_transactions_root(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.transactions_root, ::std::vec::Vec::new())
    }

    pub fn get_transactions_root(&self) -> &[u8] {
        &self.transactions_root
    }

    fn get_transactions_root_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.transactions_root
    }

    fn mut_transactions_root_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.transactions_root
    }

    // bytes receipts_root = 6;

    pub fn clear_receipts_root(&mut self) {
        self.receipts_root.clear();
    }

    // Param is passed by value, moved
    pub fn set_receipts_root(&mut self, v: ::std::vec::Vec<u8>) {
        self.receipts_root = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_receipts_root(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.receipts_root
    }

    // Take field
    pub fn take_receipts_root(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.receipts_root, ::std::vec::Vec::new())
    }

    pub fn get_receipts_root(&self) -> &[u8] {
        &self.receipts_root
    }

    fn get_receipts_root_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.receipts_root
    }

    fn mut_receipts_root_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.receipts_root
    }

    // uint64 gas_used = 7;

    pub fn clear_gas_used(&mut self) {
        self.gas_used = 0;
    }

    // Param is passed by value, moved
    pub fn set_gas_used(&mut self, v: u64) {
        self.gas_used = v;
    }

    pub fn get_gas_used(&self) -> u64 {
        self.gas_used
    }

    fn get_gas_used_for_reflect(&self) -> &u64 {
        &self.gas_used
    }

    fn mut_gas_used_for_reflect(&mut self) -> &mut u64 {
        &mut self.gas_used
    }

    // uint64 gas_limit = 8;

    pub fn clear_gas_limit(&mut self) {
        self.gas_limit = 0;
    }

    // Param is passed by value, moved
    pub fn set_gas_limit(&mut self, v: u64) {
        self.gas_limit = v;
    }

    pub fn get_gas_limit(&self) -> u64 {
        self.gas_limit
    }

    fn get_gas_limit_for_reflect(&self) -> &u64 {
        &self.gas_limit
    }

    fn mut_gas_limit_for_reflect(&mut self) -> &mut u64 {
        &mut self.gas_limit
    }

    // .Proof proof = 9;

    pub fn clear_proof(&mut self) {
        self.proof.clear();
    }

    pub fn has_proof(&self) -> bool {
        self.proof.is_some()
    }

    // Param is passed by value, moved
    pub fn set_proof(&mut self, v: Proof) {
        self.proof = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_proof(&mut self) -> &mut Proof {
        if self.proof.is_none() {
            self.proof.set_default();
        }
        self.proof.as_mut().unwrap()
    }

    // Take field
    pub fn take_proof(&mut self) -> Proof {
        self.proof.take().unwrap_or_else(|| Proof::new())
    }

    pub fn get_proof(&self) -> &Proof {
        self.proof.as_ref().unwrap_or_else(|| Proof::default_instance())
    }

    fn get_proof_for_reflect(&self) -> &::protobuf::SingularPtrField<Proof> {
        &self.proof
    }

    fn mut_proof_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Proof> {
        &mut self.proof
    }
}

impl ::protobuf::Message for BlockHeader {
    fn is_initialized(&self) -> bool {
        for v in &self.proof {
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
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.prevhash)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timestamp = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.height = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.state_root)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.transactions_root)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.receipts_root)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.gas_used = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.gas_limit = tmp;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.proof)?;
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
        if !self.prevhash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.prevhash);
        }
        if self.timestamp != 0 {
            my_size += ::protobuf::rt::value_size(2, self.timestamp, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(3, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.state_root.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.state_root);
        }
        if !self.transactions_root.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.transactions_root);
        }
        if !self.receipts_root.is_empty() {
            my_size += ::protobuf::rt::bytes_size(6, &self.receipts_root);
        }
        if self.gas_used != 0 {
            my_size += ::protobuf::rt::value_size(7, self.gas_used, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.gas_limit != 0 {
            my_size += ::protobuf::rt::value_size(8, self.gas_limit, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.proof.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.prevhash.is_empty() {
            os.write_bytes(1, &self.prevhash)?;
        }
        if self.timestamp != 0 {
            os.write_uint64(2, self.timestamp)?;
        }
        if self.height != 0 {
            os.write_uint64(3, self.height)?;
        }
        if !self.state_root.is_empty() {
            os.write_bytes(4, &self.state_root)?;
        }
        if !self.transactions_root.is_empty() {
            os.write_bytes(5, &self.transactions_root)?;
        }
        if !self.receipts_root.is_empty() {
            os.write_bytes(6, &self.receipts_root)?;
        }
        if self.gas_used != 0 {
            os.write_uint64(7, self.gas_used)?;
        }
        if self.gas_limit != 0 {
            os.write_uint64(8, self.gas_limit)?;
        }
        if let Some(ref v) = self.proof.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for BlockHeader {
    fn new() -> BlockHeader {
        BlockHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "prevhash",
                    BlockHeader::get_prevhash_for_reflect,
                    BlockHeader::mut_prevhash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp",
                    BlockHeader::get_timestamp_for_reflect,
                    BlockHeader::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    BlockHeader::get_height_for_reflect,
                    BlockHeader::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "state_root",
                    BlockHeader::get_state_root_for_reflect,
                    BlockHeader::mut_state_root_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "transactions_root",
                    BlockHeader::get_transactions_root_for_reflect,
                    BlockHeader::mut_transactions_root_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "receipts_root",
                    BlockHeader::get_receipts_root_for_reflect,
                    BlockHeader::mut_receipts_root_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "gas_used",
                    BlockHeader::get_gas_used_for_reflect,
                    BlockHeader::mut_gas_used_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "gas_limit",
                    BlockHeader::get_gas_limit_for_reflect,
                    BlockHeader::mut_gas_limit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Proof>>(
                    "proof",
                    BlockHeader::get_proof_for_reflect,
                    BlockHeader::mut_proof_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockHeader>(
                    "BlockHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockHeader {
    fn clear(&mut self) {
        self.clear_prevhash();
        self.clear_timestamp();
        self.clear_height();
        self.clear_state_root();
        self.clear_transactions_root();
        self.clear_receipts_root();
        self.clear_gas_used();
        self.clear_gas_limit();
        self.clear_proof();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Status {
    // message fields
    pub hash: ::std::vec::Vec<u8>,
    pub height: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Status {}

impl Status {
    pub fn new() -> Status {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Status {
        static mut instance: ::protobuf::lazy::Lazy<Status> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Status,
        };
        unsafe {
            instance.get(Status::new)
        }
    }

    // bytes hash = 1;

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

    // uint64 height = 2;

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

impl ::protobuf::Message for Status {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.hash)?;
                },
                2 => {
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
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.hash);
        }
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(2, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.hash.is_empty() {
            os.write_bytes(1, &self.hash)?;
        }
        if self.height != 0 {
            os.write_uint64(2, self.height)?;
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

impl ::protobuf::MessageStatic for Status {
    fn new() -> Status {
        Status::new()
    }

    fn descriptor_static(_: ::std::option::Option<Status>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    Status::get_hash_for_reflect,
                    Status::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    Status::get_height_for_reflect,
                    Status::mut_height_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Status>(
                    "Status",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Status {
    fn clear(&mut self) {
        self.clear_hash();
        self.clear_height();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Status {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Status {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountGasLimit {
    // message fields
    pub common_gas_limit: u64,
    pub specific_gas_limit: ::std::collections::HashMap<::std::string::String, u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountGasLimit {}

impl AccountGasLimit {
    pub fn new() -> AccountGasLimit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountGasLimit {
        static mut instance: ::protobuf::lazy::Lazy<AccountGasLimit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountGasLimit,
        };
        unsafe {
            instance.get(AccountGasLimit::new)
        }
    }

    // uint64 common_gas_limit = 1;

    pub fn clear_common_gas_limit(&mut self) {
        self.common_gas_limit = 0;
    }

    // Param is passed by value, moved
    pub fn set_common_gas_limit(&mut self, v: u64) {
        self.common_gas_limit = v;
    }

    pub fn get_common_gas_limit(&self) -> u64 {
        self.common_gas_limit
    }

    fn get_common_gas_limit_for_reflect(&self) -> &u64 {
        &self.common_gas_limit
    }

    fn mut_common_gas_limit_for_reflect(&mut self) -> &mut u64 {
        &mut self.common_gas_limit
    }

    // repeated .AccountGasLimit.SpecificGasLimitEntry specific_gas_limit = 2;

    pub fn clear_specific_gas_limit(&mut self) {
        self.specific_gas_limit.clear();
    }

    // Param is passed by value, moved
    pub fn set_specific_gas_limit(&mut self, v: ::std::collections::HashMap<::std::string::String, u64>) {
        self.specific_gas_limit = v;
    }

    // Mutable pointer to the field.
    pub fn mut_specific_gas_limit(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, u64> {
        &mut self.specific_gas_limit
    }

    // Take field
    pub fn take_specific_gas_limit(&mut self) -> ::std::collections::HashMap<::std::string::String, u64> {
        ::std::mem::replace(&mut self.specific_gas_limit, ::std::collections::HashMap::new())
    }

    pub fn get_specific_gas_limit(&self) -> &::std::collections::HashMap<::std::string::String, u64> {
        &self.specific_gas_limit
    }

    fn get_specific_gas_limit_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, u64> {
        &self.specific_gas_limit
    }

    fn mut_specific_gas_limit_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, u64> {
        &mut self.specific_gas_limit
    }
}

impl ::protobuf::Message for AccountGasLimit {
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
                    self.common_gas_limit = tmp;
                },
                2 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeUint64>(wire_type, is, &mut self.specific_gas_limit)?;
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
        if self.common_gas_limit != 0 {
            my_size += ::protobuf::rt::value_size(1, self.common_gas_limit, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeUint64>(2, &self.specific_gas_limit);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.common_gas_limit != 0 {
            os.write_uint64(1, self.common_gas_limit)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeUint64>(2, &self.specific_gas_limit, os)?;
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

impl ::protobuf::MessageStatic for AccountGasLimit {
    fn new() -> AccountGasLimit {
        AccountGasLimit::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountGasLimit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "common_gas_limit",
                    AccountGasLimit::get_common_gas_limit_for_reflect,
                    AccountGasLimit::mut_common_gas_limit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeUint64>(
                    "specific_gas_limit",
                    AccountGasLimit::get_specific_gas_limit_for_reflect,
                    AccountGasLimit::mut_specific_gas_limit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountGasLimit>(
                    "AccountGasLimit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountGasLimit {
    fn clear(&mut self) {
        self.clear_common_gas_limit();
        self.clear_specific_gas_limit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountGasLimit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountGasLimit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RichStatus {
    // message fields
    pub hash: ::std::vec::Vec<u8>,
    pub height: u64,
    pub nodes: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RichStatus {}

impl RichStatus {
    pub fn new() -> RichStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RichStatus {
        static mut instance: ::protobuf::lazy::Lazy<RichStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RichStatus,
        };
        unsafe {
            instance.get(RichStatus::new)
        }
    }

    // bytes hash = 1;

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

    // uint64 height = 2;

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

    // repeated bytes nodes = 3;

    pub fn clear_nodes(&mut self) {
        self.nodes.clear();
    }

    // Param is passed by value, moved
    pub fn set_nodes(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.nodes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nodes(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.nodes
    }

    // Take field
    pub fn take_nodes(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.nodes, ::protobuf::RepeatedField::new())
    }

    pub fn get_nodes(&self) -> &[::std::vec::Vec<u8>] {
        &self.nodes
    }

    fn get_nodes_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.nodes
    }

    fn mut_nodes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.nodes
    }
}

impl ::protobuf::Message for RichStatus {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.hash)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.height = tmp;
                },
                3 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.nodes)?;
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
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.hash);
        }
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(2, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.nodes {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.hash.is_empty() {
            os.write_bytes(1, &self.hash)?;
        }
        if self.height != 0 {
            os.write_uint64(2, self.height)?;
        }
        for v in &self.nodes {
            os.write_bytes(3, &v)?;
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

impl ::protobuf::MessageStatic for RichStatus {
    fn new() -> RichStatus {
        RichStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<RichStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    RichStatus::get_hash_for_reflect,
                    RichStatus::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    RichStatus::get_height_for_reflect,
                    RichStatus::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "nodes",
                    RichStatus::get_nodes_for_reflect,
                    RichStatus::mut_nodes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RichStatus>(
                    "RichStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RichStatus {
    fn clear(&mut self) {
        self.clear_hash();
        self.clear_height();
        self.clear_nodes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RichStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RichStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Transaction {
    // message fields
    pub to: ::std::string::String,
    pub nonce: ::std::string::String,
    pub quota: u64,
    pub valid_until_block: u64,
    pub data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Transaction {}

impl Transaction {
    pub fn new() -> Transaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Transaction {
        static mut instance: ::protobuf::lazy::Lazy<Transaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Transaction,
        };
        unsafe {
            instance.get(Transaction::new)
        }
    }

    // string to = 1;

    pub fn clear_to(&mut self) {
        self.to.clear();
    }

    // Param is passed by value, moved
    pub fn set_to(&mut self, v: ::std::string::String) {
        self.to = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_to(&mut self) -> &mut ::std::string::String {
        &mut self.to
    }

    // Take field
    pub fn take_to(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.to, ::std::string::String::new())
    }

    pub fn get_to(&self) -> &str {
        &self.to
    }

    fn get_to_for_reflect(&self) -> &::std::string::String {
        &self.to
    }

    fn mut_to_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.to
    }

    // string nonce = 2;

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

    // uint64 quota = 3;

    pub fn clear_quota(&mut self) {
        self.quota = 0;
    }

    // Param is passed by value, moved
    pub fn set_quota(&mut self, v: u64) {
        self.quota = v;
    }

    pub fn get_quota(&self) -> u64 {
        self.quota
    }

    fn get_quota_for_reflect(&self) -> &u64 {
        &self.quota
    }

    fn mut_quota_for_reflect(&mut self) -> &mut u64 {
        &mut self.quota
    }

    // uint64 valid_until_block = 4;

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

    // bytes data = 5;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }
}

impl ::protobuf::Message for Transaction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.to)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.nonce)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.quota = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.valid_until_block = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
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
        if !self.to.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.to);
        }
        if !self.nonce.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.nonce);
        }
        if self.quota != 0 {
            my_size += ::protobuf::rt::value_size(3, self.quota, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.valid_until_block != 0 {
            my_size += ::protobuf::rt::value_size(4, self.valid_until_block, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.to.is_empty() {
            os.write_string(1, &self.to)?;
        }
        if !self.nonce.is_empty() {
            os.write_string(2, &self.nonce)?;
        }
        if self.quota != 0 {
            os.write_uint64(3, self.quota)?;
        }
        if self.valid_until_block != 0 {
            os.write_uint64(4, self.valid_until_block)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(5, &self.data)?;
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

impl ::protobuf::MessageStatic for Transaction {
    fn new() -> Transaction {
        Transaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<Transaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "to",
                    Transaction::get_to_for_reflect,
                    Transaction::mut_to_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "nonce",
                    Transaction::get_nonce_for_reflect,
                    Transaction::mut_nonce_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "quota",
                    Transaction::get_quota_for_reflect,
                    Transaction::mut_quota_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "valid_until_block",
                    Transaction::get_valid_until_block_for_reflect,
                    Transaction::mut_valid_until_block_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    Transaction::get_data_for_reflect,
                    Transaction::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Transaction>(
                    "Transaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Transaction {
    fn clear(&mut self) {
        self.clear_to();
        self.clear_nonce();
        self.clear_quota();
        self.clear_valid_until_block();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Transaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Transaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnverifiedTransaction {
    // message fields
    pub transaction: ::protobuf::SingularPtrField<Transaction>,
    pub signature: ::std::vec::Vec<u8>,
    pub crypto: Crypto,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnverifiedTransaction {}

impl UnverifiedTransaction {
    pub fn new() -> UnverifiedTransaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnverifiedTransaction {
        static mut instance: ::protobuf::lazy::Lazy<UnverifiedTransaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnverifiedTransaction,
        };
        unsafe {
            instance.get(UnverifiedTransaction::new)
        }
    }

    // .Transaction transaction = 1;

    pub fn clear_transaction(&mut self) {
        self.transaction.clear();
    }

    pub fn has_transaction(&self) -> bool {
        self.transaction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transaction(&mut self, v: Transaction) {
        self.transaction = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transaction(&mut self) -> &mut Transaction {
        if self.transaction.is_none() {
            self.transaction.set_default();
        }
        self.transaction.as_mut().unwrap()
    }

    // Take field
    pub fn take_transaction(&mut self) -> Transaction {
        self.transaction.take().unwrap_or_else(|| Transaction::new())
    }

    pub fn get_transaction(&self) -> &Transaction {
        self.transaction.as_ref().unwrap_or_else(|| Transaction::default_instance())
    }

    fn get_transaction_for_reflect(&self) -> &::protobuf::SingularPtrField<Transaction> {
        &self.transaction
    }

    fn mut_transaction_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Transaction> {
        &mut self.transaction
    }

    // bytes signature = 2;

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

    // .Crypto crypto = 3;

    pub fn clear_crypto(&mut self) {
        self.crypto = Crypto::SECP;
    }

    // Param is passed by value, moved
    pub fn set_crypto(&mut self, v: Crypto) {
        self.crypto = v;
    }

    pub fn get_crypto(&self) -> Crypto {
        self.crypto
    }

    fn get_crypto_for_reflect(&self) -> &Crypto {
        &self.crypto
    }

    fn mut_crypto_for_reflect(&mut self) -> &mut Crypto {
        &mut self.crypto
    }
}

impl ::protobuf::Message for UnverifiedTransaction {
    fn is_initialized(&self) -> bool {
        for v in &self.transaction {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.transaction)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.signature)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.crypto = tmp;
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
        if let Some(ref v) = self.transaction.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.signature.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.signature);
        }
        if self.crypto != Crypto::SECP {
            my_size += ::protobuf::rt::enum_size(3, self.crypto);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.transaction.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.signature.is_empty() {
            os.write_bytes(2, &self.signature)?;
        }
        if self.crypto != Crypto::SECP {
            os.write_enum(3, self.crypto.value())?;
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

impl ::protobuf::MessageStatic for UnverifiedTransaction {
    fn new() -> UnverifiedTransaction {
        UnverifiedTransaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnverifiedTransaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Transaction>>(
                    "transaction",
                    UnverifiedTransaction::get_transaction_for_reflect,
                    UnverifiedTransaction::mut_transaction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    UnverifiedTransaction::get_signature_for_reflect,
                    UnverifiedTransaction::mut_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Crypto>>(
                    "crypto",
                    UnverifiedTransaction::get_crypto_for_reflect,
                    UnverifiedTransaction::mut_crypto_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnverifiedTransaction>(
                    "UnverifiedTransaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnverifiedTransaction {
    fn clear(&mut self) {
        self.clear_transaction();
        self.clear_signature();
        self.clear_crypto();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnverifiedTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnverifiedTransaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SignedTransaction {
    // message fields
    pub transaction_with_sig: ::protobuf::SingularPtrField<UnverifiedTransaction>,
    pub tx_hash: ::std::vec::Vec<u8>,
    pub signer: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SignedTransaction {}

impl SignedTransaction {
    pub fn new() -> SignedTransaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SignedTransaction {
        static mut instance: ::protobuf::lazy::Lazy<SignedTransaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SignedTransaction,
        };
        unsafe {
            instance.get(SignedTransaction::new)
        }
    }

    // .UnverifiedTransaction transaction_with_sig = 1;

    pub fn clear_transaction_with_sig(&mut self) {
        self.transaction_with_sig.clear();
    }

    pub fn has_transaction_with_sig(&self) -> bool {
        self.transaction_with_sig.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transaction_with_sig(&mut self, v: UnverifiedTransaction) {
        self.transaction_with_sig = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transaction_with_sig(&mut self) -> &mut UnverifiedTransaction {
        if self.transaction_with_sig.is_none() {
            self.transaction_with_sig.set_default();
        }
        self.transaction_with_sig.as_mut().unwrap()
    }

    // Take field
    pub fn take_transaction_with_sig(&mut self) -> UnverifiedTransaction {
        self.transaction_with_sig.take().unwrap_or_else(|| UnverifiedTransaction::new())
    }

    pub fn get_transaction_with_sig(&self) -> &UnverifiedTransaction {
        self.transaction_with_sig.as_ref().unwrap_or_else(|| UnverifiedTransaction::default_instance())
    }

    fn get_transaction_with_sig_for_reflect(&self) -> &::protobuf::SingularPtrField<UnverifiedTransaction> {
        &self.transaction_with_sig
    }

    fn mut_transaction_with_sig_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<UnverifiedTransaction> {
        &mut self.transaction_with_sig
    }

    // bytes tx_hash = 2;

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

impl ::protobuf::Message for SignedTransaction {
    fn is_initialized(&self) -> bool {
        for v in &self.transaction_with_sig {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.transaction_with_sig)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.tx_hash)?;
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
        if let Some(ref v) = self.transaction_with_sig.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.tx_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.tx_hash);
        }
        if !self.signer.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.signer);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.transaction_with_sig.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.tx_hash.is_empty() {
            os.write_bytes(2, &self.tx_hash)?;
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

impl ::protobuf::MessageStatic for SignedTransaction {
    fn new() -> SignedTransaction {
        SignedTransaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<SignedTransaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UnverifiedTransaction>>(
                    "transaction_with_sig",
                    SignedTransaction::get_transaction_with_sig_for_reflect,
                    SignedTransaction::mut_transaction_with_sig_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tx_hash",
                    SignedTransaction::get_tx_hash_for_reflect,
                    SignedTransaction::mut_tx_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signer",
                    SignedTransaction::get_signer_for_reflect,
                    SignedTransaction::mut_signer_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SignedTransaction>(
                    "SignedTransaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SignedTransaction {
    fn clear(&mut self) {
        self.clear_transaction_with_sig();
        self.clear_tx_hash();
        self.clear_signer();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SignedTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SignedTransaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockBody {
    // message fields
    pub transactions: ::protobuf::RepeatedField<SignedTransaction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockBody {}

impl BlockBody {
    pub fn new() -> BlockBody {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockBody {
        static mut instance: ::protobuf::lazy::Lazy<BlockBody> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockBody,
        };
        unsafe {
            instance.get(BlockBody::new)
        }
    }

    // repeated .SignedTransaction transactions = 1;

    pub fn clear_transactions(&mut self) {
        self.transactions.clear();
    }

    // Param is passed by value, moved
    pub fn set_transactions(&mut self, v: ::protobuf::RepeatedField<SignedTransaction>) {
        self.transactions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_transactions(&mut self) -> &mut ::protobuf::RepeatedField<SignedTransaction> {
        &mut self.transactions
    }

    // Take field
    pub fn take_transactions(&mut self) -> ::protobuf::RepeatedField<SignedTransaction> {
        ::std::mem::replace(&mut self.transactions, ::protobuf::RepeatedField::new())
    }

    pub fn get_transactions(&self) -> &[SignedTransaction] {
        &self.transactions
    }

    fn get_transactions_for_reflect(&self) -> &::protobuf::RepeatedField<SignedTransaction> {
        &self.transactions
    }

    fn mut_transactions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<SignedTransaction> {
        &mut self.transactions
    }
}

impl ::protobuf::Message for BlockBody {
    fn is_initialized(&self) -> bool {
        for v in &self.transactions {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.transactions)?;
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
        for value in &self.transactions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.transactions {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for BlockBody {
    fn new() -> BlockBody {
        BlockBody::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockBody>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SignedTransaction>>(
                    "transactions",
                    BlockBody::get_transactions_for_reflect,
                    BlockBody::mut_transactions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockBody>(
                    "BlockBody",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockBody {
    fn clear(&mut self) {
        self.clear_transactions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockBody {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockBody {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Block {
    // message fields
    pub version: u32,
    pub header: ::protobuf::SingularPtrField<BlockHeader>,
    pub body: ::protobuf::SingularPtrField<BlockBody>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Block {}

impl Block {
    pub fn new() -> Block {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Block {
        static mut instance: ::protobuf::lazy::Lazy<Block> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Block,
        };
        unsafe {
            instance.get(Block::new)
        }
    }

    // uint32 version = 1;

    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u32) {
        self.version = v;
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    fn get_version_for_reflect(&self) -> &u32 {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut u32 {
        &mut self.version
    }

    // .BlockHeader header = 2;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: BlockHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut BlockHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> BlockHeader {
        self.header.take().unwrap_or_else(|| BlockHeader::new())
    }

    pub fn get_header(&self) -> &BlockHeader {
        self.header.as_ref().unwrap_or_else(|| BlockHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<BlockHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BlockHeader> {
        &mut self.header
    }

    // .BlockBody body = 3;

    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body(&mut self, v: BlockBody) {
        self.body = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body(&mut self) -> &mut BlockBody {
        if self.body.is_none() {
            self.body.set_default();
        }
        self.body.as_mut().unwrap()
    }

    // Take field
    pub fn take_body(&mut self) -> BlockBody {
        self.body.take().unwrap_or_else(|| BlockBody::new())
    }

    pub fn get_body(&self) -> &BlockBody {
        self.body.as_ref().unwrap_or_else(|| BlockBody::default_instance())
    }

    fn get_body_for_reflect(&self) -> &::protobuf::SingularPtrField<BlockBody> {
        &self.body
    }

    fn mut_body_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BlockBody> {
        &mut self.body
    }
}

impl ::protobuf::Message for Block {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.body {
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
                    let tmp = is.read_uint32()?;
                    self.version = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.body)?;
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
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(1, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.body.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.version != 0 {
            os.write_uint32(1, self.version)?;
        }
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.body.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Block {
    fn new() -> Block {
        Block::new()
    }

    fn descriptor_static(_: ::std::option::Option<Block>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "version",
                    Block::get_version_for_reflect,
                    Block::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockHeader>>(
                    "header",
                    Block::get_header_for_reflect,
                    Block::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockBody>>(
                    "body",
                    Block::get_body_for_reflect,
                    Block::mut_body_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Block>(
                    "Block",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Block {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_header();
        self.clear_body();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Block {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Block {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockWithProof {
    // message fields
    pub blk: ::protobuf::SingularPtrField<Block>,
    pub proof: ::protobuf::SingularPtrField<Proof>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockWithProof {}

impl BlockWithProof {
    pub fn new() -> BlockWithProof {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockWithProof {
        static mut instance: ::protobuf::lazy::Lazy<BlockWithProof> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockWithProof,
        };
        unsafe {
            instance.get(BlockWithProof::new)
        }
    }

    // .Block blk = 1;

    pub fn clear_blk(&mut self) {
        self.blk.clear();
    }

    pub fn has_blk(&self) -> bool {
        self.blk.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blk(&mut self, v: Block) {
        self.blk = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blk(&mut self) -> &mut Block {
        if self.blk.is_none() {
            self.blk.set_default();
        }
        self.blk.as_mut().unwrap()
    }

    // Take field
    pub fn take_blk(&mut self) -> Block {
        self.blk.take().unwrap_or_else(|| Block::new())
    }

    pub fn get_blk(&self) -> &Block {
        self.blk.as_ref().unwrap_or_else(|| Block::default_instance())
    }

    fn get_blk_for_reflect(&self) -> &::protobuf::SingularPtrField<Block> {
        &self.blk
    }

    fn mut_blk_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Block> {
        &mut self.blk
    }

    // .Proof proof = 2;

    pub fn clear_proof(&mut self) {
        self.proof.clear();
    }

    pub fn has_proof(&self) -> bool {
        self.proof.is_some()
    }

    // Param is passed by value, moved
    pub fn set_proof(&mut self, v: Proof) {
        self.proof = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_proof(&mut self) -> &mut Proof {
        if self.proof.is_none() {
            self.proof.set_default();
        }
        self.proof.as_mut().unwrap()
    }

    // Take field
    pub fn take_proof(&mut self) -> Proof {
        self.proof.take().unwrap_or_else(|| Proof::new())
    }

    pub fn get_proof(&self) -> &Proof {
        self.proof.as_ref().unwrap_or_else(|| Proof::default_instance())
    }

    fn get_proof_for_reflect(&self) -> &::protobuf::SingularPtrField<Proof> {
        &self.proof
    }

    fn mut_proof_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Proof> {
        &mut self.proof
    }
}

impl ::protobuf::Message for BlockWithProof {
    fn is_initialized(&self) -> bool {
        for v in &self.blk {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.proof {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.blk)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.proof)?;
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
        if let Some(ref v) = self.blk.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.proof.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.blk.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.proof.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for BlockWithProof {
    fn new() -> BlockWithProof {
        BlockWithProof::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockWithProof>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Block>>(
                    "blk",
                    BlockWithProof::get_blk_for_reflect,
                    BlockWithProof::mut_blk_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Proof>>(
                    "proof",
                    BlockWithProof::get_proof_for_reflect,
                    BlockWithProof::mut_proof_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockWithProof>(
                    "BlockWithProof",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockWithProof {
    fn clear(&mut self) {
        self.clear_blk();
        self.clear_proof();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockWithProof {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockWithProof {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockTxs {
    // message fields
    pub height: u64,
    pub body: ::protobuf::SingularPtrField<BlockBody>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockTxs {}

impl BlockTxs {
    pub fn new() -> BlockTxs {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockTxs {
        static mut instance: ::protobuf::lazy::Lazy<BlockTxs> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockTxs,
        };
        unsafe {
            instance.get(BlockTxs::new)
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

    // .BlockBody body = 3;

    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body(&mut self, v: BlockBody) {
        self.body = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body(&mut self) -> &mut BlockBody {
        if self.body.is_none() {
            self.body.set_default();
        }
        self.body.as_mut().unwrap()
    }

    // Take field
    pub fn take_body(&mut self) -> BlockBody {
        self.body.take().unwrap_or_else(|| BlockBody::new())
    }

    pub fn get_body(&self) -> &BlockBody {
        self.body.as_ref().unwrap_or_else(|| BlockBody::default_instance())
    }

    fn get_body_for_reflect(&self) -> &::protobuf::SingularPtrField<BlockBody> {
        &self.body
    }

    fn mut_body_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BlockBody> {
        &mut self.body
    }
}

impl ::protobuf::Message for BlockTxs {
    fn is_initialized(&self) -> bool {
        for v in &self.body {
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
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.body)?;
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
        if let Some(ref v) = self.body.as_ref() {
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
        if let Some(ref v) = self.body.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for BlockTxs {
    fn new() -> BlockTxs {
        BlockTxs::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockTxs>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    BlockTxs::get_height_for_reflect,
                    BlockTxs::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockBody>>(
                    "body",
                    BlockTxs::get_body_for_reflect,
                    BlockTxs::mut_body_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockTxs>(
                    "BlockTxs",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockTxs {
    fn clear(&mut self) {
        self.clear_height();
        self.clear_body();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockTxs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockTxs {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ProofType {
    AuthorityRound = 0,
    Raft = 1,
    Tendermint = 2,
}

impl ::protobuf::ProtobufEnum for ProofType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ProofType> {
        match value {
            0 => ::std::option::Option::Some(ProofType::AuthorityRound),
            1 => ::std::option::Option::Some(ProofType::Raft),
            2 => ::std::option::Option::Some(ProofType::Tendermint),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ProofType] = &[
            ProofType::AuthorityRound,
            ProofType::Raft,
            ProofType::Tendermint,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ProofType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ProofType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ProofType {
}

impl ::std::default::Default for ProofType {
    fn default() -> Self {
        ProofType::AuthorityRound
    }
}

impl ::protobuf::reflect::ProtobufValue for ProofType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Crypto {
    SECP = 0,
    SM2 = 1,
}

impl ::protobuf::ProtobufEnum for Crypto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Crypto> {
        match value {
            0 => ::std::option::Option::Some(Crypto::SECP),
            1 => ::std::option::Option::Some(Crypto::SM2),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Crypto] = &[
            Crypto::SECP,
            Crypto::SM2,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Crypto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Crypto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Crypto {
}

impl ::std::default::Default for Crypto {
    fn default() -> Self {
        Crypto::SECP
    }
}

impl ::protobuf::reflect::ProtobufValue for Crypto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10blockchain.proto\"A\n\x05Proof\x12\x18\n\x07content\x18\x01\x20\
    \x01(\x0cR\x07content\x12\x1e\n\x04type\x18\x02\x20\x01(\x0e2\n.ProofTyp\
    eR\x04type\"\xa6\x02\n\x0bBlockHeader\x12\x1a\n\x08prevhash\x18\x01\x20\
    \x01(\x0cR\x08prevhash\x12\x1c\n\ttimestamp\x18\x02\x20\x01(\x04R\ttimes\
    tamp\x12\x16\n\x06height\x18\x03\x20\x01(\x04R\x06height\x12\x1d\n\nstat\
    e_root\x18\x04\x20\x01(\x0cR\tstateRoot\x12+\n\x11transactions_root\x18\
    \x05\x20\x01(\x0cR\x10transactionsRoot\x12#\n\rreceipts_root\x18\x06\x20\
    \x01(\x0cR\x0creceiptsRoot\x12\x19\n\x08gas_used\x18\x07\x20\x01(\x04R\
    \x07gasUsed\x12\x1b\n\tgas_limit\x18\x08\x20\x01(\x04R\x08gasLimit\x12\
    \x1c\n\x05proof\x18\t\x20\x01(\x0b2\x06.ProofR\x05proof\"4\n\x06Status\
    \x12\x12\n\x04hash\x18\x01\x20\x01(\x0cR\x04hash\x12\x16\n\x06height\x18\
    \x02\x20\x01(\x04R\x06height\"\xd6\x01\n\x0fAccountGasLimit\x12(\n\x10co\
    mmon_gas_limit\x18\x01\x20\x01(\x04R\x0ecommonGasLimit\x12T\n\x12specifi\
    c_gas_limit\x18\x02\x20\x03(\x0b2&.AccountGasLimit.SpecificGasLimitEntry\
    R\x10specificGasLimit\x1aC\n\x15SpecificGasLimitEntry\x12\x10\n\x03key\
    \x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\x04R\x05\
    value:\x028\x01\"N\n\nRichStatus\x12\x12\n\x04hash\x18\x01\x20\x01(\x0cR\
    \x04hash\x12\x16\n\x06height\x18\x02\x20\x01(\x04R\x06height\x12\x14\n\
    \x05nodes\x18\x03\x20\x03(\x0cR\x05nodes\"\x89\x01\n\x0bTransaction\x12\
    \x0e\n\x02to\x18\x01\x20\x01(\tR\x02to\x12\x14\n\x05nonce\x18\x02\x20\
    \x01(\tR\x05nonce\x12\x14\n\x05quota\x18\x03\x20\x01(\x04R\x05quota\x12*\
    \n\x11valid_until_block\x18\x04\x20\x01(\x04R\x0fvalidUntilBlock\x12\x12\
    \n\x04data\x18\x05\x20\x01(\x0cR\x04data\"\x86\x01\n\x15UnverifiedTransa\
    ction\x12.\n\x0btransaction\x18\x01\x20\x01(\x0b2\x0c.TransactionR\x0btr\
    ansaction\x12\x1c\n\tsignature\x18\x02\x20\x01(\x0cR\tsignature\x12\x1f\
    \n\x06crypto\x18\x03\x20\x01(\x0e2\x07.CryptoR\x06crypto\"\x8e\x01\n\x11\
    SignedTransaction\x12H\n\x14transaction_with_sig\x18\x01\x20\x01(\x0b2\
    \x16.UnverifiedTransactionR\x12transactionWithSig\x12\x17\n\x07tx_hash\
    \x18\x02\x20\x01(\x0cR\x06txHash\x12\x16\n\x06signer\x18\x03\x20\x01(\
    \x0cR\x06signer\"C\n\tBlockBody\x126\n\x0ctransactions\x18\x01\x20\x03(\
    \x0b2\x12.SignedTransactionR\x0ctransactions\"g\n\x05Block\x12\x18\n\x07\
    version\x18\x01\x20\x01(\rR\x07version\x12$\n\x06header\x18\x02\x20\x01(\
    \x0b2\x0c.BlockHeaderR\x06header\x12\x1e\n\x04body\x18\x03\x20\x01(\x0b2\
    \n.BlockBodyR\x04body\"H\n\x0eBlockWithProof\x12\x18\n\x03blk\x18\x01\
    \x20\x01(\x0b2\x06.BlockR\x03blk\x12\x1c\n\x05proof\x18\x02\x20\x01(\x0b\
    2\x06.ProofR\x05proof\"B\n\x08BlockTxs\x12\x16\n\x06height\x18\x01\x20\
    \x01(\x04R\x06height\x12\x1e\n\x04body\x18\x03\x20\x01(\x0b2\n.BlockBody\
    R\x04body*9\n\tProofType\x12\x12\n\x0eAuthorityRound\x10\0\x12\x08\n\x04\
    Raft\x10\x01\x12\x0e\n\nTendermint\x10\x02*\x1b\n\x06Crypto\x12\x08\n\
    \x04SECP\x10\0\x12\x07\n\x03SM2\x10\x01J\x89\x19\n\x06\x12\x04\0\0V\x01\
    \n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x05\0\x12\x04\x02\0\x06\x01\n\
    \n\n\x03\x05\0\x01\x12\x03\x02\x05\x0e\n\x0b\n\x04\x05\0\x02\0\x12\x03\
    \x03\x04\x17\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x03\x04\x12\n\x0c\n\x05\
    \x05\0\x02\0\x02\x12\x03\x03\x15\x16\n\x0b\n\x04\x05\0\x02\x01\x12\x03\
    \x04\x04\r\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x04\x04\x08\n\x0c\n\x05\
    \x05\0\x02\x01\x02\x12\x03\x04\x0b\x0c\n\x0b\n\x04\x05\0\x02\x02\x12\x03\
    \x05\x04\x13\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\x05\x04\x0e\n\x0c\n\
    \x05\x05\0\x02\x02\x02\x12\x03\x05\x11\x12\n\n\n\x02\x04\0\x12\x04\x08\0\
    \x0b\x01\n\n\n\x03\x04\0\x01\x12\x03\x08\x08\r\n\x0b\n\x04\x04\0\x02\0\
    \x12\x03\t\x04\x16\n\r\n\x05\x04\0\x02\0\x04\x12\x04\t\x04\x08\x0f\n\x0c\
    \n\x05\x04\0\x02\0\x05\x12\x03\t\x04\t\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\t\n\x11\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\t\x14\x15\n\x0b\n\x04\
    \x04\0\x02\x01\x12\x03\n\x04\x17\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\n\
    \x04\t\x16\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\n\x04\r\n\x0c\n\x05\x04\
    \0\x02\x01\x01\x12\x03\n\x0e\x12\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\n\
    \x15\x16\n\n\n\x02\x04\x01\x12\x04\r\0\x17\x01\n\n\n\x03\x04\x01\x01\x12\
    \x03\r\x08\x13\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x0e\x04\x17\n\r\n\x05\
    \x04\x01\x02\0\x04\x12\x04\x0e\x04\r\x15\n\x0c\n\x05\x04\x01\x02\0\x05\
    \x12\x03\x0e\x04\t\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x0e\n\x12\n\x0c\
    \n\x05\x04\x01\x02\0\x03\x12\x03\x0e\x15\x16\n\x0b\n\x04\x04\x01\x02\x01\
    \x12\x03\x0f\x04\x19\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x0f\x04\x0e\
    \x17\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x0f\x04\n\n\x0c\n\x05\x04\
    \x01\x02\x01\x01\x12\x03\x0f\x0b\x14\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\
    \x03\x0f\x17\x18\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x10\x04\x16\n\r\n\
    \x05\x04\x01\x02\x02\x04\x12\x04\x10\x04\x0f\x19\n\x0c\n\x05\x04\x01\x02\
    \x02\x05\x12\x03\x10\x04\n\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x10\
    \x0b\x11\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x10\x14\x15\n\x0b\n\x04\
    \x04\x01\x02\x03\x12\x03\x11\x04\x19\n\r\n\x05\x04\x01\x02\x03\x04\x12\
    \x04\x11\x04\x10\x16\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03\x11\x04\t\n\
    \x0c\n\x05\x04\x01\x02\x03\x01\x12\x03\x11\n\x14\n\x0c\n\x05\x04\x01\x02\
    \x03\x03\x12\x03\x11\x17\x18\n\x0b\n\x04\x04\x01\x02\x04\x12\x03\x12\x04\
    \x20\n\r\n\x05\x04\x01\x02\x04\x04\x12\x04\x12\x04\x11\x19\n\x0c\n\x05\
    \x04\x01\x02\x04\x05\x12\x03\x12\x04\t\n\x0c\n\x05\x04\x01\x02\x04\x01\
    \x12\x03\x12\n\x1b\n\x0c\n\x05\x04\x01\x02\x04\x03\x12\x03\x12\x1e\x1f\n\
    \x0b\n\x04\x04\x01\x02\x05\x12\x03\x13\x04\x1c\n\r\n\x05\x04\x01\x02\x05\
    \x04\x12\x04\x13\x04\x12\x20\n\x0c\n\x05\x04\x01\x02\x05\x05\x12\x03\x13\
    \x04\t\n\x0c\n\x05\x04\x01\x02\x05\x01\x12\x03\x13\n\x17\n\x0c\n\x05\x04\
    \x01\x02\x05\x03\x12\x03\x13\x1a\x1b\n\x0b\n\x04\x04\x01\x02\x06\x12\x03\
    \x14\x04\x18\n\r\n\x05\x04\x01\x02\x06\x04\x12\x04\x14\x04\x13\x1c\n\x0c\
    \n\x05\x04\x01\x02\x06\x05\x12\x03\x14\x04\n\n\x0c\n\x05\x04\x01\x02\x06\
    \x01\x12\x03\x14\x0b\x13\n\x0c\n\x05\x04\x01\x02\x06\x03\x12\x03\x14\x16\
    \x17\n\x0b\n\x04\x04\x01\x02\x07\x12\x03\x15\x04\x19\n\r\n\x05\x04\x01\
    \x02\x07\x04\x12\x04\x15\x04\x14\x18\n\x0c\n\x05\x04\x01\x02\x07\x05\x12\
    \x03\x15\x04\n\n\x0c\n\x05\x04\x01\x02\x07\x01\x12\x03\x15\x0b\x14\n\x0c\
    \n\x05\x04\x01\x02\x07\x03\x12\x03\x15\x17\x18\n\x0b\n\x04\x04\x01\x02\
    \x08\x12\x03\x16\x04\x14\n\r\n\x05\x04\x01\x02\x08\x04\x12\x04\x16\x04\
    \x15\x19\n\x0c\n\x05\x04\x01\x02\x08\x06\x12\x03\x16\x04\t\n\x0c\n\x05\
    \x04\x01\x02\x08\x01\x12\x03\x16\n\x0f\n\x0c\n\x05\x04\x01\x02\x08\x03\
    \x12\x03\x16\x12\x13\n\n\n\x02\x04\x02\x12\x04\x19\0\x1c\x01\n\n\n\x03\
    \x04\x02\x01\x12\x03\x19\x08\x0e\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x1a\
    \x04\x13\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x1a\x04\x19\x10\n\x0c\n\x05\
    \x04\x02\x02\0\x05\x12\x03\x1a\x04\t\n\x0c\n\x05\x04\x02\x02\0\x01\x12\
    \x03\x1a\n\x0e\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x1a\x11\x12\n\x0b\n\
    \x04\x04\x02\x02\x01\x12\x03\x1b\x04\x16\n\r\n\x05\x04\x02\x02\x01\x04\
    \x12\x04\x1b\x04\x1a\x13\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x1b\x04\
    \n\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x1b\x0b\x11\n\x0c\n\x05\x04\
    \x02\x02\x01\x03\x12\x03\x1b\x14\x15\n\n\n\x02\x04\x03\x12\x04\x1e\0!\
    \x01\n\n\n\x03\x04\x03\x01\x12\x03\x1e\x08\x17\n\x0b\n\x04\x04\x03\x02\0\
    \x12\x03\x1f\x04\x20\n\r\n\x05\x04\x03\x02\0\x04\x12\x04\x1f\x04\x1e\x19\
    \n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03\x1f\x04\n\n\x0c\n\x05\x04\x03\x02\
    \0\x01\x12\x03\x1f\x0b\x1b\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x1f\x1e\
    \x1f\n\x0b\n\x04\x04\x03\x02\x01\x12\x03\x20\x04.\n\r\n\x05\x04\x03\x02\
    \x01\x04\x12\x04\x20\x04\x1f\x20\n\x0c\n\x05\x04\x03\x02\x01\x06\x12\x03\
    \x20\x04\x16\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03\x20\x17)\n\x0c\n\
    \x05\x04\x03\x02\x01\x03\x12\x03\x20,-\n\n\n\x02\x04\x04\x12\x04#\0'\x01\
    \n\n\n\x03\x04\x04\x01\x12\x03#\x08\x12\n\x0b\n\x04\x04\x04\x02\0\x12\
    \x03$\x04\x13\n\r\n\x05\x04\x04\x02\0\x04\x12\x04$\x04#\x14\n\x0c\n\x05\
    \x04\x04\x02\0\x05\x12\x03$\x04\t\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03$\
    \n\x0e\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03$\x11\x12\n\x0b\n\x04\x04\
    \x04\x02\x01\x12\x03%\x04\x16\n\r\n\x05\x04\x04\x02\x01\x04\x12\x04%\x04\
    $\x13\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03%\x04\n\n\x0c\n\x05\x04\x04\
    \x02\x01\x01\x12\x03%\x0b\x11\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03%\
    \x14\x15\n\x0b\n\x04\x04\x04\x02\x02\x12\x03&\x04\x1d\n\x0c\n\x05\x04\
    \x04\x02\x02\x04\x12\x03&\x04\x0c\n\x0c\n\x05\x04\x04\x02\x02\x05\x12\
    \x03&\r\x12\n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x03&\x13\x18\n\x0c\n\x05\
    \x04\x04\x02\x02\x03\x12\x03&\x1b\x1c\n\n\n\x02\x05\x01\x12\x04)\0,\x01\
    \n\n\n\x03\x05\x01\x01\x12\x03)\x05\x0b\n\x0b\n\x04\x05\x01\x02\0\x12\
    \x03*\x04\r\n\x0c\n\x05\x05\x01\x02\0\x01\x12\x03*\x04\x08\n\x0c\n\x05\
    \x05\x01\x02\0\x02\x12\x03*\x0b\x0c\n\x0b\n\x04\x05\x01\x02\x01\x12\x03+\
    \x04\x0c\n\x0c\n\x05\x05\x01\x02\x01\x01\x12\x03+\x04\x07\n\x0c\n\x05\
    \x05\x01\x02\x01\x02\x12\x03+\n\x0b\n\n\n\x02\x04\x05\x12\x04.\04\x01\n\
    \n\n\x03\x04\x05\x01\x12\x03.\x08\x13\n\x0b\n\x04\x04\x05\x02\0\x12\x03/\
    \x04\x12\n\r\n\x05\x04\x05\x02\0\x04\x12\x04/\x04.\x15\n\x0c\n\x05\x04\
    \x05\x02\0\x05\x12\x03/\x04\n\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03/\x0b\
    \r\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03/\x10\x11\n\x0b\n\x04\x04\x05\
    \x02\x01\x12\x030\x04\x15\n\r\n\x05\x04\x05\x02\x01\x04\x12\x040\x04/\
    \x12\n\x0c\n\x05\x04\x05\x02\x01\x05\x12\x030\x04\n\n\x0c\n\x05\x04\x05\
    \x02\x01\x01\x12\x030\x0b\x10\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x030\
    \x13\x14\n\x0b\n\x04\x04\x05\x02\x02\x12\x031\x04\x15\n\r\n\x05\x04\x05\
    \x02\x02\x04\x12\x041\x040\x15\n\x0c\n\x05\x04\x05\x02\x02\x05\x12\x031\
    \x04\n\n\x0c\n\x05\x04\x05\x02\x02\x01\x12\x031\x0b\x10\n\x0c\n\x05\x04\
    \x05\x02\x02\x03\x12\x031\x13\x14\n\x0b\n\x04\x04\x05\x02\x03\x12\x032\
    \x04!\n\r\n\x05\x04\x05\x02\x03\x04\x12\x042\x041\x15\n\x0c\n\x05\x04\
    \x05\x02\x03\x05\x12\x032\x04\n\n\x0c\n\x05\x04\x05\x02\x03\x01\x12\x032\
    \x0b\x1c\n\x0c\n\x05\x04\x05\x02\x03\x03\x12\x032\x1f\x20\n\x0b\n\x04\
    \x04\x05\x02\x04\x12\x033\x04\x13\n\r\n\x05\x04\x05\x02\x04\x04\x12\x043\
    \x042!\n\x0c\n\x05\x04\x05\x02\x04\x05\x12\x033\x04\t\n\x0c\n\x05\x04\
    \x05\x02\x04\x01\x12\x033\n\x0e\n\x0c\n\x05\x04\x05\x02\x04\x03\x12\x033\
    \x11\x12\n\n\n\x02\x04\x06\x12\x046\0:\x01\n\n\n\x03\x04\x06\x01\x12\x03\
    6\x08\x1d\n\x0b\n\x04\x04\x06\x02\0\x12\x037\x04\x20\n\r\n\x05\x04\x06\
    \x02\0\x04\x12\x047\x046\x1f\n\x0c\n\x05\x04\x06\x02\0\x06\x12\x037\x04\
    \x0f\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x037\x10\x1b\n\x0c\n\x05\x04\x06\
    \x02\0\x03\x12\x037\x1e\x1f\n\x0b\n\x04\x04\x06\x02\x01\x12\x038\x04\x18\
    \n\r\n\x05\x04\x06\x02\x01\x04\x12\x048\x047\x20\n\x0c\n\x05\x04\x06\x02\
    \x01\x05\x12\x038\x04\t\n\x0c\n\x05\x04\x06\x02\x01\x01\x12\x038\n\x13\n\
    \x0c\n\x05\x04\x06\x02\x01\x03\x12\x038\x16\x17\n\x0b\n\x04\x04\x06\x02\
    \x02\x12\x039\x04\x16\n\r\n\x05\x04\x06\x02\x02\x04\x12\x049\x048\x18\n\
    \x0c\n\x05\x04\x06\x02\x02\x06\x12\x039\x04\n\n\x0c\n\x05\x04\x06\x02\
    \x02\x01\x12\x039\x0b\x11\n\x0c\n\x05\x04\x06\x02\x02\x03\x12\x039\x14\
    \x15\n\n\n\x02\x04\x07\x12\x04<\0@\x01\n\n\n\x03\x04\x07\x01\x12\x03<\
    \x08\x19\n\x0b\n\x04\x04\x07\x02\0\x12\x03=\x043\n\r\n\x05\x04\x07\x02\0\
    \x04\x12\x04=\x04<\x1b\n\x0c\n\x05\x04\x07\x02\0\x06\x12\x03=\x04\x19\n\
    \x0c\n\x05\x04\x07\x02\0\x01\x12\x03=\x1a.\n\x0c\n\x05\x04\x07\x02\0\x03\
    \x12\x03=12\n%\n\x04\x04\x07\x02\x01\x12\x03>\x04\x16\"\x18\x20SignedTra\
    nsaction\x20hash\n\n\r\n\x05\x04\x07\x02\x01\x04\x12\x04>\x04=3\n\x0c\n\
    \x05\x04\x07\x02\x01\x05\x12\x03>\x04\t\n\x0c\n\x05\x04\x07\x02\x01\x01\
    \x12\x03>\n\x11\n\x0c\n\x05\x04\x07\x02\x01\x03\x12\x03>\x14\x15\n\x18\n\
    \x04\x04\x07\x02\x02\x12\x03?\x04\x15\"\x0bpublic\x20key\n\n\r\n\x05\x04\
    \x07\x02\x02\x04\x12\x04?\x04>\x16\n\x0c\n\x05\x04\x07\x02\x02\x05\x12\
    \x03?\x04\t\n\x0c\n\x05\x04\x07\x02\x02\x01\x12\x03?\n\x10\n\x0c\n\x05\
    \x04\x07\x02\x02\x03\x12\x03?\x13\x14\n!\n\x02\x04\x08\x12\x04D\0F\x012\
    \x15\x20data\x20precompile\x20API\n\n\n\n\x03\x04\x08\x01\x12\x03D\x08\
    \x11\n\x0b\n\x04\x04\x08\x02\0\x12\x03E\x040\n\x0c\n\x05\x04\x08\x02\0\
    \x04\x12\x03E\x04\x0c\n\x0c\n\x05\x04\x08\x02\0\x06\x12\x03E\r\x1e\n\x0c\
    \n\x05\x04\x08\x02\0\x01\x12\x03E\x1f+\n\x0c\n\x05\x04\x08\x02\0\x03\x12\
    \x03E./\n\n\n\x02\x04\t\x12\x04H\0L\x01\n\n\n\x03\x04\t\x01\x12\x03H\x08\
    \r\n\x0b\n\x04\x04\t\x02\0\x12\x03I\x04\x17\n\r\n\x05\x04\t\x02\0\x04\
    \x12\x04I\x04H\x0f\n\x0c\n\x05\x04\t\x02\0\x05\x12\x03I\x04\n\n\x0c\n\
    \x05\x04\t\x02\0\x01\x12\x03I\x0b\x12\n\x0c\n\x05\x04\t\x02\0\x03\x12\
    \x03I\x15\x16\n\x0b\n\x04\x04\t\x02\x01\x12\x03J\x04\x1b\n\r\n\x05\x04\t\
    \x02\x01\x04\x12\x04J\x04I\x17\n\x0c\n\x05\x04\t\x02\x01\x06\x12\x03J\
    \x04\x0f\n\x0c\n\x05\x04\t\x02\x01\x01\x12\x03J\x10\x16\n\x0c\n\x05\x04\
    \t\x02\x01\x03\x12\x03J\x19\x1a\n\x0b\n\x04\x04\t\x02\x02\x12\x03K\x04\
    \x17\n\r\n\x05\x04\t\x02\x02\x04\x12\x04K\x04J\x1b\n\x0c\n\x05\x04\t\x02\
    \x02\x06\x12\x03K\x04\r\n\x0c\n\x05\x04\t\x02\x02\x01\x12\x03K\x0e\x12\n\
    \x0c\n\x05\x04\t\x02\x02\x03\x12\x03K\x15\x16\n\n\n\x02\x04\n\x12\x04N\0\
    Q\x01\n\n\n\x03\x04\n\x01\x12\x03N\x08\x16\n\x0b\n\x04\x04\n\x02\0\x12\
    \x03O\x04\x12\n\r\n\x05\x04\n\x02\0\x04\x12\x04O\x04N\x18\n\x0c\n\x05\
    \x04\n\x02\0\x06\x12\x03O\x04\t\n\x0c\n\x05\x04\n\x02\0\x01\x12\x03O\n\r\
    \n\x0c\n\x05\x04\n\x02\0\x03\x12\x03O\x10\x11\n\x0b\n\x04\x04\n\x02\x01\
    \x12\x03P\x04\x14\n\r\n\x05\x04\n\x02\x01\x04\x12\x04P\x04O\x12\n\x0c\n\
    \x05\x04\n\x02\x01\x06\x12\x03P\x04\t\n\x0c\n\x05\x04\n\x02\x01\x01\x12\
    \x03P\n\x0f\n\x0c\n\x05\x04\n\x02\x01\x03\x12\x03P\x12\x13\n\n\n\x02\x04\
    \x0b\x12\x04S\0V\x01\n\n\n\x03\x04\x0b\x01\x12\x03S\x08\x10\n\x0b\n\x04\
    \x04\x0b\x02\0\x12\x03T\x04\x16\n\r\n\x05\x04\x0b\x02\0\x04\x12\x04T\x04\
    S\x12\n\x0c\n\x05\x04\x0b\x02\0\x05\x12\x03T\x04\n\n\x0c\n\x05\x04\x0b\
    \x02\0\x01\x12\x03T\x0b\x11\n\x0c\n\x05\x04\x0b\x02\0\x03\x12\x03T\x14\
    \x15\n\x0b\n\x04\x04\x0b\x02\x01\x12\x03U\x04\x17\n\r\n\x05\x04\x0b\x02\
    \x01\x04\x12\x04U\x04T\x16\n\x0c\n\x05\x04\x0b\x02\x01\x06\x12\x03U\x04\
    \r\n\x0c\n\x05\x04\x0b\x02\x01\x01\x12\x03U\x0e\x12\n\x0c\n\x05\x04\x0b\
    \x02\x01\x03\x12\x03U\x15\x16b\x06proto3\
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
