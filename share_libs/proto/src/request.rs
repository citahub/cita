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
pub struct BlockParamsByHash {
    // message fields
    pub hash: ::std::vec::Vec<u8>,
    pub include_txs: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockParamsByHash {}

impl BlockParamsByHash {
    pub fn new() -> BlockParamsByHash {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockParamsByHash {
        static mut instance: ::protobuf::lazy::Lazy<BlockParamsByHash> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockParamsByHash,
        };
        unsafe {
            instance.get(BlockParamsByHash::new)
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

    // bool include_txs = 2;

    pub fn clear_include_txs(&mut self) {
        self.include_txs = false;
    }

    // Param is passed by value, moved
    pub fn set_include_txs(&mut self, v: bool) {
        self.include_txs = v;
    }

    pub fn get_include_txs(&self) -> bool {
        self.include_txs
    }

    fn get_include_txs_for_reflect(&self) -> &bool {
        &self.include_txs
    }

    fn mut_include_txs_for_reflect(&mut self) -> &mut bool {
        &mut self.include_txs
    }
}

impl ::protobuf::Message for BlockParamsByHash {
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
                    let tmp = is.read_bool()?;
                    self.include_txs = tmp;
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
        if self.include_txs != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.hash.is_empty() {
            os.write_bytes(1, &self.hash)?;
        }
        if self.include_txs != false {
            os.write_bool(2, self.include_txs)?;
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

impl ::protobuf::MessageStatic for BlockParamsByHash {
    fn new() -> BlockParamsByHash {
        BlockParamsByHash::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockParamsByHash>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    BlockParamsByHash::get_hash_for_reflect,
                    BlockParamsByHash::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "include_txs",
                    BlockParamsByHash::get_include_txs_for_reflect,
                    BlockParamsByHash::mut_include_txs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockParamsByHash>(
                    "BlockParamsByHash",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockParamsByHash {
    fn clear(&mut self) {
        self.clear_hash();
        self.clear_include_txs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockParamsByHash {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockParamsByHash {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockParamsByNumber {
    // message fields
    pub height: u64,
    pub include_txs: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockParamsByNumber {}

impl BlockParamsByNumber {
    pub fn new() -> BlockParamsByNumber {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockParamsByNumber {
        static mut instance: ::protobuf::lazy::Lazy<BlockParamsByNumber> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockParamsByNumber,
        };
        unsafe {
            instance.get(BlockParamsByNumber::new)
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

    // bool include_txs = 2;

    pub fn clear_include_txs(&mut self) {
        self.include_txs = false;
    }

    // Param is passed by value, moved
    pub fn set_include_txs(&mut self, v: bool) {
        self.include_txs = v;
    }

    pub fn get_include_txs(&self) -> bool {
        self.include_txs
    }

    fn get_include_txs_for_reflect(&self) -> &bool {
        &self.include_txs
    }

    fn mut_include_txs_for_reflect(&mut self) -> &mut bool {
        &mut self.include_txs
    }
}

impl ::protobuf::Message for BlockParamsByNumber {
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
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.include_txs = tmp;
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
        if self.include_txs != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.height != 0 {
            os.write_uint64(1, self.height)?;
        }
        if self.include_txs != false {
            os.write_bool(2, self.include_txs)?;
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

impl ::protobuf::MessageStatic for BlockParamsByNumber {
    fn new() -> BlockParamsByNumber {
        BlockParamsByNumber::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockParamsByNumber>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    BlockParamsByNumber::get_height_for_reflect,
                    BlockParamsByNumber::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "include_txs",
                    BlockParamsByNumber::get_include_txs_for_reflect,
                    BlockParamsByNumber::mut_include_txs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockParamsByNumber>(
                    "BlockParamsByNumber",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockParamsByNumber {
    fn clear(&mut self) {
        self.clear_height();
        self.clear_include_txs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockParamsByNumber {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockParamsByNumber {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Call {
    // message fields
    pub from: ::std::vec::Vec<u8>,
    pub to: ::std::vec::Vec<u8>,
    pub data: ::std::vec::Vec<u8>,
    // message oneof groups
    block_id: ::std::option::Option<Call_oneof_block_id>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Call {}

#[derive(Clone,PartialEq)]
pub enum Call_oneof_block_id {
    tag(BlockTag),
    height(u64),
}

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

    // .BlockTag tag = 4;

    pub fn clear_tag(&mut self) {
        self.block_id = ::std::option::Option::None;
    }

    pub fn has_tag(&self) -> bool {
        match self.block_id {
            ::std::option::Option::Some(Call_oneof_block_id::tag(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tag(&mut self, v: BlockTag) {
        self.block_id = ::std::option::Option::Some(Call_oneof_block_id::tag(v))
    }

    pub fn get_tag(&self) -> BlockTag {
        match self.block_id {
            ::std::option::Option::Some(Call_oneof_block_id::tag(v)) => v,
            _ => BlockTag::Latest,
        }
    }

    // uint64 height = 5;

    pub fn clear_height(&mut self) {
        self.block_id = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        match self.block_id {
            ::std::option::Option::Some(Call_oneof_block_id::height(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u64) {
        self.block_id = ::std::option::Option::Some(Call_oneof_block_id::height(v))
    }

    pub fn get_height(&self) -> u64 {
        match self.block_id {
            ::std::option::Option::Some(Call_oneof_block_id::height(v)) => v,
            _ => 0,
        }
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.block_id = ::std::option::Option::Some(Call_oneof_block_id::tag(is.read_enum()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.block_id = ::std::option::Option::Some(Call_oneof_block_id::height(is.read_uint64()?));
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
        if let ::std::option::Option::Some(ref v) = self.block_id {
            match v {
                &Call_oneof_block_id::tag(v) => {
                    my_size += ::protobuf::rt::enum_size(4, v);
                },
                &Call_oneof_block_id::height(v) => {
                    my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
                },
            };
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
        if let ::std::option::Option::Some(ref v) = self.block_id {
            match v {
                &Call_oneof_block_id::tag(v) => {
                    os.write_enum(4, v.value())?;
                },
                &Call_oneof_block_id::height(v) => {
                    os.write_uint64(5, v)?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor::<_, BlockTag>(
                    "tag",
                    Call::has_tag,
                    Call::get_tag,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "height",
                    Call::has_height,
                    Call::get_height,
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
        self.clear_tag();
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
    block_by_hash(BlockParamsByHash),
    block_by_height(BlockParamsByNumber),
    transaction(::std::vec::Vec<u8>),
    height(u64),
    peercount(bool),
    call(Call),
    filter(::std::string::String),
    transaction_receipt(::std::vec::Vec<u8>),
    transaction_count(TransactionCount),
    code(Code),
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

    // .BlockParamsByHash block_by_hash = 3;

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
    pub fn set_block_by_hash(&mut self, v: BlockParamsByHash) {
        self.req = ::std::option::Option::Some(Request_oneof_req::block_by_hash(v))
    }

    // Mutable pointer to the field.
    pub fn mut_block_by_hash(&mut self) -> &mut BlockParamsByHash {
        if let ::std::option::Option::Some(Request_oneof_req::block_by_hash(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(Request_oneof_req::block_by_hash(BlockParamsByHash::new()));
        }
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::block_by_hash(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_block_by_hash(&mut self) -> BlockParamsByHash {
        if self.has_block_by_hash() {
            match self.req.take() {
                ::std::option::Option::Some(Request_oneof_req::block_by_hash(v)) => v,
                _ => panic!(),
            }
        } else {
            BlockParamsByHash::new()
        }
    }

    pub fn get_block_by_hash(&self) -> &BlockParamsByHash {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::block_by_hash(ref v)) => v,
            _ => BlockParamsByHash::default_instance(),
        }
    }

    // .BlockParamsByNumber block_by_height = 4;

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
    pub fn set_block_by_height(&mut self, v: BlockParamsByNumber) {
        self.req = ::std::option::Option::Some(Request_oneof_req::block_by_height(v))
    }

    // Mutable pointer to the field.
    pub fn mut_block_by_height(&mut self) -> &mut BlockParamsByNumber {
        if let ::std::option::Option::Some(Request_oneof_req::block_by_height(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(Request_oneof_req::block_by_height(BlockParamsByNumber::new()));
        }
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::block_by_height(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_block_by_height(&mut self) -> BlockParamsByNumber {
        if self.has_block_by_height() {
            match self.req.take() {
                ::std::option::Option::Some(Request_oneof_req::block_by_height(v)) => v,
                _ => panic!(),
            }
        } else {
            BlockParamsByNumber::new()
        }
    }

    pub fn get_block_by_height(&self) -> &BlockParamsByNumber {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::block_by_height(ref v)) => v,
            _ => BlockParamsByNumber::default_instance(),
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

    // .TransactionCount transaction_count = 11;

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
    pub fn set_transaction_count(&mut self, v: TransactionCount) {
        self.req = ::std::option::Option::Some(Request_oneof_req::transaction_count(v))
    }

    // Mutable pointer to the field.
    pub fn mut_transaction_count(&mut self) -> &mut TransactionCount {
        if let ::std::option::Option::Some(Request_oneof_req::transaction_count(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(Request_oneof_req::transaction_count(TransactionCount::new()));
        }
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::transaction_count(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_transaction_count(&mut self) -> TransactionCount {
        if self.has_transaction_count() {
            match self.req.take() {
                ::std::option::Option::Some(Request_oneof_req::transaction_count(v)) => v,
                _ => panic!(),
            }
        } else {
            TransactionCount::new()
        }
    }

    pub fn get_transaction_count(&self) -> &TransactionCount {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::transaction_count(ref v)) => v,
            _ => TransactionCount::default_instance(),
        }
    }

    // .Code code = 12;

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
    pub fn set_code(&mut self, v: Code) {
        self.req = ::std::option::Option::Some(Request_oneof_req::code(v))
    }

    // Mutable pointer to the field.
    pub fn mut_code(&mut self) -> &mut Code {
        if let ::std::option::Option::Some(Request_oneof_req::code(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(Request_oneof_req::code(Code::new()));
        }
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::code(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_code(&mut self) -> Code {
        if self.has_code() {
            match self.req.take() {
                ::std::option::Option::Some(Request_oneof_req::code(v)) => v,
                _ => panic!(),
            }
        } else {
            Code::new()
        }
    }

    pub fn get_code(&self) -> &Code {
        match self.req {
            ::std::option::Option::Some(Request_oneof_req::code(ref v)) => v,
            _ => Code::default_instance(),
        }
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        if let Some(Request_oneof_req::block_by_hash(ref v)) = self.req {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_req::block_by_height(ref v)) = self.req {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_req::call(ref v)) = self.req {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_req::transaction_count(ref v)) = self.req {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_req::code(ref v)) = self.req {
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
                    self.req = ::std::option::Option::Some(Request_oneof_req::block_by_hash(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.req = ::std::option::Option::Some(Request_oneof_req::block_by_height(is.read_message()?));
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
                    self.req = ::std::option::Option::Some(Request_oneof_req::transaction_count(is.read_message()?));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.req = ::std::option::Option::Some(Request_oneof_req::code(is.read_message()?));
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
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_req::block_by_height(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_req::code(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_req::block_by_height(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
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
                    os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_req::code(ref v) => {
                    os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, BlockParamsByHash>(
                    "block_by_hash",
                    Request::has_block_by_hash,
                    Request::get_block_by_hash,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, BlockParamsByNumber>(
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, TransactionCount>(
                    "transaction_count",
                    Request::has_transaction_count,
                    Request::get_transaction_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Code>(
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
pub struct RpcBlock {
    // message fields
    pub block: ::std::vec::Vec<u8>,
    pub include_txs: bool,
    pub hash: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpcBlock {}

impl RpcBlock {
    pub fn new() -> RpcBlock {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpcBlock {
        static mut instance: ::protobuf::lazy::Lazy<RpcBlock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpcBlock,
        };
        unsafe {
            instance.get(RpcBlock::new)
        }
    }

    // bytes block = 1;

    pub fn clear_block(&mut self) {
        self.block.clear();
    }

    // Param is passed by value, moved
    pub fn set_block(&mut self, v: ::std::vec::Vec<u8>) {
        self.block = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.block
    }

    // Take field
    pub fn take_block(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.block, ::std::vec::Vec::new())
    }

    pub fn get_block(&self) -> &[u8] {
        &self.block
    }

    fn get_block_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.block
    }

    fn mut_block_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.block
    }

    // bool include_txs = 2;

    pub fn clear_include_txs(&mut self) {
        self.include_txs = false;
    }

    // Param is passed by value, moved
    pub fn set_include_txs(&mut self, v: bool) {
        self.include_txs = v;
    }

    pub fn get_include_txs(&self) -> bool {
        self.include_txs
    }

    fn get_include_txs_for_reflect(&self) -> &bool {
        &self.include_txs
    }

    fn mut_include_txs_for_reflect(&mut self) -> &mut bool {
        &mut self.include_txs
    }

    // bytes hash = 3;

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
}

impl ::protobuf::Message for RpcBlock {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.block)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.include_txs = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.hash)?;
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
        if !self.block.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.block);
        }
        if self.include_txs != false {
            my_size += 2;
        }
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.block.is_empty() {
            os.write_bytes(1, &self.block)?;
        }
        if self.include_txs != false {
            os.write_bool(2, self.include_txs)?;
        }
        if !self.hash.is_empty() {
            os.write_bytes(3, &self.hash)?;
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

impl ::protobuf::MessageStatic for RpcBlock {
    fn new() -> RpcBlock {
        RpcBlock::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpcBlock>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "block",
                    RpcBlock::get_block_for_reflect,
                    RpcBlock::mut_block_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "include_txs",
                    RpcBlock::get_include_txs_for_reflect,
                    RpcBlock::mut_include_txs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    RpcBlock::get_hash_for_reflect,
                    RpcBlock::mut_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpcBlock>(
                    "RpcBlock",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpcBlock {
    fn clear(&mut self) {
        self.clear_block();
        self.clear_include_txs();
        self.clear_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpcBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpcBlock {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FullTransaction {
    // message fields
    pub transaction: ::protobuf::SingularPtrField<super::blockchain::Transaction>,
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

    // .Transaction transaction = 1;

    pub fn clear_transaction(&mut self) {
        self.transaction.clear();
    }

    pub fn has_transaction(&self) -> bool {
        self.transaction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transaction(&mut self, v: super::blockchain::Transaction) {
        self.transaction = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transaction(&mut self) -> &mut super::blockchain::Transaction {
        if self.transaction.is_none() {
            self.transaction.set_default();
        }
        self.transaction.as_mut().unwrap()
    }

    // Take field
    pub fn take_transaction(&mut self) -> super::blockchain::Transaction {
        self.transaction.take().unwrap_or_else(|| super::blockchain::Transaction::new())
    }

    pub fn get_transaction(&self) -> &super::blockchain::Transaction {
        self.transaction.as_ref().unwrap_or_else(|| super::blockchain::Transaction::default_instance())
    }

    fn get_transaction_for_reflect(&self) -> &::protobuf::SingularPtrField<super::blockchain::Transaction> {
        &self.transaction
    }

    fn mut_transaction_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::blockchain::Transaction> {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.block_number = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.block_hash)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
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
        if let Some(ref v) = self.transaction.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.block_number != 0 {
            my_size += ::protobuf::rt::value_size(2, self.block_number, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.block_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.block_hash);
        }
        if self.index != 0 {
            my_size += ::protobuf::rt::value_size(4, self.index, ::protobuf::wire_format::WireTypeVarint);
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
        if self.block_number != 0 {
            os.write_uint64(2, self.block_number)?;
        }
        if !self.block_hash.is_empty() {
            os.write_bytes(3, &self.block_hash)?;
        }
        if self.index != 0 {
            os.write_uint32(4, self.index)?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockchain::Transaction>>(
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
    block(RpcBlock),
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

    // .RpcBlock block = 3;

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
    pub fn set_block(&mut self, v: RpcBlock) {
        self.result = ::std::option::Option::Some(Response_oneof_result::block(v))
    }

    // Mutable pointer to the field.
    pub fn mut_block(&mut self) -> &mut RpcBlock {
        if let ::std::option::Option::Some(Response_oneof_result::block(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(Response_oneof_result::block(RpcBlock::new()));
        }
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::block(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_block(&mut self) -> RpcBlock {
        if self.has_block() {
            match self.result.take() {
                ::std::option::Option::Some(Response_oneof_result::block(v)) => v,
                _ => panic!(),
            }
        } else {
            RpcBlock::new()
        }
    }

    pub fn get_block(&self) -> &RpcBlock {
        match self.result {
            ::std::option::Option::Some(Response_oneof_result::block(ref v)) => v,
            _ => RpcBlock::default_instance(),
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
        if let Some(Response_oneof_result::block(ref v)) = self.result {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_result::ts(ref v)) = self.result {
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
                    self.result = ::std::option::Option::Some(Response_oneof_result::block_number(is.read_uint64()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.result = ::std::option::Option::Some(Response_oneof_result::block(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.result = ::std::option::Option::Some(Response_oneof_result::ts(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.result = ::std::option::Option::Some(Response_oneof_result::none(is.read_bool()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.result = ::std::option::Option::Some(Response_oneof_result::peercount(is.read_uint32()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.result = ::std::option::Option::Some(Response_oneof_result::call_result(is.read_bytes()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.result = ::std::option::Option::Some(Response_oneof_result::logs(is.read_string()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.result = ::std::option::Option::Some(Response_oneof_result::receipt(is.read_string()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.result = ::std::option::Option::Some(Response_oneof_result::transaction_count(is.read_uint64()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
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
        }
        if let ::std::option::Option::Some(ref v) = self.result {
            match v {
                &Response_oneof_result::block_number(v) => {
                    my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Response_oneof_result::block(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.request_id.is_empty() {
            os.write_bytes(1, &self.request_id)?;
        }
        if let ::std::option::Option::Some(ref v) = self.result {
            match v {
                &Response_oneof_result::block_number(v) => {
                    os.write_uint64(2, v)?;
                },
                &Response_oneof_result::block(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RpcBlock>(
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

#[derive(PartialEq,Clone,Default)]
pub struct TransactionCount {
    // message fields
    pub address: ::std::vec::Vec<u8>,
    // message oneof groups
    block_id: ::std::option::Option<TransactionCount_oneof_block_id>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TransactionCount {}

#[derive(Clone,PartialEq)]
pub enum TransactionCount_oneof_block_id {
    tag(BlockTag),
    height(u64),
}

impl TransactionCount {
    pub fn new() -> TransactionCount {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransactionCount {
        static mut instance: ::protobuf::lazy::Lazy<TransactionCount> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransactionCount,
        };
        unsafe {
            instance.get(TransactionCount::new)
        }
    }

    // bytes address = 1;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::vec::Vec<u8>) {
        self.address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.address, ::std::vec::Vec::new())
    }

    pub fn get_address(&self) -> &[u8] {
        &self.address
    }

    fn get_address_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.address
    }

    fn mut_address_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.address
    }

    // .BlockTag tag = 2;

    pub fn clear_tag(&mut self) {
        self.block_id = ::std::option::Option::None;
    }

    pub fn has_tag(&self) -> bool {
        match self.block_id {
            ::std::option::Option::Some(TransactionCount_oneof_block_id::tag(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tag(&mut self, v: BlockTag) {
        self.block_id = ::std::option::Option::Some(TransactionCount_oneof_block_id::tag(v))
    }

    pub fn get_tag(&self) -> BlockTag {
        match self.block_id {
            ::std::option::Option::Some(TransactionCount_oneof_block_id::tag(v)) => v,
            _ => BlockTag::Latest,
        }
    }

    // uint64 height = 3;

    pub fn clear_height(&mut self) {
        self.block_id = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        match self.block_id {
            ::std::option::Option::Some(TransactionCount_oneof_block_id::height(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u64) {
        self.block_id = ::std::option::Option::Some(TransactionCount_oneof_block_id::height(v))
    }

    pub fn get_height(&self) -> u64 {
        match self.block_id {
            ::std::option::Option::Some(TransactionCount_oneof_block_id::height(v)) => v,
            _ => 0,
        }
    }
}

impl ::protobuf::Message for TransactionCount {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.address)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.block_id = ::std::option::Option::Some(TransactionCount_oneof_block_id::tag(is.read_enum()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.block_id = ::std::option::Option::Some(TransactionCount_oneof_block_id::height(is.read_uint64()?));
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
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.address);
        }
        if let ::std::option::Option::Some(ref v) = self.block_id {
            match v {
                &TransactionCount_oneof_block_id::tag(v) => {
                    my_size += ::protobuf::rt::enum_size(2, v);
                },
                &TransactionCount_oneof_block_id::height(v) => {
                    my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.address.is_empty() {
            os.write_bytes(1, &self.address)?;
        }
        if let ::std::option::Option::Some(ref v) = self.block_id {
            match v {
                &TransactionCount_oneof_block_id::tag(v) => {
                    os.write_enum(2, v.value())?;
                },
                &TransactionCount_oneof_block_id::height(v) => {
                    os.write_uint64(3, v)?;
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

impl ::protobuf::MessageStatic for TransactionCount {
    fn new() -> TransactionCount {
        TransactionCount::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransactionCount>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "address",
                    TransactionCount::get_address_for_reflect,
                    TransactionCount::mut_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor::<_, BlockTag>(
                    "tag",
                    TransactionCount::has_tag,
                    TransactionCount::get_tag,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "height",
                    TransactionCount::has_height,
                    TransactionCount::get_height,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TransactionCount>(
                    "TransactionCount",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransactionCount {
    fn clear(&mut self) {
        self.clear_address();
        self.clear_tag();
        self.clear_height();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TransactionCount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TransactionCount {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Code {
    // message fields
    pub address: ::std::vec::Vec<u8>,
    // message oneof groups
    block_id: ::std::option::Option<Code_oneof_block_id>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Code {}

#[derive(Clone,PartialEq)]
pub enum Code_oneof_block_id {
    tag(BlockTag),
    height(u64),
}

impl Code {
    pub fn new() -> Code {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Code {
        static mut instance: ::protobuf::lazy::Lazy<Code> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Code,
        };
        unsafe {
            instance.get(Code::new)
        }
    }

    // bytes address = 1;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::vec::Vec<u8>) {
        self.address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.address, ::std::vec::Vec::new())
    }

    pub fn get_address(&self) -> &[u8] {
        &self.address
    }

    fn get_address_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.address
    }

    fn mut_address_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.address
    }

    // .BlockTag tag = 2;

    pub fn clear_tag(&mut self) {
        self.block_id = ::std::option::Option::None;
    }

    pub fn has_tag(&self) -> bool {
        match self.block_id {
            ::std::option::Option::Some(Code_oneof_block_id::tag(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tag(&mut self, v: BlockTag) {
        self.block_id = ::std::option::Option::Some(Code_oneof_block_id::tag(v))
    }

    pub fn get_tag(&self) -> BlockTag {
        match self.block_id {
            ::std::option::Option::Some(Code_oneof_block_id::tag(v)) => v,
            _ => BlockTag::Latest,
        }
    }

    // uint64 height = 3;

    pub fn clear_height(&mut self) {
        self.block_id = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        match self.block_id {
            ::std::option::Option::Some(Code_oneof_block_id::height(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u64) {
        self.block_id = ::std::option::Option::Some(Code_oneof_block_id::height(v))
    }

    pub fn get_height(&self) -> u64 {
        match self.block_id {
            ::std::option::Option::Some(Code_oneof_block_id::height(v)) => v,
            _ => 0,
        }
    }
}

impl ::protobuf::Message for Code {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.address)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.block_id = ::std::option::Option::Some(Code_oneof_block_id::tag(is.read_enum()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.block_id = ::std::option::Option::Some(Code_oneof_block_id::height(is.read_uint64()?));
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
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.address);
        }
        if let ::std::option::Option::Some(ref v) = self.block_id {
            match v {
                &Code_oneof_block_id::tag(v) => {
                    my_size += ::protobuf::rt::enum_size(2, v);
                },
                &Code_oneof_block_id::height(v) => {
                    my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.address.is_empty() {
            os.write_bytes(1, &self.address)?;
        }
        if let ::std::option::Option::Some(ref v) = self.block_id {
            match v {
                &Code_oneof_block_id::tag(v) => {
                    os.write_enum(2, v.value())?;
                },
                &Code_oneof_block_id::height(v) => {
                    os.write_uint64(3, v)?;
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

impl ::protobuf::MessageStatic for Code {
    fn new() -> Code {
        Code::new()
    }

    fn descriptor_static(_: ::std::option::Option<Code>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "address",
                    Code::get_address_for_reflect,
                    Code::mut_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor::<_, BlockTag>(
                    "tag",
                    Code::has_tag,
                    Code::get_tag,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "height",
                    Code::has_height,
                    Code::get_height,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Code>(
                    "Code",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Code {
    fn clear(&mut self) {
        self.clear_address();
        self.clear_tag();
        self.clear_height();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Code {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Code {
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
    \n\rrequest.proto\x1a\x10blockchain.proto\"H\n\x11BlockParamsByHash\x12\
    \x12\n\x04hash\x18\x01\x20\x01(\x0cR\x04hash\x12\x1f\n\x0binclude_txs\
    \x18\x02\x20\x01(\x08R\nincludeTxs\"N\n\x13BlockParamsByNumber\x12\x16\n\
    \x06height\x18\x01\x20\x01(\x04R\x06height\x12\x1f\n\x0binclude_txs\x18\
    \x02\x20\x01(\x08R\nincludeTxs\"\x83\x01\n\x04Call\x12\x12\n\x04from\x18\
    \x01\x20\x01(\x0cR\x04from\x12\x0e\n\x02to\x18\x02\x20\x01(\x0cR\x02to\
    \x12\x12\n\x04data\x18\x03\x20\x01(\x0cR\x04data\x12\x1d\n\x03tag\x18\
    \x04\x20\x01(\x0e2\t.BlockTagH\0R\x03tag\x12\x18\n\x06height\x18\x05\x20\
    \x01(\x04H\0R\x06heightB\n\n\x08block_id\"\xf5\x03\n\x07Request\x12\x1d\
    \n\nrequest_id\x18\x01\x20\x01(\x0cR\trequestId\x12#\n\x0cblock_number\
    \x18\x02\x20\x01(\x08H\0R\x0bblockNumber\x128\n\rblock_by_hash\x18\x03\
    \x20\x01(\x0b2\x12.BlockParamsByHashH\0R\x0bblockByHash\x12>\n\x0fblock_\
    by_height\x18\x04\x20\x01(\x0b2\x14.BlockParamsByNumberH\0R\rblockByHeig\
    ht\x12\"\n\x0btransaction\x18\x05\x20\x01(\x0cH\0R\x0btransaction\x12\
    \x18\n\x06height\x18\x06\x20\x01(\x04H\0R\x06height\x12\x1e\n\tpeercount\
    \x18\x07\x20\x01(\x08H\0R\tpeercount\x12\x1b\n\x04call\x18\x08\x20\x01(\
    \x0b2\x05.CallH\0R\x04call\x12\x18\n\x06filter\x18\t\x20\x01(\tH\0R\x06f\
    ilter\x121\n\x13transaction_receipt\x18\n\x20\x01(\x0cH\0R\x12transactio\
    nReceipt\x12@\n\x11transaction_count\x18\x0b\x20\x01(\x0b2\x11.Transacti\
    onCountH\0R\x10transactionCount\x12\x1b\n\x04code\x18\x0c\x20\x01(\x0b2\
    \x05.CodeH\0R\x04codeB\x05\n\x03req\"U\n\x08RpcBlock\x12\x14\n\x05block\
    \x18\x01\x20\x01(\x0cR\x05block\x12\x1f\n\x0binclude_txs\x18\x02\x20\x01\
    (\x08R\nincludeTxs\x12\x12\n\x04hash\x18\x03\x20\x01(\x0cR\x04hash\"\x99\
    \x01\n\x0fFullTransaction\x12.\n\x0btransaction\x18\x01\x20\x01(\x0b2\
    \x0c.TransactionR\x0btransaction\x12!\n\x0cblock_number\x18\x02\x20\x01(\
    \x04R\x0bblockNumber\x12\x1d\n\nblock_hash\x18\x03\x20\x01(\x0cR\tblockH\
    ash\x12\x14\n\x05index\x18\x04\x20\x01(\rR\x05index\"\xef\x02\n\x08Respo\
    nse\x12\x1d\n\nrequest_id\x18\x01\x20\x01(\x0cR\trequestId\x12#\n\x0cblo\
    ck_number\x18\x02\x20\x01(\x04H\0R\x0bblockNumber\x12!\n\x05block\x18\
    \x03\x20\x01(\x0b2\t.RpcBlockH\0R\x05block\x12\"\n\x02ts\x18\x04\x20\x01\
    (\x0b2\x10.FullTransactionH\0R\x02ts\x12\x14\n\x04none\x18\x05\x20\x01(\
    \x08H\0R\x04none\x12\x1e\n\tpeercount\x18\x06\x20\x01(\rH\0R\tpeercount\
    \x12!\n\x0bcall_result\x18\x07\x20\x01(\x0cH\0R\ncallResult\x12\x14\n\
    \x04logs\x18\x08\x20\x01(\tH\0R\x04logs\x12\x1a\n\x07receipt\x18\t\x20\
    \x01(\tH\0R\x07receipt\x12-\n\x11transaction_count\x18\n\x20\x01(\x04H\0\
    R\x10transactionCount\x12\x14\n\x04code\x18\x0b\x20\x01(\x0cH\0R\x04code\
    B\x08\n\x06result\"q\n\x10TransactionCount\x12\x18\n\x07address\x18\x01\
    \x20\x01(\x0cR\x07address\x12\x1d\n\x03tag\x18\x02\x20\x01(\x0e2\t.Block\
    TagH\0R\x03tag\x12\x18\n\x06height\x18\x03\x20\x01(\x04H\0R\x06heightB\n\
    \n\x08block_id\"e\n\x04Code\x12\x18\n\x07address\x18\x01\x20\x01(\x0cR\
    \x07address\x12\x1d\n\x03tag\x18\x02\x20\x01(\x0e2\t.BlockTagH\0R\x03tag\
    \x12\x18\n\x06height\x18\x03\x20\x01(\x04H\0R\x06heightB\n\n\x08block_id\
    *$\n\x08BlockTag\x12\n\n\x06Latest\x10\0\x12\x0c\n\x08Earliest\x10\x01J\
    \xa4\x19\n\x06\x12\x04\0\0Z\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\
    \x02\x03\0\x12\x03\x02\x07\x19\n\n\n\x02\x04\0\x12\x04\x04\0\x07\x01\n\n\
    \n\x03\x04\0\x01\x12\x03\x04\x08\x19\n\x0b\n\x04\x04\0\x02\0\x12\x03\x05\
    \x04\x13\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x05\x04\x04\x1b\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03\x05\x04\t\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x05\n\x0e\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x05\x11\x12\n\x0b\n\x04\
    \x04\0\x02\x01\x12\x03\x06\x04\x19\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\
    \x06\x04\x05\x13\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x06\x04\x08\n\x0c\
    \n\x05\x04\0\x02\x01\x01\x12\x03\x06\t\x14\n\x0c\n\x05\x04\0\x02\x01\x03\
    \x12\x03\x06\x17\x18\n\n\n\x02\x04\x01\x12\x04\t\0\x0c\x01\n\n\n\x03\x04\
    \x01\x01\x12\x03\t\x08\x1b\n\x0b\n\x04\x04\x01\x02\0\x12\x03\n\x04\x16\n\
    \r\n\x05\x04\x01\x02\0\x04\x12\x04\n\x04\t\x1d\n\x0c\n\x05\x04\x01\x02\0\
    \x05\x12\x03\n\x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\n\x0b\x11\n\
    \x0c\n\x05\x04\x01\x02\0\x03\x12\x03\n\x14\x15\n\x0b\n\x04\x04\x01\x02\
    \x01\x12\x03\x0b\x04\x19\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x0b\x04\n\
    \x16\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x0b\x04\x08\n\x0c\n\x05\x04\
    \x01\x02\x01\x01\x12\x03\x0b\t\x14\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\
    \x03\x0b\x17\x18\n\n\n\x02\x05\0\x12\x04\x0e\0\x11\x01\n\n\n\x03\x05\0\
    \x01\x12\x03\x0e\x05\r\n\x0b\n\x04\x05\0\x02\0\x12\x03\x0f\x04\x0f\n\x0c\
    \n\x05\x05\0\x02\0\x01\x12\x03\x0f\x04\n\n\x0c\n\x05\x05\0\x02\0\x02\x12\
    \x03\x0f\r\x0e\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x10\x04\x11\n\x0c\n\x05\
    \x05\0\x02\x01\x01\x12\x03\x10\x04\x0c\n\x0c\n\x05\x05\0\x02\x01\x02\x12\
    \x03\x10\x0f\x10\n\n\n\x02\x04\x02\x12\x04\x13\0\x1b\x01\n\n\n\x03\x04\
    \x02\x01\x12\x03\x13\x08\x0c\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x14\x04\
    \x13\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x14\x04\x13\x0e\n\x0c\n\x05\x04\
    \x02\x02\0\x05\x12\x03\x14\x04\t\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\
    \x14\n\x0e\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x14\x11\x12\n\x0b\n\x04\
    \x04\x02\x02\x01\x12\x03\x15\x04\x11\n\r\n\x05\x04\x02\x02\x01\x04\x12\
    \x04\x15\x04\x14\x13\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x15\x04\t\n\
    \x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x15\n\x0c\n\x0c\n\x05\x04\x02\x02\
    \x01\x03\x12\x03\x15\x0f\x10\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\x16\x04\
    \x13\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04\x16\x04\x15\x11\n\x0c\n\x05\
    \x04\x02\x02\x02\x05\x12\x03\x16\x04\t\n\x0c\n\x05\x04\x02\x02\x02\x01\
    \x12\x03\x16\n\x0e\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x16\x11\x12\n\
    \x0c\n\x04\x04\x02\x08\0\x12\x04\x17\x04\x1a\x05\n\x0c\n\x05\x04\x02\x08\
    \0\x01\x12\x03\x17\n\x12\n\x0b\n\x04\x04\x02\x02\x03\x12\x03\x18\x08\x19\
    \n\x0c\n\x05\x04\x02\x02\x03\x06\x12\x03\x18\x08\x10\n\x0c\n\x05\x04\x02\
    \x02\x03\x01\x12\x03\x18\x11\x14\n\x0c\n\x05\x04\x02\x02\x03\x03\x12\x03\
    \x18\x17\x18\n\x0b\n\x04\x04\x02\x02\x04\x12\x03\x19\x08\x1a\n\x0c\n\x05\
    \x04\x02\x02\x04\x05\x12\x03\x19\x08\x0e\n\x0c\n\x05\x04\x02\x02\x04\x01\
    \x12\x03\x19\x0f\x15\n\x0c\n\x05\x04\x02\x02\x04\x03\x12\x03\x19\x18\x19\
    \n\n\n\x02\x04\x03\x12\x04\x1d\0-\x01\n\n\n\x03\x04\x03\x01\x12\x03\x1d\
    \x08\x0f\n\x0b\n\x04\x04\x03\x02\0\x12\x03\x1e\x04\x19\n\r\n\x05\x04\x03\
    \x02\0\x04\x12\x04\x1e\x04\x1d\x11\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03\
    \x1e\x04\t\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x1e\n\x14\n\x0c\n\x05\
    \x04\x03\x02\0\x03\x12\x03\x1e\x17\x18\n\x0c\n\x04\x04\x03\x08\0\x12\x04\
    \x1f\x04,\x05\n\x0c\n\x05\x04\x03\x08\0\x01\x12\x03\x1f\n\r\n\x0b\n\x04\
    \x04\x03\x02\x01\x12\x03\x20\x08\x1e\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\
    \x03\x20\x08\x0c\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03\x20\r\x19\n\x0c\
    \n\x05\x04\x03\x02\x01\x03\x12\x03\x20\x1c\x1d\n\x0b\n\x04\x04\x03\x02\
    \x02\x12\x03!\x08,\n\x0c\n\x05\x04\x03\x02\x02\x06\x12\x03!\x08\x19\n\
    \x0c\n\x05\x04\x03\x02\x02\x01\x12\x03!\x1a'\n\x0c\n\x05\x04\x03\x02\x02\
    \x03\x12\x03!*+\n\x0b\n\x04\x04\x03\x02\x03\x12\x03\"\x080\n\x0c\n\x05\
    \x04\x03\x02\x03\x06\x12\x03\"\x08\x1b\n\x0c\n\x05\x04\x03\x02\x03\x01\
    \x12\x03\"\x1c+\n\x0c\n\x05\x04\x03\x02\x03\x03\x12\x03\"./\n\x0b\n\x04\
    \x04\x03\x02\x04\x12\x03#\x08\x1e\n\x0c\n\x05\x04\x03\x02\x04\x05\x12\
    \x03#\x08\r\n\x0c\n\x05\x04\x03\x02\x04\x01\x12\x03#\x0e\x19\n\x0c\n\x05\
    \x04\x03\x02\x04\x03\x12\x03#\x1c\x1d\n\x0b\n\x04\x04\x03\x02\x05\x12\
    \x03$\x08\x1a\n\x0c\n\x05\x04\x03\x02\x05\x05\x12\x03$\x08\x0e\n\x0c\n\
    \x05\x04\x03\x02\x05\x01\x12\x03$\x0f\x15\n\x0c\n\x05\x04\x03\x02\x05\
    \x03\x12\x03$\x18\x19\n\x0b\n\x04\x04\x03\x02\x06\x12\x03%\x08\x1b\n\x0c\
    \n\x05\x04\x03\x02\x06\x05\x12\x03%\x08\x0c\n\x0c\n\x05\x04\x03\x02\x06\
    \x01\x12\x03%\r\x16\n\x0c\n\x05\x04\x03\x02\x06\x03\x12\x03%\x19\x1a\n\
    \x0b\n\x04\x04\x03\x02\x07\x12\x03&\x08\x16\n\x0c\n\x05\x04\x03\x02\x07\
    \x06\x12\x03&\x08\x0c\n\x0c\n\x05\x04\x03\x02\x07\x01\x12\x03&\r\x11\n\
    \x0c\n\x05\x04\x03\x02\x07\x03\x12\x03&\x14\x15\n\x0b\n\x04\x04\x03\x02\
    \x08\x12\x03'\x08\x1a\n\x0c\n\x05\x04\x03\x02\x08\x05\x12\x03'\x08\x0e\n\
    \x0c\n\x05\x04\x03\x02\x08\x01\x12\x03'\x0f\x15\n\x0c\n\x05\x04\x03\x02\
    \x08\x03\x12\x03'\x18\x19\n\x0b\n\x04\x04\x03\x02\t\x12\x03(\x08'\n\x0c\
    \n\x05\x04\x03\x02\t\x05\x12\x03(\x08\r\n\x0c\n\x05\x04\x03\x02\t\x01\
    \x12\x03(\x0e!\n\x0c\n\x05\x04\x03\x02\t\x03\x12\x03($&\n\x0b\n\x04\x04\
    \x03\x02\n\x12\x03)\x080\n\x0c\n\x05\x04\x03\x02\n\x06\x12\x03)\x08\x18\
    \n\x0c\n\x05\x04\x03\x02\n\x01\x12\x03)\x19*\n\x0c\n\x05\x04\x03\x02\n\
    \x03\x12\x03)-/\n\x0b\n\x04\x04\x03\x02\x0b\x12\x03*\x08\x17\n\x0c\n\x05\
    \x04\x03\x02\x0b\x06\x12\x03*\x08\x0c\n\x0c\n\x05\x04\x03\x02\x0b\x01\
    \x12\x03*\r\x11\n\x0c\n\x05\x04\x03\x02\x0b\x03\x12\x03*\x14\x16\n\n\n\
    \x02\x04\x04\x12\x04/\03\x01\n\n\n\x03\x04\x04\x01\x12\x03/\x08\x10\n\
    \x0b\n\x04\x04\x04\x02\0\x12\x030\x04\x14\n\r\n\x05\x04\x04\x02\0\x04\
    \x12\x040\x04/\x12\n\x0c\n\x05\x04\x04\x02\0\x05\x12\x030\x04\t\n\x0c\n\
    \x05\x04\x04\x02\0\x01\x12\x030\n\x0f\n\x0c\n\x05\x04\x04\x02\0\x03\x12\
    \x030\x12\x13\n\x0b\n\x04\x04\x04\x02\x01\x12\x031\x04\x19\n\r\n\x05\x04\
    \x04\x02\x01\x04\x12\x041\x040\x14\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\
    \x031\x04\x08\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x031\t\x14\n\x0c\n\x05\
    \x04\x04\x02\x01\x03\x12\x031\x17\x18\n\x0b\n\x04\x04\x04\x02\x02\x12\
    \x032\x04\x13\n\r\n\x05\x04\x04\x02\x02\x04\x12\x042\x041\x19\n\x0c\n\
    \x05\x04\x04\x02\x02\x05\x12\x032\x04\t\n\x0c\n\x05\x04\x04\x02\x02\x01\
    \x12\x032\n\x0e\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\x032\x11\x12\n\n\n\
    \x02\x04\x05\x12\x045\0:\x01\n\n\n\x03\x04\x05\x01\x12\x035\x08\x17\n\
    \x0b\n\x04\x04\x05\x02\0\x12\x036\x04\x20\n\r\n\x05\x04\x05\x02\0\x04\
    \x12\x046\x045\x19\n\x0c\n\x05\x04\x05\x02\0\x06\x12\x036\x04\x0f\n\x0c\
    \n\x05\x04\x05\x02\0\x01\x12\x036\x10\x1b\n\x0c\n\x05\x04\x05\x02\0\x03\
    \x12\x036\x1e\x1f\n\x0b\n\x04\x04\x05\x02\x01\x12\x037\x04\x1c\n\r\n\x05\
    \x04\x05\x02\x01\x04\x12\x047\x046\x20\n\x0c\n\x05\x04\x05\x02\x01\x05\
    \x12\x037\x04\n\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x037\x0b\x17\n\x0c\n\
    \x05\x04\x05\x02\x01\x03\x12\x037\x1a\x1b\n\x0b\n\x04\x04\x05\x02\x02\
    \x12\x038\x04\x19\n\r\n\x05\x04\x05\x02\x02\x04\x12\x048\x047\x1c\n\x0c\
    \n\x05\x04\x05\x02\x02\x05\x12\x038\x04\t\n\x0c\n\x05\x04\x05\x02\x02\
    \x01\x12\x038\n\x14\n\x0c\n\x05\x04\x05\x02\x02\x03\x12\x038\x17\x18\n\
    \x0b\n\x04\x04\x05\x02\x03\x12\x039\x04\x15\n\r\n\x05\x04\x05\x02\x03\
    \x04\x12\x049\x048\x19\n\x0c\n\x05\x04\x05\x02\x03\x05\x12\x039\x04\n\n\
    \x0c\n\x05\x04\x05\x02\x03\x01\x12\x039\x0b\x10\n\x0c\n\x05\x04\x05\x02\
    \x03\x03\x12\x039\x13\x14\n\n\n\x02\x04\x06\x12\x04<\0J\x01\n\n\n\x03\
    \x04\x06\x01\x12\x03<\x08\x10\n\x0b\n\x04\x04\x06\x02\0\x12\x03=\x04\x19\
    \n\r\n\x05\x04\x06\x02\0\x04\x12\x04=\x04<\x12\n\x0c\n\x05\x04\x06\x02\0\
    \x05\x12\x03=\x04\t\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03=\n\x14\n\x0c\n\
    \x05\x04\x06\x02\0\x03\x12\x03=\x17\x18\n\x0c\n\x04\x04\x06\x08\0\x12\
    \x04>\x04I\x05\n\x0c\n\x05\x04\x06\x08\0\x01\x12\x03>\n\x10\n\x0b\n\x04\
    \x04\x06\x02\x01\x12\x03?\x08\x20\n\x0c\n\x05\x04\x06\x02\x01\x05\x12\
    \x03?\x08\x0e\n\x0c\n\x05\x04\x06\x02\x01\x01\x12\x03?\x0f\x1b\n\x0c\n\
    \x05\x04\x06\x02\x01\x03\x12\x03?\x1e\x1f\n\x0b\n\x04\x04\x06\x02\x02\
    \x12\x03@\x08\x1b\n\x0c\n\x05\x04\x06\x02\x02\x06\x12\x03@\x08\x10\n\x0c\
    \n\x05\x04\x06\x02\x02\x01\x12\x03@\x11\x16\n\x0c\n\x05\x04\x06\x02\x02\
    \x03\x12\x03@\x19\x1a\n\x0b\n\x04\x04\x06\x02\x03\x12\x03A\x08\x1f\n\x0c\
    \n\x05\x04\x06\x02\x03\x06\x12\x03A\x08\x17\n\x0c\n\x05\x04\x06\x02\x03\
    \x01\x12\x03A\x18\x1a\n\x0c\n\x05\x04\x06\x02\x03\x03\x12\x03A\x1d\x1e\n\
    \x0b\n\x04\x04\x06\x02\x04\x12\x03B\x08\x16\n\x0c\n\x05\x04\x06\x02\x04\
    \x05\x12\x03B\x08\x0c\n\x0c\n\x05\x04\x06\x02\x04\x01\x12\x03B\r\x11\n\
    \x0c\n\x05\x04\x06\x02\x04\x03\x12\x03B\x14\x15\n\x0b\n\x04\x04\x06\x02\
    \x05\x12\x03C\x08\x1d\n\x0c\n\x05\x04\x06\x02\x05\x05\x12\x03C\x08\x0e\n\
    \x0c\n\x05\x04\x06\x02\x05\x01\x12\x03C\x0f\x18\n\x0c\n\x05\x04\x06\x02\
    \x05\x03\x12\x03C\x1b\x1c\n\x0b\n\x04\x04\x06\x02\x06\x12\x03D\x08\x1e\n\
    \x0c\n\x05\x04\x06\x02\x06\x05\x12\x03D\x08\r\n\x0c\n\x05\x04\x06\x02\
    \x06\x01\x12\x03D\x0e\x19\n\x0c\n\x05\x04\x06\x02\x06\x03\x12\x03D\x1c\
    \x1d\n\x0b\n\x04\x04\x06\x02\x07\x12\x03E\x08\x18\n\x0c\n\x05\x04\x06\
    \x02\x07\x05\x12\x03E\x08\x0e\n\x0c\n\x05\x04\x06\x02\x07\x01\x12\x03E\
    \x0f\x13\n\x0c\n\x05\x04\x06\x02\x07\x03\x12\x03E\x16\x17\n\x0b\n\x04\
    \x04\x06\x02\x08\x12\x03F\x08\x1b\n\x0c\n\x05\x04\x06\x02\x08\x05\x12\
    \x03F\x08\x0e\n\x0c\n\x05\x04\x06\x02\x08\x01\x12\x03F\x0f\x16\n\x0c\n\
    \x05\x04\x06\x02\x08\x03\x12\x03F\x19\x1a\n\x0b\n\x04\x04\x06\x02\t\x12\
    \x03G\x08&\n\x0c\n\x05\x04\x06\x02\t\x05\x12\x03G\x08\x0e\n\x0c\n\x05\
    \x04\x06\x02\t\x01\x12\x03G\x0f\x20\n\x0c\n\x05\x04\x06\x02\t\x03\x12\
    \x03G#%\n\x0b\n\x04\x04\x06\x02\n\x12\x03H\x08\x18\n\x0c\n\x05\x04\x06\
    \x02\n\x05\x12\x03H\x08\r\n\x0c\n\x05\x04\x06\x02\n\x01\x12\x03H\x0e\x12\
    \n\x0c\n\x05\x04\x06\x02\n\x03\x12\x03H\x15\x17\n\n\n\x02\x04\x07\x12\
    \x04L\0R\x01\n\n\n\x03\x04\x07\x01\x12\x03L\x08\x18\n\x0b\n\x04\x04\x07\
    \x02\0\x12\x03M\x04\x16\n\r\n\x05\x04\x07\x02\0\x04\x12\x04M\x04L\x1a\n\
    \x0c\n\x05\x04\x07\x02\0\x05\x12\x03M\x04\t\n\x0c\n\x05\x04\x07\x02\0\
    \x01\x12\x03M\n\x11\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x03M\x14\x15\n\x0c\
    \n\x04\x04\x07\x08\0\x12\x04N\x04Q\x05\n\x0c\n\x05\x04\x07\x08\0\x01\x12\
    \x03N\n\x12\n\x0b\n\x04\x04\x07\x02\x01\x12\x03O\x08\x19\n\x0c\n\x05\x04\
    \x07\x02\x01\x06\x12\x03O\x08\x10\n\x0c\n\x05\x04\x07\x02\x01\x01\x12\
    \x03O\x11\x14\n\x0c\n\x05\x04\x07\x02\x01\x03\x12\x03O\x17\x18\n\x0b\n\
    \x04\x04\x07\x02\x02\x12\x03P\x08\x1a\n\x0c\n\x05\x04\x07\x02\x02\x05\
    \x12\x03P\x08\x0e\n\x0c\n\x05\x04\x07\x02\x02\x01\x12\x03P\x0f\x15\n\x0c\
    \n\x05\x04\x07\x02\x02\x03\x12\x03P\x18\x19\n\n\n\x02\x04\x08\x12\x04T\0\
    Z\x01\n\n\n\x03\x04\x08\x01\x12\x03T\x08\x0c\n\x0b\n\x04\x04\x08\x02\0\
    \x12\x03U\x04\x16\n\r\n\x05\x04\x08\x02\0\x04\x12\x04U\x04T\x0e\n\x0c\n\
    \x05\x04\x08\x02\0\x05\x12\x03U\x04\t\n\x0c\n\x05\x04\x08\x02\0\x01\x12\
    \x03U\n\x11\n\x0c\n\x05\x04\x08\x02\0\x03\x12\x03U\x14\x15\n\x0c\n\x04\
    \x04\x08\x08\0\x12\x04V\x04Y\x05\n\x0c\n\x05\x04\x08\x08\0\x01\x12\x03V\
    \n\x12\n\x0b\n\x04\x04\x08\x02\x01\x12\x03W\x08\x19\n\x0c\n\x05\x04\x08\
    \x02\x01\x06\x12\x03W\x08\x10\n\x0c\n\x05\x04\x08\x02\x01\x01\x12\x03W\
    \x11\x14\n\x0c\n\x05\x04\x08\x02\x01\x03\x12\x03W\x17\x18\n\x0b\n\x04\
    \x04\x08\x02\x02\x12\x03X\x08\x1a\n\x0c\n\x05\x04\x08\x02\x02\x05\x12\
    \x03X\x08\x0e\n\x0c\n\x05\x04\x08\x02\x02\x01\x12\x03X\x0f\x15\n\x0c\n\
    \x05\x04\x08\x02\x02\x03\x12\x03X\x18\x19b\x06proto3\
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
