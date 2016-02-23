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
pub struct SSL_DetectionBall {
    // message fields
    confidence: ::std::option::Option<f32>,
    area: ::std::option::Option<u32>,
    x: ::std::option::Option<f32>,
    y: ::std::option::Option<f32>,
    z: ::std::option::Option<f32>,
    pixel_x: ::std::option::Option<f32>,
    pixel_y: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SSL_DetectionBall {}

impl SSL_DetectionBall {
    pub fn new() -> SSL_DetectionBall {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SSL_DetectionBall {
        static mut instance: ::protobuf::lazy::Lazy<SSL_DetectionBall> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SSL_DetectionBall,
        };
        unsafe {
            instance.get(|| {
                SSL_DetectionBall {
                    confidence: ::std::option::Option::None,
                    area: ::std::option::Option::None,
                    x: ::std::option::Option::None,
                    y: ::std::option::Option::None,
                    z: ::std::option::Option::None,
                    pixel_x: ::std::option::Option::None,
                    pixel_y: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required float confidence = 1;

    pub fn clear_confidence(&mut self) {
        self.confidence = ::std::option::Option::None;
    }

    pub fn has_confidence(&self) -> bool {
        self.confidence.is_some()
    }

    // Param is passed by value, moved
    pub fn set_confidence(&mut self, v: f32) {
        self.confidence = ::std::option::Option::Some(v);
    }

    pub fn get_confidence<'a>(&self) -> f32 {
        self.confidence.unwrap_or(0.)
    }

    // optional uint32 area = 2;

    pub fn clear_area(&mut self) {
        self.area = ::std::option::Option::None;
    }

    pub fn has_area(&self) -> bool {
        self.area.is_some()
    }

    // Param is passed by value, moved
    pub fn set_area(&mut self, v: u32) {
        self.area = ::std::option::Option::Some(v);
    }

    pub fn get_area<'a>(&self) -> u32 {
        self.area.unwrap_or(0)
    }

    // required float x = 3;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x<'a>(&self) -> f32 {
        self.x.unwrap_or(0.)
    }

    // required float y = 4;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y<'a>(&self) -> f32 {
        self.y.unwrap_or(0.)
    }

    // optional float z = 5;

    pub fn clear_z(&mut self) {
        self.z = ::std::option::Option::None;
    }

    pub fn has_z(&self) -> bool {
        self.z.is_some()
    }

    // Param is passed by value, moved
    pub fn set_z(&mut self, v: f32) {
        self.z = ::std::option::Option::Some(v);
    }

    pub fn get_z<'a>(&self) -> f32 {
        self.z.unwrap_or(0.)
    }

    // required float pixel_x = 6;

    pub fn clear_pixel_x(&mut self) {
        self.pixel_x = ::std::option::Option::None;
    }

    pub fn has_pixel_x(&self) -> bool {
        self.pixel_x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pixel_x(&mut self, v: f32) {
        self.pixel_x = ::std::option::Option::Some(v);
    }

    pub fn get_pixel_x<'a>(&self) -> f32 {
        self.pixel_x.unwrap_or(0.)
    }

    // required float pixel_y = 7;

    pub fn clear_pixel_y(&mut self) {
        self.pixel_y = ::std::option::Option::None;
    }

