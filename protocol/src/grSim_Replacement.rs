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
pub struct grSim_RobotReplacement {
    // message fields
    x: ::std::option::Option<f64>,
    y: ::std::option::Option<f64>,
    dir: ::std::option::Option<f64>,
    id: ::std::option::Option<u32>,
    yellowteam: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for grSim_RobotReplacement {}

impl grSim_RobotReplacement {
    pub fn new() -> grSim_RobotReplacement {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static grSim_RobotReplacement {
        static mut instance: ::protobuf::lazy::Lazy<grSim_RobotReplacement> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const grSim_RobotReplacement,
        };
        unsafe {
            instance.get(|| {
                grSim_RobotReplacement {
                    x: ::std::option::Option::None,
                    y: ::std::option::Option::None,
                    dir: ::std::option::Option::None,
                    id: ::std::option::Option::None,
                    yellowteam: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required double x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f64) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> f64 {
        self.x.unwrap_or(0.)
    }

    // required double y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f64) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> f64 {
        self.y.unwrap_or(0.)
    }

    // required double dir = 3;

    pub fn clear_dir(&mut self) {
        self.dir = ::std::option::Option::None;
    }

    pub fn has_dir(&self) -> bool {
        self.dir.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dir(&mut self, v: f64) {
        self.dir = ::std::option::Option::Some(v);
    }

    pub fn get_dir(&self) -> f64 {
        self.dir.unwrap_or(0.)
    }

    // required uint32 id = 4;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u32 {
        self.id.unwrap_or(0)
    }

    // required bool yellowteam = 5;

    pub fn clear_yellowteam(&mut self) {
        self.yellowteam = ::std::option::Option::None;
    }

    pub fn has_yellowteam(&self) -> bool {
        self.yellowteam.is_some()
    }

    // Param is passed by value, moved
    pub fn set_yellowteam(&mut self, v: bool) {
        self.yellowteam = ::std::option::Option::Some(v);
    }

    pub fn get_yellowteam(&self) -> bool {
        self.yellowteam.unwrap_or(false)
    }
}

impl ::protobuf::Message for grSim_RobotReplacement {
    fn is_initialized(&self) -> bool {
        if self.x.is_none() {
            return false;
        };
        if self.y.is_none() {
            return false;
        };
        if self.dir.is_none() {
            return false;
        };
        if self.id.is_none() {
            return false;
        };
        if self.yellowteam.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.dir = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.yellowteam = ::std::option::Option::Some(tmp);
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
        if self.x.is_some() {
            my_size += 9;
        };
        if self.y.is_some() {
            my_size += 9;
        };
        if self.dir.is_some() {
            my_size += 9;
        };
        for value in self.id.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.yellowteam.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.y {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.dir {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.id {
            try!(os.write_uint32(4, v));
        };
        if let Some(v) = self.yellowteam {
            try!(os.write_bool(5, v));
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
        ::std::any::TypeId::of::<grSim_RobotReplacement>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for grSim_RobotReplacement {
    fn new() -> grSim_RobotReplacement {
        grSim_RobotReplacement::new()
    }

    fn descriptor_static(_: ::std::option::Option<grSim_RobotReplacement>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "x",
                    grSim_RobotReplacement::has_x,
                    grSim_RobotReplacement::get_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "y",
                    grSim_RobotReplacement::has_y,
                    grSim_RobotReplacement::get_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "dir",
                    grSim_RobotReplacement::has_dir,
                    grSim_RobotReplacement::get_dir,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "id",
                    grSim_RobotReplacement::has_id,
                    grSim_RobotReplacement::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "yellowteam",
                    grSim_RobotReplacement::has_yellowteam,
                    grSim_RobotReplacement::get_yellowteam,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<grSim_RobotReplacement>(
                    "grSim_RobotReplacement",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for grSim_RobotReplacement {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_dir();
        self.clear_id();
        self.clear_yellowteam();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for grSim_RobotReplacement {
    fn eq(&self, other: &grSim_RobotReplacement) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.dir == other.dir &&
        self.id == other.id &&
        self.yellowteam == other.yellowteam &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for grSim_RobotReplacement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct grSim_BallReplacement {
    // message fields
    x: ::std::option::Option<f64>,
    y: ::std::option::Option<f64>,
    vx: ::std::option::Option<f64>,
    vy: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for grSim_BallReplacement {}

impl grSim_BallReplacement {
    pub fn new() -> grSim_BallReplacement {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static grSim_BallReplacement {
        static mut instance: ::protobuf::lazy::Lazy<grSim_BallReplacement> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const grSim_BallReplacement,
        };
        unsafe {
            instance.get(|| {
                grSim_BallReplacement {
                    x: ::std::option::Option::None,
                    y: ::std::option::Option::None,
                    vx: ::std::option::Option::None,
                    vy: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required double x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f64) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> f64 {
        self.x.unwrap_or(0.)
    }

    // required double y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f64) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> f64 {
        self.y.unwrap_or(0.)
    }

    // required double vx = 3;

    pub fn clear_vx(&mut self) {
        self.vx = ::std::option::Option::None;
    }

    pub fn has_vx(&self) -> bool {
        self.vx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vx(&mut self, v: f64) {
        self.vx = ::std::option::Option::Some(v);
    }

    pub fn get_vx(&self) -> f64 {
        self.vx.unwrap_or(0.)
    }

    // required double vy = 4;

    pub fn clear_vy(&mut self) {
        self.vy = ::std::option::Option::None;
    }

    pub fn has_vy(&self) -> bool {
        self.vy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vy(&mut self, v: f64) {
        self.vy = ::std::option::Option::Some(v);
    }

    pub fn get_vy(&self) -> f64 {
        self.vy.unwrap_or(0.)
    }
}

impl ::protobuf::Message for grSim_BallReplacement {
    fn is_initialized(&self) -> bool {
        if self.x.is_none() {
            return false;
        };
        if self.y.is_none() {
            return false;
        };
        if self.vx.is_none() {
            return false;
        };
        if self.vy.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.vx = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.vy = ::std::option::Option::Some(tmp);
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
        if self.x.is_some() {
            my_size += 9;
        };
        if self.y.is_some() {
            my_size += 9;
        };
        if self.vx.is_some() {
            my_size += 9;
        };
        if self.vy.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.y {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.vx {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.vy {
            try!(os.write_double(4, v));
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
        ::std::any::TypeId::of::<grSim_BallReplacement>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for grSim_BallReplacement {
    fn new() -> grSim_BallReplacement {
        grSim_BallReplacement::new()
    }

    fn descriptor_static(_: ::std::option::Option<grSim_BallReplacement>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "x",
                    grSim_BallReplacement::has_x,
                    grSim_BallReplacement::get_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "y",
                    grSim_BallReplacement::has_y,
                    grSim_BallReplacement::get_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "vx",
                    grSim_BallReplacement::has_vx,
                    grSim_BallReplacement::get_vx,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "vy",
                    grSim_BallReplacement::has_vy,
                    grSim_BallReplacement::get_vy,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<grSim_BallReplacement>(
                    "grSim_BallReplacement",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for grSim_BallReplacement {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_vx();
        self.clear_vy();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for grSim_BallReplacement {
    fn eq(&self, other: &grSim_BallReplacement) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.vx == other.vx &&
        self.vy == other.vy &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for grSim_BallReplacement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct grSim_Replacement {
    // message fields
    ball: ::protobuf::SingularPtrField<grSim_BallReplacement>,
    robots: ::protobuf::RepeatedField<grSim_RobotReplacement>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for grSim_Replacement {}

impl grSim_Replacement {
    pub fn new() -> grSim_Replacement {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static grSim_Replacement {
        static mut instance: ::protobuf::lazy::Lazy<grSim_Replacement> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const grSim_Replacement,
        };
        unsafe {
            instance.get(|| {
                grSim_Replacement {
                    ball: ::protobuf::SingularPtrField::none(),
                    robots: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .grSim_BallReplacement ball = 1;

    pub fn clear_ball(&mut self) {
        self.ball.clear();
    }

    pub fn has_ball(&self) -> bool {
        self.ball.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ball(&mut self, v: grSim_BallReplacement) {
        self.ball = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ball(&mut self) -> &mut grSim_BallReplacement {
        if self.ball.is_none() {
            self.ball.set_default();
        };
        self.ball.as_mut().unwrap()
    }

    // Take field
    pub fn take_ball(&mut self) -> grSim_BallReplacement {
        self.ball.take().unwrap_or_else(|| grSim_BallReplacement::new())
    }

    pub fn get_ball(&self) -> &grSim_BallReplacement {
        self.ball.as_ref().unwrap_or_else(|| grSim_BallReplacement::default_instance())
    }

    // repeated .grSim_RobotReplacement robots = 2;

    pub fn clear_robots(&mut self) {
        self.robots.clear();
    }

    // Param is passed by value, moved
    pub fn set_robots(&mut self, v: ::protobuf::RepeatedField<grSim_RobotReplacement>) {
        self.robots = v;
    }

    // Mutable pointer to the field.
    pub fn mut_robots(&mut self) -> &mut ::protobuf::RepeatedField<grSim_RobotReplacement> {
        &mut self.robots
    }

    // Take field
    pub fn take_robots(&mut self) -> ::protobuf::RepeatedField<grSim_RobotReplacement> {
        ::std::mem::replace(&mut self.robots, ::protobuf::RepeatedField::new())
    }

    pub fn get_robots(&self) -> &[grSim_RobotReplacement] {
        &self.robots
    }
}

impl ::protobuf::Message for grSim_Replacement {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ball));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.robots));
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
        for value in self.ball.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.robots.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ball.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.robots.iter() {
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
        ::std::any::TypeId::of::<grSim_Replacement>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for grSim_Replacement {
    fn new() -> grSim_Replacement {
        grSim_Replacement::new()
    }

    fn descriptor_static(_: ::std::option::Option<grSim_Replacement>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ball",
                    grSim_Replacement::has_ball,
                    grSim_Replacement::get_ball,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "robots",
                    grSim_Replacement::get_robots,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<grSim_Replacement>(
                    "grSim_Replacement",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for grSim_Replacement {
    fn clear(&mut self) {
        self.clear_ball();
        self.clear_robots();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for grSim_Replacement {
    fn eq(&self, other: &grSim_Replacement) -> bool {
        self.ball == other.ball &&
        self.robots == other.robots &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for grSim_Replacement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x17, 0x67, 0x72, 0x53, 0x69, 0x6d, 0x5f, 0x52, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x6d,
    0x65, 0x6e, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x5b, 0x0a, 0x16, 0x67, 0x72, 0x53,
    0x69, 0x6d, 0x5f, 0x52, 0x6f, 0x62, 0x6f, 0x74, 0x52, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x6d,
    0x65, 0x6e, 0x74, 0x12, 0x09, 0x0a, 0x01, 0x78, 0x18, 0x01, 0x20, 0x02, 0x28, 0x01, 0x12, 0x09,
    0x0a, 0x01, 0x79, 0x18, 0x02, 0x20, 0x02, 0x28, 0x01, 0x12, 0x0b, 0x0a, 0x03, 0x64, 0x69, 0x72,
    0x18, 0x03, 0x20, 0x02, 0x28, 0x01, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x04, 0x20, 0x02,
    0x28, 0x0d, 0x12, 0x12, 0x0a, 0x0a, 0x79, 0x65, 0x6c, 0x6c, 0x6f, 0x77, 0x74, 0x65, 0x61, 0x6d,
    0x18, 0x05, 0x20, 0x02, 0x28, 0x08, 0x22, 0x45, 0x0a, 0x15, 0x67, 0x72, 0x53, 0x69, 0x6d, 0x5f,
    0x42, 0x61, 0x6c, 0x6c, 0x52, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x12,
    0x09, 0x0a, 0x01, 0x78, 0x18, 0x01, 0x20, 0x02, 0x28, 0x01, 0x12, 0x09, 0x0a, 0x01, 0x79, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x01, 0x12, 0x0a, 0x0a, 0x02, 0x76, 0x78, 0x18, 0x03, 0x20, 0x02, 0x28,
    0x01, 0x12, 0x0a, 0x0a, 0x02, 0x76, 0x79, 0x18, 0x04, 0x20, 0x02, 0x28, 0x01, 0x22, 0x62, 0x0a,
    0x11, 0x67, 0x72, 0x53, 0x69, 0x6d, 0x5f, 0x52, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x6d, 0x65,
    0x6e, 0x74, 0x12, 0x24, 0x0a, 0x04, 0x62, 0x61, 0x6c, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x16, 0x2e, 0x67, 0x72, 0x53, 0x69, 0x6d, 0x5f, 0x42, 0x61, 0x6c, 0x6c, 0x52, 0x65, 0x70,
    0x6c, 0x61, 0x63, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x12, 0x27, 0x0a, 0x06, 0x72, 0x6f, 0x62, 0x6f,
    0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x67, 0x72, 0x53, 0x69, 0x6d,
    0x5f, 0x52, 0x6f, 0x62, 0x6f, 0x74, 0x52, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x6d, 0x65, 0x6e,
    0x74, 0x4a, 0xc7, 0x06, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x00, 0x00, 0x06, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x00, 0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x01, 0x00,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x01, 0x00, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x01, 0x09, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x01, 0x10, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x01, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x02, 0x00, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x02, 0x00, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x02,
    0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x02, 0x10, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x02, 0x12, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x03, 0x00, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x03, 0x00, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x03, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x03, 0x10, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x03, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x04, 0x00, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x04, 0x00, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x04, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x10, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x04, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04,
    0x12, 0x03, 0x05, 0x00, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03,
    0x05, 0x00, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x05, 0x09,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x05, 0x0e, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x05, 0x19, 0x1a, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x08, 0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01,
    0x12, 0x03, 0x08, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x09,
    0x00, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x09, 0x00, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x09, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x10, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x01, 0x12, 0x03, 0x0a, 0x00, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x0a, 0x00, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x0a, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x10,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x12, 0x13, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x0b, 0x00, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0b, 0x00, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x0b, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x0b, 0x10, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x0b, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x00,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0c, 0x00, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0c, 0x09, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0c, 0x10, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0c, 0x13, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12,
    0x04, 0x0f, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x08,
    0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x10, 0x00, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x10, 0x00, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x10, 0x09, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x10, 0x1f, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x10, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x11, 0x00, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x11, 0x00,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x11, 0x09, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x11, 0x20, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x11, 0x29, 0x2a,
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
