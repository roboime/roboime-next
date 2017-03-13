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
pub struct grSim_Robot_Command {
    // message fields
    id: ::std::option::Option<u32>,
    kickspeedx: ::std::option::Option<f32>,
    kickspeedz: ::std::option::Option<f32>,
    veltangent: ::std::option::Option<f32>,
    velnormal: ::std::option::Option<f32>,
    velangular: ::std::option::Option<f32>,
    spinner: ::std::option::Option<bool>,
    wheelsspeed: ::std::option::Option<bool>,
    wheel1: ::std::option::Option<f32>,
    wheel2: ::std::option::Option<f32>,
    wheel3: ::std::option::Option<f32>,
    wheel4: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for grSim_Robot_Command {}

impl grSim_Robot_Command {
    pub fn new() -> grSim_Robot_Command {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static grSim_Robot_Command {
        static mut instance: ::protobuf::lazy::Lazy<grSim_Robot_Command> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const grSim_Robot_Command,
        };
        unsafe {
            instance.get(|| {
                grSim_Robot_Command {
                    id: ::std::option::Option::None,
                    kickspeedx: ::std::option::Option::None,
                    kickspeedz: ::std::option::Option::None,
                    veltangent: ::std::option::Option::None,
                    velnormal: ::std::option::Option::None,
                    velangular: ::std::option::Option::None,
                    spinner: ::std::option::Option::None,
                    wheelsspeed: ::std::option::Option::None,
                    wheel1: ::std::option::Option::None,
                    wheel2: ::std::option::Option::None,
                    wheel3: ::std::option::Option::None,
                    wheel4: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 id = 1;

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

    // required float kickspeedx = 2;

    pub fn clear_kickspeedx(&mut self) {
        self.kickspeedx = ::std::option::Option::None;
    }

    pub fn has_kickspeedx(&self) -> bool {
        self.kickspeedx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kickspeedx(&mut self, v: f32) {
        self.kickspeedx = ::std::option::Option::Some(v);
    }

    pub fn get_kickspeedx(&self) -> f32 {
        self.kickspeedx.unwrap_or(0.)
    }

    // required float kickspeedz = 3;

    pub fn clear_kickspeedz(&mut self) {
        self.kickspeedz = ::std::option::Option::None;
    }

    pub fn has_kickspeedz(&self) -> bool {
        self.kickspeedz.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kickspeedz(&mut self, v: f32) {
        self.kickspeedz = ::std::option::Option::Some(v);
    }

    pub fn get_kickspeedz(&self) -> f32 {
        self.kickspeedz.unwrap_or(0.)
    }

    // required float veltangent = 4;

    pub fn clear_veltangent(&mut self) {
        self.veltangent = ::std::option::Option::None;
    }

    pub fn has_veltangent(&self) -> bool {
        self.veltangent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_veltangent(&mut self, v: f32) {
        self.veltangent = ::std::option::Option::Some(v);
    }

    pub fn get_veltangent(&self) -> f32 {
        self.veltangent.unwrap_or(0.)
    }

    // required float velnormal = 5;

    pub fn clear_velnormal(&mut self) {
        self.velnormal = ::std::option::Option::None;
    }

    pub fn has_velnormal(&self) -> bool {
        self.velnormal.is_some()
    }

    // Param is passed by value, moved
    pub fn set_velnormal(&mut self, v: f32) {
        self.velnormal = ::std::option::Option::Some(v);
    }

    pub fn get_velnormal(&self) -> f32 {
        self.velnormal.unwrap_or(0.)
    }

    // required float velangular = 6;

    pub fn clear_velangular(&mut self) {
        self.velangular = ::std::option::Option::None;
    }

    pub fn has_velangular(&self) -> bool {
        self.velangular.is_some()
    }

    // Param is passed by value, moved
    pub fn set_velangular(&mut self, v: f32) {
        self.velangular = ::std::option::Option::Some(v);
    }

    pub fn get_velangular(&self) -> f32 {
        self.velangular.unwrap_or(0.)
    }

    // required bool spinner = 7;

    pub fn clear_spinner(&mut self) {
        self.spinner = ::std::option::Option::None;
    }

    pub fn has_spinner(&self) -> bool {
        self.spinner.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spinner(&mut self, v: bool) {
        self.spinner = ::std::option::Option::Some(v);
    }

    pub fn get_spinner(&self) -> bool {
        self.spinner.unwrap_or(false)
    }

    // required bool wheelsspeed = 8;

    pub fn clear_wheelsspeed(&mut self) {
        self.wheelsspeed = ::std::option::Option::None;
    }

    pub fn has_wheelsspeed(&self) -> bool {
        self.wheelsspeed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wheelsspeed(&mut self, v: bool) {
        self.wheelsspeed = ::std::option::Option::Some(v);
    }

    pub fn get_wheelsspeed(&self) -> bool {
        self.wheelsspeed.unwrap_or(false)
    }

    // optional float wheel1 = 9;

    pub fn clear_wheel1(&mut self) {
        self.wheel1 = ::std::option::Option::None;
    }

    pub fn has_wheel1(&self) -> bool {
        self.wheel1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wheel1(&mut self, v: f32) {
        self.wheel1 = ::std::option::Option::Some(v);
    }

    pub fn get_wheel1(&self) -> f32 {
        self.wheel1.unwrap_or(0.)
    }

    // optional float wheel2 = 10;

    pub fn clear_wheel2(&mut self) {
        self.wheel2 = ::std::option::Option::None;
    }

    pub fn has_wheel2(&self) -> bool {
        self.wheel2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wheel2(&mut self, v: f32) {
        self.wheel2 = ::std::option::Option::Some(v);
    }

    pub fn get_wheel2(&self) -> f32 {
        self.wheel2.unwrap_or(0.)
    }

    // optional float wheel3 = 11;

    pub fn clear_wheel3(&mut self) {
        self.wheel3 = ::std::option::Option::None;
    }

    pub fn has_wheel3(&self) -> bool {
        self.wheel3.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wheel3(&mut self, v: f32) {
        self.wheel3 = ::std::option::Option::Some(v);
    }

    pub fn get_wheel3(&self) -> f32 {
        self.wheel3.unwrap_or(0.)
    }

    // optional float wheel4 = 12;

    pub fn clear_wheel4(&mut self) {
        self.wheel4 = ::std::option::Option::None;
    }

    pub fn has_wheel4(&self) -> bool {
        self.wheel4.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wheel4(&mut self, v: f32) {
        self.wheel4 = ::std::option::Option::Some(v);
    }

    pub fn get_wheel4(&self) -> f32 {
        self.wheel4.unwrap_or(0.)
    }
}

impl ::protobuf::Message for grSim_Robot_Command {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        if self.kickspeedx.is_none() {
            return false;
        };
        if self.kickspeedz.is_none() {
            return false;
        };
        if self.veltangent.is_none() {
            return false;
        };
        if self.velnormal.is_none() {
            return false;
        };
        if self.velangular.is_none() {
            return false;
        };
        if self.spinner.is_none() {
            return false;
        };
        if self.wheelsspeed.is_none() {
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
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.kickspeedx = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.kickspeedz = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.veltangent = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.velnormal = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.velangular = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.spinner = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.wheelsspeed = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.wheel1 = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.wheel2 = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.wheel3 = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.wheel4 = ::std::option::Option::Some(tmp);
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
        for value in self.id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.kickspeedx.is_some() {
            my_size += 5;
        };
        if self.kickspeedz.is_some() {
            my_size += 5;
        };
        if self.veltangent.is_some() {
            my_size += 5;
        };
        if self.velnormal.is_some() {
            my_size += 5;
        };
        if self.velangular.is_some() {
            my_size += 5;
        };
        if self.spinner.is_some() {
            my_size += 2;
        };
        if self.wheelsspeed.is_some() {
            my_size += 2;
        };
        if self.wheel1.is_some() {
            my_size += 5;
        };
        if self.wheel2.is_some() {
            my_size += 5;
        };
        if self.wheel3.is_some() {
            my_size += 5;
        };
        if self.wheel4.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.kickspeedx {
            try!(os.write_float(2, v));
        };
        if let Some(v) = self.kickspeedz {
            try!(os.write_float(3, v));
        };
        if let Some(v) = self.veltangent {
            try!(os.write_float(4, v));
        };
        if let Some(v) = self.velnormal {
            try!(os.write_float(5, v));
        };
        if let Some(v) = self.velangular {
            try!(os.write_float(6, v));
        };
        if let Some(v) = self.spinner {
            try!(os.write_bool(7, v));
        };
        if let Some(v) = self.wheelsspeed {
            try!(os.write_bool(8, v));
        };
        if let Some(v) = self.wheel1 {
            try!(os.write_float(9, v));
        };
        if let Some(v) = self.wheel2 {
            try!(os.write_float(10, v));
        };
        if let Some(v) = self.wheel3 {
            try!(os.write_float(11, v));
        };
        if let Some(v) = self.wheel4 {
            try!(os.write_float(12, v));
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
        ::std::any::TypeId::of::<grSim_Robot_Command>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for grSim_Robot_Command {
    fn new() -> grSim_Robot_Command {
        grSim_Robot_Command::new()
    }

    fn descriptor_static(_: ::std::option::Option<grSim_Robot_Command>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "id",
                    grSim_Robot_Command::has_id,
                    grSim_Robot_Command::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "kickspeedx",
                    grSim_Robot_Command::has_kickspeedx,
                    grSim_Robot_Command::get_kickspeedx,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "kickspeedz",
                    grSim_Robot_Command::has_kickspeedz,
                    grSim_Robot_Command::get_kickspeedz,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "veltangent",
                    grSim_Robot_Command::has_veltangent,
                    grSim_Robot_Command::get_veltangent,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "velnormal",
                    grSim_Robot_Command::has_velnormal,
                    grSim_Robot_Command::get_velnormal,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "velangular",
                    grSim_Robot_Command::has_velangular,
                    grSim_Robot_Command::get_velangular,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "spinner",
                    grSim_Robot_Command::has_spinner,
                    grSim_Robot_Command::get_spinner,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "wheelsspeed",
                    grSim_Robot_Command::has_wheelsspeed,
                    grSim_Robot_Command::get_wheelsspeed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "wheel1",
                    grSim_Robot_Command::has_wheel1,
                    grSim_Robot_Command::get_wheel1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "wheel2",
                    grSim_Robot_Command::has_wheel2,
                    grSim_Robot_Command::get_wheel2,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "wheel3",
                    grSim_Robot_Command::has_wheel3,
                    grSim_Robot_Command::get_wheel3,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "wheel4",
                    grSim_Robot_Command::has_wheel4,
                    grSim_Robot_Command::get_wheel4,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<grSim_Robot_Command>(
                    "grSim_Robot_Command",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for grSim_Robot_Command {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_kickspeedx();
        self.clear_kickspeedz();
        self.clear_veltangent();
        self.clear_velnormal();
        self.clear_velangular();
        self.clear_spinner();
        self.clear_wheelsspeed();
        self.clear_wheel1();
        self.clear_wheel2();
        self.clear_wheel3();
        self.clear_wheel4();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for grSim_Robot_Command {
    fn eq(&self, other: &grSim_Robot_Command) -> bool {
        self.id == other.id &&
        self.kickspeedx == other.kickspeedx &&
        self.kickspeedz == other.kickspeedz &&
        self.veltangent == other.veltangent &&
        self.velnormal == other.velnormal &&
        self.velangular == other.velangular &&
        self.spinner == other.spinner &&
        self.wheelsspeed == other.wheelsspeed &&
        self.wheel1 == other.wheel1 &&
        self.wheel2 == other.wheel2 &&
        self.wheel3 == other.wheel3 &&
        self.wheel4 == other.wheel4 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for grSim_Robot_Command {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct grSim_Commands {
    // message fields
    timestamp: ::std::option::Option<f64>,
    isteamyellow: ::std::option::Option<bool>,
    robot_commands: ::protobuf::RepeatedField<grSim_Robot_Command>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for grSim_Commands {}

impl grSim_Commands {
    pub fn new() -> grSim_Commands {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static grSim_Commands {
        static mut instance: ::protobuf::lazy::Lazy<grSim_Commands> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const grSim_Commands,
        };
        unsafe {
            instance.get(|| {
                grSim_Commands {
                    timestamp: ::std::option::Option::None,
                    isteamyellow: ::std::option::Option::None,
                    robot_commands: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required double timestamp = 1;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: f64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> f64 {
        self.timestamp.unwrap_or(0.)
    }

    // required bool isteamyellow = 2;

    pub fn clear_isteamyellow(&mut self) {
        self.isteamyellow = ::std::option::Option::None;
    }

    pub fn has_isteamyellow(&self) -> bool {
        self.isteamyellow.is_some()
    }

    // Param is passed by value, moved
    pub fn set_isteamyellow(&mut self, v: bool) {
        self.isteamyellow = ::std::option::Option::Some(v);
    }

    pub fn get_isteamyellow(&self) -> bool {
        self.isteamyellow.unwrap_or(false)
    }

    // repeated .grSim_Robot_Command robot_commands = 3;

    pub fn clear_robot_commands(&mut self) {
        self.robot_commands.clear();
    }

    // Param is passed by value, moved
    pub fn set_robot_commands(&mut self, v: ::protobuf::RepeatedField<grSim_Robot_Command>) {
        self.robot_commands = v;
    }

    // Mutable pointer to the field.
    pub fn mut_robot_commands(&mut self) -> &mut ::protobuf::RepeatedField<grSim_Robot_Command> {
        &mut self.robot_commands
    }

    // Take field
    pub fn take_robot_commands(&mut self) -> ::protobuf::RepeatedField<grSim_Robot_Command> {
        ::std::mem::replace(&mut self.robot_commands, ::protobuf::RepeatedField::new())
    }

    pub fn get_robot_commands(&self) -> &[grSim_Robot_Command] {
        &self.robot_commands
    }
}

impl ::protobuf::Message for grSim_Commands {
    fn is_initialized(&self) -> bool {
        if self.timestamp.is_none() {
            return false;
        };
        if self.isteamyellow.is_none() {
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
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.isteamyellow = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.robot_commands));
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
        if self.timestamp.is_some() {
            my_size += 9;
        };
        if self.isteamyellow.is_some() {
            my_size += 2;
        };
        for value in self.robot_commands.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.timestamp {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.isteamyellow {
            try!(os.write_bool(2, v));
        };
        for v in self.robot_commands.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<grSim_Commands>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for grSim_Commands {
    fn new() -> grSim_Commands {
        grSim_Commands::new()
    }

    fn descriptor_static(_: ::std::option::Option<grSim_Commands>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "timestamp",
                    grSim_Commands::has_timestamp,
                    grSim_Commands::get_timestamp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "isteamyellow",
                    grSim_Commands::has_isteamyellow,
                    grSim_Commands::get_isteamyellow,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "robot_commands",
                    grSim_Commands::get_robot_commands,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<grSim_Commands>(
                    "grSim_Commands",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for grSim_Commands {
    fn clear(&mut self) {
        self.clear_timestamp();
        self.clear_isteamyellow();
        self.clear_robot_commands();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for grSim_Commands {
    fn eq(&self, other: &grSim_Commands) -> bool {
        self.timestamp == other.timestamp &&
        self.isteamyellow == other.isteamyellow &&
        self.robot_commands == other.robot_commands &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for grSim_Commands {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x14, 0x67, 0x72, 0x53, 0x69, 0x6d, 0x5f, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x73,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xea, 0x01, 0x0a, 0x13, 0x67, 0x72, 0x53, 0x69, 0x6d,
    0x5f, 0x52, 0x6f, 0x62, 0x6f, 0x74, 0x5f, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x12, 0x0a,
    0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x12, 0x0a, 0x0a, 0x6b, 0x69,
    0x63, 0x6b, 0x73, 0x70, 0x65, 0x65, 0x64, 0x78, 0x18, 0x02, 0x20, 0x02, 0x28, 0x02, 0x12, 0x12,
    0x0a, 0x0a, 0x6b, 0x69, 0x63, 0x6b, 0x73, 0x70, 0x65, 0x65, 0x64, 0x7a, 0x18, 0x03, 0x20, 0x02,
    0x28, 0x02, 0x12, 0x12, 0x0a, 0x0a, 0x76, 0x65, 0x6c, 0x74, 0x61, 0x6e, 0x67, 0x65, 0x6e, 0x74,
    0x18, 0x04, 0x20, 0x02, 0x28, 0x02, 0x12, 0x11, 0x0a, 0x09, 0x76, 0x65, 0x6c, 0x6e, 0x6f, 0x72,
    0x6d, 0x61, 0x6c, 0x18, 0x05, 0x20, 0x02, 0x28, 0x02, 0x12, 0x12, 0x0a, 0x0a, 0x76, 0x65, 0x6c,
    0x61, 0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x18, 0x06, 0x20, 0x02, 0x28, 0x02, 0x12, 0x0f, 0x0a,
    0x07, 0x73, 0x70, 0x69, 0x6e, 0x6e, 0x65, 0x72, 0x18, 0x07, 0x20, 0x02, 0x28, 0x08, 0x12, 0x13,
    0x0a, 0x0b, 0x77, 0x68, 0x65, 0x65, 0x6c, 0x73, 0x73, 0x70, 0x65, 0x65, 0x64, 0x18, 0x08, 0x20,
    0x02, 0x28, 0x08, 0x12, 0x0e, 0x0a, 0x06, 0x77, 0x68, 0x65, 0x65, 0x6c, 0x31, 0x18, 0x09, 0x20,
    0x01, 0x28, 0x02, 0x12, 0x0e, 0x0a, 0x06, 0x77, 0x68, 0x65, 0x65, 0x6c, 0x32, 0x18, 0x0a, 0x20,
    0x01, 0x28, 0x02, 0x12, 0x0e, 0x0a, 0x06, 0x77, 0x68, 0x65, 0x65, 0x6c, 0x33, 0x18, 0x0b, 0x20,
    0x01, 0x28, 0x02, 0x12, 0x0e, 0x0a, 0x06, 0x77, 0x68, 0x65, 0x65, 0x6c, 0x34, 0x18, 0x0c, 0x20,
    0x01, 0x28, 0x02, 0x22, 0x67, 0x0a, 0x0e, 0x67, 0x72, 0x53, 0x69, 0x6d, 0x5f, 0x43, 0x6f, 0x6d,
    0x6d, 0x61, 0x6e, 0x64, 0x73, 0x12, 0x11, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61,
    0x6d, 0x70, 0x18, 0x01, 0x20, 0x02, 0x28, 0x01, 0x12, 0x14, 0x0a, 0x0c, 0x69, 0x73, 0x74, 0x65,
    0x61, 0x6d, 0x79, 0x65, 0x6c, 0x6c, 0x6f, 0x77, 0x18, 0x02, 0x20, 0x02, 0x28, 0x08, 0x12, 0x2c,
    0x0a, 0x0e, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x73,
    0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x67, 0x72, 0x53, 0x69, 0x6d, 0x5f, 0x52,
    0x6f, 0x62, 0x6f, 0x74, 0x5f, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x4a, 0xc3, 0x08, 0x0a,
    0x06, 0x12, 0x04, 0x00, 0x00, 0x13, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x00,
    0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x00, 0x08, 0x1b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x01, 0x00, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x01, 0x00, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x01, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x01, 0x10, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x01, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x02, 0x00,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x02, 0x00, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x02, 0x09, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x02, 0x0f, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x02, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x03, 0x00, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x03, 0x00, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x03,
    0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x03, 0x0f, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x03, 0x1c, 0x1d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x04, 0x00, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x04, 0x00, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x04, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x04, 0x0f, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x04, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x05, 0x00, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x05, 0x00, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x05, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x05, 0x0f, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x05, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05,
    0x12, 0x03, 0x06, 0x00, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03,
    0x06, 0x00, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x06, 0x09,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x06, 0x0f, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x06, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x07, 0x00, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x06, 0x04, 0x12, 0x03, 0x07, 0x00, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06,
    0x05, 0x12, 0x03, 0x07, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x07, 0x0e, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x07,
    0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x08, 0x00, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x08, 0x00, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x08, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x08, 0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x07, 0x03, 0x12, 0x03, 0x08, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12,
    0x03, 0x09, 0x00, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x09,
    0x00, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x09, 0x09, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x09, 0x0f, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x09, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x0a, 0x00, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x09, 0x04, 0x12, 0x03, 0x0a, 0x00, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05,
    0x12, 0x03, 0x0a, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03,
    0x0a, 0x0f, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x0a, 0x18,
    0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x0b, 0x00, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x0b, 0x00, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x0b, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0a, 0x01, 0x12, 0x03, 0x0b, 0x0f, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a,
    0x03, 0x12, 0x03, 0x0b, 0x18, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03,
    0x0c, 0x00, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x0c, 0x00,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x0c, 0x09, 0x0e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x0c, 0x0f, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x0c, 0x18, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x0f, 0x00, 0x13, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x0f, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x10, 0x00, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x10, 0x00, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x10, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x10, 0x10, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x10, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01,
    0x12, 0x03, 0x11, 0x00, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x11, 0x00, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x11, 0x09,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x11, 0x0e, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x11, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x12, 0x00, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x12, 0x00, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x06, 0x12, 0x03, 0x12, 0x09, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x12, 0x1d, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x12,
    0x2e, 0x2f,
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