    pub fn has_pixel_y(&self) -> bool {
        self.pixel_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pixel_y(&mut self, v: f32) {
        self.pixel_y = ::std::option::Option::Some(v);
    }

    pub fn get_pixel_y<'a>(&self) -> f32 {
        self.pixel_y.unwrap_or(0.)
    }
}

impl ::protobuf::Message for SSL_DetectionBall {
    fn is_initialized(&self) -> bool {
        if self.confidence.is_none() {
            return false;
        };
        if self.x.is_none() {
            return false;
        };
        if self.y.is_none() {
            return false;
        };
        if self.pixel_x.is_none() {
            return false;
        };
        if self.pixel_y.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.confidence = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.area = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.x = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.y = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.z = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.pixel_x = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.pixel_y = ::std::option::Option::Some(tmp);
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
        if self.confidence.is_some() {
            my_size += 5;
        };
        for value in self.area.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.x.is_some() {
            my_size += 5;
        };
        if self.y.is_some() {
            my_size += 5;
        };
        if self.z.is_some() {
            my_size += 5;
        };
        if self.pixel_x.is_some() {
            my_size += 5;
        };
        if self.pixel_y.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.confidence {
            try!(os.write_float(1, v));
        };
        if let Some(v) = self.area {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.x {
            try!(os.write_float(3, v));
        };
        if let Some(v) = self.y {
            try!(os.write_float(4, v));
        };
        if let Some(v) = self.z {
            try!(os.write_float(5, v));
        };
        if let Some(v) = self.pixel_x {
            try!(os.write_float(6, v));
        };
        if let Some(v) = self.pixel_y {
            try!(os.write_float(7, v));
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
        ::std::any::TypeId::of::<SSL_DetectionBall>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SSL_DetectionBall {
    fn new() -> SSL_DetectionBall {
        SSL_DetectionBall::new()
    }

    fn descriptor_static(_: ::std::option::Option<SSL_DetectionBall>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "confidence",
                    SSL_DetectionBall::has_confidence,
                    SSL_DetectionBall::get_confidence,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "area",
                    SSL_DetectionBall::has_area,
                    SSL_DetectionBall::get_area,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "x",
                    SSL_DetectionBall::has_x,
                    SSL_DetectionBall::get_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "y",
                    SSL_DetectionBall::has_y,
                    SSL_DetectionBall::get_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "z",
                    SSL_DetectionBall::has_z,
                    SSL_DetectionBall::get_z,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "pixel_x",
                    SSL_DetectionBall::has_pixel_x,
                    SSL_DetectionBall::get_pixel_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "pixel_y",
                    SSL_DetectionBall::has_pixel_y,
                    SSL_DetectionBall::get_pixel_y,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SSL_DetectionBall>(
                    "SSL_DetectionBall",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SSL_DetectionBall {
    fn clear(&mut self) {
        self.clear_confidence();
        self.clear_area();
        self.clear_x();
        self.clear_y();
        self.clear_z();
        self.clear_pixel_x();
        self.clear_pixel_y();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SSL_DetectionBall {
    fn eq(&self, other: &SSL_DetectionBall) -> bool {
        self.confidence == other.confidence &&
        self.area == other.area &&
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z &&
        self.pixel_x == other.pixel_x &&
        self.pixel_y == other.pixel_y &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SSL_DetectionBall {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SSL_DetectionRobot {
    // message fields
    confidence: ::std::option::Option<f32>,
    robot_id: ::std::option::Option<u32>,
    x: ::std::option::Option<f32>,
    y: ::std::option::Option<f32>,
    orientation: ::std::option::Option<f32>,
    pixel_x: ::std::option::Option<f32>,
    pixel_y: ::std::option::Option<f32>,
    height: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SSL_DetectionRobot {}

impl SSL_DetectionRobot {
    pub fn new() -> SSL_DetectionRobot {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SSL_DetectionRobot {
        static mut instance: ::protobuf::lazy::Lazy<SSL_DetectionRobot> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SSL_DetectionRobot,
        };
        unsafe {
            instance.get(|| {
                SSL_DetectionRobot {
                    confidence: ::std::option::Option::None,
                    robot_id: ::std::option::Option::None,
                    x: ::std::option::Option::None,
                    y: ::std::option::Option::None,
                    orientation: ::std::option::Option::None,
                    pixel_x: ::std::option::Option::None,
                    pixel_y: ::std::option::Option::None,
                    height: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required float confidence = 1;

    pub fn clear_confidence(&mut self) {
        self.confidence = ::std::option::Option::None;
    }

    pub fn has_confidence(&self) -> bool {
        self.confidence.is_some()
    }

    // Param is passed by value, moved
    pub fn set_confidence(&mut self, v: f32) {
        self.confidence = ::std::option::Option::Some(v);
    }

    pub fn get_confidence<'a>(&self) -> f32 {
        self.confidence.unwrap_or(0.)
    }

    // optional uint32 robot_id = 2;

    pub fn clear_robot_id(&mut self) {
        self.robot_id = ::std::option::Option::None;
    }

    pub fn has_robot_id(&self) -> bool {
        self.robot_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_robot_id(&mut self, v: u32) {
        self.robot_id = ::std::option::Option::Some(v);
    }

    pub fn get_robot_id<'a>(&self) -> u32 {
        self.robot_id.unwrap_or(0)
    }

    // required float x = 3;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x<'a>(&self) -> f32 {
        self.x.unwrap_or(0.)
    }

    // required float y = 4;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y<'a>(&self) -> f32 {
        self.y.unwrap_or(0.)
    }

    // optional float orientation = 5;

    pub fn clear_orientation(&mut self) {
        self.orientation = ::std::option::Option::None;
    }

    pub fn has_orientation(&self) -> bool {
        self.orientation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_orientation(&mut self, v: f32) {
        self.orientation = ::std::option::Option::Some(v);
    }

    pub fn get_orientation<'a>(&self) -> f32 {
        self.orientation.unwrap_or(0.)
    }

    // required float pixel_x = 6;

    pub fn clear_pixel_x(&mut self) {
        self.pixel_x = ::std::option::Option::None;
    }

    pub fn has_pixel_x(&self) -> bool {
        self.pixel_x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pixel_x(&mut self, v: f32) {
        self.pixel_x = ::std::option::Option::Some(v);
    }

    pub fn get_pixel_x<'a>(&self) -> f32 {
        self.pixel_x.unwrap_or(0.)
    }

    // required float pixel_y = 7;

    pub fn clear_pixel_y(&mut self) {
        self.pixel_y = ::std::option::Option::None;
    }

    pub fn has_pixel_y(&self) -> bool {
        self.pixel_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pixel_y(&mut self, v: f32) {
        self.pixel_y = ::std::option::Option::Some(v);
    }

    pub fn get_pixel_y<'a>(&self) -> f32 {
        self.pixel_y.unwrap_or(0.)
    }

    // optional float height = 8;

    pub fn clear_height(&mut self) {
        self.height = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        self.height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: f32) {
        self.height = ::std::option::Option::Some(v);
    }

    pub fn get_height<'a>(&self) -> f32 {
        self.height.unwrap_or(0.)
    }
}

impl ::protobuf::Message for SSL_DetectionRobot {
    fn is_initialized(&self) -> bool {
        if self.confidence.is_none() {
            return false;
        };
        if self.x.is_none() {
            return false;
        };
        if self.y.is_none() {
            return false;
        };
        if self.pixel_x.is_none() {
            return false;
        };
        if self.pixel_y.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.confidence = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.robot_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.x = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.y = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.orientation = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.pixel_x = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.pixel_y = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.height = ::std::option::Option::Some(tmp);
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
        if self.confidence.is_some() {
            my_size += 5;
        };
        for value in self.robot_id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.x.is_some() {
            my_size += 5;
        };
        if self.y.is_some() {
            my_size += 5;
        };
        if self.orientation.is_some() {
            my_size += 5;
        };
        if self.pixel_x.is_some() {
            my_size += 5;
        };
        if self.pixel_y.is_some() {
            my_size += 5;
        };
        if self.height.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.confidence {
            try!(os.write_float(1, v));
        };
        if let Some(v) = self.robot_id {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.x {
            try!(os.write_float(3, v));
        };
        if let Some(v) = self.y {
            try!(os.write_float(4, v));
        };
        if let Some(v) = self.orientation {
            try!(os.write_float(5, v));
        };
        if let Some(v) = self.pixel_x {
            try!(os.write_float(6, v));
        };
        if let Some(v) = self.pixel_y {
            try!(os.write_float(7, v));
        };
        if let Some(v) = self.height {
            try!(os.write_float(8, v));
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
        ::std::any::TypeId::of::<SSL_DetectionRobot>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SSL_DetectionRobot {
    fn new() -> SSL_DetectionRobot {
        SSL_DetectionRobot::new()
    }

    fn descriptor_static(_: ::std::option::Option<SSL_DetectionRobot>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "confidence",
                    SSL_DetectionRobot::has_confidence,
                    SSL_DetectionRobot::get_confidence,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "robot_id",
                    SSL_DetectionRobot::has_robot_id,
                    SSL_DetectionRobot::get_robot_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "x",
                    SSL_DetectionRobot::has_x,
                    SSL_DetectionRobot::get_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "y",
                    SSL_DetectionRobot::has_y,
                    SSL_DetectionRobot::get_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "orientation",
                    SSL_DetectionRobot::has_orientation,
                    SSL_DetectionRobot::get_orientation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "pixel_x",
                    SSL_DetectionRobot::has_pixel_x,
                    SSL_DetectionRobot::get_pixel_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "pixel_y",
                    SSL_DetectionRobot::has_pixel_y,
                    SSL_DetectionRobot::get_pixel_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "height",
                    SSL_DetectionRobot::has_height,
                    SSL_DetectionRobot::get_height,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SSL_DetectionRobot>(
                    "SSL_DetectionRobot",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SSL_DetectionRobot {
    fn clear(&mut self) {
        self.clear_confidence();
        self.clear_robot_id();
        self.clear_x();
        self.clear_y();
        self.clear_orientation();
        self.clear_pixel_x();
        self.clear_pixel_y();
        self.clear_height();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SSL_DetectionRobot {
    fn eq(&self, other: &SSL_DetectionRobot) -> bool {
        self.confidence == other.confidence &&
        self.robot_id == other.robot_id &&
        self.x == other.x &&
        self.y == other.y &&
        self.orientation == other.orientation &&
        self.pixel_x == other.pixel_x &&
        self.pixel_y == other.pixel_y &&
        self.height == other.height &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SSL_DetectionRobot {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SSL_DetectionFrame {
    // message fields
    frame_number: ::std::option::Option<u32>,
    t_capture: ::std::option::Option<f64>,
    t_sent: ::std::option::Option<f64>,
    camera_id: ::std::option::Option<u32>,
    balls: ::protobuf::RepeatedField<SSL_DetectionBall>,
    robots_yellow: ::protobuf::RepeatedField<SSL_DetectionRobot>,
    robots_blue: ::protobuf::RepeatedField<SSL_DetectionRobot>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SSL_DetectionFrame {}

impl SSL_DetectionFrame {
    pub fn new() -> SSL_DetectionFrame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SSL_DetectionFrame {
        static mut instance: ::protobuf::lazy::Lazy<SSL_DetectionFrame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SSL_DetectionFrame,
        };
        unsafe {
            instance.get(|| {
                SSL_DetectionFrame {
                    frame_number: ::std::option::Option::None,
                    t_capture: ::std::option::Option::None,
                    t_sent: ::std::option::Option::None,
                    camera_id: ::std::option::Option::None,
                    balls: ::protobuf::RepeatedField::new(),
                    robots_yellow: ::protobuf::RepeatedField::new(),
                    robots_blue: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 frame_number = 1;

    pub fn clear_frame_number(&mut self) {
        self.frame_number = ::std::option::Option::None;
    }

    pub fn has_frame_number(&self) -> bool {
        self.frame_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_frame_number(&mut self, v: u32) {
        self.frame_number = ::std::option::Option::Some(v);
    }

    pub fn get_frame_number<'a>(&self) -> u32 {
        self.frame_number.unwrap_or(0)
    }

    // required double t_capture = 2;

    pub fn clear_t_capture(&mut self) {
        self.t_capture = ::std::option::Option::None;
    }

    pub fn has_t_capture(&self) -> bool {
        self.t_capture.is_some()
    }

    // Param is passed by value, moved
    pub fn set_t_capture(&mut self, v: f64) {
        self.t_capture = ::std::option::Option::Some(v);
    }

    pub fn get_t_capture<'a>(&self) -> f64 {
        self.t_capture.unwrap_or(0.)
    }

    // required double t_sent = 3;

    pub fn clear_t_sent(&mut self) {
        self.t_sent = ::std::option::Option::None;
    }

    pub fn has_t_sent(&self) -> bool {
        self.t_sent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_t_sent(&mut self, v: f64) {
        self.t_sent = ::std::option::Option::Some(v);
    }

    pub fn get_t_sent<'a>(&self) -> f64 {
        self.t_sent.unwrap_or(0.)
    }

    // required uint32 camera_id = 4;

    pub fn clear_camera_id(&mut self) {
        self.camera_id = ::std::option::Option::None;
    }

    pub fn has_camera_id(&self) -> bool {
        self.camera_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_camera_id(&mut self, v: u32) {
        self.camera_id = ::std::option::Option::Some(v);
    }

    pub fn get_camera_id<'a>(&self) -> u32 {
        self.camera_id.unwrap_or(0)
    }

    // repeated .SSL_DetectionBall balls = 5;

    pub fn clear_balls(&mut self) {
        self.balls.clear();
    }

    // Param is passed by value, moved
    pub fn set_balls(&mut self, v: ::protobuf::RepeatedField<SSL_DetectionBall>) {
        self.balls = v;
    }

    // Mutable pointer to the field.
    pub fn mut_balls<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<SSL_DetectionBall> {
        &mut self.balls
    }

    // Take field
    pub fn take_balls(&mut self) -> ::protobuf::RepeatedField<SSL_DetectionBall> {
        ::std::mem::replace(&mut self.balls, ::protobuf::RepeatedField::new())
    }

    pub fn get_balls<'a>(&'a self) -> &'a [SSL_DetectionBall] {
        &self.balls
    }

    // repeated .SSL_DetectionRobot robots_yellow = 6;

    pub fn clear_robots_yellow(&mut self) {
        self.robots_yellow.clear();
    }

    // Param is passed by value, moved
    pub fn set_robots_yellow(&mut self, v: ::protobuf::RepeatedField<SSL_DetectionRobot>) {
        self.robots_yellow = v;
    }

    // Mutable pointer to the field.
    pub fn mut_robots_yellow<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<SSL_DetectionRobot> {
        &mut self.robots_yellow
    }

    // Take field
    pub fn take_robots_yellow(&mut self) -> ::protobuf::RepeatedField<SSL_DetectionRobot> {
        ::std::mem::replace(&mut self.robots_yellow, ::protobuf::RepeatedField::new())
    }

    pub fn get_robots_yellow<'a>(&'a self) -> &'a [SSL_DetectionRobot] {
        &self.robots_yellow
    }

    // repeated .SSL_DetectionRobot robots_blue = 7;

    pub fn clear_robots_blue(&mut self) {
        self.robots_blue.clear();
    }

    // Param is passed by value, moved
    pub fn set_robots_blue(&mut self, v: ::protobuf::RepeatedField<SSL_DetectionRobot>) {
        self.robots_blue = v;
    }

    // Mutable pointer to the field.
    pub fn mut_robots_blue<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<SSL_DetectionRobot> {
        &mut self.robots_blue
    }

    // Take field
    pub fn take_robots_blue(&mut self) -> ::protobuf::RepeatedField<SSL_DetectionRobot> {
        ::std::mem::replace(&mut self.robots_blue, ::protobuf::RepeatedField::new())
    }

    pub fn get_robots_blue<'a>(&'a self) -> &'a [SSL_DetectionRobot] {
        &self.robots_blue
    }
}

impl ::protobuf::Message for SSL_DetectionFrame {
    fn is_initialized(&self) -> bool {
        if self.frame_number.is_none() {
            return false;
        };
        if self.t_capture.is_none() {
            return false;
        };
        if self.t_sent.is_none() {
            return false;
        };
        if self.camera_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.frame_number = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.t_capture = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.t_sent = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.camera_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.balls));
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.robots_yellow));
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.robots_blue));
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
        for value in self.frame_number.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.t_capture.is_some() {
            my_size += 9;
        };
        if self.t_sent.is_some() {
            my_size += 9;
        };
        for value in self.camera_id.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.balls.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.robots_yellow.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.robots_blue.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.frame_number {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.t_capture {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.t_sent {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.camera_id {
            try!(os.write_uint32(4, v));
        };
        for v in self.balls.iter() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.robots_yellow.iter() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.robots_blue.iter() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<SSL_DetectionFrame>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SSL_DetectionFrame {
    fn new() -> SSL_DetectionFrame {
        SSL_DetectionFrame::new()
    }

    fn descriptor_static(_: ::std::option::Option<SSL_DetectionFrame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "frame_number",
                    SSL_DetectionFrame::has_frame_number,
                    SSL_DetectionFrame::get_frame_number,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "t_capture",
                    SSL_DetectionFrame::has_t_capture,
                    SSL_DetectionFrame::get_t_capture,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "t_sent",
                    SSL_DetectionFrame::has_t_sent,
                    SSL_DetectionFrame::get_t_sent,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "camera_id",
                    SSL_DetectionFrame::has_camera_id,
                    SSL_DetectionFrame::get_camera_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "balls",
                    SSL_DetectionFrame::get_balls,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "robots_yellow",
                    SSL_DetectionFrame::get_robots_yellow,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "robots_blue",
                    SSL_DetectionFrame::get_robots_blue,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SSL_DetectionFrame>(
                    "SSL_DetectionFrame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SSL_DetectionFrame {
    fn clear(&mut self) {
        self.clear_frame_number();
        self.clear_t_capture();
        self.clear_t_sent();
        self.clear_camera_id();
        self.clear_balls();
        self.clear_robots_yellow();
        self.clear_robots_blue();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SSL_DetectionFrame {
    fn eq(&self, other: &SSL_DetectionFrame) -> bool {
        self.frame_number == other.frame_number &&
        self.t_capture == other.t_capture &&
        self.t_sent == other.t_sent &&
        self.camera_id == other.camera_id &&
        self.balls == other.balls &&
        self.robots_yellow == other.robots_yellow &&
        self.robots_blue == other.robots_blue &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SSL_DetectionFrame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x24, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x5f, 0x72, 0x6f, 0x62, 0x6f, 0x63,
    0x75, 0x70, 0x5f, 0x73, 0x73, 0x6c, 0x5f, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x78, 0x0a, 0x11, 0x53, 0x53, 0x4c, 0x5f, 0x44, 0x65,
    0x74, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x42, 0x61, 0x6c, 0x6c, 0x12, 0x12, 0x0a, 0x0a, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x64, 0x65, 0x6e, 0x63, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x02, 0x12,
    0x0c, 0x0a, 0x04, 0x61, 0x72, 0x65, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x09, 0x0a,
    0x01, 0x78, 0x18, 0x03, 0x20, 0x02, 0x28, 0x02, 0x12, 0x09, 0x0a, 0x01, 0x79, 0x18, 0x04, 0x20,
    0x02, 0x28, 0x02, 0x12, 0x09, 0x0a, 0x01, 0x7a, 0x18, 0x05, 0x20, 0x01, 0x28, 0x02, 0x12, 0x0f,
    0x0a, 0x07, 0x70, 0x69, 0x78, 0x65, 0x6c, 0x5f, 0x78, 0x18, 0x06, 0x20, 0x02, 0x28, 0x02, 0x12,
    0x0f, 0x0a, 0x07, 0x70, 0x69, 0x78, 0x65, 0x6c, 0x5f, 0x79, 0x18, 0x07, 0x20, 0x02, 0x28, 0x02,
    0x22, 0x97, 0x01, 0x0a, 0x12, 0x53, 0x53, 0x4c, 0x5f, 0x44, 0x65, 0x74, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x52, 0x6f, 0x62, 0x6f, 0x74, 0x12, 0x12, 0x0a, 0x0a, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x64, 0x65, 0x6e, 0x63, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x02, 0x12, 0x10, 0x0a, 0x08, 0x72,
    0x6f, 0x62, 0x6f, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x09, 0x0a,
    0x01, 0x78, 0x18, 0x03, 0x20, 0x02, 0x28, 0x02, 0x12, 0x09, 0x0a, 0x01, 0x79, 0x18, 0x04, 0x20,
    0x02, 0x28, 0x02, 0x12, 0x13, 0x0a, 0x0b, 0x6f, 0x72, 0x69, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x02, 0x12, 0x0f, 0x0a, 0x07, 0x70, 0x69, 0x78, 0x65,
    0x6c, 0x5f, 0x78, 0x18, 0x06, 0x20, 0x02, 0x28, 0x02, 0x12, 0x0f, 0x0a, 0x07, 0x70, 0x69, 0x78,
    0x65, 0x6c, 0x5f, 0x79, 0x18, 0x07, 0x20, 0x02, 0x28, 0x02, 0x12, 0x0e, 0x0a, 0x06, 0x68, 0x65,
    0x69, 0x67, 0x68, 0x74, 0x18, 0x08, 0x20, 0x01, 0x28, 0x02, 0x22, 0xd9, 0x01, 0x0a, 0x12, 0x53,
    0x53, 0x4c, 0x5f, 0x44, 0x65, 0x74, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x46, 0x72, 0x61, 0x6d,
    0x65, 0x12, 0x14, 0x0a, 0x0c, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x5f, 0x6e, 0x75, 0x6d, 0x62, 0x65,
    0x72, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x11, 0x0a, 0x09, 0x74, 0x5f, 0x63, 0x61, 0x70,
    0x74, 0x75, 0x72, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x01, 0x12, 0x0e, 0x0a, 0x06, 0x74, 0x5f,
    0x73, 0x65, 0x6e, 0x74, 0x18, 0x03, 0x20, 0x02, 0x28, 0x01, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x61,
    0x6d, 0x65, 0x72, 0x61, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x21, 0x0a,
    0x05, 0x62, 0x61, 0x6c, 0x6c, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x53,
    0x53, 0x4c, 0x5f, 0x44, 0x65, 0x74, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x42, 0x61, 0x6c, 0x6c,
    0x12, 0x2a, 0x0a, 0x0d, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x73, 0x5f, 0x79, 0x65, 0x6c, 0x6c, 0x6f,
    0x77, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x53, 0x53, 0x4c, 0x5f, 0x44, 0x65,
    0x74, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x6f, 0x62, 0x6f, 0x74, 0x12, 0x28, 0x0a, 0x0b,
    0x72, 0x6f, 0x62, 0x6f, 0x74, 0x73, 0x5f, 0x62, 0x6c, 0x75, 0x65, 0x18, 0x07, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x13, 0x2e, 0x53, 0x53, 0x4c, 0x5f, 0x44, 0x65, 0x74, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x52, 0x6f, 0x62, 0x6f, 0x74, 0x4a, 0xbe, 0x0c, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1d,
    0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x00, 0x00, 0x08, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x00, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x01, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x01, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x01,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x01, 0x12, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x01, 0x1f, 0x20, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x02, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x02, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x02, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x02, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x02, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x03, 0x02, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x03, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x03, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x03, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x03, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x04, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x04, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x04, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x12, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x04, 0x1f, 0x20, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x05, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x04, 0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x05, 0x12, 0x03, 0x05, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x05, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x05,
    0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x06, 0x02, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x06, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x06, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x03, 0x12, 0x03, 0x06, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12,
    0x03, 0x07, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x07,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x07, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x07, 0x12, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x07, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x01, 0x12, 0x04, 0x0a, 0x00, 0x13, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12,
    0x03, 0x0a, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x01, 0x12, 0x03, 0x0c, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0c,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x21, 0x22, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x0d, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x0d, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x0e, 0x02, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0e, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0e, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x0e, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04,
    0x12, 0x03, 0x0f, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03,
    0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0f, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0f, 0x21, 0x22, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x10, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x05, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05,
    0x05, 0x12, 0x03, 0x10, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x10, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x10,
    0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x11, 0x02, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x06, 0x05, 0x12, 0x03, 0x11, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x11, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x06, 0x03, 0x12, 0x03, 0x11, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x07, 0x12,
    0x03, 0x12, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x04, 0x12, 0x03, 0x12,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x05, 0x12, 0x03, 0x12, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x01, 0x12, 0x03, 0x12, 0x12, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x03, 0x12, 0x03, 0x12, 0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x02, 0x12, 0x04, 0x15, 0x00, 0x1d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x15, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x16, 0x02,
    0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x16, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x16, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x1e, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x17, 0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x17, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x17,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x17, 0x1e, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x17, 0x2e, 0x2f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x18, 0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x18, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x18, 0x1e, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x18, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x19, 0x02, 0x30,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x19, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x19, 0x1e, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x19, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x1a, 0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x03,
    0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x06, 0x12, 0x03, 0x1a, 0x0b,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1a, 0x1e, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1a, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1b, 0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x05, 0x04, 0x12, 0x03, 0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05,
    0x06, 0x12, 0x03, 0x1b, 0x0b, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x1b, 0x1e, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x1b,
    0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x1c, 0x02, 0x30, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x04, 0x12, 0x03, 0x1c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x06, 0x06, 0x12, 0x03, 0x1c, 0x0b, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1c, 0x1e, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x06, 0x03, 0x12, 0x03, 0x1c, 0x2e, 0x2f,
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
