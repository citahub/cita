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
pub struct Message {
    // message fields
    pub cmd_id: u32,
    pub field_type: MsgType,
    pub origin: u32,
    pub operate: OperateType,
    pub content: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Message {}

impl Message {
    pub fn new() -> Message {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Message {
        static mut instance: ::protobuf::lazy::Lazy<Message> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Message,
        };
        unsafe {
            instance.get(Message::new)
        }
    }

    // uint32 cmd_id = 1;

    pub fn clear_cmd_id(&mut self) {
        self.cmd_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_cmd_id(&mut self, v: u32) {
        self.cmd_id = v;
    }

    pub fn get_cmd_id(&self) -> u32 {
        self.cmd_id
    }

    fn get_cmd_id_for_reflect(&self) -> &u32 {
        &self.cmd_id
    }

    fn mut_cmd_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.cmd_id
    }

    // .MsgType type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = MsgType::REQUEST;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: MsgType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> MsgType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &MsgType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut MsgType {
        &mut self.field_type
    }

    // uint32 origin = 3;

    pub fn clear_origin(&mut self) {
        self.origin = 0;
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: u32) {
        self.origin = v;
    }

    pub fn get_origin(&self) -> u32 {
        self.origin
    }

    fn get_origin_for_reflect(&self) -> &u32 {
        &self.origin
    }

    fn mut_origin_for_reflect(&mut self) -> &mut u32 {
        &mut self.origin
    }

    // .OperateType operate = 4;

    pub fn clear_operate(&mut self) {
        self.operate = OperateType::BROADCAST;
    }

    // Param is passed by value, moved
    pub fn set_operate(&mut self, v: OperateType) {
        self.operate = v;
    }

    pub fn get_operate(&self) -> OperateType {
        self.operate
    }

    fn get_operate_for_reflect(&self) -> &OperateType {
        &self.operate
    }

    fn mut_operate_for_reflect(&mut self) -> &mut OperateType {
        &mut self.operate
    }

    // bytes content = 5;

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
}

