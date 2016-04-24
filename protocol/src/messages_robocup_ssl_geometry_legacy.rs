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
pub struct SSL_GeometryFieldSize {
    // message fields
    line_width: ::std::option::Option<i32>,
    field_length: ::std::option::Option<i32>,
    field_width: ::std::option::Option<i32>,
    boundary_width: ::std::option::Option<i32>,
    referee_width: ::std::option::Option<i32>,
    goal_width: ::std::option::Option<i32>,
    goal_depth: ::std::option::Option<i32>,
    goal_wall_width: ::std::option::Option<i32>,
    center_circle_radius: ::std::option::Option<i32>,
    defense_radius: ::std::option::Option<i32>,
    defense_stretch: ::std::option::Option<i32>,
    free_kick_from_defense_dist: ::std::option::Option<i32>,
    penalty_spot_from_field_line_dist: ::std::option::Option<i32>,
    penalty_line_from_spot_dist: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SSL_GeometryFieldSize {}

impl SSL_GeometryFieldSize {
    pub fn new() -> SSL_GeometryFieldSize {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SSL_GeometryFieldSize {
        static mut instance: ::protobuf::lazy::Lazy<SSL_GeometryFieldSize> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SSL_GeometryFieldSize,
        };
        unsafe {
            instance.get(|| {
                SSL_GeometryFieldSize {
                    line_width: ::std::option::Option::None,
                    field_length: ::std::option::Option::None,
                    field_width: ::std::option::Option::None,
                    boundary_width: ::std::option::Option::None,
                    referee_width: ::std::option::Option::None,
                    goal_width: ::std::option::Option::None,
                    goal_depth: ::std::option::Option::None,
                    goal_wall_width: ::std::option::Option::None,
                    center_circle_radius: ::std::option::Option::None,
                    defense_radius: ::std::option::Option::None,
                    defense_stretch: ::std::option::Option::None,
                    free_kick_from_defense_dist: ::std::option::Option::None,
                    penalty_spot_from_field_line_dist: ::std::option::Option::None,
                    penalty_line_from_spot_dist: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 line_width = 1;

    pub fn clear_line_width(&mut self) {
        self.line_width = ::std::option::Option::None;
    }

    pub fn has_line_width(&self) -> bool {
        self.line_width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_line_width(&mut self, v: i32) {
        self.line_width = ::std::option::Option::Some(v);
    }

    pub fn get_line_width<'a>(&self) -> i32 {
        self.line_width.unwrap_or(0)
    }

    // required int32 field_length = 2;

    pub fn clear_field_length(&mut self) {
        self.field_length = ::std::option::Option::None;
    }

    pub fn has_field_length(&self) -> bool {
        self.field_length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_length(&mut self, v: i32) {
        self.field_length = ::std::option::Option::Some(v);
    }

    pub fn get_field_length<'a>(&self) -> i32 {
        self.field_length.unwrap_or(0)
    }

    // required int32 field_width = 3;

    pub fn clear_field_width(&mut self) {
        self.field_width = ::std::option::Option::None;
    }

    pub fn has_field_width(&self) -> bool {
        self.field_width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_width(&mut self, v: i32) {
        self.field_width = ::std::option::Option::Some(v);
    }

    pub fn get_field_width<'a>(&self) -> i32 {
        self.field_width.unwrap_or(0)
    }

    // required int32 boundary_width = 4;

    pub fn clear_boundary_width(&mut self) {
        self.boundary_width = ::std::option::Option::None;
    }

    pub fn has_boundary_width(&self) -> bool {
        self.boundary_width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_boundary_width(&mut self, v: i32) {
        self.boundary_width = ::std::option::Option::Some(v);
    }

    pub fn get_boundary_width<'a>(&self) -> i32 {
        self.boundary_width.unwrap_or(0)
    }

    // required int32 referee_width = 5;

    pub fn clear_referee_width(&mut self) {
        self.referee_width = ::std::option::Option::None;
    }

    pub fn has_referee_width(&self) -> bool {
        self.referee_width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_referee_width(&mut self, v: i32) {
        self.referee_width = ::std::option::Option::Some(v);
    }

    pub fn get_referee_width<'a>(&self) -> i32 {
        self.referee_width.unwrap_or(0)
    }

    // required int32 goal_width = 6;

    pub fn clear_goal_width(&mut self) {
        self.goal_width = ::std::option::Option::None;
    }

    pub fn has_goal_width(&self) -> bool {
        self.goal_width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_goal_width(&mut self, v: i32) {
        self.goal_width = ::std::option::Option::Some(v);
    }

    pub fn get_goal_width<'a>(&self) -> i32 {
        self.goal_width.unwrap_or(0)
    }

    // required int32 goal_depth = 7;

    pub fn clear_goal_depth(&mut self) {
        self.goal_depth = ::std::option::Option::None;
    }

    pub fn has_goal_depth(&self) -> bool {
        self.goal_depth.is_some()
    }

    // Param is passed by value, moved
    pub fn set_goal_depth(&mut self, v: i32) {
        self.goal_depth = ::std::option::Option::Some(v);
    }

    pub fn get_goal_depth<'a>(&self) -> i32 {
        self.goal_depth.unwrap_or(0)
    }

    // required int32 goal_wall_width = 8;

    pub fn clear_goal_wall_width(&mut self) {
        self.goal_wall_width = ::std::option::Option::None;
    }

    pub fn has_goal_wall_width(&self) -> bool {
        self.goal_wall_width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_goal_wall_width(&mut self, v: i32) {
        self.goal_wall_width = ::std::option::Option::Some(v);
    }

    pub fn get_goal_wall_width<'a>(&self) -> i32 {
        self.goal_wall_width.unwrap_or(0)
    }

    // required int32 center_circle_radius = 9;

    pub fn clear_center_circle_radius(&mut self) {
        self.center_circle_radius = ::std::option::Option::None;
    }

    pub fn has_center_circle_radius(&self) -> bool {
        self.center_circle_radius.is_some()
    }

    // Param is passed by value, moved
    pub fn set_center_circle_radius(&mut self, v: i32) {
        self.center_circle_radius = ::std::option::Option::Some(v);
    }

    pub fn get_center_circle_radius<'a>(&self) -> i32 {
        self.center_circle_radius.unwrap_or(0)
    }

    // required int32 defense_radius = 10;

    pub fn clear_defense_radius(&mut self) {
        self.defense_radius = ::std::option::Option::None;
    }

    pub fn has_defense_radius(&self) -> bool {
        self.defense_radius.is_some()
    }

    // Param is passed by value, moved
    pub fn set_defense_radius(&mut self, v: i32) {
        self.defense_radius = ::std::option::Option::Some(v);
    }

    pub fn get_defense_radius<'a>(&self) -> i32 {
        self.defense_radius.unwrap_or(0)
    }

    // required int32 defense_stretch = 11;

    pub fn clear_defense_stretch(&mut self) {
        self.defense_stretch = ::std::option::Option::None;
    }

    pub fn has_defense_stretch(&self) -> bool {
        self.defense_stretch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_defense_stretch(&mut self, v: i32) {
        self.defense_stretch = ::std::option::Option::Some(v);
    }

    pub fn get_defense_stretch<'a>(&self) -> i32 {
        self.defense_stretch.unwrap_or(0)
    }

    // required int32 free_kick_from_defense_dist = 12;

    pub fn clear_free_kick_from_defense_dist(&mut self) {
        self.free_kick_from_defense_dist = ::std::option::Option::None;
    }

    pub fn has_free_kick_from_defense_dist(&self) -> bool {
        self.free_kick_from_defense_dist.is_some()
    }

    // Param is passed by value, moved
    pub fn set_free_kick_from_defense_dist(&mut self, v: i32) {
        self.free_kick_from_defense_dist = ::std::option::Option::Some(v);
    }

    pub fn get_free_kick_from_defense_dist<'a>(&self) -> i32 {
        self.free_kick_from_defense_dist.unwrap_or(0)
    }

    // required int32 penalty_spot_from_field_line_dist = 13;

    pub fn clear_penalty_spot_from_field_line_dist(&mut self) {
        self.penalty_spot_from_field_line_dist = ::std::option::Option::None;
    }

    pub fn has_penalty_spot_from_field_line_dist(&self) -> bool {
        self.penalty_spot_from_field_line_dist.is_some()
    }

    // Param is passed by value, moved
    pub fn set_penalty_spot_from_field_line_dist(&mut self, v: i32) {
        self.penalty_spot_from_field_line_dist = ::std::option::Option::Some(v);
    }

    pub fn get_penalty_spot_from_field_line_dist<'a>(&self) -> i32 {
        self.penalty_spot_from_field_line_dist.unwrap_or(0)
    }

