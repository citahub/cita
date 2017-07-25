// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Command {
    NEW_BLK = 0,
}

impl ::protobuf::ProtobufEnum for Command {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Command> {
        match value {
            0 => ::std::option::Option::Some(Command::NEW_BLK),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Command] = &[
            Command::NEW_BLK,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Command>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Command", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Command {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2a,
    0x16, 0x0a, 0x07, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x12, 0x0b, 0x0a, 0x07, 0x4e, 0x45,
    0x57, 0x5f, 0x42, 0x4c, 0x4b, 0x10, 0x00, 0x4a, 0x53, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x04,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x05,
    0x00, 0x12, 0x04, 0x02, 0x00, 0x04, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03,
    0x02, 0x05, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x04, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x04, 0x0b, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x03, 0x0e, 0x0f, 0x62, 0x06, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x33,
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
