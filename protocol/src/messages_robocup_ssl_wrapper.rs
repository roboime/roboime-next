// This file is generated. Do not edit
// @generated

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct SSL_WrapperPacket {
    // message fields
    detection: ::protobuf::SingularPtrField<super::messages_robocup_ssl_detection::SSL_DetectionFrame>,
    geometry: ::protobuf::SingularPtrField<super::messages_robocup_ssl_geometry::SSL_GeometryData>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SSL_WrapperPacket {}

impl SSL_WrapperPacket {
    pub fn new() -> SSL_WrapperPacket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SSL_WrapperPacket {
        static mut instance: ::protobuf::lazy::Lazy<SSL_WrapperPacket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SSL_WrapperPacket,
        };
        unsafe {
            instance.get(|| {
                SSL_WrapperPacket {
                    detection: ::protobuf::SingularPtrField::none(),
                    geometry: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .SSL_DetectionFrame detection = 1;

    pub fn clear_detection(&mut self) {
        self.detection.clear();
    }

    pub fn has_detection(&self) -> bool {
        self.detection.is_some()
    }

    // Param is passed by value, moved
    pub fn set_detection(&mut self, v: super::messages_robocup_ssl_detection::SSL_DetectionFrame) {
        self.detection = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_detection<'a>(&'a mut self) -> &'a mut super::messages_robocup_ssl_detection::SSL_DetectionFrame {
        if self.detection.is_none() {
            self.detection.set_default();
        };
        self.detection.as_mut().unwrap()
    }

    // Take field
    pub fn take_detection(&mut self) -> super::messages_robocup_ssl_detection::SSL_DetectionFrame {
        self.detection.take().unwrap_or_else(|| super::messages_robocup_ssl_detection::SSL_DetectionFrame::new())
    }

    pub fn get_detection<'a>(&'a self) -> &'a super::messages_robocup_ssl_detection::SSL_DetectionFrame {
        self.detection.as_ref().unwrap_or_else(|| super::messages_robocup_ssl_detection::SSL_DetectionFrame::default_instance())
    }

    // optional .SSL_GeometryData geometry = 2;

    pub fn clear_geometry(&mut self) {
        self.geometry.clear();
    }

    pub fn has_geometry(&self) -> bool {
        self.geometry.is_some()
    }

    // Param is passed by value, moved
    pub fn set_geometry(&mut self, v: super::messages_robocup_ssl_geometry::SSL_GeometryData) {
        self.geometry = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_geometry<'a>(&'a mut self) -> &'a mut super::messages_robocup_ssl_geometry::SSL_GeometryData {
        if self.geometry.is_none() {
            self.geometry.set_default();
        };
        self.geometry.as_mut().unwrap()
    }

    // Take field
    pub fn take_geometry(&mut self) -> super::messages_robocup_ssl_geometry::SSL_GeometryData {
        self.geometry.take().unwrap_or_else(|| super::messages_robocup_ssl_geometry::SSL_GeometryData::new())
    }

    pub fn get_geometry<'a>(&'a self) -> &'a super::messages_robocup_ssl_geometry::SSL_GeometryData {
        self.geometry.as_ref().unwrap_or_else(|| super::messages_robocup_ssl_geometry::SSL_GeometryData::default_instance())
    }
}

impl ::protobuf::Message for SSL_WrapperPacket {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.detection));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.geometry));
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
        for value in self.detection.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.geometry.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.detection.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.geometry.as_ref() {
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

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SSL_WrapperPacket>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SSL_WrapperPacket {
    fn new() -> SSL_WrapperPacket {
        SSL_WrapperPacket::new()
    }

    fn descriptor_static(_: ::std::option::Option<SSL_WrapperPacket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "detection",
                    SSL_WrapperPacket::has_detection,
                    SSL_WrapperPacket::get_detection,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "geometry",
                    SSL_WrapperPacket::has_geometry,
                    SSL_WrapperPacket::get_geometry,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SSL_WrapperPacket>(
                    "SSL_WrapperPacket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SSL_WrapperPacket {
    fn clear(&mut self) {
        self.clear_detection();
        self.clear_geometry();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SSL_WrapperPacket {
    fn eq(&self, other: &SSL_WrapperPacket) -> bool {
        self.detection == other.detection &&
        self.geometry == other.geometry &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SSL_WrapperPacket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x22, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x5f, 0x72, 0x6f, 0x62, 0x6f, 0x63,
    0x75, 0x70, 0x5f, 0x73, 0x73, 0x6c, 0x5f, 0x77, 0x72, 0x61, 0x70, 0x70, 0x65, 0x72, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x24, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x5f, 0x72,
    0x6f, 0x62, 0x6f, 0x63, 0x75, 0x70, 0x5f, 0x73, 0x73, 0x6c, 0x5f, 0x64, 0x65, 0x74, 0x65, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x23, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x73, 0x5f, 0x72, 0x6f, 0x62, 0x6f, 0x63, 0x75, 0x70, 0x5f, 0x73, 0x73, 0x6c,
    0x5f, 0x67, 0x65, 0x6f, 0x6d, 0x65, 0x74, 0x72, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0x60, 0x0a, 0x11, 0x53, 0x53, 0x4c, 0x5f, 0x57, 0x72, 0x61, 0x70, 0x70, 0x65, 0x72, 0x50, 0x61,
    0x63, 0x6b, 0x65, 0x74, 0x12, 0x26, 0x0a, 0x09, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x53, 0x53, 0x4c, 0x5f, 0x44, 0x65,
    0x74, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x12, 0x23, 0x0a, 0x08,
    0x67, 0x65, 0x6f, 0x6d, 0x65, 0x74, 0x72, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11,
    0x2e, 0x53, 0x53, 0x4c, 0x5f, 0x47, 0x65, 0x6f, 0x6d, 0x65, 0x74, 0x72, 0x79, 0x44, 0x61, 0x74,
    0x61, 0x4a, 0xc0, 0x01, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x06, 0x01, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x00, 0x12, 0x03, 0x00, 0x07, 0x2d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x01,
    0x07, 0x2c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x03, 0x00, 0x06, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x04, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x04, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x04, 0x0b, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x1e,
    0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x04, 0x2a, 0x2b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x05, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x06, 0x12, 0x03, 0x05, 0x0b, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x05, 0x1c, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x05, 0x27, 0x28,
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
