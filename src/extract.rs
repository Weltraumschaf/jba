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

pub fn extract_constant_pool(mut class_file: &File) -> String {
    let mut buffer = [0; 2];
    class_file.read(&mut buffer[..]).unwrap();
    let mut rdr = Cursor::new(&buffer);
    format!("{} (d{})",
        format_entry("constant_pool_count:", &buffer),
        rdr.read_u16::<BigEndian>().unwrap())
}