#[cfg(test)]
#[macro_use]
extern crate hamcrest;
extern crate byteorder;

use std::io::prelude::*;

use std::fs::File;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use std::fmt::Write;

const NAME_WIDTH: usize = 21;

pub fn analyze_file(file_name: &str) -> Result<(), String> {
    println!("{}:", file_name);
    println!("{}", "=".repeat(file_name.len() + 1));

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
    println!("{}", extract_magic(&class_file));
    println!("{}", extract_minor_version(&class_file));
    println!("{}", extract_major_version(&class_file));
    println!("{}", extract_constant_pool(&class_file));

    Ok(())
}

fn extract_magic(mut class_file: &File) -> String {
    let mut buffer = [0; 4];
    class_file.read(&mut buffer[..]).unwrap();
    format!("{}", format_entry("magic:", &buffer))
}

fn extract_minor_version(mut class_file: &File) -> String {
    let mut buffer = [0; 2];
    class_file.read(&mut buffer[..]).unwrap();
    let mut rdr = Cursor::new(&buffer);
    format!("{} (d{})",
        format_entry("minor_version:", &buffer),
        rdr.read_u16::<BigEndian>().unwrap())
}

fn extract_major_version(mut class_file: &File) -> String {
    let mut buffer = [0; 2];
    class_file.read(&mut buffer[..]).unwrap();
    let mut rdr = Cursor::new(&buffer);
    format!("{} (d{})",
        format_entry("minor_version:", &buffer),
        rdr.read_u16::<BigEndian>().unwrap())
}

fn extract_constant_pool(mut class_file: &File) -> String {
    let mut buffer = [0; 2];
    class_file.read(&mut buffer[..]).unwrap();
    let mut rdr = Cursor::new(&buffer);
    format!("{} (d{})",
        format_entry("constant_pool_count:", &buffer),
        rdr.read_u16::<BigEndian>().unwrap())
}

fn format_entry(name: &str, bytes: &[u8]) -> String {
    format!("{}{}",
        pad_name(name),
        format_bytes_as_hex(&bytes)
    )
}

fn format_bytes_as_hex(bytes: &[u8]) -> String {
    let mut formatted = String::new();

    for &byte in bytes.iter() {
        write!(&mut formatted, "{:X} ", byte).expect("Unable to write");
    }

    formatted.trim().to_owned()
}

fn pad_name(name: &str) -> String {
    if name.len() < NAME_WIDTH {
        let pad_size: usize = NAME_WIDTH - name.len();
        let pad = (0..pad_size).map(|_| " ").collect::<String>();
        format!("{}{}", name, pad)
    } else {
        format!("{}", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn pad_name_name_is_shorter_than_wanted() {
        assert_that!(&pad_name("foo"), is(equal_to("foo                  ")));
    }

    #[test]
    fn pad_name_name_has_same_length_as_wanted() {
        assert_that!(&pad_name("foofoofoofoofoofoofoo"), is(equal_to("foofoofoofoofoofoofoo")));
    }

    #[test]
    fn pad_name_name_is_longer_than_wanted() {
        assert_that!(&pad_name("foofoofoofoofoofoofoofoo"), is(equal_to("foofoofoofoofoofoofoofoo")));
    }
}