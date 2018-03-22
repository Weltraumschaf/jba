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

    for constant_number in 1..number_of_constants + 1 {
        format_buffer.push_str(&format!("constant #{}\n", constant_number));
    }

    format_buffer
}