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
pub struct VoteMessage {
    // message fields
    pub proposal: ::std::vec::Vec<u8>,
    pub signature: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VoteMessage {}

impl VoteMessage {
    pub fn new() -> VoteMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VoteMessage {
        static mut instance: ::protobuf::lazy::Lazy<VoteMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VoteMessage,
        };
        unsafe {
            instance.get(VoteMessage::new)
        }
    }

    // bytes proposal = 1;

    pub fn clear_proposal(&mut self) {
        self.proposal.clear();
    }

    // Param is passed by value, moved
    pub fn set_proposal(&mut self, v: ::std::vec::Vec<u8>) {
        self.proposal = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_proposal(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.proposal
    }

    // Take field
    pub fn take_proposal(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.proposal, ::std::vec::Vec::new())
    }

    pub fn get_proposal(&self) -> &[u8] {
        &self.proposal
    }

    fn get_proposal_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.proposal
    }

    fn mut_proposal_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.proposal
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
}

impl ::protobuf::Message for VoteMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.proposal)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.signature)?;
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
        if !self.proposal.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.proposal);
        }
        if !self.signature.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.signature);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.proposal.is_empty() {
            os.write_bytes(1, &self.proposal)?;
        }
        if !self.signature.is_empty() {
            os.write_bytes(2, &self.signature)?;
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

impl ::protobuf::MessageStatic for VoteMessage {
    fn new() -> VoteMessage {
        VoteMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<VoteMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "proposal",
                    VoteMessage::get_proposal_for_reflect,
                    VoteMessage::mut_proposal_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    VoteMessage::get_signature_for_reflect,
                    VoteMessage::mut_signature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VoteMessage>(
                    "VoteMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VoteMessage {
    fn clear(&mut self) {
        self.clear_proposal();
        self.clear_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VoteMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VoteMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VoteSet {
    // message fields
    pub votes_by_sender: ::std::collections::HashMap<::std::string::String, VoteMessage>,
    pub votes_by_proposal: ::std::collections::HashMap<::std::string::String, u64>,
    pub count: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VoteSet {}

impl VoteSet {
    pub fn new() -> VoteSet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VoteSet {
        static mut instance: ::protobuf::lazy::Lazy<VoteSet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VoteSet,
        };
        unsafe {
            instance.get(VoteSet::new)
        }
    }

    // repeated .VoteSet.VotesBySenderEntry votes_by_sender = 1;

    pub fn clear_votes_by_sender(&mut self) {
        self.votes_by_sender.clear();
    }

    // Param is passed by value, moved
    pub fn set_votes_by_sender(&mut self, v: ::std::collections::HashMap<::std::string::String, VoteMessage>) {
        self.votes_by_sender = v;
    }

    // Mutable pointer to the field.
    pub fn mut_votes_by_sender(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, VoteMessage> {
        &mut self.votes_by_sender
    }

    // Take field
    pub fn take_votes_by_sender(&mut self) -> ::std::collections::HashMap<::std::string::String, VoteMessage> {
        ::std::mem::replace(&mut self.votes_by_sender, ::std::collections::HashMap::new())
    }

    pub fn get_votes_by_sender(&self) -> &::std::collections::HashMap<::std::string::String, VoteMessage> {
        &self.votes_by_sender
    }

    fn get_votes_by_sender_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, VoteMessage> {
        &self.votes_by_sender
    }

    fn mut_votes_by_sender_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, VoteMessage> {
        &mut self.votes_by_sender
    }

    // repeated .VoteSet.VotesByProposalEntry votes_by_proposal = 2;

    pub fn clear_votes_by_proposal(&mut self) {
        self.votes_by_proposal.clear();
    }

    // Param is passed by value, moved
    pub fn set_votes_by_proposal(&mut self, v: ::std::collections::HashMap<::std::string::String, u64>) {
        self.votes_by_proposal = v;
    }

    // Mutable pointer to the field.
    pub fn mut_votes_by_proposal(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, u64> {
        &mut self.votes_by_proposal
    }

    // Take field
    pub fn take_votes_by_proposal(&mut self) -> ::std::collections::HashMap<::std::string::String, u64> {
        ::std::mem::replace(&mut self.votes_by_proposal, ::std::collections::HashMap::new())
    }

    pub fn get_votes_by_proposal(&self) -> &::std::collections::HashMap<::std::string::String, u64> {
        &self.votes_by_proposal
    }

    fn get_votes_by_proposal_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, u64> {
        &self.votes_by_proposal
    }

    fn mut_votes_by_proposal_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, u64> {
        &mut self.votes_by_proposal
    }

    // uint64 count = 3;

    pub fn clear_count(&mut self) {
        self.count = 0;
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u64) {
        self.count = v;
    }

    pub fn get_count(&self) -> u64 {
        self.count
    }

    fn get_count_for_reflect(&self) -> &u64 {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut u64 {
        &mut self.count
    }
}

impl ::protobuf::Message for VoteSet {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<VoteMessage>>(wire_type, is, &mut self.votes_by_sender)?;
                },
                2 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeUint64>(wire_type, is, &mut self.votes_by_proposal)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.count = tmp;
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
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<VoteMessage>>(1, &self.votes_by_sender);
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeUint64>(2, &self.votes_by_proposal);
        if self.count != 0 {
            my_size += ::protobuf::rt::value_size(3, self.count, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<VoteMessage>>(1, &self.votes_by_sender, os)?;
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeUint64>(2, &self.votes_by_proposal, os)?;
        if self.count != 0 {
            os.write_uint64(3, self.count)?;
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

impl ::protobuf::MessageStatic for VoteSet {
    fn new() -> VoteSet {
        VoteSet::new()
    }

    fn descriptor_static(_: ::std::option::Option<VoteSet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<VoteMessage>>(
                    "votes_by_sender",
                    VoteSet::get_votes_by_sender_for_reflect,
                    VoteSet::mut_votes_by_sender_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeUint64>(
                    "votes_by_proposal",
                    VoteSet::get_votes_by_proposal_for_reflect,
                    VoteSet::mut_votes_by_proposal_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "count",
                    VoteSet::get_count_for_reflect,
                    VoteSet::mut_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VoteSet>(
                    "VoteSet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VoteSet {
    fn clear(&mut self) {
        self.clear_votes_by_sender();
        self.clear_votes_by_proposal();
        self.clear_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VoteSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VoteSet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Proposal {
    // message fields
    pub block: ::protobuf::SingularPtrField<super::blockchain::Block>,
    pub lock_round: u64,
    pub lock_votes: ::protobuf::SingularPtrField<VoteSet>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Proposal {}

impl Proposal {
    pub fn new() -> Proposal {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Proposal {
        static mut instance: ::protobuf::lazy::Lazy<Proposal> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Proposal,
        };
        unsafe {
            instance.get(Proposal::new)
        }
    }

    // .Block block = 1;

    pub fn clear_block(&mut self) {
        self.block.clear();
    }

    pub fn has_block(&self) -> bool {
        self.block.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block(&mut self, v: super::blockchain::Block) {
        self.block = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block(&mut self) -> &mut super::blockchain::Block {
        if self.block.is_none() {
            self.block.set_default();
        }
        self.block.as_mut().unwrap()
    }

    // Take field
    pub fn take_block(&mut self) -> super::blockchain::Block {
        self.block.take().unwrap_or_else(|| super::blockchain::Block::new())
    }

    pub fn get_block(&self) -> &super::blockchain::Block {
        self.block.as_ref().unwrap_or_else(|| super::blockchain::Block::default_instance())
    }

    fn get_block_for_reflect(&self) -> &::protobuf::SingularPtrField<super::blockchain::Block> {
        &self.block
    }

    fn mut_block_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::blockchain::Block> {
        &mut self.block
    }

    // uint64 lock_round = 2;

    pub fn clear_lock_round(&mut self) {
        self.lock_round = 0;
    }

    // Param is passed by value, moved
    pub fn set_lock_round(&mut self, v: u64) {
        self.lock_round = v;
    }

    pub fn get_lock_round(&self) -> u64 {
        self.lock_round
    }

    fn get_lock_round_for_reflect(&self) -> &u64 {
        &self.lock_round
    }

    fn mut_lock_round_for_reflect(&mut self) -> &mut u64 {
        &mut self.lock_round
    }

    // .VoteSet lock_votes = 3;

    pub fn clear_lock_votes(&mut self) {
        self.lock_votes.clear();
    }

    pub fn has_lock_votes(&self) -> bool {
        self.lock_votes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lock_votes(&mut self, v: VoteSet) {
        self.lock_votes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lock_votes(&mut self) -> &mut VoteSet {
        if self.lock_votes.is_none() {
            self.lock_votes.set_default();
        }
        self.lock_votes.as_mut().unwrap()
    }

    // Take field
    pub fn take_lock_votes(&mut self) -> VoteSet {
        self.lock_votes.take().unwrap_or_else(|| VoteSet::new())
    }

    pub fn get_lock_votes(&self) -> &VoteSet {
        self.lock_votes.as_ref().unwrap_or_else(|| VoteSet::default_instance())
    }

    fn get_lock_votes_for_reflect(&self) -> &::protobuf::SingularPtrField<VoteSet> {
        &self.lock_votes
    }

    fn mut_lock_votes_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<VoteSet> {
        &mut self.lock_votes
    }
}

impl ::protobuf::Message for Proposal {
    fn is_initialized(&self) -> bool {
        for v in &self.block {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.lock_votes {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.block)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.lock_round = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lock_votes)?;
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
        if let Some(ref v) = self.block.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.lock_round != 0 {
            my_size += ::protobuf::rt::value_size(2, self.lock_round, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.lock_votes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.block.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.lock_round != 0 {
            os.write_uint64(2, self.lock_round)?;
        }
        if let Some(ref v) = self.lock_votes.as_ref() {
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

impl ::protobuf::MessageStatic for Proposal {
    fn new() -> Proposal {
        Proposal::new()
    }

    fn descriptor_static(_: ::std::option::Option<Proposal>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockchain::Block>>(
                    "block",
                    Proposal::get_block_for_reflect,
                    Proposal::mut_block_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lock_round",
                    Proposal::get_lock_round_for_reflect,
                    Proposal::mut_lock_round_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VoteSet>>(
                    "lock_votes",
                    Proposal::get_lock_votes_for_reflect,
                    Proposal::mut_lock_votes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Proposal>(
                    "Proposal",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Proposal {
    fn clear(&mut self) {
        self.clear_block();
        self.clear_lock_round();
        self.clear_lock_votes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Proposal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Proposal {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ProposeStep {
    // message fields
    pub height: u64,
    pub round: u64,
    pub proposal: ::protobuf::SingularPtrField<Proposal>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ProposeStep {}

impl ProposeStep {
    pub fn new() -> ProposeStep {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ProposeStep {
        static mut instance: ::protobuf::lazy::Lazy<ProposeStep> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ProposeStep,
        };
        unsafe {
            instance.get(ProposeStep::new)
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

    // uint64 round = 2;

    pub fn clear_round(&mut self) {
        self.round = 0;
    }

    // Param is passed by value, moved
    pub fn set_round(&mut self, v: u64) {
        self.round = v;
    }

    pub fn get_round(&self) -> u64 {
        self.round
    }

    fn get_round_for_reflect(&self) -> &u64 {
        &self.round
    }

    fn mut_round_for_reflect(&mut self) -> &mut u64 {
        &mut self.round
    }

    // .Proposal proposal = 3;

    pub fn clear_proposal(&mut self) {
        self.proposal.clear();
    }

    pub fn has_proposal(&self) -> bool {
        self.proposal.is_some()
    }

    // Param is passed by value, moved
    pub fn set_proposal(&mut self, v: Proposal) {
        self.proposal = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_proposal(&mut self) -> &mut Proposal {
        if self.proposal.is_none() {
            self.proposal.set_default();
        }
        self.proposal.as_mut().unwrap()
    }

    // Take field
    pub fn take_proposal(&mut self) -> Proposal {
        self.proposal.take().unwrap_or_else(|| Proposal::new())
    }

    pub fn get_proposal(&self) -> &Proposal {
        self.proposal.as_ref().unwrap_or_else(|| Proposal::default_instance())
    }

    fn get_proposal_for_reflect(&self) -> &::protobuf::SingularPtrField<Proposal> {
        &self.proposal
    }

    fn mut_proposal_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Proposal> {
        &mut self.proposal
    }
}

impl ::protobuf::Message for ProposeStep {
    fn is_initialized(&self) -> bool {
        for v in &self.proposal {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.round = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.proposal)?;
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
        if self.round != 0 {
            my_size += ::protobuf::rt::value_size(2, self.round, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.proposal.as_ref() {
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
        if self.round != 0 {
            os.write_uint64(2, self.round)?;
        }
        if let Some(ref v) = self.proposal.as_ref() {
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

impl ::protobuf::MessageStatic for ProposeStep {
    fn new() -> ProposeStep {
        ProposeStep::new()
    }

    fn descriptor_static(_: ::std::option::Option<ProposeStep>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    ProposeStep::get_height_for_reflect,
                    ProposeStep::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "round",
                    ProposeStep::get_round_for_reflect,
                    ProposeStep::mut_round_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Proposal>>(
                    "proposal",
                    ProposeStep::get_proposal_for_reflect,
                    ProposeStep::mut_proposal_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ProposeStep>(
                    "ProposeStep",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ProposeStep {
    fn clear(&mut self) {
        self.clear_height();
        self.clear_round();
        self.clear_proposal();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProposeStep {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProposeStep {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SignedProposeStep {
    // message fields
    pub propose_step: ::protobuf::SingularPtrField<ProposeStep>,
    pub signature: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SignedProposeStep {}

impl SignedProposeStep {
    pub fn new() -> SignedProposeStep {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SignedProposeStep {
        static mut instance: ::protobuf::lazy::Lazy<SignedProposeStep> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SignedProposeStep,
        };
        unsafe {
            instance.get(SignedProposeStep::new)
        }
    }

    // .ProposeStep propose_step = 1;

    pub fn clear_propose_step(&mut self) {
        self.propose_step.clear();
    }

    pub fn has_propose_step(&self) -> bool {
        self.propose_step.is_some()
    }

    // Param is passed by value, moved
    pub fn set_propose_step(&mut self, v: ProposeStep) {
        self.propose_step = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_propose_step(&mut self) -> &mut ProposeStep {
        if self.propose_step.is_none() {
            self.propose_step.set_default();
        }
        self.propose_step.as_mut().unwrap()
    }

    // Take field
    pub fn take_propose_step(&mut self) -> ProposeStep {
        self.propose_step.take().unwrap_or_else(|| ProposeStep::new())
    }

    pub fn get_propose_step(&self) -> &ProposeStep {
        self.propose_step.as_ref().unwrap_or_else(|| ProposeStep::default_instance())
    }

    fn get_propose_step_for_reflect(&self) -> &::protobuf::SingularPtrField<ProposeStep> {
        &self.propose_step
    }

    fn mut_propose_step_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ProposeStep> {
        &mut self.propose_step
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
}

impl ::protobuf::Message for SignedProposeStep {
    fn is_initialized(&self) -> bool {
        for v in &self.propose_step {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.propose_step)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.signature)?;
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
        if let Some(ref v) = self.propose_step.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.signature.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.signature);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.propose_step.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.signature.is_empty() {
            os.write_bytes(2, &self.signature)?;
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

impl ::protobuf::MessageStatic for SignedProposeStep {
    fn new() -> SignedProposeStep {
        SignedProposeStep::new()
    }

    fn descriptor_static(_: ::std::option::Option<SignedProposeStep>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ProposeStep>>(
                    "propose_step",
                    SignedProposeStep::get_propose_step_for_reflect,
                    SignedProposeStep::mut_propose_step_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    SignedProposeStep::get_signature_for_reflect,
                    SignedProposeStep::mut_signature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SignedProposeStep>(
                    "SignedProposeStep",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SignedProposeStep {
    fn clear(&mut self) {
        self.clear_propose_step();
        self.clear_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SignedProposeStep {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SignedProposeStep {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fconsensus.proto\x1a\x10blockchain.proto\"G\n\x0bVoteMessage\x12\
    \x1a\n\x08proposal\x18\x01\x20\x01(\x0cR\x08proposal\x12\x1c\n\tsignatur\
    e\x18\x02\x20\x01(\x0cR\tsignature\"\xc3\x02\n\x07VoteSet\x12C\n\x0fvote\
    s_by_sender\x18\x01\x20\x03(\x0b2\x1b.VoteSet.VotesBySenderEntryR\rvotes\
    BySender\x12I\n\x11votes_by_proposal\x18\x02\x20\x03(\x0b2\x1d.VoteSet.V\
    otesByProposalEntryR\x0fvotesByProposal\x12\x14\n\x05count\x18\x03\x20\
    \x01(\x04R\x05count\x1aN\n\x12VotesBySenderEntry\x12\x10\n\x03key\x18\
    \x01\x20\x01(\tR\x03key\x12\"\n\x05value\x18\x02\x20\x01(\x0b2\x0c.VoteM\
    essageR\x05value:\x028\x01\x1aB\n\x14VotesByProposalEntry\x12\x10\n\x03k\
    ey\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\x04R\
    \x05value:\x028\x01\"p\n\x08Proposal\x12\x1c\n\x05block\x18\x01\x20\x01(\
    \x0b2\x06.BlockR\x05block\x12\x1d\n\nlock_round\x18\x02\x20\x01(\x04R\tl\
    ockRound\x12'\n\nlock_votes\x18\x03\x20\x01(\x0b2\x08.VoteSetR\tlockVote\
    s\"b\n\x0bProposeStep\x12\x16\n\x06height\x18\x01\x20\x01(\x04R\x06heigh\
    t\x12\x14\n\x05round\x18\x02\x20\x01(\x04R\x05round\x12%\n\x08proposal\
    \x18\x03\x20\x01(\x0b2\t.ProposalR\x08proposal\"b\n\x11SignedProposeStep\
    \x12/\n\x0cpropose_step\x18\x01\x20\x01(\x0b2\x0c.ProposeStepR\x0bpropos\
    eStep\x12\x1c\n\tsignature\x18\x02\x20\x01(\x0cR\tsignatureJ\xf5\x08\n\
    \x06\x12\x04\0\0\x20\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\
    \x12\x03\x02\x07\x19\n\n\n\x02\x04\0\x12\x04\x04\0\x07\x01\n\n\n\x03\x04\
    \0\x01\x12\x03\x04\x08\x13\n\x0b\n\x04\x04\0\x02\0\x12\x03\x05\x04\x17\n\
    \r\n\x05\x04\0\x02\0\x04\x12\x04\x05\x04\x04\x15\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03\x05\x04\t\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x05\n\x12\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x05\x15\x16\n\x0b\n\x04\x04\0\x02\x01\
    \x12\x03\x06\x04\x18\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x06\x04\x05\x17\
    \n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x06\x04\t\n\x0c\n\x05\x04\0\x02\
    \x01\x01\x12\x03\x06\n\x13\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x06\x16\
    \x17\n\n\n\x02\x04\x01\x12\x04\t\0\x0f\x01\n\n\n\x03\x04\x01\x01\x12\x03\
    \t\x08\x0f\n4\n\x04\x04\x01\x02\0\x12\x03\x0b\x041\x1a'\x20map\x20key\
    \x20is\x20H160\x20converted\x20hex\x20string.\n\n\r\n\x05\x04\x01\x02\0\
    \x04\x12\x04\x0b\x04\t\x11\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x0b\x04\
    \x1c\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x0b\x1d,\n\x0c\n\x05\x04\x01\
    \x02\0\x03\x12\x03\x0b/0\n4\n\x04\x04\x01\x02\x01\x12\x03\r\x04.\x1a'\
    \x20map\x20key\x20is\x20H256\x20converted\x20hex\x20string.\n\n\r\n\x05\
    \x04\x01\x02\x01\x04\x12\x04\r\x04\x0b1\n\x0c\n\x05\x04\x01\x02\x01\x06\
    \x12\x03\r\x04\x17\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\r\x18)\n\x0c\
    \n\x05\x04\x01\x02\x01\x03\x12\x03\r,-\n\x0b\n\x04\x04\x01\x02\x02\x12\
    \x03\x0e\x04\x15\n\r\n\x05\x04\x01\x02\x02\x04\x12\x04\x0e\x04\r.\n\x0c\
    \n\x05\x04\x01\x02\x02\x05\x12\x03\x0e\x04\n\n\x0c\n\x05\x04\x01\x02\x02\
    \x01\x12\x03\x0e\x0b\x10\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x0e\x13\
    \x14\n\n\n\x02\x04\x02\x12\x04\x11\0\x15\x01\n\n\n\x03\x04\x02\x01\x12\
    \x03\x11\x08\x10\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x12\x04\x14\n\r\n\x05\
    \x04\x02\x02\0\x04\x12\x04\x12\x04\x11\x12\n\x0c\n\x05\x04\x02\x02\0\x06\
    \x12\x03\x12\x04\t\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x12\n\x0f\n\x0c\
    \n\x05\x04\x02\x02\0\x03\x12\x03\x12\x12\x13\n\x0b\n\x04\x04\x02\x02\x01\
    \x12\x03\x13\x04\x1a\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\x13\x04\x12\
    \x14\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x13\x04\n\n\x0c\n\x05\x04\
    \x02\x02\x01\x01\x12\x03\x13\x0b\x15\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\
    \x03\x13\x18\x19\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\x14\x04\x1b\n\r\n\
    \x05\x04\x02\x02\x02\x04\x12\x04\x14\x04\x13\x1a\n\x0c\n\x05\x04\x02\x02\
    \x02\x06\x12\x03\x14\x04\x0b\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03\x14\
    \x0c\x16\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x14\x19\x1a\n\n\n\x02\
    \x04\x03\x12\x04\x17\0\x1b\x01\n\n\n\x03\x04\x03\x01\x12\x03\x17\x08\x13\
    \n\x0b\n\x04\x04\x03\x02\0\x12\x03\x18\x04\x16\n\r\n\x05\x04\x03\x02\0\
    \x04\x12\x04\x18\x04\x17\x15\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03\x18\
    \x04\n\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x18\x0b\x11\n\x0c\n\x05\x04\
    \x03\x02\0\x03\x12\x03\x18\x14\x15\n\x0b\n\x04\x04\x03\x02\x01\x12\x03\
    \x19\x04\x15\n\r\n\x05\x04\x03\x02\x01\x04\x12\x04\x19\x04\x18\x16\n\x0c\
    \n\x05\x04\x03\x02\x01\x05\x12\x03\x19\x04\n\n\x0c\n\x05\x04\x03\x02\x01\
    \x01\x12\x03\x19\x0b\x10\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03\x19\x13\
    \x14\n\x0b\n\x04\x04\x03\x02\x02\x12\x03\x1a\x04\x1a\n\r\n\x05\x04\x03\
    \x02\x02\x04\x12\x04\x1a\x04\x19\x15\n\x0c\n\x05\x04\x03\x02\x02\x06\x12\
    \x03\x1a\x04\x0c\n\x0c\n\x05\x04\x03\x02\x02\x01\x12\x03\x1a\r\x15\n\x0c\
    \n\x05\x04\x03\x02\x02\x03\x12\x03\x1a\x18\x19\n\n\n\x02\x04\x04\x12\x04\
    \x1d\0\x20\x01\n\n\n\x03\x04\x04\x01\x12\x03\x1d\x08\x19\n\x0b\n\x04\x04\
    \x04\x02\0\x12\x03\x1e\x04!\n\r\n\x05\x04\x04\x02\0\x04\x12\x04\x1e\x04\
    \x1d\x1b\n\x0c\n\x05\x04\x04\x02\0\x06\x12\x03\x1e\x04\x0f\n\x0c\n\x05\
    \x04\x04\x02\0\x01\x12\x03\x1e\x10\x1c\n\x0c\n\x05\x04\x04\x02\0\x03\x12\
    \x03\x1e\x1f\x20\n\x0b\n\x04\x04\x04\x02\x01\x12\x03\x1f\x04\x18\n\r\n\
    \x05\x04\x04\x02\x01\x04\x12\x04\x1f\x04\x1e!\n\x0c\n\x05\x04\x04\x02\
    \x01\x05\x12\x03\x1f\x04\t\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03\x1f\n\
    \x13\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03\x1f\x16\x17b\x06proto3\
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
