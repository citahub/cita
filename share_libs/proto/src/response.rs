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
pub struct FullTransaction {
    // message fields
    pub transaction: ::protobuf::SingularPtrField<super::blockchain::SignedTransaction>,
    pub block_number: u64,
    pub block_hash: ::std::vec::Vec<u8>,
    pub index: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FullTransaction {}

impl FullTransaction {
    pub fn new() -> FullTransaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FullTransaction {
        static mut instance: ::protobuf::lazy::Lazy<FullTransaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FullTransaction,
        };
        unsafe {
            instance.get(FullTransaction::new)
        }
    }

    // .SignedTransaction transaction = 1;

    pub fn clear_transaction(&mut self) {
        self.transaction.clear();
    }

    pub fn has_transaction(&self) -> bool {
        self.transaction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transaction(&mut self, v: super::blockchain::SignedTransaction) {
        self.transaction = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transaction(&mut self) -> &mut super::blockchain::SignedTransaction {
        if self.transaction.is_none() {
            self.transaction.set_default();
        };
        self.transaction.as_mut().unwrap()
    }

    // Take field
    pub fn take_transaction(&mut self) -> super::blockchain::SignedTransaction {
        self.transaction.take().unwrap_or_else(|| super::blockchain::SignedTransaction::new())
    }

    pub fn get_transaction(&self) -> &super::blockchain::SignedTransaction {
        self.transaction.as_ref().unwrap_or_else(|| super::blockchain::SignedTransaction::default_instance())
    }

    fn get_transaction_for_reflect(&self) -> &::protobuf::SingularPtrField<super::blockchain::SignedTransaction> {
        &self.transaction
    }

    fn mut_transaction_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::blockchain::SignedTransaction> {
        &mut self.transaction
    }

    // uint64 block_number = 2;

    pub fn clear_block_number(&mut self) {
        self.block_number = 0;
    }

    // Param is passed by value, moved
    pub fn set_block_number(&mut self, v: u64) {
        self.block_number = v;
    }

    pub fn get_block_number(&self) -> u64 {
        self.block_number
    }

    fn get_block_number_for_reflect(&self) -> &u64 {
        &self.block_number
    }

    fn mut_block_number_for_reflect(&mut self) -> &mut u64 {
        &mut self.block_number
    }

    // bytes block_hash = 3;

    pub fn clear_block_hash(&mut self) {
        self.block_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_block_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.block_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.block_hash
    }

    // Take field
    pub fn take_block_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.block_hash, ::std::vec::Vec::new())
    }

    pub fn get_block_hash(&self) -> &[u8] {
        &self.block_hash
    }

    fn get_block_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.block_hash
    }

    fn mut_block_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.block_hash
    }

    // uint32 index = 4;

    pub fn clear_index(&mut self) {
        self.index = 0;
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u32) {
        self.index = v;
    }

    pub fn get_index(&self) -> u32 {
        self.index
    }

    fn get_index_for_reflect(&self) -> &u32 {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut u32 {
        &mut self.index
    }
}

