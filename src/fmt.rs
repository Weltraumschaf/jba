use std::fmt::Write;

const NAME_WIDTH: usize = 21;

pub fn format_entry(name: &str, bytes: &[u8]) -> String {
    format!("{}{}",
        pad_name(name),
        format_bytes_as_hex(&bytes)
    )
}

pub fn format_bytes_as_hex(bytes: &[u8]) -> String {
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

pub fn format_constant_type(constant_type: u8) -> &'static str {
    match constant_type {
         1 => "CONSTANT_Utf8",
         3 => "CONSTANT_Integer",
         4 => "CONSTANT_Float",
         5 => "CONSTANT_Long",
         6 => "CONSTANT_Double",
         7 => "CONSTANT_Class",
         8 => "CONSTANT_String",
         9 => "CONSTANT_Fieldref",
        10 => "CONSTANT_Methodref",
        11 => "CONSTANT_InterfaceMethodref",
        12 => "CONSTANT_NameAndType",
        15 => "CONSTANT_MethodHandle",
        16 => "CONSTANT_MethodType",
        18 => "CONSTANT_InvokeDynamic",
        _ => "unknown",
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