    // required int32 penalty_line_from_spot_dist = 14;

    pub fn clear_penalty_line_from_spot_dist(&mut self) {
        self.penalty_line_from_spot_dist = ::std::option::Option::None;
    }

    pub fn has_penalty_line_from_spot_dist(&self) -> bool {
        self.penalty_line_from_spot_dist.is_some()
    }

    // Param is passed by value, moved
    pub fn set_penalty_line_from_spot_dist(&mut self, v: i32) {
        self.penalty_line_from_spot_dist = ::std::option::Option::Some(v);
    }

    pub fn get_penalty_line_from_spot_dist<'a>(&self) -> i32 {
        self.penalty_line_from_spot_dist.unwrap_or(0)
    }
}

impl ::protobuf::Message for SSL_GeometryFieldSize {
    fn is_initialized(&self) -> bool {
        if self.line_width.is_none() {
            return false;
        };
        if self.field_length.is_none() {
            return false;
        };
        if self.field_width.is_none() {
            return false;
        };
        if self.boundary_width.is_none() {
            return false;
        };
        if self.referee_width.is_none() {
            return false;
        };
        if self.goal_width.is_none() {
            return false;
        };
        if self.goal_depth.is_none() {
            return false;
        };
        if self.goal_wall_width.is_none() {
            return false;
        };
        if self.center_circle_radius.is_none() {
            return false;
        };
        if self.defense_radius.is_none() {
            return false;
        };
        if self.defense_stretch.is_none() {
            return false;
        };
        if self.free_kick_from_defense_dist.is_none() {
            return false;
        };
        if self.penalty_spot_from_field_line_dist.is_none() {
            return false;
        };
        if self.penalty_line_from_spot_dist.is_none() {
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
                    let tmp = try!(is.read_int32());
                    self.line_width = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.field_length = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.field_width = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.boundary_width = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.referee_width = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.goal_width = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.goal_depth = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.goal_wall_width = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.center_circle_radius = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.defense_radius = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.defense_stretch = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.free_kick_from_defense_dist = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.penalty_spot_from_field_line_dist = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.penalty_line_from_spot_dist = ::std::option::Option::Some(tmp);
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
        for value in self.line_width.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.field_length.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.field_width.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.boundary_width.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.referee_width.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.goal_width.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.goal_depth.iter() {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.goal_wall_width.iter() {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.center_circle_radius.iter() {
            my_size += ::protobuf::rt::value_size(9, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.defense_radius.iter() {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.defense_stretch.iter() {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.free_kick_from_defense_dist.iter() {
            my_size += ::protobuf::rt::value_size(12, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.penalty_spot_from_field_line_dist.iter() {
            my_size += ::protobuf::rt::value_size(13, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.penalty_line_from_spot_dist.iter() {
            my_size += ::protobuf::rt::value_size(14, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.line_width {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.field_length {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.field_width {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.boundary_width {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.referee_width {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.goal_width {
            try!(os.write_int32(6, v));
        };
        if let Some(v) = self.goal_depth {
            try!(os.write_int32(7, v));
        };
        if let Some(v) = self.goal_wall_width {
            try!(os.write_int32(8, v));
        };
        if let Some(v) = self.center_circle_radius {
            try!(os.write_int32(9, v));
        };
        if let Some(v) = self.defense_radius {
            try!(os.write_int32(10, v));
        };
        if let Some(v) = self.defense_stretch {
            try!(os.write_int32(11, v));
        };
        if let Some(v) = self.free_kick_from_defense_dist {
            try!(os.write_int32(12, v));
        };
        if let Some(v) = self.penalty_spot_from_field_line_dist {
            try!(os.write_int32(13, v));
        };
        if let Some(v) = self.penalty_line_from_spot_dist {
            try!(os.write_int32(14, v));
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
        ::std::any::TypeId::of::<SSL_GeometryFieldSize>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SSL_GeometryFieldSize {
    fn new() -> SSL_GeometryFieldSize {
        SSL_GeometryFieldSize::new()
    }

    fn descriptor_static(_: ::std::option::Option<SSL_GeometryFieldSize>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "line_width",
                    SSL_GeometryFieldSize::has_line_width,
                    SSL_GeometryFieldSize::get_line_width,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "field_length",
                    SSL_GeometryFieldSize::has_field_length,
                    SSL_GeometryFieldSize::get_field_length,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "field_width",
                    SSL_GeometryFieldSize::has_field_width,
                    SSL_GeometryFieldSize::get_field_width,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "boundary_width",
                    SSL_GeometryFieldSize::has_boundary_width,
                    SSL_GeometryFieldSize::get_boundary_width,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "referee_width",
                    SSL_GeometryFieldSize::has_referee_width,
                    SSL_GeometryFieldSize::get_referee_width,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "goal_width",
                    SSL_GeometryFieldSize::has_goal_width,
                    SSL_GeometryFieldSize::get_goal_width,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "goal_depth",
                    SSL_GeometryFieldSize::has_goal_depth,
                    SSL_GeometryFieldSize::get_goal_depth,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "goal_wall_width",
                    SSL_GeometryFieldSize::has_goal_wall_width,
                    SSL_GeometryFieldSize::get_goal_wall_width,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "center_circle_radius",
                    SSL_GeometryFieldSize::has_center_circle_radius,
                    SSL_GeometryFieldSize::get_center_circle_radius,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "defense_radius",
                    SSL_GeometryFieldSize::has_defense_radius,
                    SSL_GeometryFieldSize::get_defense_radius,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "defense_stretch",
                    SSL_GeometryFieldSize::has_defense_stretch,
                    SSL_GeometryFieldSize::get_defense_stretch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "free_kick_from_defense_dist",
                    SSL_GeometryFieldSize::has_free_kick_from_defense_dist,
                    SSL_GeometryFieldSize::get_free_kick_from_defense_dist,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "penalty_spot_from_field_line_dist",
                    SSL_GeometryFieldSize::has_penalty_spot_from_field_line_dist,
                    SSL_GeometryFieldSize::get_penalty_spot_from_field_line_dist,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "penalty_line_from_spot_dist",
                    SSL_GeometryFieldSize::has_penalty_line_from_spot_dist,
                    SSL_GeometryFieldSize::get_penalty_line_from_spot_dist,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SSL_GeometryFieldSize>(
                    "SSL_GeometryFieldSize",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SSL_GeometryFieldSize {
    fn clear(&mut self) {
        self.clear_line_width();
        self.clear_field_length();
        self.clear_field_width();
        self.clear_boundary_width();
        self.clear_referee_width();
        self.clear_goal_width();
        self.clear_goal_depth();
        self.clear_goal_wall_width();
        self.clear_center_circle_radius();
        self.clear_defense_radius();
        self.clear_defense_stretch();
        self.clear_free_kick_from_defense_dist();
        self.clear_penalty_spot_from_field_line_dist();
        self.clear_penalty_line_from_spot_dist();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SSL_GeometryFieldSize {
    fn eq(&self, other: &SSL_GeometryFieldSize) -> bool {
        self.line_width == other.line_width &&
        self.field_length == other.field_length &&
        self.field_width == other.field_width &&
        self.boundary_width == other.boundary_width &&
        self.referee_width == other.referee_width &&
        self.goal_width == other.goal_width &&
        self.goal_depth == other.goal_depth &&
        self.goal_wall_width == other.goal_wall_width &&
        self.center_circle_radius == other.center_circle_radius &&
        self.defense_radius == other.defense_radius &&
        self.defense_stretch == other.defense_stretch &&
        self.free_kick_from_defense_dist == other.free_kick_from_defense_dist &&
        self.penalty_spot_from_field_line_dist == other.penalty_spot_from_field_line_dist &&
        self.penalty_line_from_spot_dist == other.penalty_line_from_spot_dist &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SSL_GeometryFieldSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SSL_GeometryData {
    // message fields
    field: ::protobuf::SingularPtrField<SSL_GeometryFieldSize>,
    calib: ::protobuf::RepeatedField<super::messages_robocup_ssl_geometry::SSL_GeometryCameraCalibration>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SSL_GeometryData {}

impl SSL_GeometryData {
    pub fn new() -> SSL_GeometryData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SSL_GeometryData {
        static mut instance: ::protobuf::lazy::Lazy<SSL_GeometryData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SSL_GeometryData,
        };
        unsafe {
            instance.get(|| {
                SSL_GeometryData {
                    field: ::protobuf::SingularPtrField::none(),
                    calib: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .RoboCup2014Legacy.Geometry.SSL_GeometryFieldSize field = 1;

    pub fn clear_field(&mut self) {
        self.field.clear();
    }

    pub fn has_field(&self) -> bool {
        self.field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field(&mut self, v: SSL_GeometryFieldSize) {
        self.field = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field<'a>(&'a mut self) -> &'a mut SSL_GeometryFieldSize {
        if self.field.is_none() {
            self.field.set_default();
        };
        self.field.as_mut().unwrap()
    }

    // Take field
    pub fn take_field(&mut self) -> SSL_GeometryFieldSize {
        self.field.take().unwrap_or_else(|| SSL_GeometryFieldSize::new())
    }

    pub fn get_field<'a>(&'a self) -> &'a SSL_GeometryFieldSize {
        self.field.as_ref().unwrap_or_else(|| SSL_GeometryFieldSize::default_instance())
    }

    // repeated .SSL_GeometryCameraCalibration calib = 2;

    pub fn clear_calib(&mut self) {
        self.calib.clear();
    }

    // Param is passed by value, moved
    pub fn set_calib(&mut self, v: ::protobuf::RepeatedField<super::messages_robocup_ssl_geometry::SSL_GeometryCameraCalibration>) {
        self.calib = v;
    }

    // Mutable pointer to the field.
    pub fn mut_calib<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<super::messages_robocup_ssl_geometry::SSL_GeometryCameraCalibration> {
        &mut self.calib
    }

    // Take field
    pub fn take_calib(&mut self) -> ::protobuf::RepeatedField<super::messages_robocup_ssl_geometry::SSL_GeometryCameraCalibration> {
        ::std::mem::replace(&mut self.calib, ::protobuf::RepeatedField::new())
    }

    pub fn get_calib<'a>(&'a self) -> &'a [super::messages_robocup_ssl_geometry::SSL_GeometryCameraCalibration] {
        &self.calib
    }
}

impl ::protobuf::Message for SSL_GeometryData {
    fn is_initialized(&self) -> bool {
        if self.field.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.field));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.calib));
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
        for value in self.field.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.calib.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.calib.iter() {
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
        ::std::any::TypeId::of::<SSL_GeometryData>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SSL_GeometryData {
    fn new() -> SSL_GeometryData {
        SSL_GeometryData::new()
    }

    fn descriptor_static(_: ::std::option::Option<SSL_GeometryData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "field",
                    SSL_GeometryData::has_field,
                    SSL_GeometryData::get_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "calib",
                    SSL_GeometryData::get_calib,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SSL_GeometryData>(
                    "SSL_GeometryData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SSL_GeometryData {
    fn clear(&mut self) {
        self.clear_field();
        self.clear_calib();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SSL_GeometryData {
    fn eq(&self, other: &SSL_GeometryData) -> bool {
        self.field == other.field &&
        self.calib == other.calib &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SSL_GeometryData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x2a, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x5f, 0x72, 0x6f, 0x62, 0x6f, 0x63,
    0x75, 0x70, 0x5f, 0x73, 0x73, 0x6c, 0x5f, 0x67, 0x65, 0x6f, 0x6d, 0x65, 0x74, 0x72, 0x79, 0x5f,
    0x6c, 0x65, 0x67, 0x61, 0x63, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1a, 0x52, 0x6f,
    0x62, 0x6f, 0x43, 0x75, 0x70, 0x32, 0x30, 0x31, 0x34, 0x4c, 0x65, 0x67, 0x61, 0x63, 0x79, 0x2e,
    0x47, 0x65, 0x6f, 0x6d, 0x65, 0x74, 0x72, 0x79, 0x1a, 0x23, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x73, 0x5f, 0x72, 0x6f, 0x62, 0x6f, 0x63, 0x75, 0x70, 0x5f, 0x73, 0x73, 0x6c, 0x5f, 0x67,
    0x65, 0x6f, 0x6d, 0x65, 0x74, 0x72, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x8a, 0x03,
    0x0a, 0x15, 0x53, 0x53, 0x4c, 0x5f, 0x47, 0x65, 0x6f, 0x6d, 0x65, 0x74, 0x72, 0x79, 0x46, 0x69,
    0x65, 0x6c, 0x64, 0x53, 0x69, 0x7a, 0x65, 0x12, 0x12, 0x0a, 0x0a, 0x6c, 0x69, 0x6e, 0x65, 0x5f,
    0x77, 0x69, 0x64, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05, 0x12, 0x14, 0x0a, 0x0c, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x5f, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x18, 0x02, 0x20, 0x02, 0x28,
    0x05, 0x12, 0x13, 0x0a, 0x0b, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x77, 0x69, 0x64, 0x74, 0x68,
    0x18, 0x03, 0x20, 0x02, 0x28, 0x05, 0x12, 0x16, 0x0a, 0x0e, 0x62, 0x6f, 0x75, 0x6e, 0x64, 0x61,
    0x72, 0x79, 0x5f, 0x77, 0x69, 0x64, 0x74, 0x68, 0x18, 0x04, 0x20, 0x02, 0x28, 0x05, 0x12, 0x15,
    0x0a, 0x0d, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x65, 0x5f, 0x77, 0x69, 0x64, 0x74, 0x68, 0x18,
    0x05, 0x20, 0x02, 0x28, 0x05, 0x12, 0x12, 0x0a, 0x0a, 0x67, 0x6f, 0x61, 0x6c, 0x5f, 0x77, 0x69,
    0x64, 0x74, 0x68, 0x18, 0x06, 0x20, 0x02, 0x28, 0x05, 0x12, 0x12, 0x0a, 0x0a, 0x67, 0x6f, 0x61,
    0x6c, 0x5f, 0x64, 0x65, 0x70, 0x74, 0x68, 0x18, 0x07, 0x20, 0x02, 0x28, 0x05, 0x12, 0x17, 0x0a,
    0x0f, 0x67, 0x6f, 0x61, 0x6c, 0x5f, 0x77, 0x61, 0x6c, 0x6c, 0x5f, 0x77, 0x69, 0x64, 0x74, 0x68,
    0x18, 0x08, 0x20, 0x02, 0x28, 0x05, 0x12, 0x1c, 0x0a, 0x14, 0x63, 0x65, 0x6e, 0x74, 0x65, 0x72,
    0x5f, 0x63, 0x69, 0x72, 0x63, 0x6c, 0x65, 0x5f, 0x72, 0x61, 0x64, 0x69, 0x75, 0x73, 0x18, 0x09,
    0x20, 0x02, 0x28, 0x05, 0x12, 0x16, 0x0a, 0x0e, 0x64, 0x65, 0x66, 0x65, 0x6e, 0x73, 0x65, 0x5f,
    0x72, 0x61, 0x64, 0x69, 0x75, 0x73, 0x18, 0x0a, 0x20, 0x02, 0x28, 0x05, 0x12, 0x17, 0x0a, 0x0f,
    0x64, 0x65, 0x66, 0x65, 0x6e, 0x73, 0x65, 0x5f, 0x73, 0x74, 0x72, 0x65, 0x74, 0x63, 0x68, 0x18,
    0x0b, 0x20, 0x02, 0x28, 0x05, 0x12, 0x23, 0x0a, 0x1b, 0x66, 0x72, 0x65, 0x65, 0x5f, 0x6b, 0x69,
    0x63, 0x6b, 0x5f, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x64, 0x65, 0x66, 0x65, 0x6e, 0x73, 0x65, 0x5f,
    0x64, 0x69, 0x73, 0x74, 0x18, 0x0c, 0x20, 0x02, 0x28, 0x05, 0x12, 0x29, 0x0a, 0x21, 0x70, 0x65,
    0x6e, 0x61, 0x6c, 0x74, 0x79, 0x5f, 0x73, 0x70, 0x6f, 0x74, 0x5f, 0x66, 0x72, 0x6f, 0x6d, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x6c, 0x69, 0x6e, 0x65, 0x5f, 0x64, 0x69, 0x73, 0x74, 0x18,
    0x0d, 0x20, 0x02, 0x28, 0x05, 0x12, 0x23, 0x0a, 0x1b, 0x70, 0x65, 0x6e, 0x61, 0x6c, 0x74, 0x79,
    0x5f, 0x6c, 0x69, 0x6e, 0x65, 0x5f, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x73, 0x70, 0x6f, 0x74, 0x5f,
    0x64, 0x69, 0x73, 0x74, 0x18, 0x0e, 0x20, 0x02, 0x28, 0x05, 0x22, 0x83, 0x01, 0x0a, 0x10, 0x53,
    0x53, 0x4c, 0x5f, 0x47, 0x65, 0x6f, 0x6d, 0x65, 0x74, 0x72, 0x79, 0x44, 0x61, 0x74, 0x61, 0x12,
    0x40, 0x0a, 0x05, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x31,
    0x2e, 0x52, 0x6f, 0x62, 0x6f, 0x43, 0x75, 0x70, 0x32, 0x30, 0x31, 0x34, 0x4c, 0x65, 0x67, 0x61,
    0x63, 0x79, 0x2e, 0x47, 0x65, 0x6f, 0x6d, 0x65, 0x74, 0x72, 0x79, 0x2e, 0x53, 0x53, 0x4c, 0x5f,
    0x47, 0x65, 0x6f, 0x6d, 0x65, 0x74, 0x72, 0x79, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x53, 0x69, 0x7a,
    0x65, 0x12, 0x2d, 0x0a, 0x05, 0x63, 0x61, 0x6c, 0x69, 0x62, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x1e, 0x2e, 0x53, 0x53, 0x4c, 0x5f, 0x47, 0x65, 0x6f, 0x6d, 0x65, 0x74, 0x72, 0x79, 0x43,
    0x61, 0x6d, 0x65, 0x72, 0x61, 0x43, 0x61, 0x6c, 0x69, 0x62, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x4a, 0x9d, 0x09, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1a, 0x01, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x00, 0x12, 0x03, 0x00, 0x07, 0x2c, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x22,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x03, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x04, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x04, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x04, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x11, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x04, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x05, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x05, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x05, 0x11, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x05,
    0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x06, 0x02, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x06, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x06, 0x11, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x06, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12,
    0x03, 0x07, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x07,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x07, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x07, 0x11, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x07, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x08, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x08, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05,
    0x12, 0x03, 0x08, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x08, 0x11, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x08, 0x21,
    0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x09, 0x02, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x09, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x05, 0x01, 0x12, 0x03, 0x09, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x09, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03,
    0x0a, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x0a, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x0a, 0x11, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x0a, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x07, 0x12, 0x03, 0x0b, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07,
    0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x05, 0x12,
    0x03, 0x0b, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x0b,
    0x11, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x0b, 0x23, 0x24,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x0c, 0x02, 0x2a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x08, 0x01, 0x12, 0x03, 0x0c, 0x11, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03,
    0x12, 0x03, 0x0c, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x0d,
    0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x0d, 0x11, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x0d, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x0a, 0x12, 0x03, 0x0e, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x04,
    0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x05, 0x12, 0x03,
    0x0e, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x0e, 0x11,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x0e, 0x23, 0x25, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x0f, 0x02, 0x32, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0b, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b,
    0x01, 0x12, 0x03, 0x0f, 0x11, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x03, 0x12,
    0x03, 0x0f, 0x2f, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x10, 0x02,
    0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x10, 0x11, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x10, 0x35, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x0d, 0x12, 0x03, 0x11, 0x02, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x04, 0x12,
    0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x11,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x11, 0x11, 0x2c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x11, 0x2f, 0x31, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x17, 0x00, 0x1a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x17, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x18, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x18, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x18, 0x0b, 0x20, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x21, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x01, 0x12, 0x03, 0x19, 0x02, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x19, 0x0b, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x19,
    0x29, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x19, 0x31, 0x32,
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
