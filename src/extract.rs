///! This module provides functions to extract information from a Java bytecode file.
///!
///! The format of a bytecode file is:
///!```ignore
///! ClassFile {
///!     u4             magic;
///!     u2             minor_version;
///!     u2             major_version;
///!     u2             constant_pool_count;
///!     cp_info        constant_pool[constant_pool_count - 1];
///!     u2             access_flags;
///!     u2             this_class;
///!     u2             super_class;
///!     u2             interfaces_count;
///!     u2             interfaces[interfaces_count];
///!     u2             fields_count;
///!     field_info     fields[fields_count];
///!     u2             methods_count;
///!     method_info    methods[methods_count];
///!     u2             attributes_count;
///!     attribute_info attributes[attributes_count];
///! }
///!```
use fmt::*;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::prelude::*;
use std::fs::File;
use std::str;

pub fn extract_magic(mut class_file: &File) -> String {
    let mut buffer = [0; 4];
    class_file.read(&mut buffer[..]).unwrap();
    format!("{}", format_entry("magic:", &buffer))
}

pub fn extract_minor_version(mut class_file: &File) -> String {
    let mut buffer = [0; 2];
    class_file.read(&mut buffer[..]).unwrap();
    let mut rdr = Cursor::new(&buffer);
    format!("{} (d{})",
        format_entry("minor_version:", &buffer),
        rdr.read_u16::<BigEndian>().unwrap())
}

pub fn extract_major_version(mut class_file: &File) -> String {
    let mut buffer = [0; 2];
    class_file.read(&mut buffer[..]).unwrap();
    let mut rdr = Cursor::new(&buffer);
    format!("{} (d{})",
        format_entry("minor_version:", &buffer),
        rdr.read_u16::<BigEndian>().unwrap())
}

