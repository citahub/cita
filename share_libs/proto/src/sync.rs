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
pub struct SyncRequest {
    // message fields
    pub heights: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SyncRequest {}

impl SyncRequest {
    pub fn new() -> SyncRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SyncRequest {
        static mut instance: ::protobuf::lazy::Lazy<SyncRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SyncRequest,
        };
        unsafe {
            instance.get(SyncRequest::new)
        }
    }

    // repeated uint64 heights = 1;

    pub fn clear_heights(&mut self) {
        self.heights.clear();
    }

    // Param is passed by value, moved
    pub fn set_heights(&mut self, v: ::std::vec::Vec<u64>) {
        self.heights = v;
    }

    // Mutable pointer to the field.
    pub fn mut_heights(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.heights
    }

    // Take field
    pub fn take_heights(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.heights, ::std::vec::Vec::new())
    }

    pub fn get_heights(&self) -> &[u64] {
        &self.heights
    }

    fn get_heights_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.heights
    }

    fn mut_heights_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.heights
    }
}

impl ::protobuf::Message for SyncRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.heights)?;
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
        for value in &self.heights {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.heights {
            os.write_uint64(1, *v)?;
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

impl ::protobuf::MessageStatic for SyncRequest {
    fn new() -> SyncRequest {
        SyncRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SyncRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "heights",
                    SyncRequest::get_heights_for_reflect,
                    SyncRequest::mut_heights_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SyncRequest>(
                    "SyncRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SyncRequest {
    fn clear(&mut self) {
        self.clear_heights();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SyncRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SyncRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SyncResponse {
    // message fields
    pub blocks: ::protobuf::RepeatedField<super::blockchain::Block>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SyncResponse {}

impl SyncResponse {
    pub fn new() -> SyncResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SyncResponse {
        static mut instance: ::protobuf::lazy::Lazy<SyncResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SyncResponse,
        };
        unsafe {
            instance.get(SyncResponse::new)
        }
    }

    // repeated .Block blocks = 1;

    pub fn clear_blocks(&mut self) {
        self.blocks.clear();
    }

    // Param is passed by value, moved
    pub fn set_blocks(&mut self, v: ::protobuf::RepeatedField<super::blockchain::Block>) {
        self.blocks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blocks(&mut self) -> &mut ::protobuf::RepeatedField<super::blockchain::Block> {
        &mut self.blocks
    }

    // Take field
    pub fn take_blocks(&mut self) -> ::protobuf::RepeatedField<super::blockchain::Block> {
        ::std::mem::replace(&mut self.blocks, ::protobuf::RepeatedField::new())
    }

    pub fn get_blocks(&self) -> &[super::blockchain::Block] {
        &self.blocks
    }

    fn get_blocks_for_reflect(&self) -> &::protobuf::RepeatedField<super::blockchain::Block> {
        &self.blocks
    }

    fn mut_blocks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::blockchain::Block> {
        &mut self.blocks
    }
}

impl ::protobuf::Message for SyncResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.blocks {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.blocks)?;
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
        for value in &self.blocks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.blocks {
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

impl ::protobuf::MessageStatic for SyncResponse {
    fn new() -> SyncResponse {
        SyncResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SyncResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockchain::Block>>(
                    "blocks",
                    SyncResponse::get_blocks_for_reflect,
                    SyncResponse::mut_blocks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SyncResponse>(
                    "SyncResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SyncResponse {
    fn clear(&mut self) {
        self.clear_blocks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SyncResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SyncResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nsync.proto\x1a\x10blockchain.proto\"'\n\x0bSyncRequest\x12\x18\n\x07\
    heights\x18\x01\x20\x03(\x04R\x07heights\".\n\x0cSyncResponse\x12\x1e\n\
    \x06blocks\x18\x01\x20\x03(\x0b2\x06.BlockR\x06blocksJ\xd7\x01\n\x06\x12\
    \x04\0\0\x0b\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\
    \x03\x07\x19\n\n\n\x02\x04\0\x12\x04\x05\0\x07\x01\n\n\n\x03\x04\0\x01\
    \x12\x03\x05\x08\x13\n\x0b\n\x04\x04\0\x02\0\x12\x03\x06\x04\x20\n\x0c\n\
    \x05\x04\0\x02\0\x04\x12\x03\x06\x04\x0c\n\x0c\n\x05\x04\0\x02\0\x05\x12\
    \x03\x06\r\x13\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x06\x14\x1b\n\x0c\n\
    \x05\x04\0\x02\0\x03\x12\x03\x06\x1e\x1f\n\n\n\x02\x04\x01\x12\x04\t\0\
    \x0b\x01\n\n\n\x03\x04\x01\x01\x12\x03\t\x08\x14\n\x0b\n\x04\x04\x01\x02\
    \0\x12\x03\n\x04\x1e\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\n\x04\x0c\n\
    \x0c\n\x05\x04\x01\x02\0\x06\x12\x03\n\r\x12\n\x0c\n\x05\x04\x01\x02\0\
    \x01\x12\x03\n\x13\x19\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\n\x1c\x1db\
    \x06proto3\
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
