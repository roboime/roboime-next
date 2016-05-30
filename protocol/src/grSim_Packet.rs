// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct grSim_Packet {
    // message fields
    commands: ::protobuf::SingularPtrField<super::grSim_Commands::grSim_Commands>,
    replacement: ::protobuf::SingularPtrField<super::grSim_Replacement::grSim_Replacement>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for grSim_Packet {}

impl grSim_Packet {
    pub fn new() -> grSim_Packet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static grSim_Packet {
        static mut instance: ::protobuf::lazy::Lazy<grSim_Packet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const grSim_Packet,
        };
        unsafe {
            instance.get(|| {
                grSim_Packet {
                    commands: ::protobuf::SingularPtrField::none(),
                    replacement: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .grSim_Commands commands = 1;

    pub fn clear_commands(&mut self) {
        self.commands.clear();
    }

    pub fn has_commands(&self) -> bool {
        self.commands.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commands(&mut self, v: super::grSim_Commands::grSim_Commands) {
        self.commands = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_commands(&mut self) -> &mut super::grSim_Commands::grSim_Commands {
        if self.commands.is_none() {
            self.commands.set_default();
        };
        self.commands.as_mut().unwrap()
    }

    // Take field
    pub fn take_commands(&mut self) -> super::grSim_Commands::grSim_Commands {
        self.commands.take().unwrap_or_else(|| super::grSim_Commands::grSim_Commands::new())
    }

    pub fn get_commands(&self) -> &super::grSim_Commands::grSim_Commands {
        self.commands.as_ref().unwrap_or_else(|| super::grSim_Commands::grSim_Commands::default_instance())
    }

    // optional .grSim_Replacement replacement = 2;

    pub fn clear_replacement(&mut self) {
        self.replacement.clear();
    }

    pub fn has_replacement(&self) -> bool {
        self.replacement.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replacement(&mut self, v: super::grSim_Replacement::grSim_Replacement) {
        self.replacement = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_replacement(&mut self) -> &mut super::grSim_Replacement::grSim_Replacement {
        if self.replacement.is_none() {
            self.replacement.set_default();
        };
        self.replacement.as_mut().unwrap()
    }

    // Take field
    pub fn take_replacement(&mut self) -> super::grSim_Replacement::grSim_Replacement {
        self.replacement.take().unwrap_or_else(|| super::grSim_Replacement::grSim_Replacement::new())
    }

    pub fn get_replacement(&self) -> &super::grSim_Replacement::grSim_Replacement {
        self.replacement.as_ref().unwrap_or_else(|| super::grSim_Replacement::grSim_Replacement::default_instance())
    }
}

impl ::protobuf::Message for grSim_Packet {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.commands));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.replacement));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.commands.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.replacement.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.commands.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.replacement.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<grSim_Packet>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for grSim_Packet {
    fn new() -> grSim_Packet {
        grSim_Packet::new()
    }

    fn descriptor_static(_: ::std::option::Option<grSim_Packet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "commands",
                    grSim_Packet::has_commands,
                    grSim_Packet::get_commands,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "replacement",
                    grSim_Packet::has_replacement,
                    grSim_Packet::get_replacement,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<grSim_Packet>(
                    "grSim_Packet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for grSim_Packet {
    fn clear(&mut self) {
        self.clear_commands();
        self.clear_replacement();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for grSim_Packet {
    fn eq(&self, other: &grSim_Packet) -> bool {
        self.commands == other.commands &&
        self.replacement == other.replacement &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for grSim_Packet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x12, 0x67, 0x72, 0x53, 0x69, 0x6d, 0x5f, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x14, 0x67, 0x72, 0x53, 0x69, 0x6d, 0x5f, 0x43, 0x6f, 0x6d, 0x6d,
    0x61, 0x6e, 0x64, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x67, 0x72, 0x53, 0x69,
    0x6d, 0x5f, 0x52, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0x5a, 0x0a, 0x0c, 0x67, 0x72, 0x53, 0x69, 0x6d, 0x5f, 0x50, 0x61, 0x63,
    0x6b, 0x65, 0x74, 0x12, 0x21, 0x0a, 0x08, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x73, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x67, 0x72, 0x53, 0x69, 0x6d, 0x5f, 0x43, 0x6f,
    0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x73, 0x12, 0x27, 0x0a, 0x0b, 0x72, 0x65, 0x70, 0x6c, 0x61, 0x63,
    0x65, 0x6d, 0x65, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x67, 0x72,
    0x53, 0x69, 0x6d, 0x5f, 0x52, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x4a,
    0xc0, 0x01, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x05, 0x01, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00,
    0x12, 0x03, 0x00, 0x07, 0x1d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x01, 0x07, 0x20,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02, 0x00, 0x05, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x03, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x03, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x03, 0x0d,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x1c, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x03, 0x27, 0x28, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x04, 0x04, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x04, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x04, 0x0d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x04, 0x1f, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x04,
    0x2d, 0x2e,
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
