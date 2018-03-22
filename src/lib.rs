#[cfg(test)]
#[macro_use]
extern crate hamcrest;
extern crate byteorder;

mod extract;
mod fmt;

use std::fs::File;
use extract::*;

pub fn analyze_file(file_name: &str) -> Result<(), String> {
    println!("{}:", file_name);
    println!("{}", "=".repeat(file_name.len() + 1));

    let class_file = match File::open(file_name) {
        Ok(file) => file,
        Err(_) => {
            return Err(format!("Can' read file {}!", file_name));
        }
    };

    println!("{}", extract_magic(&class_file));
    println!("{}", extract_minor_version(&class_file));
    println!("{}", extract_major_version(&class_file));
    println!("{}", extract_constant_pool(&class_file));

    Ok(())
}

