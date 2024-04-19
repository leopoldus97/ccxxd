use std::{
    error::Error,
    io::{Read, Write},
};

pub fn read_hex_as_bytes(
    input: &mut Box<dyn Read>,
    output: &mut Box<dyn Write>,
) -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)?;

    let mut bytes = Vec::new();
    let mut byte = 0;
    let mut byte_count = 0;

    for line in buffer.lines() {
        let buffer = line.split("  ").collect::<Vec<&str>>()[1];

        for c in buffer.chars() {
            if c.is_whitespace() {
                continue;
            }

            let byte_value = match c {
                '0'..='9' => c as u8 - b'0',
                'a'..='f' => c as u8 - b'a' + 10,
                'A'..='F' => c as u8 - b'A' + 10,
                _ => continue,
            };

            byte = (byte << 4) | byte_value;
            byte_count += 1;

            if byte_count == 2 {
                bytes.push(byte);
                byte = 0;
                byte_count = 0;
            }
        }
    }

    output.write_all(&bytes)?;

    Ok(())
}