/// Extracts the constant pool.
///
/// All constant_pool table entries have the following general format:
/// ```ignore
/// cp_info {
///     u1 tag;
///     u1 info[];
/// }
/// ```
/// Constant pool tags:
/// - CONSTANT_Class = 7
/// - CONSTANT_Fieldref = 9
/// - CONSTANT_Methodref = 10
/// - CONSTANT_InterfaceMethodref = 11
/// - CONSTANT_String = 8
/// - CONSTANT_Integer = 3
/// - CONSTANT_Float = 4
/// - CONSTANT_Long = 5
/// - CONSTANT_Double = 6
/// - CONSTANT_NameAndType = 12
/// - CONSTANT_Utf8 = 1
/// - CONSTANT_MethodHandle = 15
/// - CONSTANT_MethodType = 16
/// - CONSTANT_InvokeDynamic = 18
pub fn extract_constant_pool(mut class_file: &File) -> String {
    let mut buffer = [0; 2];
    class_file.read(&mut buffer[..]).unwrap();
    let mut rdr = Cursor::new(&buffer);
    let number_of_constants = rdr.read_u16::<BigEndian>().unwrap();
    let mut format_buffer = String::new();
    format_buffer.push_str(&format!("{} (d{})\n",
        format_entry("constant_pool_count:", &buffer),
        number_of_constants));

    for constant_number in 1..number_of_constants {
        format_buffer.push_str(&format!(" constant #{}:\n", constant_number));

        let mut buffer = [0; 1];
        class_file.read(&mut buffer[..]).unwrap();
        let constant_type = buffer[0];
        format_buffer.push_str(
            &format!("  type {} ({})\n", constant_type, format_constant_type(constant_type)));

        match constant_type {
            1 => {
                // CONSTANT_Utf8
                let mut buffer = [0; 2];
                class_file.read(&mut buffer[..]).unwrap();
                let mut rdr = Cursor::new(&buffer);
                let length = rdr.read_u16::<BigEndian>().unwrap();
                let mut buffer = vec![0; length as usize];
                class_file.read(&mut buffer[..]).unwrap();
                let content = str::from_utf8(&buffer).unwrap();
                format_buffer.push_str(
                    &format!("  bytes: {} (\"{}\")\n", format_bytes_as_hex(&buffer), content));
            },
            3 | 4 => {
                // CONSTANT_Integer
                // CONSTANT_Float
                let mut buffer = [0; 4];
                class_file.read(&mut buffer[..]).unwrap();
                format_buffer.push_str(
                    &format!("  bytes: {}\n", format_bytes_as_hex(&buffer)));
            },
            5 | 6 => {
                // CONSTANT_Long
                // CONSTANT_Double
                let mut buffer = [0; 4];
                class_file.read(&mut buffer[..]).unwrap();
                format_buffer.push_str(
                    &format!("  high_bytes: {}\n", format_bytes_as_hex(&buffer)));

                let mut buffer = [0; 4];
                class_file.read(&mut buffer[..]).unwrap();
                format_buffer.push_str(
                    &format!("  low_bytes: {}\n", format_bytes_as_hex(&buffer)));
            },
            7 => {
                // CONSTANT_Class
                let mut buffer = [0; 2];
                class_file.read(&mut buffer[..]).unwrap();
                let mut rdr = Cursor::new(&buffer);
                let name_index = rdr.read_u16::<BigEndian>().unwrap();
                format_buffer.push_str(&format!("  name_index: #{}\n", name_index));
            },
            8 => {
                // CONSTANT_String
                let mut buffer = [0; 2];
                class_file.read(&mut buffer[..]).unwrap();
                let mut rdr = Cursor::new(&buffer);
                let string_index = rdr.read_u16::<BigEndian>().unwrap();
                format_buffer.push_str(&format!("  string_index: #{}\n", string_index));
            },
            9 | 10 | 11 => {
                // CONSTANT_Fieldref
                // CONSTANT_Methodref
                // CONSTANT_InterfaceMethodref
                let mut buffer = [0; 2];
                class_file.read(&mut buffer[..]).unwrap();
                let mut rdr = Cursor::new(&buffer);
                let class_index = rdr.read_u16::<BigEndian>().unwrap();
                format_buffer.push_str(&format!("  class_index: #{}\n", class_index));

                let mut buffer = [0; 2];
                class_file.read(&mut buffer[..]).unwrap();
                let mut rdr = Cursor::new(&buffer);
                let name_and_type_index = rdr.read_u16::<BigEndian>().unwrap();
                format_buffer.push_str(&format!("  name_and_type_index: #{}\n", name_and_type_index));
            },
            12 => {
                // CONSTANT_NameAndType
                let mut buffer = [0; 2];
                class_file.read(&mut buffer[..]).unwrap();
                let mut rdr = Cursor::new(&buffer);
                let name_index = rdr.read_u16::<BigEndian>().unwrap();
                format_buffer.push_str(&format!("  name_index: #{}\n", name_index));

                let mut buffer = [0; 2];
                class_file.read(&mut buffer[..]).unwrap();
                let mut rdr = Cursor::new(&buffer);
                let descriptor_index = rdr.read_u16::<BigEndian>().unwrap();
                format_buffer.push_str(&format!("  descriptor_index: #{}\n", descriptor_index));
            },
            15 => {
                // CONSTANT_MethodHandle
                let mut buffer = [0; 1];
                class_file.read(&mut buffer[..]).unwrap();
                let reference_kind = buffer[0];
                format_buffer.push_str(&format!("  reference_kind: #{}\n", reference_kind));

                let mut buffer = [0; 2];
                class_file.read(&mut buffer[..]).unwrap();
                let mut rdr = Cursor::new(&buffer);
                let reference_index = rdr.read_u16::<BigEndian>().unwrap();
                format_buffer.push_str(&format!("  reference_index: #{}\n", reference_index));
            },
            16 => {
                // CONSTANT_MethodType
                let mut buffer = [0; 2];
                class_file.read(&mut buffer[..]).unwrap();
                let mut rdr = Cursor::new(&buffer);
                let descriptor_index = rdr.read_u16::<BigEndian>().unwrap();
                format_buffer.push_str(&format!("  descriptor_index: #{}\n", descriptor_index));
            },
            18 => {
                // CONSTANT_InvokeDynamic
                let mut buffer = [0; 2];
                class_file.read(&mut buffer[..]).unwrap();
                let mut rdr = Cursor::new(&buffer);
                let bootstrap_method_attr_index = rdr.read_u16::<BigEndian>().unwrap();
                format_buffer.push_str(
                    &format!("  bootstrap_method_attr_index: #{}\n", bootstrap_method_attr_index));

                let mut buffer = [0; 2];
                class_file.read(&mut buffer[..]).unwrap();
                let mut rdr = Cursor::new(&buffer);
                let name_and_type_index = rdr.read_u16::<BigEndian>().unwrap();
                format_buffer.push_str(&format!("  name_and_type_index: #{}\n", name_and_type_index));
            },
            _ => format_buffer.push_str(&format!("Unknown constant type: {}!", constant_type)),
        }
    }

    format_buffer
}
