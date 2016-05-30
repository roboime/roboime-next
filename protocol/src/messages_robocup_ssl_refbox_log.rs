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
pub struct Log_Frame {
    // message fields
    frame: ::protobuf::SingularPtrField<super::messages_robocup_ssl_detection::SSL_DetectionFrame>,
    refbox_cmd: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Log_Frame {}

impl Log_Frame {
    pub fn new() -> Log_Frame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Log_Frame {
        static mut instance: ::protobuf::lazy::Lazy<Log_Frame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Log_Frame,
        };
        unsafe {
            instance.get(|| {
                Log_Frame {
                    frame: ::protobuf::SingularPtrField::none(),
                    refbox_cmd: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .SSL_DetectionFrame frame = 1;

    pub fn clear_frame(&mut self) {
        self.frame.clear();
    }

    pub fn has_frame(&self) -> bool {
        self.frame.is_some()
    }

    // Param is passed by value, moved
    pub fn set_frame(&mut self, v: super::messages_robocup_ssl_detection::SSL_DetectionFrame) {
        self.frame = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_frame(&mut self) -> &mut super::messages_robocup_ssl_detection::SSL_DetectionFrame {
        if self.frame.is_none() {
            self.frame.set_default();
        };
        self.frame.as_mut().unwrap()
    }

    // Take field
    pub fn take_frame(&mut self) -> super::messages_robocup_ssl_detection::SSL_DetectionFrame {
        self.frame.take().unwrap_or_else(|| super::messages_robocup_ssl_detection::SSL_DetectionFrame::new())
    }

    pub fn get_frame(&self) -> &super::messages_robocup_ssl_detection::SSL_DetectionFrame {
        self.frame.as_ref().unwrap_or_else(|| super::messages_robocup_ssl_detection::SSL_DetectionFrame::default_instance())
    }

    // required string refbox_cmd = 2;

    pub fn clear_refbox_cmd(&mut self) {
        self.refbox_cmd.clear();
    }

    pub fn has_refbox_cmd(&self) -> bool {
        self.refbox_cmd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_refbox_cmd(&mut self, v: ::std::string::String) {
        self.refbox_cmd = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_refbox_cmd(&mut self) -> &mut ::std::string::String {
        if self.refbox_cmd.is_none() {
            self.refbox_cmd.set_default();
        };
        self.refbox_cmd.as_mut().unwrap()
    }

    // Take field
    pub fn take_refbox_cmd(&mut self) -> ::std::string::String {
        self.refbox_cmd.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_refbox_cmd(&self) -> &str {
        match self.refbox_cmd.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Log_Frame {
    fn is_initialized(&self) -> bool {
        if self.frame.is_none() {
            return false;
        };
        if self.refbox_cmd.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.frame));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.refbox_cmd));
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
        for value in self.frame.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.refbox_cmd.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.frame.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.refbox_cmd.as_ref() {
            try!(os.write_string(2, &v));
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
        ::std::any::TypeId::of::<Log_Frame>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Log_Frame {
    fn new() -> Log_Frame {
        Log_Frame::new()
    }

    fn descriptor_static(_: ::std::option::Option<Log_Frame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "frame",
                    Log_Frame::has_frame,
                    Log_Frame::get_frame,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "refbox_cmd",
                    Log_Frame::has_refbox_cmd,
                    Log_Frame::get_refbox_cmd,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Log_Frame>(
                    "Log_Frame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Log_Frame {
    fn clear(&mut self) {
        self.clear_frame();
        self.clear_refbox_cmd();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Log_Frame {
    fn eq(&self, other: &Log_Frame) -> bool {
        self.frame == other.frame &&
        self.refbox_cmd == other.refbox_cmd &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Log_Frame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Refbox_Log {
    // message fields
    log: ::protobuf::RepeatedField<Log_Frame>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Refbox_Log {}

impl Refbox_Log {
    pub fn new() -> Refbox_Log {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Refbox_Log {
        static mut instance: ::protobuf::lazy::Lazy<Refbox_Log> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Refbox_Log,
        };
        unsafe {
            instance.get(|| {
                Refbox_Log {
                    log: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .Log_Frame log = 1;

    pub fn clear_log(&mut self) {
        self.log.clear();
    }

    // Param is passed by value, moved
    pub fn set_log(&mut self, v: ::protobuf::RepeatedField<Log_Frame>) {
        self.log = v;
    }

    // Mutable pointer to the field.
    pub fn mut_log(&mut self) -> &mut ::protobuf::RepeatedField<Log_Frame> {
        &mut self.log
    }

    // Take field
    pub fn take_log(&mut self) -> ::protobuf::RepeatedField<Log_Frame> {
        ::std::mem::replace(&mut self.log, ::protobuf::RepeatedField::new())
    }

    pub fn get_log(&self) -> &[Log_Frame] {
        &self.log
    }
}

impl ::protobuf::Message for Refbox_Log {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.log));
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
        for value in self.log.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.log.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<Refbox_Log>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Refbox_Log {
    fn new() -> Refbox_Log {
        Refbox_Log::new()
    }

    fn descriptor_static(_: ::std::option::Option<Refbox_Log>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "log",
                    Refbox_Log::get_log,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Refbox_Log>(
                    "Refbox_Log",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Refbox_Log {
    fn clear(&mut self) {
        self.clear_log();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Refbox_Log {
    fn eq(&self, other: &Refbox_Log) -> bool {
        self.log == other.log &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Refbox_Log {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x25, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x5f, 0x72, 0x6f, 0x62, 0x6f, 0x63,
    0x75, 0x70, 0x5f, 0x73, 0x73, 0x6c, 0x5f, 0x72, 0x65, 0x66, 0x62, 0x6f, 0x78, 0x5f, 0x6c, 0x6f,
    0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x24, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x73, 0x5f, 0x72, 0x6f, 0x62, 0x6f, 0x63, 0x75, 0x70, 0x5f, 0x73, 0x73, 0x6c, 0x5f, 0x64, 0x65,
    0x74, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x43, 0x0a,
    0x09, 0x4c, 0x6f, 0x67, 0x5f, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x12, 0x22, 0x0a, 0x05, 0x66, 0x72,
    0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x53, 0x53, 0x4c, 0x5f,
    0x44, 0x65, 0x74, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x12, 0x12,
    0x0a, 0x0a, 0x72, 0x65, 0x66, 0x62, 0x6f, 0x78, 0x5f, 0x63, 0x6d, 0x64, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x09, 0x22, 0x25, 0x0a, 0x0a, 0x52, 0x65, 0x66, 0x62, 0x6f, 0x78, 0x5f, 0x4c, 0x6f, 0x67,
    0x12, 0x17, 0x0a, 0x03, 0x6c, 0x6f, 0x67, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0a, 0x2e,
    0x4c, 0x6f, 0x67, 0x5f, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x4a, 0x92, 0x02, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x0b, 0x01, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x00, 0x07, 0x2d, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02, 0x00, 0x06, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x04, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x04,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x04, 0x0d, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x20, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x04, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x05, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x05, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x05, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x05, 0x14, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x05, 0x21,
    0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x08, 0x00, 0x0b, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x08, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x0a, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x0a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0a,
    0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x17, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x1d, 0x1e,
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
