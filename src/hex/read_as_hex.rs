use std::{
    error::Error,
    io::{Read, Write},
};

use crate::utils::power::is_power_of_two;

pub struct Hexdump {
    pub cols: u16,
    pub little_endian: bool,
    pub group_size: u8,
    pub len: u32,
    pub seek: u32,
}

pub fn read_bytes_as_hex(
    matches: Hexdump,
    input: &mut Box<dyn Read>,
    output: &mut Box<dyn Write>,
) -> Result<(), Box<dyn Error>> {
    let Hexdump {
        cols,
        little_endian,
        group_size,
        len,
        seek,
    } = matches;

    let mut index: u32 = 0;
    let leftover: u16;
    let mut bytes_written: u32 = 0;
    let mut chars_written: u32 = 0;

    {
        let mut skipped_bytes = vec![0; seek as usize];
        input.read_exact(&mut skipped_bytes)?;
        index += seek / cols as u32;
        leftover = (seek % cols as u32) as u16;
    }

    loop {
        let mut buffer = vec![0; cols as usize];
        let bytes_read = input.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        write!(output, "{:08x}:  ", (index * cols as u32) + leftover as u32)?;

        let chunks = buffer.chunks(group_size as usize);

        for bytes in chunks {
            let bytes = if little_endian {
                if !is_power_of_two(group_size as u32) {
                    panic!("Group size must be a power of two in little-endian mode.");
                }
                let mut bytes = bytes.to_vec();
                bytes.reverse();
                bytes
            } else {
                bytes.to_vec()
            };

            for byte in bytes {
                if bytes_written >= len {
                    write!(output, "  ")?;
                } else {
                    write!(output, "{:02x}", byte)?;
                    bytes_written += 1;
                }
            }

            write!(output, " ")?;
        }

        write!(output, "  ")?;

        for byte in buffer.iter() {
            if chars_written < len {
                let c = if *byte >= 32 && *byte <= 126 {
                    *byte as char
                } else {
                    '.'
                };
                write!(output, "{}", c)?;
                chars_written += 1;
            }
        }

        writeln!(output)?;

        if bytes_written >= len {
            break;
        }

        index += 1;
    }

    Ok(())
}
