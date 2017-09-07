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
pub struct VerifyReqMsg {
    // message fields
    pub valid_until_block: u64,
    pub hash: ::std::vec::Vec<u8>,
    pub signature: ::std::vec::Vec<u8>,
    pub tx_hash: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VerifyReqMsg {}

impl VerifyReqMsg {
    pub fn new() -> VerifyReqMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VerifyReqMsg {
        static mut instance: ::protobuf::lazy::Lazy<VerifyReqMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VerifyReqMsg,
        };
        unsafe {
            instance.get(VerifyReqMsg::new)
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

    // bytes tx_hash = 4;

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
}

impl ::protobuf::Message for VerifyReqMsg {
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
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.tx_hash)?;
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
        if !self.tx_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.tx_hash);
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
        if !self.tx_hash.is_empty() {
            os.write_bytes(4, &self.tx_hash)?;
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

impl ::protobuf::MessageStatic for VerifyReqMsg {
    fn new() -> VerifyReqMsg {
        VerifyReqMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<VerifyReqMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "valid_until_block",
                    VerifyReqMsg::get_valid_until_block_for_reflect,
                    VerifyReqMsg::mut_valid_until_block_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    VerifyReqMsg::get_hash_for_reflect,
                    VerifyReqMsg::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    VerifyReqMsg::get_signature_for_reflect,
                    VerifyReqMsg::mut_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tx_hash",
                    VerifyReqMsg::get_tx_hash_for_reflect,
                    VerifyReqMsg::mut_tx_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VerifyReqMsg>(
                    "VerifyReqMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VerifyReqMsg {
    fn clear(&mut self) {
        self.clear_valid_until_block();
        self.clear_hash();
        self.clear_signature();
        self.clear_tx_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VerifyReqMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VerifyReqMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VerifyReq {
    // message fields
    pub reqs: ::protobuf::RepeatedField<VerifyReqMsg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VerifyReq {}

impl VerifyReq {
    pub fn new() -> VerifyReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VerifyReq {
        static mut instance: ::protobuf::lazy::Lazy<VerifyReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VerifyReq,
        };
        unsafe {
            instance.get(VerifyReq::new)
        }
    }

    // repeated .VerifyReqMsg reqs = 1;

    pub fn clear_reqs(&mut self) {
        self.reqs.clear();
    }

    // Param is passed by value, moved
    pub fn set_reqs(&mut self, v: ::protobuf::RepeatedField<VerifyReqMsg>) {
        self.reqs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_reqs(&mut self) -> &mut ::protobuf::RepeatedField<VerifyReqMsg> {
        &mut self.reqs
    }

    // Take field
    pub fn take_reqs(&mut self) -> ::protobuf::RepeatedField<VerifyReqMsg> {
        ::std::mem::replace(&mut self.reqs, ::protobuf::RepeatedField::new())
    }

    pub fn get_reqs(&self) -> &[VerifyReqMsg] {
        &self.reqs
    }

    fn get_reqs_for_reflect(&self) -> &::protobuf::RepeatedField<VerifyReqMsg> {
        &self.reqs
    }

    fn mut_reqs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<VerifyReqMsg> {
        &mut self.reqs
    }
}

impl ::protobuf::Message for VerifyReq {
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
        for value in &self.reqs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.reqs {
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

impl ::protobuf::MessageStatic for VerifyReq {
    fn new() -> VerifyReq {
        VerifyReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<VerifyReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VerifyReqMsg>>(
                    "reqs",
                    VerifyReq::get_reqs_for_reflect,
                    VerifyReq::mut_reqs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VerifyReq>(
                    "VerifyReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VerifyReq {
    fn clear(&mut self) {
        self.clear_reqs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VerifyReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VerifyReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VerifyRespMsg {
    // message fields
    pub tx_hash: ::std::vec::Vec<u8>,
    pub ret: Ret,
    pub signer: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VerifyRespMsg {}

impl VerifyRespMsg {
    pub fn new() -> VerifyRespMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VerifyRespMsg {
        static mut instance: ::protobuf::lazy::Lazy<VerifyRespMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VerifyRespMsg,
        };
        unsafe {
            instance.get(VerifyRespMsg::new)
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

impl ::protobuf::Message for VerifyRespMsg {
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

impl ::protobuf::MessageStatic for VerifyRespMsg {
    fn new() -> VerifyRespMsg {
        VerifyRespMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<VerifyRespMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tx_hash",
                    VerifyRespMsg::get_tx_hash_for_reflect,
                    VerifyRespMsg::mut_tx_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Ret>>(
                    "ret",
                    VerifyRespMsg::get_ret_for_reflect,
                    VerifyRespMsg::mut_ret_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signer",
                    VerifyRespMsg::get_signer_for_reflect,
                    VerifyRespMsg::mut_signer_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VerifyRespMsg>(
                    "VerifyRespMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VerifyRespMsg {
    fn clear(&mut self) {
        self.clear_tx_hash();
        self.clear_ret();
        self.clear_signer();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VerifyRespMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VerifyRespMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VerifyResp {
    // message fields
    pub resps: ::protobuf::RepeatedField<VerifyRespMsg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VerifyResp {}

impl VerifyResp {
    pub fn new() -> VerifyResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VerifyResp {
        static mut instance: ::protobuf::lazy::Lazy<VerifyResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VerifyResp,
        };
        unsafe {
            instance.get(VerifyResp::new)
        }
    }

    // repeated .VerifyRespMsg resps = 1;

    pub fn clear_resps(&mut self) {
        self.resps.clear();
    }

    // Param is passed by value, moved
    pub fn set_resps(&mut self, v: ::protobuf::RepeatedField<VerifyRespMsg>) {
        self.resps = v;
    }

    // Mutable pointer to the field.
    pub fn mut_resps(&mut self) -> &mut ::protobuf::RepeatedField<VerifyRespMsg> {
        &mut self.resps
    }

    // Take field
    pub fn take_resps(&mut self) -> ::protobuf::RepeatedField<VerifyRespMsg> {
        ::std::mem::replace(&mut self.resps, ::protobuf::RepeatedField::new())
    }

    pub fn get_resps(&self) -> &[VerifyRespMsg] {
        &self.resps
    }

    fn get_resps_for_reflect(&self) -> &::protobuf::RepeatedField<VerifyRespMsg> {
        &self.resps
    }

    fn mut_resps_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<VerifyRespMsg> {
        &mut self.resps
    }
}

impl ::protobuf::Message for VerifyResp {
    fn is_initialized(&self) -> bool {
        for v in &self.resps {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.resps)?;
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
        for value in &self.resps {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.resps {
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

impl ::protobuf::MessageStatic for VerifyResp {
    fn new() -> VerifyResp {
        VerifyResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<VerifyResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VerifyRespMsg>>(
                    "resps",
                    VerifyResp::get_resps_for_reflect,
                    VerifyResp::mut_resps_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VerifyResp>(
                    "VerifyResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VerifyResp {
    fn clear(&mut self) {
        self.clear_resps();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VerifyResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VerifyResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Ret {
    Ok = 0,
    Dup = 1,
    OutOfTime = 2,
    BadSig = 3,
}

impl ::protobuf::ProtobufEnum for Ret {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Ret> {
        match value {
            0 => ::std::option::Option::Some(Ret::Ok),
            1 => ::std::option::Option::Some(Ret::Dup),
            2 => ::std::option::Option::Some(Ret::OutOfTime),
            3 => ::std::option::Option::Some(Ret::BadSig),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Ret] = &[
            Ret::Ok,
            Ret::Dup,
            Ret::OutOfTime,
            Ret::BadSig,
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
    \n\nauth.proto\"\x85\x01\n\x0cVerifyReqMsg\x12*\n\x11valid_until_block\
    \x18\x01\x20\x01(\x04R\x0fvalidUntilBlock\x12\x12\n\x04hash\x18\x02\x20\
    \x01(\x0cR\x04hash\x12\x1c\n\tsignature\x18\x03\x20\x01(\x0cR\tsignature\
    \x12\x17\n\x07tx_hash\x18\x04\x20\x01(\x0cR\x06txHash\".\n\tVerifyReq\
    \x12!\n\x04reqs\x18\x01\x20\x03(\x0b2\r.VerifyReqMsgR\x04reqs\"X\n\rVeri\
    fyRespMsg\x12\x17\n\x07tx_hash\x18\x01\x20\x01(\x0cR\x06txHash\x12\x16\n\
    \x03ret\x18\x02\x20\x01(\x0e2\x04.RetR\x03ret\x12\x16\n\x06signer\x18\
    \x03\x20\x01(\x0cR\x06signer\"2\n\nVerifyResp\x12$\n\x05resps\x18\x01\
    \x20\x03(\x0b2\x0e.VerifyRespMsgR\x05resps*1\n\x03Ret\x12\x06\n\x02Ok\
    \x10\0\x12\x07\n\x03Dup\x10\x01\x12\r\n\tOutOfTime\x10\x02\x12\n\n\x06Ba\
    dSig\x10\x03J\xaf\x07\n\x06\x12\x04\0\0\x1c\x01\n\x08\n\x01\x0c\x12\x03\
    \0\0\x12\n\n\n\x02\x05\0\x12\x04\x02\0\x07\x01\n\n\n\x03\x05\0\x01\x12\
    \x03\x02\x05\x08\n\x0b\n\x04\x05\0\x02\0\x12\x03\x03\x04\x0b\n\x0c\n\x05\
    \x05\0\x02\0\x01\x12\x03\x03\x04\x06\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\
    \x03\t\n\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x04\x04\x0c\n\x0c\n\x05\x05\0\
    \x02\x01\x01\x12\x03\x04\x04\x07\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\
    \x04\n\x0b\n\x0b\n\x04\x05\0\x02\x02\x12\x03\x05\x04\x12\n\x0c\n\x05\x05\
    \0\x02\x02\x01\x12\x03\x05\x04\r\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\
    \x05\x10\x11\n\x0b\n\x04\x05\0\x02\x03\x12\x03\x06\x04\x0f\n\x0c\n\x05\
    \x05\0\x02\x03\x01\x12\x03\x06\x04\n\n\x0c\n\x05\x05\0\x02\x03\x02\x12\
    \x03\x06\r\x0e\n\n\n\x02\x04\0\x12\x04\t\0\x0e\x01\n\n\n\x03\x04\0\x01\
    \x12\x03\t\x08\x14\n\x0b\n\x04\x04\0\x02\0\x12\x03\n\x04!\n\r\n\x05\x04\
    \0\x02\0\x04\x12\x04\n\x04\t\x16\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\n\
    \x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\n\x0b\x1c\n\x0c\n\x05\x04\0\
    \x02\0\x03\x12\x03\n\x1f\x20\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0b\x04\
    \x13\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x0b\x04\n!\n\x0c\n\x05\x04\0\
    \x02\x01\x05\x12\x03\x0b\x04\t\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0b\
    \n\x0e\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0b\x11\x12\n\x0b\n\x04\x04\
    \0\x02\x02\x12\x03\x0c\x04\x18\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x0c\
    \x04\x0b\x13\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x0c\x04\t\n\x0c\n\x05\
    \x04\0\x02\x02\x01\x12\x03\x0c\n\x13\n\x0c\n\x05\x04\0\x02\x02\x03\x12\
    \x03\x0c\x16\x17\n\x0b\n\x04\x04\0\x02\x03\x12\x03\r\x04\x16\n\r\n\x05\
    \x04\0\x02\x03\x04\x12\x04\r\x04\x0c\x18\n\x0c\n\x05\x04\0\x02\x03\x05\
    \x12\x03\r\x04\t\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\r\n\x11\n\x0c\n\
    \x05\x04\0\x02\x03\x03\x12\x03\r\x14\x15\n\n\n\x02\x04\x01\x12\x04\x10\0\
    \x12\x01\n\n\n\x03\x04\x01\x01\x12\x03\x10\x08\x11\n\x0b\n\x04\x04\x01\
    \x02\0\x12\x03\x11\x04#\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x11\x04\
    \x0c\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x11\r\x19\n\x0c\n\x05\x04\x01\
    \x02\0\x01\x12\x03\x11\x1a\x1e\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x11\
    !\"\n\n\n\x02\x04\x02\x12\x04\x14\0\x18\x01\n\n\n\x03\x04\x02\x01\x12\
    \x03\x14\x08\x15\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x15\x04\x16\n\r\n\x05\
    \x04\x02\x02\0\x04\x12\x04\x15\x04\x14\x17\n\x0c\n\x05\x04\x02\x02\0\x05\
    \x12\x03\x15\x04\t\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x15\n\x11\n\x0c\
    \n\x05\x04\x02\x02\0\x03\x12\x03\x15\x14\x15\n\x0b\n\x04\x04\x02\x02\x01\
    \x12\x03\x16\x04\x10\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\x16\x04\x15\
    \x16\n\x0c\n\x05\x04\x02\x02\x01\x06\x12\x03\x16\x04\x07\n\x0c\n\x05\x04\
    \x02\x02\x01\x01\x12\x03\x16\x08\x0b\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\
    \x03\x16\x0e\x0f\n\x18\n\x04\x04\x02\x02\x02\x12\x03\x17\x04\x15\"\x0bpu\
    blic\x20key\n\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04\x17\x04\x16\x10\n\
    \x0c\n\x05\x04\x02\x02\x02\x05\x12\x03\x17\x04\t\n\x0c\n\x05\x04\x02\x02\
    \x02\x01\x12\x03\x17\n\x10\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x17\
    \x13\x14\n\n\n\x02\x04\x03\x12\x04\x1a\0\x1c\x01\n\n\n\x03\x04\x03\x01\
    \x12\x03\x1a\x08\x12\n\x0b\n\x04\x04\x03\x02\0\x12\x03\x1b\x04%\n\x0c\n\
    \x05\x04\x03\x02\0\x04\x12\x03\x1b\x04\x0c\n\x0c\n\x05\x04\x03\x02\0\x06\
    \x12\x03\x1b\r\x1a\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x1b\x1b\x20\n\
    \x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x1b#$b\x06proto3\
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
