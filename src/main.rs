use std::error::Error;

use ccxxd::{
    get_input, get_output,
    hex::{
        read_as_hex::{read_bytes_as_hex, Hexdump},
        read_from_hex::read_hex_as_bytes,
    },
    utils::arg_parser::get_matches,
};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = get_matches();

    let mut input = get_input(matches.input)?;
    let mut output = get_output(matches.output)?;

    if matches.reverse {
        read_hex_as_bytes(&mut input, &mut output)?;
    } else {
        let matches = Hexdump {
            cols: matches.cols,
            little_endian: matches.little_endian,
            group_size: matches.group_size,
            len: matches.len,
            seek: matches.seek,
        };
        read_bytes_as_hex(matches, &mut input, &mut output)?;
    }

    Ok(())
}