impl ::protobuf::Message for Message {
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
                    let tmp = is.read_uint32()?;
                    self.cmd_id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.origin = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.operate = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.content)?;
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
        if self.cmd_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.cmd_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.field_type != MsgType::REQUEST {
            my_size += ::protobuf::rt::enum_size(2, self.field_type);
        }
        if self.origin != 0 {
            my_size += ::protobuf::rt::value_size(3, self.origin, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.operate != OperateType::BROADCAST {
            my_size += ::protobuf::rt::enum_size(4, self.operate);
        }
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.content);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.cmd_id != 0 {
            os.write_uint32(1, self.cmd_id)?;
        }
        if self.field_type != MsgType::REQUEST {
            os.write_enum(2, self.field_type.value())?;
        }
        if self.origin != 0 {
            os.write_uint32(3, self.origin)?;
        }
        if self.operate != OperateType::BROADCAST {
            os.write_enum(4, self.operate.value())?;
        }
        if !self.content.is_empty() {
            os.write_bytes(5, &self.content)?;
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

impl ::protobuf::MessageStatic for Message {
    fn new() -> Message {
        Message::new()
    }

    fn descriptor_static(_: ::std::option::Option<Message>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "cmd_id",
                    Message::get_cmd_id_for_reflect,
                    Message::mut_cmd_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MsgType>>(
                    "type",
                    Message::get_field_type_for_reflect,
                    Message::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "origin",
                    Message::get_origin_for_reflect,
                    Message::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OperateType>>(
                    "operate",
                    Message::get_operate_for_reflect,
                    Message::mut_operate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "content",
                    Message::get_content_for_reflect,
                    Message::mut_content_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Message>(
                    "Message",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Message {
    fn clear(&mut self) {
        self.clear_cmd_id();
        self.clear_field_type();
        self.clear_origin();
        self.clear_operate();
        self.clear_content();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MsgType {
    REQUEST = 0,
    HEADER = 1,
    BLOCK = 2,
    STATUS = 3,
    MSG = 4,
    RESPONSE = 5,
    VERIFY_TX_REQ = 6,
    VERIFY_TX_RESP = 7,
    VERIFY_BLK_REQ = 8,
    VERIFY_BLK_RESP = 9,
    BLOCK_TXHASHES = 10,
    BLOCK_TXHASHES_REQ = 11,
    BLOCK_WITH_PROOF = 12,
    BLOCK_TXS = 13,
    RICH_STATUS = 14,
    SYNC_REQ = 15,
    SYNC_RES = 16,
}

impl ::protobuf::ProtobufEnum for MsgType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MsgType> {
        match value {
            0 => ::std::option::Option::Some(MsgType::REQUEST),
            1 => ::std::option::Option::Some(MsgType::HEADER),
            2 => ::std::option::Option::Some(MsgType::BLOCK),
            3 => ::std::option::Option::Some(MsgType::STATUS),
            4 => ::std::option::Option::Some(MsgType::MSG),
            5 => ::std::option::Option::Some(MsgType::RESPONSE),
            6 => ::std::option::Option::Some(MsgType::VERIFY_TX_REQ),
            7 => ::std::option::Option::Some(MsgType::VERIFY_TX_RESP),
            8 => ::std::option::Option::Some(MsgType::VERIFY_BLK_REQ),
            9 => ::std::option::Option::Some(MsgType::VERIFY_BLK_RESP),
            10 => ::std::option::Option::Some(MsgType::BLOCK_TXHASHES),
            11 => ::std::option::Option::Some(MsgType::BLOCK_TXHASHES_REQ),
            12 => ::std::option::Option::Some(MsgType::BLOCK_WITH_PROOF),
            13 => ::std::option::Option::Some(MsgType::BLOCK_TXS),
            14 => ::std::option::Option::Some(MsgType::RICH_STATUS),
            15 => ::std::option::Option::Some(MsgType::SYNC_REQ),
            16 => ::std::option::Option::Some(MsgType::SYNC_RES),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MsgType] = &[
            MsgType::REQUEST,
            MsgType::HEADER,
            MsgType::BLOCK,
            MsgType::STATUS,
            MsgType::MSG,
            MsgType::RESPONSE,
            MsgType::VERIFY_TX_REQ,
            MsgType::VERIFY_TX_RESP,
            MsgType::VERIFY_BLK_REQ,
            MsgType::VERIFY_BLK_RESP,
            MsgType::BLOCK_TXHASHES,
            MsgType::BLOCK_TXHASHES_REQ,
            MsgType::BLOCK_WITH_PROOF,
            MsgType::BLOCK_TXS,
            MsgType::RICH_STATUS,
            MsgType::SYNC_REQ,
            MsgType::SYNC_RES,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<MsgType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MsgType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MsgType {
}

impl ::std::default::Default for MsgType {
    fn default() -> Self {
        MsgType::REQUEST
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum OperateType {
    BROADCAST = 0,
    SINGLE = 1,
    SUBTRACT = 2,
}

impl ::protobuf::ProtobufEnum for OperateType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OperateType> {
        match value {
            0 => ::std::option::Option::Some(OperateType::BROADCAST),
            1 => ::std::option::Option::Some(OperateType::SINGLE),
            2 => ::std::option::Option::Some(OperateType::SUBTRACT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [OperateType] = &[
            OperateType::BROADCAST,
            OperateType::SINGLE,
            OperateType::SUBTRACT,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<OperateType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("OperateType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for OperateType {
}

impl ::std::default::Default for OperateType {
    fn default() -> Self {
        OperateType::BROADCAST
    }
}

impl ::protobuf::reflect::ProtobufValue for OperateType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13communication.proto\"\x98\x01\n\x07Message\x12\x15\n\x06cmd_id\x18\
    \x01\x20\x01(\rR\x05cmdId\x12\x1c\n\x04type\x18\x02\x20\x01(\x0e2\x08.Ms\
    gTypeR\x04type\x12\x16\n\x06origin\x18\x03\x20\x01(\rR\x06origin\x12&\n\
    \x07operate\x18\x04\x20\x01(\x0e2\x0c.OperateTypeR\x07operate\x12\x18\n\
    \x07content\x18\x05\x20\x01(\x0cR\x07content*\x9e\x02\n\x07MsgType\x12\
    \x0b\n\x07REQUEST\x10\0\x12\n\n\x06HEADER\x10\x01\x12\t\n\x05BLOCK\x10\
    \x02\x12\n\n\x06STATUS\x10\x03\x12\x07\n\x03MSG\x10\x04\x12\x0c\n\x08RES\
    PONSE\x10\x05\x12\x11\n\rVERIFY_TX_REQ\x10\x06\x12\x12\n\x0eVERIFY_TX_RE\
    SP\x10\x07\x12\x12\n\x0eVERIFY_BLK_REQ\x10\x08\x12\x13\n\x0fVERIFY_BLK_R\
    ESP\x10\t\x12\x12\n\x0eBLOCK_TXHASHES\x10\n\x12\x16\n\x12BLOCK_TXHASHES_\
    REQ\x10\x0b\x12\x14\n\x10BLOCK_WITH_PROOF\x10\x0c\x12\r\n\tBLOCK_TXS\x10\
    \r\x12\x0f\n\x0bRICH_STATUS\x10\x0e\x12\x0c\n\x08SYNC_REQ\x10\x0f\x12\
    \x0c\n\x08SYNC_RES\x10\x10*6\n\x0bOperateType\x12\r\n\tBROADCAST\x10\0\
    \x12\n\n\x06SINGLE\x10\x01\x12\x0c\n\x08SUBTRACT\x10\x02J\xec\t\n\x06\
    \x12\x04\0\0\"\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x05\0\x12\
    \x04\x02\0\x14\x01\n\n\n\x03\x05\0\x01\x12\x03\x02\x05\x0c\n\x0b\n\x04\
    \x05\0\x02\0\x12\x03\x03\x04\x10\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x03\
    \x04\x0b\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x03\x0e\x0f\n\x0b\n\x04\x05\
    \0\x02\x01\x12\x03\x04\x04\x0f\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x04\
    \x04\n\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x04\r\x0e\n\x0b\n\x04\x05\0\
    \x02\x02\x12\x03\x05\x04\x0e\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\x05\
    \x04\t\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x05\x0c\r\n\x0b\n\x04\x05\0\
    \x02\x03\x12\x03\x06\x04\x0f\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03\x06\
    \x04\n\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03\x06\r\x0e\n\x0b\n\x04\x05\0\
    \x02\x04\x12\x03\x07\x04\x0c\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x03\x07\
    \x04\x07\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x07\n\x0b\n\x0b\n\x04\x05\
    \0\x02\x05\x12\x03\x08\x04\x11\n\x0c\n\x05\x05\0\x02\x05\x01\x12\x03\x08\
    \x04\x0c\n\x0c\n\x05\x05\0\x02\x05\x02\x12\x03\x08\x0f\x10\n\x0b\n\x04\
    \x05\0\x02\x06\x12\x03\t\x04\x16\n\x0c\n\x05\x05\0\x02\x06\x01\x12\x03\t\
    \x04\x11\n\x0c\n\x05\x05\0\x02\x06\x02\x12\x03\t\x14\x15\n\x0b\n\x04\x05\
    \0\x02\x07\x12\x03\n\x04\x17\n\x0c\n\x05\x05\0\x02\x07\x01\x12\x03\n\x04\
    \x12\n\x0c\n\x05\x05\0\x02\x07\x02\x12\x03\n\x15\x16\n\x0b\n\x04\x05\0\
    \x02\x08\x12\x03\x0b\x04\x17\n\x0c\n\x05\x05\0\x02\x08\x01\x12\x03\x0b\
    \x04\x12\n\x0c\n\x05\x05\0\x02\x08\x02\x12\x03\x0b\x15\x16\n\x0b\n\x04\
    \x05\0\x02\t\x12\x03\x0c\x04\x18\n\x0c\n\x05\x05\0\x02\t\x01\x12\x03\x0c\
    \x04\x13\n\x0c\n\x05\x05\0\x02\t\x02\x12\x03\x0c\x16\x17\n\x0b\n\x04\x05\
    \0\x02\n\x12\x03\r\x04\x18\n\x0c\n\x05\x05\0\x02\n\x01\x12\x03\r\x04\x12\
    \n\x0c\n\x05\x05\0\x02\n\x02\x12\x03\r\x15\x17\n\x0b\n\x04\x05\0\x02\x0b\
    \x12\x03\x0e\x04\x1c\n\x0c\n\x05\x05\0\x02\x0b\x01\x12\x03\x0e\x04\x16\n\
    \x0c\n\x05\x05\0\x02\x0b\x02\x12\x03\x0e\x19\x1b\n\x0b\n\x04\x05\0\x02\
    \x0c\x12\x03\x0f\x04\x1a\n\x0c\n\x05\x05\0\x02\x0c\x01\x12\x03\x0f\x04\
    \x14\n\x0c\n\x05\x05\0\x02\x0c\x02\x12\x03\x0f\x17\x19\n\x0b\n\x04\x05\0\
    \x02\r\x12\x03\x10\x04\x13\n\x0c\n\x05\x05\0\x02\r\x01\x12\x03\x10\x04\r\
    \n\x0c\n\x05\x05\0\x02\r\x02\x12\x03\x10\x10\x12\n\x0b\n\x04\x05\0\x02\
    \x0e\x12\x03\x11\x04\x15\n\x0c\n\x05\x05\0\x02\x0e\x01\x12\x03\x11\x04\
    \x0f\n\x0c\n\x05\x05\0\x02\x0e\x02\x12\x03\x11\x12\x14\n\x0b\n\x04\x05\0\
    \x02\x0f\x12\x03\x12\x04\x12\n\x0c\n\x05\x05\0\x02\x0f\x01\x12\x03\x12\
    \x04\x0c\n\x0c\n\x05\x05\0\x02\x0f\x02\x12\x03\x12\x0f\x11\n\x0b\n\x04\
    \x05\0\x02\x10\x12\x03\x13\x04\x12\n\x0c\n\x05\x05\0\x02\x10\x01\x12\x03\
    \x13\x04\x0c\n\x0c\n\x05\x05\0\x02\x10\x02\x12\x03\x13\x0f\x11\n\n\n\x02\
    \x05\x01\x12\x04\x16\0\x1a\x01\n\n\n\x03\x05\x01\x01\x12\x03\x16\x05\x10\
    \n\x0b\n\x04\x05\x01\x02\0\x12\x03\x17\x04\x12\n\x0c\n\x05\x05\x01\x02\0\
    \x01\x12\x03\x17\x04\r\n\x0c\n\x05\x05\x01\x02\0\x02\x12\x03\x17\x10\x11\
    \n\x0b\n\x04\x05\x01\x02\x01\x12\x03\x18\x04\x0f\n\x0c\n\x05\x05\x01\x02\
    \x01\x01\x12\x03\x18\x04\n\n\x0c\n\x05\x05\x01\x02\x01\x02\x12\x03\x18\r\
    \x0e\n\x0b\n\x04\x05\x01\x02\x02\x12\x03\x19\x04\x11\n\x0c\n\x05\x05\x01\
    \x02\x02\x01\x12\x03\x19\x04\x0c\n\x0c\n\x05\x05\x01\x02\x02\x02\x12\x03\
    \x19\x0f\x10\n\n\n\x02\x04\0\x12\x04\x1c\0\"\x01\n\n\n\x03\x04\0\x01\x12\
    \x03\x1c\x08\x0f\n\x0b\n\x04\x04\0\x02\0\x12\x03\x1d\x04\x16\n\r\n\x05\
    \x04\0\x02\0\x04\x12\x04\x1d\x04\x1c\x11\n\x0c\n\x05\x04\0\x02\0\x05\x12\
    \x03\x1d\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x1d\x0b\x11\n\x0c\n\
    \x05\x04\0\x02\0\x03\x12\x03\x1d\x14\x15\n\x0b\n\x04\x04\0\x02\x01\x12\
    \x03\x1e\x04\x15\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x1e\x04\x1d\x16\n\
    \x0c\n\x05\x04\0\x02\x01\x06\x12\x03\x1e\x04\x0b\n\x0c\n\x05\x04\0\x02\
    \x01\x01\x12\x03\x1e\x0c\x10\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x1e\
    \x13\x14\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x1f\x04\x16\n\r\n\x05\x04\0\
    \x02\x02\x04\x12\x04\x1f\x04\x1e\x15\n\x0c\n\x05\x04\0\x02\x02\x05\x12\
    \x03\x1f\x04\n\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x1f\x0b\x11\n\x0c\n\
    \x05\x04\0\x02\x02\x03\x12\x03\x1f\x14\x15\n\x0b\n\x04\x04\0\x02\x03\x12\
    \x03\x20\x04\x1c\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\x20\x04\x1f\x16\n\
    \x0c\n\x05\x04\0\x02\x03\x06\x12\x03\x20\x04\x0f\n\x0c\n\x05\x04\0\x02\
    \x03\x01\x12\x03\x20\x10\x17\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x20\
    \x1a\x1b\n\x0b\n\x04\x04\0\x02\x04\x12\x03!\x04\x16\n\r\n\x05\x04\0\x02\
    \x04\x04\x12\x04!\x04\x20\x1c\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03!\x04\
    \t\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03!\n\x11\n\x0c\n\x05\x04\0\x02\
    \x04\x03\x12\x03!\x14\x15b\x06proto3\
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
