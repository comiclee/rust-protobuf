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
pub struct SourceContext {
    // message fields
    pub file_name: ::std::string::String,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl SourceContext {
    pub fn new() -> SourceContext {
        ::std::default::Default::default()
    }

    // string file_name = 1;

    pub fn clear_file_name(&mut self) {
        self.file_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_file_name(&mut self, v: ::std::string::String) {
        self.file_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file_name(&mut self) -> &mut ::std::string::String {
        &mut self.file_name
    }

    // Take field
    pub fn take_file_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.file_name, ::std::string::String::new())
    }

    pub fn get_file_name(&self) -> &str {
        &self.file_name
    }
}

impl ::protobuf::Message for SourceContext {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.file_name)?;
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
        if !self.file_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.file_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.file_name.is_empty() {
            os.write_string(1, &self.file_name)?;
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


    fn new() -> SourceContext {
        SourceContext::new()
    }
}

impl ::protobuf::Clear for SourceContext {
    fn clear(&mut self) {
        self.clear_file_name();
        self.unknown_fields.clear();
    }
}
