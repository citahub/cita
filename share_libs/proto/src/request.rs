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
pub struct Call {
    // message fields
    pub from: ::std::vec::Vec<u8>,
    pub to: ::std::vec::Vec<u8>,
    pub data: ::std::vec::Vec<u8>,
    pub height: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Call {}

impl Call {
    pub fn new() -> Call {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Call {
        static mut instance: ::protobuf::lazy::Lazy<Call> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Call,
        };
        unsafe {
            instance.get(Call::new)
        }
    }

    // bytes from = 1;

    pub fn clear_from(&mut self) {
        self.from.clear();
    }

    // Param is passed by value, moved
    pub fn set_from(&mut self, v: ::std::vec::Vec<u8>) {
        self.from = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.from
    }

    // Take field
    pub fn take_from(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.from, ::std::vec::Vec::new())
    }

    pub fn get_from(&self) -> &[u8] {
        &self.from
    }

    fn get_from_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.from
    }

    fn mut_from_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.from
    }

    // bytes to = 2;

    pub fn clear_to(&mut self) {
        self.to.clear();
    }

    // Param is passed by value, moved
    pub fn set_to(&mut self, v: ::std::vec::Vec<u8>) {
        self.to = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_to(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.to
    }

    // Take field
    pub fn take_to(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.to, ::std::vec::Vec::new())
    }

    pub fn get_to(&self) -> &[u8] {
        &self.to
    }

    fn get_to_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.to
    }

    fn mut_to_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.to
    }

    // bytes data = 3;

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

    // string height = 4;

    pub fn clear_height(&mut self) {
        self.height.clear();
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: ::std::string::String) {
        self.height = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_height(&mut self) -> &mut ::std::string::String {
        &mut self.height
    }

    // Take field
    pub fn take_height(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.height, ::std::string::String::new())
    }

    pub fn get_height(&self) -> &str {
        &self.height
    }

    fn get_height_for_reflect(&self) -> &::std::string::String {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.height
    }
}

