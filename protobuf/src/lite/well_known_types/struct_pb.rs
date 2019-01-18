// This file is generated by rust-protobuf 2.2.4. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default,Debug)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Struct {
    // message fields
    pub fields: ::std::collections::HashMap<::std::string::String, Value>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl Struct {
    pub fn new() -> Struct {
        ::std::default::Default::default()
    }

    // repeated .google.protobuf.Struct.FieldsEntry fields = 1;

    pub fn clear_fields(&mut self) {
        self.fields.clear();
    }

    // Param is passed by value, moved
    pub fn set_fields(&mut self, v: ::std::collections::HashMap<::std::string::String, Value>) {
        self.fields = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fields(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, Value> {
        &mut self.fields
    }

    // Take field
    pub fn take_fields(&mut self) -> ::std::collections::HashMap<::std::string::String, Value> {
        ::std::mem::replace(&mut self.fields, ::std::collections::HashMap::new())
    }

    pub fn get_fields(&self) -> &::std::collections::HashMap<::std::string::String, Value> {
        &self.fields
    }
}

impl ::protobuf::Message for Struct {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Value>>(wire_type, is, &mut self.fields)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Value>>(1, &self.fields);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Value>>(1, &self.fields, os)?;
        os.write_unknown_fields(self.get_unknown_fields())?;
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


    fn new() -> Struct {
        Struct::new()
    }
}

impl ::protobuf::Clear for Struct {
    fn clear(&mut self) {
        self.clear_fields();
        self.unknown_fields.clear();
    }
}

#[derive(PartialEq,Clone,Default,Debug)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Value {
    // message oneof groups
    pub kind: ::std::option::Option<Value_oneof_kind>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

#[derive(Clone,PartialEq,Debug)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum Value_oneof_kind {
    null_value(NullValue),
    number_value(f64),
    string_value(::std::string::String),
    bool_value(bool),
    struct_value(Struct),
    list_value(ListValue),
}

impl Value {
    pub fn new() -> Value {
        ::std::default::Default::default()
    }

    // .google.protobuf.NullValue null_value = 1;

    pub fn clear_null_value(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_null_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::null_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_null_value(&mut self, v: NullValue) {
        self.kind = ::std::option::Option::Some(Value_oneof_kind::null_value(v))
    }

    pub fn get_null_value(&self) -> NullValue {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::null_value(v)) => v,
            _ => NullValue::NULL_VALUE,
        }
    }

    // double number_value = 2;

    pub fn clear_number_value(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_number_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::number_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_number_value(&mut self, v: f64) {
        self.kind = ::std::option::Option::Some(Value_oneof_kind::number_value(v))
    }

    pub fn get_number_value(&self) -> f64 {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::number_value(v)) => v,
            _ => 0.,
        }
    }

    // string string_value = 3;

    pub fn clear_string_value(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_string_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::string_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_string_value(&mut self, v: ::std::string::String) {
        self.kind = ::std::option::Option::Some(Value_oneof_kind::string_value(v))
    }

    // Mutable pointer to the field.
    pub fn mut_string_value(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Value_oneof_kind::string_value(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(Value_oneof_kind::string_value(::std::string::String::new()));
        }
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::string_value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_string_value(&mut self) -> ::std::string::String {
        if self.has_string_value() {
            match self.kind.take() {
                ::std::option::Option::Some(Value_oneof_kind::string_value(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_string_value(&self) -> &str {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::string_value(ref v)) => v,
            _ => "",
        }
    }

    // bool bool_value = 4;

    pub fn clear_bool_value(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_bool_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::bool_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bool_value(&mut self, v: bool) {
        self.kind = ::std::option::Option::Some(Value_oneof_kind::bool_value(v))
    }

    pub fn get_bool_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::bool_value(v)) => v,
            _ => false,
        }
    }

    // .google.protobuf.Struct struct_value = 5;

    pub fn clear_struct_value(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_struct_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::struct_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_struct_value(&mut self, v: Struct) {
        self.kind = ::std::option::Option::Some(Value_oneof_kind::struct_value(v))
    }

    // Mutable pointer to the field.
    pub fn mut_struct_value(&mut self) -> &mut Struct {
        if let ::std::option::Option::Some(Value_oneof_kind::struct_value(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(Value_oneof_kind::struct_value(Struct::new()));
        }
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::struct_value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_struct_value(&mut self) -> Struct {
        if self.has_struct_value() {
            match self.kind.take() {
                ::std::option::Option::Some(Value_oneof_kind::struct_value(v)) => v,
                _ => panic!(),
            }
        } else {
            Struct::new()
        }
    }

    pub fn get_struct_value(&self) -> &Struct {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::struct_value(ref v)) => v,
            _ => panic!(),
        }
    }

    // .google.protobuf.ListValue list_value = 6;

    pub fn clear_list_value(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_list_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::list_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_list_value(&mut self, v: ListValue) {
        self.kind = ::std::option::Option::Some(Value_oneof_kind::list_value(v))
    }

    // Mutable pointer to the field.
    pub fn mut_list_value(&mut self) -> &mut ListValue {
        if let ::std::option::Option::Some(Value_oneof_kind::list_value(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(Value_oneof_kind::list_value(ListValue::new()));
        }
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::list_value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_list_value(&mut self) -> ListValue {
        if self.has_list_value() {
            match self.kind.take() {
                ::std::option::Option::Some(Value_oneof_kind::list_value(v)) => v,
                _ => panic!(),
            }
        } else {
            ListValue::new()
        }
    }

    pub fn get_list_value(&self) -> &ListValue {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::list_value(ref v)) => v,
            _ => panic!(),
        }
    }
}

impl ::protobuf::Message for Value {
    fn is_initialized(&self) -> bool {
        if let Some(Value_oneof_kind::struct_value(ref v)) = self.kind {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Value_oneof_kind::list_value(ref v)) = self.kind {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Value_oneof_kind::null_value(is.read_enum()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Value_oneof_kind::number_value(is.read_double()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Value_oneof_kind::string_value(is.read_string()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Value_oneof_kind::bool_value(is.read_bool()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Value_oneof_kind::struct_value(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Value_oneof_kind::list_value(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.kind {
            match v {
                &Value_oneof_kind::null_value(v) => {
                    my_size += ::protobuf::rt::enum_size(1, v);
                },
                &Value_oneof_kind::number_value(v) => {
                    my_size += 9;
                },
                &Value_oneof_kind::string_value(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
                &Value_oneof_kind::bool_value(v) => {
                    my_size += 2;
                },
                &Value_oneof_kind::struct_value(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Value_oneof_kind::list_value(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.kind {
            match v {
                &Value_oneof_kind::null_value(v) => {
                    os.write_enum(1, v.value())?;
                },
                &Value_oneof_kind::number_value(v) => {
                    os.write_double(2, v)?;
                },
                &Value_oneof_kind::string_value(ref v) => {
                    os.write_string(3, v)?;
                },
                &Value_oneof_kind::bool_value(v) => {
                    os.write_bool(4, v)?;
                },
                &Value_oneof_kind::struct_value(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Value_oneof_kind::list_value(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
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


    fn new() -> Value {
        Value::new()
    }
}

impl ::protobuf::Clear for Value {
    fn clear(&mut self) {
        self.clear_null_value();
        self.clear_number_value();
        self.clear_string_value();
        self.clear_bool_value();
        self.clear_struct_value();
        self.clear_list_value();
        self.unknown_fields.clear();
    }
}

#[derive(PartialEq,Clone,Default,Debug)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct ListValue {
    // message fields
    pub values: ::protobuf::RepeatedField<Value>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl ListValue {
    pub fn new() -> ListValue {
        ::std::default::Default::default()
    }

    // repeated .google.protobuf.Value values = 1;

    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    // Param is passed by value, moved
    pub fn set_values(&mut self, v: ::protobuf::RepeatedField<Value>) {
        self.values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_values(&mut self) -> &mut ::protobuf::RepeatedField<Value> {
        &mut self.values
    }

    // Take field
    pub fn take_values(&mut self) -> ::protobuf::RepeatedField<Value> {
        ::std::mem::replace(&mut self.values, ::protobuf::RepeatedField::new())
    }

    pub fn get_values(&self) -> &[Value] {
        &self.values
    }
}

impl ::protobuf::Message for ListValue {
    fn is_initialized(&self) -> bool {
        for v in &self.values {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.values)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.values {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.values {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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


    fn new() -> ListValue {
        ListValue::new()
    }
}

impl ::protobuf::Clear for ListValue {
    fn clear(&mut self) {
        self.clear_values();
        self.unknown_fields.clear();
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum NullValue {
    NULL_VALUE = 0,
}

impl ::protobuf::ProtobufEnum for NullValue {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<NullValue> {
        match value {
            0 => ::std::option::Option::Some(NullValue::NULL_VALUE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [NullValue] = &[
            NullValue::NULL_VALUE,
        ];
        values
    }
}

impl ::std::marker::Copy for NullValue {
}

impl ::std::default::Default for NullValue {
    fn default() -> Self {
        NullValue::NULL_VALUE
    }
}

