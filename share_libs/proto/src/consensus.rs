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
pub struct Proposal {
    // message fields
    pub block: ::protobuf::SingularPtrField<super::blockchain::Block>,
    pub islock: bool,
    pub lock_round: u64,
    pub lock_votes: ::protobuf::RepeatedField<Vote>,
    pub round: u64,
    pub height: u64,
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

    // bool islock = 2;

    pub fn clear_islock(&mut self) {
        self.islock = false;
    }

    // Param is passed by value, moved
    pub fn set_islock(&mut self, v: bool) {
        self.islock = v;
    }

    pub fn get_islock(&self) -> bool {
        self.islock
    }

    fn get_islock_for_reflect(&self) -> &bool {
        &self.islock
    }

    fn mut_islock_for_reflect(&mut self) -> &mut bool {
        &mut self.islock
    }

    // uint64 lock_round = 3;

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

    // repeated .Vote lock_votes = 4;

    pub fn clear_lock_votes(&mut self) {
        self.lock_votes.clear();
    }

    // Param is passed by value, moved
    pub fn set_lock_votes(&mut self, v: ::protobuf::RepeatedField<Vote>) {
        self.lock_votes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_lock_votes(&mut self) -> &mut ::protobuf::RepeatedField<Vote> {
        &mut self.lock_votes
    }

    // Take field
    pub fn take_lock_votes(&mut self) -> ::protobuf::RepeatedField<Vote> {
        ::std::mem::replace(&mut self.lock_votes, ::protobuf::RepeatedField::new())
    }

    pub fn get_lock_votes(&self) -> &[Vote] {
        &self.lock_votes
    }

    fn get_lock_votes_for_reflect(&self) -> &::protobuf::RepeatedField<Vote> {
        &self.lock_votes
    }

    fn mut_lock_votes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Vote> {
        &mut self.lock_votes
    }

    // uint64 round = 5;

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

    // uint64 height = 6;

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
                    let tmp = is.read_bool()?;
                    self.islock = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.lock_round = tmp;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.lock_votes)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.round = tmp;
                },
                6 => {
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
        if let Some(ref v) = self.block.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.islock != false {
            my_size += 2;
        }
        if self.lock_round != 0 {
            my_size += ::protobuf::rt::value_size(3, self.lock_round, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.lock_votes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.round != 0 {
            my_size += ::protobuf::rt::value_size(5, self.round, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(6, self.height, ::protobuf::wire_format::WireTypeVarint);
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
        if self.islock != false {
            os.write_bool(2, self.islock)?;
        }
        if self.lock_round != 0 {
            os.write_uint64(3, self.lock_round)?;
        }
        for v in &self.lock_votes {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.round != 0 {
            os.write_uint64(5, self.round)?;
        }
        if self.height != 0 {
            os.write_uint64(6, self.height)?;
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "islock",
                    Proposal::get_islock_for_reflect,
                    Proposal::mut_islock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lock_round",
                    Proposal::get_lock_round_for_reflect,
                    Proposal::mut_lock_round_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Vote>>(
                    "lock_votes",
                    Proposal::get_lock_votes_for_reflect,
                    Proposal::mut_lock_votes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "round",
                    Proposal::get_round_for_reflect,
                    Proposal::mut_round_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    Proposal::get_height_for_reflect,
                    Proposal::mut_height_for_reflect,
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
        self.clear_islock();
        self.clear_lock_round();
        self.clear_lock_votes();
        self.clear_round();
        self.clear_height();
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
pub struct Vote {
    // message fields
    pub sender: ::std::vec::Vec<u8>,
    pub proposal: ::std::vec::Vec<u8>,
    pub signature: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Vote {}

impl Vote {
    pub fn new() -> Vote {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Vote {
        static mut instance: ::protobuf::lazy::Lazy<Vote> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Vote,
        };
        unsafe {
            instance.get(Vote::new)
        }
    }

    // bytes sender = 1;

    pub fn clear_sender(&mut self) {
        self.sender.clear();
    }

    // Param is passed by value, moved
    pub fn set_sender(&mut self, v: ::std::vec::Vec<u8>) {
        self.sender = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sender(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.sender
    }

    // Take field
    pub fn take_sender(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.sender, ::std::vec::Vec::new())
    }

    pub fn get_sender(&self) -> &[u8] {
        &self.sender
    }

    fn get_sender_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.sender
    }

    fn mut_sender_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.sender
    }

    // bytes proposal = 2;

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
}

impl ::protobuf::Message for Vote {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.sender)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.proposal)?;
                },
                3 => {
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
        if !self.sender.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.sender);
        }
        if !self.proposal.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.proposal);
        }
        if !self.signature.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.signature);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.sender.is_empty() {
            os.write_bytes(1, &self.sender)?;
        }
        if !self.proposal.is_empty() {
            os.write_bytes(2, &self.proposal)?;
        }
        if !self.signature.is_empty() {
            os.write_bytes(3, &self.signature)?;
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

impl ::protobuf::MessageStatic for Vote {
    fn new() -> Vote {
        Vote::new()
    }

    fn descriptor_static(_: ::std::option::Option<Vote>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "sender",
                    Vote::get_sender_for_reflect,
                    Vote::mut_sender_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "proposal",
                    Vote::get_proposal_for_reflect,
                    Vote::mut_proposal_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    Vote::get_signature_for_reflect,
                    Vote::mut_signature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Vote>(
                    "Vote",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Vote {
    fn clear(&mut self) {
        self.clear_sender();
        self.clear_proposal();
        self.clear_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Vote {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Vote {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SignedProposal {
    // message fields
    pub proposal: ::protobuf::SingularPtrField<Proposal>,
    pub signature: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SignedProposal {}

impl SignedProposal {
    pub fn new() -> SignedProposal {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SignedProposal {
        static mut instance: ::protobuf::lazy::Lazy<SignedProposal> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SignedProposal,
        };
        unsafe {
            instance.get(SignedProposal::new)
        }
    }

    // .Proposal proposal = 1;

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

impl ::protobuf::Message for SignedProposal {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.proposal)?;
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
        if let Some(ref v) = self.proposal.as_ref() {
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
        if let Some(ref v) = self.proposal.as_ref() {
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

impl ::protobuf::MessageStatic for SignedProposal {
    fn new() -> SignedProposal {
        SignedProposal::new()
    }

    fn descriptor_static(_: ::std::option::Option<SignedProposal>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Proposal>>(
                    "proposal",
                    SignedProposal::get_proposal_for_reflect,
                    SignedProposal::mut_proposal_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    SignedProposal::get_signature_for_reflect,
                    SignedProposal::mut_signature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SignedProposal>(
                    "SignedProposal",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SignedProposal {
    fn clear(&mut self) {
        self.clear_proposal();
        self.clear_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SignedProposal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SignedProposal {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fconsensus.proto\x1a\x10blockchain.proto\"\xb3\x01\n\x08Proposal\
    \x12\x1c\n\x05block\x18\x01\x20\x01(\x0b2\x06.BlockR\x05block\x12\x16\n\
    \x06islock\x18\x02\x20\x01(\x08R\x06islock\x12\x1d\n\nlock_round\x18\x03\
    \x20\x01(\x04R\tlockRound\x12$\n\nlock_votes\x18\x04\x20\x03(\x0b2\x05.V\
    oteR\tlockVotes\x12\x14\n\x05round\x18\x05\x20\x01(\x04R\x05round\x12\
    \x16\n\x06height\x18\x06\x20\x01(\x04R\x06height\"X\n\x04Vote\x12\x16\n\
    \x06sender\x18\x01\x20\x01(\x0cR\x06sender\x12\x1a\n\x08proposal\x18\x02\
    \x20\x01(\x0cR\x08proposal\x12\x1c\n\tsignature\x18\x03\x20\x01(\x0cR\ts\
    ignature\"U\n\x0eSignedProposal\x12%\n\x08proposal\x18\x01\x20\x01(\x0b2\
    \t.ProposalR\x08proposal\x12\x1c\n\tsignature\x18\x02\x20\x01(\x0cR\tsig\
    natureJ\x8c\n\n\x06\x12\x04\0\0)\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\
    \n\x02\x03\0\x12\x03\x02\x07\x19\n\xc8\x02\n\x02\x04\0\x12\x04\x11\0\x18\
    \x012M\x20message\x20VoteMessage\x20{\n\x20\x20\x20\x20\x20bytes\x20prop\
    osal\x20=\x201;\n\x20\x20\x20\x20\x20bytes\x20signature\x20=\x202;\n\x20\
    }\n2\xec\x01\x20message\x20VoteSet\x20{\n\x20\x20\x20\x20\x20//\x20map\
    \x20key\x20is\x20H160\x20converted\x20hex\x20string.\n\x20\x20\x20\x20\
    \x20map<string,\x20VoteMessage>\x20votes_by_sender\x20=\x201;\n\x20\x20\
    \x20\x20\x20//\x20map\x20key\x20is\x20H256\x20converted\x20hex\x20string\
    .\n\x20\x20\x20\x20\x20map<string,\x20uint64>\x20votes_by_proposal\x20=\
    \x202;\n\x20\x20\x20\x20\x20uint64\x20count\x20=\x203;\n\x20}\n\n\n\n\
    \x03\x04\0\x01\x12\x03\x11\x08\x10\n\x0b\n\x04\x04\0\x02\0\x12\x03\x12\
    \x04\x14\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x12\x04\x11\x12\n\x0c\n\x05\
    \x04\0\x02\0\x06\x12\x03\x12\x04\t\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x12\n\x0f\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x12\x12\x13\n\x0b\n\x04\
    \x04\0\x02\x01\x12\x03\x13\x04\x14\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\
    \x13\x04\x12\x14\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x13\x04\x08\n\x0c\
    \n\x05\x04\0\x02\x01\x01\x12\x03\x13\t\x0f\n\x0c\n\x05\x04\0\x02\x01\x03\
    \x12\x03\x13\x12\x13\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x14\x04\x1a\n\r\n\
    \x05\x04\0\x02\x02\x04\x12\x04\x14\x04\x13\x14\n\x0c\n\x05\x04\0\x02\x02\
    \x05\x12\x03\x14\x04\n\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x14\x0b\x15\
    \n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x14\x18\x19\n\x0b\n\x04\x04\0\x02\
    \x03\x12\x03\x15\x04!\n\x0c\n\x05\x04\0\x02\x03\x04\x12\x03\x15\x04\x0c\
    \n\x0c\n\x05\x04\0\x02\x03\x06\x12\x03\x15\r\x11\n\x0c\n\x05\x04\0\x02\
    \x03\x01\x12\x03\x15\x12\x1c\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x15\
    \x1f\x20\n\x0b\n\x04\x04\0\x02\x04\x12\x03\x16\x04\x15\n\r\n\x05\x04\0\
    \x02\x04\x04\x12\x04\x16\x04\x15!\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\
    \x16\x04\n\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x16\x0b\x10\n\x0c\n\x05\
    \x04\0\x02\x04\x03\x12\x03\x16\x13\x14\n\x0b\n\x04\x04\0\x02\x05\x12\x03\
    \x17\x04\x16\n\r\n\x05\x04\0\x02\x05\x04\x12\x04\x17\x04\x16\x15\n\x0c\n\
    \x05\x04\0\x02\x05\x05\x12\x03\x17\x04\n\n\x0c\n\x05\x04\0\x02\x05\x01\
    \x12\x03\x17\x0b\x11\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\x17\x14\x15\n\
    \n\n\x02\x04\x01\x12\x04\x1a\0\x1e\x01\n\n\n\x03\x04\x01\x01\x12\x03\x1a\
    \x08\x0c\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x1b\x04\x15\n\r\n\x05\x04\x01\
    \x02\0\x04\x12\x04\x1b\x04\x1a\x0e\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\
    \x1b\x04\t\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x1b\n\x10\n\x0c\n\x05\
    \x04\x01\x02\0\x03\x12\x03\x1b\x13\x14\n\x0b\n\x04\x04\x01\x02\x01\x12\
    \x03\x1c\x04\x17\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x1c\x04\x1b\x15\n\
    \x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x1c\x04\t\n\x0c\n\x05\x04\x01\x02\
    \x01\x01\x12\x03\x1c\n\x12\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x1c\
    \x15\x16\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x1d\x04\x18\n\r\n\x05\x04\
    \x01\x02\x02\x04\x12\x04\x1d\x04\x1c\x17\n\x0c\n\x05\x04\x01\x02\x02\x05\
    \x12\x03\x1d\x04\t\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x1d\n\x13\n\
    \x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x1d\x16\x17\nq\n\x02\x04\x02\x12\
    \x04&\0)\x012e\x20message\x20ProposeStep\x20{\n\x20\x20\x20\x20\x20uint6\
    4\x20height\x20=\x201;\n\x20\x20\x20\x20\x20uint64\x20round\x20=\x202;\n\
    \x20\x20\x20\x20\x20Proposal\x20proposal\x20=\x203;\n\x20}\n\n\n\n\x03\
    \x04\x02\x01\x12\x03&\x08\x16\n\x0b\n\x04\x04\x02\x02\0\x12\x03'\x04\x1a\
    \n\r\n\x05\x04\x02\x02\0\x04\x12\x04'\x04&\x18\n\x0c\n\x05\x04\x02\x02\0\
    \x06\x12\x03'\x04\x0c\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03'\r\x15\n\x0c\
    \n\x05\x04\x02\x02\0\x03\x12\x03'\x18\x19\n\x0b\n\x04\x04\x02\x02\x01\
    \x12\x03(\x04\x18\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04(\x04'\x1a\n\x0c\
    \n\x05\x04\x02\x02\x01\x05\x12\x03(\x04\t\n\x0c\n\x05\x04\x02\x02\x01\
    \x01\x12\x03(\n\x13\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03(\x16\x17b\
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