impl ::protobuf::Message for Call {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.from)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.to)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.height)?;
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
        if !self.from.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.from);
        }
        if !self.to.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.to);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.data);
        }
        if !self.height.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.height);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.from.is_empty() {
            os.write_bytes(1, &self.from)?;
        }
        if !self.to.is_empty() {
            os.write_bytes(2, &self.to)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(3, &self.data)?;
        }
        if !self.height.is_empty() {
            os.write_string(4, &self.height)?;
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

impl ::protobuf::MessageStatic for Call {
    fn new() -> Call {
        Call::new()
    }

    fn descriptor_static(_: ::std::option::Option<Call>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "from",
                    Call::get_from_for_reflect,
                    Call::mut_from_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "to",
                    Call::get_to_for_reflect,
                    Call::mut_to_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    Call::get_data_for_reflect,
                    Call::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "height",
                    Call::get_height_for_reflect,
                    Call::mut_height_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Call>(
                    "Call",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Call {
    fn clear(&mut self) {
        self.clear_from();
        self.clear_to();
        self.clear_data();
        self.clear_height();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Call {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Call {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Request {
    // message fields
    pub request_id: ::std::vec::Vec<u8>,
    // message oneof groups
    pub req: ::std::option::Option<Request_oneof_req>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Request {}

#[derive(Clone,PartialEq)]
pub enum Request_oneof_req {
    block_number(bool),
    block_by_hash(::std::string::String),
    block_by_height(::std::string::String),
    transaction(::std::vec::Vec<u8>),
    height(u64),
    peercount(bool),
    call(Call),
    filter(::std::string::String),
    transaction_receipt(::std::vec::Vec<u8>),
    transaction_count(::std::string::String),
    code(::std::string::String),
    new_filter(::std::string::String),
    new_block_filter(bool),
    uninstall_filter(u64),
    filter_changes(u64),
    filter_logs(u64),
    un_tx(super::blockchain::UnverifiedTransaction),
    batch_req(BatchRequest),
}

impl Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Request {
        static mut instance: ::protobuf::lazy::Lazy<Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Request,
        };
        unsafe {
            instance.get(Request::new)
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

    // bool block_number = 2;

    pub fn clear_block_number(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_block_number(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::block_number(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_block_number(&mut self, v: bool) {
        self.req = ::std::option::Option::Some(Request_oneof_req::block_number(v))
    }

    pub fn get_block_number(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::block_number(v)) => v,
            _ => false,
        }
    }

    // string block_by_hash = 3;

    pub fn clear_block_by_hash(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_block_by_hash(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::block_by_hash(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_block_by_hash(&mut self, v: ::std::string::String) {
        self.req = ::std::option::Option::Some(Request_oneof_req::block_by_hash(v))
    }

    // Mutable pointer to the field.
    pub fn mut_block_by_hash(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Request_oneof_req::block_by_hash(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(Request_oneof_req::block_by_hash(::std::string::String::new()));
        }
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::block_by_hash(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_block_by_hash(&mut self) -> ::std::string::String {
        if self.has_block_by_hash() {
            match self.req.take() {
                ::std::option::Option::Some(Request_oneof_req::block_by_hash(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_block_by_hash(&self) -> &str {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::block_by_hash(ref v)) => v,
            _ => "",
        }
    }

    // string block_by_height = 4;

    pub fn clear_block_by_height(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_block_by_height(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::block_by_height(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_block_by_height(&mut self, v: ::std::string::String) {
        self.req = ::std::option::Option::Some(Request_oneof_req::block_by_height(v))
    }

    // Mutable pointer to the field.
    pub fn mut_block_by_height(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Request_oneof_req::block_by_height(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(Request_oneof_req::block_by_height(::std::string::String::new()));
        }
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::block_by_height(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_block_by_height(&mut self) -> ::std::string::String {
        if self.has_block_by_height() {
            match self.req.take() {
                ::std::option::Option::Some(Request_oneof_req::block_by_height(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_block_by_height(&self) -> &str {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::block_by_height(ref v)) => v,
            _ => "",
        }
    }

    // bytes transaction = 5;

    pub fn clear_transaction(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_transaction(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::transaction(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_transaction(&mut self, v: ::std::vec::Vec<u8>) {
        self.req = ::std::option::Option::Some(Request_oneof_req::transaction(v))
    }

    // Mutable pointer to the field.
    pub fn mut_transaction(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(Request_oneof_req::transaction(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(Request_oneof_req::transaction(::std::vec::Vec::new()));
        }
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::transaction(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_transaction(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_transaction() {
            match self.req.take() {
                ::std::option::Option::Some(Request_oneof_req::transaction(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_transaction(&self) -> &[u8] {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::transaction(ref v)) => v,
            _ => &[],
        }
    }

    // uint64 height = 6;

    pub fn clear_height(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::height(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u64) {
        self.req = ::std::option::Option::Some(Request_oneof_req::height(v))
    }

    pub fn get_height(&self) -> u64 {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::height(v)) => v,
            _ => 0,
        }
    }

    // bool peercount = 7;

    pub fn clear_peercount(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_peercount(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::peercount(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_peercount(&mut self, v: bool) {
        self.req = ::std::option::Option::Some(Request_oneof_req::peercount(v))
    }

    pub fn get_peercount(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::peercount(v)) => v,
            _ => false,
        }
    }

    // .Call call = 8;

    pub fn clear_call(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_call(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::call(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_call(&mut self, v: Call) {
        self.req = ::std::option::Option::Some(Request_oneof_req::call(v))
    }

    // Mutable pointer to the field.
    pub fn mut_call(&mut self) -> &mut Call {
        if let ::std::option::Option::Some(Request_oneof_req::call(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(Request_oneof_req::call(Call::new()));
        }
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::call(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_call(&mut self) -> Call {
        if self.has_call() {
            match self.req.take() {
                ::std::option::Option::Some(Request_oneof_req::call(v)) => v,
                _ => panic!(),
            }
        } else {
            Call::new()
        }
    }

    pub fn get_call(&self) -> &Call {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::call(ref v)) => v,
            _ => Call::default_instance(),
        }
    }

    // string filter = 9;

    pub fn clear_filter(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_filter(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::filter(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_filter(&mut self, v: ::std::string::String) {
        self.req = ::std::option::Option::Some(Request_oneof_req::filter(v))
    }

    // Mutable pointer to the field.
    pub fn mut_filter(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Request_oneof_req::filter(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(Request_oneof_req::filter(::std::string::String::new()));
        }
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::filter(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_filter(&mut self) -> ::std::string::String {
        if self.has_filter() {
            match self.req.take() {
                ::std::option::Option::Some(Request_oneof_req::filter(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_filter(&self) -> &str {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::filter(ref v)) => v,
            _ => "",
        }
    }

    // bytes transaction_receipt = 10;

    pub fn clear_transaction_receipt(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_transaction_receipt(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::transaction_receipt(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_transaction_receipt(&mut self, v: ::std::vec::Vec<u8>) {
        self.req = ::std::option::Option::Some(Request_oneof_req::transaction_receipt(v))
    }

    // Mutable pointer to the field.
    pub fn mut_transaction_receipt(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(Request_oneof_req::transaction_receipt(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(Request_oneof_req::transaction_receipt(::std::vec::Vec::new()));
        }
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::transaction_receipt(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_transaction_receipt(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_transaction_receipt() {
            match self.req.take() {
                ::std::option::Option::Some(Request_oneof_req::transaction_receipt(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_transaction_receipt(&self) -> &[u8] {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::transaction_receipt(ref v)) => v,
            _ => &[],
        }
    }

    // string transaction_count = 11;

    pub fn clear_transaction_count(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_transaction_count(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::transaction_count(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_transaction_count(&mut self, v: ::std::string::String) {
        self.req = ::std::option::Option::Some(Request_oneof_req::transaction_count(v))
    }

    // Mutable pointer to the field.
    pub fn mut_transaction_count(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Request_oneof_req::transaction_count(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(Request_oneof_req::transaction_count(::std::string::String::new()));
        }
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::transaction_count(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_transaction_count(&mut self) -> ::std::string::String {
        if self.has_transaction_count() {
            match self.req.take() {
                ::std::option::Option::Some(Request_oneof_req::transaction_count(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_transaction_count(&self) -> &str {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::transaction_count(ref v)) => v,
            _ => "",
        }
    }

    // string code = 12;

    pub fn clear_code(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_code(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::code(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: ::std::string::String) {
        self.req = ::std::option::Option::Some(Request_oneof_req::code(v))
    }

    // Mutable pointer to the field.
    pub fn mut_code(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Request_oneof_req::code(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(Request_oneof_req::code(::std::string::String::new()));
        }
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::code(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_code(&mut self) -> ::std::string::String {
        if self.has_code() {
            match self.req.take() {
                ::std::option::Option::Some(Request_oneof_req::code(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_code(&self) -> &str {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::code(ref v)) => v,
            _ => "",
        }
    }

    // string new_filter = 13;

    pub fn clear_new_filter(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_new_filter(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::new_filter(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_new_filter(&mut self, v: ::std::string::String) {
        self.req = ::std::option::Option::Some(Request_oneof_req::new_filter(v))
    }

    // Mutable pointer to the field.
    pub fn mut_new_filter(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Request_oneof_req::new_filter(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(Request_oneof_req::new_filter(::std::string::String::new()));
        }
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::new_filter(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_new_filter(&mut self) -> ::std::string::String {
        if self.has_new_filter() {
            match self.req.take() {
                ::std::option::Option::Some(Request_oneof_req::new_filter(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_new_filter(&self) -> &str {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::new_filter(ref v)) => v,
            _ => "",
        }
    }

    // bool new_block_filter = 14;

    pub fn clear_new_block_filter(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_new_block_filter(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::new_block_filter(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_new_block_filter(&mut self, v: bool) {
        self.req = ::std::option::Option::Some(Request_oneof_req::new_block_filter(v))
    }

    pub fn get_new_block_filter(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::new_block_filter(v)) => v,
            _ => false,
        }
    }

    // uint64 uninstall_filter = 15;

    pub fn clear_uninstall_filter(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_uninstall_filter(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::uninstall_filter(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_uninstall_filter(&mut self, v: u64) {
        self.req = ::std::option::Option::Some(Request_oneof_req::uninstall_filter(v))
    }

    pub fn get_uninstall_filter(&self) -> u64 {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::uninstall_filter(v)) => v,
            _ => 0,
        }
    }

    // uint64 filter_changes = 16;

    pub fn clear_filter_changes(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_filter_changes(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::filter_changes(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_filter_changes(&mut self, v: u64) {
        self.req = ::std::option::Option::Some(Request_oneof_req::filter_changes(v))
    }

    pub fn get_filter_changes(&self) -> u64 {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::filter_changes(v)) => v,
            _ => 0,
        }
    }

    // uint64 filter_logs = 17;

    pub fn clear_filter_logs(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_filter_logs(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::filter_logs(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_filter_logs(&mut self, v: u64) {
        self.req = ::std::option::Option::Some(Request_oneof_req::filter_logs(v))
    }

    pub fn get_filter_logs(&self) -> u64 {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::filter_logs(v)) => v,
            _ => 0,
        }
    }

    // .UnverifiedTransaction un_tx = 18;

    pub fn clear_un_tx(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_un_tx(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::un_tx(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_un_tx(&mut self, v: super::blockchain::UnverifiedTransaction) {
        self.req = ::std::option::Option::Some(Request_oneof_req::un_tx(v))
    }

    // Mutable pointer to the field.
    pub fn mut_un_tx(&mut self) -> &mut super::blockchain::UnverifiedTransaction {
        if let ::std::option::Option::Some(Request_oneof_req::un_tx(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(Request_oneof_req::un_tx(super::blockchain::UnverifiedTransaction::new()));
        }
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::un_tx(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_un_tx(&mut self) -> super::blockchain::UnverifiedTransaction {
        if self.has_un_tx() {
            match self.req.take() {
                ::std::option::Option::Some(Request_oneof_req::un_tx(v)) => v,
                _ => panic!(),
            }
        } else {
            super::blockchain::UnverifiedTransaction::new()
        }
    }

    pub fn get_un_tx(&self) -> &super::blockchain::UnverifiedTransaction {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::un_tx(ref v)) => v,
            _ => super::blockchain::UnverifiedTransaction::default_instance(),
        }
    }

    // .BatchRequest batch_req = 19;

    pub fn clear_batch_req(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_batch_req(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::batch_req(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_batch_req(&mut self, v: BatchRequest) {
        self.req = ::std::option::Option::Some(Request_oneof_req::batch_req(v))
    }

    // Mutable pointer to the field.
    pub fn mut_batch_req(&mut self) -> &mut BatchRequest {
        if let ::std::option::Option::Some(Request_oneof_req::batch_req(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(Request_oneof_req::batch_req(BatchRequest::new()));
        }
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::batch_req(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_batch_req(&mut self) -> BatchRequest {
        if self.has_batch_req() {
            match self.req.take() {
                ::std::option::Option::Some(Request_oneof_req::batch_req(v)) => v,
                _ => panic!(),
            }
        } else {
            BatchRequest::new()
        }
    }

    pub fn get_batch_req(&self) -> &BatchRequest {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::batch_req(ref v)) => v,
            _ => BatchRequest::default_instance(),
        }
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        if let Some(Request_oneof_req::call(ref v)) = self.req {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_req::un_tx(ref v)) = self.req {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_req::batch_req(ref v)) = self.req {
            if !v.is_initialized() {
                return false;
            }
        }
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
                    }
                    self.req = ::std::option::Option::Some(Request_oneof_req::block_number(is.read_bool()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.req = ::std::option::Option::Some(Request_oneof_req::block_by_hash(is.read_string()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.req = ::std::option::Option::Some(Request_oneof_req::block_by_height(is.read_string()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.req = ::std::option::Option::Some(Request_oneof_req::transaction(is.read_bytes()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.req = ::std::option::Option::Some(Request_oneof_req::height(is.read_uint64()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.req = ::std::option::Option::Some(Request_oneof_req::peercount(is.read_bool()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.req = ::std::option::Option::Some(Request_oneof_req::call(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.req = ::std::option::Option::Some(Request_oneof_req::filter(is.read_string()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.req = ::std::option::Option::Some(Request_oneof_req::transaction_receipt(is.read_bytes()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.req = ::std::option::Option::Some(Request_oneof_req::transaction_count(is.read_string()?));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.req = ::std::option::Option::Some(Request_oneof_req::code(is.read_string()?));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.req = ::std::option::Option::Some(Request_oneof_req::new_filter(is.read_string()?));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.req = ::std::option::Option::Some(Request_oneof_req::new_block_filter(is.read_bool()?));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.req = ::std::option::Option::Some(Request_oneof_req::uninstall_filter(is.read_uint64()?));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.req = ::std::option::Option::Some(Request_oneof_req::filter_changes(is.read_uint64()?));
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.req = ::std::option::Option::Some(Request_oneof_req::filter_logs(is.read_uint64()?));
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.req = ::std::option::Option::Some(Request_oneof_req::un_tx(is.read_message()?));
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.req = ::std::option::Option::Some(Request_oneof_req::batch_req(is.read_message()?));
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
        }
        if let ::std::option::Option::Some(ref v) = self.req {
            match v {
                &Request_oneof_req::block_number(v) => {
                    my_size += 2;
                },
                &Request_oneof_req::block_by_hash(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
                &Request_oneof_req::block_by_height(ref v) => {
                    my_size += ::protobuf::rt::string_size(4, &v);
                },
                &Request_oneof_req::transaction(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(5, &v);
                },
                &Request_oneof_req::height(v) => {
                    my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Request_oneof_req::peercount(v) => {
                    my_size += 2;
                },
                &Request_oneof_req::call(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_req::filter(ref v) => {
                    my_size += ::protobuf::rt::string_size(9, &v);
                },
                &Request_oneof_req::transaction_receipt(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(10, &v);
                },
                &Request_oneof_req::transaction_count(ref v) => {
                    my_size += ::protobuf::rt::string_size(11, &v);
                },
                &Request_oneof_req::code(ref v) => {
                    my_size += ::protobuf::rt::string_size(12, &v);
                },
                &Request_oneof_req::new_filter(ref v) => {
                    my_size += ::protobuf::rt::string_size(13, &v);
                },
                &Request_oneof_req::new_block_filter(v) => {
                    my_size += 2;
                },
                &Request_oneof_req::uninstall_filter(v) => {
                    my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Request_oneof_req::filter_changes(v) => {
                    my_size += ::protobuf::rt::value_size(16, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Request_oneof_req::filter_logs(v) => {
                    my_size += ::protobuf::rt::value_size(17, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Request_oneof_req::un_tx(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_req::batch_req(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.request_id.is_empty() {
            os.write_bytes(1, &self.request_id)?;
        }
        if let ::std::option::Option::Some(ref v) = self.req {
            match v {
                &Request_oneof_req::block_number(v) => {
                    os.write_bool(2, v)?;
                },
                &Request_oneof_req::block_by_hash(ref v) => {
                    os.write_string(3, v)?;
                },
                &Request_oneof_req::block_by_height(ref v) => {
                    os.write_string(4, v)?;
                },
                &Request_oneof_req::transaction(ref v) => {
                    os.write_bytes(5, v)?;
                },
                &Request_oneof_req::height(v) => {
                    os.write_uint64(6, v)?;
                },
                &Request_oneof_req::peercount(v) => {
                    os.write_bool(7, v)?;
                },
                &Request_oneof_req::call(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_req::filter(ref v) => {
                    os.write_string(9, v)?;
                },
                &Request_oneof_req::transaction_receipt(ref v) => {
                    os.write_bytes(10, v)?;
                },
                &Request_oneof_req::transaction_count(ref v) => {
                    os.write_string(11, v)?;
                },
                &Request_oneof_req::code(ref v) => {
                    os.write_string(12, v)?;
                },
                &Request_oneof_req::new_filter(ref v) => {
                    os.write_string(13, v)?;
                },
                &Request_oneof_req::new_block_filter(v) => {
                    os.write_bool(14, v)?;
                },
                &Request_oneof_req::uninstall_filter(v) => {
                    os.write_uint64(15, v)?;
                },
                &Request_oneof_req::filter_changes(v) => {
                    os.write_uint64(16, v)?;
                },
                &Request_oneof_req::filter_logs(v) => {
                    os.write_uint64(17, v)?;
                },
                &Request_oneof_req::un_tx(ref v) => {
                    os.write_tag(18, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_req::batch_req(ref v) => {
                    os.write_tag(19, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

impl ::protobuf::MessageStatic for Request {
    fn new() -> Request {
        Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "request_id",
                    Request::get_request_id_for_reflect,
                    Request::mut_request_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "block_number",
                    Request::has_block_number,
                    Request::get_block_number,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "block_by_hash",
                    Request::has_block_by_hash,
                    Request::get_block_by_hash,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "block_by_height",
                    Request::has_block_by_height,
                    Request::get_block_by_height,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "transaction",
                    Request::has_transaction,
                    Request::get_transaction,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "height",
                    Request::has_height,
                    Request::get_height,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "peercount",
                    Request::has_peercount,
                    Request::get_peercount,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Call>(
                    "call",
                    Request::has_call,
                    Request::get_call,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "filter",
                    Request::has_filter,
                    Request::get_filter,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "transaction_receipt",
                    Request::has_transaction_receipt,
                    Request::get_transaction_receipt,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "transaction_count",
                    Request::has_transaction_count,
                    Request::get_transaction_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "code",
                    Request::has_code,
                    Request::get_code,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "new_filter",
                    Request::has_new_filter,
                    Request::get_new_filter,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "new_block_filter",
                    Request::has_new_block_filter,
                    Request::get_new_block_filter,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "uninstall_filter",
                    Request::has_uninstall_filter,
                    Request::get_uninstall_filter,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "filter_changes",
                    Request::has_filter_changes,
                    Request::get_filter_changes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "filter_logs",
                    Request::has_filter_logs,
                    Request::get_filter_logs,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::blockchain::UnverifiedTransaction>(
                    "un_tx",
                    Request::has_un_tx,
                    Request::get_un_tx,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, BatchRequest>(
                    "batch_req",
                    Request::has_batch_req,
                    Request::get_batch_req,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Request>(
                    "Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.clear_request_id();
        self.clear_block_number();
        self.clear_block_by_hash();
        self.clear_block_by_height();
        self.clear_transaction();
        self.clear_height();
        self.clear_peercount();
        self.clear_call();
        self.clear_filter();
        self.clear_transaction_receipt();
        self.clear_transaction_count();
        self.clear_code();
        self.clear_new_filter();
        self.clear_new_block_filter();
        self.clear_uninstall_filter();
        self.clear_filter_changes();
        self.clear_filter_logs();
        self.clear_un_tx();
        self.clear_batch_req();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BatchRequest {
    // message fields
    pub new_tx_requests: ::protobuf::RepeatedField<Request>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BatchRequest {}

impl BatchRequest {
    pub fn new() -> BatchRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BatchRequest {
        static mut instance: ::protobuf::lazy::Lazy<BatchRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BatchRequest,
        };
        unsafe {
            instance.get(BatchRequest::new)
        }
    }

    // repeated .Request new_tx_requests = 1;

    pub fn clear_new_tx_requests(&mut self) {
        self.new_tx_requests.clear();
    }

    // Param is passed by value, moved
    pub fn set_new_tx_requests(&mut self, v: ::protobuf::RepeatedField<Request>) {
        self.new_tx_requests = v;
    }

    // Mutable pointer to the field.
    pub fn mut_new_tx_requests(&mut self) -> &mut ::protobuf::RepeatedField<Request> {
        &mut self.new_tx_requests
    }

    // Take field
    pub fn take_new_tx_requests(&mut self) -> ::protobuf::RepeatedField<Request> {
        ::std::mem::replace(&mut self.new_tx_requests, ::protobuf::RepeatedField::new())
    }

    pub fn get_new_tx_requests(&self) -> &[Request] {
        &self.new_tx_requests
    }

    fn get_new_tx_requests_for_reflect(&self) -> &::protobuf::RepeatedField<Request> {
        &self.new_tx_requests
    }

    fn mut_new_tx_requests_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Request> {
        &mut self.new_tx_requests
    }
}

impl ::protobuf::Message for BatchRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.new_tx_requests {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.new_tx_requests)?;
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
        for value in &self.new_tx_requests {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.new_tx_requests {
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

impl ::protobuf::MessageStatic for BatchRequest {
    fn new() -> BatchRequest {
        BatchRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<BatchRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Request>>(
                    "new_tx_requests",
                    BatchRequest::get_new_tx_requests_for_reflect,
                    BatchRequest::mut_new_tx_requests_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BatchRequest>(
                    "BatchRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BatchRequest {
    fn clear(&mut self) {
        self.clear_new_tx_requests();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BatchRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BatchRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum BlockTag {
    Latest = 0,
    Earliest = 1,
}

impl ::protobuf::ProtobufEnum for BlockTag {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<BlockTag> {
        match value {
            0 => ::std::option::Option::Some(BlockTag::Latest),
            1 => ::std::option::Option::Some(BlockTag::Earliest),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [BlockTag] = &[
            BlockTag::Latest,
            BlockTag::Earliest,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<BlockTag>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("BlockTag", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for BlockTag {
}

impl ::std::default::Default for BlockTag {
    fn default() -> Self {
        BlockTag::Latest
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockTag {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rrequest.proto\x1a\x10blockchain.proto\"V\n\x04Call\x12\x12\n\x04from\
    \x18\x01\x20\x01(\x0cR\x04from\x12\x0e\n\x02to\x18\x02\x20\x01(\x0cR\x02\
    to\x12\x12\n\x04data\x18\x03\x20\x01(\x0cR\x04data\x12\x16\n\x06height\
    \x18\x04\x20\x01(\tR\x06height\"\xd4\x05\n\x07Request\x12\x1d\n\nrequest\
    _id\x18\x01\x20\x01(\x0cR\trequestId\x12#\n\x0cblock_number\x18\x02\x20\
    \x01(\x08H\0R\x0bblockNumber\x12$\n\rblock_by_hash\x18\x03\x20\x01(\tH\0\
    R\x0bblockByHash\x12(\n\x0fblock_by_height\x18\x04\x20\x01(\tH\0R\rblock\
    ByHeight\x12\"\n\x0btransaction\x18\x05\x20\x01(\x0cH\0R\x0btransaction\
    \x12\x18\n\x06height\x18\x06\x20\x01(\x04H\0R\x06height\x12\x1e\n\tpeerc\
    ount\x18\x07\x20\x01(\x08H\0R\tpeercount\x12\x1b\n\x04call\x18\x08\x20\
    \x01(\x0b2\x05.CallH\0R\x04call\x12\x18\n\x06filter\x18\t\x20\x01(\tH\0R\
    \x06filter\x121\n\x13transaction_receipt\x18\n\x20\x01(\x0cH\0R\x12trans\
    actionReceipt\x12-\n\x11transaction_count\x18\x0b\x20\x01(\tH\0R\x10tran\
    sactionCount\x12\x14\n\x04code\x18\x0c\x20\x01(\tH\0R\x04code\x12\x1f\n\
    \nnew_filter\x18\r\x20\x01(\tH\0R\tnewFilter\x12*\n\x10new_block_filter\
    \x18\x0e\x20\x01(\x08H\0R\x0enewBlockFilter\x12+\n\x10uninstall_filter\
    \x18\x0f\x20\x01(\x04H\0R\x0funinstallFilter\x12'\n\x0efilter_changes\
    \x18\x10\x20\x01(\x04H\0R\rfilterChanges\x12!\n\x0bfilter_logs\x18\x11\
    \x20\x01(\x04H\0R\nfilterLogs\x12-\n\x05un_tx\x18\x12\x20\x01(\x0b2\x16.\
    UnverifiedTransactionH\0R\x04unTx\x12,\n\tbatch_req\x18\x13\x20\x01(\x0b\
    2\r.BatchRequestH\0R\x08batchReqB\x05\n\x03req\"@\n\x0cBatchRequest\x120\
    \n\x0fnew_tx_requests\x18\x01\x20\x03(\x0b2\x08.RequestR\rnewTxRequests*\
    $\n\x08BlockTag\x12\n\n\x06Latest\x10\0\x12\x0c\n\x08Earliest\x10\x01J\
    \x9f\r\n\x06\x12\x04\0\0*\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\
    \x03\0\x12\x03\x02\x07\x19\n\n\n\x02\x05\0\x12\x04\x04\0\x07\x01\n\n\n\
    \x03\x05\0\x01\x12\x03\x04\x05\r\n\x0b\n\x04\x05\0\x02\0\x12\x03\x05\x04\
    \x0f\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x05\x04\n\n\x0c\n\x05\x05\0\x02\
    \0\x02\x12\x03\x05\r\x0e\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x06\x04\x11\n\
    \x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x06\x04\x0c\n\x0c\n\x05\x05\0\x02\
    \x01\x02\x12\x03\x06\x0f\x10\n\n\n\x02\x04\0\x12\x04\t\0\x0e\x01\n\n\n\
    \x03\x04\0\x01\x12\x03\t\x08\x0c\n\x0b\n\x04\x04\0\x02\0\x12\x03\n\x04\
    \x13\n\r\n\x05\x04\0\x02\0\x04\x12\x04\n\x04\t\x0e\n\x0c\n\x05\x04\0\x02\
    \0\x05\x12\x03\n\x04\t\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\n\n\x0e\n\x0c\
    \n\x05\x04\0\x02\0\x03\x12\x03\n\x11\x12\n\x0b\n\x04\x04\0\x02\x01\x12\
    \x03\x0b\x04\x11\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x0b\x04\n\x13\n\x0c\
    \n\x05\x04\0\x02\x01\x05\x12\x03\x0b\x04\t\n\x0c\n\x05\x04\0\x02\x01\x01\
    \x12\x03\x0b\n\x0c\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0b\x0f\x10\n\
    \x0b\n\x04\x04\0\x02\x02\x12\x03\x0c\x04\x13\n\r\n\x05\x04\0\x02\x02\x04\
    \x12\x04\x0c\x04\x0b\x11\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x0c\x04\t\
    \n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x0c\n\x0e\n\x0c\n\x05\x04\0\x02\
    \x02\x03\x12\x03\x0c\x11\x12\n\x0b\n\x04\x04\0\x02\x03\x12\x03\r\x04\x16\
    \n\r\n\x05\x04\0\x02\x03\x04\x12\x04\r\x04\x0c\x13\n\x0c\n\x05\x04\0\x02\
    \x03\x05\x12\x03\r\x04\n\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\r\x0b\x11\
    \n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\r\x14\x15\n\n\n\x02\x04\x01\x12\
    \x04\x10\0&\x01\n\n\n\x03\x04\x01\x01\x12\x03\x10\x08\x0f\n\x0b\n\x04\
    \x04\x01\x02\0\x12\x03\x11\x04\x19\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\
    \x11\x04\x10\x11\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x11\x04\t\n\x0c\n\
    \x05\x04\x01\x02\0\x01\x12\x03\x11\n\x14\n\x0c\n\x05\x04\x01\x02\0\x03\
    \x12\x03\x11\x17\x18\n\x0c\n\x04\x04\x01\x08\0\x12\x04\x12\x04%\x05\n\
    \x0c\n\x05\x04\x01\x08\0\x01\x12\x03\x12\n\r\n\x0b\n\x04\x04\x01\x02\x01\
    \x12\x03\x13\x08\x1e\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x13\x08\x0c\
    \n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x13\r\x19\n\x0c\n\x05\x04\x01\
    \x02\x01\x03\x12\x03\x13\x1c\x1d\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x14\
    \x08!\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\x14\x08\x0e\n\x0c\n\x05\
    \x04\x01\x02\x02\x01\x12\x03\x14\x0f\x1c\n\x0c\n\x05\x04\x01\x02\x02\x03\
    \x12\x03\x14\x1f\x20\n\x0b\n\x04\x04\x01\x02\x03\x12\x03\x15\x08#\n\x0c\
    \n\x05\x04\x01\x02\x03\x05\x12\x03\x15\x08\x0e\n\x0c\n\x05\x04\x01\x02\
    \x03\x01\x12\x03\x15\x0f\x1e\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03\x15\
    !\"\n\x0b\n\x04\x04\x01\x02\x04\x12\x03\x16\x08\x1e\n\x0c\n\x05\x04\x01\
    \x02\x04\x05\x12\x03\x16\x08\r\n\x0c\n\x05\x04\x01\x02\x04\x01\x12\x03\
    \x16\x0e\x19\n\x0c\n\x05\x04\x01\x02\x04\x03\x12\x03\x16\x1c\x1d\n\x0b\n\
    \x04\x04\x01\x02\x05\x12\x03\x17\x08\x1a\n\x0c\n\x05\x04\x01\x02\x05\x05\
    \x12\x03\x17\x08\x0e\n\x0c\n\x05\x04\x01\x02\x05\x01\x12\x03\x17\x0f\x15\
    \n\x0c\n\x05\x04\x01\x02\x05\x03\x12\x03\x17\x18\x19\n\x0b\n\x04\x04\x01\
    \x02\x06\x12\x03\x18\x08\x1b\n\x0c\n\x05\x04\x01\x02\x06\x05\x12\x03\x18\
    \x08\x0c\n\x0c\n\x05\x04\x01\x02\x06\x01\x12\x03\x18\r\x16\n\x0c\n\x05\
    \x04\x01\x02\x06\x03\x12\x03\x18\x19\x1a\n\x0b\n\x04\x04\x01\x02\x07\x12\
    \x03\x19\x08\x16\n\x0c\n\x05\x04\x01\x02\x07\x06\x12\x03\x19\x08\x0c\n\
    \x0c\n\x05\x04\x01\x02\x07\x01\x12\x03\x19\r\x11\n\x0c\n\x05\x04\x01\x02\
    \x07\x03\x12\x03\x19\x14\x15\n\x0b\n\x04\x04\x01\x02\x08\x12\x03\x1a\x08\
    \x1a\n\x0c\n\x05\x04\x01\x02\x08\x05\x12\x03\x1a\x08\x0e\n\x0c\n\x05\x04\
    \x01\x02\x08\x01\x12\x03\x1a\x0f\x15\n\x0c\n\x05\x04\x01\x02\x08\x03\x12\
    \x03\x1a\x18\x19\n\x0b\n\x04\x04\x01\x02\t\x12\x03\x1b\x08'\n\x0c\n\x05\
    \x04\x01\x02\t\x05\x12\x03\x1b\x08\r\n\x0c\n\x05\x04\x01\x02\t\x01\x12\
    \x03\x1b\x0e!\n\x0c\n\x05\x04\x01\x02\t\x03\x12\x03\x1b$&\n\x0b\n\x04\
    \x04\x01\x02\n\x12\x03\x1c\x08&\n\x0c\n\x05\x04\x01\x02\n\x05\x12\x03\
    \x1c\x08\x0e\n\x0c\n\x05\x04\x01\x02\n\x01\x12\x03\x1c\x0f\x20\n\x0c\n\
    \x05\x04\x01\x02\n\x03\x12\x03\x1c#%\n\x0b\n\x04\x04\x01\x02\x0b\x12\x03\
    \x1d\x08\x19\n\x0c\n\x05\x04\x01\x02\x0b\x05\x12\x03\x1d\x08\x0e\n\x0c\n\
    \x05\x04\x01\x02\x0b\x01\x12\x03\x1d\x0f\x13\n\x0c\n\x05\x04\x01\x02\x0b\
    \x03\x12\x03\x1d\x16\x18\n\x0b\n\x04\x04\x01\x02\x0c\x12\x03\x1e\x08\x1f\
    \n\x0c\n\x05\x04\x01\x02\x0c\x05\x12\x03\x1e\x08\x0e\n\x0c\n\x05\x04\x01\
    \x02\x0c\x01\x12\x03\x1e\x0f\x19\n\x0c\n\x05\x04\x01\x02\x0c\x03\x12\x03\
    \x1e\x1c\x1e\n\x0b\n\x04\x04\x01\x02\r\x12\x03\x1f\x08#\n\x0c\n\x05\x04\
    \x01\x02\r\x05\x12\x03\x1f\x08\x0c\n\x0c\n\x05\x04\x01\x02\r\x01\x12\x03\
    \x1f\r\x1d\n\x0c\n\x05\x04\x01\x02\r\x03\x12\x03\x1f\x20\"\n\x0b\n\x04\
    \x04\x01\x02\x0e\x12\x03\x20\x08%\n\x0c\n\x05\x04\x01\x02\x0e\x05\x12\
    \x03\x20\x08\x0e\n\x0c\n\x05\x04\x01\x02\x0e\x01\x12\x03\x20\x0f\x1f\n\
    \x0c\n\x05\x04\x01\x02\x0e\x03\x12\x03\x20\"$\n\x0b\n\x04\x04\x01\x02\
    \x0f\x12\x03!\x08#\n\x0c\n\x05\x04\x01\x02\x0f\x05\x12\x03!\x08\x0e\n\
    \x0c\n\x05\x04\x01\x02\x0f\x01\x12\x03!\x0f\x1d\n\x0c\n\x05\x04\x01\x02\
    \x0f\x03\x12\x03!\x20\"\n\x0b\n\x04\x04\x01\x02\x10\x12\x03\"\x08\x20\n\
    \x0c\n\x05\x04\x01\x02\x10\x05\x12\x03\"\x08\x0e\n\x0c\n\x05\x04\x01\x02\
    \x10\x01\x12\x03\"\x0f\x1a\n\x0c\n\x05\x04\x01\x02\x10\x03\x12\x03\"\x1d\
    \x1f\n>\n\x04\x04\x01\x02\x11\x12\x03#\x08)\"1\xe4\xba\xa4\xe6\x98\x93\
    \xe7\xbb\x9f\xe4\xb8\x80\xe5\x88\xb0\xe8\xbf\x99\xe9\x87\x8c\xe4\xba\x86\
    \xe3\x80\x82\xe5\x88\x92\xe5\x88\x86\xe5\x9c\xa8\xe8\xaf\xb7\xe6\xb1\x82\
    \xe9\x87\x8c\xe9\x9d\xa2\n\n\x0c\n\x05\x04\x01\x02\x11\x06\x12\x03#\x08\
    \x1d\n\x0c\n\x05\x04\x01\x02\x11\x01\x12\x03#\x1e#\n\x0c\n\x05\x04\x01\
    \x02\x11\x03\x12\x03#&(\n\x0b\n\x04\x04\x01\x02\x12\x12\x03$\x08$\n\x0c\
    \n\x05\x04\x01\x02\x12\x06\x12\x03$\x08\x14\n\x0c\n\x05\x04\x01\x02\x12\
    \x01\x12\x03$\x15\x1e\n\x0c\n\x05\x04\x01\x02\x12\x03\x12\x03$!#\n\n\n\
    \x02\x04\x02\x12\x04(\0*\x01\n\n\n\x03\x04\x02\x01\x12\x03(\x08\x14\n\
    \x0b\n\x04\x04\x02\x02\0\x12\x03)\x04)\n\x0c\n\x05\x04\x02\x02\0\x04\x12\
    \x03)\x04\x0c\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03)\r\x14\n\x0c\n\x05\
    \x04\x02\x02\0\x01\x12\x03)\x15$\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03)'\
    (b\x06proto3\
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