impl ::protobuf::Message for FullTransaction {
    fn is_initialized(&self) -> bool {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.block_number = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.block_hash)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.index = tmp;
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
        if let Some(v) = self.transaction.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.block_number != 0 {
            my_size += ::protobuf::rt::value_size(2, self.block_number, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.block_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.block_hash);
        };
        if self.index != 0 {
            my_size += ::protobuf::rt::value_size(4, self.index, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.transaction.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.block_number != 0 {
            os.write_uint64(2, self.block_number)?;
        };
        if !self.block_hash.is_empty() {
            os.write_bytes(3, &self.block_hash)?;
        };
        if self.index != 0 {
            os.write_uint32(4, self.index)?;
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

impl ::protobuf::MessageStatic for FullTransaction {
    fn new() -> FullTransaction {
        FullTransaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<FullTransaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockchain::SignedTransaction>>(
                    "transaction",
                    FullTransaction::get_transaction_for_reflect,
                    FullTransaction::mut_transaction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "block_number",
                    FullTransaction::get_block_number_for_reflect,
                    FullTransaction::mut_block_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "block_hash",
                    FullTransaction::get_block_hash_for_reflect,
                    FullTransaction::mut_block_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "index",
                    FullTransaction::get_index_for_reflect,
                    FullTransaction::mut_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FullTransaction>(
                    "FullTransaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FullTransaction {
    fn clear(&mut self) {
        self.clear_transaction();
        self.clear_block_number();
        self.clear_block_hash();
        self.clear_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FullTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FullTransaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Response {
    // message fields
    pub request_id: ::std::vec::Vec<u8>,
    pub code: i64,
    // message oneof groups
    pub data: ::std::option::Option<Response_oneof_data>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Response {}

#[derive(Clone,PartialEq)]
pub enum Response_oneof_data {
    error_msg(::std::string::String),
    tx_state(::std::string::String),
    block_number(u64),
    block(::std::string::String),
    ts(FullTransaction),
    peercount(u32),
    call_result(::std::vec::Vec<u8>),
    logs(::std::string::String),
    receipt(::std::string::String),
    transaction_count(u64),
    contract_code(::std::vec::Vec<u8>),
    filter_id(u64),
    uninstall_filter(bool),
    filter_changes(::std::string::String),
    filter_logs(::std::string::String),
    none(bool),
}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Response {
        static mut instance: ::protobuf::lazy::Lazy<Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Response,
        };
        unsafe {
            instance.get(Response::new)
        }
    }

    // bytes request_id = 1;

    pub fn clear_request_id(&mut self) {
        self.request_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_request_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.request_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.request_id
    }

    // Take field
    pub fn take_request_id(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.request_id, ::std::vec::Vec::new())
    }

    pub fn get_request_id(&self) -> &[u8] {
        &self.request_id
    }

    fn get_request_id_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.request_id
    }

    fn mut_request_id_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.request_id
    }

    // int64 code = 2;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: i64) {
        self.code = v;
    }

    pub fn get_code(&self) -> i64 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &i64 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut i64 {
        &mut self.code
    }

    // string error_msg = 3;

    pub fn clear_error_msg(&mut self) {
        self.data = ::std::option::Option::None;
    }

    pub fn has_error_msg(&self) -> bool {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::error_msg(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error_msg(&mut self, v: ::std::string::String) {
        self.data = ::std::option::Option::Some(Response_oneof_data::error_msg(v))
    }

    // Mutable pointer to the field.
    pub fn mut_error_msg(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Response_oneof_data::error_msg(_)) = self.data {
        } else {
            self.data = ::std::option::Option::Some(Response_oneof_data::error_msg(::std::string::String::new()));
        }
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::error_msg(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error_msg(&mut self) -> ::std::string::String {
        if self.has_error_msg() {
            match self.data.take() {
                ::std::option::Option::Some(Response_oneof_data::error_msg(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_error_msg(&self) -> &str {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::error_msg(ref v)) => v,
            _ => "",
        }
    }

    // string tx_state = 4;

    pub fn clear_tx_state(&mut self) {
        self.data = ::std::option::Option::None;
    }

    pub fn has_tx_state(&self) -> bool {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::tx_state(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tx_state(&mut self, v: ::std::string::String) {
        self.data = ::std::option::Option::Some(Response_oneof_data::tx_state(v))
    }

    // Mutable pointer to the field.
    pub fn mut_tx_state(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Response_oneof_data::tx_state(_)) = self.data {
        } else {
            self.data = ::std::option::Option::Some(Response_oneof_data::tx_state(::std::string::String::new()));
        }
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::tx_state(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_tx_state(&mut self) -> ::std::string::String {
        if self.has_tx_state() {
            match self.data.take() {
                ::std::option::Option::Some(Response_oneof_data::tx_state(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_tx_state(&self) -> &str {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::tx_state(ref v)) => v,
            _ => "",
        }
    }

    // uint64 block_number = 5;

    pub fn clear_block_number(&mut self) {
        self.data = ::std::option::Option::None;
    }

    pub fn has_block_number(&self) -> bool {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::block_number(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_block_number(&mut self, v: u64) {
        self.data = ::std::option::Option::Some(Response_oneof_data::block_number(v))
    }

    pub fn get_block_number(&self) -> u64 {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::block_number(v)) => v,
            _ => 0,
        }
    }

    // string block = 6;

    pub fn clear_block(&mut self) {
        self.data = ::std::option::Option::None;
    }

    pub fn has_block(&self) -> bool {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::block(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_block(&mut self, v: ::std::string::String) {
        self.data = ::std::option::Option::Some(Response_oneof_data::block(v))
    }

    // Mutable pointer to the field.
    pub fn mut_block(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Response_oneof_data::block(_)) = self.data {
        } else {
            self.data = ::std::option::Option::Some(Response_oneof_data::block(::std::string::String::new()));
        }
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::block(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_block(&mut self) -> ::std::string::String {
        if self.has_block() {
            match self.data.take() {
                ::std::option::Option::Some(Response_oneof_data::block(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_block(&self) -> &str {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::block(ref v)) => v,
            _ => "",
        }
    }

    // .FullTransaction ts = 7;

    pub fn clear_ts(&mut self) {
        self.data = ::std::option::Option::None;
    }

    pub fn has_ts(&self) -> bool {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::ts(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ts(&mut self, v: FullTransaction) {
        self.data = ::std::option::Option::Some(Response_oneof_data::ts(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ts(&mut self) -> &mut FullTransaction {
        if let ::std::option::Option::Some(Response_oneof_data::ts(_)) = self.data {
        } else {
            self.data = ::std::option::Option::Some(Response_oneof_data::ts(FullTransaction::new()));
        }
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::ts(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ts(&mut self) -> FullTransaction {
        if self.has_ts() {
            match self.data.take() {
                ::std::option::Option::Some(Response_oneof_data::ts(v)) => v,
                _ => panic!(),
            }
        } else {
            FullTransaction::new()
        }
    }

    pub fn get_ts(&self) -> &FullTransaction {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::ts(ref v)) => v,
            _ => FullTransaction::default_instance(),
        }
    }

    // uint32 peercount = 8;

    pub fn clear_peercount(&mut self) {
        self.data = ::std::option::Option::None;
    }

    pub fn has_peercount(&self) -> bool {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::peercount(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_peercount(&mut self, v: u32) {
        self.data = ::std::option::Option::Some(Response_oneof_data::peercount(v))
    }

    pub fn get_peercount(&self) -> u32 {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::peercount(v)) => v,
            _ => 0,
        }
    }

    // bytes call_result = 9;

    pub fn clear_call_result(&mut self) {
        self.data = ::std::option::Option::None;
    }

    pub fn has_call_result(&self) -> bool {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::call_result(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_call_result(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::std::option::Option::Some(Response_oneof_data::call_result(v))
    }

    // Mutable pointer to the field.
    pub fn mut_call_result(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(Response_oneof_data::call_result(_)) = self.data {
        } else {
            self.data = ::std::option::Option::Some(Response_oneof_data::call_result(::std::vec::Vec::new()));
        }
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::call_result(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_call_result(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_call_result() {
            match self.data.take() {
                ::std::option::Option::Some(Response_oneof_data::call_result(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_call_result(&self) -> &[u8] {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::call_result(ref v)) => v,
            _ => &[],
        }
    }

    // string logs = 10;

    pub fn clear_logs(&mut self) {
        self.data = ::std::option::Option::None;
    }

    pub fn has_logs(&self) -> bool {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::logs(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_logs(&mut self, v: ::std::string::String) {
        self.data = ::std::option::Option::Some(Response_oneof_data::logs(v))
    }

    // Mutable pointer to the field.
    pub fn mut_logs(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Response_oneof_data::logs(_)) = self.data {
        } else {
            self.data = ::std::option::Option::Some(Response_oneof_data::logs(::std::string::String::new()));
        }
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::logs(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_logs(&mut self) -> ::std::string::String {
        if self.has_logs() {
            match self.data.take() {
                ::std::option::Option::Some(Response_oneof_data::logs(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_logs(&self) -> &str {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::logs(ref v)) => v,
            _ => "",
        }
    }

    // string receipt = 11;

    pub fn clear_receipt(&mut self) {
        self.data = ::std::option::Option::None;
    }

    pub fn has_receipt(&self) -> bool {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::receipt(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_receipt(&mut self, v: ::std::string::String) {
        self.data = ::std::option::Option::Some(Response_oneof_data::receipt(v))
    }

    // Mutable pointer to the field.
    pub fn mut_receipt(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Response_oneof_data::receipt(_)) = self.data {
        } else {
            self.data = ::std::option::Option::Some(Response_oneof_data::receipt(::std::string::String::new()));
        }
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::receipt(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_receipt(&mut self) -> ::std::string::String {
        if self.has_receipt() {
            match self.data.take() {
                ::std::option::Option::Some(Response_oneof_data::receipt(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_receipt(&self) -> &str {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::receipt(ref v)) => v,
            _ => "",
        }
    }

    // uint64 transaction_count = 12;

    pub fn clear_transaction_count(&mut self) {
        self.data = ::std::option::Option::None;
    }

    pub fn has_transaction_count(&self) -> bool {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::transaction_count(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_transaction_count(&mut self, v: u64) {
        self.data = ::std::option::Option::Some(Response_oneof_data::transaction_count(v))
    }

    pub fn get_transaction_count(&self) -> u64 {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::transaction_count(v)) => v,
            _ => 0,
        }
    }

    // bytes contract_code = 13;

    pub fn clear_contract_code(&mut self) {
        self.data = ::std::option::Option::None;
    }

    pub fn has_contract_code(&self) -> bool {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::contract_code(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_contract_code(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::std::option::Option::Some(Response_oneof_data::contract_code(v))
    }

    // Mutable pointer to the field.
    pub fn mut_contract_code(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(Response_oneof_data::contract_code(_)) = self.data {
        } else {
            self.data = ::std::option::Option::Some(Response_oneof_data::contract_code(::std::vec::Vec::new()));
        }
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::contract_code(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_contract_code(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_contract_code() {
            match self.data.take() {
                ::std::option::Option::Some(Response_oneof_data::contract_code(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_contract_code(&self) -> &[u8] {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::contract_code(ref v)) => v,
            _ => &[],
        }
    }

    // uint64 filter_id = 14;

    pub fn clear_filter_id(&mut self) {
        self.data = ::std::option::Option::None;
    }

    pub fn has_filter_id(&self) -> bool {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::filter_id(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_filter_id(&mut self, v: u64) {
        self.data = ::std::option::Option::Some(Response_oneof_data::filter_id(v))
    }

    pub fn get_filter_id(&self) -> u64 {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::filter_id(v)) => v,
            _ => 0,
        }
    }

    // bool uninstall_filter = 15;

    pub fn clear_uninstall_filter(&mut self) {
        self.data = ::std::option::Option::None;
    }

    pub fn has_uninstall_filter(&self) -> bool {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::uninstall_filter(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_uninstall_filter(&mut self, v: bool) {
        self.data = ::std::option::Option::Some(Response_oneof_data::uninstall_filter(v))
    }

    pub fn get_uninstall_filter(&self) -> bool {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::uninstall_filter(v)) => v,
            _ => false,
        }
    }

    // string filter_changes = 16;

    pub fn clear_filter_changes(&mut self) {
        self.data = ::std::option::Option::None;
    }

    pub fn has_filter_changes(&self) -> bool {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::filter_changes(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_filter_changes(&mut self, v: ::std::string::String) {
        self.data = ::std::option::Option::Some(Response_oneof_data::filter_changes(v))
    }

    // Mutable pointer to the field.
    pub fn mut_filter_changes(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Response_oneof_data::filter_changes(_)) = self.data {
        } else {
            self.data = ::std::option::Option::Some(Response_oneof_data::filter_changes(::std::string::String::new()));
        }
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::filter_changes(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_filter_changes(&mut self) -> ::std::string::String {
        if self.has_filter_changes() {
            match self.data.take() {
                ::std::option::Option::Some(Response_oneof_data::filter_changes(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_filter_changes(&self) -> &str {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::filter_changes(ref v)) => v,
            _ => "",
        }
    }

    // string filter_logs = 17;

    pub fn clear_filter_logs(&mut self) {
        self.data = ::std::option::Option::None;
    }

    pub fn has_filter_logs(&self) -> bool {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::filter_logs(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_filter_logs(&mut self, v: ::std::string::String) {
        self.data = ::std::option::Option::Some(Response_oneof_data::filter_logs(v))
    }

    // Mutable pointer to the field.
    pub fn mut_filter_logs(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Response_oneof_data::filter_logs(_)) = self.data {
        } else {
            self.data = ::std::option::Option::Some(Response_oneof_data::filter_logs(::std::string::String::new()));
        }
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::filter_logs(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_filter_logs(&mut self) -> ::std::string::String {
        if self.has_filter_logs() {
            match self.data.take() {
                ::std::option::Option::Some(Response_oneof_data::filter_logs(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_filter_logs(&self) -> &str {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::filter_logs(ref v)) => v,
            _ => "",
        }
    }

    // bool none = 18;

    pub fn clear_none(&mut self) {
        self.data = ::std::option::Option::None;
    }

    pub fn has_none(&self) -> bool {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::none(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_none(&mut self, v: bool) {
        self.data = ::std::option::Option::Some(Response_oneof_data::none(v))
    }

    pub fn get_none(&self) -> bool {
        match self.data {
            ::std::option::Option::Some(Response_oneof_data::none(v)) => v,
            _ => false,
        }
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.request_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.code = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.data = ::std::option::Option::Some(Response_oneof_data::error_msg(is.read_string()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.data = ::std::option::Option::Some(Response_oneof_data::tx_state(is.read_string()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.data = ::std::option::Option::Some(Response_oneof_data::block_number(is.read_uint64()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.data = ::std::option::Option::Some(Response_oneof_data::block(is.read_string()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.data = ::std::option::Option::Some(Response_oneof_data::ts(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.data = ::std::option::Option::Some(Response_oneof_data::peercount(is.read_uint32()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.data = ::std::option::Option::Some(Response_oneof_data::call_result(is.read_bytes()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.data = ::std::option::Option::Some(Response_oneof_data::logs(is.read_string()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.data = ::std::option::Option::Some(Response_oneof_data::receipt(is.read_string()?));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.data = ::std::option::Option::Some(Response_oneof_data::transaction_count(is.read_uint64()?));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.data = ::std::option::Option::Some(Response_oneof_data::contract_code(is.read_bytes()?));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.data = ::std::option::Option::Some(Response_oneof_data::filter_id(is.read_uint64()?));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.data = ::std::option::Option::Some(Response_oneof_data::uninstall_filter(is.read_bool()?));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.data = ::std::option::Option::Some(Response_oneof_data::filter_changes(is.read_string()?));
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.data = ::std::option::Option::Some(Response_oneof_data::filter_logs(is.read_string()?));
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.data = ::std::option::Option::Some(Response_oneof_data::none(is.read_bool()?));
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
        if !self.request_id.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.request_id);
        };
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(2, self.code, ::protobuf::wire_format::WireTypeVarint);
        };
        if let ::std::option::Option::Some(ref v) = self.data {
            match v {
                &Response_oneof_data::error_msg(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
                &Response_oneof_data::tx_state(ref v) => {
                    my_size += ::protobuf::rt::string_size(4, &v);
                },
                &Response_oneof_data::block_number(v) => {
                    my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Response_oneof_data::block(ref v) => {
                    my_size += ::protobuf::rt::string_size(6, &v);
                },
                &Response_oneof_data::ts(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_data::peercount(v) => {
                    my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Response_oneof_data::call_result(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(9, &v);
                },
                &Response_oneof_data::logs(ref v) => {
                    my_size += ::protobuf::rt::string_size(10, &v);
                },
                &Response_oneof_data::receipt(ref v) => {
                    my_size += ::protobuf::rt::string_size(11, &v);
                },
                &Response_oneof_data::transaction_count(v) => {
                    my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Response_oneof_data::contract_code(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(13, &v);
                },
                &Response_oneof_data::filter_id(v) => {
                    my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Response_oneof_data::uninstall_filter(v) => {
                    my_size += 2;
                },
                &Response_oneof_data::filter_changes(ref v) => {
                    my_size += ::protobuf::rt::string_size(16, &v);
                },
                &Response_oneof_data::filter_logs(ref v) => {
                    my_size += ::protobuf::rt::string_size(17, &v);
                },
                &Response_oneof_data::none(v) => {
                    my_size += 3;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.request_id.is_empty() {
            os.write_bytes(1, &self.request_id)?;
        };
        if self.code != 0 {
            os.write_int64(2, self.code)?;
        };
        if let ::std::option::Option::Some(ref v) = self.data {
            match v {
                &Response_oneof_data::error_msg(ref v) => {
                    os.write_string(3, v)?;
                },
                &Response_oneof_data::tx_state(ref v) => {
                    os.write_string(4, v)?;
                },
                &Response_oneof_data::block_number(v) => {
                    os.write_uint64(5, v)?;
                },
                &Response_oneof_data::block(ref v) => {
                    os.write_string(6, v)?;
                },
                &Response_oneof_data::ts(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_data::peercount(v) => {
                    os.write_uint32(8, v)?;
                },
                &Response_oneof_data::call_result(ref v) => {
                    os.write_bytes(9, v)?;
                },
                &Response_oneof_data::logs(ref v) => {
                    os.write_string(10, v)?;
                },
                &Response_oneof_data::receipt(ref v) => {
                    os.write_string(11, v)?;
                },
                &Response_oneof_data::transaction_count(v) => {
                    os.write_uint64(12, v)?;
                },
                &Response_oneof_data::contract_code(ref v) => {
                    os.write_bytes(13, v)?;
                },
                &Response_oneof_data::filter_id(v) => {
                    os.write_uint64(14, v)?;
                },
                &Response_oneof_data::uninstall_filter(v) => {
                    os.write_bool(15, v)?;
                },
                &Response_oneof_data::filter_changes(ref v) => {
                    os.write_string(16, v)?;
                },
                &Response_oneof_data::filter_logs(ref v) => {
                    os.write_string(17, v)?;
                },
                &Response_oneof_data::none(v) => {
                    os.write_bool(18, v)?;
                },
            };
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

impl ::protobuf::MessageStatic for Response {
    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "request_id",
                    Response::get_request_id_for_reflect,
                    Response::mut_request_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "code",
                    Response::get_code_for_reflect,
                    Response::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "error_msg",
                    Response::has_error_msg,
                    Response::get_error_msg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "tx_state",
                    Response::has_tx_state,
                    Response::get_tx_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "block_number",
                    Response::has_block_number,
                    Response::get_block_number,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "block",
                    Response::has_block,
                    Response::get_block,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, FullTransaction>(
                    "ts",
                    Response::has_ts,
                    Response::get_ts,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor::<_>(
                    "peercount",
                    Response::has_peercount,
                    Response::get_peercount,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "call_result",
                    Response::has_call_result,
                    Response::get_call_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "logs",
                    Response::has_logs,
                    Response::get_logs,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "receipt",
                    Response::has_receipt,
                    Response::get_receipt,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "transaction_count",
                    Response::has_transaction_count,
                    Response::get_transaction_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "contract_code",
                    Response::has_contract_code,
                    Response::get_contract_code,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "filter_id",
                    Response::has_filter_id,
                    Response::get_filter_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "uninstall_filter",
                    Response::has_uninstall_filter,
                    Response::get_uninstall_filter,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "filter_changes",
                    Response::has_filter_changes,
                    Response::get_filter_changes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "filter_logs",
                    Response::has_filter_logs,
                    Response::get_filter_logs,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "none",
                    Response::has_none,
                    Response::get_none,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Response>(
                    "Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.clear_request_id();
        self.clear_code();
        self.clear_error_msg();
        self.clear_tx_state();
        self.clear_block_number();
        self.clear_block();
        self.clear_ts();
        self.clear_peercount();
        self.clear_call_result();
        self.clear_logs();
        self.clear_receipt();
        self.clear_transaction_count();
        self.clear_contract_code();
        self.clear_filter_id();
        self.clear_uninstall_filter();
        self.clear_filter_changes();
        self.clear_filter_logs();
        self.clear_none();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0e, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x1a, 0x10, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0x9f, 0x01, 0x0a, 0x0f, 0x46, 0x75, 0x6c, 0x6c, 0x54, 0x72, 0x61, 0x6e, 0x73,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x34, 0x0a, 0x0b, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x53, 0x69,
    0x67, 0x6e, 0x65, 0x64, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52,
    0x0b, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x21, 0x0a, 0x0c,
    0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x0b, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x12,
    0x1d, 0x0a, 0x0a, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x09, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x48, 0x61, 0x73, 0x68, 0x12, 0x14,
    0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x69,
    0x6e, 0x64, 0x65, 0x78, 0x22, 0xdb, 0x04, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x49, 0x64,
    0x12, 0x12, 0x0a, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x04,
    0x63, 0x6f, 0x64, 0x65, 0x12, 0x1d, 0x0a, 0x09, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x5f, 0x6d, 0x73,
    0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x08, 0x65, 0x72, 0x72, 0x6f, 0x72,
    0x4d, 0x73, 0x67, 0x12, 0x1b, 0x0a, 0x08, 0x74, 0x78, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x07, 0x74, 0x78, 0x53, 0x74, 0x61, 0x74, 0x65,
    0x12, 0x23, 0x0a, 0x0c, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x48, 0x00, 0x52, 0x0b, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x4e,
    0x75, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x16, 0x0a, 0x05, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x05, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x22, 0x0a,
    0x02, 0x74, 0x73, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x46, 0x75, 0x6c, 0x6c,
    0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52, 0x02, 0x74,
    0x73, 0x12, 0x1e, 0x0a, 0x09, 0x70, 0x65, 0x65, 0x72, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x08,
    0x20, 0x01, 0x28, 0x0d, 0x48, 0x00, 0x52, 0x09, 0x70, 0x65, 0x65, 0x72, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x12, 0x21, 0x0a, 0x0b, 0x63, 0x61, 0x6c, 0x6c, 0x5f, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x18, 0x09, 0x20, 0x01, 0x28, 0x0c, 0x48, 0x00, 0x52, 0x0a, 0x63, 0x61, 0x6c, 0x6c, 0x52, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x12, 0x14, 0x0a, 0x04, 0x6c, 0x6f, 0x67, 0x73, 0x18, 0x0a, 0x20, 0x01,
    0x28, 0x09, 0x48, 0x00, 0x52, 0x04, 0x6c, 0x6f, 0x67, 0x73, 0x12, 0x1a, 0x0a, 0x07, 0x72, 0x65,
    0x63, 0x65, 0x69, 0x70, 0x74, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x07, 0x72,
    0x65, 0x63, 0x65, 0x69, 0x70, 0x74, 0x12, 0x2d, 0x0a, 0x11, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x0c, 0x20, 0x01, 0x28,
    0x04, 0x48, 0x00, 0x52, 0x10, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x25, 0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63,
    0x74, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0c, 0x48, 0x00, 0x52, 0x0c,
    0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x43, 0x6f, 0x64, 0x65, 0x12, 0x1d, 0x0a, 0x09,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x04, 0x48,
    0x00, 0x52, 0x08, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x49, 0x64, 0x12, 0x2b, 0x0a, 0x10, 0x75,
    0x6e, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x18,
    0x0f, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x0f, 0x75, 0x6e, 0x69, 0x6e, 0x73, 0x74, 0x61,
    0x6c, 0x6c, 0x46, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x12, 0x27, 0x0a, 0x0e, 0x66, 0x69, 0x6c, 0x74,
    0x65, 0x72, 0x5f, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x18, 0x10, 0x20, 0x01, 0x28, 0x09,
    0x48, 0x00, 0x52, 0x0d, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65,
    0x73, 0x12, 0x21, 0x0a, 0x0b, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x5f, 0x6c, 0x6f, 0x67, 0x73,
    0x18, 0x11, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0a, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72,
    0x4c, 0x6f, 0x67, 0x73, 0x12, 0x14, 0x0a, 0x04, 0x6e, 0x6f, 0x6e, 0x65, 0x18, 0x12, 0x20, 0x01,
    0x28, 0x08, 0x48, 0x00, 0x52, 0x04, 0x6e, 0x6f, 0x6e, 0x65, 0x42, 0x06, 0x0a, 0x04, 0x64, 0x61,
    0x74, 0x61, 0x4a, 0xa0, 0x0e, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x29, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x02,
    0x07, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00, 0x09, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x05, 0x04, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x05, 0x04, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x05, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05,
    0x16, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x24, 0x25,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06, 0x04, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x06, 0x04, 0x05, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x06, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x06, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x07, 0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x07, 0x04,
    0x06, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x07, 0x04, 0x09,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x07, 0x0a, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x07, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x08, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x04, 0x12, 0x04, 0x08, 0x04, 0x07, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x08, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x08, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x08,
    0x13, 0x14, 0x0a, 0xcc, 0x01, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x14, 0x00, 0x29, 0x01, 0x32,
    0xbf, 0x01, 0x70, 0x75, 0x62, 0x20, 0x6d, 0x6f, 0x64, 0x20, 0x73, 0x75, 0x62, 0x6d, 0x6f, 0x64,
    0x75, 0x6c, 0x65, 0x73, 0x20, 0x7b, 0x0a, 0x70, 0x75, 0x62, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x74,
    0x20, 0x4a, 0x53, 0x4f, 0x4e, 0x5f, 0x52, 0x50, 0x43, 0x3a, 0x20, 0x75, 0x33, 0x32, 0x20, 0x3d,
    0x20, 0x31, 0x3b, 0x0a, 0x70, 0x75, 0x62, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x74, 0x20, 0x4e, 0x45,
    0x54, 0x3a, 0x20, 0x75, 0x33, 0x32, 0x20, 0x3d, 0x20, 0x32, 0x3b, 0x0a, 0x70, 0x75, 0x62, 0x20,
    0x63, 0x6f, 0x6e, 0x73, 0x74, 0x20, 0x43, 0x48, 0x41, 0x49, 0x4e, 0x3a, 0x20, 0x75, 0x33, 0x32,
    0x20, 0x3d, 0x20, 0x33, 0x3b, 0x0a, 0x70, 0x75, 0x62, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x74, 0x20,
    0x43, 0x4f, 0x4e, 0x53, 0x45, 0x4e, 0x53, 0x55, 0x53, 0x3a, 0x20, 0x75, 0x33, 0x32, 0x20, 0x3d,
    0x20, 0x34, 0x3b, 0x0a, 0x70, 0x75, 0x62, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x74, 0x20, 0x43, 0x4f,
    0x4e, 0x53, 0x45, 0x4e, 0x53, 0x55, 0x53, 0x5f, 0x43, 0x4d, 0x44, 0x3a, 0x20, 0x75, 0x33, 0x32,
    0x20, 0x3d, 0x20, 0x35, 0x3b, 0x0a, 0x70, 0x75, 0x62, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x74, 0x20,
    0x41, 0x55, 0x54, 0x48, 0x3a, 0x20, 0x75, 0x33, 0x32, 0x20, 0x3d, 0x20, 0x36, 0x3b, 0x0a, 0x7d,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x14, 0x08, 0x10, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x15, 0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x15, 0x04, 0x14, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x15, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x15, 0x0a, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x15, 0x17, 0x18, 0x0a, 0xea, 0x01, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x16, 0x04,
    0x13, 0x22, 0xdc, 0x01, 0xe8, 0xbf, 0x99, 0xe4, 0xb8, 0xaa, 0xe5, 0xae, 0x9a, 0xe4, 0xb9, 0x89,
    0xe5, 0x8f, 0xaf, 0xe4, 0xbb, 0xa5, 0xe6, 0xa0, 0xb9, 0xe6, 0x8d, 0xae, 0xe6, 0xa8, 0xa1, 0xe5,
    0x9d, 0x97, 0xe7, 0x9a, 0x84, 0x73, 0x75, 0x62, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x73, 0xe7,
    0x9a, 0x84, 0xe5, 0x80, 0xbc, 0xe6, 0x9d, 0xa5, 0xe5, 0x88, 0x92, 0xe5, 0x88, 0x86, 0xef, 0xbc,
    0x8c, 0xe7, 0xb2, 0x97, 0xe5, 0x88, 0x92, 0xe5, 0x88, 0x86, 0xef, 0xbc, 0x9a, 0x30, 0xef, 0xbc,
    0x9a, 0xe6, 0xad, 0xa3, 0xe7, 0xa1, 0xae, 0xef, 0xbc, 0x8c, 0xe5, 0x85, 0xb6, 0xe4, 0xbb, 0x96,
    0xe8, 0xa1, 0xa8, 0xe7, 0xa4, 0xba, 0xe9, 0x94, 0x99, 0xe8, 0xaf, 0xaf, 0xef, 0xbc, 0x8c, 0x32,
    0xef, 0xbc, 0x9a, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0xe7, 0x9a, 0x84, 0xe9, 0x94, 0x99,
    0xe8, 0xaf, 0xaf, 0xef, 0xbc, 0x8c, 0x33, 0xef, 0xbc, 0x9a, 0x63, 0x68, 0x61, 0x69, 0x6e, 0xe7,
    0x9a, 0x84, 0xe9, 0x94, 0x99, 0xe8, 0xaf, 0xaf, 0xef, 0xbc, 0x8c, 0x34, 0xef, 0xbc, 0x9a, 0x63,
    0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0xe9, 0x94, 0x99, 0xe8, 0xaf, 0xaf, 0xef, 0xbc,
    0x8c, 0x36, 0x3a, 0x61, 0x75, 0x74, 0x68, 0xe9, 0x94, 0x99, 0xe8, 0xaf, 0xaf, 0xe3, 0x80, 0x82,
    0xe6, 0xad, 0xa3, 0xe5, 0xa5, 0xbd, 0xe5, 0xaf, 0xb9, 0xe5, 0xba, 0x94, 0xe6, 0xa8, 0xa1, 0xe5,
    0x9d, 0x97, 0xe7, 0x9a, 0x84, 0x73, 0x75, 0x62, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x73, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04, 0x16, 0x04, 0x15, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x16, 0x04, 0x09, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x16, 0x0a, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x16, 0x11, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x08,
    0x00, 0x12, 0x04, 0x17, 0x04, 0x28, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x08, 0x00, 0x01,
    0x12, 0x03, 0x17, 0x0a, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x18,
    0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x18, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x18, 0x0f, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x18, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x19, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x19, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x19, 0x0f, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x19, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x1a, 0x08, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x1a, 0x08, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1a, 0x0f, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1a, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x05, 0x12, 0x03, 0x1b, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05,
    0x12, 0x03, 0x1b, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x1b, 0x0f, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x1b, 0x17,
    0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x1c, 0x08, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x06, 0x12, 0x03, 0x1c, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1c, 0x18, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x06, 0x03, 0x12, 0x03, 0x1c, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x07,
    0x12, 0x03, 0x1d, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x05, 0x12, 0x03,
    0x1d, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x01, 0x12, 0x03, 0x1d, 0x0f,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x03, 0x12, 0x03, 0x1d, 0x1b, 0x1c, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x08, 0x12, 0x03, 0x1e, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x08, 0x05, 0x12, 0x03, 0x1e, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x08, 0x01, 0x12, 0x03, 0x1e, 0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08,
    0x03, 0x12, 0x03, 0x1e, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x09, 0x12, 0x03,
    0x1f, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x05, 0x12, 0x03, 0x1f, 0x08,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x01, 0x12, 0x03, 0x1f, 0x0f, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x03, 0x12, 0x03, 0x1f, 0x16, 0x18, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x0a, 0x12, 0x03, 0x20, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x0a, 0x05, 0x12, 0x03, 0x20, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a,
    0x01, 0x12, 0x03, 0x20, 0x0f, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x03, 0x12,
    0x03, 0x20, 0x19, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0b, 0x12, 0x03, 0x21, 0x08,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x21, 0x08, 0x0e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x21, 0x0f, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x21, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x0c, 0x12, 0x03, 0x22, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c,
    0x05, 0x12, 0x03, 0x22, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c, 0x01, 0x12,
    0x03, 0x22, 0x0e, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x22,
    0x1e, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0d, 0x12, 0x03, 0x23, 0x08, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x23, 0x08, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x23, 0x0f, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x23, 0x1b, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x0e, 0x12, 0x03, 0x24, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e, 0x05, 0x12,
    0x03, 0x24, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x24,
    0x0d, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x24, 0x20, 0x22,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0f, 0x12, 0x03, 0x25, 0x08, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x0f, 0x05, 0x12, 0x03, 0x25, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x25, 0x0f, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x0f, 0x03, 0x12, 0x03, 0x25, 0x20, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x10, 0x12,
    0x03, 0x26, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x10, 0x05, 0x12, 0x03, 0x26,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x10, 0x01, 0x12, 0x03, 0x26, 0x0f, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x10, 0x03, 0x12, 0x03, 0x26, 0x1d, 0x1f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x11, 0x12, 0x03, 0x27, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x11, 0x05, 0x12, 0x03, 0x27, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x11, 0x01, 0x12, 0x03, 0x27, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x11, 0x03,
    0x12, 0x03, 0x27, 0x14, 0x16, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];

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
