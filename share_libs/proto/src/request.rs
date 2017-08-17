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
        };
        if !self.to.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.to);
        };
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.data);
        };
        if !self.height.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.height);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.from.is_empty() {
            os.write_bytes(1, &self.from)?;
        };
        if !self.to.is_empty() {
            os.write_bytes(2, &self.to)?;
        };
        if !self.data.is_empty() {
            os.write_bytes(3, &self.data)?;
        };
        if !self.height.is_empty() {
            os.write_string(4, &self.height)?;
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
}

impl ::protobuf::Message for Request {
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
                    self.req = ::std::option::Option::Some(Request_oneof_req::block_number(is.read_bool()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req = ::std::option::Option::Some(Request_oneof_req::block_by_hash(is.read_string()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req = ::std::option::Option::Some(Request_oneof_req::block_by_height(is.read_string()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req = ::std::option::Option::Some(Request_oneof_req::transaction(is.read_bytes()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req = ::std::option::Option::Some(Request_oneof_req::height(is.read_uint64()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req = ::std::option::Option::Some(Request_oneof_req::peercount(is.read_bool()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req = ::std::option::Option::Some(Request_oneof_req::call(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req = ::std::option::Option::Some(Request_oneof_req::filter(is.read_string()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req = ::std::option::Option::Some(Request_oneof_req::transaction_receipt(is.read_bytes()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req = ::std::option::Option::Some(Request_oneof_req::transaction_count(is.read_string()?));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req = ::std::option::Option::Some(Request_oneof_req::code(is.read_string()?));
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
pub struct FullTransaction {
    // message fields
    transaction: ::protobuf::SingularPtrField<super::blockchain::SignedTransaction>,
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
    // message oneof groups
    pub result: ::std::option::Option<Response_oneof_result>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Response {}

#[derive(Clone,PartialEq)]
pub enum Response_oneof_result {
    block_number(u64),
    block(::std::string::String),
    ts(FullTransaction),
    none(bool),
    peercount(u32),
    call_result(::std::vec::Vec<u8>),
    logs(::std::string::String),
    receipt(::std::string::String),
    transaction_count(u64),
    code(::std::vec::Vec<u8>),
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

    // uint64 block_number = 2;

    pub fn clear_block_number(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_block_number(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::block_number(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_block_number(&mut self, v: u64) {
        self.result = ::std::option::Option::Some(Response_oneof_result::block_number(v))
    }

    pub fn get_block_number(&self) -> u64 {
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::block_number(v)) => v,
            _ => 0,
        }
    }

    // string block = 3;

    pub fn clear_block(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_block(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::block(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_block(&mut self, v: ::std::string::String) {
        self.result = ::std::option::Option::Some(Response_oneof_result::block(v))
    }

    // Mutable pointer to the field.
    pub fn mut_block(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Response_oneof_result::block(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(Response_oneof_result::block(::std::string::String::new()));
        }
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::block(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_block(&mut self) -> ::std::string::String {
        if self.has_block() {
            match self.result.take() {
                ::std::option::Option::Some(Response_oneof_result::block(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_block(&self) -> &str {
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::block(ref v)) => v,
            _ => "",
        }
    }

    // .FullTransaction ts = 4;

    pub fn clear_ts(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_ts(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::ts(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ts(&mut self, v: FullTransaction) {
        self.result = ::std::option::Option::Some(Response_oneof_result::ts(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ts(&mut self) -> &mut FullTransaction {
        if let ::std::option::Option::Some(Response_oneof_result::ts(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(Response_oneof_result::ts(FullTransaction::new()));
        }
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::ts(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ts(&mut self) -> FullTransaction {
        if self.has_ts() {
            match self.result.take() {
                ::std::option::Option::Some(Response_oneof_result::ts(v)) => v,
                _ => panic!(),
            }
        } else {
            FullTransaction::new()
        }
    }

    pub fn get_ts(&self) -> &FullTransaction {
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::ts(ref v)) => v,
            _ => FullTransaction::default_instance(),
        }
    }

    // bool none = 5;

    pub fn clear_none(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_none(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::none(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_none(&mut self, v: bool) {
        self.result = ::std::option::Option::Some(Response_oneof_result::none(v))
    }

    pub fn get_none(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::none(v)) => v,
            _ => false,
        }
    }

    // uint32 peercount = 6;

    pub fn clear_peercount(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_peercount(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::peercount(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_peercount(&mut self, v: u32) {
        self.result = ::std::option::Option::Some(Response_oneof_result::peercount(v))
    }

    pub fn get_peercount(&self) -> u32 {
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::peercount(v)) => v,
            _ => 0,
        }
    }

    // bytes call_result = 7;

    pub fn clear_call_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_call_result(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::call_result(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_call_result(&mut self, v: ::std::vec::Vec<u8>) {
        self.result = ::std::option::Option::Some(Response_oneof_result::call_result(v))
    }

    // Mutable pointer to the field.
    pub fn mut_call_result(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(Response_oneof_result::call_result(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(Response_oneof_result::call_result(::std::vec::Vec::new()));
        }
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::call_result(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_call_result(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_call_result() {
            match self.result.take() {
                ::std::option::Option::Some(Response_oneof_result::call_result(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_call_result(&self) -> &[u8] {
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::call_result(ref v)) => v,
            _ => &[],
        }
    }

    // string logs = 8;

    pub fn clear_logs(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_logs(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::logs(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_logs(&mut self, v: ::std::string::String) {
        self.result = ::std::option::Option::Some(Response_oneof_result::logs(v))
    }

    // Mutable pointer to the field.
    pub fn mut_logs(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Response_oneof_result::logs(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(Response_oneof_result::logs(::std::string::String::new()));
        }
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::logs(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_logs(&mut self) -> ::std::string::String {
        if self.has_logs() {
            match self.result.take() {
                ::std::option::Option::Some(Response_oneof_result::logs(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_logs(&self) -> &str {
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::logs(ref v)) => v,
            _ => "",
        }
    }

    // string receipt = 9;

    pub fn clear_receipt(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_receipt(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::receipt(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_receipt(&mut self, v: ::std::string::String) {
        self.result = ::std::option::Option::Some(Response_oneof_result::receipt(v))
    }

    // Mutable pointer to the field.
    pub fn mut_receipt(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Response_oneof_result::receipt(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(Response_oneof_result::receipt(::std::string::String::new()));
        }
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::receipt(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_receipt(&mut self) -> ::std::string::String {
        if self.has_receipt() {
            match self.result.take() {
                ::std::option::Option::Some(Response_oneof_result::receipt(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_receipt(&self) -> &str {
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::receipt(ref v)) => v,
            _ => "",
        }
    }

    // uint64 transaction_count = 10;

    pub fn clear_transaction_count(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_transaction_count(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::transaction_count(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_transaction_count(&mut self, v: u64) {
        self.result = ::std::option::Option::Some(Response_oneof_result::transaction_count(v))
    }

    pub fn get_transaction_count(&self) -> u64 {
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::transaction_count(v)) => v,
            _ => 0,
        }
    }

    // bytes code = 11;

    pub fn clear_code(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_code(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::code(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: ::std::vec::Vec<u8>) {
        self.result = ::std::option::Option::Some(Response_oneof_result::code(v))
    }

    // Mutable pointer to the field.
    pub fn mut_code(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(Response_oneof_result::code(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(Response_oneof_result::code(::std::vec::Vec::new()));
        }
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::code(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_code(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_code() {
            match self.result.take() {
                ::std::option::Option::Some(Response_oneof_result::code(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_code(&self) -> &[u8] {
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::code(ref v)) => v,
            _ => &[],
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
                    self.result = ::std::option::Option::Some(Response_oneof_result::block_number(is.read_uint64()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(Response_oneof_result::block(is.read_string()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(Response_oneof_result::ts(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(Response_oneof_result::none(is.read_bool()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(Response_oneof_result::peercount(is.read_uint32()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(Response_oneof_result::call_result(is.read_bytes()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(Response_oneof_result::logs(is.read_string()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(Response_oneof_result::receipt(is.read_string()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(Response_oneof_result::transaction_count(is.read_uint64()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(Response_oneof_result::code(is.read_bytes()?));
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
        if let ::std::option::Option::Some(ref v) = self.result {
            match v {
                &Response_oneof_result::block_number(v) => {
                    my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Response_oneof_result::block(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
                &Response_oneof_result::ts(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_result::none(v) => {
                    my_size += 2;
                },
                &Response_oneof_result::peercount(v) => {
                    my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Response_oneof_result::call_result(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(7, &v);
                },
                &Response_oneof_result::logs(ref v) => {
                    my_size += ::protobuf::rt::string_size(8, &v);
                },
                &Response_oneof_result::receipt(ref v) => {
                    my_size += ::protobuf::rt::string_size(9, &v);
                },
                &Response_oneof_result::transaction_count(v) => {
                    my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Response_oneof_result::code(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(11, &v);
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
        if let ::std::option::Option::Some(ref v) = self.result {
            match v {
                &Response_oneof_result::block_number(v) => {
                    os.write_uint64(2, v)?;
                },
                &Response_oneof_result::block(ref v) => {
                    os.write_string(3, v)?;
                },
                &Response_oneof_result::ts(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_result::none(v) => {
                    os.write_bool(5, v)?;
                },
                &Response_oneof_result::peercount(v) => {
                    os.write_uint32(6, v)?;
                },
                &Response_oneof_result::call_result(ref v) => {
                    os.write_bytes(7, v)?;
                },
                &Response_oneof_result::logs(ref v) => {
                    os.write_string(8, v)?;
                },
                &Response_oneof_result::receipt(ref v) => {
                    os.write_string(9, v)?;
                },
                &Response_oneof_result::transaction_count(v) => {
                    os.write_uint64(10, v)?;
                },
                &Response_oneof_result::code(ref v) => {
                    os.write_bytes(11, v)?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "none",
                    Response::has_none,
                    Response::get_none,
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
                    "code",
                    Response::has_code,
                    Response::get_code,
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
        self.clear_block_number();
        self.clear_block();
        self.clear_ts();
        self.clear_none();
        self.clear_peercount();
        self.clear_call_result();
        self.clear_logs();
        self.clear_receipt();
        self.clear_transaction_count();
        self.clear_code();
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

    fn enum_descriptor_static(_: Option<BlockTag>) -> &'static ::protobuf::reflect::EnumDescriptor {
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

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x10, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x22, 0x56, 0x0a, 0x04, 0x43, 0x61, 0x6c, 0x6c, 0x12, 0x12, 0x0a, 0x04, 0x66, 0x72, 0x6f,
    0x6d, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x66, 0x72, 0x6f, 0x6d, 0x12, 0x0e, 0x0a,
    0x02, 0x74, 0x6f, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x02, 0x74, 0x6f, 0x12, 0x12, 0x0a,
    0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74,
    0x61, 0x12, 0x16, 0x0a, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x22, 0xb1, 0x03, 0x0a, 0x07, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x72, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x49, 0x64, 0x12, 0x23, 0x0a, 0x0c, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x6e, 0x75,
    0x6d, 0x62, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x0b, 0x62, 0x6c,
    0x6f, 0x63, 0x6b, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x24, 0x0a, 0x0d, 0x62, 0x6c, 0x6f,
    0x63, 0x6b, 0x5f, 0x62, 0x79, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09,
    0x48, 0x00, 0x52, 0x0b, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x42, 0x79, 0x48, 0x61, 0x73, 0x68, 0x12,
    0x28, 0x0a, 0x0f, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x62, 0x79, 0x5f, 0x68, 0x65, 0x69, 0x67,
    0x68, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0d, 0x62, 0x6c, 0x6f, 0x63,
    0x6b, 0x42, 0x79, 0x48, 0x65, 0x69, 0x67, 0x68, 0x74, 0x12, 0x22, 0x0a, 0x0b, 0x74, 0x72, 0x61,
    0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x48, 0x00,
    0x52, 0x0b, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x18, 0x0a,
    0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04, 0x48, 0x00, 0x52,
    0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x12, 0x1e, 0x0a, 0x09, 0x70, 0x65, 0x65, 0x72, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x09, 0x70, 0x65,
    0x65, 0x72, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x1b, 0x0a, 0x04, 0x63, 0x61, 0x6c, 0x6c, 0x18,
    0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x43, 0x61, 0x6c, 0x6c, 0x48, 0x00, 0x52, 0x04,
    0x63, 0x61, 0x6c, 0x6c, 0x12, 0x18, 0x0a, 0x06, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x18, 0x09,
    0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x06, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x12, 0x31,
    0x0a, 0x13, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x72, 0x65,
    0x63, 0x65, 0x69, 0x70, 0x74, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0c, 0x48, 0x00, 0x52, 0x12, 0x74,
    0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x63, 0x65, 0x69, 0x70,
    0x74, 0x12, 0x2d, 0x0a, 0x11, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x10,
    0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x75, 0x6e, 0x74,
    0x12, 0x14, 0x0a, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00,
    0x52, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x42, 0x05, 0x0a, 0x03, 0x72, 0x65, 0x71, 0x22, 0x9f, 0x01,
    0x0a, 0x0f, 0x46, 0x75, 0x6c, 0x6c, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x34, 0x0a, 0x0b, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x53, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x54,
    0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0b, 0x74, 0x72, 0x61, 0x6e,
    0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x21, 0x0a, 0x0c, 0x62, 0x6c, 0x6f, 0x63, 0x6b,
    0x5f, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x62,
    0x6c, 0x6f, 0x63, 0x6b, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x1d, 0x0a, 0x0a, 0x62, 0x6c,
    0x6f, 0x63, 0x6b, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09,
    0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x48, 0x61, 0x73, 0x68, 0x12, 0x14, 0x0a, 0x05, 0x69, 0x6e, 0x64,
    0x65, 0x78, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x22,
    0xe4, 0x02, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1d, 0x0a, 0x0a,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x09, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x49, 0x64, 0x12, 0x23, 0x0a, 0x0c, 0x62,
    0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x04, 0x48, 0x00, 0x52, 0x0b, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72,
    0x12, 0x16, 0x0a, 0x05, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x48,
    0x00, 0x52, 0x05, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x22, 0x0a, 0x02, 0x74, 0x73, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x46, 0x75, 0x6c, 0x6c, 0x54, 0x72, 0x61, 0x6e, 0x73,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52, 0x02, 0x74, 0x73, 0x12, 0x14, 0x0a, 0x04,
    0x6e, 0x6f, 0x6e, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x04, 0x6e, 0x6f,
    0x6e, 0x65, 0x12, 0x1e, 0x0a, 0x09, 0x70, 0x65, 0x65, 0x72, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18,
    0x06, 0x20, 0x01, 0x28, 0x0d, 0x48, 0x00, 0x52, 0x09, 0x70, 0x65, 0x65, 0x72, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x12, 0x21, 0x0a, 0x0b, 0x63, 0x61, 0x6c, 0x6c, 0x5f, 0x72, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0c, 0x48, 0x00, 0x52, 0x0a, 0x63, 0x61, 0x6c, 0x6c, 0x52,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x14, 0x0a, 0x04, 0x6c, 0x6f, 0x67, 0x73, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x04, 0x6c, 0x6f, 0x67, 0x73, 0x12, 0x1a, 0x0a, 0x07, 0x72,
    0x65, 0x63, 0x65, 0x69, 0x70, 0x74, 0x18, 0x09, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x07,
    0x72, 0x65, 0x63, 0x65, 0x69, 0x70, 0x74, 0x12, 0x2d, 0x0a, 0x11, 0x74, 0x72, 0x61, 0x6e, 0x73,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x0a, 0x20, 0x01,
    0x28, 0x04, 0x48, 0x00, 0x52, 0x10, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x14, 0x0a, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x0b,
    0x20, 0x01, 0x28, 0x0c, 0x48, 0x00, 0x52, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x42, 0x08, 0x0a, 0x06,
    0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x2a, 0x24, 0x0a, 0x08, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x54,
    0x61, 0x67, 0x12, 0x0a, 0x0a, 0x06, 0x4c, 0x61, 0x74, 0x65, 0x73, 0x74, 0x10, 0x00, 0x12, 0x0c,
    0x0a, 0x08, 0x45, 0x61, 0x72, 0x6c, 0x69, 0x65, 0x73, 0x74, 0x10, 0x01, 0x4a, 0xde, 0x10, 0x0a,
    0x06, 0x12, 0x04, 0x00, 0x00, 0x37, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00,
    0x12, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x02, 0x07, 0x19, 0x0a, 0x0a, 0x0a, 0x02,
    0x05, 0x00, 0x12, 0x04, 0x04, 0x00, 0x07, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12,
    0x03, 0x04, 0x05, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x04,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x05, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x06, 0x0f, 0x10, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x09, 0x00,
    0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x0c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x04, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0a, 0x04, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x0a, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x0a, 0x0a, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x0a, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x04,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x0b, 0x04, 0x0a, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0b, 0x04, 0x09, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x0a, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0b, 0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x0c, 0x04, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04,
    0x12, 0x04, 0x0c, 0x04, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x0c, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0c,
    0x0a, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x11, 0x12,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x04, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x0d, 0x04, 0x0c, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0d, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x0d, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x10, 0x00,
    0x20, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x10, 0x08, 0x0f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x11, 0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x11, 0x04, 0x10, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x11, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x11, 0x0a, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x11, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x08, 0x00, 0x12, 0x04, 0x12, 0x04,
    0x1f, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x08, 0x00, 0x01, 0x12, 0x03, 0x12, 0x0a, 0x0d,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x13, 0x08, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x13, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x13, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x13, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12,
    0x03, 0x14, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x14,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x14, 0x0f, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x14, 0x1f, 0x20, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x15, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x15, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x15, 0x0f, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x15, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x16,
    0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x16, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x16, 0x0e, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x16, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x17, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x05, 0x05, 0x12, 0x03, 0x17, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01,
    0x12, 0x03, 0x17, 0x0f, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03,
    0x17, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x18, 0x08, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x05, 0x12, 0x03, 0x18, 0x08, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x18, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x06, 0x03, 0x12, 0x03, 0x18, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x07, 0x12, 0x03, 0x19, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x06,
    0x12, 0x03, 0x19, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x01, 0x12, 0x03,
    0x19, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x03, 0x12, 0x03, 0x19, 0x14,
    0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x08, 0x12, 0x03, 0x1a, 0x08, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x05, 0x12, 0x03, 0x1a, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x08, 0x01, 0x12, 0x03, 0x1a, 0x0f, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x08, 0x03, 0x12, 0x03, 0x1a, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x09,
    0x12, 0x03, 0x1b, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x05, 0x12, 0x03,
    0x1b, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x01, 0x12, 0x03, 0x1b, 0x0e,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x03, 0x12, 0x03, 0x1b, 0x24, 0x26, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0a, 0x12, 0x03, 0x1c, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x1c, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x0a, 0x01, 0x12, 0x03, 0x1c, 0x0f, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a,
    0x03, 0x12, 0x03, 0x1c, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0b, 0x12, 0x03,
    0x1d, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x1d, 0x08,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x1d, 0x0f, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x1d, 0x16, 0x18, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x02, 0x12, 0x04, 0x22, 0x00, 0x27, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x22, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x23,
    0x04, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x23, 0x04, 0x22,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x23, 0x04, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x23, 0x16, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x23, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x24, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x24, 0x04, 0x23, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x24, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x24, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x24, 0x1a,
    0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x25, 0x04, 0x19, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x04, 0x25, 0x04, 0x24, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x25, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x25, 0x0a, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x25, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x26, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x04, 0x26,
    0x04, 0x25, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x26, 0x04,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x26, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x26, 0x13, 0x14, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x03, 0x12, 0x04, 0x29, 0x00, 0x37, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01,
    0x12, 0x03, 0x29, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x2a,
    0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0x2a, 0x04, 0x29,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2a, 0x04, 0x09, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2a, 0x0a, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2a, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x03, 0x08, 0x00, 0x12, 0x04, 0x2b, 0x04, 0x36, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x08,
    0x00, 0x01, 0x12, 0x03, 0x2b, 0x0a, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12,
    0x03, 0x2c, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2c,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2c, 0x0f, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2c, 0x1e, 0x1f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x2d, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2d, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x2d, 0x0f, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x2d, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x2e,
    0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x06, 0x12, 0x03, 0x2e, 0x08, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2e, 0x18, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2e, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x2f, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x2f, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x2f, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x2f, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x05, 0x12, 0x03, 0x30, 0x08, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x05, 0x12, 0x03, 0x30, 0x08, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x01, 0x12, 0x03, 0x30, 0x0f, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x05, 0x03, 0x12, 0x03, 0x30, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x06, 0x12, 0x03, 0x31, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x05,
    0x12, 0x03, 0x31, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x01, 0x12, 0x03,
    0x31, 0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x03, 0x12, 0x03, 0x31, 0x1c,
    0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x07, 0x12, 0x03, 0x32, 0x08, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x05, 0x12, 0x03, 0x32, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x07, 0x01, 0x12, 0x03, 0x32, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x07, 0x03, 0x12, 0x03, 0x32, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x08,
    0x12, 0x03, 0x33, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x05, 0x12, 0x03,
    0x33, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x01, 0x12, 0x03, 0x33, 0x0f,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x03, 0x12, 0x03, 0x33, 0x19, 0x1a, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x09, 0x12, 0x03, 0x34, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x09, 0x05, 0x12, 0x03, 0x34, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x09, 0x01, 0x12, 0x03, 0x34, 0x0f, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x09,
    0x03, 0x12, 0x03, 0x34, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x0a, 0x12, 0x03,
    0x35, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x35, 0x08,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x35, 0x0e, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x35, 0x15, 0x17, 0x62, 0x06, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x33,
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
