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

#[derive(Clone,Default)]
pub struct Command {
    // message oneof groups
    CommandType: ::std::option::Option<Command_oneof_CommandType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Command {}

#[derive(Clone,PartialEq)]
pub enum Command_oneof_CommandType {
    set_light(SetLight),
    set_group(SetGroup),
    initialize(Initialize),
}

impl Command {
    pub fn new() -> Command {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Command {
        static mut instance: ::protobuf::lazy::Lazy<Command> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Command,
        };
        unsafe {
            instance.get(|| {
                Command {
                    CommandType: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .light_controller.SetLight set_light = 1;

    pub fn clear_set_light(&mut self) {
        self.CommandType = ::std::option::Option::None;
    }

    pub fn has_set_light(&self) -> bool {
        match self.CommandType {
            ::std::option::Option::Some(Command_oneof_CommandType::set_light(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_light(&mut self, v: SetLight) {
        self.CommandType = ::std::option::Option::Some(Command_oneof_CommandType::set_light(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set_light(&mut self) -> &mut SetLight {
        if let ::std::option::Option::Some(Command_oneof_CommandType::set_light(_)) = self.CommandType {
        } else {
            self.CommandType = ::std::option::Option::Some(Command_oneof_CommandType::set_light(SetLight::new()));
        }
        match self.CommandType {
            ::std::option::Option::Some(Command_oneof_CommandType::set_light(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_light(&mut self) -> SetLight {
        if self.has_set_light() {
            match self.CommandType.take() {
                ::std::option::Option::Some(Command_oneof_CommandType::set_light(v)) => v,
                _ => panic!(),
            }
        } else {
            SetLight::new()
        }
    }

    pub fn get_set_light(&self) -> &SetLight {
        match self.CommandType {
            ::std::option::Option::Some(Command_oneof_CommandType::set_light(ref v)) => v,
            _ => SetLight::default_instance(),
        }
    }

    // optional .light_controller.SetGroup set_group = 2;

    pub fn clear_set_group(&mut self) {
        self.CommandType = ::std::option::Option::None;
    }

    pub fn has_set_group(&self) -> bool {
        match self.CommandType {
            ::std::option::Option::Some(Command_oneof_CommandType::set_group(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_group(&mut self, v: SetGroup) {
        self.CommandType = ::std::option::Option::Some(Command_oneof_CommandType::set_group(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set_group(&mut self) -> &mut SetGroup {
        if let ::std::option::Option::Some(Command_oneof_CommandType::set_group(_)) = self.CommandType {
        } else {
            self.CommandType = ::std::option::Option::Some(Command_oneof_CommandType::set_group(SetGroup::new()));
        }
        match self.CommandType {
            ::std::option::Option::Some(Command_oneof_CommandType::set_group(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_group(&mut self) -> SetGroup {
        if self.has_set_group() {
            match self.CommandType.take() {
                ::std::option::Option::Some(Command_oneof_CommandType::set_group(v)) => v,
                _ => panic!(),
            }
        } else {
            SetGroup::new()
        }
    }

    pub fn get_set_group(&self) -> &SetGroup {
        match self.CommandType {
            ::std::option::Option::Some(Command_oneof_CommandType::set_group(ref v)) => v,
            _ => SetGroup::default_instance(),
        }
    }

    // optional .light_controller.Initialize initialize = 3;

    pub fn clear_initialize(&mut self) {
        self.CommandType = ::std::option::Option::None;
    }

    pub fn has_initialize(&self) -> bool {
        match self.CommandType {
            ::std::option::Option::Some(Command_oneof_CommandType::initialize(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_initialize(&mut self, v: Initialize) {
        self.CommandType = ::std::option::Option::Some(Command_oneof_CommandType::initialize(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_initialize(&mut self) -> &mut Initialize {
        if let ::std::option::Option::Some(Command_oneof_CommandType::initialize(_)) = self.CommandType {
        } else {
            self.CommandType = ::std::option::Option::Some(Command_oneof_CommandType::initialize(Initialize::new()));
        }
        match self.CommandType {
            ::std::option::Option::Some(Command_oneof_CommandType::initialize(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_initialize(&mut self) -> Initialize {
        if self.has_initialize() {
            match self.CommandType.take() {
                ::std::option::Option::Some(Command_oneof_CommandType::initialize(v)) => v,
                _ => panic!(),
            }
        } else {
            Initialize::new()
        }
    }

    pub fn get_initialize(&self) -> &Initialize {
        match self.CommandType {
            ::std::option::Option::Some(Command_oneof_CommandType::initialize(ref v)) => v,
            _ => Initialize::default_instance(),
        }
    }
}

impl ::protobuf::Message for Command {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.CommandType = ::std::option::Option::Some(Command_oneof_CommandType::set_light(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.CommandType = ::std::option::Option::Some(Command_oneof_CommandType::set_group(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.CommandType = ::std::option::Option::Some(Command_oneof_CommandType::initialize(try!(is.read_message())));
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
        if let ::std::option::Option::Some(ref v) = self.CommandType {
            match v {
                &Command_oneof_CommandType::set_light(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Command_oneof_CommandType::set_group(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Command_oneof_CommandType::initialize(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.CommandType {
            match v {
                &Command_oneof_CommandType::set_light(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Command_oneof_CommandType::set_group(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Command_oneof_CommandType::initialize(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
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
        ::std::any::TypeId::of::<Command>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Command {
    fn new() -> Command {
        Command::new()
    }

    fn descriptor_static(_: ::std::option::Option<Command>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "set_light",
                    Command::has_set_light,
                    Command::get_set_light,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "set_group",
                    Command::has_set_group,
                    Command::get_set_group,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "initialize",
                    Command::has_initialize,
                    Command::get_initialize,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Command>(
                    "Command",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Command {
    fn clear(&mut self) {
        self.clear_set_light();
        self.clear_set_group();
        self.clear_initialize();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Command {
    fn eq(&self, other: &Command) -> bool {
        self.CommandType == other.CommandType &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Command {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetLight {
    // message fields
    light_group: ::std::option::Option<u32>,
    light_id: ::std::option::Option<u32>,
    color: ::protobuf::SingularPtrField<Color>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetLight {}

impl SetLight {
    pub fn new() -> SetLight {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetLight {
        static mut instance: ::protobuf::lazy::Lazy<SetLight> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetLight,
        };
        unsafe {
            instance.get(|| {
                SetLight {
                    light_group: ::std::option::Option::None,
                    light_id: ::std::option::Option::None,
                    color: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 light_group = 1;

    pub fn clear_light_group(&mut self) {
        self.light_group = ::std::option::Option::None;
    }

    pub fn has_light_group(&self) -> bool {
        self.light_group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_light_group(&mut self, v: u32) {
        self.light_group = ::std::option::Option::Some(v);
    }

    pub fn get_light_group(&self) -> u32 {
        self.light_group.unwrap_or(0)
    }

    // optional uint32 light_id = 2;

    pub fn clear_light_id(&mut self) {
        self.light_id = ::std::option::Option::None;
    }

    pub fn has_light_id(&self) -> bool {
        self.light_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_light_id(&mut self, v: u32) {
        self.light_id = ::std::option::Option::Some(v);
    }

    pub fn get_light_id(&self) -> u32 {
        self.light_id.unwrap_or(0)
    }

    // optional .light_controller.Color color = 3;

    pub fn clear_color(&mut self) {
        self.color.clear();
    }

    pub fn has_color(&self) -> bool {
        self.color.is_some()
    }

    // Param is passed by value, moved
    pub fn set_color(&mut self, v: Color) {
        self.color = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_color(&mut self) -> &mut Color {
        if self.color.is_none() {
            self.color.set_default();
        };
        self.color.as_mut().unwrap()
    }

    // Take field
    pub fn take_color(&mut self) -> Color {
        self.color.take().unwrap_or_else(|| Color::new())
    }

    pub fn get_color(&self) -> &Color {
        self.color.as_ref().unwrap_or_else(|| Color::default_instance())
    }
}

impl ::protobuf::Message for SetLight {
    fn is_initialized(&self) -> bool {
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
                    self.light_group = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.light_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.color));
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
        for value in &self.light_group {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.light_id {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.color {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.light_group {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.light_id {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.color.as_ref() {
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
        ::std::any::TypeId::of::<SetLight>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetLight {
    fn new() -> SetLight {
        SetLight::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetLight>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "light_group",
                    SetLight::has_light_group,
                    SetLight::get_light_group,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "light_id",
                    SetLight::has_light_id,
                    SetLight::get_light_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "color",
                    SetLight::has_color,
                    SetLight::get_color,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetLight>(
                    "SetLight",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetLight {
    fn clear(&mut self) {
        self.clear_light_group();
        self.clear_light_id();
        self.clear_color();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetLight {
    fn eq(&self, other: &SetLight) -> bool {
        self.light_group == other.light_group &&
        self.light_id == other.light_id &&
        self.color == other.color &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetLight {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetGroup {
    // message fields
    light_group: ::std::option::Option<u32>,
    color: ::protobuf::SingularPtrField<Color>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetGroup {}

impl SetGroup {
    pub fn new() -> SetGroup {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetGroup {
        static mut instance: ::protobuf::lazy::Lazy<SetGroup> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetGroup,
        };
        unsafe {
            instance.get(|| {
                SetGroup {
                    light_group: ::std::option::Option::None,
                    color: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 light_group = 1;

    pub fn clear_light_group(&mut self) {
        self.light_group = ::std::option::Option::None;
    }

    pub fn has_light_group(&self) -> bool {
        self.light_group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_light_group(&mut self, v: u32) {
        self.light_group = ::std::option::Option::Some(v);
    }

    pub fn get_light_group(&self) -> u32 {
        self.light_group.unwrap_or(0)
    }

    // optional .light_controller.Color color = 2;

    pub fn clear_color(&mut self) {
        self.color.clear();
    }

    pub fn has_color(&self) -> bool {
        self.color.is_some()
    }

    // Param is passed by value, moved
    pub fn set_color(&mut self, v: Color) {
        self.color = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_color(&mut self) -> &mut Color {
        if self.color.is_none() {
            self.color.set_default();
        };
        self.color.as_mut().unwrap()
    }

    // Take field
    pub fn take_color(&mut self) -> Color {
        self.color.take().unwrap_or_else(|| Color::new())
    }

    pub fn get_color(&self) -> &Color {
        self.color.as_ref().unwrap_or_else(|| Color::default_instance())
    }
}

impl ::protobuf::Message for SetGroup {
    fn is_initialized(&self) -> bool {
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
                    self.light_group = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.color));
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
        for value in &self.light_group {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.color {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.light_group {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.color.as_ref() {
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
        ::std::any::TypeId::of::<SetGroup>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetGroup {
    fn new() -> SetGroup {
        SetGroup::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetGroup>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "light_group",
                    SetGroup::has_light_group,
                    SetGroup::get_light_group,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "color",
                    SetGroup::has_color,
                    SetGroup::get_color,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetGroup>(
                    "SetGroup",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetGroup {
    fn clear(&mut self) {
        self.clear_light_group();
        self.clear_color();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetGroup {
    fn eq(&self, other: &SetGroup) -> bool {
        self.light_group == other.light_group &&
        self.color == other.color &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetGroup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Color {
    // message fields
    red: ::std::option::Option<i32>,
    green: ::std::option::Option<i32>,
    blue: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Color {}

impl Color {
    pub fn new() -> Color {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Color {
        static mut instance: ::protobuf::lazy::Lazy<Color> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Color,
        };
        unsafe {
            instance.get(|| {
                Color {
                    red: ::std::option::Option::None,
                    green: ::std::option::Option::None,
                    blue: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 red = 1;

    pub fn clear_red(&mut self) {
        self.red = ::std::option::Option::None;
    }

    pub fn has_red(&self) -> bool {
        self.red.is_some()
    }

    // Param is passed by value, moved
    pub fn set_red(&mut self, v: i32) {
        self.red = ::std::option::Option::Some(v);
    }

    pub fn get_red(&self) -> i32 {
        self.red.unwrap_or(0)
    }

    // optional int32 green = 2;

    pub fn clear_green(&mut self) {
        self.green = ::std::option::Option::None;
    }

    pub fn has_green(&self) -> bool {
        self.green.is_some()
    }

    // Param is passed by value, moved
    pub fn set_green(&mut self, v: i32) {
        self.green = ::std::option::Option::Some(v);
    }

    pub fn get_green(&self) -> i32 {
        self.green.unwrap_or(0)
    }

    // optional int32 blue = 3;

    pub fn clear_blue(&mut self) {
        self.blue = ::std::option::Option::None;
    }

    pub fn has_blue(&self) -> bool {
        self.blue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blue(&mut self, v: i32) {
        self.blue = ::std::option::Option::Some(v);
    }

    pub fn get_blue(&self) -> i32 {
        self.blue.unwrap_or(0)
    }
}

impl ::protobuf::Message for Color {
    fn is_initialized(&self) -> bool {
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
                    self.red = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.green = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.blue = ::std::option::Option::Some(tmp);
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
        for value in &self.red {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.green {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.blue {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.red {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.green {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.blue {
            try!(os.write_int32(3, v));
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
        ::std::any::TypeId::of::<Color>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Color {
    fn new() -> Color {
        Color::new()
    }

    fn descriptor_static(_: ::std::option::Option<Color>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "red",
                    Color::has_red,
                    Color::get_red,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "green",
                    Color::has_green,
                    Color::get_green,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "blue",
                    Color::has_blue,
                    Color::get_blue,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Color>(
                    "Color",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Color {
    fn clear(&mut self) {
        self.clear_red();
        self.clear_green();
        self.clear_blue();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.red == other.red &&
        self.green == other.green &&
        self.blue == other.blue &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Color {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Initialize {
    // message fields
    string_lengths: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Initialize {}

impl Initialize {
    pub fn new() -> Initialize {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Initialize {
        static mut instance: ::protobuf::lazy::Lazy<Initialize> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Initialize,
        };
        unsafe {
            instance.get(|| {
                Initialize {
                    string_lengths: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated uint32 string_lengths = 1;

    pub fn clear_string_lengths(&mut self) {
        self.string_lengths.clear();
    }

    // Param is passed by value, moved
    pub fn set_string_lengths(&mut self, v: ::std::vec::Vec<u32>) {
        self.string_lengths = v;
    }

    // Mutable pointer to the field.
    pub fn mut_string_lengths(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.string_lengths
    }

    // Take field
    pub fn take_string_lengths(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.string_lengths, ::std::vec::Vec::new())
    }

    pub fn get_string_lengths(&self) -> &[u32] {
        &self.string_lengths
    }
}

impl ::protobuf::Message for Initialize {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.string_lengths));
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
        for value in &self.string_lengths {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.string_lengths {
            try!(os.write_uint32(1, *v));
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
        ::std::any::TypeId::of::<Initialize>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Initialize {
    fn new() -> Initialize {
        Initialize::new()
    }

    fn descriptor_static(_: ::std::option::Option<Initialize>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_u32_accessor(
                    "string_lengths",
                    Initialize::get_string_lengths,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Initialize>(
                    "Initialize",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Initialize {
    fn clear(&mut self) {
        self.clear_string_lengths();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Initialize {
    fn eq(&self, other: &Initialize) -> bool {
        self.string_lengths == other.string_lengths &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Initialize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x10, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c,
    0x65, 0x72, 0x22, 0xae, 0x01, 0x0a, 0x07, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x12, 0x2f,
    0x0a, 0x09, 0x73, 0x65, 0x74, 0x5f, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x1a, 0x2e, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f,
    0x6c, 0x6c, 0x65, 0x72, 0x2e, 0x53, 0x65, 0x74, 0x4c, 0x69, 0x67, 0x68, 0x74, 0x48, 0x00, 0x12,
    0x2f, 0x0a, 0x09, 0x73, 0x65, 0x74, 0x5f, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2e, 0x53, 0x65, 0x74, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x48, 0x00,
    0x12, 0x32, 0x0a, 0x0a, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x5f, 0x63, 0x6f, 0x6e,
    0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2e, 0x49, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x6c, 0x69,
    0x7a, 0x65, 0x48, 0x00, 0x42, 0x0d, 0x0a, 0x0b, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x54,
    0x79, 0x70, 0x65, 0x22, 0x59, 0x0a, 0x08, 0x53, 0x65, 0x74, 0x4c, 0x69, 0x67, 0x68, 0x74, 0x12,
    0x13, 0x0a, 0x0b, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x5f, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0d, 0x12, 0x10, 0x0a, 0x08, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x5f, 0x69, 0x64,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x26, 0x0a, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x5f, 0x63, 0x6f,
    0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2e, 0x43, 0x6f, 0x6c, 0x6f, 0x72, 0x22, 0x47,
    0x0a, 0x08, 0x53, 0x65, 0x74, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x12, 0x13, 0x0a, 0x0b, 0x6c, 0x69,
    0x67, 0x68, 0x74, 0x5f, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12,
    0x26, 0x0a, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17,
    0x2e, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65,
    0x72, 0x2e, 0x43, 0x6f, 0x6c, 0x6f, 0x72, 0x22, 0x31, 0x0a, 0x05, 0x43, 0x6f, 0x6c, 0x6f, 0x72,
    0x12, 0x0b, 0x0a, 0x03, 0x72, 0x65, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0d, 0x0a,
    0x05, 0x67, 0x72, 0x65, 0x65, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0c, 0x0a, 0x04,
    0x62, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x22, 0x24, 0x0a, 0x0a, 0x49, 0x6e,
    0x69, 0x74, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x12, 0x16, 0x0a, 0x0e, 0x73, 0x74, 0x72, 0x69,
    0x6e, 0x67, 0x5f, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0d,
    0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
