#[cfg(test)]
#[macro_use]
extern crate hamcrest;
extern crate byteorder;

use std::io::prelude::*;

use std::fs::File;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use std::fmt::Write;

pub fn analyze_file(file_name: &str) -> Result<(), String> {
    println!("{}:", file_name);

    let class_file = match File::open(file_name) {
        Ok(file) => file,
        Err(_) => {
            return Err(format!("Can' read file {}!", file_name));
        }
    };

    /*
    ClassFile {
        u4             magic;
        u2             minor_version;
        u2             major_version;
        u2             constant_pool_count;
        cp_info        constant_pool[constant_pool_count-1];
        u2             access_flags;
        u2             this_class;
        u2             super_class;
        u2             interfaces_count;
        u2             interfaces[interfaces_count];
        u2             fields_count;
        field_info     fields[fields_count];
        u2             methods_count;
        method_info    methods[methods_count];
        u2             attributes_count;
        attribute_info attributes[attributes_count];
    }
    */
    extract_magic(&class_file);
    extract_minor_version(&class_file);
    extract_major_version(&class_file);

    Ok(())
}

fn extract_magic(mut class_file: &File) {
    let mut buffer = [0; 4];
    class_file.read(&mut buffer[..]).unwrap();
    println!("magic:         {}", format_bytes_as_hex(&buffer));
}

fn extract_minor_version(mut class_file: &File) {
    let mut buffer = [0; 2];
    class_file.read(&mut buffer[..]).unwrap();
    let mut rdr = Cursor::new(&buffer);
    println!("minor_version: {} (d{})",
        format_bytes_as_hex(&buffer),
        rdr.read_u16::<BigEndian>().unwrap());
}

fn extract_major_version(mut class_file: &File) {
    let mut buffer = [0; 2];
    class_file.read(&mut buffer[..]).unwrap();
    let mut rdr = Cursor::new(&buffer);
    println!("minor_version: {} (d{})",
        format_bytes_as_hex(&buffer),
        rdr.read_u16::<BigEndian>().unwrap());
}

fn format_bytes_as_hex(bytes: &[u8]) -> String {
    let mut formatted = String::new();

    for &byte in bytes.iter() {
        write!(&mut formatted, "{:X} ", byte).expect("Unable to write");
    }

    formatted.trim().to_owned()
}


