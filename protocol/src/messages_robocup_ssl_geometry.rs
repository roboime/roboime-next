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
pub struct Vector2f {
    // message fields
    x: ::std::option::Option<f32>,
    y: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Vector2f {}

impl Vector2f {
    pub fn new() -> Vector2f {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Vector2f {
        static mut instance: ::protobuf::lazy::Lazy<Vector2f> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Vector2f,
        };
        unsafe {
            instance.get(|| {
                Vector2f {
                    x: ::std::option::Option::None,
                    y: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required float x = 1;

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

    // required float y = 2;

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
}

impl ::protobuf::Message for Vector2f {
    fn is_initialized(&self) -> bool {
        if self.x.is_none() {
            return false;
        };
        if self.y.is_none() {
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
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.y = ::std::option::Option::Some(tmp);
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
            my_size += 5;
        };
        if self.y.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            try!(os.write_float(1, v));
        };
        if let Some(v) = self.y {
            try!(os.write_float(2, v));
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
        ::std::any::TypeId::of::<Vector2f>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Vector2f {
    fn new() -> Vector2f {
        Vector2f::new()
    }

    fn descriptor_static(_: ::std::option::Option<Vector2f>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "x",
                    Vector2f::has_x,
                    Vector2f::get_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "y",
                    Vector2f::has_y,
                    Vector2f::get_y,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Vector2f>(
                    "Vector2f",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Vector2f {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Vector2f {
    fn eq(&self, other: &Vector2f) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Vector2f {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SSL_FieldLineSegment {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    p1: ::protobuf::SingularPtrField<Vector2f>,
    p2: ::protobuf::SingularPtrField<Vector2f>,
    thickness: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SSL_FieldLineSegment {}

impl SSL_FieldLineSegment {
    pub fn new() -> SSL_FieldLineSegment {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SSL_FieldLineSegment {
        static mut instance: ::protobuf::lazy::Lazy<SSL_FieldLineSegment> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SSL_FieldLineSegment,
        };
        unsafe {
            instance.get(|| {
                SSL_FieldLineSegment {
                    name: ::protobuf::SingularField::none(),
                    p1: ::protobuf::SingularPtrField::none(),
                    p2: ::protobuf::SingularPtrField::none(),
                    thickness: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required .Vector2f p1 = 2;

    pub fn clear_p1(&mut self) {
        self.p1.clear();
    }

    pub fn has_p1(&self) -> bool {
        self.p1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_p1(&mut self, v: Vector2f) {
        self.p1 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_p1<'a>(&'a mut self) -> &'a mut Vector2f {
        if self.p1.is_none() {
            self.p1.set_default();
        };
        self.p1.as_mut().unwrap()
    }

    // Take field
    pub fn take_p1(&mut self) -> Vector2f {
        self.p1.take().unwrap_or_else(|| Vector2f::new())
    }

    pub fn get_p1<'a>(&'a self) -> &'a Vector2f {
        self.p1.as_ref().unwrap_or_else(|| Vector2f::default_instance())
    }

    // required .Vector2f p2 = 3;

    pub fn clear_p2(&mut self) {
        self.p2.clear();
    }

    pub fn has_p2(&self) -> bool {
        self.p2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_p2(&mut self, v: Vector2f) {
        self.p2 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_p2<'a>(&'a mut self) -> &'a mut Vector2f {
        if self.p2.is_none() {
            self.p2.set_default();
        };
        self.p2.as_mut().unwrap()
    }

    // Take field
    pub fn take_p2(&mut self) -> Vector2f {
        self.p2.take().unwrap_or_else(|| Vector2f::new())
    }

    pub fn get_p2<'a>(&'a self) -> &'a Vector2f {
        self.p2.as_ref().unwrap_or_else(|| Vector2f::default_instance())
    }

    // required float thickness = 4;

    pub fn clear_thickness(&mut self) {
        self.thickness = ::std::option::Option::None;
    }

    pub fn has_thickness(&self) -> bool {
        self.thickness.is_some()
    }

    // Param is passed by value, moved
    pub fn set_thickness(&mut self, v: f32) {
        self.thickness = ::std::option::Option::Some(v);
    }

    pub fn get_thickness<'a>(&self) -> f32 {
        self.thickness.unwrap_or(0.)
    }
}

impl ::protobuf::Message for SSL_FieldLineSegment {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.p1.is_none() {
            return false;
        };
        if self.p2.is_none() {
            return false;
        };
        if self.thickness.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.p1));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.p2));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.thickness = ::std::option::Option::Some(tmp);
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
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.p1.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.p2.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.thickness.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.p1.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.p2.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.thickness {
            try!(os.write_float(4, v));
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
        ::std::any::TypeId::of::<SSL_FieldLineSegment>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SSL_FieldLineSegment {
    fn new() -> SSL_FieldLineSegment {
        SSL_FieldLineSegment::new()
    }

    fn descriptor_static(_: ::std::option::Option<SSL_FieldLineSegment>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    SSL_FieldLineSegment::has_name,
                    SSL_FieldLineSegment::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "p1",
                    SSL_FieldLineSegment::has_p1,
                    SSL_FieldLineSegment::get_p1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "p2",
                    SSL_FieldLineSegment::has_p2,
                    SSL_FieldLineSegment::get_p2,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "thickness",
                    SSL_FieldLineSegment::has_thickness,
                    SSL_FieldLineSegment::get_thickness,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SSL_FieldLineSegment>(
                    "SSL_FieldLineSegment",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SSL_FieldLineSegment {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_p1();
        self.clear_p2();
        self.clear_thickness();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SSL_FieldLineSegment {
    fn eq(&self, other: &SSL_FieldLineSegment) -> bool {
        self.name == other.name &&
        self.p1 == other.p1 &&
        self.p2 == other.p2 &&
        self.thickness == other.thickness &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SSL_FieldLineSegment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SSL_FieldCicularArc {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    center: ::protobuf::SingularPtrField<Vector2f>,
    radius: ::std::option::Option<f32>,
    a1: ::std::option::Option<f32>,
    a2: ::std::option::Option<f32>,
    thickness: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SSL_FieldCicularArc {}

impl SSL_FieldCicularArc {
    pub fn new() -> SSL_FieldCicularArc {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SSL_FieldCicularArc {
        static mut instance: ::protobuf::lazy::Lazy<SSL_FieldCicularArc> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SSL_FieldCicularArc,
        };
        unsafe {
            instance.get(|| {
                SSL_FieldCicularArc {
                    name: ::protobuf::SingularField::none(),
                    center: ::protobuf::SingularPtrField::none(),
                    radius: ::std::option::Option::None,
                    a1: ::std::option::Option::None,
                    a2: ::std::option::Option::None,
                    thickness: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required .Vector2f center = 2;

    pub fn clear_center(&mut self) {
        self.center.clear();
    }

    pub fn has_center(&self) -> bool {
        self.center.is_some()
    }

    // Param is passed by value, moved
    pub fn set_center(&mut self, v: Vector2f) {
        self.center = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_center<'a>(&'a mut self) -> &'a mut Vector2f {
        if self.center.is_none() {
            self.center.set_default();
        };
        self.center.as_mut().unwrap()
    }

    // Take field
    pub fn take_center(&mut self) -> Vector2f {
        self.center.take().unwrap_or_else(|| Vector2f::new())
    }

    pub fn get_center<'a>(&'a self) -> &'a Vector2f {
        self.center.as_ref().unwrap_or_else(|| Vector2f::default_instance())
    }

    // required float radius = 3;

    pub fn clear_radius(&mut self) {
        self.radius = ::std::option::Option::None;
    }

    pub fn has_radius(&self) -> bool {
        self.radius.is_some()
    }

    // Param is passed by value, moved
    pub fn set_radius(&mut self, v: f32) {
        self.radius = ::std::option::Option::Some(v);
    }

    pub fn get_radius<'a>(&self) -> f32 {
        self.radius.unwrap_or(0.)
    }

    // required float a1 = 4;

    pub fn clear_a1(&mut self) {
        self.a1 = ::std::option::Option::None;
    }

    pub fn has_a1(&self) -> bool {
        self.a1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_a1(&mut self, v: f32) {
        self.a1 = ::std::option::Option::Some(v);
    }

    pub fn get_a1<'a>(&self) -> f32 {
        self.a1.unwrap_or(0.)
    }

    // required float a2 = 5;

    pub fn clear_a2(&mut self) {
        self.a2 = ::std::option::Option::None;
    }

    pub fn has_a2(&self) -> bool {
        self.a2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_a2(&mut self, v: f32) {
        self.a2 = ::std::option::Option::Some(v);
    }

    pub fn get_a2<'a>(&self) -> f32 {
        self.a2.unwrap_or(0.)
    }

    // required float thickness = 6;

    pub fn clear_thickness(&mut self) {
        self.thickness = ::std::option::Option::None;
    }

    pub fn has_thickness(&self) -> bool {
        self.thickness.is_some()
    }

    // Param is passed by value, moved
    pub fn set_thickness(&mut self, v: f32) {
        self.thickness = ::std::option::Option::Some(v);
    }

    pub fn get_thickness<'a>(&self) -> f32 {
        self.thickness.unwrap_or(0.)
    }
}

impl ::protobuf::Message for SSL_FieldCicularArc {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.center.is_none() {
            return false;
        };
        if self.radius.is_none() {
            return false;
        };
        if self.a1.is_none() {
            return false;
        };
        if self.a2.is_none() {
            return false;
        };
        if self.thickness.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.center));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.radius = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.a1 = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.a2 = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.thickness = ::std::option::Option::Some(tmp);
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
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.center.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.radius.is_some() {
            my_size += 5;
        };
        if self.a1.is_some() {
            my_size += 5;
        };
        if self.a2.is_some() {
            my_size += 5;
        };
        if self.thickness.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.center.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.radius {
            try!(os.write_float(3, v));
        };
        if let Some(v) = self.a1 {
            try!(os.write_float(4, v));
        };
        if let Some(v) = self.a2 {
            try!(os.write_float(5, v));
        };
        if let Some(v) = self.thickness {
            try!(os.write_float(6, v));
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
        ::std::any::TypeId::of::<SSL_FieldCicularArc>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SSL_FieldCicularArc {
    fn new() -> SSL_FieldCicularArc {
        SSL_FieldCicularArc::new()
    }

    fn descriptor_static(_: ::std::option::Option<SSL_FieldCicularArc>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    SSL_FieldCicularArc::has_name,
                    SSL_FieldCicularArc::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "center",
                    SSL_FieldCicularArc::has_center,
                    SSL_FieldCicularArc::get_center,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "radius",
                    SSL_FieldCicularArc::has_radius,
                    SSL_FieldCicularArc::get_radius,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "a1",
                    SSL_FieldCicularArc::has_a1,
                    SSL_FieldCicularArc::get_a1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "a2",
                    SSL_FieldCicularArc::has_a2,
                    SSL_FieldCicularArc::get_a2,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "thickness",
                    SSL_FieldCicularArc::has_thickness,
                    SSL_FieldCicularArc::get_thickness,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SSL_FieldCicularArc>(
                    "SSL_FieldCicularArc",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SSL_FieldCicularArc {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_center();
        self.clear_radius();
        self.clear_a1();
        self.clear_a2();
        self.clear_thickness();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SSL_FieldCicularArc {
    fn eq(&self, other: &SSL_FieldCicularArc) -> bool {
        self.name == other.name &&
        self.center == other.center &&
        self.radius == other.radius &&
        self.a1 == other.a1 &&
        self.a2 == other.a2 &&
        self.thickness == other.thickness &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SSL_FieldCicularArc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SSL_GeometryFieldSize {
    // message fields
    field_length: ::std::option::Option<i32>,
    field_width: ::std::option::Option<i32>,
    goal_width: ::std::option::Option<i32>,
    goal_depth: ::std::option::Option<i32>,
    boundary_width: ::std::option::Option<i32>,
    field_lines: ::protobuf::RepeatedField<SSL_FieldLineSegment>,
    field_arcs: ::protobuf::RepeatedField<SSL_FieldCicularArc>,
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
                    field_length: ::std::option::Option::None,
                    field_width: ::std::option::Option::None,
                    goal_width: ::std::option::Option::None,
                    goal_depth: ::std::option::Option::None,
                    boundary_width: ::std::option::Option::None,
                    field_lines: ::protobuf::RepeatedField::new(),
                    field_arcs: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 field_length = 1;

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

    // required int32 field_width = 2;

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

    // required int32 goal_width = 3;

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

    // required int32 goal_depth = 4;

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

    // required int32 boundary_width = 5;

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

    // repeated .SSL_FieldLineSegment field_lines = 6;

    pub fn clear_field_lines(&mut self) {
        self.field_lines.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_lines(&mut self, v: ::protobuf::RepeatedField<SSL_FieldLineSegment>) {
        self.field_lines = v;
    }

    // Mutable pointer to the field.
    pub fn mut_field_lines<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<SSL_FieldLineSegment> {
        &mut self.field_lines
    }

    // Take field
    pub fn take_field_lines(&mut self) -> ::protobuf::RepeatedField<SSL_FieldLineSegment> {
        ::std::mem::replace(&mut self.field_lines, ::protobuf::RepeatedField::new())
    }

    pub fn get_field_lines<'a>(&'a self) -> &'a [SSL_FieldLineSegment] {
        &self.field_lines
    }

    // repeated .SSL_FieldCicularArc field_arcs = 7;

    pub fn clear_field_arcs(&mut self) {
        self.field_arcs.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_arcs(&mut self, v: ::protobuf::RepeatedField<SSL_FieldCicularArc>) {
        self.field_arcs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_field_arcs<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<SSL_FieldCicularArc> {
        &mut self.field_arcs
    }

    // Take field
    pub fn take_field_arcs(&mut self) -> ::protobuf::RepeatedField<SSL_FieldCicularArc> {
        ::std::mem::replace(&mut self.field_arcs, ::protobuf::RepeatedField::new())
    }

    pub fn get_field_arcs<'a>(&'a self) -> &'a [SSL_FieldCicularArc] {
        &self.field_arcs
    }
}

impl ::protobuf::Message for SSL_GeometryFieldSize {
    fn is_initialized(&self) -> bool {
        if self.field_length.is_none() {
            return false;
        };
        if self.field_width.is_none() {
            return false;
        };
        if self.goal_width.is_none() {
            return false;
        };
        if self.goal_depth.is_none() {
            return false;
        };
        if self.boundary_width.is_none() {
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
                    self.field_length = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.field_width = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.goal_width = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.goal_depth = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.boundary_width = ::std::option::Option::Some(tmp);
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.field_lines));
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.field_arcs));
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
        for value in self.field_length.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.field_width.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.goal_width.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.goal_depth.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.boundary_width.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.field_lines.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.field_arcs.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_length {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.field_width {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.goal_width {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.goal_depth {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.boundary_width {
            try!(os.write_int32(5, v));
        };
        for v in self.field_lines.iter() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.field_arcs.iter() {
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
                    "boundary_width",
                    SSL_GeometryFieldSize::has_boundary_width,
                    SSL_GeometryFieldSize::get_boundary_width,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "field_lines",
                    SSL_GeometryFieldSize::get_field_lines,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "field_arcs",
                    SSL_GeometryFieldSize::get_field_arcs,
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
        self.clear_field_length();
        self.clear_field_width();
        self.clear_goal_width();
        self.clear_goal_depth();
        self.clear_boundary_width();
        self.clear_field_lines();
        self.clear_field_arcs();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SSL_GeometryFieldSize {
    fn eq(&self, other: &SSL_GeometryFieldSize) -> bool {
        self.field_length == other.field_length &&
        self.field_width == other.field_width &&
        self.goal_width == other.goal_width &&
        self.goal_depth == other.goal_depth &&
        self.boundary_width == other.boundary_width &&
        self.field_lines == other.field_lines &&
        self.field_arcs == other.field_arcs &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SSL_GeometryFieldSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SSL_GeometryCameraCalibration {
    // message fields
    camera_id: ::std::option::Option<u32>,
    focal_length: ::std::option::Option<f32>,
    principal_point_x: ::std::option::Option<f32>,
    principal_point_y: ::std::option::Option<f32>,
    distortion: ::std::option::Option<f32>,
    q0: ::std::option::Option<f32>,
    q1: ::std::option::Option<f32>,
    q2: ::std::option::Option<f32>,
    q3: ::std::option::Option<f32>,
    tx: ::std::option::Option<f32>,
    ty: ::std::option::Option<f32>,
    tz: ::std::option::Option<f32>,
    derived_camera_world_tx: ::std::option::Option<f32>,
    derived_camera_world_ty: ::std::option::Option<f32>,
    derived_camera_world_tz: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SSL_GeometryCameraCalibration {}

impl SSL_GeometryCameraCalibration {
    pub fn new() -> SSL_GeometryCameraCalibration {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SSL_GeometryCameraCalibration {
        static mut instance: ::protobuf::lazy::Lazy<SSL_GeometryCameraCalibration> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SSL_GeometryCameraCalibration,
        };
        unsafe {
            instance.get(|| {
                SSL_GeometryCameraCalibration {
                    camera_id: ::std::option::Option::None,
                    focal_length: ::std::option::Option::None,
                    principal_point_x: ::std::option::Option::None,
                    principal_point_y: ::std::option::Option::None,
                    distortion: ::std::option::Option::None,
                    q0: ::std::option::Option::None,
                    q1: ::std::option::Option::None,
                    q2: ::std::option::Option::None,
                    q3: ::std::option::Option::None,
                    tx: ::std::option::Option::None,
                    ty: ::std::option::Option::None,
                    tz: ::std::option::Option::None,
                    derived_camera_world_tx: ::std::option::Option::None,
                    derived_camera_world_ty: ::std::option::Option::None,
                    derived_camera_world_tz: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 camera_id = 1;

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

    // required float focal_length = 2;

    pub fn clear_focal_length(&mut self) {
        self.focal_length = ::std::option::Option::None;
    }

    pub fn has_focal_length(&self) -> bool {
        self.focal_length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_focal_length(&mut self, v: f32) {
        self.focal_length = ::std::option::Option::Some(v);
    }

    pub fn get_focal_length<'a>(&self) -> f32 {
        self.focal_length.unwrap_or(0.)
    }

    // required float principal_point_x = 3;

    pub fn clear_principal_point_x(&mut self) {
        self.principal_point_x = ::std::option::Option::None;
    }

    pub fn has_principal_point_x(&self) -> bool {
        self.principal_point_x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_principal_point_x(&mut self, v: f32) {
        self.principal_point_x = ::std::option::Option::Some(v);
    }

    pub fn get_principal_point_x<'a>(&self) -> f32 {
        self.principal_point_x.unwrap_or(0.)
    }

    // required float principal_point_y = 4;

    pub fn clear_principal_point_y(&mut self) {
        self.principal_point_y = ::std::option::Option::None;
    }

    pub fn has_principal_point_y(&self) -> bool {
        self.principal_point_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_principal_point_y(&mut self, v: f32) {
        self.principal_point_y = ::std::option::Option::Some(v);
    }

    pub fn get_principal_point_y<'a>(&self) -> f32 {
        self.principal_point_y.unwrap_or(0.)
    }

    // required float distortion = 5;

    pub fn clear_distortion(&mut self) {
        self.distortion = ::std::option::Option::None;
    }

    pub fn has_distortion(&self) -> bool {
        self.distortion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_distortion(&mut self, v: f32) {
        self.distortion = ::std::option::Option::Some(v);
    }

    pub fn get_distortion<'a>(&self) -> f32 {
        self.distortion.unwrap_or(0.)
    }

    // required float q0 = 6;

    pub fn clear_q0(&mut self) {
        self.q0 = ::std::option::Option::None;
    }

    pub fn has_q0(&self) -> bool {
        self.q0.is_some()
    }

    // Param is passed by value, moved
    pub fn set_q0(&mut self, v: f32) {
        self.q0 = ::std::option::Option::Some(v);
    }

    pub fn get_q0<'a>(&self) -> f32 {
        self.q0.unwrap_or(0.)
    }

    // required float q1 = 7;

    pub fn clear_q1(&mut self) {
        self.q1 = ::std::option::Option::None;
    }

    pub fn has_q1(&self) -> bool {
        self.q1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_q1(&mut self, v: f32) {
        self.q1 = ::std::option::Option::Some(v);
    }

    pub fn get_q1<'a>(&self) -> f32 {
        self.q1.unwrap_or(0.)
    }

    // required float q2 = 8;

    pub fn clear_q2(&mut self) {
        self.q2 = ::std::option::Option::None;
    }

    pub fn has_q2(&self) -> bool {
        self.q2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_q2(&mut self, v: f32) {
        self.q2 = ::std::option::Option::Some(v);
    }

    pub fn get_q2<'a>(&self) -> f32 {
        self.q2.unwrap_or(0.)
    }

    // required float q3 = 9;

    pub fn clear_q3(&mut self) {
        self.q3 = ::std::option::Option::None;
    }

    pub fn has_q3(&self) -> bool {
        self.q3.is_some()
    }

    // Param is passed by value, moved
    pub fn set_q3(&mut self, v: f32) {
        self.q3 = ::std::option::Option::Some(v);
    }

    pub fn get_q3<'a>(&self) -> f32 {
        self.q3.unwrap_or(0.)
    }

    // required float tx = 10;

    pub fn clear_tx(&mut self) {
        self.tx = ::std::option::Option::None;
    }

    pub fn has_tx(&self) -> bool {
        self.tx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tx(&mut self, v: f32) {
        self.tx = ::std::option::Option::Some(v);
    }

    pub fn get_tx<'a>(&self) -> f32 {
        self.tx.unwrap_or(0.)
    }

    // required float ty = 11;

    pub fn clear_ty(&mut self) {
        self.ty = ::std::option::Option::None;
    }

    pub fn has_ty(&self) -> bool {
        self.ty.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ty(&mut self, v: f32) {
        self.ty = ::std::option::Option::Some(v);
    }

    pub fn get_ty<'a>(&self) -> f32 {
        self.ty.unwrap_or(0.)
    }

    // required float tz = 12;

    pub fn clear_tz(&mut self) {
        self.tz = ::std::option::Option::None;
    }

    pub fn has_tz(&self) -> bool {
        self.tz.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tz(&mut self, v: f32) {
        self.tz = ::std::option::Option::Some(v);
    }

    pub fn get_tz<'a>(&self) -> f32 {
        self.tz.unwrap_or(0.)
    }

    // optional float derived_camera_world_tx = 13;

    pub fn clear_derived_camera_world_tx(&mut self) {
        self.derived_camera_world_tx = ::std::option::Option::None;
    }

    pub fn has_derived_camera_world_tx(&self) -> bool {
        self.derived_camera_world_tx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_derived_camera_world_tx(&mut self, v: f32) {
        self.derived_camera_world_tx = ::std::option::Option::Some(v);
    }

    pub fn get_derived_camera_world_tx<'a>(&self) -> f32 {
        self.derived_camera_world_tx.unwrap_or(0.)
    }

    // optional float derived_camera_world_ty = 14;

    pub fn clear_derived_camera_world_ty(&mut self) {
        self.derived_camera_world_ty = ::std::option::Option::None;
    }

    pub fn has_derived_camera_world_ty(&self) -> bool {
        self.derived_camera_world_ty.is_some()
    }

    // Param is passed by value, moved
    pub fn set_derived_camera_world_ty(&mut self, v: f32) {
        self.derived_camera_world_ty = ::std::option::Option::Some(v);
    }

    pub fn get_derived_camera_world_ty<'a>(&self) -> f32 {
        self.derived_camera_world_ty.unwrap_or(0.)
    }

    // optional float derived_camera_world_tz = 15;

    pub fn clear_derived_camera_world_tz(&mut self) {
        self.derived_camera_world_tz = ::std::option::Option::None;
    }

    pub fn has_derived_camera_world_tz(&self) -> bool {
        self.derived_camera_world_tz.is_some()
    }

    // Param is passed by value, moved
    pub fn set_derived_camera_world_tz(&mut self, v: f32) {
        self.derived_camera_world_tz = ::std::option::Option::Some(v);
    }

    pub fn get_derived_camera_world_tz<'a>(&self) -> f32 {
        self.derived_camera_world_tz.unwrap_or(0.)
    }
}

impl ::protobuf::Message for SSL_GeometryCameraCalibration {
    fn is_initialized(&self) -> bool {
        if self.camera_id.is_none() {
            return false;
        };
        if self.focal_length.is_none() {
            return false;
        };
        if self.principal_point_x.is_none() {
            return false;
        };
        if self.principal_point_y.is_none() {
            return false;
        };
        if self.distortion.is_none() {
            return false;
        };
        if self.q0.is_none() {
            return false;
        };
        if self.q1.is_none() {
            return false;
        };
        if self.q2.is_none() {
            return false;
        };
        if self.q3.is_none() {
            return false;
        };
        if self.tx.is_none() {
            return false;
        };
        if self.ty.is_none() {
            return false;
        };
        if self.tz.is_none() {
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
                    self.camera_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.focal_length = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.principal_point_x = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.principal_point_y = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.distortion = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.q0 = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.q1 = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.q2 = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.q3 = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.tx = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.ty = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.tz = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.derived_camera_world_tx = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.derived_camera_world_ty = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.derived_camera_world_tz = ::std::option::Option::Some(tmp);
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
        for value in self.camera_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.focal_length.is_some() {
            my_size += 5;
        };
        if self.principal_point_x.is_some() {
            my_size += 5;
        };
        if self.principal_point_y.is_some() {
            my_size += 5;
        };
        if self.distortion.is_some() {
            my_size += 5;
        };
        if self.q0.is_some() {
            my_size += 5;
        };
        if self.q1.is_some() {
            my_size += 5;
        };
        if self.q2.is_some() {
            my_size += 5;
        };
        if self.q3.is_some() {
            my_size += 5;
        };
        if self.tx.is_some() {
            my_size += 5;
        };
        if self.ty.is_some() {
            my_size += 5;
        };
        if self.tz.is_some() {
            my_size += 5;
        };
        if self.derived_camera_world_tx.is_some() {
            my_size += 5;
        };
        if self.derived_camera_world_ty.is_some() {
            my_size += 5;
        };
        if self.derived_camera_world_tz.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.camera_id {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.focal_length {
            try!(os.write_float(2, v));
        };
        if let Some(v) = self.principal_point_x {
            try!(os.write_float(3, v));
        };
        if let Some(v) = self.principal_point_y {
            try!(os.write_float(4, v));
        };
        if let Some(v) = self.distortion {
            try!(os.write_float(5, v));
        };
        if let Some(v) = self.q0 {
            try!(os.write_float(6, v));
        };
        if let Some(v) = self.q1 {
            try!(os.write_float(7, v));
        };
        if let Some(v) = self.q2 {
            try!(os.write_float(8, v));
        };
        if let Some(v) = self.q3 {
            try!(os.write_float(9, v));
        };
        if let Some(v) = self.tx {
            try!(os.write_float(10, v));
        };
        if let Some(v) = self.ty {
            try!(os.write_float(11, v));
        };
        if let Some(v) = self.tz {
            try!(os.write_float(12, v));
        };
        if let Some(v) = self.derived_camera_world_tx {
            try!(os.write_float(13, v));
        };
        if let Some(v) = self.derived_camera_world_ty {
            try!(os.write_float(14, v));
        };
        if let Some(v) = self.derived_camera_world_tz {
            try!(os.write_float(15, v));
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
        ::std::any::TypeId::of::<SSL_GeometryCameraCalibration>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SSL_GeometryCameraCalibration {
    fn new() -> SSL_GeometryCameraCalibration {
        SSL_GeometryCameraCalibration::new()
    }

    fn descriptor_static(_: ::std::option::Option<SSL_GeometryCameraCalibration>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "camera_id",
                    SSL_GeometryCameraCalibration::has_camera_id,
                    SSL_GeometryCameraCalibration::get_camera_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "focal_length",
                    SSL_GeometryCameraCalibration::has_focal_length,
                    SSL_GeometryCameraCalibration::get_focal_length,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "principal_point_x",
                    SSL_GeometryCameraCalibration::has_principal_point_x,
                    SSL_GeometryCameraCalibration::get_principal_point_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "principal_point_y",
                    SSL_GeometryCameraCalibration::has_principal_point_y,
                    SSL_GeometryCameraCalibration::get_principal_point_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "distortion",
                    SSL_GeometryCameraCalibration::has_distortion,
                    SSL_GeometryCameraCalibration::get_distortion,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "q0",
                    SSL_GeometryCameraCalibration::has_q0,
                    SSL_GeometryCameraCalibration::get_q0,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "q1",
                    SSL_GeometryCameraCalibration::has_q1,
                    SSL_GeometryCameraCalibration::get_q1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "q2",
                    SSL_GeometryCameraCalibration::has_q2,
                    SSL_GeometryCameraCalibration::get_q2,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "q3",
                    SSL_GeometryCameraCalibration::has_q3,
                    SSL_GeometryCameraCalibration::get_q3,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "tx",
                    SSL_GeometryCameraCalibration::has_tx,
                    SSL_GeometryCameraCalibration::get_tx,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "ty",
                    SSL_GeometryCameraCalibration::has_ty,
                    SSL_GeometryCameraCalibration::get_ty,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "tz",
                    SSL_GeometryCameraCalibration::has_tz,
                    SSL_GeometryCameraCalibration::get_tz,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "derived_camera_world_tx",
                    SSL_GeometryCameraCalibration::has_derived_camera_world_tx,
                    SSL_GeometryCameraCalibration::get_derived_camera_world_tx,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "derived_camera_world_ty",
                    SSL_GeometryCameraCalibration::has_derived_camera_world_ty,
                    SSL_GeometryCameraCalibration::get_derived_camera_world_ty,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "derived_camera_world_tz",
                    SSL_GeometryCameraCalibration::has_derived_camera_world_tz,
                    SSL_GeometryCameraCalibration::get_derived_camera_world_tz,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SSL_GeometryCameraCalibration>(
                    "SSL_GeometryCameraCalibration",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SSL_GeometryCameraCalibration {
    fn clear(&mut self) {
        self.clear_camera_id();
        self.clear_focal_length();
        self.clear_principal_point_x();
        self.clear_principal_point_y();
        self.clear_distortion();
        self.clear_q0();
        self.clear_q1();
        self.clear_q2();
        self.clear_q3();
        self.clear_tx();
        self.clear_ty();
        self.clear_tz();
        self.clear_derived_camera_world_tx();
        self.clear_derived_camera_world_ty();
        self.clear_derived_camera_world_tz();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SSL_GeometryCameraCalibration {
    fn eq(&self, other: &SSL_GeometryCameraCalibration) -> bool {
        self.camera_id == other.camera_id &&
        self.focal_length == other.focal_length &&
        self.principal_point_x == other.principal_point_x &&
        self.principal_point_y == other.principal_point_y &&
        self.distortion == other.distortion &&
        self.q0 == other.q0 &&
        self.q1 == other.q1 &&
        self.q2 == other.q2 &&
        self.q3 == other.q3 &&
        self.tx == other.tx &&
        self.ty == other.ty &&
        self.tz == other.tz &&
        self.derived_camera_world_tx == other.derived_camera_world_tx &&
        self.derived_camera_world_ty == other.derived_camera_world_ty &&
        self.derived_camera_world_tz == other.derived_camera_world_tz &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SSL_GeometryCameraCalibration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SSL_GeometryData {
    // message fields
    field: ::protobuf::SingularPtrField<SSL_GeometryFieldSize>,
    calib: ::protobuf::RepeatedField<SSL_GeometryCameraCalibration>,
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

    // required .SSL_GeometryFieldSize field = 1;

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
    pub fn set_calib(&mut self, v: ::protobuf::RepeatedField<SSL_GeometryCameraCalibration>) {
        self.calib = v;
    }

    // Mutable pointer to the field.
    pub fn mut_calib<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<SSL_GeometryCameraCalibration> {
        &mut self.calib
    }

    // Take field
    pub fn take_calib(&mut self) -> ::protobuf::RepeatedField<SSL_GeometryCameraCalibration> {
        ::std::mem::replace(&mut self.calib, ::protobuf::RepeatedField::new())
    }

    pub fn get_calib<'a>(&'a self) -> &'a [SSL_GeometryCameraCalibration] {
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
    0x0a, 0x23, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x5f, 0x72, 0x6f, 0x62, 0x6f, 0x63,
    0x75, 0x70, 0x5f, 0x73, 0x73, 0x6c, 0x5f, 0x67, 0x65, 0x6f, 0x6d, 0x65, 0x74, 0x72, 0x79, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x20, 0x0a, 0x08, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x32,
    0x66, 0x12, 0x09, 0x0a, 0x01, 0x78, 0x18, 0x01, 0x20, 0x02, 0x28, 0x02, 0x12, 0x09, 0x0a, 0x01,
    0x79, 0x18, 0x02, 0x20, 0x02, 0x28, 0x02, 0x22, 0x65, 0x0a, 0x14, 0x53, 0x53, 0x4c, 0x5f, 0x46,
    0x69, 0x65, 0x6c, 0x64, 0x4c, 0x69, 0x6e, 0x65, 0x53, 0x65, 0x67, 0x6d, 0x65, 0x6e, 0x74, 0x12,
    0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x15, 0x0a,
    0x02, 0x70, 0x31, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x56, 0x65, 0x63, 0x74,
    0x6f, 0x72, 0x32, 0x66, 0x12, 0x15, 0x0a, 0x02, 0x70, 0x32, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0b,
    0x32, 0x09, 0x2e, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x32, 0x66, 0x12, 0x11, 0x0a, 0x09, 0x74,
    0x68, 0x69, 0x63, 0x6b, 0x6e, 0x65, 0x73, 0x73, 0x18, 0x04, 0x20, 0x02, 0x28, 0x02, 0x22, 0x79,
    0x0a, 0x13, 0x53, 0x53, 0x4c, 0x5f, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x43, 0x69, 0x63, 0x75, 0x6c,
    0x61, 0x72, 0x41, 0x72, 0x63, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x09, 0x12, 0x19, 0x0a, 0x06, 0x63, 0x65, 0x6e, 0x74, 0x65, 0x72, 0x18, 0x02, 0x20,
    0x02, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x32, 0x66, 0x12, 0x0e,
    0x0a, 0x06, 0x72, 0x61, 0x64, 0x69, 0x75, 0x73, 0x18, 0x03, 0x20, 0x02, 0x28, 0x02, 0x12, 0x0a,
    0x0a, 0x02, 0x61, 0x31, 0x18, 0x04, 0x20, 0x02, 0x28, 0x02, 0x12, 0x0a, 0x0a, 0x02, 0x61, 0x32,
    0x18, 0x05, 0x20, 0x02, 0x28, 0x02, 0x12, 0x11, 0x0a, 0x09, 0x74, 0x68, 0x69, 0x63, 0x6b, 0x6e,
    0x65, 0x73, 0x73, 0x18, 0x06, 0x20, 0x02, 0x28, 0x02, 0x22, 0xd8, 0x01, 0x0a, 0x15, 0x53, 0x53,
    0x4c, 0x5f, 0x47, 0x65, 0x6f, 0x6d, 0x65, 0x74, 0x72, 0x79, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x53,
    0x69, 0x7a, 0x65, 0x12, 0x14, 0x0a, 0x0c, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x6c, 0x65, 0x6e,
    0x67, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x5f, 0x77, 0x69, 0x64, 0x74, 0x68, 0x18, 0x02, 0x20, 0x02, 0x28, 0x05, 0x12, 0x12,
    0x0a, 0x0a, 0x67, 0x6f, 0x61, 0x6c, 0x5f, 0x77, 0x69, 0x64, 0x74, 0x68, 0x18, 0x03, 0x20, 0x02,
    0x28, 0x05, 0x12, 0x12, 0x0a, 0x0a, 0x67, 0x6f, 0x61, 0x6c, 0x5f, 0x64, 0x65, 0x70, 0x74, 0x68,
    0x18, 0x04, 0x20, 0x02, 0x28, 0x05, 0x12, 0x16, 0x0a, 0x0e, 0x62, 0x6f, 0x75, 0x6e, 0x64, 0x61,
    0x72, 0x79, 0x5f, 0x77, 0x69, 0x64, 0x74, 0x68, 0x18, 0x05, 0x20, 0x02, 0x28, 0x05, 0x12, 0x2a,
    0x0a, 0x0b, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x6c, 0x69, 0x6e, 0x65, 0x73, 0x18, 0x06, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x53, 0x53, 0x4c, 0x5f, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x4c,
    0x69, 0x6e, 0x65, 0x53, 0x65, 0x67, 0x6d, 0x65, 0x6e, 0x74, 0x12, 0x28, 0x0a, 0x0a, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x5f, 0x61, 0x72, 0x63, 0x73, 0x18, 0x07, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x14,
    0x2e, 0x53, 0x53, 0x4c, 0x5f, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x43, 0x69, 0x63, 0x75, 0x6c, 0x61,
    0x72, 0x41, 0x72, 0x63, 0x22, 0xc9, 0x02, 0x0a, 0x1d, 0x53, 0x53, 0x4c, 0x5f, 0x47, 0x65, 0x6f,
    0x6d, 0x65, 0x74, 0x72, 0x79, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x43, 0x61, 0x6c, 0x69, 0x62,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61,
    0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x14, 0x0a, 0x0c, 0x66, 0x6f, 0x63,
    0x61, 0x6c, 0x5f, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x18, 0x02, 0x20, 0x02, 0x28, 0x02, 0x12,
    0x19, 0x0a, 0x11, 0x70, 0x72, 0x69, 0x6e, 0x63, 0x69, 0x70, 0x61, 0x6c, 0x5f, 0x70, 0x6f, 0x69,
    0x6e, 0x74, 0x5f, 0x78, 0x18, 0x03, 0x20, 0x02, 0x28, 0x02, 0x12, 0x19, 0x0a, 0x11, 0x70, 0x72,
    0x69, 0x6e, 0x63, 0x69, 0x70, 0x61, 0x6c, 0x5f, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x5f, 0x79, 0x18,
    0x04, 0x20, 0x02, 0x28, 0x02, 0x12, 0x12, 0x0a, 0x0a, 0x64, 0x69, 0x73, 0x74, 0x6f, 0x72, 0x74,
    0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x02, 0x28, 0x02, 0x12, 0x0a, 0x0a, 0x02, 0x71, 0x30, 0x18,
    0x06, 0x20, 0x02, 0x28, 0x02, 0x12, 0x0a, 0x0a, 0x02, 0x71, 0x31, 0x18, 0x07, 0x20, 0x02, 0x28,
    0x02, 0x12, 0x0a, 0x0a, 0x02, 0x71, 0x32, 0x18, 0x08, 0x20, 0x02, 0x28, 0x02, 0x12, 0x0a, 0x0a,
    0x02, 0x71, 0x33, 0x18, 0x09, 0x20, 0x02, 0x28, 0x02, 0x12, 0x0a, 0x0a, 0x02, 0x74, 0x78, 0x18,
    0x0a, 0x20, 0x02, 0x28, 0x02, 0x12, 0x0a, 0x0a, 0x02, 0x74, 0x79, 0x18, 0x0b, 0x20, 0x02, 0x28,
    0x02, 0x12, 0x0a, 0x0a, 0x02, 0x74, 0x7a, 0x18, 0x0c, 0x20, 0x02, 0x28, 0x02, 0x12, 0x1f, 0x0a,
    0x17, 0x64, 0x65, 0x72, 0x69, 0x76, 0x65, 0x64, 0x5f, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x5f,
    0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x74, 0x78, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x02, 0x12, 0x1f,
    0x0a, 0x17, 0x64, 0x65, 0x72, 0x69, 0x76, 0x65, 0x64, 0x5f, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61,
    0x5f, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x74, 0x79, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x02, 0x12,
    0x1f, 0x0a, 0x17, 0x64, 0x65, 0x72, 0x69, 0x76, 0x65, 0x64, 0x5f, 0x63, 0x61, 0x6d, 0x65, 0x72,
    0x61, 0x5f, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x74, 0x7a, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x02,
    0x22, 0x68, 0x0a, 0x10, 0x53, 0x53, 0x4c, 0x5f, 0x47, 0x65, 0x6f, 0x6d, 0x65, 0x74, 0x72, 0x79,
    0x44, 0x61, 0x74, 0x61, 0x12, 0x25, 0x0a, 0x05, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x53, 0x53, 0x4c, 0x5f, 0x47, 0x65, 0x6f, 0x6d, 0x65, 0x74,
    0x72, 0x79, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x53, 0x69, 0x7a, 0x65, 0x12, 0x2d, 0x0a, 0x05, 0x63,
    0x61, 0x6c, 0x69, 0x62, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x53, 0x53, 0x4c,
    0x5f, 0x47, 0x65, 0x6f, 0x6d, 0x65, 0x74, 0x72, 0x79, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x43,
    0x61, 0x6c, 0x69, 0x62, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4a, 0xc6, 0x1a, 0x0a, 0x06, 0x12,
    0x04, 0x01, 0x00, 0x45, 0x01, 0x0a, 0x20, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x01, 0x00, 0x04,
    0x01, 0x1a, 0x14, 0x20, 0x41, 0x20, 0x32, 0x44, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x20, 0x76,
    0x65, 0x63, 0x74, 0x6f, 0x72, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x01, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x02, 0x02, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x02, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x02, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x02, 0x11, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x02, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x03, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x03, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x03, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x03, 0x11, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x03, 0x15, 0x16, 0x0a, 0x94, 0x02,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0a, 0x00, 0x13, 0x01, 0x1a, 0x87, 0x02, 0x20, 0x52, 0x65,
    0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x61, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x20, 0x6d, 0x61, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x73, 0x20, 0x61, 0x20, 0x6c, 0x69,
    0x6e, 0x65, 0x20, 0x73, 0x65, 0x67, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65,
    0x73, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x61, 0x20, 0x73, 0x74, 0x61, 0x72,
    0x74, 0x20, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x70, 0x31, 0x2c, 0x0a, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x65, 0x6e, 0x64, 0x20, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x70, 0x32, 0x2c, 0x20, 0x61,
    0x6e, 0x64, 0x20, 0x61, 0x20, 0x6c, 0x69, 0x6e, 0x65, 0x20, 0x74, 0x68, 0x69, 0x63, 0x6b, 0x6e,
    0x65, 0x73, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x61,
    0x6e, 0x64, 0x20, 0x65, 0x6e, 0x64, 0x20, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x73, 0x20, 0x61, 0x72,
    0x65, 0x20, 0x61, 0x6c, 0x6f, 0x6e, 0x67, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x65, 0x6e,
    0x74, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x69, 0x6e, 0x65, 0x2c,
    0x20, 0x73, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x68, 0x69, 0x63, 0x6b, 0x6e, 0x65, 0x73,
    0x73, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x69, 0x6e, 0x65, 0x20, 0x65, 0x78,
    0x74, 0x65, 0x6e, 0x64, 0x73, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x69, 0x63, 0x6b, 0x6e, 0x65,
    0x73, 0x73, 0x20, 0x2f, 0x20, 0x32, 0x0a, 0x20, 0x6f, 0x6e, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65,
    0x72, 0x20, 0x73, 0x69, 0x64, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x69,
    0x6e, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x1c,
    0x0a, 0x2a, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x02, 0x1b, 0x1a, 0x1d, 0x20,
    0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x20, 0x6d, 0x61, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x0c, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x0c, 0x19, 0x1a, 0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x02,
    0x1b, 0x1a, 0x22, 0x20, 0x53, 0x74, 0x61, 0x72, 0x74, 0x20, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x69, 0x6e, 0x65, 0x20, 0x73, 0x65, 0x67, 0x6d,
    0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0e, 0x0b,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x14, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0e, 0x19, 0x1a, 0x0a, 0x2d, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x10, 0x02, 0x1b, 0x1a, 0x20, 0x20, 0x45, 0x6e, 0x64,
    0x20, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x69,
    0x6e, 0x65, 0x20, 0x73, 0x65, 0x67, 0x6d, 0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x06, 0x12, 0x03, 0x10, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x10, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x10, 0x19, 0x1a, 0x0a, 0x2d, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x12, 0x02,
    0x1f, 0x1a, 0x20, 0x20, 0x54, 0x68, 0x69, 0x63, 0x6b, 0x6e, 0x65, 0x73, 0x73, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x69, 0x6e, 0x65, 0x20, 0x73, 0x65, 0x67, 0x6d, 0x65, 0x6e,
    0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x12, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x12, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x12, 0x11, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x12, 0x1d, 0x1e, 0x0a, 0x94, 0x01, 0x0a, 0x02,
    0x04, 0x02, 0x12, 0x04, 0x17, 0x00, 0x24, 0x01, 0x1a, 0x87, 0x01, 0x20, 0x52, 0x65, 0x70, 0x72,
    0x65, 0x73, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x61, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x6d,
    0x61, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x73, 0x20, 0x61, 0x20, 0x63, 0x69, 0x72, 0x63,
    0x75, 0x6c, 0x61, 0x72, 0x20, 0x61, 0x72, 0x63, 0x20, 0x73, 0x65, 0x67, 0x6d, 0x65, 0x6e, 0x74,
    0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20,
    0x63, 0x65, 0x6e, 0x74, 0x65, 0x72, 0x20, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x2c, 0x20, 0x61, 0x0a,
    0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x2c, 0x20, 0x61, 0x6e,
    0x20, 0x65, 0x6e, 0x64, 0x20, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x61, 0x6e, 0x20, 0x61, 0x72, 0x63, 0x20, 0x74, 0x68, 0x69, 0x63, 0x6b, 0x6e, 0x65, 0x73, 0x73,
    0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x17, 0x08, 0x1b, 0x0a, 0x2a,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x19, 0x02, 0x1b, 0x1a, 0x1d, 0x20, 0x4e, 0x61,
    0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x20, 0x6d, 0x61, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x19, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x19, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19,
    0x19, 0x1a, 0x0a, 0x30, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x02, 0x1f, 0x1a,
    0x23, 0x20, 0x43, 0x65, 0x6e, 0x74, 0x65, 0x72, 0x20, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x6f,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x69, 0x72, 0x63, 0x75, 0x6c, 0x61, 0x72, 0x20, 0x61,
    0x72, 0x63, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1b,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x1b, 0x0b, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1b, 0x14, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1b, 0x1d, 0x1e, 0x0a, 0x21, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1d, 0x02, 0x1c, 0x1a, 0x14, 0x20, 0x52, 0x61, 0x64, 0x69,
    0x75, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x72, 0x63, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x1d, 0x1a, 0x1b, 0x0a, 0x36, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x1f, 0x02, 0x18, 0x1a, 0x29, 0x20, 0x53, 0x74, 0x61, 0x72, 0x74, 0x20, 0x61, 0x6e, 0x67,
    0x6c, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x2d, 0x63, 0x6c,
    0x6f, 0x63, 0x6b, 0x77, 0x69, 0x73, 0x65, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x1f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x1f, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1f, 0x11, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x1f, 0x16, 0x17, 0x0a, 0x34, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x21, 0x02, 0x18, 0x1a, 0x27, 0x20, 0x45, 0x6e, 0x64, 0x20, 0x61, 0x6e, 0x67, 0x6c, 0x65,
    0x20, 0x69, 0x6e, 0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x2d, 0x63, 0x6c, 0x6f, 0x63,
    0x6b, 0x77, 0x69, 0x73, 0x65, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x03, 0x21, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x21, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x21, 0x11, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x21, 0x16, 0x17, 0x0a, 0x24, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x23,
    0x02, 0x1f, 0x1a, 0x17, 0x20, 0x54, 0x68, 0x69, 0x63, 0x6b, 0x6e, 0x65, 0x73, 0x73, 0x20, 0x6f,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x72, 0x63, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x05, 0x04, 0x12, 0x03, 0x23, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x05, 0x05, 0x12, 0x03, 0x23, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01,
    0x12, 0x03, 0x23, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03,
    0x23, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x26, 0x00, 0x2e, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x26, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x00, 0x12, 0x03, 0x27, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x27, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x27, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x27,
    0x11, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x27, 0x20, 0x21,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x28, 0x02, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x28, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x28, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x28, 0x11, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x28, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x29,
    0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x29, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x29, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x29, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x29, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x03, 0x12, 0x03, 0x2a, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x2a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x2a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2a, 0x11,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2a, 0x1e, 0x1f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x2b, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x04, 0x04, 0x12, 0x03, 0x2b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x2b, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x2b, 0x11, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x2b, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x05, 0x12, 0x03, 0x2c, 0x02,
    0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x04, 0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x06, 0x12, 0x03, 0x2c, 0x0b, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x05, 0x01, 0x12, 0x03, 0x2c, 0x20, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x05, 0x03, 0x12, 0x03, 0x2c, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x06, 0x12, 0x03, 0x2d, 0x02, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x04, 0x12,
    0x03, 0x2d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x06, 0x12, 0x03, 0x2d,
    0x0b, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x01, 0x12, 0x03, 0x2d, 0x1f, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x03, 0x12, 0x03, 0x2d, 0x2c, 0x2d, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x30, 0x00, 0x40, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04,
    0x01, 0x12, 0x03, 0x30, 0x08, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03,
    0x31, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x31, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x31, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x31, 0x12, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x31, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x01, 0x12, 0x03, 0x32, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x32, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x32, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x32,
    0x11, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x32, 0x20, 0x21,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x33, 0x02, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x33, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x33, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x33, 0x11, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x33, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x34,
    0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x04, 0x12, 0x03, 0x34, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x05, 0x12, 0x03, 0x34, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x34, 0x11, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x34, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x04, 0x12, 0x03, 0x35, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x04,
    0x12, 0x03, 0x35, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x05, 0x12, 0x03,
    0x35, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x01, 0x12, 0x03, 0x35, 0x11,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x03, 0x12, 0x03, 0x35, 0x1e, 0x1f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x05, 0x12, 0x03, 0x36, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x05, 0x04, 0x12, 0x03, 0x36, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x05, 0x05, 0x12, 0x03, 0x36, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x36, 0x11, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x03, 0x12,
    0x03, 0x36, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x06, 0x12, 0x03, 0x37, 0x02,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x04, 0x12, 0x03, 0x37, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x05, 0x12, 0x03, 0x37, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x06, 0x01, 0x12, 0x03, 0x37, 0x11, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x06, 0x03, 0x12, 0x03, 0x37, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x07, 0x12, 0x03, 0x38, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x04, 0x12,
    0x03, 0x38, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x05, 0x12, 0x03, 0x38,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x01, 0x12, 0x03, 0x38, 0x11, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x03, 0x12, 0x03, 0x38, 0x16, 0x17, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x08, 0x12, 0x03, 0x39, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x08, 0x04, 0x12, 0x03, 0x39, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x08, 0x05, 0x12, 0x03, 0x39, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x01,
    0x12, 0x03, 0x39, 0x11, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x03, 0x12, 0x03,
    0x39, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x09, 0x12, 0x03, 0x3a, 0x02, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x04, 0x12, 0x03, 0x3a, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x05, 0x12, 0x03, 0x3a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x09, 0x01, 0x12, 0x03, 0x3a, 0x11, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x09, 0x03, 0x12, 0x03, 0x3a, 0x16, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0a,
    0x12, 0x03, 0x3b, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a, 0x04, 0x12, 0x03,
    0x3b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x3b, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x3b, 0x11, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x3b, 0x16, 0x18, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x0b, 0x12, 0x03, 0x3c, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x0b, 0x04, 0x12, 0x03, 0x3c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0b,
    0x05, 0x12, 0x03, 0x3c, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0b, 0x01, 0x12,
    0x03, 0x3c, 0x11, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x3c,
    0x16, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0c, 0x12, 0x03, 0x3d, 0x02, 0x2e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x3d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x3d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x3d, 0x11, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x0c, 0x03, 0x12, 0x03, 0x3d, 0x2b, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0d, 0x12,
    0x03, 0x3e, 0x02, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x3e,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x3e, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x3e, 0x11, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x3e, 0x2b, 0x2d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x0e, 0x12, 0x03, 0x3f, 0x02, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x0e, 0x04, 0x12, 0x03, 0x3f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0e, 0x05,
    0x12, 0x03, 0x3f, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0e, 0x01, 0x12, 0x03,
    0x3f, 0x11, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x3f, 0x2b,
    0x2d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x42, 0x00, 0x45, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x42, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x00, 0x12, 0x03, 0x43, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x43, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x43,
    0x0b, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x43, 0x21, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x43, 0x29, 0x2a, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x44, 0x02, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x44, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x44, 0x0b, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x44, 0x29, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x44, 0x31, 0x32,
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
