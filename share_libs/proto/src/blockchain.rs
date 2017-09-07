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

    // .Proof proof = 8;

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
        if let Some(ref v) = self.proof.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
pub struct TxResponse {
    // message fields
    pub hash: ::std::vec::Vec<u8>,
    pub result: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TxResponse {}

impl TxResponse {
    pub fn new() -> TxResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TxResponse {
        static mut instance: ::protobuf::lazy::Lazy<TxResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TxResponse,
        };
        unsafe {
            instance.get(TxResponse::new)
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

    // bytes result = 2;

    pub fn clear_result(&mut self) {
        self.result.clear();
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: ::std::vec::Vec<u8>) {
        self.result = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_result(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.result
    }

    // Take field
    pub fn take_result(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.result, ::std::vec::Vec::new())
    }

    pub fn get_result(&self) -> &[u8] {
        &self.result
    }

    fn get_result_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.result
    }
}

impl ::protobuf::Message for TxResponse {
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
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.result)?;
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
        if !self.result.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.result);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.hash.is_empty() {
            os.write_bytes(1, &self.hash)?;
        }
        if !self.result.is_empty() {
            os.write_bytes(2, &self.result)?;
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

impl ::protobuf::MessageStatic for TxResponse {
    fn new() -> TxResponse {
        TxResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<TxResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    TxResponse::get_hash_for_reflect,
                    TxResponse::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "result",
                    TxResponse::get_result_for_reflect,
                    TxResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TxResponse>(
                    "TxResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TxResponse {
    fn clear(&mut self) {
        self.clear_hash();
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TxResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TxResponse {
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
    eR\x04type\"\x89\x02\n\x0bBlockHeader\x12\x1a\n\x08prevhash\x18\x01\x20\
    \x01(\x0cR\x08prevhash\x12\x1c\n\ttimestamp\x18\x02\x20\x01(\x04R\ttimes\
    tamp\x12\x16\n\x06height\x18\x03\x20\x01(\x04R\x06height\x12\x1d\n\nstat\
    e_root\x18\x04\x20\x01(\x0cR\tstateRoot\x12+\n\x11transactions_root\x18\
    \x05\x20\x01(\x0cR\x10transactionsRoot\x12#\n\rreceipts_root\x18\x06\x20\
    \x01(\x0cR\x0creceiptsRoot\x12\x19\n\x08gas_used\x18\x07\x20\x01(\x04R\
    \x07gasUsed\x12\x1c\n\x05proof\x18\x08\x20\x01(\x0b2\x06.ProofR\x05proof\
    \"4\n\x06Status\x12\x12\n\x04hash\x18\x01\x20\x01(\x0cR\x04hash\x12\x16\
    \n\x06height\x18\x02\x20\x01(\x04R\x06height\"\x89\x01\n\x0bTransaction\
    \x12\x0e\n\x02to\x18\x01\x20\x01(\tR\x02to\x12\x14\n\x05nonce\x18\x02\
    \x20\x01(\tR\x05nonce\x12\x14\n\x05quota\x18\x03\x20\x01(\x04R\x05quota\
    \x12*\n\x11valid_until_block\x18\x04\x20\x01(\x04R\x0fvalidUntilBlock\
    \x12\x12\n\x04data\x18\x05\x20\x01(\x0cR\x04data\"\x86\x01\n\x15Unverifi\
    edTransaction\x12.\n\x0btransaction\x18\x01\x20\x01(\x0b2\x0c.Transactio\
    nR\x0btransaction\x12\x1c\n\tsignature\x18\x02\x20\x01(\x0cR\tsignature\
    \x12\x1f\n\x06crypto\x18\x03\x20\x01(\x0e2\x07.CryptoR\x06crypto\"\x8e\
    \x01\n\x11SignedTransaction\x12H\n\x14transaction_with_sig\x18\x01\x20\
    \x01(\x0b2\x16.UnverifiedTransactionR\x12transactionWithSig\x12\x17\n\
    \x07tx_hash\x18\x02\x20\x01(\x0cR\x06txHash\x12\x16\n\x06signer\x18\x03\
    \x20\x01(\x0cR\x06signer\"8\n\nTxResponse\x12\x12\n\x04hash\x18\x01\x20\
    \x01(\x0cR\x04hash\x12\x16\n\x06result\x18\x02\x20\x01(\x0cR\x06result\"\
    C\n\tBlockBody\x126\n\x0ctransactions\x18\x01\x20\x03(\x0b2\x12.SignedTr\
    ansactionR\x0ctransactions\"g\n\x05Block\x12\x18\n\x07version\x18\x01\
    \x20\x01(\rR\x07version\x12$\n\x06header\x18\x02\x20\x01(\x0b2\x0c.Block\
    HeaderR\x06header\x12\x1e\n\x04body\x18\x03\x20\x01(\x0b2\n.BlockBodyR\
    \x04body*9\n\tProofType\x12\x12\n\x0eAuthorityRound\x10\0\x12\x08\n\x04R\
    aft\x10\x01\x12\x0e\n\nTendermint\x10\x02*\x1b\n\x06Crypto\x12\x08\n\x04\
    SECP\x10\0\x12\x07\n\x03SM2\x10\x01J\x92\x14\n\x06\x12\x04\0\0E\x01\n\
    \x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x05\0\x12\x04\x02\0\x06\x01\n\n\
    \n\x03\x05\0\x01\x12\x03\x02\x05\x0e\n\x0b\n\x04\x05\0\x02\0\x12\x03\x03\
    \x04\x17\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x03\x04\x12\n\x0c\n\x05\x05\
    \0\x02\0\x02\x12\x03\x03\x15\x16\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x04\
    \x04\r\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x04\x04\x08\n\x0c\n\x05\x05\
    \0\x02\x01\x02\x12\x03\x04\x0b\x0c\n\x0b\n\x04\x05\0\x02\x02\x12\x03\x05\
    \x04\x13\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\x05\x04\x0e\n\x0c\n\x05\
    \x05\0\x02\x02\x02\x12\x03\x05\x11\x12\n\n\n\x02\x04\0\x12\x04\x08\0\x0b\
    \x01\n\n\n\x03\x04\0\x01\x12\x03\x08\x08\r\n\x0b\n\x04\x04\0\x02\0\x12\
    \x03\t\x04\x16\n\r\n\x05\x04\0\x02\0\x04\x12\x04\t\x04\x08\x0f\n\x0c\n\
    \x05\x04\0\x02\0\x05\x12\x03\t\x04\t\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \t\n\x11\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\t\x14\x15\n\x0b\n\x04\x04\0\
    \x02\x01\x12\x03\n\x04\x17\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\n\x04\t\
    \x16\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\n\x04\r\n\x0c\n\x05\x04\0\x02\
    \x01\x01\x12\x03\n\x0e\x12\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\n\x15\
    \x16\n\n\n\x02\x04\x01\x12\x04\r\0\x16\x01\n\n\n\x03\x04\x01\x01\x12\x03\
    \r\x08\x13\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x0e\x04\x17\n\r\n\x05\x04\
    \x01\x02\0\x04\x12\x04\x0e\x04\r\x15\n\x0c\n\x05\x04\x01\x02\0\x05\x12\
    \x03\x0e\x04\t\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x0e\n\x12\n\x0c\n\
    \x05\x04\x01\x02\0\x03\x12\x03\x0e\x15\x16\n\x0b\n\x04\x04\x01\x02\x01\
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
    \x17\n\x0b\n\x04\x04\x01\x02\x07\x12\x03\x15\x04\x14\n\r\n\x05\x04\x01\
    \x02\x07\x04\x12\x04\x15\x04\x14\x18\n\x0c\n\x05\x04\x01\x02\x07\x06\x12\
    \x03\x15\x04\t\n\x0c\n\x05\x04\x01\x02\x07\x01\x12\x03\x15\n\x0f\n\x0c\n\
    \x05\x04\x01\x02\x07\x03\x12\x03\x15\x12\x13\n\n\n\x02\x04\x02\x12\x04\
    \x18\0\x1b\x01\n\n\n\x03\x04\x02\x01\x12\x03\x18\x08\x0e\n\x0b\n\x04\x04\
    \x02\x02\0\x12\x03\x19\x04\x13\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x19\
    \x04\x18\x10\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x19\x04\t\n\x0c\n\x05\
    \x04\x02\x02\0\x01\x12\x03\x19\n\x0e\n\x0c\n\x05\x04\x02\x02\0\x03\x12\
    \x03\x19\x11\x12\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x1a\x04\x16\n\r\n\
    \x05\x04\x02\x02\x01\x04\x12\x04\x1a\x04\x19\x13\n\x0c\n\x05\x04\x02\x02\
    \x01\x05\x12\x03\x1a\x04\n\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x1a\
    \x0b\x11\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x1a\x14\x15\n\n\n\x02\
    \x05\x01\x12\x04\x1d\0\x20\x01\n\n\n\x03\x05\x01\x01\x12\x03\x1d\x05\x0b\
    \n\x0b\n\x04\x05\x01\x02\0\x12\x03\x1e\x04\r\n\x0c\n\x05\x05\x01\x02\0\
    \x01\x12\x03\x1e\x04\x08\n\x0c\n\x05\x05\x01\x02\0\x02\x12\x03\x1e\x0b\
    \x0c\n\x0b\n\x04\x05\x01\x02\x01\x12\x03\x1f\x04\x0c\n\x0c\n\x05\x05\x01\
    \x02\x01\x01\x12\x03\x1f\x04\x07\n\x0c\n\x05\x05\x01\x02\x01\x02\x12\x03\
    \x1f\n\x0b\n\n\n\x02\x04\x03\x12\x04\"\0(\x01\n\n\n\x03\x04\x03\x01\x12\
    \x03\"\x08\x13\n\x0b\n\x04\x04\x03\x02\0\x12\x03#\x04\x12\n\r\n\x05\x04\
    \x03\x02\0\x04\x12\x04#\x04\"\x15\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03#\
    \x04\n\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03#\x0b\r\n\x0c\n\x05\x04\x03\
    \x02\0\x03\x12\x03#\x10\x11\n\x0b\n\x04\x04\x03\x02\x01\x12\x03$\x04\x15\
    \n\r\n\x05\x04\x03\x02\x01\x04\x12\x04$\x04#\x12\n\x0c\n\x05\x04\x03\x02\
    \x01\x05\x12\x03$\x04\n\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03$\x0b\x10\
    \n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03$\x13\x14\n\x0b\n\x04\x04\x03\
    \x02\x02\x12\x03%\x04\x15\n\r\n\x05\x04\x03\x02\x02\x04\x12\x04%\x04$\
    \x15\n\x0c\n\x05\x04\x03\x02\x02\x05\x12\x03%\x04\n\n\x0c\n\x05\x04\x03\
    \x02\x02\x01\x12\x03%\x0b\x10\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03%\
    \x13\x14\n\x0b\n\x04\x04\x03\x02\x03\x12\x03&\x04!\n\r\n\x05\x04\x03\x02\
    \x03\x04\x12\x04&\x04%\x15\n\x0c\n\x05\x04\x03\x02\x03\x05\x12\x03&\x04\
    \n\n\x0c\n\x05\x04\x03\x02\x03\x01\x12\x03&\x0b\x1c\n\x0c\n\x05\x04\x03\
    \x02\x03\x03\x12\x03&\x1f\x20\n\x0b\n\x04\x04\x03\x02\x04\x12\x03'\x04\
    \x13\n\r\n\x05\x04\x03\x02\x04\x04\x12\x04'\x04&!\n\x0c\n\x05\x04\x03\
    \x02\x04\x05\x12\x03'\x04\t\n\x0c\n\x05\x04\x03\x02\x04\x01\x12\x03'\n\
    \x0e\n\x0c\n\x05\x04\x03\x02\x04\x03\x12\x03'\x11\x12\n\n\n\x02\x04\x04\
    \x12\x04*\0.\x01\n\n\n\x03\x04\x04\x01\x12\x03*\x08\x1d\n\x0b\n\x04\x04\
    \x04\x02\0\x12\x03+\x04\x20\n\r\n\x05\x04\x04\x02\0\x04\x12\x04+\x04*\
    \x1f\n\x0c\n\x05\x04\x04\x02\0\x06\x12\x03+\x04\x0f\n\x0c\n\x05\x04\x04\
    \x02\0\x01\x12\x03+\x10\x1b\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03+\x1e\
    \x1f\n\x0b\n\x04\x04\x04\x02\x01\x12\x03,\x04\x18\n\r\n\x05\x04\x04\x02\
    \x01\x04\x12\x04,\x04+\x20\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03,\x04\
    \t\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03,\n\x13\n\x0c\n\x05\x04\x04\
    \x02\x01\x03\x12\x03,\x16\x17\n\x0b\n\x04\x04\x04\x02\x02\x12\x03-\x04\
    \x16\n\r\n\x05\x04\x04\x02\x02\x04\x12\x04-\x04,\x18\n\x0c\n\x05\x04\x04\
    \x02\x02\x06\x12\x03-\x04\n\n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x03-\x0b\
    \x11\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\x03-\x14\x15\n\n\n\x02\x04\x05\
    \x12\x040\04\x01\n\n\n\x03\x04\x05\x01\x12\x030\x08\x19\n\x0b\n\x04\x04\
    \x05\x02\0\x12\x031\x043\n\r\n\x05\x04\x05\x02\0\x04\x12\x041\x040\x1b\n\
    \x0c\n\x05\x04\x05\x02\0\x06\x12\x031\x04\x19\n\x0c\n\x05\x04\x05\x02\0\
    \x01\x12\x031\x1a.\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03112\n%\n\x04\x04\
    \x05\x02\x01\x12\x032\x04\x16\"\x18\x20SignedTransaction\x20hash\n\n\r\n\
    \x05\x04\x05\x02\x01\x04\x12\x042\x0413\n\x0c\n\x05\x04\x05\x02\x01\x05\
    \x12\x032\x04\t\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x032\n\x11\n\x0c\n\
    \x05\x04\x05\x02\x01\x03\x12\x032\x14\x15\n\x18\n\x04\x04\x05\x02\x02\
    \x12\x033\x04\x15\"\x0bpublic\x20key\n\n\r\n\x05\x04\x05\x02\x02\x04\x12\
    \x043\x042\x16\n\x0c\n\x05\x04\x05\x02\x02\x05\x12\x033\x04\t\n\x0c\n\
    \x05\x04\x05\x02\x02\x01\x12\x033\n\x10\n\x0c\n\x05\x04\x05\x02\x02\x03\
    \x12\x033\x13\x14\n!\n\x02\x04\x06\x12\x048\0;\x012\x15\x20data\x20preco\
    mpile\x20API\n\n\n\n\x03\x04\x06\x01\x12\x038\x08\x12\n\x0b\n\x04\x04\
    \x06\x02\0\x12\x039\x04\x13\n\r\n\x05\x04\x06\x02\0\x04\x12\x049\x048\
    \x14\n\x0c\n\x05\x04\x06\x02\0\x05\x12\x039\x04\t\n\x0c\n\x05\x04\x06\
    \x02\0\x01\x12\x039\n\x0e\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x039\x11\x12\
    \n\x0b\n\x04\x04\x06\x02\x01\x12\x03:\x04\x15\n\r\n\x05\x04\x06\x02\x01\
    \x04\x12\x04:\x049\x13\n\x0c\n\x05\x04\x06\x02\x01\x05\x12\x03:\x04\t\n\
    \x0c\n\x05\x04\x06\x02\x01\x01\x12\x03:\n\x10\n\x0c\n\x05\x04\x06\x02\
    \x01\x03\x12\x03:\x13\x14\n\n\n\x02\x04\x07\x12\x04=\0?\x01\n\n\n\x03\
    \x04\x07\x01\x12\x03=\x08\x11\n\x0b\n\x04\x04\x07\x02\0\x12\x03>\x040\n\
    \x0c\n\x05\x04\x07\x02\0\x04\x12\x03>\x04\x0c\n\x0c\n\x05\x04\x07\x02\0\
    \x06\x12\x03>\r\x1e\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x03>\x1f+\n\x0c\n\
    \x05\x04\x07\x02\0\x03\x12\x03>./\n\n\n\x02\x04\x08\x12\x04A\0E\x01\n\n\
    \n\x03\x04\x08\x01\x12\x03A\x08\r\n\x0b\n\x04\x04\x08\x02\0\x12\x03B\x04\
    \x17\n\r\n\x05\x04\x08\x02\0\x04\x12\x04B\x04A\x0f\n\x0c\n\x05\x04\x08\
    \x02\0\x05\x12\x03B\x04\n\n\x0c\n\x05\x04\x08\x02\0\x01\x12\x03B\x0b\x12\
    \n\x0c\n\x05\x04\x08\x02\0\x03\x12\x03B\x15\x16\n\x0b\n\x04\x04\x08\x02\
    \x01\x12\x03C\x04\x1b\n\r\n\x05\x04\x08\x02\x01\x04\x12\x04C\x04B\x17\n\
    \x0c\n\x05\x04\x08\x02\x01\x06\x12\x03C\x04\x0f\n\x0c\n\x05\x04\x08\x02\
    \x01\x01\x12\x03C\x10\x16\n\x0c\n\x05\x04\x08\x02\x01\x03\x12\x03C\x19\
    \x1a\n\x0b\n\x04\x04\x08\x02\x02\x12\x03D\x04\x17\n\r\n\x05\x04\x08\x02\
    \x02\x04\x12\x04D\x04C\x1b\n\x0c\n\x05\x04\x08\x02\x02\x06\x12\x03D\x04\
    \r\n\x0c\n\x05\x04\x08\x02\x02\x01\x12\x03D\x0e\x12\n\x0c\n\x05\x04\x08\
    \x02\x02\x03\x12\x03D\x15\x16b\x06proto3\
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
